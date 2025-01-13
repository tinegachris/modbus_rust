use modbus_project::modbus::tcp::start_tcp_client;
use std::env;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    // Get the server IP and port from environment variables or default values
    let server_ip = env::var("MODBUS_SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("MODBUS_SERVER_PORT")
        .unwrap_or_else(|_| "502".to_string())
        .parse::<u16>()
        .expect("Invalid server port");

    println!("Starting Modbus TCP Client...");

    if let Err(e) = start_tcp_client(&server_ip, server_port).await {
        eprintln!("Error starting Modbus TCP Client: {}", e);
    } else {
        println!("Modbus TCP Client started successfully.");
    }
}

// Explanation
// 1. Environment Variables
// The program retrieves the Modbus server IP address and port from environment variables (MODBUS_SERVER_IP and MODBUS_SERVER_PORT).
// If these environment variables are not set, the program defaults to 127.0.0.1 for the IP address (localhost) and 502 for the port (the default Modbus TCP port).
// 2. Calling the TCP Client
// The start_tcp_client function from the modbus_project::modbus::tcp module is called to initiate the Modbus TCP client communication.
// The server IP and port are passed as arguments.
// 3. Error Handling
// If any error occurs while starting the client, it prints an error message.
// 4. Tokio Runtime
// The #[tokio::main] macro is used to create the asynchronous runtime for running the TCP client.