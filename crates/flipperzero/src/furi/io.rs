//! Furi I/O API.

use core::ffi::c_char;

use flipperzero_sys as sys;

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.len();
        unsafe {
            if sys::furi_thread_stdout_write(s.as_ptr().cast::<c_char>(), len) != len {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl ufmt::uWrite for Stdout {
    type Error = core::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        let len = s.len();
        unsafe {
            if sys::furi_thread_stdout_write(s.as_ptr().cast::<c_char>(), len) != len {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi_thread_stdout_flush() != 0 {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}
