use std::{
    fs::{self},
    str::FromStr,
};

use gtk4::{
    gdk::{Key, ModifierType},
    glib::Propagation,
    prelude::*,
    EventControllerKey,
};
use gtk4::{glib, Application, ApplicationWindow, Button};
use log_manager::LogManager;
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

    app.run()
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

    let event_controller = EventControllerKey::new();
    event_controller.connect_key_pressed(move |_, key, _, modifier| {
        match key {
            Key::v | Key::V => {
                if modifier == ModifierType::META_MASK {
                    println!("Pressed Meta V");
                }

                println!("{:?} -- {:?}", key, modifier);
            }
            _ => (),
        }
        Propagation::Proceed
    });

    window.present();
    window.add_controller(event_controller);
}
