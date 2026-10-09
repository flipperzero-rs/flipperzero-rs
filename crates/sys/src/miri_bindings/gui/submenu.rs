extern crate alloc;

use crate::miri_bindings::gui::view::{
    View, view_alloc, view_free, view_set_context, view_set_input_callback,
};
use crate::miri_bindings::input::{InputEvent, InputType};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::{c_char, c_void};
use core::option::Option;

struct SubmenuItem {
    label: *const c_char,
    index: u32,
    callback: Callbacks,
    callback_context: *mut c_void,
}

enum Callbacks {
    Normal(SubmenuItemCallback),
    Extended(SubmenuItemCallbackEx),
}

pub struct Submenu {
    selected_item_index: u32,
    items: Vec<SubmenuItem>,
    view: *mut View,
}

pub type SubmenuItemCallback = Option<unsafe extern "C" fn(context: *mut c_void, index: u32)>;
pub type SubmenuItemCallbackEx =
    Option<unsafe extern "C" fn(context: *mut c_void, input_type: InputType, index: u32)>;

#[doc = "Allocate and initialize submenu\n\n This submenu is used to select one option\n\n # Returns\n\nSubmenu instance"]
pub unsafe fn submenu_alloc() -> *mut Submenu {
    let view = unsafe { view_alloc() };

    unsafe extern "C" fn handle_input(
        input_event: *mut InputEvent,
        context: *mut core::ffi::c_void,
    ) -> bool {
        let context: &mut Submenu = unsafe { &mut *context.cast() };
        let input_event = unsafe { &*input_event };

        use crate::miri_bindings::input;

        match input_event.key {
            input::InputKeyOk => {
                let selected_item = context
                    .items
                    .get_mut(context.selected_item_index as usize)
                    .expect("Index should always be in range and have a matching item");

                match selected_item.callback {
                    Callbacks::Normal(callback) => {
                        if let Some(callback) = callback {
                            unsafe { callback(selected_item.callback_context, selected_item.index) }
                        }
                    }
                    Callbacks::Extended(callback) => {
                        if let Some(callback) = callback {
                            unsafe {
                                callback(
                                    selected_item.callback_context,
                                    input_event.type_,
                                    selected_item.index,
                                )
                            }
                        }
                    }
                };

                true
            }
            input::InputKeyDown => {
                context.selected_item_index =
                    (context.selected_item_index + 1) % context.items.len() as u32;
                true
            }
            input::InputKeyUp => {
                if context.selected_item_index == 0 {
                    context.selected_item_index = (context.items.len() as u32) - 1;
                } else {
                    context.selected_item_index -= 1;
                }

                true
            }
            _ => false,
        }
    }

    let submenu = Submenu {
        view,
        items: Vec::new(),
        selected_item_index: 0,
    };

    let view = submenu.view;

    let res = Box::into_raw(Box::new(submenu));
    unsafe { view_set_input_callback(view, Some(handle_input)) };
    unsafe { view_set_context(view, res as *mut _) };

    res
}
#[doc = "Deinitialize and free submenu\n\n # Arguments\n\n* `submenu` - Submenu instance"]
pub unsafe fn submenu_free(submenu: *mut Submenu) {
    let submenu = unsafe { Box::from_raw(submenu) };
    unsafe { view_free(submenu.view) };
}
#[doc = "Get submenu view\n\n # Arguments\n\n* `submenu` - Submenu instance\n\n # Returns\n\nView instance that can be used for embedding"]
pub unsafe fn submenu_get_view(submenu: *mut Submenu) -> *mut View {
    let submenu = unsafe { &mut *submenu };
    submenu.view
}
#[doc = "Add item to submenu\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `label` - menu item label\n * `index` - menu item index, used for callback, may be\n the same with other items\n * `callback` - menu item callback\n * `callback_context` - menu item callback context"]
pub unsafe fn submenu_add_item(
    submenu: *mut Submenu,
    label: *const c_char,
    index: u32,
    callback: SubmenuItemCallback,
    callback_context: *mut c_void,
) {
    let submenu = unsafe { &mut *submenu };
    submenu.items.push(SubmenuItem {
        label,
        index,
        callback: Callbacks::Normal(callback),
        callback_context,
    });
}
#[doc = "Add item to submenu with extended press events\n\n # Arguments\n\n* `submenu` - Submenu instance\n * `label` - menu item label\n * `index` - menu item index, used for callback, may be\n the same with other items\n * `callback` - menu item extended callback\n * `callback_context` - menu item callback context"]
pub unsafe fn submenu_add_item_ex(
    submenu: *mut Submenu,
    label: *const c_char,
    index: u32,
    callback: SubmenuItemCallbackEx,
    callback_context: *mut c_void,
) {
    let submenu = unsafe { &mut *submenu };
    submenu.items.push(SubmenuItem {
        label,
        index,
        callback: Callbacks::Extended(callback),
        callback_context,
    });
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
