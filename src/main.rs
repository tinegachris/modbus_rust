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
        ModbusProtocol::TCP => handle_tcp(config).await?,
        ModbusProtocol::RTU => handle_rtu(config).await?,
    }

    Ok(())
}

async fn handle_tcp(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    match config.role {
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
    }
    Ok(())
}

async fn handle_rtu(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    match config.role {
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
    }
    Ok(())
}
