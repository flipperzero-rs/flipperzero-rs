//! Furi Hal RTC

use core::mem::MaybeUninit;

use flipperzero_sys as sys;

pub use crate::datetime::DateTime;

/// Get RTC Date Time.
pub fn datetime() -> DateTime {
    let mut datetime = MaybeUninit::uninit();
    unsafe {
        sys::furi_hal_rtc_get_datetime(datetime.as_mut_ptr());
    }

    unsafe { datetime.assume_init() }
}

/// Set RTC Date Time.
pub fn set_datetime(datetime: &DateTime) {
    unsafe {
        // SAFETY: C function only reads from pointer
        sys::furi_hal_rtc_set_datetime(datetime as *const _ as *mut _);
    }
}

/// Get RTC UNIX Timestamp (seconds from UNIX epoch).
pub fn timestamp() -> u32 {
    unsafe { sys::furi_hal_rtc_get_timestamp() }
}
