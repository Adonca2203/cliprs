extern crate clipboard_master;

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use log::error;
use std::io;

use crate::server::tokio_server::DATA;

use super::clipboard_manager::ClipboardManager;

pub struct X11ClipboardManager {
    ctx: ClipboardContext,
}

impl ClipboardHandler for X11ClipboardManager {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_contents() {
            Ok(contents) => {
                if contents.trim().is_empty() {
                    return CallbackResult::Next;
                }
                let mut lock = DATA.lock().unwrap();

                if lock.contains(&contents) {
                    lock.remove(&contents);
                }

                lock.insert(contents);

                drop(lock);
            }
            Err(err) => {
                error!("Error getting clipboard contents: {:?}", err);
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        error!("Error getting clipboard contents: {:?}", error);
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
            Ok(ctx) => Self { ctx },
            Err(err) => panic!("Unable to start context {err}"),
        }
    }
}
