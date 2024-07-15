extern crate clipboard_master;

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::io::Write;
use std::{fs::OpenOptions, io};

use super::clipboard_manager::ClipboardManager;

const PACKAGE: &str = env!("CARGO_PKG_NAME");

pub struct X11ClipboardManager {
    ctx: ClipboardContext,
    logs_path: String,
    stderr: String,
}

impl ClipboardHandler for X11ClipboardManager {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_contents() {
            Ok(contents) => {
                if contents.trim().is_empty() {
                    return CallbackResult::Next;
                }
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&self.logs_path)
                    .unwrap();

                let msg = format!(
                    "{}\n#!block-end\n",
                    contents.trim().to_string()
                );

                let _ = file.write_all(msg.as_bytes());
            }
            Err(err) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&self.stderr)
                    .unwrap();

                let msg = format!("Error getting clipboard contents: {}", err.to_string());
                let _ = file.write_all(msg.as_bytes());
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.stderr)
            .unwrap();

        let msg = format!("Error getting clipboard contents: {}", error.to_string());
        let _ = file.write_all(msg.as_bytes());
        CallbackResult::Next
    }
}

impl ClipboardManager for X11ClipboardManager {
    fn run(&self) {
        let mut master = Master::new(Self::new());

        master.run().expect("Success");
    }
}

impl X11ClipboardManager {
    pub fn new() -> Self {
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
}
