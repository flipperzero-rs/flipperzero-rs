//! Furi Kernel primitives.

use flipperzero_sys as sys;
use ufmt::derive::uDebug;

use crate::furi;

/// Check if CPU is in IRQ; or kernel running and IRQ is masked.
pub fn is_irq_or_masked() -> bool {
    // SAFETY: this function has no invariants to uphold
    unsafe { sys::furi_kernel_is_irq_or_masked() }
}

/// Check if kernel is running.
pub fn is_running() -> bool {
    unsafe { sys::furi_kernel_is_running() }
}

/// Kernel lock state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, uDebug)]
pub enum LockState {
    /// normal scheduling
    Unlocked = 0,
    /// scheduling paused
    Locked = 1,
}

impl From<i32> for LockState {
    fn from(value: i32) -> Self {
        match value {
            0 => LockState::Unlocked,
            // Assume any non-zero value is `Locked``.
            // This can be modified if new lock states are added in the future.
            _ => LockState::Locked,
        }
    }
}

impl From<LockState> for i32 {
    fn from(value: LockState) -> Self {
        value as i32
    }
}

/// Lock kernel, pause process scheduling.
///
/// <div class="warning">This should never be called in an interrupt request context.</div>
///
/// Returns previous lock state.
pub fn lock() -> furi::Result<LockState> {
    let status = sys::furi::Status::from(unsafe { sys::furi_kernel_lock() });

    status.into_result().map(LockState::from)
}

/// Unlock kernel, resume process scheduling.
///
/// <div class="warning">This should never be called in an interrupt request context.</div>
///
/// Returns previous lock state.
pub fn unlock() -> furi::Result<LockState> {
    let status = sys::furi::Status::from(unsafe { sys::furi_kernel_unlock() });

    Ok(status.into_result()?.into())
}

/// Restore kernel lock state.
///
/// <div class="warning">This should never be called in an interrupt request context.</div>
///
/// Returns previous lock state.
pub fn restore_lock(state: LockState) -> furi::Result<LockState> {
    let status = sys::furi::Status::from(unsafe { sys::furi_kernel_restore_lock(state.into()) });

    Ok(status.into_result()?.into())
}

/// Return kernel tick frequency in hertz.
#[inline]
pub fn get_tick_frequency() -> u32 {
    unsafe { sys::furi_kernel_get_tick_frequency() }
}

/// Return current kernel tick value.
///
/// The duration of a tick depends on kernel configuration.
/// The value can be discovered with [`get_tick_frequency`].
#[inline]
pub fn get_tick() -> u32 {
    unsafe { sys::furi_get_tick() }
}
