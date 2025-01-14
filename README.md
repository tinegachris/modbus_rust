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
tokio = { version = "1", features = ["full"] }
tokio-serial = "0.6"
serde = { version = "1", features = ["derive"] }
tracing-subscriber = "0.3"
env_logger = "0.10"
```

## Examples

### TCP Server

```rust
use tokio_modbus::server::tcp::{Server};
use tokio_modbus::prelude::{Value, ModbusContext};

#[tokio::main]
async fn main() {
    let server = Server::new();
    server.serve().await.unwrap();
}
```

### RTU Master

```rust
use tokio_modbus::prelude::{Value, ModbusContext};
use tokio_serial::SerialStream;
use tokio_modbus::rtu;

#[tokio::main]
async fn main() {
    let port = SerialStream::open("/dev/ttyUSB0").unwrap();
    let ctx = rtu::connect(port, 1).await.unwrap();
    let response:Vec<Value> = ctx.read_holding_registers(0x0000, 7).await.unwrap();
    //let response = ctx.read_holding_registers(0x1000, 7).await.unwrap();
    println!("Response: {:?}", response);
}
```

## License

This project is licensed under the MIT License.
