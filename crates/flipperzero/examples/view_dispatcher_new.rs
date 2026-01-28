//! Demonstrates use of the ViewDispatcher module.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;
use flipperzero::gui::{
    Gui,
    view::{View, ViewCallbacks},
    view_dispatcher::{
        StopDispatcher, ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherRef,
        ViewDispatcherType, ViewDispatcherView,
    },
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

manifest!(name = "Rust ViewDispatcher example");
entry!(main);

struct TextInput {
    raw: NonNull<sys::TextInput>,
}

impl TextInput {
    fn new() -> Self {
        let raw = unsafe { sys::text_input_alloc() };
        let raw = unsafe { NonNull::new_unchecked(raw) };
        TextInput { raw }
    }

    fn view(&self) -> View<()> {
        let view_ptr = unsafe { sys::text_input_get_view(self.raw.as_ptr()) };
        unsafe { View::new_from_raw(view_ptr) }
    }
}

struct Counter {
    view: View<CounterCallback>,
}

impl Counter {
    fn new() -> Self {
        let callbacks = CounterCallback;
        let view = View::new(callbacks);
        Counter { view }
    }
}

struct CounterCallback;

impl ViewCallbacks for CounterCallback {
    fn on_draw(&mut self, canvas: flipperzero::gui::canvas::CanvasView) {
        todo!()
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    struct State<'a> {
        text_input_view: Option<ViewDispatcherView<'a, ()>>,
        counter_view: Option<ViewDispatcherView<'a, CounterCallback>>,
    }

    impl ViewDispatcherCallbacks for State<'_> {
    }

    let mut state = State {
        text_input_view: None,
        counter_view: None,
    };
    let gui = Gui::open();

    let mut view_dispatcher = ViewDispatcher::new(state, &gui, ViewDispatcherType::Fullscreen);

    let text_input = TextInput::new();
    let Ok(text_input_view) = view_dispatcher.add_view(0, text_input.view()) else {
        unreachable!()
    };
    let _ = view_dispatcher.get_context_mut().text_input_view.insert(text_input_view);

    let counter = Counter::new();
    let Ok(counter_view) = view_dispatcher.add_view(1, counter.view) else {
        unreachable!()
    };
    let _ = view_dispatcher.get_context_mut().counter_view.insert(counter_view);

    view_dispatcher.run();

    0
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    // TODO: Is there any benefit to Miri in hooking up the binary arguments to
    // the test runner?
    let ret = main(None);

    ret.try_into().unwrap_or(isize::MAX)
}
