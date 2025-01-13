use tokio_modbus::client::rtu;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Server};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio_serial::SerialPortBuilderExt;

pub async fn start_master(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Open the serial port for Modbus RTU
    let port = tokio_serial::new(serial_port, baud_rate)
        .open_native_async()
        .await?;

    // Create a Modbus RTU context
    let mut ctx = rtu::connect(port).await?;
    println!("Connected to Modbus RTU slave on {}", serial_port);

    // Example: Reading holding registers from address 0 to 9
    let response = ctx.read_holding_registers(0, 10).await?;
    println!("Read holding registers: {:?}", response);

    // Example: Writing to a single register
    ctx.write_single_register(0, 42).await?;
    println!("Wrote 42 to register 0");

    Ok(())
}

pub async fn start_slave(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Open the serial port for Modbus RTU
    let port = tokio_serial::new(serial_port, baud_rate)
        .open_native_async()
        .await?;

    println!("Starting Modbus RTU slave on {}", serial_port);

    // Example shared state: A mutex-guarded vector of registers
    let shared_data = Arc::new(Mutex::new(vec![0; 100]));

    let server = Server::new(move |ctx| {
        let data = shared_data.clone();
        async move {
            match ctx {
                server::Request::ReadHoldingRegisters(start, count) => {
                    let data = data.lock().await;
                    Ok(server::Response::HoldingRegisters(
                        data[start as usize..(start + count) as usize].to_vec(),
                    ))
                }
                server::Request::WriteSingleRegister(addr, value) => {
                    let mut data = data.lock().await;
                    data[addr as usize] = value;
                    Ok(server::Response::Empty)
                }
                _ => Err(server::Error::Other("Unsupported request".into())),
            }
        }
    });

    // Run the slave (server)
    server.serve(port).await?;

    Ok(())
}


// Explanation
// 1. start_master (RTU Master / Client)
// Opens the serial port for Modbus RTU using the tokio_serial crate.
// Creates a Modbus RTU context with rtu::connect.
// Demonstrates:
// Reading holding registers using read_holding_registers.
// Writing to a single register using write_single_register.
// 2. start_slave (RTU Slave / Server)
// Opens the serial port using the tokio_serial crate for Modbus RTU communication.
// Uses a shared state (Arc<Mutex<Vec<u16>>>) to simulate holding registers.
// Responds to Modbus requests:
// ReadHoldingRegisters: Reads a range of registers.
// WriteSingleRegister: Writes a value to a single register.
// 3. Shared State for Registers
// The Arc<Mutex<Vec<u16>>> is used to simulate holding registers in memory for the slave.
// It ensures safe, concurrent access to the registers when handling multiple requests.
// 4. Serial Communication
// Uses tokio_serial to open the serial port asynchronously.
// The port is configured using the SerialPortBuilderExt trait from the tokio_serial crate.
// The communication is handled asynchronously using Tokio, ensuring non-blocking I/O.
// 5. Address Parsing
// The serial port is passed directly to the function (/dev/ttyUSB0 or COM1 on Windows).