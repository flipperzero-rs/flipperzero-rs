use core::ffi::{CStr, c_void};
use core::ptr::NonNull;

use flipperzero_sys::furi::UnsafeRecord;
use flipperzero_sys::{self as sys, HasFlag};

use crate::io::*;

/// Storage service handle.
#[derive(Clone)]
pub struct Storage {
    record: UnsafeRecord<sys::Storage>,
}

impl Storage {
    pub const NAME: &CStr = c"storage";

    /// Open handle to Storage service.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Access raw Furi Storage record.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::Storage {
        self.record.as_ptr()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OpenOptions {
    access_mode: sys::FS_AccessMode,
    open_mode: sys::FS_OpenMode,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self::default()
    }

    fn from_parts(access_mode: sys::FS_AccessMode, open_mode: sys::FS_OpenMode) -> Self {
        OpenOptions {
            access_mode,
            open_mode,
        }
    }

    /// Read access
    pub fn read(self, set: bool) -> Self {
        OpenOptions::from_parts(
            if set {
                self.access_mode | sys::FSAM_READ
            } else {
                self.access_mode & !sys::FSAM_READ
            },
            self.open_mode,
        )
    }

    /// Write access
    pub fn write(self, set: bool) -> Self {
        OpenOptions::from_parts(
            if set {
                self.access_mode | sys::FSAM_WRITE
            } else {
                self.access_mode & !sys::FSAM_WRITE
            },
            self.open_mode,
        )
    }

    /// Open file, fail if file doesn't exist
    pub fn open_existing(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FSOM_OPEN_EXISTING
            } else {
                self.open_mode & !sys::FSOM_OPEN_EXISTING
            },
        )
    }

    /// Open file. Create new file if not exist
    pub fn open_always(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FSOM_OPEN_ALWAYS
            } else {
                self.open_mode & !sys::FSOM_OPEN_ALWAYS
            },
        )
    }

    /// Open file. Create new file if not exist. Set R/W pointer to EOF
    pub fn open_append(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FSOM_OPEN_APPEND
            } else {
                self.open_mode & !sys::FSOM_OPEN_APPEND
            },
        )
    }

    /// Creates a new file. Fails if the file is exist
    pub fn create_new(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FSOM_CREATE_NEW
            } else {
                self.open_mode & !sys::FSOM_CREATE_NEW
            },
        )
    }

    /// Creates a new file. If file exist, truncate to zero size
    pub fn create_always(self, set: bool) -> Self {
        OpenOptions::from_parts(
            self.access_mode,
            if set {
                self.open_mode | sys::FSOM_CREATE_ALWAYS
            } else {
                self.open_mode & !sys::FSOM_CREATE_ALWAYS
            },
        )
    }

    pub fn open(self, path: &CStr) -> Result<File, Error> {
        // It's possible to produce a nonsensical `open_mode` using the above
        // operations, so we have some logic here to drop any extraneous
        // information. The possible open modes form a partial order (for
        // example, `create_new` is more specialized than `truncate`) so we
        // search for the first "on" bit in this sequence, and use that as the
        // open mode.
        let canonicalized_open_mode = if self.open_mode.has_flag(sys::FSOM_CREATE_NEW) {
            sys::FSOM_CREATE_NEW
        } else if self.open_mode.has_flag(sys::FSOM_CREATE_ALWAYS) {
            sys::FSOM_CREATE_ALWAYS
        } else if self.open_mode.has_flag(sys::FSOM_OPEN_APPEND) {
            sys::FSOM_OPEN_APPEND
        } else if self.open_mode.has_flag(sys::FSOM_OPEN_ALWAYS) {
            sys::FSOM_OPEN_ALWAYS
        } else {
            sys::FSOM_OPEN_EXISTING
        };

        let f = File::new();
        if unsafe {
            sys::storage_file_open(
                f.as_ptr(),
                path.as_ptr().cast(),
                self.access_mode,
                canonicalized_open_mode,
            )
        } {
            Ok(f)
        } else {
            // Per docs, "you need to close the file even if the open operation
            // failed," but this is handled by `Drop`.
            Err(Error::from_sys(f.get_raw_error()).unwrap())
        }
    }
}

/// Basic, unbuffered file handle
#[allow(dead_code)]
pub struct File {
    raw: NonNull<sys::File>,
    storage: Storage,
}

impl File {
    pub fn new() -> Self {
        let storage = Storage::open();
        Self {
            // SAFETY: Alloc always returns a valid non-null pointer or `furi_panic`s.
            raw: unsafe { NonNull::new_unchecked(sys::storage_file_alloc(storage.as_ptr())) },
            storage,
        }
    }

    /// Obtain raw Furi file handle.
    ///
    /// This pointer must not be `free`d or otherwise invalidated.
    /// It must not be referenced after `File` as been dropped.
    pub fn as_ptr(&self) -> *mut sys::File {
        self.raw.as_ptr()
    }

    /// Get last error.
    fn get_raw_error(&self) -> sys::FS_Error {
        // SAFETY: Pointer is always non-null and valid `sys::File`
        unsafe { sys::storage_file_get_error(self.as_ptr()) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            // `storage_file_close` calls `storage_file_sync`
            // internally, so it's not necesssary to call it here.
            sys::storage_file_close(self.as_ptr());
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let bytes_read = unsafe {
            sys::storage_file_read(self.as_ptr(), buf.as_mut_ptr() as *mut c_void, buf.len())
        };

        match Error::from_sys(self.get_raw_error()) {
            Some(err) => Err(err),
            None => Ok(bytes_read),
        }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize, Error> {
        let (from_start, offset) = match pos {
            SeekFrom::Start(n) => (true, n.try_into().map_err(|_| Error::InvalidParameter)?),
            SeekFrom::Current(n) => (false, n.try_into().map_err(|_| Error::InvalidParameter)?),
            SeekFrom::End(n) => {
                // TODO: Per str4d, "for SeekFrom::End we will need to measure
                // the length of the file, and then use from_start = true and
                // offset = file_length - n."
                //
                // How can we perform this subtraction safely?
                let file_length: i64 = self.stream_len()?.try_into().unwrap();
                (
                    true,
                    (file_length - n)
                        .try_into()
                        .map_err(|_| Error::InvalidParameter)?,
                )
            }
        };
        unsafe {
            if sys::storage_file_seek(self.as_ptr(), offset, from_start) {
                Ok(sys::storage_file_tell(self.as_ptr())
                    .try_into()
                    .map_err(|_| Error::InvalidParameter)?)
            } else {
                Err(Error::from_sys(self.get_raw_error()).unwrap())
            }
        }
    }

    fn rewind(&mut self) -> Result<(), Error> {
        self.seek(SeekFrom::Start(0)).map(|_| {})
    }

    fn stream_len(&mut self) -> Result<usize, Error> {
        Ok(unsafe {
            sys::storage_file_size(self.as_ptr())
                .try_into()
                .map_err(|_| Error::InvalidParameter)?
        })
    }

    fn stream_position(&mut self) -> Result<usize, Error> {
        Ok(unsafe {
            sys::storage_file_tell(self.as_ptr())
                .try_into()
                .map_err(|_| Error::InvalidParameter)?
        })
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let bytes_written = unsafe {
            sys::storage_file_write(self.as_ptr(), buf.as_ptr() as *mut c_void, buf.len())
        };

        match Error::from_sys(self.get_raw_error()) {
            Some(err) => Err(err),
            None => Ok(bytes_written),
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
