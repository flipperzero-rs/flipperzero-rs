//! Demonstrates use of the Variable Item List module.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

#[cfg(miri)]
use alloc::sync::Arc;
use core::ffi::CStr;
use core::sync::atomic::{AtomicI8, Ordering};
use flipperzero::gui::variable_item_list::{
    Callback, OnCurrentValueTextChangedCallbacks, VariableItem, VariableItemList,
};
use flipperzero::gui::view_dispatcher::{DontBind, ViewDispatcherInner};
use flipperzero::gui::{
    Gui,
    view_dispatcher::{
        StopDispatcher, ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherType,
    },
};
use flipperzero::{format, prelude::FuriString};
use flipperzero_rt::{entry, manifest};
#[cfg(miri)]
use flipperzero::gui::variable_item_list::{self, UniqueCallbackForEachItem, VariableItemListBoundToViewDispatcher};

manifest!(name = "Rust Variable Item List example");
entry!(main);

#[cfg(miri)]
unsafe extern "Rust" {
    pub fn miri_thread_spawn(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;
    pub fn miri_thread_join(thread_id: usize) -> bool;
    pub fn miri_set_thread_name(thread_id: usize, name: *const u8) -> bool;
    pub safe fn miri_write_to_stdout(bytes: &[u8]);
}

struct State {}

impl ViewDispatcherCallbacks for State {
    type BindCustom = DontBind;
    type BindTick = DontBind;

    fn on_navigation<T>(&self, _view_dispatcher: &ViewDispatcherInner<T>) -> StopDispatcher
    where
        T: ViewDispatcherCallbacks,
    {
        StopDispatcher::Yes
    }
}

struct IncrementGlobalCounterCallback<'a> {
    counter: &'a AtomicI8,
    increment_by: i8,
}

impl Callback for IncrementGlobalCounterCallback<'_> {
    fn on_click(&self, _item: &VariableItem) -> () {
        self.counter.fetch_add(self.increment_by, Ordering::Relaxed);
    }
}

struct IncrementGlobalCounterByVariableCallback<'a> {
    counter: &'a AtomicI8,
    increment_by: &'a AtomicI8,
}

struct ChangeIncrementAmountCallback<'a> {
    number_of_options: u8,
    min_value: i8,
    increment_amount: &'a AtomicI8,
}

impl Callback for IncrementGlobalCounterByVariableCallback<'_> {
    fn on_click(&self, _item: &VariableItem) -> () {
        self.counter
            .fetch_add(self.increment_by.load(Ordering::Relaxed), Ordering::Relaxed);
    }
}

impl OnCurrentValueTextChangedCallbacks for ChangeIncrementAmountCallback<'_> {
    fn get_new_label(&self, _item: &VariableItem, value: u8) -> flipperzero::prelude::FuriString {
        let val: i8 = (self.number_of_options - value) as i8 + self.min_value;
        self.increment_amount.store(val, Ordering::Relaxed);

        FuriString::from(format!("{}", val))
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    let gui = Gui::open();

    #[cfg(miri)]
    let miri_gui = {
        let view_port_gui: Arc<flipperzero_sys::Gui> = unsafe { Arc::from_raw(gui.as_ptr()) };
        let miri_gui = view_port_gui.clone();
        let _ = Arc::into_raw(view_port_gui);
        miri_gui
    };

    let mut view_dispatcher = ViewDispatcher::new(State {}, &gui, ViewDispatcherType::Fullscreen);

    let counter = AtomicI8::new(0);

    let mut variable_item_list = VariableItemList::new();
    variable_item_list.push_item_plaintext(c"First Item".into());
    variable_item_list.push_item_with_on_click_callback(
        "Add two".into(),
        IncrementGlobalCounterCallback {
            counter: &counter,
            increment_by: 2,
        },
    );
    variable_item_list.push_item_with_on_click_callback(
        "Add three".into(),
        IncrementGlobalCounterCallback {
            counter: &counter,
            increment_by: 3,
        },
    );
    variable_item_list.push_item_with_on_click_callback(
        "Subtract one".into(),
        IncrementGlobalCounterCallback {
            counter: &counter,
            increment_by: -1,
        },
    );

    let increment_amount = AtomicI8::new(0);
    let change_counter_callback = IncrementGlobalCounterByVariableCallback {
        counter: &counter,
        increment_by: &increment_amount,
    };

    let number_of_options = 6;

    let modify_increment_callback = ChangeIncrementAmountCallback {
        number_of_options: number_of_options.clone(),
        min_value: -2,
        increment_amount: &increment_amount,
    };

    variable_item_list
        .push_item_with_on_click_callback("Add variable amount".into(), change_counter_callback);
    variable_item_list.push_item_with_options(
        "Variable amount to add".into(),
        6,
        modify_increment_callback,
    );

    let variable_item_list_view =
        variable_item_list.bind_to_view_dispatcher(0, &mut view_dispatcher);

    variable_item_list_view.switch_to_view();

    #[cfg(not(miri))]
    let status = run_until_exit(view_dispatcher);
    #[cfg(miri)]
    let status = run_until_exit_miri(view_dispatcher, variable_item_list_view, miri_gui);

    status
}

#[cfg(not(miri))]
fn run_until_exit(view_dispatcher: ViewDispatcher<'_, State>) -> i32 {
    view_dispatcher.run();

    0
}

#[cfg(miri)]
fn run_until_exit_miri(
    view_dispatcher: ViewDispatcher<'_, State>,
    variable_item_list_view: VariableItemListBoundToViewDispatcher<'_, '_, State, UniqueCallbackForEachItem<'_>>,
    gui: Arc<flipperzero_sys::Gui>,
) -> i32 {
    use alloc::sync::Arc;

    let thread_id = {
        // SAFETY: Arc was generated above
        unsafe { miri_thread_spawn(send_events_for_miri, Arc::into_raw(gui) as *mut _) }
    };

    unsafe { miri_set_thread_name(thread_id, c"miri event sender".as_ptr()) };

    let view_dispatcher = view_dispatcher.run();
    miri_write_to_stdout(b"View Dispatcher returned from run\n");

    assert_eq!(Arc::strong_count(&view_dispatcher.0), 2, "Before drop");

    miri_write_to_stdout(b"Attempting to drop variable item list view\n");
    drop(variable_item_list_view);

    assert_eq!(Arc::strong_count(&view_dispatcher.0), 1, "After drop");
    miri_write_to_stdout(b"Dropping view dispatcher\n");

    drop(view_dispatcher);

    unsafe { miri_thread_join(thread_id) };

    0
}

#[cfg(miri)]
extern "Rust" fn send_events_for_miri(data: *mut ()) {
    use flipperzero::input::{InputEvent, InputKey, InputType};
    use flipperzero_sys as sys;

    let gui: Arc<sys::Gui> = unsafe { Arc::from_raw(data as *const _) };

    {
        let mut gui = gui.lock(b"send input event 1");
        let input_event = InputEvent {
            sequence: 0.into(),
            key: InputKey::Up,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"*** Up event 0\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock(b"send input event 2");
        let input_event = InputEvent {
            sequence: 1.into(),
            key: InputKey::Back,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"*** Back event 1\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock(b"send input event 3");
        let input_event = InputEvent {
            sequence: 2.into(),
            key: InputKey::Down,
            r#type: InputType::Short,
        };
        miri_write_to_stdout(b"*** Down event 2\n");
        sys::GuiInner::send_input_event(&mut gui, input_event.into());
    }

    {
        let mut gui = gui.lock(b"send input event 4");
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
