use cursive::views::{Dialog};
use cursive::Cursive;
use opi::*;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(
        Dialog::text("Welcome to OPiRST\n   LED Status")
            .title("OPiRST")
            .button("Settings", show_next),
    );
    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("LED Status")
            .title("Question 1")
            .button("Turn On", |s| show_answer(s, "LED Turned On", Status::On, Triggering::None))
            .button("Turn Off", |s| show_answer(s, "LED Turned Off", Status::Off, Triggering::None))
            .button("Blinking", |s| show_answer(s, "LED Blinking", Status::On, Triggering::HeartBeat)),
    );
}

fn show_answer(s: &mut Cursive, msg: &str, status: Status, triggering: Triggering) {

    let mut opi = OPI5::new();
    opi.led_status(status);
    opi.led_triggering(triggering);
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
}
