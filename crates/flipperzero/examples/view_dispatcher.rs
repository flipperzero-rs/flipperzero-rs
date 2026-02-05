//! Demonstrates use of the ViewDispatcher module.

#![no_main]
#![no_std]

extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

#[cfg(miri)]
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::CStr;
use flipperzero::gui::canvas::CanvasView;
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
use flipperzero_sys as sys;

manifest!(name = "Rust ViewDispatcher example");
entry!(main);

#[cfg(miri)]
unsafe extern "Rust" {
    pub fn miri_thread_spawn(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;
    pub fn miri_thread_join(thread_id: usize) -> bool;
    pub fn miri_set_thread_name(thread_id: usize, name: *const u8) -> bool;
    pub safe fn miri_write_to_stdout(bytes: &[u8]);
}

#[cfg(not(miri))]
fn miri_write_to_stdout(_bytes: &[u8]) {}

struct State {}

impl ViewDispatcherCallbacks for State {
    type BindCustom = DontBind;
    type BindTick = DontBind;

    fn on_navigation<T>(&self, view_dispatcher: &ViewDispatcherInner<T>) -> StopDispatcher
    where
        T: ViewDispatcherCallbacks,
    {
        StopDispatcher::Yes
    }
}

struct Counter<'a> {
    view: View<CounterCallback<'a>>,
}

impl<'a> Counter<'a> {
    fn new(state: ViewDispatcherRef<'a, State>) -> Self {
        let callbacks = CounterCallback { counter: 0, state };
        let view = View::new(callbacks);
        Counter { view }
    }
}

struct CounterCallback<'a> {
    counter: u8,
    state: ViewDispatcherRef<'a, State>,
}

impl ViewCallbacks for CounterCallback<'_> {
    fn on_draw(&mut self, canvas: CanvasView) {}

    fn on_input(&mut self, event: InputEvent) -> EventBubbling {
        match event.key {
            InputKey::Up => {
                self.counter += 1;
                if self.counter > 10 {
                    self.counter = 0;
                }

                miri_write_to_stdout(b"Counter up\n");

                EventBubbling::Consumed
            }
            InputKey::Down => {
                if self.counter == 0 {
                    self.counter = 10;
                } else {
                    self.counter -= 1;
                }

                miri_write_to_stdout(b"Counter down\n");

                EventBubbling::Consumed
            }
            InputKey::Right => {
                miri_write_to_stdout(b"Counter right\n");

                self.state.switch_to_view(1);

                EventBubbling::Consumed
            }
            InputKey::Left => todo!(),
            InputKey::Ok => {
                self.counter = 0;

                miri_write_to_stdout(b"Counter OK\n");

                EventBubbling::Consumed
            }
            InputKey::Back => {
                if self.counter == 0 {
                    miri_write_to_stdout(b"Counter back when counter was 0\n");
                    EventBubbling::ReturnForAdditionalProcessing
                } else {
                    miri_write_to_stdout(b"Counter back when counter was not 0\n");
                    EventBubbling::Consumed
                }
            }
        }
    }

    fn on_back_event(&mut self) -> Option<u32> {
        miri_write_to_stdout(b"Getting view that should be returned to from the counter view\n");
        None
    }
}

struct MazeGridVertex<'a> {
    view: View<MazeCallbacks<'a>>,
}

impl<'a> MazeGridVertex<'a> {
    fn new(state: ViewDispatcherRef<'a, State>) -> Self {
        let callbacks = MazeCallbacks {
            stack: Vec::new(),
            state,
        };
        let view = View::new(callbacks);
        MazeGridVertex { view }
    }
}

struct MazeCallbacks<'a> {
    stack: Vec<flipperzero::input::InputKey>,
    state: ViewDispatcherRef<'a, State>,
}

impl ViewCallbacks for MazeCallbacks<'_> {
    fn on_input(&mut self, event: InputEvent) -> EventBubbling {
        if event.r#type == InputType::Short {
            match event.key {
                InputKey::Up => todo!(),
                InputKey::Down => todo!(),
                InputKey::Right => todo!(),
                InputKey::Left => todo!(),
                InputKey::Ok => todo!(),
                InputKey::Back => {
                    if self.stack.pop().is_some() {
                        EventBubbling::Consumed
                    } else {
                        EventBubbling::ReturnForAdditionalProcessing
                    }
                }
            }
        } else if event.r#type == InputType::Long && event.key == InputKey::Back {
            self.stack.clear();
            EventBubbling::Consumed
        } else {
            EventBubbling::ReturnForAdditionalProcessing
        }
    }

    fn on_back_event(&mut self) -> Option<u32> {
        Some(0)
    }

    fn on_draw(&mut self, canvas: CanvasView) {
        todo!()
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    let gui = Gui::open();

    #[cfg(miri)]
    let miri_gui = {
        let view_port_gui: Arc<sys::Gui> = unsafe { Arc::from_raw(gui.as_ptr()) };
        let miri_gui = view_port_gui.clone();
        let _ = Arc::into_raw(view_port_gui);
        miri_gui
    };

    let mut view_dispatcher = ViewDispatcher::new(State {}, &gui, ViewDispatcherType::Fullscreen);

    let counter = Counter::new(view_dispatcher.get_ref());
    let Ok(counter_view) = view_dispatcher.add_view(0, counter.view) else {
        unreachable!()
    };

    counter_view.switch_to_view();

    let maze = MazeGridVertex::new(view_dispatcher.get_ref());
    let Ok(_maze_view) = view_dispatcher.add_view(1, maze.view) else {
        unreachable!()
    };

    #[cfg(not(miri))]
    let status = run_until_exit(view_dispatcher);
    #[cfg(miri)]
    let status = run_until_exit_miri(view_dispatcher, miri_gui);

    status
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
