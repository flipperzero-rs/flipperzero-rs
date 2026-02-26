use core::ffi::c_void;
use core::num::NonZero;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::furi::stream_buffer::StreamBuffer;
use crate::furi::thread::{self, ThreadId};
use crate::furi::time::FuriDuration;
use crate::{debug, furi, trace, warn};
use flipperzero_sys::{self as sys, HasFlag};
use sys::furi::FuriBox;

pub type SerialId = sys::FuriHalSerialId;

pub const LPUART: SerialId = sys::FuriHalSerialIdLpuart;
pub const USART: SerialId = sys::FuriHalSerialIdUsart;

/// Handle to Serial interface.
pub struct SerialHandle {
    handle: NonNull<sys::FuriHalSerialHandle>,
}

impl SerialHandle {
    /// Acquire Serial interface.
    ///
    /// Returns [`furi::Error::ResourceBusy`] if interface is currently in use.
    pub fn acquire(serial_id: SerialId) -> furi::Result<Self> {
        let handle = unsafe { sys::furi_hal_serial_control_acquire(serial_id) };

        let handle = match NonNull::new(handle) {
            None => return Err(furi::Error::ResourceBusy),
            Some(h) => h,
        };

        Ok(SerialHandle { handle })
    }

    /// Get raw Serial Handle.
    ///
    /// You must not deallocate, free or otherwise invalidate this pointer otherwise undefined behaviour will result.
    pub fn as_ptr(&self) -> *mut sys::FuriHalSerialHandle {
        self.handle.as_ptr()
    }

    /// Initialize Serial.
    ///
    /// Configures GPIO, configures and enables transceiver.
    pub fn init(&self, baud: u32) {
        unsafe { sys::furi_hal_serial_init(self.handle.as_ptr(), baud) }
    }

    /// Deinitialize Serial.
    ///
    /// Configures GPIO to analog, clears callback and callback context, disables hardware.
    pub fn deinit(&self) {
        unsafe { sys::furi_hal_serial_deinit(self.handle.as_ptr()) }
    }

    /// Suspend operation.
    ///
    /// Suspend hardware, settings and callbacks are preserved.
    pub fn suspend(&self) {
        unsafe { sys::furi_hal_serial_suspend(self.handle.as_ptr()) }
    }

    /// Resume operation.
    ///
    /// Resume hardware from suspended state.
    pub fn resume(&self) {
        unsafe { sys::furi_hal_serial_resume(self.handle.as_ptr()) }
    }

    /// Check if baud rate supported.
    pub fn is_baud_rate_supported(&self, baud: u32) -> bool {
        unsafe { sys::furi_hal_serial_is_baud_rate_supported(self.handle.as_ptr(), baud) }
    }

    /// Set baud rate.
    pub fn set_baud_rate(&self, baud: u32) {
        unsafe { sys::furi_hal_serial_set_br(self.handle.as_ptr(), baud) }
    }

    /// Transmits data in semi-blocking mode
    ///
    /// Fills transmission pipe with data, returns as soon as all bytes from buffer are in the pipe.
    ///
    /// Real transmission will be completed later. Use [`SerialHandle::tx_wait_complete`] to wait for completion if you need it.
    pub fn tx(&self, buffer: &[u8]) {
        unsafe { sys::furi_hal_serial_tx(self.handle.as_ptr(), buffer.as_ptr(), buffer.len()) }
    }

    /// Wait until transmission is completed.
    ///
    /// Ensures that all data has been sent.
    pub fn tx_wait_complete(&self) {
        unsafe { sys::furi_hal_serial_tx_wait_complete(self.handle.as_ptr()) }
    }

    pub fn async_receiver<F: FnMut(&[u8])>(&self, on_rx: F) -> AsyncSerialReceiver<'_, F> {
        AsyncSerialReceiver::new(self, on_rx)
    }
}

impl Drop for SerialHandle {
    fn drop(&mut self) {
        unsafe { sys::furi_hal_serial_control_release(self.handle.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WorkerEvent(u32);

impl WorkerEvent {
    /// Stop worker.
    pub const FLAG_STOP: u32 = (1 << 0);
    /// New data available.
    pub const FLAG_DATA: u32 = (1 << 1);
    /// Bus idle detected.
    pub const FLAG_IDLE: u32 = (1 << 2);
    /// No space for received data.
    pub const FLAG_OVERRUN_ERROR: u32 = (1 << 3);
    /// Incorrect frame detected.
    pub const FLAG_FRAMING_ERROR: u32 = (1 << 4);
    /// Noise on the line detected.
    pub const FLAG_NOISE_ERROR: u32 = (1 << 5);

    /// Mask of all supported events.
    pub const MASK: u32 = Self::FLAG_STOP
        | Self::FLAG_DATA
        | Self::FLAG_IDLE
        | Self::FLAG_OVERRUN_ERROR
        | Self::FLAG_FRAMING_ERROR
        | Self::FLAG_NOISE_ERROR;

    pub fn is_stop(self) -> bool {
        self.0 & Self::FLAG_STOP != 0
    }

    pub fn is_rx_data(self) -> bool {
        self.0 & Self::FLAG_DATA != 0
    }

    pub fn is_rx_idle(self) -> bool {
        self.0 & Self::FLAG_IDLE != 0
    }

    pub fn is_error(self) -> bool {
        self.0 & (Self::FLAG_OVERRUN_ERROR | Self::FLAG_FRAMING_ERROR | Self::FLAG_NOISE_ERROR) != 0
    }

    pub fn is_overrun_error(self) -> bool {
        self.0 & Self::FLAG_OVERRUN_ERROR != 0
    }

    pub fn is_framing_error(self) -> bool {
        self.0 & Self::FLAG_FRAMING_ERROR != 0
    }

    pub fn is_noise_error(self) -> bool {
        self.0 & Self::FLAG_NOISE_ERROR != 0
    }
}

/// Asyncronous receiver of serial data.
///
/// This spawns a dedicated worker thread to dispatch received data.
pub struct AsyncSerialReceiver<'a, F>
where
    F: FnMut(&[u8]),
{
    handle: &'a SerialHandle,
    context: FuriBox<Context<F>>,
}

struct Context<F: FnMut(&[u8])> {
    rx_stream: StreamBuffer,
    on_rx: F,
    worker_thread: AtomicPtr<sys::FuriThread>,
}

impl<'a, F> AsyncSerialReceiver<'a, F>
where
    F: FnMut(&[u8]),
{
    fn new(handle: &'a SerialHandle, on_rx: F) -> Self {
        let rx_stream = StreamBuffer::new(NonZero::new(2048).unwrap(), 1);

        let mut context = FuriBox::new(Context {
            rx_stream,
            on_rx,
            worker_thread: AtomicPtr::new(ptr::null_mut()),
        });

        unsafe {
            // SAFETY: Grabbing the context pointer with `as_mut_ptr` is fine,
            // since it doesn't create an intermediate referece.
            let worker_thread = sys::furi_thread_alloc_ex(
                c"AsyncSerialReceiverWorker".as_ptr(),
                1024,
                Some(async_serial_receiver_worker::<F>),
                FuriBox::as_mut_ptr(&mut context).cast::<c_void>(),
            );

            // SAFETY: Since thread hasn't started yet, it's still safe to reference `Context`.
            context
                .worker_thread
                .store(worker_thread, Ordering::Release);

            // SAFETY: From this point on we must carefully respect the aliasing rules.
            sys::furi_thread_start(worker_thread);

            // SAFETY: Grabbing the context pointer with `as_mut_ptr` is fine,
            // since it doesn't create an intermediate referece.
            sys::furi_hal_serial_async_rx_start(
                handle.as_ptr(),
                Some(async_serial_receiver_rx_callback::<F>),
                FuriBox::as_mut_ptr(&mut context).cast(),
                true,
            );
        }

        AsyncSerialReceiver { handle, context }
    }
}

impl<F: FnMut(&[u8])> Drop for AsyncSerialReceiver<'_, F> {
    fn drop(&mut self) {
        // Ensure that callback is removed so it no longer references `Context`.
        unsafe { sys::furi_hal_serial_async_rx_stop(self.handle.as_ptr()) };

        // SAFETY: Worker thread is still running, so be careful not to create a reference to `Context`.
        // Using `as_mut_ptr` is fine since it only creates a reference to the `Box` not the `Context` inside.
        let context = FuriBox::as_mut_ptr(&mut self.context);
        let worker_thread = unsafe { (*context).worker_thread.load(Ordering::Acquire) };

        if !worker_thread.is_null() {
            let thread_id = unsafe { thread::ThreadId::from_furi_thread(worker_thread) };
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

unsafe extern "C" fn async_serial_receiver_rx_callback<F: FnMut(&[u8])>(
    handle: *mut sys::FuriHalSerialHandle,
    event: sys::FuriHalSerialRxEvent,
    context: *mut c_void,
) {
    let context = context.cast_const().cast::<Context<F>>();

    let mut flags = 0u32;

    if event.has_flag(sys::FuriHalSerialRxEventData) {
        let data = [unsafe { sys::furi_hal_serial_async_rx(handle) }];

        unsafe { (*context).rx_stream.send(&data, FuriDuration::ZERO) };
        flags |= WorkerEvent::FLAG_DATA;
    }

    if event.has_flag(sys::FuriHalSerialRxEventIdle) {
        flags |= WorkerEvent::FLAG_IDLE;
    }

    if event.has_flag(sys::FuriHalSerialRxEventOverrunError) {
        flags |= WorkerEvent::FLAG_OVERRUN_ERROR;
    }

    if event.has_flag(sys::FuriHalSerialRxEventFrameError) {
        flags |= WorkerEvent::FLAG_FRAMING_ERROR;
    }

    if event.has_flag(sys::FuriHalSerialRxEventNoiseError) {
        flags |= WorkerEvent::FLAG_NOISE_ERROR;
    }

    let worker_thread = unsafe { (*context).worker_thread.load(Ordering::Acquire) };
    if !worker_thread.is_null() {
        let thread_id = unsafe { ThreadId::from_furi_thread(worker_thread) };
        thread::set_flags(thread_id, flags).unwrap();
    }
}

const SERIAL_WORKER_BUFFER_LEN: usize = 64;

unsafe extern "C" fn async_serial_receiver_worker<F: FnMut(&[u8])>(context: *mut c_void) -> i32 {
    debug!("Starting async worker");
    assert!(!context.is_null());
    let context = context.cast::<Context<F>>();

    loop {
        let events = WorkerEvent(
            thread::wait_any_flags(WorkerEvent::MASK, true, FuriDuration::MAX).unwrap_or(0),
        );
        trace!("WorkerEvent: {}", events.0);

        if events.is_stop() {
            debug!("Stopping async worker");
            break;
        }

        if events.is_rx_data() {
            loop {
                let mut data = [0u8; SERIAL_WORKER_BUFFER_LEN];
                let len = unsafe { (*context).rx_stream.receive(&mut data, FuriDuration::ZERO) };

                if len == 0 {
                    break;
                }

                unsafe { ((*context).on_rx)(&data[..len]) }
            }
        }

        if events.is_rx_idle() {
            trace!("idle");
        }

        if events.is_error() {
            if events.is_overrun_error() {
                warn!("overrun");
            }

            if events.is_framing_error() {
                warn!("framing error");
            }

            if events.is_noise_error() {
                warn!("noise error");
            }
        }
    }

    0
}
