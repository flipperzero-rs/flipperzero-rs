//! Internal implementation details.

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
