use serde::Deserialize;

/// Represents the two main Modbus protocols.
#[derive(Debug, Deserialize)]
pub enum ModbusProtocol {
    TCP, // For communication over IP networks.
    RTU, // For communication over serial interfaces.
}

/// Represents the role of a Modbus device.
#[derive(Debug, Deserialize)]
pub enum ModbusRole {
    Client, // Also known as "Master" in RTU terminology.
    Server, // Also known as "Slave" in RTU terminology.
}

// Serde Integration
// The Deserialize derive macro enables these enums to be deserialized from the configuration file (config.toml).

// Usage
// These enums will be used in both the config.rs file for parsing configurations and the tcp.rs/rtu.rs files to determine the mode of operation.
