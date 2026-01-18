extern crate alloc;

use crate::lock::SpinLock;
use crate::miri_bindings::utils::*;
use alloc::boxed::Box;
use alloc::sync::{Arc, Weak};
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;

pub struct ViewPortInnerDrawCallback {
    pub callback: ViewPortDrawCallback,
    pub context: *mut c_void,
}

#[repr(C)]
pub struct ViewPortInner {
    pub draw_callback: Option<ViewPortInnerDrawCallback>,

    enabled: bool,

    pub gui: Option<Arc<super::Gui>>,
}

pub type ViewPort = SpinLock<ViewPortInner>;

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
        enabled: false,
        gui: None,
    });
    {
        let mut view_port = view_port.lock();
        view_port.enabled = true;
        view_port.gui = None;
    }
    Box::into_raw(Box::new(view_port))
}

#[doc = "ViewPort deallocator\n\n Ensure that view_port was unregistered in GUI system before use.\n\n # Arguments\n\n* `view_port` - ViewPort instance"]
pub unsafe fn view_port_free(view_port: *mut ViewPort) {
    drop(unsafe { Box::from_raw(view_port) });
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
    // NOTE: we're intentionally being extra specific with dereferences here, so that it's clearer
    // where the locks are being taken, and where they're being used
    let mut view_port_guard = (unsafe { &mut *view_port }).lock();
    let mut view_port = &mut *view_port_guard;
    view_port.enabled = enabled;

    let gui_arc = view_port
        .gui
        .as_mut()
        .expect("ViewPort must have been added to the GUI in order to be Enabled");

    let mut gui_guard = gui_arc.lock();
    let mut gui = &mut *gui_guard;
    // calling this makes the GUI service thread stop, waiting for the lock (that is
    // currently being held here, to allow this mutable method call to be made).
    // as soon as it acquires the lock, it attempts to read its view_port.
    // since we are still reading the view_port here (we've not dropped the guard yet, and can't
    // until after the gui_guard is dropped bcs of how that was acquired), that causes UB, due to
    // Stacked Borrrow rules.
    // TODO: FIX?? maybe the ViewPort = SpinLock<ViewPortInner> is at fault, and we should instead
    // make ViewPort into a struct: { Inner: SpinLock<ViewPortInner>, Gui }? Then, we can drop the
    // locks in the correct order?
    gui.request_redraw();
}

pub unsafe fn view_port_is_enabled(view_port: *const ViewPort) -> bool {
    let view_port = (unsafe { &*view_port }).lock();
    view_port.enabled
}

#[doc = "ViewPort event callbacks\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `callback` - appropriate callback function\n * `context` - context to pass to callback"]
pub unsafe fn view_port_draw_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortDrawCallback,
    context: *mut c_void,
) {
    let mut view_port = (unsafe { &mut *view_port }).lock();
    view_port.draw_callback = Some(ViewPortInnerDrawCallback { callback, context });
}

pub unsafe fn view_port_input_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortInputCallback,
    context: *mut c_void,
) {
    todo!()
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
