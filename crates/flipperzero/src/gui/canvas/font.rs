use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, Font as SysFont};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// The font used to draw text.
///
/// Corresponds to raw [`SysFont`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Font {
    /// The primary font.
    Primary,
    /// The secondary font.
    Secondary,
    /// The keyboard font.
    Keyboard,
    /// The font with big numbers.
    BigNumbers,
}

impl Font {
    /// Gets the total number of available fonts.
    ///
    /// # Example
    ///
    /// ```
    /// # use flipperzero::gui::canvas::Font;
    /// assert_eq!(Font::total_number(), 4);
    /// ```
    pub const fn total_number() -> usize {
        sys::FontTotalNumber.0 as usize
    }
}

impl TryFrom<SysFont> for Font {
    type Error = FromSysFontError;

    fn try_from(value: SysFont) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::FontPrimary => Self::Primary,
            sys::FontSecondary => Self::Secondary,
            sys::FontKeyboard => Self::Keyboard,
            sys::FontBigNumbers => Self::BigNumbers,
            sys::FontTotalNumber => Err(Self::Error::TotalNumber)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Font> for SysFont {
    fn from(value: Font) -> Self {
        match value {
            Font::Primary => sys::FontPrimary,
            Font::Secondary => sys::FontSecondary,
            Font::Keyboard => sys::FontKeyboard,
            Font::BigNumbers => sys::FontBigNumbers,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysFont`] to [`Font`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysFontError {
    /// The [`SysFont`] is [`TotalNumber`][sys::FontTotalNumber]
    /// which is a meta-value used to track enum size.
    TotalNumber,
    /// The [`SysFont`] is an invalid value
    /// other than [`TotalNumber`][sys::FontTotalNumber].
    Invalid(SysFont),
}

impl Display for FromSysFontError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TotalNumber => write!(
                f,
                "font ID {} (TotalNumber) is a meta-value",
                sys::FontTotalNumber.0,
            ),
            Self::Invalid(id) => write!(f, "font ID {} is invalid", id.0),
        }
    }
}

impl uDebug for FromSysFontError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::TotalNumber => uwrite!(f, "TotalNumber"),
            Self::Invalid(id) => uwrite!(f, "Invalid({})", id.0),
        }
    }
}

impl uDisplay for FromSysFontError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::TotalNumber => uwrite!(
                f,
                "font ID {} (TotalNumber) is a meta-value",
                sys::FontTotalNumber.0,
            ),
            Self::Invalid(id) => uwrite!(f, "font ID {} is invalid", id.0),
        }
    }
}

impl_std_error!(FromSysFontError);
