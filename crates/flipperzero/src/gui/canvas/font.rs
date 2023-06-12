use crate::internals::macros::impl_std_error;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, Font as SysFont};
use ufmt::{derive::uDebug, uDisplay, uWrite, uwrite};

#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
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
        sys::Font_FontTotalNumber as usize
    }
}

impl TryFrom<SysFont> for Font {
    type Error = FromSysFontError;

    fn try_from(value: SysFont) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::Font_FontPrimary => Self::Primary,
            sys::Font_FontSecondary => Self::Secondary,
            sys::Font_FontKeyboard => Self::Keyboard,
            sys::Font_FontBigNumbers => Self::BigNumbers,
            sys::Font_FontTotalNumber => Err(Self::Error::TotalNumber)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Font> for SysFont {
    fn from(value: Font) -> Self {
        match value {
            Font::Primary => sys::Font_FontPrimary,
            Font::Secondary => sys::Font_FontSecondary,
            Font::Keyboard => sys::Font_FontKeyboard,
            Font::BigNumbers => sys::Font_FontBigNumbers,
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysFontError {
    TotalNumber,
    Invalid(SysFont),
}

impl Display for FromSysFontError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TotalNumber => write!(
                f,
                "font ID {} (TotalNumber) is a meta-value",
                sys::Font_FontTotalNumber,
            ),
            Self::Invalid(id) => write!(f, "font ID {id} is invalid"),
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
                sys::Font_FontTotalNumber,
            ),
            Self::Invalid(id) => uwrite!(f, "font ID {} is invalid", id),
        }
    }
}

impl_std_error!(FromSysFontError);