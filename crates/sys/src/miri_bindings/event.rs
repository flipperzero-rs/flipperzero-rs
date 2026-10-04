#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuriEventFlag {
    _unused: [u8; 0],
}
#[doc = "Allocate FuriEventFlag\n\n # Returns\n\npointer to FuriEventFlag"]
pub unsafe fn furi_event_flag_alloc() -> *mut FuriEventFlag {
    todo!()
}
#[doc = "Deallocate FuriEventFlag\n\n # Arguments\n\n* `instance` - pointer to FuriEventFlag"]
pub unsafe fn furi_event_flag_free(instance: *mut FuriEventFlag) {
    todo!()
}
#[doc = "Set flags\n\n result of this function can be flags that you've just asked to\n set or not if someone was waiting for them and asked to clear it.\n It is highly recommended to read this function and\n xEventGroupSetBits source code.\n\n # Arguments\n\n* `instance` - pointer to FuriEventFlag\n * `flags` (direction in) - The flags to set\n\n # Returns\n\nResulting flags(see warning) or error (FuriStatus)"]
pub unsafe fn furi_event_flag_set(instance: *mut FuriEventFlag, flags: u32) -> u32 {
    todo!()
}
#[doc = "Clear flags\n\n # Arguments\n\n* `instance` - pointer to FuriEventFlag\n * `flags` (direction in) - The flags\n\n # Returns\n\nResulting flags or error (FuriStatus)"]
pub unsafe fn furi_event_flag_clear(instance: *mut FuriEventFlag, flags: u32) -> u32 {
    todo!()
}
#[doc = "Get flags\n\n # Arguments\n\n* `instance` - pointer to FuriEventFlag\n\n # Returns\n\nResulting flags"]
pub unsafe fn furi_event_flag_get(instance: *mut FuriEventFlag) -> u32 {
    todo!()
}
#[doc = "Wait flags\n\n # Arguments\n\n* `instance` - pointer to FuriEventFlag\n * `flags` (direction in) - The flags\n * `options` (direction in) - The option flags\n * `timeout` (direction in) - The timeout\n\n # Returns\n\nResulting flags or error (FuriStatus)"]
pub unsafe fn furi_event_flag_wait(
    instance: *mut FuriEventFlag,
    flags: u32,
    options: u32,
    timeout: u32,
) -> u32 {
    todo!()
}
