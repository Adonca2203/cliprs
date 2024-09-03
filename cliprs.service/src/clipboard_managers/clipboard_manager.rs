use std::{collections::HashSet, fmt::Debug};

use clipboard_master::ClipboardHandler;
use log::debug;
use mini_redis::Frame;

use super::x11_clipboard_manager::X11ClipboardManager;

#[derive(Clone, Debug)]
pub struct VectorWrapper<String>(Vec<String>);

impl VectorWrapper<String> {
    pub fn new() -> Self {
        VectorWrapper(Vec::<String>::new())
    }

    pub fn insert(&mut self, value: String) {
        self.0.push(value);
    }

    pub fn remove(&mut self, value: &String) {
        let index = self.0.iter().position(|x| x == value).unwrap();
        self.0.remove(index);
    }

    pub fn contains(&self, value: &String) -> bool {
        self.0.contains(value)
    }

    pub fn to_comma_separated(&self) -> String {
        let reversed = self.0.iter().rev();
        reversed.map(|x| x.to_string()).collect::<Vec<_>>().join(",")
    }
}

impl Into<Vec<Frame>> for VectorWrapper<String> {
    fn into(self) -> Vec<Frame> {
        let mut ret_val = Vec::<Frame>::new();

        for value in self.0 {
            debug!("Val: {:?}", value);
            if value.trim().is_empty() {
                continue;
            }
            ret_val.push(Frame::Simple(value.trim().to_string()));
        }
        debug!("Ret Val: {:?}", ret_val);

        ret_val
    }
}

pub trait ClipboardManager: ClipboardHandler + Send + Sync {
    fn run(&self);
}

pub fn initialize() -> Box<dyn ClipboardManager> {
    if cfg!(all(
        unix,
        not(any(target_os = "macos", target_os = "android"))
    )) {
        return Box::new(X11ClipboardManager::new());
    }

    panic!("OS not available");
}
