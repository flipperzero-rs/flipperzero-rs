//! Low-level wrappers around Furi API.

#[cfg(not(miri))]
mod alloc;
mod record;
mod status;

#[cfg(not(miri))]
pub use alloc::FuriBox;
pub use record::UnsafeRecord;
pub use status::{Error, Status};
