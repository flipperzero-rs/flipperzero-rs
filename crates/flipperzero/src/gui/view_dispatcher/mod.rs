//! View dispatcher APIs.

mod r#type;

extern crate alloc;
use alloc::{
    collections::{BTreeMap, btree_map::Entry},
    sync::{Arc, Weak},
};

use core::{
    ffi::c_void,
    marker::PhantomData,
    num::NonZeroU32,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use flipperzero_sys::{self as sys, ViewDispatcher as SysViewDispatcher};
pub use r#type::*;

use crate::gui::Gui;

#[cfg(feature = "alloc")]
use crate::gui::view::{View, ViewCallbacks};

type ViewSet = BTreeMap<u32, AtomicBool>;

#[doc(hidden)]
pub mod view_id {
    /// Special view ID which hides drawing view_port.
    const NONE: u32 = 0xFFFFFFFF;

    /// Special view ID which ignores navigation event.
    pub const IGNORE: u32 = 0xFFFFFFFE;
}

pub struct ViewDispatcher<'a, C: ViewDispatcherCallbacks>(
    #[cfg(miri)] pub Arc<ViewDispatcherInner<'a, C>>,
    #[cfg(not(miri))] Arc<ViewDispatcherInner<'a, C>>,
);

/// System ViewDispatcher.
///
/// A holder for a collection of views, which can be switched between. The current view will be
/// drawn to the canvas, and will receive all input events.
pub struct ViewDispatcherInner<'a, C: ViewDispatcherCallbacks> {
    inner: NonNull<SysViewDispatcher>,
    callbacks: C,
    // TODO: propose API to Flipper for checked view addition/removal, which would allow for this
    // local field to be removed
    views: ViewSet,
    _phantom: PhantomData<&'a mut Gui>,
}

unsafe impl<'a, V: ViewDispatcherCallbacks> Send for ViewDispatcherInner<'a, V> {}

unsafe impl<'a, V: ViewDispatcherCallbacks> Sync for ViewDispatcherInner<'a, V> {}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcher<'a, C> {
    pub fn new(callbacks: C, gui: &'a Gui, kind: ViewDispatcherType) -> Self {
        Self(ViewDispatcherInner::new(callbacks, gui, kind))
    }

    pub fn get_context_mut(&mut self) -> &mut C {
        let inner = unsafe { Arc::get_mut_unchecked(&mut self.0) };
        &mut inner.callbacks
    }

    pub fn get_ref(&mut self) -> ViewDispatcherRef<'a, C> {
        ViewDispatcherRef {
            inner: Arc::downgrade(&self.0),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn add_view<VC: ViewCallbacks>(
        &mut self,
        id: u32,
        view: View<VC>,
    ) -> Result<ViewDispatcherView<'a, VC, C>, View<VC>> {
        miri_write_to_stdout(b"Adding view to dispatcher\n");
        let view_dispatcher = self.0.clone();
        // SAFETY: the only references to the ViewDispatcherInner are ourselves, and any potential
        // reference stored in the sys::ViewDispatcher's context, which are not mutable. As such,
        // it's okay to get a mutable reference here
        let inner = unsafe { Arc::get_mut_unchecked(&mut self.0) };
        inner.add_view(view_dispatcher, id, view)
    }

    /// Runs this view dispatcher.
    ///
    /// This will block until the view dispatcher gets stopped.
    pub fn run(self) -> Self {
        let raw = self.0.as_raw();

        // SAFETY: `raw` is valid
        // and this is a `ViewDispatcher` with a queue
        unsafe { sys::view_dispatcher_run(raw) };

        self
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcherInner<'a, C> {
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::{
    /// #     gui::{
    /// #         view_dispatcher::{
    /// #             ViewDispatcher, ViewDispatcherCallbacks,
    /// #             ViewDispatcherRef, ViewDispatcherOps, ViewDispatcherType,
    /// #         },
    /// #         Gui,
    /// #     },
    /// #     log,
    /// # };
    /// struct MyCallbacks {
    ///     value: u32,
    /// }
    /// impl ViewDispatcherCallbacks for MyCallbacks {
    ///     fn on_custom(&mut self, view_dispatcher: ViewDispatcherRef<'_>, event: u32) -> bool {
    ///         log!("{} + {} = {}", self.value, event, self.value + event);
    ///         true
    ///     }
    /// }
    /// let mut gui = Gui::new();
    /// let mut view_dispatcher = ViewDispatcher::new(MyCallbacks {
    ///     value: 10
    /// }, &mut gui, ViewDispatcherType::Fullscreen);
    ///
    /// view_dispatcher.send_custom_event(20);
    /// // should print `10 + 20 = 30`
    /// ```
    pub fn new(callbacks: C, gui: &'a Gui, kind: ViewDispatcherType) -> Arc<Self> {
        let inner = unsafe { NonNull::new_unchecked(sys::view_dispatcher_alloc()) };
        let raw = inner.as_ptr();

        let register_custom_event_callback = C::BindCustom::bind(&callbacks, raw);
        let register_navigation_event_callback = C::BindNavigation::bind(&callbacks, raw);
        let register_tick_event_callback = C::BindTick::bind(&callbacks, raw);

        // SAFETY: both pointers are guaranteed to be non-null
        let view_dispatcher = Arc::new(Self {
            inner,
            callbacks,
            views: BTreeMap::new(),
            _phantom: PhantomData,
        });

        // and store context if at least one event should be registered
        if register_custom_event_callback
            || register_navigation_event_callback
            || register_tick_event_callback
        {
            // NOTE: we don't want to increment the count here, as there's no way to get the
            // event context back out of the inner sys::ViewDispatcher, and the inner
            // sys::ViewDispatcher is guaranteed to live exactly as long as this struct anyway
            let view_dispatcher = Arc::as_ptr(&view_dispatcher);
            let context = view_dispatcher.cast::<c_void>().cast_mut();
            // SAFETY: `raw` is valid
            // and `context` is valid and lives with this struct
            unsafe { sys::view_dispatcher_set_event_callback_context(raw, context) };
        }

        {
            let raw = view_dispatcher.as_raw();
            let gui = gui.as_ptr();
            let kind = kind.into();
            // SAFETY: both pointers are valid and `kind` is a valid numeric value
            // and the newly created view dispatcher does not have a Gui yet
            unsafe { sys::view_dispatcher_attach_to_gui(raw, gui, kind) };
        }

        view_dispatcher
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcherInner<'a, C> {
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> *mut SysViewDispatcher {
        self.inner.as_ptr()
    }
}

impl<'a, C: ViewDispatcherCallbacks> Drop for ViewDispatcherInner<'a, C> {
    fn drop(&mut self) {
        unsafe { sys::view_dispatcher_free(self.as_raw()) };
    }
}

/// Reference to a ViewDispatcher.
#[allow(unused)]
pub struct ViewDispatcherRef<'a, C: ViewDispatcherCallbacks> {
    inner: Weak<ViewDispatcherInner<'a, C>>,
}

struct CallbacksRef<'a, 'b, C: ViewDispatcherCallbacks> {
    dispatcher: Arc<ViewDispatcherInner<'a, C>>,
    phantom: PhantomData<&'b ViewDispatcherInner<'a, C>>,
}

impl<'a, 'b, C: ViewDispatcherCallbacks> Deref for CallbacksRef<'a, 'b, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.dispatcher.callbacks
    }
}

impl<'a, 'b, C: ViewDispatcherCallbacks> Drop for CallbacksRef<'a, 'b, C> {
    fn drop(&mut self) {
        let _ = Arc::downgrade(&self.dispatcher);
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcherRef<'a, C> {
    pub fn get_context<'b>(&'b self) -> impl Deref<Target = C> {
        let dispatcher = self.inner.upgrade().unwrap();
        CallbacksRef {
            dispatcher,
            phantom: PhantomData,
        }
    }

    pub fn switch_to_view(&self, id: u32) -> () {
        let view_dispatcher = self.inner.upgrade().unwrap();
        let raw = (&*view_dispatcher).as_raw();

        miri_write_to_stdout(b"View dispatcher switch to view\n");
        unsafe { sys::view_dispatcher_switch_to_view(raw, id) };

        let _ = Arc::into_raw(view_dispatcher);
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcherInner<'a, C> {
    /// Stops this view dispatcher.
    ///
    /// This will make the [`ViewDispatcher::run`] caller unfreeze.
    pub fn stop(&self) {
        let raw = self.as_raw();
        // SAFETY: `raw` should be valid and point to a ViewDispatcher with a queue
        unsafe { sys::view_dispatcher_stop(raw) };
    }

    pub fn send_custom_event(&mut self, event: u32) {
        let raw = self.as_raw();
        // SAFETY: `raw` should be valid
        unsafe { sys::view_dispatcher_send_custom_event(raw, event) };
    }

    #[cfg(feature = "alloc")]
    fn add_view<VC: ViewCallbacks>(
        &mut self,
        view_dispatcher: Arc<Self>,
        id: u32,
        view: View<VC>,
    ) -> Result<ViewDispatcherView<'a, VC, C>, View<VC>> {
        match self.views.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(AtomicBool::new(true));
                Ok(self.add_view_on_success(view_dispatcher, id, view))
            }
            Entry::Occupied(entry) => {
                let entry = entry.get();

                if entry
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    Ok(self.add_view_on_success(view_dispatcher, id, view))
                } else {
                    Err(view)
                }
            }
        }
    }

    #[cfg(feature = "alloc")]
    fn add_view_on_success<VC: ViewCallbacks>(
        &self,
        view_dispatcher: Arc<Self>,
        id: u32,
        view: View<VC>,
    ) -> ViewDispatcherView<'a, VC, C> {
        let raw = self.as_raw();
        let view_ptr = view.as_raw();
        unsafe { sys::view_dispatcher_add_view(raw, id, view_ptr) };

        ViewDispatcherView {
            view_dispatcher,
            view,
            id,
            phantom: PhantomData,
        }
    }

    pub fn get_context_mut(&mut self) -> &mut C {
        &mut self.callbacks
    }
}

#[cfg(feature = "alloc")]
#[must_use]
pub struct ViewDispatcherView<'a, VC: ViewCallbacks, VDC: ViewDispatcherCallbacks> {
    view_dispatcher: Arc<ViewDispatcherInner<'a, VDC>>,
    view: View<VC>,
    pub id: u32,
    phantom: PhantomData<&'a ViewDispatcher<'a, VDC>>,
}

#[cfg(feature = "alloc")]
impl<'a, VC: ViewCallbacks, VDC: ViewDispatcherCallbacks> ViewDispatcherView<'a, VC, VDC> {
    pub fn switch_to_view(&self) {
        let raw = (&*self.view_dispatcher).as_raw();

        miri_write_to_stdout(b"View dispatcher switch to view\n");
        unsafe { sys::view_dispatcher_switch_to_view(raw, self.id) };
    }

    pub fn as_inner(&self) -> &ViewDispatcherInner<'a, VDC> {
        &self.view_dispatcher
    }
}

#[cfg(feature = "alloc")]
impl<VC: ViewCallbacks, VDC: ViewDispatcherCallbacks> Drop for ViewDispatcherView<'_, VC, VDC> {
    fn drop(&mut self) {
        unsafe { sys::view_dispatcher_remove_view(self.view_dispatcher.as_raw(), self.id) };

        let entry = self
            .view_dispatcher
            .views()
            .get(&self.id)
            .expect("Id must have been inserted for this struct to exist");
        entry.store(false, Ordering::SeqCst);
    }
}

impl<'a, C: ViewDispatcherCallbacks> ViewDispatcherInner<'a, C> {
    pub fn switch_to_view(&mut self, id: u32) {
        if self.views().contains_key(&id) {
            let raw = self.as_raw();
            unsafe { sys::view_dispatcher_switch_to_view(raw, id) };
        }
    }

    #[inline(always)]
    fn views(&self) -> &ViewSet {
        &self.views
    }

    #[inline(always)]
    fn views_mut(&mut self) -> &mut ViewSet {
        &mut self.views
    }
}

/// Should a back event stop the view dispatcher.
#[derive(Debug, PartialEq, Eq)]
pub enum StopDispatcher {
    /// Stops the currently [running](`ViewDispatcher::run`) [event loop](`sys::FuriEventLoop`).
    Yes,

    /// Discards the [input event](`crate::input::InputEvent`).
    No,
}

trait BindOption<T: CallbackOption> {
    fn bind<C: ViewDispatcherCallbacks>(context: &C, raw: *mut SysViewDispatcher) -> bool;
}

pub struct ShouldBind<T> {
    _phantom: core::marker::PhantomData<T>,
}
impl<T: CallbackOption> BindOption<T> for ShouldBind<T> {
    fn bind<C: ViewDispatcherCallbacks>(context: &C, raw: *mut SysViewDispatcher) -> bool {
        T::bind::<C>(context, raw)
    }
}

pub struct DontBind;

impl<T: CallbackOption> BindOption<T> for DontBind {
    fn bind<C>(_context: &C, _raw: *mut SysViewDispatcher) -> bool {
        false
    }
}

trait CallbackOption {
    fn bind<T: ViewDispatcherCallbacks>(context: &T, raw: *mut SysViewDispatcher) -> bool;
}

unsafe extern "Rust" {
    pub safe fn miri_write_to_stdout(bytes: &[u8]);
}

pub struct Custom;
impl CallbackOption for Custom {
    fn bind<C: ViewDispatcherCallbacks>(_context: &C, raw: *mut SysViewDispatcher) -> bool {
        miri_write_to_stdout(b"registering custom event handler\n");
        pub unsafe extern "C" fn dispatch_custom<C: ViewDispatcherCallbacks>(
            context: *mut c_void,
            event: u32,
        ) -> bool {
            let context: Arc<ViewDispatcherInner<C>> = unsafe { Arc::from_raw(context as *mut _) };
            // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
            // and the callback is accessed exclusively by this function
            // NOTE: there is no requirement that `Context<C>` be `Send`, as
            // `dispatch_custom` is only ever called by `raw`'s event loop, which is
            // (presumably/probably) called on the same thread that `Context<C>` was
            // constructed on
            // TODO: `Context<C>` should not be `Send`?
            let result = context.callbacks.on_custom(&context, event);

            let _ = Arc::into_raw(context);

            result
        }

        let callback = Some(dispatch_custom::<C> as _);
        // SAFETY: `raw` is valid and `callbacks` is valid and lives with this struct
        unsafe { sys::view_dispatcher_set_custom_event_callback(raw, callback) };

        true
    }
}

pub struct Navigation;
impl CallbackOption for Navigation {
    fn bind<C: ViewDispatcherCallbacks>(_context: &C, raw: *mut SysViewDispatcher) -> bool {
        miri_write_to_stdout(b"registering navigation event handler\n");
        pub unsafe extern "C" fn dispatch_navigation<C: ViewDispatcherCallbacks>(
            context: *mut c_void,
        ) -> bool {
            let context: Arc<ViewDispatcherInner<C>> = unsafe { Arc::from_raw(context as *mut _) };
            // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
            // and the callback is accessed exclusively by this function
            // NOTE: there is no requirement that `Context<C>` be `Send`, as
            // `dispatch_custom` is only ever called by `raw`'s event loop, which is
            // (presumably/probably) called on the same thread that `Context<C>` was
            // constructed on
            // TODO: `Context<C>` should not be `Send`?
            let result = context.callbacks.on_navigation(&context);

            let _ = Arc::into_raw(context);

            result == StopDispatcher::Yes
        }

        let callback = Some(dispatch_navigation::<C> as _);
        // SAFETY: `raw` is valid
        // and `callbacks` is valid and lives with this struct
        unsafe { sys::view_dispatcher_set_navigation_event_callback(raw, callback) };

        true
    }
}

pub struct Tick;
impl CallbackOption for Tick {
    fn bind<C: ViewDispatcherCallbacks>(context: &C, raw: *mut SysViewDispatcher) -> bool {
        miri_write_to_stdout(b"registering tick event handler\n");
        pub unsafe extern "C" fn dispatch_tick<C: ViewDispatcherCallbacks>(context: *mut c_void) {
            let context: Arc<ViewDispatcherInner<C>> = unsafe { Arc::from_raw(context as *mut _) };
            // SAFETY: `context` is stored in a `Box` which is a member of `ViewDispatcher`
            // and the callback is accessed exclusively by this function
            let result = context.callbacks.on_tick(&context);

            let _ = Arc::into_raw(context);

            result
        }

        let tick_period = context.tick_period().get();
        let callback = Some(dispatch_tick::<C> as _);
        // SAFETY: `raw` is valid
        // and `callbacks` is valid and lives with this struct
        unsafe { sys::view_dispatcher_set_tick_event_callback(raw, callback, tick_period) };

        true
    }
}

/// Callbacks for [`ViewDispatcher`].
#[allow(unused_variables)]
pub trait ViewDispatcherCallbacks {
    type BindCustom: BindOption<Custom> = ShouldBind<Custom>;
    type BindNavigation: BindOption<Navigation> = ShouldBind<Navigation>;
    type BindTick: BindOption<Tick> = ShouldBind<Tick>;

    /// Called on a Custom Event [`sys::view_dispatcher_send_custom_event`], if that custom event
    /// is otherwise not consumed.
    ///
    /// Only called if the view_dispatcher's current view's [`sys::ViewCustomCallback`] method
    /// returns false.
    ///
    /// The return value of this function is unused.
    ///
    /// The majority of usages of this method in the flipper's codebase is to dispatch to
    /// [`sys::scene_manager_handle_custom_event`].
    fn on_custom<T>(&self, view_dispatcher: &ViewDispatcherInner<T>, event: u32) -> bool
    where
        T: ViewDispatcherCallbacks,
    {
        false
    }

    /// Called on a (short) Back input event, if that input event is otherwise not consumed.
    ///
    /// Only called if:
    ///  * the view_dispatcher's current view does not consume the input.
    ///  * the view_dispatcher's current view does not define a previous view.
    fn on_navigation<T>(&self, view_dispatcher: &ViewDispatcherInner<T>) -> StopDispatcher
    where
        T: ViewDispatcherCallbacks,
    {
        // TODO: should this default to Yes? If the user doesn't define this, and an event isn't
        // handled by the view, and reaches this point, it should probably shut down the app?
        StopDispatcher::No
    }

    // SAFETY: only ViewDispatcherInners may be created which exclusively own their EventLoops, and
    // so changing the tick_period (which is done alongside setting the tick callback) is
    // permitted.
    fn on_tick<T>(&self, view_dispatcher: &ViewDispatcherInner<T>)
    where
        T: ViewDispatcherCallbacks,
    {
    }

    #[must_use]
    fn tick_period(&self) -> NonZeroU32 {
        // Some arbitrary default
        NonZeroU32::new(100).unwrap()
    }
}

impl ViewDispatcherCallbacks for () {
    type BindCustom = DontBind;
    type BindNavigation = DontBind;
    type BindTick = DontBind;
}
