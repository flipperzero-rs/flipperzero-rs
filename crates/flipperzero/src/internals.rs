//! Internal implementation details.

#[cfg(feature = "alloc")]
pub(crate) mod alloc {
    use alloc::boxed::Box;
    use core::{mem, ops::{Deref, DerefMut}, ptr::NonNull};

    /// Wrapper for a [`NonNull`] created from [`Box`]
    /// which does not imply uniqueness which the box does.
    ///
    /// # Intended use
    ///
    /// This is intended to be used instead of [`Box`] whenever
    /// an allocation occurs on creation of a wrapper which needs
    /// to store extra information on the heap, such as FFI-callback contexts,
    /// in this case this struct has to be stored as a field
    /// and the raw pointer provided by it should be passed to the FFI.
    ///
    /// The caller must guarantee that by the moment this structure is dropped
    /// no one continues using the pointers.
    ///
    /// # Safety
    ///
    /// While there are no `unsafe` methods in this struct,
    /// it is easy to misuse the pointers provided by its methods, namely:
    ///
    /// * [`NonUniqueBox::as_ptr`]
    /// * [`NonUniqueBox::as_non_null`]
    ///
    /// so it should be used with extra care, i.e. all uses of the pointers
    /// should follow the rules such as stacked borrows
    /// and should never be used after the drop of this structure.
    ///
    /// As a rule of thumb, it should only be stored in private fields
    /// of the `struct`s to help with holding a pointer to an owned allocation
    /// without upholding `Box`s uniqueness guarantees.
    ///
    /// # Examples
    ///
    /// Wrapper structure for some callback:
    /// ```no_run
    /// # struct FfiFoo;
    /// # struct Context {
    /// #     bar: i32,
    /// #     baz: u8,
    /// # }
    /// # extern "C" {
    /// #     fn foo_alloc() -> *mut FfiFoo;
    /// #     fn foo_set_callback(foo: *mut FfiFoo, ctx: Context);
    /// #     fn foo_free(foo: *mut FfiFoo);
    /// # }
    /// # use std::ptr::NonNull;
    /// # use crate::internals::alloc::NonUniqueBox;
    /// pub struct Foo {
    ///     inner: FooInner,
    ///     context: NonUniqueBox<Context>,
    /// }
    /// struct FooInner(NonNull<FfiFoo>);
    /// impl Drop for FooInner {
    ///     fn drop(&mut self) {
    ///         let raw = self.0.as_ptr();
    ///         // SAFETY: `raw` should be a valid pointer
    ///         unsafe { foo_free(raw) };
    ///     }
    /// }
    /// impl Foo {
    ///     fn new() -> Foo {
    ///         let inner = FooInner(
    ///             // SAFETY: we uphold `foo_alloc` invariant
    ///             // and it is never null
    ///             unsafe { NonNull::new_unchecked(foo_alloc()) }
    ///         );
    ///         let context = NonUniqueBox::new(Context { bar: 123, baz: 456 });
    ///         Self { inner, context }
    ///     }
    /// }
    ///```
    #[repr(transparent)]
    pub(crate) struct NonUniqueBox<T: ?Sized>(NonNull<T>);

    impl<T> NonUniqueBox<T> {
        #[inline(always)]
        pub(crate) fn new(value: T) -> Self {
            let value = Box::into_raw(Box::new(value));
            // SAFETY: `value` has just been allocated via `Box`
            Self(unsafe { NonNull::new_unchecked(value) })
        }
    }

    impl<T: ?Sized> Deref for NonUniqueBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { self.0.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for NonUniqueBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { self.0.as_mut() }
        }
    }


    impl<T: ?Sized> NonUniqueBox<T> {
        #[inline(always)]
        pub(crate) fn as_non_null(&self) -> NonNull<T> {
            self.0
        }

        #[inline(always)]
        pub(crate) fn as_ptr(&self) -> *mut T {
            self.0.as_ptr()
        }

        /// Converts this back into a [`Box`].
        ///
        /// # Safety
        ///
        /// This methods is safe since it user's responsibility
        /// to correctly use the pointers created from this wrapper,
        /// but it still is important to keep in mind that this is easy to misuse.
        pub(crate) fn to_box(self) -> Box<T> {
            let raw = self.0.as_ptr();
            mem::forget(self);
            // SAFETY: `raw` should have been created from `Box`
            // and it's user's responsibility to correctly use the exposed pointer
            unsafe { Box::from_raw(raw) }
        }
    }

    impl<T: ?Sized> Drop for NonUniqueBox<T> {
        fn drop(&mut self) {
            let raw = self.0.as_ptr();
            // SAFETY: `raw` should have been created from `Box`
            // and it's user's responsibility to correctly use the exposed pointer
            let _ = unsafe { Box::from_raw(raw) };
        }
    }
}

/// Operations which have unstable implementations
/// but still may be implemented manually on `stable` channel.
///
/// This will use core implementations if `unstable_intrinsics` feature is enabled
/// falling back to ad-hoc implementations otherwise.
#[allow(dead_code)] // this functions may be unused if a specific feature set does not require them
pub(crate) mod ops {
    pub(crate) const fn div_ceil_u16(divident: u16, divisor: u16) -> u16 {
        #[cfg(feature = "unstable_intrinsics")]
        {
            divident.div_ceil(divisor)
        }
        #[cfg(not(feature = "unstable_intrinsics"))]
        {
            let quotient = divident / divisor;
            let remainder = divident % divisor;
            if remainder != 0 {
                quotient + 1
            } else {
                quotient
            }
        }
    }
}

pub(crate) mod macros {
    /// Generates an implementation of `std::error::Error` for the passed type
    /// hidden behind an `std` feature flag.
    macro_rules! impl_std_error {
        ($error_type:ident) => {
            impl ::core::error::Error for $error_type {}
        };
    }

    pub(crate) use impl_std_error;
}
