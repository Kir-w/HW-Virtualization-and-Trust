/*Registres ESP32 :
Les registres AVR sont remplacés par des équivalents ESP32 comme GPIO_ENABLE_REG et GPIO_OUT_REG.
Largeur des registres :
Les registres ESP32 sont sur 32 bits, pas 8 bits comme sur AVR.
Configuration des broches :
Sur ESP32, chaque GPIO est configuré individuellement, contrairement à AVR qui utilise des registres pour des groupes de broches.
*/

pub struct Gpio;

pub enum PinMode {
    Output,
    Input,
}


impl Gpio {
    pub fn new() -> Self {
        Gpio
    }

    pub fn pin_mode(&self, pin: u8, mode: PinMode) {
        unsafe {
            let reg = 0x3FF44000 + (pin as u32) * 4; // Calculer l'adresse GPIO
            match mode {
                PinMode::Output => core::ptr::write_volatile(reg as *mut u32, 1),
                PinMode::Input => core::ptr::write_volatile(reg as *mut u32, 0),
            }
        }
    }

    pub fn digital_write(&self, pin: u8, value: bool) {
        let reg = 0x3FF44004 + (pin as u32) * 4; // Adresse de sortie
        unsafe {
            core::ptr::write_volatile(reg as *mut u32, if value { 1 } else { 0 });
        }
    }
}
