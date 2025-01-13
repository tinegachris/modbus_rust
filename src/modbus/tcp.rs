use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_client(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = address.parse()?;
    let mut ctx = tcp::connect(socket_addr).await?;
    println!("Connected to Modbus TCP server at {}", address);

    let response = ctx.read_holding_registers(0, 10).await?;
    println!("Read holding registers: {:?}", response);

    ctx.write_single_register(0, 42).await?;
    println!("Wrote 42 to register 0");

    Ok(())
}

pub async fn start_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = address.parse()?;
    println!("Starting Modbus TCP server at {}", address);

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

    server.serve(socket_addr).await?;
    Ok(())
}
