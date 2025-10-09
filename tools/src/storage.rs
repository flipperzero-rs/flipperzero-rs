//! Storage interface.

use std::fmt::Display;
use std::io::{Read, Write};
use std::ops::Add;
use std::path::Path;
use std::{fs, io};

use bytes::BytesMut;
use regex::Regex;
use serialport::SerialPort;

use crate::serial::{CLI_EOL, SerialCli};

const BUF_SIZE: usize = 1024;

/// Interface to Flipper device storage.
pub struct FlipperStorage {
    cli: SerialCli,
}

impl FlipperStorage {
    /// Create new [`FlipperStorage`] connected to a [`SerialPort`].
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            cli: SerialCli::new(port),
        }
    }

    /// Start serial interface.
    pub fn start(&mut self) -> io::Result<()> {
        self.cli.start()
    }

    /// Get reference to underlying [`SerialPort`].
    pub fn port(&self) -> &dyn SerialPort {
        self.cli.port()
    }

    /// Get mutable reference to underlying [`SerialPort`].
    pub fn port_mut(&mut self) -> &mut dyn SerialPort {
        self.cli.port_mut()
    }

    /// Get mutable reference to underlying [`SerialCli`].
    pub fn cli_mut(&mut self) -> &mut SerialCli {
        &mut self.cli
    }

    /// List files and directories on the device.
    pub fn list_tree(&mut self, path: &FlipperPath) -> io::Result<()> {
        // Note: The `storage list` command expects that paths do not end with a slash.
        self.cli
            .send_and_wait_eol(&format!("storage list {}", path))?;

        let data = self.cli.read_until_prompt()?;
        for line in CLI_EOL.split(&data).map(String::from_utf8_lossy) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(error) = SerialCli::get_error(line) {
                eprintln!("ERROR: {error}");
                continue;
            }

            if line == "Empty" {
                continue;
            }

            if let Some((typ, info)) = line.split_once(' ') {
                match typ {
                    // Directory
                    "[D]" => {
                        let path = path.clone() + info;

                        eprintln!("{path}");
                        self.list_tree(&path)?;
                    }
                    // File
                    "[F]" => {
                        if let Some((name, size)) = info.rsplit_once(' ') {
                            let path = path.clone() + name;

                            eprintln!("{path}, size {size}");
                        }
                    }
                    // We got something unexpected, ignore it
                    _ => (),
                }
            }
        }

        Ok(())
    }

    /// Send local file to the device.
    pub fn send_file(&mut self, from: impl AsRef<Path>, to: &FlipperPath) -> io::Result<()> {
        // Try to create directory on Flipper
        if let Some(dir) = to.0.rsplit_once('/') {
            self.mkdir(&FlipperPath::from(dir.0)).ok();
        }
        self.remove(to).ok();

        let mut file = fs::File::open(from.as_ref())?;

        let mut buf = [0u8; BUF_SIZE];
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }

            self.cli
                .send_and_wait_eol(&format!("storage write_chunk \"{to}\" {n}"))?;
            let line = self.cli.read_until_eol()?;
            let line = String::from_utf8_lossy(&line);

            if let Some(error) = SerialCli::get_error(&line) {
                self.cli.read_until_prompt()?;

                return Err(io::Error::other(error));
            }

            self.port_mut().write_all(&buf[..n])?;
            self.cli.read_until_prompt()?;
        }

        Ok(())
    }

    /// Receive remote file from the device.
    pub fn receive_file(&mut self, from: &FlipperPath, to: impl AsRef<Path>) -> io::Result<()> {
        let mut file = fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(to.as_ref())?;

        let data = self.read_file(from)?;
        file.write_all(&data)?;

        Ok(())
    }

    /// Read file data from the device.
    pub fn read_file(&mut self, path: &FlipperPath) -> io::Result<BytesMut> {
        self.cli
            .send_and_wait_eol(&format!("storage read_chunks \"{path}\" {}", BUF_SIZE))?;
        let line = self.cli.read_until_eol()?;
        let line = String::from_utf8_lossy(&line);

        if let Some(error) = SerialCli::get_error(&line) {
            self.cli.read_until_prompt()?;

            return Err(io::Error::other(error));
        }

        let (_, size) = line
            .split_once(": ")
            .ok_or_else(|| io::Error::other("failed to read chunk size"))?;
        let size: usize = size
            .parse()
            .map_err(|_| io::Error::other("failed to parse chunk size"))?;

        let mut data = BytesMut::with_capacity(BUF_SIZE);

        let mut buf = [0u8; BUF_SIZE];
        while data.len() < size {
            self.cli.read_until_ready()?;
            self.cli.send_line("y")?;

            let n = (size - data.len()).min(BUF_SIZE);
            self.port_mut().read_exact(&mut buf[..n])?;
            data.extend_from_slice(&buf[..n]);
        }

        Ok(data)
    }

    /// Does the file or directory exist on the device?
    pub fn exist(&mut self, path: &FlipperPath) -> io::Result<bool> {
        let exist = match self.stat(path) {
            Err(_err) => false,
            Ok(_) => true,
        };

        Ok(exist)
    }

    /// Does the directory exist on the device?
    pub fn exist_dir(&mut self, path: &FlipperPath) -> io::Result<bool> {
        let exist = match self.stat(path) {
            Err(_err) => false,
            Ok(stat) => stat.contains("Directory") || stat.contains("Storage"),
        };

        Ok(exist)
    }

    /// Does the file exist on the device?
    pub fn exist_file(&mut self, path: &FlipperPath) -> io::Result<bool> {
        let exist = match self.stat(path) {
            Err(_err) => false,
            Ok(stat) => stat.contains("File, size:"),
        };

        Ok(exist)
    }

    /// File size in bytes
    pub fn size(&mut self, path: &FlipperPath) -> io::Result<usize> {
        let line = self.stat(path)?;

        let size = Regex::new(r"File, size: (.+)b")
            .unwrap()
            .captures(&line)
            .and_then(|m| m[1].parse::<usize>().ok())
            .ok_or_else(|| io::Error::other("failed to parse size"))?;

        Ok(size)
    }

    /// Stat a file or directory.
    fn stat(&mut self, path: &FlipperPath) -> io::Result<String> {
        self.cli
            .send_and_wait_eol(&format!("storage stat {path}"))?;
        let line = self.cli.consume_response()?;

        Ok(line)
    }

    /// Make directory on the device.
    pub fn mkdir(&mut self, path: &FlipperPath) -> io::Result<()> {
        self.cli
            .send_and_wait_eol(&format!("storage mkdir {path}"))?;
        self.cli.consume_response()?;

        Ok(())
    }

    /// Format external storage.
    pub fn format_ext(&mut self) -> io::Result<()> {
        self.cli.send_and_wait_eol("storage format /ext")?;
        self.cli.send_and_wait_eol("y")?;
        self.cli.consume_response()?;

        Ok(())
    }

    /// Remove file or directory.
    pub fn remove(&mut self, path: &FlipperPath) -> io::Result<()> {
        self.cli
            .send_and_wait_eol(&format!("storage remove {path}"))?;
        self.cli.consume_response()?;

        Ok(())
    }

    /// Calculate MD5 hash of file.
    pub fn md5sum(&mut self, path: &FlipperPath) -> io::Result<String> {
        self.cli.send_and_wait_eol(&format!("storage md5 {path}"))?;
        let line = self.cli.consume_response()?;

        Ok(line)
    }
}

/// A path on the Flipper device.
///
/// [`FlipperPath`] maintains certain invariants:
/// - Paths are valid UTF-8
/// - Paths are always absolute (start with `/`)
/// - Paths do not end in a `/`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlipperPath(String);

impl FlipperPath {
    /// Create a new [`FlipperPath`].
    pub fn new() -> Self {
        Self(String::from("/"))
    }

    /// Push a path fragment to this path
    pub fn push(&mut self, path: &str) {
        let path = path.trim_end_matches('/');
        if path.starts_with('/') {
            // Absolute path
            self.0 = String::from(path);
        } else {
            // Relative path
            if !self.0.ends_with('/') {
                self.0 += "/";
            }
            self.0 += path;
        }
    }
}

impl Default for FlipperPath {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for FlipperPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for FlipperPath {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for FlipperPath {
    fn from(mut value: String) -> Self {
        if let Some(p) = value.rfind(|c| c != '/') {
            // Drop any trailing `/`
            value.truncate(p + 1);
        }

        if !value.starts_with('/') {
            // Make path absolute
            let mut path = Self::new();
            path.0.extend([value]);

            path
        } else {
            Self(value)
        }
    }
}

impl From<&str> for FlipperPath {
    fn from(value: &str) -> Self {
        FlipperPath::from(value.to_string())
    }
}

impl Add<&str> for FlipperPath {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.push(rhs);

        self
    }
}
