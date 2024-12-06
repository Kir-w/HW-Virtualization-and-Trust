use core::ptr::{read_volatile, write_volatile};

// Définition des registres pour l'USART sur l'Atmega328p
const UDR0: *mut u8 = 0xC6 as *mut u8;   // USART Data Register
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low

pub struct Usart;

impl Usart {
    pub fn init(baud_rate: u16) {
        let ubrr = ((16_000_000u32 / (16 * baud_rate as u32)) - 1) as u16;
        unsafe {
            write_volatile(UBRR0H, (ubrr >> 8) as u8);
            write_volatile(UBRR0L, ubrr as u8);

            // Activer TX et RX
            write_volatile(UCSR0B, (1 << 3) |(1 << 4) );
            // Configurer USART : 8 data bits, 1 stop bit, no parity
            write_volatile(UCSR0C, (1 << 1) | (1 << 2));
        }
    }

    pub fn send(data: u8) {
        unsafe {
            while read_volatile(UCSR0A) & (1 << 5) == 0 {}
            write_volatile(UDR0, data);
        }
    }

    pub fn receive() -> u8 { // essaye et fonctionne sans vrm utile ????
        unsafe {
            while read_volatile(UCSR0A) & (1 << 7) == 0 {}
            read_volatile(UDR0)
        }
    }
}
