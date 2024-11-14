use core::ptr::{read_volatile, write_volatile};
use super::Usart;

// Registres spécifiques à l'USART de l'ATmega328p
const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;
const UCSRA: *mut u8 = 0xC0 as *mut u8;

pub struct Atmega328pUsart;

impl Usart for Atmega328pUsart {
    fn init(&self, baud_rate: u32) {
        // Calcule la valeur UBRR pour le baud rate spécifié
        let ubrr = ((16_000_000 / (16 * baud_rate)) - 1) as u16;
        unsafe {
            write_volatile(UBRR0H, (ubrr >> 8) as u8);
            write_volatile(UBRR0L, ubrr as u8);
            write_volatile(UCSR0B, 1 << 3 | 1 << 4); // Active TX et RX
            write_volatile(UCSR0C, 1 << 1 | 1 << 2); // Configure 8 bits de données, pas de parité, 1 bit de stop
        }
    }

    fn write_byte(&self, byte: u8) {
        unsafe {
            while read_volatile(UCSRA) & (1 << 5) == 0 {} // Attente de l'autorisation de transmission
            write_volatile(UDR0, byte); // Envoie le byte
        }
    }

    fn read_byte(&self) -> u8 {
        unsafe {
            while read_volatile(UCSRA) & (1 << 7) == 0 {} // Attente d'une donnée reçue
            read_volatile(UDR0)
        }
    }
}
