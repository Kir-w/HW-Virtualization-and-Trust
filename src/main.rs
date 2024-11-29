#![no_std]
#![no_main]

mod gpio;
mod usart;
mod spi;

#[cfg(target_arch = "avr")]
use gpio::{Gpio, PinMode};
#[cfg(target_arch = "avr")]
use usart::Usart;

#[cfg(target_arch = "arm")]
use gpio::{Gpio, PinMode};
#[cfg(target_arch = "arm")]
use usart::Usart;

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
        loop {
            gpio.digital_write(5, true); // Turn LED on
            Usart::send(1); // Send signal
            delay();
            gpio.digital_write(5, false); // Turn LED off
            Usart::send(0); // Send signal
            delay();
        }
    }
     
    #[cfg(target_arch = "arm")]
    {
        gpio.pin_mode(2, PinMode::Output); // Pin 2 on Cortex-M7
        Usart::init(9600); // Initialise USART
        loop {
            let led_state = Usart::receive(); // Receive signal
            gpio.digital_write(2, led_state != 0); // Set LED state
        }
    }

    #[cfg(target_arch = "avr")]
    {
        spi::Spi::init_master(); // Initialiser le SPI en mode maître

        loop {
            // Envoyer des données via SPI
            spi::Spi::send(0xAA);

            // Lire une réponse via SPI
            let received_data = spi::Spi::receive();

            // Vous pouvez utiliser les données reçues pour contrôler une LED, par exemple
        }
    }

}

fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}

