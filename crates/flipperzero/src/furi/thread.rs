//! Furi Thread API.

use core::time;
#[cfg(feature = "alloc")]
use core::{
    ffi::{CStr, c_void},
    fmt,
    ptr::NonNull,
    str,
};

#[cfg(feature = "alloc")]
use alloc::{
    boxed::Box,
    ffi::{CString, NulError},
    string::String,
    sync::Arc,
};

use flipperzero_sys::{self as sys, FuriFlagNoClear, FuriFlagWaitAll, FuriFlagWaitAny, HasFlag};

use crate::furi::time::FuriDuration;

#[cfg(feature = "alloc")]
const MIN_STACK_SIZE: usize = 1024;

/// Thread factory, which can be used in order to configure the properties of a new thread.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub struct Builder {
    /// Guaranteed to be UTF-8.
    name: Option<CString>,
    stack_size: Option<usize>,
    heap_trace_enabled: Option<bool>,
}

#[cfg(feature = "alloc")]
impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl Builder {
    /// Generates the base configuration for spawning a thread, from which configuration
    /// methods can be chained.
    pub fn new() -> Self {
        Self {
            name: None,
            stack_size: None,
            heap_trace_enabled: None,
        }
    }

    /// Names the thread-to-be.
    ///
    /// Returns an error if the name contains null bytes (`\0`).
    pub fn name(mut self, name: String) -> Result<Self, NulError> {
        CString::new(name).map(|name| {
            self.name = Some(name);
            self
        })
    }

    /// Sets the size of the stack (in bytes) for the new thread.
    pub fn stack_size(mut self, size: usize) -> Self {
        self.stack_size = Some(size);
        self
    }

    /// Enables heap tracing.
    ///
    /// By default, heap tracing is enabled if the Flipper Zero's "heap track mode" is
    /// either set to "All", or set to "Tree" and the parent's heap tracing is enabled.
    pub fn enable_heap_trace(mut self) -> Self {
        self.heap_trace_enabled = Some(true);
        self
    }

    /// Spawns a new thread by taking ownership of the `Builder`, and returns its
    /// [`JoinHandle`].
    pub fn spawn<F>(self, f: F) -> JoinHandle
    where
        F: FnOnce() -> i32,
        F: Send + 'static,
    {
        let Builder {
            name,
            stack_size,
            heap_trace_enabled,
        } = self;
        #[allow(clippy::arc_with_non_send_sync)] // TODO: is using `Arc` neccessary/sound here?
        let thread = Arc::new(Thread::new(name, stack_size, heap_trace_enabled));

        // We need to box twice because trait objects are fat pointers, so we need the
        // second box to obtain a thin pointer to use as the context.
        type ThreadBody = Box<dyn FnOnce() -> i32>;
        let thread_body: Box<ThreadBody> = Box::new(Box::new(f));
        unsafe extern "C" fn run_thread_body(context: *mut c_void) -> i32 {
            let thread_body = unsafe { Box::from_raw(context as *mut ThreadBody) };
            thread_body()
        }
        let callback: sys::FuriThreadCallback = Some(run_thread_body);
        let context = Box::into_raw(thread_body);

        unsafe extern "C" fn run_state_callback(
            _thread: *mut sys::FuriThread,
            state: sys::FuriThreadState,
            context: *mut c_void,
        ) {
            if state == sys::FuriThreadStateStopped {
                // SAFETY: We can drop the `Arc<Thread>` at the end of this function call,
                // because:
                // - `FuriThreadStateStopped` only occurs once.
                // - `FuriThreadStateStopped` is always the final state.
                let context = unsafe { Arc::from_raw(context as *mut Thread) };

                if let Some(thread) = Arc::into_inner(context) {
                    // SAFETY: No `Thread` instances exist at this point:
                    // - `JoinHandle` isn't Clone, and the one inside `JoinHandle` has
                    //   been dropped (because we were able to successfully extract the
                    //   `Thread` from the `Arc`).
                    // - Any obtained via `thread::current()` were dropped before the
                    //   thread stopped, because `Thread` isn't Send.
                    //
                    // Only two other pointers to the thread remain, and neither are read
                    // after this callback exits:
                    // - The one inside `furi_thread_body`.
                    // - The one inside the thread's local storage.
                    unsafe { sys::furi_thread_free(thread.thread.as_ptr()) };
                }
            }
        }
        let state_callback: sys::FuriThreadStateCallback = Some(run_state_callback);
        let state_context = Arc::into_raw(thread.clone());

        unsafe {
            sys::furi_thread_set_callback(thread.thread.as_ptr(), callback);
            sys::furi_thread_set_context(thread.thread.as_ptr(), context as *mut c_void);
            sys::furi_thread_set_state_callback(thread.thread.as_ptr(), state_callback);
            sys::furi_thread_set_state_context(
                thread.thread.as_ptr(),
                state_context as *mut c_void,
            );
            sys::furi_thread_start(thread.thread.as_ptr());
        }

        JoinHandle {
            context: Some(thread),
        }
    }
}

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// This call will create a thread using default parameters of [`Builder`]. If you want to
/// specify the stack size or the name of the thread, use that API instead.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn spawn<F>(f: F) -> JoinHandle
where
    F: FnOnce() -> i32,
    F: Send + 'static,
{
    Builder::new().spawn(f)
}

/// Gets a handle to the thread that invokes it.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn current() -> Thread {
    use alloc::borrow::ToOwned;

    let thread = NonNull::new(unsafe { sys::furi_thread_get_current() })
        .expect("furi_thread_get_current shouldn't return null");

    let name = {
        let name = unsafe { sys::furi_thread_get_name(sys::furi_thread_get_current_id()) };
        (!name.is_null())
            .then(|| {
                // SAFETY: The Flipper Zero firmware ensures that all thread names have a
                // null terminator.
                unsafe { CStr::from_ptr(name) }.to_owned()
            })
            .and_then(|name| {
                // Ensure that the name is valid UTF-8. This will be true for threads
                // created via `Builder`, but may not be true for the current thread.
                name.to_str().is_ok().then_some(name)
            })
    };

    Thread { name, thread }
}

/// Cooperatively gives up a timeslice to the OS scheduler.
pub fn yield_now() {
    unsafe { sys::furi_thread_yield() };
}

/// Puts the current thread to sleep for at least `duration`.
///
/// Durations under 1 hour are accurate to microseconds, while durations of
/// 1 hour or more are only accurate to milliseconds.
///
/// Will panic if requested to sleep for durations more than `2^32` microseconds (~49 days).
///
/// See [`sleep_ticks`] to sleep based on system timer ticks.
pub fn sleep(duration: core::time::Duration) {
    if duration > time::Duration::from_millis(u32::MAX as u64) {
        panic!("sleep exceeds maximum supported duration")
    }

    unsafe {
        // For durations of 1h+, use delay_ms so uint32_t doesn't overflow
        if duration < time::Duration::from_secs(3600) {
            sys::furi_delay_us(duration.as_micros() as u32);
        } else {
            sys::furi_delay_ms(duration.as_millis() as u32);
        }
    }
}

/// Puts the current thread to sleep for at least `duration`.
///
/// The maximum supported duration is `2^32` ticks (system timer dependent).
///
/// See [`sleep`] to sleep based on arbitary duration.
pub fn sleep_ticks(duration: FuriDuration) {
    unsafe {
        sys::furi_delay_tick(duration.as_ticks());
    }
}

/// A unique identifier for a running thread.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ThreadId(sys::FuriThreadId);

impl ThreadId {
    /// Get the `ThreadId` for the current thread.
    pub fn current() -> Self {
        ThreadId(unsafe { sys::furi_thread_get_current_id() })
    }

    /// Get the `ThreadId` for a specific C `FuriThread`.
    ///
    /// # Safety
    ///
    /// The thread pointer must be non-null and point to a valid `FuriThread`.
    pub unsafe fn from_furi_thread(thread: *mut sys::FuriThread) -> ThreadId {
        ThreadId(unsafe { sys::furi_thread_get_id(thread) })
    }
}

/// Set one-or-more notification flags on a thread.
///
/// Returns the value of the thread's notification flags after the specified `flags` have been set.
pub fn set_flags(thread_id: ThreadId, flags: u32) -> Result<u32, sys::furi::Status> {
    let result = unsafe { sys::furi_thread_flags_set(thread_id.0, flags) };

    if sys::FuriFlag(result).has_flag(sys::FuriFlagError) {
        return Err((result as i32).into());
    }

    Ok(result)
}

/// Clear one-or-more of the current thread's notification flags.
///
/// Returns the value of the current thread's notification flags after the specified `flags` have been cleared.
pub fn clear_flags(flags: u32) -> Result<u32, sys::furi::Status> {
    let result = unsafe { sys::furi_thread_flags_clear(flags) };

    if sys::FuriFlag(result).has_flag(sys::FuriFlagError) {
        return Err((result as i32).into());
    }

    Ok(result)
}

/// Get the value of the current thread's notification flags.
pub fn get_flags() -> Result<u32, sys::furi::Status> {
    let result = unsafe { sys::furi_thread_flags_get() };

    if sys::FuriFlag(result).has_flag(sys::FuriFlagError) {
        return Err((result as i32).into());
    }

    Ok(result)
}

/// Wait for up-to `timeout` for a change to any of the specified notification `flags` for the current thread.
///
/// If `clear`, then the specified flags will be cleared after a notification is received.
pub fn wait_any_flags(
    flags: u32,
    clear: bool,
    timeout: FuriDuration,
) -> Result<u32, sys::furi::Status> {
    let options = FuriFlagWaitAny.0 | (if clear { 0 } else { FuriFlagNoClear.0 });
    let result = unsafe { sys::furi_thread_flags_wait(flags, options, timeout.0) };

    if sys::FuriFlag(result).has_flag(sys::FuriFlagError) {
        return Err((result as i32).into());
    }

    Ok(result)
}

/// Wait for up-to `timeout` for a change to all of the specified notification `flags` for the current thread.
///
/// If `clear`, then the specified flags will be cleared after a notification is received.
pub fn wait_all_flags(
    flags: u32,
    clear: bool,
    timeout: FuriDuration,
) -> Result<u32, sys::furi::Status> {
    let options = FuriFlagWaitAll.0 | (if clear { 0 } else { FuriFlagNoClear.0 });
    let result = unsafe { sys::furi_thread_flags_wait(flags, options, timeout.0) };

    if sys::FuriFlag(result).has_flag(sys::FuriFlagError) {
        return Err((result as i32).into());
    }

    Ok(result)
}

/// A handle to a thread.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub struct Thread {
    /// Guaranteed to be UTF-8.
    name: Option<CString>,
    thread: NonNull<sys::FuriThread>,
}

#[cfg(feature = "alloc")]
impl Thread {
    fn new(
        name: Option<CString>,
        stack_size: Option<usize>,
        heap_trace_enabled: Option<bool>,
    ) -> Self {
        let stack_size = stack_size.unwrap_or(MIN_STACK_SIZE);

        unsafe {
            let thread = sys::furi_thread_alloc();
            if let Some(name) = name.as_deref() {
                sys::furi_thread_set_name(thread, name.as_ptr());
            }
            sys::furi_thread_set_stack_size(thread, stack_size);
            if let Some(heap_trace_enabled) = heap_trace_enabled {
                if heap_trace_enabled {
                    sys::furi_thread_enable_heap_trace(thread);
                }
            }
            Thread {
                name,
                thread: NonNull::new_unchecked(thread),
            }
        }
    }

    /// Gets the thread's unique identifier.
    ///
    /// Returns `None` if the thread has terminated.
    pub fn id(&self) -> Option<ThreadId> {
        // TODO: The Rust stdlib generates its own unique IDs for threads that are valid
        // even after a thread terminates.
        let id = unsafe { sys::furi_thread_get_id(self.thread.as_ptr()) };
        if id.is_null() {
            None
        } else {
            Some(ThreadId(id))
        }
    }

    /// Gets the thread's name.
    ///
    /// Returns `None` if the thread has terminated, or is unnamed, or has a name that is
    /// not valid UTF-8.
    pub fn name(&self) -> Option<&str> {
        self.cname()
            .map(|s| unsafe { str::from_utf8_unchecked(s.to_bytes()) })
    }

    fn cname(&self) -> Option<&CStr> {
        self.name.as_deref()
    }
}

#[cfg(feature = "alloc")]
impl fmt::Debug for Thread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Thread")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "alloc")]
impl ufmt::uDebug for Thread {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        // TODO: ufmt doesn't provide an impl of uDebug for &str.
        f.debug_struct("Thread")?.finish()
    }
}

/// An owned permission to join on a thread (block on its termination).
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub struct JoinHandle {
    context: Option<Arc<Thread>>,
}

#[cfg(feature = "alloc")]
impl Drop for JoinHandle {
    fn drop(&mut self) {
        let context = self
            .context
            .take()
            .expect("Drop should only be called once");

        if let Some(thread) = Arc::into_inner(context) {
            // We were able to successfully extract the `Thread` from the `Arc`. This
            // means there are no other references, so the thread is stopped and we can
            // free its memory.
            unsafe { sys::furi_thread_free(thread.thread.as_ptr()) };
        }
    }
}

#[cfg(feature = "alloc")]
impl JoinHandle {
    /// Extracts a handle to the underlying thread.
    pub fn thread(&self) -> &Thread {
        self.context.as_ref().expect("Drop has not been called")
    }

    /// Waits for the associated thread to finish.
    ///
    /// This function will return immediately if the associated thread has already
    /// finished.
    pub fn join(self) -> i32 {
        let thread = self.thread();
        unsafe {
            sys::furi_thread_join(thread.thread.as_ptr());
            sys::furi_thread_get_return_code(thread.thread.as_ptr())
        }
    }

    /// Checks if the associated thread has finished running its main function.
    pub fn is_finished(&self) -> bool {
        self.thread().id().is_none()
    }
}

#[cfg(feature = "alloc")]
impl fmt::Debug for JoinHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinHandle").finish_non_exhaustive()
    }
}

#[cfg(feature = "alloc")]
impl ufmt::uDebug for JoinHandle {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.debug_struct("JoinHandle")?.finish()
    }
}
