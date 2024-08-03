use std::{
    fs::{self},
    str::FromStr,
};

use gtk4::{glib, Application, ApplicationWindow, Button};
use gtk4::{prelude::*, Box, SearchEntry};
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

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let mut manager = LogManager::new();
    manager.update_logs();

    let vbox = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(6)
        .build();

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search history...")
        .build();

    vbox.append(&search_entry);

    for item in manager.history {
        let button = Button::builder().label(item).build();

        vbox.append(&button);
    }

    let button_container = vbox.clone();

    search_entry.connect_changed(move |entry| {
        let text = entry.text().to_string();

        for child in &button_container.observe_children() {
            if let Some(button) = child.unwrap().downcast_ref::<Button>() {
                if button.label().unwrap().contains(&text) {
                    button.show();
                } else {
                    button.hide();
                }
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cliprs")
        .default_width(360)
        .default_height(600)
        .child(&vbox)
        .build();

    window.present();
}
