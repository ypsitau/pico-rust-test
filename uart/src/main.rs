#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::uart;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut uart = {
        let uart = p.UART0;
        let tx = p.PIN_0;
        let rx = p.PIN_1;
        let config = uart::Config::default();
        uart::Uart::new_blocking(uart, tx, rx, config)
    };
    defmt::unwrap!(uart.blocking_write("Hello World!\r\n".as_bytes()));
    loop {
        defmt::unwrap!(uart.blocking_write("hello there!\r\n".as_bytes()));
        Timer::after_millis(1000).await;
    }
}

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"your program name"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
