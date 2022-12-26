use opi::*;
fn main() {
    let mut opi = OPI5::new();
    opi.led_status(Status::On);
    opi.led_triggering(Triggering::HeartBeat);
}