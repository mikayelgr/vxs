# vxs

## Overview

VXS is a key-value blob database designed for high performance and scalability. It provides a robust server implementation with features like graceful shutdown, connection limiting, and structured logging. The project is written in Rust and leverages asynchronous programming with Tokio.

## Features

- **Key-Value Blob Storage**: Efficiently store and retrieve large binary objects.
- **Graceful Shutdown**: Ensures all active connections are properly terminated during shutdown.
- **Connection Limiting**: Configurable maximum number of concurrent connections.
- **Structured Logging**: Uses `fern` for detailed and customizable logging.
- **Command-Line Interface**: Powered by `structopt` for easy configuration and usage.

## Installation

1. Ensure you have Rust installed. You can install it via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/michaelgrigoryan25/vxs.git
   cd vxs
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Starting the Server

Run the server with the following command:

```bash
cargo run -- start --address 127.0.0.1:3000 --verbose --log-path debug.log --max-connections 1500
```

#### Command-Line Options

- `--address`: The address and port to bind the server (default: `127.0.0.1:0`).
- `--verbose`: Enable verbose logging.
- `--log-path`: Path to the log file (default: `debug.log`).
- `--max-connections`: Maximum number of concurrent connections (default: `1500`).

### Logging

Logs are written to the specified log file and the console. The log format includes timestamps, log levels, and messages.

## Code Structure

- **`src/bin/vxs.rs`**: Entry point for the application. Handles command-line arguments and initializes the server.
- **`src/server.rs`**: Core server logic, including connection handling and graceful shutdown.
- **`src/shutdown.rs`**: Implements the shutdown signal mechanism.
- **`src/cmd.rs`**: Defines the command-line interface using `structopt`.
- **`src/lib.rs`**: Contains utility traits and logging configuration.

## Development

### Formatting

The project uses `rustfmt` for code formatting. Configuration is defined in `rustfmt.toml`:

```toml
edition = "2021"
hard_tabs = true
use_field_init_shorthand = true
```

Run the formatter with:

```bash
cargo fmt
```

### Testing

Run the tests with:

```bash
cargo test
```

### Debugging

Debugging configurations are provided in `.vscode/launch.json` for Visual Studio Code. You can debug unit tests or the executable directly.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

> Note: The project has been unmaintained since 2022. This README was generated for overall understanding of the project by ChatGPT, and there may be some inconsistencies as I do not have the time to review/work on this.
