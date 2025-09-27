//! Wallet implementation
//!
//! This module provides wallet functionality for Unicoin including
//! hierarchical deterministic wallets, multi-signature support, and
//! secure key management.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod derivation;
pub mod multisig;

pub use derivation::{HDWallet, DerivationPath, BIP44CoinType, BIP44Purpose};
pub use multisig::{MultiSigWallet, MultiSigConfig, MultiSigTransaction, MultiSigStatus};

/// Main wallet structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    /// Wallet identifier
    pub id: String,
    /// List of accounts
    pub accounts: Vec<Account>,
    /// Wallet metadata
    pub metadata: WalletMetadata,
}

/// Account within a wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account identifier
    pub id: String,
    /// Account name
    pub name: String,
    /// Master public key
    pub master_public_key: crate::crypto::PublicKey,
    /// Account index
    pub account_index: u32,
    /// List of addresses
    pub addresses: Vec<Address>,
}

/// Address within an account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Address string
    pub address: String,
    /// Address index
    pub address_index: u32,
    /// Public key
    pub public_key: crate::crypto::PublicKey,
    /// Balance in satoshis
    pub balance: u64,
}

/// Wallet metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    /// Wallet name
    pub name: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Last updated timestamp
    pub updated_at: u64,
    /// Wallet version
    pub version: String,
}

impl Wallet {
    /// Create a new wallet
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            accounts: Vec::new(),
            metadata: WalletMetadata {
                name,
                created_at: crate::utils::timestamp(),
                updated_at: crate::utils::timestamp(),
                version: crate::VERSION.to_string(),
            },
        }
    }

    /// Add a new account to the wallet
    pub fn add_account(&mut self, name: String, account_index: u32) -> Result<()> {
        let (private_key, public_key) = crate::crypto::generate_key_pair(crate::crypto::CryptoAlgorithm::Secp256k1)?;
        
        let account = Account {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            master_public_key: public_key,
            account_index,
            addresses: Vec::new(),
        };

        self.accounts.push(account);
        self.metadata.updated_at = crate::utils::timestamp();
        Ok(())
    }

    /// Generate a new address for an account
    pub fn generate_address(&mut self, account_id: &str, address_index: u32) -> Result<String> {
        let account = self.accounts.iter_mut()
            .find(|acc| acc.id == account_id)
            .ok_or_else(|| UnicoinError::Wallet("Account not found".to_string()))?;

        let (_, public_key) = crate::crypto::generate_key_pair(crate::crypto::CryptoAlgorithm::Secp256k1)?;
        let address = public_key.to_hash().to_hex();

        let address_entry = Address {
            address: address.clone(),
            address_index,
            public_key,
            balance: 0,
        };

        account.addresses.push(address_entry);
        self.metadata.updated_at = crate::utils::timestamp();

        Ok(address)
    }

    /// Get wallet balance
    pub fn get_balance(&self) -> u64 {
        self.accounts.iter()
            .flat_map(|account| &account.addresses)
            .map(|address| address.balance)
            .sum()
    }

    /// Update address balance
    pub fn update_address_balance(&mut self, address: &str, balance: u64) -> Result<()> {
        for account in &mut self.accounts {
            for addr in &mut account.addresses {
                if addr.address == address {
                    addr.balance = balance;
                    self.metadata.updated_at = crate::utils::timestamp();
                    return Ok(());
                }
            }
        }
        Err(UnicoinError::Wallet("Address not found".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new("Test Wallet".to_string());
        
        assert_eq!(wallet.metadata.name, "Test Wallet");
        assert_eq!(wallet.accounts.len(), 0);
        assert!(!wallet.id.is_empty());
    }

    #[test]
    fn test_account_creation() {
        let mut wallet = Wallet::new("Test Wallet".to_string());
        wallet.add_account("Test Account".to_string(), 0).unwrap();
        
        assert_eq!(wallet.accounts.len(), 1);
        assert_eq!(wallet.accounts[0].name, "Test Account");
    }

    #[test]
    fn test_address_generation() {
        let mut wallet = Wallet::new("Test Wallet".to_string());
        wallet.add_account("Test Account".to_string(), 0).unwrap();
        
        let account_id = &wallet.accounts[0].id;
        let address = wallet.generate_address(account_id, 0).unwrap();
        
        assert!(!address.is_empty());
        assert_eq!(wallet.accounts[0].addresses.len(), 1);
    }
}
