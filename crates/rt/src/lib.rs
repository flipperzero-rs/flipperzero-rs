//! Rust Runtime for the Flipper Zero.
//!
//! This must be build with `-Z no-unique-section-names` to ensure that this module
//! is linked directly into the `.text` section.

#![cfg_attr(miri, feature(core_intrinsics))]
#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod manifest;
pub mod panic_handler;
mod thread;

/// The C entry point.
///
/// This just delegates to the user's Rust entry point.
///
/// # Safety
///
/// This should never be called manually.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start(args: *mut u8) -> i32 {
    unsafe extern "Rust" {
        fn main(args: *mut u8) -> i32;
    }

    unsafe { main(args) }
}

/// Defines the entry point.
///
/// Must have the following signature: `fn(Option<&CStr>) -> i32`.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        // Force the section to `.text` instead of `.text.main`.
        // lld seems not to automatically rename `.rel.text.main` properly.
        #[cfg(not(miri))]
        #[unsafe(export_name = "main")]
        pub unsafe fn __main(args: *mut u8) -> i32 {
            // type check the entry function
            let f: fn(Option<&::core::ffi::CStr>) -> i32 = $path;

            let args = if args.is_null() {
                None
            } else {
                // SAFETY: Flipper Zero passes arguments to FAPs as a C string.
                let args = unsafe { core::ffi::CStr::from_ptr(args.cast_const().cast()) };
                Some(args)
            };

            let ret = f(args);

            // Clean up app state.
            $crate::__macro_support::__wait_for_thread_completion();

            ret
        }
    };
}

#[doc(hidden)]
pub mod __macro_support {
    /// ⚠️ WARNING: This is *not* a stable API! ⚠️
    ///
    /// This function, and all code contained in the `__macro_support` module, is a
    /// *private* API of `flipperzero-rt`. It is exposed publicly because it is used by
    /// the `flipperzero-rt` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions without
    /// warning.
    pub use crate::thread::wait_for_completion as __wait_for_thread_completion;
}
