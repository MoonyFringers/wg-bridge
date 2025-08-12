// config.rs
// Copyright (c) 2025 Lunatic Fringers
// This file is part of "WG-Bridge" under the AGPL-3.0-or-later license.
// See the LICENSE file in the project root or <https://www.gnu.org/licenses/> for details.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use dirs::home_dir;

use crate::core::logger::Logger;

/// Global, thread-safe static storage for the application configuration.
/// Initialized once using `Config::init`.
static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

/// Default filename used to store the configuration in the user's home directory.
const FILENAME: &str = ".wgbconf.json";

/// Represents the full application configuration.
///
/// Holds general application metadata and user-specific settings.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Name of the application.
    pub app_name: String,

    /// Version string of the application.
    pub version: String,

    /// List of user-specific configuration entries.
    pub user: Vec<UserConfig>,
}

/// Represents a single user's configuration settings.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    /// Path to the user's VPN or WireGuard configuration file.
    pub config_path: String,

    /// Whether OTP (One-Time Password) is required for this user.
    pub otp: bool,

    /// The OTP URI (typically for QR-code or 2FA setup).
    pub otp_uri: String,
}

impl Config {
    /// Initializes the global configuration.
    ///
    /// This method should be called once at startup. It attempts to load an existing
    /// configuration file from the user's home directory. If it doesn't exist, it
    /// creates a default configuration and saves it.
    ///
    /// # Panics
    ///
    /// Panics if the configuration has already been initialized or if the
    /// internal mutex is poisoned.
    pub fn init() {
        let log = Logger::get();
        let config;

        let config_path: PathBuf = match home_dir() {
            Some(mut path) => {
                path.push(FILENAME);
                path
            }
            None => {
                eprintln!("Could not determine the config file path.");
                return;
            }
        };

        if !config_path.exists() {
            config = Config {
                app_name: "WGBridge".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                user: vec![],
            };
            if let Err(_e) = Config::save_config(&config, &config_path.to_string_lossy()) {
                log.error("Problem saving the config file");
            }
        } else {
            let load_conf = Config::load_config(&config_path);
            if load_conf.is_err() {
                log.error("Failed to read the configuration");
            }
            config = load_conf.unwrap();
        }

        CONFIG.set(Mutex::new(config)).expect("Configuration already initialized");
    }

    /// Loads the configuration from the given file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Config` if successful, or an error if the file
    /// cannot be read or parsed.
    pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Saves the given configuration to a file.
    ///
    /// # Arguments
    ///
    /// * `config` - The `Config` object to save.
    /// * `path` - The file path (as a string) to save the configuration to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn save_config(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(config)?;
        fs::write(path, json_string)?;
        Ok(())
    }

    /// Returns a mutable reference to the global configuration.
    ///
    /// Internally locks the mutex around the `Config`. This allows safe, mutable access
    /// across threads.
    ///
    /// # Panics
    ///
    /// Panics if the configuration has not been initialized or if the mutex is poisoned.
    pub fn get() -> std::sync::MutexGuard<'static, Config> {
        CONFIG.get().expect("Configuration not initialized").lock().expect("Poisoned mutex")
    }
}
