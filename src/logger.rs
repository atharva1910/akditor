use chrono;
use std::io::Write;

pub struct Logger {
}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(&self, message: &str) {
        // Write message to file
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .unwrap();
        writeln!(
            &mut file,
            "{}",
            format!("[{}]: {}", chrono::Local::now(), message)
        )
        .unwrap();
    }
}
