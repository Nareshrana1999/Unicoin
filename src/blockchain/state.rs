//! Blockchain state management
//!
//! This module manages the current state of the blockchain including
//! UTXO set, account balances, and other state information.

use crate::{
    crypto::Hash,
    blockchain::{TransactionOutpoint, TransactionOutput},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the current state of the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainState {
    /// Current blockchain height
    pub height: u64,
    /// Hash of the latest block
    pub latest_block_hash: Hash,
    /// Unspent Transaction Output (UTXO) set
    pub utxo_set: HashMap<TransactionOutpoint, TransactionOutput>,
    /// Account balances (for faster lookups)
    pub balances: HashMap<Hash, u64>,
    /// Total supply of Unicoin in circulation
    pub total_supply: u64,
    /// Block difficulty target
    pub difficulty_target: u64,
    /// Last difficulty adjustment timestamp
    pub last_difficulty_adjustment: u64,
}

impl BlockchainState {
    /// Create a new blockchain state
    pub fn new() -> Self {
        Self {
            height: 0,
            latest_block_hash: Hash::zero(),
            utxo_set: HashMap::new(),
            balances: HashMap::new(),
            total_supply: 0,
            difficulty_target: crate::MAX_SUPPLY, // Start with maximum difficulty
            last_difficulty_adjustment: 0,
        }
    }

    /// Load blockchain state from database
    pub fn load(db: &sled::Db) -> Result<Self> {
        if let Some(state_bytes) = db.get(b"blockchain_state")? {
            let state: BlockchainState = bincode::deserialize(&state_bytes)?;
            Ok(state)
        } else {
            Ok(Self::new())
        }
    }

    /// Save blockchain state to database
    pub fn save(&self, db: &sled::Db) -> Result<()> {
        let state_bytes = bincode::serialize(self)?;
        db.insert(b"blockchain_state", state_bytes)?;
        db.flush()?;
        Ok(())
    }

    /// Get the balance of an address
    pub fn get_balance(&self, address: &Hash) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Update balance for an address
    pub fn update_balance(&mut self, address: Hash, balance: u64) {
        if balance > 0 {
            self.balances.insert(address, balance);
        } else {
            self.balances.remove(&address);
        }
    }

    /// Add a new UTXO to the set
    pub fn add_utxo(&mut self, outpoint: TransactionOutpoint, output: TransactionOutput) {
        self.utxo_set.insert(outpoint, output);
    }

    /// Remove a UTXO from the set
    pub fn remove_utxo(&mut self, outpoint: &TransactionOutpoint) {
        self.utxo_set.remove(outpoint);
    }

    /// Check if a UTXO exists
    pub fn has_utxo(&self, outpoint: &TransactionOutpoint) -> bool {
        self.utxo_set.contains_key(outpoint)
    }

    /// Get a UTXO by outpoint
    pub fn get_utxo(&self, outpoint: &TransactionOutpoint) -> Option<&TransactionOutput> {
        self.utxo_set.get(outpoint)
    }

    /// Calculate total balance from UTXO set
    pub fn calculate_total_balance(&self, address: &Hash) -> u64 {
        self.utxo_set
            .values()
            .filter(|output| &output.recipient.to_hash() == address)
            .map(|output| output.amount)
            .sum()
    }

    /// Update balances based on UTXO set
    pub fn update_balances_from_utxo(&mut self) {
        self.balances.clear();
        
        for output in self.utxo_set.values() {
            let address = output.recipient.to_hash();
            let current_balance = self.balances.get(&address).copied().unwrap_or(0);
            self.balances.insert(address, current_balance + output.amount);
        }
    }

    /// Adjust difficulty based on block time
    pub fn adjust_difficulty(&mut self, current_timestamp: u64) {
        const TARGET_BLOCK_TIME: u64 = crate::BLOCK_TIME;
        const ADJUSTMENT_INTERVAL: u64 = 2016; // Adjust every 2016 blocks (similar to Bitcoin)

        if self.height % ADJUSTMENT_INTERVAL == 0 && self.height > 0 {
            let time_span = current_timestamp - self.last_difficulty_adjustment;
            let expected_time = ADJUSTMENT_INTERVAL * TARGET_BLOCK_TIME;

            // Adjust difficulty to maintain target block time
            if time_span < expected_time / 4 {
                // Too fast, increase difficulty
                self.difficulty_target = self.difficulty_target / 2;
            } else if time_span > expected_time * 4 {
                // Too slow, decrease difficulty
                self.difficulty_target = self.difficulty_target * 2;
            } else {
                // Normal adjustment
                self.difficulty_target = (self.difficulty_target * expected_time) / time_span;
            }

            // Ensure difficulty doesn't go below minimum or above maximum
            self.difficulty_target = self.difficulty_target.max(1).min(crate::MAX_SUPPLY);
            
            self.last_difficulty_adjustment = current_timestamp;
        }
    }

    /// Get current difficulty target
    pub fn get_difficulty_target(&self) -> u64 {
        self.difficulty_target
    }

    /// Get the total number of UTXOs
    pub fn utxo_count(&self) -> usize {
        self.utxo_set.len()
    }

    /// Get all UTXOs for a specific address
    pub fn get_utxos_for_address(&self, address: &Hash) -> Vec<(TransactionOutpoint, &TransactionOutput)> {
        self.utxo_set
            .iter()
            .filter(|(_, output)| &output.recipient.to_hash() == address)
            .map(|(outpoint, output)| (*outpoint, output))
            .collect()
    }

    /// Validate the state consistency
    pub fn validate(&self) -> Result<()> {
        // Check that total supply doesn't exceed maximum
        if self.total_supply > crate::MAX_SUPPLY {
            return Err(UnicoinError::Blockchain(
                "Total supply exceeds maximum".to_string()
            ));
        }

        // Check that balances match UTXO set
        let mut calculated_balances = HashMap::new();
        for output in self.utxo_set.values() {
            let address = output.recipient.to_hash();
            let balance = calculated_balances.get(&address).copied().unwrap_or(0);
            calculated_balances.insert(address, balance + output.amount);
        }

        for (address, balance) in &self.balances {
            let calculated_balance = calculated_balances.get(address).copied().unwrap_or(0);
            if *balance != calculated_balance {
                return Err(UnicoinError::Blockchain(
                    format!("Balance mismatch for address: expected {}, got {}", calculated_balance, balance)
                ));
            }
        }

        Ok(())
    }

    /// Get statistics about the blockchain state
    pub fn get_statistics(&self) -> BlockchainStatistics {
        BlockchainStatistics {
            height: self.height,
            total_supply: self.total_supply,
            utxo_count: self.utxo_count(),
            unique_addresses: self.balances.len(),
            difficulty_target: self.difficulty_target,
        }
    }
}

/// Statistics about the blockchain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStatistics {
    pub height: u64,
    pub total_supply: u64,
    pub utxo_count: usize,
    pub unique_addresses: usize,
    pub difficulty_target: u64,
}

impl Default for BlockchainState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::PublicKey;

    #[test]
    fn test_blockchain_state_creation() {
        let state = BlockchainState::new();
        assert_eq!(state.height, 0);
        assert_eq!(state.utxo_set.len(), 0);
        assert_eq!(state.balances.len(), 0);
    }

    #[test]
    fn test_utxo_operations() {
        let mut state = BlockchainState::new();
        let outpoint = TransactionOutpoint::new(Hash::random(), 0);
        let output = TransactionOutput::new(1000, PublicKey::random());

        // Add UTXO
        state.add_utxo(outpoint, output.clone());
        assert!(state.has_utxo(&outpoint));
        assert_eq!(state.get_utxo(&outpoint), Some(&output));

        // Remove UTXO
        state.remove_utxo(&outpoint);
        assert!(!state.has_utxo(&outpoint));
        assert_eq!(state.get_utxo(&outpoint), None);
    }

    #[test]
    fn test_balance_calculation() {
        let mut state = BlockchainState::new();
        let address = Hash::random();
        let outpoint1 = TransactionOutpoint::new(Hash::random(), 0);
        let outpoint2 = TransactionOutpoint::new(Hash::random(), 0);
        
        let output1 = TransactionOutput {
            amount: 1000,
            recipient: PublicKey::from_hash(address),
            script_pubkey: Vec::new(),
            data: None,
        };
        
        let output2 = TransactionOutput {
            amount: 2000,
            recipient: PublicKey::from_hash(address),
            script_pubkey: Vec::new(),
            data: None,
        };

        state.add_utxo(outpoint1, output1);
        state.add_utxo(outpoint2, output2);
        state.update_balances_from_utxo();

        assert_eq!(state.get_balance(&address), 3000);
    }

    #[test]
    fn test_difficulty_adjustment() {
        let mut state = BlockchainState::new();
        state.height = 2016; // Trigger adjustment
        state.last_difficulty_adjustment = 1000;
        
        let initial_target = state.difficulty_target;
        state.adjust_difficulty(1000 + 2016 * crate::BLOCK_TIME); // Normal time
        
        // Difficulty should be adjusted
        assert_ne!(state.difficulty_target, initial_target);
    }
}
