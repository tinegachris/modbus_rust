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
    pub serial_port: Option<String>,  // For RTU protocol
    pub baud_rate: Option<u32>,       // For RTU protocol
}

/// Load the configuration from a file or environment
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new("./config.toml");
    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            return Err(Box::new(e));
        }
    };
    let config: Config = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => return Err(Box::new(e)),
    };
    Ok(config)
}
