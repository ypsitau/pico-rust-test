//! # Example: Fonts
//!
//! Demonstrate some of the available builtin fonts. A full list of fonts can be found in the
//! [embedded-graphics documentation](https://docs.rs/embedded-graphics).

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_5X8, FONT_6X12, FONT_9X15},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn draw_text<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::BinaryColor>,
{
    use eg::prelude::*;

    // Show smallest font with black font on white background (default value for fonts)
    Text::new(
        "Hello World! - default style 5x8",
        Point::new(15, 15),
        MonoTextStyle::new(&FONT_5X8, eg::pixelcolor::BinaryColor::On),
    )
    .draw(display)?;

    // Show smallest font with white font on black background
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(eg::pixelcolor::BinaryColor::Off)
        .background_color(eg::pixelcolor::BinaryColor::On)
        .build();

    Text::new("Hello World! - inverse 5x8", Point::new(15, 30), style).draw(display)?;

    // Show 6x12 Font
    Text::new(
        "Hello 6x12!",
        Point::new(15, 45),
        MonoTextStyle::new(&FONT_6X12, eg::pixelcolor::BinaryColor::On),
    )
    .draw(display)?;

    // Show 9x15 Font
    Text::new(
        "Hello 9x15!",
        Point::new(15, 70),
        MonoTextStyle::new(&FONT_9X15, eg::pixelcolor::BinaryColor::On),
    )
    .draw(display)?;

    // Show 10x20 Font
    Text::new(
        "Hello 10x20!",
        Point::new(15, 95),
        MonoTextStyle::new(&FONT_10X20, eg::pixelcolor::BinaryColor::On),
    )
    .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<eg::pixelcolor::BinaryColor> = SimulatorDisplay::new(Size::new(350, 200));
    draw_text(&mut display)?;
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
