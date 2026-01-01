//! Sandbox MCP (Model Context Protocol) Server
//!
//! This is a Rust implementation of a Model Context Protocol server with sandboxed execution capabilities.
//! Rebranded and refactored from the original sandbox-mco implementation.

pub mod config;
pub mod error;
pub mod sandbox;
pub mod server;

pub use config::Config;
pub use error::{Error, Result};
pub use sandbox::Sandbox;
pub use server::McpServer;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
