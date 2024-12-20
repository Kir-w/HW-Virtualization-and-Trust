pub struct Spi;

// Registre SPI
const SPCR: *mut u8 = 0x2C as *mut u8; // SPI Registre de contrôle
const SPSR: *mut u8 = 0x2D as *mut u8; // SPI Registre de statut
const SPDR: *mut u8 = 0x2E as *mut u8; // SPI Registre de données

impl Spi {
    /// Initialise le SPI en mode maître
    pub fn init_master() {
        unsafe {
            *SPCR = (1 << 6) | (1 << 4) | (1 << 5); // Active SPI, met en mode Master, met Clock Rate en fck/16
        }
    }

    /// Envoie de la data via SPI
    pub fn send(data: u8) {
        unsafe {
            *SPDR = data; // Charge les données dans le buffer
            while (*SPSR & (1 << 7)) == 0 {} // Attente jusqu'à ce que la transmission soit complète
        }
    }

    /// Réception de la data via SPI
    pub fn receive() -> u8 {
        // Envoyer un octet bidon pour générer l'horloge
        Spi::send(0xFF);
        unsafe { *SPDR } // Lire les données reçues
    }
}
