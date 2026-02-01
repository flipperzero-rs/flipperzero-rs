extern crate alloc;

use alloc::boxed::Box;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextInput {
    view: Box<super::view::View>,
}

pub type TextInputCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void)>;
pub type TextInputValidatorCallback = ::core::option::Option<
    unsafe extern "C" fn(
        text: *const core::ffi::c_char,
        error: *mut crate::FuriString,
        context: *mut core::ffi::c_void,
    ) -> bool,
>;

#[doc = "Allocate and initialize text input\n\n This text input is used to enter string\n\n # Returns\n\nTextInput instance"]
pub unsafe fn text_input_alloc() -> *mut TextInput {
    let view = unsafe { super::view::view_alloc() };
    let view = unsafe { Box::from_raw(view) };
    let text_input = TextInput { view };
    Box::into_raw(Box::new(text_input))
}
#[doc = "Deinitialize and free text input\n\n # Arguments\n\n* `text_input` - TextInput instance"]
pub unsafe fn text_input_free(text_input: *mut TextInput) {
    let text_input = unsafe { Box::from_raw(text_input) };
    let view = Box::into_raw(text_input.view);
    unsafe { super::view::view_free(view) }
}

#[doc = "Clean text input view Note: this function does not free memory\n\n # Arguments\n\n* `text_input` - Text input instance"]
pub unsafe fn text_input_reset(text_input: *mut TextInput) {
    todo!()
}
#[doc = "Get text input view\n\n # Arguments\n\n* `text_input` - TextInput instance\n\n # Returns\n\nView instance that can be used for embedding"]
pub unsafe fn text_input_get_view(text_input: *mut TextInput) -> *mut super::View {
    let text_input = unsafe { Box::from_raw(text_input) };
    Box::into_raw(text_input.view)
}

#[doc = "Set text input result callback\n\n # Arguments\n\n* `text_input` - TextInput instance\n * `callback` - callback fn\n * `callback_context` - callback context\n * `text_buffer` - pointer to YOUR text buffer, that we going\n to modify\n * `text_buffer_size` - YOUR text buffer size in bytes. Max string\n length will be text_buffer_size-1.\n * `clear_default_text` - clear text from text_buffer on first OK\n event"]
pub unsafe fn text_input_set_result_callback(
    text_input: *mut TextInput,
    callback: TextInputCallback,
    callback_context: *mut core::ffi::c_void,
    text_buffer: *mut core::ffi::c_char,
    text_buffer_size: usize,
    clear_default_text: bool,
) {
    todo!()
}
#[doc = "Sets the minimum length of a TextInput\n # Arguments\n\n* `[in]` - text_input TextInput\n * `[in]` - minimum_length Minimum input length"]
pub unsafe fn text_input_set_minimum_length(text_input: *mut TextInput, minimum_length: usize) {
    todo!()
}
pub unsafe fn text_input_set_validator(
    text_input: *mut TextInput,
    callback: TextInputValidatorCallback,
    callback_context: *mut core::ffi::c_void,
) {
    todo!()
}
pub unsafe fn text_input_get_validator_callback(
    text_input: *mut TextInput,
) -> TextInputValidatorCallback {
    todo!()
}
pub unsafe fn text_input_get_validator_callback_context(
    text_input: *mut TextInput,
) -> *mut core::ffi::c_void {
    todo!()
}
#[doc = "Set text input header text\n\n # Arguments\n\n* `text_input` - TextInput instance\n * `text` - text to be shown"]
pub unsafe fn text_input_set_header_text(
    text_input: *mut TextInput,
    text: *const core::ffi::c_char,
) {
    todo!()
}
