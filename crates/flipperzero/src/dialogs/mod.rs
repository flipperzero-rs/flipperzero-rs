//! Flipper Zero dialogs.

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

use core::ffi::{CStr, c_void};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

use flipperzero_sys as sys;
use sys::furi::UnsafeRecord;

use crate::furi::string::FuriString;
use crate::gui::canvas::Align;

/// A handle to the Dialogs app.
#[derive(Clone)]
pub struct DialogsApp {
    record: UnsafeRecord<sys::DialogsApp>,
}

/// A dialog message.
pub struct DialogMessage<'a> {
    data: NonNull<sys::DialogMessage>,
    _phantom: PhantomData<&'a CStr>,
}

/// A dialog file browser options.
#[repr(transparent)]
pub struct DialogFileBrowserOptions<'a> {
    data: sys::DialogsFileBrowserOptions,
    _phantom: PhantomData<&'a ()>,
}

/// Button pressed on a dialog.
pub enum DialogMessageButton {
    Back,
    Left,
    Right,
    Center,
}

impl DialogsApp {
    pub const NAME: &CStr = c"dialogs";

    /// Obtains a handle to the Dialogs app.
    pub fn open() -> Self {
        Self {
            record: unsafe { UnsafeRecord::open(Self::NAME) },
        }
    }

    /// Get pointer to raw [`sys::DialogsApp`] record.
    ///
    /// This pointer must not be `free`d or otherwise invalidated.
    /// It should not be referenced after [`DialogsApp`] has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::DialogsApp {
        self.record.as_ptr()
    }

    /// Displays a message.
    pub fn show_message(&mut self, message: &DialogMessage) -> DialogMessageButton {
        let button_sys = unsafe { sys::dialog_message_show(self.as_ptr(), message.data.as_ptr()) };

        DialogMessageButton::from_sys(button_sys).expect("Invalid button")
    }

    /// Displays a file browser.
    ///  - path is a optional preselected file path
    ///  - options are optional file browser options
    pub fn show_file_browser(
        &mut self,
        path: Option<&mut FuriString>,
        options: Option<&DialogFileBrowserOptions>,
    ) -> Option<FuriString> {
        let mut result_path = FuriString::new();
        // path will be unmodified but needs to be a valid FuriString.
        // We can reuse the empty result_path if path is not provided.
        let path = path.unwrap_or(&mut result_path).as_mut_ptr();
        let options = options
            .map(|opts| &opts.data as *const sys::DialogsFileBrowserOptions)
            .unwrap_or(ptr::null());
        unsafe {
            sys::dialog_file_browser_show(self.as_ptr(), result_path.as_mut_ptr(), path, options)
        }
        .then_some(result_path)
    }
}

impl<'a> DialogMessage<'a> {
    /// Allocates a new dialog message.
    pub fn new() -> Self {
        let data = unsafe { NonNull::new_unchecked(sys::dialog_message_alloc()) };

        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Sets the labels of the buttons.
    pub fn set_buttons(
        &mut self,
        // FIXME: these are unsound for non-UTF8 string
        left: Option<&'a CStr>,
        center: Option<&'a CStr>,
        right: Option<&'a CStr>,
    ) {
        let left = left.map_or(ptr::null(), |l| l.as_ptr());
        let center = center.map_or(ptr::null(), |l| l.as_ptr());
        let right = right.map_or(ptr::null(), |l| l.as_ptr());

        unsafe {
            sys::dialog_message_set_buttons(self.data.as_ptr(), left, center, right);
        }
    }

    /// Sets the header text.
    pub fn set_header(
        &mut self,
        // FIXME: this is unsound for non-UTF8 string
        header: &'a CStr,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
    ) {
        unsafe {
            sys::dialog_message_set_header(
                self.data.as_ptr(),
                header.as_ptr(),
                x,
                y,
                horizontal.to_sys(),
                vertical.to_sys(),
            );
        }
    }

    /// Sets the body text.
    pub fn set_text(
        &mut self,
        // FIXME: this is unsound for non-UTF8 string
        text: &'a CStr,
        x: u8,
        y: u8,
        horizontal: Align,
        vertical: Align,
    ) {
        unsafe {
            sys::dialog_message_set_text(
                self.data.as_ptr(),
                text.as_ptr(),
                x,
                y,
                horizontal.to_sys(),
                vertical.to_sys(),
            );
        }
    }

    /// Clears the header text.
    pub fn clear_header(&mut self) {
        unsafe {
            sys::dialog_message_set_header(
                self.data.as_ptr(),
                ptr::null(),
                0,
                0,
                sys::AlignLeft,
                sys::AlignTop,
            );
        }
    }

    /// Clears the body text.
    pub fn clear_text(&mut self) {
        unsafe {
            sys::dialog_message_set_text(
                self.data.as_ptr(),
                ptr::null(),
                0,
                0,
                sys::AlignLeft,
                sys::AlignTop,
            );
        }
    }
}

impl Drop for DialogMessage<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::dialog_message_free(self.data.as_ptr());
        }
    }
}

impl Default for DialogMessage<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogMessageButton {
    fn from_sys(sys: sys::DialogMessageButton) -> Option<Self> {
        match sys {
            sys::DialogMessageButtonBack => Some(Self::Back),
            sys::DialogMessageButtonLeft => Some(Self::Left),
            sys::DialogMessageButtonCenter => Some(Self::Center),
            sys::DialogMessageButtonRight => Some(Self::Right),
            _ => None,
        }
    }
}

impl Default for DialogFileBrowserOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> DialogFileBrowserOptions<'a> {
    pub fn new() -> Self {
        // SAFETY: the string is a valid UTF-8
        unsafe { Self::with_extension(c"*") }
    }

    /// Creates a new dialog file browser options and initializes to default values.
    ///
    /// # Safety
    ///
    /// `extension` should be a valid UTF-8 string
    ///
    /// # Compatibility
    ///
    /// This function's signature may change in the future to make it safe.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::dialogs::DialogFileBrowserOptions;
    /// let options = DialogFileBrowserOptions::new(c"*");
    /// ```
    ///
    /// ## Lifetime covariance:
    ///
    /// Even if `'static` lifetime is involved in the creation of options,
    /// the resulting lifetime will be the most applicable one:
    ///
    /// ```
    /// # use core::ffi::CStr;
    /// # use flipperzero::dialogs::DialogFileBrowserOptions;
    /// // has `'static` lifetime
    /// const EXTENSION: &CStr = c"txt";
    /// // has "local" lifetime, aka `'a`
    /// let base_path_bytes = [b'/', b'r', b'o', b'o', b't'];
    /// let base_path = CStr::from_bytes_with_nul(&base_path_bytes).unwrap();
    /// // the most appropriate lifetime `'a` is used
    /// // SAFETY: `EXTENSION` is a valid UTF-8 string
    /// let mut options = unsafe { DialogFileBrowserOptions::new(EXTENSION) }
    ///     .set_base_path(base_path);
    /// ```
    ///
    /// Still this should not allow the options to outlive its components:
    ///
    /// ```compile_fail
    /// # use core::ffi::CStr;
    /// # use flipperzero::dialogs::DialogFileBrowserOptions;
    /// # use flipperzero_sys::{cstr, DialogsFileBrowserOptions};
    /// const EXTENSION: &CStr = cstr!("*");
    /// // SAFETY: `EXTENSION` is a valid UTF-8 string
    /// let mut options = unsafe { DialogFileBrowserOptions::new(EXTENSION) };
    /// {
    ///     let base_path_bytes = [b'/', b'r', b'o', b'o', b't'];
    ///     let base_path = CStr::from_bytes_with_nul(&base_path_bytes).unwrap();
    ///     options = options.set_base_path(base_path);
    /// }
    /// ```
    pub unsafe fn with_extension(extension: &'a CStr) -> Self {
        let mut options = MaybeUninit::<sys::DialogsFileBrowserOptions>::uninit();
        let uninit_options = options.as_mut_ptr();
        let extension = extension.as_ptr();
        // TODO: as for now, we stick to default (NULL) icon,
        //  although we may want to make it customizable via this function's parameter
        //  once there are safe Icon-related APIs
        let icon = ptr::null();
        // SAFETY: all pointers are valid (`icon` is allowed to be NULL)
        // and options is intentionally uninitialized
        // since it is the called function's job to do it
        unsafe { sys::dialog_file_browser_set_basic_options(uninit_options, extension, icon) };
        Self {
            // SAFETY: data has just been initialized fully
            // as guaranteed by the previously called function's contract
            data: unsafe { options.assume_init() },
            _phantom: PhantomData,
        }
    }

    /// Set file extension to be offered for selection.
    ///
    /// # Safety
    ///
    /// `extension` should be a valid UTF-8 string
    ///
    /// # Compatibility
    ///
    /// This function's signature may change in the future to make it safe.
    pub unsafe fn set_extension(mut self, extension: &'a CStr) -> Self {
        self.data.extension = extension.as_ptr();
        self
    }

    /// Set root folder path for navigation with back key.
    ///
    /// # Safety
    ///
    /// `base_path` should be a valid UTF-8 string
    ///
    /// # Compatibility
    ///
    /// This function's signature may change in the future to make it safe.
    pub unsafe fn set_base_path(mut self, base_path: &'a CStr) -> Self {
        self.data.base_path = base_path.as_ptr();
        self
    }

    /// Set file icon.
    pub fn set_icon(mut self, icon: &'a sys::Icon) -> Self {
        self.data.icon = icon as *const sys::Icon;
        self
    }

    /// Do not show assets folder if true.
    pub fn set_skip_assets(mut self, skip_assets: bool) -> Self {
        self.data.skip_assets = skip_assets;
        self
    }

    /// Hide dot files if true.
    pub fn set_hide_dot_files(mut self, hide_dot_files: bool) -> Self {
        self.data.hide_dot_files = hide_dot_files;
        self
    }

    /// Hide extensions for files if true.
    pub fn set_hide_ext(mut self, hide_ext: bool) -> Self {
        self.data.hide_ext = hide_ext;
        self
    }

    /// Set callback function and context for providing custom icon and entry name.
    pub fn set_item_loader_callback(
        mut self,
        callback: sys::FileBrowserLoadItemCallback,
        context: *mut c_void,
    ) -> Self {
        self.data.item_loader_callback = callback;
        self.data.item_loader_context = context;
        self
    }
}

/// Displays a simple dialog.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn alert(text: &str) {
    let text = CString::new(text.as_bytes()).unwrap();

    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    message.set_text(&text, 0, 0, Align::Left, Align::Top);
    message.set_buttons(None, Some(c"OK"), None);

    dialogs.show_message(&message);
}
