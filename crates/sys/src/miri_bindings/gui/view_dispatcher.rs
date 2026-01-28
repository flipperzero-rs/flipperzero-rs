extern crate alloc;

use crate::{GuiLayerDesktop, ViewPort, gui_add_view_port, view_port_alloc};
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::ffi::c_void;
use core::ptr::NonNull;

#[doc = "< Desktop layer: fullscreen with status bar on top of it. For internal usage."]
pub const ViewDispatcherTypeDesktop: ViewDispatcherType = ViewDispatcherType(0);
#[doc = "< Window layer: with status bar"]
pub const ViewDispatcherTypeWindow: ViewDispatcherType = ViewDispatcherType(1);
#[doc = "< Fullscreen layer: without status bar"]
pub const ViewDispatcherTypeFullscreen: ViewDispatcherType = ViewDispatcherType(2);
#[repr(transparent)]
#[doc = "ViewDispatcher view_port placement"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewDispatcherType(pub core::ffi::c_uchar);

#[repr(C)]
pub struct ViewDispatcher {
    view_port: NonNull<ViewPort>,

    pub custom_event_callback: Option<ViewDispatcherCustomEventCallback>,
    pub navigation_event_callback: Option<ViewDispatcherNavigationEventCallback>,
    pub tick_event_callback: Option<ViewDispatcherTickEventCallback>,
    pub context: *mut c_void,

    gui: Option<Arc<crate::lock::SpinLock<super::GuiInner>>>,
}

#[doc = "Prototype for custom event callback"]
pub type ViewDispatcherCustomEventCallback = ::core::option::Option<
    unsafe extern "C" fn(context: *mut core::ffi::c_void, event: u32) -> bool,
>;
#[doc = "Prototype for navigation event callback"]
pub type ViewDispatcherNavigationEventCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void) -> bool>;
#[doc = "Prototype for tick event callback"]
pub type ViewDispatcherTickEventCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void)>;

#[doc = "Allocate ViewDispatcher instance\n\n # Returns\n\npointer to ViewDispatcher instance"]
pub unsafe fn view_dispatcher_alloc() -> *mut ViewDispatcher {
    let view_port = unsafe { NonNull::new_unchecked(view_port_alloc()) };

    Box::into_raw(Box::new(ViewDispatcher {
        view_port,
        gui: None,
        custom_event_callback: None,
        navigation_event_callback: None,
        tick_event_callback: None,
        context: core::ptr::null_mut(),
    }))
}
#[doc = "Free ViewDispatcher instance\n\n All added views MUST be removed using view_dispatcher_remove_view()\n before calling this function.\n\n # Arguments\n\n* `view_dispatcher` - pointer to ViewDispatcher"]
pub unsafe fn view_dispatcher_free(view_dispatcher: *mut ViewDispatcher) {
    todo!()
}

#[doc = "Enable queue support\n\n > **Deprecated** Do NOT use in new code and remove all calls to it from existing code.\n The queue support is now always enabled during construction. If no queue support\n is required, consider using ViewHolder instead.\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance"]
pub unsafe fn view_dispatcher_enable_queue(view_dispatcher: *mut ViewDispatcher) {
    unimplemented!("This method is deprecated")
}
#[doc = "Send custom event\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `event` (direction in) - The event"]
pub unsafe fn view_dispatcher_send_custom_event(view_dispatcher: *mut ViewDispatcher, event: u32) {
    todo!()
}
#[doc = "Set custom event handler\n\n Called on Custom Event, if it is not consumed by view\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `callback` - ViewDispatcherCustomEventCallback instance"]
pub unsafe fn view_dispatcher_set_custom_event_callback(
    view_dispatcher: *mut ViewDispatcher,
    callback: ViewDispatcherCustomEventCallback,
) {
    let mut view_dispatcher = unsafe { &mut *view_dispatcher };
    view_dispatcher.custom_event_callback = Some(callback);
}
#[doc = "Set navigation event handler\n\n Called on Input Short Back Event, if it is not consumed by view\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `callback` - ViewDispatcherNavigationEventCallback instance"]
pub unsafe fn view_dispatcher_set_navigation_event_callback(
    view_dispatcher: *mut ViewDispatcher,
    callback: ViewDispatcherNavigationEventCallback,
) {
    let mut view_dispatcher = unsafe { &mut *view_dispatcher };
    view_dispatcher.navigation_event_callback = Some(callback);
}
#[doc = "Set tick event handler\n\n Requires the event loop to be owned by the view dispatcher, i.e.\n it should have been instantiated with `view_dispatcher_alloc`, not\n `view_dispatcher_alloc_ex`.\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `callback` - ViewDispatcherTickEventCallback\n * `tick_period` - callback call period"]
pub unsafe fn view_dispatcher_set_tick_event_callback(
    view_dispatcher: *mut ViewDispatcher,
    callback: ViewDispatcherTickEventCallback,
    tick_period: u32,
) {
    todo!()
}
#[doc = "Set event callback context\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `context` - pointer to context"]
pub unsafe fn view_dispatcher_set_event_callback_context(
    view_dispatcher: *mut ViewDispatcher,
    context: *mut core::ffi::c_void,
) {
    let mut view_dispatcher = unsafe { &mut *view_dispatcher };
    view_dispatcher.context = context;
}
#[doc = "Run ViewDispatcher\n\n This function will start the event loop and block until view_dispatcher_stop() is called\n or the current thread receives a FuriSignalExit signal.\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance"]
pub unsafe fn view_dispatcher_run(view_dispatcher: *mut ViewDispatcher) {
    todo!()
}
#[doc = "Stop ViewDispatcher\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance"]
pub unsafe fn view_dispatcher_stop(view_dispatcher: *mut ViewDispatcher) {
    todo!()
}
#[doc = "Add view to ViewDispatcher\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `view_id` - View id to register\n * `view` - View instance"]
pub unsafe fn view_dispatcher_add_view(
    view_dispatcher: *mut ViewDispatcher,
    view_id: u32,
    view: *mut super::View,
) {
    todo!()
}
#[doc = "Remove view from ViewDispatcher\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `view_id` - View id to remove"]
pub unsafe fn view_dispatcher_remove_view(view_dispatcher: *mut ViewDispatcher, view_id: u32) {
    todo!()
}
#[doc = "Switch to View\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `view_id` - View id to register\n switching may be delayed till input events complementarity\n reached"]
pub unsafe fn view_dispatcher_switch_to_view(view_dispatcher: *mut ViewDispatcher, view_id: u32) {
    todo!()
}
#[doc = "Send ViewPort of this ViewDispatcher instance to front\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance"]
pub unsafe fn view_dispatcher_send_to_front(view_dispatcher: *mut ViewDispatcher) {
    todo!()
}
#[doc = "Send ViewPort of this ViewDispatcher instance to back\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance"]
pub unsafe fn view_dispatcher_send_to_back(view_dispatcher: *mut ViewDispatcher) {
    todo!()
}
#[doc = "Attach ViewDispatcher to GUI\n\n # Arguments\n\n* `view_dispatcher` - ViewDispatcher instance\n * `gui` - GUI instance to attach to\n * `type` (direction in) - The type"]
pub unsafe fn view_dispatcher_attach_to_gui(
    view_dispatcher: *mut ViewDispatcher,
    gui: *mut super::Gui,
    type_: ViewDispatcherType,
) {
    let view_dispatcher = unsafe { &mut *view_dispatcher };
    unsafe { gui_add_view_port(gui, view_dispatcher.view_port.as_ptr(), GuiLayerDesktop) };

    let main_gui = unsafe { Arc::from_raw(gui) };
    view_dispatcher.gui = Some(main_gui.clone());
    let _ = Arc::into_raw(main_gui);
}
