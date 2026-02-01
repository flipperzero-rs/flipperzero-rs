extern crate alloc;

use alloc::boxed::Box;
use crate::Canvas;

pub const ViewOrientationHorizontal: ViewOrientation = ViewOrientation(0);
pub const ViewOrientationHorizontalFlip: ViewOrientation = ViewOrientation(1);
pub const ViewOrientationVertical: ViewOrientation = ViewOrientation(2);
pub const ViewOrientationVerticalFlip: ViewOrientation = ViewOrientation(3);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewOrientation(pub core::ffi::c_uchar);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct View {}

impl View {
    pub(super) fn draw(&mut self, canvas: *mut Canvas) -> () {
        todo!()
    }
}

#[doc = "View Draw callback\n # Arguments\n\n* `canvas` - pointer to canvas\n * `model` - pointer to model\n called from GUI thread"]
pub type ViewDrawCallback = ::core::option::Option<
    unsafe extern "C" fn(canvas: *mut super::Canvas, model: *mut core::ffi::c_void),
>;
#[doc = "View Input callback\n # Arguments\n\n* `event` - pointer to input event data\n * `context` - pointer to context\n # Returns\n\ntrue if event handled, false if event ignored\n called from GUI thread"]
pub type ViewInputCallback = ::core::option::Option<
    unsafe extern "C" fn(event: *mut crate::InputEvent, context: *mut core::ffi::c_void) -> bool,
>;
#[doc = "View Custom callback\n # Arguments\n\n* `event` - number of custom event\n * `context` - pointer to context\n # Returns\n\ntrue if event handled, false if event ignored"]
pub type ViewCustomCallback = ::core::option::Option<
    unsafe extern "C" fn(event: u32, context: *mut core::ffi::c_void) -> bool,
>;
#[doc = "View navigation callback\n # Arguments\n\n* `context` - pointer to context\n # Returns\n\nnext view id\n called from GUI thread"]
pub type ViewNavigationCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void) -> u32>;
#[doc = "View callback\n # Arguments\n\n* `context` - pointer to context\n called from GUI thread"]
pub type ViewCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void)>;
#[doc = "View Update Callback Called upon model change, need to be propagated to GUI\n throw ViewPort update\n # Arguments\n\n* `view` - pointer to view\n * `context` - pointer to context\n called from GUI thread"]
pub type ViewUpdateCallback =
    ::core::option::Option<unsafe extern "C" fn(view: *mut View, context: *mut core::ffi::c_void)>;

#[doc = "Model is not allocated"]
pub const ViewModelTypeNone: ViewModelType = ViewModelType(0);
#[doc = "Model consist of atomic types and/or partial update is not critical for rendering.\n Lock free."]
pub const ViewModelTypeLockFree: ViewModelType = ViewModelType(1);
#[doc = "Model access is guarded with mutex.\n Locking gui thread."]
pub const ViewModelTypeLocking: ViewModelType = ViewModelType(2);

#[repr(transparent)]
#[doc = "View model types"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewModelType(pub core::ffi::c_uchar);

#[doc = "Allocate and init View\n # Returns\n\nView instance"]
pub unsafe fn view_alloc() -> *mut crate::View {
    Box::into_raw(Box::new(View {} ))
}
#[doc = "Free View\n\n # Arguments\n\n* `view` - instance"]
pub unsafe fn view_free(view: *mut crate::View) {
    todo!()
}
#[doc = "Set View Draw callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - draw callback"]
pub unsafe fn view_set_draw_callback(view: *mut crate::View, callback: ViewDrawCallback) {
    todo!()
}
#[doc = "Set View Input callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_input_callback(view: *mut crate::View, callback: ViewInputCallback) {
    todo!()
}
#[doc = "Set View Custom callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_custom_callback(view: *mut crate::View, callback: ViewCustomCallback) {
    todo!()
}
#[doc = "Set Navigation Previous callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_previous_callback(view: *mut crate::View, callback: ViewNavigationCallback) {
    todo!()
}
#[doc = "Set Enter callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - callback"]
pub unsafe fn view_set_enter_callback(view: *mut crate::View, callback: ViewCallback) {
    todo!()
}
#[doc = "Set Exit callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - callback"]
pub unsafe fn view_set_exit_callback(view: *mut crate::View, callback: ViewCallback) {
    todo!()
}
#[doc = "Set Update callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - callback"]
pub unsafe fn view_set_update_callback(view: *mut crate::View, callback: ViewUpdateCallback) {
    todo!()
}
#[doc = "Set View Draw callback\n\n # Arguments\n\n* `view` - View instance\n * `context` - context for callbacks"]
pub unsafe fn view_set_update_callback_context(
    view: *mut crate::View,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
#[doc = "Set View Draw callback\n\n # Arguments\n\n* `view` - View instance\n * `context` - context for callbacks"]
pub unsafe fn view_set_context(view: *mut crate::View, context: *mut core::ffi::c_void) {
    todo!()
}
#[doc = "Set View Orientation\n\n # Arguments\n\n* `view` - View instance\n * `orientation` - either vertical or horizontal"]
pub unsafe fn view_set_orientation(view: *mut crate::View, orientation: ViewOrientation) {
    todo!()
}
