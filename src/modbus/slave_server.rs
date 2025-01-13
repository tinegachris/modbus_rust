use modbus::server::{self, ModbusTcpServer, ModbusRtuServer};
use modbus::prelude::*;
use std::error::Error;
use tokio::time::Duration;

/// Modbus TCP Server logic
pub async fn start_tcp_server(server_ip: &str, server_port: u16) -> Result<(), Box<dyn Error>> {
    // Create Modbus TCP server
    let mut server = ModbusTcpServer::new(format!("{}:{}", server_ip, server_port)).await?;
    
    // Add some registers (e.g., 10 holding registers initialized to 0)
    server.add_holding_registers(0, 10)?;
    
    // Run the server
    println!("Modbus TCP Server is running...");
    server.run().await?;

    Ok(())
}

/// Modbus RTU Server logic
pub async fn start_rtu_server(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    // Create Modbus RTU server
    let mut server = ModbusRtuServer::new(serial_port, baud_rate).await?;

    // Add some registers (e.g., 10 holding registers initialized to 0)
    server.add_holding_registers(0, 10)?;

    // Run the server
    println!("Modbus RTU Server is running...");
    server.run().await?;

    Ok(())
}


// Explanation of slave.rs:
// 1. Modbus TCP Server:
// We create a Modbus TCP server using ModbusTcpServer and bind it to the given IP and port.
// The start_tcp_server function initializes 10 holding registers (set to 0) and runs the server.
// The server will listen for requests from Modbus TCP clients and respond to them accordingly.
// 2. Modbus RTU Server:
// For the Modbus RTU server, we use ModbusRtuServer, binding it to the given serial port and baud rate.
// The start_rtu_server function also initializes 10 holding registers (set to 0) and runs the server to listen for incoming RTU requests.