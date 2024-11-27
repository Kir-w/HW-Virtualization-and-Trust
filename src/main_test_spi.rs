#![no_std]
#![no_main]

mod gpio;
mod usart;
mod spi; // Ajout du module SPI

use gpio::{atmega328p::Atmega328pGpio, cortex_m7::CortexM7Gpio, Gpio};
use usart::{atmega328p::Atmega328pUsart, cortex_m7::CortexM7Usart, Usart};
use spi::{atmega328p::Atmega328pSpi, cortex_m7::CortexM7Spi, Spi, SpiConfig};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // GPIO initialisation
    let gpio_atmega = Atmega328pGpio;
    let gpio_cortex = CortexM7Gpio;

    // USART initialisation
    let usart_atmega = Atmega328pUsart;
    let usart_cortex = CortexM7Usart;

    // SPI initialisation
    let mut spi_atmega = Atmega328pSpi::new();
    let mut spi_cortex = CortexM7Spi::new();

    let spi_config = SpiConfig {
        mode: 0,
        frequency: 1_000_000,
    };

    spi_atmega.init(spi_config); // Initialisation SPI pour ATmega328p
    spi_cortex.init(spi_config); // Initialisation SPI pour Cortex-M7

    loop {
        // GPIO: Allume les LED
        gpio_atmega.digital_write(5, true);
        gpio_cortex.digital_write(5, true);

        // USART: Envoie 'A'
        usart_atmega.write_byte(0x41);
        usart_cortex.write_byte(0x41);

        // SPI: Transfert de données
        let sent_data = [0xAA, 0xBB, 0xCC];
        let received_atmega = spi_atmega.transfer(&sent_data);
        let received_cortex = spi_cortex.transfer(&sent_data);

        // Affiche les résultats (si un mécanisme d'affichage est disponible)
        // Vous pouvez aussi utiliser USART pour envoyer les résultats
        // Exemple fictif : print!("Atmega: {:?}, Cortex: {:?}", received_atmega, received_cortex);

        delay(); // Pause
    }
}

// Fonction de délai basique
fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
