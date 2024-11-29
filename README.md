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

It abstracts the Atmega328p (Arduino Uno) and another target : the Cortex-M7.
The src folder has now multiple modules depending on the target, linked with main.rs.


*The files build.rs, memory.x, openocd.cfg and openocd.gdb were created for the emulation with qemu, they are from https://github.com/rust-embedded/cortex-m-quickstart.


