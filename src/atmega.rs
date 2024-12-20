//EQUIVALENT DE LA FONCTION MAIN POUR ATMEGA QUI SERA APPELE DANS LE MAIN SI ON UTILISE UNE ARDUINO
pub mod ard_gpio;
use ard_gpio::{Gpio, PinMode};

pub mod ard_usart;
use ard_usart::Usart;

pub mod ard_spi;
use ard_spi::Spi;

pub mod ard_i2c;
use ard_i2c::I2cInterface;



pub fn fn_atm()-> ! {
    let gpio = Gpio::new();
    gpio.pin_mode(5, PinMode::Output); // PORTB5, Arduino Pin 13
        Usart::init(9600); // Initialise USART
        //Spi::init_master();//Initialise SPI
        I2cInterface::init(); // initialise i2c

        loop {
            gpio.digital_write(5, true); // Turn LED on
            Usart::send(1); // Send signal via usart
            //Spi::send(0x11); //send data via spi
            let sensor_data = I2cInterface::read_sensor(0x40, 0x00); // read captor via I2C

            delay();

            gpio.digital_write(5, false); // Turn LED off
            Usart::send(0); // Send signal
            //Spi::receive(); //read response
            delay();

        // Traiter les données du capteur I2C (par exemple, envoyer via USART)
        Usart::send(sensor_data);

        }
}

fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
