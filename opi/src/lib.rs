pub struct OPI5{
    led: LED,
}

impl OPI5 {
    pub fn new() -> Self {
        OPI5 { led: LED::new() }
    }

    pub fn led_status(&mut self, status: Status) {
        match status {
            Status::On => self.led.turn_on(),
            Status::Off => self.led.turn_off()
        }
    }

    pub fn led_triggering(&mut self, triggering: Triggering) {
        match triggering {
            Triggering::HeartBeat => self.led.blink(),
            Triggering::None => self.led.stop_blink()
        }
    }
}

pub struct LED {
    status: Status,
    trigger: Triggering,
}

pub enum Status {
    On,
    Off 
}

pub enum Triggering {
    HeartBeat,
    None
}

impl LED {
    pub fn new() -> Self {
        LED { status: Status::On, trigger: Triggering::HeartBeat }
    }

    pub fn turn_off(&mut self) {
        self.status = Status::Off;
    }

    pub fn turn_on(&mut self) {
        self.status = Status::On;
    }

    pub fn blink(&mut self) {
        self.trigger = Triggering::HeartBeat;
    }

    pub fn stop_blink(&mut self) {
        self.trigger = Triggering::None;
    }
}
