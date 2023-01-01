use cursive::views::Dialog;
use cursive::Cursive;
use opi::utils::led;
use opi::OPI5;

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
            .title("LED Settings")
            .button("Turn On", |s| {
                show_answer(s, "LED Turned On", led::Status::On, led::Triggering::None)
            })
            .button("Blinking", |s| {
                show_answer(
                    s,
                    "LED Blinking",
                    led::Status::On,
                    led::Triggering::HeartBeat,
                )
            })
            .button("Turn Off", |s| {
                show_answer(s, "LED Turned Off", led::Status::Off, led::Triggering::None)
            })
            .button("Temp", |s| show_temp(s)),
    );
}

fn show_answer(s: &mut Cursive, msg: &str, status: led::Status, triggering: led::Triggering) {
    let mut opi = OPI5::new();
    opi.led_triggering(triggering);
    opi.led_status(status);
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
}

fn show_temp(s: &mut Cursive) {
    let opi = OPI5::new();
    s.pop_layer();
    s.add_layer(Dialog::text("text"));
}
