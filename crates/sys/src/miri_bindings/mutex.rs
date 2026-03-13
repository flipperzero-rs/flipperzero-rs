extern crate alloc;

use crate::miri_bindings::utils::*;
use crate::{FuriStatus, FuriStatusError, FuriStatusErrorTimeout, FuriStatusOk};
use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct FuriMutex(AtomicBool);

pub const FuriMutexTypeNormal: FuriMutexType = FuriMutexType(0);
pub const FuriMutexTypeRecursive: FuriMutexType = FuriMutexType(1);

pub struct FuriMutexType(u8);

#[doc = "Allocate FuriMutex\n\n # Arguments\n\n* `type` (direction in) - The mutex type\n\n # Returns\n\npointer to FuriMutex instance"]
pub unsafe fn furi_mutex_alloc(_type: FuriMutexType) -> *mut FuriMutex {
    Box::into_raw(Box::new(FuriMutex(AtomicBool::new(false))))
}

#[doc = "Free FuriMutex\n\n # Arguments\n\n* `instance` - The pointer to FuriMutex instance"]
pub unsafe fn furi_mutex_free(instance: *mut FuriMutex) {
    drop(unsafe { Box::from_raw(instance) })
}

#[doc = "Acquire mutex\n\n # Arguments\n\n* `instance` - The pointer to FuriMutex instance\n * `timeout` (direction in) - The timeout\n\n # Returns\n\nThe furi status."]
pub unsafe fn furi_mutex_acquire(instance: *mut FuriMutex, timeout: u32) -> FuriStatus {
    let mutex = &(unsafe { &*instance }).0;
    while mutex
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        // Because of how the furi_mutex is used in the RawMutex, we know that the timeout will
        // only ever be 0 (which should fail if we didn't get the lock) or u32::MAX (when we should
        // never fail out of this)
        if timeout == 0 {
            return FuriStatusErrorTimeout;
        }
        miri_spin_loop();
    }

    FuriStatusOk
}

#[doc = "Release mutex\n\n # Arguments\n\n* `instance` - The pointer to FuriMutex instance\n\n # Returns\n\nThe furi status."]
pub unsafe fn furi_mutex_release(instance: *mut FuriMutex) -> FuriStatus {
    let mutex = &(unsafe { &*instance }).0;
    match mutex.compare_exchange(true, false, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => FuriStatusOk,
        Err(_) => FuriStatusError,
    }
}
