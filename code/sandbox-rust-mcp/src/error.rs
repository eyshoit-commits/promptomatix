//! Error types for sandbox-mcp

use thiserror::Error;

/// Result type alias for sandbox-mcp operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in sandbox-mcp
#[derive(Error, Debug)]
pub enum Error {
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Sandbox execution errors
    #[error("Sandbox execution error: {0}")]
    SandboxExecution(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// MCP protocol errors
    #[error("MCP protocol error: {0}")]
    Protocol(String),

    /// Generic errors
    #[error("{0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
