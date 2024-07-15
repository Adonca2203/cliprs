use std::{
    fs::{self},
    str::FromStr,
};

use sysinfo::{Pid, System};

fn main() {
    let pid = fs::read_to_string("/tmp/cliprs.pid").unwrap();

    let sys = System::new_all();
    let p = Pid::from_str(&pid);

    if sys.process(p.unwrap()).unwrap().name() != "cliprs" {
        panic!("cliprs service is not running");
    }
}
