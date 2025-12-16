//! Metadata describing log data.
//
// The structs and enums in this file are extracted from the `tracing-core` crate with
// adaptions to Furi. The original code is copyright (c) 2019 Tokio Contributors

use core::{cmp, fmt, str::FromStr};

use crate::internals;
use flipperzero_sys as sys;
use ufmt::derive::uDebug;

/// Describes the level of verbosity of a log message.
///
/// # Comparing Levels
///
/// `Level` implements the [`PartialOrd`] and [`Ord`] traits, allowing two
/// `Level`s to be compared to determine which is considered more or less
/// verbose. Levels which are more verbose are considered "greater than" levels
/// which are less verbose, with [`Level::ERROR`] considered the lowest, and
/// [`Level::TRACE`] considered the highest.
///
/// For example:
/// ```
/// use flipperzero::furi::log::Level;
///
/// assert!(Level::TRACE > Level::DEBUG);
/// assert!(Level::ERROR < Level::WARN);
/// assert!(Level::INFO <= Level::DEBUG);
/// assert_eq!(Level::TRACE, Level::TRACE);
/// ```
///
/// # Filtering
///
/// `Level`s are typically used to implement filtering that determines which
/// log messages are enabled. Depending on the use case, more or less
/// verbose diagnostics may be desired. For example, when running in
/// development, [`DEBUG`]-level logs may be enabled by default. When running in
/// production, only [`INFO`]-level and lower logs might be enabled. Libraries
/// may include very verbose diagnostics at the [`DEBUG`] and/or [`TRACE`] levels.
/// Applications using those libraries typically chose to ignore those logs. However, when
/// debugging an issue involving said libraries, it may be useful to temporarily
/// enable the more verbose logs.
///
/// The [`LevelFilter`] type is provided to enable filtering logs by
/// verbosity. `Level`s can be compared against [`LevelFilter`]s, and
/// [`LevelFilter`] has a variant for each `Level`, which compares analogously
/// to that level. In addition, [`LevelFilter`] adds a [`LevelFilter::OFF`]
/// variant, which is considered "less verbose" than every other `Level`. This is
/// intended to allow filters to completely disable logging in a particular context.
///
/// For example:
/// ```
/// use flipperzero::furi::log::{Level, LevelFilter};
///
/// assert!(LevelFilter::OFF < Level::TRACE);
/// assert!(LevelFilter::TRACE > Level::DEBUG);
/// assert!(LevelFilter::ERROR < Level::WARN);
/// assert!(LevelFilter::INFO <= Level::DEBUG);
/// assert!(LevelFilter::INFO >= Level::INFO);
/// ```
///
/// ## Examples
///
/// `Level` should generally be used with the [`log`] macro via its associated
/// constants. You can also use the helper macros like [`warn`] directly without
/// needing to specify a `Level`.
///
/// [`DEBUG`]: Level::DEBUG
/// [`INFO`]: Level::INFO
/// [`TRACE`]: Level::TRACE
/// [`log`]: crate::log
/// [`warn`]: crate::warn
#[derive(Copy, Clone, Debug, uDebug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Level(LevelInner);

/// A filter comparable to a verbosity [`Level`].
///
/// If a [`Level`] is considered less than a `LevelFilter`, it should be
/// considered enabled; if greater than or equal to the `LevelFilter`,
/// that level is disabled. See [`LevelFilter::current`] for more
/// details.
///
/// Note that this is essentially identical to the `Level` type, but with the
/// addition of an [`OFF`] level that completely disables all logging
/// instrumentation.
///
/// See the documentation for the [`Level`] type to see how `Level`s
/// and `LevelFilter`s interact.
///
/// [`OFF`]: LevelFilter::OFF
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct LevelFilter(LevelFilterInner);

/// Indicates that a string could not be parsed to a valid level.
#[derive(Clone, Debug, uDebug)]
pub struct ParseLevelFilterError(());

// ===== impl Level =====

impl Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: Level = Level(LevelInner::Error);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: Level = Level(LevelInner::Warn);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: Level = Level(LevelInner::Info);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: Level = Level(LevelInner::Debug);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: Level = Level(LevelInner::Trace);

    /// Returns the string representation of the `Level`.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        }
    }

    pub(crate) fn to_furi(self) -> sys::FuriLogLevel {
        match self {
            Level::TRACE => sys::FuriLogLevelTrace,
            Level::DEBUG => sys::FuriLogLevelDebug,
            Level::INFO => sys::FuriLogLevelInfo,
            Level::WARN => sys::FuriLogLevelWarn,
            Level::ERROR => sys::FuriLogLevelError,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Level::TRACE => f.pad("TRACE"),
            Level::DEBUG => f.pad("DEBUG"),
            Level::INFO => f.pad("INFO"),
            Level::WARN => f.pad("WARN"),
            Level::ERROR => f.pad("ERROR"),
        }
    }
}

impl ufmt::uDisplay for Level {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(self.as_str())
    }
}

internals::macros::impl_std_error!(ParseLevelError);

impl FromStr for Level {
    type Err = ParseLevelError;
    fn from_str(s: &str) -> Result<Self, ParseLevelError> {
        match s {
            s if s.eq_ignore_ascii_case("error") => Ok(Level::ERROR),
            s if s.eq_ignore_ascii_case("warn") => Ok(Level::WARN),
            s if s.eq_ignore_ascii_case("info") => Ok(Level::INFO),
            s if s.eq_ignore_ascii_case("debug") => Ok(Level::DEBUG),
            s if s.eq_ignore_ascii_case("trace") => Ok(Level::TRACE),
            _ => Err(ParseLevelError { _p: () }),
        }
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, uDebug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum LevelInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = sys::FuriLogLevelTrace.0 as usize,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = sys::FuriLogLevelDebug.0 as usize,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = sys::FuriLogLevelInfo.0 as usize,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = sys::FuriLogLevelWarn.0 as usize,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = sys::FuriLogLevelError.0 as usize,
}

// === impl LevelFilter ===

impl From<Level> for LevelFilter {
    #[inline]
    fn from(level: Level) -> Self {
        Self::from_level(level)
    }
}

impl From<Option<Level>> for LevelFilter {
    #[inline]
    fn from(level: Option<Level>) -> Self {
        level.map(Self::from_level).unwrap_or(Self::OFF)
    }
}

impl From<LevelFilter> for Option<Level> {
    #[inline]
    fn from(filter: LevelFilter) -> Self {
        filter.into_level()
    }
}

impl LevelFilter {
    /// The "off" level.
    ///
    /// Designates that trace instrumentation should be completely disabled.
    pub const OFF: LevelFilter = LevelFilter(LevelFilterInner::Off);
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: LevelFilter = LevelFilter::from_level(Level::ERROR);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: LevelFilter = LevelFilter::from_level(Level::WARN);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: LevelFilter = LevelFilter::from_level(Level::INFO);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: LevelFilter = LevelFilter::from_level(Level::DEBUG);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: LevelFilter = LevelFilter::from_level(Level::TRACE);

    /// Returns a `LevelFilter` that enables log messages with verbosity up
    /// to and including `level`.
    pub const fn from_level(level: Level) -> Self {
        Self(match level.0 {
            LevelInner::Trace => LevelFilterInner::Trace,
            LevelInner::Debug => LevelFilterInner::Debug,
            LevelInner::Info => LevelFilterInner::Info,
            LevelInner::Warn => LevelFilterInner::Warn,
            LevelInner::Error => LevelFilterInner::Error,
        })
    }

    /// Returns the most verbose [`Level`] that this filter accepts, or `None`
    /// if it is [`OFF`].
    ///
    /// [`Level`]: super::Level
    /// [`OFF`]: LevelFilter::OFF
    pub const fn into_level(self) -> Option<Level> {
        match self.0 {
            LevelFilterInner::Trace => Some(Level::TRACE),
            LevelFilterInner::Debug => Some(Level::DEBUG),
            LevelFilterInner::Info => Some(Level::INFO),
            LevelFilterInner::Warn => Some(Level::WARN),
            LevelFilterInner::Error => Some(Level::ERROR),
            LevelFilterInner::Off => None,
        }
    }

    /// Returns a `LevelFilter` that matches the most verbose [`Level`] that the
    /// Furi Logging system will enable.
    ///
    /// User code should treat this as a *hint*. If a given log message has a
    /// level *higher* than the returned `LevelFilter`, it will not be enabled.
    /// However, if the level is less than or equal to this value, the log
    /// message is *not* guaranteed to be enabled; the Furi Logging system may
    /// perform additional filtering.
    ///
    /// Therefore, comparing a given log message's level to the returned
    /// `LevelFilter` **can** be used for determining if something is
    /// *disabled*, but **should not** be used for determining if something is
    /// *enabled*.
    ///
    /// [`Level`]: super::Level
    #[inline(always)]
    pub fn current() -> Self {
        match unsafe { sys::furi_log_get_level() } {
            // Default log level is defined in `furi/core/log.c` in the FlipperZero firmware.
            sys::FuriLogLevelDefault => Self::INFO,
            sys::FuriLogLevelNone => Self::OFF,
            sys::FuriLogLevelError => Self::ERROR,
            sys::FuriLogLevelWarn => Self::WARN,
            sys::FuriLogLevelInfo => Self::INFO,
            sys::FuriLogLevelDebug => Self::DEBUG,
            sys::FuriLogLevelTrace => Self::TRACE,
            #[cfg(debug_assertions)]
            unknown => unreachable!(
                "/!\\ `LevelFilter` representation seems to have changed! /!\\ \n\
                This is a bug (and it's pretty bad). Please contact the `flipperzero` \
                maintainers. Thank you and I'm sorry.\n \
                The offending repr was: {:?}",
                unknown,
            ),
            #[cfg(not(debug_assertions))]
            _ => unsafe {
                // Using `unreachable_unchecked` here (rather than
                // `unreachable!()`) is necessary to ensure that rustc generates
                // an identity conversion from integer -> discriminant, rather
                // than generating a lookup table. We want to ensure this
                // function is a single `bl` instruction (sometimes followed by
                // a `subs` instruction to handle `FuriLogLevelDefault`) if at
                // all possible, because it is called *every* time a logging
                // callsite is hit; and it is (potentially) the only code in the
                // hottest path for skipping a majority of callsites when level
                // filtering is in use.
                //
                // safety: This branch is only truly unreachable if we guarantee
                // that no values other than the possible enum discriminants
                // will *ever* be present. The log filter is initialized by the
                // Flipper Zero SDK to `FuriLogLevelDefault`, which is not a
                // valid `LevelFilter` discriminant but is specifically handled
                // above. It is set either internally by the Flipper Zero, or
                // through the Flipper Zero SDK. The latter we expose via the
                // `set_max` function, which takes a `LevelFilter` parameter;
                // this restricts the inputs to `set_max` to the set of valid
                // discriminants. Therefore, **as long as `furi_log_set_level`
                // is only ever called by `set_max`**, this is safe.
                core::hint::unreachable_unchecked()
            },
        }
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("off"),
            LevelFilter::ERROR => f.pad("error"),
            LevelFilter::WARN => f.pad("warn"),
            LevelFilter::INFO => f.pad("info"),
            LevelFilter::DEBUG => f.pad("debug"),
            LevelFilter::TRACE => f.pad("trace"),
        }
    }
}

impl fmt::Debug for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("LevelFilter::OFF"),
            LevelFilter::ERROR => f.pad("LevelFilter::ERROR"),
            LevelFilter::WARN => f.pad("LevelFilter::WARN"),
            LevelFilter::INFO => f.pad("LevelFilter::INFO"),
            LevelFilter::DEBUG => f.pad("LevelFilter::DEBUG"),
            LevelFilter::TRACE => f.pad("LevelFilter::TRACE"),
        }
    }
}

impl ufmt::uDisplay for LevelFilter {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match *self {
            LevelFilter::OFF => f.write_str("off"),
            LevelFilter::ERROR => f.write_str("error"),
            LevelFilter::WARN => f.write_str("warn"),
            LevelFilter::INFO => f.write_str("info"),
            LevelFilter::DEBUG => f.write_str("debug"),
            LevelFilter::TRACE => f.write_str("trace"),
        }
    }
}

impl ufmt::uDebug for LevelFilter {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match *self {
            LevelFilter::OFF => f.write_str("LevelFilter::OFF"),
            LevelFilter::ERROR => f.write_str("LevelFilter::ERROR"),
            LevelFilter::WARN => f.write_str("LevelFilter::WARN"),
            LevelFilter::INFO => f.write_str("LevelFilter::INFO"),
            LevelFilter::DEBUG => f.write_str("LevelFilter::DEBUG"),
            LevelFilter::TRACE => f.write_str("LevelFilter::TRACE"),
        }
    }
}

impl FromStr for LevelFilter {
    type Err = ParseLevelFilterError;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "" => Some(LevelFilter::ERROR),
            s if s.eq_ignore_ascii_case("error") => Some(LevelFilter::ERROR),
            s if s.eq_ignore_ascii_case("warn") => Some(LevelFilter::WARN),
            s if s.eq_ignore_ascii_case("info") => Some(LevelFilter::INFO),
            s if s.eq_ignore_ascii_case("debug") => Some(LevelFilter::DEBUG),
            s if s.eq_ignore_ascii_case("trace") => Some(LevelFilter::TRACE),
            s if s.eq_ignore_ascii_case("off") => Some(LevelFilter::OFF),
            _ => None,
        }
        .ok_or(ParseLevelFilterError(()))
    }
}

/// Returned if parsing a `Level` fails.
#[derive(Debug)]
pub struct ParseLevelError {
    _p: (),
}

impl fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level: expected one of \"error\", \"warn\", \
             \"info\", \"debug\", \"trace\"",
        )
    }
}

impl ufmt::uDisplay for ParseLevelError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(
            "error parsing level: expected one of \"error\", \"warn\", \
             \"info\", \"debug\", \"trace\"",
        )
    }
}

impl fmt::Display for ParseLevelFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level filter: expected one of \"off\", \"error\", \
            \"warn\", \"info\", \"debug\", \"trace\"",
        )
    }
}

impl ufmt::uDisplay for ParseLevelFilterError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(
            "error parsing level filter: expected one of \"off\", \"error\", \
            \"warn\", \"info\", \"debug\", \"trace\"",
        )
    }
}

internals::macros::impl_std_error!(ParseLevelFilterError);

#[repr(usize)]
#[derive(Copy, Clone, Debug, uDebug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum LevelFilterInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = sys::FuriLogLevelTrace.0 as usize,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = sys::FuriLogLevelDebug.0 as usize,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = sys::FuriLogLevelInfo.0 as usize,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = sys::FuriLogLevelWarn.0 as usize,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = sys::FuriLogLevelError.0 as usize,
    /// The "off" level.
    ///
    /// Designates that trace instrumentation should be completely disabled.
    Off = sys::FuriLogLevelNone.0 as usize,
}

// ==== Level and LevelFilter comparisons ====

impl PartialEq<LevelFilter> for Level {
    #[inline(always)]
    fn eq(&self, other: &LevelFilter) -> bool {
        self.0 as usize == (other.0 as usize)
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline(always)]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((self.0 as usize).cmp(&(other.0 as usize)))
    }
}

impl PartialEq<Level> for LevelFilter {
    #[inline(always)]
    fn eq(&self, other: &Level) -> bool {
        (self.0 as usize) == other.0 as usize
    }
}

impl PartialOrd<Level> for LevelFilter {
    #[inline(always)]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((self.0 as usize).cmp(&(other.0 as usize)))
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn level_from_str() {
        assert_eq!("error".parse::<Level>().unwrap(), Level::ERROR);
    }

    #[test]
    fn filter_level_conversion() {
        let mapping = [
            (LevelFilter::OFF, None),
            (LevelFilter::ERROR, Some(Level::ERROR)),
            (LevelFilter::WARN, Some(Level::WARN)),
            (LevelFilter::INFO, Some(Level::INFO)),
            (LevelFilter::DEBUG, Some(Level::DEBUG)),
            (LevelFilter::TRACE, Some(Level::TRACE)),
        ];
        for (filter, level) in mapping.iter() {
            assert_eq!(filter.into_level(), *level);
            match level {
                Some(level) => {
                    let actual: LevelFilter = (*level).into();
                    assert_eq!(actual, *filter);
                }
                None => {
                    let actual: LevelFilter = None.into();
                    assert_eq!(actual, *filter);
                }
            }
        }
    }

    #[test]
    fn level_filter_is_usize_sized() {
        assert_eq!(
            mem::size_of::<LevelFilter>(),
            mem::size_of::<usize>(),
            "`LevelFilter` is no longer `usize`-sized! global MAX_LEVEL may now be invalid!"
        )
    }

    #[test]
    fn level_filter_reprs() {
        let mapping = [
            (LevelFilter::OFF, LevelFilterInner::Off as usize),
            (LevelFilter::ERROR, LevelFilterInner::Error as usize),
            (LevelFilter::WARN, LevelFilterInner::Warn as usize),
            (LevelFilter::INFO, LevelFilterInner::Info as usize),
            (LevelFilter::DEBUG, LevelFilterInner::Debug as usize),
            (LevelFilter::TRACE, LevelFilterInner::Trace as usize),
        ];
        for &(filter, expected) in &mapping {
            let repr = unsafe {
                // safety: The entire purpose of this test is to assert that the
                // actual repr matches what we expect it to be --- we're testing
                // that *other* unsafe code is sound using the transmuted value.
                // We're not going to do anything with it that might be unsound.
                mem::transmute::<LevelFilter, usize>(filter)
            };
            assert_eq!(expected, repr, "repr changed for {:?}", filter)
        }
    }
}
