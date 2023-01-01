use std::process::Command;

pub struct TEMP {
    gpu_thermal : f32,
    littlecore_thermal : f32,
    bigcore0_thermal : f32,
    bigcore1_thermal : f32,
    npu_thermal : f32,
    center_thermal : f32,
    soc_thermal : f32,
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