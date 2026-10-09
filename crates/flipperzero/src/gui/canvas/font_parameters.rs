use core::{
    fmt::{self, Display, Formatter},
    num::NonZeroU8,
};

use flipperzero_sys::CanvasFontParameters as SysCanvasFontParameters;
use ufmt::{derive::uDebug, uDisplay, uWrite};

use crate::internals::macros::impl_std_error;

/// Font parameters on a canvas.
///
/// Corresponds to raw [`SysCanvasFontParameters`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash)]
pub struct CanvasFontParameters {
    pub leading_default: NonZeroU8,
    pub leading_min: NonZeroU8,
    pub height: NonZeroU8,
    pub descender: u8,
}

impl TryFrom<SysCanvasFontParameters> for CanvasFontParameters {
    type Error = FromSysCanvasFontParameters;

    fn try_from(value: SysCanvasFontParameters) -> Result<Self, Self::Error> {
        Ok(Self {
            leading_default: value
                .leading_default
                .try_into()
                .or(Err(Self::Error::ZeroLeadingDefault))?,
            leading_min: value
                .leading_min
                .try_into()
                .or(Err(Self::Error::ZeroLeadingMin))?,
            height: value.height.try_into().or(Err(Self::Error::ZeroHeight))?,
            descender: value.descender,
        })
    }
}

impl From<CanvasFontParameters> for SysCanvasFontParameters {
    fn from(value: CanvasFontParameters) -> Self {
        Self {
            leading_default: value.leading_default.into(),
            leading_min: value.leading_min.into(),
            height: value.height.into(),
            descender: value.descender,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysCanvasFontParameters`] to [`CanvasFontParameters`].
///
/// All of these correspond to errors in individual parameters.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FromSysCanvasFontParameters {
    /// [`SysCanvasFontParameters::leading_default`] field is set to `0`.
    ZeroLeadingDefault,
    /// [`SysCanvasFontParameters::leading_min`] field is set to `0`.
    ZeroLeadingMin,
    /// [`SysCanvasFontParameters::height`] field is set to `0`.
    ZeroHeight,
}

impl Display for FromSysCanvasFontParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FromSysCanvasFontParameters::ZeroLeadingDefault => "leading_default is zero",
            FromSysCanvasFontParameters::ZeroLeadingMin => "leading_min is zero",
            FromSysCanvasFontParameters::ZeroHeight => "height is zero",
        })
    }
}

impl uDisplay for FromSysCanvasFontParameters {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_str(match self {
            FromSysCanvasFontParameters::ZeroLeadingDefault => "leading_default is zero",
            FromSysCanvasFontParameters::ZeroLeadingMin => "leading_min is zero",
            FromSysCanvasFontParameters::ZeroHeight => "height is zero",
        })
    }
}

impl_std_error!(FromSysCanvasFontParameters);
