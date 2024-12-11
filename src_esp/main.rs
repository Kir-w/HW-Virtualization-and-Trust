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
use spi::Spi;
use i2c::I2c;

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

    let mut gpio = Gpio::new(pins);
    let mut usart = Usart::new();
    let mut spi = Spi::new();
    let mut i2c = I2c::new();

    loop {
        gpio.blink_led();
        usart.send_data(1);
        spi.transfer_data(0x11);
        i2c.read_sensor();
    }
}
