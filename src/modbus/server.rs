use modbus::server::{ModbusTcpServer, ModbusRtuServer};
use modbus::prelude::*;
use std::error::Error;
use tokio::time::Duration;

/// Modbus TCP Server logic
pub async fn start_tcp_server(server_ip: &str, server_port: u16) -> Result<(), Box<dyn Error>> {
    // Create and configure Modbus TCP server
    let mut server = ModbusTcpServer::new(format!("{}:{}", server_ip, server_port)).await?;
    server.add_holding_registers(0, vec![0; 10])?;

    // Run the server
    println!("Modbus TCP Server is running...");
    server.run().await?;

    Ok(())
}

/// Modbus RTU Server logic
pub async fn start_rtu_server(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    // Create and configure Modbus RTU server
    let mut server = ModbusRtuServer::new(serial_port, baud_rate).await?;
    server.add_holding_registers(0, vec![0; 10])?;

    // Run the server
    println!("Modbus RTU Server is running...");
    server.run().await?;

    Ok(())
}
