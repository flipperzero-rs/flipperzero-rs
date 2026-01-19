//! Panic handler for Furi applications.
//! This will print the panic info to stdout and then trigger a crash.

use core::panic::PanicInfo;

#[cfg(not(miri))]
use flipperzero_sys as sys;

#[cfg(not(miri))]
#[panic_handler]
pub fn panic(panic_info: &PanicInfo<'_>) -> ! {
    // Format: "thread: 'App Name' panicked at 'panic!', panic.rs:5"
    // Note: Don't use `format!` as it pulls in 10 KiB+ of formatting code.
    unsafe {
        let thread_id = sys::furi_thread_get_current_id();
        let thread_name = if !thread_id.is_null() {
            sys::furi_thread_get_name(thread_id)
        } else {
            c"unknown".as_ptr()
        };

        sys::__wrap_printf(c"\x1b[0;31mthread: '%s' paniced".as_ptr(), thread_name);

        if let Some(s) = panic_info.message().as_str() {
            sys::__wrap_printf(c" at '%.*s'".as_ptr(), s.len(), s.as_ptr());
        }

        if let Some(location) = panic_info.location() {
            let file = location.file();
            let line = location.line();

            sys::__wrap_printf(c", %.*s:%u".as_ptr(), file.len(), file.as_ptr(), line);
        }

        sys::__wrap_printf(c"\x1b[0m\r\n".as_ptr());
        sys::furi_thread_stdout_flush();

        // Ensure there's plenty of time to fully flush the console
        sys::furi_delay_ms(500);

        sys::crash!("Rust panic")
    }
}

#[cfg(miri)]
unsafe extern "Rust" {
    safe fn miri_write_to_stderr(bytes: &[u8]);
}

#[cfg(miri)]
#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    use flipperzero_sys::alloc::string::ToString;

    miri_write_to_stderr(
        panic_info
            .message()
            .to_string()
            .as_bytes(),
    );
    miri_write_to_stderr(b"\n");
    core::intrinsics::abort(); //~ ERROR: the program aborted execution
}
