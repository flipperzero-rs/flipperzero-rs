//! Bluetooth example for Flipper Zero.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::{ffi::CStr, time::Duration};

use flipperzero::{
    bluetooth::{
        self,
        beacon::{AdPacket, Beacon},
    },
    error,
    furi::thread,
    info,
};
use flipperzero_rt::{entry, manifest};
use uuid::uuid;

#[cfg(feature = "alloc")]
use flipperzero::bluetooth::beacon::EddystoneUrlScheme;

manifest!(name = "Rust Bluetooth example");
entry!(main);

fn main(_args: Option<&CStr>) -> i32 {
    info!("Bluetooth is alive: {}", bluetooth::is_alive());
    info!("Bluetooth is active: {}", bluetooth::is_active());
    info!("Bluetooth state: {}", bluetooth::dump_state());

    if let Err(e) = test_beacon() {
        error!("Failed to test beacon: {:?}", e);
        return 1;
    }

    0
}

fn test_beacon() -> Result<(), bluetooth::beacon::Error> {
    let mut beacon = Beacon::acquire()?;

    // Set an iBeacon data packet.
    beacon.set_data_packet(AdPacket::i_beacon(
        uuid!("817D05F9-8424-4185-8C0E-9B4B9FA031F1"),
        42,
        1337,
        0xC6,
    ))?;

    info!("Broadcasting iBeacon for 10 seconds");
    beacon.start()?;
    thread::sleep(Duration::from_secs(10));
    beacon.stop()?;

    #[cfg(feature = "alloc")]
    {
        // Set an Eddystone-URL data packet.
        beacon.set_data_packet(AdPacket::eddystone_url(
            0xC6,
            EddystoneUrlScheme::HttpsWww,
            "rust-lang.org",
        ))?;

        info!("Broadcasting Eddystone-URL for 10 seconds");
        beacon.start()?;
        thread::sleep(Duration::from_secs(10));
        beacon.stop()?;
    }

    Ok(())
}
