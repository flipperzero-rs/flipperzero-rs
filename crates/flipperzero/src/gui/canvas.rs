//! Canvases.

use core::cell::UnsafeCell;
use core::marker::PhantomPinned;

use flipperzero_sys as sys;

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl Align {
    pub fn to_sys(&self) -> sys::Align {
        match self {
            Self::Left => sys::AlignLeft,
            Self::Right => sys::AlignRight,
            Self::Top => sys::AlignTop,
            Self::Bottom => sys::AlignBottom,
            Self::Center => sys::AlignCenter,
        }
    }
}

/// Graphics Canvas.
#[repr(transparent)]
pub struct Canvas {
    raw: UnsafeCell<sys::Canvas>,
    _marker: PhantomPinned,
}

impl Canvas {
    /// Get Canvas reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::Canvas`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw<'a>(raw: *mut sys::Canvas) -> &'a Self {
        unsafe { &*(raw.cast()) }
    }

    /// Get Canvas reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::Canvas`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw_mut<'a>(raw: *mut sys::Canvas) -> &'a mut Self {
        unsafe { &mut *(raw.cast()) }
    }

    /// Get pointer to raw [`sys::Canvas`].
    pub fn as_ptr(&self) -> *mut sys::Canvas {
        self.raw.get()
    }

    /// Get Canvas width and height.
    pub fn get_size(&self) -> (usize, usize) {
        unsafe {
            (
                sys::canvas_width(self.as_ptr()),
                sys::canvas_height(self.as_ptr()),
            )
        }
    }

    /// Clear Canvas.
    pub fn clear(&self) {
        unsafe { sys::canvas_clear(self.as_ptr()) }
    }

    /// Commit Canvas and send buffer to display.
    pub fn commit(&self) {
        unsafe { sys::canvas_commit(self.as_ptr()) }
    }
}

/// Support for [`embedded-graphics``](https://crates.io/crates/embedded-graphics) crate.
#[cfg(feature = "embedded-graphics")]
mod embedded_graphics {
    use super::*;
    use embedded_graphics_core::pixelcolor::BinaryColor;
    use embedded_graphics_core::prelude::*;
    use embedded_graphics_core::primitives::Rectangle;

    impl Dimensions for Canvas {
        fn bounding_box(&self) -> Rectangle {
            let (width, height) = self.get_size();

            Rectangle {
                top_left: (0, 0).into(),
                size: (width as u32, height as u32).into(),
            }
        }
    }

    impl DrawTarget for Canvas {
        type Color = BinaryColor;
        type Error = core::convert::Infallible;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let (width, height) = self.get_size();

            // Canvas size should easily fit into an `i32`
            let width = width.try_into().unwrap();
            let height = height.try_into().unwrap();

            unsafe {
                for Pixel(Point { x, y }, color) in pixels.into_iter() {
                    if (0..=width).contains(&x) && (0..=height).contains(&y) {
                        sys::canvas_set_color(self.as_ptr(), map_color(color));
                        sys::canvas_draw_dot(self.as_ptr(), x, y);
                    }
                }
            }

            Ok(())
        }

        fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            // Clamp rectangle coordinates to visible display area
            let area = area.intersection(&self.bounding_box());

            // Do not draw if the intersection size is zero.
            if area.bottom_right().is_none() {
                return Ok(());
            }

            unsafe {
                sys::canvas_set_color(self.as_ptr(), map_color(color));
                sys::canvas_draw_box(
                    self.as_ptr(),
                    area.top_left.x,
                    area.top_left.y,
                    area.size.width as usize,
                    area.size.height as usize,
                );
            }

            Ok(())
        }
    }

    /// Map embedded-graphics color to Furi color.
    #[inline]
    const fn map_color(color: BinaryColor) -> sys::Color {
        if color.is_on() {
            sys::ColorBlack
        } else {
            sys::ColorWhite
        }
    }
}
