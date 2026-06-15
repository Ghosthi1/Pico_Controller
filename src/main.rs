#![no_std]
#![no_main]

use cortex_m::asm::delay;
use panic_halt as _;
use rp235x_hal as hal;
use hal::clocks::Clock;
use embedded_hal::digital::{InputPin, OutputPin};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let mut led_pin = pins.gpio28.into_push_pull_output();
    let mut right_button = pins.gpio2.into_pull_up_input();
    let mut middle_right_button = pins.gpio3.into_pull_up_input();
    let mut middle_left_button = pins.gpio4.into_pull_up_input();
    let mut left_button = pins.gpio5.into_pull_up_input();

    led_pin.set_high().unwrap();

    loop {
        match (right_button.is_low().unwrap(), middle_right_button.is_low().unwrap(), middle_left_button.is_low().unwrap(), left_button.is_low().unwrap()) {
            (true, false,false,false) => {
                led_pin.set_low().unwrap();
            }
            (false, true, false,false) => {
                led_pin.set_low().unwrap();
            }
            (false, false, true, false) => {
                led_pin.set_low().unwrap();
            }
            (false, false, false, true) => {
                led_pin.set_low().unwrap();
            }
            _ => { led_pin.set_high().unwrap();}
        }
    }
}