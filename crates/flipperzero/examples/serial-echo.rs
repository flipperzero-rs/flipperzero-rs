//! Serial Echo example for Flipper Zero.
//! While running, echos input on LPUART (15 = TX, 16 = RX) until End-of-Transmission (Ctrl+D).

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::furi::event_flag::EventFlag;
use flipperzero::furi::time::FuriDuration;
use flipperzero::serial::{LPUART, SerialHandle, SerialId};
use flipperzero::{debug, info};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Serial Echo",
    app_version = 1,
    has_icon = true,
    // See `docs/icons.md` for icon format
    icon = "icons/rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

/// ASCII End-of-Transmission character (Ctrl + D)
const EOT: u8 = 0x04;

/// Serial to use (LPUART: 15 = TX, 16 = RX)
const CHANNEL: SerialId = LPUART;

/// Expected baud-rate (bits/second)
const BAUD_RATE: u32 = 9600;

const FLAG_STOP: u32 = 1 << 0;

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    let serial = SerialHandle::acquire(CHANNEL).unwrap();
    serial.init(BAUD_RATE);

    serial.tx("Start input echo. Press Ctrl+D to exit.\r\n".as_bytes());
    serial.tx_wait_complete();

    let event = EventFlag::new();
    let _reciever = serial.async_receiver(|data| {
        debug!("Received {} bytes", data.len());

        if data.iter().any(|&b| b == EOT) {
            info!("Got End-of-Transmission byte");
            event.set(FLAG_STOP).unwrap();
        }

        // Echo input
        serial.tx(data);
        serial.tx_wait_complete();
    });

    event
        .wait_all_flags(FLAG_STOP, false, FuriDuration::MAX)
        .unwrap();

    serial.tx("Stop input echo.\r\n".as_bytes());
    serial.tx_wait_complete();

    0
}
