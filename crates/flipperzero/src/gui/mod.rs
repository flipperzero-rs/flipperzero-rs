//! GUI service.

pub mod canvas;

use core::ffi::CStr;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::ptr;

use canvas::Canvas;
use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;

/// GUI service record.
pub struct Gui {
    record: UnsafeRecord<sys::Gui>,
}

impl Gui {
    const NAME: &CStr = c"gui";

    /// Open record to GUI service.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Obtain raw pointer to GUI service.
    ///
    /// This pointer must not be free'd or used after the Gui object has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::Gui {
        self.record.as_ptr()
    }

    /// Get gui canvas frame buffer size in bytes.
    pub fn get_framebuffer_size(&self) -> usize {
        unsafe { sys::gui_get_framebuffer_size(self.as_ptr()) }
    }

    /// When lockdown mode is enabled, only GuiLayerDesktop is shown.
    /// This feature prevents services from showing sensitive information when flipper is locked.
    pub fn set_lockdown(&self, lockdown: bool) {
        unsafe { sys::gui_set_lockdown(self.as_ptr(), lockdown) }
    }

    /// Acquire Direct Draw lock to allow accessing the Canvas in monopoly mode.
    ///
    /// While holding the Direct Draw lock, all input and draw call dispatch
    /// functions in the GUI service are disabled. No other applications or
    /// services will be able to draw until the lock is released.
    pub fn direct_draw_acquire(&self) -> ExclusiveCanvas<'_> {
        ExclusiveCanvas::new(self)
    }
}

/// A RAII implementation of a "scope lock" for the GUI Direct Draw Lock. When this
/// structure is dropped, the Direct Draw Lock will be released.
///
/// This method return Canvas instance for use in monopoly mode. Direct draw lock
/// disables input and draw call dispatch functions in GUI service. No other
/// applications or services will be able to draw until `direct_draw_release`
/// call.
pub struct ExclusiveCanvas<'a> {
    gui: &'a Gui,
    canvas: ptr::NonNull<sys::Canvas>,
    _marker: PhantomData<&'a mut Canvas>,
}

impl<'a> ExclusiveCanvas<'a> {
    fn new(gui: &'a Gui) -> Self {
        ExclusiveCanvas {
            gui,
            // SAFETY: Returned pointer is always a valid non-null Canvas.
            canvas: unsafe {
                ptr::NonNull::new_unchecked(sys::gui_direct_draw_acquire(gui.as_ptr()))
            },
            _marker: PhantomData,
        }
    }

    /// Get Canvas.
    ///
    /// This can't be used with a temporary reference.
    ///
    /// ```compile_fail
    /// # let gui = Gui::open();
    /// let canvas = gui.direct_draw_acquire().canvas();
    /// ```
    pub fn canvas(&self) -> &Canvas {
        unsafe { Canvas::from_raw(self.canvas.as_ptr()) }
    }

    /// Get mutable Canvas.
    ///
    /// This can't be used with a temporary reference.
    ///
    /// ```compile_fail
    /// # let gui = Gui::open();
    /// let canvas = gui.direct_draw_acquire().canvas_mut();
    /// ```
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        unsafe { Canvas::from_raw_mut(self.canvas.as_ptr()) }
    }
}

impl Deref for ExclusiveCanvas<'_> {
    type Target = Canvas;

    fn deref(&self) -> &Self::Target {
        self.canvas()
    }
}

impl DerefMut for ExclusiveCanvas<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.canvas_mut()
    }
}

impl Drop for ExclusiveCanvas<'_> {
    fn drop(&mut self) {
        unsafe { sys::gui_direct_draw_release(self.gui.as_ptr()) }
    }
}
