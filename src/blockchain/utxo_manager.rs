//! Advanced UTXO Management System
//!
//! This module provides comprehensive UTXO management with real transaction processing,
//! balance calculation, and efficient querying.

use crate::{
    blockchain::{Transaction, TransactionInput, TransactionOutput, TransactionOutpoint, TransactionType},
    crypto::{Hash, PublicKey},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};

/// Unspent Transaction Output with metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UTXO {
    /// Transaction ID that created this output
    pub tx_id: Hash,
    /// Output index within the transaction
    pub output_index: u32,
    /// The actual output
    pub output: TransactionOutput,
    /// Block height when this UTXO was created
    pub height: u64,
    /// Whether this UTXO is coinbase (from mining/staking reward)
    pub is_coinbase: bool,
    /// Confirmation count
    pub confirmations: u64,
    /// Creation timestamp
    pub created_at: u64,
    /// Spending transaction ID (if spent)
    pub spent_by: Option<Hash>,
}

impl UTXO {
    /// Create a new UTXO
    pub fn new(
        tx_id: Hash,
        output_index: u32,
        output: TransactionOutput,
        height: u64,
        is_coinbase: bool,
    ) -> Self {
        Self {
            tx_id,
            output_index,
            output,
            height,
            is_coinbase,
            confirmations: 0,
            created_at: crate::utils::timestamp(),
            spent_by: None,
        }
    }

    /// Check if this UTXO is spendable
    pub fn is_spendable(&self, current_height: u64) -> bool {
        if self.spent_by.is_some() {
            return false;
        }

        if self.is_coinbase {
            // Coinbase outputs need 100 confirmations
            current_height.saturating_sub(self.height) >= 100
        } else {
            // Regular outputs need 1 confirmation
            current_height.saturating_sub(self.height) >= 1
        }
    }

    /// Update confirmation count
    pub fn update_confirmations(&mut self, current_height: u64) {
        self.confirmations = current_height.saturating_sub(self.height);
    }

    /// Mark as spent
    pub fn mark_spent(&mut self, spending_tx_id: Hash) {
        self.spent_by = Some(spending_tx_id);
    }

    /// Get the outpoint for this UTXO
    pub fn outpoint(&self) -> TransactionOutpoint {
        TransactionOutpoint::new(self.tx_id, self.output_index)
    }
}

/// Address balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    /// Total confirmed balance
    pub confirmed: u64,
    /// Unconfirmed balance
    pub unconfirmed: u64,
    /// Total balance
    pub total: u64,
    /// Number of UTXOs
    pub utxo_count: usize,
    /// Last activity timestamp
    pub last_activity: Option<u64>,
}

/// UTXO selection algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoinSelectionAlgorithm {
    /// Select UTXOs to minimize the number of inputs
    MinimizeInputs,
    /// Select UTXOs to minimize change output
    MinimizeChange,
    /// Select UTXOs randomly (for privacy)
    Random,
    /// Select UTXOs by age (oldest first)
    OldestFirst,
}

/// Coin selection result
#[derive(Debug, Clone)]
pub struct CoinSelectionResult {
    /// Selected UTXOs
    pub selected_utxos: Vec<UTXO>,
    /// Total input amount
    pub total_input: u64,
    /// Change amount (if any)
    pub change_amount: u64,
    /// Fee amount
    pub fee_amount: u64,
}

/// Advanced UTXO Manager
#[derive(Debug)]
pub struct UTXOManager {
    /// Map of address to UTXOs
    utxos_by_address: Arc<RwLock<HashMap<Vec<u8>, Vec<UTXO>>>>,
    /// Map of (tx_id, output_index) to UTXO for quick lookup
    utxo_map: Arc<RwLock<HashMap<(Hash, u32), UTXO>>>,
    /// Total supply tracking
    total_supply: Arc<RwLock<u64>>,
    /// Current blockchain height
    current_height: Arc<RwLock<u64>>,
    /// Transaction index for quick lookups
    transaction_index: Arc<RwLock<HashMap<Hash, Vec<TransactionOutpoint>>>>,
}

impl UTXOManager {
    /// Create a new UTXO manager
    pub fn new() -> Self {
        Self {
            utxos_by_address: Arc::new(RwLock::new(HashMap::new())),
            utxo_map: Arc::new(RwLock::new(HashMap::new())),
            total_supply: Arc::new(RwLock::new(0)),
            current_height: Arc::new(RwLock::new(0)),
            transaction_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a new UTXO to the set
    pub fn add_utxo(&self, utxo: UTXO) -> Result<()> {
        let key = (utxo.tx_id, utxo.output_index);
        
        // Check if UTXO already exists
        {
            let utxo_map = self.utxo_map.read().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire UTXO map lock".to_string()
            ))?;
            
            if utxo_map.contains_key(&key) {
                return Err(UnicoinError::Blockchain(
                    "UTXO already exists".to_string()
                ));
            }
        }

        // Add to address index
        {
            let mut utxos_by_address = self.utxos_by_address.write().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire address index lock".to_string()
            ))?;
            
            let address = utxo.output.address.clone();
            utxos_by_address
                .entry(address)
                .or_insert_with(Vec::new)
                .push(utxo.clone());
        }

        // Add to main map
        {
            let mut utxo_map = self.utxo_map.write().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire UTXO map lock".to_string()
            ))?;
            utxo_map.insert(key, utxo.clone());
        }
        
        // Update total supply
        {
            let mut total_supply = self.total_supply.write().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire total supply lock".to_string()
            ))?;
            *total_supply += utxo.output.amount;
        }

        Ok(())
    }

    /// Remove a UTXO from the set (mark as spent)
    pub fn spend_utxo(&self, tx_id: Hash, output_index: u32, spending_tx_id: Hash) -> Result<UTXO> {
        let key = (tx_id, output_index);
        
        let mut utxo_map = self.utxo_map.write().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire UTXO map lock".to_string()
        ))?;
        
        let mut utxo = utxo_map.remove(&key).ok_or_else(|| UnicoinError::Blockchain(
            "UTXO not found".to_string()
        ))?;

        // Mark as spent
        utxo.mark_spent(spending_tx_id);

        // Update total supply
        {
            let mut total_supply = self.total_supply.write().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire total supply lock".to_string()
            ))?;
            *total_supply = total_supply.saturating_sub(utxo.output.amount);
        }

        // Remove from address index
        {
            let mut utxos_by_address = self.utxos_by_address.write().map_err(|_| UnicoinError::Blockchain(
                "Failed to acquire address index lock".to_string()
            ))?;
            
            if let Some(utxos) = utxos_by_address.get_mut(&utxo.output.address) {
                utxos.retain(|u| !(u.tx_id == tx_id && u.output_index == output_index));
                
                // Remove empty entries
                if utxos.is_empty() {
                    utxos_by_address.remove(&utxo.output.address);
                }
            }
        }

        Ok(utxo)
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &[u8], include_unconfirmed: bool) -> Result<AddressBalance> {
        let utxos_by_address = self.utxos_by_address.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire address index lock".to_string()
        ))?;
        
        let utxos = utxos_by_address.get(address).cloned().unwrap_or_default();
        let current_height = *self.current_height.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire height lock".to_string()
        ))?;

        let mut confirmed = 0u64;
        let mut unconfirmed = 0u64;
        let mut utxo_count = 0;
        let mut last_activity = None;

        for utxo in utxos {
            if utxo.spent_by.is_some() {
                continue; // Skip spent UTXOs
            }

            utxo_count += 1;
            
            if let Some(activity) = last_activity {
                if utxo.created_at > activity {
                    last_activity = Some(utxo.created_at);
                }
            } else {
                last_activity = Some(utxo.created_at);
            }

            if utxo.is_spendable(current_height) {
                confirmed += utxo.output.amount;
            } else {
                unconfirmed += utxo.output.amount;
            }
        }

        Ok(AddressBalance {
            confirmed,
            unconfirmed,
            total: confirmed + unconfirmed,
            utxo_count,
            last_activity,
        })
    }

    /// Get all UTXOs for an address
    pub fn get_utxos(&self, address: &[u8]) -> Result<Vec<UTXO>> {
        let utxos_by_address = self.utxos_by_address.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire address index lock".to_string()
        ))?;
        
        let utxos = utxos_by_address.get(address).cloned().unwrap_or_default();
        
        // Filter out spent UTXOs
        Ok(utxos.into_iter().filter(|utxo| utxo.spent_by.is_none()).collect())
    }

    /// Select UTXOs for a transaction
    pub fn select_utxos(
        &self,
        address: &[u8],
        amount: u64,
        fee_per_byte: u64,
        algorithm: CoinSelectionAlgorithm,
    ) -> Result<CoinSelectionResult> {
        let utxos = self.get_utxos(address)?;
        let current_height = *self.current_height.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire height lock".to_string()
        ))?;

        // Filter spendable UTXOs
        let spendable_utxos: Vec<UTXO> = utxos
            .into_iter()
            .filter(|utxo| utxo.is_spendable(current_height))
            .collect();

        if spendable_utxos.is_empty() {
            return Err(UnicoinError::Blockchain(
                "No spendable UTXOs available".to_string()
            ));
        }

        // Sort UTXOs based on algorithm
        let mut sorted_utxos = spendable_utxos;
        match algorithm {
            CoinSelectionAlgorithm::MinimizeInputs => {
                sorted_utxos.sort_by(|a, b| b.output.amount.cmp(&a.output.amount));
            }
            CoinSelectionAlgorithm::MinimizeChange => {
                sorted_utxos.sort_by(|a, b| a.output.amount.cmp(&b.output.amount));
            }
            CoinSelectionAlgorithm::OldestFirst => {
                sorted_utxos.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            }
            CoinSelectionAlgorithm::Random => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                sorted_utxos.shuffle(&mut rng);
            }
        }

        // Select UTXOs
        let mut selected_utxos = Vec::new();
        let mut total_input = 0u64;
        let mut estimated_fee = 0u64;

        for utxo in sorted_utxos {
            selected_utxos.push(utxo.clone());
            total_input += utxo.output.amount;

            // Estimate fee based on input count and outputs
            estimated_fee = (selected_utxos.len() * 148 + 2 * 34 + 10) as u64 * fee_per_byte;

            if total_input >= amount + estimated_fee {
                break;
            }
        }

        if total_input < amount + estimated_fee {
            return Err(UnicoinError::Blockchain(
                "Insufficient funds".to_string()
            ));
        }

        let change_amount = total_input - amount - estimated_fee;

        Ok(CoinSelectionResult {
            selected_utxos,
            total_input,
            change_amount,
            fee_amount: estimated_fee,
        })
    }

    /// Process a transaction and update UTXO set
    pub fn process_transaction(&self, transaction: &Transaction, height: u64) -> Result<()> {
        // Remove spent UTXOs
        for input in &transaction.inputs {
            if transaction.tx_type != TransactionType::Coinbase {
                self.spend_utxo(input.previous_tx_id, input.previous_output_index, transaction.hash())?;
            }
        }

        // Add new UTXOs
        for (output_index, output) in transaction.outputs.iter().enumerate() {
            let utxo = UTXO::new(
                transaction.hash(),
                output_index as u32,
                output.clone(),
                height,
                transaction.tx_type == TransactionType::Coinbase,
            );
            self.add_utxo(utxo)?;
        }

        Ok(())
    }

    /// Update blockchain height
    pub fn update_height(&self, height: u64) -> Result<()> {
        let mut current_height = self.current_height.write().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire height lock".to_string()
        ))?;
        *current_height = height;
        Ok(())
    }

    /// Get total supply
    pub fn get_total_supply(&self) -> Result<u64> {
        let total_supply = self.total_supply.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire total supply lock".to_string()
        ))?;
        Ok(*total_supply)
    }

    /// Get current height
    pub fn get_current_height(&self) -> Result<u64> {
        let current_height = self.current_height.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire height lock".to_string()
        ))?;
        Ok(*current_height)
    }

    /// Get UTXO by outpoint
    pub fn get_utxo(&self, outpoint: &TransactionOutpoint) -> Result<Option<UTXO>> {
        let utxo_map = self.utxo_map.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire UTXO map lock".to_string()
        ))?;
        
        let key = (outpoint.tx_id, outpoint.output_index);
        Ok(utxo_map.get(&key).cloned())
    }

    /// Get statistics
    pub fn get_stats(&self) -> Result<UTXOStats> {
        let utxo_map = self.utxo_map.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire UTXO map lock".to_string()
        ))?;
        
        let utxos_by_address = self.utxos_by_address.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire address index lock".to_string()
        ))?;
        
        let total_supply = self.total_supply.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire total supply lock".to_string()
        ))?;
        
        let current_height = self.current_height.read().map_err(|_| UnicoinError::Blockchain(
            "Failed to acquire height lock".to_string()
        ))?;

        Ok(UTXOStats {
            total_utxos: utxo_map.len(),
            total_addresses: utxos_by_address.len(),
            total_supply: *total_supply,
            current_height: *current_height,
        })
    }

    /// Validate transaction against UTXO set
    pub fn validate_transaction(&self, transaction: &Transaction) -> Result<bool> {
        let mut total_input = 0u64;
        let mut total_output = 0u64;

        // Validate inputs
        for input in &transaction.inputs {
            if transaction.tx_type == TransactionType::Coinbase {
                continue;
            }

            let utxo = self.get_utxo(&input.previous_output)?;
            if utxo.is_none() {
                return Ok(false); // UTXO not found
            }

            let utxo = utxo.unwrap();
            if utxo.spent_by.is_some() {
                return Ok(false); // Already spent
            }

            total_input += utxo.output.amount;
        }

        // Validate outputs
        for output in &transaction.outputs {
            total_output += output.amount;
        }

        // Check if inputs >= outputs + fee
        if total_input < total_output + transaction.fee {
            return Ok(false);
        }

        Ok(true)
    }
}

/// UTXO statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXOStats {
    pub total_utxos: usize,
    pub total_addresses: usize,
    pub total_supply: u64,
    pub current_height: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::Hash;

    #[test]
    fn test_utxo_creation() {
        let tx_id = Hash::random();
        let output = TransactionOutput::new(1000, PublicKey::random());
        let utxo = UTXO::new(tx_id, 0, output, 100, false);

        assert_eq!(utxo.tx_id, tx_id);
        assert_eq!(utxo.output_index, 0);
        assert_eq!(utxo.height, 100);
        assert!(!utxo.is_coinbase);
    }

    #[test]
    fn test_utxo_spendability() {
        let tx_id = Hash::random();
        let output = TransactionOutput::new(1000, PublicKey::random());
        let utxo = UTXO::new(tx_id, 0, output, 100, false);

        // Should not be spendable at same height
        assert!(!utxo.is_spendable(100));
        
        // Should be spendable after 1 confirmation
        assert!(utxo.is_spendable(101));

        // Test coinbase UTXO
        let coinbase_utxo = UTXO::new(tx_id, 0, output, 100, true);
        assert!(!coinbase_utxo.is_spendable(199));
        assert!(coinbase_utxo.is_spendable(200));
    }

    #[test]
    fn test_utxo_manager() {
        let manager = UTXOManager::new();
        
        let tx_id = Hash::random();
        let output = TransactionOutput::new(1000, PublicKey::random());
        let utxo = UTXO::new(tx_id, 0, output.clone(), 100, false);

        // Add UTXO
        assert!(manager.add_utxo(utxo.clone()).is_ok());

        // Get balance
        let balance = manager.get_balance(&output.address, false).unwrap();
        assert_eq!(balance.total, 1000);

        // Update height and check spendability
        manager.update_height(101).unwrap();
        let spendable_utxos = manager.get_utxos(&output.address).unwrap();
        assert!(!spendable_utxos.is_empty());
    }

    #[test]
    fn test_coin_selection() {
        let manager = UTXOManager::new();
        let address = vec![1, 2, 3, 4];
        
        // Add multiple UTXOs
        for i in 0..5 {
            let tx_id = Hash::random();
            let output = TransactionOutput::new((i + 1) * 1000, PublicKey::random());
            let utxo = UTXO::new(tx_id, 0, output, 100, false);
            manager.add_utxo(utxo).unwrap();
        }

        manager.update_height(101).unwrap();

        // Test coin selection
        let result = manager.select_utxos(&address, 2500, 10, CoinSelectionAlgorithm::MinimizeInputs).unwrap();
        assert!(result.total_input >= 2500);
        assert!(!result.selected_utxos.is_empty());
    }
}
