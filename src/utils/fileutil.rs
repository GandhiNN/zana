use std::fmt;
use std::path::PathBuf;
use std::time;

#[derive(Default)]
pub struct FileAge {
    pub age_duration: time::Duration,
    pub seconds: u64,
    pub hours: u64,
    pub minutes: u64,
}

impl fmt::Display for FileAge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }
}

pub fn get_file_age(file: PathBuf) -> FileAge {
    let metadata = std::fs::metadata(file).unwrap();
    let t = metadata.modified().unwrap().elapsed().unwrap();
    let age = t.as_secs();
    let hours = age / 3600;
    let minutes = age % 3600 / 60;
    let seconds = age % 3600 % 60;
    FileAge {
        age_duration: t,
        seconds,
        hours,
        minutes,
    }
}
