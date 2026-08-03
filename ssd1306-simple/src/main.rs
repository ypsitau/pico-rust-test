#![no_std]
#![no_main]

use core::fmt::Write;
use defmt::expect;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::Point,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use ssd1306::prelude::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<peripherals::I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let i2c0 = {
        let sda = p.PIN_16;
        let scl = p.PIN_17;
        let mut config = i2c::Config::default();
        config.frequency = 400_000;
        i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, config)
    };
    let mut display = {
        let interface = ssd1306::I2CDisplayInterface::new(i2c0);
        ssd1306::Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode()
    };
    expect!(display.init().await);
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();
    for i in 0..=1 {
        let mut text: String<16> = String::new();
        expect!(write!(text, "line.{}", i + 1));
        expect!(
            Text::with_baseline(
                text.as_str(),
                Point::new(0, i * 20),
                text_style,
                Baseline::Top,
            )
            .draw(&mut display)
        );
    }
    expect!(display.flush().await);
    loop {
        Timer::after_millis(1000).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"ssd1306-simple"),
    embassy_rp::binary_info::rp_program_description!(c"Hello OLED"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
