/*Changements majeurs pour l'ESP32
Registres SPI spécifiques à l'ESP32 :

Les registres AVR (SPCR, SPSR, SPDR) sont remplacés par des registres SPI de l'ESP32, comme SPI_CMD, SPI_W0, etc.
Configuration du SPI :

Le SPI de l'ESP32 doit être configuré manuellement pour activer le mode maître, les longueurs des données MOSI/MISO, et les fréquences d'horloge.
Mode de transfert :

La transmission et la réception sur ESP32 se fait via un buffer (SPI_W0). Lors de la réception, un octet "dummy" (0xFF) doit être envoyé pour générer l'horloge.
Horloge SPI :

L'ESP32 permet une configuration précise de l'horloge SPI via SPI_CLOCK.*/
pub struct Spi;
use core::ptr::{read_volatile, write_volatile};

// Définition des registres SPI pour l'ESP32 (SPI2 utilisé comme maître par défaut)
const SPI_BASE: u32 = 0x3FF42000; // Base de SPI2
const SPI_CMD: *mut u32 = (SPI_BASE + 0x00) as *mut u32; // SPI Command Register
const SPI_MOSI_DLEN: *mut u32 = (SPI_BASE + 0x24) as *mut u32; // MOSI Data Length Register
const SPI_MISO_DLEN: *mut u32 = (SPI_BASE + 0x28) as *mut u32; // MISO Data Length Register
const SPI_W0: *mut u32 = (SPI_BASE + 0x80) as *mut u32; // Data Buffer
const SPI_CTRL: *mut u32 = (SPI_BASE + 0x08) as *mut u32; // Control Register
const SPI_CLOCK: *mut u32 = (SPI_BASE + 0x1C) as *mut u32; // Clock Configuration Register
const SPI_USER: *mut u32 = (SPI_BASE + 0x1C) as *mut u32; // User Configuration Register
const SPI_USER1: *mut u32 = (SPI_BASE + 0x20) as *mut u32; // User Configuration Register 1

/// Initialisation SPI en mode maître
pub fn init_master() {
    unsafe {
        // Configurer SPI en maître
        write_volatile(SPI_CTRL, 0); // Désactiver les fonctions inutiles
        write_volatile(SPI_USER, (1 << 6)); // Activer MOSI
        write_volatile(SPI_CLOCK, 0x0000000E); // Configurer SPI Clock (fck/16)

        // Longueur des données
        write_volatile(SPI_MOSI_DLEN, 7); // Longueur MOSI : 8 bits
        write_volatile(SPI_MISO_DLEN, 7); // Longueur MISO : 8 bits
    }
}

/// Envoi de données via SPI
pub fn send(data: u8) {
    unsafe {
        write_volatile(SPI_W0, data as u32); // Charger la donnée dans le buffer
        write_volatile(SPI_CMD, 1 << 7); // Lancer la transmission
        while (read_volatile(SPI_CMD) & (1 << 7)) != 0 {} // Attendre la fin de la transmission
    }
}

/// Réception de données via SPI
pub fn receive() -> u8 {
    send(0xFF); // Envoyer un octet "dummy" pour générer l'horloge
    unsafe { read_volatile(SPI_W0) as u8 } // Lire la donnée reçue depuis le buffer
}
