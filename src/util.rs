use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use serde_json::Value;

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("\x1b[36m[DEBUG] {}\x1b[0m", format!($($arg)*));
        }
    };
}

pub fn get_client_secret_path() -> Result<PathBuf> {
    let config: Value = serde_json::from_str(&fs::read_to_string(crate::CONFIG_FILE)?)
        .context("Failed to read client secret configuration")?;

    Ok(PathBuf::from(config["client_secret_path"].as_str()
        .context("client_secret_path not found in configuration")?))
}