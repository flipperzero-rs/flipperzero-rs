//! Demonstrates use of the Variable Item List module.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

use alloc::boxed::Box;
#[cfg(miri)]
use alloc::sync::Arc;
use core::ffi::CStr;
use core::sync::atomic::{AtomicI8, Ordering};
use flipperzero::gui::canvas::CanvasView;
use flipperzero::gui::variable_item_list::{Callback, VariableItem, VariableItemList};
use flipperzero::gui::view::EventBubbling;
use flipperzero::gui::view_dispatcher::{DontBind, ViewDispatcherInner};
use flipperzero::gui::{
    Gui,
    view::{View, ViewCallbacks},
    view_dispatcher::{
        StopDispatcher, ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherRef,
        ViewDispatcherType,
    },
};
use flipperzero::input::{InputEvent, InputKey, InputType};
use flipperzero_rt::{entry, manifest};

manifest!(name = "Rust Variable Item List example");
entry!(main);

struct State {}

impl ViewDispatcherCallbacks for State {}

struct IncrementGlobalCounterCallback<'a> {
    counter: &'a AtomicI8,
    increment_by: i8,
}

impl Callback for IncrementGlobalCounterCallback<'_> {
    fn on_click(&self, item: &VariableItem) -> () {
        self.counter.fetch_add(self.increment_by, Ordering::SeqCst);
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    let gui = Gui::open();

    let mut view_dispatcher = ViewDispatcher::new(State {}, &gui, ViewDispatcherType::Fullscreen);

    let counter = AtomicI8::new(0);

    let mut variable_item_list = VariableItemList::new();
    variable_item_list.push_item_plaintext("First Item".into());
    variable_item_list.push_item_with_on_click_callback(
        "Add two".into(),
        Box::new(IncrementGlobalCounterCallback { counter: &counter, increment_by: 2 }),
    );
    variable_item_list.push_item_with_on_click_callback(
        "Add three".into(),
        Box::new(IncrementGlobalCounterCallback { counter: &counter, increment_by: 3 }),
    );
    variable_item_list.push_item_with_on_click_callback(
        "Subtract one".into(),
        Box::new(IncrementGlobalCounterCallback { counter: &counter, increment_by: -1 }),
    );

    let variable_item_list_view =
        variable_item_list.bind_to_view_dispatcher(0, &mut view_dispatcher);

    variable_item_list_view.switch_to_view();

    #[cfg(not(miri))]
    let status = run_until_exit(view_dispatcher);
    #[cfg(miri)]
    let status = run_until_exit_miri(view_dispatcher, miri_gui);

    0
}

#[cfg(not(miri))]
fn run_until_exit(view_dispatcher: ViewDispatcher<'_, State>) -> i32 {
    view_dispatcher.run();

    0
}

#[cfg(miri)]
fn run_until_exit_miri(view_dispatcher: ViewDispatcher<'_, State>, gui: Arc<sys::Gui>) -> i32 {
    assert_eq!(
        Arc::strong_count(&view_dispatcher.0),
        3,
        "(before run) [ViewDispatcher, state (via CounterViewRef), state (via MazeViewRef)]]"
    );

    let thread_id = {
        // SAFETY: Arc was generated above
        unsafe { miri_thread_spawn(send_events_for_miri, Arc::into_raw(gui) as *mut _) }
    };

    unsafe { miri_set_thread_name(thread_id, c"miri event sender".as_ptr()) };

    let view_dispatcher = view_dispatcher.run();

    unsafe { miri_thread_join(thread_id) };
    assert_eq!(
        Arc::strong_count(&view_dispatcher.0),
        3,
        "(after run) [ViewDispatcher, state (via CounterViewRef), state (via MazeViewRef)]]"
    );

    drop(view_dispatcher);

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
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"Up event 0\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock();
        let input_event = InputEvent {
            sequence: 1.into(),
            key: InputKey::Back,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"Back event 1\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock();
        let input_event = InputEvent {
            sequence: 2.into(),
            key: InputKey::Down,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"Down event 2\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock();
        let input_event = InputEvent {
            sequence: 3.into(),
            key: InputKey::Back,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"Back event 3\n");
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
