//! Hierarchical deterministic key derivation
//!
//! This module provides BIP32/BIP44 compliant key derivation for Unicoin wallets.

use crate::{
    crypto::{PrivateKey, PublicKey, CryptoAlgorithm, Hash},
    utils::{derive_child_key, generate_seed},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Key derivation path
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivationPath {
    /// Path components
    pub components: Vec<u32>,
    /// Whether this is a hardened path
    pub is_hardened: Vec<bool>,
}

/// HD wallet key derivation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDWallet {
    /// Master seed
    seed: [u8; 32],
    /// Master private key
    master_private_key: PrivateKey,
    /// Master public key
    master_public_key: PublicKey,
    /// Account index
    account_index: u32,
    /// Change index (0 = external, 1 = internal)
    change_index: u32,
    /// Address index
    address_index: u32,
    /// Derived keys cache
    derived_keys: HashMap<String, (PrivateKey, PublicKey)>,
}

/// BIP44 path components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BIP44Purpose {
    /// BIP44 purpose (44')
    BIP44 = 44,
}

/// BIP44 coin types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BIP44CoinType {
    /// Bitcoin mainnet (0')
    Bitcoin = 0,
    /// Bitcoin testnet (1')
    BitcoinTestnet = 1,
    /// Ethereum (60')
    Ethereum = 60,
    /// Unicoin mainnet (1000')
    Unicoin = 1000,
    /// Unicoin testnet (1001')
    UnicoinTestnet = 1001,
}

impl DerivationPath {
    /// Create a new derivation path
    pub fn new(components: Vec<u32>, is_hardened: Vec<bool>) -> Result<Self> {
        if components.len() != is_hardened.len() {
            return Err(UnicoinError::Wallet(
                "Components and hardened flags must have same length".to_string()
            ));
        }

        Ok(Self {
            components,
            is_hardened,
        })
    }

    /// Create BIP44 derivation path
    pub fn new_bip44(coin_type: BIP44CoinType, account: u32, change: u32, address: u32) -> Self {
        Self {
            components: vec![
                BIP44Purpose::BIP44 as u32,
                coin_type as u32,
                account,
                change,
                address,
            ],
            is_hardened: vec![true, true, true, false, false],
        }
    }

    /// Create Unicoin mainnet path
    pub fn new_unicoin_mainnet(account: u32, change: u32, address: u32) -> Self {
        Self::new_bip44(BIP44CoinType::Unicoin, account, change, address)
    }

    /// Create Unicoin testnet path
    pub fn new_unicoin_testnet(account: u32, change: u32, address: u32) -> Self {
        Self::new_bip44(BIP44CoinType::UnicoinTestnet, account, change, address)
    }

    /// Convert path to string format (e.g., "m/44'/1000'/0'/0/0")
    pub fn to_string(&self) -> String {
        let mut path = String::from("m");
        
        for (i, (component, hardened)) in self.components.iter().zip(self.is_hardened.iter()).enumerate() {
            path.push('/');
            if *hardened {
                path.push_str(&format!("{}'", component));
            } else {
                path.push_str(&component.to_string());
            }
        }
        
        path
    }

    /// Parse path from string format
    pub fn from_string(path_str: &str) -> Result<Self> {
        if !path_str.starts_with("m/") {
            return Err(UnicoinError::Wallet("Invalid path format".to_string()));
        }

        let parts: Vec<&str> = path_str[2..].split('/').collect();
        let mut components = Vec::new();
        let mut is_hardened = Vec::new();

        for part in parts {
            let hardened = part.ends_with('\'');
            let component_str = if hardened { &part[..part.len() - 1] } else { part };
            
            let component = component_str.parse::<u32>()
                .map_err(|_| UnicoinError::Wallet("Invalid path component".to_string()))?;
            
            components.push(component);
            is_hardened.push(hardened);
        }

        Self::new(components, is_hardened)
    }

    /// Get the depth of the path
    pub fn depth(&self) -> usize {
        self.components.len()
    }

    /// Check if path is valid for BIP44
    pub fn is_bip44_valid(&self) -> bool {
        self.depth() >= 5 && 
        self.components[0] == BIP44Purpose::BIP44 as u32 &&
        self.is_hardened[0] &&
        self.is_hardened[1] &&
        self.is_hardened[2] &&
        !self.is_hardened[3] &&
        !self.is_hardened[4]
    }
}

impl HDWallet {
    /// Create a new HD wallet from seed
    pub fn new_from_seed(seed: [u8; 32], algorithm: CryptoAlgorithm) -> Result<Self> {
        let master_private_key = PrivateKey::new(algorithm, seed.to_vec());
        let master_public_key = master_private_key.public_key();

        Ok(Self {
            seed,
            master_private_key,
            master_public_key,
            account_index: 0,
            change_index: 0,
            address_index: 0,
            derived_keys: HashMap::new(),
        })
    }

    /// Create a new HD wallet with random seed
    pub fn new_random(algorithm: CryptoAlgorithm) -> Result<Self> {
        let seed = generate_seed();
        Self::new_from_seed(seed, algorithm)
    }

    /// Create a new HD wallet from mnemonic phrase
    pub fn new_from_mnemonic(mnemonic: &str, passphrase: &str, algorithm: CryptoAlgorithm) -> Result<Self> {
        let seed = Self::mnemonic_to_seed(mnemonic, passphrase)?;
        Self::new_from_seed(seed, algorithm)
    }

    /// Derive a key pair from the given path
    pub fn derive_key_pair(&mut self, path: &DerivationPath) -> Result<(PrivateKey, PublicKey)> {
        let path_str = path.to_string();
        
        // Check cache first
        if let Some(cached) = self.derived_keys.get(&path_str) {
            return Ok(cached.clone());
        }

        // Derive private key along the path
        let mut current_key = self.master_private_key.data.clone();
        
        for (i, (component, hardened)) in path.components.iter().zip(path.is_hardened.iter()).enumerate() {
            let child_key = derive_child_key(&current_key, *component, *hardened)?;
            current_key = child_key.to_vec();
        }

        // Create private and public keys
        let private_key = PrivateKey::new(self.master_private_key.algorithm, current_key);
        let public_key = private_key.public_key();

        // Cache the derived keys
        self.derived_keys.insert(path_str, (private_key.clone(), public_key.clone()));

        Ok((private_key, public_key))
    }

    /// Derive an address from the given path
    pub fn derive_address(&mut self, path: &DerivationPath) -> Result<String> {
        let (_, public_key) = self.derive_key_pair(path)?;
        Ok(public_key.to_hash().to_hex())
    }

    /// Get the master public key
    pub fn get_master_public_key(&self) -> PublicKey {
        self.master_public_key.clone()
    }

    /// Get the master private key (use with caution)
    pub fn get_master_private_key(&self) -> PrivateKey {
        self.master_private_key.clone()
    }

    /// Get the seed (use with caution)
    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
    }

    /// Generate a new address for the given account and change
    pub fn generate_address(&mut self, account: u32, change: u32, address: u32) -> Result<String> {
        let path = DerivationPath::new_unicoin_mainnet(account, change, address);
        self.derive_address(&path)
    }

    /// Generate the next address for the current account
    pub fn generate_next_address(&mut self, account: u32, change: u32) -> Result<String> {
        let address = self.generate_address(account, change, self.address_index)?;
        self.address_index += 1;
        Ok(address)
    }

    /// Sign a message with a key from the given path
    pub fn sign_with_path(&mut self, path: &DerivationPath, message: &[u8]) -> Result<crate::crypto::Signature> {
        let (private_key, _) = self.derive_key_pair(path)?;
        private_key.sign(message)
    }

    /// Verify a signature with a key from the given path
    pub fn verify_with_path(&mut self, path: &DerivationPath, message: &[u8], signature: &crate::crypto::Signature) -> Result<bool> {
        let (_, public_key) = self.derive_key_pair(path)?;
        crate::crypto::verify_signature(message, signature, &public_key)
    }

    /// Export wallet to mnemonic phrase
    pub fn export_mnemonic(&self) -> Result<String> {
        // This would require implementing BIP39 mnemonic generation
        // For now, we'll return a placeholder
        Ok("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string())
    }

    /// Import wallet from mnemonic phrase
    pub fn import_from_mnemonic(mnemonic: &str, passphrase: &str, algorithm: CryptoAlgorithm) -> Result<Self> {
        Self::new_from_mnemonic(mnemonic, passphrase, algorithm)
    }

    /// Convert mnemonic to seed using PBKDF2
    fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Result<[u8; 32]> {
        let salt = format!("mnemonic{}", passphrase);
        crate::crypto::derive_key_from_password(mnemonic.as_bytes(), salt.as_bytes(), 2048)
    }

    /// Clear the derived keys cache
    pub fn clear_cache(&mut self) {
        self.derived_keys.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            cached_keys: self.derived_keys.len(),
            cache_hit_ratio: 0.0, // Would be calculated from actual usage
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub cached_keys: usize,
    pub cache_hit_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivation_path_creation() {
        let path = DerivationPath::new(vec![44, 1000, 0, 0, 0], vec![true, true, true, false, false]).unwrap();
        assert_eq!(path.depth(), 5);
        assert!(path.is_bip44_valid());
    }

    #[test]
    fn test_derivation_path_string_conversion() {
        let path = DerivationPath::new_unicoin_mainnet(0, 0, 0);
        let path_str = path.to_string();
        assert_eq!(path_str, "m/44'/1000'/0'/0/0");

        let parsed_path = DerivationPath::from_string(&path_str).unwrap();
        assert_eq!(path, parsed_path);
    }

    #[test]
    fn test_hd_wallet_creation() {
        let wallet = HDWallet::new_random(CryptoAlgorithm::Secp256k1).unwrap();
        assert!(!wallet.get_master_public_key().to_hash().is_zero());
    }

    #[test]
    fn test_key_derivation() {
        let mut wallet = HDWallet::new_random(CryptoAlgorithm::Secp256k1).unwrap();
        let path = DerivationPath::new_unicoin_mainnet(0, 0, 0);

        let (private_key, public_key) = wallet.derive_key_pair(&path).unwrap();
        assert_eq!(public_key, private_key.public_key());
    }

    #[test]
    fn test_address_generation() {
        let mut wallet = HDWallet::new_random(CryptoAlgorithm::Secp256k1).unwrap();
        
        let address1 = wallet.generate_address(0, 0, 0).unwrap();
        let address2 = wallet.generate_address(0, 0, 1).unwrap();
        
        assert_ne!(address1, address2);
        assert!(!address1.is_empty());
        assert!(!address2.is_empty());
    }

    #[test]
    fn test_signature_operations() {
        let mut wallet = HDWallet::new_random(CryptoAlgorithm::Secp256k1).unwrap();
        let path = DerivationPath::new_unicoin_mainnet(0, 0, 0);
        let message = b"test message";

        let signature = wallet.sign_with_path(&path, message).unwrap();
        let is_valid = wallet.verify_with_path(&path, message, &signature).unwrap();
        
        assert!(is_valid);
    }
}
