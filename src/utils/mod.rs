//! Utility functions and helpers
//!
//! This module provides various utility functions used throughout Unicoin.

use crate::{Result, UnicoinError};
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in seconds since Unix epoch
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Get current timestamp in milliseconds since Unix epoch
pub fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Format a timestamp as a human-readable string
pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0);
    match datetime {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}

/// Calculate the difficulty target based on block time
pub fn calculate_difficulty_target(block_time: u64, target_block_time: u64, current_target: u64) -> u64 {
    if block_time == 0 {
        return current_target;
    }

    // Adjust difficulty to maintain target block time
    let adjustment = (target_block_time * 100) / block_time;
    let new_target = (current_target * adjustment) / 100;

    // Ensure target doesn't go below minimum or above maximum
    new_target.max(1).min(u64::MAX)
}

/// Convert satoshis to UNI (with 9 decimal places)
pub fn satoshis_to_uni(satoshis: u64) -> f64 {
    satoshis as f64 / 1_000_000_000.0
}

/// Convert UNI to satoshis (with 9 decimal places)
pub fn uni_to_satoshis(uni: f64) -> Result<u64> {
    let satoshis = (uni * 1_000_000_000.0).round() as u64;
    
    if satoshis > crate::MAX_SUPPLY {
        return Err(UnicoinError::InvalidInput(
            "Amount exceeds maximum supply".to_string()
        ));
    }
    
    Ok(satoshis)
}

/// Calculate transaction fee based on transaction size and priority
pub fn calculate_transaction_fee(size: usize, priority: TransactionPriority) -> u64 {
    let base_fee = crate::MIN_TRANSACTION_FEE;
    let size_fee = (size as u64 * 1000) / 1024; // 1000 satoshis per KB
    
    match priority {
        TransactionPriority::Low => base_fee + size_fee,
        TransactionPriority::Normal => (base_fee + size_fee) * 2,
        TransactionPriority::High => (base_fee + size_fee) * 4,
        TransactionPriority::Urgent => (base_fee + size_fee) * 8,
    }
}

/// Transaction priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Validate an address format
pub fn validate_address(address: &str) -> Result<bool> {
    if address.is_empty() {
        return Err(UnicoinError::InvalidInput("Address cannot be empty".to_string()));
    }

    // Check if it's a valid hex string
    if address.starts_with("0x") {
        let hex_part = &address[2..];
        if hex_part.len() == 40 && hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(true);
        }
    }

    // Check if it's a valid base58 string (Bitcoin-style)
    if is_valid_base58(address) {
        return Ok(true);
    }

    // Check if it's a valid bech32 string
    if is_valid_bech32(address) {
        return Ok(true);
    }

    Err(UnicoinError::InvalidInput("Invalid address format".to_string()))
}

/// Check if a string is valid base58
fn is_valid_base58(s: &str) -> bool {
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    
    if s.is_empty() {
        return false;
    }
    
    s.chars().all(|c| BASE58_ALPHABET.contains(c))
}

/// Check if a string is valid bech32
fn is_valid_bech32(s: &str) -> bool {
    // Simplified bech32 validation
    if !s.starts_with("uni1") && !s.starts_with("uni1") {
        return false;
    }
    
    if s.len() < 14 || s.len() > 74 {
        return false;
    }
    
    // Check for valid characters
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '1')
}

/// Calculate Merkle root from a list of hashes
pub fn calculate_merkle_root(hashes: &[crate::crypto::Hash]) -> crate::crypto::Hash {
    if hashes.is_empty() {
        return crate::crypto::Hash::zero();
    }

    if hashes.len() == 1 {
        return hashes[0];
    }

    let mut current_level = hashes.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        
        for chunk in current_level.chunks(2) {
            if chunk.len() == 2 {
                let combined = combine_hashes(chunk[0], chunk[1]);
                next_level.push(combined);
            } else {
                // Odd number of hashes, duplicate the last one
                let combined = combine_hashes(chunk[0], chunk[0]);
                next_level.push(combined);
            }
        }
        
        current_level = next_level;
    }

    current_level[0]
}

/// Combine two hashes using double SHA-256
fn combine_hashes(left: crate::crypto::Hash, right: crate::crypto::Hash) -> crate::crypto::Hash {
    let mut combined = Vec::new();
    combined.extend_from_slice(left.as_bytes());
    combined.extend_from_slice(right.as_bytes());
    crate::crypto::double_sha256(&combined)
}

/// Generate a random nonce
pub fn generate_nonce() -> u64 {
    use rand::RngCore;
    rand::thread_rng().next_u64()
}

/// Generate a random seed
pub fn generate_seed() -> [u8; 32] {
    use rand::RngCore;
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

/// Derive a child key from a parent key using BIP32
pub fn derive_child_key(parent_key: &[u8], child_index: u32, hardened: bool) -> Result<[u8; 32]> {
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    
    type HmacSha512 = Hmac<Sha512>;
    
    let mut hmac = HmacSha512::new_from_slice(parent_key)
        .map_err(|_| UnicoinError::Crypto("Invalid parent key".to_string()))?;
    
    let child_index_bytes = if hardened {
        (child_index + 0x80000000).to_be_bytes()
    } else {
        child_index.to_be_bytes()
    };
    
    hmac.update(&child_index_bytes);
    let result = hmac.finalize().into_bytes();
    
    let mut child_key = [0u8; 32];
    child_key.copy_from_slice(&result[..32]);
    
    Ok(child_key)
}

/// Calculate the distance between two hashes (for DHT routing)
pub fn calculate_hash_distance(hash1: &crate::crypto::Hash, hash2: &crate::crypto::Hash) -> u64 {
    hash1.distance_from(hash2)
}

/// Check if a hash is within a certain range
pub fn is_hash_in_range(hash: &crate::crypto::Hash, start: &crate::crypto::Hash, end: &crate::crypto::Hash) -> bool {
    let hash_value = hash.as_u64();
    let start_value = start.as_u64();
    let end_value = end.as_u64();
    
    if start_value <= end_value {
        hash_value >= start_value && hash_value <= end_value
    } else {
        // Handle wraparound case
        hash_value >= start_value || hash_value <= end_value
    }
}

/// Serialize data to bytes
pub fn serialize_to_bytes<T>(data: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    bincode::serialize(data)
        .map_err(|e| UnicoinError::Serialization(serde_json::Error::from(e)))
}

/// Deserialize data from bytes
pub fn deserialize_from_bytes<T>(bytes: &[u8]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    bincode::deserialize(bytes)
        .map_err(|e| UnicoinError::Serialization(serde_json::Error::from(e)))
}

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex::decode(hex)
        .map_err(|e| UnicoinError::InvalidInput(format!("Invalid hex string: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_functions() {
        let timestamp = timestamp();
        let timestamp_ms = timestamp_ms();
        
        assert!(timestamp > 0);
        assert!(timestamp_ms > timestamp * 1000);
        assert!(timestamp_ms < timestamp * 1000 + 1000);
    }

    #[test]
    fn test_uni_conversion() {
        let satoshis = 1_000_000_000;
        let uni = satoshis_to_uni(satoshis);
        assert_eq!(uni, 1.0);
        
        let converted_back = uni_to_satoshis(uni).unwrap();
        assert_eq!(converted_back, satoshis);
    }

    #[test]
    fn test_transaction_fee_calculation() {
        let size = 1024; // 1 KB
        let low_fee = calculate_transaction_fee(size, TransactionPriority::Low);
        let high_fee = calculate_transaction_fee(size, TransactionPriority::High);
        
        assert!(high_fee > low_fee);
        assert!(low_fee >= crate::MIN_TRANSACTION_FEE);
    }

    #[test]
    fn test_address_validation() {
        // Valid hex address
        assert!(validate_address("0x1234567890abcdef1234567890abcdef12345678").is_ok());
        
        // Invalid addresses
        assert!(validate_address("").is_err());
        assert!(validate_address("invalid").is_err());
    }

    #[test]
    fn test_merkle_root_calculation() {
        use crate::crypto::Hash;
        
        let hash1 = Hash::from_string("test1");
        let hash2 = Hash::from_string("test2");
        let hash3 = Hash::from_string("test3");
        
        let merkle_root = calculate_merkle_root(&[hash1, hash2, hash3]);
        assert!(!merkle_root.is_zero());
        
        // Same hashes should produce same merkle root
        let merkle_root2 = calculate_merkle_root(&[hash1, hash2, hash3]);
        assert_eq!(merkle_root, merkle_root2);
    }

    #[test]
    fn test_hash_distance() {
        use crate::crypto::Hash;
        
        let hash1 = Hash::from_string("test1");
        let hash2 = Hash::from_string("test2");
        
        let distance = calculate_hash_distance(&hash1, &hash2);
        assert!(distance > 0);
        
        // Distance from self should be 0
        let self_distance = calculate_hash_distance(&hash1, &hash1);
        assert_eq!(self_distance, 0);
    }

    #[test]
    fn test_serialization() {
        let data = vec![1, 2, 3, 4, 5];
        let bytes = serialize_to_bytes(&data).unwrap();
        let deserialized: Vec<i32> = deserialize_from_bytes(&bytes).unwrap();
        
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_hex_conversion() {
        let original = vec![0x01, 0x02, 0x03, 0x04];
        let hex = bytes_to_hex(&original);
        let converted = hex_to_bytes(&hex).unwrap();
        
        assert_eq!(original, converted);
    }
}
