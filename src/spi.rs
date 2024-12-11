#![no_std]

pub struct Spi;

impl Spi {
    // Initialisation de l'interface SPI
    pub fn init_master() {
        // Activer le SPI, mode maître, fréquence, etc.
        // Paramètres de configuration SPI : SPI maître, 4 MHz, SPI mode 0
        // L'ATmega328p utilise des registres spécifiques pour SPI : SPCR, SPSR, etc.

        // SPCR : SPI Control Register
        // Configure le mode maître, l'activation du SPI et l'activation de l'interruption
        // SPSR : SPI Status Register (ce registre contient des informations de statut)
        // La vitesse de communication dépend de l'horloge du MCU

        // Configurer SPI pour l'utilisation en mode maître
        unsafe {
            // Configure SPI pour le mode maître (MSTR), avec la division de fréquence appropriée
            // Par exemple, on configure le diviseur pour 4 MHz avec une fréquence de CPU à 16 MHz
            // SPCR = (1 << SPE) | (1 << MSTR) | (1 << SPR0);
            core::ptr::write_volatile(0x2C as *mut u8, 0x50); // SPI Control Register (SPCR) - MSTR, SPE, SPR0
            core::ptr::write_volatile(0x2D as *mut u8, 0x00); // SPI Status Register (SPSR) - Pas d'extension
        }
    }

    // Fonction pour envoyer des données via SPI
    pub fn send(data: u8) {
        unsafe {
            // Attendre que le buffer de données SPI soit vide (c'est-à-dire prêt à envoyer)
            while core::ptr::read_volatile(0x2D as *const u8) & (1 << 7) == 0 {}

            // Envoyer les données via le registre SPDR (SPI Data Register)
            core::ptr::write_volatile(0x2F as *mut u8, data); // SPDR : SPI Data Register
        }
    }

    // Fonction pour recevoir des données via SPI
    pub fn receive() -> u8 {
        unsafe {
            // Attendre que les données soient disponibles (dès que SPI a reçu un octet)
            while core::ptr::read_volatile(0x2D as *const u8) & (1 << 7) == 0 {}

            // Lire les données reçues dans le registre SPDR
            core::ptr::read_volatile(0x2F as *const u8) // SPDR : SPI Data Register
        }
    }
}
