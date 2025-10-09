use super::{NotificationSequence, messages};
use crate::notification_sequence;

pub const RESET_RED: NotificationSequence = notification_sequence![messages::RED_0];
pub const RESET_GREEN: NotificationSequence = notification_sequence![messages::GREEN_0];
pub const RESET_BLUE: NotificationSequence = notification_sequence![messages::BLUE_0];
pub const RESET_RGB: NotificationSequence =
    notification_sequence![messages::RED_0, messages::GREEN_0, messages::BLUE_0];

pub const ONLY_RED: NotificationSequence = notification_sequence![
    messages::RED_255,
    messages::GREEN_0,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];
pub const ONLY_GREEN: NotificationSequence = notification_sequence![
    messages::RED_0,
    messages::GREEN_255,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];
pub const ONLY_BLUE: NotificationSequence = notification_sequence![
    messages::RED_0,
    messages::GREEN_0,
    messages::BLUE_255,
    messages::DO_NOT_RESET,
];

pub const SET_RED: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DO_NOT_RESET];
pub const SET_GREEN: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DO_NOT_RESET];
pub const SET_BLUE: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DO_NOT_RESET];

pub const SOLID_YELLOW: NotificationSequence = notification_sequence![
    messages::RED_255,
    messages::GREEN_255,
    messages::BLUE_0,
    messages::DO_NOT_RESET,
];

pub const BLINK_RED_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DELAY_10];
pub const BLINK_GREEN_10: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DELAY_10];
pub const BLINK_BLUE_10: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DELAY_10];
pub const BLINK_YELLOW_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_255, messages::DELAY_10,];
pub const BLINK_CYAN_10: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::BLUE_255, messages::DELAY_10,];
pub const BLINK_MAGNENTA_10: NotificationSequence =
    notification_sequence![messages::RED_255, messages::BLUE_255, messages::DELAY_10];

pub const BLINK_RED_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::DELAY_100];
pub const BLINK_GREEN_100: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::DELAY_100];
pub const BLINK_BLUE_100: NotificationSequence =
    notification_sequence![messages::BLUE_255, messages::DELAY_100];
pub const BLINK_YELLOW_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::GREEN_255, messages::DELAY_100,];
pub const BLINK_CYAN_100: NotificationSequence =
    notification_sequence![messages::GREEN_255, messages::BLUE_255, messages::DELAY_100,];
pub const BLINK_MAGNENTA_100: NotificationSequence =
    notification_sequence![messages::RED_255, messages::BLUE_255, messages::DELAY_100,];

pub const BLINK_START_RED: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_RED,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_GREEN: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_GREEN,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_BLUE: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_BLUE,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_YELLOW: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_YELLOW,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_CYAN: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_CYAN,
    messages::DO_NOT_RESET,
];
pub const BLINK_START_MAGENTA: NotificationSequence = notification_sequence![
    messages::BLINK_START_10,
    messages::BLINK_SET_COLOR_MAGENTA,
    messages::DO_NOT_RESET,
];
pub const BLINK_STOP: NotificationSequence = notification_sequence![messages::BLINK_STOP];
