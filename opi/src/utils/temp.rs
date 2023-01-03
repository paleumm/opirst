use std::fmt;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct TEMP {
    gpu_thermal: f32,
    littlecore_thermal: f32,
    bigcore0_thermal: f32,
    bigcore1_thermal: f32,
    npu_thermal: f32,
    center_thermal: f32,
    soc_thermal: f32,
}

impl TEMP {
    pub fn new() -> Self {
        let out = Command::new("sensors").output().unwrap();
        let out = str::from_utf8(&out.stdout).unwrap();
        TEMP::extract_temperatures(&out)
    }

    pub fn extract_temperatures(input: &str) -> Self {
        let mut temps = Vec::new();
        for line in input.lines() {
            if line.starts_with("temp1:") {
                let temp_str = line.split(":").nth(1).unwrap();
                let temp: f32 = temp_str.trim().parse().unwrap();
                temps.push(temp);
            }
        }
        TEMP {
            gpu_thermal: temps[0],
            littlecore_thermal: temps[1],
            bigcore0_thermal: temps[2],
            bigcore1_thermal: temps[3],
            npu_thermal: temps[4],
            center_thermal: temps[5],
            soc_thermal: temps[6],
        }
    }

    pub fn as_vec(&self) -> Vec<f32> {
        [
            self.gpu_thermal,
            self.littlecore_thermal,
            self.bigcore0_thermal,
            self.bigcore1_thermal,
            self.npu_thermal,
            self.center_thermal,
            self.soc_thermal,
        ]
        .to_vec()
    }

    pub fn get_gpu_thermal(&self) -> f32 {
        self.gpu_thermal
    }

    pub fn get_littlecore_thermal(&self) -> f32 {
        self.littlecore_thermal
    }

    pub fn get_bigcore0_thermal(&self) -> f32 {
        self.bigcore0_thermal
    }

    pub fn get_bigcore1_thermal(&self) -> f32 {
        self.bigcore1_thermal
    }

    pub fn get_npu_thermal(&self) -> f32 {
        self.npu_thermal
    }

    pub fn get_center_thermal(&self) -> f32 {
        self.center_thermal
    }

    pub fn get_soc_thermal(&self) -> f32 {
        self.soc_thermal
    }
}

impl fmt::Display for TEMP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "gpu_thermal\t: {}\n
            littlecore_thermal\t: {}\n
            bigcore0_thermal\t: {}\n
            bigcore1_thermal\t: {}\n
            npu_thermal\t: {}\n
            center_thermal\t: {}\n
            soc_thermal\t: {}",
            self.gpu_thermal,
            self.littlecore_thermal,
            self.bigcore0_thermal,
            self.bigcore1_thermal,
            self.npu_thermal,
            self.center_thermal,
            self.soc_thermal
        )
    }
}

impl Into<String> for TEMP {
    fn into(self) -> String {
        format!(
            "gpu_thermal\t: {}\n
            littlecore_thermal\t: {}\n
            bigcore0_thermal\t: {}\n
            bigcore1_thermal\t: {}\n
            npu_thermal\t: {}\n
            center_thermal\t: {}\n
            soc_thermal\t: {}",
            self.gpu_thermal,
            self.littlecore_thermal,
            self.bigcore0_thermal,
            self.bigcore1_thermal,
            self.npu_thermal,
            self.center_thermal,
            self.soc_thermal
        )
    }
}
