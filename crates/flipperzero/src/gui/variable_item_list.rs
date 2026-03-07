//! Safe wrapper for dealing with allocations and freeing for the variable item list

#[cfg(feature = "alloc")]
use crate::furi::string::FuriString;
use crate::furi::sync::Mutex;
#[cfg(feature = "alloc")]
use crate::gui::view::View;
#[cfg(feature = "alloc")]
use crate::gui::view_dispatcher::{ViewDispatcher, ViewDispatcherCallbacks, ViewDispatcherView};
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};
use core::mem::MaybeUninit;
#[cfg(feature = "alloc")]
use core::ops::{Deref, DerefMut};
use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};
use flipperzero_sys as sys;

#[cfg(feature = "alloc")]
pub struct VariableItemList<'a, T> {
    inner: VariableItemListInner,
    strings: Vec<FuriString>,
    context: CallbackContext<'a, T>,
}

pub struct VariableItem {
    list_index: usize,
    inner: NonNull<sys::VariableItem>,
}

type CallbackContext<'a, T: 'a> = Mutex<CallbackContextInner<'a, T>>;

struct CallbackContextInner<'a, T: 'a> {
    callback: T,
    items: Vec<VariableItemType<'a>>,
}

enum VariableItemType<'a> {
    Plain(VariableItem),
    WithValues(VariableItemValueCallbacksContext<'a>),
}

pub struct UniqueCallbackForEachItem<'a>(Vec<(usize, Box<dyn Callback + 'a>)>);

pub struct VariableItemValueCallbacksContext<'a> {
    callbacks: Box<dyn OnCurrentValueTextChangedCallbacks + 'a>,
    value_label: FuriString,
    item: MaybeUninit<VariableItem>,
}

pub trait Callback {
    fn on_click(&self, item: &VariableItem) -> ();
}

pub trait OnCurrentValueTextChangedCallbacks {
    fn get_new_label(&self, item: &VariableItem, value: u8) -> FuriString;

    fn react_to_change(&self) -> () {}
}

#[cfg(feature = "alloc")]
impl<'callbacks> VariableItemList<'callbacks, UniqueCallbackForEachItem<'callbacks>> {
    pub fn new() -> Self {
        let inner = {
            let variable_item_list = unsafe { sys::variable_item_list_alloc() };
            unsafe { NonNull::new_unchecked(variable_item_list) }
        };

        unsafe extern "C" fn dispatch_callback(context: *mut c_void, index: u32) -> () {
            let context =
                unsafe { &mut *(context as *mut CallbackContext<UniqueCallbackForEachItem>) };

            let mut context = context.lock();

            let Some(callback) = context
                .callback
                .0
                .iter_mut()
                .find_map(|(item_id, callback)| (*item_id == index as usize).then_some(callback))
            else {
                return;
            };

            todo!()
        }

        let callback_context = CallbackContextInner {
            callback: UniqueCallbackForEachItem(Vec::new()),
            items: Vec::new(),
        };

        let res = Self {
            inner: VariableItemListInner(inner),
            strings: Vec::new(),
            context: Mutex::new(callback_context),
        };

        unsafe {
            sys::variable_item_list_set_enter_callback(
                inner.as_ptr(),
                Some(dispatch_callback),
                (&raw const res.context).cast_mut().cast(),
            );
        };

        res
    }

    // NOTE: Label must be owned here; the pointer must be valid for as long as the item exists.
    // Unless we want to accept a CStr and return something with a lifetime, and require the user
    // to keep track of that, this is the best we've got.
    pub fn push_item_plaintext(&mut self, label: FuriString) -> () {
        let mut context = self.context.lock();

        let variable_item = unsafe {
            sys::variable_item_list_add(self.as_raw(), label.as_c_ptr(), 0, None, ptr::null_mut())
        };

        let inner = unsafe { NonNull::new_unchecked(variable_item) };
        let list_index = context.items.len();
        let item = VariableItem { inner, list_index };

        context.items.push(VariableItemType::Plain(item));
        self.strings.push(label);
    }

    pub fn push_item_with_on_click_callback<C: Callback + 'callbacks>(
        &mut self,
        label: FuriString,
        callback: C,
    ) -> () {
        let mut context = self.context.lock();

        let variable_item = unsafe {
            sys::variable_item_list_add(self.as_raw(), label.as_c_ptr(), 0, None, ptr::null_mut())
        };

        let inner = unsafe { NonNull::new_unchecked(variable_item) };
        let list_index = context.items.len();
        let item = VariableItem { inner, list_index };

        context.items.push(VariableItemType::Plain(item));
        self.strings.push(label);

        context.callback.0.push((list_index, Box::new(callback)));
    }

    pub fn push_item_with_options<C: OnCurrentValueTextChangedCallbacks + 'callbacks>(
        &mut self,
        label: FuriString,
        number_of_options: u8,
        callbacks: C,
    ) -> () {
        let mut context = self.context.lock();

        unsafe extern "C" fn dispatch_value_changed_callback(raw: *mut sys::VariableItem) {
            let context = unsafe { sys::variable_item_get_context(raw) };
            let context = unsafe { &mut *(context as *mut VariableItemValueCallbacksContext) };
            let callbacks = &context.callbacks;
            let item = unsafe { context.item.assume_init_ref() };

            let value = unsafe { sys::variable_item_get_current_value_index(raw) };

            let new_label = context.callbacks.get_new_label(item, value);
            unsafe { sys::variable_item_set_current_value_text(raw, new_label.as_c_ptr()) };
            context.value_label = new_label;

            context.callbacks.react_to_change();
        }

        let list_index = context.items.len();

        let mut value_callbacks_context = VariableItemValueCallbacksContext {
            callbacks: Box::new(callbacks),
            value_label: FuriString::new(),
            item: MaybeUninit::uninit(),
        };

        let variable_item = unsafe {
            sys::variable_item_list_add(
                self.as_raw(),
                label.as_c_ptr(),
                number_of_options as u8,
                Some(dispatch_value_changed_callback),
                (&raw const value_callbacks_context).cast_mut().cast(),
            )
        };

        let inner = unsafe { NonNull::new_unchecked(variable_item) };
        let item = VariableItem { inner, list_index };

        let value_label = value_callbacks_context.callbacks.get_new_label(&item, 0);
        unsafe {
            sys::variable_item_set_current_value_text(item.inner.as_ptr(), value_label.as_c_ptr())
        };
        value_callbacks_context.value_label = value_label;

        value_callbacks_context.item.write(item);

        context
            .items
            .push(VariableItemType::WithValues(value_callbacks_context));
        self.strings.push(label);
    }

    pub fn clear(&mut self) -> () {
        {
            let mut context = self.context.lock();
            context.items.clear();
            context.callback.0.clear();
        }

        self.strings.clear();

        unsafe { sys::variable_item_list_reset(self.as_raw()) };
    }
}

#[cfg(feature = "alloc")]
impl<'callback, C: Callback + 'callback> VariableItemList<'callback, C> {
    pub fn new_with_callback(mut on_click_callback: C) -> Self {
        let inner = {
            let variable_item_list = unsafe { sys::variable_item_list_alloc() };
            unsafe { NonNull::new_unchecked(variable_item_list) }
        };

        unsafe extern "C" fn dispatch_callback<C: Callback>(
            context: *mut c_void,
            index: u32,
        ) -> () {
            let context = unsafe { &mut *(context as *mut CallbackContext<C>) };

            let mut context = context.lock();

            let item = context
                .items
                .get(index as usize)
                .expect("No item with given index in local collection");

            match item {
                VariableItemType::Plain(item) => context.callback.on_click(item),
                VariableItemType::WithValues(value_context) => {
                    let item = unsafe { value_context.item.assume_init_ref() };
                    context.callback.on_click(item)
                }
            }
        }

        let callback_context = CallbackContextInner {
            callback: on_click_callback,
            items: Vec::new(),
        };

        let res = Self {
            inner: VariableItemListInner(inner),
            strings: Vec::new(),
            context: Mutex::new(callback_context),
        };

        unsafe {
            sys::variable_item_list_set_enter_callback(
                inner.as_ptr(),
                Some(dispatch_callback::<C>),
                (&raw const res.context).cast_mut().cast(),
            );
        };

        res
    }

    pub fn clear(&mut self) -> () {
        {
            let mut context = self.context.lock();
            context.items.clear();
        }
        self.strings.clear();

        unsafe { sys::variable_item_list_reset(self.as_raw()) };
    }
}

impl<'callback, T> VariableItemList<'callback, T> {
    /// Get pointer to the underlying [`sys::VariableItemList`].
    pub fn as_raw(&self) -> *mut sys::VariableItemList {
        self.inner.0.as_ptr()
    }

    /// Consumes the `VariableItemList`, adding its `View` to the `ViewDispatcher` and returning a
    /// `VariableItemListBoundToViewDispatcher`.
    ///
    /// In the Flipper's codebase, the `VariableItemList` is almost always used alongside a
    /// [`sys::SceneManager`]. However, it is possible to just treat it as any other view, and use
    /// it directly with the `ViewDispatcher`.
    ///
    /// Note that the variable item list does not define a [previous
    /// view](`crate::gui::view::ViewCallbacks::on_back_event`), and so any back events that occur
    /// while this view is current will not be consumed, and will hand control to
    /// [`ViewDispatcherCallbacks::on_navigation`].
    pub fn bind_to_view_dispatcher<'a, 'gui, C: ViewDispatcherCallbacks>(
        self,
        id: u32,
        view_dispatcher: &'a mut ViewDispatcher<'gui, C>,
    ) -> VariableItemListBoundToViewDispatcher<'callback, 'gui, C, T> {
        let raw = unsafe { sys::variable_item_list_get_view(self.as_raw()) };
        let view = unsafe { View::new_from_raw(raw) };

        match view_dispatcher.add_view(id, view) {
            Ok(view) => VariableItemListBoundToViewDispatcher { inner: self, view },
            Err(_view) => todo!("handle the id already being used"),
        }
    }
}

impl<T> Drop for VariableItemList<'_, T> {
    fn drop(&mut self) {
        let mut context = self.context.lock();
        context.items.clear();
        self.strings.clear();

        unsafe { sys::variable_item_list_free(self.as_raw()) };
    }
}

/// VariableItemList is usually used alongside a [Scene Manager](`sys::SceneManager`), but may also be used
/// directly.
#[cfg(feature = "alloc")]
pub struct VariableItemListBoundToViewDispatcher<
    'callbacks,
    'gui,
    C: ViewDispatcherCallbacks,
    OnClickCallbacks: 'callbacks,
> {
    inner: VariableItemList<'callbacks, OnClickCallbacks>,
    view: ViewDispatcherView<'gui, (), C>,
}

#[cfg(feature = "alloc")]
impl<'callbacks, 'gui, VDC: ViewDispatcherCallbacks, OnClickCallbacks: 'callbacks>
    VariableItemListBoundToViewDispatcher<'callbacks, 'gui, VDC, OnClickCallbacks>
{
    pub fn switch_to_view(&self) -> () {
        self.view.switch_to_view();
    }
}

#[cfg(feature = "alloc")]
impl<'callbacks, 'gui, VDC: ViewDispatcherCallbacks, OnClickCallbacks: 'callbacks> Deref
    for VariableItemListBoundToViewDispatcher<'callbacks, 'gui, VDC, OnClickCallbacks>
{
    type Target = VariableItemList<'callbacks, OnClickCallbacks>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "alloc")]
impl<'callbacks, 'gui, VDC: ViewDispatcherCallbacks, OnClickCallbacks: 'callbacks> DerefMut
    for VariableItemListBoundToViewDispatcher<'callbacks, 'gui, VDC, OnClickCallbacks>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct VariableItemListInner(NonNull<sys::VariableItemList>);
