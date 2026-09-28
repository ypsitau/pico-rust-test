//! # Example: Text alignment
//!
//! Draw left/center/right aligned text in a containing box.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

fn draw_text<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    use eg::prelude::*;
    let bounding_box = display.bounding_box();

    let character_style = eg::mono_font::MonoTextStyleBuilder::new()
        .font(&eg::mono_font::ascii::FONT_8X13)
        .text_color(eg::pixelcolor::Rgb888::CSS_TOMATO).build();

    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Left).baseline(eg::text::Baseline::Top).build();
    eg::text::Text::with_text_style("Left aligned text, origin top left",
        bounding_box.top_left, character_style, text_style).draw(display)?;

    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Center).baseline(eg::text::Baseline::Middle).build();
    eg::text::Text::with_text_style("Center aligned text, origin center center",
        bounding_box.center(), character_style, text_style).draw(display)?;

    let text_style = eg::text::TextStyleBuilder::new()
        .alignment(eg::text::Alignment::Right).baseline(eg::text::Baseline::Bottom).build();
    eg::text::Text::with_text_style("Right aligned text, origin bottom right",
        bounding_box.bottom_right().unwrap(), character_style, text_style).draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(512, 128));
    draw_text(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Text alignment", &output_settings).show_static(&display);
    Ok(())
}
