//! Peer discovery implementation
//!
//! This module provides peer discovery functionality for finding other Unicoin nodes.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Peer discovery service
pub struct PeerDiscovery {
    /// Known peers
    known_peers: RwLock<HashMap<String, DiscoveredPeer>>,
    /// Bootstrap peers
    bootstrap_peers: Vec<BootstrapPeer>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

/// Discovered peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer public key
    pub public_key: crate::crypto::PublicKey,
    /// Discovery timestamp
    pub discovered_at: u64,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Peer reputation
    pub reputation: i32,
}

/// Bootstrap peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapPeer {
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer public key
    pub public_key: crate::crypto::PublicKey,
}

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Maximum number of peers to discover
    pub max_peers: usize,
    /// Discovery timeout in seconds
    pub discovery_timeout: u64,
    /// Peer refresh interval in seconds
    pub refresh_interval: u64,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable DHT discovery
    pub enable_dht: bool,
}

impl PeerDiscovery {
    /// Create a new peer discovery service
    pub fn new() -> Self {
        Self {
            known_peers: RwLock::new(HashMap::new()),
            bootstrap_peers: Vec::new(),
            config: DiscoveryConfig::default(),
        }
    }

    /// Create a new peer discovery service with configuration
    pub fn with_config(config: DiscoveryConfig) -> Self {
        Self {
            known_peers: RwLock::new(HashMap::new()),
            bootstrap_peers: Vec::new(),
            config,
        }
    }

    /// Add a bootstrap peer
    pub fn add_bootstrap_peer(&mut self, peer: BootstrapPeer) {
        self.bootstrap_peers.push(peer);
    }

    /// Start peer discovery
    pub async fn start(&self) -> Result<()> {
        // Start mDNS discovery if enabled
        if self.config.enable_mdns {
            self.start_mdns_discovery().await?;
        }

        // Start DHT discovery if enabled
        if self.config.enable_dht {
            self.start_dht_discovery().await?;
        }

        // Start bootstrap peer discovery
        self.discover_bootstrap_peers().await?;

        Ok(())
    }

    /// Discover peers using mDNS
    async fn start_mdns_discovery(&self) -> Result<()> {
        // Placeholder implementation for mDNS discovery
        // In a real implementation, this would use libp2p's mDNS functionality
        tracing::info!("Starting mDNS peer discovery");
        Ok(())
    }

    /// Discover peers using DHT
    async fn start_dht_discovery(&self) -> Result<()> {
        // Placeholder implementation for DHT discovery
        // In a real implementation, this would use libp2p's Kademlia DHT
        tracing::info!("Starting DHT peer discovery");
        Ok(())
    }

    /// Discover bootstrap peers
    async fn discover_bootstrap_peers(&self) -> Result<()> {
        tracing::info!("Discovering bootstrap peers");
        
        for bootstrap_peer in &self.bootstrap_peers {
            // Attempt to connect to bootstrap peer
            if self.connect_to_bootstrap_peer(bootstrap_peer).await? {
                self.add_discovered_peer(DiscoveredPeer {
                    address: bootstrap_peer.address.clone(),
                    port: bootstrap_peer.port,
                    public_key: bootstrap_peer.public_key.clone(),
                    discovered_at: crate::utils::timestamp(),
                    last_seen: crate::utils::timestamp(),
                    reputation: 100, // Bootstrap peers start with high reputation
                }).await?;
            }
        }

        Ok(())
    }

    /// Attempt to connect to a bootstrap peer
    async fn connect_to_bootstrap_peer(&self, peer: &BootstrapPeer) -> Result<bool> {
        // Placeholder implementation
        // In a real implementation, this would attempt to establish a connection
        tracing::info!("Attempting to connect to bootstrap peer: {}:{}", peer.address, peer.port);
        Ok(true) // Simulate successful connection
    }

    /// Add a discovered peer
    async fn add_discovered_peer(&self, peer: DiscoveredPeer) -> Result<()> {
        let mut known_peers = self.known_peers.write().await;
        
        if known_peers.len() >= self.config.max_peers {
            return Err(UnicoinError::Network("Maximum peers reached".to_string()));
        }

        let peer_key = format!("{}:{}", peer.address, peer.port);
        known_peers.insert(peer_key, peer);
        
        Ok(())
    }

    /// Get discovered peers
    pub async fn get_discovered_peers(&self) -> Vec<DiscoveredPeer> {
        let known_peers = self.known_peers.read().await;
        known_peers.values().cloned().collect()
    }

    /// Get active peers (seen recently)
    pub async fn get_active_peers(&self, timeout_seconds: u64) -> Vec<DiscoveredPeer> {
        let known_peers = self.known_peers.read().await;
        let current_time = crate::utils::timestamp();
        
        known_peers.values()
            .filter(|peer| current_time - peer.last_seen <= timeout_seconds)
            .cloned()
            .collect()
    }

    /// Update peer last seen timestamp
    pub async fn update_peer_last_seen(&self, address: &str, port: u16) -> Result<()> {
        let mut known_peers = self.known_peers.write().await;
        let peer_key = format!("{}:{}", address, port);
        
        if let Some(peer) = known_peers.get_mut(&peer_key) {
            peer.last_seen = crate::utils::timestamp();
            Ok(())
        } else {
            Err(UnicoinError::Network("Peer not found".to_string()))
        }
    }

    /// Remove inactive peers
    pub async fn cleanup_inactive_peers(&self, timeout_seconds: u64) -> usize {
        let mut known_peers = self.known_peers.write().await;
        let current_time = crate::utils::timestamp();
        let mut to_remove = Vec::new();

        for (peer_key, peer) in known_peers.iter() {
            if current_time - peer.last_seen > timeout_seconds {
                to_remove.push(peer_key.clone());
            }
        }

        let removed_count = to_remove.len();
        for peer_key in to_remove {
            known_peers.remove(&peer_key);
        }

        removed_count
    }

    /// Get discovery statistics
    pub async fn get_discovery_statistics(&self) -> DiscoveryStatistics {
        let known_peers = self.known_peers.read().await;
        let total_peers = known_peers.len();
        let active_peers = known_peers.values()
            .filter(|peer| crate::utils::timestamp() - peer.last_seen <= 300) // 5 minutes
            .count();

        DiscoveryStatistics {
            total_discovered_peers: total_peers,
            active_peers,
            bootstrap_peers: self.bootstrap_peers.len(),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            max_peers: 1000,
            discovery_timeout: 30,
            refresh_interval: 300, // 5 minutes
            enable_mdns: true,
            enable_dht: true,
        }
    }
}

/// Discovery statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStatistics {
    pub total_discovered_peers: usize,
    pub active_peers: usize,
    pub bootstrap_peers: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_discovery_creation() {
        let discovery = PeerDiscovery::new();
        let stats = discovery.get_discovery_statistics().await;
        assert_eq!(stats.total_discovered_peers, 0);
        assert_eq!(stats.active_peers, 0);
    }

    #[tokio::test]
    async fn test_bootstrap_peer_management() {
        let mut discovery = PeerDiscovery::new();
        
        let bootstrap_peer = BootstrapPeer {
            address: "127.0.0.1".to_string(),
            port: 30303,
            public_key: crate::crypto::PublicKey::random(),
        };
        
        discovery.add_bootstrap_peer(bootstrap_peer);
        let stats = discovery.get_discovery_statistics().await;
        assert_eq!(stats.bootstrap_peers, 1);
    }

    #[tokio::test]
    async fn test_peer_cleanup() {
        let discovery = PeerDiscovery::new();
        
        // Add a peer that will be considered inactive
        let peer = DiscoveredPeer {
            address: "127.0.0.1".to_string(),
            port: 30303,
            public_key: crate::crypto::PublicKey::random(),
            discovered_at: crate::utils::timestamp(),
            last_seen: crate::utils::timestamp() - 1000, // 1000 seconds ago
            reputation: 100,
        };
        
        discovery.add_discovered_peer(peer).await.unwrap();
        
        // Cleanup inactive peers (timeout = 600 seconds)
        let removed = discovery.cleanup_inactive_peers(600).await;
        assert_eq!(removed, 1);
        
        let stats = discovery.get_discovery_statistics().await;
        assert_eq!(stats.total_discovered_peers, 0);
    }
}
