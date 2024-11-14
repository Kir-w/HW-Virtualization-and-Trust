pub mod atmega328p;
pub mod cortex_m7;

pub trait Gpio {
    fn pin_mode(&self, pin: u8, mode: bool);
    fn digital_write(&self, pin: u8, value: bool);
}
