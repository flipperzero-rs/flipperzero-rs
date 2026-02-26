//! Path manipulation.

use core::ffi::CStr;
use core::ptr;

use crate::furi::string::FuriString;

/// A slice of a path (akin to [`str`]).
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Path(CStr);

impl Path {
    pub fn new<S: AsRef<CStr> + ?Sized>(s: &S) -> &Self {
        let s = s.as_ref();

        // SAFETY: Path is repr(transparent) to CStr.
        unsafe { &*(ptr::from_ref(s) as *const Path) }
    }

    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }
}

impl Default for &Path {
    fn default() -> Self {
        Path::new(c"")
    }
}

impl AsRef<Path> for &Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl AsRef<CStr> for Path {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl AsRef<Path> for CStr {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for FuriString {
    fn as_ref(&self) -> &Path {
        Path::new(self.as_c_str())
    }
}
