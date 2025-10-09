use super::{NotificationSequence, messages};
use crate::notification_sequence;

pub const RESET_VIBRO: NotificationSequence = notification_sequence![messages::VIBRO_OFF];
pub const VIBRO_ON: NotificationSequence = notification_sequence![messages::VIBRO_ON];

pub const SINGLE_VIBRO: NotificationSequence =
    notification_sequence![messages::VIBRO_ON, messages::DELAY_100, messages::VIBRO_OFF,];

pub const DOUBLE_VIBRO: NotificationSequence = notification_sequence![
    messages::VIBRO_ON,
    messages::DELAY_100,
    messages::VIBRO_OFF,
    messages::DELAY_100,
    messages::VIBRO_ON,
    messages::DELAY_100,
    messages::VIBRO_OFF,
];
