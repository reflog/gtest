#![no_std]
#![no_main]

use panic_halt as _;
use pygamer as hal;

use embedded_graphics::prelude::*;

use embedded_graphics::image::Image;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use shared::entities;
use tinytga::Tga;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();
    let tga: Tga = Tga::from_slice(include_bytes!("../../assets/ground.tga")).unwrap();
    let image = Image::new(&tga, Point::zero());
    image.draw(&mut display).unwrap();
    let mut wolf = entities::Wolf::new();
    wolf.draw(&mut display);
    loop {}
}
