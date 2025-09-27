//! Unicoin Node - Main entry point
//! 
//! This binary runs a Unicoin node that participates in the network,
//! validates transactions, and maintains the blockchain.

use unicoin::{
    core::UnicoinNode,
    config::UnicoinConfig,
    cli::CliApp,
    Result,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Check if we should run in CLI mode
    let args: Vec<String> = std::env::args().collect();
    
    // If CLI arguments are provided, run CLI mode
    if args.len() > 1 && args[1] != "start" {
        let cli_app = CliApp::new();
        return cli_app.run().await;
    }

    // Otherwise, run in normal node mode
    run_node().await
}

/// Run the Unicoin node
async fn run_node() -> Result<()> {
    // Load configuration
    let config = load_configuration().await?;
    
    // Initialize logging based on configuration
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Unicoin Node v{}", unicoin::VERSION);

    // Create and start the node
    let node = UnicoinNode::new(config).await?;
    
    // Start the node
    if let Err(e) = node.start().await {
        error!("Failed to start node: {}", e);
        std::process::exit(1);
    }

    // Wait for shutdown signal
    if let Err(e) = node.wait_for_shutdown().await {
        error!("Node error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Load configuration from file or use defaults
async fn load_configuration() -> Result<UnicoinConfig> {
    // Try to load from config file
    if let Ok(config) = UnicoinConfig::load_from_file("config.toml") {
        info!("Configuration loaded from config.toml");
        return Ok(config);
    }

    // Use default configuration
    info!("Using default configuration");
    let config = UnicoinConfig::new();
    
    // Save default configuration to file
    if let Err(e) = config.save_to_file("config.toml") {
        error!("Failed to save default configuration: {}", e);
    } else {
        info!("Default configuration saved to config.toml");
    }

    Ok(config)
}
