# HW-Virtualization-and-Trust

### Hardware Abstract Layer project in Rust

### Group with Ariane ODONI, Maïa ROCA, Kylie WU (OCC3)

**First feature (October) : the GPIO feature**  

It abstracts the Atmega328p (Arduino Uno).  
The code should allow to control all the digital pin of the target, as input or output. More precisely, it can configure any digital pin as input, or as output, and read and write all digital pins for now.

[CORRECTION GPIO] (Don't hesitate to remove this part)
Consider subdividing your project into separate modules. 
You can only use the I/O registers of port B here (and not the C port for example).
It would be nice to have something to prevent modifying the register in an incoherent way. For example, if I do ``` gpio.pin_mode(40, PinMode::Output);```, it won't bug during the compilation, but it may generate a problem on your hardware.

**Second feature (November) : the USART feature**

The Universal Synchronous/Asynchronous Receiver/Transmitter feature is a flexible serial communication interface. It allows for both asynchronous UART and synchronous SPI communication. It abstracts the Atmega328p (Arduino Uno) and another target : the ARM Cortex-M.
The src folder has now multiple modules depending on the target, linked with main.rs.

[CORRECTION USART] (Don't hesitate to remove this part)
You mix two different architectures, so it cannot work correctly
You didn't really implement the USART feature for the second target

**Third feature : the SPI feature**

The Serial Peripheral Interface feature is a communication mode supported by USART, for both of the targets here. It's a synchronous serial communication protocol that allows for fast data exchange.

[CORRECTION SPI] (don't hesitate to remove that part)
You could implement the peripheral/slave mode as well.
You are using some external HAL (rp_pico, embedded_hal) for a part of your SPI feature.
About the other external HAL avr-device (in your Cargo.toml file), even if you are not using it, I recommand you to remove it from your dependencies, it could generate errors.
You could abstract more the register content, for example 0x2C is not very explicit, you may want to customize your parameters more accurately (therefore you could use more freely CPOL, CPHA, BR, MSTR...).

**Fourth feature (December) : I2C feature**

The Inter-Integrated Circuit feature is a serial communication protocol for many applications. Data are sent or received at one time (half-duplex protocol), and it supports many devices at the same time. It also supports many masters and slaves but there is a slower data transfer rate than SPI.

## Comments
The files build.rs, memory.x, openocd.cfg and openocd.gdb were created for the emulation with QEMU, they are from https://github.com/rust-embedded/cortex-m-quickstart.

The report is available in .pdf, or here : https://docs.google.com/document/d/1T9L17Sxoja9nH0tQfcbXIxWXTTl7WOaOAidKeFSuELI/edit?usp=sharing
