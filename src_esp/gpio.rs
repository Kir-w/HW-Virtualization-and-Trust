use rp_pico::hal::gpio::{Pin, PushPullOutput};

pub struct Gpio {
    led: Pin<PushPullOutput>,
}

impl Gpio {
    pub fn new(led: Pin<PushPullOutput>) -> Self {
        Self { led }
    }

    pub fn blink_led(&mut self) {
        self.led.set_high().unwrap();
        cortex_m::asm::delay(500_000);
        self.led.set_low().unwrap();
        cortex_m::asm::delay(500_000);
    }
}

