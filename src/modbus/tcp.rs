use std::net::SocketAddr;
use std::error::Error;

use tokio_modbus::prelude::*;

pub async fn read_tcp_data(ip: &str, port: u16, slave: u8, register: u16) -> Result<u16, Box<dyn Error>> {
    let socket_addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    let mut ctx = tcp::connect_slave(socket_addr, Slave(slave)).await?;
    let response = ctx.read_holding_registers(register, 1).await?;
    ctx.disconnect().await?;
    Ok(response[0])
}
