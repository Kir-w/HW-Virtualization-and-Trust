pub mod atmega328p;
pub mod cortex_m7;

pub trait Usart {
    fn init(&self, baud_rate: u32);
    fn write_byte(&self, byte: u8);
    fn read_byte(&self) -> u8;
}
