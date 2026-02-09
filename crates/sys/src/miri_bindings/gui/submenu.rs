use crate::miri_bindings::gui::view::View;
use crate::miri_bindings::input::InputType;
use core::ffi::{c_char, c_void};
use core::option::Option;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Submenu {
    _unused: [u8; 0],
}
pub type SubmenuItemCallback = Option<unsafe extern "C" fn(context: *mut c_void, index: u32)>;
pub type SubmenuItemCallbackEx =
    Option<unsafe extern "C" fn(context: *mut c_void, input_type: InputType, index: u32)>;
#[doc = "Allocate and initialize submenu\n\n This submenu is used to select one option\n\n # Returns\n\nSubmenu instance"]
pub unsafe fn submenu_alloc() -> *mut Submenu {
    todo!()
}
#[doc = "Deinitialize and free submenu\n\n # Arguments\n\n* `submenu` - Submenu instance"]
pub unsafe fn submenu_free(submenu: *mut Submenu) {
    todo!()
}
#[doc = "Get submenu view\n\n # Arguments\n\n* `submenu` - Submenu instance\n\n # Returns\n\nView instance that can be used for embedding"]
pub unsafe fn submenu_get_view(submenu: *mut Submenu) -> *mut View {
    todo!()
}
#[doc = "Add item to submenu\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `label` - menu item label\n * `index` - menu item index, used for callback, may be\n the same with other items\n * `callback` - menu item callback\n * `callback_context` - menu item callback context"]
pub unsafe fn submenu_add_item(
    submenu: *mut Submenu,
    label: *const c_char,
    index: u32,
    callback: SubmenuItemCallback,
    callback_context: *mut c_void,
) {
    todo!()
}
#[doc = "Add item to submenu with extended press events\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `label` - menu item label\n * `index` - menu item index, used for callback, may be\n the same with other items\n * `callback` - menu item extended callback\n * `callback_context` - menu item callback context"]
pub unsafe fn submenu_add_item_ex(
    submenu: *mut Submenu,
    label: *const c_char,
    index: u32,
    callback: SubmenuItemCallbackEx,
    callback_context: *mut c_void,
) {
    todo!()
}
#[doc = "Change label of an existing item\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `index` - The index of the item\n * `label` - The new label"]
pub unsafe fn submenu_change_item_label(submenu: *mut Submenu, index: u32, label: *const c_char) {
    todo!()
}
#[doc = "Remove all items from submenu\n\n # Arguments\n\n* `submenu` - Submenu instance"]
pub unsafe fn submenu_reset(submenu: *mut Submenu) {
    todo!()
}
#[doc = "Get submenu selected item index\n\n # Arguments\n\n* `submenu` - Submenu instance\n\n # Returns\n\nIndex of the selected item"]
pub unsafe fn submenu_get_selected_item(submenu: *mut Submenu) -> u32 {
    todo!()
}
#[doc = "Set submenu selected item by index\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `index` - The index of the selected item"]
pub unsafe fn submenu_set_selected_item(submenu: *mut Submenu, index: u32) {
    todo!()
}
#[doc = "Set optional header for submenu\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `header` - header to set"]
pub unsafe fn submenu_set_header(submenu: *mut Submenu, header: *const c_char) {
    todo!()
}
