use clipboard_manager::ClipboardManager;

mod clipboard_manager;

fn main() {
    let manager = ClipboardManager::new();

    manager.run();
}
