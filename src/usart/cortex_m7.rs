use super::Usart;

pub struct CortexM7Usart;

impl Usart for CortexM7Usart {
    fn init(&self, baud_rate: u32) {
        // Configuration spécifique pour le Cortex-M7 (STM32) ici
    }

    fn write_byte(&self, byte: u8) {
        // Envoie le byte avec la configuration USART du Cortex-M7
    }

    fn read_byte(&self) -> u8 {
        // Lit le byte reçu pour le Cortex-M7
        0 // Remplacer par la lecture réelle
    }
}
