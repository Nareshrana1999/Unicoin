//! Network configuration for Unicoin
//!
//! This module defines network-related configuration options including
//! peer management, protocol settings, and connection parameters.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use crate::Result;

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listening address
    pub listen_address: IpAddr,
    /// Listening port
    pub listen_port: u16,
    /// Maximum number of peers
    pub max_peers: u32,
    /// Enable peer discovery
    pub enable_discovery: bool,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable DHT discovery
    pub enable_dht: bool,
    /// Bootstrap nodes
    pub bootstrap_nodes: Vec<String>,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Read timeout (seconds)
    pub read_timeout: u64,
    /// Write timeout (seconds)
    pub write_timeout: u64,
    /// Keep-alive interval (seconds)
    pub keep_alive_interval: u64,
    /// Maximum message size (bytes)
    pub max_message_size: usize,
    /// Enable encryption
    pub enable_encryption: bool,
    /// Enable compression
    pub enable_compression: bool,
    /// Network ID (for chain separation)
    pub network_id: u32,
    /// Enable NAT traversal
    pub enable_nat_traversal: bool,
    /// Enable UPnP
    pub enable_upnp: bool,
    /// Enable hole punching
    pub enable_hole_punching: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".parse().unwrap(),
            listen_port: 30303,
            max_peers: 100,
            enable_discovery: true,
            enable_mdns: true,
            enable_dht: true,
            bootstrap_nodes: vec![
                "/ip4/127.0.0.1/tcp/30303/p2p/12D3KooWExample".to_string(),
            ],
            connection_timeout: 30,
            read_timeout: 60,
            write_timeout: 60,
            keep_alive_interval: 30,
            max_message_size: 1024 * 1024, // 1MB
            enable_encryption: true,
            enable_compression: true,
            network_id: 1,
            enable_nat_traversal: true,
            enable_upnp: true,
            enable_hole_punching: true,
        }
    }
}

impl NetworkConfig {
    /// Create a new network configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the listening socket address
    pub fn listen_addr(&self) -> SocketAddr {
        SocketAddr::new(self.listen_address, self.listen_port)
    }

    /// Set the listening address
    pub fn set_listen_address(&mut self, address: IpAddr, port: u16) {
        self.listen_address = address;
        self.listen_port = port;
    }

    /// Add a bootstrap node
    pub fn add_bootstrap_node(&mut self, node: String) {
        if !self.bootstrap_nodes.contains(&node) {
            self.bootstrap_nodes.push(node);
        }
    }

    /// Remove a bootstrap node
    pub fn remove_bootstrap_node(&mut self, node: &str) {
        self.bootstrap_nodes.retain(|n| n != node);
    }

    /// Validate the network configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_peers == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_peers must be greater than 0".to_string()));
        }

        if self.max_message_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_message_size must be greater than 0".to_string()));
        }

        if self.connection_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("connection_timeout must be greater than 0".to_string()));
        }

        if self.read_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("read_timeout must be greater than 0".to_string()));
        }

        if self.write_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("write_timeout must be greater than 0".to_string()));
        }

        if self.keep_alive_interval == 0 {
            return Err(crate::UnicoinError::InvalidConfig("keep_alive_interval must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Check if discovery is enabled
    pub fn discovery_enabled(&self) -> bool {
        self.enable_discovery
    }

    /// Check if mDNS discovery is enabled
    pub fn mdns_enabled(&self) -> bool {
        self.enable_mdns && self.enable_discovery
    }

    /// Check if DHT discovery is enabled
    pub fn dht_enabled(&self) -> bool {
        self.enable_dht && self.enable_discovery
    }

    /// Check if encryption is enabled
    pub fn encryption_enabled(&self) -> bool {
        self.enable_encryption
    }

    /// Check if compression is enabled
    pub fn compression_enabled(&self) -> bool {
        self.enable_compression
    }

    /// Check if NAT traversal is enabled
    pub fn nat_traversal_enabled(&self) -> bool {
        self.enable_nat_traversal
    }

    /// Check if UPnP is enabled
    pub fn upnp_enabled(&self) -> bool {
        self.enable_upnp && self.enable_nat_traversal
    }

    /// Check if hole punching is enabled
    pub fn hole_punching_enabled(&self) -> bool {
        self.enable_hole_punching && self.enable_nat_traversal
    }
}
