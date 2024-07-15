use std::{
    fs::{self},
    str::FromStr,
    thread,
};

use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button};
use log_manager::LogManager;
use rdev::{listen, Event, EventType, Key};
use sysinfo::{Pid, System};
mod log_manager;

const APP_ID: &str = "org.cliprs";
const PATH_TO_PID: &str = "/tmp/cliprs.pid";

fn main() -> glib::ExitCode {
    let pid = fs::read_to_string(PATH_TO_PID).unwrap();

    let sys = System::new_all();
    let p = Pid::from_str(&pid);

    if sys.process(p.unwrap()).unwrap().name() != "cliprs" {
        panic!("cliprs service is not running");
    }

    let app = Application::builder().application_id(APP_ID).build();
    let mut manager = LogManager::new();
    manager.update_logs();

    app.connect_activate(build_ui);

    thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });
    app.run()
}

fn callback(event: Event) {
    if EventType::KeyPress(Key::MetaLeft) == event.event_type {
        println!("Pressed left meta");
    }
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| button.set_label("Hello World!"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cliprs")
        .default_width(360)
        .default_height(600)
        .child(&button)
        .build();

    window.present();
}
