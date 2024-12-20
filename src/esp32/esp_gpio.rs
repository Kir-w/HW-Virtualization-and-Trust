/*Registres ESP32 :
Les registres AVR sont remplacés par des équivalents ESP32 comme GPIO_ENABLE_REG et GPIO_OUT_REG.
Largeur des registres :
Les registres ESP32 sont sur 32 bits, pas 8 bits comme sur AVR.
Configuration des broches :
Sur ESP32, chaque GPIO est configuré individuellement, contrairement à AVR qui utilise des registres pour des groupes de broches.
*/
use core::ptr::{read_volatile, write_volatile};

// Définition des registres GPIO pour l'Atmega328P
const PORTB: *mut u8 = 0x25 as *mut u8; // Registre de sortie pour PORTB
const DDRB: *mut u8 = 0x24 as *mut u8;  // Registre de direction pour PORTB
const PINB: *const u8 = 0x23 as *const u8; // Registre d'entrée pour PORTB

// Énumération : représente le mode d'un pin
#[derive(Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
}

// Structure: représentation d'un GPIO
pub struct Gpio {
    pin: u8, // Broche GPIO utilisée (0-7 pour PORTB)
}

impl Gpio {
    /// Crée une nouvelle structure Gpio pour une broche donnée
    pub fn new(pin: u8) -> Self {
        Gpio { pin }
    }

    /// Configure le mode d'un pin (entrée ou sortie)
    pub fn pin_mode(&self, mode: PinMode) {
        match mode {
            PinMode::Input => {
                // Configurer le pin comme entrée
                unsafe {
                    let current_ddr = read_volatile(DDRB);
                    write_volatile(DDRB, current_ddr & !(1 << self.pin));
                }
            }
            PinMode::Output => {
                // Configurer le pin comme sortie
                unsafe {
                    let current_ddr = read_volatile(DDRB);
                    write_volatile(DDRB, current_ddr | (1 << self.pin));
                }
            }
        }
    }

    /// Écrit une valeur numérique (haut ou bas) sur un pin
    pub fn digital_write(&self, value: bool) {
        unsafe {
            if value {
                // Mettre le pin à 1
                let current_port = read_volatile(PORTB);
                write_volatile(PORTB, current_port | (1 << self.pin));
            } else {
                // Mettre le pin à 0
                let current_port = read_volatile(PORTB);
                write_volatile(PORTB, current_port & !(1 << self.pin));
            }
        }
    }

    /// Lit la valeur numérique (haut ou bas) d'un pin
    pub fn digital_read(&self) -> bool {
        unsafe {
            // Lire la valeur actuelle des entrées GPIO
            let pin_state = read_volatile(PINB);
            (pin_state & (1 << self.pin)) != 0
        }
    }
}
