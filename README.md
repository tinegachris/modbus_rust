# Modbus Rust Project

This project is a Rust implementation for Modbus communication. It includes both TCP and RTU protocols.

## Features

- Modbus TCP and RTU support
- Asynchronous communication using Tokio
- Serial port communication
- Logging and tracing support

## Dependencies

- `tokio`
- `tokio-serial`
- `serde`
- `tracing-subscriber`
- `env_logger`

## Project Structure

- `src/lib.rs`: Main library file
- `src/modbus/`: Modbus protocol implementation
  - `common.rs`: Common utilities and definitions
  - `master_client.rs`: Modbus master client implementation
  - `rtu.rs`: Modbus RTU protocol implementation
  - `slave_server.rs`: Modbus slave server implementation
  - `tcp.rs`: Modbus TCP protocol implementation
- `src/config.rs`: Configuration handling

## Usage

To use this library, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
tokio = "1.43.0"
tokio-serial = "4.6.1"
serde = "1.0.217"
tracing-subscriber = "0.3.19"
env_logger = "0.9.0"
```

## Examples

### TCP Server

```rust
use tokio_modbus::prelude::*;
use tokio_modbus::server::tcp::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();
    server.serve().await.unwrap();
}
```

### RTU Master

```rust
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

#[tokio::main]
async fn main() {
    let port = SerialStream::open("/dev/ttyUSB0").unwrap();
    let mut ctx = rtu::connect(port).await.unwrap();
    let response = ctx.read_holding_registers(0x1000, 7).await.unwrap();
    println!("Response: {:?}", response);
}
```

## License

This project is licensed under the MIT License.