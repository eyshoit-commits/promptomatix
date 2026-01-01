//! MCP Server implementation

use crate::error::{Error, Result};
use crate::sandbox::{ExecutionRequest, ExecutionResponse, Sandbox};
use crate::Config;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// MCP protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum McpMessage {
    /// Execute command in sandbox
    Execute {
        /// Request ID
        id: String,
        /// Execution request
        request: ExecutionRequest,
    },

    /// Response to execution
    ExecuteResponse {
        /// Request ID
        id: String,
        /// Execution response
        response: ExecutionResponse,
    },

    /// Error response
    Error {
        /// Request ID
        id: String,
        /// Error message
        message: String,
    },

    /// Ping message
    Ping {
        /// Request ID
        id: String,
    },

    /// Pong response
    Pong {
        /// Request ID
        id: String,
    },
}

/// MCP Server state
pub struct McpServer {
    config: Arc<Config>,
    sandbox: Arc<RwLock<Sandbox>>,
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(config: Config) -> Self {
        let sandbox = Sandbox::new(config.clone());
        Self {
            config: Arc::new(config),
            sandbox: Arc::new(RwLock::new(sandbox)),
        }
    }

    /// Start the MCP server
    pub async fn start(&self) -> Result<()> {
        info!(
            "Starting MCP server on {}:{}",
            self.config.host, self.config.port
        );

        // Server implementation would go here
        // For now, this is a minimal implementation
        info!("MCP server is ready to accept requests");

        Ok(())
    }

    /// Handle an MCP message
    pub async fn handle_message(&self, message: McpMessage) -> Result<McpMessage> {
        match message {
            McpMessage::Execute { id, request } => {
                info!("Processing execute request: {}", id);
                let sandbox = self.sandbox.read().await;
                match sandbox.execute(request).await {
                    Ok(response) => Ok(McpMessage::ExecuteResponse { id, response }),
                    Err(e) => {
                        error!("Execution failed: {}", e);
                        Ok(McpMessage::Error {
                            id,
                            message: e.to_string(),
                        })
                    }
                }
            }
            McpMessage::Ping { id } => {
                info!("Received ping: {}", id);
                Ok(McpMessage::Pong { id })
            }
            _ => Err(Error::Protocol(
                "Invalid message type for request".to_string(),
            )),
        }
    }

    /// Stop the MCP server
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping MCP server");
        let sandbox = self.sandbox.read().await;
        sandbox.cleanup().await?;
        Ok(())
    }

    /// Get server configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_ping_pong() {
        let config = Config::default();
        let server = McpServer::new(config);

        let ping = McpMessage::Ping {
            id: "test-1".to_string(),
        };

        let response = server.handle_message(ping).await.unwrap();
        match response {
            McpMessage::Pong { id } => assert_eq!(id, "test-1"),
            _ => panic!("Expected Pong response"),
        }
    }

    #[tokio::test]
    async fn test_execute_message() {
        let config = Config::default();
        let server = McpServer::new(config);

        let request = ExecutionRequest {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            cwd: None,
        };

        let execute = McpMessage::Execute {
            id: "test-2".to_string(),
            request,
        };

        let response = server.handle_message(execute).await.unwrap();
        match response {
            McpMessage::ExecuteResponse { id, response } => {
                assert_eq!(id, "test-2");
                assert!(response.success);
            }
            _ => panic!("Expected ExecuteResponse"),
        }
    }
}
