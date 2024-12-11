#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

mod i2c;
use i2c::I2cInterface;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut i2c = I2cInterface::new(pins.a4, pins.a5);

    loop {
        let temperature = i2c.read_sensor(0x40, 0x00); // Exemple : lire un registre
        arduino_hal::delay_ms(1000);
    }
}
