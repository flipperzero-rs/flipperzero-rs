//! BLE Serial Echo example for Flipper Zero.
//! While running, echos input received over BLE serial back to the sender.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::bluetooth::serial::BleSerial;
use flipperzero::furi::event_flag::EventFlag;
use flipperzero::furi::time::FuriDuration;
use flipperzero::{debug, info};
use flipperzero_rt::{entry, manifest};

manifest!(
    name = "BLE Serial Echo",
    app_version = 1,
    has_icon = true,
    icon = "icons/rustacean-10x10.icon",
);

entry!(main);

const FLAG_STOP: u32 = 1 << 0;

fn main(_args: Option<&CStr>) -> i32 {
    let ble_serial = match BleSerial::start() {
        Ok(s) => s,
        Err(_) => {
            info!("Failed to start BLE serial");
            return 1;
        }
    };

    info!("BLE serial started. Connect via BLE to echo data.");

    let event = EventFlag::new();
    let _receiver = ble_serial.async_receiver(|data| {
        debug!("Received {} bytes", data.len());

        // Echo input back over BLE serial
        ble_serial.tx(data);
    });

    // Wait until stopped (e.g. via back button or BLE reset request)
    event
        .wait_all_flags(FLAG_STOP, false, FuriDuration::MAX)
        .unwrap();

    0
}
