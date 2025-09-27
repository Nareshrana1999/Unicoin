//! Memory pool (mempool) implementation
//!
//! This module provides transaction memory pool management for Unicoin.

use crate::{
    blockchain::Transaction,
    crypto::Hash,
    utils::timestamp,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, HashSet};
use tokio::sync::RwLock;

/// Transaction priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TransactionPriority {
    /// Low priority (normal transactions)
    Low = 1,
    /// Normal priority (standard transactions)
    Normal = 2,
    /// High priority (important transactions)
    High = 3,
    /// Critical priority (emergency transactions)
    Critical = 4,
}

/// Transaction in the mempool with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolTransaction {
    /// The transaction
    pub transaction: Transaction,
    /// Transaction priority
    pub priority: TransactionPriority,
    /// Time added to mempool
    pub added_at: u64,
    /// Transaction fee per byte
    pub fee_per_byte: f64,
    /// Transaction size in bytes
    pub size: usize,
    /// Number of times transaction was rejected
    pub rejection_count: u32,
}

/// Memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolConfig {
    /// Maximum number of transactions in mempool
    pub max_transactions: usize,
    /// Maximum mempool size in bytes
    pub max_size_bytes: usize,
    /// Transaction expiry time in seconds
    pub expiry_time: u64,
    /// Minimum fee per byte
    pub min_fee_per_byte: f64,
    /// Maximum transaction size
    pub max_transaction_size: usize,
}

/// Memory pool for managing pending transactions
pub struct Mempool {
    /// Transactions by hash
    transactions: RwLock<HashMap<Hash, MempoolTransaction>>,
    /// Transactions by priority (sorted by fee rate)
    priority_queues: RwLock<HashMap<TransactionPriority, BTreeMap<u64, Vec<Hash>>>>,
    /// Transactions by sender address
    sender_transactions: RwLock<HashMap<Hash, HashSet<Hash>>>,
    /// Total mempool size in bytes
    total_size: RwLock<usize>,
    /// Configuration
    config: MempoolConfig,
}

impl MempoolTransaction {
    /// Create a new mempool transaction
    pub fn new(transaction: Transaction, priority: TransactionPriority) -> Self {
        let size = transaction.size();
        let fee_per_byte = if size > 0 {
            transaction.fee as f64 / size as f64
        } else {
            0.0
        };

        Self {
            transaction,
            priority,
            added_at: timestamp(),
            fee_per_byte,
            size,
            rejection_count: 0,
        }
    }

    /// Calculate transaction score for ordering
    pub fn calculate_score(&self) -> u64 {
        // Higher score = higher priority
        let priority_multiplier = self.priority as u64;
        let fee_score = (self.fee_per_byte * 1000.0) as u64; // Convert to integer
        let age_penalty = (timestamp() - self.added_at) / 60; // Penalty for age in minutes

        priority_multiplier * 1000000 + fee_score - age_penalty
    }

    /// Check if transaction has expired
    pub fn is_expired(&self, expiry_time: u64) -> bool {
        timestamp() - self.added_at > expiry_time
    }

    /// Increment rejection count
    pub fn increment_rejection_count(&mut self) {
        self.rejection_count += 1;
    }

    /// Check if transaction should be removed due to rejections
    pub fn should_remove(&self) -> bool {
        self.rejection_count >= 3
    }
}

impl Default for MempoolConfig {
    fn default() -> Self {
        Self {
            max_transactions: 10000,
            max_size_bytes: 100 * 1024 * 1024, // 100 MB
            expiry_time: 3600, // 1 hour
            min_fee_per_byte: 0.001, // 0.001 satoshi per byte
            max_transaction_size: 1024 * 1024, // 1 MB
        }
    }
}

impl Mempool {
    /// Create a new mempool with default configuration
    pub fn new() -> Self {
        Self::with_config(MempoolConfig::default())
    }

    /// Create a new mempool with custom configuration
    pub fn with_config(config: MempoolConfig) -> Self {
        Self {
            transactions: RwLock::new(HashMap::new()),
            priority_queues: RwLock::new(HashMap::new()),
            sender_transactions: RwLock::new(HashMap::new()),
            total_size: RwLock::new(0),
            config,
        }
    }

    /// Add a transaction to the mempool
    pub async fn add_transaction(&self, transaction: Transaction, priority: TransactionPriority) -> Result<()> {
        let tx_hash = transaction.hash();
        
        // Validate transaction
        self.validate_transaction(&transaction)?;

        // Check if transaction already exists
        if self.transactions.read().await.contains_key(&tx_hash) {
            return Err(UnicoinError::Blockchain("Transaction already in mempool".to_string()));
        }

        // Create mempool transaction
        let mempool_tx = MempoolTransaction::new(transaction.clone(), priority);
        
        // Check size limits
        if mempool_tx.size > self.config.max_transaction_size {
            return Err(UnicoinError::Blockchain("Transaction too large".to_string()));
        }

        // Check fee requirements
        if mempool_tx.fee_per_byte < self.config.min_fee_per_byte {
            return Err(UnicoinError::Blockchain("Transaction fee too low".to_string()));
        }

        // Check mempool limits
        let mut total_size = self.total_size.write().await;
        if *total_size + mempool_tx.size > self.config.max_size_bytes {
            return Err(UnicoinError::Blockchain("Mempool size limit exceeded".to_string()));
        }

        let transactions = self.transactions.read().await;
        if transactions.len() >= self.config.max_transactions {
            return Err(UnicoinError::Blockchain("Mempool transaction limit exceeded".to_string()));
        }

        drop(transactions);
        drop(total_size);

        // Add transaction to mempool
        {
            let mut transactions = self.transactions.write().await;
            transactions.insert(tx_hash, mempool_tx);
        }

        // Add to priority queue
        self.add_to_priority_queue(&tx_hash, &mempool_tx).await;

        // Add to sender transactions
        if let Some(input) = transaction.inputs.first() {
            let sender_hash = input.previous_output.tx_hash;
            let mut sender_transactions = self.sender_transactions.write().await;
            sender_transactions.entry(sender_hash).or_insert_with(HashSet::new).insert(tx_hash);
        }

        // Update total size
        {
            let mut total_size = self.total_size.write().await;
            *total_size += mempool_tx.size;
        }

        Ok(())
    }

    /// Remove a transaction from the mempool
    pub async fn remove_transaction(&self, tx_hash: &Hash) -> Result<Option<MempoolTransaction>> {
        let mempool_tx = {
            let mut transactions = self.transactions.write().await;
            transactions.remove(tx_hash)
        };

        if let Some(tx) = mempool_tx {
            // Remove from priority queue
            self.remove_from_priority_queue(&tx).await;

            // Remove from sender transactions
            if let Some(input) = tx.transaction.inputs.first() {
                let sender_hash = input.previous_output.tx_hash;
                let mut sender_transactions = self.sender_transactions.write().await;
                if let Some(sender_set) = sender_transactions.get_mut(&sender_hash) {
                    sender_set.remove(tx_hash);
                    if sender_set.is_empty() {
                        sender_transactions.remove(&sender_hash);
                    }
                }
            }

            // Update total size
            {
                let mut total_size = self.total_size.write().await;
                *total_size = total_size.saturating_sub(tx.size);
            }

            Ok(Some(tx))
        } else {
            Ok(None)
        }
    }

    /// Get transactions for block creation (sorted by priority)
    pub async fn get_transactions_for_block(&self, max_size: usize) -> Vec<Transaction> {
        let mut selected_transactions = Vec::new();
        let mut current_size = 0;

        // Get transactions from priority queues (highest priority first)
        let priority_order = [
            TransactionPriority::Critical,
            TransactionPriority::High,
            TransactionPriority::Normal,
            TransactionPriority::Low,
        ];

        for priority in priority_order {
            let priority_queues = self.priority_queues.read().await;
            if let Some(queue) = priority_queues.get(&priority) {
                // Iterate through transactions in this priority (highest fee first)
                for (_, tx_hashes) in queue.iter().rev() {
                    for tx_hash in tx_hashes {
                        if current_size >= max_size {
                            break;
                        }

                        let transactions = self.transactions.read().await;
                        if let Some(mempool_tx) = transactions.get(tx_hash) {
                            if current_size + mempool_tx.size <= max_size {
                                selected_transactions.push(mempool_tx.transaction.clone());
                                current_size += mempool_tx.size;
                            }
                        }
                    }
                }
            }
        }

        selected_transactions
    }

    /// Clean up expired transactions
    pub async fn cleanup_expired(&self) -> usize {
        let mut expired_hashes = Vec::new();
        
        {
            let transactions = self.transactions.read().await;
            for (hash, mempool_tx) in transactions.iter() {
                if mempool_tx.is_expired(self.config.expiry_time) || mempool_tx.should_remove() {
                    expired_hashes.push(*hash);
                }
            }
        }

        let removed_count = expired_hashes.len();
        for hash in expired_hashes {
            self.remove_transaction(&hash).await.ok();
        }

        removed_count
    }

    /// Get mempool statistics
    pub async fn get_statistics(&self) -> MempoolStatistics {
        let transactions = self.transactions.read().await;
        let total_size = *self.total_size.read().await;

        let mut priority_counts = HashMap::new();
        let mut total_fees = 0u64;

        for mempool_tx in transactions.values() {
            let count = priority_counts.entry(mempool_tx.priority).or_insert(0);
            *count += 1;
            total_fees += mempool_tx.transaction.fee;
        }

        MempoolStatistics {
            total_transactions: transactions.len(),
            total_size_bytes: total_size,
            priority_distribution: priority_counts,
            total_fees,
            average_fee_per_byte: if total_size > 0 {
                total_fees as f64 / total_size as f64
            } else {
                0.0
            },
        }
    }

    /// Add transaction to priority queue
    async fn add_to_priority_queue(&self, tx_hash: &Hash, mempool_tx: &MempoolTransaction) {
        let score = mempool_tx.calculate_score();
        let mut priority_queues = self.priority_queues.write().await;
        let queue = priority_queues.entry(mempool_tx.priority).or_insert_with(BTreeMap::new);
        queue.entry(score).or_insert_with(Vec::new).push(*tx_hash);
    }

    /// Remove transaction from priority queue
    async fn remove_from_priority_queue(&self, mempool_tx: &MempoolTransaction) {
        let score = mempool_tx.calculate_score();
        let mut priority_queues = self.priority_queues.write().await;
        if let Some(queue) = priority_queues.get_mut(&mempool_tx.priority) {
            if let Some(tx_hashes) = queue.get_mut(&score) {
                tx_hashes.retain(|&hash| hash != mempool_tx.transaction.hash());
                if tx_hashes.is_empty() {
                    queue.remove(&score);
                }
            }
        }
    }

    /// Validate transaction before adding to mempool
    fn validate_transaction(&self, transaction: &Transaction) -> Result<()> {
        // Basic transaction validation
        transaction.validate()?;

        // Check for double spending in mempool
        // This would require checking against current mempool transactions
        // For now, we'll do basic validation

        Ok(())
    }
}

/// Mempool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolStatistics {
    pub total_transactions: usize,
    pub total_size_bytes: usize,
    pub priority_distribution: HashMap<TransactionPriority, usize>,
    pub total_fees: u64,
    pub average_fee_per_byte: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::{TransactionInput, TransactionOutput, TransactionType};

    #[tokio::test]
    async fn test_mempool_creation() {
        let mempool = Mempool::new();
        let stats = mempool.get_statistics().await;
        assert_eq!(stats.total_transactions, 0);
        assert_eq!(stats.total_size_bytes, 0);
    }

    #[tokio::test]
    async fn test_add_transaction() {
        let mempool = Mempool::new();
        let transaction = create_test_transaction();
        
        let result = mempool.add_transaction(transaction, TransactionPriority::Normal).await;
        assert!(result.is_ok());

        let stats = mempool.get_statistics().await;
        assert_eq!(stats.total_transactions, 1);
    }

    #[tokio::test]
    async fn test_remove_transaction() {
        let mempool = Mempool::new();
        let transaction = create_test_transaction();
        let tx_hash = transaction.hash();
        
        mempool.add_transaction(transaction, TransactionPriority::Normal).await.unwrap();
        
        let removed = mempool.remove_transaction(&tx_hash).await.unwrap();
        assert!(removed.is_some());

        let stats = mempool.get_statistics().await;
        assert_eq!(stats.total_transactions, 0);
    }

    fn create_test_transaction() -> Transaction {
        Transaction {
            tx_type: TransactionType::Transfer,
            inputs: vec![TransactionInput::new(
                crate::blockchain::TransactionOutpoint::new(Hash::random(), 0),
                vec![1, 2, 3, 4],
            )],
            outputs: vec![TransactionOutput::new(1000, crate::crypto::PublicKey::random())],
            fee: 100,
            timestamp: timestamp(),
            signature: None,
            data: None,
        }
    }
}
