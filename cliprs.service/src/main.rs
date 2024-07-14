mod clipboard_managers;

use clipboard_managers::x11_clipboard_manager::X11ClipboardManager;
use fork::{daemon, Fork};

fn main() {
    if let Ok(Fork::Child) = daemon(false, false) {
        X11ClipboardManager::run();
    }
}
