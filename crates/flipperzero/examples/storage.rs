//! Storage example for Flipper Zero.
//! This app writes "Hello, Rust!" to a file on the SD card. Then it opens a file browser dialog and
//! lets the user select the file. Finally, it reads the file back and prints the contents to the console.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::dialogs::{DialogFileBrowserOptions, DialogsApp};
use flipperzero::furi::string::FuriString;
use flipperzero::io::*;
use flipperzero::path::Path;
use flipperzero::println;
use flipperzero::storage::*;
use flipperzero_rt::{entry, manifest};

manifest!(name = "Rust storage example");
entry!(main);

fn main(_args: Option<&CStr>) -> i32 {
    // First, we'll create a file on the SD card and write "Hello, Rust!" to it.
    let path = Path::new(c"/ext/hello-rust.txt");

    match File::create(path) {
        Ok(mut handle) => {
            if let Err(e) = handle.write(b"Hello, Rust!") {
                println!("couldn't write to file: {}", e);
            }
        }
        Err(e) => println!("couldn't open path: {}", e),
    }

    // Next, we'll open a file browser dialog and let the user select the file.
    let mut dialogs_app = DialogsApp::open();
    let file_browser_options = DialogFileBrowserOptions::new().set_hide_ext(false);
    let mut start_path = FuriString::from(path.as_c_str());
    let result_path =
        dialogs_app.show_file_browser(Some(&mut start_path), Some(&file_browser_options));
    if let Some(result_path) = result_path {
        println!("file selected {}", result_path);

        // Now, we'll open it and read it back.
        match read_to_string(&result_path) {
            Ok(string) => println!("read from file: {}", string),
            Err(e) => println!("couldn't read to string: {}", e),
        }
    } else {
        println!("no file selected");
    }

    0 // File is synchronized and closed when `file` goes out of scope.
}
