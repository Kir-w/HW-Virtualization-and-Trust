use core::ptr::{read_volatile, write_volatile};

// Définition des registres pour l'interface SPI sur l'Atmega328p
const SPCR: *mut u8 = 0x4C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x4D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x4E as *mut u8; // SPI Data Register

pub struct Spi;

impl Spi {
    // Initialise le SPI en mode maître avec un diviseur d'horloge
    pub fn init_master() {
        unsafe {
            // Configure SPCR : SPI activé, mode maître, diviseur d'horloge = 16
            write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0));
        }
    }

    // Envoie un octet de données via SPI
    pub fn send(data: u8) {
        unsafe {
            write_volatile(SPDR, data); // Charge les données dans le registre SPDR
            // Attendre la fin de la transmission
            while read_volatile(SPSR) & (1 << 7) == 0 {}
        }
    }

    // Reçoit un octet de données via SPI
    pub fn receive() -> u8 {
        unsafe {
            // Envoyer un octet vide pour générer un cycle d'horloge
            write_volatile(SPDR, 0x00);
            // Attendre la fin de la réception
            while read_volatile(SPSR) & (1 << 7) == 0 {}
            read_volatile(SPDR)
        }
    }
}