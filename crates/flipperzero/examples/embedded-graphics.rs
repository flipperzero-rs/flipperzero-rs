//! embedded-graphics example for Flipper Zero.
//! This is based off the embedded-graphics "hello-world" example.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// embedded-graphics requires a global allocator.
extern crate flipperzero_alloc;

use core::ffi::CStr;
use core::time::Duration;

use flipperzero::furi::thread::sleep;
use flipperzero::gui::Gui;
use flipperzero_rt::{entry, manifest};

use embedded_graphics::mono_font::{MonoTextStyle, ascii::FONT_6X10};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
};
use embedded_graphics::text::{Alignment, Text};

// Define the FAP Manifest for this application
manifest!(
    name = "Embedded Graphics",
    app_version = 1,
    has_icon = true,
    // See `docs/icons.md` for icon format
    icon = "icons/rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    let gui = Gui::open();
    let mut canvas = gui.direct_draw_acquire();

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let yoffset = 14;

    // Draw a 3px wide outline around the display.
    canvas
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut *canvas)
        .unwrap();

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut *canvas)
    .unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut *canvas)
        .unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut *canvas)
        .unwrap();

    // Draw centered text.
    let text = "embedded-graphics";
    Text::with_alignment(
        text,
        canvas.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut *canvas)
    .unwrap();

    // You must commit the canvas for it to display on screen.
    canvas.commit();

    // Show for a few seconds, then exit.
    sleep(Duration::from_secs(5));

    0
}
