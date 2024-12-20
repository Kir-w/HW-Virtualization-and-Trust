/*Changements Majeurs pour l'ESP32
Registres I2C spécifiques :

Les registres de l'Atmega sont remplacés par les registres I2C bas-niveau de l'ESP32 (I2C_BASE, I2C_FIFO, etc.).
Configuration de l'horloge :

La période SCL (haute et basse) est calculée en fonction de la fréquence CPU de l'ESP32 et de la fréquence I2C cible.
FIFO I2C :

L'ESP32 utilise une FIFO pour transmettre et recevoir les données, contrairement à l'Atmega qui utilise des registres simples.
Commandes I2C :

Les commandes (I2C_CMD) permettent de lancer des séquences spécifiques, comme Start + Write ou Start + Read.*/
pub struct I2cInterface;
use core::ptr::{read_volatile, write_volatile};

// Définition des registres I2C pour l'ESP32 (I2C0 utilisé par défaut)
const I2C_BASE: u32 = 0x60013000; // Base de I2C0
const I2C_SCL_LOW_PERIOD: *mut u32 = (I2C_BASE + 0x10) as *mut u32;
const I2C_SCL_HIGH_PERIOD: *mut u32 = (I2C_BASE + 0x14) as *mut u32;
const I2C_CMD: *mut u32 = (I2C_BASE + 0x58) as *mut u32;
const I2C_FIFO: *mut u32 = (I2C_BASE + 0x60) as *mut u32;
const I2C_CTRL: *mut u32 = (I2C_BASE + 0x00) as *mut u32;
const I2C_STATUS: *const u32 = (I2C_BASE + 0x2C) as *const u32;

impl I2cInterface {
    /// Initialisation de l'I2C en mode maître
    pub fn init(freq_hz: u32) {
        unsafe {
            // Configurer les périodes de l'horloge (SCL low/high)
            let scl_period = 80_000_000 / freq_hz / 2; // Fréquence CPU par moitié d'horloge I2C
            write_volatile(I2C_SCL_LOW_PERIOD, scl_period);
            write_volatile(I2C_SCL_HIGH_PERIOD, scl_period);

            // Activer I2C et configurer comme maître
            write_volatile(I2C_CTRL, 0x1); // Mode maître activé
        }
    }

    /// Envoi d'un octet via I2C
    pub fn send(address: u8, data: u8) {
        unsafe {
            // Charger l'adresse et les données dans le FIFO
            write_volatile(I2C_FIFO, (address << 1) as u32); // Adresse avec bit d'écriture
            write_volatile(I2C_FIFO, data as u32); // Charger les données

            // Lancer la transmission
            write_volatile(I2C_CMD, 0x1); // Envoi Start + Write

            // Attendre que la transmission soit terminée
            while (read_volatile(I2C_STATUS) & 0x1) != 0 {} // Vérifier si le bus est occupé
        }
    }

    /// Lecture d'un octet via I2C
    pub fn read(address: u8, register: u8) -> u8 {
        unsafe {
            // Charger l'adresse et le registre à lire
            write_volatile(I2C_FIFO, (address << 1) as u32); // Adresse avec bit d'écriture
            write_volatile(I2C_FIFO, register as u32); // Charger le registre

            // Lancer l'envoi de l'adresse et du registre
            write_volatile(I2C_CMD, 0x1); // Envoi Start + Write
            while (read_volatile(I2C_STATUS) & 0x1) != 0 {} // Attendre que le bus soit libre

            // Charger l'adresse avec bit de lecture
            write_volatile(I2C_FIFO, ((address << 1) | 0x1) as u32);

            // Lancer la lecture
            write_volatile(I2C_CMD, 0x2); // Envoi Start + Read
            while (read_volatile(I2C_STATUS) & 0x1) != 0 {} // Attendre la fin

            // Lire les données du FIFO
            read_volatile(I2C_FIFO) as u8
        }
    }

    /// Fonction pour lire un capteur via I2C
    pub fn read_sensor(address: u8, register: u8) -> u8 {
        Self::read(address, register)
    }
}
