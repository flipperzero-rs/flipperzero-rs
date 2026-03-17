//! APIs for the Submenu View.

#[cfg(feature = "alloc")]
use crate::gui::view::View;
use crate::gui::view::ViewCallbacks;
#[cfg(feature = "alloc")]
use crate::gui::view_dispatcher::ViewDispatcherView;
use crate::gui::view_dispatcher::{ViewDispatcher, ViewDispatcherCallbacks};
use crate::input::InputType;
use core::ops::{Deref, DerefMut};
use core::{
    ffi::{CStr, c_void},
    ptr::{self, NonNull},
};
use flipperzero_sys as sys;

/// Submenu.
pub struct Submenu {
    inner: SubmenuInner,
    count: u32,
}

impl Submenu {
    /// Constructs a new submenu view.
    pub fn new() -> Self {
        let inner = unsafe { sys::submenu_alloc() };
        let inner = unsafe { NonNull::new_unchecked(inner) };
        Self {
            inner: SubmenuInner(inner),
            count: 0,
        }
    }

    /// Adds a new item to the submenu that, when interacted with, will do nothing.
    pub fn add_plaintext_item<'label>(&mut self, label: &'label CStr) -> SubmenuItemRef<'label> {
        let raw = self.as_raw();
        let index = self.count;
        self.count += 1;

        // NOTE: oh my goddd flipper devs _why_ is the index _like this_
        // so the index doesn't actually have any bearing on the order in the list. the submenu is
        // drawn in insert order. the docs say that the index is "used for callbacks" and doesn't
        // have to be unique.
        //
        // HOWEVER. sys::submenu_{get,set}_selected_item and sys::submenu_change_item_label do
        // operate, searching through the index list and using the first item for which the index
        // matches. _whyyy_
        unsafe { sys::submenu_add_item(raw, label.as_ptr(), index, None, ptr::null_mut()) };

        SubmenuItemRef {
            inner: self.inner.clone(),
            label,
            index,
        }
    }

    /// Adds a new item to the submenu that, when receiving an [`Ok`](crate::input::InputKey::Ok)
    /// event, will invoke a custom callback for the input event.
    ///
    /// # Examples
    ///
    /// ```
    /// # use flipperzero::furi::log::Level;
    /// # use flipperzero::furi::time::FuriInstant;
    /// # use flipperzero::gui::submenu::{SubmenuCustomItem, Submenu};
    /// # use flipperzero::input::InputKey;
    /// # use flipperzero::log;
    ///
    /// struct MyCallbacks {
    ///     event_start_time: Option<FuriInstant>,
    /// }
    ///
    /// impl SubmenuCustomItem for MyCallbacks {
    ///    fn handle_input_event(&mut self, input_type: InputType) -> () {
    ///        match input_type {
    ///            InputType::Press => self.event_start_time = Some(FuriInstant::now()),
    ///            InputType::Release => {
    ///                let start_time = self
    ///                    .event_start_time
    ///                    .take()
    ///                    .expect("Release must have been proceeded by a press");
    ///
    ///                 let elapsed_time = start_time.elapsed();
    ///                 log!(
    ///                     Level::INFO,
    ///                     "OK press lasted for {}.{}",
    ///                     elapsed_time.as_secs(),
    ///                     elapsed_time.as_millis()
    ///                 );
    ///             }
    ///             _ => {}
    ///         }
    ///     }
    /// }
    ///
    /// let mut submenu = Submenu::new();
    ///
    /// let item = submenu.add_custom_item(c"Duration Item", &mut MyCallbacks {});
    /// ```
    pub fn add_custom_item<'item, C: SubmenuCustomItem>(
        &mut self,
        label: &'item CStr,
        callback: &'item mut C,
    ) -> SubmenuItemRef<'item> {
        let raw = self.as_raw();
        let index = self.count;
        self.count += 1;

        unsafe extern "C" fn dispatch_input_event<C: SubmenuCustomItem>(
            context: *mut c_void,
            input_type: sys::InputType,
            _index: u32,
        ) -> () {
            let callback = unsafe { &mut *context.cast::<C>() };

            callback.handle_input_event(
                input_type.try_into()
                .expect("Input event is generated in the flipper codebase and shouldn't have any invalid values"))
        }

        unsafe {
            sys::submenu_add_item_ex(
                raw,
                label.as_ptr(),
                index,
                Some(dispatch_input_event::<C>),
                ptr::from_mut(callback).cast(),
            )
        };

        SubmenuItemRef {
            inner: self.inner.clone(),
            label,
            index,
        }
    }

    /// Returns a raw pointer to the [sys::Submenu] owned by this `Submenu`.
    pub fn as_raw(&self) -> *mut sys::Submenu {
        self.inner.0.as_ptr()
    }

    /// Consumes the `Submenu`, adding its `View` to the `ViewDispatcher` and returning a
    /// `SubmenuBoundToViewDispatcher`.
    ///
    /// In the Flipper's codebase, the `Submenu` is almost always used alongside a
    /// [`sys::SceneManager`]. However, it is possible to just treat it as any other view, and use
    /// it directly with the `ViewDispatcher`.
    ///
    /// Note that the submenu does not define a [previous view](`ViewCallbacks::on_back_event`),
    /// and so any back events that occur while this view is current will not be consumed, and will
    /// hand control to [`ViewDispatcherCallbacks::on_navigation`].
    #[cfg(feature = "alloc")]
    pub fn bind_to_view_dispatcher<'a, 'gui, C: ViewDispatcherCallbacks>(
        self,
        id: u32,
        view_dispatcher: &'a mut ViewDispatcher<'gui, C>,
    ) -> SubmenuBoundToViewDispatcher<'gui, C> {
        let raw = unsafe { sys::submenu_get_view(self.as_raw()) };
        let view = unsafe { View::new_from_raw(raw) };

        match view_dispatcher.add_view(id, view) {
            Ok(view) => SubmenuBoundToViewDispatcher { inner: self, view },
            Err(_view) => todo!("handle the id already being used"),
        }
    }
}

/// A trait that allows for custom handling of [`Ok`](crate::input::InputKey::Ok) events.
pub trait SubmenuCustomItem {
    fn handle_input_event(&mut self, input_type: InputType) -> ();
}

/// Submenu is usually used alongside a [Scene Manager](`sys::SceneManager`), but may also be used
/// directly.
#[cfg(feature = "alloc")]
pub struct SubmenuBoundToViewDispatcher<'gui, C: ViewDispatcherCallbacks> {
    inner: Submenu,
    view: ViewDispatcherView<'gui, (), C>,
}

#[cfg(feature = "alloc")]
impl<'gui, VDC: ViewDispatcherCallbacks> SubmenuBoundToViewDispatcher<'gui, VDC> {
    /// Adds a new item to the submenu that, when interacted with, will switch the
    /// `ViewDispatcher`'s current `View` to the one provided to this method.
    pub fn add_nav_item<'label, VC: ViewCallbacks>(
        &mut self,
        label: &'label CStr,
        // TODO: allow this to take a view id?
        view: &ViewDispatcherView<'gui, VC, VDC>,
    ) -> SubmenuItemRef<'label> {
        let raw = self.as_raw();
        let index = self.inner.count;
        self.inner.count += 1;

        extern "C" fn switch_to_view<'a, VC: ViewCallbacks, VDC: ViewDispatcherCallbacks + 'a>(
            context: *mut c_void,
            _index: u32,
        ) -> () {
            let view = unsafe { &*context.cast::<ViewDispatcherView<'a, VC, VDC>>() };
            view.switch_to_view();
        }

        let context = ptr::from_ref(view).cast_mut();

        unsafe {
            sys::submenu_add_item(
                raw,
                label.as_ptr(),
                index,
                Some(switch_to_view::<VC, VDC>),
                context.cast(),
            )
        };

        SubmenuItemRef {
            inner: self.inner.inner.clone(),
            label,
            index,
        }
    }
}

#[cfg(feature = "alloc")]
impl<'gui, VDC: ViewDispatcherCallbacks> Deref for SubmenuBoundToViewDispatcher<'gui, VDC> {
    type Target = Submenu;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "alloc")]
impl<'gui, VDC: ViewDispatcherCallbacks> DerefMut for SubmenuBoundToViewDispatcher<'gui, VDC> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// System Submenu.
#[derive(Clone)]
struct SubmenuInner(NonNull<sys::Submenu>);

/// A reference to an item contained in the [`Submenu`].
#[must_use]
pub struct SubmenuItemRef<'a> {
    inner: SubmenuInner,
    label: &'a CStr,
    index: u32,
}

impl<'a> SubmenuItemRef<'a> {
    /// Make this item the currently selected item in the [`Submenu`].
    pub fn select(&mut self) -> () {
        let raw = self.inner.0.as_ptr();

        unsafe { sys::submenu_set_selected_item(raw, self.index) };
    }

    /// Check if this item is currently selected.
    pub fn is_selected(&self) -> bool {
        let raw = self.inner.0.as_ptr();

        let selected_item_index = unsafe { sys::submenu_get_selected_item(raw) };
        self.index == selected_item_index
    }
}
