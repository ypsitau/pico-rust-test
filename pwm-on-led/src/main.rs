#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
//use embassy_rp::gpio::{Level, Output};
use embassy_rp::pwm::{Pwm, SetDutyCycle};
use embassy_time::Timer;

use defmt_rtt as _;
use panic_probe as _;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    //let mut led = Output::new(p.PIN_25, Level::Low);
    let mut pwm = Pwm::new_output_b(p.PWM_SLICE4, p.PIN_25, Default::default());
    loop {
        for i in 0..=100 {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle_percent(i);
        }
        for i in (0..=100).rev() {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle_percent(i);
        }
        Timer::after_millis(500).await;
    }
}

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"pico-rust"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
