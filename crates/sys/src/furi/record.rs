//! Low-level wrappers around Furi Record API.

use core::ffi::CStr;
use core::ptr::NonNull;

/// Low-level wrapper of a record handle.
///
/// This effectively acts as a reference count for the open underlying Record.
pub struct UnsafeRecord<T> {
    name: &'static CStr,
    raw: NonNull<T>,
}

impl<T> UnsafeRecord<T> {
    /// Opens a record.
    ///
    /// # Safety
    ///
    /// `T` must be the correct C type for the record identified by `name`.
    pub unsafe fn open(name: &'static CStr) -> Self {
        Self {
            name,
            // SAFETY: `furi_record_open` blocks until the record is initialized with a valid value.
            raw: unsafe { NonNull::new_unchecked(crate::furi_record_open(name.as_ptr()).cast()) },
        }
    }

    /// Returns the record data as a raw pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.raw.as_ptr()
    }
}

impl<T> Clone for UnsafeRecord<T> {
    fn clone(&self) -> Self {
        // SAFETY: Opening a record multiple times just increases its reference count.
        unsafe { Self::open(self.name) }
    }
}

impl<T> Drop for UnsafeRecord<T> {
    fn drop(&mut self) {
        unsafe {
            // decrement the holders count
            crate::furi_record_close(self.name.as_ptr());
        }
    }
}

pub mod miri {
    #[macro_export]
    macro_rules! miri_assert_record_count {
        ($item:expr, $count:literal, $($arg:tt)+) => {
            #[cfg(miri)]
            assert_eq!(
                {
                    extern crate alloc;
                    use alloc::sync::Arc;

                    // SAFETY: The value held by a record struct (held by the UnsafeRecord provided
                    // in "item") is defined under MIRI to be an Arc that holds the actual data.
                    //
                    // The Arc is required as records always have associated background threads,
                    // and so in order to share information between the consumer and that
                    // background thread (and to ensure that the background thread has been created
                    // and is running) under MIRI, we require that the object is held in an Arc and
                    // protected by a mutex.
                    let inner: Arc<_> = Arc::from_raw($item.as_ptr());
                    let count = Arc::strong_count(&inner);
                    // Intentionally leak again?
                    let _inner = Arc::into_raw(inner);

                    count
                },
                $count,
                $($arg)+
                //core::format_args!($arg)
            );
        };
    }
}
