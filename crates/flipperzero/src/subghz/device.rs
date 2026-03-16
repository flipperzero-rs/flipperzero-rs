//! Safe wrapper utilizing the API provided in `lib/subghz/devices/devices.h`

use super::{error::SubGhzError, preset::SubGhzPreset};
use core::{ffi::CStr, ptr::null_mut};
use flipperzero_sys as sys;
use uom::si::{frequency::hertz, u32::Frequency};

pub struct SubGhz {
    device: &'static sys::SubGhzDevice,
}

impl SubGhz {
    /// For the internal device use [`cc1101_int`]
    ///
    /// [`cc1101_int`]: https://github.com/flipperdevices/flipperzero-firmware/blob/dev/lib/subghz/devices/cc1101_int/cc1101_int_interconnect.c
    pub fn subghz_devices_get_by_name(name: &CStr) -> Option<SubGhz> {
        unsafe { sys::subghz_devices_init() }

        // Safety: input type enforeced by CStr type.
        let device = unsafe { sys::subghz_devices_get_by_name(name.as_ptr()) };
        // Safety: This pointer should either be null or return a reference, I don't see how it could not.
        // Further, the reference should be static as it relates to the variable:
        // `static SubGhzDeviceRegistry* subghz_device_registry = NULL;` in `lib/subghz/devices/registry.c``
        let device = unsafe { device.as_ref() }?;
        Some(Self { device })
    }

    // No clue what sort of error may occur, if using the internal device, it will always return Ok()
    pub fn begin(&mut self) -> Result<(), SubGhzError> {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        let failed = unsafe { sys::subghz_devices_begin(self.device) };

        // False indicates Ok
        if failed {
            Err(SubGhzError::UnableToOpenDevice)
        } else {
            Ok(())
        }
    }

    pub fn is_connect(&mut self) -> bool {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe { sys::subghz_devices_is_connect(self.device) }
    }

    pub fn reset(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            sys::subghz_devices_reset(self.device);
        }
    }

    pub fn sleep(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            sys::subghz_devices_sleep(self.device);
        }
    }

    pub fn idle(&mut self) {
        unsafe {
            sys::subghz_devices_idle(self.device);
        }
    }

    pub fn load_preset(&mut self, preset: SubGhzPreset) {
        let furi_preset = preset.into_furi_preset();

        let preset_data = if let SubGhzPreset::Custom(data) = preset {
            data.as_ptr() as *mut u8
        } else {
            null_mut::<u8>()
        };

        // Safety: Type enforcement of the builtin presets, so a valid pointer is passed based on the input Preset.
        // Further, there is a cast to a mutable pointer, it looks like the chain of callback eventually calls a
        // function that takes a const pointer, so I don't believe that the data in the pointer get mutated.
        unsafe {
            sys::subghz_devices_load_preset(self.device, furi_preset, preset_data);
        }
    }

    // If the returned frequency from the internal API function call is 0, this function returns an error.
    pub fn set_frequency(&mut self, freq: Frequency) -> Result<Frequency, SubGhzError> {
        let actual_freq = unsafe { sys::subghz_devices_set_frequency(self.device, freq.value) };

        if actual_freq != 0 {
            Ok(Frequency::new::<hertz>(actual_freq))
            // Ok(actual_freq)
        } else {
            Err(SubGhzError::UnableToSetFrequency)
        }
    }

    pub fn is_frequency_valid(&mut self, freq: Frequency) -> bool {
        unsafe { sys::subghz_devices_is_frequency_valid(self.device, freq.value) }
    }

    pub fn is_frequency_allowed(&self, freq: Frequency) -> bool {
        unsafe { sys::furi_hal_region_is_frequency_allowed(freq.value) }
    }

    pub fn set_async_mirror_pin(&mut self) {
        unimplemented!("Should probably create safe abstraction around GpioPin first, have not investigated if this is needed");
    }

    pub fn get_data_gpio(&mut self) {
        unimplemented!("Should probably create safe abstraction around GpioPin first, have not investigated if this is needed");
    }

    pub fn set_tx(&mut self) -> Result<(), SubGhzError> {
        let result = unsafe { sys::subghz_devices_set_tx(self.device) };

        // Returns true if Ok
        if result {
            Ok(())
        } else {
            Err(SubGhzError::CannotTxOnFrequency)
        }
    }

    pub fn flush_tx(&mut self) {
        unsafe { sys::subghz_devices_flush_tx(self.device) };
    }

    pub fn set_rx(&mut self) {
        unsafe {
            sys::subghz_devices_set_rx(self.device);
        }
    }

    pub fn flush_rx(&mut self) {
        unsafe {
            sys::subghz_devices_flush_rx(self.device);
        }
    }

    pub fn get_rssi(&mut self) -> Option<f32> {
        let rssi_function = unsafe { self.device.interconnect.as_ref() }?.get_rssi?;
        Some(unsafe { rssi_function() })
    }

    pub fn get_lqi(&mut self) -> Option<u8> {
        let lqi_function = unsafe { self.device.interconnect.as_ref()?.get_lqi? };
        Some(unsafe { lqi_function() })
    }

    pub fn rx_pipe_not_empty(&mut self) -> bool {
        unsafe { sys::subghz_devices_rx_pipe_not_empty(self.device) }
    }

    pub fn is_rx_crc_valid(&mut self) -> bool {
        unsafe { sys::subghz_devices_is_rx_data_crc_valid(self.device) }
    }

    pub fn read_packet(&mut self, buf: &mut [u8]) -> usize {
        let mut size = u8::try_from(buf.len()).unwrap_or(u8::MAX);

        unsafe {
            sys::subghz_devices_read_packet(self.device, buf.as_mut_ptr(), &mut size);
        };
        size as usize
    }

    pub fn write_packet(&mut self, buf: &[u8]) -> Result<(), SubGhzError> {
        if buf.len() > 255 {
            return Err(SubGhzError::PacketTooLong { len: buf.len() });
        }

        let size = buf.len() as u8;

        unsafe {
            sys::subghz_devices_write_packet(self.device, buf.as_ptr(), size);
        }
        Ok(())
    }
}

impl Drop for SubGhz {
    fn drop(&mut self) {
        // Safety: self.device is not Null so this will not crash when furi_check is invoked.
        unsafe {
            sys::subghz_devices_end(self.device);
            sys::subghz_devices_deinit();
        }
    }
}
