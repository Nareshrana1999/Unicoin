//! Blockchain configuration for Unicoin
//!
//! This module defines blockchain-related configuration options including
//! block parameters, transaction settings, and chain-specific parameters.

use serde::{Deserialize, Serialize};
use crate::Result;

/// Blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Block time in seconds
    pub block_time: u64,
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Maximum transaction size in bytes
    pub max_transaction_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: u32,
    /// Enable difficulty adjustment
    pub difficulty_adjustment: bool,
    /// Difficulty adjustment interval (blocks)
    pub difficulty_adjustment_interval: u32,
    /// Minimum difficulty
    pub min_difficulty: u64,
    /// Maximum difficulty
    pub max_difficulty: u64,
    /// Block reward (in smallest unit)
    pub block_reward: u64,
    /// Halving interval (blocks)
    pub halving_interval: u64,
    /// Maximum supply (in smallest unit)
    pub max_supply: u64,
    /// Enable UTXO model
    pub enable_utxo: bool,
    /// Enable SegWit
    pub enable_segwit: bool,
    /// Enable Lightning Network
    pub enable_lightning: bool,
    /// Transaction fee rate (per byte)
    pub fee_rate: u64,
    /// Minimum transaction fee
    pub min_fee: u64,
    /// Enable transaction replacement
    pub enable_rbf: bool,
    /// Maximum replacement fee bump
    pub max_fee_bump: u64,
    /// Enable CPFP (Child Pays For Parent)
    pub enable_cpfp: bool,
    /// Mempool size limit
    pub mempool_size_limit: usize,
    /// Transaction expiry time (seconds)
    pub transaction_expiry: u64,
    /// Enable transaction prioritization
    pub enable_priority: bool,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            block_time: 10, // 10 seconds
            max_block_size: 4 * 1024 * 1024, // 4MB
            max_transaction_size: 1024 * 1024, // 1MB
            max_transactions_per_block: 10000,
            difficulty_adjustment: true,
            difficulty_adjustment_interval: 2016, // Every ~3.5 hours
            min_difficulty: 1,
            max_difficulty: u64::MAX,
            block_reward: 50 * 100_000_000, // 50 Unicoin (8 decimals)
            halving_interval: 210_000, // Every ~4 years
            max_supply: 21_000_000 * 100_000_000, // 21M Unicoin
            enable_utxo: true,
            enable_segwit: true,
            enable_lightning: true,
            fee_rate: 10, // 10 satoshis per byte
            min_fee: 1000, // 1000 satoshis
            enable_rbf: true,
            max_fee_bump: 100_000_000, // 1 Unicoin
            enable_cpfp: true,
            mempool_size_limit: 100_000,
            transaction_expiry: 3600, // 1 hour
            enable_priority: true,
        }
    }
}

impl BlockchainConfig {
    /// Create a new blockchain configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the blockchain configuration
    pub fn validate(&self) -> Result<()> {
        if self.block_time == 0 {
            return Err(crate::UnicoinError::InvalidConfig("block_time must be greater than 0".to_string()));
        }

        if self.max_block_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_block_size must be greater than 0".to_string()));
        }

        if self.max_transaction_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_transaction_size must be greater than 0".to_string()));
        }

        if self.max_transactions_per_block == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_transactions_per_block must be greater than 0".to_string()));
        }

        if self.difficulty_adjustment && self.difficulty_adjustment_interval == 0 {
            return Err(crate::UnicoinError::InvalidConfig("difficulty_adjustment_interval must be greater than 0".to_string()));
        }

        if self.min_difficulty >= self.max_difficulty {
            return Err(crate::UnicoinError::InvalidConfig("min_difficulty must be less than max_difficulty".to_string()));
        }

        if self.block_reward == 0 {
            return Err(crate::UnicoinError::InvalidConfig("block_reward must be greater than 0".to_string()));
        }

        if self.halving_interval == 0 {
            return Err(crate::UnicoinError::InvalidConfig("halving_interval must be greater than 0".to_string()));
        }

        if self.max_supply == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_supply must be greater than 0".to_string()));
        }

        if self.fee_rate == 0 {
            return Err(crate::UnicoinError::InvalidConfig("fee_rate must be greater than 0".to_string()));
        }

        if self.min_fee == 0 {
            return Err(crate::UnicoinError::InvalidConfig("min_fee must be greater than 0".to_string()));
        }

        if self.mempool_size_limit == 0 {
            return Err(crate::UnicoinError::InvalidConfig("mempool_size_limit must be greater than 0".to_string()));
        }

        if self.transaction_expiry == 0 {
            return Err(crate::UnicoinError::InvalidConfig("transaction_expiry must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Get the current block reward based on block height
    pub fn get_block_reward(&self, height: u64) -> u64 {
        let halvings = height / self.halving_interval;
        if halvings >= 64 {
            0
        } else {
            self.block_reward >> halvings
        }
    }

    /// Check if UTXO model is enabled
    pub fn utxo_enabled(&self) -> bool {
        self.enable_utxo
    }

    /// Check if SegWit is enabled
    pub fn segwit_enabled(&self) -> bool {
        self.enable_segwit
    }

    /// Check if Lightning Network is enabled
    pub fn lightning_enabled(&self) -> bool {
        self.enable_lightning
    }

    /// Check if RBF (Replace By Fee) is enabled
    pub fn rbf_enabled(&self) -> bool {
        self.enable_rbf
    }

    /// Check if CPFP is enabled
    pub fn cpfp_enabled(&self) -> bool {
        self.enable_cpfp
    }

    /// Check if transaction prioritization is enabled
    pub fn priority_enabled(&self) -> bool {
        self.enable_priority
    }

    /// Check if difficulty adjustment is enabled
    pub fn difficulty_adjustment_enabled(&self) -> bool {
        self.difficulty_adjustment
    }

    /// Calculate the maximum transaction fee
    pub fn max_transaction_fee(&self) -> u64 {
        (self.max_transaction_size as u64) * self.fee_rate
    }

    /// Calculate the minimum transaction fee for a given size
    pub fn calculate_fee(&self, size: usize) -> u64 {
        std::cmp::max(
            self.min_fee,
            (size as u64) * self.fee_rate
        )
    }
}
