//! Demonstrates use of the ViewDispatcher module.
//!
//! This app prompts the user for a name then says hello.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

use alloc::boxed::Box;
use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;
use flipperzero::furi::string::FuriString;
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

manifest!(name = "Rust ViewDispatcher example");
entry!(main);

enum AppView {
    Widget = 0,
    TextInput = 1,
}

struct App {
    name: [c_char; 16],
    view_dispatcher: NonNull<sys::ViewDispatcher>,
    widget: NonNull<sys::Widget>,
    text_input: NonNull<sys::TextInput>,
}

impl App {
    pub fn new() -> Box<Self> {
        Box::new(App {
            name: Default::default(),
            view_dispatcher: unsafe { NonNull::new_unchecked(sys::view_dispatcher_alloc()) },
            widget: unsafe { NonNull::new_unchecked(sys::widget_alloc()) },
            text_input: unsafe { NonNull::new_unchecked(sys::text_input_alloc()) },
        })
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            sys::view_dispatcher_free(self.view_dispatcher.as_ptr());
            sys::widget_free(self.widget.as_ptr());
            sys::text_input_free(self.text_input.as_ptr());
        }
    }
}

pub unsafe extern "C" fn text_input_callback(context: *mut c_void) {
    let app = context as *mut App;
    let mut message = FuriString::from("Hello ");
    unsafe {
        message.push_c_str(CStr::from_ptr((*app).name.as_ptr()));
        sys::widget_add_string_element(
            (*app).widget.as_ptr(),
            128 / 2,
            64 / 2,
            sys::AlignCenter,
            sys::AlignCenter,
            sys::FontPrimary,
            message.as_c_ptr(),
        );
        sys::view_dispatcher_switch_to_view(
            (*app).view_dispatcher.as_ptr(),
            AppView::Widget as u32,
        );
    }
}

pub unsafe extern "C" fn navigation_event_callback(context: *mut c_void) -> bool {
    let view_dispatcher = context as *mut sys::ViewDispatcher;
    unsafe {
        sys::view_dispatcher_stop(view_dispatcher);
        sys::view_dispatcher_remove_view(view_dispatcher, AppView::Widget as u32);
        sys::view_dispatcher_remove_view(view_dispatcher, AppView::TextInput as u32);
    }

    true
}

fn main(_args: Option<&CStr>) -> i32 {
    let mut app = App::new();

    unsafe {
        sys::view_dispatcher_enable_queue(app.view_dispatcher.as_ptr());
        sys::view_dispatcher_set_event_callback_context(
            app.view_dispatcher.as_ptr(),
            app.view_dispatcher.as_ptr() as *mut c_void,
        );
        sys::view_dispatcher_set_navigation_event_callback(
            app.view_dispatcher.as_ptr(),
            Some(navigation_event_callback),
        );
        sys::view_dispatcher_add_view(
            app.view_dispatcher.as_ptr(),
            AppView::Widget as u32,
            sys::widget_get_view(app.widget.as_ptr()),
        );
        sys::view_dispatcher_add_view(
            app.view_dispatcher.as_ptr(),
            AppView::TextInput as u32,
            sys::text_input_get_view(app.text_input.as_ptr()),
        );
    }

    unsafe {
        let gui = UnsafeRecord::open(c"gui");
        sys::view_dispatcher_attach_to_gui(
            app.view_dispatcher.as_ptr(),
            gui.as_ptr(),
            sys::ViewDispatcherTypeFullscreen,
        );

        sys::text_input_reset(app.text_input.as_ptr());
        sys::text_input_set_header_text(app.text_input.as_ptr(), c"Enter your name".as_ptr());

        sys::text_input_set_result_callback(
            app.text_input.as_ptr(),
            Some(text_input_callback),
            &*app as *const App as *mut c_void,
            app.name.as_mut_ptr(),
            app.name.len(),
            true,
        );

        sys::view_dispatcher_switch_to_view(
            app.view_dispatcher.as_ptr(),
            AppView::TextInput as u32,
        );
        sys::view_dispatcher_run(app.view_dispatcher.as_ptr());
    }

    0
}
