//! Advanced wallet implementation
//!
//! This module provides a full-featured wallet with HD key derivation,
//! address management, transaction creation, and balance tracking.

use crate::{
    blockchain::{Transaction, TransactionInput, TransactionOutput, TransactionType, TransactionOutpoint},
    crypto::{PrivateKey, PublicKey, Hash, Signature},
    wallet::{derivation::{HDWallet, DerivationPath, BIP44CoinType}, multisig::MultiSigWallet},
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Wallet types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletType {
    /// Single-signature wallet
    SingleSig,
    /// Multi-signature wallet
    MultiSig,
    /// Hardware wallet
    Hardware,
    /// Watch-only wallet
    WatchOnly,
}

/// Wallet status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletStatus {
    /// Wallet is unlocked and ready
    Unlocked,
    /// Wallet is locked with password
    Locked,
    /// Wallet is encrypted
    Encrypted,
    /// Wallet is corrupted
    Corrupted,
}

/// Address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    /// Address string
    pub address: String,
    /// Derivation path
    pub path: String,
    /// Address index
    pub index: u32,
    /// Label/name for the address
    pub label: Option<String>,
    /// Current balance
    pub balance: u64,
    /// Total received
    pub total_received: u64,
    /// Total sent
    pub total_sent: u64,
    /// Transaction count
    pub transaction_count: u32,
    /// Last used timestamp
    pub last_used: Option<u64>,
}

/// Transaction history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionHistory {
    /// Transaction hash
    pub tx_hash: String,
    /// Transaction type
    pub tx_type: String,
    /// Amount (positive for received, negative for sent)
    pub amount: i64,
    /// Fee paid
    pub fee: u64,
    /// Block height (None if unconfirmed)
    pub block_height: Option<u64>,
    /// Confirmation count
    pub confirmations: u32,
    /// Timestamp
    pub timestamp: u64,
    /// From addresses
    pub from_addresses: Vec<String>,
    /// To addresses
    pub to_addresses: Vec<String>,
    /// Transaction status
    pub status: String,
    /// Memo/message
    pub memo: Option<String>,
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Wallet name
    pub name: String,
    /// Wallet type
    pub wallet_type: WalletType,
    /// Coin type (mainnet/testnet)
    pub coin_type: BIP44CoinType,
    /// Account index
    pub account_index: u32,
    /// Gap limit for address generation
    pub gap_limit: u32,
    /// Enable auto-address generation
    pub auto_address_generation: bool,
    /// Transaction fee rate (satoshis per byte)
    pub fee_rate: u64,
    /// Default change address type
    pub change_address_type: String,
    /// Enable transaction replacement
    pub enable_rbf: bool,
    /// Enable CPFP
    pub enable_cpfp: bool,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            wallet_type: WalletType::SingleSig,
            coin_type: BIP44CoinType::Unicoin,
            account_index: 0,
            gap_limit: 20,
            auto_address_generation: true,
            fee_rate: 10, // 10 satoshis per byte
            change_address_type: "bech32".to_string(),
            enable_rbf: true,
            enable_cpfp: true,
        }
    }
}

/// Complete wallet implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWallet {
    /// Wallet configuration
    pub config: WalletConfig,
    /// HD wallet for key derivation
    pub hd_wallet: HDWallet,
    /// Wallet status
    pub status: WalletStatus,
    /// Addresses managed by this wallet
    pub addresses: HashMap<String, AddressInfo>,
    /// Transaction history
    pub transaction_history: Vec<TransactionHistory>,
    /// UTXO set for this wallet
    pub utxos: HashMap<String, TransactionOutput>,
    /// Wallet balance
    pub balance: u64,
    /// Unconfirmed balance
    pub unconfirmed_balance: u64,
    /// Wallet creation timestamp
    pub created_at: u64,
    /// Last sync timestamp
    pub last_sync: u64,
    /// Wallet file path
    pub wallet_path: Option<PathBuf>,
}

impl AdvancedWallet {
    /// Create a new wallet
    pub fn new(config: WalletConfig, seed: Option<[u8; 32]>) -> Result<Self> {
        let hd_wallet = if let Some(seed) = seed {
            HDWallet::new_from_seed(seed, crate::crypto::CryptoAlgorithm::Secp256k1)?
        } else {
            HDWallet::new_random(crate::crypto::CryptoAlgorithm::Secp256k1)?
        };

        Ok(Self {
            config,
            hd_wallet,
            status: WalletStatus::Unlocked,
            addresses: HashMap::new(),
            transaction_history: Vec::new(),
            utxos: HashMap::new(),
            balance: 0,
            unconfirmed_balance: 0,
            created_at: crate::utils::timestamp(),
            last_sync: 0,
            wallet_path: None,
        })
    }

    /// Create a new wallet from mnemonic
    pub fn new_from_mnemonic(config: WalletConfig, mnemonic: &str, passphrase: &str) -> Result<Self> {
        let hd_wallet = HDWallet::new_from_mnemonic(mnemonic, passphrase, crate::crypto::CryptoAlgorithm::Secp256k1)?;
        
        Ok(Self {
            config,
            hd_wallet,
            status: WalletStatus::Unlocked,
            addresses: HashMap::new(),
            transaction_history: Vec::new(),
            utxos: HashMap::new(),
            balance: 0,
            unconfirmed_balance: 0,
            created_at: crate::utils::timestamp(),
            last_sync: 0,
            wallet_path: None,
        })
    }

    /// Generate a new address
    pub fn generate_address(&mut self, label: Option<String>) -> Result<String> {
        let address_index = self.get_next_address_index();
        let path = DerivationPath::new_unicoin_mainnet(
            self.config.account_index,
            0, // External addresses
            address_index,
        );

        let address = self.hd_wallet.derive_address(&path)?;
        
        let address_info = AddressInfo {
            address: address.clone(),
            path: path.to_string(),
            index: address_index,
            label,
            balance: 0,
            total_received: 0,
            total_sent: 0,
            transaction_count: 0,
            last_used: None,
        };

        self.addresses.insert(address.clone(), address_info);
        Ok(address)
    }

    /// Generate a change address
    pub fn generate_change_address(&mut self) -> Result<String> {
        let address_index = self.get_next_change_address_index();
        let path = DerivationPath::new_unicoin_mainnet(
            self.config.account_index,
            1, // Change addresses
            address_index,
        );

        let address = self.hd_wallet.derive_address(&path)?;
        
        let address_info = AddressInfo {
            address: address.clone(),
            path: path.to_string(),
            index: address_index,
            label: Some("Change".to_string()),
            balance: 0,
            total_received: 0,
            total_sent: 0,
            transaction_count: 0,
            last_used: None,
        };

        self.addresses.insert(address.clone(), address_info);
        Ok(address)
    }

    /// Get the next address index
    fn get_next_address_index(&self) -> u32 {
        let external_addresses: Vec<&AddressInfo> = self.addresses.values()
            .filter(|info| info.path.contains("/0/"))
            .collect();
        
        if external_addresses.is_empty() {
            0
        } else {
            external_addresses.iter()
                .map(|info| info.index)
                .max()
                .unwrap_or(0) + 1
        }
    }

    /// Get the next change address index
    fn get_next_change_address_index(&self) -> u32 {
        let change_addresses: Vec<&AddressInfo> = self.addresses.values()
            .filter(|info| info.path.contains("/1/"))
            .collect();
        
        if change_addresses.is_empty() {
            0
        } else {
            change_addresses.iter()
                .map(|info| info.index)
                .max()
                .unwrap_or(0) + 1
        }
    }

    /// Get address information
    pub fn get_address_info(&self, address: &str) -> Option<&AddressInfo> {
        self.addresses.get(address)
    }

    /// List all addresses
    pub fn list_addresses(&self) -> Vec<&AddressInfo> {
        self.addresses.values().collect()
    }

    /// Create a transaction
    pub fn create_transaction(&mut self, to_address: String, amount: u64, fee: Option<u64>) -> Result<Transaction> {
        if self.status != WalletStatus::Unlocked {
            return Err(UnicoinError::Wallet("Wallet is locked".to_string()));
        }

        let fee_amount = fee.unwrap_or(self.config.fee_rate * 250); // Estimate 250 bytes
        let total_amount = amount + fee_amount;

        // Check if we have enough balance
        if self.balance < total_amount {
            return Err(UnicoinError::Wallet("Insufficient balance".to_string()));
        }

        // Select UTXOs to spend
        let (selected_utxos, total_input) = self.select_utxos(total_amount)?;

        // Create transaction inputs
        let mut inputs = Vec::new();
        for (outpoint, _) in &selected_utxos {
            let path_str = self.get_path_for_outpoint(outpoint)?;
            let path = DerivationPath::from_string(&path_str)?;
            let (private_key, _) = self.hd_wallet.derive_key_pair(&path)?;
            
            // Create signature (simplified for now)
            let script_sig = private_key.to_bytes();
            
            inputs.push(TransactionInput::new_bitcoin(
                outpoint.tx_hash,
                outpoint.output_index,
                script_sig,
            ));
        }

        // Create transaction outputs
        let mut outputs = Vec::new();
        
        // Recipient output
        outputs.push(TransactionOutput::new(
            amount,
            PublicKey::from_hash(Hash::from_hex(&to_address)?),
        ));

        // Change output (if needed)
        let change_amount = total_input - total_amount;
        if change_amount > 546 { // Dust threshold
            let change_address = self.generate_change_address()?;
            outputs.push(TransactionOutput::new(
                change_amount,
                PublicKey::from_hash(Hash::from_hex(&change_address)?),
            ));
        }

        // Create transaction
        let mut transaction = Transaction::new(
            TransactionType::Transfer,
            inputs,
            outputs,
            fee_amount,
        );

        // Sign the transaction
        transaction.sign(&self.get_master_private_key()?)?;

        Ok(transaction)
    }

    /// Select UTXOs to spend
    fn select_utxos(&self, target_amount: u64) -> Result<(Vec<(TransactionOutpoint, TransactionOutput)>, u64)> {
        // Simple UTXO selection (could be improved with coin selection algorithms)
        let mut selected_utxos = Vec::new();
        let mut total_amount = 0u64;

        for (outpoint, output) in &self.utxos {
            selected_utxos.push((outpoint.clone(), output.clone()));
            total_amount += output.amount;
            
            if total_amount >= target_amount {
                break;
            }
        }

        if total_amount < target_amount {
            return Err(UnicoinError::Wallet("Insufficient UTXOs".to_string()));
        }

        Ok((selected_utxos, total_amount))
    }

    /// Get derivation path for a UTXO
    fn get_path_for_outpoint(&self, _outpoint: &TransactionOutpoint) -> Result<String> {
        // This would need to track which path was used for each UTXO
        // For now, return a default path
        Ok("m/44'/1000'/0'/0/0".to_string())
    }

    /// Get master private key
    fn get_master_private_key(&self) -> Result<PrivateKey> {
        Ok(self.hd_wallet.get_master_private_key())
    }

    /// Update wallet balance
    pub fn update_balance(&mut self) -> Result<()> {
        self.balance = self.addresses.values().map(|info| info.balance).sum();
        
        // Update UTXO set (this would typically come from blockchain sync)
        // For now, we'll keep it as is
        
        Ok(())
    }

    /// Add transaction to history
    pub fn add_transaction(&mut self, tx_history: TransactionHistory) {
        self.transaction_history.push(tx_history);
        self.last_sync = crate::utils::timestamp();
    }

    /// Get transaction history
    pub fn get_transaction_history(&self, limit: Option<usize>) -> Vec<&TransactionHistory> {
        let mut history: Vec<&TransactionHistory> = self.transaction_history.iter().collect();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            history.truncate(limit);
        }
        
        history
    }

    /// Export wallet to file
    pub fn export_wallet(&self, path: PathBuf, password: Option<String>) -> Result<()> {
        let wallet_data = if let Some(pwd) = password {
            // Encrypt wallet data
            self.encrypt_wallet_data(pwd)?
        } else {
            serde_json::to_string(self)?
        };

        std::fs::write(&path, wallet_data)?;
        Ok(())
    }

    /// Import wallet from file
    pub fn import_wallet(path: PathBuf, password: Option<String>) -> Result<Self> {
        let wallet_data = std::fs::read_to_string(&path)?;
        
        let wallet: AdvancedWallet = if let Some(pwd) = password {
            // Decrypt wallet data
            Self::decrypt_wallet_data(wallet_data, pwd)?
        } else {
            serde_json::from_str(&wallet_data)?
        };

        Ok(wallet)
    }

    /// Encrypt wallet data
    fn encrypt_wallet_data(&self, password: String) -> Result<String> {
        // TODO: Implement proper encryption
        Ok(serde_json::to_string(self)?)
    }

    /// Decrypt wallet data
    fn decrypt_wallet_data(encrypted_data: String, _password: String) -> Result<AdvancedWallet> {
        // TODO: Implement proper decryption
        Ok(serde_json::from_str(&encrypted_data)?)
    }

    /// Lock the wallet
    pub fn lock(&mut self) {
        self.status = WalletStatus::Locked;
    }

    /// Unlock the wallet
    pub fn unlock(&mut self, _password: String) -> Result<()> {
        // TODO: Implement password verification
        self.status = WalletStatus::Unlocked;
        Ok(())
    }

    /// Get wallet statistics
    pub fn get_stats(&self) -> WalletStats {
        WalletStats {
            total_addresses: self.addresses.len(),
            external_addresses: self.addresses.values().filter(|info| info.path.contains("/0/")).count(),
            change_addresses: self.addresses.values().filter(|info| info.path.contains("/1/")).count(),
            total_transactions: self.transaction_history.len(),
            balance: self.balance,
            unconfirmed_balance: self.unconfirmed_balance,
            created_at: self.created_at,
            last_sync: self.last_sync,
        }
    }

    /// Validate wallet integrity
    pub fn validate(&self) -> Result<bool> {
        // Check if all addresses can be derived
        for address_info in self.addresses.values() {
            let path = DerivationPath::from_string(&address_info.path)?;
            let derived_address = self.hd_wallet.derive_address(&path)?;
            
            if derived_address != address_info.address {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStats {
    pub total_addresses: usize,
    pub external_addresses: usize,
    pub change_addresses: usize,
    pub total_transactions: usize,
    pub balance: u64,
    pub unconfirmed_balance: u64,
    pub created_at: u64,
    pub last_sync: u64,
}
