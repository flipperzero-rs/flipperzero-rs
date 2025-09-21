//! I2C interface for the Flipper Zero.

use core::fmt;

use flipperzero_sys as sys;

use crate::furi::time::FuriDuration;

/// The address of the internal LED controller.
///
/// This is an [LP5562].
///
/// [LP5562]: https://www.ti.com/lit/ds/symlink/lp5562.pdf
pub const INTERNAL_LED_CONTROLLER: DeviceAddress = DeviceAddress::new(0x30);

/// The address of the internal battery fuel gauge.
///
/// This is a [BQ27220].
///
/// [BQ27220]: https://www.ti.com/lit/ds/symlink/bq27220.pdf
pub const INTERNAL_BATTERY_FUEL_GAUGE: DeviceAddress = DeviceAddress::new(0x55);

/// The address of the internal battery charger.
///
/// This is a [BQ25896].
///
/// [BQ25896]: https://www.ti.com/lit/ds/symlink/bq25896.pdf
pub const INTERNAL_BATTERY_CHARGER: DeviceAddress = DeviceAddress::new(0x6B);

/// A 7-bit I2C device address.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DeviceAddress(u8);

impl fmt::Debug for DeviceAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DeviceAddress")
            .field(&(self.0 >> 1))
            .finish()
    }
}

impl ufmt::uDebug for DeviceAddress {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_tuple("DeviceAddress")?
            .field(&(self.0 >> 1))?
            .finish()
    }
}

impl DeviceAddress {
    /// Constructs a `DeviceAddress` from its 7-bit value.
    ///
    /// The upper bit of `addr` is ignored; `0x00` and `0x80` are treated as the same
    /// address.
    pub const fn new(addr: u8) -> Self {
        // The SDK takes addresses in 8-bit form, so we store them that way to reduce the
        // number of places we need to handle the shift.
        Self(addr << 1)
    }
}

/// A handle to an I2C bus.
#[derive(Clone, Copy)]
enum BusKind {
    Internal,
    External,
}

/// An I2C bus on the Flipper Zero.
#[derive(Clone, Copy)]
pub struct Bus(BusKind);

impl Bus {
    /// The internal (power) I2C bus.
    ///
    /// This has three devices:
    /// - [`INTERNAL_LED_CONTROLLER`]
    /// - [`INTERNAL_BATTERY_FUEL_GAUGE`]
    /// - [`INTERNAL_BATTERY_CHARGER`]
    pub const INTERNAL: Self = Self(BusKind::Internal);

    /// The external I2C bus.
    ///
    /// - Connect `SCL` to pin `C0`.
    /// - Connect `SDA` to pin `C1`.
    ///
    /// # Warning
    ///
    /// Only connect 3.3V peripherals directly to your Flipper Zero, or you risk damaging
    /// it. For I2C devices that operate at different voltages, use a level shifter.
    pub const EXTERNAL: Self = Self(BusKind::External);

    /// Acquires a handle to the given I2C bus.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn acquire(self) -> BusHandle {
        // SAFETY: We block until we acquire a handle to the selected bus, so nothing else
        // will be using it while we have a raw pointer to it. We don't convert this to a
        // `&'static mut` reference because this will be disallowed in Rust 2024 edition.
        BusHandle::acquire(match self.0 {
            BusKind::Internal => &raw const sys::furi_hal_i2c_handle_power,
            BusKind::External => &raw const sys::furi_hal_i2c_handle_external,
        })
    }

    /// Acquires a handle to the given I2C bus and then runs the given function.
    ///
    /// Blocks indefinitely until the bus is available.
    pub fn with_handle<T>(self, f: impl FnOnce(BusHandle) -> T) -> T {
        f(self.acquire())
    }
}

/// A handle to an I2C bus on the Flipper Zero.
pub struct BusHandle {
    handle: *const sys::FuriHalI2cBusHandle,
}

impl Drop for BusHandle {
    fn drop(&mut self) {
        unsafe { sys::furi_hal_i2c_release(self.handle) };
    }
}

impl BusHandle {
    /// Acquires a handle to the given I2C bus.
    ///
    /// Blocks indefinitely until the Flipper Zero bus is locally available.
    fn acquire(handle: *const sys::FuriHalI2cBusHandle) -> Self {
        unsafe { sys::furi_hal_i2c_acquire(handle) };
        Self { handle }
    }

    /// Enumerates the devices that are present and ready on this bus.
    pub fn enumerate_devices(
        &mut self,
        per_device_timeout: FuriDuration,
    ) -> impl Iterator<Item = DeviceAddress> + '_ {
        (0x00..0x80).filter_map(move |addr| {
            let device = DeviceAddress::new(addr);
            self.is_device_ready(device, per_device_timeout)
                .then_some(device)
        })
    }

    /// Checks if the device with address `i2c_addr` is present and ready on the bus.
    ///
    /// Returns `true` if the device is present and ready, false otherwise.
    pub fn is_device_ready(&mut self, device: DeviceAddress, timeout: FuriDuration) -> bool {
        unsafe {
            sys::furi_hal_i2c_is_device_ready(self.handle, device.0, timeout.as_millis() as u32)
        }
    }

    /// Reads the 8-bit register at `reg_addr` on `device`.
    pub fn read_u8(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        timeout: FuriDuration,
    ) -> Result<u8, Error> {
        let mut data = 0;
        if unsafe {
            sys::furi_hal_i2c_read_reg_8(
                self.handle,
                device.0,
                reg_addr,
                &mut data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(data)
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Reads the 16-bit register at `reg_addr` on `device`.
    pub fn read_u16(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        timeout: FuriDuration,
    ) -> Result<u16, Error> {
        let mut data = 0;
        if unsafe {
            sys::furi_hal_i2c_read_reg_16(
                self.handle,
                device.0,
                reg_addr,
                &mut data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(data)
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Reads `device`'s memory starting at `mem_addr` into the given buffer.
    pub fn read_exact(
        &mut self,
        device: DeviceAddress,
        mem_addr: u8,
        buf: &mut [u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_read_mem(
                self.handle,
                device.0,
                mem_addr,
                buf.as_mut_ptr(),
                buf.len(),
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given value into the 8-bit register at `reg_addr` on `device`.
    pub fn write_u8(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        data: u8,
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_reg_8(
                self.handle,
                device.0,
                reg_addr,
                data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given value into the 16-bit register at `reg_addr` on `device`.
    pub fn write_u16(
        &mut self,
        device: DeviceAddress,
        reg_addr: u8,
        data: u16,
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_reg_16(
                self.handle,
                device.0,
                reg_addr,
                data,
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Writes the given data into `device`'s memory starting at `mem_addr`.
    pub fn write_all(
        &mut self,
        device: DeviceAddress,
        mem_addr: u8,
        data: &[u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        if unsafe {
            sys::furi_hal_i2c_write_mem(
                self.handle,
                device.0,
                mem_addr,
                data.as_ptr(),
                data.len(),
                timeout.as_millis() as u32,
            )
        } {
            Ok(())
        } else {
            Err(Error::TransferFailed)
        }
    }

    /// Reads data from `device` and writes it to the `data` buffer.
    pub fn tx(
        &mut self,
        device: DeviceAddress,
        data: &[u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        unsafe {
            sys::furi_hal_i2c_tx(
                self.handle,
                device.0,
                data.as_ptr(),
                data.len(),
                timeout.as_millis() as u32,
            )
        }
        .then_some(())
        .ok_or(Error::TransferFailed)
    }

    /// Writes the given data to `device`.
    pub fn rx(
        &mut self,
        device: DeviceAddress,
        data: &mut [u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        unsafe {
            sys::furi_hal_i2c_rx(
                self.handle,
                device.0,
                data.as_mut_ptr(),
                data.len(),
                timeout.as_millis() as u32,
            )
        }
        .then_some(())
        .ok_or(Error::TransferFailed)
    }

    /// Writes the data in `write` to `device` and then reads from it into the `read` buffer.
    pub fn trx(
        &mut self,
        device: DeviceAddress,
        write: &[u8],
        read: &mut [u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        unsafe {
            sys::furi_hal_i2c_trx(
                self.handle,
                device.0,
                write.as_ptr(),
                write.len(),
                read.as_mut_ptr(),
                read.len(),
                timeout.as_millis() as u32,
            )
        }
        .then_some(())
        .ok_or(Error::TransferFailed)
    }

    /// Execute the provided operations on the I2C bus.
    ///
    /// Transaction contract:
    /// - Before executing the first operation an ST is sent automatically. This is followed by SAD+R/W as appropriate.
    /// - Data from adjacent operations of the same type are sent after each other without an SP or SR.
    /// - Between adjacent operations of a different type an SR and SAD+R/W is sent.
    /// - After executing the last operation an SP is sent automatically.
    /// - If the last operation is a `Read` the master does not send an acknowledge for the last byte.
    ///
    /// - `ST` = start condition
    /// - `SAD+R/W` = slave address followed by bit 1 to indicate reading or 0 to indicate writing
    /// - `SR` = repeated start condition
    /// - `SP` = stop condition
    pub fn transaction(
        &mut self,
        device: DeviceAddress,
        operations: &mut [Operation],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        self.transaction_impl(device, operations, timeout)
    }

    // This is similar to `trx`, the only difference being that it sends a RESTART condition
    // between the two transfers instead of STOP + START
    #[cfg(any(feature = "embedded-hal", feature = "embedded-hal-0"))]
    fn write_read_impl(
        &mut self,
        device: DeviceAddress,
        write: &[u8],
        read: &mut [u8],
        timeout: FuriDuration,
    ) -> Result<(), Error> {
        unsafe {
            sys::furi_hal_i2c_tx_ext(
                self.handle,
                device.0.into(),
                false,
                write.as_ptr(),
                write.len(),
                sys::FuriHalI2cBeginStart,
                sys::FuriHalI2cEndAwaitRestart,
                timeout.as_millis() as u32,
            ) && sys::furi_hal_i2c_rx_ext(
                self.handle,
                device.0.into(),
                false,
                read.as_mut_ptr(),
                read.len(),
                sys::FuriHalI2cBeginRestart,
                sys::FuriHalI2cEndStop,
                timeout.as_millis() as u32,
            )
        }
        .then_some(())
        .ok_or(Error::TransferFailed)
    }

    // This function is generic to allow the implementation of both versions of the embedded_hal
    // transaction traits
    fn transaction_impl<'a, O>(
        &mut self,
        device: DeviceAddress,
        operations: &mut [O],
        timeout: FuriDuration,
    ) -> Result<(), Error>
    where
        O: OperationLike + 'a,
    {
        use sys::{
            FuriHalI2cBeginRestart as BeginRestart, FuriHalI2cBeginResume as BeginResume,
            FuriHalI2cBeginStart as BeginStart, FuriHalI2cEndAwaitRestart as EndAwaitRestart,
            FuriHalI2cEndPause as EndPause, FuriHalI2cEndStop as EndStop,
        };

        let mut operations = operations.iter_mut().peekable();
        let mut start = BeginStart;
        let address = device.0.into();

        while let Some(op) = operations.next() {
            let (end, next_start) = match (op.kind(), operations.peek().map(|next| next.kind())) {
                (OperationKind::Read, Some(OperationKind::Read))
                | (OperationKind::Write, Some(OperationKind::Write)) => (EndPause, BeginResume),
                (_, Some(_)) => (EndAwaitRestart, BeginRestart),
                (_, None) => (EndStop, BeginStart),
            };

            let result = unsafe {
                match op.as_op() {
                    Operation::Read(buffer) => flipperzero_sys::furi_hal_i2c_rx_ext(
                        self.handle,
                        address,
                        false,
                        buffer.as_mut_ptr(),
                        buffer.len(),
                        start,
                        end,
                        timeout.as_millis() as u32,
                    ),
                    Operation::Write(buffer) => flipperzero_sys::furi_hal_i2c_tx_ext(
                        self.handle,
                        address,
                        false,
                        buffer.as_ptr(),
                        buffer.len(),
                        start,
                        end,
                        timeout.as_millis() as u32,
                    ),
                }
            };

            if !result {
                return Err(Error::TransferFailed);
            }

            start = next_start;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    TransferFailed,
}

/// I2C operation.
///
/// Several operations can be combined as part of a transaction.
#[derive(Debug, PartialEq, Eq)]
pub enum Operation<'a> {
    /// Read data into the provided buffer
    Read(&'a mut [u8]),
    /// Write data from the provided buffer
    Write(&'a [u8]),
}

// These exist to allow compatibility with both embedded_hal 1.0 and 0.2 versions of the Operation
// enum

enum OperationKind {
    Read,
    Write,
}

trait OperationLike {
    fn as_op(&mut self) -> Operation<'_>;
    fn kind(&self) -> OperationKind;
}

impl OperationLike for Operation<'_> {
    fn as_op(&mut self) -> Operation<'_> {
        match self {
            Operation::Read(buffer) => Operation::Read(buffer),
            Operation::Write(buffer) => Operation::Write(buffer),
        }
    }

    fn kind(&self) -> OperationKind {
        match self {
            Operation::Read(_) => OperationKind::Read,
            Operation::Write(_) => OperationKind::Write,
        }
    }
}

// embedded_hal specific

/// An I2C bus implementing the embedded-hal traits
///
/// It acquires and releases a handle to the underlying bus for each function, similar to
/// [embedded-hal-bus](https://docs.rs/embedded-hal-bus/0.1.0-rc.1/embedded_hal_bus/index.html)'
/// [MutexDevice](https://docs.rs/embedded-hal-bus/0.1.0-rc.1/embedded_hal_bus/i2c/struct.MutexDevice.html).
/// It uses the same timeout duration for each operation.
#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-0"))]
pub struct EmbeddedHalBus {
    bus: Bus,
    /// The timeout used for each operation
    timeout: FuriDuration,
}

#[cfg(any(feature = "embedded-hal", feature = "embedded-hal-0"))]
impl EmbeddedHalBus {
    pub fn new(bus: Bus, timeout: FuriDuration) -> Self {
        Self { bus, timeout }
    }

    pub fn set_timeout(&mut self, timeout: FuriDuration) {
        self.timeout = timeout
    }
}

// embedded_hal 1.0 implementations

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::ErrorType for EmbeddedHalBus {
    type Error = Error;
}

#[cfg(feature = "embedded-hal")]
impl OperationLike for embedded_hal::i2c::Operation<'_> {
    fn as_op(&mut self) -> Operation<'_> {
        match self {
            embedded_hal::i2c::Operation::Read(buffer) => Operation::Read(buffer),
            embedded_hal::i2c::Operation::Write(buffer) => Operation::Write(buffer),
        }
    }

    fn kind(&self) -> OperationKind {
        match self {
            embedded_hal::i2c::Operation::Read(_) => OperationKind::Read,
            embedded_hal::i2c::Operation::Write(_) => OperationKind::Write,
        }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::I2c for EmbeddedHalBus {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .transaction_impl(DeviceAddress::new(address), operations, self.timeout)
    }

    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .rx(DeviceAddress::new(address), read, self.timeout)
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .tx(DeviceAddress::new(address), write, self.timeout)
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .write_read_impl(DeviceAddress::new(address), write, read, self.timeout)
    }
}

// embedded_hal 0.2 implementations

#[cfg(feature = "embedded-hal-0")]
impl OperationLike for embedded_hal_0::blocking::i2c::Operation<'_> {
    fn as_op(&mut self) -> Operation<'_> {
        match self {
            embedded_hal_0::blocking::i2c::Operation::Read(buffer) => Operation::Read(buffer),
            embedded_hal_0::blocking::i2c::Operation::Write(buffer) => Operation::Write(buffer),
        }
    }

    fn kind(&self) -> OperationKind {
        match self {
            embedded_hal_0::blocking::i2c::Operation::Read(_) => OperationKind::Read,
            embedded_hal_0::blocking::i2c::Operation::Write(_) => OperationKind::Write,
        }
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::i2c::Read for EmbeddedHalBus {
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .rx(DeviceAddress::new(address), buffer, self.timeout)
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::i2c::Write for EmbeddedHalBus {
    type Error = Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .tx(DeviceAddress::new(address), bytes, self.timeout)
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::i2c::WriteRead for EmbeddedHalBus {
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .write_read_impl(DeviceAddress::new(address), bytes, buffer, self.timeout)
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::i2c::Transactional for EmbeddedHalBus {
    type Error = Error;

    fn exec(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_0::blocking::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus
            .acquire()
            .transaction_impl(DeviceAddress::new(address), operations, self.timeout)
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::{
        Bus, DeviceAddress, INTERNAL_BATTERY_CHARGER, INTERNAL_BATTERY_FUEL_GAUGE,
        INTERNAL_LED_CONTROLLER,
    };
    use crate::furi::time::FuriDuration;

    #[test]
    fn enumerate_devices() {
        const INTERNAL_DEVICES: &[DeviceAddress] = &[
            INTERNAL_LED_CONTROLLER,
            INTERNAL_BATTERY_FUEL_GAUGE,
            INTERNAL_BATTERY_CHARGER,
        ];

        let mut bus = Bus::INTERNAL.acquire();
        for (i, device) in bus
            .enumerate_devices(FuriDuration::from_millis(50))
            .enumerate()
        {
            if let Some(&expected) = INTERNAL_DEVICES.get(i) {
                assert_eq!(expected, device);
            }
        }
    }
}
