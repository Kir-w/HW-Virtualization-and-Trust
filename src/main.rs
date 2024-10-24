#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Definitions for ports and registers, with each address
const PORTB: *mut u8 = 0x25 as *mut u8;
const DDRB: *mut u8 = 0x24 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// enumeration to represent the state of a pin (input or output)
#[derive(Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
}

// structure to represent GPIO
pub struct Gpio {
    port: *mut u8,
    ddr: *mut u8,
    pin: *const u8,
}

impl Gpio {
    // this creates a new GPIO instance for PORTB
    pub fn new() -> Self {
        Gpio {
            port: PORTB,
            ddr: DDRB,
            pin: PINB,
        }
    }

    // configures a pin as either input (0) or output (1)
    pub fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => {
                    *self.ddr &= !1 << pin;
                }
                PinMode::Output => {
                    *self.ddr |= 1 <<pin; 
                }
            }
        }
    }

    // reads the state of a pin
    pub fn digital_read(&self, pin: u8) -> bool {
        unsafe {
            (*self.pin & 1 << pin) != 0 // returns true if the bit is 1 (high)
        }
    }

    // writes a value to the pin
    pub fn digital_write(&self, pin: u8, value: bool) {
        unsafe {
            if value {
                *self.port |= 1 << pin; // sets the bit to 1 (high)
            } else {
                *self.port &= !1 << pin; // sets the bit to 0 (low)
            }
        }
    }
}

// Example
#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio = Gpio::new();

    gpio.pin_mode(2, PinMode::Output); // set pin 2 as output
    gpio.digital_write(2, true); // set pin 2 to high

    gpio.pin_mode(3, PinMode::Input); // set pin 3 as input
    let state = gpio.digital_read(3); // read the state of pin 3

    loop {
        gpio.digital_write(2, !state); // inverse state of pin 3
        for _ in 0..100_000 { // delay
            unsafe { core::ptr::read_volatile(&state) }; // NOP (no operation)
        }
    }
}
