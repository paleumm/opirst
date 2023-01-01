use std::process::Command;

pub struct TEMP {
    temp: f32,
}

impl TEMP {
    pub fn new() -> Self {
        let out = Command::new("sensors").output().unwrap();
        println!("printing:\n{}", String::from_utf8(out.stdout).unwrap());
        TEMP { temp: 0.0 }
    }

    pub fn get_temp(&self) -> f32 {
        self.temp
    }
}