//! Unicoin Node Implementation
//!
//! This module provides the main node implementation that integrates all components
//! including blockchain, consensus, network, transaction processing, and API.

use crate::{
    blockchain::{
        Blockchain, UTXOManager, TransactionProcessor, Transaction, Block,
        TransactionResult, TransactionStatus, CoinSelectionAlgorithm,
    },
    consensus::ConsensusEngine,
    network::{P2PNetwork, NetworkMessage},
    config::UnicoinConfig,
    api::ApiServer,
    crypto::{Hash, PublicKey, PrivateKey},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

/// Node status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    /// Node is starting up
    Starting,
    /// Node is running normally
    Running,
    /// Node is syncing with the network
    Syncing,
    /// Node is stopping
    Stopping,
    /// Node is stopped
    Stopped,
    /// Node encountered an error
    Error(String),
}

/// Node statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    /// Node status
    pub status: NodeStatus,
    /// Blockchain height
    pub blockchain_height: u64,
    /// Number of connected peers
    pub connected_peers: usize,
    /// Total transactions processed
    pub total_transactions: u64,
    /// Total blocks processed
    pub total_blocks: u64,
    /// Network uptime
    pub uptime: u64,
    /// Memory usage (MB)
    pub memory_usage: u64,
    /// CPU usage percentage
    pub cpu_usage: f64,
}

/// Unicoin Node
#[derive(Debug)]
pub struct UnicoinNode {
    /// Configuration
    config: UnicoinConfig,
    /// Blockchain instance
    blockchain: Arc<Blockchain>,
    /// UTXO manager
    utxo_manager: Arc<UTXOManager>,
    /// Transaction processor
    transaction_processor: Arc<TransactionProcessor>,
    /// Consensus engine
    consensus_engine: Arc<ConsensusEngine>,
    /// P2P network
    p2p_network: Arc<P2PNetwork>,
    /// API server
    api_server: Arc<ApiServer>,
    /// Node status
    status: Arc<RwLock<NodeStatus>>,
    /// Node statistics
    stats: Arc<RwLock<NodeStats>>,
    /// Start time
    start_time: std::time::Instant,
}

impl UnicoinNode {
    /// Create a new Unicoin node
    pub async fn new(config: UnicoinConfig) -> Result<Self> {
        info!("🚀 Initializing Unicoin Node v{}", crate::VERSION);

        // Initialize blockchain
        let blockchain = Arc::new(Blockchain::new()?);
        info!("✅ Blockchain initialized");

        // Initialize UTXO manager
        let utxo_manager = Arc::new(UTXOManager::new());
        info!("✅ UTXO manager initialized");

        // Initialize consensus engine
        let consensus_engine = Arc::new(ConsensusEngine::new(blockchain.clone())?);
        info!("✅ Consensus engine initialized");

        // Initialize transaction processor
        let transaction_processor = Arc::new(TransactionProcessor::new(
            utxo_manager.clone(),
            consensus_engine.clone(),
        ));
        info!("✅ Transaction processor initialized");

        // Initialize P2P network
        let node_id = format!("unicoin-{}", uuid::Uuid::new_v4());
        let listen_addr = format!("{}:{}", config.network.listen_address, config.network.listen_port)
            .parse()
            .map_err(|e| UnicoinError::Network(format!("Invalid listen address: {}", e)))?;
        
        let p2p_network = Arc::new(P2PNetwork::new(
            listen_addr,
            node_id,
            config.network.network_id,
        ));
        info!("✅ P2P network initialized");

        // Initialize API server
        let api_server = Arc::new(ApiServer::new(config.api.clone()));
        info!("✅ API server initialized");

        let start_time = std::time::Instant::now();

        Ok(Self {
            config,
            blockchain,
            utxo_manager,
            transaction_processor,
            consensus_engine,
            p2p_network,
            api_server,
            status: Arc::new(RwLock::new(NodeStatus::Starting)),
            stats: Arc::new(RwLock::new(NodeStats {
                status: NodeStatus::Starting,
                blockchain_height: 0,
                connected_peers: 0,
                total_transactions: 0,
                total_blocks: 0,
                uptime: 0,
                memory_usage: 0,
                cpu_usage: 0.0,
            })),
            start_time,
        })
    }

    /// Start the node
    pub async fn start(&self) -> Result<()> {
        info!("🎯 Starting Unicoin Node...");

        // Update status
        {
            let mut status = self.status.write().await;
            *status = NodeStatus::Starting;
        }

        // Start P2P network
        self.p2p_network.start().await?;
        info!("🌐 P2P network started");

        // Start consensus engine
        self.consensus_engine.start().await?;
        info!("⚡ Consensus engine started");

        // Start API server
        self.api_server.start().await?;
        info!("🔌 API server started");

        // Update status to running
        {
            let mut status = self.status.write().await;
            *status = NodeStatus::Running;
        }

        info!("🎉 Unicoin Node is now running!");

        // Start background tasks
        self.start_background_tasks().await?;

        Ok(())
    }

    /// Start background tasks
    async fn start_background_tasks(&self) -> Result<()> {
        // Statistics updater
        let stats = self.stats.clone();
        let start_time = self.start_time;
        let status = self.status.clone();
        let p2p_network = self.p2p_network.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            loop {
                interval.tick().await;

                let current_status = status.read().await.clone();
                let network_stats = p2p_network.get_stats();
                let uptime = start_time.elapsed().as_secs();

                let mut stats = stats.write().await;
                stats.status = current_status;
                stats.connected_peers = network_stats.connected_peers;
                stats.uptime = uptime;
                stats.memory_usage = Self::get_memory_usage();
                stats.cpu_usage = Self::get_cpu_usage();
            }
        });

        Ok(())
    }

    /// Stop the node
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Unicoin Node...");

        // Update status
        {
            let mut status = self.status.write().await;
            *status = NodeStatus::Stopping;
        }

        // Stop API server
        // Note: ApiServer doesn't have a stop method yet, but it should be graceful

        // Stop P2P network
        // Note: P2PNetwork doesn't have a stop method yet, but it should be graceful

        // Update status
        {
            let mut status = self.status.write().await;
            *status = NodeStatus::Stopped;
        }

        info!("✅ Unicoin Node stopped");
        Ok(())
    }

    /// Send a transaction
    pub async fn send_transaction(&self, transaction: Transaction) -> Result<TransactionResult> {
        // Process transaction
        let result = self.transaction_processor.process_transaction(transaction.clone()).await?;

        // Broadcast to network if successful
        if result.status == TransactionStatus::Pending {
            self.p2p_network.broadcast_transaction(transaction).await?;
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_transactions += 1;
        }

        Ok(result)
    }

    /// Create and send a transaction
    pub async fn create_and_send_transaction(
        &self,
        sender_address: Vec<u8>,
        recipient_address: Vec<u8>,
        amount: u64,
        fee_per_byte: Option<u64>,
        memo: Option<String>,
        private_key: &PrivateKey,
    ) -> Result<TransactionResult> {
        // Create transaction
        let transaction = self.transaction_processor.create_transaction(
            sender_address,
            recipient_address,
            amount,
            fee_per_byte,
            memo,
            CoinSelectionAlgorithm::MinimizeInputs,
        ).await?;

        // Sign transaction
        let signed_transaction = self.transaction_processor.sign_transaction(
            transaction,
            private_key,
        ).await?;

        // Send transaction
        self.send_transaction(signed_transaction).await
    }

    /// Get balance for an address
    pub async fn get_balance(&self, address: &[u8], include_unconfirmed: bool) -> Result<crate::blockchain::AddressBalance> {
        self.utxo_manager.get_balance(address, include_unconfirmed)
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, tx_hash: &Hash) -> Result<TransactionResult> {
        self.transaction_processor.get_transaction_status(tx_hash).await
    }

    /// Get node status
    pub async fn get_status(&self) -> NodeStatus {
        self.status.read().await.clone()
    }

    /// Get node statistics
    pub async fn get_stats(&self) -> NodeStats {
        self.stats.read().await.clone()
    }

    /// Get connected peers
    pub fn get_connected_peers(&self) -> Vec<crate::network::P2PPeerInfo> {
        self.p2p_network.get_connected_peers()
    }

    /// Get blockchain height
    pub async fn get_blockchain_height(&self) -> Result<u64> {
        self.utxo_manager.get_current_height()
    }

    /// Get UTXO manager reference
    pub fn utxo_manager(&self) -> Arc<UTXOManager> {
        self.utxo_manager.clone()
    }

    /// Get transaction processor reference
    pub fn transaction_processor(&self) -> Arc<TransactionProcessor> {
        self.transaction_processor.clone()
    }

    /// Get P2P network reference
    pub fn p2p_network(&self) -> Arc<P2PNetwork> {
        self.p2p_network.clone()
    }

    /// Get memory usage (placeholder implementation)
    fn get_memory_usage() -> u64 {
        // In a real implementation, this would use system APIs to get actual memory usage
        512 // Placeholder: 512 MB
    }

    /// Get CPU usage (placeholder implementation)
    fn get_cpu_usage() -> f64 {
        // In a real implementation, this would use system APIs to get actual CPU usage
        15.0 // Placeholder: 15%
    }

    /// Wait for shutdown signal
    pub async fn wait_for_shutdown(&self) -> Result<()> {
        tokio::signal::ctrl_c().await?;
        info!("🛑 Shutdown signal received");
        self.stop().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_creation() {
        let config = UnicoinConfig::new();
        let node = UnicoinNode::new(config).await;
        assert!(node.is_ok());
    }

    #[tokio::test]
    async fn test_node_status() {
        let config = UnicoinConfig::new();
        let node = UnicoinNode::new(config).await.unwrap();
        
        let status = node.get_status().await;
        assert_eq!(status, NodeStatus::Starting);
    }

    #[tokio::test]
    async fn test_node_stats() {
        let config = UnicoinConfig::new();
        let node = UnicoinNode::new(config).await.unwrap();
        
        let stats = node.get_stats().await;
        assert_eq!(stats.status, NodeStatus::Starting);
        assert_eq!(stats.total_transactions, 0);
    }
}
