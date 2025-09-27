//! Hash implementation
//!
//! This module provides the Hash type and related functionality for Unicoin.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use blake3::Hasher as Blake3Hasher;
use std::fmt;
use rand::RngCore;

/// A cryptographic hash value
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Create a hash from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash_bytes = [0u8; 32];
        let len = std::cmp::min(bytes.len(), 32);
        hash_bytes[..len].copy_from_slice(&bytes[..len]);
        Self(hash_bytes)
    }

    /// Create a hash from a 32-byte array
    pub fn from_array(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a zero hash (all zeros)
    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    /// Create a random hash
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }

    /// Create a hash from a string (using SHA-256)
    pub fn from_string(s: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        Self::from_bytes(&hasher.finalize())
    }

    /// Get the hash as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Get the hash as a byte vector
    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| format!("Invalid hex string: {}", e))?;
        
        if bytes.len() != 32 {
            return Err(format!("Hash must be 32 bytes, got {}", bytes.len()));
        }

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&bytes);
        Ok(Self(hash_bytes))
    }

    /// Compute SHA-256 hash of this hash (double hashing)
    pub fn double_hash(&self) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&self.0);
        let first_hash = hasher.finalize();
        
        let mut hasher = Sha256::new();
        hasher.update(&first_hash);
        Self::from_bytes(&hasher.finalize())
    }

    /// Compute BLAKE3 hash of this hash
    pub fn blake3_hash(&self) -> Self {
        let mut hasher = Blake3Hasher::new();
        hasher.update(&self.0);
        Self::from_bytes(&hasher.finalize())
    }

    /// Check if this is a zero hash
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Get the first 8 bytes as a u64 (for difficulty calculations)
    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes([
            self.0[0], self.0[1], self.0[2], self.0[3],
            self.0[4], self.0[5], self.0[6], self.0[7],
        ])
    }

    /// Check if this hash meets a difficulty target
    pub fn meets_target(&self, target: u64) -> bool {
        self.as_u64() <= target
    }

    /// Compute the distance from another hash (XOR)
    pub fn distance_from(&self, other: &Hash) -> u64 {
        let mut distance = 0u64;
        for i in 0..4 {
            let self_u64 = u64::from_le_bytes([
                self.0[i * 8], self.0[i * 8 + 1], self.0[i * 8 + 2], self.0[i * 8 + 3],
                self.0[i * 8 + 4], self.0[i * 8 + 5], self.0[i * 8 + 6], self.0[i * 8 + 7],
            ]);
            let other_u64 = u64::from_le_bytes([
                other.0[i * 8], other.0[i * 8 + 1], other.0[i * 8 + 2], other.0[i * 8 + 3],
                other.0[i * 8 + 4], other.0[i * 8 + 5], other.0[i * 8 + 6], other.0[i * 8 + 7],
            ]);
            distance += self_u64 ^ other_u64;
        }
        distance
    }

    /// Combine this hash with another hash
    pub fn combine(&self, other: &Hash) -> Self {
        let mut combined = Vec::new();
        combined.extend_from_slice(&self.0);
        combined.extend_from_slice(&other.0);
        
        let mut hasher = Sha256::new();
        hasher.update(&combined);
        Self::from_bytes(&hasher.finalize())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::BitXor for Hash {
    type Output = Hash;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = [0u8; 32];
        for i in 0..32 {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        Hash(result)
    }
}

impl std::ops::BitXorAssign for Hash {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..32 {
            self.0[i] ^= rhs.0[i];
        }
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_creation() {
        let hash1 = Hash::from_string("test");
        let hash2 = Hash::from_string("test");
        let hash3 = Hash::from_string("different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_hex_conversion() {
        let original = Hash::from_string("test");
        let hex = original.to_hex();
        let restored = Hash::from_hex(&hex).unwrap();
        
        assert_eq!(original, restored);
    }

    #[test]
    fn test_hash_operations() {
        let hash1 = Hash::from_string("first");
        let hash2 = Hash::from_string("second");
        
        let combined = hash1.combine(&hash2);
        let combined2 = hash2.combine(&hash1);
        
        // Order matters for combination
        assert_ne!(combined, combined2);
        
        // XOR operation
        let xored = hash1.clone() ^ hash2.clone();
        let xored_back = xored ^ hash2;
        assert_eq!(xored_back, hash1);
    }

    #[test]
    fn test_hash_target() {
        let hash = Hash::from_string("test");
        let target = u64::MAX;
        
        assert!(hash.meets_target(target));
        assert!(!hash.meets_target(0));
    }

    #[test]
    fn test_hash_distance() {
        let hash1 = Hash::from_string("test1");
        let hash2 = Hash::from_string("test2");
        let hash3 = Hash::from_string("test1");
        
        let distance1 = hash1.distance_from(&hash2);
        let distance2 = hash1.distance_from(&hash3);
        
        assert!(distance1 > distance2);
        assert_eq!(distance2, 0);
    }

    #[test]
    fn test_zero_hash() {
        let zero = Hash::zero();
        assert!(zero.is_zero());
        
        let non_zero = Hash::from_string("test");
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_random_hash() {
        let hash1 = Hash::random();
        let hash2 = Hash::random();
        
        // Very unlikely to be the same
        assert_ne!(hash1, hash2);
        
        // Should not be zero
        assert!(!hash1.is_zero());
        assert!(!hash2.is_zero());
    }
}
