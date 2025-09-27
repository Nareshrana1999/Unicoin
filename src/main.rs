//! Unicoin Node - Main entry point
//! 
//! This binary runs a Unicoin node that participates in the network,
//! validates transactions, and maintains the blockchain.

use unicoin::{
    blockchain::Blockchain,
    network::NetworkNode,
    consensus::ConsensusEngine,
    Result,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
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

    // Start the node
    let node = UnicoinNode {
        blockchain,
        consensus,
        network,
    };

    // Run the node
    if let Err(e) = node.run().await {
        error!("Node error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Main Unicoin node structure
pub struct UnicoinNode {
    blockchain: Blockchain,
    consensus: ConsensusEngine,
    network: NetworkNode,
}

impl UnicoinNode {
    /// Run the Unicoin node
    pub async fn run(mut self) -> Result<()> {
        info!("Unicoin node is running...");

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
        network_handle.abort();
        consensus_handle.abort();

        info!("Unicoin node stopped");
        Ok(())
    }
}
