use super::Gpio;

pub struct CortexM7Gpio;

impl Gpio for CortexM7Gpio {
    fn pin_mode(&self, pin: u8, mode: bool) {
        // Configuration du mode pour un Cortex-M7 (adapter pour le modèle spécifique)
    }

    fn digital_write(&self, pin: u8, value: bool) {
        // Écriture sur le GPIO pour un Cortex-M7 (adapter pour le modèle spécifique)
    }
}
