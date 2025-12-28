//! GUI service.

mod gui_layer;

pub mod canvas;
pub mod icon;
#[cfg(not(miri))]
pub mod icon_animation;
pub mod view;
pub mod view_dispatcher;
pub mod view_port;
#[cfg(feature = "xbm")]
pub mod xbm;

use core::ffi::CStr;
use core::ops::{Deref, DerefMut};

use canvas::CanvasView;
use flipperzero_sys as sys;
use flipperzero_sys::furi::UnsafeRecord;
use crate::gui::view_port::{ViewPort, ViewPortCallbacks};

pub use gui_layer::*;

/// System GUI wrapper.
pub struct Gui {
    record: UnsafeRecord<sys::Gui>,
}

impl Gui {
    /// Furi record corresponding to GUI.
    pub const NAME: &'static CStr = c"gui";

    /// Open record to GUI service.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flipperzero::gui::{view_port::ViewPort, Gui, GuiLayer};
    /// let view_port = ViewPort::new(());
    /// // create a GUI with a view port added to it
    /// let mut gui = Gui::new();
    /// let view_port = gui.add_view_port(view_port, GuiLayer::Desktop);
    /// ```
    pub fn open() -> Self {
        // SAFETY: `NAME` is a constant
        let gui = unsafe { UnsafeRecord::open(Self::NAME) };

        Self { record: gui }
    }

    /// Obtain raw pointer to GUI service.
    ///
    /// This pointer must not be free'd or used after the Gui object has been dropped.
    #[inline]
    pub fn as_ptr(&self) -> *mut sys::Gui {
        self.record.as_ptr()
    }

    pub fn add_view_port<VPC: ViewPortCallbacks>(
        &mut self,
        view_port: ViewPort<VPC>,
        layer: GuiLayer,
    ) -> GuiViewPort<'_, VPC> {
        let raw = self.as_ptr();
        let view_port_ptr = view_port.as_raw();
        let layer = layer.into();

        // SAFETY: all pointers are valid and `view_port` outlives this `Gui`
        unsafe { sys::gui_add_view_port(raw, view_port_ptr, layer) };

        GuiViewPort {
            parent: self,
            view_port,
        }
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
    pub fn direct_draw_acquire(&mut self) -> ExclusiveCanvas<'_> {
        let raw = self.as_ptr();

        // SAFETY: `raw` is a valid pointer
        let canvas = unsafe { CanvasView::from_raw(sys::gui_direct_draw_acquire(raw)) };

        ExclusiveCanvas { gui: self, canvas }
    }
}

/// `ViewPort` bound to a `Gui`.
pub struct GuiViewPort<'a, VPC: ViewPortCallbacks> {
    parent: &'a Gui,
    view_port: ViewPort<VPC>,
}

impl<'a, VPC: ViewPortCallbacks> GuiViewPort<'a, VPC> {
    /// Get the underlying `ViewPort`
    pub fn view_port(&self) -> &ViewPort<VPC> {
        &self.view_port
    }

    /// Get a mutable reference to the underlying `ViewPort`
    pub fn view_port_mut(&mut self) -> &mut ViewPort<VPC> {
        &mut self.view_port
    }

    /// Send this view port to the front of the GUI.
    pub fn send_to_front(&mut self) {
        let gui = self.parent.as_ptr();
        let view_port = self.view_port.as_raw();

        // SAFETY: `self.parent` outlives this `GuiVewPort`
        unsafe { sys::gui_view_port_send_to_front(gui, view_port) };
    }

    // pub fn send_to_back(&mut self) {
    //     let gui = self.parent.as_gui();
    //     let view_port = self.view_port.as_raw();
    //
    //     unsafe { sys::gui_view_port_send_to_back(gui, view_port) };
    // }

    /// Queue a GUI update.
    ///
    /// Note that the actual update will happen on another thread, whenever the GUI service
    /// receives the signal. This method will not block.
    pub fn update(&mut self) {
        let view_port = self.view_port.as_raw();

        // SAFETY: `view_port` is a valid pointer
        unsafe { sys::view_port_update(view_port) }
    }
}

impl<VPC: ViewPortCallbacks> Drop for GuiViewPort<'_, VPC> {
    fn drop(&mut self) {
        let gui = self.parent.as_ptr();
        let view_port = self.view_port().as_raw();

        // SAFETY: `gui` and `view_port` are valid pointers
        // and this view port should have been added to the gui on creation
        unsafe {
            sys::view_port_enabled_set(view_port, false);
            sys::gui_remove_view_port(gui, view_port);
            // the object has to be deallocated since the ownership was transferred to the `Gui`
            sys::view_port_free(view_port);
        }
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
    gui: &'a mut Gui,
    canvas: CanvasView<'a>,
}

impl Drop for ExclusiveCanvas<'_> {
    fn drop(&mut self) {
        let gui = self.gui.as_ptr();
        // SAFETY: this instance should have been created from `gui`
        // using `gui_direct_draw_acquire`
        // and will no longer be available since it is dropped
        unsafe { sys::gui_direct_draw_release(gui) };
    }
}

impl<'a> Deref for ExclusiveCanvas<'a> {
    type Target = CanvasView<'a>;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

impl<'a> DerefMut for ExclusiveCanvas<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.canvas
    }
}
