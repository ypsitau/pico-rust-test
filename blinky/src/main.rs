//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = gpio::Output::new(p.PIN_25, gpio::Level::Low);

    loop {
        defmt::info!("led on!");
        led.set_high();
        Timer::after_secs(1).await;

        defmt::info!("led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}
