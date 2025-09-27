//! Message routing implementation
//!
//! This module provides message routing functionality for Unicoin network communication.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Message routing service
pub struct MessageRouting {
    /// Routing table
    routing_table: RwLock<HashMap<String, RouteEntry>>,
    /// Message cache
    message_cache: RwLock<HashMap<String, CachedMessage>>,
    /// Routing configuration
    config: RoutingConfig,
}

/// Route entry in the routing table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEntry {
    /// Destination address
    pub destination: String,
    /// Next hop address
    pub next_hop: String,
    /// Route cost (lower is better)
    pub cost: u32,
    /// Route creation timestamp
    pub created_at: u64,
    /// Last used timestamp
    pub last_used: u64,
    /// Route status
    pub status: RouteStatus,
}

/// Route status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteStatus {
    /// Route is active
    Active,
    /// Route is inactive
    Inactive,
    /// Route failed
    Failed,
}

/// Cached message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMessage {
    /// Message content
    pub content: Vec<u8>,
    /// Message hash
    pub hash: crate::crypto::Hash,
    /// Creation timestamp
    pub created_at: u64,
    /// TTL in seconds
    pub ttl: u64,
}

/// Routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Maximum routing table size
    pub max_routes: usize,
    /// Message cache size
    pub cache_size: usize,
    /// Default message TTL in seconds
    pub default_ttl: u64,
    /// Route timeout in seconds
    pub route_timeout: u64,
    /// Enable message caching
    pub enable_caching: bool,
}

impl MessageRouting {
    /// Create a new message routing service
    pub fn new() -> Self {
        Self {
            routing_table: RwLock::new(HashMap::new()),
            message_cache: RwLock::new(HashMap::new()),
            config: RoutingConfig::default(),
        }
    }

    /// Create a new message routing service with configuration
    pub fn with_config(config: RoutingConfig) -> Self {
        Self {
            routing_table: RwLock::new(HashMap::new()),
            message_cache: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Add a route to the routing table
    pub async fn add_route(&self, destination: String, next_hop: String, cost: u32) -> Result<()> {
        let mut routing_table = self.routing_table.write().await;
        
        if routing_table.len() >= self.config.max_routes {
            return Err(UnicoinError::Network("Maximum routes reached".to_string()));
        }

        let route_entry = RouteEntry {
            destination: destination.clone(),
            next_hop,
            cost,
            created_at: crate::utils::timestamp(),
            last_used: 0,
            status: RouteStatus::Active,
        };

        routing_table.insert(destination, route_entry);
        Ok(())
    }

    /// Remove a route from the routing table
    pub async fn remove_route(&self, destination: &str) -> Result<()> {
        let mut routing_table = self.routing_table.write().await;
        routing_table.remove(destination)
            .ok_or_else(|| UnicoinError::Network("Route not found".to_string()))?;
        Ok(())
    }

    /// Find the best route to a destination
    pub async fn find_route(&self, destination: &str) -> Option<RouteEntry> {
        let routing_table = self.routing_table.read().await;
        routing_table.get(destination).cloned()
    }

    /// Update route usage
    pub async fn update_route_usage(&self, destination: &str) -> Result<()> {
        let mut routing_table = self.routing_table.write().await;
        if let Some(route) = routing_table.get_mut(destination) {
            route.last_used = crate::utils::timestamp();
            Ok(())
        } else {
            Err(UnicoinError::Network("Route not found".to_string()))
        }
    }

    /// Cache a message
    pub async fn cache_message(&self, message_hash: crate::crypto::Hash, content: Vec<u8>, ttl: Option<u64>) -> Result<()> {
        if !self.config.enable_caching {
            return Ok(()); // Caching disabled
        }

        let mut message_cache = self.message_cache.write().await;
        
        if message_cache.len() >= self.config.cache_size {
            // Remove oldest message
            self.evict_oldest_message(&mut message_cache).await;
        }

        let cached_message = CachedMessage {
            content,
            hash: message_hash,
            created_at: crate::utils::timestamp(),
            ttl: ttl.unwrap_or(self.config.default_ttl),
        };

        message_cache.insert(message_hash.to_hex(), cached_message);
        Ok(())
    }

    /// Get a cached message
    pub async fn get_cached_message(&self, message_hash: &crate::crypto::Hash) -> Option<Vec<u8>> {
        let mut message_cache = self.message_cache.write().await;
        let hash_str = message_hash.to_hex();
        
        if let Some(cached_message) = message_cache.get(&hash_str) {
            // Check if message is still valid
            let current_time = crate::utils::timestamp();
            if current_time - cached_message.created_at <= cached_message.ttl {
                return Some(cached_message.content.clone());
            } else {
                // Message expired, remove it
                message_cache.remove(&hash_str);
            }
        }
        
        None
    }

    /// Route a message to its destination
    pub async fn route_message(&self, destination: &str, message: Vec<u8>) -> Result<bool> {
        // Check cache first
        let message_hash = crate::crypto::sha256(&message);
        if self.get_cached_message(&message_hash).await.is_some() {
            return Ok(true); // Message already cached
        }

        // Find route to destination
        if let Some(route) = self.find_route(destination).await {
            // Update route usage
            self.update_route_usage(destination).await?;

            // Cache the message
            self.cache_message(message_hash, message, None).await?;

            // In a real implementation, this would send the message to the next hop
            tracing::info!("Routing message to {} via {}", destination, route.next_hop);
            
            Ok(true)
        } else {
            tracing::warn!("No route found to destination: {}", destination);
            Ok(false)
        }
    }

    /// Remove oldest message from cache
    async fn evict_oldest_message(&self, cache: &mut HashMap<String, CachedMessage>) {
        let oldest_key = cache.iter()
            .min_by_key(|(_, message)| message.created_at)
            .map(|(key, _)| key.clone());
        
        if let Some(key) = oldest_key {
            cache.remove(&key);
        }
    }

    /// Clean up expired routes
    pub async fn cleanup_expired_routes(&self) -> usize {
        let mut routing_table = self.routing_table.write().await;
        let current_time = crate::utils::timestamp();
        let mut to_remove = Vec::new();

        for (destination, route) in routing_table.iter() {
            if current_time - route.created_at > self.config.route_timeout {
                to_remove.push(destination.clone());
            }
        }

        let removed_count = to_remove.len();
        for destination in to_remove {
            routing_table.remove(&destination);
        }

        removed_count
    }

    /// Clean up expired cached messages
    pub async fn cleanup_expired_messages(&self) -> usize {
        let mut message_cache = self.message_cache.write().await;
        let current_time = crate::utils::timestamp();
        let mut to_remove = Vec::new();

        for (hash, message) in message_cache.iter() {
            if current_time - message.created_at > message.ttl {
                to_remove.push(hash.clone());
            }
        }

        let removed_count = to_remove.len();
        for hash in to_remove {
            message_cache.remove(&hash);
        }

        removed_count
    }

    /// Get routing statistics
    pub async fn get_routing_statistics(&self) -> RoutingStatistics {
        let routing_table = self.routing_table.read().await;
        let message_cache = self.message_cache.read().await;

        let total_routes = routing_table.len();
        let active_routes = routing_table.values()
            .filter(|route| route.status == RouteStatus::Active)
            .count();
        let cached_messages = message_cache.len();

        RoutingStatistics {
            total_routes,
            active_routes,
            cached_messages,
            cache_hit_rate: 0.0, // Would be calculated from actual usage
        }
    }
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            max_routes: 1000,
            cache_size: 10000,
            default_ttl: 3600, // 1 hour
            route_timeout: 86400, // 24 hours
            enable_caching: true,
        }
    }
}

/// Routing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingStatistics {
    pub total_routes: usize,
    pub active_routes: usize,
    pub cached_messages: usize,
    pub cache_hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_routing_creation() {
        let routing = MessageRouting::new();
        let stats = routing.get_routing_statistics().await;
        assert_eq!(stats.total_routes, 0);
        assert_eq!(stats.active_routes, 0);
        assert_eq!(stats.cached_messages, 0);
    }

    #[tokio::test]
    async fn test_route_management() {
        let routing = MessageRouting::new();
        
        // Add a route
        routing.add_route("127.0.0.1".to_string(), "127.0.0.2".to_string(), 10).await.unwrap();
        
        // Find the route
        let route = routing.find_route("127.0.0.1").await.unwrap();
        assert_eq!(route.destination, "127.0.0.1");
        assert_eq!(route.next_hop, "127.0.0.2");
        assert_eq!(route.cost, 10);
        assert_eq!(route.status, RouteStatus::Active);
        
        // Remove the route
        routing.remove_route("127.0.0.1").await.unwrap();
        assert!(routing.find_route("127.0.0.1").await.is_none());
    }

    #[tokio::test]
    async fn test_message_caching() {
        let routing = MessageRouting::new();
        let message = b"test message".to_vec();
        let message_hash = crate::crypto::sha256(&message);
        
        // Cache the message
        routing.cache_message(message_hash, message.clone(), Some(60)).await.unwrap();
        
        // Retrieve the message
        let cached = routing.get_cached_message(&message_hash).await.unwrap();
        assert_eq!(cached, message);
        
        // Try to get non-existent message
        let fake_hash = crate::crypto::Hash::random();
        assert!(routing.get_cached_message(&fake_hash).await.is_none());
    }

    #[tokio::test]
    async fn test_message_routing() {
        let routing = MessageRouting::new();
        
        // Add a route
        routing.add_route("127.0.0.1".to_string(), "127.0.0.2".to_string(), 10).await.unwrap();
        
        // Route a message
        let message = b"test message".to_vec();
        let result = routing.route_message("127.0.0.1", message).await.unwrap();
        assert!(result);
        
        // Try to route to non-existent destination
        let result = routing.route_message("127.0.0.3", message).await.unwrap();
        assert!(!result);
    }
}
