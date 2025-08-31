//! UART serial interface.

use std::io;

use bytes::BytesMut;
use once_cell::sync::Lazy;
use regex::bytes::Regex as BytesRegex;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};

/// STMicroelectronics Virtual COM Port
const HWID: (u16, u16) = (0x0483, 0x5740);
const BUF_SIZE: usize = 1024;
pub const BAUD_115200: u32 = 115200;

pub static CLI_PROMPT: Lazy<BytesRegex> = Lazy::new(|| BytesRegex::new(r">: ").unwrap());
pub static CLI_EOL: Lazy<BytesRegex> = Lazy::new(|| BytesRegex::new(r"\r\n").unwrap());
pub static CLI_READY: Lazy<BytesRegex> = Lazy::new(|| BytesRegex::new(r"Ready\?\r\n").unwrap());

/// Try to find the Flipper Zero USB serial port.
pub fn find_flipperzero(port_name: Option<&str>) -> Option<SerialPortInfo> {
    let ports = serialport::available_ports().ok()?;

    ports.into_iter().find(|p| {
        if let Some(port) = port_name {
            // Search for port by name
            p.port_name == port
        } else {
            // Auto-detect port
            matches!(&p.port_type, SerialPortType::UsbPort(usb) if (usb.vid, usb.pid) == HWID)
        }
    })
}

/// Serial Command-line interface.
pub struct SerialCli {
    reader: SerialReader,
}

impl SerialCli {
    /// Create a new [`SerialCli`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            reader: SerialReader::new(port),
        }
    }

    /// Get reference to underlying [`SerialPort`].
    pub fn port(&self) -> &dyn SerialPort {
        self.reader.get_ref()
    }

    /// Get mutable reference to underlying [`SerialPort`].
    pub fn port_mut(&mut self) -> &mut dyn SerialPort {
        self.reader.get_mut()
    }

    /// Reset serial to prompt.
    pub fn start(&mut self) -> io::Result<()> {
        self.port().clear(serialport::ClearBuffer::Input)?;
        self.port_mut()
            .write_data_terminal_ready(true)
            .expect("failed to set DTR");

        // Send command with known syntax to make sure buffer is flushed
        self.send_line("device_info")?;
        self.reader
            .read_until(&BytesRegex::new(r"hardware_model").unwrap(), true)?;

        // Read buffer until we get prompt
        self.read_until_prompt()?;

        Ok(())
    }

    /// Send line to device.
    pub fn send_line(&mut self, line: &str) -> io::Result<()> {
        write!(self.port_mut(), "{line}\r")
    }

    /// Send line to device and wait for next end-of-line.
    pub fn send_and_wait_eol(&mut self, line: &str) -> io::Result<BytesMut> {
        self.send_line(line)?;

        self.read_until_eol()
    }

    /// Send line to device and wait for next CLI prompt.
    pub fn send_and_wait_prompt(&mut self, line: &str) -> io::Result<BytesMut> {
        self.send_line(line)?;

        self.read_until_prompt()
    }

    /// Read until next CLI prompt.
    pub fn read_until_prompt(&mut self) -> io::Result<BytesMut> {
        self.reader.read_until(&CLI_PROMPT, true)
    }

    /// Read until next CLI "Ready?" prompt.
    pub fn read_until_ready(&mut self) -> io::Result<BytesMut> {
        self.reader.read_until(&CLI_READY, true)
    }

    /// Read until next end-of-line.
    pub fn read_until_eol(&mut self) -> io::Result<BytesMut> {
        self.reader.read_until(&CLI_EOL, true)
    }

    /// Consume command respose, checking for errors.
    pub fn consume_response(&mut self) -> io::Result<String> {
        let line = self.reader.read_until(&CLI_EOL, true)?;
        let line = String::from_utf8_lossy(&line);
        self.read_until_prompt()?;

        if let Some(error) = Self::get_error(&line) {
            return Err(io::Error::other(error));
        }

        Ok(line.into_owned())
    }

    /// Extract error text.
    pub fn get_error(data: &str) -> Option<&str> {
        let (_, text) = data.split_once("Storage error: ")?;

        Some(text.trim())
    }
}

/// Buffered reader for [`SerialPort`].
pub struct SerialReader {
    port: Box<dyn SerialPort>,
    buffer: BytesMut,
}

impl SerialReader {
    /// Create new [`SerialReader`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            port,
            buffer: BytesMut::with_capacity(BUF_SIZE),
        }
    }

    /// Get reference to underlying [`SerialPort`].
    pub fn get_ref(&self) -> &dyn SerialPort {
        self.port.as_ref()
    }

    /// Get mutable reference to underlying [`SerialPort`].
    pub fn get_mut(&mut self) -> &mut dyn SerialPort {
        self.port.as_mut()
    }

    /// Read from [`SerialPort`] until [`BytesRegex`] is matched.
    pub fn read_until(&mut self, regex: &BytesRegex, trim: bool) -> io::Result<BytesMut> {
        let mut buf = [0u8; BUF_SIZE];
        loop {
            if let Some(m) = regex.find(&self.buffer) {
                let start = m.start();
                let end = m.end();

                let mut data = self.buffer.split_to(end);
                if trim {
                    // Trim the matched pattern
                    data.truncate(data.len() - (end - start));
                }

                return Ok(data);
            }

            // We always read at least 1 byte.
            let n = (self.port.bytes_to_read()? as usize).clamp(1, buf.len());

            self.port.read_exact(&mut buf[..n])?;
            self.buffer.extend_from_slice(&buf[..n]);
        }
    }
}
