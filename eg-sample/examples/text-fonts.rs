//! # Example: Fonts
//!
//! Demonstrate some of the available builtin fonts. A full list of fonts can be found in the
//! [embedded-graphics documentation](https://docs.rs/embedded-graphics).

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

fn draw_text<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::BinaryColor>,
{
    const LINE_SPACING: i32 = 4;
    use eg::prelude::*;
    let entries = [
        ("Hello World! - FONT_4X6", &eg::mono_font::ascii::FONT_4X6),
        ("Hello World! - FONT_5X8", &eg::mono_font::ascii::FONT_5X8),
        ("Hello World! - FONT_6X12", &eg::mono_font::ascii::FONT_6X12),
        ("Hello World! - FONT_7X13", &eg::mono_font::ascii::FONT_7X13),
        ("Hello World! - FONT_8X13", &eg::mono_font::ascii::FONT_8X13),
        ("Hello World! - FONT_9X15", &eg::mono_font::ascii::FONT_9X15),
        ("Hello World! - FONT_10X20", &eg::mono_font::ascii::FONT_10X20),
    ];
    let mut pt = Point::new(15, 4);
    for (text, font) in entries.iter() {
        pt.y += font.character_size.height as i32;
        let mut character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(font).text_color(eg::pixelcolor::BinaryColor::On).build();
        eg::text::Text::new(text, pt, character_style).draw(display)?;
        pt.y += font.character_size.height as i32 + LINE_SPACING;
        let mut character_style = eg::mono_font::MonoTextStyleBuilder::new()
            .font(font).text_color(eg::pixelcolor::BinaryColor::Off).background_color(eg::pixelcolor::BinaryColor::On).build();
        eg::text::Text::new(text, pt, character_style).draw(display)?;
        pt.y += LINE_SPACING;
    }
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<eg::pixelcolor::BinaryColor>::new(Size::new(350, 240));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Fonts", &output_settings).show_static(&display);
    Ok(())
}
