pub mod atmega328p;
pub mod cortex_m7;

pub trait Spi {
    fn init(&self, mode: u8, frequency: u32);
    fn write_byte(&self, byte: u8);
    fn read_byte(&self) -> u8;
    fn transfer(&self, data: &[u8]) -> Vec<u8>; // Ajout pour transférer des données
}
