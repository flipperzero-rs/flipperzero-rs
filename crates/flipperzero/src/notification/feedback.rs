use super::{NotificationSequence, messages, sounds};
use crate::notification_sequence;

pub const CHARGING: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_0];
pub const CHARGED: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::RED_0];
pub const NOT_CHARGING: NotificationSequence =
    notification_sequence![messages::RED_0, messages::GREEN_0];

pub const SUCCESS: NotificationSequence = notification_sequence![
    messages::DISPLAY_BACKLIGHT_ON,
    messages::GREEN_255,
    messages::VIBRO_ON,
    sounds::C5,
    messages::DELAY_50,
    messages::VIBRO_OFF,
    sounds::E5,
    messages::DELAY_50,
    sounds::G5,
    messages::DELAY_50,
    sounds::C6,
    messages::DELAY_50,
    messages::SOUND_OFF,
];

pub const ERROR: NotificationSequence = notification_sequence![
    messages::DISPLAY_BACKLIGHT_ON,
    messages::RED_255,
    messages::VIBRO_ON,
    sounds::C5,
    messages::DELAY_100,
    messages::VIBRO_OFF,
    messages::SOUND_OFF,
    messages::DELAY_100,
    messages::VIBRO_ON,
    sounds::C5,
    messages::DELAY_100,
    messages::VIBRO_OFF,
    messages::SOUND_OFF,
];

pub const AUDIO_VISUAL_ALERT: NotificationSequence = notification_sequence![
    messages::FORCE_SPEAKER_VOLUME_SETTING_1,
    messages::FORCE_VIBRO_SETTING_ON,
    messages::FORCE_DISPLAY_BRIGHTNESS_SETTING_1,
    messages::VIBRO_ON,
    messages::DISPLAY_BACKLIGHT_ON,
    sounds::C7,
    messages::DELAY_250,
    messages::DISPLAY_BACKLIGHT_OFF,
    sounds::C4,
    messages::DELAY_250,
    messages::DISPLAY_BACKLIGHT_ON,
    sounds::C7,
    messages::DELAY_250,
    messages::DISPLAY_BACKLIGHT_OFF,
    sounds::C4,
    messages::DELAY_250,
    messages::DISPLAY_BACKLIGHT_ON,
    sounds::C7,
    messages::DELAY_250,
    messages::DISPLAY_BACKLIGHT_OFF,
    sounds::C4,
    messages::DELAY_250,
    messages::SOUND_OFF,
    messages::VIBRO_OFF,
];
