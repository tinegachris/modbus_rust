use modbus::client::{ModbusTcpClient, ModbusRtuClient};
use modbus::prelude::*;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/// Modbus TCP Client logic
pub async fn start_tcp_client(server_ip: &str, server_port: u16) -> Result<(), Box<dyn Error>> {
    let address = format!("{}:{}", server_ip, server_port);
    let stream = TcpStream::connect(address).await?;
    let (reader, mut writer) = stream.split();
    let mut client = ModbusTcpClient::new(reader, writer);

    perform_modbus_operations(&mut client).await?;
    Ok(())
}

/// Modbus RTU Client logic
pub async fn start_rtu_client(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    let mut client = ModbusRtuClient::new(serial_port, baud_rate)?;

    perform_modbus_operations(&mut client).await?;
    Ok(())
}

async fn perform_modbus_operations<C: ModbusClient>(client: &mut C) -> Result<(), Box<dyn Error>>{
    // Read Holding Registers
    let holding_registers = match client.read_holding_registers(0, 5).await {
        Ok(registers) => registers,
        Err(e) => {
            println!("Error reading holding registers: {}", e);
            return Err(Box::new(e));
        }
    };
    println!("Holding Registers: {:?}", holding_registers);

    // Write Single Register
    if let Err(e) = client.write_single_register(0, 123).await {
        println!("Error writing single register: {}", e);
        return Err(Box::new(e));
    }

    println!("Wrote value 123 to register 0.");

    Ok(())
}
