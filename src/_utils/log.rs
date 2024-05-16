use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use chrono::Local;

pub fn print_log(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("{} - {}\n", timestamp, message);
    println!("{}", log_entry);

    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        match create_dir_all(log_dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to create log directory: {}", e);
                return;
            }
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("logs/{}.log", Local::now().format("%Y-%m-%d")))
        .unwrap();

    if let Err(e) = file.write_all(log_entry.as_bytes()) {
        eprintln!("Failed to write to log file: {}", e);
    }
}
