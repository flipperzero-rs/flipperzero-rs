extern crate alloc;

use crate::CallbackWithContext;
use crate::lock::SpinLock;
use crate::miri_bindings::utils::*;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_void;

#[repr(C)]
pub struct ViewPortInner {
    pub draw_callback: Option<CallbackWithContext<ViewPortDrawCallback>>,
    pub input_callback: Option<CallbackWithContext<ViewPortInputCallback>>,

    pub(super) enabled: bool,
}

pub struct ViewPort {
    pub inner: SpinLock<ViewPortInner>,
    pub gui: Option<Arc<super::Gui>>,
}

pub const ViewPortOrientationHorizontal: ViewPortOrientation = ViewPortOrientation(0);
pub const ViewPortOrientationHorizontalFlip: ViewPortOrientation = ViewPortOrientation(1);
pub const ViewPortOrientationVertical: ViewPortOrientation = ViewPortOrientation(2);
pub const ViewPortOrientationVerticalFlip: ViewPortOrientation = ViewPortOrientation(3);
#[doc = "< Special value, don't use it"]
pub const ViewPortOrientationMAX: ViewPortOrientation = ViewPortOrientation(4);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewPortOrientation(pub core::ffi::c_uchar);
#[doc = "ViewPort Draw callback\n called from GUI thread"]
pub type ViewPortDrawCallback =
    ::core::option::Option<unsafe extern "C" fn(canvas: *mut super::Canvas, context: *mut c_void)>;
#[doc = "ViewPort Input callback\n called from GUI thread"]
pub type ViewPortInputCallback = ::core::option::Option<
    unsafe extern "C" fn(event: *mut crate::InputEvent, context: *mut c_void),
>;

#[doc = "ViewPort allocator\n\n always returns view_port or stops system if not enough memory.\n\n # Returns\n\nViewPort instance"]
pub unsafe fn view_port_alloc() -> *mut ViewPort {
    let view_port = SpinLock::new(ViewPortInner {
        draw_callback: None,
        input_callback: None,
        enabled: true,
    });
    let view_port = ViewPort {
        inner: view_port,
        gui: None,
    };
    Box::into_raw(Box::new(view_port))
}

#[doc = "ViewPort deallocator\n\n Ensure that view_port was unregistered in GUI system before use.\n\n # Arguments\n\n* `view_port` - ViewPort instance"]
pub unsafe fn view_port_free(view_port: *mut ViewPort) {
    let view_port_ref: &mut ViewPort = unsafe { &mut *view_port };
    let view_port = unsafe { Box::from_raw(view_port) };
    drop(view_port);
}

#[doc = "Set view_port width.\n\n Will be used to limit canvas drawing area and autolayout feature.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `width` - wanted width, 0 - auto."]
pub unsafe fn view_port_set_width(view_port: *mut ViewPort, width: u8) {
    todo!()
}
pub unsafe fn view_port_get_width(view_port: *const ViewPort) -> u8 {
    todo!()
}
#[doc = "Set view_port height.\n\n Will be used to limit canvas drawing area and autolayout feature.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `height` - wanted height, 0 - auto."]
pub unsafe fn view_port_set_height(view_port: *mut ViewPort, height: u8) {
    todo!()
}
pub unsafe fn view_port_get_height(view_port: *const ViewPort) -> u8 {
    todo!()
}
#[doc = "Enable or disable view_port rendering.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `enabled` - Indicates if enabled\n automatically dispatches update event"]
pub unsafe fn view_port_enabled_set(view_port: *mut ViewPort, enabled: bool) {
    miri_write_to_stdout(if enabled {
        b"Enabling view port\n"
    } else {
        b"Disabling view port\n"
    });
    // NOTE: we're intentionally being extra specific with dereferences here, so that it's clearer
    // where the locks are being taken, and where they're being used
    let view_port = unsafe { &mut *view_port };
    let mut view_port_guard = view_port.inner.lock();
    let view_port_inner = &mut *view_port_guard;
    view_port_inner.enabled = enabled;

    let Some(gui_arc) = view_port.gui.as_mut() else {
        return;
    };

    let mut gui_guard = gui_arc.lock();
    let gui = &mut *gui_guard;
    gui.request_redraw();

    drop(view_port_guard);
    drop(gui_guard);
}

pub unsafe fn view_port_is_enabled(view_port: *const ViewPort) -> bool {
    let view_port = (unsafe { &*view_port }).inner.lock();
    view_port.enabled
}

#[doc = "ViewPort event callbacks\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `callback` - appropriate callback function\n * `context` - context to pass to callback"]
pub unsafe fn view_port_draw_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortDrawCallback,
    context: *mut c_void,
) {
    let mut view_port = (unsafe { &mut *view_port }).inner.lock();
    view_port.draw_callback = Some(CallbackWithContext { callback, context });
}

pub unsafe fn view_port_input_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortInputCallback,
    context: *mut c_void,
) {
    let mut view_port = (unsafe { &mut *view_port }).inner.lock();
    view_port.input_callback = Some(CallbackWithContext { callback, context });
}
#[doc = "Emit update signal to GUI system.\n\n Rendering will happen later after GUI system process signal.\n\n # Arguments\n\n* `view_port` - ViewPort instance"]
pub unsafe fn view_port_update(view_port: *mut ViewPort) {
    todo!()
}
#[doc = "Set ViewPort orientation.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `orientation` - display orientation, horizontal or vertical."]
pub unsafe fn view_port_set_orientation(
    view_port: *mut ViewPort,
    orientation: ViewPortOrientation,
) {
    todo!()
}
pub unsafe fn view_port_get_orientation(view_port: *const ViewPort) -> ViewPortOrientation {
    todo!()
}
