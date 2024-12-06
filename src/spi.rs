use core::ptr::{read_volatile, write_volatile};

// Définition des registres pour l'interface SPI sur l'Atmega328p
const SPCR: *mut u8 = 0x4C as *mut u8;   // SPI Control Register
const SPSR: *mut u8 = 0x4D as *mut u8;   // SPI Status Register
const SPDR: *mut u8 = 0x4E as *mut u8;   // SPI Data Register
const DDRB: *mut u8 = 0x24 as *mut u8;   // Data Direction Register B (PORTB)

pub struct Spi;

impl Spi {
    // Initialiser SPI en mode maître
    pub fn init_master() {
        unsafe {
            // Configurer les broches SPI comme sorties
            write_volatile(DDRB, (1 << 3) | (1 << 5) | (1 << 7)); // SCK (PB5), MOSI (PB3), SS (PB2)

            // Configurer SPCR : SPI activé, mode maître, CPOL = 0, CPHA = 0, diviseur d'horloge = 16
            write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0)); // SPI activé, maître, diviseur d'horloge
        }
    }

    // Envoyer un octet de données via SPI
    pub fn send(data: u8) {
        unsafe {
            write_volatile(SPDR, data); // Mettre les données dans le registre SPDR
            while read_volatile(SPSR) & (1 << 7) == 0 {} // Attendre que la transmission soit terminée
        }
    }

    // Recevoir un octet de données via SPI
    pub fn receive() -> u8 {
        unsafe {
            // Envoyer un octet vide pour générer un cycle d'horloge
            write_volatile(SPDR, 0x00);
            while read_volatile(SPSR) & (1 << 7) == 0 {} // Attendre la fin de la réception
            read_volatile(SPDR)
        }
    }
}