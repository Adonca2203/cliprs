use clipboard_master::ClipboardHandler;

use super::x11_clipboard_manager::X11ClipboardManager;

pub trait ClipboardManager: ClipboardHandler + Send + Sync {
    fn run(&self);
    fn get_history(&self) -> Vec<String>;
}

pub fn initialize() -> Box<dyn ClipboardManager> {
    if cfg!(all(unix, not(any(target_os="macos", target_os="android")))) {
        return Box::new(X11ClipboardManager::new());
    }

    panic!("OS not available");
}
