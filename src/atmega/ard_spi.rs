pub struct Spi;
// SPI Registers
const SPCR: *mut u8 = 0x2C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x2D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x2E as *mut u8; // SPI Data Register


// Initialisation SPI
pub fn init_master() {
    unsafe {
        *SPCR = (1 << 6) | (1 << 4) | (1 << 5); // Enable SPI, Set as Master, Set Clock Rate fck/16
    }
}

// Envoie de la data via SPI
pub fn send(data: u8) {
    unsafe {
        *SPDR = data; // Load data into the buffer
        while (*SPSR & (1 << 7)) == 0 {} // Wait until transmission complete
    }
}

// Reception de la data via SPI
pub fn receive() -> u8 {
    send(0xFF); // Send dummy byte to generate clock
    unsafe { *SPDR } // Read received data from the buffer
}