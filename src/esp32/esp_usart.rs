/*Changements majeurs pour ESP32
Adresses de registre spécifiques à l'ESP32 :

Les adresses comme UART_BASE et UART_FIFO remplacent celles de l'USART.
Chaque UART (UART0, UART1, UART2) a des registres similaires avec un offset différent.
Configuration du baud rate :

L'ESP32 utilise un diviseur d'horloge (calculez avec 80_000_000 / baud_rate pour une horloge à 80 MHz).
FIFO :

L'ESP32 utilise une FIFO (First In, First Out) pour gérer les données entrantes et sortantes.
Statut du FIFO :

On doit vérifier si le FIFO est prêt à envoyer ou à recevoir des données en lisant les bits du registre UART_STATUS.*/
pub struct Usart;
use core::ptr::{read_volatile};

// Définition des registres UART pour l'ESP32
const UART_BASE: u32 = 0x3FF40000; // Base pour UART0
const UART_FIFO: *mut u32 = (UART_BASE + 0x0) as *mut u32;
/*const UART_INT_RAW: *const u32 = (UART_BASE + 0x4) as *const u32;
const UART_CONF0: *mut u32 = (UART_BASE + 0x20) as *mut u32;
const UART_CLKDIV: *mut u32 = (UART_BASE + 0x14) as *mut u32;
*/const UART_STATUS: *const u32 = (UART_BASE + 0x1C) as *const u32;


impl Usart {
    /// Initialise l'UART avec le baud rate spécifié
    pub fn init(_baud_rate: u32) {
        
    }

    /// Envoie un octet via UART
    pub fn send(_data: u8) {
       
    }

    /// Reçoit un octet via UART
    pub fn receive() -> u8 { // Pas utilisée avec cette version du code
        unsafe {
            while read_volatile(UART_STATUS) & (1 << 0) == 0 {} // Attendre que des données soient disponibles

            read_volatile(UART_FIFO) as u8 // Lecture des données depuis le FIFO
        }
    }
}
