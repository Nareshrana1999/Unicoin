//! Unicoin Node - Main entry point
//! 
//! This binary runs a Unicoin node that participates in the network,
//! validates transactions, and maintains the blockchain.

use unicoin::{
    blockchain::Blockchain,
    network::NetworkNode,
    consensus::ConsensusEngine,
    config::{UnicoinConfig, ConfigBuilder},
    api::ApiServer,
    Result,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = load_configuration().await?;
    
    // Initialize logging based on configuration
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Unicoin Node v{}", unicoin::VERSION);

    // Initialize the blockchain
    let blockchain = Blockchain::new()?;
    info!("Blockchain initialized");

    // Initialize consensus engine
    let consensus = ConsensusEngine::new(blockchain.clone())?;
    info!("Consensus engine initialized");

    // Initialize network node
    let network = NetworkNode::new(consensus.clone())?;
    info!("Network node initialized");

    // Initialize API server
    let api_server = ApiServer::new(config.api.clone());
    info!("API server initialized");

    // Start the node
    let node = UnicoinNode {
        blockchain,
        consensus,
        network,
        api_server,
        config,
    };

    // Run the node
    if let Err(e) = node.run().await {
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

/// Main Unicoin node structure
pub struct UnicoinNode {
    blockchain: Blockchain,
    consensus: ConsensusEngine,
    network: NetworkNode,
    api_server: ApiServer,
    config: UnicoinConfig,
}

impl UnicoinNode {
    /// Run the Unicoin node
    pub async fn run(mut self) -> Result<()> {
        info!("Unicoin node is running...");

        // Start API server
        let api_handle = tokio::spawn(async move {
            self.api_server.start().await
        });

        // Start network services
        let network_handle = tokio::spawn(async move {
            self.network.start().await
        });

        // Start consensus services
        let consensus_handle = tokio::spawn(async move {
            self.consensus.start().await
        });

        // Wait for shutdown signal
        tokio::signal::ctrl_c().await?;
        info!("Shutdown signal received");

        // Graceful shutdown
        api_handle.abort();
        network_handle.abort();
        consensus_handle.abort();

        info!("Unicoin node stopped");
        Ok(())
    }
}
