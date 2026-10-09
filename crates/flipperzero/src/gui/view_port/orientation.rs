use core::fmt::{self, Display, Formatter};

use flipperzero_sys::{self as sys, ViewPortOrientation as SysViewPortOrientation};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

use crate::internals::macros::impl_std_error;

/// Orientation of a view port.
///
/// Corresponds to raw [`SysViewPortOrientation`].
///
/// # Examples
///
/// Basic
///
/// ```
/// # use flipperzero::gui::view_port::ViewPort;
/// # use flipperzero::log;
/// let view_port = ViewPort::new(());
/// let orientation = view_port.get_orientation();
/// if matches!(orientation, ViewPortOrientation::Horizontal) {
///     log!("Currently in horizontal orientation")
/// }
/// ```
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ViewPortOrientation {
    /// Horizontal orientation.
    Horizontal,
    /// Flipped horizontal orientation.
    HorizontalFlip,
    /// Vertical orientation.
    Vertical,
    /// Flipped vertical orientation.
    VerticalFlip,
}

impl ViewPortOrientation {
    /// Checks that this orientation is horizontal.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::view_port::ViewPortOrientation;
    /// assert!(ViewPortOrientation::Horizontal.is_horizontal());
    /// assert!(ViewPortOrientation::HorizontalFlip.is_horizontal());
    /// assert!(!ViewPortOrientation::Vertical.is_horizontal());
    /// assert!(!ViewPortOrientation::VerticalFlip.is_horizontal());
    /// ```
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::HorizontalFlip)
    }

    /// Checks that this orientation is vertical.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::view_port::ViewPortOrientation;
    /// assert!(ViewPortOrientation::Vertical.is_vertical());
    /// assert!(ViewPortOrientation::VerticalFlip.is_vertical());
    /// assert!(!ViewPortOrientation::Horizontal.is_vertical());
    /// assert!(!ViewPortOrientation::HorizontalFlip.is_vertical());
    /// ```
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::VerticalFlip)
    }

    /// Checks that this orientation is flipped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::view_port::ViewPortOrientation;
    /// assert!(ViewPortOrientation::HorizontalFlip.is_flipped());
    /// assert!(ViewPortOrientation::VerticalFlip.is_flipped());
    /// assert!(!ViewPortOrientation::Horizontal.is_flipped());
    /// assert!(!ViewPortOrientation::Vertical.is_flipped());
    /// ```
    pub const fn is_flipped(self) -> bool {
        matches!(self, Self::HorizontalFlip | Self::VerticalFlip)
    }
}

impl TryFrom<SysViewPortOrientation> for ViewPortOrientation {
    type Error = FromSysViewPortOrientationError;

    fn try_from(value: SysViewPortOrientation) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::ViewPortOrientationHorizontal => Self::Horizontal,
            sys::ViewPortOrientationHorizontalFlip => Self::HorizontalFlip,
            sys::ViewPortOrientationVertical => Self::Vertical,
            sys::ViewPortOrientationVerticalFlip => Self::VerticalFlip,
            sys::ViewPortOrientationMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<ViewPortOrientation> for SysViewPortOrientation {
    fn from(value: ViewPortOrientation) -> Self {
        match value {
            ViewPortOrientation::Horizontal => sys::ViewPortOrientationHorizontal,
            ViewPortOrientation::HorizontalFlip => sys::ViewPortOrientationHorizontalFlip,
            ViewPortOrientation::Vertical => sys::ViewPortOrientationVertical,
            ViewPortOrientation::VerticalFlip => sys::ViewPortOrientationVerticalFlip,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysViewPortOrientation`] to [`ViewPortOrientation`].
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysViewPortOrientationError {
    /// The [`SysViewPortOrientation`]
    /// is [`MAX`][sys::ViewPortOrientationMAX]
    /// which is a meta-value used to track enum size.
    Max,
    /// The [`SysViewPortOrientation`] is an invalid value
    /// other than [`MAX`][sys::ViewPortOrientationMAX].
    Invalid(SysViewPortOrientation),
}

impl Display for FromSysViewPortOrientationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "view port orientation ID {} (MAX) is a meta-value",
                sys::ViewPortOrientationMAX.0,
            ),
            Self::Invalid(id) => write!(f, "view port orientation ID {} is invalid", id.0),
        }
    }
}

impl uDebug for FromSysViewPortOrientationError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(f, "Max"),
            Self::Invalid(id) => uwrite!(f, "Invalid({})", id.0),
        }
    }
}

impl uDisplay for FromSysViewPortOrientationError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "view port orientation ID {} (MAX) is a meta-value",
                sys::ViewPortOrientationMAX.0,
            ),
            Self::Invalid(id) => uwrite!(f, "view port orientation ID {} is invalid", id.0),
        }
    }
}

impl_std_error!(FromSysViewPortOrientationError);
