use modbus_project::modbus::tcp::start_tcp_server;
use std::env;

#[tokio::main]
async fn main() {
    // Get the server IP and port from environment variables or default values
    let server_ip = env::var("MODBUS_SERVER_IP").unwrap_or("127.0.0.1".to_string());
    let server_port: u16 = env::var("MODBUS_SERVER_PORT")
        .unwrap_or("502".to_string())
        .parse()
        .expect("Invalid server port");

    println!("Starting Modbus TCP Server...");

    if let Err(e) = start_tcp_server(&server_ip, server_port).await {
        eprintln!("Error: {}", e);
    }
}


// Explanation
// 1. Environment Variables
// The program retrieves the server IP address and port from environment variables (MODBUS_SERVER_IP and MODBUS_SERVER_PORT).
// If these environment variables are not set, the program defaults to 127.0.0.1 for the IP address (localhost) and 502 for the port (the default Modbus TCP port).
// 2. Calling the TCP Server
// The start_tcp_server function from the modbus_project::modbus::tcp module is called to initiate the Modbus TCP server.
// The server IP and port are passed as arguments.
// 3. Error Handling
// If any error occurs while starting the server, it prints an error message.
// 4. Tokio Runtime
// The #[tokio::main] macro is used to create the asynchronous runtime for running the TCP server.