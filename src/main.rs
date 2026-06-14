#![no_std]
#![no_main]

use panic_halt as _;
use rp235x_hal as hal;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let _pac = hal::pac::Peripherals::take().unwrap();

    loop {}
}