use crate::spi::{Spi, SpiConfig};

pub struct SpiDriver {
    // Ajoutez des champs nécessaires pour gérer l'état SPI
}

impl SpiDriver {
    /// Crée une nouvelle instance de l'abstraction SPI pour l'Atmega328p
    pub fn new() -> Self {
        // Initialisation spécifique au matériel (si nécessaire)
        Self {}
    }
}

impl Spi for SpiDriver {
    /// Initialisation du SPI pour l'Atmega328p
    fn init(&mut self, config: SpiConfig) {
        unsafe {
            // Activer le SPI en configurant les registres
            // Exemple pour Atmega328p :
            // SPCR : SPI Control Register
            // SPE  (bit 6) : Enable SPI
            // MSTR (bit 4) : Set as Master
            // SPR1, SPR0 (bits 0 et 1) : Clock rate select

            // Configure le registre SPCR
            let spcr = (1 << 6) | (1 << 4); // SPE et MSTR activés
            let spr = match config.frequency {
                400_000 => 0b00,      // Fréquence d'horloge divisée par 4
                1_000_000 => 0b01,   // Fréquence d'horloge divisée par 16
                _ => 0b11,           // Par défaut : diviseur le plus élevé
            };

            core::ptr::write_volatile(0x2C as *mut u8, spcr | spr); // Adresse de SPCR
        }
    }

    /// Transfert de données SPI
    fn transfer(&mut self, data: &[u8]) -> Vec<u8> {
        let mut received_data = Vec::new();

        for byte in data {
            unsafe {
                // Écrire l'octet à envoyer dans SPDR (SPI Data Register)
                core::ptr::write_volatile(0x2E as *mut u8, *byte); // Adresse de SPDR

                // Attendre que la transmission soit terminée (vérifier le flag SPIF)
                while (core::ptr::read_volatile(0x2D as *const u8) & (1 << 7)) == 0 {} // Adresse de SPSR

                // Lire la donnée reçue de SPDR
                let received_byte = core::ptr::read_volatile(0x2E as *const u8); // Adresse de SPDR
                received_data.push(received_byte);
            }
        }

        received_data
    }
}
