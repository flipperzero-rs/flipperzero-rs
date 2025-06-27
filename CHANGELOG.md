# Changelog

All notable changes to the crates in this workspace will be documented in this
file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- `flipperzero::bluetooth`, providing APIs for interacting with Bluetooth:
  - `beacon` module for operating with the built-in Bluetooth LE beacon.
  - `test_patterns` module with pattern generators and listeners.

### Changed

### Removed

## [0.15.0]

### Added

- `flipperzero::furi::hal::power::Power` handle to Power service
- High-level serial API (#216)
- Add `gui::Gui` record and basic support for using `gui::Canvas` in monopoly
  mode
- Add [`embedded-graphics`](https://crates.io/crates/embedded-graphics) support
  allowing `gui::Canvas` to be used as a
  [`DrawTarget`](https://docs.rs/embedded-graphics/latest/embedded_graphics/draw_target/trait.DrawTarget.html)
  (#214)

### Changed

- Updated to SDK 86.0 (firmware 1.3.4)
- `flipperzero::furi::time::Duration` has been renamed to `FuriDuration`
- `flipperzero::furi::time::Instant` has been renamed to `FuriInstant`
- `flipperzero_sys::furi::UnsafeRecord` now implements `Clone` to support
  reference-count use
- `flipperzero::notification::NotificationService` has been renamed
  `NotificationApp` to match the underlying C API

### Removed

## [0.14.0]

### Added

- `flipperzero_sys::furi::FuriBox` for low-level heap allocations
- `flipperzero_sys::halt!` macro
- `flipperzero::storage::Storage` handle to Storage service

### Changed

- Updated to SDK 79.2 (firmware 1.2.0)
- Enums in sys bindings now are newtype-enums and no longer have a duplicate
  prefix
- Fixed passing of crash message `__furi_crash_implementation`
- Allow no argument calls to `flipperzero_sys::crash!` and
  `flipperzero_sys::halt!` macros
- `flipperzero_sys::furi::UnsafeRecord::open` now takes a `&'static CStr`
- Fixed overread when printing panic message to console
- Added short delay in panic handler so message can be seen on USB console
  before `furi_crash`

### Removed

- No features were removed

## [0.13.0]

### Added

- `flipperzero::dialogs::DialogFileBrowserOptions`
- `flipperzero::furi::kernel` module exposing most `furi_kernel_*` APIs
- `as_ticks()` method to `flipperzero::furi::time::Duration`
- `flipperzero::furi::thread::sleep_ticks` function to sleep for exact duration
- `TryFrom<core::time::Duration>` for `flipperzero::furi::time::Duration`
- `sys::furi::Error` as error type for Kernel operations.

### Changed

- Updated to SDK 78 (firmware 1.1.2)
- `flipperzero::dialogs::DialogFileBrowserOptions` now uses native
  initialization function.
- `flipperzero::time::Duration::MAX` is now the maximum duration representable.
- `sys::furi::Status::err_or_else` has been replaced by
  `sys::furi::Status::into_result`.

### Removed

- `flipperzero::furi::duration_to_ticks` in favour of `TryFrom` traits

## [0.12.0]

### Added

- `flipperzero::gpio::i2c`, providing a Rust interface to the external 3.3V I2C
  bus over GPIO pins C0 and C1, as well as the internal (power) I2C bus.
- `flipperzero::furi::string::FuriString::into_raw`, allowing ownership of the
  string to be able to be handed over to C code.

### Changed

- Updated to SDK 73 (firmware 1.0.1🥳).
- Switched to `nightly-2024-09-10` compiler
- `flipperzero_rt::entry` macro now requires a function with type signature
  `fn(Option<&CStr>) -> i32` instead of `fn(*mut u8) -> i32`.
- `flipperzero::furi::string::FuriString::as_mut_ptr` is now public to allow for
  it to be used with low-level C APIs (e.g. `furi_string_printf`).

### Removed

- `flipperzero::toolbox::{Md5, Sha256}` (due to their removal from the Flipper
  Zero SDK API).
- `flipperzero_sys::c_string!`, since `CStr` literals are stable now and the
  macro did not provide any validations.

## [0.11.0]

### Added

- `flipperzero::furi::time::{Duration, Instant}` implementations.
- `impl Default for flipperzero::dialogs::DialogMessage`
- `impl Default for flipperzero::toolbox::Crc32`

### Changed

- Migrated to SDK API 35.0 (firmware 0.89.0).
- `flipperzero_test::tests` now allows `#[cfg(..)]` attributes on test methods.

### Documentation

- Feature flags are now documented and the items guarded by them are now
  annotated.

## [0.10.0]

### Added

- `flipperzero::format` macro.
- `flipperzero::furi::sync::FuriMutex`
- `flipperzero::furi::time`, containing the currently unusable placeholder types
  `Duration` and `Instant`.

### Changed

- Migrated to SDK API 28.2 (firmware 0.84.1).
- `flipperzero::furi::sync` has been rewritten using the `lock_api` crate.
  - `Mutex<T>` is now a type alias for `lock_api::Mutex<FuriMutex, T>`.
  - `MutexGuard<'a, T>` is now a type alias for
    `lock_api::MutexGuard<'a, FuriMutex, T>`.
- `flipperzero::{print, println}` macros now panic if they cannot write to
  stdout.

## [0.9.0]

### Added

- `flipperzero::{log, error, warn, info, debug, trace}` logging macros.
- `flipperzero::furi::log`, providing the `Level` and `LevelFilter` types for
  use with the logging macros.
- `flipperzero::furi::rng`, providing the `HwRng` type compatible with the
  `rand` crate.
- `flipperzero::furi::string`, providing the `FuriString` type that implements a
  `CString`-like string with a `String`-like API.
- In `flipperzero::furi::thread`:
  - `Builder, Thread, ThreadId, JoinHandle`
  - `spawn, current, yield_now`
- In `flipperzero::io`:
  - `Error::description`
  - `impl ufmt::uDisplay for Error`
- `flipperzero::notification`, providing constants for standard notification
  sequences, and APIs for running them.
- `flipperzero::toolbox`, providing wrapper types around several tools from the
  Flipper Zero SDK.
- `flipperzero-test` crate, providing a test harness for running tests directly
  on a Flipper Zero.

### Changed

- Migrated to SDK API 23.0 (firmware 0.82.3).
- MSRV is now 1.70.
- Bumped pinned nightly Rust version to `nightly-2023-05-03`.
- `flipperzero_rt::entry!` now waits for threads with the same app ID to finish.
  This prevents crashes when a `JoinHandle` is dropped instead of joined, and
  would outlive the main function.

## [0.8.0]

### Added

- `flipperzero::io`, providing `Read, Write, Seek` traits similar to the ones in
  `std::io`.
- `flipperzero::storage`, providing `OpenOptions` and `File` structs similar to
  the ones in `std::fs`.
- `impl ufmt::{uDebug, uDisplay} for flipperzero_sys::furi::Status`

### Changed

- Migrated to SDK API 20.0 (firmware 0.80.1).
- MSRV is now 1.64.
- Bumped pinned nightly Rust version to `nightly-2023-03-10`.
- `flipperzero::{print, println}` macros now use the `ufmt` crate, and are
  restricted to its supported argument syntax.

## [0.7.2]

### Fixed

- Removed unintended prefix from all bindings documentation.

## [0.7.1]

### Added

- `flipperzero::dolphin`, providing types for interacting with the dolphin.

### Changed

- `flipperzero_sys` bindings documentation is now transformed from Doxygen.

## [0.7.0]

### Changed

- Migrated to SDK API 14.0 (firmware 0.77.1).

## [0.6.0]

### Added

- `flipperzero/alloc` feature flag.
- `flipperzero::dialogs`, providing the `DialogsApp` and `DialogMessage` types
  for creating simple dialogs.
- `flipperzero::gui::canvas:Align` enum, used by `DialogMessage`.
- `flipperzero_sys::furi::UnsafeRecord`
- Inline functions required to use the Furi HAL GPIO interface.

### Changed

- Migrated to SDK API 11.2 (firmware 0.74.2).
- Macros now allow trailing commas.

## [0.5.0]

### Changed

- Migrated to SDK API 10.1 (firmware 0.73.1).

## [0.4.1]

### Added

- Custom icon support to `flipperzero_rt::manifest!` macro.

## [0.4.0]

### Changed

- Migrated to SDK API 7.5 (firmware 0.71.1).
- `flipperzero_rt::manifest::ManifestBase` fields `api_version_major` and
  `api_version_minor` have been combined into an `api_version` field.

## [0.3.1]

### Fixed

- `flipperzero-sys` bindings now use short enums as required.

## [0.3.0]

### Added

- `flipperzero-rt` crate, enabling standalone app binaries to be built.

### Changed

- Hand-written bindings are replaced by generated bindings from SDK API 2.2
  (firmware 0.69).

### Removed

- `flipperzero::panic_handler` (moved to `flipperzero_rt::panic_handler`).

## [0.2.0]

### Added

- `flipperzero::{print, println}` macros.
- `flipperzero::furi::{message_queue, sync}` modules.
- `flipperzero::furi::{Result, Error}` type aliases.
- `flipperzero-alloc` crate, providing global allocator support.
- `flipperzero-sys` crate has more hand-written bindings.

### Changed

- MSRV is now 1.64.
- `flipperzero::furi::Stdout` moved into `flipperzero::furi::io` module.
- `flipperzero::furi::sleep` moved into `flipperzero::furi::thread` module.

## [0.1.0]

Initial release!

[Unreleased]: https://github.com/flipperzero-rs/flipperzero/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.15.0
[0.14.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.14.0
[0.13.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.13.0
[0.12.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.12.0
[0.11.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.11.0
[0.10.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.10.0
[0.9.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.9.0
[0.8.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.8.0
[0.7.2]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.7.2
[0.7.1]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.7.1
[0.7.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.7.0
[0.6.1]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.6.1
[0.6.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.6.0
[0.5.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.5.0
[0.4.1]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.4.1
[0.4.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.4.0
[0.3.1]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.3.1
[0.3.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.3.0
[0.2.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.2.0
[0.1.0]: https://github.com/flipperzero-rs/flipperzero/releases/tag/v0.1.0
