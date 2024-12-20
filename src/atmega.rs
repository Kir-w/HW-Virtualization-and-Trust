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
        gpio.digital_write(5, true); // Allume la LED
        Usart::send(1); // Envoie le signal via usart
        //Spi::send(0x11); // µEnvoie la donnée via spi
        let sensor_data = I2cInterface::read_sensor(0x40, 0x00); // Lit le capteur via I2C

        delay();

        gpio.digital_write(5, false); // Eteint la LED
        Usart::send(0); // Envoie le signal
        //Spi::receive(); // Lit la réponse
        delay();

        // Traiter les données du capteur I2C
        Usart::send(sensor_data);

        }
}

fn delay() {
    for _ in 0..400000 {
        unsafe { core::ptr::read_volatile(&0) };
    }
}
