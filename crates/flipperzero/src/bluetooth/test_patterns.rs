//! Various Bluetooth test patterns.

use flipperzero_sys as sys;
use ufmt::derive::uDebug;

use crate::info;

use super::Bluetooth;

/// A BLE tone transmitter.
pub struct CarrierTx {
    _service: Bluetooth,
}

impl Drop for CarrierTx {
    fn drop(&mut self) {
        self.stop();
    }
}

impl CarrierTx {
    /// Acquires the Bluetooth service and constructs a new tone transmitter.
    pub fn prepare() -> Self {
        let _service = Bluetooth::open();
        unsafe { sys::furi_hal_bt_reinit() };
        Self { _service }
    }

    /// Starts transmitting a carrier tone on the specified channel with the given power.
    ///
    /// Returns an error if the following requirements are not satisfied:
    /// - `channel` is an integer between 0 and 39 inclusive.
    /// - `power` is an integer in dB between 0 and 6 inclusive.
    pub fn start(&mut self, channel: u8, power: u8) -> Result<(), Error> {
        if !(0..=39).contains(&channel) {
            Err(Error::InvalidChannel)
        } else if !(0..=6).contains(&power) {
            Err(Error::InvalidPower)
        } else {
            unsafe { sys::furi_hal_bt_start_tone_tx(channel, 0x19 + power) };
            Ok(())
        }
    }

    /// Stops transmitting a carrier tone.
    pub fn stop(&mut self) {
        unsafe { sys::furi_hal_bt_stop_tone_tx() };
    }
}

/// A test packet transmitter.
pub struct PacketTx {
    _service: Bluetooth,
}

impl PacketTx {
    /// Acquires the Bluetooth service and constructs a new test packet transmitter.
    pub fn prepare() -> Self {
        let _service = Bluetooth::open();
        unsafe { sys::furi_hal_bt_reinit() };
        Self { _service }
    }

    /// Starts transmitting test packets on the specified channel with the given power.
    ///
    /// Returns an error if the following requirements are not satisfied:
    /// - `channel` is an integer between 0 and 39 inclusive.
    /// - `datarate` is an integer between 1 and 2 inclusive.
    pub fn start(&mut self, channel: u8, pattern: Pattern, datarate: u8) -> Result<(), Error> {
        if !(0..=39).contains(&channel) {
            Err(Error::InvalidChannel)
        } else if !(1..=2).contains(&datarate) {
            Err(Error::InvalidDatarate)
        } else {
            unsafe { sys::furi_hal_bt_start_packet_tx(channel, pattern.to_raw(), datarate) };
            Ok(())
        }
    }

    /// Stops transmitting test packets.
    ///
    /// Returns the number of packets sent.
    pub fn stop(&mut self) -> u16 {
        let sent_count = unsafe { sys::furi_hal_bt_stop_packet_test() };
        info!("Sent {} packets", sent_count);
        sent_count
    }
}

/// A packet receiver.
pub struct PacketRx {
    _service: Bluetooth,
}

impl Drop for PacketRx {
    fn drop(&mut self) {
        self.stop();
    }
}

impl PacketRx {
    /// Acquires the Bluetooth service and constructs a new packet receiver.
    pub fn prepare() -> Self {
        let _service = Bluetooth::open();
        unsafe { sys::furi_hal_bt_reinit() };
        Self { _service }
    }

    /// Starts listening for packets on the specified channel.
    ///
    /// Returns an error if the following requirements are not satisfied:
    /// - `channel` is an integer between 0 and 39 inclusive.
    pub fn start(&mut self, channel: u8, datarate: u8) -> Result<(), Error> {
        if !(0..=39).contains(&channel) {
            Err(Error::InvalidChannel)
        } else if !(1..=2).contains(&datarate) {
            Err(Error::InvalidDatarate)
        } else {
            unsafe { sys::furi_hal_bt_start_packet_rx(channel, datarate) };
            Ok(())
        }
    }

    /// Returns the current Received Signal Strength Indicator (RSSI) in dBm.
    pub fn rssi(&self) -> f32 {
        unsafe { sys::furi_hal_bt_get_rssi() }
    }

    /// Stops listening for packets.
    ///
    /// Returns the number of packets received.
    pub fn stop(&mut self) -> u16 {
        let received_count = unsafe { sys::furi_hal_bt_stop_packet_test() };
        info!("Received {} packets", received_count);
        received_count
    }
}

/// A pattern of test packets.
#[derive(Clone, Copy, Debug, uDebug, PartialEq, Eq)]
pub enum Pattern {
    /// Pseudo-Random bit sequence 9.
    PseudoRandomBitSequence9,
    /// Pattern of alternating bits '11110000'.
    AlternatingNibbles,
    /// Pattern of alternating bits '10101010'.
    AlternatingBits,
    /// Pseudo-Random bit sequence 15.
    PseudoRandomBitSequence15,
    /// Pattern of All '1' bits.
    AllOnes,
    /// Pattern of All '0' bits.
    AllZeros,
}

impl Pattern {
    fn to_raw(self) -> u8 {
        match self {
            Pattern::PseudoRandomBitSequence9 => 0,
            Pattern::AlternatingNibbles => 1,
            Pattern::AlternatingBits => 2,
            Pattern::PseudoRandomBitSequence15 => 3,
            Pattern::AllOnes => 4,
            Pattern::AllZeros => 5,
        }
    }
}

/// Errors that can occur while producing test patterns.
pub enum Error {
    InvalidChannel,
    InvalidDatarate,
    InvalidPower,
}
