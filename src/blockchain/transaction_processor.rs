//! Transaction Processing Engine
//!
//! This module provides comprehensive transaction processing including validation,
//! execution, and integration with the UTXO set and blockchain.

use crate::{
    blockchain::{
        Transaction, TransactionInput, TransactionOutput, TransactionType, TransactionOutpoint,
        Block, BlockHeader, UTXOManager, CoinSelectionAlgorithm,
    },
    crypto::{Hash, PublicKey, PrivateKey, Signature},
    consensus::ConsensusEngine,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Transaction processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    /// Transaction hash
    pub tx_hash: Hash,
    /// Processing status
    pub status: TransactionStatus,
    /// Block height (if confirmed)
    pub block_height: Option<u64>,
    /// Confirmation count
    pub confirmations: u32,
    /// Processing timestamp
    pub timestamp: u64,
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Transaction processing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    /// Transaction is pending in mempool
    Pending,
    /// Transaction is confirmed in a block
    Confirmed,
    /// Transaction failed validation
    Failed,
    /// Transaction was rejected
    Rejected,
}

/// Transaction fee calculation
#[derive(Debug, Clone, Copy)]
pub struct FeeCalculator {
    /// Base fee per transaction
    pub base_fee: u64,
    /// Fee per byte
    pub fee_per_byte: u64,
    /// Minimum fee
    pub min_fee: u64,
    /// Maximum fee
    pub max_fee: u64,
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self {
            base_fee: 100,      // 100 satoshis
            fee_per_byte: 10,   // 10 satoshis per byte
            min_fee: 100,       // 100 satoshis minimum
            max_fee: 10000,     // 10,000 satoshis maximum
        }
    }
}

impl FeeCalculator {
    /// Calculate fee for a transaction
    pub fn calculate_fee(&self, transaction: &Transaction) -> u64 {
        let size = transaction.size() as u64;
        let calculated_fee = self.base_fee + (size * self.fee_per_byte);
        
        calculated_fee.clamp(self.min_fee, self.max_fee)
    }

    /// Estimate fee for a transaction with given inputs and outputs
    pub fn estimate_fee(&self, input_count: usize, output_count: usize) -> u64 {
        // Estimate transaction size
        let estimated_size = (input_count * 148) + (output_count * 34) + 10; // Base transaction size
        let calculated_fee = self.base_fee + (estimated_size as u64 * self.fee_per_byte);
        
        calculated_fee.clamp(self.min_fee, self.max_fee)
    }
}

/// Transaction processor
#[derive(Debug)]
pub struct TransactionProcessor {
    /// UTXO manager
    utxo_manager: Arc<UTXOManager>,
    /// Consensus engine
    consensus_engine: Arc<ConsensusEngine>,
    /// Fee calculator
    fee_calculator: FeeCalculator,
    /// Transaction mempool
    mempool: Arc<tokio::sync::RwLock<HashMap<Hash, Transaction>>>,
    /// Processing statistics
    stats: Arc<tokio::sync::RwLock<ProcessingStats>>,
}

/// Processing statistics
#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    pub total_processed: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_fees_collected: u64,
    pub average_processing_time: f64,
}

impl TransactionProcessor {
    /// Create a new transaction processor
    pub fn new(utxo_manager: Arc<UTXOManager>, consensus_engine: Arc<ConsensusEngine>) -> Self {
        Self {
            utxo_manager,
            consensus_engine,
            fee_calculator: FeeCalculator::default(),
            mempool: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            stats: Arc::new(tokio::sync::RwLock::new(ProcessingStats::default())),
        }
    }

    /// Process a new transaction
    pub async fn process_transaction(&self, transaction: Transaction) -> Result<TransactionResult> {
        let start_time = std::time::Instant::now();
        let tx_hash = transaction.hash();
        
        // Validate transaction
        if !self.validate_transaction(&transaction).await? {
            return Ok(TransactionResult {
                tx_hash,
                status: TransactionStatus::Failed,
                block_height: None,
                confirmations: 0,
                timestamp: crate::utils::timestamp(),
                error: Some("Transaction validation failed".to_string()),
            });
        }

        // Check if transaction already exists
        {
            let mempool = self.mempool.read().await;
            if mempool.contains_key(&tx_hash) {
                return Ok(TransactionResult {
                    tx_hash,
                    status: TransactionStatus::Pending,
                    block_height: None,
                    confirmations: 0,
                    timestamp: crate::utils::timestamp(),
                    error: None,
                });
            }
        }

        // Add to mempool
        {
            let mut mempool = self.mempool.write().await;
            mempool.insert(tx_hash, transaction.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_processed += 1;
            stats.successful_transactions += 1;
            
            let processing_time = start_time.elapsed().as_millis() as f64;
            stats.average_processing_time = 
                (stats.average_processing_time * (stats.total_processed - 1) as f64 + processing_time) 
                / stats.total_processed as f64;
        }

        Ok(TransactionResult {
            tx_hash,
            status: TransactionStatus::Pending,
            block_height: None,
            confirmations: 0,
            timestamp: crate::utils::timestamp(),
            error: None,
        })
    }

    /// Validate a transaction
    pub async fn validate_transaction(&self, transaction: &Transaction) -> Result<bool> {
        // Basic transaction validation
        transaction.validate()?;

        // Check transaction size
        if transaction.size() > 1_000_000 { // 1MB limit
            return Ok(false);
        }

        // Validate UTXO references
        if !self.utxo_manager.validate_transaction(transaction)? {
            return Ok(false);
        }

        // Check fee
        let calculated_fee = self.fee_calculator.calculate_fee(transaction);
        if transaction.fee < calculated_fee {
            return Ok(false);
        }

        // Validate signatures
        if !self.validate_signatures(transaction).await? {
            return Ok(false);
        }

        // Consensus validation
        if !self.consensus_engine.validate_transaction(transaction).await? {
            return Ok(false);
        }

        Ok(true)
    }

    /// Validate transaction signatures
    async fn validate_signatures(&self, transaction: &Transaction) -> Result<bool> {
        if transaction.tx_type == TransactionType::Coinbase {
            return Ok(true); // Coinbase transactions don't need signature validation
        }

        for input in &transaction.inputs {
            // Get the UTXO being spent
            let utxo = self.utxo_manager.get_utxo(&input.previous_output)?
                .ok_or_else(|| UnicoinError::Blockchain(
                    "UTXO not found for signature validation".to_string()
                ))?;

            // Extract public key from UTXO output
            let public_key = utxo.output.address.clone();
            
            // Create message hash for signing
            let message = transaction.hash_for_signing().to_bytes();
            
            // Verify signature
            if let Some(signature) = &transaction.signature {
                let pub_key = PublicKey::from_bytes(public_key)?;
                if !pub_key.verify(&message, signature)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Create a transaction
    pub async fn create_transaction(
        &self,
        sender_address: Vec<u8>,
        recipient_address: Vec<u8>,
        amount: u64,
        fee_per_byte: Option<u64>,
        memo: Option<String>,
        algorithm: CoinSelectionAlgorithm,
    ) -> Result<Transaction> {
        // Check sender balance
        let balance = self.utxo_manager.get_balance(&sender_address, false)?;
        if balance.confirmed < amount {
            return Err(UnicoinError::Blockchain(
                "Insufficient balance".to_string()
            ));
        }

        // Calculate fee
        let fee_rate = fee_per_byte.unwrap_or(self.fee_calculator.fee_per_byte);
        let estimated_fee = self.fee_calculator.estimate_fee(1, 2); // Assume 1 input, 2 outputs

        // Select UTXOs
        let selection = self.utxo_manager.select_utxos(
            &sender_address,
            amount,
            fee_rate,
            algorithm,
        )?;

        // Create transaction inputs
        let mut inputs = Vec::new();
        for utxo in &selection.selected_utxos {
            let input = TransactionInput::new_bitcoin(
                utxo.tx_id,
                utxo.output_index,
                vec![], // Signature will be added later
            );
            inputs.push(input);
        }

        // Create transaction outputs
        let mut outputs = Vec::new();
        
        // Recipient output
        let recipient_pubkey = PublicKey::from_bytes(recipient_address.clone())?;
        outputs.push(TransactionOutput::new(amount, recipient_pubkey));

        // Change output (if needed)
        if selection.change_amount > 546 { // Dust threshold
            let sender_pubkey = PublicKey::from_bytes(sender_address.clone())?;
            outputs.push(TransactionOutput::new(selection.change_amount, sender_pubkey));
        }

        // Create transaction
        let mut transaction = Transaction::new(
            TransactionType::Transfer,
            inputs,
            outputs,
            selection.fee_amount,
        );

        // Add memo if provided
        if let Some(memo_text) = memo {
            transaction.data = Some(memo_text.into_bytes());
        }

        Ok(transaction)
    }

    /// Sign a transaction
    pub async fn sign_transaction(
        &self,
        mut transaction: Transaction,
        private_key: &PrivateKey,
    ) -> Result<Transaction> {
        // Create signature
        let message = transaction.hash_for_signing().to_bytes();
        let signature = private_key.sign(&message)?;
        
        transaction.signature = Some(signature);
        
        // Re-validate after signing
        transaction.validate()?;
        
        Ok(transaction)
    }

    /// Process a block and update UTXO set
    pub async fn process_block(&self, block: &Block, height: u64) -> Result<()> {
        // Process all transactions in the block
        for transaction in &block.transactions {
            self.utxo_manager.process_transaction(transaction, height)?;
        }

        // Update blockchain height
        self.utxo_manager.update_height(height)?;

        // Remove processed transactions from mempool
        {
            let mut mempool = self.mempool.write().await;
            for transaction in &block.transactions {
                mempool.remove(&transaction.hash());
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_fees_collected += block.transactions.iter()
                .map(|tx| tx.fee)
                .sum::<u64>();
        }

        Ok(())
    }

    /// Get mempool transactions
    pub async fn get_mempool(&self) -> Result<Vec<Transaction>> {
        let mempool = self.mempool.read().await;
        Ok(mempool.values().cloned().collect())
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, tx_hash: &Hash) -> Result<TransactionResult> {
        // Check if in mempool
        {
            let mempool = self.mempool.read().await;
            if mempool.contains_key(tx_hash) {
                return Ok(TransactionResult {
                    tx_hash: *tx_hash,
                    status: TransactionStatus::Pending,
                    block_height: None,
                    confirmations: 0,
                    timestamp: crate::utils::timestamp(),
                    error: None,
                });
            }
        }

        // TODO: Check blockchain for confirmed transactions
        // This would require blockchain integration

        Err(UnicoinError::Blockchain(
            "Transaction not found".to_string()
        ))
    }

    /// Estimate transaction fee
    pub fn estimate_fee(&self, input_count: usize, output_count: usize) -> u64 {
        self.fee_calculator.estimate_fee(input_count, output_count)
    }

    /// Get processing statistics
    pub async fn get_stats(&self) -> Result<ProcessingStats> {
        let stats = self.stats.read().await;
        Ok(stats.clone())
    }

    /// Clear mempool (for testing)
    pub async fn clear_mempool(&self) {
        let mut mempool = self.mempool.write().await;
        mempool.clear();
    }

    /// Get UTXO manager reference
    pub fn utxo_manager(&self) -> Arc<UTXOManager> {
        self.utxo_manager.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Blockchain;

    #[tokio::test]
    async fn test_transaction_processor() {
        let blockchain = Blockchain::new().unwrap();
        let utxo_manager = Arc::new(UTXOManager::new());
        let consensus_engine = Arc::new(ConsensusEngine::new(blockchain).unwrap());
        let processor = TransactionProcessor::new(utxo_manager.clone(), consensus_engine);

        // Test fee calculation
        let fee = processor.estimate_fee(1, 2);
        assert!(fee > 0);

        // Test transaction creation
        let sender_address = vec![1, 2, 3, 4];
        let recipient_address = vec![5, 6, 7, 8];
        
        // Add some UTXOs for the sender
        let tx_id = crate::crypto::Hash::random();
        let output = TransactionOutput::new(10000, PublicKey::from_bytes(sender_address.clone()).unwrap());
        let utxo = UTXO::new(tx_id, 0, output, 100, false);
        utxo_manager.add_utxo(utxo).unwrap();
        utxo_manager.update_height(101).unwrap();

        let transaction = processor.create_transaction(
            sender_address,
            recipient_address,
            5000,
            None,
            Some("Test transaction".to_string()),
            CoinSelectionAlgorithm::MinimizeInputs,
        ).await.unwrap();

        assert_eq!(transaction.outputs.len(), 2); // Recipient + change
        assert!(transaction.fee > 0);
    }

    #[test]
    fn test_fee_calculator() {
        let calculator = FeeCalculator::default();
        
        let transaction = Transaction::new(
            TransactionType::Transfer,
            vec![],
            vec![],
            100,
        );

        let fee = calculator.calculate_fee(&transaction);
        assert!(fee >= calculator.min_fee);
        assert!(fee <= calculator.max_fee);

        let estimated = calculator.estimate_fee(2, 3);
        assert!(estimated > 0);
    }
}
