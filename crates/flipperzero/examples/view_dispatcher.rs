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
use flipperzero::furi::log::Level;
use flipperzero::furi::time::FuriInstant;
use flipperzero::gui::Gui;
use flipperzero::gui::canvas::CanvasView;
use flipperzero::gui::submenu::{Submenu, SubmenuCustomItem};
use flipperzero::gui::view::{EventBubbling, View, ViewCallbacks};
use flipperzero::gui::view_dispatcher::{
    DontBind, StopDispatcher, ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherInner,
    ViewDispatcherRef, ViewDispatcherType,
};
use flipperzero::input::{InputEvent, InputKey, InputType};
use flipperzero::log;
use flipperzero_rt::{entry, manifest};
#[cfg(miri)]
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
        Some(0)
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

struct DurationCallback {
    event_start_time: Option<FuriInstant>,
}

impl SubmenuCustomItem for DurationCallback {
    fn handle_input_event(&mut self, input_type: InputType) -> () {
        match input_type {
            InputType::Press => self.event_start_time = Some(FuriInstant::now()),
            InputType::Release => {
                let start_time = self
                    .event_start_time
                    .take()
                    .expect("Release must have been proceeded by a press");

                let elapsed_time = start_time.elapsed();
                log!(
                    Level::INFO,
                    "OK press lasted for {}.{}",
                    elapsed_time.as_secs(),
                    elapsed_time.as_millis()
                );
            }
            _ => {}
        }
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

    let submenu = Submenu::new();
    let mut submenu = submenu.bind_to_view_dispatcher(0, &mut view_dispatcher);

    submenu.switch_to_view();

    let counter = Counter::new(view_dispatcher.get_ref());
    let Ok(counter_view) = view_dispatcher.add_view(1, counter.view) else {
        unreachable!()
    };

    let maze = MazeGridVertex::new(view_dispatcher.get_ref());
    let Ok(maze_view) = view_dispatcher.add_view(2, maze.view) else {
        unreachable!()
    };

    let _submenu_counter_item = submenu.add_nav_item(c"Counter", &counter_view);
    let _submenu_maze_item = submenu.add_nav_item(c"Maze", &maze_view);
    let _submenu_plain_item = submenu.add_plaintext_item(c"Plaintext");
    let _submenu_duration_item = submenu.add_custom_item(
        c"Duration",
        &mut DurationCallback {
            event_start_time: None,
        },
    );

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
        4,
        "(before run) [ViewDispatcher, Submenu, state (via CounterViewRef), state (via MazeViewRef)]]"
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
        4,
        "(after run) [ViewDispatcher, Submenu, state (via CounterViewRef), state (via MazeViewRef)]]"
    );

    drop(view_dispatcher);

    0
}

#[cfg(miri)]
extern "Rust" fn send_events_for_miri(data: *mut ()) {
    use flipperzero::input::miri::send;
    let gui: Arc<sys::Gui> = unsafe { Arc::from_raw(data as *const _) };

    send!(Ok event to gui); // enter the counter view

    send!(Up event to gui); // counter increase
    send!(Back event to gui); // reset counter
    send!(Down event to gui 2 times); // counter decrease
    send!(Back event to gui); // exit back to submenu
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize {
    // TODO: Is there any benefit to Miri in hooking up the binary arguments to
    // the test runner?
    let ret = main(None);

    ret.try_into().unwrap_or(isize::MAX)
}
