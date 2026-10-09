use crate::internals::macros::impl_std_error;
use core::ffi::CStr;
use core::fmt::{self, Display, Formatter};
use flipperzero_sys::{self as sys, InputKey as SysInputKey};
use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, uwrite};

/// Input key of a Flipper, i.e. its button.
///
/// Corresponds to raw [`SysInputKey`].
#[derive(Copy, Clone, Debug, uDebug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InputKey {
    /// **Up** key (top triangle).
    Up,
    /// **Down** key (bottom triangle).
    Down,
    /// **Right** key (right triangle).
    Right,
    /// **Left** key (left triangle).
    Left,
    /// **Ok** key (central round).
    Ok,
    /// **Back** key (right bottom backward arrow).
    Back,
}

impl InputKey {
    /// Gets the name of this input key.
    /// Unlike `Debug` and `uDebug` which use Rust enum name,
    /// this relies on Flipper's API intended for this purpose.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::input::InputKey;
    /// assert_eq!(InputKey::Up.name(), "Up");
    /// ```
    pub fn name(self) -> &'static CStr {
        let this = SysInputKey::from(self);
        // SAFETY: `this` is a valid enum value
        // and the returned string is a static string
        unsafe { CStr::from_ptr(sys::input_get_key_name(this)) }
    }
}

impl TryFrom<SysInputKey> for InputKey {
    type Error = FromSysInputKeyError;

    fn try_from(value: SysInputKey) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::InputKeyUp => Self::Up,
            sys::InputKeyDown => Self::Down,
            sys::InputKeyRight => Self::Right,
            sys::InputKeyLeft => Self::Left,
            sys::InputKeyOk => Self::Ok,
            sys::InputKeyBack => Self::Back,
            sys::InputKeyMAX => Err(Self::Error::Max)?,
            invalid => Err(Self::Error::Invalid(invalid))?,
        })
    }
}

impl From<InputKey> for SysInputKey {
    fn from(value: InputKey) -> Self {
        match value {
            InputKey::Up => sys::InputKeyUp,
            InputKey::Down => sys::InputKeyDown,
            InputKey::Right => sys::InputKeyRight,
            InputKey::Left => sys::InputKeyLeft,
            InputKey::Ok => sys::InputKeyOk,
            InputKey::Back => sys::InputKeyBack,
        }
    }
}

/// An error which may occur while trying
/// to convert raw [`SysInputKey`] to [`InputKey`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FromSysInputKeyError {
    /// The [`SysInputKey`] is [`MAX`][sys::InputKeyMAX]
    /// which is a meta-value used to track enum size.
    Max,
    /// The [`SysInputKey`] is an invalid value
    /// other than [`MAX`][sys::InputKeyMAX].
    Invalid(SysInputKey),
}

impl Display for FromSysInputKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Max => write!(
                f,
                "input key ID {} (MAX) is a meta-value",
                sys::InputKeyMAX.0,
            ),
            Self::Invalid(id) => write!(f, "input key ID {} is invalid", id.0),
        }
    }
}

impl uDebug for FromSysInputKeyError {
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

impl uDisplay for FromSysInputKeyError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Self::Max => uwrite!(
                f,
                "input key ID {} (Max) is a meta-value",
                sys::InputKeyMAX.0,
            ),
            Self::Invalid(id) => uwrite!(f, "input key ID {} is invalid", id.0),
        }
    }
}

impl_std_error!(FromSysInputKeyError);
