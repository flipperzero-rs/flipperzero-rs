
#[doc = "Check if CPU is in IRQ or kernel running and IRQ is masked\n\n Originally this primitive was born as a workaround for FreeRTOS kernel primitives shenanigans with PRIMASK.\n\n Meaningful use cases are:\n\n - When kernel is started and you want to ensure that you are not in IRQ or IRQ is not masked(like in critical section)\n - When kernel is not started and you want to make sure that you are not in IRQ mode, ignoring PRIMASK.\n\n As you can see there will be edge case when kernel is not started and PRIMASK is not 0 that may cause some funky behavior.\n Most likely it will happen after kernel primitives being used, but control not yet passed to kernel.\n It's up to you to figure out if it is safe for your code or not.\n\n # Returns\n\ntrue if CPU is in IRQ or kernel running and IRQ is masked"]
pub unsafe fn furi_kernel_is_irq_or_masked() -> bool {
    false
}
#[doc = "Check if kernel is running\n\n # Returns\n\ntrue if running, false otherwise"]
pub unsafe fn furi_kernel_is_running() -> bool {
    true
}
#[doc = "Lock kernel, pause process scheduling\n\n This should never be called in interrupt request context.\n\n # Returns\n\nprevious lock state(0 - unlocked, 1 - locked)"]
pub unsafe fn furi_kernel_lock() -> i32 {
    todo!()
}
#[doc = "Unlock kernel, resume process scheduling\n\n This should never be called in interrupt request context.\n\n # Returns\n\nprevious lock state(0 - unlocked, 1 - locked)"]
pub unsafe fn furi_kernel_unlock() -> i32 {
    todo!()
}
#[doc = "Restore kernel lock state\n\n This should never be called in interrupt request context.\n\n # Arguments\n\n* `lock` (direction in) - The lock state\n\n # Returns\n\nnew lock state or error"]
pub unsafe fn furi_kernel_restore_lock(lock: i32) -> i32 {
    todo!()
}
#[doc = "Get kernel systick frequency\n\n # Returns\n\nsystick counts per second"]
pub unsafe fn furi_kernel_get_tick_frequency() -> u32 {
    todo!()
}
