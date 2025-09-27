//! Peer-to-Peer Network Implementation
//!
//! This module provides the core P2P networking functionality for Unicoin nodes,
//! including peer discovery, message routing, and protocol implementation.

use crate::{
    blockchain::{Block, Transaction},
    crypto::Hash,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Handshake message
    Handshake(HandshakeMessage),
    /// Peer list request
    GetPeers,
    /// Peer list response
    Peers(Vec<PeerInfo>),
    /// New transaction
    Transaction(Transaction),
    /// New block
    Block(Block),
    /// Block request
    GetBlocks(GetBlocksMessage),
    /// Block response
    Blocks(Vec<Block>),
    /// Ping message
    Ping(PingMessage),
    /// Pong response
    Pong(PongMessage),
    /// Version message
    Version(VersionMessage),
    /// Verack message
    VerAck,
}

/// Handshake message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    /// Protocol version
    pub version: u32,
    /// Node ID
    pub node_id: String,
    /// Network ID
    pub network_id: u32,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Timestamp
    pub timestamp: u64,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeerInfo {
    /// Peer address
    pub address: SocketAddr,
    /// Peer ID
    pub peer_id: String,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Connection status
    pub status: PeerStatus,
    /// Peer capabilities
    pub capabilities: Vec<String>,
    /// Peer version
    pub version: String,
}

/// Peer connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PeerStatus {
    /// Peer is connected
    Connected,
    /// Peer is disconnected
    Disconnected,
    /// Peer is banned
    Banned,
    /// Peer connection is pending
    Pending,
}

/// Get blocks message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlocksMessage {
    /// Starting block hash
    pub start_hash: Hash,
    /// Ending block hash
    pub end_hash: Hash,
    /// Maximum number of blocks to return
    pub max_blocks: u32,
}

/// Ping message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    /// Nonce for ping response
    pub nonce: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Pong message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    /// Nonce from ping
    pub nonce: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Version message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMessage {
    /// Protocol version
    pub version: u32,
    /// Services supported
    pub services: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Remote address
    pub addr_recv: SocketAddr,
    /// Local address
    pub addr_from: SocketAddr,
    /// Random nonce
    pub nonce: u64,
    /// User agent
    pub user_agent: String,
    /// Block height
    pub start_height: u64,
}

/// Network statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub connected_peers: usize,
    pub total_peers: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub uptime: Duration,
}

/// Peer-to-peer network manager
#[derive(Debug)]
pub struct P2PNetwork {
    /// Local node ID
    node_id: String,
    /// Listening address
    listen_addr: SocketAddr,
    /// Connected peers
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    /// Known peers
    known_peers: Arc<RwLock<HashSet<SocketAddr>>>,
    /// Message channels
    message_tx: mpsc::UnboundedSender<NetworkMessage>,
    message_rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<NetworkMessage>>>>,
    /// Network statistics
    stats: Arc<RwLock<NetworkStats>>,
    /// Start time for uptime calculation
    start_time: Instant,
    /// Protocol version
    protocol_version: u32,
    /// Network ID
    network_id: u32,
}

impl P2PNetwork {
    /// Create a new P2P network
    pub fn new(listen_addr: SocketAddr, node_id: String, network_id: u32) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        
        Self {
            node_id,
            listen_addr,
            peers: Arc::new(RwLock::new(HashMap::new())),
            known_peers: Arc::new(RwLock::new(HashSet::new())),
            message_tx,
            message_rx: Arc::new(RwLock::new(Some(message_rx))),
            stats: Arc::new(RwLock::new(NetworkStats::default())),
            start_time: Instant::now(),
            protocol_version: 1,
            network_id,
        }
    }

    /// Start the P2P network
    pub async fn start(&self) -> Result<()> {
        // Start listening for connections
        self.start_listener().await?;
        
        // Start peer discovery
        self.start_peer_discovery().await?;
        
        // Start message processing
        self.start_message_processor().await?;
        
        // Start periodic tasks
        self.start_periodic_tasks().await?;

        Ok(())
    }

    /// Start listening for incoming connections
    async fn start_listener(&self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(self.listen_addr).await
            .map_err(|e| UnicoinError::Network(format!("Failed to bind to {}: {}", self.listen_addr, e)))?;

        println!("🌐 P2P Network listening on {}", self.listen_addr);

        let peers = self.peers.clone();
        let message_tx = self.message_tx.clone();
        let node_id = self.node_id.clone();
        let protocol_version = self.protocol_version;
        let network_id = self.network_id;

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                println!("📡 New connection from {}", addr);
                
                // Handle new connection
                let peer_info = PeerInfo {
                    address: addr,
                    peer_id: format!("peer_{}", addr),
                    last_seen: crate::utils::timestamp(),
                    status: PeerStatus::Connected,
                    capabilities: vec!["full_node".to_string()],
                    version: format!("unicoin/{}", crate::VERSION),
                };

                // Add to peers
                {
                    let mut peers = peers.write().unwrap();
                    peers.insert(peer_info.peer_id.clone(), peer_info.clone());
                }

                // Send handshake
                let handshake = NetworkMessage::Handshake(HandshakeMessage {
                    version: protocol_version,
                    node_id: node_id.clone(),
                    network_id,
                    capabilities: vec!["full_node".to_string()],
                    timestamp: crate::utils::timestamp(),
                });

                if let Err(e) = message_tx.send(handshake) {
                    eprintln!("Failed to send handshake: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Start peer discovery
    async fn start_peer_discovery(&self) -> Result<()> {
        // Add some bootstrap peers
        let bootstrap_peers = vec![
            "127.0.0.1:30301".parse::<SocketAddr>().unwrap(),
            "127.0.0.1:30302".parse::<SocketAddr>().unwrap(),
            "127.0.0.1:30303".parse::<SocketAddr>().unwrap(),
        ];

        let known_peers = self.known_peers.clone();
        {
            let mut known_peers = known_peers.write().unwrap();
            for peer in bootstrap_peers {
                known_peers.insert(peer);
            }
        }

        Ok(())
    }

    /// Start message processor
    async fn start_message_processor(&self) -> Result<()> {
        let peers = self.peers.clone();
        let stats = self.stats.clone();

        // Take the receiver from the RwLock
        let message_rx = {
            let mut rx_guard = self.message_rx.write().unwrap();
            rx_guard.take()
        };

        if let Some(mut message_rx) = message_rx {
            tokio::spawn(async move {
                while let Some(message) = message_rx.recv().await {
                    // Update statistics
                    {
                        let mut stats = stats.write().unwrap();
                        stats.messages_received += 1;
                    }

                    // Process message
                    match message {
                        NetworkMessage::Handshake(handshake) => {
                            println!("🤝 Received handshake from node: {}", handshake.node_id);
                        }
                        NetworkMessage::GetPeers => {
                            println!("📋 Peer list requested");
                        }
                        NetworkMessage::Peers(peer_list) => {
                            println!("📋 Received {} peers", peer_list.len());
                        }
                        NetworkMessage::Transaction(tx) => {
                            println!("💸 Received transaction: {}", tx.hash());
                        }
                        NetworkMessage::Block(block) => {
                            println!("📦 Received block: {}", block.header.height);
                        }
                        NetworkMessage::GetBlocks(get_blocks) => {
                            println!("📦 Block request from {} to {}", 
                                get_blocks.start_hash, get_blocks.end_hash);
                        }
                        NetworkMessage::Blocks(blocks) => {
                            println!("📦 Received {} blocks", blocks.len());
                        }
                        NetworkMessage::Ping(ping) => {
                            println!("🏓 Ping received (nonce: {})", ping.nonce);
                        }
                        NetworkMessage::Pong(pong) => {
                            println!("🏓 Pong received (nonce: {})", pong.nonce);
                        }
                        NetworkMessage::Version(version) => {
                            println!("📢 Version message: {}", version.user_agent);
                        }
                        NetworkMessage::VerAck => {
                            println!("✅ Version acknowledgment received");
                        }
                    }
                }
            });
        }

        Ok(())
    }

    /// Start periodic tasks
    async fn start_periodic_tasks(&self) -> Result<()> {
        let peers = self.peers.clone();
        let stats = self.stats.clone();
        let start_time = self.start_time;

        // Ping peers every 30 seconds
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                
                // Send ping to all connected peers
                let peers_to_ping = {
                    let peers = peers.read().unwrap();
                    peers.values()
                        .filter(|peer| peer.status == PeerStatus::Connected)
                        .cloned()
                        .collect::<Vec<_>>()
                };

                for peer in peers_to_ping {
                    println!("🏓 Pinging peer: {}", peer.address);
                    // In a real implementation, this would send a ping message
                }
            }
        });

        // Update statistics every minute
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                
                let mut stats = stats.write().unwrap();
                stats.uptime = start_time.elapsed();
                
                println!("📊 Network Stats: {} peers, {} messages received", 
                    stats.connected_peers, stats.messages_received);
            }
        });

        Ok(())
    }

    /// Connect to a peer
    pub async fn connect_peer(&self, addr: SocketAddr) -> Result<()> {
        println!("🔗 Connecting to peer: {}", addr);
        
        // Add to known peers
        {
            let mut known_peers = self.known_peers.write().unwrap();
            known_peers.insert(addr);
        }

        // In a real implementation, this would establish a TCP connection
        // and perform the handshake protocol

        Ok(())
    }

    /// Disconnect from a peer
    pub async fn disconnect_peer(&self, peer_id: &str) -> Result<()> {
        println!("🔌 Disconnecting peer: {}", peer_id);
        
        let mut peers = self.peers.write().unwrap();
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.status = PeerStatus::Disconnected;
        }

        Ok(())
    }

    /// Broadcast a message to all connected peers
    pub async fn broadcast_message(&self, message: NetworkMessage) -> Result<()> {
        let peers = self.peers.read().unwrap();
        let connected_peers: Vec<_> = peers.values()
            .filter(|peer| peer.status == PeerStatus::Connected)
            .collect();

        println!("📡 Broadcasting message to {} peers", connected_peers.len());

        // In a real implementation, this would send the message to each peer
        for peer in connected_peers {
            println!("  → Sending to {}", peer.address);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.messages_sent += 1;
        }

        Ok(())
    }

    /// Send a transaction to the network
    pub async fn broadcast_transaction(&self, transaction: Transaction) -> Result<()> {
        println!("💸 Broadcasting transaction: {}", transaction.hash());
        
        let message = NetworkMessage::Transaction(transaction);
        self.broadcast_message(message).await?;

        Ok(())
    }

    /// Send a block to the network
    pub async fn broadcast_block(&self, block: Block) -> Result<()> {
        println!("📦 Broadcasting block: {}", block.header.height);
        
        let message = NetworkMessage::Block(block);
        self.broadcast_message(message).await?;

        Ok(())
    }

    /// Get connected peers
    pub fn get_connected_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().unwrap();
        peers.values()
            .filter(|peer| peer.status == PeerStatus::Connected)
            .cloned()
            .collect()
    }

    /// Get all known peers
    pub fn get_known_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().unwrap();
        peers.values().cloned().collect()
    }

    /// Get network statistics
    pub fn get_stats(&self) -> NetworkStats {
        let stats = self.stats.read().unwrap();
        let peers = self.peers.read().unwrap();
        
        NetworkStats {
            connected_peers: peers.values()
                .filter(|peer| peer.status == PeerStatus::Connected)
                .count(),
            total_peers: peers.len(),
            messages_sent: stats.messages_sent,
            messages_received: stats.messages_received,
            bytes_sent: stats.bytes_sent,
            bytes_received: stats.bytes_received,
            uptime: start_time.elapsed(),
        }
    }

    /// Get message sender for broadcasting
    pub fn message_sender(&self) -> mpsc::UnboundedSender<NetworkMessage> {
        self.message_tx.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_p2p_network_creation() {
        let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let network = P2PNetwork::new(addr, "test_node".to_string(), 1);
        
        assert_eq!(network.node_id, "test_node");
        assert_eq!(network.network_id, 1);
    }

    #[tokio::test]
    async fn test_peer_connection() {
        let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let network = P2PNetwork::new(addr, "test_node".to_string(), 1);
        
        let peer_addr = "127.0.0.1:30301".parse::<SocketAddr>().unwrap();
        let result = network.connect_peer(peer_addr).await;
        assert!(result.is_ok());
        
        let known_peers = network.get_known_peers();
        assert!(known_peers.iter().any(|peer| peer.address == peer_addr));
    }

    #[test]
    fn test_network_message_serialization() {
        let handshake = HandshakeMessage {
            version: 1,
            node_id: "test_node".to_string(),
            network_id: 1,
            capabilities: vec!["full_node".to_string()],
            timestamp: crate::utils::timestamp(),
        };

        let message = NetworkMessage::Handshake(handshake);
        let serialized = bincode::serialize(&message).unwrap();
        let deserialized: NetworkMessage = bincode::deserialize(&serialized).unwrap();

        match deserialized {
            NetworkMessage::Handshake(hs) => {
                assert_eq!(hs.version, 1);
                assert_eq!(hs.node_id, "test_node");
            }
            _ => panic!("Expected handshake message"),
        }
    }
}
