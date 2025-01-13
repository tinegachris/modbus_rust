use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ModbusProtocol {
    TCP,
    RTU,
}

#[derive(Debug, Deserialize)]
pub enum ModbusRole {
    Client, // Also represents Master
    Server, // Also represents Slave
}


// Explanation
// 1. ModbusProtocol Enum
// Represents the two main Modbus protocols:
// TCP: For communication over IP networks.
// RTU: For communication over serial interfaces.
// 2. ModbusRole Enum
// Represents the role of a Modbus device:
// Client: Also known as a "Master" in RTU terminology.
// Server: Also known as a "Slave" in RTU terminology.
// 3. Serde Integration
// The Deserialize derive macro enables these enums to be deserialized from the configuration file (config.toml).
// 4. Usage
// These enums will be used in both the config.rs file for parsing configurations and the tcp.rs/rtu.rs files to determine the mode of operation.