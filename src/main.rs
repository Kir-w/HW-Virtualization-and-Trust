#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Sélection des modules selon la feature acctivée
// Remettre le default features dans le cargo toml pour lancer directement l'atmega
// Sinon faire la commande suivante pour lancer l'atmega:
// cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release --features atmega
// ou la commande suivante pour lancer l'esp32
// cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release --features esp32
#[cfg(feature = "atmega")]
mod atmega;
#[cfg(feature = "atmega")]
use atmega::fn_atm;

#[cfg(feature = "esp32")]
mod esp32;
#[cfg(feature = "esp32")]
use esp32::fn_esp;

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Fonction principale
#[no_mangle]
fn main() -> ! {
    // ctf_if permet de sélectionner une feature directement au moment de la compilation
    cfg_if::cfg_if! {
        if #[cfg(feature = "atmega")] {
            fn_atm();
        } else if #[cfg(feature = "esp32")] {
            fn_esp();
        } else {
            compile_error!("Aucune feature valide n'est activée : activez 'atmega' ou 'esp32'.");
        }
    }
}
