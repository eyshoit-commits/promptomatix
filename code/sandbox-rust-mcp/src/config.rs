//! Configuration module for sandbox-mcp

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server host address
    pub host: String,

    /// Server port
    pub port: u16,

    /// Maximum execution time in seconds
    pub max_execution_time: u64,

    /// Maximum memory limit in MB
    pub max_memory_mb: usize,

    /// Working directory for sandbox
    pub work_dir: PathBuf,

    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_execution_time: 30,
            max_memory_mb: 512,
            work_dir: PathBuf::from("/tmp/sandbox-rust-mcp"),
            verbose: false,
        }
    }
}

impl Config {
    /// Create a new configuration with custom values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the host address
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    /// Set the port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the maximum execution time
    pub fn with_max_execution_time(mut self, seconds: u64) -> Self {
        self.max_execution_time = seconds;
        self
    }

    /// Set the maximum memory limit
    pub fn with_max_memory(mut self, mb: usize) -> Self {
        self.max_memory_mb = mb;
        self
    }

    /// Set the working directory
    pub fn with_work_dir(mut self, path: PathBuf) -> Self {
        self.work_dir = path;
        self
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}
