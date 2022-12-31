// use std::process::Command;

pub struct TEMP {
    temp: f32,
}

impl TEMP {
    pub fn new() -> Self {
        TEMP { temp: 0.0 }
    }

    pub fn get_temp(&self) -> f32 {
        self.temp
    }
}