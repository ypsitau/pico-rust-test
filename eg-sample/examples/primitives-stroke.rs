//! # Example: Primitive stroke styles
//!
//! This example demonstrates the different stroke styles available for primitives.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

const PADDING: i32 = 16;

/// Draws all embedded-graphics primitives.
fn draw_primitives<Display>(display: &mut Display, w: u32) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    use eg::prelude::*;
    use eg::primitives::PrimitiveStyleBuilder;
    eg::primitives::Triangle::new(Point::new(0, 64), Point::new(64, 0), Point::new(64, 64))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_ORANGE_RED).stroke_width(w).build())
        .draw(display)?;
    eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(64, 64))
        .translate(Point::new(64 + PADDING, 0))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_GOLD).stroke_width(w).build())
        .draw(display)?;
    eg::primitives::Line::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new((64 + PADDING) * 2, 0))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_SEA_GREEN).stroke_width(w).build())
        .draw(display)?;
    eg::primitives::Circle::new(Point::new(0, 0), 64)
        .translate(Point::new((64 + PADDING) * 3, 0))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_TEAL).stroke_width(w).build())
        .draw(display)?;
    eg::primitives::RoundedRectangle::new(
            eg::primitives::Rectangle::new(Point::new(0, 0), Size::new(64, 64)),
            eg::primitives::CornerRadii::new(Size::new(16, 16)))
        .translate(Point::new((64 + PADDING) * 4, 0))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_STEEL_BLUE).stroke_width(w).build())
        .draw(display)?;
    eg::primitives::Ellipse::new(Point::new(0, 0), Size::new(96, 64))
        .translate(Point::new((64 + PADDING) * 5, 0))
        .into_styled(PrimitiveStyleBuilder::new()
            .stroke_color(eg::pixelcolor::Rgb888::CSS_FUCHSIA).stroke_width(w).build())
        .draw(display)
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(512, 256));
    let mut position = Point::new(10, 10);
    draw_primitives(&mut display.translated(position), 1)?;
    position.y += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 3)?;
    position.y += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 10)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Strokes", &output_settings).show_static(&display);
    Ok(())
}
