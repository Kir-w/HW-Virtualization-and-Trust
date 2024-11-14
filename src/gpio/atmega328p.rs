use core::ptr::{read_volatile, write_volatile};
use super::Gpio;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

pub struct Atmega328pGpio;

impl Gpio for Atmega328pGpio {
    fn pin_mode(&self, pin: u8, mode: bool) {
        unsafe {
            let current_ddr = read_volatile(DDRB);
            if mode {
                write_volatile(DDRB, current_ddr | (1 << pin));
            } else {
                write_volatile(DDRB, current_ddr & !(1 << pin));
            }
        }
    }

    fn digital_write(&self, pin: u8, value: bool) {
        unsafe {
            let current_port = read_volatile(PORTB);
            if value {
                write_volatile(PORTB, current_port | (1 << pin));
            } else {
                write_volatile(PORTB, current_port & !(1 << pin));
            }
        }
    }
}
