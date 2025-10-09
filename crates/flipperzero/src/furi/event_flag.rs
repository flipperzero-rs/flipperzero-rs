//! Furi Event Flag.

use core::ptr::NonNull;

use crate::furi::Error;
use crate::furi::time::FuriDuration;
use flipperzero_sys as sys;
use flipperzero_sys::furi::Status;

pub struct EventFlag {
    raw: NonNull<sys::FuriEventFlag>,
}

impl EventFlag {
    pub fn new() -> Self {
        Self {
            // SAFETY: Alloc always returns valid non-null pointer or triggers `furi_crash`.
            raw: unsafe { NonNull::new_unchecked(sys::furi_event_flag_alloc()) },
        }
    }

    /// Get pointer to raw [`sys::FuriEventFlag`].
    ///
    /// This pointer must not be `free`d or otherwise invalidated.
    /// It must not be referenced after [`EventFlag`] has been dropped.
    pub fn as_ptr(&self) -> *mut sys::FuriEventFlag {
        self.raw.as_ptr()
    }

    /// Set flags.
    ///
    /// # Warning
    /// The result of this function can be flags that you've just asked to
    /// set or not if someone was waiting for them and asked to clear it.
    /// It is highly recommended to read the `furi_event_flag_set`
    /// and `xEventGroupSetBits`` source code.
    pub fn set(&self, flags: u32) -> Result<u32, Error> {
        Status::from(unsafe { sys::furi_event_flag_set(self.as_ptr(), flags) })
            .into_result()
            .map(|s| s as u32)
    }

    /// Clear flags
    pub fn clear(&self, flags: u32) -> Result<u32, Error> {
        Status::from(unsafe { sys::furi_event_flag_clear(self.as_ptr(), flags) })
            .into_result()
            .map(|s| s as u32)
    }

    /// Get flags.
    pub fn get(&self) -> u32 {
        unsafe { sys::furi_event_flag_get(self.as_ptr()) }
    }

    /// Wait for up-to `timeout` for a change to any of the specified notification `flags`.
    ///
    /// If `clear`, then the specified flags will be cleared after a notification is received.
    pub fn wait_any_flags(
        &self,
        flags: u32,
        clear: bool,
        timeout: FuriDuration,
    ) -> Result<u32, Error> {
        let mut options = sys::FuriFlagWaitAny;
        if !clear {
            options |= sys::FuriFlagNoClear;
        }

        Status::from(unsafe {
            sys::furi_event_flag_wait(self.as_ptr(), flags, options.0, timeout.0)
        })
        .into_result()
        .map(|s| s as u32)
    }

    /// Wait for up-to `timeout` for a change to all of the specified notification `flags`.
    ///
    /// If `clear`, then the specified flags will be cleared after a notification is received.
    pub fn wait_all_flags(
        &self,
        flags: u32,
        clear: bool,
        timeout: FuriDuration,
    ) -> Result<u32, Error> {
        let mut options = sys::FuriFlagWaitAll;
        if !clear {
            options |= sys::FuriFlagNoClear;
        }

        Status::from(unsafe {
            sys::furi_event_flag_wait(self.as_ptr(), flags, options.0, timeout.0)
        })
        .into_result()
        .map(|s| s as u32)
    }
}

impl Default for EventFlag {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EventFlag {
    fn drop(&mut self) {
        // SAFETY: Pointer is valid and non-null
        unsafe { sys::furi_event_flag_free(self.as_ptr()) }
    }
}
