use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub struct LED {
    status: Status,
    trigger: Triggering,
}

pub enum Status {
    On,
    Off,
}

pub enum Triggering {
    HeartBeat,
    None,
}

impl LED {
    pub fn new() -> Self {
        LED {
            status: Status::On,
            trigger: Triggering::HeartBeat,
        }
    }

    pub fn turn_off(&mut self) {
        self.status = Status::Off;
        echo("0", &Path::new("/sys/class/leds/status_led/brightness")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    pub fn turn_on(&mut self) {
        self.status = Status::On;
        echo("1", &Path::new("/sys/class/leds/status_led/brightness")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    pub fn blink(&mut self) {
        self.trigger = Triggering::HeartBeat;
        echo(
            "heartbeat",
            &Path::new("/sys/class/leds/status_led/trigger"),
        )
        .unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    pub fn stop_blink(&mut self) {
        self.trigger = Triggering::None;
        echo("none", &Path::new("/sys/class/leds/status_led/trigger")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}
