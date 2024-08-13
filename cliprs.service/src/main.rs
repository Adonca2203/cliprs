mod clipboard_managers;
mod server;

use std::{fs::OpenOptions, io::Write, process::exit};

use fork::{fork, setsid, Fork};
use server::tokio_server::ApplicationServer;

fn main() {
    match daemonize_and_get_pid() {
        Ok(pid) => {
            let mut pid_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("/tmp/cliprs.pid")
                .unwrap();

            if let Err(err) = pid_file.write(&pid.to_string().as_bytes()) {
                panic!("Could not write pid to file, {}", err);
            }
            tokio_main();
        }
        Err(err) => {
            panic!("Could not fork {}", err);
        }
    }
}

fn daemonize_and_get_pid() -> Result<i32, i32> {
    match fork() {
        Ok(Fork::Parent(_)) => exit(0),
        Ok(Fork::Child) => {
            let sid = setsid();
            Ok(sid.unwrap())
        }
        Err(err) => Err(err),
    }
}

#[tokio::main]
async fn tokio_main() {
    let server = ApplicationServer::new();

    let _ = server.run().await;
}
