//! Version information.

use core::ffi::CStr;

use flipperzero_sys as sys;

#[repr(transparent)]
pub struct Version(sys::Version);

impl Version {
    /// Get current running firmware version.
    pub fn get() -> &'static Self {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { Self::from_ptr(sys::version_get()) }
    }

    /// Get version from pointer.
    ///
    /// # Safety
    /// Pointer must be non-NULL and valid for `'a`.
    pub unsafe fn from_ptr<'a>(ptr: *const sys::Version) -> &'a Version {
        // SAFETY: Version has same layout as C Version struct.
        unsafe { &*ptr.cast() }
    }

    /// Pointer to version data.
    pub fn as_ptr(&self) -> *const sys::Version {
        &raw const self.0
    }

    /// git commit hash.
    pub fn git_hash(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_githash(self.as_ptr())) }
    }

    /// git branch.
    pub fn git_branch(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_gitbranch(self.as_ptr())) }
    }

    /// Number of commits in git branch.
    pub fn git_branchnum(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_gitbranchnum(self.as_ptr())) }
    }

    /// Build version.
    pub fn version(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_version(self.as_ptr())) }
    }

    /// Hardware target this firmware was built for.
    pub fn target(&self) -> u8 {
        unsafe { sys::version_get_target(self.as_ptr()) }
    }

    /// Check if this build is "dirty" (source code had uncommited changes).
    pub fn is_dirty(&self) -> bool {
        unsafe { sys::version_get_dirty_flag(self.as_ptr()) }
    }

    /// Get firmware origin. "Official" for mainline firmware, fork name for forks.
    pub fn firmware_origin(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_firmware_origin(self.as_ptr())) }
    }

    /// Get git repo origin.
    pub fn git_origin(&self) -> &'static CStr {
        // SAFETY: Returned pointer is a pointer to static, so never NULL.
        unsafe { CStr::from_ptr(sys::version_get_git_origin(self.as_ptr())) }
    }
}
