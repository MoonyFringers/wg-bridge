// logger.rs
// Copyright (c) 2025 Lunatic Fringers
// This file is part of "WG-Bridge" under the AGPL-3.0-or-later license.
// See the LICENSE file in the project root or <https://www.gnu.org/licenses/> for details.

use chrono::Local;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};

/// Define a struct to be used for multithreaded writing to a log file.
#[derive(Clone, Debug)]
pub struct Logger {
  sender: Sender<String>,
}

/// Define a variable to enable the Singleton pattern.
static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Implements the logic to write the log file
#[allow(dead_code)]
impl Logger {
  /// Function to initialize the Logger by creating a new thread used for
  /// writing to the file and setting the LOGGER singleton variable.
  ///
  /// This function creates a background logging thread that listens for messages
  /// sent via a channel. It appends the messages to the specified log file.
  /// If the logger has not been initialized, it will panic with "Logger already initialized".
  ///
  /// # Arguments
  /// * `log_file`: The path to the log file where log messages will be written.
  pub fn init(log_file: &str) {
    // Create a channel to send logs to the logging thread
    let (tx, rx) = mpsc::channel::<String>();
    let log_file = log_file.to_string();

    // Spawn a background logging thread
    std::thread::spawn(move || {
      let mut file = OpenOptions::new()
          .create(true)
          .append(true)
          .open(&log_file)
          .expect("Failed to open log file");

      for message in rx {
        if let Err(e) = writeln!(file, "{message}") {
          eprintln!("Failed to write log: {e}");
        }
        let _ = file.flush();
      }
    });

    let logger = Logger { sender: tx };
    LOGGER.set(logger).expect("Logger already initialized");
  }

  /// Function to send log messages to the background thread.
  ///
  /// This method formats the log message with a timestamp and log level.
  /// The formatted message is then sent to the background thread for writing to the log file.
  ///
  /// # Arguments
  /// * `level`: The log level (e.g., "DEBUG", "INFO", "WARN", "ERROR").
  /// * `message`: The log message to be logged.
  fn log(&self, level: &str, message: &str) {
    // Format timestamp with milliseconds
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    // The timestamp and level are left-aligned with 20 and 8 padding spaces,
    // respectively.
    let log_message = format!("{timestamp:<20} - {level:<8}  {message}");
    let _ = self.sender.send(log_message);
  }

  /// Sends a log message and an associated error to the background thread.
  ///
  /// This method constructs two log entries:
  /// - One for the general message.
  /// - One for the string representation of the error.
  ///
  /// Both entries are formatted with the same timestamp and log level.
  ///
  /// # Arguments
  /// * `level`: The severity level of the log message.
  /// * `message`: The custom error context or description.
  /// * `error`: The error object implementing the `Error` trait.
  fn log_error<T:Error>(&self, level: &str, message: &str, error: &T){
    // Format timestamp with milliseconds
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    // The timestamp and level are left-aligned with 20 and 8 padding spaces,
    // respectively.
    let log_message = format!("{timestamp:<20} - {level:<8}  {message}");
    let log_error = format!("{timestamp:<20} - {level:<8}  {error}");
    let _ = self.sender.send(log_message);
    let _ = self.sender.send(log_error);
  }

  /// Function to write debug messages (only in non-release versions).
  ///
  /// This method writes messages with the "DEBUG" log level.
  /// It is only compiled in non-release (debug) builds.
  ///
  /// # Arguments
  /// * `message`: The debug message to be logged.
  pub fn debug(&self, message: &str) {
    self.log("DEBUG", message);
  }

  /// Logs a debug-level message with an associated error.
  ///
  /// This method is useful during development to trace debug-level issues
  /// with error context.
  ///
  /// # Arguments
  /// * `message`: A debug message describing the context.
  /// * `error`: An error object implementing the `Error` trait.
  pub fn debug_error<T:Error>(&self, message: &str, error: &T) {
    self.log_error("DEBUG", message, error);
  }

  /// Function to write info messages.
  ///
  /// This method writes messages with the "INFO" log level.
  ///
  /// # Arguments
  /// * `message`: The info message to be logged.
  pub fn info(&self, message: &str) {
    self.log("INFO", message);
  }

  /// Logs a info-level message with an associated error.
  ///
  /// This method logs a warning message along with the error details,
  /// using the `INFO` severity level.
  ///
  /// # Arguments
  /// * `message`: A info message describing the issue.
  /// * `error`: An error object implementing the `Error` trait.
  pub fn info_error<T:Error>(&self, message: &str, error: &T) {
    self.log_error("INFO", message, error);
  }

  /// Function to write warning messages.
  ///
  /// This method writes messages with the "WARN" log level.
  ///
  /// # Arguments
  /// * `message`: The warning message to be logged.
  pub fn warn(&self, message: &str) {
    self.log("WARN", message);
  }

  /// Logs a warning-level message with an associated error.
  ///
  /// This method logs a warning message along with the error details,
  /// using the `WARN` severity level.
  ///
  /// # Arguments
  /// * `message`: A warning message describing the issue.
  /// * `error`: An error object implementing the `Error` trait.
  pub fn warn_error<T:Error>(&self, message: &str, error: &T) {
    self.log_error("WARN", message, error);
  }

  /// Function to write error messages.
  ///
  /// This method writes messages with the "ERROR" log level.
  ///
  /// # Arguments
  /// * `message`: The error message to be logged.
  pub fn error(&self, message: &str) {
    self.log("ERROR", message);
  }

  /// Logs an error-level message with an associated error.
  ///
  /// This method logs both a custom error message and the actual error object,
  /// making it easier to trace issues in logs.
  ///
  /// # Arguments
  /// * `message`: A descriptive error message.
  /// * `error`: An error object implementing the `Error` trait.
  pub fn error_error<T:Error>(&self, message: &str, error: &T) {
    self.log_error("ERROR", message, error);
  }

  /// Retrieves a reference to the initialized `Logger` instance.
  ///
  /// This function ensures that the `Logger` is only initialized once using `OnceLock`.
  /// If the `Logger` has already been initialized, it returns a reference to the singleton instance.
  /// If the `Logger` has not been initialized, it panics with the message "Logger not initialized".
  ///
  /// # Returns
  /// * `&'static Logger`: A reference to the singleton `Logger` instance, which lives for the duration of the program.
  pub fn get() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
  }
}
