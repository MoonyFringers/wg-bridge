// logger_test.rs
// Copyright (c) 2025 Lunatic Fringers
// This file is part of "WG-Bridge" under the AGPL-3.0-or-later license.
// See the LICENSE file in the project root or <https://www.gnu.org/licenses/> for details.


use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::Duration;

use wgb::core::logger::Logger;

#[test]
fn test_logger_write_and_read() {
    let log_path = "/tmp/test_logger.log";

    // Clean up old test file
    let _ = fs::remove_file(log_path);

    Logger::init(log_path);
    let logger = Logger::get();

    logger.debug("Debug message");
    logger.info("Info message");
    logger.warn("Warn message");
    logger.error("Error message");

    // Wait to ensure background thread writes to file
    thread::sleep(Duration::from_millis(100));

    // Check if file exists
    assert!(Path::new(log_path).exists());

    // Read file contents
    let mut file = File::open(log_path).expect("Failed to open log file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read log file");

    assert!(content.contains("DEBUG"));
    assert!(content.contains("INFO"));
    assert!(content.contains("WARN"));
    assert!(content.contains("ERROR"));

    // Clean up
    let _ = fs::remove_file(log_path);
}
