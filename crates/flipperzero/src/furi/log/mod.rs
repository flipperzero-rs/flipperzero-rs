//! Furi Logging system.

pub(crate) mod metadata;

pub use metadata::{Level, LevelFilter};

/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!` based
/// argument list.
///
/// # Examples
///
/// ```
/// use flipperzero::{log, furi::log::Level};
///
/// # fn main() {
/// let error_code = 42;
///
/// log!(Level::ERROR, "Failed to handle the florp: {}", error_code);
/// log!(target: "events", Level::INFO, "Finished the documentation!");
/// # }
/// ```
#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $msg:expr $(, $arg:expr)*) => ({
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;

        if $lvl <= $crate::furi::log::LevelFilter::current() {
            const TARGET: *const ::core::ffi::c_char =
                match ::core::ffi::CStr::from_bytes_with_nul(
                    ::core::concat!($target, "\0").as_bytes(),
                ) {
                    Ok(cstr) => cstr.as_ptr(),
                    Err(_error) => panic!("target contains NULs"),
                };

            let mut buf = $crate::__macro_support::FuriString::new();
            $crate::__macro_support::ufmt::uwrite!(&mut buf, $msg $(, $arg)*)
                .expect("can append to FuriString");
            // don't pass raw expression to the internal `unsafe` block
            let lvl = $crate::__macro_support::__level_to_furi($lvl);
            let buf = buf.as_c_ptr();
            unsafe {
                $crate::__macro_support::__sys::furi_log_print_format(lvl, TARGET, buf);
            };
        }
    });

    ($lvl:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: module_path!(), $lvl, $msg $(, $arg)*)
    );
}

#[macro_export]
macro_rules! log_str {
    (target: $target:expr, $lvl:expr, $str:expr) => ({
        // The `uwrite!` macro expects `ufmt` in scope
        use $crate::__macro_support::ufmt;

        if $lvl <= $crate::furi::log::LevelFilter::current() {
            const TARGET: *const ::core::ffi::c_char =
                match ::core::ffi::CStr::from_bytes_with_nul(
                    ::core::concat!($target, "\0").as_bytes(),
                ) {
                    Ok(cstr) => cstr.as_ptr(),
                    Err(_error) => panic!("target contains NULs"),
                };

            // don't pass raw expression to the internal `unsafe` block
            let lvl = $crate::__macro_support::__level_to_furi($lvl);
            let buf = $str.as_c_ptr();
            unsafe {
                $crate::__macro_support::__sys::furi_log_print_format(lvl, TARGET, buf);
            };
        }
    });

    ($lvl:expr, $msg:expr) => (
        $crate::log_str!(target: module_path!(), $lvl, $msg)
    );
}


/// Logs a message at the error level.
///
/// # Examples
///
/// ```
/// use flipperzero::error;
///
/// # fn main() {
/// let error_code = 42;
/// let name = "Flipper";
///
/// error!("Failed to handle the florp: {}", error_code);
/// error!(target: "events", "Missed birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::ERROR, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::ERROR, $msg $(, $arg)*)
    );
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```
/// use flipperzero::warn;
///
/// # fn main() {
/// let name = "Flipper";
///
/// warn!("Event almost started!");
/// warn!(target: "events", "About to miss the birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::WARN, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::WARN, $msg $(, $arg)*)
    );
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```
/// use flipperzero::info;
///
/// # fn main() {
/// let name = "Flipper";
///
/// info!("It's {}'s birthday today!", name);
/// info!(target: "events", "Birthday party today: {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::INFO, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::INFO, $msg $(, $arg)*)
    );
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```
/// use flipperzero::debug;
///
/// # fn main() {
/// let name = "Flipper";
///
/// debug!("Creating {} event", 1);
/// debug!(target: "events", "New event created: birthday party for {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::DEBUG, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::DEBUG, $msg $(, $arg)*)
    );
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```
/// use flipperzero::trace;
///
/// # fn main() {
/// let name = "Flipper";
///
/// trace!("About to show how the target field works");
/// trace!(target: "events", "Scanning for events involving {}", name);
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $msg:expr $(, $arg:expr)*) => (
        $crate::log!(target: $target, $crate::furi::log::Level::TRACE, $msg $(, $arg)*)
    );

    ($msg:expr $(, $arg:expr)*) => (
        $crate::log!($crate::furi::log::Level::TRACE, $msg $(, $arg)*)
    );
}
