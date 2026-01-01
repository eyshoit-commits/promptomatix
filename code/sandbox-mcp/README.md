# Sandbox MCP (Model Context Protocol)

**Rust implementation of the Model Context Protocol sandbox server**

Rebranded and refactored from `sandbox-mco` to `sandbox-mcp` with improved architecture and implementation.

## Overview

Sandbox MCP is a secure, isolated execution environment for running code and commands through the Model Context Protocol (MCP). It provides:

- **Isolated Execution**: Sandboxed command execution with configurable limits
- **MCP Protocol Support**: Full implementation of the Model Context Protocol
- **Resource Limits**: Configurable timeout and memory constraints
- **Async Runtime**: Built on Tokio for high-performance async operations
- **Type Safety**: Strong typing with Rust's type system
- **Error Handling**: Comprehensive error types and handling

## Architecture

The implementation is organized into several modules:

- **`config`**: Configuration management and builder pattern
- **`error`**: Error types and result handling
- **`sandbox`**: Core sandbox execution logic
- **`server`**: MCP server implementation and message handling
- **`lib.rs`**: Public API surface
- **`main.rs`**: Binary entry point

## Features

### Secure Sandbox Execution

Execute commands in an isolated environment with:
- Timeout controls
- Memory limits
- Working directory isolation
- Environment variable management

### MCP Protocol

Implements Model Context Protocol message types:
- `Execute`: Run commands in the sandbox
- `ExecuteResponse`: Return execution results
- `Ping/Pong`: Health checks
- `Error`: Error responses

### Async/Await

Built on Tokio for:
- Non-blocking I/O
- Concurrent request handling
- Efficient resource utilization

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source

```bash
# Navigate to the sandbox-mcp directory
cd code/sandbox-mcp

# Build the project
cargo build --release

# Run tests
cargo test

# Run the server
cargo run --release
```

## Usage

### Running the Server

```bash
# Run with default configuration
cargo run --release

# Or run the compiled binary
./target/release/sandbox-mcp
```

### Configuration

The server can be configured programmatically:

```rust
use sandbox_mcp::{Config, McpServer};

let config = Config::default()
    .with_host("0.0.0.0".to_string())
    .with_port(8080)
    .with_max_execution_time(60)
    .with_max_memory(1024);

let server = McpServer::new(config);
```

### Default Configuration

- **Host**: `127.0.0.1`
- **Port**: `8080`
- **Max Execution Time**: `30 seconds`
- **Max Memory**: `512 MB`
- **Work Directory**: `/tmp/sandbox-mcp`

## API Examples

### Execute a Command

```rust
use sandbox_mcp::{Sandbox, ExecutionRequest, Config};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let sandbox = Sandbox::new(config);
    
    let request = ExecutionRequest {
        command: "echo".to_string(),
        args: vec!["Hello, MCP!".to_string()],
        env: HashMap::new(),
        cwd: None,
    };
    
    let response = sandbox.execute(request).await.unwrap();
    println!("Output: {}", response.stdout);
}
```

### Handle MCP Messages

```rust
use sandbox_mcp::{McpServer, McpMessage, ExecutionRequest, Config};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let server = McpServer::new(Config::default());
    
    let request = ExecutionRequest {
        command: "ls".to_string(),
        args: vec!["-la".to_string()],
        env: HashMap::new(),
        cwd: None,
    };
    
    let message = McpMessage::Execute {
        id: "req-1".to_string(),
        request,
    };
    
    let response = server.handle_message(message).await.unwrap();
    println!("Response: {:?}", response);
}
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_sandbox_echo
```

### Code Quality

```bash
# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Check for unused dependencies
cargo udeps
```

## Security Considerations

⚠️ **Important Security Notes**:

- The sandbox provides process-level isolation but is not a full security sandbox
- Commands run with the permissions of the server process
- Configure appropriate resource limits for production use
- Review and validate all commands before execution
- Consider using additional OS-level sandboxing (cgroups, namespaces, etc.)

## Migration from sandbox-mco

This is a refactored and rebranded version of `sandbox-mco`. Key changes:

1. **Name**: `sandbox-mco` → `sandbox-mcp` (Model Context Protocol)
2. **Language**: Python/TypeScript → Rust
3. **Architecture**: Improved modular design
4. **Performance**: Better async handling with Tokio
5. **Type Safety**: Strong typing throughout
6. **Error Handling**: Comprehensive error types

## Dependencies

- **tokio**: Async runtime
- **serde**: Serialization framework
- **serde_json**: JSON support
- **anyhow**: Error handling
- **thiserror**: Error derive macros
- **tracing**: Logging and diagnostics
- **async-trait**: Async trait support

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

Apache License 2.0 - See LICENSE.txt for details

## Version History

### v0.1.0 (Current)
- Initial Rust implementation
- Rebrand from sandbox-mco to sandbox-mcp
- Core MCP protocol support
- Sandbox execution engine
- Async/await throughout
- Comprehensive error handling

## Support

For issues, questions, or contributions:
- Open an issue on GitHub
- Contact: Promptomatix Contributors

---

**Note**: This is a refactored implementation of the sandbox execution environment, optimized for the Promptomatix framework's Model Context Protocol requirements.
