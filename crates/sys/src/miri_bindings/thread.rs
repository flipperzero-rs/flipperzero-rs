#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriWait(pub core::ffi::c_uint);
#[doc = "< Thread is stopped and is safe to release. Event delivered from system init thread(TCB cleanup routine). It is safe to release thread instance."]
pub const FuriThreadStateStopped: FuriThreadState = FuriThreadState(0);
#[repr(transparent)]
#[doc = "Enumeration of possible FuriThread states.\n\n Many of the FuriThread functions MUST ONLY be called when the thread is STOPPED."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriThreadState(pub core::ffi::c_uchar);
#[repr(transparent)]
#[doc = "Enumeration of possible FuriThread priorities."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriThreadPriority(pub core::ffi::c_uchar);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuriThread {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuriThreadList {
    _unused: [u8; 0],
}
#[doc = "Unique thread identifier type (used by the OS kernel)."]
pub type FuriThreadId = *mut core::ffi::c_void;
#[doc = "Thread callback function pointer type.\n\n The function to be used as a thread callback MUST follow this signature.\n\n # Arguments\n\n* `context` (direction in, out) - pointer to a user-specified object\n # Returns\n\nvalue to be used as the thread return code"]
pub type FuriThreadCallback =
    ::core::option::Option<unsafe extern "C" fn(context: *mut core::ffi::c_void) -> i32>;
#[doc = "Standard output callback function pointer type.\n\n The function to be used as a standard output callback MUST follow this signature.\n\n The handler MUST process ALL of the provided data before returning.\n\n # Arguments\n\n* `data` (direction in) - pointer to the data to be written to the standard out\n * `size` (direction in) - size of the data in bytes\n * `context` (direction in) - optional context"]
pub type FuriThreadStdoutWriteCallback = ::core::option::Option<
    unsafe extern "C" fn(
        data: *const core::ffi::c_char,
        size: usize,
        context: *mut core::ffi::c_void,
    ),
>;
#[doc = "Standard input callback function pointer type\n\n The function to be used as a standard input callback MUST follow this signature.\n\n # Arguments\n\n* `buffer` (direction out) - buffer to read data into\n * `size` (direction in) - maximum number of bytes to read into the buffer\n * `timeout` (direction in) - how long to wait for (in ticks) before giving up\n * `context` (direction in) - optional context\n # Returns\n\nnumber of bytes that was actually read into the buffer"]
pub type FuriThreadStdinReadCallback = ::core::option::Option<
    unsafe extern "C" fn(
        buffer: *mut core::ffi::c_char,
        size: usize,
        timeout: FuriWait,
        context: *mut core::ffi::c_void,
    ) -> usize,
>;
#[doc = "State change callback function pointer type.\n\n The function to be used as a state callback MUST follow this\n signature.\n\n # Arguments\n\n* `thread` (direction in) - to the FuriThread instance that changed the state\n * `state` (direction in) - identifier of the state the thread has transitioned\n to\n * `context` (direction in, out) - pointer to a user-specified object"]
pub type FuriThreadStateCallback = ::core::option::Option<
    unsafe extern "C" fn(
        thread: *mut FuriThread,
        state: FuriThreadState,
        context: *mut core::ffi::c_void,
    ),
>;
#[doc = "Signal handler callback function pointer type.\n\n The function to be used as a signal handler callback MUS follow this signature.\n\n # Arguments\n\n* `signal` (direction in) - value of the signal to be handled by the recipient\n * `arg` (direction in, out) - optional argument (can be of any value, including NULL)\n * `context` (direction in, out) - pointer to a user-specified object\n # Returns\n\ntrue if the signal was handled, false otherwise"]
pub type FuriThreadSignalCallback = ::core::option::Option<
    unsafe extern "C" fn(
        signal: u32,
        arg: *mut core::ffi::c_void,
        context: *mut core::ffi::c_void,
    ) -> bool,
>;
#[doc = "Create a FuriThread instance.\n\n # Returns\n\npointer to the created FuriThread instance"]
pub unsafe fn furi_thread_alloc() -> *mut FuriThread {
    todo!()
}
#[doc = "Create a FuriThread instance w/ extra parameters.\n\n # Arguments\n\n* `name` (direction in) - human-readable thread name (can be NULL)\n * `stack_size` (direction in) - stack size in bytes (can be changed later)\n * `callback` (direction in) - pointer to a function to be executed in this thread\n * `context` (direction in) - pointer to a user-specified object (will be passed to the callback)\n # Returns\n\npointer to the created FuriThread instance"]
pub unsafe fn furi_thread_alloc_ex(
    name: *const core::ffi::c_char,
    stack_size: u32,
    callback: FuriThreadCallback,
    context: *mut core::ffi::c_void,
) -> *mut FuriThread {
    todo!()
}
#[doc = "Delete a FuriThread instance.\n\n The thread MUST be stopped when calling this function.\n\n see furi_thread_join for caveats on stopping a thread.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be deleted"]
pub unsafe fn furi_thread_free(thread: *mut FuriThread) {
    todo!()
}
#[doc = "Set the name of a FuriThread instance.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `name` (direction in) - human-readable thread name (can be NULL)"]
pub unsafe fn furi_thread_set_name(thread: *mut FuriThread, name: *const core::ffi::c_char) {
    todo!()
}
#[doc = "Set the application ID of a FuriThread instance.\n\n The thread MUST be stopped when calling this function.\n\n Technically, it is like a \"process id\", but it is not a system-wide unique identifier.\n All threads spawned by the same app will have the same appid.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `appid` (direction in) - thread application ID (can be NULL)"]
pub unsafe fn furi_thread_set_appid(thread: *mut FuriThread, appid: *const core::ffi::c_char) {
    todo!()
}
#[doc = "Set the stack size of a FuriThread instance.\n\n The thread MUST be stopped when calling this function. Additionally, it is NOT possible\n to change the stack size of a service thread under any circumstances.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `stack_size` (direction in) - stack size in bytes"]
pub unsafe fn furi_thread_set_stack_size(thread: *mut FuriThread, stack_size: usize) {
    todo!()
}
#[doc = "Set the user callback function to be executed in a FuriThread.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `callback` (direction in) - pointer to a user-specified function to be executed in this thread"]
pub unsafe fn furi_thread_set_callback(thread: *mut FuriThread, callback: FuriThreadCallback) {
    todo!()
}
#[doc = "Set the callback function context.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `context` (direction in) - pointer to a user-specified object (will be passed to the callback, can be NULL)"]
pub unsafe fn furi_thread_set_context(thread: *mut FuriThread, context: *mut core::ffi::c_void) {
    todo!()
}
#[doc = "Set the priority of a FuriThread.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `priority` (direction in) - priority level value"]
pub unsafe fn furi_thread_set_priority(thread: *mut FuriThread, priority: FuriThreadPriority) {
    todo!()
}
#[doc = "Get the priority of a FuriThread.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\npriority level value"]
pub unsafe fn furi_thread_get_priority(thread: *mut FuriThread) -> FuriThreadPriority {
    todo!()
}
#[doc = "Set the priority of the current FuriThread.\n\n # Arguments\n\n* `priority` - priority level value"]
pub unsafe fn furi_thread_set_current_priority(priority: FuriThreadPriority) {
    todo!()
}
#[doc = "Get the priority of the current FuriThread.\n\n # Returns\n\npriority level value"]
pub unsafe fn furi_thread_get_current_priority() -> FuriThreadPriority {
    todo!()
}
#[doc = "Set the callback function to be executed upon a state thransition of a FuriThread.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `callback` (direction in) - pointer to a user-specified callback function"]
pub unsafe fn furi_thread_set_state_callback(
    thread: *mut FuriThread,
    callback: FuriThreadStateCallback,
) {
    todo!()
}
#[doc = "Set the state change callback context.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `context` (direction in) - pointer to a user-specified object (will be passed to the callback, can be NULL)"]
pub unsafe fn furi_thread_set_state_context(
    thread: *mut FuriThread,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
#[doc = "Get the state of a FuriThread isntance.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\nthread state value"]
pub unsafe fn furi_thread_get_state(thread: *mut FuriThread) -> FuriThreadState {
    todo!()
}
#[doc = "Set a signal handler callback for a FuriThread instance.\n\n The thread MUST be stopped when calling this function if calling it from another thread.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified\n * `callback` (direction in) - pointer to a user-specified callback function\n * `context` (direction in) - pointer to a user-specified object (will be passed to the callback, can be NULL)"]
pub unsafe fn furi_thread_set_signal_callback(
    thread: *mut FuriThread,
    callback: FuriThreadSignalCallback,
    context: *mut core::ffi::c_void,
) {
    todo!()
}
#[doc = "Get a signal callback for a FuriThread instance.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\npointer to the callback function or NULL if none has been set"]
pub unsafe fn furi_thread_get_signal_callback(
    thread: *const FuriThread,
) -> FuriThreadSignalCallback {
    todo!()
}
#[doc = "Send a signal to a FuriThread instance.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be signaled\n * `signal` (direction in) - signal value to be sent\n * `arg` (direction in, out) - optional argument (can be of any value, including NULL)"]
pub unsafe fn furi_thread_signal(
    thread: *const FuriThread,
    signal: u32,
    arg: *mut core::ffi::c_void,
) -> bool {
    todo!()
}
#[doc = "Start a FuriThread instance.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be started"]
pub unsafe fn furi_thread_start(thread: *mut FuriThread) {
    todo!()
}
#[doc = "Wait for a FuriThread to exit.\n\n The thread callback function must return in order for the FuriThread instance to become joinable.\n\n Use this method only when the CPU is not busy (i.e. when the\n Idle task receives control), otherwise it will wait forever.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be joined\n # Returns\n\nalways true"]
pub unsafe fn furi_thread_join(thread: *mut FuriThread) -> bool {
    todo!()
}
#[doc = "Get the unique identifier of a FuriThread instance.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\nunique identifier value or NULL if thread is not running"]
pub unsafe fn furi_thread_get_id(thread: *mut FuriThread) -> FuriThreadId {
    todo!()
}
#[doc = "Enable heap usage tracing for a FuriThread.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in, out) - pointer to the FuriThread instance to be modified"]
pub unsafe fn furi_thread_enable_heap_trace(thread: *mut FuriThread) {
    todo!()
}
#[doc = "Get heap usage by a FuriThread instance.\n\n The heap trace MUST be enabled before callgin this function.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\nheap usage in bytes"]
pub unsafe fn furi_thread_get_heap_size(thread: *mut FuriThread) -> usize {
    todo!()
}
#[doc = "Get the return code of a FuriThread instance.\n\n This value is equal to the return value of the thread callback function.\n\n The thread MUST be stopped when calling this function.\n\n # Arguments\n\n* `thread` (direction in) - pointer to the FuriThread instance to be queried\n # Returns\n\nreturn code value"]
pub unsafe fn furi_thread_get_return_code(thread: *mut FuriThread) -> i32 {
    todo!()
}
#[doc = "Get the unique identifier of the current FuriThread.\n\n # Returns\n\nunique identifier value"]
pub unsafe fn furi_thread_get_current_id() -> FuriThreadId {
    todo!()
}
#[doc = "Get the FuriThread instance associated with the current thread.\n\n # Returns\n\npointer to a FuriThread instance or NULL if this thread does not belong to Furi"]
pub unsafe fn furi_thread_get_current() -> *mut FuriThread {
    todo!()
}
#[doc = "Return control to the scheduler."]
pub unsafe fn furi_thread_yield() {
    todo!()
}
#[doc = "Set the thread flags of a FuriThread.\n\n Can be used as a simple inter-thread communication mechanism.\n\n # Arguments\n\n* `thread_id` (direction in) - unique identifier of the thread to be notified\n * `flags` (direction in) - bitmask of thread flags to set\n # Returns\n\nbitmask combination of previous and newly set flags"]
pub unsafe fn furi_thread_flags_set(thread_id: FuriThreadId, flags: u32) -> u32 {
    todo!()
}
#[doc = "Clear the thread flags of the current FuriThread.\n\n # Arguments\n\n* `flags` (direction in) - bitmask of thread flags to clear\n # Returns\n\nbitmask of thread flags before clearing"]
pub unsafe fn furi_thread_flags_clear(flags: u32) -> u32 {
    todo!()
}
#[doc = "Get the thread flags of the current FuriThread.\n # Returns\n\ncurrent bitmask of thread flags"]
pub unsafe fn furi_thread_flags_get() -> u32 {
    todo!()
}
#[doc = "Wait for some thread flags to be set.\n\n [`FuriFlag`] for option and error flags.\n\n # Arguments\n\n* `flags` (direction in) - bitmask of thread flags to wait for\n * `options` (direction in) - combination of option flags determining the behavior of the function\n * `timeout` (direction in) - maximum time to wait in milliseconds (use FuriWaitForever to wait forever)\n # Returns\n\nbitmask combination of received thread and error flags"]
pub unsafe fn furi_thread_flags_wait(flags: u32, options: u32, timeout: u32) -> u32 {
    todo!()
}
#[doc = "Enumerate all threads.\n\n # Arguments\n\n* `thread_list` (direction out) - pointer to the FuriThreadList container\n\n # Returns\n\ntrue on success, false otherwise"]
pub unsafe fn furi_thread_enumerate(thread_list: *mut FuriThreadList) -> bool {
    todo!()
}
#[doc = "Get the name of a thread based on its unique identifier.\n\n # Arguments\n\n* `thread_id` (direction in) - unique identifier of the thread to be queried\n # Returns\n\npointer to a zero-terminated string or NULL"]
pub unsafe fn furi_thread_get_name(thread_id: FuriThreadId) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Get the application id of a thread based on its unique identifier.\n\n # Arguments\n\n* `thread_id` (direction in) - unique identifier of the thread to be queried\n # Returns\n\npointer to a zero-terminated string"]
pub unsafe fn furi_thread_get_appid(thread_id: FuriThreadId) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Allocate FuriThreadList instance\n\n # Returns\n\nFuriThreadList instance"]
pub unsafe fn furi_thread_list_alloc() -> *mut FuriThreadList {
    todo!()
}
#[doc = "Free FuriThreadList instance\n\n # Arguments\n\n* `instance` - The FuriThreadList instance to free"]
pub unsafe fn furi_thread_list_free(instance: *mut FuriThreadList) {
    todo!()
}
#[doc = "Get FuriThreadList instance size\n\n # Arguments\n\n* `instance` - The instance\n\n # Returns\n\nItem count"]
pub unsafe fn furi_thread_list_size(instance: *mut FuriThreadList) -> usize {
    todo!()
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuriThreadListItem {
    #[doc = "< Pointer to FuriThread, valid while it is running"]
    pub thread: *mut FuriThread,
    #[doc = "< Thread application id, valid while it is running"]
    pub app_id: *const core::ffi::c_char,
    #[doc = "< Thread name, valid while it is running"]
    pub name: *const core::ffi::c_char,
    #[doc = "< Thread priority"]
    pub priority: FuriThreadPriority,
    #[doc = "< Thread stack address"]
    pub stack_address: u32,
    #[doc = "< Thread heap size if tracking enabled, 0 - otherwise"]
    pub heap: usize,
    #[doc = "< Thread stack size"]
    pub stack_size: u32,
    #[doc = "< Thread minimum of the stack size ever reached"]
    pub stack_min_free: u32,
    #[doc = "< Thread state, can be: \"Running\", \"Ready\", \"Blocked\", \"Suspended\", \"Deleted\", \"Invalid\""]
    pub state: *const core::ffi::c_char,
    #[doc = "< Thread CPU usage time in percents (including interrupts happened while running)"]
    pub cpu: f32,
    #[doc = "< Thread previous runtime counter"]
    pub counter_previous: u32,
    #[doc = "< Thread current runtime counter"]
    pub counter_current: u32,
    #[doc = "< Thread last seen tick"]
    pub tick: u32,
}
#[doc = "Get item at position\n\n # Arguments\n\n* `instance` - The FuriThreadList instance\n * `position` (direction in) - The position of the item\n\n # Returns\n\nThe FuriThreadListItem instance"]
pub unsafe fn furi_thread_list_get_at(
    instance: *mut FuriThreadList,
    position: usize,
) -> *mut FuriThreadListItem {
    todo!()
}

#[doc = "Write data to buffered standard output.\n\n > **Note:** You can also use the standard C `putc`, `puts`, `printf` and friends.\n\n # Arguments\n\n* `data` (direction in) - pointer to the data to be written\n * `size` (direction in) - data size in bytes\n # Returns\n\nnumber of bytes that was actually written"]
pub unsafe fn furi_thread_stdout_write(data: *const core::ffi::c_char, size: usize) -> usize {
    todo!()
}
#[doc = "Flush buffered data to standard output.\n\n # Returns\n\nerror code value"]
pub unsafe fn furi_thread_stdout_flush() -> i32 {
    todo!()
}
