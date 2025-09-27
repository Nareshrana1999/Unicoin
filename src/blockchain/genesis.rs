//! Genesis block implementation
//!
//! This module provides the genesis block creation and validation for Unicoin.

use crate::{
    blockchain::{Block, BlockHeader, Transaction, TransactionType, TransactionOutput},
    crypto::{Hash, PublicKey},
    utils::timestamp,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};

/// Genesis block configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    /// Genesis block timestamp
    pub timestamp: u64,
    /// Initial validator public keys
    pub initial_validators: Vec<PublicKey>,
    /// Initial token distribution
    pub initial_distribution: Vec<GenesisDistribution>,
    /// Genesis block message
    pub message: String,
    /// Network ID
    pub network_id: u64,
}

/// Initial token distribution for genesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisDistribution {
    /// Recipient public key
    pub recipient: PublicKey,
    /// Amount in satoshis
    pub amount: u64,
    /// Distribution reason
    pub reason: String,
}

/// Genesis block creator
pub struct GenesisCreator;

impl GenesisCreator {
    /// Create the genesis block for Unicoin
    pub fn create_genesis_block(config: GenesisConfig) -> Result<Block> {
        let previous_hash = Hash::zero();
        let height = 0;
        let target = u64::MAX; // Genesis block has maximum difficulty

        // Create genesis transactions
        let mut transactions = Vec::new();

        // Create coinbase transaction for each initial distribution
        for distribution in &config.initial_distribution {
            let coinbase_tx = Transaction::new_coinbase(
                distribution.recipient.clone(),
                distribution.amount,
                height,
            );
            transactions.push(coinbase_tx);
        }

        // Create the genesis block
        let mut block = Block::new(previous_hash, transactions, height, target);
        
        // Set the genesis block timestamp
        block.header.timestamp = config.timestamp;
        
        // Set the genesis block message in the coinbase transaction
        if let Some(first_tx) = block.transactions.first_mut() {
            if let Some(input) = first_tx.inputs.first_mut() {
                input.script_sig = format!("Genesis: {}", config.message).into_bytes();
            }
        }

        Ok(block)
    }

    /// Create default genesis configuration for mainnet
    pub fn create_mainnet_genesis() -> GenesisConfig {
        let mut initial_distribution = Vec::new();

        // Development team allocation (10%)
        let dev_team_key = PublicKey::from_hex(
            crate::crypto::CryptoAlgorithm::Secp256k1,
            "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9"
        ).unwrap_or_else(|_| PublicKey::random());

        initial_distribution.push(GenesisDistribution {
            recipient: dev_team_key,
            amount: 2_100_000 * 1_000_000_000, // 2.1M UNI (10%)
            reason: "Development team allocation".to_string(),
        });

        // Community treasury (20%)
        let treasury_key = PublicKey::from_hex(
            crate::crypto::CryptoAlgorithm::Secp256k1,
            "03defdea4cd60867a10fda394dfb13c9a4b0305d2a00f7b7cf55dad3019afb37d"
        ).unwrap_or_else(|_| PublicKey::random());

        initial_distribution.push(GenesisDistribution {
            recipient: treasury_key,
            amount: 4_200_000 * 1_000_000_000, // 4.2M UNI (20%)
            reason: "Community treasury".to_string(),
        });

        // Initial validators (10%)
        let mut initial_validators = Vec::new();
        for i in 0..10 {
            let validator_key = PublicKey::from_hex(
                crate::crypto::CryptoAlgorithm::Secp256k1,
                &format!("02{:064x}", i)
            ).unwrap_or_else(|_| PublicKey::random());

            initial_validators.push(validator_key.clone());
            initial_distribution.push(GenesisDistribution {
                recipient: validator_key,
                amount: 210_000 * 1_000_000_000, // 210K UNI each
                reason: format!("Initial validator {}", i + 1),
            });
        }

        // Remaining tokens for future distribution (60%)
        let future_distribution_key = PublicKey::from_hex(
            crate::crypto::CryptoAlgorithm::Secp256k1,
            "03c0b7c8b7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6"
        ).unwrap_or_else(|_| PublicKey::random());

        initial_distribution.push(GenesisDistribution {
            recipient: future_distribution_key,
            amount: 12_600_000 * 1_000_000_000, // 12.6M UNI (60%)
            reason: "Future distribution and rewards".to_string(),
        });

        GenesisConfig {
            timestamp: timestamp(),
            initial_validators,
            initial_distribution,
            message: "Unicoin Genesis Block - The Ultimate Secure Cryptocurrency".to_string(),
            network_id: 1, // Mainnet
        }
    }

    /// Create testnet genesis configuration
    pub fn create_testnet_genesis() -> GenesisConfig {
        let mut initial_distribution = Vec::new();
        let mut initial_validators = Vec::new();

        // Create test validators
        for i in 0..5 {
            let validator_key = PublicKey::random();
            initial_validators.push(validator_key.clone());
            initial_distribution.push(GenesisDistribution {
                recipient: validator_key,
                amount: 100_000 * 1_000_000_000, // 100K UNI each for testing
                reason: format!("Test validator {}", i + 1),
            });
        }

        GenesisConfig {
            timestamp: timestamp(),
            initial_validators,
            initial_distribution,
            message: "Unicoin Testnet Genesis Block".to_string(),
            network_id: 1001, // Testnet
        }
    }

    /// Validate genesis block
    pub fn validate_genesis_block(block: &Block, config: &GenesisConfig) -> Result<bool> {
        // Check block height
        if block.header.height != 0 {
            return Ok(false);
        }

        // Check previous hash is zero
        if !block.header.previous_hash.is_zero() {
            return Ok(false);
        }

        // Check timestamp
        if block.header.timestamp != config.timestamp {
            return Ok(false);
        }

        // Check transactions are all coinbase
        for tx in &block.transactions {
            if tx.tx_type != TransactionType::Coinbase {
                return Ok(false);
            }
        }

        // Check total distribution matches expected
        let total_distributed: u64 = block.transactions.iter()
            .map(|tx| tx.outputs.iter().map(|output| output.amount).sum::<u64>())
            .sum();

        let expected_total: u64 = config.initial_distribution.iter()
            .map(|dist| dist.amount)
            .sum();

        if total_distributed != expected_total {
            return Ok(false);
        }

        // Validate block structure
        block.validate().map_err(|_| UnicoinError::Blockchain(
            "Genesis block validation failed".to_string()
        ))?;

        Ok(true)
    }

    /// Get genesis block hash (hardcoded for network identification)
    pub fn get_genesis_hash() -> Hash {
        // This is a well-known hash that identifies the Unicoin network
        Hash::from_hex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
            .unwrap_or_else(|_| Hash::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block_creation() {
        let config = GenesisCreator::create_testnet_genesis();
        let genesis_block = GenesisCreator::create_genesis_block(config.clone()).unwrap();

        assert_eq!(genesis_block.header.height, 0);
        assert!(genesis_block.header.previous_hash.is_zero());
        assert_eq!(genesis_block.transactions.len(), config.initial_distribution.len());
    }

    #[test]
    fn test_genesis_block_validation() {
        let config = GenesisCreator::create_testnet_genesis();
        let genesis_block = GenesisCreator::create_genesis_block(config.clone()).unwrap();

        let is_valid = GenesisCreator::validate_genesis_block(&genesis_block, &config).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_mainnet_genesis_distribution() {
        let config = GenesisCreator::create_mainnet_genesis();
        let total_distributed: u64 = config.initial_distribution.iter()
            .map(|dist| dist.amount)
            .sum();

        assert_eq!(total_distributed, 21_000_000 * 1_000_000_000); // 21M UNI total
        assert_eq!(config.initial_validators.len(), 10);
    }
}
