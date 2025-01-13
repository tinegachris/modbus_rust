use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_client(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the address
    let socket_addr: SocketAddr = address.parse()?;

    // Connect to the Modbus server
    let mut ctx = tcp::connect(socket_addr).await?;
    println!("Connected to Modbus TCP server at {}", address);

    // Example: Reading holding registers from address 0 to 9
    let response = ctx.read_holding_registers(0, 10).await?;
    println!("Read holding registers: {:?}", response);

    // Example: Writing to a single register
    ctx.write_single_register(0, 42).await?;
    println!("Wrote 42 to register 0");

    Ok(())
}

pub async fn start_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the address
    let socket_addr: SocketAddr = address.parse()?;

    println!("Starting Modbus TCP server at {}", address);

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

    // Run the server
    server.serve(socket_addr).await?;

    Ok(())
}


// Explanation
// 1. start_client
// Connects to a Modbus TCP server at the provided address.
// Demonstrates:
// Reading holding registers (read_holding_registers).
// Writing to a single register (write_single_register).
// 2. start_server
// Starts a Modbus TCP server at the provided address.
// Demonstrates:
// Reading holding registers using a shared state (Arc<Mutex<Vec<u16>>>).
// Writing to a single register.
// Uses a handler to process incoming Modbus requests and responds appropriately.
// 3. Shared State
// The server uses Arc<Mutex<Vec<u16>>> to simulate holding registers in memory.
// Ensures thread-safe access when handling concurrent requests.
// 4. Address Parsing
// Uses std::net::SocketAddr to validate and parse the provided address.