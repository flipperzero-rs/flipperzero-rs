extern crate alloc;

use crate::lock::SpinLock;
use crate::{Canvas, GuiLayerDesktop, InputEvent, ViewPort, gui_add_view_port, view_port_alloc};
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, btree_map::Entry};
use alloc::sync::Arc;
use core::ffi::c_void;
use core::ptr::{NonNull, null_mut};
use core::sync::atomic::{AtomicPtr, Ordering};

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

    views: BTreeMap<u32, NonNull<super::View>>,
    current_view: AtomicPtr<super::View>,

    gui: Option<Arc<crate::lock::SpinLock<super::GuiInner>>>,
}

impl ViewDispatcher {
    fn draw(&self) -> () {
        todo!()
    }
}

#[doc = "Prototype for custom event callback"]
pub type ViewDispatcherCustomEventCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut c_void, event: u32) -> bool>;
#[doc = "Prototype for navigation event callback"]
pub type ViewDispatcherNavigationEventCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut c_void) -> bool>;
#[doc = "Prototype for tick event callback"]
pub type ViewDispatcherTickEventCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut c_void)>;

#[doc = "Allocate ViewDispatcher instance\n\n # Returns\n\npointer to ViewDispatcher instance"]
pub unsafe fn view_dispatcher_alloc() -> *mut ViewDispatcher {
    let view_port = unsafe { NonNull::new_unchecked(view_port_alloc()) };

    let mut view_dispatcher = ViewDispatcher {
        view_port,
        gui: None,
        custom_event_callback: None,
        navigation_event_callback: None,
        tick_event_callback: None,
        views: BTreeMap::new(),
        current_view: AtomicPtr::new(null_mut()),
        context: core::ptr::null_mut(),
    };

    let mut view_dispatcher = Arc::new(view_dispatcher);

    {
        pub unsafe extern "C" fn dispatch_draw(canvas: *mut Canvas, context: *mut c_void) {
            let view_dispatcher = unsafe { Arc::from_raw(context as *mut ViewDispatcher) };

            let current_view = view_dispatcher.current_view.load(Ordering::SeqCst);
            if !current_view.is_null() {
                let mut current_view = unsafe { &mut *current_view };
                current_view.draw(canvas);
            }

            let _ = Arc::into_raw(view_dispatcher);
        }

        pub unsafe extern "C" fn queue_input_event(
            input_event: *mut InputEvent,
            context: *mut c_void,
        ) {
            todo!()
        }

        let context = Arc::into_raw(view_dispatcher.clone());
        let context = context.cast::<c_void>().cast_mut();
        let view_dispatcher_ref = unsafe { Arc::get_mut_unchecked(&mut view_dispatcher) };

        let mut view_port = (unsafe { view_dispatcher_ref.view_port.as_mut() })
            .inner
            .lock();

        view_port.draw_callback = Some(super::ViewPortInnerCallback {
            callback: Some(dispatch_draw),
            context,
        });
        view_port.input_callback = Some(super::ViewPortInnerCallback {
            callback: Some(queue_input_event),
            context,
        });
    }

    debug_assert_eq!(
        Arc::strong_count(&view_dispatcher),
        2,
        "[view_dispatcher_alloc, view_dispatcher's view_port callback context]"
    );

    Arc::into_raw(view_dispatcher).cast_mut()
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
    context: *mut c_void,
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
    let mut view_dispatcher = unsafe { Arc::from_raw(view_dispatcher).clone() };
    let mut view_dispatcher = unsafe { Arc::get_mut_unchecked(&mut view_dispatcher) };
    let Entry::Vacant(entry) = view_dispatcher.views.entry(view_id) else {
        panic!("The view_id is already in use");
    };

    let guard = view_dispatcher.gui.as_deref().map(SpinLock::lock);
    let view = unsafe { NonNull::new_unchecked(view) };
    let _ = entry.insert(view);
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
    let view_dispatcher = unsafe { Arc::from_raw(view_dispatcher) };

    debug_assert_eq!(
        Arc::strong_count(&view_dispatcher),
        2,
        "[dispatcher::new, dispatcher callback context]"
    );

    let mut view_dispatcher = view_dispatcher.clone();

    let main_gui = unsafe { Arc::from_raw(gui) };
    {
        let view_dispatcher = unsafe { Arc::get_mut_unchecked(&mut view_dispatcher) };
        view_dispatcher.gui = Some(main_gui.clone());
    }
    let _ = Arc::into_raw(main_gui);

    unsafe { gui_add_view_port(gui, view_dispatcher.view_port.as_ptr(), GuiLayerDesktop) };
}
