use flipperzero_sys as sys;

/// FlipperZero apps that can generate [`Deed`]s.
///
/// This list may grow over time, and it is not recommended to exhaustively match against
/// it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum App {
    SubGhz,
    Rfid,
    Nfc,
    Ir,
    Ibutton,
    Badusb,
    Plugin,

    /// Any `DolphinApp` enum value from the Flipper Zero SDK that's not part of this
    /// list.
    ///
    /// Apps that are `Uncategorized` now may move to a different or a new [`App`] variant
    /// in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::DolphinApp),
}

impl App {
    fn from_raw(raw: sys::DolphinApp) -> Self {
        match raw {
            sys::DolphinAppSubGhz => App::SubGhz,
            sys::DolphinAppRfid => App::Rfid,
            sys::DolphinAppNfc => App::Nfc,
            sys::DolphinAppIr => App::Ir,
            sys::DolphinAppIbutton => App::Ibutton,
            sys::DolphinAppBadusb => App::Badusb,
            sys::DolphinAppPlugin => App::Plugin,
            raw => App::Uncategorized(raw),
        }
    }

    fn to_raw(self) -> sys::DolphinApp {
        match self {
            App::SubGhz => sys::DolphinAppSubGhz,
            App::Rfid => sys::DolphinAppRfid,
            App::Nfc => sys::DolphinAppNfc,
            App::Ir => sys::DolphinAppIr,
            App::Ibutton => sys::DolphinAppIbutton,
            App::Badusb => sys::DolphinAppBadusb,
            App::Plugin => sys::DolphinAppPlugin,
            App::Uncategorized(raw) => raw,
        }
    }

    /// Returns the limit for this app.
    ///
    /// The FlipperZero SDK refers to returned value as `icounter_limit`.
    pub fn limit(self) -> u8 {
        unsafe { sys::dolphin_deed_get_app_limit(self.to_raw()) }
    }
}

/// Deeds that can contribute to the level of your [`Dolphin`].
///
/// This list may grow over time, and it is not recommended to exhaustively match against
/// it.
///
/// [`Dolphin`]: super::Dolphin
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Deed {
    SubGhzReceiverInfo,
    SubGhzSave,
    SubGhzRawRec,
    SubGhzAddManually,
    SubGhzSend,
    SubGhzFrequencyAnalyzer,

    RfidRead,
    RfidReadSuccess,
    RfidSave,
    RfidEmulate,
    RfidAdd,

    NfcRead,
    NfcReadSuccess,
    NfcSave,
    NfcDetectReader,
    NfcEmulate,
    NfcKeyAdd,
    NfcAddSave,
    NfcAddEmulate,

    IrSend,
    IrLearnSuccess,
    IrSave,

    IbuttonRead,
    IbuttonReadSuccess,
    IbuttonSave,
    IbuttonEmulate,
    IbuttonAdd,

    BadUsbPlayScript,

    U2fAuthorized,
    GpioUartBridge,

    PluginStart,
    PluginGameStart,
    PluginGameWin,

    /// Any `DolphinDeed` enum value from the Flipper Zero SDK that's not part of this
    /// list.
    ///
    /// Deeds that are `Uncategorized` now may move to a different or a new [`Deed`]
    /// variant in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::DolphinDeed),
}

impl Deed {
    pub(super) fn to_raw(self) -> sys::DolphinDeed {
        match self {
            Deed::SubGhzReceiverInfo => sys::DolphinDeedSubGhzReceiverInfo,
            Deed::SubGhzSave => sys::DolphinDeedSubGhzSave,
            Deed::SubGhzRawRec => sys::DolphinDeedSubGhzRawRec,
            Deed::SubGhzAddManually => sys::DolphinDeedSubGhzAddManually,
            Deed::SubGhzSend => sys::DolphinDeedSubGhzSend,
            Deed::SubGhzFrequencyAnalyzer => sys::DolphinDeedSubGhzFrequencyAnalyzer,
            Deed::RfidRead => sys::DolphinDeedRfidRead,
            Deed::RfidReadSuccess => sys::DolphinDeedRfidReadSuccess,
            Deed::RfidSave => sys::DolphinDeedRfidSave,
            Deed::RfidEmulate => sys::DolphinDeedRfidEmulate,
            Deed::RfidAdd => sys::DolphinDeedRfidAdd,
            Deed::NfcRead => sys::DolphinDeedNfcRead,
            Deed::NfcReadSuccess => sys::DolphinDeedNfcReadSuccess,
            Deed::NfcSave => sys::DolphinDeedNfcSave,
            Deed::NfcDetectReader => sys::DolphinDeedNfcDetectReader,
            Deed::NfcEmulate => sys::DolphinDeedNfcEmulate,
            Deed::NfcKeyAdd => sys::DolphinDeedNfcKeyAdd,
            Deed::NfcAddSave => sys::DolphinDeedNfcAddSave,
            Deed::NfcAddEmulate => sys::DolphinDeedNfcAddEmulate,
            Deed::IrSend => sys::DolphinDeedIrSend,
            Deed::IrLearnSuccess => sys::DolphinDeedIrLearnSuccess,
            Deed::IrSave => sys::DolphinDeedIrSave,
            Deed::IbuttonRead => sys::DolphinDeedIbuttonRead,
            Deed::IbuttonReadSuccess => sys::DolphinDeedIbuttonReadSuccess,
            Deed::IbuttonSave => sys::DolphinDeedIbuttonSave,
            Deed::IbuttonEmulate => sys::DolphinDeedIbuttonEmulate,
            Deed::IbuttonAdd => sys::DolphinDeedIbuttonAdd,
            Deed::BadUsbPlayScript => sys::DolphinDeedBadUsbPlayScript,
            Deed::U2fAuthorized => sys::DolphinDeedU2fAuthorized,
            Deed::GpioUartBridge => sys::DolphinDeedGpioUartBridge,
            Deed::PluginStart => sys::DolphinDeedPluginStart,
            Deed::PluginGameStart => sys::DolphinDeedPluginGameStart,
            Deed::PluginGameWin => sys::DolphinDeedPluginGameWin,
            Deed::Uncategorized(raw) => raw,
        }
    }

    /// Returns the FlipperZero app that this deed canonically corresponds to.
    pub fn app(self) -> App {
        App::from_raw(unsafe { sys::dolphin_deed_get_app(self.to_raw()) })
    }

    /// Returns the weight of this deed.
    ///
    /// The FlipperZero SDK refers to returned value as `icounter`.
    pub fn weight(self) -> u8 {
        unsafe { sys::dolphin_deed_get_weight(self.to_raw()) }
    }
}
