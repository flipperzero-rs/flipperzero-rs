
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IconAnimation {
    _unused: [u8; 0],
}
