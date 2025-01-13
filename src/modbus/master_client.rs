use modbus::client::{ModbusTcpClient, ModbusRtuClient};
use modbus::prelude::*;
use std::error::Error;

/// Modbus TCP Client logic
pub async fn start_tcp_client(server_ip: &str, server_port: u16) -> Result<(), Box<dyn Error>> {
    let address = format!("{}:{}", server_ip, server_port);
    let mut client = ModbusTcpClient::new(address).await?;

    perform_modbus_operations(&mut client).await
}

/// Modbus RTU Client logic
pub async fn start_rtu_client(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    let mut client = ModbusRtuClient::new(serial_port, baud_rate).await?;

    perform_modbus_operations(&mut client).await
}

async fn perform_modbus_operations<C: ModbusClient>(client: &mut C) -> Result<(), Box<dyn Error>> {
    let holding_registers = client.read_holding_registers(0, 5).await?;
    println!("Holding Registers: {:?}", holding_registers);

    client.write_single_register(0, 123).await?;
    println!("Wrote value 123 to register 0.");

    Ok(())
}
