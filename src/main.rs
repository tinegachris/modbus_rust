pub mod modbus;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;
use tokio_modbus::client::rtu;
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use std::error::Error;

#[derive(Debug)]
enum ModbusProtocol {
    RTU,
    TCP,
}

async fn read_data(protocol: ModbusProtocol, address: &str, unit_id: u8, register: u16) -> Result<u16, Box<dyn Error>> {
    match protocol {
        ModbusProtocol::RTU => {
            let tty_path = address;
            let builder = tokio_serial::new(tty_path, 9600);
            let port = SerialStream::open(&builder)?;
            let mut ctx = rtu::connect_slave(port, unit_id).await?;
            let response = ctx.read_holding_registers(register, 1).await?;
            Ok(response[0])
        }
        ModbusProtocol::TCP => {
            let socket_addr = address.parse()?;
            let mut ctx = tcp::connect_slave(socket_addr, unit_id).await?;
            let response = ctx.read_holding_registers(register, 1).await?;
            Ok(response[0])
        }
    }
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    // Example for RTU Communication
    match read_data(ModbusProtocol::RTU, "/dev/ttyUSB0", 1, 100).await {
        Ok(value) => println!("RTU: Register value is {}", value),
        Err(err) => eprintln!("RTU Error: {}", err),
    }

    // Example for TCP Communication
    match read_data(ModbusProtocol::TCP, "192.168.1.100:502", 1, 100).await {
        Ok(value) => println!("TCP: Register value is {}", value),
        Err(err) => eprintln!("TCP Error: {}", err),
    }

    Ok(())
}
