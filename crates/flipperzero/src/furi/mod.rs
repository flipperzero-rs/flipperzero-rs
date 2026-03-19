//! Furi API.

pub mod event_flag;
#[cfg(not(miri))]
pub mod hal;
pub mod io;
pub mod kernel;
pub mod log;
#[cfg(not(miri))]
pub mod message_queue;
#[cfg(not(miri))]
pub mod rng;
#[cfg(not(miri))]
pub mod stream_buffer;
pub mod string;
pub mod sync;
pub mod thread;
pub mod time;

use flipperzero_sys as sys;

/// Furi Result type.
pub type Result<T> = core::result::Result<T, Error>;
/// Furi Error type.
pub type Error = sys::furi::Error;
