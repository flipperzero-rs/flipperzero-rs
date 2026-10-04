#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Widget {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WidgetElement {
    _unused: [u8; 0],
}
#[doc = "Allocate Widget that holds Widget Elements\n\n # Returns\n\nWidget instance"]
pub fn widget_alloc() -> *mut Widget {
    todo!()
}
#[doc = "Free Widget\n > **Note:** this function free allocated Widget Elements\n\n # Arguments\n\n* `widget` - Widget instance"]
pub fn widget_free(widget: *mut Widget) {
    todo!()
}
#[doc = "Reset Widget\n\n # Arguments\n\n* `widget` - Widget instance"]
pub fn widget_reset(widget: *mut Widget) {
    todo!()
}
#[doc = "Get Widget view\n\n # Arguments\n\n* `widget` - Widget instance\n\n # Returns\n\nView instance"]
pub fn widget_get_view(widget: *mut Widget) -> *mut super::View {
    todo!()
}
#[doc = "Add Multi String Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `horizontal` - Align instance\n * `vertical` - Align instance\n * `font` - Font instance\n * `text` (direction in) - The text"]
pub fn widget_add_string_multiline_element(
    widget: *mut Widget,
    x: u8,
    y: u8,
    horizontal: crate::Align,
    vertical: crate::Align,
    font: crate::Font,
    text: *const core::ffi::c_char,
) {
    todo!()
}
#[doc = "Add String Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `horizontal` - Align instance\n * `vertical` - Align instance\n * `font` - Font instance\n * `text` (direction in) - The text"]
pub fn widget_add_string_element(
    widget: *mut Widget,
    x: u8,
    y: u8,
    horizontal: crate::Align,
    vertical: crate::Align,
    font: crate::Font,
    text: *const core::ffi::c_char,
) {
    todo!()
}
#[doc = "Add Text Box Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - width to fit text\n * `height` - height to fit text\n * `horizontal` - Align instance\n * `vertical` - Align instance\n * `text` (direction in) - Formatted text. The following formats are available:\n \"text- bold font is used\n \"text- monospaced font is used\n \"text- white text on black background\n * `strip_to_dots` - Strip text to ... if does not fit to width"]
pub fn widget_add_text_box_element(
    widget: *mut Widget,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    horizontal: crate::Align,
    vertical: crate::Align,
    text: *const core::ffi::c_char,
    strip_to_dots: bool,
) {
    todo!()
}
#[doc = "Add Text Scroll Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - width to fit text\n * `height` - height to fit text\n * `text` (direction in) - Formatted text. Default format: align left, Secondary font.\n The following formats are available:\n \"text\" - sets bold font before until next 'symbol\n \"text- sets monospaced font before until next 'symbol\n \"text\" - sets center horizontal align until the next 'symbol\n \"text\" - sets right horizontal align until the next 'symbol"]
pub fn widget_add_text_scroll_element(
    widget: *mut Widget,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    text: *const core::ffi::c_char,
) {
    todo!()
}
#[doc = "Add Button Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `button_type` - GuiButtonType instance\n * `text` - text on allocated button\n * `callback` - ButtonCallback instance\n * `context` - pointer to context"]
pub fn widget_add_button_element(
    widget: *mut Widget,
    button_type: crate::GuiButtonType,
    text: *const core::ffi::c_char,
    callback: crate::ButtonCallback,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
#[doc = "Add Icon Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - top left x coordinate\n * `y` - top left y coordinate\n * `icon` - Icon instance"]
pub fn widget_add_icon_element(widget: *mut Widget, x: u8, y: u8, icon: *const crate::Icon) {
    todo!()
}
#[doc = "Add Rect Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - top left x coordinate\n * `y` - top left y coordinate\n * `width` - rect width\n * `height` - rect height\n * `radius` - corner radius\n * `fill` - whether to fill the box or not"]
pub fn widget_add_rect_element(
    widget: *mut Widget,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    radius: u8,
    fill: bool,
) {
    todo!()
}
#[doc = "Add Circle Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x` - center x coordinate\n * `y` - center y coordinate\n * `radius` - circle radius\n * `fill` - whether to fill the circle or not"]
pub fn widget_add_circle_element(widget: *mut Widget, x: u8, y: u8, radius: u8, fill: bool) {
    todo!()
}
#[doc = "Add Line Element\n\n # Arguments\n\n* `widget` - Widget instance\n * `x1` - first x coordinate\n * `y1` - first y coordinate\n * `x2` - second x coordinate\n * `y2` - second y coordinate"]
pub fn widget_add_line_element(widget: *mut Widget, x1: u8, y1: u8, x2: u8, y2: u8) {
    todo!()
}
