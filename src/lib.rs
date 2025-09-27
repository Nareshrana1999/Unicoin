//! Unicoin - Universal Coin
//! The Ultimate Secure Decentralized Cryptocurrency
//!
//! This library provides the core functionality for Unicoin, including:
//! - Secure blockchain implementation
//! - Quantum-resistant cryptography
//! - Privacy-preserving transactions
//! - Scalable consensus mechanisms
//! - Cross-chain interoperability

pub mod blockchain;
pub mod crypto;
pub mod wallet;
pub mod privacy;
pub mod consensus;
pub mod network;
pub mod smart_contracts;
pub mod governance;
pub mod bridges;
pub mod defi;
pub mod ai;
pub mod nft;
pub mod core;
pub mod utils;

pub use blockchain::{Block, BlockHeader, Transaction, Blockchain};
pub use crypto::{Hash, PublicKey, PrivateKey, Signature};
pub use wallet::Wallet;
pub use privacy::ZeroKnowledgeProof;

/// Unicoin version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_VERSION: u32 = 1;

/// Maximum supply of Unicoin (21 million coins)
pub const MAX_SUPPLY: u64 = 21_000_000 * 1_000_000_000; // 21M coins with 9 decimal places

/// Block time in seconds (target: 10 seconds)
pub const BLOCK_TIME: u64 = 10;

/// Maximum transactions per block
pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 10000;

/// Minimum transaction fee (in satoshis)
pub const MIN_TRANSACTION_FEE: u64 = 1000;

/// Result type for Unicoin operations
pub type Result<T> = std::result::Result<T, UnicoinError>;

/// Main error type for Unicoin operations
#[derive(Debug, thiserror::Error)]
pub enum UnicoinError {
    #[error("Cryptographic error: {0}")]
    Crypto(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    #[error("Privacy error: {0}")]
    Privacy(String),
    
    #[error("Consensus error: {0}")]
    Consensus(String),
    
    #[error("Smart contract error: {0}")]
    SmartContract(String),
    
    #[error("Governance error: {0}")]
    Governance(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
