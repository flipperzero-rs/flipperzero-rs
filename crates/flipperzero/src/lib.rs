//! High-level bindings for the Flipper Zero.
//!
//! # Features
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![no_std]
#![cfg_attr(all(test, not(miri)), no_main)]
#![cfg_attr(all(test, miri), feature(start))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(any(feature = "alloc", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
extern crate alloc;

pub mod dialogs;
pub mod dolphin;
pub mod furi;
pub mod gpio;
pub mod gui;
pub mod io;
pub mod macros;
pub mod notification;
pub mod storage;
pub mod subghz;
pub mod toolbox;

#[doc(hidden)]
pub mod __macro_support {
    use crate::furi::log::Level;

    // Re-export for use in macros
    pub use ufmt;

    pub use crate::furi::string::FuriString;

    /// ⚠️ WARNING: This is *not* a stable API! ⚠️
    ///
    /// This module, and all code contained in the `__macro_support` module, is a
    /// *private* API of `flipperzero`. It is exposed publicly because it is used by the
    /// `flipperzero` macros, but it is not part of the stable versioned API. Breaking
    /// changes to this module may occur in small-numbered versions without warning.
    pub use flipperzero_sys as __sys;

    /// ⚠️ WARNING: This is *not* a stable API! ⚠️
    ///
    /// This function, and all code contained in the `__macro_support` module, is a
    /// *private* API of `flipperzero`. It is exposed publicly because it is used by the
    /// `flipperzero` macros, but it is not part of the stable versioned API. Breaking
    /// changes to this module may occur in small-numbered versions without warning.
    pub fn __level_to_furi(level: Level) -> __sys::FuriLogLevel {
        level.to_furi()
    }
}

flipperzero_test::tests_runner!(
    name = "flipperzero-rs Unit Tests",
    stack_size = 4096,
    [
        crate::furi::log::metadata::tests,
        crate::furi::message_queue::tests,
        crate::furi::rng::tests,
        crate::furi::string::tests,
        crate::furi::sync::tests,
        crate::furi::time::tests,
        crate::gpio::i2c::tests,
        crate::toolbox::crc32::tests,
        // crate::toolbox::md5::tests,
        // crate::toolbox::sha256::tests,
    ]
);
