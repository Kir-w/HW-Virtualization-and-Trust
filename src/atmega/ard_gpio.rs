use core::ptr::{read_volatile, write_volatile};

// Definition des registres pour portB
const PORTB: *mut u8 = 0x25 as *mut u8;
const DDRB: *mut u8 = 0x24 as *mut u8;
const PINB: *const u8 = 0x23 as *const u8;

// Enumeration: les etats de la pin
#[derive(Clone, Copy)]
pub enum PinMode {
    _Input,
    Output,
}

// Structure: le GPIO
pub struct Gpio {
    port: *mut u8,
    ddr: *mut u8,
    _pin: *const u8,
}

impl Gpio {
    pub fn new() -> Self {
        Gpio {
            port: PORTB,
            ddr: DDRB,
            _pin: PINB,
        }
    }

    pub fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            let current_ddr: u8 = read_volatile(self.ddr);
            match mode {
                PinMode::_Input => write_volatile(self.ddr, current_ddr & !(1 << pin)),
                PinMode::Output => write_volatile(self.ddr, current_ddr | (1 << pin)),
            }
        }
    }

    pub fn digital_write(&self, pin: u8, value: bool) {
        unsafe {
            let current_port = read_volatile(self.port);
            if value {
                write_volatile(self.port, current_port | (1 << pin));
            } else {
                write_volatile(self.port, current_port & !(1 << pin));
            }
        }
    }
}
