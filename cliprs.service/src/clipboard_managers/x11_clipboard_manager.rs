extern crate clipboard_master;

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::io::Write;
use std::{fs::OpenOptions, io};

const PACKAGE: &str = env!("CARGO_PKG_NAME");

impl ClipboardHandler for X11ClipboardManager {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_contents() {
            Ok(contents) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&self.logs_path)
                    .unwrap();

                let str: String = contents.to_string();
                if let Err(e) = writeln!(file, "{str}") {
                    println!("Error getting clipboard contents {}", e);
                }
            }
            Err(_) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&self.stderr)
                    .unwrap();

                if let Err(e) = writeln!(file, "Error getting clipboard contents") {
                    println!("Error getting clipboard contents {}", e);
                }
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprint!("Error: {:?}", error);
        CallbackResult::Next
    }
}

pub struct X11ClipboardManager {
    ctx: ClipboardContext,
    logs_path: String,
    stderr: String,
}

impl X11ClipboardManager {
    fn new() -> Self {
        match ClipboardContext::new() {
            Ok(ctx) => {
                let stdout = format!("/tmp/{}.log", PACKAGE);
                let stderr = format!("/tmp/{}.err", PACKAGE);
                Self {
                    ctx,
                    logs_path: stdout,
                    stderr,
                }
            }
            Err(err) => panic!("Unable to start context {err}"),
        }
    }

    pub fn run() {
        let mut master = Master::new(X11ClipboardManager::new());

        master.run().expect("Success");
    }
}
