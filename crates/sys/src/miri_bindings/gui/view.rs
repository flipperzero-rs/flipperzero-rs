extern crate alloc;

use crate::{Canvas, InputEvent, miri_bindings::utils::miri_alloc};
use alloc::boxed::Box;
use core::{ffi::c_void, ptr::NonNull};

pub const ViewOrientationHorizontal: ViewOrientation = ViewOrientation(0);
pub const ViewOrientationHorizontalFlip: ViewOrientation = ViewOrientation(1);
pub const ViewOrientationVertical: ViewOrientation = ViewOrientation(2);
pub const ViewOrientationVerticalFlip: ViewOrientation = ViewOrientation(3);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewOrientation(pub core::ffi::c_uchar);

pub const IGNORE: u32 = 0xFFFFFFFE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct View {
    draw_callback: Option<ViewDrawCallback>,
    pub(super) input_callback: Option<ViewInputCallback>,
    pub(super) previous_callback: Option<ViewNavigationCallback>,

    pub(super) context: Option<NonNull<c_void>>,
    model: Option<NonNull<c_void>>,
}

impl View {
    pub(super) fn draw(&mut self, canvas: *mut Canvas) -> () {
        todo!()
    }

    pub(super) fn process_input(&mut self, input_event: &mut InputEvent) -> bool {
        let Some(input_callback) = self.input_callback else {
            return false;
        };

        let input_callback =
            input_callback.expect("ViewPortInputCallback is only nullable for FFI reasons");
        let context = self
            .context
            .map_or_else(core::ptr::null_mut, NonNull::as_ptr);

        unsafe { input_callback(core::ptr::from_mut(input_event), context) }
    }

    pub(super) fn process_previous(&mut self) -> u32 {
        let Some(previous_callback) = self.previous_callback else {
            return IGNORE;
        };

        let previous_callback =
            previous_callback.expect("ViewPortPreviousCallback is only nullable for FFI reasons");
        let context = self
            .context
            .map_or_else(core::ptr::null_mut, NonNull::as_ptr);

        unsafe { previous_callback(context) }
    }
}

#[doc = "View Draw callback\n # Arguments\n\n* `canvas` - pointer to canvas\n * `model` - pointer to model\n called from GUI thread"]
pub type ViewDrawCallback =
    ::core::option::Option<unsafe extern "C" fn(canvas: *mut super::Canvas, model: *mut c_void)>;
#[doc = "View Input callback\n # Arguments\n\n* `event` - pointer to input event data\n * `context` - pointer to context\n # Returns\n\ntrue if event handled, false if event ignored\n called from GUI thread"]
pub type ViewInputCallback = ::core::option::Option<
    unsafe extern "C" fn(event: *mut crate::InputEvent, context: *mut c_void) -> bool,
>;
#[doc = "View Custom callback\n # Arguments\n\n* `event` - number of custom event\n * `context` - pointer to context\n # Returns\n\ntrue if event handled, false if event ignored"]
pub type ViewCustomCallback =
    ::core::option::Option<unsafe extern "C" fn(event: u32, context: *mut c_void) -> bool>;
#[doc = "View navigation callback\n # Arguments\n\n* `context` - pointer to context\n # Returns\n\nnext view id\n called from GUI thread"]
pub type ViewNavigationCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut c_void) -> u32>;
#[doc = "View callback\n # Arguments\n\n* `context` - pointer to context\n called from GUI thread"]
pub type ViewCallback = ::core::option::Option<unsafe extern "C" fn(context: *mut c_void)>;
#[doc = "View Update Callback Called upon model change, need to be propagated to GUI\n throw ViewPort update\n # Arguments\n\n* `view` - pointer to view\n * `context` - pointer to context\n called from GUI thread"]
pub type ViewUpdateCallback =
    ::core::option::Option<unsafe extern "C" fn(view: *mut View, context: *mut c_void)>;

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
    let view = View {
        draw_callback: None,
        input_callback: None,
        previous_callback: None,
        context: None,
        model: None,
    };

    Box::into_raw(Box::new(view))
}
#[doc = "Free View\n\n # Arguments\n\n* `view` - instance"]
pub unsafe fn view_free(view: *mut crate::View) {
    drop(unsafe { Box::from_raw(view) });
}
#[doc = "Set View Draw callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - draw callback"]
pub unsafe fn view_set_draw_callback(view: *mut crate::View, callback: ViewDrawCallback) {
    let view = unsafe { &mut *view };
    view.draw_callback = Some(callback);
}
#[doc = "Set View Input callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_input_callback(view: *mut crate::View, callback: ViewInputCallback) {
    let view = unsafe { &mut *view };
    view.input_callback = Some(callback);
}
#[doc = "Set View Custom callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_custom_callback(view: *mut crate::View, callback: ViewCustomCallback) {
    todo!()
}
#[doc = "Set Navigation Previous callback\n\n # Arguments\n\n* `view` - View instance\n * `callback` - input callback"]
pub unsafe fn view_set_previous_callback(view: *mut crate::View, callback: ViewNavigationCallback) {
    let view = unsafe { &mut *view };
    view.previous_callback = Some(callback);
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
pub unsafe fn view_set_update_callback_context(view: *mut crate::View, context: *mut c_void) {
    todo!()
}
#[doc = "Set View Draw callback\n\n # Arguments\n\n* `view` - View instance\n * `context` - context for callbacks"]
pub unsafe fn view_set_context(view: *mut crate::View, context: *mut c_void) {
    let context = unsafe { NonNull::new_unchecked(context) };
    let view = unsafe { &mut *view };
    view.context = Some(context);
}
#[doc = "Set View Orientation\n\n # Arguments\n\n* `view` - View instance\n * `orientation` - either vertical or horizontal"]
pub unsafe fn view_set_orientation(view: *mut crate::View, orientation: ViewOrientation) {
    todo!()
}

#[doc = "Allocate view model.\n\n # Arguments\n\n* `view` - View instance\n * `type` - View Model Type\n * `size` - size"]
pub unsafe fn view_allocate_model(view: *mut View, type_: ViewModelType, size: usize) {
    assert!(type_ == ViewModelTypeLockFree);

    let model = unsafe { miri_alloc(size, 4) };
    let model = unsafe { NonNull::new_unchecked(model as *mut c_void) };

    let view = unsafe { &mut *view };
    view.model = Some(model);
}
#[doc = "Free view model data memory.\n\n # Arguments\n\n* `view` - View instance"]
pub unsafe fn view_free_model(view: *mut View) {
    todo!()
}
#[doc = "Get view model data\n\n # Arguments\n\n* `view` - View instance\n\n # Returns\n\npointer to model data\n Don't forget to commit model changes"]
pub unsafe fn view_get_model(view: *mut View) -> *mut c_void {
    let view = unsafe { &mut *view };
    match view.model {
        Some(model) => model.as_ptr(),
        None => core::ptr::null_mut(),
    }
}
#[doc = "Commit view model\n\n # Arguments\n\n* `view` - View instance\n * `update` - true if you want to emit view update, false otherwise"]
pub unsafe fn view_commit_model(view: *mut View, update: bool) {
    todo!()
}
