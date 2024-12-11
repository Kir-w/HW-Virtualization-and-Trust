use rp_pico::hal::uart::{self, Enabled, UartPeripheral};
use rp_pico::hal::gpio::{FunctionUart, Pin, PinId};

pub struct Usart {
    uart: UartPeripheral<Enabled, uart::Uart0>,
}

impl Usart {
    pub fn new<UartTxPin: PinId, UartRxPin: PinId>(
        tx_pin: Pin<UartTxPin, FunctionUart>,
        rx_pin: Pin<UartRxPin, FunctionUart>,
        uart_block: uart::Uart0,
    ) -> Self {
        let mut uart = UartPeripheral::new(uart_block, tx_pin, rx_pin, &mut uart::UartConfig::default());
        uart.enable(115200.bps());
        Self { uart }
    }

    pub fn send_data(&mut self, data: u8) {
        self.uart.write(data).unwrap();
    }
}
