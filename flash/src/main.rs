//! Example demonstrating blocking NOR flash operations on an embedded board.
#![no_std]
#![no_main]

mod flash_buf;

mod emb {
    pub use embassy_executor as executor;
}
use embedded_storage::{
    self as es,
    nor_flash::ReadNorFlash as _, nor_flash::NorFlash as _
};

struct Board<Flash> {
    pub flash: Flash,
}

#[emb::executor::main]
async fn main(_spawner: emb::executor::Spawner) {
    let mut board = init_board();
    info!("Read: {}bytes, Write: {}bytes, Erase: {}bytes", board.flash_read_size(), board.flash_write_size(), board.flash_erase_size());
    let offset = 0x0010_0000;
    let erase_size = board.flash_erase_size() as u32;
    board.flash.erase(offset, offset + erase_size).unwrap();
    let mut buf = [0u8; 256];
    board.flash.read(offset, &mut buf).unwrap();
    info!("read after erase: {=[u8]:02x}", buf);
    for (i, pbuf) in buf.iter_mut().enumerate() {
        *pbuf = (i & 0xff) as u8;
    }
    board.flash.write(offset, &buf).unwrap();
    buf.fill(0u8);
    info!("cleared: {=[u8]:02x}", buf);
    board.flash.read(offset, &mut buf).unwrap();
    info!("read after write: {=[u8]:02x}", buf);
    let _ = core::future::pending::<()>();
}

//-----------------------------------------------------------------------------
// board-specific implementation
//-----------------------------------------------------------------------------
use defmt::info;
use embassy_rp as rp;
use {defmt_rtt as _, panic_probe as _};

#[allow(dead_code)]
impl<Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash> Board<Flash> {
    const fn flash_read_size(&self) -> usize {
        Flash::READ_SIZE
    }
    const fn flash_write_size(&self) -> usize {
        Flash::WRITE_SIZE
    }
    const fn flash_erase_size(&self) -> usize {
        Flash::ERASE_SIZE
    }
}

//rp::bind_interrupts!(struct Irqs {
//    DMA_IRQ_0 => rp::dma::InterruptHandler<rp::peripherals::DMA_CH0>;
//});

fn init_board() -> Board<impl es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash> {
    const FLASH_CAPACITY: usize = 2 * 1024 * 1024;
    let p = rp::init(Default::default());
    type Flash = rp::flash::Flash<'static, rp::peripherals::FLASH, rp::flash::Blocking, FLASH_CAPACITY>;
    let flash = Flash::new_blocking(p.FLASH);
    //type Flash = rp::flash::Flash<'static, rp::peripherals::FLASH, rp::flash::Async, FLASH_CAPACITY>;
    //let flash = Flash::new(p.FLASH, p.DMA_CH0, Irqs);
    Board { flash }
}
