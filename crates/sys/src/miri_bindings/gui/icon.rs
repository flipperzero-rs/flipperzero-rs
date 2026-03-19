#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Icon {
    pub width: u16,
    pub height: u16,
    pub frame_count: u8,
    pub frame_rate: u8,
    pub frames: *const *const u8,
}
pub const IconRotation0: IconRotation = IconRotation(0);
pub const IconRotation90: IconRotation = IconRotation(1);
pub const IconRotation180: IconRotation = IconRotation(2);
pub const IconRotation270: IconRotation = IconRotation(3);
#[repr(transparent)]
#[doc = "Icon rotation"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct IconRotation(pub core::ffi::c_uchar);

#[doc = "Get icon width\n\n # Arguments\n\n* `instance` (direction in) - pointer to Icon data\n\n # Returns\n\nwidth in pixels"]
pub unsafe fn icon_get_width(instance: *const Icon) -> u16 {
    todo!()
}
#[doc = "Get icon height\n\n # Arguments\n\n* `instance` (direction in) - pointer to Icon data\n\n # Returns\n\nheight in pixels"]
pub unsafe fn icon_get_height(instance: *const Icon) -> u16 {
    todo!()
}
#[doc = "Get Icon XBM bitmap data for the first frame\n\n # Arguments\n\n* `instance` (direction in) - pointer to Icon data\n\n # Returns\n\npointer to compressed XBM bitmap data"]
pub unsafe fn icon_get_data(instance: *const Icon) -> *const u8 {
    todo!()
}
#[doc = "Get Icon frame count\n\n # Arguments\n\n* `instance` (direction in) - pointer to Icon data\n\n # Returns\n\nframe count"]
pub unsafe fn icon_get_frame_count(instance: *const Icon) -> u32 {
    todo!()
}
#[doc = "Get Icon XBM bitmap data for a particular frame\n\n # Arguments\n\n* `instance` (direction in) - pointer to Icon data\n * `frame` (direction in) - frame index\n\n # Returns\n\npointer to compressed XBM bitmap data"]
pub unsafe fn icon_get_frame_data(instance: *const Icon, frame: u32) -> *const u8 {
    todo!()
}
