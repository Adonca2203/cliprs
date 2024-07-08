extern crate clipboard_master;

use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::io;

impl ClipboardHandler for ClipboardManager {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_contents() {
            Ok(contents) => {
                println!("{contents}");
                self.log_clipboard(contents);
            }
            Err(_) => println!("Error getting clipboard contents"),
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprint!("Error: {:?}", error);
        CallbackResult::Next
    }
}

pub struct ClipboardManager {
    ctx: ClipboardContext,
    log: Vec<String>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        match ClipboardContext::new() {
            Ok(ctx) => Self {
                ctx,
                log: Vec::new(),
            },
            Err(err) => panic!("Unable to start context {err}"),
        }
    }

    pub fn run(&self) {
        let mut master = Master::new(ClipboardManager::new());

        master.run().expect("Success");
    }

    fn log_clipboard(&mut self, content: String) {
        self.log.push(content);
    }

    pub fn show_log(&self) -> &Vec<String> {
        &self.log
    }
}
