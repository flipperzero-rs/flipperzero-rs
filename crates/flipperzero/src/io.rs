use core::ffi::CStr;
use core::fmt;

use flipperzero_sys as sys;

use crate::furi::string::FuriString;

/// How many bytes to read at a time.
/// This is kept small as the buffer is often stack allocated.
pub(crate) const DEFAULT_BUF_SIZE: usize = 64;

/// A specialized `Result` type for I/O operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Stream and file system related error kinds.
///
/// This list may grow over time, and it is not recommended to exhaustively
/// match against it.
///
/// # Handling errors and matching on `Error`
///
/// In application code, use `match` for the `Error` values you are expecting;
/// use `_` to match "all other errors".
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    NotReady,
    Exists,
    NotExists,
    InvalidParameter,
    Denied,
    InvalidName,
    Internal,
    NotImplemented,
    AlreadyOpen,

    /// I/O error specific to `flipperzero-rs` to represent the case a call to
    /// `write` returned `Ok(0)`, meaning that the operation could not be
    /// completed.
    WriteZero,

    /// Any I/O error from the Flipper Zero SDK that's not part of this list.
    ///
    /// Errors that are `Uncategorized` now may move to a different or a new [`Error`]
    /// variant in the future.
    #[non_exhaustive]
    #[doc(hidden)]
    Uncategorized(sys::FS_Error),
}

impl Error {
    pub fn to_sys(&self) -> Option<sys::FS_Error> {
        match self {
            Self::NotReady => Some(sys::FSE_NOT_READY),
            Self::Exists => Some(sys::FSE_EXIST),
            Self::NotExists => Some(sys::FSE_NOT_EXIST),
            Self::InvalidParameter => Some(sys::FSE_INVALID_PARAMETER),
            Self::Denied => Some(sys::FSE_DENIED),
            Self::InvalidName => Some(sys::FSE_INVALID_NAME),
            Self::Internal => Some(sys::FSE_INTERNAL),
            Self::NotImplemented => Some(sys::FSE_NOT_IMPLEMENTED),
            Self::AlreadyOpen => Some(sys::FSE_ALREADY_OPEN),
            Self::Uncategorized(error_code) => Some(*error_code),
            _ => None,
        }
    }

    pub fn from_sys(err: sys::FS_Error) -> Option<Self> {
        match err {
            sys::FSE_OK => None,
            sys::FSE_NOT_READY => Some(Self::NotReady),
            sys::FSE_EXIST => Some(Self::Exists),
            sys::FSE_NOT_EXIST => Some(Self::NotExists),
            sys::FSE_INVALID_PARAMETER => Some(Self::InvalidParameter),
            sys::FSE_DENIED => Some(Self::Denied),
            sys::FSE_INVALID_NAME => Some(Self::InvalidName),
            sys::FSE_INTERNAL => Some(Self::Internal),
            sys::FSE_NOT_IMPLEMENTED => Some(Self::NotImplemented),
            sys::FSE_ALREADY_OPEN => Some(Self::AlreadyOpen),
            error_code => Some(Self::Uncategorized(error_code)),
        }
    }

    /// Description associated with [`Error`].
    pub fn description(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::filesystem_api_error_get_desc(self.to_sys().unwrap())) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description().to_bytes().escape_ascii().fmt(f)
    }
}

impl ufmt::uDisplay for Error {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> core::result::Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        for c in self.description().to_bytes().escape_ascii() {
            f.write_char(c as char)?;
        }

        Ok(())
    }
}

/// Trait comparable to `std::Read` for the Flipper Zero API
pub trait Read {
    /// Reads some bytes from this source into the given buffer, returning how many bytes
    /// were read.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Reads all bytes until EOF in this source, appending them to `buf``.
    ///
    /// If successful, this function returns the number of bytes which were read and appended to `buf``.
    fn read_to_string(&mut self, string: &mut FuriString) -> Result<usize> {
        default_read_to_string(self, string)
    }
}

pub(crate) fn default_read_to_string<R: Read + ?Sized>(
    r: &mut R,
    string: &mut FuriString,
) -> Result<usize> {
    let mut total_bytes_read = 0;

    let mut buf = [0u8; DEFAULT_BUF_SIZE];
    loop {
        let bytes_read = r.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }

        total_bytes_read += bytes_read;

        for ch in buf[0..bytes_read].iter().copied() {
            string.push(ch as char);
        }
    }

    Ok(total_bytes_read)
}

/// Trait comparable to `std::Seek` for the Flipper Zero API
pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize>;

    fn rewind(&mut self) -> Result<()> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    fn stream_len(&mut self) -> Result<usize> {
        let old_pos = self.stream_position()?;
        let len = self.seek(SeekFrom::End(0))?;

        // Avoid seeking a third time when we were already at the end of the
        // stream. The branch is usually way cheaper than a seek operation.
        if old_pos != len {
            self.seek(SeekFrom::Start(
                old_pos.try_into().map_err(|_| Error::InvalidParameter)?,
            ))?;
        }

        Ok(len)
    }

    fn stream_position(&mut self) -> Result<usize> {
        self.seek(SeekFrom::Current(0))
    }
}

/// Trait comparable to `std::Write` for the Flipper Zero API
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(Error::WriteZero),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is used by the Seek trait.
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
