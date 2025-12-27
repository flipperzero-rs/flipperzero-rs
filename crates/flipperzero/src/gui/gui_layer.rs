use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, GuiLayer as SysGuiLayer};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// The font used to draw text.
///
/// Corresponds to raw [`SysGuiLayer`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GuiLayer {
    /// Desktop layer for internal use. Like fullscreen but with status bar.
    Desktop,
    /// Window layer, status bar is shown.
    Window,
    /// Status bar left-side layer, auto-layout.
    StatusBarLeft,
    /// Status bar right-side layer, auto-layout
    StatusBarRight,
    /// Fullscreen layer, no status bar.
    Fullscreen,
}

impl TryFrom<SysGuiLayer> for GuiLayer {
    type Error = FromSysGuiLayerError;

    fn try_from(value: SysGuiLayer) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::GuiLayerDesktop => Self::Desktop,
            sys::GuiLayerWindow => Self::Window,
            sys::GuiLayerStatusBarLeft => Self::StatusBarLeft,
            sys::GuiLayerStatusBarRight => Self::StatusBarRight,
            sys::GuiLayerFullscreen => Self::Fullscreen,
            sys::GuiLayerMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<GuiLayer> for SysGuiLayer {
    fn from(value: GuiLayer) -> Self {
        match value {
            GuiLayer::Desktop => sys::GuiLayerDesktop,
            GuiLayer::Window => sys::GuiLayerWindow,
            GuiLayer::StatusBarLeft => sys::GuiLayerStatusBarLeft,
            GuiLayer::StatusBarRight => sys::GuiLayerStatusBarRight,
            GuiLayer::Fullscreen => sys::GuiLayerFullscreen,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysGuiLayer`] to [`GuiLayer`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysGuiLayerError {
    /// The [`SysGuiLayer`] is [`MAX`][sys::GuiLayerMAX]
    /// which is a meta-value used to track enum size.
    Max,
    /// The [`SysGuiLayer`] is an invalid value
    /// other than [`MAX`][sys::GuiLayerMAX].
    Invalid(SysGuiLayer),
}

impl Display for FromSysGuiLayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "gui layer ID {} (MAX) is a meta-value",
                sys::GuiLayerMAX.0,
            ),
            Self::Invalid(id) => write!(f, "gui layer ID {} is invalid", id.0),
        }
    }
}

impl uDebug for FromSysGuiLayerError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(f, "Max({})", sys::GuiLayerMAX.0,),
            Self::Invalid(id) => uwrite!(f, "Invalid({})", id.0),
        }
    }
}

impl uDisplay for FromSysGuiLayerError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "gui layer ID {} (MAX) is a meta-value",
                sys::GuiLayerMAX.0,
            ),
            Self::Invalid(id) => uwrite!(f, "gui layer ID {} is invalid", id.0),
        }
    }
}

impl_std_error!(FromSysGuiLayerError);
