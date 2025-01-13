pub mod modbus {
    pub mod tcp;
    pub mod rtu;

    use tokio_modbus::prelude::*;
    use std::error::Error;

    // Common types or constants
    pub type ModbusResult<T> = Result<T, Box<dyn Error>>;

    #[derive(Debug, Clone, Copy)]
    pub enum ModbusProtocol {
        RTU,
        TCP,
    }

    // Unified interface for Modbus operations
    pub async fn read_data(
        protocol: ModbusProtocol,
        address: &str,
        unit_id: u8,
        register: u16,
    ) -> ModbusResult<u16> {
        match protocol {
            ModbusProtocol::RTU => {
                let serial = SerialStream::open(address)?;
                let mut ctx = rtu::connect_slave(serial, unit_id).await?;
                let response = ctx.read_holding_registers(register, 1).await?;
                Ok(response[0])
            }
            ModbusProtocol::TCP => {
                let (ip, port) = parse_address(address)?;
                let mut ctx = tcp::connect((ip, port)).await?;
                let response = ctx.read_holding_registers(register, 1).await?;
                Ok(response[0])
            }
        }
    }

    // Utility to parse TCP address
    fn parse_address(address: &str) -> ModbusResult<(&str, u16)> {
        let mut parts = address.split(':');
        let ip = parts.next().ok_or_else(|| format!("Invalid address format: {}", address))?;
        let port_str = parts.next().ok_or_else(|| format!("Invalid address format: {}", address))?;
        let port: u16 = port_str.parse()?;
        Ok((ip, port))
    }
}