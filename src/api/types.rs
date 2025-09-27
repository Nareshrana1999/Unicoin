//! API types for Unicoin
//!
//! This module defines common types used across the API endpoints
//! including request/response structures and data transfer objects.

use serde::{Deserialize, Serialize};
use crate::{blockchain::Block, crypto::Hash};

/// Block information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub hash: Hash,
    pub height: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub nonce: u64,
    pub difficulty: u64,
    pub transaction_count: u32,
    pub size: usize,
    pub version: u32,
}

impl From<Block> for BlockInfo {
    fn from(block: Block) -> Self {
        Self {
            hash: block.hash(),
            height: block.header.height,
            timestamp: block.header.timestamp,
            previous_hash: block.header.previous_hash,
            merkle_root: block.header.merkle_root,
            nonce: block.header.nonce,
            difficulty: block.header.difficulty,
            transaction_count: block.transactions.len() as u32,
            size: std::mem::size_of_val(&block),
            version: block.header.version,
        }
    }
}

/// Transaction information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: Hash,
    pub block_hash: Option<Hash>,
    pub block_height: Option<u64>,
    pub timestamp: u64,
    pub inputs: Vec<TransactionInputInfo>,
    pub outputs: Vec<TransactionOutputInfo>,
    pub fee: u64,
    pub size: usize,
    pub version: u32,
    pub lock_time: u32,
    pub confirmations: u32,
}

/// Transaction input information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInputInfo {
    pub previous_tx_hash: Hash,
    pub previous_output_index: u32,
    pub script_sig: String,
    pub sequence: u32,
    pub witness: Vec<String>,
}

/// Transaction output information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutputInfo {
    pub value: u64,
    pub script_pubkey: String,
    pub address: String,
    pub output_index: u32,
}

/// Address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub balance: u64,
    pub total_received: u64,
    pub total_sent: u64,
    pub transaction_count: u32,
    pub first_seen: u64,
    pub last_seen: u64,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub version: String,
    pub protocol_version: u32,
    pub network_id: u32,
    pub block_height: u64,
    pub difficulty: u64,
    pub hash_rate: f64,
    pub connected_peers: u32,
    pub sync_status: SyncStatus,
    pub uptime: u64,
}

/// Sync status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Fully synced
    Synced,
    /// Syncing in progress
    Syncing,
    /// Not synced
    NotSynced,
    /// Error during sync
    SyncError,
}

/// Node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub version: String,
    pub protocol_version: u32,
    pub network: String,
    pub listening_address: String,
    pub public_key: String,
    pub uptime: u64,
    pub start_time: u64,
    pub features: Vec<String>,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
    pub port: u16,
    pub version: String,
    pub protocol_version: u32,
    pub connected_since: u64,
    pub last_seen: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub ping_time: u64,
    pub is_outbound: bool,
}

/// Mempool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub transaction_count: u32,
    pub total_size: usize,
    pub fee_rate_range: FeeRateRange,
    pub oldest_transaction: u64,
    pub newest_transaction: u64,
}

/// Fee rate range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRateRange {
    pub min: u64,
    pub max: u64,
    pub median: u64,
    pub average: u64,
}

/// Mining information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningInfo {
    pub difficulty: u64,
    pub hash_rate: f64,
    pub block_reward: u64,
    pub next_halving_height: u64,
    pub blocks_until_halving: u64,
    pub estimated_time_to_halving: u64,
}

/// Wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub wallet_id: String,
    pub balance: u64,
    pub address_count: u32,
    pub transaction_count: u32,
    pub last_activity: u64,
    pub is_encrypted: bool,
    pub is_locked: bool,
}

/// Smart contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractInfo {
    pub address: String,
    pub creator: String,
    pub creation_tx_hash: Hash,
    pub creation_block: u64,
    pub bytecode: String,
    pub abi: String,
    pub balance: u64,
    pub transaction_count: u32,
    pub last_activity: u64,
}

/// DeFi protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiProtocolInfo {
    pub protocol_name: String,
    pub protocol_type: String,
    pub total_value_locked: u64,
    pub active_users: u32,
    pub transaction_count: u32,
    pub fees_collected: u64,
    pub apy: f64,
    pub risk_score: u8,
}

/// NFT information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTInfo {
    pub token_id: String,
    pub contract_address: String,
    pub owner: String,
    pub metadata: String,
    pub creation_tx_hash: Hash,
    pub creation_block: u64,
    pub last_transfer: u64,
    pub transfer_count: u32,
    pub price: Option<u64>,
}

/// API statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub uptime: u64,
}

/// Search request parameters
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub results: Vec<SearchResultItem>,
    pub total: u64,
    pub took_ms: u64,
}

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub item_type: String,
    pub item_id: String,
    pub title: String,
    pub description: String,
    pub relevance_score: f64,
    pub metadata: serde_json::Value,
}

/// Transaction broadcast request
#[derive(Debug, Deserialize)]
pub struct BroadcastTransactionRequest {
    pub transaction: String, // Hex-encoded transaction
    pub wait_for_confirmation: Option<bool>,
}

/// Transaction broadcast response
#[derive(Debug, Serialize)]
pub struct BroadcastTransactionResponse {
    pub transaction_hash: Hash,
    pub status: String,
    pub message: Option<String>,
}

/// Address generation request
#[derive(Debug, Deserialize)]
pub struct GenerateAddressRequest {
    pub wallet_id: Option<String>,
    pub address_type: Option<String>, // "p2pkh", "p2sh", "bech32"
    pub label: Option<String>,
}

/// Address generation response
#[derive(Debug, Serialize)]
pub struct GenerateAddressResponse {
    pub address: String,
    pub public_key: String,
    pub private_key: Option<String>, // Only returned if requested
    pub label: Option<String>,
}

/// Balance query request
#[derive(Debug, Deserialize)]
pub struct BalanceQueryRequest {
    pub addresses: Vec<String>,
    pub include_unconfirmed: Option<bool>,
}

/// Balance query response
#[derive(Debug, Serialize)]
pub struct BalanceQueryResponse {
    pub balances: Vec<AddressBalance>,
    pub total_balance: u64,
    pub unconfirmed_balance: u64,
}

/// Address balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    pub address: String,
    pub balance: u64,
    pub unconfirmed_balance: u64,
    pub transaction_count: u32,
}

/// Transaction history request
#[derive(Debug, Deserialize)]
pub struct TransactionHistoryRequest {
    pub address: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
}

/// Transaction history response
#[derive(Debug, Serialize)]
pub struct TransactionHistoryResponse {
    pub transactions: Vec<TransactionInfo>,
    pub total: u64,
    pub has_more: bool,
}
