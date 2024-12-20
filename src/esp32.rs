pub mod esp_gpio;
use esp_gpio::{Gpio, PinMode};

pub mod esp_usart;
use esp_usart::Usart;

pub mod esp_spi;
use esp_spi::Spi;

pub mod esp_i2c;
use esp_i2c::I2cInterface;

pub fn fn_esp() -> ! {
    let gpio = Gpio::new();
    gpio.pin_mode(2,PinMode::Output); // Configurer GPIO2 -> led embarquée
    Spi::init_master();
    Usart::init(9600); // Initialiser USART
    I2cInterface::init(400_000); // Initialiser I2C avec une fréquence de 400kHz

    loop {
        // Allumer la LED
        gpio.digital_write(2,true);
        Usart::send(1); // Envoyer un signal via USART
        Spi::send(0x11); // Envoyer des données via SPI
        let sensor_data = I2cInterface::read_sensor(0x40, 0x00); // Lire un capteur via I2C

        delay();

        // Éteindre la LED
        gpio.digital_write(2,false);
        Usart::send(0); // Envoyer un autre signal
        Spi::receive(); // Lire la réponse SPI
        delay();

        // Traiter les données du capteur I2C
        Usart::send(sensor_data);
    }
}

/// Implémentation d'une fonction de délai basique
fn delay() {
    for _ in 0..400_000 {
        // Attente simple
    }
}
