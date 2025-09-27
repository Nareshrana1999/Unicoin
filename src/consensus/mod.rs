//! Consensus mechanism implementation
//!
//! This module provides the Proof-of-Stake consensus mechanism for Unicoin.

use crate::{
    blockchain::{Block, Blockchain, BlockchainState},
    crypto::{Hash, PublicKey, PrivateKey},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub mod proof_of_stake;
pub mod validator;
pub mod slashing;

pub use proof_of_stake::ProofOfStake;
pub use validator::{Validator, ValidatorSet};
pub use slashing::{SlashingCondition, SlashingPenalty};

/// Consensus engine for Unicoin
pub struct ConsensusEngine {
    /// The blockchain instance
    blockchain: Blockchain,
    /// Validator set
    validators: RwLock<ValidatorSet>,
    /// Proof-of-stake mechanism
    pos: ProofOfStake,
    /// Current epoch
    current_epoch: u64,
    /// Epoch length in blocks
    epoch_length: u64,
}

impl ConsensusEngine {
    /// Create a new consensus engine
    pub fn new(blockchain: Blockchain) -> Result<Self> {
        let validators = RwLock::new(ValidatorSet::new());
        let pos = ProofOfStake::new();
        
        Ok(Self {
            blockchain,
            validators,
            pos,
            current_epoch: 0,
            epoch_length: 100, // 100 blocks per epoch
        })
    }

    /// Start the consensus engine
    pub async fn start(&self) -> Result<()> {
        // Initialize validators from blockchain state
        self.initialize_validators().await?;
        
        // Start consensus loop
        self.consensus_loop().await
    }

    /// Initialize validators from the blockchain
    async fn initialize_validators(&self) -> Result<()> {
        let mut validators = self.validators.write().await;
        let state = self.blockchain.state();
        
        // Load validators from blockchain state
        // This is a simplified implementation
        validators.clear();
        
        Ok(())
    }

    /// Main consensus loop
    async fn consensus_loop(&self) -> Result<()> {
        loop {
            // Wait for block time
            tokio::time::sleep(tokio::time::Duration::from_secs(crate::BLOCK_TIME)).await;
            
            // Check if we should propose a block
            if self.should_propose_block().await? {
                if let Err(e) = self.propose_block().await {
                    tracing::error!("Failed to propose block: {}", e);
                }
            }
            
            // Update epoch if necessary
            if self.blockchain.height() % self.epoch_length == 0 {
                self.update_epoch().await?;
            }
        }
    }

    /// Check if this node should propose a block
    async fn should_propose_block(&self) -> Result<bool> {
        let validators = self.validators.read().await;
        let height = self.blockchain.height();
        let slot = height % self.epoch_length;
        
        // Simple round-robin selection
        let validator_count = validators.len();
        if validator_count == 0 {
            return Ok(false);
        }
        
        let selected_validator_index = slot as usize % validator_count;
        // In a real implementation, you'd check if this node is the selected validator
        Ok(true) // Simplified for now
    }

    /// Propose a new block
    async fn propose_block(&self) -> Result<()> {
        // Collect transactions from mempool
        let transactions = self.collect_transactions().await?;
        
        // Create new block
        let previous_hash = self.blockchain.latest_block_hash();
        let height = self.blockchain.height() + 1;
        let target = self.blockchain.state().get_difficulty_target();
        
        let mut block = Block::new(previous_hash, transactions, height, target);
        
        // Sign the block
        self.sign_block(&mut block).await?;
        
        // Add block to blockchain
        let mut blockchain = self.blockchain.clone();
        blockchain.add_block(block)?;
        
        tracing::info!("Proposed new block at height {}", height);
        Ok(())
    }

    /// Collect transactions for the next block
    async fn collect_transactions(&self) -> Result<Vec<crate::blockchain::Transaction>> {
        // In a real implementation, this would collect from a mempool
        // For now, return empty transactions
        Ok(vec![])
    }

    /// Sign a block
    async fn sign_block(&self, block: &mut Block) -> Result<()> {
        // In a real implementation, this would use the validator's private key
        // For now, just set a random nonce
        use rand::RngCore;
        block.header.set_nonce(rand::thread_rng().next_u64());
        
        Ok(())
    }

    /// Update the current epoch
    async fn update_epoch(&self) -> Result<()> {
        let new_epoch = self.blockchain.height() / self.epoch_length;
        if new_epoch > self.current_epoch {
            self.current_epoch = new_epoch;
            tracing::info!("Updated to epoch {}", new_epoch);
            
            // Update validator set for new epoch
            self.update_validator_set().await?;
        }
        Ok(())
    }

    /// Update the validator set for the new epoch
    async fn update_validator_set(&self) -> Result<()> {
        let mut validators = self.validators.write().await;
        
        // In a real implementation, this would:
        // 1. Check for new stake delegations
        // 2. Remove validators who were slashed
        // 3. Update validator stakes
        
        // For now, just log the update
        tracing::info!("Updated validator set for epoch {}", self.current_epoch);
        
        Ok(())
    }

    /// Validate a block according to consensus rules
    pub async fn validate_block(&self, block: &Block) -> Result<bool> {
        // Basic blockchain validation
        if let Err(_) = block.validate() {
            return Ok(false);
        }
        
        // Consensus-specific validation
        if !self.validate_consensus_rules(block).await? {
            return Ok(false);
        }
        
        // Check if block meets difficulty target
        if !block.header.meets_target() {
            return Ok(false);
        }
        
        Ok(true)
    }

    /// Validate consensus-specific rules
    async fn validate_consensus_rules(&self, block: &Block) -> Result<bool> {
        // Check if the block proposer is a valid validator
        let validators = self.validators.read().await;
        let height = block.header.height;
        let slot = height % self.epoch_length;
        let validator_count = validators.len();
        
        if validator_count == 0 {
            return Ok(false);
        }
        
        let expected_validator_index = slot as usize % validator_count;
        // In a real implementation, you'd verify the block signature matches the expected validator
        
        Ok(true) // Simplified for now
    }

    /// Get the current validator set
    pub async fn get_validators(&self) -> ValidatorSet {
        self.validators.read().await.clone()
    }

    /// Add a new validator
    pub async fn add_validator(&self, validator: Validator) -> Result<()> {
        let mut validators = self.validators.write().await;
        validators.add_validator(validator)?;
        Ok(())
    }

    /// Remove a validator
    pub async fn remove_validator(&self, validator_address: &Hash) -> Result<()> {
        let mut validators = self.validators.write().await;
        validators.remove_validator(validator_address)?;
        Ok(())
    }

    /// Get current epoch
    pub fn get_current_epoch(&self) -> u64 {
        self.current_epoch
    }

    /// Get epoch length
    pub fn get_epoch_length(&self) -> u64 {
        self.epoch_length
    }
}

/// Consensus parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusParams {
    /// Block time in seconds
    pub block_time: u64,
    /// Epoch length in blocks
    pub epoch_length: u64,
    /// Minimum stake required to become a validator
    pub min_stake: u64,
    /// Maximum number of validators
    pub max_validators: usize,
    /// Slashing conditions
    pub slashing_conditions: Vec<SlashingCondition>,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            block_time: crate::BLOCK_TIME,
            epoch_length: 100,
            min_stake: 100_000_000, // 100 UNI minimum stake
            max_validators: 1000,
            slashing_conditions: vec![
                SlashingCondition::DoubleSign,
                SlashingCondition::Downtime,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::BlockchainState;

    #[tokio::test]
    async fn test_consensus_engine_creation() {
        let blockchain = Blockchain::new().unwrap();
        let consensus = ConsensusEngine::new(blockchain).unwrap();
        
        assert_eq!(consensus.get_current_epoch(), 0);
        assert_eq!(consensus.get_epoch_length(), 100);
    }

    #[tokio::test]
    async fn test_validator_management() {
        let blockchain = Blockchain::new().unwrap();
        let consensus = ConsensusEngine::new(blockchain).unwrap();
        
        let validator = Validator::new(
            PublicKey::random(),
            1000,
            crate::utils::timestamp(),
        );
        
        consensus.add_validator(validator).await.unwrap();
        let validators = consensus.get_validators().await;
        assert_eq!(validators.len(), 1);
    }

    #[test]
    fn test_consensus_params() {
        let params = ConsensusParams::default();
        
        assert_eq!(params.block_time, crate::BLOCK_TIME);
        assert_eq!(params.epoch_length, 100);
        assert_eq!(params.min_stake, 100_000_000);
        assert_eq!(params.max_validators, 1000);
        assert!(!params.slashing_conditions.is_empty());
    }
}
