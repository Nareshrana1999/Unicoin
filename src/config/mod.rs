//! Configuration module for Unicoin
//!
//! This module provides comprehensive configuration management for all Unicoin components
//! including blockchain, network, consensus, and feature settings.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::Result;

pub mod network;
pub mod blockchain;
pub mod consensus;
pub mod features;
pub mod performance;
pub mod security;
pub mod api;
pub mod logging;

pub use network::NetworkConfig;
pub use blockchain::BlockchainConfig;
pub use consensus::ConsensusConfig;
pub use features::FeaturesConfig;
pub use performance::PerformanceConfig;
pub use security::SecurityConfig;
pub use api::ApiConfig;
pub use logging::LoggingConfig;

/// Main configuration structure for Unicoin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicoinConfig {
    /// Network configuration
    pub network: NetworkConfig,
    /// Blockchain configuration
    pub blockchain: BlockchainConfig,
    /// Consensus configuration
    pub consensus: ConsensusConfig,
    /// Features configuration
    pub features: FeaturesConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// API configuration
    pub api: ApiConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Data directory path
    pub data_dir: PathBuf,
    /// Configuration file path
    pub config_file: Option<PathBuf>,
}

impl Default for UnicoinConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            blockchain: BlockchainConfig::default(),
            consensus: ConsensusConfig::default(),
            features: FeaturesConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            api: ApiConfig::default(),
            logging: LoggingConfig::default(),
            data_dir: PathBuf::from("./data"),
            config_file: None,
        }
    }
}

impl UnicoinConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from file
    pub fn load_from_file<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        let config: UnicoinConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
        let path = path.into();
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        self.network.validate()?;
        self.blockchain.validate()?;
        self.consensus.validate()?;
        self.features.validate()?;
        self.performance.validate()?;
        self.security.validate()?;
        self.api.validate()?;
        self.logging.validate()?;
        Ok(())
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    /// Set the data directory path
    pub fn set_data_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.data_dir = path.into();
    }
}

/// Configuration builder for easy setup
pub struct ConfigBuilder {
    config: UnicoinConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: UnicoinConfig::new(),
        }
    }

    /// Set network configuration
    pub fn network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    /// Set blockchain configuration
    pub fn blockchain(mut self, blockchain: BlockchainConfig) -> Self {
        self.config.blockchain = blockchain;
        self
    }

    /// Set consensus configuration
    pub fn consensus(mut self, consensus: ConsensusConfig) -> Self {
        self.config.consensus = consensus;
        self
    }

    /// Set features configuration
    pub fn features(mut self, features: FeaturesConfig) -> Self {
        self.config.features = features;
        self
    }

    /// Set performance configuration
    pub fn performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }

    /// Set security configuration
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    /// Set API configuration
    pub fn api(mut self, api: ApiConfig) -> Self {
        self.config.api = api;
        self
    }

    /// Set logging configuration
    pub fn logging(mut self, logging: LoggingConfig) -> Self {
        self.config.logging = logging;
        self
    }

    /// Set data directory
    pub fn data_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.config.data_dir = path.into();
        self
    }

    /// Build the final configuration
    pub fn build(self) -> Result<UnicoinConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
