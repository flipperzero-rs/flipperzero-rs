//! Common sounds that can be produced by the Flipper Zero.

use super::{NotificationMessage, NotificationSequence, messages};
use crate::notification_sequence;

pub const RESET_SOUND: NotificationSequence = notification_sequence![messages::SOUND_OFF];
pub static CLICK: NotificationMessage = NotificationMessage::sound_on(1.0, 1.0);

pub const C0: NotificationMessage = NotificationMessage::sound_on(16.35, 1.0);
pub const CS0: NotificationMessage = NotificationMessage::sound_on(17.32, 1.0);
pub const D0: NotificationMessage = NotificationMessage::sound_on(18.35, 1.0);
pub const DS0: NotificationMessage = NotificationMessage::sound_on(19.45, 1.0);
pub const E0: NotificationMessage = NotificationMessage::sound_on(20.6, 1.0);
pub const F0: NotificationMessage = NotificationMessage::sound_on(21.83, 1.0);
pub const FS0: NotificationMessage = NotificationMessage::sound_on(23.12, 1.0);
pub const G0: NotificationMessage = NotificationMessage::sound_on(24.5, 1.0);
pub const GS0: NotificationMessage = NotificationMessage::sound_on(25.96, 1.0);
pub const A0: NotificationMessage = NotificationMessage::sound_on(27.5, 1.0);
pub const AS0: NotificationMessage = NotificationMessage::sound_on(29.14, 1.0);
pub const B0: NotificationMessage = NotificationMessage::sound_on(30.87, 1.0);

pub const C1: NotificationMessage = NotificationMessage::sound_on(32.7, 1.0);
pub const CS1: NotificationMessage = NotificationMessage::sound_on(34.65, 1.0);
pub const D1: NotificationMessage = NotificationMessage::sound_on(36.71, 1.0);
pub const DS1: NotificationMessage = NotificationMessage::sound_on(38.89, 1.0);
pub const E1: NotificationMessage = NotificationMessage::sound_on(41.2, 1.0);
pub const F1: NotificationMessage = NotificationMessage::sound_on(43.65, 1.0);
pub const FS1: NotificationMessage = NotificationMessage::sound_on(46.25, 1.0);
pub const G1: NotificationMessage = NotificationMessage::sound_on(49.0, 1.0);
pub const GS1: NotificationMessage = NotificationMessage::sound_on(51.91, 1.0);
pub const A1: NotificationMessage = NotificationMessage::sound_on(55.0, 1.0);
pub const AS1: NotificationMessage = NotificationMessage::sound_on(58.27, 1.0);
pub const B1: NotificationMessage = NotificationMessage::sound_on(61.74, 1.0);

pub const C2: NotificationMessage = NotificationMessage::sound_on(65.41, 1.0);
pub const CS2: NotificationMessage = NotificationMessage::sound_on(69.3, 1.0);
pub const D2: NotificationMessage = NotificationMessage::sound_on(73.42, 1.0);
pub const DS2: NotificationMessage = NotificationMessage::sound_on(77.78, 1.0);
pub const E2: NotificationMessage = NotificationMessage::sound_on(82.41, 1.0);
pub const F2: NotificationMessage = NotificationMessage::sound_on(87.31, 1.0);
pub const FS2: NotificationMessage = NotificationMessage::sound_on(92.5, 1.0);
pub const G2: NotificationMessage = NotificationMessage::sound_on(98.0, 1.0);
pub const GS2: NotificationMessage = NotificationMessage::sound_on(103.83, 1.0);
pub const A2: NotificationMessage = NotificationMessage::sound_on(110.0, 1.0);
pub const AS2: NotificationMessage = NotificationMessage::sound_on(116.54, 1.0);
pub const B2: NotificationMessage = NotificationMessage::sound_on(123.47, 1.0);

pub const C3: NotificationMessage = NotificationMessage::sound_on(130.81, 1.0);
pub const CS3: NotificationMessage = NotificationMessage::sound_on(138.59, 1.0);
pub const D3: NotificationMessage = NotificationMessage::sound_on(146.83, 1.0);
pub const DS3: NotificationMessage = NotificationMessage::sound_on(155.56, 1.0);
pub const E3: NotificationMessage = NotificationMessage::sound_on(164.81, 1.0);
pub const F3: NotificationMessage = NotificationMessage::sound_on(174.61, 1.0);
pub const FS3: NotificationMessage = NotificationMessage::sound_on(185.0, 1.0);
pub const G3: NotificationMessage = NotificationMessage::sound_on(196.0, 1.0);
pub const GS3: NotificationMessage = NotificationMessage::sound_on(207.65, 1.0);
pub const A3: NotificationMessage = NotificationMessage::sound_on(220.0, 1.0);
pub const AS3: NotificationMessage = NotificationMessage::sound_on(233.08, 1.0);
pub const B3: NotificationMessage = NotificationMessage::sound_on(246.94, 1.0);

pub const C4: NotificationMessage = NotificationMessage::sound_on(261.63, 1.0);
pub const CS4: NotificationMessage = NotificationMessage::sound_on(277.18, 1.0);
pub const D4: NotificationMessage = NotificationMessage::sound_on(293.66, 1.0);
pub const DS4: NotificationMessage = NotificationMessage::sound_on(311.13, 1.0);
pub const E4: NotificationMessage = NotificationMessage::sound_on(329.63, 1.0);
pub const F4: NotificationMessage = NotificationMessage::sound_on(349.23, 1.0);
pub const FS4: NotificationMessage = NotificationMessage::sound_on(369.99, 1.0);
pub const G4: NotificationMessage = NotificationMessage::sound_on(392.0, 1.0);
pub const GS4: NotificationMessage = NotificationMessage::sound_on(415.3, 1.0);
pub const A4: NotificationMessage = NotificationMessage::sound_on(440.0, 1.0);
pub const AS4: NotificationMessage = NotificationMessage::sound_on(466.16, 1.0);
pub const B4: NotificationMessage = NotificationMessage::sound_on(493.88, 1.0);

pub const C5: NotificationMessage = NotificationMessage::sound_on(523.25, 1.0);
pub const CS5: NotificationMessage = NotificationMessage::sound_on(554.37, 1.0);
pub const D5: NotificationMessage = NotificationMessage::sound_on(587.33, 1.0);
pub const DS5: NotificationMessage = NotificationMessage::sound_on(622.25, 1.0);
pub const E5: NotificationMessage = NotificationMessage::sound_on(659.26, 1.0);
pub const F5: NotificationMessage = NotificationMessage::sound_on(698.46, 1.0);
pub const FS5: NotificationMessage = NotificationMessage::sound_on(739.99, 1.0);
pub const G5: NotificationMessage = NotificationMessage::sound_on(783.99, 1.0);
pub const GS5: NotificationMessage = NotificationMessage::sound_on(830.61, 1.0);
pub const A5: NotificationMessage = NotificationMessage::sound_on(880.0, 1.0);
pub const AS5: NotificationMessage = NotificationMessage::sound_on(932.33, 1.0);
pub const B5: NotificationMessage = NotificationMessage::sound_on(987.77, 1.0);

pub const C6: NotificationMessage = NotificationMessage::sound_on(1046.5, 1.0);
pub const CS6: NotificationMessage = NotificationMessage::sound_on(1108.73, 1.0);
pub const D6: NotificationMessage = NotificationMessage::sound_on(1174.66, 1.0);
pub const DS6: NotificationMessage = NotificationMessage::sound_on(1244.51, 1.0);
pub const E6: NotificationMessage = NotificationMessage::sound_on(1318.51, 1.0);
pub const F6: NotificationMessage = NotificationMessage::sound_on(1396.91, 1.0);
pub const FS6: NotificationMessage = NotificationMessage::sound_on(1479.98, 1.0);
pub const G6: NotificationMessage = NotificationMessage::sound_on(1567.98, 1.0);
pub const GS6: NotificationMessage = NotificationMessage::sound_on(1661.22, 1.0);
pub const A6: NotificationMessage = NotificationMessage::sound_on(1760.0, 1.0);
pub const AS6: NotificationMessage = NotificationMessage::sound_on(1864.66, 1.0);
pub const B6: NotificationMessage = NotificationMessage::sound_on(1975.53, 1.0);

pub const C7: NotificationMessage = NotificationMessage::sound_on(2093.0, 1.0);
pub const CS7: NotificationMessage = NotificationMessage::sound_on(2217.46, 1.0);
pub const D7: NotificationMessage = NotificationMessage::sound_on(2349.32, 1.0);
pub const DS7: NotificationMessage = NotificationMessage::sound_on(2489.02, 1.0);
pub const E7: NotificationMessage = NotificationMessage::sound_on(2637.02, 1.0);
pub const F7: NotificationMessage = NotificationMessage::sound_on(2793.83, 1.0);
pub const FS7: NotificationMessage = NotificationMessage::sound_on(2959.96, 1.0);
pub const G7: NotificationMessage = NotificationMessage::sound_on(3135.96, 1.0);
pub const GS7: NotificationMessage = NotificationMessage::sound_on(3322.44, 1.0);
pub const A7: NotificationMessage = NotificationMessage::sound_on(3520.0, 1.0);
pub const AS7: NotificationMessage = NotificationMessage::sound_on(3729.31, 1.0);
pub const B7: NotificationMessage = NotificationMessage::sound_on(3951.07, 1.0);

pub const C8: NotificationMessage = NotificationMessage::sound_on(4186.01, 1.0);
pub const CS8: NotificationMessage = NotificationMessage::sound_on(4434.92, 1.0);
pub const D8: NotificationMessage = NotificationMessage::sound_on(4698.64, 1.0);
pub const DS8: NotificationMessage = NotificationMessage::sound_on(4978.03, 1.0);
pub const E8: NotificationMessage = NotificationMessage::sound_on(5274.04, 1.0);
pub const F8: NotificationMessage = NotificationMessage::sound_on(5587.65, 1.0);
pub const FS8: NotificationMessage = NotificationMessage::sound_on(5919.91, 1.0);
pub const G8: NotificationMessage = NotificationMessage::sound_on(6271.93, 1.0);
pub const GS8: NotificationMessage = NotificationMessage::sound_on(6644.88, 1.0);
pub const A8: NotificationMessage = NotificationMessage::sound_on(7040.0, 1.0);
pub const AS8: NotificationMessage = NotificationMessage::sound_on(7458.62, 1.0);
pub const B8: NotificationMessage = NotificationMessage::sound_on(7902.13, 1.0);
