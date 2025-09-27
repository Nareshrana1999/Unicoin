//! Blockchain module - Core blockchain implementation
//!
//! This module provides the fundamental blockchain data structures and operations
//! including blocks, transactions, and the main blockchain state.

use crate::{
    crypto::{Hash, PublicKey, Signature},
    utils::timestamp,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod block;
pub mod transaction;
pub mod state;
pub mod genesis;
pub mod mempool;
pub mod merkle_tree;
pub mod utxo;
pub mod script;
pub mod segwit;
pub mod lightning;
pub mod difficulty;
pub mod block_reward;

pub use block::{Block, BlockHeader, BlockTemplate};
pub use transaction::{Transaction, TransactionType, TransactionInput, TransactionOutput, CoinbaseTransaction};
pub use state::BlockchainState;
pub use genesis::{GenesisConfig, GenesisDistribution, GenesisCreator};
pub use mempool::{Mempool, MempoolTransaction, MempoolConfig, TransactionPriority};
pub use merkle_tree::{MerkleTree, MerkleProof, SparseMerkleTree};
pub use utxo::{UTXO, UTXOSet, UTXOProvider};
pub use script::{Script, ScriptOp, ScriptEngine, P2PKH, P2SH, P2WPKH, P2WSH};
pub use segwit::{SegWitTransaction, Witness, SegWitValidator};
pub use lightning::{LightningNetwork, PaymentChannel, LightningNode, HTLC};
pub use difficulty::{DifficultyAdjustment, Target, MiningDifficulty};
pub use block_reward::{BlockReward, HalvingSchedule, InflationControl};

/// Main blockchain structure
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Current blockchain state
    state: BlockchainState,
    /// Database for storing blocks and transactions
    db: sled::Db,
}

impl Blockchain {
    /// Create a new blockchain instance
    pub fn new() -> Result<Self> {
        let db = sled::open("unicoin_blockchain")?;
        let state = BlockchainState::load(&db)?;
        
        Ok(Self { state, db })
    }

    /// Get the current blockchain height
    pub fn height(&self) -> u64 {
        self.state.height
    }

    /// Get the latest block hash
    pub fn latest_block_hash(&self) -> Hash {
        self.state.latest_block_hash
    }

    /// Add a new block to the blockchain
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        // Validate the block
        self.validate_block(&block)?;

        // Update state
        self.state.height += 1;
        self.state.latest_block_hash = block.hash();

        // Store block in database
        let block_bytes = bincode::serialize(&block)?;
        self.db.insert(&block.hash().as_bytes(), block_bytes)?;

        // Update UTXO set
        self.update_utxo_set(&block)?;

        // Commit changes
        self.state.save(&self.db)?;

        Ok(())
    }

    /// Validate a block
    fn validate_block(&self, block: &Block) -> Result<()> {
        // Validate block header
        if block.header.height != self.height() + 1 {
            return Err(UnicoinError::Blockchain(
                "Invalid block height".to_string()
            ));
        }

        // Validate previous block hash
        if block.header.previous_hash != self.latest_block_hash() {
            return Err(UnicoinError::Blockchain(
                "Invalid previous block hash".to_string()
            ));
        }

        // Validate transactions
        for tx in &block.transactions {
            self.validate_transaction(tx)?;
        }

        // Validate block hash
        if block.hash() != block.header.hash() {
            return Err(UnicoinError::Blockchain(
                "Invalid block hash".to_string()
            ));
        }

        Ok(())
    }

    /// Validate a transaction
    fn validate_transaction(&self, tx: &Transaction) -> Result<()> {
        // Validate signature
        if !tx.verify_signature() {
            return Err(UnicoinError::Blockchain(
                "Invalid transaction signature".to_string()
            ));
        }

        // Validate inputs exist in UTXO set
        for input in &tx.inputs {
            if !self.state.utxo_set.contains_key(&input.previous_output) {
                return Err(UnicoinError::Blockchain(
                    "Input not found in UTXO set".to_string()
                ));
            }
        }

        // Validate output amounts are positive
        for output in &tx.outputs {
            if output.amount == 0 {
                return Err(UnicoinError::Blockchain(
                    "Output amount cannot be zero".to_string()
                ));
            }
        }

        // Validate total input >= total output + fee
        let input_total: u64 = tx.inputs.iter()
            .map(|input| self.state.utxo_set[&input.previous_output].amount)
            .sum();
        
        let output_total: u64 = tx.outputs.iter()
            .map(|output| output.amount)
            .sum();

        if input_total < output_total + tx.fee {
            return Err(UnicoinError::Blockchain(
                "Insufficient input amount".to_string()
            ));
        }

        Ok(())
    }

    /// Update UTXO set after adding a block
    fn update_utxo_set(&mut self, block: &Block) -> Result<()> {
        for tx in &block.transactions {
            // Remove spent outputs
            for input in &tx.inputs {
                self.state.utxo_set.remove(&input.previous_output);
            }

            // Add new outputs
            for (index, output) in tx.outputs.iter().enumerate() {
                let outpoint = TransactionOutpoint {
                    tx_hash: tx.hash(),
                    output_index: index as u32,
                };
                self.state.utxo_set.insert(outpoint, output.clone());
            }
        }
        Ok(())
    }

    /// Get a block by hash
    pub fn get_block(&self, hash: &Hash) -> Result<Option<Block>> {
        if let Some(block_bytes) = self.db.get(hash.as_bytes())? {
            let block: Block = bincode::deserialize(&block_bytes)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Get a transaction by hash
    pub fn get_transaction(&self, hash: &Hash) -> Result<Option<Transaction>> {
        // Search through all blocks to find the transaction
        let mut current_height = 0;
        while current_height <= self.height() {
            if let Some(block) = self.get_block_by_height(current_height)? {
                for tx in &block.transactions {
                    if tx.hash() == *hash {
                        return Ok(Some(tx.clone()));
                    }
                }
            }
            current_height += 1;
        }
        Ok(None)
    }

    /// Get a block by height
    fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        // This is a simplified implementation
        // In a real blockchain, you'd maintain a height-to-hash mapping
        if height == 0 {
            return self.get_genesis_block();
        }

        let mut current_hash = self.latest_block_hash();
        let mut current_height = self.height();

        while current_height > height {
            if let Some(block) = self.get_block(&current_hash)? {
                current_hash = block.header.previous_hash;
                current_height -= 1;
            } else {
                return Ok(None);
            }
        }

        if current_height == height {
            self.get_block(&current_hash)
        } else {
            Ok(None)
        }
    }

    /// Get the genesis block
    fn get_genesis_block(&self) -> Result<Option<Block>> {
        // For now, return None - genesis block creation will be implemented separately
        Ok(None)
    }

    /// Get blockchain state
    pub fn state(&self) -> &BlockchainState {
        &self.state
    }
}

/// Transaction outpoint (reference to a specific output)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionOutpoint {
    pub tx_hash: Hash,
    pub output_index: u32,
}

impl TransactionOutpoint {
    /// Create a new transaction outpoint
    pub fn new(tx_hash: Hash, output_index: u32) -> Self {
        Self { tx_hash, output_index }
    }
}
