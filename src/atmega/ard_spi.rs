//SPI pour l'arduino
pub struct Spi;
// SPI Registers
const SPCR: *mut u8 = 0x2C as *mut u8; 
const SPSR: *mut u8 = 0x2D as *mut u8; 
const SPDR: *mut u8 = 0x2E as *mut u8; 

// Initialisation SPI
pub fn init_master() {
    unsafe {
        *SPCR = (1 << 6) | (1 << 4) | (1 << 5); // Enable SPI, Set as Master, Set Clock Rate fck/16
    }
}

// Envoie de la data via SPI
pub fn send(data: u8) {
    unsafe {
        *SPDR = data; 
        while (*SPSR & (1 << 7)) == 0 {} // Attendre jusqu'à la transmission complete
        }
}

// Reception de la data via SPI
pub fn receive() -> u8 {
    send(0xFF); // Envoie dummy byte à generate clock
    unsafe { *SPDR } // Lire la data reçu
}
