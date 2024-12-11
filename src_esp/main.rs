#![no_std]
#![no_main]

use rp_pico::entry;
use rp_pico::hal::{self, pac, watchdog::Watchdog};
use rp_pico::hal::clocks::Clock;
use embedded_hal::digital::v2::OutputPin;

mod gpio;
mod usart;
mod spi;
mod i2c;

use gpio::Gpio;
use usart::Usart;
use spi::SpiInterface;
use i2c::I2cInterface;

use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut gpio = Gpio::new(pins.gpio25.into_push_pull_output());
    let mut usart = Usart::new(
        pins.gpio0.into_function_uart(),
        pins.gpio1.into_function_uart(),
        pac.UART0,
    );
    let mut spi = SpiInterface::new(
        pins.gpio18.into_function_spi(),
        pins.gpio19.into_function_spi(),
        pins.gpio16.into_function_spi(),
        pac.SPI0,
    );
    let mut i2c = I2cInterface::new(
        pins.gpio8.into_function_i2c(),
        pins.gpio9.into_function_i2c(),
        pac.I2C0,
    );

    loop {
        gpio.blink_led();
        usart.send_data(1);
        let _spi_response = spi.transfer_data(0x11);
        let _i2c_data = i2c.read_sensor(0x40, 0x00); // Exemple avec une adresse et un registre
    }
}
