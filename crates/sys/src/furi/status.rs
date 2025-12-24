//! Low-level wrappers around Furi Status API.

use core::fmt::Display;

use crate::FuriStatus;

/// The error type for Furi kernel operations.
#[derive(Clone, Copy, Debug, ufmt::derive::uDebug, Eq, PartialEq)]
pub enum Error {
    Unspecified,
    TimedOut,
    ResourceBusy,
    InvalidParameter,
    OutOfMemory,
    ForbiddenInISR,
    Other(i32),
}

impl Error {
    /// Describe the kind of error.
    pub fn description(&self) -> &str {
        match self {
            Self::Unspecified => "Unspecified RTOS error",
            Self::TimedOut => "Operation not completed within the timeout period",
            Self::ResourceBusy => "Resource not available",
            Self::InvalidParameter => "Parameter error",
            Self::OutOfMemory => "System is out of memory",
            Self::ForbiddenInISR => "Not allowed in ISR context",
            _ => "Unknown",
        }
    }
}

/// Create [`Error`] from [`FuriStatus`].
impl TryFrom<FuriStatus> for Error {
    type Error = i32;

    fn try_from(status: crate::FuriStatus) -> core::result::Result<Self, Self::Error> {
        match status {
            crate::FuriStatusError => Ok(Self::Unspecified),
            crate::FuriStatusErrorTimeout => Ok(Self::TimedOut),
            crate::FuriStatusErrorResource => Ok(Self::ResourceBusy),
            crate::FuriStatusErrorParameter => Ok(Self::InvalidParameter),
            crate::FuriStatusErrorNoMemory => Ok(Self::OutOfMemory),
            crate::FuriStatusErrorISR => Ok(Self::ForbiddenInISR),
            crate::FuriStatus(x) => {
                if x < 0 {
                    Ok(Self::Other(x))
                } else {
                    Err(x)
                }
            }
        }
    }
}

/// Create [`FuriStatus`] from [`Error`].
impl From<Error> for FuriStatus {
    fn from(error: Error) -> Self {
        match error {
            Error::Unspecified => crate::FuriStatusError,
            Error::TimedOut => crate::FuriStatusErrorTimeout,
            Error::ResourceBusy => crate::FuriStatusErrorResource,
            Error::InvalidParameter => crate::FuriStatusErrorParameter,
            Error::OutOfMemory => crate::FuriStatusErrorNoMemory,
            Error::ForbiddenInISR => crate::FuriStatusErrorISR,
            Error::Other(x) => crate::FuriStatus(x),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.description(), FuriStatus::from(*self).0)
    }
}

impl ufmt::uDisplay for Error {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{} ({})", self.description(), FuriStatus::from(*self).0)
    }
}

/// Operation status.
/// The Furi API switches between using `enum FuriStatus`, `int32_t` and `uint32_t`.
/// Since these all use the same bit representation, we can just "cast" the returns to this type.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, ufmt::derive::uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[must_use]
pub struct Status(pub i32);

impl Status {
    /// Operation completed successfully.
    pub const OK: Status = Status(0);
    /// Unspecified RTOS error: run-time error but no other error message fits.
    pub const ERR: Status = Status(-1);
    /// Operation not completed within the timeout period.
    pub const ERR_TIMEOUT: Status = Status(-2);
    /// Resource not available.
    pub const ERR_RESOURCE: Status = Status(-3);
    /// Parameter error.
    pub const ERR_PARAMETER: Status = Status(-4);
    /// System is out of memory: it was impossible to allocate or reserve memory for the operation.
    pub const ERR_NO_MEMORY: Status = Status(-5);
    /// Not allowed in ISR context: the function cannot be called from interrupt service routines.
    pub const ERR_ISR: Status = Status(-6);

    /// Describes the status result of the operation.
    pub fn description(self) -> &'static str {
        match self {
            Self::OK => "Operation completed successfully",
            Self::ERR => "Unspecified RTOS error",
            Self::ERR_TIMEOUT => "Operation not completed within the timeout period",
            Self::ERR_RESOURCE => "Resource not available",
            Self::ERR_PARAMETER => "Parameter error",
            Self::ERR_NO_MEMORY => "System is out of memory",
            Self::ERR_ISR => "Not allowed in ISR context",
            _ => "Unknown",
        }
    }

    /// Check if this is an error status.
    pub fn is_err(self) -> bool {
        self != Self::OK
    }

    /// Convert into [`Result`] type.
    pub fn into_result(self) -> Result<i32, Error> {
        match Error::try_from(crate::FuriStatus(self.0)) {
            Err(x) => Ok(x),
            Ok(err) => Err(err),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}: {}", self, self.description())
    }
}

impl ufmt::uDisplay for Status {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{:?}: {}", self, self.description())
    }
}

impl From<crate::FuriStatus> for Status {
    fn from(code: FuriStatus) -> Self {
        Status(code.0)
    }
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        Status(value)
    }
}

impl From<u32> for Status {
    fn from(value: u32) -> Self {
        Status(value as i32)
    }
}

impl From<Status> for Result<i32, Error> {
    fn from(status: Status) -> Self {
        status.into_result()
    }
}
