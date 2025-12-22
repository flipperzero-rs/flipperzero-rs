//! Notification example for Flipper Zero

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;
use core::time::Duration;

use flipperzero::{
    furi::thread::sleep,
    notification::{NotificationApp, feedback, led},
};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(name = "Rust notification example");

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    let mut app = NotificationApp::open();

    // Set the notification LED to different colours
    for sequences in [&led::ONLY_RED, &led::ONLY_GREEN, &led::ONLY_BLUE] {
        app.notify(sequences);
        sleep(Duration::from_secs(1));
    }

    app.notify(&led::RESET_RGB);

    // Success!
    app.notify_blocking(&feedback::SUCCESS);

    0
}
