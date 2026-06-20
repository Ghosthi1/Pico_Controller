#![no_std]
#![no_main]

use panic_halt as _;
use rp235x_hal as hal;
use embedded_hal::digital::{InputPin, OutputPin};
use usb_device::{class_prelude::*, prelude::*};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

// Tells the pico where to start
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// Pico 2w frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// What the USB is and what buttons exist in it
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_4) = {
            #[packed_bits = 4] #[item_settings(data,variable,absolute)] buttons=input;
        };
        (usage_page = GENERIC_DESKTOP,) = {
            #[packed_bits = 4] #[item_settings(constant,variable,absolute)] padding=input;
        };
    }
)]
#[allow(dead_code)]
struct GamepadReport {
    buttons: u8,
    padding: u8,
}

#[hal::entry] // designates the start of the code
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap(); //Allows peripherals ot be accessed but only by a single function at a time

    // Sets up the clock for the pico
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio28.into_push_pull_output();
    let mut right_button = pins.gpio2.into_pull_up_input();
    let mut middle_right_button = pins.gpio3.into_pull_up_input();
    let mut middle_left_button = pins.gpio4.into_pull_up_input();
    let mut left_button = pins.gpio5.into_pull_up_input();

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USB,
        pac.USB_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut hid = HIDClass::new(&usb_bus, GamepadReport::desc(), 1);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[StringDescriptors::default()
            .manufacturer("Me")
            .product("Pico Gamepad")])
        .unwrap()
        .build();

    loop {
        usb_dev.poll(&mut [&mut hid]);

        let buttons: u8 = (right_button.is_low().unwrap() as u8)
            | ((middle_right_button.is_low().unwrap() as u8) << 1)
            | ((middle_left_button.is_low().unwrap() as u8) << 2)
            | ((left_button.is_low().unwrap() as u8) << 3);

        hid.push_input(&GamepadReport { buttons, padding: 0 }).ok();

        led.set_state((buttons != 0).into()).unwrap();
    }
}