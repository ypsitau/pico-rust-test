//! This example shows how to use SPI (Serial Peripheral Interface) in the RP2040 chip.
//!
//! Example written for a display using the ST7789 chip. Possibly the Waveshare Pico-ResTouch
//! (https://www.waveshare.com/wiki/Pico-ResTouch-LCD-2.8)

#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio;
use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use mipidsi::options::{Orientation, Rotation};
use {defmt_rtt as _, panic_probe as _};

use crate::touch_xpt2046::Touch;

embassy_rp::bind_interrupts!(struct Irqs {
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH0>, embassy_rp::dma::InterruptHandler<embassy_rp::peripherals::DMA_CH1>;
});

const FREQ_DISPLAY: u32 = 64_000_000;
const FREQ_TOUCH: u32 = 200_000;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello World!");

    let spi_clk = p.PIN_10;
    let spi_mosi = p.PIN_11;
    let spi_miso = p.PIN_12;
    let display_rst = p.PIN_6;
    let display_dcx = p.PIN_7;
    let display_cs = p.PIN_8;
    let display_bl = p.PIN_9;
    let touch_cs = p.PIN_14;
    let _touch_irq = p.PIN_15;

    let spi_bus_shared = {
        let spi_bus = embassy_rp::spi::Spi::new_blocking(p.SPI1, spi_clk, spi_mosi, spi_miso, Default::default());
        //let spi_bus = embassy_rp::spi::Spi::new(p.SPI1, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, Irqs, Default::default());
        embassy_sync::blocking_mutex::Mutex::<embassy_sync::blocking_mutex::raw::NoopRawMutex, _>::new(RefCell::new(spi_bus))
    };
    let mut touch = {
        let spi_device = {
            let mut config = embassy_rp::spi::Config::default();
            config.frequency = FREQ_TOUCH;
            config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
            config.polarity = embassy_rp::spi::Polarity::IdleHigh;
            SpiDeviceWithConfig::new(&spi_bus_shared, gpio::Output::new(touch_cs, gpio::Level::High), config)
        };
        Touch::new(spi_device)
    };
    let mut display = {
        let dcx = gpio::Output::new(display_dcx, gpio::Level::Low);
        let rst = gpio::Output::new(display_rst, gpio::Level::Low);
        let spi_device = {
            let mut config = embassy_rp::spi::Config::default();
            config.frequency = FREQ_DISPLAY;
            config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
            config.polarity = embassy_rp::spi::Polarity::IdleHigh;
            SpiDeviceWithConfig::new(&spi_bus_shared, gpio::Output::new(display_cs, gpio::Level::High), config)
        };
        let display_interface = display_interface_spi::SPIInterface::new(spi_device, dcx);
        //use mipidsi::models::ST7789 as DisplayModel;
        use mipidsi::models::ILI9341Rgb565 as DisplayModel;
        mipidsi::Builder::new(DisplayModel, display_interface)
            .display_size(240, 320)
            .reset_pin(rst)
            .orientation(Orientation::new().rotate(Rotation::Deg90).flip_horizontal())
            .init(&mut embassy_time::Delay)
            .unwrap()

    };
    let _bl = gpio::Output::new(display_bl, gpio::Level::High);
    display.clear(pixelcolor::Rgb565::BLACK).unwrap();
    {
        let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86);
        let ferris = Image::new(&raw_image_data, Point::new(34, 68));
        ferris.draw(&mut display).unwrap();
    }
    {
        let style = MonoTextStyle::new(&FONT_10X20, pixelcolor::Rgb565::GREEN);
        Text::new(
            "Hello embedded_graphics \n + embassy + RP2040!",
            Point::new(20, 200),
            style,
        ).draw(&mut display).unwrap();
    }
    loop {
        if let Some((x, y)) = touch.read() {
            let style = PrimitiveStyleBuilder::new().fill_color(pixelcolor::Rgb565::BLUE).build();

            Rectangle::new(Point::new(x - 1, y - 1), Size::new(3, 3))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();
        }
    }
}

/// Driver for the XPT2046 resistive touchscreen sensor
mod touch_xpt2046 {
    use embedded_hal_1::spi::{Operation, SpiDevice};
    //use embedded_hal_async::spi::{Operation, SpiDevice};

    struct Calibration {
        xraw_max: i32,
        xraw_min: i32,
        yraw_min: i32,
        yraw_max: i32,
        x_range: i32,
        y_range: i32,
    }

    const CALIBRATION: Calibration = Calibration {
        xraw_min: 340,
        xraw_max: 3880,
        yraw_min: 262,
        yraw_max: 3850,
        x_range: 320,
        y_range: 240,
    };

    pub struct Touch<SPI: SpiDevice> {
        spi: SPI,
    }

    impl<SPI: SpiDevice> Touch<SPI> {
        pub fn new(spi: SPI) -> Self {
            Self { spi }
        }

        pub fn read(&mut self) -> Option<(i32, i32)> {
            let mut xbytes = [0u8; 2];
            let mut ybytes = [0u8; 2];
            self.spi
                .transaction(&mut [
                    Operation::Write(&[0x90]),
                    Operation::Read(&mut xbytes),
                    Operation::Write(&[0xd0]),
                    Operation::Read(&mut ybytes),
                ]).unwrap();
            let xraw = (u16::from_be_bytes(xbytes) >> 3) as i32;
            let yraw = (u16::from_be_bytes(ybytes) >> 3) as i32;
            let cal = &CALIBRATION;
            let x = ((xraw - cal.xraw_min) * cal.x_range / (cal.xraw_max - cal.xraw_min)).clamp(0, cal.x_range);
            let y = ((yraw - cal.yraw_min) * cal.y_range / (cal.yraw_max - cal.yraw_min)).clamp(0, cal.y_range);
            if x == 0 && y == 0 { None } else { Some((x, y)) }
        }
    }
}
