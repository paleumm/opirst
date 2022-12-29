pub mod utils;
use utils::led::{Status, Triggering, LED};

pub struct OPI5 {
    led: LED,
}

impl OPI5 {
    pub fn new() -> Self {
        OPI5 { led: LED::new() }
    }

    pub fn led_status(&mut self, status: Status) {
        match status {
            Status::On => self.led.turn_on(),
            Status::Off => self.led.turn_off(),
        }
    }

    pub fn led_triggering(&mut self, triggering: Triggering) {
        match triggering {
            Triggering::HeartBeat => self.led.blink(),
            Triggering::None => self.led.stop_blink(),
        }
    }
}
