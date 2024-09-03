use std::{
    fs::{self},
    str::FromStr,
};

use gtk4::{
    glib::{self},
    Application, ApplicationWindow, Button,
};
use gtk4::{prelude::*, Box, SearchEntry};
use log::debug;
use log_manager::LogManager;
use sysinfo::{Pid, System};
mod log_manager;
use mini_redis::client;

const APP_ID: &str = "org.cliprs";
const PATH_TO_PID: &str = "/tmp/cliprs.pid";

fn main() -> glib::ExitCode {
    simple_logging::log_to_file("/tmp/cliprs-gtk.debug", log::LevelFilter::Debug).unwrap();
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
    let logs = LogManager::get_logs();
    debug!("logs: {:?}", logs);

    let vbox = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(6)
        .build();

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search history...")
        .build();

    vbox.append(&search_entry);

    for item in logs {
        if item.is_empty() {
            continue;
        }

        let button = Button::builder().label(item).build();

        button.connect_clicked(move |btn| {
            use tokio::runtime::Runtime;

            let rt = Runtime::new().expect("Created tokio Runtime");
            rt.block_on(async {
                let addr = "127.0.0.1:6379";
                if let Ok(mut conn) = client::connect(addr).await {
                    let _ = conn.set(&btn.label().unwrap().to_string(), "".into()).await;
                }
            });
        });

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
