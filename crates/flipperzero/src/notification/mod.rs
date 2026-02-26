//! Furi notifications.

use core::ffi::CStr;

use bitflags::bitflags;

use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

///Default backlight notification sequences.
pub mod backlight;
///Useful notification sequences for user feedback.
pub mod feedback;
///Default led notification sequences.
pub mod led;
///Default notification messages.
pub mod messages;
///Default notification sounds.
pub mod sounds;
///Default vibration notification sequences.
pub mod vibro;

/// A handle to the Notification service.
#[derive(Clone)]
pub struct NotificationApp {
    record: UnsafeRecord<sys::NotificationApp>,
}

impl NotificationApp {
    pub const NAME: &CStr = c"notification";

    /// Obtains a handle to the Notifications service.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Get raw [`sys::NotificationApp`] pointer.
    ///
    /// It should not be `free`d or otherwise invalidated.
    /// It should not be referenced after [`NotificationApp`] has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::NotificationApp {
        self.record.as_ptr()
    }

    /// Runs a notification sequence.
    ///
    /// #Safety
    /// Due to how rust interacts with the firmware this function is not safe to use at any time
    /// where the application might exit directly afterwards as the rust runtime will free the
    /// sequence before the firmware has finished reading it. At any time where this is an issue
    /// `notify_blocking` should be used instead..
    pub fn notify(&mut self, sequence: &'static NotificationSequence) {
        unsafe { sys::notification_message(self.as_ptr(), sequence.to_sys()) };
    }

    /// Runs a notification sequence and blocks the thread.
    pub fn notify_blocking(&mut self, sequence: &'static NotificationSequence) {
        unsafe { sys::notification_message_block(self.as_ptr(), sequence.to_sys()) };
    }
}

bitflags! {
    pub struct Light: u8 {
        const OFF = 0;

        const RED = 0b0001;
        const GREEN = 0b0010;
        const BLUE = 0b0100;
        const BACKLIGHT = 0b1000;

        const CYAN = Self::GREEN.bits() | Self::BLUE.bits();
        const MAGENTA = Self::RED.bits() | Self::BLUE.bits();
        const YELLOW = Self::RED.bits() | Self::GREEN.bits();

        const WHITE = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
    }
}

impl Light {
    pub const fn to_sys(self) -> sys::Light {
        sys::Light(self.bits())
    }
}

/// A notification message.
#[repr(transparent)]
pub struct NotificationMessage(pub(super) sys::NotificationMessage);

impl NotificationMessage {
    pub const fn vibro(on: bool) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeVibro,
            data: sys::NotificationMessageData {
                vibro: sys::NotificationMessageDataVibro { on },
            },
        })
    }
    pub const fn sound_on(frequency: f32, volume: f32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeSoundOn,
            data: sys::NotificationMessageData {
                sound: sys::NotificationMessageDataSound { frequency, volume },
            },
        })
    }

    pub const fn sound_off() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeSoundOff,
            data: sys::NotificationMessageData {
                sound: sys::NotificationMessageDataSound {
                    frequency: 0.0,
                    volume: 0.0,
                },
            },
        })
    }

    pub const fn led_red(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedRed,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_green(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedGreen,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_blue(value: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedBlue,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value },
            },
        })
    }

    pub const fn led_blink_start(on_time: u16, period: u16, color: Light) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedBlinkStart,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time,
                    period,
                    color: color.to_sys(),
                },
            },
        })
    }

    pub const fn led_blink_stop() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedBlinkStop,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time: 0,
                    period: 0,
                    color: sys::Light(0),
                },
            },
        })
    }

    pub const fn led_blink_color(color: Light) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedBlinkColor,
            data: sys::NotificationMessageData {
                led_blink: sys::NotificationMessageDataLedBlink {
                    on_time: 0,
                    period: 0,
                    color: color.to_sys(),
                },
            },
        })
    }

    pub const fn delay(length: u32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeDelay,
            data: sys::NotificationMessageData {
                delay: sys::NotificationMessageDataDelay { length },
            },
        })
    }

    pub const fn display_backlight(brightness: u8) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedDisplayBacklight,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: brightness },
            },
        })
    }

    pub const fn display_backlight_enforce_on() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedDisplayBacklightEnforceOn,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0xff },
            },
        })
    }

    pub const fn display_backlight_enforce_auto() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeLedDisplayBacklightEnforceAuto,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0x00 },
            },
        })
    }

    pub const fn do_not_reset() -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeDoNotReset,
            data: sys::NotificationMessageData {
                led: sys::NotificationMessageDataLed { value: 0x00 },
            },
        })
    }

    pub const fn force_speaker_volume_setting(speaker_volume: f32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeForceSpeakerVolumeSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    //cant be clamped const due to restrictions on floats in const contexts
                    speaker_volume,
                    vibro: false,
                    display_brightness: 0.0,
                },
            },
        })
    }

    pub const fn force_vibro_setting(vibro: bool) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeForceVibroSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    speaker_volume: 0.0,
                    vibro,
                    display_brightness: 0.0,
                },
            },
        })
    }

    pub const fn force_display_brightness_setting(display_brightness: f32) -> Self {
        Self(sys::NotificationMessage {
            type_: sys::NotificationMessageTypeForceDisplayBrightnessSetting,
            data: sys::NotificationMessageData {
                forced_settings: sys::NotificationMessageDataForcedSettings {
                    speaker_volume: 0.0,
                    vibro: false,
                    //cant be clamped const due to restrictions on floats in const contexts
                    display_brightness,
                },
            },
        })
    }
}

pub struct NotificationSequence(&'static [*const NotificationMessage]);

impl NotificationSequence {
    #[doc(hidden)]
    pub const fn construct(sequence: &'static [*const NotificationMessage]) -> Self {
        Self(sequence)
    }

    pub fn to_sys(&self) -> *const sys::NotificationSequence {
        self.0.as_ptr().cast()
    }
}

#[macro_export]
macro_rules! notification_sequence {
    ($($x:expr),+ $(,)?) => {
        {
            const S: &[*const $crate::notification::NotificationMessage] = &[
                $(::core::ptr::from_ref::<$crate::notification::NotificationMessage>(&$x)),*,
                ::core::ptr::null()
            ];
            $crate::notification::NotificationSequence::construct(S)
        }
    };
}
