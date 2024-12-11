use rp_pico::hal::spi::{Enabled, Spi, SpiConfig};
use rp_pico::hal::gpio::{Pin, PinId, FunctionSpi};

pub struct SpiInterface {
    spi: Spi<Enabled, rp_pico::hal::pac::SPI0>,
}

impl SpiInterface {
    pub fn new<SpiSckPin: PinId, SpiMosiPin: PinId, SpiMisoPin: PinId>(
        sck: Pin<SpiSckPin, FunctionSpi>,
        mosi: Pin<SpiMosiPin, FunctionSpi>,
        miso: Pin<SpiMisoPin, FunctionSpi>,
        spi_block: rp_pico::hal::pac::SPI0,
    ) -> Self {
        let spi = Spi::new(spi_block, sck, mosi, miso, 500.khz(), SpiConfig::default());
        Self { spi }
    }

    pub fn transfer_data(&mut self, data: u8) -> u8 {
        let mut buffer = [data];
        self.spi.transfer(&mut buffer).unwrap();
        buffer[0]
    }
}
