//! # Example: Triangles
//!
//! Shows multiple triangles with different styles.

use embedded_graphics as eg;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, Triangle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn draw_triangles<Display>(display: &mut Display) -> Result<(), Display::Error>
where
    Display: eg::draw_target::DrawTarget<Color = eg::pixelcolor::Rgb888>,
{
    let padding = Point::new(10, 20);
    let size = display.bounding_box().size.height as i32 - (padding.y * 2);
    let half_size = size / 2;
    let offset = size + 10;

    // Triangle pointing up
    let base_triangle = Triangle::new(
        Point::new(0, size),
        Point::new(half_size, 0),
        Point::new(size, size),
    ).translate(padding);

    // Triangle pointing down
    let flipped_triangle = Triangle::new(
        Point::new(0, 0),
        Point::new(size, 0),
        Point::new(half_size, size),
    ).translate(padding);

    // Inside thick stroke, no fill
    let inside_thick_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_SALMON)
        .stroke_width(10)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    base_triangle
        .into_styled(inside_thick_stroke)
        .draw(display)?;

    // Center stroke alignment with fill
    let center_stroke_fill = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_AQUAMARINE)
        .stroke_width(10)
        .fill_color(eg::pixelcolor::Rgb888::CSS_CADET_BLUE)
        .build();
    flipped_triangle
        .translate(Point::new(offset, 0))
        .into_styled(center_stroke_fill)
        .draw(display)?;

    // Outside stroke alignment with fill
    let outside_stroke_fill = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_FIRE_BRICK)
        .stroke_width(9)
        .stroke_alignment(StrokeAlignment::Outside)
        .fill_color(eg::pixelcolor::Rgb888::CSS_WHITE_SMOKE)
        .build();
    base_triangle
        .translate(Point::new(offset * 2, 0))
        .into_styled(outside_stroke_fill)
        .draw(display)?;

    // Fill only
    flipped_triangle
        .translate(Point::new(offset * 3, 0))
        .into_styled(PrimitiveStyle::with_fill(eg::pixelcolor::Rgb888::CSS_CORAL))
        .draw(display)?;

    // 1px stroke, no fill
    base_triangle
        .translate(Point::new(offset * 4, 0))
        .into_styled(PrimitiveStyle::with_stroke(eg::pixelcolor::Rgb888::WHITE, 1))
        .draw(display)?;

    // Really thick stroke with inside alignment
    let thick_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(eg::pixelcolor::Rgb888::CSS_DARK_TURQUOISE)
        .stroke_width(20)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    flipped_triangle
        .translate(Point::new(offset * 5, 0))
        .into_styled(thick_stroke)
        .draw(display)?;
    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<eg::pixelcolor::Rgb888>::new(Size::new(600, 128));
    draw_triangles(&mut display)?;
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Triangles", &output_settings).show_static(&display);
    Ok(())
}
