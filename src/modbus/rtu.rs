use tokio_modbus::client::rtu;
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Server};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_serial::SerialPortBuilderExt;

pub async fn start_master(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let port = tokio_serial::new(serial_port, baud_rate)
        .open_native_async()
        .await?;

    let ctx = rtu::connect(port).await?;
    println!("Connected to Modbus RTU slave on {}", serial_port);

    let mut ctx = ctx;

    loop {
        match ctx.read_holding_registers(0, 10).await {
            Ok(response) => println!("Read holding registers: {:?}", response),
            Err(e) => println!("Error reading holding registers: {}", e),
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        match ctx.write_single_register(0, 42).await {
            Ok(_) => {},
            Err(e) => println!("Error writing to register 0: {}", e),
        }
        println!("Wrote 42 to register 0");
    }

    Ok(())
}

pub async fn start_slave(serial_port: &str, baud_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let port = tokio_serial::new(serial_port, baud_rate)
        .open_native_async()
        .await?;

    println!("Starting Modbus RTU slave on {}", serial_port);

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

    server.serve(port).await?;

    Ok(())
}
