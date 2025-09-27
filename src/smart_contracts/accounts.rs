use crate::crypto::{hash::Hash, keys::PublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Account manager for Ethereum-compatible accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountManager {
    /// Account storage
    pub accounts: HashMap<Hash, AccountInfo>,
    /// Account counter for contract addresses
    pub account_counter: u64,
    /// Account state root
    pub state_root: Hash,
}

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account address
    pub address: Hash,
    /// Account type
    pub account_type: AccountType,
    /// Account balance in wei
    pub balance: u64,
    /// Account nonce
    pub nonce: u64,
    /// Account state
    pub state: AccountState,
    /// Account code (for contracts)
    pub code: Option<Vec<u8>>,
    /// Account code hash
    pub code_hash: Hash,
    /// Account storage
    pub storage: HashMap<Hash, Hash>,
    /// Account storage root
    pub storage_root: Hash,
    /// Account creation timestamp
    pub created_at: u64,
    /// Last update timestamp
    pub updated_at: u64,
}

/// Account types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    /// Externally owned account (EOA)
    EOA,
    /// Contract account
    Contract,
    /// System account
    System,
    /// Precompiled contract
    Precompiled,
}

/// Account states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountState {
    /// Active account
    Active,
    /// Frozen account
    Frozen,
    /// Deleted account
    Deleted,
    /// Pending creation
    Pending,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            account_counter: 0,
            state_root: Hash::zero(),
        }
    }

    /// Create a new externally owned account
    pub fn create_eoa(&mut self, public_key: &PublicKey, initial_balance: u64) -> Hash {
        let address = self.derive_address_from_public_key(public_key);
        
        let account = AccountInfo {
            address,
            account_type: AccountType::EOA,
            balance: initial_balance,
            nonce: 0,
            state: AccountState::Active,
            code: None,
            code_hash: Hash::zero(),
            storage: HashMap::new(),
            storage_root: Hash::zero(),
            created_at: crate::utils::timestamp(),
            updated_at: crate::utils::timestamp(),
        };
        
        self.accounts.insert(address, account);
        self.update_state_root();
        
        address
    }

    /// Create a new contract account
    pub fn create_contract(&mut self, creator_address: Hash, code: Vec<u8>) -> Hash {
        let address = self.generate_contract_address(creator_address);
        
        let code_hash = Hash::sha256(&code);
        
        let account = AccountInfo {
            address,
            account_type: AccountType::Contract,
            balance: 0,
            nonce: 0,
            state: AccountState::Active,
            code: Some(code),
            code_hash,
            storage: HashMap::new(),
            storage_root: Hash::zero(),
            created_at: crate::utils::timestamp(),
            updated_at: crate::utils::timestamp(),
        };
        
        self.accounts.insert(address, account);
        self.update_state_root();
        
        address
    }

    /// Generate contract address from creator address and nonce
    fn generate_contract_address(&mut self, creator_address: Hash) -> Hash {
        // Ethereum-style address generation
        let mut data = Vec::new();
        data.extend_from_slice(&creator_address.to_bytes());
        data.extend_from_slice(&self.account_counter.to_be_bytes());
        
        let hash = Hash::sha256(&data);
        self.account_counter += 1;
        
        hash
    }

    /// Derive address from public key
    fn derive_address_from_public_key(&self, public_key: &PublicKey) -> Hash {
        let pubkey_bytes = public_key.to_bytes();
        let hash = crate::crypto::hash::ripemd160(&pubkey_bytes);
        Hash::new(hash)
    }

    /// Get account information
    pub fn get_account(&self, address: &Hash) -> Option<&AccountInfo> {
        self.accounts.get(address)
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Hash) -> u64 {
        self.accounts.get(address)
            .map(|account| account.balance)
            .unwrap_or(0)
    }

    /// Get account nonce
    pub fn get_nonce(&self, address: &Hash) -> u64 {
        self.accounts.get(address)
            .map(|account| account.nonce)
            .unwrap_or(0)
    }

    /// Check if account exists
    pub fn account_exists(&self, address: &Hash) -> bool {
        self.accounts.contains_key(address)
    }

    /// Check if account is a contract
    pub fn is_contract(&self, address: &Hash) -> bool {
        self.accounts.get(address)
            .map(|account| account.account_type == AccountType::Contract)
            .unwrap_or(false)
    }

    /// Transfer funds between accounts
    pub fn transfer(&mut self, from: &Hash, to: &Hash, amount: u64) -> Result<(), String> {
        if let Some(from_account) = self.accounts.get_mut(from) {
            if from_account.balance < amount {
                return Err("Insufficient balance".to_string());
            }
            if from_account.state != AccountState::Active {
                return Err("Source account is not active".to_string());
            }
            
            from_account.balance -= amount;
            from_account.updated_at = crate::utils::timestamp();
        } else {
            return Err("Source account not found".to_string());
        }
        
        // Create destination account if it doesn't exist
        if !self.account_exists(to) {
            self.accounts.insert(*to, AccountInfo {
                address: *to,
                account_type: AccountType::EOA,
                balance: 0,
                nonce: 0,
                state: AccountState::Active,
                code: None,
                code_hash: Hash::zero(),
                storage: HashMap::new(),
                storage_root: Hash::zero(),
                created_at: crate::utils::timestamp(),
                updated_at: crate::utils::timestamp(),
            });
        }
        
        if let Some(to_account) = self.accounts.get_mut(to) {
            to_account.balance += amount;
            to_account.updated_at = crate::utils::timestamp();
        }
        
        self.update_state_root();
        Ok(())
    }

    /// Increment account nonce
    pub fn increment_nonce(&mut self, address: &Hash) -> Result<u64, String> {
        if let Some(account) = self.accounts.get_mut(address) {
            if account.state != AccountState::Active {
                return Err("Account is not active".to_string());
            }
            
            account.nonce += 1;
            account.updated_at = crate::utils::timestamp();
            self.update_state_root();
            
            Ok(account.nonce)
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Set account state
    pub fn set_account_state(&mut self, address: &Hash, state: AccountState) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.state = state;
            account.updated_at = crate::utils::timestamp();
            self.update_state_root();
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Update account storage
    pub fn set_storage(&mut self, address: &Hash, key: Hash, value: Hash) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            if account.state != AccountState::Active {
                return Err("Account is not active".to_string());
            }
            
            account.storage.insert(key, value);
            account.updated_at = crate::utils::timestamp();
            self.update_storage_root(account);
            self.update_state_root();
            
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Get account storage value
    pub fn get_storage(&self, address: &Hash, key: &Hash) -> Option<Hash> {
        self.accounts.get(address)
            .and_then(|account| account.storage.get(key).copied())
    }

    /// Delete account storage
    pub fn delete_storage(&mut self, address: &Hash, key: &Hash) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.storage.remove(key);
            account.updated_at = crate::utils::timestamp();
            self.update_storage_root(account);
            self.update_state_root();
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Update account code
    pub fn update_code(&mut self, address: &Hash, code: Vec<u8>) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(address) {
            if account.state != AccountState::Active {
                return Err("Account is not active".to_string());
            }
            
            let code_hash = Hash::sha256(&code);
            account.code = Some(code);
            account.code_hash = code_hash;
            account.updated_at = crate::utils::timestamp();
            self.update_state_root();
            
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Get account code
    pub fn get_code(&self, address: &Hash) -> Option<&Vec<u8>> {
        self.accounts.get(address)
            .and_then(|account| account.code.as_ref())
    }

    /// Update storage root for account
    fn update_storage_root(&self, account: &mut AccountInfo) {
        // Simplified storage root calculation
        // In a real implementation, this would use a Merkle tree
        let mut storage_data = Vec::new();
        for (key, value) in &account.storage {
            storage_data.extend_from_slice(&key.to_bytes());
            storage_data.extend_from_slice(&value.to_bytes());
        }
        account.storage_root = Hash::sha256(&storage_data);
    }

    /// Update global state root
    fn update_state_root(&mut self) {
        // Simplified state root calculation
        // In a real implementation, this would use a Merkle tree
        let mut state_data = Vec::new();
        for (address, account) in &self.accounts {
            state_data.extend_from_slice(&address.to_bytes());
            state_data.extend_from_slice(&account.balance.to_be_bytes());
            state_data.extend_from_slice(&account.nonce.to_be_bytes());
            state_data.extend_from_slice(&account.code_hash.to_bytes());
            state_data.extend_from_slice(&account.storage_root.to_bytes());
        }
        self.state_root = Hash::sha256(&state_data);
    }

    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&AccountInfo> {
        self.accounts.values().collect()
    }

    /// Get accounts by type
    pub fn get_accounts_by_type(&self, account_type: AccountType) -> Vec<&AccountInfo> {
        self.accounts.values()
            .filter(|account| account.account_type == account_type)
            .collect()
    }

    /// Get account statistics
    pub fn get_stats(&self) -> AccountStats {
        let total_accounts = self.accounts.len();
        let eoa_count = self.accounts.values()
            .filter(|account| account.account_type == AccountType::EOA)
            .count();
        let contract_count = self.accounts.values()
            .filter(|account| account.account_type == AccountType::Contract)
            .count();
        
        let total_balance: u64 = self.accounts.values()
            .map(|account| account.balance)
            .sum();
        
        AccountStats {
            total_accounts,
            eoa_count,
            contract_count,
            total_balance,
            state_root: self.state_root,
        }
    }

    /// Clear all accounts (for testing)
    pub fn clear(&mut self) {
        self.accounts.clear();
        self.account_counter = 0;
        self.state_root = Hash::zero();
    }
}

/// Account statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStats {
    pub total_accounts: usize,
    pub eoa_count: usize,
    pub contract_count: usize,
    pub total_balance: u64,
    pub state_root: Hash,
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_manager_creation() {
        let manager = AccountManager::new();
        assert_eq!(manager.accounts.len(), 0);
        assert_eq!(manager.account_counter, 0);
    }

    #[test]
    fn test_create_eoa() {
        let mut manager = AccountManager::new();
        let public_key = PublicKey::random();
        
        let address = manager.create_eoa(&public_key, 1000000);
        
        assert!(manager.account_exists(&address));
        assert_eq!(manager.get_balance(&address), 1000000);
        assert_eq!(manager.get_nonce(&address), 0);
        assert!(!manager.is_contract(&address));
    }

    #[test]
    fn test_create_contract() {
        let mut manager = AccountManager::new();
        let creator_address = Hash::new([1u8; 32]);
        let code = vec![1, 2, 3, 4, 5];
        
        let address = manager.create_contract(creator_address, code.clone());
        
        assert!(manager.account_exists(&address));
        assert!(manager.is_contract(&address));
        assert_eq!(manager.get_code(&address), Some(&code));
    }

    #[test]
    fn test_transfer() {
        let mut manager = AccountManager::new();
        let public_key = PublicKey::random();
        let from_address = manager.create_eoa(&public_key, 1000000);
        let to_address = Hash::new([2u8; 32]);
        
        let result = manager.transfer(&from_address, &to_address, 500000);
        assert!(result.is_ok());
        
        assert_eq!(manager.get_balance(&from_address), 500000);
        assert_eq!(manager.get_balance(&to_address), 500000);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut manager = AccountManager::new();
        let public_key = PublicKey::random();
        let from_address = manager.create_eoa(&public_key, 100000);
        let to_address = Hash::new([2u8; 32]);
        
        let result = manager.transfer(&from_address, &to_address, 200000);
        assert!(result.is_err());
    }

    #[test]
    fn test_increment_nonce() {
        let mut manager = AccountManager::new();
        let public_key = PublicKey::random();
        let address = manager.create_eoa(&public_key, 1000000);
        
        let nonce = manager.increment_nonce(&address);
        assert!(nonce.is_ok());
        assert_eq!(nonce.unwrap(), 1);
        
        let nonce = manager.increment_nonce(&address);
        assert!(nonce.is_ok());
        assert_eq!(nonce.unwrap(), 2);
    }

    #[test]
    fn test_storage_operations() {
        let mut manager = AccountManager::new();
        let creator_address = Hash::new([1u8; 32]);
        let address = manager.create_contract(creator_address, vec![1, 2, 3]);
        
        let key = Hash::new([1u8; 32]);
        let value = Hash::new([2u8; 32]);
        
        let result = manager.set_storage(&address, key, value);
        assert!(result.is_ok());
        
        assert_eq!(manager.get_storage(&address, &key), Some(value));
        
        let result = manager.delete_storage(&address, &key);
        assert!(result.is_ok());
        
        assert_eq!(manager.get_storage(&address, &key), None);
    }

    #[test]
    fn test_account_stats() {
        let mut manager = AccountManager::new();
        let public_key = PublicKey::random();
        let eoa_address = manager.create_eoa(&public_key, 1000000);
        
        let creator_address = Hash::new([1u8; 32]);
        let contract_address = manager.create_contract(creator_address, vec![1, 2, 3]);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_accounts, 2);
        assert_eq!(stats.eoa_count, 1);
        assert_eq!(stats.contract_count, 1);
        assert_eq!(stats.total_balance, 1000000);
    }
}
