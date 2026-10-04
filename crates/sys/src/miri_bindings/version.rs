#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Version {
    _unused: [u8; 0],
}
#[doc = "Get current running firmware version handle.\n\n You can store it somewhere. But if you want to retrieve data, you have to use\n 'version_*_get()' set of functions. Also, 'version_*_get()' imply to use this\n handle if no handle (NULL_PTR) provided.\n\n # Returns\n\npointer to Version data."]
pub unsafe fn version_get() -> *const Version {
    todo!()
}
#[doc = "Get git commit hash.\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\ngit hash"]
pub unsafe fn version_get_githash(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get git branch.\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\ngit branch"]
pub unsafe fn version_get_gitbranch(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get number of commit in git branch.\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\nnumber of commit"]
pub unsafe fn version_get_gitbranchnum(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get build date.\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\nbuild date"]
pub unsafe fn version_get_builddate(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get build version. Build version is last tag in git history.\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\nbuild date"]
pub unsafe fn version_get_version(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get hardware target this firmware was built for\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\nbuild date"]
pub unsafe fn version_get_target(v: *const Version) -> u8 {
    todo!()
}
#[doc = "Get flag indicating if this build is \"dirty\" (source code had uncommited changes)\n\n # Arguments\n\n* `v` - pointer to Version data. NULL for currently running\n software.\n\n # Returns\n\nbuild date"]
pub unsafe fn version_get_dirty_flag(v: *const Version) -> bool {
    todo!()
}
#[doc = "Get firmware origin. \"Official\" for mainline firmware, fork name for forks.\n Set by FIRMWARE_ORIGIN fbt argument."]
pub unsafe fn version_get_firmware_origin(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get git repo origin"]
pub unsafe fn version_get_git_origin(v: *const Version) -> *const core::ffi::c_char {
    todo!()
}
