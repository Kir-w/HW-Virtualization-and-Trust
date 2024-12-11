use rp_pico::hal::gpio::{Pin, Function, PushPullOutput};

pub struct Gpio {
    led: Pin<Function<PushPullOutput>>,
}

impl Gpio {
    pub fn new(mut pins: rp_pico::hal::gpio::Pins) -> Self {
        Self {
            led: pins.gpio25.into_push_pull_output(),
        }
    }

    pub fn blink_led(&mut self) {
        self.led.set_high().unwrap();
        cortex_m::asm::delay(500_000);
        self.led.set_low().unwrap();
        cortex_m::asm::delay(500_000);
    }
}
