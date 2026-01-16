mod canvas;
mod icon;
mod text_input;
mod variable_item_list;
mod view_dispatcher;
mod view_port;
mod widget;

extern crate alloc;

pub use canvas::*;
pub use icon::*;
pub use text_input::*;
pub use variable_item_list::*;
pub use view_dispatcher::*;
pub use view_port::*;
pub use widget::*;

use crate::InputEvent;
use crate::miri_bindings::utils::*;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

#[doc = "Set lockdown mode\n\n When lockdown mode is enabled, only GuiLayerDesktop is shown.\n This feature prevents services from showing sensitive information when flipper is locked.\n\n # Arguments\n\n* `gui` - Gui instance\n * `lockdown` - bool, true if enabled"]
pub unsafe fn gui_set_lockdown(gui: *mut Gui, lockdown: bool) {
    todo!()
}
#[doc = "Acquire Direct Draw lock and get Canvas instance\n\n This method return Canvas instance for use in monopoly mode. Direct draw lock\n disables input and draw call dispatch functions in GUI service. No other\n applications or services will be able to draw until gui_direct_draw_release\n call.\n\n # Arguments\n\n* `gui` - The graphical user interface\n\n # Returns\n\nCanvas instance"]
pub unsafe fn gui_direct_draw_acquire(gui: *mut Gui) -> *mut Canvas {
    todo!()
}
#[doc = "Release Direct Draw Lock\n\n Release Direct Draw Lock, enables Input and Draw call processing. Canvas\n acquired in gui_direct_draw_acquire will become invalid after this call.\n\n # Arguments\n\n* `gui` - Gui instance"]
pub unsafe fn gui_direct_draw_release(gui: *mut Gui) {
    todo!()
}
#[doc = "Get gui canvas frame buffer size\n *\n # Arguments\n\n* `gui` - Gui instance\n # Returns\n\nsize_t size of frame buffer in bytes"]
pub unsafe fn gui_get_framebuffer_size(gui: *const Gui) -> usize {
    todo!()
}
#[doc = "< Desktop layer for internal use. Like fullscreen but with status bar"]
pub const GuiLayerDesktop: GuiLayer = GuiLayer(0);
#[doc = "< Window layer, status bar is shown"]
pub const GuiLayerWindow: GuiLayer = GuiLayer(1);
#[doc = "< Status bar left-side layer, auto-layout"]
pub const GuiLayerStatusBarLeft: GuiLayer = GuiLayer(2);
#[doc = "< Status bar right-side layer, auto-layout"]
pub const GuiLayerStatusBarRight: GuiLayer = GuiLayer(3);
#[doc = "< Fullscreen layer, no status bar"]
pub const GuiLayerFullscreen: GuiLayer = GuiLayer(4);
#[doc = "< Don't use or move, special value"]
pub const GuiLayerMAX: GuiLayer = GuiLayer(5);
#[repr(transparent)]
#[doc = "Gui layers"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GuiLayer(pub core::ffi::c_uchar);

mod lock {
    use crate::miri_bindings::utils::*;
    use core::cell::UnsafeCell;
    use core::ops::{Deref, DerefMut};
    use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

    pub struct SpinLock<T> {
        data: UnsafeCell<T>,
        inner: AtomicBool,
    }

    pub struct SpinLockGuard<'a, T> {
        lock: &'a SpinLock<T>,
    }

    impl<T> SpinLock<T> {
        pub fn new(data: T) -> Self {
            Self {
                data: data.into(),
                inner: AtomicBool::new(false),
            }
        }

        pub fn lock(&self) -> SpinLockGuard<'_, T> {
            // NOTE: SeqCst has been used all over here, bcs it's definitely correct, and I haven't got
            // a good enough handle on the other orderings to pick one that would also be correct but
            // more efficient.
            while !self
                .inner
                .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                miri_spin_loop();
            }
            SpinLockGuard { lock: self }
        }
    }

    impl<'a, T> Deref for SpinLockGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { &mut *self.lock.data.get() }
        }
    }

    impl<'a, T> Drop for SpinLockGuard<'a, T> {
        fn drop(&mut self) {
            // NOTE: SeqCst has been used all over here, bcs it's definitely correct, and I haven't got
            // a good enough handle on the other orderings to pick one that would also be correct but
            // more efficient.
            self.lock.inner.store(false, Ordering::SeqCst);
        }
    }
}

#[repr(C)]
pub struct Gui {
    thread_id: usize,
    input_channel: Option<InputEvent>,
    request_redraw: bool,
    stop: bool,
}

impl Gui {
    // This isn't _entirely_ correct to the source; in that, the GUI record is created and
    // populated by the GUI svc thread, not the other way around.
    pub fn spawn() -> Arc<lock::SpinLock<Self>> {
        let gui = Self {
            thread_id: 0,
            input_channel: None,
            request_redraw: false,
            stop: false,
        };
        let mut gui = Arc::new(lock::SpinLock::new(gui));

        let thread_id = {
            let gui = gui.clone();
            let gui_ptr = Arc::into_raw(gui);
            // SAFETY: Arc was generated above
            unsafe { miri_thread_spawn(thread_start, gui_ptr as *mut _) }
        };

        {
            gui.lock().thread_id = thread_id;
        }

        extern "Rust" fn thread_start(data: *mut ()) {
            // SAFETY: data is guaranteed to have been created from an arc, just above
            let gui: Arc<lock::SpinLock<Gui>> = unsafe { Arc::from_raw(data as *const _) };

            loop {
                let gui = &mut gui.lock();

                if let Some(input) = gui.input_channel.take() {
                    gui.process_input(input);
                }

                if gui.request_redraw {
                    gui.redraw();
                }

                if gui.stop {
                    break;
                }

                miri_spin_loop();
            }
        }

        gui
    }

    fn process_input(&self, input: InputEvent) -> () {
        todo!();
    }

    fn redraw(&self) -> () {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct View {
    _unused: [u8; 0],
}
#[doc = "Add view_port to view_port tree\n\n > thread safe\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance\n * `layer` (direction in) - GuiLayer where to place view_port"]
pub fn gui_add_view_port(gui: *mut Gui, view_port: *mut ViewPort, layer: GuiLayer) {
    todo!()
}
#[doc = "Remove view_port from rendering tree\n\n > thread safe\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance"]
pub fn gui_remove_view_port(gui: *mut Gui, view_port: *mut ViewPort) {
    todo!()
}
#[doc = "Send ViewPort to the front\n\n Places selected ViewPort to the top of the drawing stack\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance"]
pub fn gui_view_port_send_to_front(gui: *mut Gui, view_port: *mut ViewPort) {
    todo!()
}
pub const GuiButtonTypeLeft: GuiButtonType = GuiButtonType(0);
pub const GuiButtonTypeCenter: GuiButtonType = GuiButtonType(1);
pub const GuiButtonTypeRight: GuiButtonType = GuiButtonType(2);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GuiButtonType(pub core::ffi::c_uchar);
pub type ButtonCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: GuiButtonType,
        type_: crate::InputType,
        context: *mut core::ffi::c_void,
    ),
>;
