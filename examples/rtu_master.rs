use modbus_project::modbus::rtu::start_master;
use std::env;

#[tokio::main]
async fn main() {
    // Get the serial port and baud rate from environment variables or default values
    let serial_port = env::var("MODBUS_SERIAL_PORT").unwrap_or("/dev/ttyUSB0".to_string());
    let baud_rate: u32 = env::var("MODBUS_BAUD_RATE")
        .unwrap_or("9600".to_string())
        .parse()
        .expect("Invalid baud rate");

    println!("Starting Modbus RTU Master...");
    if let Err(e) = start_master(&serial_port, baud_rate).await {
        eprintln!("Error: {}", e);
    }
}


// Explanation
// 1. Environment Variables
// The program first attempts to get the serial port and baud rate from environment variables (MODBUS_SERIAL_PORT and MODBUS_BAUD_RATE).
// If the environment variables are not set, it falls back to default values (/dev/ttyUSB0 for Linux or /COM1 on Windows for the serial port and 9600 for the baud rate).
// 2. Calling the RTU Master
// The start_master function from the modbus_project::modbus::rtu module is called to initiate the Modbus RTU master/client communication.
// The serial port and baud rate are passed as arguments.
// 3. Error Handling
// If any error occurs while starting the master, it will print an error message.
// 4. Tokio Runtime
// The #[tokio::main] macro sets up the asynchronous Tokio runtime.