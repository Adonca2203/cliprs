mod clipboard_managers;

use std::{fs::OpenOptions, io::Write, process::exit};

use clipboard_managers::clipboard_manager::initialize;
use fork::{fork, setsid, Fork};

fn main() {
    match daemonize_and_get_pid() {
        Ok((_, pid)) => {
            let mut pid_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("/tmp/cliprs.pid")
                .unwrap();

            if let Err(err) = pid_file.write(&pid.to_string().as_bytes()) {
                panic!("Could not write pid to file, {}", err);
            }

            let manager = initialize();
            manager.run();
        }
        Err(err) => {
            panic!("Could not fork {}", err);
        }
    }
}

fn daemonize_and_get_pid() -> Result<(Fork, i32), i32> {
    match fork() {
        Ok(Fork::Parent(_)) => exit(0),
        Ok(Fork::Child) => {
            let sid = setsid();
            Ok((fork().unwrap(), sid.unwrap()))
        }
        Err(err) => Err(err),
    }
}
