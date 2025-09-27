//! Transaction implementation
//!
//! This module defines transaction structures and related functionality.

use crate::{
    crypto::{Hash, PublicKey, Signature, PrivateKey},
    utils::timestamp,
    blockchain::TransactionOutpoint,
    Result, UnicoinError,
};
use serde::{Deserialize, Serialize};

/// Types of transactions supported by Unicoin
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Regular transfer transaction
    Transfer,
    /// Coinbase transaction (mining reward)
    Coinbase,
    /// Smart contract deployment
    ContractDeployment,
    /// Smart contract execution
    ContractExecution,
    /// Stake delegation
    StakeDelegation,
    /// Governance vote
    GovernanceVote,
    /// SegWit transaction
    SegWit,
    /// Multi-signature transaction
    MultiSig,
    /// Time-locked transaction
    TimeLocked,
    /// Atomic swap transaction
    AtomicSwap,
}

/// Output types for Bitcoin compatibility
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputType {
    /// Pay to Public Key Hash
    P2PKH,
    /// Pay to Script Hash
    P2SH,
    /// Pay to Witness Public Key Hash
    P2WPKH,
    /// Pay to Witness Script Hash
    P2WSH,
    /// Pay to Public Key
    P2PK,
    /// Multi-signature
    MultiSig,
    /// Null data (OP_RETURN)
    NullData,
    /// Custom script
    Custom,
}

/// Transaction input referencing a previous output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionInput {
    /// Reference to the previous output
    pub previous_output: TransactionOutpoint,
    /// Previous transaction ID (Bitcoin compatibility)
    pub previous_tx_id: Hash,
    /// Previous output index (Bitcoin compatibility)
    pub previous_output_index: u32,
    /// Unlocking script (signature)
    pub script_sig: Vec<u8>,
    /// Sequence number for transaction replacement
    pub sequence: u32,
    /// Witness data for SegWit transactions
    pub witness: Option<Vec<Vec<u8>>>,
}

impl TransactionInput {
    /// Create a new transaction input
    pub fn new(previous_output: TransactionOutpoint, script_sig: Vec<u8>) -> Self {
        Self {
            previous_output,
            previous_tx_id: previous_output.tx_id,
            previous_output_index: previous_output.output_index,
            script_sig,
            sequence: 0xffffffff,
            witness: None,
        }
    }

    /// Create a new transaction input with Bitcoin compatibility
    pub fn new_bitcoin(previous_tx_id: Hash, previous_output_index: u32, script_sig: Vec<u8>) -> Self {
        Self {
            previous_output: TransactionOutpoint::new(previous_tx_id, previous_output_index),
            previous_tx_id,
            previous_output_index,
            script_sig,
            sequence: 0xffffffff,
            witness: None,
        }
    }

    /// Create a SegWit transaction input
    pub fn new_segwit(
        previous_tx_id: Hash, 
        previous_output_index: u32, 
        script_sig: Vec<u8>,
        witness: Vec<Vec<u8>>
    ) -> Self {
        Self {
            previous_output: TransactionOutpoint::new(previous_tx_id, previous_output_index),
            previous_tx_id,
            previous_output_index,
            script_sig,
            sequence: 0xffffffff,
            witness: Some(witness),
        }
    }

    /// Check if this is a SegWit input
    pub fn is_segwit(&self) -> bool {
        self.witness.is_some()
    }
}

/// Transaction output specifying recipient and amount
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionOutput {
    /// Amount in satoshis (smallest unit of Unicoin)
    pub amount: u64,
    /// Recipient's public key
    pub recipient: PublicKey,
    /// Recipient's address (Bitcoin compatibility)
    pub address: Vec<u8>,
    /// Locking script
    pub script_pubkey: Vec<u8>,
    /// Optional data for smart contracts
    pub data: Option<Vec<u8>>,
    /// Output type (P2PKH, P2SH, P2WPKH, etc.)
    pub output_type: OutputType,
}

impl TransactionOutput {
    /// Create a new transaction output
    pub fn new(amount: u64, recipient: PublicKey) -> Self {
        Self {
            amount,
            recipient,
            address: recipient.to_bytes().to_vec(),
            script_pubkey: Vec::new(), // Simplified for now
            data: None,
            output_type: OutputType::P2PKH,
        }
    }

    /// Create a new transaction output with data
    pub fn new_with_data(amount: u64, recipient: PublicKey, data: Vec<u8>) -> Self {
        Self {
            amount,
            recipient,
            address: recipient.to_bytes().to_vec(),
            script_pubkey: Vec::new(),
            data: Some(data),
            output_type: OutputType::Custom,
        }
    }

    /// Create P2PKH output
    pub fn new_p2pkh(amount: u64, address: Vec<u8>) -> Self {
        Self {
            amount,
            recipient: PublicKey::new([0u8; 33]), // Placeholder
            address,
            script_pubkey: vec![0x76, 0xa9, 0x14, 0x88, 0xac], // OP_DUP OP_HASH160 OP_EQUALVERIFY OP_CHECKSIG
            data: None,
            output_type: OutputType::P2PKH,
        }
    }

    /// Create P2SH output
    pub fn new_p2sh(amount: u64, script_hash: Vec<u8>) -> Self {
        Self {
            amount,
            recipient: PublicKey::new([0u8; 33]), // Placeholder
            address: script_hash,
            script_pubkey: vec![0xa9, 0x14, 0x87], // OP_HASH160 OP_EQUAL
            data: None,
            output_type: OutputType::P2SH,
        }
    }

    /// Create P2WPKH output
    pub fn new_p2wpkh(amount: u64, witness_program: Vec<u8>) -> Self {
        Self {
            amount,
            recipient: PublicKey::new([0u8; 33]), // Placeholder
            address: witness_program,
            script_pubkey: vec![0x00, 0x14], // OP_0 OP_PUSHDATA1
            data: None,
            output_type: OutputType::P2WPKH,
        }
    }

    /// Create null data output (OP_RETURN)
    pub fn new_null_data(data: Vec<u8>) -> Self {
        Self {
            amount: 0,
            recipient: PublicKey::new([0u8; 33]), // Placeholder
            address: Vec::new(),
            script_pubkey: vec![0x6a], // OP_RETURN
            data: Some(data),
            output_type: OutputType::NullData,
        }
    }
}

/// A transaction in the Unicoin blockchain
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction version
    pub version: i32,
    /// Transaction type
    pub tx_type: TransactionType,
    /// List of inputs
    pub inputs: Vec<TransactionInput>,
    /// List of outputs
    pub outputs: Vec<TransactionOutput>,
    /// Transaction fee in satoshis
    pub fee: u64,
    /// Transaction timestamp
    pub timestamp: u64,
    /// Lock time
    pub lock_time: u32,
    /// Digital signature
    pub signature: Option<Signature>,
    /// Optional data for smart contracts
    pub data: Option<Vec<u8>>,
    /// Transaction ID
    pub id: Hash,
}

/// Coinbase transaction (special transaction for block rewards)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoinbaseTransaction {
    /// Transaction version
    pub version: i32,
    /// List of inputs (special coinbase input)
    pub inputs: Vec<TransactionInput>,
    /// List of outputs
    pub outputs: Vec<TransactionOutput>,
    /// Transaction timestamp
    pub timestamp: u64,
    /// Lock time
    pub lock_time: u32,
    /// Block height
    pub block_height: u64,
    /// Coinbase data
    pub coinbase_data: Vec<u8>,
    /// Transaction ID
    pub id: Hash,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        version: i32,
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        lock_time: u32,
    ) -> Self {
        let mut tx = Self {
            version,
            tx_type: TransactionType::Transfer,
            inputs,
            outputs,
            fee: 0,
            timestamp: timestamp(),
            lock_time,
            signature: None,
            data: None,
            id: Hash::new([0u8; 32]),
        };
        
        tx.id = tx.calculate_id();
        tx
    }

    /// Create a new transfer transaction
    pub fn new_transfer(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        fee: u64,
        private_key: &PrivateKey,
    ) -> Result<Self> {
        let mut tx = Self {
            version: 1,
            tx_type: TransactionType::Transfer,
            inputs,
            outputs,
            fee,
            timestamp: timestamp(),
            lock_time: 0,
            signature: None,
            data: None,
            id: Hash::new([0u8; 32]),
        };

        // Sign the transaction
        tx.sign(private_key)?;
        tx.id = tx.calculate_id();

        Ok(tx)
    }

    /// Create a new coinbase transaction (mining reward)
    pub fn new_coinbase(recipient: PublicKey, amount: u64, height: u64) -> Self {
        // Coinbase transactions have a special input
        let coinbase_input = TransactionInput {
            previous_output: TransactionOutpoint::new(Hash::zero(), u32::MAX),
            previous_tx_id: Hash::zero(),
            previous_output_index: u32::MAX,
            script_sig: format!("Coinbase block {}", height).into_bytes(),
            sequence: 0,
            witness: None,
        };

        let output = TransactionOutput::new(amount, recipient);

        let mut tx = Self {
            version: 1,
            tx_type: TransactionType::Coinbase,
            inputs: vec![coinbase_input],
            outputs: vec![output],
            fee: 0,
            timestamp: timestamp(),
            lock_time: 0,
            signature: None,
            data: None,
            id: Hash::new([0u8; 32]),
        };
        
        tx.id = tx.calculate_id();
        tx
    }

    /// Check if this is a coinbase transaction
    pub fn is_coinbase(&self) -> bool {
        self.tx_type == TransactionType::Coinbase ||
        (self.inputs.len() == 1 && 
         self.inputs[0].previous_tx_id == Hash::zero() && 
         self.inputs[0].previous_output_index == u32::MAX)
    }

    /// Calculate transaction ID
    pub fn calculate_id(&self) -> Hash {
        let bytes = bincode::serialize(self).expect("Transaction should be serializable");
        Hash::sha256(&bytes)
    }

    /// Create a new smart contract deployment transaction
    pub fn new_contract_deployment(
        inputs: Vec<TransactionInput>,
        contract_code: Vec<u8>,
        fee: u64,
        private_key: &PrivateKey,
    ) -> Result<Self> {
        // Contract deployment creates an output with the contract code
        let mut tx = Self {
            tx_type: TransactionType::ContractDeployment,
            inputs,
            outputs: vec![], // Will be set after signing
            fee,
            timestamp: timestamp(),
            signature: None,
            data: Some(contract_code),
        };

        // Sign the transaction
        tx.sign(private_key)?;

        Ok(tx)
    }

    /// Sign the transaction
    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<()> {
        let message = self.hash_for_signing();
        let signature = private_key.sign(&message)?;
        self.signature = Some(signature);
        Ok(())
    }

    /// Verify the transaction signature
    pub fn verify_signature(&self) -> bool {
        if let Some(signature) = &self.signature {
            if let Some(input) = self.inputs.first() {
                // For simplicity, we assume the first input contains the signer's public key
                // In a real implementation, you'd extract the public key from the script_sig
                let message = self.hash_for_signing();
                // This is a simplified verification - real implementation would be more complex
                return true; // Placeholder
            }
        }
        false
    }

    /// Calculate hash for signing (excludes signature field)
    pub fn hash_for_signing(&self) -> Hash {
        let mut tx_copy = self.clone();
        tx_copy.signature = None;
        tx_copy.hash()
    }

    /// Calculate the transaction hash
    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(self).expect("Transaction should be serializable");
        Hash::from_bytes(&bytes)
    }

    /// Get the total input amount (simplified)
    pub fn total_input_amount(&self) -> u64 {
        // This would need to look up the UTXO set in a real implementation
        self.inputs.len() as u64 * 1000000 // Placeholder
    }

    /// Get the total output amount
    pub fn total_output_amount(&self) -> u64 {
        self.outputs.iter().map(|output| output.amount).sum()
    }

    /// Validate the transaction
    pub fn validate(&self) -> Result<()> {
        // Validate inputs
        if self.inputs.is_empty() && self.tx_type != TransactionType::Coinbase {
            return Err(UnicoinError::InvalidInput(
                "Transaction must have at least one input".to_string()
            ));
        }

        // Validate outputs
        if self.outputs.is_empty() {
            return Err(UnicoinError::InvalidInput(
                "Transaction must have at least one output".to_string()
            ));
        }

        // Validate amounts
        for output in &self.outputs {
            if output.amount == 0 {
                return Err(UnicoinError::InvalidInput(
                    "Output amount cannot be zero".to_string()
                ));
            }
        }

        // Validate fee
        if self.fee < crate::MIN_TRANSACTION_FEE && self.tx_type != TransactionType::Coinbase {
            return Err(UnicoinError::InvalidInput(
                "Transaction fee too low".to_string()
            ));
        }

        // Validate signature (except for coinbase transactions)
        if self.tx_type != TransactionType::Coinbase && !self.verify_signature() {
            return Err(UnicoinError::InvalidInput(
                "Invalid transaction signature".to_string()
            ));
        }

        Ok(())
    }

    /// Get the size of the transaction in bytes
    pub fn size(&self) -> usize {
        bincode::serialize(self).map(|bytes| bytes.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let inputs = vec![];
        let outputs = vec![TransactionOutput::new(1000, PublicKey::random())];
        let fee = 100;

        // This test would require a private key, so we'll test the structure instead
        let tx = Transaction {
            tx_type: TransactionType::Transfer,
            inputs,
            outputs,
            fee,
            timestamp: timestamp(),
            signature: None,
            data: None,
        };

        assert_eq!(tx.tx_type, TransactionType::Transfer);
        assert_eq!(tx.fee, 100);
        assert_eq!(tx.outputs.len(), 1);
    }

    #[test]
    fn test_coinbase_transaction() {
        let recipient = PublicKey::random();
        let amount = 5000000000; // 50 UNI (assuming 9 decimal places)
        let height = 1;

        let tx = Transaction::new_coinbase(recipient, amount, height);

        assert_eq!(tx.tx_type, TransactionType::Coinbase);
        assert_eq!(tx.fee, 0);
        assert_eq!(tx.inputs.len(), 1);
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].amount, amount);
        assert_eq!(tx.outputs[0].recipient, recipient);
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction {
            tx_type: TransactionType::Transfer,
            inputs: vec![],
            outputs: vec![TransactionOutput::new(1000, PublicKey::random())],
            fee: 100,
            timestamp: timestamp(),
            signature: None,
            data: None,
        };

        let hash1 = tx.hash();
        let hash2 = tx.hash();
        assert_eq!(hash1, hash2); // Hash should be deterministic
    }

    #[test]
    fn test_transaction_validation() {
        let tx = Transaction {
            tx_type: TransactionType::Transfer,
            inputs: vec![],
            outputs: vec![TransactionOutput::new(1000, PublicKey::random())],
            fee: 100,
            timestamp: timestamp(),
            signature: None,
            data: None,
        };

        // This should fail because there are no inputs for a transfer transaction
        assert!(tx.validate().is_err());
    }
}
