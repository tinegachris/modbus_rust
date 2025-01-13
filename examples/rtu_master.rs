use modbus_project::modbus::rtu::start_master;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the serial port and baud rate from environment variables or default values
    let serial_port = env::var("MODBUS_SERIAL_PORT").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());
    let baud_rate: u32 = env::var("MODBUS_BAUD_RATE")
        .unwrap_or_else(|_| "9600".to_string())
        .parse()
        .expect("Invalid baud rate");

    println!("Starting Modbus RTU Master...");
    start_master(&serial_port, baud_rate).await?;

    Ok(())
}
