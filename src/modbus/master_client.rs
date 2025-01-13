use modbus::client::{self, ModbusTcpClient, ModbusRtuClient};
use modbus::prelude::*;
use tokio::time::Duration;
use std::error::Error;

/// Modbus TCP Client logic
pub async fn start_tcp_client(server_ip: &str, server_port: u16) -> Result<(), Box<dyn Error>> {
    // Create Modbus TCP client
    let mut client = ModbusTcpClient::new(format!("{}:{}", server_ip, server_port)).await?;
    
    // Read Holding Registers (Example: Reading 5 registers starting from address 0)
    let holding_registers: Vec<u16> = client.read_holding_registers(0, 5).await?;
    println!("Holding Registers: {:?}", holding_registers);

    // Write to a Holding Register (Example: Write value 123 to register at address 0)
    client.write_single_register(0, 123).await?;
    println!("Wrote value 123 to register 0.");

    Ok(())
}

/// Modbus RTU Client logic
pub async fn start_rtu_client(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    // Create Modbus RTU client
    let mut client = ModbusRtuClient::new(serial_port, baud_rate).await?;
    
    // Read Holding Registers (Example: Reading 5 registers starting from address 0)
    let holding_registers: Vec<u16> = client.read_holding_registers(0, 5).await?;
    println!("Holding Registers: {:?}", holding_registers);

    // Write to a Holding Register (Example: Write value 456 to register at address 0)
    client.write_single_register(0, 456).await?;
    println!("Wrote value 456 to register 0.");

    Ok(())
}


// Explanation of master.rs:
// 1. Modbus TCP Client:
// We create a Modbus TCP client using the ModbusTcpClient and configure it with the server's IP and port.
// The start_tcp_client function performs the following tasks:
// Reads 5 holding registers starting from address 0.
// Writes the value 123 to holding register 0.
// 2. Modbus RTU Client:
// Similarly, for Modbus RTU communication, we create a ModbusRtuClient and configure it with the serial port and baud rate.
// The start_rtu_client function does the following:
// Reads 5 holding registers starting from address 0.
// Writes the value 456 to holding register 0.