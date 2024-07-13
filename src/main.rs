mod clipboard_managers;

use clipboard_managers::x11_clipboard_manager::X11ClipboardManager;
use fork::{daemon, Fork};

fn main() {
    let x11 = X11ClipboardManager::new();

    if let Ok(Fork::Child) = daemon(false, false) {
        x11.run();
    }
}
