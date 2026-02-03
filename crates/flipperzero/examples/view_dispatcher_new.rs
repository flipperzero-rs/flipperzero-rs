//! Demonstrates use of the ViewDispatcher module.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

#[cfg(miri)]
use alloc::sync::Arc;
use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;
use flipperzero::gui::view_dispatcher::DontBind;
use flipperzero::gui::{
    Gui,
    view::{View, ViewCallbacks},
    view_dispatcher::{
        StopDispatcher, ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherRef,
        ViewDispatcherType, ViewDispatcherView,
    },
};
#[cfg(miri)]
use flipperzero::input::{InputEvent, InputKey, InputType};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

manifest!(name = "Rust ViewDispatcher example");
entry!(main);

#[cfg(miri)]
unsafe extern "Rust" {
    pub fn miri_thread_spawn(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;
    pub fn miri_thread_join(thread_id: usize) -> bool;
    pub fn miri_set_thread_name(thread_id: usize, name: *const u8) -> bool;
}

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
        text_input_view: Option<ViewDispatcherView<'a, (), State<'a>>>,
        counter_view: Option<ViewDispatcherView<'a, CounterCallback, State<'a>>>,
    }

    impl ViewDispatcherCallbacks for State<'_> {
        type BindCustom = DontBind;
        type BindNavigation = DontBind;
        type BindTick = DontBind;
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
    let text_input_view = text_input_view;
    let _ = view_dispatcher
        .get_context_mut()
        .text_input_view
        .insert(text_input_view);

    let counter = Counter::new();
    let Ok(counter_view) = view_dispatcher.add_view(1, counter.view) else {
        unreachable!()
    };
    let counter_view = counter_view;
    let _ = view_dispatcher
        .get_context_mut()
        .counter_view
        .insert(counter_view);

    #[cfg(not(miri))]
    let status = run_until_exit(view_dispatcher);
    #[cfg(miri)]
    let status = run_until_exit_miri(view_dispatcher, gui);

    0
}

#[cfg(not(miri))]
fn run_until_exit(view_dispatcher: ViewDispatcher<'_, State<'_>>) -> i32 {
    view_dispatcher.run();

    0
}

#[cfg(miri)]
fn run_until_exit_miri(view_dispatcher: &ViewDispatcher<'_, State<'_>>, gui: Arc<sys::Gui>) -> i32 {
    let thread_id = {
        let gui_ptr = Arc::into_raw(gui.clone());
        // SAFETY: Arc was generated above
        unsafe { miri_thread_spawn(send_events_for_miri, gui_ptr as *mut _) }
    };

    unsafe { miri_set_thread_name(thread_id, c"miri event sender".as_ptr()) };

    view_dispatcher.run();

    unsafe { miri_thread_join(thread_id) };

    0
}

#[cfg(miri)]
extern "Rust" fn send_events_for_miri(data: *mut ()) {
    let gui: Arc<sys::Gui> = unsafe { Arc::from_raw(data as *const _) };

    {
        let mut gui = gui.lock();
        let input_event = InputEvent {
            sequence: 0.into(),
            key: InputKey::Up,
            r#type: InputType::Press,
        };
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    // TODO: Is there any benefit to Miri in hooking up the binary arguments to
    // the test runner?
    let ret = main(None);

    ret.try_into().unwrap_or(isize::MAX)
}
