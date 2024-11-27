#[cfg(feature = "atmega328p")]
pub mod atmega328p;

#[cfg(feature = "cortex-m7")]
pub mod cortex_m7;

#[cfg(feature = "atmega328p")]
pub use atmega328p::SpiDriver as AtmegaSpi;

#[cfg(feature = "cortex-m7")]
pub use cortex_m7::SpiDriver as CortexM7Spi;

pub struct SpiConfig {
    pub mode: u8,     // Mode SPI (0, 1, 2, ou 3)
    pub frequency: u32, // Fréquence de l'horloge
}

pub trait Spi {
    fn init(&mut self, config: SpiConfig);
    fn transfer(&mut self, data: &[u8]) -> Vec<u8>;
}
