// main.rs
// Copyright (c) 2025 Lunatic Fringers
// This file is part of "WG-Bridge" under the AGPL-3.0-or-later license.
// See the LICENSE file in the project root or <https://www.gnu.org/licenses/> for details.


pub mod cli;
pub mod core;
pub mod ui;

use core::logger::Logger;
use core::config::{Config};
use chrono::Local;



fn main() {
  // // Initializing logger
  let date = Local::now().format("%Y-%m-%d").to_string();
  let log_path = &format!("./{date}.log");
  Logger::init(log_path);
  let _log = Logger::get();

  // Open config
  Config::init();
  let config = Config::get();
  println!("{}", config.app_name);
  println!("{}", config.version);

  let userconfs = config.user.iter().clone();

  for usr in userconfs{
    println!("{}", usr.config_path);
    println!("{}", usr.otp);
    println!("{}", usr.otp_uri);
  }
}
