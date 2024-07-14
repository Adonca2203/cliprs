mod clipboard_managers;

use clipboard_managers::clipboard_manager::initialize;
use fork::{daemon, Fork};

fn main() {
    if let Ok(Fork::Child) = daemon(false, false) {
        let manager = initialize();
        manager.run();
    }
}
