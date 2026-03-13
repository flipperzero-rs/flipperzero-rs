extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_void};
use core::option::Option;
use alloc::rc::Rc;
use crate::miri_bindings::gui::view::{View, view_alloc, view_free};
use crate::CallbackWithContext;

pub struct VariableItemList {
    enter_callback: Option<CallbackWithContext<VariableItemListEnterCallback>>,

    items: Vec<Rc<VariableItem>>,
    view: *mut View,
}

pub struct VariableItem {
    current_value_index: Option<u8>,
    current_value_text: Option<&'static CStr>,
}

pub type VariableItemChangeCallback = Option<unsafe extern "C" fn(item: *mut VariableItem)>;
pub type VariableItemListEnterCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, index: u32)>;

#[doc = "Allocate and initialize VariableItemList\n\n # Returns\n\nVariableItemList*"]
pub unsafe fn variable_item_list_alloc() -> *mut VariableItemList {
    let view = unsafe { view_alloc() };

    Box::into_raw(Box::new(VariableItemList {
        enter_callback: None,
        items: Vec::new(),
        view,
    }))
}
#[doc = "Deinitialize and free VariableItemList\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance"]
pub unsafe fn variable_item_list_free(variable_item_list: *mut VariableItemList) {
    let variable_item_list = unsafe { Box::from_raw(variable_item_list) };
    unsafe { view_free(variable_item_list.view) };
    drop(variable_item_list)
}

#[doc = "Clear all elements from list\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance"]
pub unsafe fn variable_item_list_reset(variable_item_list: *mut VariableItemList) {
    todo!()
}
#[doc = "Get VariableItemList View instance\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n\n # Returns\n\nView instance"]
pub unsafe fn variable_item_list_get_view(
    variable_item_list: *mut VariableItemList,
) -> *mut crate::View {
    unsafe { &*variable_item_list }.view
}
#[doc = "Add item to VariableItemList\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n * `label` - item name\n * `values_count` - item values count\n * `change_callback` - called on value change in gui\n * `context` - item context\n\n # Returns\n\nVariableItem* item instance"]
pub unsafe fn variable_item_list_add(
    variable_item_list: *mut VariableItemList,
    label: *const c_char,
    values_count: u8,
    change_callback: VariableItemChangeCallback,
    context: *mut c_void,
) -> *mut VariableItem {
    let mut variable_item_list = unsafe { &mut *variable_item_list };
    let item = Rc::new(VariableItem {
        current_value_index: None,
        current_value_text: None,
    });
    variable_item_list.items.push(item.clone());
    Rc::as_ptr(&item).cast_mut()
}

#[doc = "Set enter callback\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n * `callback` - VariableItemListEnterCallback instance\n * `context` - pointer to context"]
pub unsafe fn variable_item_list_set_enter_callback(
    variable_item_list: *mut VariableItemList,
    callback: VariableItemListEnterCallback,
    context: *mut c_void,
) {
    let mut variable_item_list = unsafe { &mut *variable_item_list };
    variable_item_list.enter_callback = Some(CallbackWithContext { callback, context });
}

pub unsafe fn variable_item_list_set_selected_item(
    variable_item_list: *mut VariableItemList,
    index: u8,
) {
    todo!()
}
pub unsafe fn variable_item_list_get_selected_item_index(
    variable_item_list: *mut VariableItemList,
) -> u8 {
    todo!()
}
#[doc = "Set item current selected index\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `current_value_index` - The current value index"]
pub unsafe fn variable_item_set_current_value_index(
    item: *mut VariableItem,
    current_value_index: u8,
) {
    let item = unsafe { &mut *item };
    item.current_value_index = Some(current_value_index);
}
#[doc = "Set number of values for item\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `values_count` - The new values count"]
pub unsafe fn variable_item_set_values_count(item: *mut VariableItem, values_count: u8) {
    todo!()
}
#[doc = "Set item current selected text\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `current_value_text` - The current value text"]
pub unsafe fn variable_item_set_current_value_text(
    item: *mut VariableItem,
    current_value_text: *const c_char,
) {
    let item = unsafe { &mut *item };
    item.current_value_text = Some(unsafe { CStr::from_ptr(current_value_text) });
}
#[doc = "Get item current selected index\n\n # Arguments\n\n* `item` - VariableItem* instance\n\n # Returns\n\nuint8_t current selected index"]
pub unsafe fn variable_item_get_current_value_index(item: *mut VariableItem) -> u8 {
    todo!()
}
#[doc = "Get item context\n\n # Arguments\n\n* `item` - VariableItem* instance\n\n # Returns\n\nvoid* item context"]
pub unsafe fn variable_item_get_context(item: *mut VariableItem) -> *mut c_void {
    todo!()
}
