//! Digital signatures implementation
//!
//! This module provides the Signature type and related functionality.

use crate::{Result, UnicoinError, crypto::CryptoAlgorithm};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A digital signature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    /// The cryptographic algorithm used
    pub algorithm: CryptoAlgorithm,
    /// The signature data
    pub data: Vec<u8>,
}

impl Signature {
    /// Create a new signature
    pub fn new(algorithm: CryptoAlgorithm, data: Vec<u8>) -> Self {
        Self { algorithm, data }
    }

    /// Create a Secp256k1 signature
    pub fn new_secp256k1(data: Vec<u8>) -> Self {
        Self {
            algorithm: CryptoAlgorithm::Secp256k1,
            data,
        }
    }

    /// Create an Ed25519 signature
    pub fn new_ed25519(data: [u8; 64]) -> Self {
        Self {
            algorithm: CryptoAlgorithm::Ed25519,
            data: data.to_vec(),
        }
    }

    /// Create a post-quantum signature
    pub fn new_post_quantum(data: Vec<u8>) -> Self {
        Self {
            algorithm: CryptoAlgorithm::PostQuantum,
            data,
        }
    }

    /// Get the signature size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.data)
    }

    /// Create from hex string
    pub fn from_hex(algorithm: CryptoAlgorithm, hex_str: &str) -> Result<Self> {
        let data = hex::decode(hex_str)
            .map_err(|e| UnicoinError::Crypto(format!("Invalid hex string: {}", e)))?;
        
        Ok(Self::new(algorithm, data))
    }

    /// Verify that this signature is valid format
    pub fn is_valid_format(&self) -> bool {
        match self.algorithm {
            CryptoAlgorithm::Secp256k1 => self.data.len() == 64,
            CryptoAlgorithm::Ed25519 => self.data.len() == 64,
            CryptoAlgorithm::PostQuantum => self.data.len() > 0, // Variable size
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.to_hex())
    }
}

/// A multi-signature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiSignature {
    /// List of signatures
    pub signatures: Vec<Signature>,
    /// Required number of signatures
    pub threshold: usize,
}

impl MultiSignature {
    /// Create a new multi-signature
    pub fn new(signatures: Vec<Signature>, threshold: usize) -> Self {
        Self { signatures, threshold }
    }

    /// Add a signature to the multi-signature
    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    /// Check if the multi-signature has enough signatures
    pub fn has_enough_signatures(&self) -> bool {
        self.signatures.len() >= self.threshold
    }

    /// Get the number of signatures
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }

    /// Verify that all signatures are valid format
    pub fn is_valid_format(&self) -> bool {
        self.signatures.iter().all(|sig| sig.is_valid_format())
    }
}

/// A threshold signature scheme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThresholdSignature {
    /// The threshold signature data
    pub data: Vec<u8>,
    /// The threshold used
    pub threshold: usize,
    /// Total participants
    pub total_participants: usize,
}

impl ThresholdSignature {
    /// Create a new threshold signature
    pub fn new(data: Vec<u8>, threshold: usize, total_participants: usize) -> Self {
        Self {
            data,
            threshold,
            total_participants,
        }
    }

    /// Verify that the threshold is valid
    pub fn is_valid_threshold(&self) -> bool {
        self.threshold > 0 && self.threshold <= self.total_participants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_creation() {
        let signature = Signature::new_secp256k1(vec![1u8; 64]);
        
        assert_eq!(signature.algorithm, CryptoAlgorithm::Secp256k1);
        assert_eq!(signature.size(), 64);
        assert!(signature.is_valid_format());
    }

    #[test]
    fn test_signature_hex_conversion() {
        let original = Signature::new_secp256k1(vec![1u8; 64]);
        let hex = original.to_hex();
        let restored = Signature::from_hex(CryptoAlgorithm::Secp256k1, &hex).unwrap();
        
        assert_eq!(original, restored);
    }

    #[test]
    fn test_ed25519_signature() {
        let signature = Signature::new_ed25519([1u8; 64]);
        
        assert_eq!(signature.algorithm, CryptoAlgorithm::Ed25519);
        assert_eq!(signature.size(), 64);
        assert!(signature.is_valid_format());
    }

    #[test]
    fn test_multi_signature() {
        let sig1 = Signature::new_secp256k1(vec![1u8; 64]);
        let sig2 = Signature::new_secp256k1(vec![2u8; 64]);
        let threshold = 2;
        
        let mut multi_sig = MultiSignature::new(vec![sig1], threshold);
        
        assert!(!multi_sig.has_enough_signatures());
        assert_eq!(multi_sig.signature_count(), 1);
        
        multi_sig.add_signature(sig2);
        
        assert!(multi_sig.has_enough_signatures());
        assert_eq!(multi_sig.signature_count(), 2);
    }

    #[test]
    fn test_threshold_signature() {
        let threshold_sig = ThresholdSignature::new(vec![1u8; 32], 3, 5);
        
        assert!(threshold_sig.is_valid_threshold());
        assert_eq!(threshold_sig.threshold, 3);
        assert_eq!(threshold_sig.total_participants, 5);
    }

    #[test]
    fn test_invalid_threshold() {
        let invalid_threshold = ThresholdSignature::new(vec![1u8; 32], 0, 5);
        assert!(!invalid_threshold.is_valid_threshold());
        
        let invalid_threshold2 = ThresholdSignature::new(vec![1u8; 32], 6, 5);
        assert!(!invalid_threshold2.is_valid_threshold());
    }
}
