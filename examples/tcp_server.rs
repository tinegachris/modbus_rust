use modbus_project::modbus::tcp::start_tcp_server;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get the server IP and port from environment variables or use default values
    let server_ip = env::var("MODBUS_SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("MODBUS_SERVER_PORT")
        .unwrap_or_else(|_| "502".to_string())
        .parse()
        .expect("Invalid server port");

    let server_addr: SocketAddr = format!("{}:{}", server_ip, server_port)
        .parse()
        .expect("Invalid server address");

    println!("Starting Modbus TCP Server at {}...", server_addr);

    if let Err(e) = start_tcp_server(&server_ip, server_port).await {
        eprintln!("Error: {}", e);
    }
}
