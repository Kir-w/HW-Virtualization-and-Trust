# HW-Virtualization-and-trust

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

The Universal Synchronous/Asynchronous Receiver/Transmitter feature is a flexible serial communication interface. It abstracts the Atmega328p (Arduino Uno) and another target : the Cortex-M7.
The src folder has now multiple modules depending on the target, linked with main.rs.

**Third feature : the SPI feature**

The Serial Peripheral Interface feature is a communication mode supported by USART, for both of the targets here. It's a synchronous serial communication protocol that allows for fast data exchange.

*The files build.rs, memory.x, openocd.cfg and openocd.gdb were created for the emulation with qemu, they are from https://github.com/rust-embedded/cortex-m-quickstart.


