mod modbus;
mod config;

use modbus::common::{ModbusProtocol, ModbusRole};
use config::load_config;
use modbus::{tcp, rtu};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = load_config()?;

    // Handle based on protocol and role
    match config.protocol {
        ModbusProtocol::TCP => match config.role {
            ModbusRole::Client => {
                if let Some(address) = config.tcp_address {
                    tcp::start_client(&address).await?;
                } else {
                    eprintln!("TCP Client requires an address.");
                }
            }
            ModbusRole::Server => {
                if let Some(address) = config.tcp_address {
                    tcp::start_server(&address).await?;
                } else {
                    eprintln!("TCP Server requires an address.");
                }
            }
        },
        ModbusProtocol::RTU => match config.role {
            ModbusRole::Master => {
                if let Some(port) = config.serial_port {
                    if let Some(baud_rate) = config.baud_rate {
                        rtu::start_master(&port, baud_rate).await?;
                    } else {
                        eprintln!("RTU Master requires a baud rate.");
                    }
                } else {
                    eprintln!("RTU Master requires a serial port.");
                }
            }
            ModbusRole::Slave => {
                if let Some(port) = config.serial_port {
                    if let Some(baud_rate) = config.baud_rate {
                        rtu::start_slave(&port, baud_rate).await?;
                    } else {
                        eprintln!("RTU Slave requires a baud rate.");
                    }
                } else {
                    eprintln!("RTU Slave requires a serial port.");
                }
            }
        },
    }

    Ok(())
}

// Configuration Loading:

// The load_config function fetches user-defined configurations for protocol, role, and connection details.
// Protocol and Role Handling:

// The match statement handles the combination of protocol (TCP or RTU) and role (Client/Master or Server/Slave).
// Error Handling:

// Prints error messages if required configurations are missing (e.g., address, serial port, or baud rate).
// Asynchronous Execution:

// Uses tokio::main to enable async runtime for handling asynchronous operations.