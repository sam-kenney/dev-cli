//! Config for the search command.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The config for the search command.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub search_engine_id: String,
}

/// Implement the `Config` struct.
impl Config {
    /// Load the config from the config file.
    pub async fn load_from_file() -> Self {
        let path = Config::dir();

        Config::create_file().await;
        let contents = tokio::fs::read_to_string(path)
            .await
            .expect("Failed to read file");
        let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse TOML");
        config
    }

    /// Get the path to the config file.
    pub fn dir() -> PathBuf {
        let path = dirs::home_dir().unwrap();
        path.join(".dev-config/config.yaml")
    }

    /// Create the config file if it doesn't exist.
    async fn create_file() {
        // Get root directory
        let path = Config::dir();

        if path.exists() {
            return;
        }

        println!("Creating config file: {:?}", path);

        if !path.exists() {
            tokio::fs::create_dir_all(path.parent().unwrap())
                .await
                .unwrap_or_else(|e| panic!("Failed to create directory: {}", e))
        }

        tokio::fs::write(path, "api_key: \"\"\nsearch_engine_id: \"\"")
            .await
            .expect("Failed to write to file");
    }
}
