//! Peer management implementation
//!
//! This module provides peer discovery and management functionality.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: String,
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer public key
    pub public_key: crate::crypto::PublicKey,
    /// Connection status
    pub status: PeerStatus,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Connection latency in milliseconds
    pub latency: u64,
    /// Peer reputation score
    pub reputation: i32,
}

/// Peer connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerStatus {
    /// Connected and active
    Connected,
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connection failed
    Failed,
}

/// Peer manager
pub struct PeerManager {
    /// Map of peer IDs to peer info
    peers: RwLock<HashMap<String, PeerInfo>>,
    /// Maximum number of peers
    max_peers: usize,
}

impl PeerInfo {
    /// Create new peer info
    pub fn new(
        peer_id: String,
        address: String,
        port: u16,
        public_key: crate::crypto::PublicKey,
    ) -> Self {
        Self {
            peer_id,
            address,
            port,
            public_key,
            status: PeerStatus::Disconnected,
            last_seen: crate::utils::timestamp(),
            latency: 0,
            reputation: 100, // Start with neutral reputation
        }
    }

    /// Update peer activity
    pub fn update_activity(&mut self) {
        self.last_seen = crate::utils::timestamp();
    }

    /// Check if peer is active
    pub fn is_active(&self) -> bool {
        self.status == PeerStatus::Connected
    }

    /// Check if peer is trusted
    pub fn is_trusted(&self) -> bool {
        self.reputation > 50
    }

    /// Update peer reputation
    pub fn update_reputation(&mut self, change: i32) {
        self.reputation = (self.reputation + change).max(0).min(200);
    }
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new() -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
            max_peers: 100,
        }
    }

    /// Create a new peer manager with maximum peer limit
    pub fn with_max_peers(max_peers: usize) -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
            max_peers,
        }
    }

    /// Add a peer to the manager
    pub async fn add_peer(&self, peer_id: String) -> Result<()> {
        let mut peers = self.peers.write().await;
        
        if peers.len() >= self.max_peers {
            return Err(UnicoinError::Network("Maximum peers reached".to_string()));
        }

        if peers.contains_key(&peer_id) {
            return Err(UnicoinError::Network("Peer already exists".to_string()));
        }

        // Create basic peer info
        let peer_info = PeerInfo::new(
            peer_id.clone(),
            "127.0.0.1".to_string(), // Placeholder
            30303,                   // Placeholder
            crate::crypto::PublicKey::random(), // Placeholder
        );

        peers.insert(peer_id, peer_info);
        Ok(())
    }

    /// Remove a peer from the manager
    pub async fn remove_peer(&self, peer_id: String) -> Result<()> {
        let mut peers = self.peers.write().await;
        peers.remove(&peer_id)
            .ok_or_else(|| UnicoinError::Network("Peer not found".to_string()))?;
        Ok(())
    }

    /// Get peer information
    pub async fn get_peer_info(&self, peer_id: String) -> Option<PeerInfo> {
        let peers = self.peers.read().await;
        peers.get(&peer_id).cloned()
    }

    /// Update peer status
    pub async fn update_peer_status(&self, peer_id: String, status: PeerStatus) -> Result<()> {
        let mut peers = self.peers.write().await;
        if let Some(peer_info) = peers.get_mut(&peer_id) {
            peer_info.status = status;
            peer_info.update_activity();
            Ok(())
        } else {
            Err(UnicoinError::Network("Peer not found".to_string()))
        }
    }

    /// Update peer latency
    pub async fn update_peer_latency(&self, peer_id: String, latency: u64) -> Result<()> {
        let mut peers = self.peers.write().await;
        if let Some(peer_info) = peers.get_mut(&peer_id) {
            peer_info.latency = latency;
            peer_info.update_activity();
            Ok(())
        } else {
            Err(UnicoinError::Network("Peer not found".to_string()))
        }
    }

    /// Update peer reputation
    pub async fn update_peer_reputation(&self, peer_id: String, change: i32) -> Result<()> {
        let mut peers = self.peers.write().await;
        if let Some(peer_info) = peers.get_mut(&peer_id) {
            peer_info.update_reputation(change);
            Ok(())
        } else {
            Err(UnicoinError::Network("Peer not found".to_string()))
        }
    }

    /// Get all peers
    pub async fn get_all_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get active peers only
    pub async fn get_active_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values()
            .filter(|peer| peer.is_active())
            .cloned()
            .collect()
    }

    /// Get trusted peers only
    pub async fn get_trusted_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values()
            .filter(|peer| peer.is_trusted())
            .cloned()
            .collect()
    }

    /// Get peer count
    pub async fn get_peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }

    /// Get active peer count
    pub async fn get_active_peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.values().filter(|peer| peer.is_active()).count()
    }

    /// Check if peer exists
    pub async fn has_peer(&self, peer_id: &str) -> bool {
        let peers = self.peers.read().await;
        peers.contains_key(peer_id)
    }

    /// Clean up inactive peers
    pub async fn cleanup_inactive_peers(&self, timeout_seconds: u64) -> usize {
        let mut peers = self.peers.write().await;
        let current_time = crate::utils::timestamp();
        let mut to_remove = Vec::new();

        for (peer_id, peer_info) in peers.iter() {
            if current_time - peer_info.last_seen > timeout_seconds {
                to_remove.push(peer_id.clone());
            }
        }

        let removed_count = to_remove.len();
        for peer_id in to_remove {
            peers.remove(&peer_id);
        }

        removed_count
    }

    /// Get peer statistics
    pub async fn get_peer_statistics(&self) -> PeerStatistics {
        let peers = self.peers.read().await;
        let total_peers = peers.len();
        let active_peers = peers.values().filter(|p| p.is_active()).count();
        let trusted_peers = peers.values().filter(|p| p.is_trusted()).count();
        let avg_latency = if total_peers > 0 {
            peers.values().map(|p| p.latency).sum::<u64>() / total_peers as u64
        } else {
            0
        };

        PeerStatistics {
            total_peers,
            active_peers,
            trusted_peers,
            avg_latency,
        }
    }
}

/// Peer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatistics {
    pub total_peers: usize,
    pub active_peers: usize,
    pub trusted_peers: usize,
    pub avg_latency: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_manager_creation() {
        let manager = PeerManager::new();
        assert_eq!(manager.get_peer_count().await, 0);
        assert_eq!(manager.get_active_peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_peer_management() {
        let manager = PeerManager::new();
        let peer_id = "test_peer".to_string();

        // Add peer
        manager.add_peer(peer_id.clone()).await.unwrap();
        assert_eq!(manager.get_peer_count().await, 1);
        assert!(manager.has_peer(&peer_id).await);

        // Update peer status
        manager.update_peer_status(peer_id.clone(), PeerStatus::Connected).await.unwrap();
        assert_eq!(manager.get_active_peer_count().await, 1);

        // Remove peer
        manager.remove_peer(peer_id.clone()).await.unwrap();
        assert_eq!(manager.get_peer_count().await, 0);
        assert!(!manager.has_peer(&peer_id).await);
    }

    #[tokio::test]
    async fn test_peer_reputation() {
        let manager = PeerManager::new();
        let peer_id = "test_peer".to_string();

        manager.add_peer(peer_id.clone()).await.unwrap();

        // Update reputation
        manager.update_peer_reputation(peer_id.clone(), -20).await.unwrap();
        
        let peer_info = manager.get_peer_info(peer_id).await.unwrap();
        assert_eq!(peer_info.reputation, 80);
        assert!(peer_info.is_trusted()); // Still trusted

        manager.update_peer_reputation(peer_id.clone(), -40).await.unwrap();
        let peer_info = manager.get_peer_info(peer_id).await.unwrap();
        assert_eq!(peer_info.reputation, 40);
        assert!(!peer_info.is_trusted()); // No longer trusted
    }
}
