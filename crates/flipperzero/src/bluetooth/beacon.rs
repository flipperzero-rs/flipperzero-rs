//! Bluetooth beacon.

use flipperzero_sys as sys;
use rand_core::RngCore;
use ufmt::derive::uDebug;
use uuid::Uuid;

use crate::furi::rng::HwRng;

/// The maximum length of the data broadcast by the beacon.
pub const BEACON_MAX_DATA_SIZE: usize = 31;

/// The Bluetooth beacon that can be broadcast from the Flipper Zero.
pub struct Beacon {
    config: sys::GapExtraBeaconConfig,
    data: [u8; BEACON_MAX_DATA_SIZE],
    data_len: usize,
}

impl Beacon {
    /// Acquires a handle to the Flipper Zero's Bluetooth beacon.
    ///
    /// Returns an error if the beacon was unconfigured and the default config could not
    /// be set.
    pub fn acquire() -> Result<Self, Error> {
        let existing_config = unsafe { sys::furi_hal_bt_extra_beacon_get_config().as_ref() };

        let config = if let Some(config) = existing_config {
            // If the address type is unknown to us, raise an error.
            if Address::from_config(config).is_none() {
                return Err(Error::UnknownAddressType);
            }

            *config
        } else {
            // No existing config, generate a new one with a random static address.
            let mut address = [0; 6];
            HwRng.fill_bytes(&mut address);
            address[5] |= 0b1100_0000;

            let config = sys::GapExtraBeaconConfig {
                min_adv_interval_ms: 50,
                max_adv_interval_ms: 150,
                adv_channel_map: sys::GapAdvChannelMapAll,
                adv_power_level: sys::GapAdvPowerLevel_0dBm,
                address_type: sys::GapAddressTypeRandom,
                address,
            };

            if !unsafe { sys::furi_hal_bt_extra_beacon_set_config(&config) } {
                return Err(Error::FailedToSetConfig);
            }

            config
        };

        // Extract the beacon's data, if any.
        let mut data = [0; BEACON_MAX_DATA_SIZE];
        let data_len = unsafe { sys::furi_hal_bt_extra_beacon_get_data(data.as_mut_ptr()) }.into();

        Ok(Self {
            config,
            data,
            data_len,
        })
    }

    fn update_config(&mut self) -> Result<(), Error> {
        let is_active = self.is_active();
        self.stop()?;

        if !unsafe { sys::furi_hal_bt_extra_beacon_set_config(&self.config) } {
            return Err(Error::FailedToSetConfig);
        }

        // Restart the beacon if it was running.
        if is_active {
            self.start()?;
        }

        Ok(())
    }

    /// Returns the current beacon address.
    pub fn address(&self) -> Address {
        Address::from_config(&self.config).expect("valid by construction")
    }

    /// Configures the beacon address.
    ///
    /// If a random static address is provided, the two most significant bits of the 48-bit
    /// address will be set to 1 (random static addresses are only 46 bits).
    pub fn set_address(&mut self, address: Address) -> Result<(), Error> {
        match address {
            Address::Public(address) => {
                self.config.address_type = sys::GapAddressTypePublic;
                self.config.address = address;
            }
            Address::RandomStatic(mut address) => {
                address[5] |= 0b1100_0000;
                self.config.address_type = sys::GapAddressTypeRandom;
                self.config.address = address;
            }
        }

        self.update_config()
    }

    /// Returns the last configured beacon data.
    pub fn data(&self) -> &[u8] {
        &self.data[..self.data_len]
    }

    /// Sets the beacon data to the given packet.
    ///
    /// Can be called in any state.
    ///
    /// Returns an error if the packet fails to be set.
    pub fn set_data_packet(&mut self, packet: AdPacket) -> Result<(), Error> {
        // SAFETY: AdPacket data length is correct by construction.
        if unsafe {
            !sys::furi_hal_bt_extra_beacon_set_data(packet.data.as_ptr(), packet.data_len as u8)
        } {
            Err(Error::FailedToSetData)
        } else {
            // Data set succeeded, update local cache.
            self.data = packet.data;
            self.data_len = packet.data_len;
            Ok(())
        }
    }

    /// Checks if the beacon is active.
    pub fn is_active(&self) -> bool {
        unsafe { sys::furi_hal_bt_extra_beacon_is_active() }
    }

    /// Starts the beacon.
    ///
    /// Returns an error if the beacon is not in the stopped state.
    pub fn start(&mut self) -> Result<(), Error> {
        if self.is_active() {
            Err(Error::BeaconActive)
        } else if unsafe { sys::furi_hal_bt_extra_beacon_start() } {
            Ok(())
        } else {
            Err(Error::FailedToStartBeacon)
        }
    }

    /// Stops the beacon.
    pub fn stop(&mut self) -> Result<(), Error> {
        if unsafe { sys::furi_hal_bt_extra_beacon_stop() } {
            Ok(())
        } else {
            Err(Error::FailedToStopBeacon)
        }
    }
}

/// The Bluetooth address of the [`Beacon`].
pub enum Address {
    Public([u8; 6]),
    RandomStatic([u8; 6]),
}

impl Address {
    fn from_config(config: &sys::GapExtraBeaconConfig) -> Option<Self> {
        match config.address_type {
            sys::GapAddressTypePublic => Some(Address::Public(config.address)),
            sys::GapAddressTypeRandom => Some(Address::RandomStatic(config.address)),
            _ => None,
        }
    }
}

/// An Advertisement Data packet.
pub struct AdPacket {
    data: [u8; BEACON_MAX_DATA_SIZE],
    data_len: usize,
}

impl AdPacket {
    /// Constructs an empty Advertisement Data packet.
    pub fn empty() -> Self {
        Self {
            data: [0; BEACON_MAX_DATA_SIZE],
            data_len: 0,
        }
    }

    /// Appends an Advertisement Data structure to the data packet.
    ///
    /// Returns an error if the resulting packet would be longer than
    /// [`BEACON_MAX_DATA_SIZE`].
    pub fn push<const N: usize>(&mut self, ad_type: AdType, ad_data: [u8; N]) -> Result<(), Error> {
        let new_len = self.data_len + 2 + N;
        if new_len > BEACON_MAX_DATA_SIZE {
            Err(Error::DataTooLong)
        } else {
            let buffer = &mut self.data[self.data_len..new_len];
            buffer[0] = 1 + N as u8;
            buffer[1] = ad_type.to_number();
            buffer[2..].copy_from_slice(&ad_data);
            self.data_len = new_len;
            Ok(())
        }
    }

    /// Appends an Advertisement Data structure to the data packet.
    ///
    /// Returns an error if the resulting packet would be longer than
    /// [`BEACON_MAX_DATA_SIZE`].
    pub fn push_var(&mut self, ad_type: AdType, ad_data: &[u8]) -> Result<(), Error> {
        let new_len = self.data_len + 2 + ad_data.len();
        if new_len > BEACON_MAX_DATA_SIZE {
            Err(Error::DataTooLong)
        } else {
            let buffer = &mut self.data[self.data_len..new_len];
            buffer[0] = 1 + ad_data.len() as u8;
            buffer[1] = ad_type.to_number();
            buffer[2..].copy_from_slice(ad_data);
            self.data_len = new_len;
            Ok(())
        }
    }

    /// Concatenates an Advertisement Data structure to this packet.
    ///
    /// Returns an error if the resulting packet would be longer than
    /// [`BEACON_MAX_DATA_SIZE`].
    pub fn with<const N: usize>(
        mut self,
        ad_type: AdType,
        ad_data: [u8; N],
    ) -> Result<Self, Error> {
        self.push(ad_type, ad_data)?;
        Ok(self)
    }

    /// Concatenates an Advertisement Data structure to this packet.
    ///
    /// Returns an error if the resulting packet would be longer than
    /// [`BEACON_MAX_DATA_SIZE`].
    pub fn with_var(mut self, ad_type: AdType, ad_data: &[u8]) -> Result<Self, Error> {
        self.push_var(ad_type, ad_data)?;
        Ok(self)
    }

    /// Constructs an iBeacon Advertisement Data packet.
    ///
    /// `signal_power` is the calibrated RSSI at 1 metre.
    pub fn i_beacon(uuid: Uuid, major: u16, minor: u16, signal_power: u8) -> Self {
        let major_bytes = major.to_be_bytes();
        let minor_bytes = minor.to_be_bytes();
        let mut mf_data = [
            0x4C,
            0x00,
            0x02,
            0x15,
            0x00, // UUID
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            major_bytes[0],
            major_bytes[1],
            minor_bytes[0],
            minor_bytes[1],
            signal_power,
        ];
        mf_data[4..20].copy_from_slice(uuid.as_bytes());

        Self::empty()
            // LE and BR/EDR flag.
            .with(AdType::Flags, [0x06])
            .expect("within size limits")
            // iBeacon AD structure
            .with(AdType::ManufacturerSpecificData, mf_data)
            .expect("within size limits")
    }

    /// Constructs an [AltBeacon] Advertisement Data packet.
    ///
    /// > For interoperability purposes, the first 16+ bytes of the beacon identifier
    /// > should be unique to the advertiser's organizational unit. Any remaining bytes of
    /// > the beacon identifier may be subdivided as needed for the use case.
    ///
    /// `ref_rssi` is the average RSSI at 1 metre.
    ///
    /// Returns `None` if `ref_rssi` is not between -127 and 0 inclusive.
    ///
    /// [AltBeacon]: https://github.com/AltBeacon/spec
    pub fn alt_beacon(
        mfg_id: u16,
        beacon_id: [u8; 20],
        ref_rssi: i8,
        mfg_reserved: u8,
    ) -> Option<Self> {
        if !(-127..=0).contains(&ref_rssi) {
            return None;
        }

        let mfg_id_bytes = mfg_id.to_le_bytes();
        let mut mf_data = [
            mfg_id_bytes[0],
            mfg_id_bytes[1],
            0xBE,
            0xAC,
            0x00, // Beacon ID
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            ref_rssi as u8,
            mfg_reserved,
        ];
        mf_data[4..24].copy_from_slice(&beacon_id);

        Some(
            Self::empty()
                // AltBeacon AD structure
                .with(AdType::ManufacturerSpecificData, mf_data)
                .expect("within size limits"),
        )
    }

    /// Constructs an [Eddystone-URL] Advertisement Data packet.
    ///
    /// [Eddystone-URL]: https://github.com/google/eddystone/tree/master/eddystone-url
    #[cfg(feature = "alloc")]
    pub fn eddystone_url(tx_power: u8, url_scheme: EddystoneUrlScheme, url: &str) -> Self {
        let mut service_data = alloc::vec![
            0xAA, // Eddystone ID
            0xFE,
            0x10, // Frame type
            tx_power,
            url_scheme.to_number(),
        ];

        let mut url_bytes = url.as_bytes();
        while url_bytes.len() >= 4 {
            let (byte, taken) = match (&url_bytes[..4], url_bytes.get(4)) {
                (b".com", Some(b'/')) => (0x00, 5),
                (b".org", Some(b'/')) => (0x01, 5),
                (b".edu", Some(b'/')) => (0x02, 5),
                (b".net", Some(b'/')) => (0x03, 5),
                (b".inf", Some(b'o')) => {
                    if let Some(b'/') = url_bytes.get(5) {
                        (0x04, 6)
                    } else {
                        (0x0B, 5)
                    }
                }
                (b".biz", Some(b'/')) => (0x05, 5),
                (b".gov", Some(b'/')) => (0x06, 5),
                (b".com", None) => (0x07, 4),
                (b".org", None) => (0x08, 4),
                (b".edu", None) => (0x09, 4),
                (b".net", None) => (0x0A, 4),
                (b".biz", None) => (0x0C, 4),
                (b".gov", None) => (0x0D, 4),
                ([b, ..], _) => (*b, 1),
                ([], _) => unreachable!(),
            };
            service_data.push(byte);
            url_bytes = &url_bytes[taken..];
        }
        service_data.extend_from_slice(url_bytes);

        Self::empty()
            // Service list containing the Eddystone ID
            .with(AdType::CompleteListOf16BitServiceClassUuids, [0xAA, 0xFE])
            .expect("within size limits")
            // Eddystone-URL AD structure
            .with_var(AdType::ServiceData16BitUuid, &service_data)
            .expect("within size limits")
    }
}

/// [Assigned numbers] for Advertising Data types.
///
/// [Assigned numbers]: https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/core/ad_types.yaml
#[derive(Clone, Copy, Debug, uDebug, PartialEq, Eq)]
pub enum AdType {
    // Core Specification Supplement, Part A, Section 1.3
    Flags,

    // Core Specification Supplement, Part A, Section 1.1
    IncompleteListOf16BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.1
    CompleteListOf16BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.1
    IncompleteListOf32BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.1
    CompleteListOf32BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.1
    IncompleteListOf128BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.1
    CompleteListOf128BitServiceClassUuids,

    // Core Specification Supplement, Part A, Section 1.2
    ShortenedLocalName,

    // Core Specification Supplement, Part A, Section 1.2
    CompleteLocalName,

    // Core Specification Supplement, Part A, Section 1.5
    TxPowerLevel,

    // Core Specification Supplement, Part A, Section 1.6
    ClassOfDevice,

    // Core Specification Supplement, Part A, Section 1.6
    SimplePairingHashC192,

    // Core Specification Supplement, Part A, Section 1.6
    SimplePairingRandomizerR192,

    // Device ID Profile
    DeviceId,

    // Core Specification Supplement, Part A, Section 1.8
    SecurityManagerTkValue,

    // Core Specification Supplement, Part A, Section 1.7
    SecurityManagerOutOfBandFlags,

    // Core Specification Supplement, Part A, Section 1.9
    PeripheralConnectionIntervalRange,

    // Core Specification Supplement, Part A, Section 1.10
    ListOf16BitServiceSolicitationUuids,

    // Core Specification Supplement, Part A, Section 1.10
    ListOf128BitServiceSolicitationUuids,

    // Core Specification Supplement, Part A, Section 1.11
    ServiceData16BitUuid,

    // Core Specification Supplement, Part A, Section 1.13
    PublicTargetAddress,

    // Core Specification Supplement, Part A, Section 1.14
    RandomTargetAddress,

    // Core Specification Supplement, Part A, Section 1.12
    Appearance,

    // Core Specification Supplement, Part A, Section 1.15
    AdvertisingInterval,

    // Core Specification Supplement, Part A, Section 1.16
    LeBluetoothDeviceAddress,

    // Core Specification Supplement, Part A, Section 1.17
    LeRole,

    // Core Specification Supplement, Part A, Section 1.6
    SimplePairingHashC256,

    // Core Specification Supplement, Part A, Section 1.6
    SimplePairingRandomizerR256,

    // Core Specification Supplement, Part A, Section 1.10
    ListOf32BitServiceSolicitationUuids,

    // Core Specification Supplement, Part A, Section 1.11
    ServiceData32BitUuid,

    // Core Specification Supplement, Part A, Section 1.11
    ServiceData128BitUuid,

    // Core Specification Supplement, Part A, Section 1.6
    LeSecureConnectionsConfirmationValue,

    // Core Specification Supplement, Part A, Section 1.6
    LeSecureConnectionsRandomValue,

    // Core Specification Supplement, Part A, Section 1.18
    Uri,

    // Indoor Positioning Service
    IndoorPositioning,

    // Transport Discovery Service
    TransportDiscoveryData,

    // Core Specification Supplement, Part A, Section 1.19
    LeSupportedFeatures,

    // Core Specification Supplement, Part A, Section 1.20
    ChannelMapUpdateIndication,

    // Mesh Profile Specification, Section 5.2.1
    PbAdv,

    // Mesh Profile Specification, Section 3.3.1
    MeshMessage,

    // Mesh Profile Specification, Section 3.9
    MeshBeacon,

    // Core Specification Supplement, Part A, Section 1.21
    BigInfo,

    // Core Specification Supplement, Part A, Section 1.22
    BroadcastCode,

    // Coordinated Set Identification Profile v1.0 or later
    ResolvableSetIdentifier,

    // Core Specification Supplement, Part A, Section 1.15
    AdvertisingIntervalLong,

    // Public Broadcast Profile v1.0 or later
    BroadcastName,

    // Core Specification Supplement, Part A, Section 1.23
    EncryptedAdvertisingData,

    // Core Specification Supplement, Part A, Section 1.24
    PeriodicAdvertisingResponseTimingInformation,

    // ESL Profile
    ElectronicShelfLabel,

    // 3D Synchronization Profile
    InformationData3D,

    // Core Specification Supplement, Part A, Section 1.4
    ManufacturerSpecificData,
}

impl AdType {
    fn to_number(self) -> u8 {
        match self {
            AdType::Flags => 0x01,
            AdType::IncompleteListOf16BitServiceClassUuids => 0x02,
            AdType::CompleteListOf16BitServiceClassUuids => 0x03,
            AdType::IncompleteListOf32BitServiceClassUuids => 0x04,
            AdType::CompleteListOf32BitServiceClassUuids => 0x05,
            AdType::IncompleteListOf128BitServiceClassUuids => 0x06,
            AdType::CompleteListOf128BitServiceClassUuids => 0x07,
            AdType::ShortenedLocalName => 0x08,
            AdType::CompleteLocalName => 0x09,
            AdType::TxPowerLevel => 0x0A,
            AdType::ClassOfDevice => 0x0D,
            AdType::SimplePairingHashC192 => 0x0E,
            AdType::SimplePairingRandomizerR192 => 0x0F,
            AdType::DeviceId => 0x10,
            AdType::SecurityManagerTkValue => 0x10,
            AdType::SecurityManagerOutOfBandFlags => 0x11,
            AdType::PeripheralConnectionIntervalRange => 0x12,
            AdType::ListOf16BitServiceSolicitationUuids => 0x14,
            AdType::ListOf128BitServiceSolicitationUuids => 0x15,
            AdType::ServiceData16BitUuid => 0x16,
            AdType::PublicTargetAddress => 0x17,
            AdType::RandomTargetAddress => 0x18,
            AdType::Appearance => 0x19,
            AdType::AdvertisingInterval => 0x1A,
            AdType::LeBluetoothDeviceAddress => 0x1B,
            AdType::LeRole => 0x1C,
            AdType::SimplePairingHashC256 => 0x1D,
            AdType::SimplePairingRandomizerR256 => 0x1E,
            AdType::ListOf32BitServiceSolicitationUuids => 0x1F,
            AdType::ServiceData32BitUuid => 0x20,
            AdType::ServiceData128BitUuid => 0x21,
            AdType::LeSecureConnectionsConfirmationValue => 0x22,
            AdType::LeSecureConnectionsRandomValue => 0x23,
            AdType::Uri => 0x24,
            AdType::IndoorPositioning => 0x25,
            AdType::TransportDiscoveryData => 0x26,
            AdType::LeSupportedFeatures => 0x27,
            AdType::ChannelMapUpdateIndication => 0x28,
            AdType::PbAdv => 0x29,
            AdType::MeshMessage => 0x2A,
            AdType::MeshBeacon => 0x2B,
            AdType::BigInfo => 0x2C,
            AdType::BroadcastCode => 0x2D,
            AdType::ResolvableSetIdentifier => 0x2E,
            AdType::AdvertisingIntervalLong => 0x2F,
            AdType::BroadcastName => 0x30,
            AdType::EncryptedAdvertisingData => 0x31,
            AdType::PeriodicAdvertisingResponseTimingInformation => 0x32,
            AdType::ElectronicShelfLabel => 0x34,
            AdType::InformationData3D => 0x3D,
            AdType::ManufacturerSpecificData => 0xFF,
        }
    }
}

/// The URL Scheme Prefix byte defines the identifier scheme, an optional prefix and how
/// the remainder of the URL is encoded.
#[cfg(feature = "alloc")]
#[derive(Clone, Copy, Debug, uDebug, PartialEq, Eq)]
pub enum EddystoneUrlScheme {
    /// URL prefixed with `http://www.`
    HttpWww,
    /// URL prefixed with `https://www.`
    HttpsWww,
    /// URL prefixed with `http://`
    Http,
    /// URL prefixed with `https://`
    Https,
}

#[cfg(feature = "alloc")]
impl EddystoneUrlScheme {
    fn to_number(self) -> u8 {
        match self {
            EddystoneUrlScheme::HttpWww => 0x00,
            EddystoneUrlScheme::HttpsWww => 0x01,
            EddystoneUrlScheme::Http => 0x02,
            EddystoneUrlScheme::Https => 0x03,
        }
    }
}

/// Errors that can occur while interacting with the [`Beacon`].
#[derive(Clone, Copy, Debug, uDebug, PartialEq, Eq)]
pub enum Error {
    BeaconActive,
    DataTooLong,
    FailedToSetConfig,
    FailedToSetData,
    FailedToStartBeacon,
    FailedToStopBeacon,
    UnknownAddressType,
}
