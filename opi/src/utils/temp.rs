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

    pub fn get_temp(&self) -> Vec<f32> {
        self.temp
    }

    fn extract_temperatures(input: &str) -> Vec<f32> {
        let mut temperatures = Vec::new();
        for line in input.lines() {
            if line.starts_with("temp1:") {
                let temp_str = line.split(":").nth(1).unwrap();
                let temp: f32 = temp_str.trim().parse().unwrap();
                temperatures.push(temp);
            }
        }
        temperatures
    }
}