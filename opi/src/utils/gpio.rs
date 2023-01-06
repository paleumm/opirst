use std::process::Command;

pub struct GPIO {
    // GND: Pin
}

impl GPIO {
    pub fn new() {
        
    }
    pub fn read_all() -> String {
        let out = Command::new("gpio").args(["read", "all"]).output().unwrap();
        String::from_utf8(out.stdout).unwrap()
    }
}

struct Pin {
    name: String,
    mode: Mode,
    v: u8,
    wpi: WPi,
}

impl Pin {
    fn new() {
        
    }
}

struct Mode;

struct WPi;