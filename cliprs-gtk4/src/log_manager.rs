use std::{fs, io, time::SystemTime};

const PATH_TO_LOGS: &str = "/tmp/cliprs.log";
const END_BLOCK: &str = "#!block-end";
pub struct LogManager {
    last_edited: Option<SystemTime>,
    pub history: Vec<String>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            last_edited: None,
            history: Vec::new(),
        }
    }

    pub fn update_logs(&mut self) {
        if self.does_log_need_update() {
            self.history = match Self::parse_log() {
                Ok(logs) => logs,
                Err(err) => panic!("{}", err),
            };
        }

        dbg!(&self.history);
    }

    fn does_log_need_update(&mut self) -> bool {
        let last_modified = Self::get_last_modified();
        match self.last_edited {
            None => {
                self.last_edited = Some(last_modified);
                return true;
            }
            Some(time) => {
                if time != last_modified {
                    self.last_edited = Some(last_modified);
                    return true;
                }
            }
        }

        false
    }

    fn get_last_modified() -> SystemTime {
        match fs::metadata(PATH_TO_LOGS) {
            Ok(data) => {
                let most_recent = match data.modified() {
                    Ok(time) => time,
                    Err(err) => panic!("{}", err),
                };
                most_recent
            }
            Err(_) => {
                panic!("Invalid OS for reading modified metadata");
            }
        }
    }

    fn parse_log() -> Result<Vec<String>, io::Error> {
        match fs::read_to_string(PATH_TO_LOGS) {
            Ok(contents) => {
                let resp: Vec<String> = contents
                    .split(END_BLOCK)
                    .map(|s| s.trim().to_string())
                    .collect();

                Ok(resp)
            }
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }
}
