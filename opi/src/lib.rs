pub mod utils;
use utils::led::{Status, Triggering, LED};
use utils::temp::TEMP;
use utils::gpio::GPIO;

pub struct OPI5 {
    led: LED,
    pub temp: TEMP,
}

impl OPI5 {
    pub fn new() -> Self {
        OPI5 {
            led: LED::new(),
            temp: TEMP::new(),
        }
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

    pub fn get_temp(&self) -> Vec<f32> {
        self.temp.as_vec()
    }

    pub fn get_gpio() -> String {
        GPIO::read_all()
    }
}
