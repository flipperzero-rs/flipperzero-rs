use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, Color as SysColor};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// Color on the canvas.
///
/// Corresponds to raw [`SysColor`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Color {
    /// White color is used.
    White,
    /// Black color is used.
    Black,
    /// The color is inverted.
    Xor,
}

impl TryFrom<SysColor> for Color {
    type Error = FromSysColorError;

    fn try_from(value: SysColor) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::ColorWhite => Self::White,
            sys::ColorBlack => Self::Black,
            sys::ColorXOR => Self::Xor,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Color> for SysColor {
    fn from(value: Color) -> Self {
        match value {
            Color::White => sys::ColorWhite,
            Color::Black => sys::ColorBlack,
            Color::Xor => sys::ColorXOR,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysColor`] to [`Color`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysColorError {
    /// The [`SysColor`] is an invalid value.
    Invalid(SysColor),
}

impl Display for FromSysColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::Invalid(id) = self;
        write!(f, "color ID {} is invalid", id.0)
    }
}

impl uDebug for FromSysColorError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "Invalid({})", id.0)
    }
}

impl uDisplay for FromSysColorError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "color ID {} is invalid", id.0)
    }
}

impl_std_error!(FromSysColorError);
