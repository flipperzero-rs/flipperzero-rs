use core::ptr::NonNull;
use flipperzero_sys::{self as sys, Icon as SysIcon};

pub struct Icon {
    raw: NonNull<SysIcon>,
}

impl Icon {
    /// Construct an `Icon` from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// `raw` should be a valid non-null pointer to [`sys::Canvas`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flipperzero::gui::icon::Icon;
    ///
    /// let ptr = todo!();
    /// let canvas = unsafe { Icon::from_raw(ptr) };
    /// ```
    pub unsafe fn from_raw(raw: *mut SysIcon) -> Self {
        // SAFETY: the caller is required to provide the valid pointer
        let raw = unsafe { NonNull::new_unchecked(raw) };
        Self { raw }
    }

    #[inline]
    #[must_use]
    pub fn as_raw(&self) -> *mut SysIcon {
        self.raw.as_ptr()
    }

    pub fn get_width(&self) -> u16 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_get_width(raw) }
    }

    pub fn get_height(&self) -> u16 {
        let raw = self.raw.as_ptr();
        // SAFETY: `raw` is always valid
        unsafe { sys::icon_get_height(raw) }
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }
}
