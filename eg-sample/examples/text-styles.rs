//! # Example: Text styles
//!
//! Display a sentence of text using different styles, colors an decorations.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

fn draw_text<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    use eg::prelude::*;
    let character_style_normal = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg::mono_font::ascii::FONT_8X13).text_color(eg::pixelcolor::Rgb888::WHITE).build();
    // First line
    let position = Point::new(15, 15);
    let position = {
        eg::text::Text::new("A sentence with normal, ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(eg::pixelcolor::Rgb888::CSS_YELLOW).underline().build();
        eg::text::Text::new("yellow underline", position, character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .strikethrough_with_color(eg::pixelcolor::Rgb888::RED).build();
        eg::text::Text::new("red strikethrough", position, character_style).draw(display)?
    };
    let _ = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    // Second line
    let position = Point::new(15, 15 + eg::mono_font::ascii::FONT_8X13.character_size.height as i32);
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(&eg::mono_font::ascii::FONT_8X13_BOLD).build();
        eg::text::Text::new("bold", position,
            character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(", ", position, character_style_normal).draw(display)?
    };
    let position = {
        let character_style = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .text_color(eg::pixelcolor::Rgb888::CSS_TOMATO).background_color(eg::pixelcolor::Rgb888::CSS_WHEAT).build();
        eg::text::Text::new("highlighted", position, character_style).draw(display)?
    };
    let position = {
        eg::text::Text::new(" and ", position, character_style_normal).draw(display)?
    };
    let position = {
        let italic = eg::mono_font::MonoTextStyleBuilder::from(&character_style_normal)
            .font(&eg::mono_font::ascii::FONT_8X13_ITALIC).build();
        eg::text::Text::new("italic", position, italic).draw(display)?
    };
    let _ = {
        eg::text::Text::new(" text!", position, character_style_normal).draw(display)?
    };
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(512, 128));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Text styles", &output_settings).show_static(&display);
    Ok(())
}
