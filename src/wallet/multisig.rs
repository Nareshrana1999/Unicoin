//! Multi-signature wallet implementation
//!
//! This module provides multi-signature wallet functionality for Unicoin.

use crate::{
    crypto::{PrivateKey, PublicKey, Signature, CryptoAlgorithm},
    utils::timestamp,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-signature wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigConfig {
    /// Total number of signers
    pub total_signers: u8,
    /// Required number of signatures
    pub required_signatures: u8,
    /// Wallet identifier
    pub wallet_id: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Signer public keys
    pub signer_keys: Vec<PublicKey>,
}

/// Multi-signature transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    /// Transaction data to be signed
    pub transaction_data: Vec<u8>,
    /// Transaction hash
    pub transaction_hash: crate::crypto::Hash,
    /// Signatures collected
    pub signatures: HashMap<u8, Signature>,
    /// Transaction status
    pub status: MultiSigStatus,
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp
    pub expires_at: u64,
}

/// Multi-signature transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultiSigStatus {
    /// Pending signatures
    Pending,
    /// Sufficient signatures collected
    Ready,
    /// Transaction executed
    Executed,
    /// Transaction expired
    Expired,
    /// Transaction rejected
    Rejected,
}

/// Multi-signature wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    /// Wallet configuration
    pub config: MultiSigConfig,
    /// Pending transactions
    pub pending_transactions: HashMap<crate::crypto::Hash, MultiSigTransaction>,
    /// Executed transactions
    pub executed_transactions: Vec<crate::crypto::Hash>,
    /// Wallet balance
    pub balance: u64,
}

/// Threshold signature scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSignature {
    /// Signature data
    pub signature_data: Vec<u8>,
    /// Threshold used
    pub threshold: u8,
    /// Total participants
    pub total_participants: u8,
    /// Participant indices
    pub participant_indices: Vec<u8>,
}

impl MultiSigConfig {
    /// Create a new multi-signature configuration
    pub fn new(
        total_signers: u8,
        required_signatures: u8,
        signer_keys: Vec<PublicKey>,
    ) -> Result<Self> {
        if required_signatures > total_signers {
            return Err(UnicoinError::Wallet(
                "Required signatures cannot exceed total signers".to_string()
            ));
        }

        if required_signatures == 0 {
            return Err(UnicoinError::Wallet(
                "At least one signature is required".to_string()
            ));
        }

        if signer_keys.len() != total_signers as usize {
            return Err(UnicoinError::Wallet(
                "Number of signer keys must match total signers".to_string()
            ));
        }

        // Check for duplicate keys
        let mut key_hashes = std::collections::HashSet::new();
        for key in &signer_keys {
            let key_hash = key.to_hash();
            if !key_hashes.insert(key_hash) {
                return Err(UnicoinError::Wallet(
                    "Duplicate signer keys are not allowed".to_string()
                ));
            }
        }

        Ok(Self {
            total_signers,
            required_signatures,
            wallet_id: uuid::Uuid::new_v4().to_string(),
            created_at: timestamp(),
            signer_keys,
        })
    }

    /// Get signer index by public key
    pub fn get_signer_index(&self, public_key: &PublicKey) -> Option<u8> {
        self.signer_keys.iter().position(|key| key == public_key).map(|i| i as u8)
    }

    /// Validate signer key
    pub fn is_valid_signer(&self, public_key: &PublicKey) -> bool {
        self.signer_keys.contains(public_key)
    }

    /// Get wallet address (derived from all signer keys)
    pub fn get_wallet_address(&self) -> crate::crypto::Hash {
        // Create a deterministic hash from all signer keys
        let mut combined_data = Vec::new();
        for key in &self.signer_keys {
            combined_data.extend_from_slice(key.to_hash().as_bytes());
        }
        
        // Add configuration data
        combined_data.extend_from_slice(&self.required_signatures.to_le_bytes());
        combined_data.extend_from_slice(&self.total_signers.to_le_bytes());
        
        crate::crypto::sha256(&combined_data)
    }
}

impl MultiSigTransaction {
    /// Create a new multi-signature transaction
    pub fn new(transaction_data: Vec<u8>, expires_in_seconds: u64) -> Self {
        let transaction_hash = crate::crypto::sha256(&transaction_data);
        let expires_at = timestamp() + expires_in_seconds;

        Self {
            transaction_data,
            transaction_hash,
            signatures: HashMap::new(),
            status: MultiSigStatus::Pending,
            created_at: timestamp(),
            expires_at,
        }
    }

    /// Add a signature to the transaction
    pub fn add_signature(&mut self, signer_index: u8, signature: Signature) -> Result<()> {
        if self.status != MultiSigStatus::Pending {
            return Err(UnicoinError::Wallet(
                "Cannot add signature to non-pending transaction".to_string()
            ));
        }

        if self.is_expired() {
            self.status = MultiSigStatus::Expired;
            return Err(UnicoinError::Wallet(
                "Transaction has expired".to_string()
            ));
        }

        self.signatures.insert(signer_index, signature);
        Ok(())
    }

    /// Check if transaction has sufficient signatures
    pub fn has_sufficient_signatures(&self, required: u8) -> bool {
        self.signatures.len() >= required as usize
    }

    /// Check if transaction is expired
    pub fn is_expired(&self) -> bool {
        timestamp() > self.expires_at
    }

    /// Update transaction status
    pub fn update_status(&mut self, required_signatures: u8) {
        if self.is_expired() {
            self.status = MultiSigStatus::Expired;
        } else if self.has_sufficient_signatures(required_signatures) {
            self.status = MultiSigStatus::Ready;
        } else {
            self.status = MultiSigStatus::Pending;
        }
    }

    /// Get signature count
    pub fn signature_count(&self) -> u8 {
        self.signatures.len() as u8
    }

    /// Get remaining time until expiration
    pub fn remaining_time(&self) -> u64 {
        if self.expires_at > timestamp() {
            self.expires_at - timestamp()
        } else {
            0
        }
    }
}

impl MultiSigWallet {
    /// Create a new multi-signature wallet
    pub fn new(config: MultiSigConfig) -> Self {
        Self {
            config,
            pending_transactions: HashMap::new(),
            executed_transactions: Vec::new(),
            balance: 0,
        }
    }

    /// Create a transaction for signing
    pub fn create_transaction(&mut self, transaction_data: Vec<u8>, expires_in_seconds: u64) -> crate::crypto::Hash {
        let mut multisig_tx = MultiSigTransaction::new(transaction_data, expires_in_seconds);
        multisig_tx.update_status(self.config.required_signatures);
        
        let tx_hash = multisig_tx.transaction_hash;
        self.pending_transactions.insert(tx_hash, multisig_tx);
        
        tx_hash
    }

    /// Sign a transaction
    pub fn sign_transaction(
        &mut self,
        tx_hash: &crate::crypto::Hash,
        signer_private_key: &PrivateKey,
    ) -> Result<()> {
        // Verify the signer is authorized
        let signer_public_key = signer_private_key.public_key();
        let signer_index = self.config.get_signer_index(&signer_public_key)
            .ok_or_else(|| UnicoinError::Wallet("Unauthorized signer".to_string()))?;

        // Get the transaction
        let multisig_tx = self.pending_transactions.get_mut(tx_hash)
            .ok_or_else(|| UnicoinError::Wallet("Transaction not found".to_string()))?;

        // Create signature
        let signature = signer_private_key.sign(&multisig_tx.transaction_data)?;

        // Add signature
        multisig_tx.add_signature(signer_index, signature)?;

        // Update status
        multisig_tx.update_status(self.config.required_signatures);

        Ok(())
    }

    /// Execute a transaction (when sufficient signatures are collected)
    pub fn execute_transaction(&mut self, tx_hash: &crate::crypto::Hash) -> Result<()> {
        let multisig_tx = self.pending_transactions.get_mut(tx_hash)
            .ok_or_else(|| UnicoinError::Wallet("Transaction not found".to_string()))?;

        if multisig_tx.status != MultiSigStatus::Ready {
            return Err(UnicoinError::Wallet(
                "Transaction is not ready for execution".to_string()
            ));
        }

        // Verify all signatures
        for (signer_index, signature) in &multisig_tx.signatures {
            let signer_public_key = &self.config.signer_keys[*signer_index as usize];
            let is_valid = crate::crypto::verify_signature(
                &multisig_tx.transaction_data,
                signature,
                signer_public_key,
            )?;

            if !is_valid {
                return Err(UnicoinError::Wallet(
                    "Invalid signature found".to_string()
                ));
            }
        }

        // Mark as executed
        multisig_tx.status = MultiSigStatus::Executed;
        self.executed_transactions.push(*tx_hash);

        // Remove from pending
        self.pending_transactions.remove(tx_hash);

        Ok(())
    }

    /// Reject a transaction
    pub fn reject_transaction(&mut self, tx_hash: &crate::crypto::Hash) -> Result<()> {
        if let Some(multisig_tx) = self.pending_transactions.get_mut(tx_hash) {
            multisig_tx.status = MultiSigStatus::Rejected;
            Ok(())
        } else {
            Err(UnicoinError::Wallet("Transaction not found".to_string()))
        }
    }

    /// Get pending transactions
    pub fn get_pending_transactions(&self) -> Vec<&MultiSigTransaction> {
        self.pending_transactions.values().collect()
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, tx_hash: &crate::crypto::Hash) -> Option<&MultiSigTransaction> {
        self.pending_transactions.get(tx_hash)
    }

    /// Clean up expired transactions
    pub fn cleanup_expired_transactions(&mut self) -> usize {
        let mut expired_hashes = Vec::new();
        
        for (hash, tx) in &self.pending_transactions {
            if tx.is_expired() {
                expired_hashes.push(*hash);
            }
        }

        let expired_count = expired_hashes.len();
        for hash in expired_hashes {
            if let Some(tx) = self.pending_transactions.get_mut(&hash) {
                tx.status = MultiSigStatus::Expired;
            }
        }

        expired_count
    }

    /// Get wallet statistics
    pub fn get_statistics(&self) -> MultiSigStatistics {
        MultiSigStatistics {
            total_signers: self.config.total_signers,
            required_signatures: self.config.required_signatures,
            pending_transactions: self.pending_transactions.len(),
            executed_transactions: self.executed_transactions.len(),
            balance: self.balance,
            wallet_address: self.config.get_wallet_address(),
        }
    }

    /// Update wallet balance
    pub fn update_balance(&mut self, new_balance: u64) {
        self.balance = new_balance;
    }
}

/// Multi-signature wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigStatistics {
    pub total_signers: u8,
    pub required_signatures: u8,
    pub pending_transactions: usize,
    pub executed_transactions: usize,
    pub balance: u64,
    pub wallet_address: crate::crypto::Hash,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multisig_config_creation() {
        let signer_keys = vec![
            PublicKey::random(),
            PublicKey::random(),
            PublicKey::random(),
        ];

        let config = MultiSigConfig::new(3, 2, signer_keys).unwrap();
        assert_eq!(config.total_signers, 3);
        assert_eq!(config.required_signatures, 2);
        assert_eq!(config.signer_keys.len(), 3);
    }

    #[test]
    fn test_multisig_config_validation() {
        let signer_keys = vec![PublicKey::random(), PublicKey::random()];

        // Invalid: required > total
        let result = MultiSigConfig::new(2, 3, signer_keys.clone());
        assert!(result.is_err());

        // Invalid: required = 0
        let result = MultiSigConfig::new(2, 0, signer_keys.clone());
        assert!(result.is_err());

        // Valid
        let result = MultiSigConfig::new(2, 2, signer_keys);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multisig_transaction() {
        let tx_data = b"test transaction data".to_vec();
        let mut multisig_tx = MultiSigTransaction::new(tx_data, 3600); // 1 hour

        assert_eq!(multisig_tx.status, MultiSigStatus::Pending);
        assert_eq!(multisig_tx.signature_count(), 0);
        assert!(!multisig_tx.is_expired());
    }

    #[test]
    fn test_multisig_wallet_operations() {
        let signer_keys = vec![
            PublicKey::random(),
            PublicKey::random(),
            PublicKey::random(),
        ];
        let config = MultiSigConfig::new(3, 2, signer_keys).unwrap();
        let mut wallet = MultiSigWallet::new(config);

        let tx_data = b"test transaction".to_vec();
        let tx_hash = wallet.create_transaction(tx_data, 3600);

        assert!(wallet.get_transaction(&tx_hash).is_some());
        assert_eq!(wallet.get_pending_transactions().len(), 1);
    }

    #[test]
    fn test_signature_collection() {
        let signer_keys = vec![
            PublicKey::random(),
            PublicKey::random(),
            PublicKey::random(),
        ];
        let config = MultiSigConfig::new(3, 2, signer_keys).unwrap();
        let mut wallet = MultiSigWallet::new(config);

        let tx_data = b"test transaction".to_vec();
        let tx_hash = wallet.create_transaction(tx_data, 3600);

        // This test would require actual private keys to sign
        // For now, we'll just verify the structure
        assert!(wallet.get_transaction(&tx_hash).is_some());
    }
}
