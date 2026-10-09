//! Demonstrates use of the Flipper Zero GUI.
//!
//! This app writes "Hello, Rust!" to the display.
//!
//! Currently uses unsafe `sys` bindings as there is no high level GUI API yet.

#![no_main]
#![no_std]
#![cfg_attr(not(miri), forbid(unsafe_code))]

#[cfg(feature = "xbm")]
mod xbm_images;

// Required for panic handler
extern crate flipperzero_rt;

// Required for allocator
extern crate alloc;
#[cfg(feature = "alloc")]
extern crate flipperzero_alloc;

use alloc::{ffi::CString, string::ToString};
use core::ffi::CStr;

#[cfg(miri)]
use alloc::sync::Arc;
#[cfg(miri)]
use core::sync::atomic::AtomicBool;
use core::sync::atomic::{AtomicU8, Ordering};
#[cfg(not(miri))]
use flipperzero::furi::{message_queue::MessageQueue, println, time::FuriDuration};
#[cfg(feature = "xbm")]
use flipperzero::gui::xbm::{ByteArray, XbmImage};
use flipperzero::{
    gui::{
        Gui, GuiLayer,
        canvas::CanvasView,
        view_port::{ViewPort, ViewPortCallbacks},
    },
    input::{InputEvent, InputKey, InputType},
};
use flipperzero_rt::{entry, manifest};
#[cfg(miri)]
use flipperzero_sys as sys;
#[cfg(not(miri))]
use flipperzero_sys::furi::Error;

manifest!(name = "Rust GUI example");
entry!(main);

/// An image of an 8x8 plus.
///
/// It is important to note that byte bits are read in reverse order
/// but since this image is symmetric we don't need to reverse the bytes
/// unlike in [`RS_IMAGE`].
#[cfg(feature = "xbm")]
const PLUS_IMAGE: XbmImage<ByteArray<8>> = XbmImage::new_from_array::<8, 8>([
    0b00_11_11_00,
    0b00_11_11_00,
    0b11_11_11_11,
    0b11_11_11_11,
    0b11_11_11_11,
    0b11_11_11_11,
    0b00_11_11_00,
    0b10_11_11_01,
]);

/// An image of an 8x8 R and S letters.
#[cfg(feature = "xbm")]
const RS_IMAGE: XbmImage<ByteArray<8>> = XbmImage::new_from_array::<8, 8>([
    0b11100000u8.reverse_bits(),
    0b10010000u8.reverse_bits(),
    0b11100000u8.reverse_bits(),
    0b10100110u8.reverse_bits(),
    0b10011000u8.reverse_bits(),
    0b00000110u8.reverse_bits(),
    0b00000001u8.reverse_bits(),
    0b00000110u8.reverse_bits(),
]);

fn main(_args: Option<&CStr>) -> i32 {
    #[cfg(not(miri))]
    struct State<'a> {
        text: &'a CStr,
        exit_event_queue: &'a MessageQueue<()>,
        counter: AtomicU8,
    }
    #[cfg(miri)]
    struct State<'a> {
        text: &'a CStr,
        counter: AtomicU8,
        stop_signal: AtomicBool,
    }

    impl ViewPortCallbacks for State<'_> {
        #[cfg(feature = "xbm")]
        fn on_draw(&mut self, mut canvas: CanvasView) {
            canvas.draw_xbm(2, 2, &PLUS_IMAGE);
            canvas.draw_str(10, 31, self.text);
            let bottom_text =
                CString::new(self.counter.load(Ordering::Relaxed).to_string().as_bytes())
                    .expect("should be a valid string");
            canvas.draw_str(80, 10, bottom_text);
            canvas.draw_xbm(100, 50, &RS_IMAGE);
            canvas.draw_xbm(0, 32, &xbm_images::ferris::IMAGE);
        }
        #[cfg(not(feature = "xbm"))]
        fn on_draw(&mut self, mut canvas: CanvasView) {
            canvas.draw_str(10, 31, self.text);
            let bottom_text =
                CString::new(self.counter.load(Ordering::Relaxed).to_string().as_bytes())
                    .expect("should be a valid string");
            canvas.draw_str(80, 10, bottom_text);
        }

        fn on_input(&mut self, event: InputEvent) {
            if event.r#type == InputType::Press {
                match event.key {
                    InputKey::Up => {
                        let _ = self
                            .counter
                            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |counter| {
                                Some((counter + 1) % 10)
                            })
                            .expect("F always returns Some");
                    }
                    InputKey::Down => {
                        let _ = self
                            .counter
                            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |counter| {
                                Some(if counter == 0 { 10 } else { counter - 1 })
                            })
                            .expect("F always returns Some");
                    }
                    InputKey::Back => {
                        #[cfg(not(miri))]
                        self.exit_event_queue
                            .put((), FuriDuration::MAX)
                            .expect("failed to put event into the queue");
                        #[cfg(miri)]
                        self.stop_signal.store(true, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }
        }
    }

    #[cfg(not(miri))]
    let exit_event_queue = MessageQueue::new(32);

    #[cfg(not(miri))]
    let state = State {
        text: CStr::from_bytes_with_nul(b"Hi there!\0").expect("correct string"),
        exit_event_queue: &exit_event_queue,
        counter: AtomicU8::new(0),
    };
    #[cfg(miri)]
    let state = State {
        text: CStr::from_bytes_with_nul(b"Hi there!\0").expect("correct string"),
        counter: AtomicU8::new(0),
        stop_signal: AtomicBool::new(false),
    };

    let view_port = ViewPort::new(state);

    let mut gui = Gui::open();

    #[cfg(miri)]
    let miri_gui = {
        let view_port_gui: Arc<sys::Gui> = unsafe { Arc::from_raw(gui.as_ptr()) };
        let miri_gui = view_port_gui.clone();
        let _ = Arc::into_raw(view_port_gui);
        miri_gui
    };

    let view_port = gui.add_view_port(view_port, GuiLayer::Fullscreen);

    #[cfg(not(miri))]
    let status = run_until_exit(&exit_event_queue);
    #[cfg(miri)]
    let status = run_until_exit_miri(miri_gui);

    drop(view_port);

    status
}

#[cfg(not(miri))]
fn run_until_exit(exit_event_queue: &MessageQueue<()>) -> i32 {
    loop {
        match exit_event_queue.get(FuriDuration::from_millis(100)) {
            Ok(()) => {
                println!("Exit pressed");
                break 0;
            }
            Err(Error::TimedOut) => {} // it's okay to continue polling
            Err(e) => {
                println!("ERROR while receiving event: {:?}", e);
                break 1;
            }
        }
    }
}

#[cfg(miri)]
fn run_until_exit_miri(gui: Arc<sys::Gui>) -> i32 {
    use flipperzero::input::miri::send;

    send!(Ok event to gui); // no behaviour bound
    send!(Down event to gui); // increment count
    send!(Down event to gui); // increment count
    send!(Back event to gui); // leave

    0
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    // TODO: Is there any benefit to Miri in hooking up the binary arguments to
    // the test runner?
    main(None).try_into().unwrap_or(isize::MAX)
}
