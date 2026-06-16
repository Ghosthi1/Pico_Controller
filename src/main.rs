#![no_std]
#![no_main]

use panic_halt as _;
use rp235x_hal as hal;
use embedded_hal::digital::{InputPin, OutputPin};
use usb_device::bus::UsbBusAllocator;

static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
      (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
          (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_4) = {
              #[packed_bits = 4] #[item_settings(data,variable,absolute)] buttons=input;
          };
          (usage_page = GENERIC_DESKTOP,) = {
              #[packed_bits = 4] #[item_settings(constant,variable,absolute)]
  padding=input;
          };
      }
)]

#[allow(dead_code)]
struct GamepadReport {
    buttons: u8,
    padding: u8,
}

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
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