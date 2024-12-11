#![no_std]
#![no_main]

#[cfg(feature= "gpio")]
pub mod gpio;
use gpio::{Gpio, PinMode};

#[cfg(feature = "usart")]
pub mod usart;
use usart::Usart;

#[cfg(feature = "spi")]
pub mod spi;
use spi::Spi;

#[cfg(feature = "i2c")]
pub mod i2c;
use i2c::I2cInterface;


// To handle the errors
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio = Gpio::new();

    #[cfg(target_arch = "avr")]
    {
        
        gpio.pin_mode(5, PinMode::Output); // PORTB5, Arduino Pin 13
        Usart::init(9600); // Initialise USART
        Spi::init_master();//Initialise SPI
        I2cInterface::init(); // initialise i2c

        loop {
            gpio.digital_write(5, true); // Turn LED on
            Usart::send(1); // Send signal via usart
            Spi::send(0x11); //send data via spi
            let sensor_data = I2cInterface::read_sensor(0x40, 0x00); // read captor via I2C

            delay();

            gpio.digital_write(5, false); // Turn LED off
            Usart::send(0); // Send signal
            Spi::receive(); //read response
            delay();

            // Traiter les données du capteur I2C (par exemple, envoyer via USART)
            Usart::send(sensor_data);

        }
    }

    /*
    #[cfg(target_arch = "arm")]
    {
        gpio.pin_mode(2, PinMode::Output); // Pin 2 on Cortex-M7
        Usart::init(9600); // Initialise USART
        loop {
            let led_state = Usart::receive(); // Receive signal
            gpio.digital_write(2, led_state != 0); // Set LED state
        }
    }*/

    /*#[cfg(target_arch = "avr")] // unreacheable ?? 
    {
        Spi::init_master();//Initialise SPI
        loop {
            Spi::send(0x11); //send data via spi
            delay();
            Spi::receive(); //read response
            delay();
        }
    }*/

}

fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}