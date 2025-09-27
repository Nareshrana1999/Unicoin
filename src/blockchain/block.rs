//! Block implementation
//!
//! This module defines the block structure and related functionality.

use crate::{
    crypto::Hash,
    utils::timestamp,
    blockchain::transaction::Transaction,
};
use serde::{Deserialize, Serialize};

/// Block header containing metadata about the block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block version
    pub version: u32,
    /// Hash of the previous block
    pub previous_hash: Hash,
    /// Merkle root of transactions in this block
    pub merkle_root: Hash,
    /// Timestamp when the block was created
    pub timestamp: u64,
    /// Block height
    pub height: u64,
    /// Nonce for proof-of-stake
    pub nonce: u64,
    /// Block difficulty target
    pub target: u64,
}

impl BlockHeader {
    /// Create a new block header
    pub fn new(
        previous_hash: Hash,
        merkle_root: Hash,
        height: u64,
        target: u64,
    ) -> Self {
        Self {
            version: 1,
            previous_hash,
            merkle_root,
            timestamp: timestamp(),
            height,
            nonce: 0,
            target,
        }
    }

    /// Calculate the hash of this block header
    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).expect("BlockHeader should be serializable");
        Hash::from_bytes(&bytes)
    }

    /// Update the nonce
    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    /// Check if the block meets the difficulty target
    pub fn meets_target(&self) -> bool {
        let hash = self.hash();
        let hash_value = u64::from_le_bytes([
            hash.as_bytes()[0], hash.as_bytes()[1], 
            hash.as_bytes()[2], hash.as_bytes()[3],
            hash.as_bytes()[4], hash.as_bytes()[5], 
            hash.as_bytes()[6], hash.as_bytes()[7],
        ]);
        hash_value <= self.target
    }
}

/// A block in the blockchain
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// List of transactions in this block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new(
        previous_hash: Hash,
        transactions: Vec<Transaction>,
        height: u64,
        target: u64,
    ) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let header = BlockHeader::new(previous_hash, merkle_root, height, target);
        
        Self { header, transactions }
    }

    /// Calculate the hash of this block
    pub fn hash(&self) -> Hash {
        self.header.hash()
    }

    /// Calculate the Merkle root of transactions
    fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return Hash::zero();
        }

        if transactions.len() == 1 {
            return transactions[0].hash();
        }

        // Create a list of transaction hashes
        let mut hashes: Vec<Hash> = transactions.iter().map(|tx| tx.hash()).collect();

        // Build the Merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                if chunk.len() == 2 {
                    // Combine two hashes
                    let combined = Self::combine_hashes(chunk[0], chunk[1]);
                    next_level.push(combined);
                } else {
                    // Odd number of hashes, duplicate the last one
                    let combined = Self::combine_hashes(chunk[0], chunk[0]);
                    next_level.push(combined);
                }
            }
            
            hashes = next_level;
        }

        hashes[0]
    }

    /// Combine two hashes using double SHA-256
    fn combine_hashes(left: Hash, right: Hash) -> Hash {
        let mut combined = Vec::new();
        combined.extend_from_slice(left.as_bytes());
        combined.extend_from_slice(right.as_bytes());
        Hash::from_bytes(&combined)
    }

    /// Validate the block structure
    pub fn validate(&self) -> Result<(), String> {
        // Validate header
        if self.header.version == 0 {
            return Err("Invalid block version".to_string());
        }

        if self.header.timestamp == 0 {
            return Err("Invalid timestamp".to_string());
        }

        // Validate Merkle root
        let calculated_merkle_root = Self::calculate_merkle_root(&self.transactions);
        if calculated_merkle_root != self.header.merkle_root {
            return Err("Invalid Merkle root".to_string());
        }

        // Validate transaction count
        if self.transactions.len() > crate::MAX_TRANSACTIONS_PER_BLOCK {
            return Err("Too many transactions in block".to_string());
        }

        // Validate first transaction is coinbase (for mining rewards)
        if !self.transactions.is_empty() && self.transactions[0].tx_type != crate::blockchain::transaction::TransactionType::Coinbase {
            return Err("First transaction must be coinbase".to_string());
        }

        Ok(())
    }

    /// Get the size of the block in bytes
    pub fn size(&self) -> usize {
        bincode::serialize(self).map(|bytes| bytes.len()).unwrap_or(0)
    }

    /// Get the total transaction fees in this block
    pub fn total_fees(&self) -> u64 {
        self.transactions.iter()
            .filter(|tx| tx.tx_type != crate::blockchain::transaction::TransactionType::Coinbase)
            .map(|tx| tx.fee)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::transaction::{Transaction, TransactionType, TransactionOutput};

    #[test]
    fn test_block_creation() {
        let previous_hash = Hash::random();
        let transactions = vec![];
        let height = 1;
        let target = 1000;

        let block = Block::new(previous_hash, transactions, height, target);
        
        assert_eq!(block.header.height, height);
        assert_eq!(block.header.previous_hash, previous_hash);
        assert_eq!(block.transactions.len(), 0);
    }

    #[test]
    fn test_merkle_root_calculation() {
        let transactions = vec![];
        let merkle_root = Block::calculate_merkle_root(&transactions);
        assert_eq!(merkle_root, Hash::zero());
    }

    #[test]
    fn test_block_hash() {
        let previous_hash = Hash::random();
        let transactions = vec![];
        let height = 1;
        let target = 1000;

        let block = Block::new(previous_hash, transactions, height, target);
        let hash = block.hash();
        
        // Hash should be deterministic
        let hash2 = block.hash();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_block_validation() {
        let previous_hash = Hash::random();
        let transactions = vec![];
        let height = 1;
        let target = 1000;

        let mut block = Block::new(previous_hash, transactions, height, target);
        
        // Valid block should pass validation
        assert!(block.validate().is_ok());

        // Invalid version should fail
        block.header.version = 0;
        assert!(block.validate().is_err());
        block.header.version = 1; // Restore

        // Invalid timestamp should fail
        block.header.timestamp = 0;
        assert!(block.validate().is_err());
    }
}
