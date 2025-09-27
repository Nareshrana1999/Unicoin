use crate::crypto::hash::Hash;
use crate::blockchain::transaction::{TransactionOutput, TransactionInput};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Unspent Transaction Output (UTXO)
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
}

impl UTXO {
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
        }
    }

    /// Check if this UTXO is spendable
    pub fn is_spendable(&self, current_height: u64) -> bool {
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
}

/// UTXO Set - manages all unspent transaction outputs
#[derive(Debug)]
pub struct UTXOSet {
    /// Map of address to UTXOs
    utxos_by_address: HashMap<Vec<u8>, HashSet<UTXO>>,
    /// Map of (tx_id, output_index) to UTXO for quick lookup
    utxo_map: HashMap<(Hash, u32), UTXO>,
    /// Total supply tracking
    total_supply: u64,
    /// Current blockchain height
    current_height: u64,
}

impl UTXOSet {
    pub fn new() -> Self {
        Self {
            utxos_by_address: HashMap::new(),
            utxo_map: HashMap::new(),
            total_supply: 0,
            current_height: 0,
        }
    }

    /// Add a new UTXO to the set
    pub fn add_utxo(&mut self, utxo: UTXO) -> Result<(), String> {
        let key = (utxo.tx_id, utxo.output_index);
        
        if self.utxo_map.contains_key(&key) {
            return Err("UTXO already exists".to_string());
        }

        // Add to address index
        let address = utxo.output.address.clone();
        self.utxos_by_address
            .entry(address)
            .or_insert_with(HashSet::new)
            .insert(utxo.clone());

        // Add to main map
        self.utxo_map.insert(key, utxo);
        
        // Update total supply
        self.total_supply += self.utxo_map[&key].output.amount;

        Ok(())
    }

    /// Remove a UTXO from the set (when spent)
    pub fn remove_utxo(&mut self, tx_id: Hash, output_index: u32) -> Result<UTXO, String> {
        let key = (tx_id, output_index);
        
        if let Some(utxo) = self.utxo_map.remove(&key) {
            // Remove from address index
            if let Some(address_utxos) = self.utxos_by_address.get_mut(&utxo.output.address) {
                address_utxos.remove(&utxo);
            }
            
            // Update total supply
            self.total_supply = self.total_supply.saturating_sub(utxo.output.amount);
            
            Ok(utxo)
        } else {
            Err("UTXO not found".to_string())
        }
    }

    /// Get UTXO by transaction ID and output index
    pub fn get_utxo(&self, tx_id: Hash, output_index: u32) -> Option<&UTXO> {
        self.utxo_map.get(&(tx_id, output_index))
    }

    /// Get all UTXOs for an address
    pub fn get_utxos_for_address(&self, address: &[u8]) -> Vec<&UTXO> {
        self.utxos_by_address
            .get(address)
            .map(|utxos| utxos.iter().collect())
            .unwrap_or_default()
    }

    /// Get spendable UTXOs for an address
    pub fn get_spendable_utxos(&self, address: &[u8]) -> Vec<&UTXO> {
        self.get_utxos_for_address(address)
            .into_iter()
            .filter(|utxo| utxo.is_spendable(self.current_height))
            .collect()
    }

    /// Calculate total balance for an address
    pub fn get_balance(&self, address: &[u8]) -> u64 {
        self.get_spendable_utxos(address)
            .iter()
            .map(|utxo| utxo.output.amount)
            .sum()
    }

    /// Get total supply
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }

    /// Update blockchain height
    pub fn update_height(&mut self, new_height: u64) {
        self.current_height = new_height;
        
        // Update confirmation counts for all UTXOs
        for utxo in self.utxo_map.values_mut() {
            utxo.update_confirmations(new_height);
        }
    }

    /// Validate transaction inputs against UTXO set
    pub fn validate_inputs(&self, inputs: &[TransactionInput]) -> Result<(), String> {
        for input in inputs {
            let utxo = self.get_utxo(input.previous_tx_id, input.previous_output_index)
                .ok_or_else(|| "Input references non-existent UTXO".to_string())?;
            
            if !utxo.is_spendable(self.current_height) {
                return Err("Attempting to spend immature UTXO".to_string());
            }
        }
        Ok(())
    }

    /// Apply transaction to UTXO set
    pub fn apply_transaction(&mut self, tx: &crate::blockchain::transaction::Transaction) -> Result<(), String> {
        // Remove spent UTXOs
        for input in &tx.inputs {
            self.remove_utxo(input.previous_tx_id, input.previous_output_index)?;
        }
        
        // Add new UTXOs
        for (index, output) in tx.outputs.iter().enumerate() {
            let utxo = UTXO::new(
                tx.id,
                index as u32,
                output.clone(),
                self.current_height,
                tx.is_coinbase(),
            );
            self.add_utxo(utxo)?;
        }
        
        Ok(())
    }

    /// Revert transaction from UTXO set
    pub fn revert_transaction(&mut self, tx: &crate::blockchain::transaction::Transaction) -> Result<(), String> {
        // Remove new UTXOs
        for (index, _output) in tx.outputs.iter().enumerate() {
            self.remove_utxo(tx.id, index as u32)?;
        }
        
        // Restore spent UTXOs
        for input in &tx.inputs {
            // Note: In a real implementation, we'd need to restore the original UTXO
            // This is a simplified version
        }
        
        Ok(())
    }

    /// Get UTXO set statistics
    pub fn get_stats(&self) -> UTXOStats {
        UTXOStats {
            total_utxos: self.utxo_map.len(),
            total_supply: self.total_supply,
            unique_addresses: self.utxos_by_address.len(),
            current_height: self.current_height,
        }
    }
}

/// UTXO Set statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXOStats {
    pub total_utxos: usize,
    pub total_supply: u64,
    pub unique_addresses: usize,
    pub current_height: u64,
}

/// UTXO Provider trait for different storage backends
pub trait UTXOProvider {
    fn get_utxo(&self, tx_id: Hash, output_index: u32) -> Result<Option<UTXO>, String>;
    fn add_utxo(&mut self, utxo: UTXO) -> Result<(), String>;
    fn remove_utxo(&mut self, tx_id: Hash, output_index: u32) -> Result<Option<UTXO>, String>;
    fn get_utxos_for_address(&self, address: &[u8]) -> Result<Vec<UTXO>, String>;
    fn get_stats(&self) -> Result<UTXOStats, String>;
}

/// In-memory UTXO provider
pub struct MemoryUTXOProvider {
    utxo_set: Arc<RwLock<UTXOSet>>,
}

impl MemoryUTXOProvider {
    pub fn new() -> Self {
        Self {
            utxo_set: Arc::new(RwLock::new(UTXOSet::new())),
        }
    }
}

impl UTXOProvider for MemoryUTXOProvider {
    fn get_utxo(&self, tx_id: Hash, output_index: u32) -> Result<Option<UTXO>, String> {
        let utxo_set = self.utxo_set.read().map_err(|_| "Lock error".to_string())?;
        Ok(utxo_set.get_utxo(tx_id, output_index).cloned())
    }

    fn add_utxo(&mut self, utxo: UTXO) -> Result<(), String> {
        let mut utxo_set = self.utxo_set.write().map_err(|_| "Lock error".to_string())?;
        utxo_set.add_utxo(utxo)
    }

    fn remove_utxo(&mut self, tx_id: Hash, output_index: u32) -> Result<Option<UTXO>, String> {
        let mut utxo_set = self.utxo_set.write().map_err(|_| "Lock error".to_string())?;
        utxo_set.remove_utxo(tx_id, output_index)
    }

    fn get_utxos_for_address(&self, address: &[u8]) -> Result<Vec<UTXO>, String> {
        let utxo_set = self.utxo_set.read().map_err(|_| "Lock error".to_string())?;
        Ok(utxo_set.get_utxos_for_address(address).into_iter().cloned().collect())
    }

    fn get_stats(&self) -> Result<UTXOStats, String> {
        let utxo_set = self.utxo_set.read().map_err(|_| "Lock error".to_string())?;
        Ok(utxo_set.get_stats())
    }
}

impl Default for UTXOSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MemoryUTXOProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::Hash;
    use crate::blockchain::transaction::TransactionOutput;

    #[test]
    fn test_utxo_creation() {
        let tx_id = Hash::new([1u8; 32]);
        let output = TransactionOutput {
            address: vec![1, 2, 3, 4],
            amount: 1000,
            script_pubkey: vec![0x76, 0xa9, 0x14], // P2PKH script
        };
        
        let utxo = UTXO::new(tx_id, 0, output, 100, false);
        
        assert_eq!(utxo.tx_id, tx_id);
        assert_eq!(utxo.output_index, 0);
        assert_eq!(utxo.output.amount, 1000);
        assert_eq!(utxo.height, 100);
        assert!(!utxo.is_coinbase);
    }

    #[test]
    fn test_utxo_spendability() {
        let tx_id = Hash::new([1u8; 32]);
        let output = TransactionOutput {
            address: vec![1, 2, 3, 4],
            amount: 1000,
            script_pubkey: vec![0x76, 0xa9, 0x14],
        };
        
        // Regular UTXO
        let mut utxo = UTXO::new(tx_id, 0, output.clone(), 100, false);
        assert!(utxo.is_spendable(101)); // 1 confirmation
        
        // Coinbase UTXO
        let mut coinbase_utxo = UTXO::new(tx_id, 1, output, 100, true);
        assert!(!coinbase_utxo.is_spendable(150)); // 50 confirmations
        assert!(coinbase_utxo.is_spendable(201)); // 101 confirmations
    }

    #[test]
    fn test_utxo_set_operations() {
        let mut utxo_set = UTXOSet::new();
        let tx_id = Hash::new([1u8; 32]);
        let output = TransactionOutput {
            address: vec![1, 2, 3, 4],
            amount: 1000,
            script_pubkey: vec![0x76, 0xa9, 0x14],
        };
        
        let utxo = UTXO::new(tx_id, 0, output, 100, false);
        
        // Add UTXO
        assert!(utxo_set.add_utxo(utxo.clone()).is_ok());
        assert_eq!(utxo_set.get_total_supply(), 1000);
        
        // Get UTXO
        let retrieved = utxo_set.get_utxo(tx_id, 0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().output.amount, 1000);
        
        // Remove UTXO
        let removed = utxo_set.remove_utxo(tx_id, 0);
        assert!(removed.is_ok());
        assert_eq!(utxo_set.get_total_supply(), 0);
    }
}
