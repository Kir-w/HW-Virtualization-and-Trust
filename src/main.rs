#![no_std]
#![no_main]

use core::panic::PanicInfo;
use gpio::{Gpio, PinMode}; // Importation of the gpio module

// To handle the errors
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Main fonction :
#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio = Gpio::new();

    gpio.pin_mode(5, PinMode::Output); // PORTB5, Arduino pin 13 for the led

    loop {
        gpio.digital_write(5, true); // Lights up the LED
        delay();                     // Pause
        gpio.digital_write(5, false); // Turns off the LED
        delay();                     // Pause
    }
}

fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
