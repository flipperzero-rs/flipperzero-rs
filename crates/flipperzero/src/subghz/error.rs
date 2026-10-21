use core::{error::Error, fmt::Display};

use ufmt::derive::uDebug;

#[derive(Clone, Copy, Debug, uDebug)]
pub enum SubGhzError {
    UnableToSetFrequency,
    PacketTooLong { len: usize },
    CannotTxOnFrequency,
    UnableToOpenDevice,
}

impl Display for SubGhzError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnableToSetFrequency => write!(f, "Unable to set the frequency"),
            Self::PacketTooLong{len} => write!(f, "The packet length {len} is too long, the device can only be given packets of max length 255"),
            Self::CannotTxOnFrequency => write!(f,"Unable to Tx on the given frequeny (Region Locked?)"),
            Self::UnableToOpenDevice => write!(f,"Was unable to get a handle to the device")
        }
    }
}

impl Error for SubGhzError {}
