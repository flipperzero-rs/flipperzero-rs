//! Alloc support for the Flipper Zero.
//! *Note:* This currently requires using nightly.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

use core::alloc::{GlobalAlloc, Layout};

#[cfg(not(miri))]
use flipperzero_sys as sys;

/// Global allocator for Flipper Zero.
///
/// Memory is allocated using the firmware's `aligned_malloc` and so may only be
/// manually freed with `aligned_free`. It is not safe to use regular `realloc` or `free`
/// on memory obtained from this allocator.
pub struct FuriAlloc;

#[cfg(not(miri))]
unsafe impl GlobalAlloc for FuriAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { sys::aligned_malloc(layout.size(), layout.align()).cast() }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { sys::aligned_free(ptr.cast()) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // Firmware guarantees that all heap allocations are zeroed
        // https://github.com/flipperdevices/flipperzero-firmware/issues/1747#issuecomment-1253636552
        unsafe { self.alloc(layout) }
    }
}

#[cfg(miri)]
unsafe extern "Rust" {
    fn miri_alloc(size: usize, align: usize) -> *mut u8;
    fn miri_dealloc(ptr: *mut u8, size: usize, align: usize);
}

#[cfg(miri)]
unsafe impl GlobalAlloc for FuriAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { miri_alloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { miri_dealloc(ptr, layout.size(), layout.align()) };
    }
}

#[global_allocator]
static ALLOCATOR: FuriAlloc = FuriAlloc;
