#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

// Definitions for PORTB registers
const PORTB: *mut u8 = 0x25 as *mut u8;
const DDRB: *mut u8 = 0x24 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Enumeration: represent the state of a pin
#[derive(Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
}

// Structure: represent GPIO
pub struct Gpio {
    port: *mut u8,
    ddr: *mut u8,
    pin: *const u8,
}

impl Gpio {
    // Creates a new GPIO instance for PORTB
    pub fn new() -> Self {
        Gpio {
            port: PORTB,
            ddr: DDRB,
            pin: PINB,
        }
    }

    // Configures a pin as input (0) or output (1)
    pub fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            let current_ddr = read_volatile(self.ddr);
            match mode {
                PinMode::Input => write_volatile(self.ddr, current_ddr & !(1 << pin)),
                PinMode::Output => write_volatile(self.ddr, current_ddr | (1 << pin)),
            }
        }
    }

    // Reads the state of a pin
    pub fn digital_read(&self, pin: u8) -> bool {
        unsafe {
            (read_volatile(self.pin) & (1 << pin)) != 0 // Returns true if the bit is 1 (high)
        }
    }

    // Writes a value to the pin
    pub fn digital_write(&self, pin: u8, value: bool) {
        unsafe {
            let current_port = read_volatile(self.port);
            if value {
                write_volatile(self.port, current_port | (1 << pin)); // Sets the bit to 1 (high)
            } else {
                write_volatile(self.port, current_port & !(1 << pin)); // Sets the bit to 0 (low)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio = Gpio::new();

    gpio.pin_mode(5, PinMode::Output); // Set pin 5 (PORTB5, Arduino pin 13) as output (led)

    loop {
        gpio.digital_write(5, true); // Turn on LED
        delay();                     // Delay
        gpio.digital_write(5, false); // Turn off LED
        delay();                     // Delay
    }
}

// Delay function
fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
