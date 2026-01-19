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

pub use gui_inner::GuiInner;

use crate::miri_bindings::lock::SpinLock;
use alloc::sync::Arc;
use core::ptr::NonNull;

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
pub type Gui = SpinLock<gui_inner::GuiInner>;

pub(crate) mod gui_inner {
    extern crate alloc;

    use super::canvas::{self, Canvas};
    use super::view_port::{self, ViewPort, ViewPortInnerDrawCallback};
    use crate::InputEvent;
    use crate::miri_bindings::utils::*;

    use crate::miri_bindings::lock::SpinLock;
    use alloc::sync::Arc;
    use core::ptr::NonNull;

    #[repr(C)]
    pub struct GuiInner {
        canvas: Canvas,

        pub thread_id: usize,
        input_channel: Option<InputEvent>,
        request_redraw: bool,
        pub stop: bool,

        pub view_port: Option<NonNull<ViewPort>>,
    }

    impl GuiInner {
        // This isn't _entirely_ correct to the source; in that, the GUI record is created and
        // populated by the GUI svc thread, not the other way around.
        pub fn spawn() -> Arc<SpinLock<Self>> {
            let canvas = Canvas {};

            let gui = Self {
                canvas,
                thread_id: 0,
                input_channel: None,
                request_redraw: false,
                stop: false,
                view_port: None,
            };
            let gui = Arc::new(SpinLock::new(gui));

            let thread_id = {
                let gui_ptr = Arc::into_raw(gui.clone());
                // SAFETY: Arc was generated above
                unsafe { miri_thread_spawn(thread_start, gui_ptr as *mut _) }
            };

            {
                gui.lock().thread_id = thread_id;
            }

            extern "Rust" fn thread_start(data: *mut ()) {
                // SAFETY: data is guaranteed to have been created from an arc, just above
                let gui: Arc<SpinLock<GuiInner>> = unsafe { Arc::from_raw(data as *const _) };
                debug_assert_eq!(Arc::strong_count(&gui), 2, "immediately post gui thread spawn");

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

        fn redraw(&mut self) -> () {
            unsafe { canvas::canvas_clear(&mut self.canvas) };

            let Some(view_port) = self.view_port else {
                // nothing to do if there's no view port
                return;
            };

            if !unsafe { view_port::view_port_is_enabled(view_port.as_ref()) } {
                return;
            }

            let mut view_port = (unsafe { view_port.as_ref() }).lock();

            let &mut ViewPortInnerDrawCallback { callback: ref draw_callback, context: mut draw_callback_context } = view_port.draw_callback
                .as_mut()
                .expect("ViewPorts should only be registered with the GUI after their draw callbacks have been set");
            let draw_callback =
                draw_callback.expect("ViewPortDrawCallback is only nullable for FFI reasons");
            unsafe { draw_callback(&raw mut self.canvas, draw_callback_context) };
        }

        pub fn request_redraw(&mut self) -> () {
            self.request_redraw = true;
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct View {
    _unused: [u8; 0],
}
#[doc = "Add view_port to view_port tree\n\n > thread safe\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance\n * `layer` (direction in) - GuiLayer where to place view_port"]
pub unsafe fn gui_add_view_port(gui: *mut Gui, view_port: *mut ViewPort, layer: GuiLayer) {
    {
        let view_port: &SpinLock<ViewPortInner> = unsafe { &mut *view_port };
        let mut view_port = view_port.lock();
        let main_gui = unsafe { Arc::from_raw(gui) };
        view_port.gui = Some(main_gui.clone());
        let _ = Arc::into_raw(main_gui);
    }

    let gui: &Gui = unsafe { &*gui };
    let mut gui_guard = gui.lock();

    let view_port = unsafe { NonNull::new_unchecked(view_port) };
    gui_guard.view_port.replace(view_port);

    gui_guard.request_redraw();
}

#[doc = "Remove view_port from rendering tree\n\n > thread safe\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance"]
pub unsafe fn gui_remove_view_port(gui: *mut Gui, view_port: *mut ViewPort) {
    let gui: &Gui = unsafe { &*gui };
    // NOTE: we need to take the GUI lock here to ensure that the service thread isn't able to
    // proceed, as it might attempt to reference the view_port at the same time that we do
    let mut gui_guard = gui.lock();

    {
        let view_port: &SpinLock<ViewPortInner> = unsafe { &mut *view_port };
        let mut view_port = view_port.lock();
        view_port.gui = None;
    }

    gui_guard.view_port = None;

    gui_guard.request_redraw();
}

#[doc = "Send ViewPort to the front\n\n Places selected ViewPort to the top of the drawing stack\n\n # Arguments\n\n* `gui` - Gui instance\n * `view_port` - ViewPort instance"]
pub unsafe fn gui_view_port_send_to_front(gui: *mut Gui, view_port: *mut ViewPort) {
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
