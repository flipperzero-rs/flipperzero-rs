//! Furi HAL version information.

use core::ffi::CStr;
use core::ptr::slice_from_raw_parts;

use flipperzero_sys as sys;

pub use sys::FuriHalVersionOtpVersion as OtpVersion;
pub use sys::FuriHalVersionOtpVersion0 as OtpVersion0;
pub use sys::FuriHalVersionOtpVersion1 as OtpVersion1;
pub use sys::FuriHalVersionOtpVersion2 as OtpVersion2;
pub use sys::FuriHalVersionOtpVersionEmpty as OtpVersionEmpty;
pub use sys::FuriHalVersionOtpVersionUnknown as OtpVersionUnknown;

pub use sys::FuriHalVersionColor as Color;
pub use sys::FuriHalVersionColorBlack as ColorBlack;
pub use sys::FuriHalVersionColorTransparent as ColorTransparent;
pub use sys::FuriHalVersionColorUnknown as ColorUnknown;
pub use sys::FuriHalVersionColorWhite as ColorWhite;

pub use sys::FuriHalVersionRegion as Region;
pub use sys::FuriHalVersionRegionEuRu as RegionEuRu;
pub use sys::FuriHalVersionRegionJp as RegionJp;
pub use sys::FuriHalVersionRegionUnknown as RegionUnknown;
pub use sys::FuriHalVersionRegionUsCaAu as RegionUsCaAu;
pub use sys::FuriHalVersionRegionWorld as RegionWorld;

pub use sys::FuriHalVersionDisplay as Display;
pub use sys::FuriHalVersionDisplayErc as DisplayErc;
pub use sys::FuriHalVersionDisplayMgg as DisplayMgg;
pub use sys::FuriHalVersionDisplayUnknown as DisplayUnknown;

use crate::version::Version;

/// Check target firmware version matches device.
pub fn do_i_belong_here() -> bool {
    unsafe { sys::furi_hal_version_do_i_belong_here() }
}

/// Model name.
pub fn model_name() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_model_name()) }
}

/// Model code.
pub fn model_code() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_model_code()) }
}

/// FCC ID.
pub fn fcc_id() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_fcc_id()) }
}

/// IC ID.
pub fn ic_id() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_ic_id()) }
}

/// MIC ID.
pub fn mic_id() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_mic_id()) }
}

/// SRRC ID.
pub fn srrc_id() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_srrc_id()) }
}

/// NCC ID.
pub fn ncc_id() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_ncc_id()) }
}

/// OTP version.
pub fn otp_version() -> OtpVersion {
    unsafe { sys::furi_hal_version_get_otp_version() }
}

/// Hardware version.
pub fn hardware_version() -> u8 {
    unsafe { sys::furi_hal_version_get_hw_version() }
}

/// Hardware target.
pub fn hardware_target() -> u8 {
    unsafe { sys::furi_hal_version_get_hw_target() }
}

/// Hardware body.
pub fn hardware_body() -> u8 {
    unsafe { sys::furi_hal_version_get_hw_body() }
}

/// Hardware body color.
pub fn hardware_color() -> Color {
    unsafe { sys::furi_hal_version_get_hw_color() }
}

/// Hardware interconnect.
pub fn hardware_interconnect() -> u8 {
    unsafe { sys::furi_hal_version_get_hw_connect() }
}

/// Hardware region.
pub fn hardware_region() -> Region {
    unsafe { sys::furi_hal_version_get_hw_region() }
}

/// Hardware region name.
pub fn hardware_region_name() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_hw_region_name()) }
}

/// Hardware display.
pub fn hardware_display() -> Display {
    unsafe { sys::furi_hal_version_get_hw_display() }
}

/// Hardware manufacture timestamp.
pub fn hardware_timestamp() -> u32 {
    unsafe { sys::furi_hal_version_get_hw_timestamp() }
}

/// Target name.
pub fn name() -> Option<&'static CStr> {
    let name_ptr = unsafe { sys::furi_hal_version_get_name_ptr() };
    if name_ptr.is_null() {
        return None;
    }

    Some(unsafe { CStr::from_ptr(name_ptr) })
}

/// Target device name.
pub fn device_name() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_device_name_ptr()) }
}

/// BLE local device name.
pub fn ble_local_device_name() -> &'static CStr {
    unsafe { CStr::from_ptr(sys::furi_hal_version_get_ble_local_device_name_ptr()) }
}

/// BLE MAC address.
pub fn ble_mac() -> &'static [u8] {
    const BLE_MAC_SIZE: usize = 6;
    let ble_mac_ptr = unsafe { sys::furi_hal_version_get_ble_mac() };

    unsafe { &*slice_from_raw_parts(ble_mac_ptr, BLE_MAC_SIZE) }
}

/// Firmware version.
pub fn firmware_version() -> &'static Version {
    // SAFETY: Returned pointer is a pointer to static, so never NULL.
    unsafe { Version::from_ptr(sys::furi_hal_version_get_firmware_version()) }
}

/// Platform UID
pub fn uid() -> &'static [u8] {
    let uid = unsafe { sys::furi_hal_version_uid() };
    let uid_size = unsafe { sys::furi_hal_version_uid_size() };

    unsafe { &*slice_from_raw_parts(uid, uid_size) }
}
