mod key;
mod r#type;

use flipperzero_sys::{self as sys, InputEvent as SysInputEvent};
use ufmt::derive::uDebug;
// public type alias for an anonymous union
pub use sys::InputEvent__bindgen_ty_1 as SysInputEventSequence;

pub use key::*;
pub use r#type::*;

/// Input event occurring on user actions.
///
/// Corresponds to raw [`SysInputEvent`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InputEvent {
    /// Sequence qualifying the event.
    pub sequence: InputEventSequence,
    /// Physical key causing the event.
    pub key: InputKey,
    /// The type of the event.
    pub r#type: InputType,
}

impl TryFrom<SysInputEvent> for InputEvent {
    type Error = FromSysInputEventError;

    fn try_from(value: SysInputEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            sequence: value.__bindgen_anon_1.into(),
            key: value.key.try_into()?,
            r#type: value.type_.try_into()?,
        })
    }
}

impl From<InputEvent> for SysInputEvent {
    fn from(value: InputEvent) -> Self {
        Self {
            __bindgen_anon_1: value.sequence.into(),
            key: value.key.into(),
            type_: value.r#type.into(),
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysInputEvent`] to [`InputEvent`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysInputEventError {
    InvalidKey(FromSysInputKeyError),
    InvalidType(FromSysInputTypeError),
}

impl From<FromSysInputKeyError> for FromSysInputEventError {
    fn from(value: FromSysInputKeyError) -> Self {
        Self::InvalidKey(value)
    }
}

impl From<FromSysInputTypeError> for FromSysInputEventError {
    fn from(value: FromSysInputTypeError) -> Self {
        Self::InvalidType(value)
    }
}

/// [`InputEvent`] sequence.
///
/// This is a transparent view over [`u32`](prim@u32) with the following representation:
///
/// | Bits    | 31..30 | 29..0  |
/// |---------|--------|--------|
/// | Payload | Source | Counter|
///
/// Corresponds to raw [`SysInputEventSequence`].
///
/// # Example usage
///
/// Decoding a raw `u32` value:
///
/// ```
/// use flipperzero::input::InputEventSequence;
/// let sequence = InputEventSequence::from(0b10__000000_10101010_11110000_11111111u32);
/// assert_eq!(0b10, sequence.source());
/// assert_eq!(0b10101010_11110000_11111111, sequence.counter());
/// ```
#[repr(transparent)]
#[derive(Copy, Clone, Debug, uDebug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct InputEventSequence(u32);

impl InputEventSequence {
    const SOURCE_SHIFT: u32 = 30;
    const SOURCE_MASK: u32 = (u32::MAX) >> Self::SOURCE_SHIFT;
    const COUNTER_MASK: u32 = !(Self::SOURCE_MASK << Self::SOURCE_SHIFT);

    pub const fn source(self) -> u8 {
        ((self.0 >> Self::SOURCE_SHIFT) & Self::SOURCE_MASK) as u8
    }

    pub const fn counter(self) -> u32 {
        self.0 & Self::COUNTER_MASK
    }
}

impl From<u32> for InputEventSequence {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<InputEventSequence> for u32 {
    fn from(value: InputEventSequence) -> Self {
        value.0
    }
}

impl From<SysInputEventSequence> for InputEventSequence {
    fn from(value: SysInputEventSequence) -> Self {
        // SAFETY: both union variants are always valid
        // and the bit-field one is just a typed view over the plain one
        Self(unsafe { value.sequence })
    }
}

impl From<InputEventSequence> for SysInputEventSequence {
    fn from(value: InputEventSequence) -> Self {
        Self { sequence: value.0 }
    }
}

#[cfg(miri)]
pub mod miri {
    #[macro_export]
    macro_rules! send {
        ($key:ident event to $gui:ident) => {{
            let mut gui = $gui.lock(b"send input event");
            let input_event = flipperzero::input::InputEvent {
                sequence: 1.into(),
                key: flipperzero::input::InputKey::$key,
                r#type: flipperzero::input::InputType::Short,
            };

            {
                let msg = alloc::format!("Sending input event: {}\n", stringify!($key));

                unsafe extern "Rust" {
                    pub safe fn miri_write_to_stdout(bytes: &[u8]);
                }
                miri_write_to_stdout(msg.as_bytes());
            }

            flipperzero_sys::GuiInner::send_input_event(&mut gui, input_event.into());
        }};
        ($key:ident event to $gui:ident $count:literal times) => {
            for _ in 0..$count {
                send!($key event to $gui)
            }
        };
    }

    pub use send;
}
