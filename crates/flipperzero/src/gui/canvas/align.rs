use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, Align as SysAlign};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// Alignment of an object on the canvas.
///
/// Corresponds to raw [`SysAlign`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Align {
    /// The values are aligned relative to the right.
    Left,
    /// The values are aligned relative to the left.
    Right,
    /// The values are aligned relative to the top.
    Top,
    /// The values are aligned relative to the bottom.
    Bottom,
    /// The values are aligned relative to the center.
    Center,
}

impl TryFrom<SysAlign> for Align {
    type Error = FromSysAlignError;

    fn try_from(value: SysAlign) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::AlignLeft => Self::Left,
            sys::AlignRight => Self::Right,
            sys::AlignTop => Self::Top,
            sys::AlignBottom => Self::Bottom,
            sys::AlignCenter => Self::Center,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<Align> for SysAlign {
    fn from(value: Align) -> Self {
        match value {
            Align::Left => sys::AlignLeft,
            Align::Right => sys::AlignRight,
            Align::Top => sys::AlignTop,
            Align::Bottom => sys::AlignBottom,
            Align::Center => sys::AlignCenter,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysAlign`] to [`Align`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysAlignError {
    /// The [`SysAlign`] is an invalid value.
    Invalid(SysAlign),
}

impl Display for FromSysAlignError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::Invalid(id) = self;
        write!(f, "align ID {} is invalid", id.0)
    }
}

impl uDebug for FromSysAlignError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "Invalid({})", id.0)
    }
}

impl uDisplay for FromSysAlignError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "align ID {} is invalid", id.0)
    }
}

impl_std_error!(FromSysAlignError);
