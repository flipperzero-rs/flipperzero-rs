//! Bluetooth APIs for the Flipper Zero

use crate::{error, furi::string::FuriString};
use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

pub mod beacon;
pub mod test_patterns;

/// Returns `true` if core2 (which runs Bluetooth) is alive.
pub fn is_alive() -> bool {
    unsafe { sys::furi_hal_bt_is_alive() }
}

/// Checks if BLE state is active.
///
/// Returns `true` if the device is connected or advertising.
pub fn is_active() -> bool {
    unsafe { sys::furi_hal_bt_is_active() }
}

/// Returns a string containing the BT/BLE system component state.
pub fn dump_state() -> FuriString {
    let mut buffer = FuriString::new();
    unsafe { sys::furi_hal_bt_dump_state(buffer.as_mut_ptr()) }
    buffer
}

/// A handle to the Bluetooth service.
pub(crate) struct Bluetooth {
    bt: UnsafeRecord<sys::Bt>,
}

impl Drop for Bluetooth {
    fn drop(&mut self) {
        if !unsafe { sys::bt_profile_restore_default(self.bt.as_ptr()) } {
            error!("Failed to restore default Bluetooth profile");
        }
    }
}

impl Bluetooth {
    /// Obtains a handle to the Bluetooth service.
    pub(crate) fn open() -> Self {
        unsafe {
            let bt = UnsafeRecord::open(c"bt");
            sys::bt_disconnect(bt.as_ptr());
            Self { bt }
        }
    }
}
