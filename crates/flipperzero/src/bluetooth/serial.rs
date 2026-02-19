//! BLE serial profile wrapper.
//!
//! This module provides a safe Rust interface to the Flipper Zero's BLE serial profile,
//! allowing apps to expose a BLE serial interface for communication with phones and
//! other BLE central devices.

use core::ffi::c_void;
use core::num::NonZero;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::furi::stream_buffer::StreamBuffer;
use crate::furi::thread::{self, ThreadId};
use crate::furi::time::FuriDuration;
use crate::{debug, trace};
use flipperzero_sys::{self as sys};
use sys::furi::FuriBox;

use super::Bluetooth;

/// BLE serial errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Failed to start the BLE serial profile.
    FailedToStartProfile,
}

/// Handle to the BLE serial profile.
///
/// Makes the Flipper appear as a BLE serial device. The default Bluetooth profile
/// is restored when this handle is dropped.
pub struct BleSerial {
    // Field order matters: `profile` has no Drop impl but must remain valid until
    // `_bt` is dropped (which calls `bt_profile_restore_default`).
    profile: NonNull<sys::FuriHalBleProfileBase>,
    _bt: Bluetooth,
}

impl BleSerial {
    /// Start the BLE serial profile.
    ///
    /// Makes the Flipper appear as a BLE serial device. The default profile
    /// is restored when this handle is dropped.
    pub fn start() -> Result<Self, Error> {
        let bt = Bluetooth::open();

        let profile = unsafe {
            sys::bt_profile_start(bt.as_ptr(), sys::ble_profile_serial, ptr::null_mut())
        };

        let profile = NonNull::new(profile).ok_or(Error::FailedToStartProfile)?;

        Ok(BleSerial { profile, _bt: bt })
    }

    /// Send data over BLE serial.
    ///
    /// Returns `true` on success.
    pub fn tx(&self, data: &[u8]) -> bool {
        unsafe {
            sys::ble_profile_serial_tx(
                self.profile.as_ptr(),
                data.as_ptr() as *mut u8,
                data.len() as u16,
            )
        }
    }

    /// Set the RPC active status.
    pub fn set_rpc_active(&self, active: bool) {
        unsafe { sys::ble_profile_serial_set_rpc_active(self.profile.as_ptr(), active) }
    }

    /// Notify that the application buffer is empty.
    pub fn notify_buffer_is_empty(&self) {
        unsafe { sys::ble_profile_serial_notify_buffer_is_empty(self.profile.as_ptr()) }
    }

    /// Create an async receiver for incoming BLE serial data.
    ///
    /// The provided callback will be invoked with received data on a dedicated worker thread.
    /// The receiver is active until dropped.
    pub fn async_receiver<F: FnMut(&[u8])>(&self, on_rx: F) -> AsyncBleSerialReceiver<'_, F> {
        AsyncBleSerialReceiver::new(self, on_rx)
    }
}

// BLE serial callback buffer size passed to `ble_profile_serial_set_event_callback`.
const BLE_SERIAL_BUFFER_SIZE: u16 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WorkerEvent(u32);

impl WorkerEvent {
    /// Stop worker.
    pub const FLAG_STOP: u32 = 1 << 0;
    /// New data available.
    pub const FLAG_DATA: u32 = 1 << 1;
    /// Data sent notification.
    pub const FLAG_DATA_SENT: u32 = 1 << 2;

    /// Mask of all supported events.
    pub const MASK: u32 = Self::FLAG_STOP | Self::FLAG_DATA | Self::FLAG_DATA_SENT;

    pub fn is_stop(self) -> bool {
        self.0 & Self::FLAG_STOP != 0
    }

    pub fn is_rx_data(self) -> bool {
        self.0 & Self::FLAG_DATA != 0
    }

    pub fn is_data_sent(self) -> bool {
        self.0 & Self::FLAG_DATA_SENT != 0
    }
}

/// Async receiver that dispatches BLE serial data via callback.
///
/// This spawns a dedicated worker thread to dispatch received data.
pub struct AsyncBleSerialReceiver<'a, F>
where
    F: FnMut(&[u8]),
{
    ble_serial: &'a BleSerial,
    context: FuriBox<Context<F>>,
}

struct Context<F: FnMut(&[u8])> {
    rx_stream: StreamBuffer,
    on_rx: F,
    worker_thread: AtomicPtr<sys::FuriThread>,
}

impl<'a, F> AsyncBleSerialReceiver<'a, F>
where
    F: FnMut(&[u8]),
{
    fn new(ble_serial: &'a BleSerial, on_rx: F) -> Self {
        let rx_stream = StreamBuffer::new(NonZero::new(2048).unwrap(), 1);

        let mut context = FuriBox::new(Context {
            rx_stream,
            on_rx,
            worker_thread: AtomicPtr::new(ptr::null_mut()),
        });

        unsafe {
            // SAFETY: Grabbing the context pointer with `as_mut_ptr` is fine,
            // since it doesn't create an intermediate reference.
            let worker_thread = sys::furi_thread_alloc_ex(
                c"AsyncBleSerialWorker".as_ptr(),
                1024,
                Some(async_ble_serial_worker::<F>),
                FuriBox::as_mut_ptr(&mut context) as *mut _,
            );

            // SAFETY: Since thread hasn't started yet, it's still safe to reference `Context`.
            context
                .worker_thread
                .store(worker_thread, Ordering::Release);

            // SAFETY: From this point on we must carefully respect the aliasing rules.
            sys::furi_thread_start(worker_thread);

            // SAFETY: Grabbing the context pointer with `as_mut_ptr` is fine,
            // since it doesn't create an intermediate reference.
            sys::ble_profile_serial_set_event_callback(
                ble_serial.profile.as_ptr(),
                BLE_SERIAL_BUFFER_SIZE,
                Some(ble_serial_event_callback::<F>),
                FuriBox::as_mut_ptr(&mut context).cast(),
            );
        }

        AsyncBleSerialReceiver {
            ble_serial,
            context,
        }
    }
}

impl<F: FnMut(&[u8])> Drop for AsyncBleSerialReceiver<'_, F> {
    fn drop(&mut self) {
        // Clear the callback so it no longer references `Context`.
        unsafe {
            sys::ble_profile_serial_set_event_callback(
                self.ble_serial.profile.as_ptr(),
                0,
                None,
                ptr::null_mut(),
            );
        }

        // SAFETY: Worker thread is still running, so be careful not to create a reference to `Context`.
        // Using `as_mut_ptr` is fine since it only creates a reference to the `Box` not the `Context` inside.
        let context = FuriBox::as_mut_ptr(&mut self.context);
        let worker_thread = unsafe { (*context).worker_thread.load(Ordering::Acquire) };

        if !worker_thread.is_null() {
            let thread_id = unsafe { ThreadId::from_furi_thread(worker_thread) };
            thread::set_flags(thread_id, WorkerEvent::FLAG_STOP).unwrap();

            unsafe {
                (*context)
                    .worker_thread
                    .store(ptr::null_mut(), Ordering::Release);
                sys::furi_thread_join(worker_thread);
                sys::furi_thread_free(worker_thread);
            }
        }
    }
}

unsafe extern "C" fn ble_serial_event_callback<F: FnMut(&[u8])>(
    event: sys::SerialServiceEvent,
    context: *mut c_void,
) -> u16 {
    let context = context.cast_const() as *const Context<F>;

    let mut flags = 0u32;
    let mut consumed: u16 = 0;

    if event.event == sys::SerialServiceEventTypeDataReceived {
        let data =
            unsafe { core::slice::from_raw_parts(event.data.buffer, event.data.size as usize) };

        unsafe { (*context).rx_stream.send(data, FuriDuration::ZERO) };
        flags |= WorkerEvent::FLAG_DATA;
        consumed = event.data.size;
    }

    if event.event == sys::SerialServiceEventTypeDataSent {
        flags |= WorkerEvent::FLAG_DATA_SENT;
    }

    if event.event == sys::SerialServiceEventTypesBleResetRequest {
        flags |= WorkerEvent::FLAG_STOP;
    }

    if flags != 0 {
        let worker_thread = unsafe { (*context).worker_thread.load(Ordering::Acquire) };
        if !worker_thread.is_null() {
            let thread_id = unsafe { ThreadId::from_furi_thread(worker_thread) };
            thread::set_flags(thread_id, flags).unwrap();
        }
    }

    consumed
}

const BLE_SERIAL_WORKER_BUFFER_LEN: usize = 64;

unsafe extern "C" fn async_ble_serial_worker<F: FnMut(&[u8])>(context: *mut c_void) -> i32 {
    debug!("Starting BLE serial async worker");
    assert!(!context.is_null());
    let context = context.cast::<Context<F>>();

    loop {
        let events = WorkerEvent(
            thread::wait_any_flags(WorkerEvent::MASK, true, FuriDuration::MAX).unwrap_or(0),
        );
        trace!("BLE serial WorkerEvent: {}", events.0);

        if events.is_stop() {
            debug!("Stopping BLE serial async worker");
            break;
        }

        if events.is_rx_data() {
            loop {
                let mut data = [0u8; BLE_SERIAL_WORKER_BUFFER_LEN];
                let len = unsafe { (*context).rx_stream.receive(&mut data, FuriDuration::ZERO) };

                if len == 0 {
                    break;
                }

                unsafe { ((*context).on_rx)(&data[..len]) }
            }
        }

        if events.is_data_sent() {
            trace!("BLE serial data sent");
        }
    }

    0
}
