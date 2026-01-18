use core::ffi::CStr;

pub const ColorWhite: Color = Color(0);
pub const ColorBlack: Color = Color(1);
pub const ColorXOR: Color = Color(2);
#[repr(transparent)]
#[doc = "Color enumeration"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Color(pub core::ffi::c_uchar);
pub const FontPrimary: Font = Font(0);
pub const FontSecondary: Font = Font(1);
pub const FontKeyboard: Font = Font(2);
pub const FontBigNumbers: Font = Font(3);
pub const FontTotalNumber: Font = Font(4);
#[repr(transparent)]
#[doc = "Fonts enumeration"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Font(pub core::ffi::c_uchar);
pub const AlignLeft: Align = Align(0);
pub const AlignRight: Align = Align(1);
pub const AlignTop: Align = Align(2);
pub const AlignBottom: Align = Align(3);
pub const AlignCenter: Align = Align(4);
#[repr(transparent)]
#[doc = "Alignment enumeration"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Align(pub core::ffi::c_uchar);
pub const CanvasOrientationHorizontal: CanvasOrientation = CanvasOrientation(0);
pub const CanvasOrientationHorizontalFlip: CanvasOrientation = CanvasOrientation(1);
pub const CanvasOrientationVertical: CanvasOrientation = CanvasOrientation(2);
pub const CanvasOrientationVerticalFlip: CanvasOrientation = CanvasOrientation(3);
#[repr(transparent)]
#[doc = "Canvas Orientation"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CanvasOrientation(pub core::ffi::c_uchar);
pub const CanvasDirectionLeftToRight: CanvasDirection = CanvasDirection(0);
pub const CanvasDirectionTopToBottom: CanvasDirection = CanvasDirection(1);
pub const CanvasDirectionRightToLeft: CanvasDirection = CanvasDirection(2);
pub const CanvasDirectionBottomToTop: CanvasDirection = CanvasDirection(3);
#[repr(transparent)]
#[doc = "Font Direction"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CanvasDirection(pub core::ffi::c_uchar);
#[doc = "Font parameters"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CanvasFontParameters {
    pub leading_default: u8,
    pub leading_min: u8,
    pub height: u8,
    pub descender: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Canvas {}
#[doc = "Reset canvas drawing tools configuration\n\n # Arguments\n\n* `canvas` - Canvas instance"]
pub unsafe fn canvas_reset(canvas: *mut Canvas) {
    todo!()
}
#[doc = "Commit canvas. Send buffer to display\n\n # Arguments\n\n* `canvas` - Canvas instance"]
pub unsafe fn canvas_commit(canvas: *mut Canvas) {
    todo!()
}
#[doc = "Get Canvas width\n\n # Arguments\n\n* `canvas` - Canvas instance\n\n # Returns\n\nwidth in pixels."]
pub unsafe fn canvas_width(canvas: *const Canvas) -> usize {
    todo!()
}
#[doc = "Get Canvas height\n\n # Arguments\n\n* `canvas` - Canvas instance\n\n # Returns\n\nheight in pixels."]
pub unsafe fn canvas_height(canvas: *const Canvas) -> usize {
    todo!()
}
#[doc = "Get current font height\n\n # Arguments\n\n* `canvas` - Canvas instance\n\n # Returns\n\nheight in pixels."]
pub unsafe fn canvas_current_font_height(canvas: *const Canvas) -> usize {
    todo!()
}
#[doc = "Get font parameters\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `font` - Font\n\n # Returns\n\npointer to CanvasFontParameters structure"]
pub unsafe fn canvas_get_font_params(canvas: *const Canvas, font: Font) -> *const CanvasFontParameters {
    todo!()
}
#[doc = "Clear canvas\n\n # Arguments\n\n* `canvas` - Canvas instance"]
pub unsafe fn canvas_clear(canvas: *mut Canvas) {
    todo!()
}
#[doc = "Set drawing color\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `color` - Color"]
pub unsafe fn canvas_set_color(canvas: *mut Canvas, color: Color) {
    todo!()
}
#[doc = "Set font swap Argument String Rotation Description\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `dir` - Direction font"]
pub unsafe fn canvas_set_font_direction(canvas: *mut Canvas, dir: CanvasDirection) {
    todo!()
}
#[doc = "Invert drawing color\n\n # Arguments\n\n* `canvas` - Canvas instance"]
pub unsafe fn canvas_invert_color(canvas: *mut Canvas) {
    todo!()
}
#[doc = "Set drawing font\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `font` - Font"]
pub unsafe fn canvas_set_font(canvas: *mut Canvas, font: Font) {
    todo!()
}
#[doc = "Set custom drawing font\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `font` - Pointer to u8g2 const uint8_t* font array"]
pub unsafe fn canvas_set_custom_u8g2_font(canvas: *mut Canvas, font: *const u8) {
    todo!()
}
#[doc = "Draw string at position of baseline defined by x, y.\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - anchor point x coordinate\n * `y` - anchor point y coordinate\n * `str` - C-string"]
pub unsafe fn canvas_draw_str(canvas: *mut Canvas, x: i32, y: i32, str_: *const core::ffi::c_char) {
    let _s = unsafe { CStr::from_ptr(str_) };
}
#[doc = "Draw aligned string defined by x, y.\n\n Align calculated from position of baseline, string width and ascent (height\n of the glyphs above the baseline)\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - anchor point x coordinate\n * `y` - anchor point y coordinate\n * `horizontal` - horizontal alignment\n * `vertical` - vertical alignment\n * `str` - C-string"]
pub unsafe fn canvas_draw_str_aligned(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    horizontal: Align,
    vertical: Align,
    str_: *const core::ffi::c_char,
) {
    todo!()
}
#[doc = "Get string width\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `str` - C-string\n\n # Returns\n\nwidth in pixels."]
pub unsafe fn canvas_string_width(canvas: *mut Canvas, str_: *const core::ffi::c_char) -> u16 {
    todo!()
}
#[doc = "Get glyph width\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `symbol` (direction in) - character\n\n # Returns\n\nwidth in pixels"]
pub unsafe fn canvas_glyph_width(canvas: *mut Canvas, symbol: u16) -> usize {
    todo!()
}
#[doc = "Draw bitmap picture at position defined by x,y.\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - width of bitmap\n * `height` - height of bitmap\n * `compressed_bitmap_data` - compressed bitmap data"]
pub unsafe fn canvas_draw_bitmap(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    compressed_bitmap_data: *const u8,
) {
    todo!()
}
#[doc = "Draw icon at position defined by x,y with rotation and flip.\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `icon` - Icon instance\n * `rotation` - IconRotation"]
pub unsafe fn canvas_draw_icon_ex(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    icon: *const super::Icon,
    rotation: super::IconRotation,
) {
    todo!()
}
#[doc = "Draw animation at position defined by x,y.\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `icon_animation` - IconAnimation instance"]
pub unsafe fn canvas_draw_icon_animation(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    icon_animation: *mut super::IconAnimation,
) {
    todo!()
}
#[doc = "Draw icon at position defined by x,y.\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `icon` - Icon instance"]
pub unsafe fn canvas_draw_icon(canvas: *mut Canvas, x: i32, y: i32, icon: *const super::Icon) {
    todo!()
}
#[doc = "Draw XBM bitmap\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` (direction in) - bitmap width\n * `height` (direction in) - bitmap height\n * `bitmap` - pointer to XBM bitmap data"]
pub unsafe fn canvas_draw_xbm(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    bitmap: *const u8,
) {
    todo!()
}
#[doc = "Draw rotated XBM bitmap\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` (direction in) - bitmap width\n * `height` (direction in) - bitmap height\n * `rotation` (direction in) - bitmap rotation\n * `bitmap_data` - pointer to XBM bitmap data"]
pub unsafe fn canvas_draw_xbm_ex(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    rotation: super::IconRotation,
    bitmap_data: *const u8,
) {
    todo!()
}
#[doc = "Draw dot at x,y\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate"]
pub unsafe fn canvas_draw_dot(canvas: *mut Canvas, x: i32, y: i32) {
    todo!()
}
#[doc = "Draw box of width, height at x,y\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - box width\n * `height` - box height"]
pub unsafe fn canvas_draw_box(canvas: *mut Canvas, x: i32, y: i32, width: usize, height: usize) {
    todo!()
}
#[doc = "Draw frame of width, height at x,y\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - frame width\n * `height` - frame height"]
pub unsafe fn canvas_draw_frame(canvas: *mut Canvas, x: i32, y: i32, width: usize, height: usize) {
    todo!()
}
#[doc = "Draw line from x1,y1 to x2,y2\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x1` - x1 coordinate\n * `y1` - y1 coordinate\n * `x2` - x2 coordinate\n * `y2` - y2 coordinate"]
pub unsafe fn canvas_draw_line(canvas: *mut Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    todo!()
}
#[doc = "Draw circle at x,y with radius r\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `radius` - radius"]
pub unsafe fn canvas_draw_circle(canvas: *mut Canvas, x: i32, y: i32, radius: usize) {
    todo!()
}
#[doc = "Draw disc at x,y with radius r\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `radius` - radius"]
pub unsafe fn canvas_draw_disc(canvas: *mut Canvas, x: i32, y: i32, radius: usize) {
    todo!()
}
#[doc = "Draw triangle with given base and height lengths and their intersection\n coordinate\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate of base and height intersection\n * `y` - y coordinate of base and height intersection\n * `base` - length of triangle side\n * `height` - length of triangle height\n * `dir` - CanvasDirection triangle orientation"]
pub unsafe fn canvas_draw_triangle(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    base: usize,
    height: usize,
    dir: CanvasDirection,
) {
    todo!()
}
#[doc = "Draw glyph\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `ch` - character"]
pub unsafe fn canvas_draw_glyph(canvas: *mut Canvas, x: i32, y: i32, ch: u16) {
    todo!()
}
#[doc = "Set transparency mode\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `alpha` - transparency mode"]
pub unsafe fn canvas_set_bitmap_mode(canvas: *mut Canvas, alpha: bool) {
    todo!()
}
#[doc = "Draw rounded-corner frame of width, height at x,y, with round value radius\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - frame width\n * `height` - frame height\n * `radius` - frame corner radius"]
pub unsafe fn canvas_draw_rframe(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    radius: usize,
) {
    todo!()
}
#[doc = "Draw rounded-corner box of width, height at x,y, with round value raduis\n\n # Arguments\n\n* `canvas` - Canvas instance\n * `x` - x coordinate\n * `y` - y coordinate\n * `width` - box width\n * `height` - box height\n * `radius` - box corner radius"]
pub unsafe fn canvas_draw_rbox(
    canvas: *mut Canvas,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    radius: usize,
) {
    todo!()
}
