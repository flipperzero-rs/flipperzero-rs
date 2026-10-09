use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, CanvasDirection as SysCanvasDirection};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// Direction of an element on the canvas.
///
/// Corresponds to raw [`SysCanvasDirection`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CanvasDirection {
    /// The direction is from left to right.
    LeftToRight,
    /// The direction is from top to bottom.
    TopToBottom,
    /// The direction is from right to left.
    RightToLeft,
    /// The direction is from bottom to top.
    BottomToTop,
}

impl TryFrom<SysCanvasDirection> for CanvasDirection {
    type Error = FromSysCanvasDirectionError;

    fn try_from(value: SysCanvasDirection) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::CanvasDirectionLeftToRight => Self::LeftToRight,
            sys::CanvasDirectionTopToBottom => Self::TopToBottom,
            sys::CanvasDirectionRightToLeft => Self::RightToLeft,
            sys::CanvasDirectionBottomToTop => Self::BottomToTop,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<CanvasDirection> for SysCanvasDirection {
    fn from(value: CanvasDirection) -> Self {
        match value {
            CanvasDirection::BottomToTop => sys::CanvasDirectionBottomToTop,
            CanvasDirection::LeftToRight => sys::CanvasDirectionLeftToRight,
            CanvasDirection::RightToLeft => sys::CanvasDirectionRightToLeft,
            CanvasDirection::TopToBottom => sys::CanvasDirectionTopToBottom,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysCanvasDirection`] to [`CanvasDirection`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysCanvasDirectionError {
    /// The [`SysCanvasDirection`] is an invalid value.
    Invalid(SysCanvasDirection),
}

impl Display for FromSysCanvasDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::Invalid(id) = self;
        write!(f, "canvas direction ID {} is invalid", id.0)
    }
}

impl uDebug for FromSysCanvasDirectionError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "Invalid({})", id.0)
    }
}

impl uDisplay for FromSysCanvasDirectionError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self::Invalid(id) = self;
        uwrite!(f, "canvas direction ID {} is invalid", id.0)
    }
}

impl_std_error!(FromSysCanvasDirectionError);
