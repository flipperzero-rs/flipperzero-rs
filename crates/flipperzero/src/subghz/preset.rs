use flipperzero_sys as sys;
use ufmt::derive::uDebug;

#[derive(Clone, Copy, Debug, uDebug)]
pub enum SubGhzPreset<'a> {
    /// default configuration
    IDLE,
    /// OOK, bandwidth 270kHz, asynchronous
    Ook270Async,
    /// OOK, bandwidth 650kHz, asynchronous
    Ook650Async,
    /// FM, deviation 2.380371 kHz, asynchronous
    _2FSKDev238Async,
    /// FM, deviation 47.60742 kHz, asynchronous
    _2FSKDev476Async,
    /// MSK, deviation 47.60742 kHz, 99.97Kb/s, asynchronous
    MSK99_97KbAsync,
    /// GFSK, deviation 19.042969 kHz, 9.996Kb/s, asynchronous
    GFSK9_99KbAsync,
    Custom(CustomSubGhzPreset<'a>),
}

impl<'a> SubGhzPreset<'a> {
    pub fn into_furi_preset(self) -> sys::FuriHalSubGhzPreset {
        match self {
            Self::IDLE => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetIDLE,
            Self::Ook270Async => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetOok270Async,
            Self::Ook650Async => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetOok650Async,
            Self::_2FSKDev238Async => sys::FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev238Async,
            Self::_2FSKDev476Async => sys::FuriHalSubGhzPreset_FuriHalSubGhzPreset2FSKDev476Async,
            Self::MSK99_97KbAsync => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetMSK99_97KbAsync,
            Self::GFSK9_99KbAsync => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetGFSK9_99KbAsync,
            Self::Custom(_) => sys::FuriHalSubGhzPreset_FuriHalSubGhzPresetCustom,
        }
    }
}

/// This struct is defined for creating custom configurations. It enforces the null padding needed at the end. along with the power amplifier config?
#[derive(Clone, Copy, Debug, uDebug)]
pub struct CustomSubGhzPreset<'a>(&'a [u8]);

impl<'a> CustomSubGhzPreset<'a> {
    /// # Safety
    ///
    /// Read the manual for the CC1101 chip and review the [`furi_hal_subghz_load_custom_preset()`](https://github.com/flipperdevices/flipperzero-firmware/blob/0eaad8bf64f01a6f932647a9cda5475dd9ea1524/targets/f7/furi_hal/furi_hal_subghz.c#L162) function provided by the flipper API
    pub unsafe fn from_raw(raw_config: &[u8]) -> CustomSubGhzPreset {
        CustomSubGhzPreset(raw_config)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}
