//! Low-level wrappers around Furi memory allocation API.

use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

/// Heap-allocated value.
///
/// This is intended for situations where it is not possible to rely upon a global allocator.
/// Most users should make use of `flipperzero-alloc`.
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct FuriBox<T: ?Sized>(NonNull<T>);

impl<T> FuriBox<T> {
    /// Allocates and initializes a correctly aligned value on the system heap.
    pub fn new(value: T) -> Self {
        let ptr = unsafe { crate::aligned_malloc(size_of::<T>(), align_of::<T>()) as *mut T };
        assert!(!ptr.is_null());
        assert!(ptr.is_aligned());

        unsafe {
            ptr.write(value);

            // SAFETY: Pointer is non-null, aligned and represents a valid `T`
            FuriBox::from_raw(ptr)
        }
    }

    #[cfg(miri)]
    /// Allocates and initializes a correctly aligned value on the system heap.
    pub fn new(value: T) -> Self {
        unsafe {
            // SAFETY: Pointer is non-null, aligned and represents a valid `T`
            FuriBox::from_raw(value)
        }
    }

    /// Consume the box and return raw pointer.
    ///
    /// Caller is responsible for calling `T::drop()` and freeing the pointer with `aligned_free`.
    pub fn into_raw(b: FuriBox<T>) -> *mut T {
        b.0.as_ptr()
    }

    /// Constructs a box from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory problems.
    ///
    /// The caller is responsible for ensuring the pointer is non-null, was allocated using `aligned_malloc`
    /// with the correct alignment for `T` and that the memory represents a valid `T`.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        FuriBox(unsafe { NonNull::new_unchecked(raw) })
    }

    /// Returns a raw pointer to the Box’s contents.
    ///
    /// The caller must ensure that the Box outlives the pointer this function returns,
    /// or else it will end up dangling.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer
    /// or any pointer derived from it. If you need to mutate the contents of the Box, use `as_mut_ptr``.
    ///
    /// This method guarantees that for the purpose of the aliasing model,
    /// this method does not materialize a reference to the underlying memory,
    /// and thus the returned pointer will remain valid when mixed with
    /// other calls to as_ptr and as_mut_ptr.
    pub fn as_ptr(b: &FuriBox<T>) -> *const T {
        b.0.as_ptr().cast_const()
    }

    /// Returns a raw mutable pointer to the Box’s contents.
    ///
    /// The caller must ensure that the Box outlives the pointer this function returns,
    /// or else it will end up dangling.
    ///
    /// This method guarantees that for the purpose of the aliasing model,
    /// this method does not materialize a reference to the underlying memory,
    /// and thus the returned pointer will remain valid when mixed with
    /// other calls to `as_ptr`` and `as_mut_ptr``.
    pub fn as_mut_ptr(b: &mut FuriBox<T>) -> *mut T {
        b.0.as_ptr()
    }
}

impl<T: ?Sized> Drop for FuriBox<T> {
    fn drop(&mut self) {
        // SAFETY: Pointer is valid, correctly-aligned, non-null (enforced by `new`)
        // and won't be accessed after `drop_in_place` is called.
        unsafe {
            self.0.drop_in_place();
            crate::aligned_free(self.0.as_ptr().cast());
        }
    }
}

impl<T: ?Sized> AsRef<T> for FuriBox<T> {
    fn as_ref(&self) -> &T {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> AsMut<T> for FuriBox<T> {
    fn as_mut(&mut self) -> &mut T {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> Deref for FuriBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for FuriBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Pointer is non-null, aligned and represents a valid `T`
        unsafe { self.0.as_mut() }
    }
}
