use rp_pico::hal::i2c::{I2C, I2CPeripheral, Enabled};
use rp_pico::hal::gpio::{Pin, PinId, FunctionI2C};

pub struct I2cInterface {
    i2c: I2CPeripheral<Enabled, rp_pico::hal::pac::I2C0>,
}

impl I2cInterface {
    pub fn new<I2cSdaPin: PinId, I2cSclPin: PinId>(
        sda: Pin<I2cSdaPin, FunctionI2C>,
        scl: Pin<I2cSclPin, FunctionI2C>,
        i2c_block: rp_pico::hal::pac::I2C0,
    ) -> Self {
        let mut i2c = I2CPeripheral::new(i2c_block, sda, scl);
        i2c.enable(100.khz());
        Self { i2c }
    }

    pub fn read_sensor(&mut self, address: u8, register: u8) -> u8 {
        let mut buffer = [0];
        self.i2c.write_read(address, &[register], &mut buffer).unwrap();
        buffer[0]
    }
}
