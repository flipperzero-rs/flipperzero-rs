//! Low-level bindings for the Flipper Zero.

#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

// Features that identify thumbv7em-none-eabihf.
// NOTE: `arm_target_feature` is currently unstable (see rust-lang/rust#44839)
#[cfg(not(any(
    all(
        target_arch = "arm",
        //target_feature = "thumb2",
        //target_feature = "v7",
        //target_feature = "dsp",
        target_os = "none",
        target_abi = "eabihf",
    ),
    miri
)))]
core::compile_error!("This crate requires `--target thumbv7em-none-eabihf`");

pub mod furi;
mod inlines;

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::transmute_int_to_bool,
    clippy::useless_transmute,
    rustdoc::broken_intra_doc_links,
// https://github.com/rust-lang/rust-bindgen/issues/2807
    unnecessary_transmutes,
// https://github.com/rust-lang/rust-bindgen/issues/3053
    clippy::ptr_offset_with_cast,
)]
#[rustfmt::skip]
mod bindings;

/// Crash the system.
///
/// May be provided with an optional message which should not contain NULs.
/// The following will not compile:
///
/// ```compile_fail
/// flipperzero_sys::crash!("Has a \0 NUL");
/// ```
///
/// # Examples
///
/// Crash the system with a *"Hello world!"* message:
///
/// ```
/// flipperzero_sys::crash!("Hello world!");
/// ```
///
/// Crash the system with default *"Fatal Error"* message:
///
/// ```
/// flipperzero_sys::crash!();
/// ```
#[macro_export]
macro_rules! crash {
    () => {
        $crate::__crash_implementation!(::core::ptr::null());
    };
    ($msg:expr $(,)?) => {{
        let message = const {
            match ::core::ffi::CStr::from_bytes_with_nul(::core::concat!($msg, "\0").as_bytes()) {
                Err(_) => c"nul in crash message",
                Ok(m) => m,
            }
        };

        $crate::__crash_implementation!(message.as_ptr());
    }};
}

/// Crash the system.
///
/// This is an internal implementation detail.
#[doc(hidden)]
#[macro_export]
macro_rules! __crash_implementation {
    ($ptr:expr) => {
        unsafe {
            // Crash message is passed via r12
            ::core::arch::asm!(
                "ldr pc,=__furi_crash_implementation",
                in("r12") ($ptr),
                options(nostack),
            );

            ::core::hint::unreachable_unchecked();
        }
    };
}

/// Halt the system.
///
/// May be provided with an optional message which should not contain NULs.
/// The following will not compile:
///
/// ```compile_fail
/// flipperzero_sys::halt!("Has a \0 NUL");
/// ```
///
/// # Examples
///
/// Halt the system with a *"Hello world!"* message:
///
/// ```
/// flipperzero_sys::crash!("Hello world!");
/// ```
///
/// Halt the system with default *"System halt requested."* message:
///
/// ```
/// flipperzero_sys::crash!();
/// ```
#[macro_export]
macro_rules! halt {
    () => {
        $crate::__halt_implementation!(::core::ptr::null());
    };
    ($msg:expr $(,)?) => {{
        // Optional message
        let message = const {
            match ::core::ffi::CStr::from_bytes_with_nul(::core::concat!($msg, "\0").as_bytes()) {
                Err(_) => c"nul in halt message",
                Ok(m) => m,
            }
        };

        $crate::__halt_implementation!(message.as_ptr());
    }};
}

/// Halt the system.
///
/// This is an internal implementation detail.
#[doc(hidden)]
#[macro_export]
macro_rules! __halt_implementation {
    ($ptr:expr) => {
        unsafe {
            // Halt message is passed via r12
            ::core::arch::asm!(
                "ldr pc,=__furi_halt_implementation",
                in("r12") ($ptr),
                options(nomem, nostack))
            ;

            ::core::hint::unreachable_unchecked();
        }
    };
}

/// Check if flag is set.
///
/// Typically implemented as `(self & flag) == flag`.
pub trait HasFlag {
    fn has_flag(self, flag: Self) -> bool;
}

/// Implement bitfield operations for "bitfield" enums.
#[doc(hidden)]
macro_rules! impl_bitfield_enum {
    ($t:ty) => {
        impl ::core::default::Default for $t {
            #[inline]
            fn default() -> Self {
                Self(0)
            }
        }
        impl ::core::ops::BitOr<$t> for $t {
            type Output = Self;

            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl ::core::ops::BitOrAssign for $t {
            #[inline]
            fn bitor_assign(&mut self, rhs: $t) {
                self.0 |= rhs.0;
            }
        }
        impl ::core::ops::BitAnd<$t> for $t {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl ::core::ops::BitAndAssign for $t {
            #[inline]
            fn bitand_assign(&mut self, rhs: $t) {
                self.0 &= rhs.0;
            }
        }
        impl ::core::ops::Not for $t {
            type Output = Self;
            #[inline]
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        impl HasFlag for $t {
            #[inline]
            fn has_flag(self, flag: Self) -> bool {
                (self.0 & flag.0) == flag.0
            }
        }
    };
}

impl_bitfield_enum!(CliCommandFlag);
impl_bitfield_enum!(FS_AccessMode);
impl_bitfield_enum!(FS_Flags);
impl_bitfield_enum!(FS_OpenMode);
impl_bitfield_enum!(FuriFlag);
impl_bitfield_enum!(FuriHalNfcEvent);
impl_bitfield_enum!(FuriHalRtcFlag);
impl_bitfield_enum!(FuriHalSerialRxEvent);
impl_bitfield_enum!(iButtonProtocolFeature);
impl_bitfield_enum!(Light);
impl_bitfield_enum!(MfUltralightFeatureSupport);
impl_bitfield_enum!(SubGhzProtocolFlag);

// Re-export bindings
pub use bindings::*;

// Definition of inline functions
pub use inlines::furi_hal_gpio::*;
