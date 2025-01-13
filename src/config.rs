use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::modbus::common::{ModbusProtocol, ModbusRole};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub protocol: ModbusProtocol,
    pub role: ModbusRole,
    pub tcp_address: Option<String>,  // For TCP protocol
    pub serial_port: Option<String>, // For RTU protocol
    pub baud_rate: Option<u32>,      // For RTU protocol
}

/// Load the configuration from a file or environment
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    // Path to the configuration file
    let config_path = Path::new("config.toml");

    // Read the file content
    let config_content = fs::read_to_string(config_path)?;

    // Parse the content as TOML
    let config: Config = toml::from_str(&config_content)?;

    Ok(config)
}


// 1. Struct for Configuration (Config)
// Defines the settings for the project:
// protocol: Whether to use TCP or RTU (from ModbusProtocol in common.rs).
// role: Master/Client or Slave/Server (from ModbusRole in common.rs).
// tcp_address: TCP address (if using TCP protocol).
// serial_port: Serial port (if using RTU protocol).
// baud_rate: Baud rate (only relevant for RTU protocol).
// 2. load_config Function
// Reads configuration from a config.toml file.
// Parses the content using the toml crate and deserializes it into the Config struct.
// 3. Error Handling
// Any issues with file reading or parsing are returned as Box<dyn Error>.