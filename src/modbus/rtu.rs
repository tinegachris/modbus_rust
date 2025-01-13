use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortBuilderExt};
use std::error::Error;

pub async fn read_rtu_data(port_name: &str, slave: u8, register: u16) -> Result<u16, Box<dyn Error>> {
    let serial = Serial::from_path(port_name, Default::default())?
        .baud_rate(19200)
        .open_native_async()?;

    let mut ctx = rtu::connect_slave(serial, Slave(slave)).await?;
    let response = ctx.read_holding_registers(register, 1).await?;
    ctx.disconnect().await?;

    response.get(0).cloned().ok_or_else(|| "Failed to read register".into())
}
