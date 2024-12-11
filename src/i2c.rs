#![no_std]

pub struct I2cInterface;

impl I2cInterface {
    // Initialisation de l'I2C en tant que maître
    pub fn init() {
        // Configuration des registres pour initialiser l'I2C en mode maître
        unsafe {
            // Activer l'I2C, définir la vitesse et la configuration du périphérique
            // Dans cet exemple, nous configurons une vitesse de 100 kHz, typique pour I2C
            // Nous utilisons une fréquence de CPU de 16 MHz, donc une valeur appropriée pour TWBR

            // Activer le périphérique TWI (I2C) et définir le mode maître
            core::ptr::write_volatile(0xB8 as *mut u8, 0x00); // TWI Control Register (TWCR) - Désactivation de TWI avant modification

            // Configurer la vitesse (TWBR)
            core::ptr::write_volatile(0xB8 as *mut u8, 0x48); // Valeur de division pour une vitesse de 100 kHz à 16 MHz
            core::ptr::write_volatile(0xB9 as *mut u8, 0x00); // TWBR (TWI Bit Rate Register)

            // Activer le périphérique et le préparer à l'envoi de données
            core::ptr::write_volatile(0xB8 as *mut u8, 0x04); // TWI Control Register (TWCR) - Activation du périphérique
        }
    }

    // Envoi d'un octet via I2C
    pub fn send(address: u8, data: u8) {
        unsafe {
            // Définir l'adresse du périphérique I2C (avec l'indicateur d'écriture)
            core::ptr::write_volatile(0xB3 as *mut u8, address << 1); // Registre TWDR (Data Register)
            core::ptr::write_volatile(0xB8 as *mut u8, 0x04); // Réactiver TWI pour envoyer

            // Attendre que le périphérique soit prêt à envoyer
            while core::ptr::read_volatile(0xB9 as *const u8) & (1 << 7) == 0 {}

            // Envoyer les données
            core::ptr::write_volatile(0xB3 as *mut u8, data); // Charger les données à envoyer
            core::ptr::write_volatile(0xB8 as *mut u8, 0x04); // Envoyer les données via TWI

            // Attendre que le périphérique termine l'envoi
            while core::ptr::read_volatile(0xB9 as *const u8) & (1 << 7) == 0 {}
        }
    }

    // Lire un octet via I2C (sans acknowledgement)
    pub fn read(address: u8, register: u8) -> u8 {
        unsafe {
            // Définir l'adresse et le registre à lire
            core::ptr::write_volatile(0xB3 as *mut u8, (address << 1) | 0x01); // Registre TWDR : définir l'adresse du périphérique (avec lecture)

            // Activer la communication
            core::ptr::write_volatile(0xB8 as *mut u8, 0x04); // TWCR : Activer TWI pour débuter la communication

            // Attendre la fin de l'envoi
            while core::ptr::read_volatile(0xB9 as *const u8) & (1 << 7) == 0 {}

            // Lire les données de l'octet de registre
            core::ptr::write_volatile(0xB3 as *mut u8, register); // Registre du périphérique

            // Attendre que la réponse soit prête
            while core::ptr::read_volatile(0xB9 as *const u8) & (1 << 7) == 0 {}

            // Lire l'octet retourné
            let data = core::ptr::read_volatile(0xB3 as *const u8);
            return data;
        }
    }

    // Fonction pour lire un capteur via I2C (exemple d'un capteur avec adresse 0x40)
    pub fn read_sensor(address: u8, register: u8) -> u8 {
        Self::read(address, register)
    }
}