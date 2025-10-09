//! Example Images application.
//! See https://github.com/flipperdevices/flipperzero-firmware/blob/dev/applications/examples/example_images/example_images.c

#![no_std]
#![no_main]

// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::{CStr, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr::addr_of;

use flipperzero_sys::furi::UnsafeRecord;

use flipperzero_rt as rt;
use flipperzero_sys as sys;

use sys::Icon;

rt::manifest!(name = "Example: Images");
rt::entry!(main);

static mut TARGET_ICON: Icon = Icon {
    width: 48,
    height: 32,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { &raw const TARGET_FRAMES[0] },
};
static mut TARGET_FRAMES: [*const u8; 1] = [include_bytes!("icons/rustacean-48x32.icon").as_ptr()];

static mut IMAGE_POSITION: ImagePosition = ImagePosition { x: 0, y: 0 };

#[repr(C)]
struct ImagePosition {
    pub x: i32,
    pub y: i32,
}

// Screen is 128x64 px
extern "C" fn app_draw_callback(canvas: *mut sys::Canvas, _ctx: *mut c_void) {
    unsafe {
        sys::canvas_clear(canvas);
        sys::canvas_draw_icon(
            canvas,
            IMAGE_POSITION.x,
            IMAGE_POSITION.y,
            addr_of!(TARGET_ICON) as *const Icon as *const c_void as *const sys::Icon,
        );
    }
}

extern "C" fn app_input_callback(input_event: *mut sys::InputEvent, ctx: *mut c_void) {
    unsafe {
        let event_queue = ctx as *mut sys::FuriMessageQueue;
        sys::furi_message_queue_put(event_queue, input_event as *mut c_void, 0);
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    unsafe {
        let event_queue = sys::furi_message_queue_alloc(8, mem::size_of::<sys::InputEvent>() as u32)
            as *mut sys::FuriMessageQueue;

        // Configure view port
        let view_port = sys::view_port_alloc();
        sys::view_port_draw_callback_set(
            view_port,
            Some(app_draw_callback),
            view_port as *mut c_void,
        );
        sys::view_port_input_callback_set(
            view_port,
            Some(app_input_callback),
            event_queue as *mut c_void,
        );

        // Register view port in GUI
        let gui = UnsafeRecord::open(c"gui");
        sys::gui_add_view_port(gui.as_ptr(), view_port, sys::GuiLayerFullscreen);

        let mut event: MaybeUninit<sys::InputEvent> = MaybeUninit::uninit();

        let mut running = true;
        while running {
            if sys::furi_message_queue_get(event_queue, event.as_mut_ptr() as *mut c_void, 100)
                == sys::FuriStatusOk
            {
                let event = event.assume_init();
                if event.type_ == sys::InputTypePress || event.type_ == sys::InputTypeRepeat {
                    match event.key {
                        sys::InputKeyLeft => IMAGE_POSITION.x -= 2,
                        sys::InputKeyRight => IMAGE_POSITION.x += 2,
                        sys::InputKeyUp => IMAGE_POSITION.y -= 2,
                        sys::InputKeyDown => IMAGE_POSITION.y += 2,
                        _ => running = false,
                    }
                }
            }
            sys::view_port_update(view_port);
        }

        sys::view_port_enabled_set(view_port, false);
        sys::gui_remove_view_port(gui.as_ptr(), view_port);
        sys::view_port_free(view_port);
        sys::furi_message_queue_free(event_queue);
    }

    0
}
