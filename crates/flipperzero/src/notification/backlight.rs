use super::{NotificationSequence, messages};
use crate::notification_sequence;

pub const RESET_DISPLAY: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_OFF];

pub const DISPLAY_BACKLIGHT_ON: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ON];
pub const DISPLAY_BACKLIGHT_OFF: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_OFF];

pub const DISPLAY_BACKLIGHT_ENFORCE_ON: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ENFORCE_ON];
pub const DISPLAY_BACKLIGHT_ENFORCE_AUTO: NotificationSequence =
    notification_sequence![messages::DISPLAY_BACKLIGHT_ENFORCE_AUTO];

pub const DISPLAY_BACKLIGHT_OFF_DELAY_1000: NotificationSequence =
    notification_sequence![messages::DELAY_1000, messages::DISPLAY_BACKLIGHT_OFF];
