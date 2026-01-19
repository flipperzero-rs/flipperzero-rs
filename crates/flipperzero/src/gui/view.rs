//! View related APIs.

use core::ptr::NonNull;

use core::ffi::c_void;
use flipperzero_sys::{
    self as sys, Canvas as SysCanvas, InputEvent as SysInputEvent, View as SysView,
};

#[cfg(feature = "alloc")]
use crate::internals::alloc::NonUniqueBox;
use crate::{gui::canvas::CanvasView, input::InputEvent};

/// UI view.
#[cfg(feature = "alloc")]
pub struct View<C: ViewCallbacks> {
    inner: ViewInner,
    callbacks: NonUniqueBox<C>,
}

/// UI view.
#[cfg(not(feature = "alloc"))]
pub struct View<C: ViewCallbacks> {
    inner: ViewInner,
    callbacks: core::marker::PhantomData<C>,
}

#[cfg(feature = "alloc")]
impl<C: ViewCallbacks> View<C> {
    pub fn new(callbacks: C) -> Self {
        let inner = ViewInner::new();
        let callbacks = NonUniqueBox::new(callbacks);

        let view = Self { inner, callbacks };

        {
            pub unsafe extern "C" fn dispatch_draw<C: ViewCallbacks>(
                canvas: *mut SysCanvas,
                context: *mut c_void,
            ) {
                // SAFETY: `canvas` is guaranteed to be a valid pointer
                let canvas = unsafe { CanvasView::from_raw(canvas) };

                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `View`
                // and the callback is accessed exclusively by this function
                unsafe { &mut *context }.on_draw(canvas);
            }

            let callback = Some(dispatch_draw::<C> as _);
            unsafe { sys::view_set_draw_callback(view.inner.0.as_ptr(), callback) };
        }

        {
            pub unsafe extern "C" fn dispatch_previous<C: ViewCallbacks>(
                context: *mut c_void,
            ) -> u32 {
                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `View`
                // and the callback is accessed exclusively by this function
                match unsafe { &mut *context }.on_back_event() {
                    Some(scene_id) => scene_id,
                    None => super::view_dispatcher::view_id::IGNORE,
                }
            }

            let callback = Some(dispatch_previous::<C> as _);
            unsafe { sys::view_set_previous_callback(view.inner.0.as_ptr(), callback) };
        }

        {
            pub unsafe extern "C" fn dispatch_input<C: ViewCallbacks>(
                input_event: *mut SysInputEvent,
                context: *mut c_void,
            ) -> bool {
                let input_event: InputEvent = (unsafe { *input_event })
                    .try_into()
                    .expect("`input_event` should be a valid event");

                let context: *mut C = context.cast();
                // SAFETY: `context` is stored in a `Box` which is a member of `View`
                // and the callback is accessed exclusively by this function
                match unsafe { &mut *context }.on_input(input_event) {
                    EventBubbling::Consumed => true,
                    EventBubbling::ReturnForAdditionalProcessing => false,
                }
            }

            let callback = Some(dispatch_input::<C> as _);
            unsafe { sys::view_set_input_callback(view.inner.0.as_ptr(), callback) };
        }

        view
    }
}

impl<C: ViewCallbacks> View<C> {
    /// Creates a copy of raw pointer to the [`sys::View`].
    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> *mut SysView {
        self.inner.0.as_ptr()
    }
}

/// Plain alloc-free wrapper over a [`SysView`].
struct ViewInner(NonNull<SysView>);

impl ViewInner {
    fn new() -> Self {
        // SAFETY: allocation either succeeds producing a valid non-null pointer
        // or stops the system on OOM
        Self(unsafe { NonNull::new_unchecked(sys::view_alloc()) })
    }
}

impl Drop for ViewInner {
    fn drop(&mut self) {
        let raw = self.0.as_ptr();
        // SAFETY: `raw` is valid
        unsafe { sys::view_free(raw) }
    }
}

pub enum EventBubbling {
    Consumed,
    ReturnForAdditionalProcessing,
}

/// Callbacks of the [`View`]
#[allow(unused_variables)]
pub trait ViewCallbacks: Send {
    /// Draw the view onto the canvas.
    ///
    /// NOTE: called from the GUI thread
    fn on_draw(&mut self, canvas: CanvasView);

    /// React to a user input.
    fn on_input(&mut self, event: InputEvent) -> EventBubbling {
        EventBubbling::ReturnForAdditionalProcessing
    }

    fn on_custom_event(&mut self, event: u32) {}

    /// Provide the ID of a scene to switch to, on a (short) Back input event.
    ///
    /// Note; this is only called if the view is owned by a [`sys::ViewDispatcher`].
    ///
    /// If none is returned, the [view dispatcher's navigation event
    /// callback](`sys::ViewDispatcherNavigationEventCallback`) will be invoked, which may then
    /// optionally stop the view dispatcher.
    fn on_back_event(&mut self) -> Option<u32> {
        None
    }
}

// TODO: is there a case for implementing ViewCallbacks on the unit type?? why would you want to?
// impl ViewCallbacks for () {}
