//! # Example: Triangles
//!
//! Shows multiple triangles with different styles.

use embedded_graphics as eg;
use embedded_graphics_simulator as eg_sim;

fn draw_triangles<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    use eg::prelude::*;
    use eg::primitives::PrimitiveStyleBuilder;

    let padding = Point::new(10, 20);
    let size = display.bounding_box().size.height as i32 - (padding.y * 2);
    let offset = size + 10;

    let triangle_up = eg::primitives::Triangle::new(
        Point::new(0, size), Point::new(size / 2, 0), Point::new(size, size)
    ).translate(padding);

    let triangle_down = eg::primitives::Triangle::new(
        Point::new(0, 0), Point::new(size, 0), Point::new(size / 2, size),
    ).translate(padding);

    // Inside thick stroke, no fill
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_SALMON)
        .stroke_width(10)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside)
        .build();
    triangle_up.into_styled(style).draw(display)?;

    // Center stroke alignment with fill
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_AQUAMARINE)
        .stroke_width(10)
        .fill_color(eg::pixelcolor::Rgb888::CSS_CADET_BLUE)
        .build();
    triangle_down.translate(Point::new(offset, 0)).into_styled(style).draw(display)?;

    // Outside stroke alignment with fill
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_FIRE_BRICK)
        .stroke_width(9)
        .stroke_alignment(eg::primitives::StrokeAlignment::Outside)
        .fill_color(eg::pixelcolor::Rgb888::CSS_WHITE_SMOKE)
        .build();
    triangle_up.translate(Point::new(offset * 2, 0)).into_styled(style).draw(display)?;

    // Fill only
    let style = PrimitiveStyleBuilder::new()
        .fill_color(eg::pixelcolor::Rgb888::CSS_CORAL)
        .build();
    triangle_down.translate(Point::new(offset * 3, 0)).into_styled(style).draw(display)?;

    // 1px stroke, no fill
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::WHITE)
        .stroke_width(1)
        .build();
    triangle_up.translate(Point::new(offset * 4, 0)).into_styled(style).draw(display)?;

    // Really thick stroke with inside alignment
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_DARK_TURQUOISE)
        .stroke_width(20)
        .stroke_alignment(eg::primitives::StrokeAlignment::Inside)
        .build();
    triangle_down.translate(Point::new(offset * 5, 0)).into_styled(style).draw(display)?;
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    use eg::prelude::*;
    let mut display = eg_sim::SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(600, 128));
    draw_triangles(&mut display)?;
    let output_settings = eg_sim::OutputSettingsBuilder::new().scale(2).build();
    eg_sim::Window::new("Triangles", &output_settings).show_static(&display);
    Ok(())
}
