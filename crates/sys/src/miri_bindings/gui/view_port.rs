#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewPort {
    _unused: [u8; 0],
}
pub const ViewPortOrientationHorizontal: ViewPortOrientation = ViewPortOrientation(0);
pub const ViewPortOrientationHorizontalFlip: ViewPortOrientation = ViewPortOrientation(1);
pub const ViewPortOrientationVertical: ViewPortOrientation = ViewPortOrientation(2);
pub const ViewPortOrientationVerticalFlip: ViewPortOrientation = ViewPortOrientation(3);
#[doc = "< Special value, don't use it"]
pub const ViewPortOrientationMAX: ViewPortOrientation = ViewPortOrientation(4);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ViewPortOrientation(pub core::ffi::c_uchar);
#[doc = "ViewPort Draw callback\n called from GUI thread"]
pub type ViewPortDrawCallback = ::core::option::Option<
    unsafe extern "C" fn(canvas: *mut super::Canvas, context: *mut core::ffi::c_void),
>;
#[doc = "ViewPort Input callback\n called from GUI thread"]
pub type ViewPortInputCallback = ::core::option::Option<
    unsafe extern "C" fn(event: *mut crate::InputEvent, context: *mut core::ffi::c_void),
>;
#[doc = "ViewPort allocator\n\n always returns view_port or stops system if not enough memory.\n\n # Returns\n\nViewPort instance"]
pub fn view_port_alloc() -> *mut ViewPort {
    todo!()
}
#[doc = "ViewPort deallocator\n\n Ensure that view_port was unregistered in GUI system before use.\n\n # Arguments\n\n* `view_port` - ViewPort instance"]
pub fn view_port_free(view_port: *mut ViewPort) {
    todo!()
}
#[doc = "Set view_port width.\n\n Will be used to limit canvas drawing area and autolayout feature.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `width` - wanted width, 0 - auto."]
pub fn view_port_set_width(view_port: *mut ViewPort, width: u8) {
    todo!()
}
pub fn view_port_get_width(view_port: *const ViewPort) -> u8 {
    todo!()
}
#[doc = "Set view_port height.\n\n Will be used to limit canvas drawing area and autolayout feature.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `height` - wanted height, 0 - auto."]
pub fn view_port_set_height(view_port: *mut ViewPort, height: u8) {
    todo!()
}
pub fn view_port_get_height(view_port: *const ViewPort) -> u8 {
    todo!()
}
#[doc = "Enable or disable view_port rendering.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `enabled` - Indicates if enabled\n automatically dispatches update event"]
pub fn view_port_enabled_set(view_port: *mut ViewPort, enabled: bool) {
    todo!()
}
pub fn view_port_is_enabled(view_port: *const ViewPort) -> bool {
    todo!()
}
#[doc = "ViewPort event callbacks\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `callback` - appropriate callback function\n * `context` - context to pass to callback"]
pub fn view_port_draw_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortDrawCallback,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
pub fn view_port_input_callback_set(
    view_port: *mut ViewPort,
    callback: ViewPortInputCallback,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
#[doc = "Emit update signal to GUI system.\n\n Rendering will happen later after GUI system process signal.\n\n # Arguments\n\n* `view_port` - ViewPort instance"]
pub fn view_port_update(view_port: *mut ViewPort) {
    todo!()
}
#[doc = "Set ViewPort orientation.\n\n # Arguments\n\n* `view_port` - ViewPort instance\n * `orientation` - display orientation, horizontal or vertical."]
pub fn view_port_set_orientation(view_port: *mut ViewPort, orientation: ViewPortOrientation) {
    todo!()
}
pub fn view_port_get_orientation(view_port: *const ViewPort) -> ViewPortOrientation {
    todo!()
}
