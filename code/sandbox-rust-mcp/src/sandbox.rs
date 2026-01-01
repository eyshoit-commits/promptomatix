//! Sandbox execution module

use crate::error::{Error, Result};
use crate::Config;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// Sandbox execution context
#[derive(Debug, Clone)]
pub struct Sandbox {
    config: Config,
}

/// Request for sandbox execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// Command to execute
    pub command: String,

    /// Arguments for the command
    pub args: Vec<String>,

    /// Environment variables
    pub env: std::collections::HashMap<String, String>,

    /// Working directory (relative to sandbox root)
    pub cwd: Option<String>,
}

/// Response from sandbox execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResponse {
    /// Exit code
    pub exit_code: i32,

    /// Standard output
    pub stdout: String,

    /// Standard error
    pub stderr: String,

    /// Execution time in milliseconds
    pub execution_time_ms: u128,

    /// Whether the execution was successful
    pub success: bool,
}

impl Sandbox {
    /// Create a new sandbox instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute a command in the sandbox
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResponse> {
        info!(
            "Executing command: {} with {} args",
            request.command,
            request.args.len()
        );

        // Ensure work directory exists
        tokio::fs::create_dir_all(&self.config.work_dir)
            .await
            .map_err(|e| Error::SandboxExecution(format!("Failed to create work dir: {}", e)))?;

        let start_time = std::time::Instant::now();

        // Build the command
        let mut cmd = Command::new(&request.command);
        cmd.args(&request.args)
            .current_dir(&self.config.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Set environment variables
        for (key, value) in &request.env {
            cmd.env(key, value);
        }

        // Execute with timeout
        let execution_timeout = Duration::from_secs(self.config.max_execution_time);
        let output = match timeout(execution_timeout, cmd.output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                warn!("Command execution failed: {}", e);
                return Err(Error::SandboxExecution(format!(
                    "Command execution failed: {}",
                    e
                )));
            }
            Err(_) => {
                warn!(
                    "Command timed out after {} seconds",
                    self.config.max_execution_time
                );
                return Err(Error::SandboxExecution(format!(
                    "Command timed out after {} seconds",
                    self.config.max_execution_time
                )));
            }
        };

        let execution_time = start_time.elapsed();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        debug!(
            "Command completed with exit code: {}, success: {}",
            exit_code, success
        );

        Ok(ExecutionResponse {
            exit_code,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time_ms: execution_time.as_millis(),
            success,
        })
    }

    /// Get sandbox configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Clean up sandbox resources
    pub async fn cleanup(&self) -> Result<()> {
        info!("Cleaning up sandbox resources");
        // Could implement cleanup logic here if needed
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_echo() {
        let config = Config::default();
        let sandbox = Sandbox::new(config);

        let request = ExecutionRequest {
            command: "echo".to_string(),
            args: vec!["Hello, sandbox!".to_string()],
            env: std::collections::HashMap::new(),
            cwd: None,
        };

        let response = sandbox.execute(request).await.unwrap();
        assert!(response.success);
        assert!(response.stdout.contains("Hello, sandbox!"));
    }
}
