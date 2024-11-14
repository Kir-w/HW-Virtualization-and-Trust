#![no_std]
#![no_main]

mod gpio;
mod usart;

use gpio::{atmega328p::Atmega328pGpio, cortex_m7::CortexM7Gpio, Gpio};
use usart::{atmega328p::Atmega328pUsart, cortex_m7::CortexM7Usart, Usart};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio_atmega = Atmega328pGpio;
    let gpio_cortex = CortexM7Gpio;

    let usart_atmega = Atmega328pUsart;
    let usart_cortex = CortexM7Usart;

    gpio_atmega.pin_mode(5, true); // Configure le pin 5 (LED sur ATmega328p) comme sortie
    gpio_cortex.pin_mode(5, true); // Configure le pin 5 (LED sur Cortex-M7) comme sortie

    usart_atmega.init(9600); // Initialise l'USART pour ATmega328p
    usart_cortex.init(9600); // Initialise l'USART pour Cortex-M7

    loop {
        gpio_atmega.digital_write(5, true); // Allume la LED sur ATmega328p
        gpio_cortex.digital_write(5, true); // Allume la LED sur Cortex-M7

        usart_atmega.write_byte(0x41); // Envoie 'A' via USART sur ATmega328p
        usart_cortex.write_byte(0x41); // Envoie 'A' via USART sur Cortex-M7

        delay();

        gpio_atmega.digital_write(5, false); // Éteint la LED sur ATmega328p
        gpio_cortex.digital_write(5, false); // Éteint la LED sur Cortex-M7

        delay();
    }
}

// Fonction de délai basique
fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
