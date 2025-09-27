//! Integration tests for Unicoin
//!
//! This module contains integration tests that test the complete
//! blockchain functionality including genesis block, transactions,
//! and block validation.

use unicoin::{
    blockchain::{Blockchain, Block, Transaction, TransactionType, TransactionInput, TransactionOutput},
    crypto::{Hash, PublicKey, PrivateKey},
    consensus::ConsensusEngine,
    config::UnicoinConfig,
    Result,
};
use tempfile::tempdir;

/// Test blockchain initialization
#[tokio::test]
async fn test_blockchain_initialization() -> Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_blockchain");
    
    // Create a new blockchain
    let blockchain = Blockchain::new()?;
    
    // Verify initial state
    assert_eq!(blockchain.height(), 0);
    assert!(blockchain.latest_block_hash().is_zero());
    
    Ok(())
}

/// Test genesis block creation and validation
#[tokio::test]
async fn test_genesis_block_creation() -> Result<()> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_genesis");
    
    // Create blockchain which should automatically create genesis block
    let blockchain = Blockchain::new()?;
    
    // Get the genesis block
    let genesis_block = blockchain.get_genesis_block()?;
    assert!(genesis_block.is_some());
    
    let genesis = genesis_block.unwrap();
    assert_eq!(genesis.header.height, 0);
    assert!(genesis.header.previous_hash.is_zero());
    assert!(!genesis.transactions.is_empty());
    
    // All transactions should be coinbase
    for tx in &genesis.transactions {
        assert_eq!(tx.tx_type, TransactionType::Coinbase);
    }
    
    Ok(())
}

/// Test transaction creation and validation
#[tokio::test]
async fn test_transaction_creation() -> Result<()> {
    // Create test keys
    let sender_private = PrivateKey::random();
    let sender_public = sender_private.public_key();
    let recipient_public = PublicKey::random();
    
    // Create transaction outputs
    let outputs = vec![
        TransactionOutput::new(1000, recipient_public.clone()),
        TransactionOutput::new(500, sender_public.clone()), // Change
    ];
    
    // Create transaction inputs (simplified for testing)
    let inputs = vec![
        TransactionInput::new_bitcoin(
            Hash::random(),
            0,
            vec![], // Empty script for now
        ),
    ];
    
    // Create transaction
    let mut transaction = Transaction::new(
        TransactionType::Transfer,
        inputs,
        outputs,
        100, // Fee
    );
    
    // Sign the transaction
    transaction.sign(&sender_private)?;
    
    // Validate the transaction
    transaction.validate()?;
    
    assert_eq!(transaction.tx_type, TransactionType::Transfer);
    assert_eq!(transaction.fee, 100);
    assert_eq!(transaction.outputs.len(), 2);
    
    Ok(())
}

/// Test block creation and validation
#[tokio::test]
async fn test_block_creation() -> Result<()> {
    let temp_dir = tempdir()?;
    let blockchain = Blockchain::new()?;
    
    // Create a test transaction
    let sender_private = PrivateKey::random();
    let recipient_public = PublicKey::random();
    
    let outputs = vec![TransactionOutput::new(1000, recipient_public)];
    let inputs = vec![TransactionInput::new_bitcoin(Hash::random(), 0, vec![])];
    
    let mut transaction = Transaction::new(
        TransactionType::Transfer,
        inputs,
        outputs,
        100,
    );
    transaction.sign(&sender_private)?;
    
    // Create a block with the transaction
    let transactions = vec![transaction];
    let block = Block::new(
        blockchain.latest_block_hash(),
        transactions,
        blockchain.height() + 1,
        1, // Target
    );
    
    // Validate the block
    block.validate()?;
    
    assert_eq!(block.header.height, 1);
    assert_eq!(block.transactions.len(), 1);
    assert!(!block.header.previous_hash.is_zero());
    
    Ok(())
}

/// Test blockchain state management
#[tokio::test]
async fn test_blockchain_state() -> Result<()> {
    let temp_dir = tempdir()?;
    let mut blockchain = Blockchain::new()?;
    
    // Verify initial state
    let initial_state = blockchain.state();
    assert_eq!(initial_state.height, 0);
    assert!(initial_state.latest_block_hash.is_zero());
    assert_eq!(initial_state.total_supply, 0);
    
    // Test balance lookup
    let test_address = Hash::random();
    let balance = initial_state.get_balance(&test_address);
    assert_eq!(balance, 0);
    
    Ok(())
}

/// Test consensus engine initialization
#[tokio::test]
async fn test_consensus_engine() -> Result<()> {
    let temp_dir = tempdir()?;
    let blockchain = Blockchain::new()?;
    
    // Create consensus engine
    let consensus = ConsensusEngine::new(blockchain)?;
    
    // Verify consensus engine is created successfully
    assert!(consensus.is_ok());
    
    Ok(())
}

/// Test configuration loading
#[tokio::test]
async fn test_configuration_loading() -> Result<()> {
    // Test default configuration
    let config = UnicoinConfig::new();
    
    // Validate configuration
    config.validate()?;
    
    // Check some key settings
    assert!(config.network.max_peers > 0);
    assert!(config.blockchain.block_time > 0);
    assert!(config.consensus.min_validators > 0);
    assert!(config.features.enable_ai);
    assert!(config.features.enable_defi);
    
    Ok(())
}

/// Test crypto operations
#[tokio::test]
async fn test_crypto_operations() -> Result<()> {
    // Test key generation
    let private_key = PrivateKey::random();
    let public_key = private_key.public_key();
    
    assert_ne!(private_key, PrivateKey::random());
    assert_eq!(public_key, private_key.public_key());
    
    // Test hashing
    let data = b"Hello, Unicoin!";
    let hash1 = Hash::from_bytes(data);
    let hash2 = Hash::from_bytes(data);
    
    assert_eq!(hash1, hash2);
    assert!(!hash1.is_zero());
    
    // Test signature creation and verification
    let message = b"Test message";
    let signature = private_key.sign(message)?;
    
    assert!(public_key.verify(message, &signature)?);
    
    // Test with wrong message
    let wrong_message = b"Wrong message";
    assert!(!public_key.verify(wrong_message, &signature)?);
    
    Ok(())
}

/// Test API handlers
#[tokio::test]
async fn test_api_handlers() -> Result<()> {
    use unicoin::api::ApiHandlers;
    
    let handlers = ApiHandlers::new();
    
    // Test blockchain info endpoint
    let response = handlers.get_blockchain_info().await?;
    assert!(response.0.success);
    
    // Test health check endpoint
    let health_response = handlers.health_check().await?;
    assert!(health_response.0.success);
    
    // Test node info endpoint
    let node_response = handlers.get_node_info().await?;
    assert!(node_response.0.success);
    
    Ok(())
}

/// Test performance benchmarks
#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    use std::time::Instant;
    
    // Benchmark transaction creation
    let start = Instant::now();
    
    for _ in 0..1000 {
        let private_key = PrivateKey::random();
        let public_key = private_key.public_key();
        
        let outputs = vec![TransactionOutput::new(1000, public_key)];
        let inputs = vec![TransactionInput::new_bitcoin(Hash::random(), 0, vec![])];
        
        let mut transaction = Transaction::new(
            TransactionType::Transfer,
            inputs,
            outputs,
            100,
        );
        transaction.sign(&private_key)?;
    }
    
    let duration = start.elapsed();
    println!("Created 1000 transactions in {:?}", duration);
    
    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_secs() < 1);
    
    Ok(())
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    use unicoin::UnicoinError;
    
    // Test invalid transaction
    let invalid_outputs = vec![TransactionOutput::new(0, PublicKey::random())]; // Zero amount
    let inputs = vec![TransactionInput::new_bitcoin(Hash::random(), 0, vec![])];
    
    let transaction = Transaction::new(
        TransactionType::Transfer,
        inputs,
        outputs,
        100,
    );
    
    // Should fail validation
    assert!(transaction.validate().is_err());
    
    Ok(())
}

/// Test network simulation
#[tokio::test]
async fn test_network_simulation() -> Result<()> {
    use unicoin::network::NetworkNode;
    
    // Create multiple blockchain instances to simulate network
    let blockchain1 = Blockchain::new()?;
    let blockchain2 = Blockchain::new()?;
    
    // Both should have the same genesis block
    let genesis1 = blockchain1.get_genesis_block()?;
    let genesis2 = blockchain2.get_genesis_block()?;
    
    assert!(genesis1.is_some());
    assert!(genesis2.is_some());
    
    // Genesis blocks should be identical
    assert_eq!(genesis1.unwrap().hash(), genesis2.unwrap().hash());
    
    Ok(())
}
