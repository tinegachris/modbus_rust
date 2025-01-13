use modbus_project::modbus::rtu::start_slave;
use std::env;

#[tokio::main]
async fn main() {
    // Get the serial port and baud rate from environment variables or default values
    let serial_port = env::var("MODBUS_SERIAL_PORT").unwrap_or("/dev/ttyUSB0".to_string());
    let baud_rate: u32 = env::var("MODBUS_BAUD_RATE")
        .unwrap_or("9600".to_string())
        .parse()
        .expect("Invalid baud rate");

    println!("Starting Modbus RTU Slave...");
    if let Err(e) = start_slave(&serial_port, baud_rate).await {
        eprintln!("Error: {}", e);
    }
}


// Explanation
// 1. Environment Variables
// Similar to rtu_master.rs, this program checks for the MODBUS_SERIAL_PORT and MODBUS_BAUD_RATE environment variables to determine the serial port and baud rate for communication.
// If these variables are not set, it defaults to /dev/ttyUSB0 (or COM port for Windows) and 9600 baud rate.
// 2. Calling the RTU Slave
// The start_slave function from the modbus_project::modbus::rtu module is called to initiate the Modbus RTU slave/server communication.
// The serial port and baud rate are passed to the start_slave function.
// 3. Error Handling
// If any error occurs while starting the slave, an error message will be printed.
// 4. Tokio Runtime
// The #[tokio::main] macro is used to run the program asynchronously with Tokio's runtime.