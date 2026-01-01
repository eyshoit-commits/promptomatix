//! Sandbox Rust MCP Server - Main Entry Point
//!
//! Rust implementation of the Model Context Protocol (MCP) sandbox server.
//! Rebranded and refactored from sandbox-mco.

use sandbox_rust_mcp::{Config, McpServer, Result, VERSION};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!("Sandbox Rust MCP Server v{}", VERSION);
    info!("Rebranded from sandbox-mco - Rust implementation");

    // Load configuration
    let config = Config::default()
        .with_verbose(true)
        .with_host("127.0.0.1".to_string())
        .with_port(8080);

    info!("Configuration loaded:");
    info!("  Host: {}", config.host);
    info!("  Port: {}", config.port);
    info!("  Max execution time: {}s", config.max_execution_time);
    info!("  Max memory: {}MB", config.max_memory_mb);
    info!("  Work directory: {:?}", config.work_dir);

    // Create and start the MCP server
    let server = McpServer::new(config);

    info!("Starting MCP server...");
    server.start().await?;

    // Keep the server running
    // In a real implementation, this would listen for connections
    info!("Server is running. Press Ctrl+C to stop.");

    // Wait for shutdown signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");

    info!("Shutdown signal received");
    server.stop().await?;
    info!("Server stopped successfully");

    Ok(())
}
