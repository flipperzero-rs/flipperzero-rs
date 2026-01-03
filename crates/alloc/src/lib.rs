//! Alloc support for the Flipper Zero.
//! *Note:* This currently requires using nightly.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

use core::alloc::{GlobalAlloc, Layout};

use flipperzero_sys as sys;

pub struct FuriAlloc;

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
        // https://github.com/flipperdevices/flipperzero-firmware/issues/1747#issuecomment-1253636552
        unsafe { self.alloc(layout) }
    }
}

#[global_allocator]
static ALLOCATOR: FuriAlloc = FuriAlloc;
