#![no_std] 
#![no_main]

use core::panic::PanicInfo;

//on appelle la fonction fn_atm depuis le fichier atmega si le feature utiliser est atmega
#[cfg(feature = "atmega")]
mod atmega;

#[cfg(feature = "atmega")]
use atmega::fn_atm;

//on appelle la fonction fn_esp depuis le fichier esp32 si le feature utiliser est esp32
#[cfg(feature = "esp32")]
mod esp32;

#[cfg(feature = "esp32")]
use esp32::fn_esp;

// To handle the error
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn main()->!{
    #[cfg(feature = "atmega")] 
    fn_atm();
    loop {
    }

    #[cfg(feature = "esp32")]
    fn_esp();
    loop {
    }
}
