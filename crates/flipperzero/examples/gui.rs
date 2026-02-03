//! Demonstrates use of the Flipper Zero GUI.
//!
//! This app writes "Hello, Rust!" to the display.
//!
//! Currently uses unsafe `sys` bindings as there is no high level GUI API yet.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use core::ffi::{CStr, c_void};
use core::ptr;
use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys::furi::UnsafeRecord;
use flipperzero_sys::{self as sys, Gui};

const FULLSCREEN: sys::GuiLayer = sys::GuiLayerFullscreen;

manifest!(name = "Rust GUI example");
entry!(main);

/// View draw handler.
pub unsafe extern "C" fn draw_callback(canvas: *mut sys::Canvas, _context: *mut c_void) {
    unsafe {
        sys::canvas_draw_str(canvas, 39, 31, c"Hello, Rust!".as_ptr());
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    // Currently there is no high level GUI bindings,
    // so this all has to be done using the `sys` bindings.
    unsafe {
        let view_port = sys::view_port_alloc();
        sys::view_port_draw_callback_set(view_port, Some(draw_callback), ptr::null_mut());

        {
            let gui: UnsafeRecord<Gui> = UnsafeRecord::open(c"gui");
            #[cfg(miri)]
            debug_assert_eq!(
                {
                    extern crate alloc;
                    use alloc::sync::Arc;

                    let gui: Arc<Gui> = Arc::from_raw(gui.as_ptr());
                    let count = Arc::strong_count(&gui);
                    // Intentionally leak again?
                    let _gui = Arc::into_raw(gui);

                    count
                },
                3,
                "[unsafe record, static cell, gui service thread]"
            );

            sys::gui_add_view_port(gui.as_ptr(), view_port, FULLSCREEN);

            #[cfg(miri)]
            debug_assert_eq!(
                {
                    extern crate alloc;
                    use alloc::sync::Arc;

                    let gui: Arc<Gui> = Arc::from_raw(gui.as_ptr());
                    let count = Arc::strong_count(&gui);
                    // Intentionally leak again?
                    let _gui = Arc::into_raw(gui);

                    count
                },
                4,
                "[unsafe record, static cell, gui service thread, view_port reference]"
            );
            sleep(Duration::from_secs(1));

            sys::gui_remove_view_port(gui.as_ptr(), view_port);
            sys::view_port_enabled_set(view_port, false);

            #[cfg(miri)]
            debug_assert_eq!(
                {
                    extern crate alloc;
                    use alloc::sync::Arc;

                    let gui: Arc<Gui> = Arc::from_raw(gui.as_ptr());
                    let count = Arc::strong_count(&gui);
                    // Intentionally leak again?
                    let _gui = Arc::into_raw(gui);

                    count
                },
                3,
                "[unsafe record, static cell, gui service thread]"
            );
        }
        sys::view_port_free(view_port);
    }

    0
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    // TODO: Is there any benefit to Miri in hooking up the binary arguments to
    // the test runner?
    main(None).try_into().unwrap_or(isize::MAX)
}
