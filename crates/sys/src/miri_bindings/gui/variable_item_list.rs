#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VariableItemList {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VariableItem {
    _unused: [u8; 0],
}
pub type VariableItemChangeCallback =
    ::core::option::Option<unsafe extern "C" fn(item: *mut VariableItem)>;
pub type VariableItemListEnterCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void, index: u32)>;
#[doc = "Allocate and initialize VariableItemList\n\n # Returns\n\nVariableItemList*"]
pub fn variable_item_list_alloc() -> *mut VariableItemList {
    todo!()
}
#[doc = "Deinitialize and free VariableItemList\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance"]
pub fn variable_item_list_free(variable_item_list: *mut VariableItemList) {
    todo!()
}
#[doc = "Clear all elements from list\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance"]
pub fn variable_item_list_reset(variable_item_list: *mut VariableItemList) {
    todo!()
}
#[doc = "Get VariableItemList View instance\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n\n # Returns\n\nView instance"]
pub fn variable_item_list_get_view(variable_item_list: *mut VariableItemList) -> *mut crate::View {
    todo!()
}
#[doc = "Add item to VariableItemList\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n * `label` - item name\n * `values_count` - item values count\n * `change_callback` - called on value change in gui\n * `context` - item context\n\n # Returns\n\nVariableItem* item instance"]
pub fn variable_item_list_add(
    variable_item_list: *mut VariableItemList,
    label: *const core::ffi::c_char,
    values_count: u8,
    change_callback: VariableItemChangeCallback,
    context: *mut core::ffi::c_void,
) -> *mut VariableItem {
    todo!()
}
#[doc = "Set enter callback\n\n # Arguments\n\n* `variable_item_list` - VariableItemList instance\n * `callback` - VariableItemListEnterCallback instance\n * `context` - pointer to context"]
pub fn variable_item_list_set_enter_callback(
    variable_item_list: *mut VariableItemList,
    callback: VariableItemListEnterCallback,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
pub fn variable_item_list_set_selected_item(variable_item_list: *mut VariableItemList, index: u8) {
    todo!()
}
pub fn variable_item_list_get_selected_item_index(variable_item_list: *mut VariableItemList) -> u8 {
    todo!()
}
#[doc = "Set item current selected index\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `current_value_index` - The current value index"]
pub fn variable_item_set_current_value_index(item: *mut VariableItem, current_value_index: u8) {
    todo!()
}
#[doc = "Set number of values for item\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `values_count` - The new values count"]
pub fn variable_item_set_values_count(item: *mut VariableItem, values_count: u8) {
    todo!()
}
#[doc = "Set item current selected text\n\n # Arguments\n\n* `item` - VariableItem* instance\n * `current_value_text` - The current value text"]
pub fn variable_item_set_current_value_text(
    item: *mut VariableItem,
    current_value_text: *const core::ffi::c_char,
) {
    todo!()
}
#[doc = "Get item current selected index\n\n # Arguments\n\n* `item` - VariableItem* instance\n\n # Returns\n\nuint8_t current selected index"]
pub fn variable_item_get_current_value_index(item: *mut VariableItem) -> u8 {
    todo!()
}
#[doc = "Get item context\n\n # Arguments\n\n* `item` - VariableItem* instance\n\n # Returns\n\nvoid* item context"]
pub fn variable_item_get_context(item: *mut VariableItem) -> *mut core::ffi::c_void {
    todo!()
}
