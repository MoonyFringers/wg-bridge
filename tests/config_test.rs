// config_test.rs
// Copyright (c) 2025 Lunatic Fringers
// This file is part of "WG-Bridge" under the AGPL-3.0-or-later license.
// See the LICENSE file in the project root or <https://www.gnu.org/licenses/> for details.

use wgb::core::config::{Config, UserConfig};

#[cfg(test)]
mod tests {
  use super::*;
  use std::env::temp_dir;
  use std::fs;

  fn sample_config() -> Config {
      Config {
          app_name: "MyApp".to_string(),
          version: "0-alpha".to_string(),
          user: vec![UserConfig {
              config_path: "/tmp/file".to_string(),
              otp: false,
              otp_uri: "https://www.google.com".to_string()
          }],
      }
  }

  #[test]
  fn test_serialize_config() {
      let config = sample_config();
      let json = serde_json::to_string_pretty(&config).unwrap();

      assert!(json.contains("\"app_name\": \"MyApp\""));
      assert!(json.contains("\"version\": \"0-alpha\""));
  }

  #[test]
  fn test_deserialize_config() {
      let json = r#"
      {
          "app_name": "MyApp",
          "version": "0.0.0-alpha1",
          "user": [{
              "config_path": "/tmp/file2",
              "otp": true,
              "otp_uri": "https://www.google.it"
          }]
      }
      "#;

      let config: Config = serde_json::from_str(json).unwrap();
      assert_eq!(config.app_name, "MyApp");
      assert_eq!(config.user.get(0).unwrap().config_path, "/tmp/file2");
  }

  #[test]
  fn test_save_and_load_config() {
      let config = sample_config();
      let file_path = temp_dir().join("test_config.json");

      Config::save_config(&config, &file_path.to_string_lossy().into_owned()).unwrap();
      let loaded = Config::load_config(&file_path).unwrap();

      assert_eq!(config, loaded);

      // Clean up
      let _ = fs::remove_file(file_path);
  }
}
