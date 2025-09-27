//! Cryptographic keys implementation
//!
//! This module provides PublicKey and PrivateKey types for Unicoin.

use crate::{Result, UnicoinError, crypto::{CryptoAlgorithm, Hash}};
use serde::{Deserialize, Serialize};
use secp256k1::{Secp256k1, SecretKey, PublicKey as Secp256k1PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::RngCore;
use std::fmt;

/// A cryptographic public key
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey {
    /// The cryptographic algorithm used
    pub algorithm: CryptoAlgorithm,
    /// The public key data
    pub data: Vec<u8>,
}

impl PublicKey {
    /// Create a new public key
    pub fn new(algorithm: CryptoAlgorithm, data: Vec<u8>) -> Self {
        Self { algorithm, data }
    }

    /// Create a Secp256k1 public key
    pub fn new_secp256k1(data: Vec<u8>) -> Self {
        Self {
            algorithm: CryptoAlgorithm::Secp256k1,
            data,
        }
    }

    /// Create an Ed25519 public key
    pub fn new_ed25519(data: [u8; 32]) -> Self {
        Self {
            algorithm: CryptoAlgorithm::Ed25519,
            data: data.to_vec(),
        }
    }

    /// Create a post-quantum public key
    pub fn new_post_quantum(data: Vec<u8>) -> Self {
        Self {
            algorithm: CryptoAlgorithm::PostQuantum,
            data,
        }
    }

    /// Convert to hash (for address generation)
    pub fn to_hash(&self) -> Hash {
        match self.algorithm {
            CryptoAlgorithm::Secp256k1 => {
                // For Secp256k1, use compressed public key + RIPEMD-160
                let compressed = self.get_compressed_secp256k1().unwrap_or_else(|_| self.data.clone());
                crate::crypto::ripemd160(&compressed);
                Hash::from_bytes(&crate::crypto::ripemd160(&compressed))
            }
            CryptoAlgorithm::Ed25519 => {
                // For Ed25519, use the public key directly
                Hash::from_bytes(&self.data)
            }
            CryptoAlgorithm::PostQuantum => {
                // For post-quantum, use SHA-256 of the key
                crate::crypto::sha256(&self.data)
            }
        }
    }

    /// Get compressed Secp256k1 public key
    pub fn get_compressed_secp256k1(&self) -> Result<Vec<u8>> {
        if self.algorithm != CryptoAlgorithm::Secp256k1 {
            return Err(UnicoinError::Crypto(
                "Not a Secp256k1 public key".to_string()
            ));
        }

        let secp = Secp256k1::new();
        let pubkey = Secp256k1PublicKey::from_slice(&self.data)?;
        Ok(pubkey.serialize().to_vec())
    }

    /// Verify that this public key is valid
    pub fn verify(&self) -> bool {
        match self.algorithm {
            CryptoAlgorithm::Secp256k1 => {
                Secp256k1PublicKey::from_slice(&self.data).is_ok()
            }
            CryptoAlgorithm::Ed25519 => {
                if self.data.len() != 32 {
                    return false;
                }
                VerifyingKey::from_bytes(&self.data.try_into().unwrap_or([0u8; 32])).is_ok()
            }
            CryptoAlgorithm::PostQuantum => {
                // Post-quantum key validation would be implemented here
                !self.data.is_empty()
            }
        }
    }

    /// Get the key size in bytes
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
        
        let public_key = Self::new(algorithm, data);
        if !public_key.verify() {
            return Err(UnicoinError::Crypto("Invalid public key".to_string()));
        }
        
        Ok(public_key)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.to_hex())
    }
}

/// A cryptographic private key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey {
    /// The cryptographic algorithm used
    pub algorithm: CryptoAlgorithm,
    /// The private key data
    pub data: Vec<u8>,
}

impl PrivateKey {
    /// Create a new private key
    pub fn new(algorithm: CryptoAlgorithm, data: Vec<u8>) -> Self {
        Self { algorithm, data }
    }

    /// Generate a new Secp256k1 private key
    pub fn new_secp256k1() -> Result<Self> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        
        Ok(Self {
            algorithm: CryptoAlgorithm::Secp256k1,
            data: secret_key.secret_bytes().to_vec(),
        })
    }

    /// Generate a new Ed25519 private key
    pub fn new_ed25519() -> Result<Self> {
        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        
        Ok(Self {
            algorithm: CryptoAlgorithm::Ed25519,
            data: signing_key.to_bytes().to_vec(),
        })
    }

    /// Generate a new post-quantum private key
    pub fn new_post_quantum() -> Result<Self> {
        // Post-quantum key generation would be implemented here
        let mut data = vec![0u8; 64]; // Placeholder size
        rand::thread_rng().fill_bytes(&mut data);
        
        Ok(Self {
            algorithm: CryptoAlgorithm::PostQuantum,
            data,
        })
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> PublicKey {
        match self.algorithm {
            CryptoAlgorithm::Secp256k1 => {
                let secp = Secp256k1::new();
                let secret_key = SecretKey::from_slice(&self.data).unwrap();
                let public_key = Secp256k1PublicKey::from_secret_key(&secp, &secret_key);
                PublicKey::new_secp256k1(public_key.serialize().to_vec())
            }
            CryptoAlgorithm::Ed25519 => {
                let signing_key = SigningKey::from_bytes(&self.data.try_into().unwrap_or([0u8; 32]));
                let verifying_key = signing_key.verifying_key();
                PublicKey::new_ed25519(verifying_key.to_bytes())
            }
            CryptoAlgorithm::PostQuantum => {
                // Post-quantum public key derivation would be implemented here
                let mut pubkey_data = vec![0u8; 32]; // Placeholder
                rand::thread_rng().fill_bytes(&mut pubkey_data);
                PublicKey::new_post_quantum(pubkey_data)
            }
        }
    }

    /// Sign a message with this private key
    pub fn sign(&self, message: &[u8]) -> Result<crate::crypto::Signature> {
        crate::crypto::sign_message(message, self)
    }

    /// Verify that this private key is valid
    pub fn verify(&self) -> bool {
        match self.algorithm {
            CryptoAlgorithm::Secp256k1 => {
                SecretKey::from_slice(&self.data).is_ok()
            }
            CryptoAlgorithm::Ed25519 => {
                if self.data.len() != 32 {
                    return false;
                }
                SigningKey::from_bytes(&self.data.try_into().unwrap_or([0u8; 32])).is_ok()
            }
            CryptoAlgorithm::PostQuantum => {
                // Post-quantum key validation would be implemented here
                !self.data.is_empty()
            }
        }
    }

    /// Get the key size in bytes
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
        
        let private_key = Self::new(algorithm, data);
        if !private_key.verify() {
            return Err(UnicoinError::Crypto("Invalid private key".to_string()));
        }
        
        Ok(private_key)
    }

    /// Encrypt the private key with a password
    pub fn encrypt(&self, password: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
        
        let salt = crate::crypto::generate_salt();
        let key = crate::crypto::derive_key_from_password(password, &salt, 100000)?;
        
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(&salt[..12]); // Use first 12 bytes as nonce
        
        let ciphertext = cipher.encrypt(nonce, self.data.as_ref())
            .map_err(|e| UnicoinError::Crypto(format!("Encryption failed: {}", e)))?;
        
        // Prepend salt to ciphertext
        let mut encrypted = salt.to_vec();
        encrypted.extend_from_slice(&ciphertext);
        
        Ok(encrypted)
    }

    /// Decrypt the private key with a password
    pub fn decrypt(encrypted_data: &[u8], password: &[u8]) -> Result<Self> {
        use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
        
        if encrypted_data.len() < 32 {
            return Err(UnicoinError::Crypto("Invalid encrypted data".to_string()));
        }
        
        let (salt, ciphertext) = encrypted_data.split_at(32);
        let key = crate::crypto::derive_key_from_password(password, salt, 100000)?;
        
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(&salt[..12]);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| UnicoinError::Crypto(format!("Decryption failed: {}", e)))?;
        
        // Try to determine algorithm from data length
        let algorithm = if plaintext.len() == 32 {
            if plaintext.iter().all(|&b| b == 0) {
                CryptoAlgorithm::PostQuantum // Placeholder
            } else {
                CryptoAlgorithm::Ed25519
            }
        } else if plaintext.len() == 32 {
            CryptoAlgorithm::Secp256k1
        } else {
            CryptoAlgorithm::PostQuantum
        };
        
        Ok(Self::new(algorithm, plaintext))
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.to_hex())
    }
}

// Helper trait for converting to Hash
pub trait ToHash {
    fn to_hash(self) -> Hash;
}

impl ToHash for PublicKey {
    fn to_hash(self) -> Hash {
        self.to_hash()
    }
}

impl ToHash for &PublicKey {
    fn to_hash(self) -> Hash {
        self.to_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let private_key = PrivateKey::new_secp256k1().unwrap();
        let public_key = private_key.public_key();
        
        assert_eq!(private_key.algorithm, CryptoAlgorithm::Secp256k1);
        assert_eq!(public_key.algorithm, CryptoAlgorithm::Secp256k1);
        assert!(private_key.verify());
        assert!(public_key.verify());
    }

    #[test]
    fn test_ed25519_keys() {
        let private_key = PrivateKey::new_ed25519().unwrap();
        let public_key = private_key.public_key();
        
        assert_eq!(private_key.algorithm, CryptoAlgorithm::Ed25519);
        assert_eq!(public_key.algorithm, CryptoAlgorithm::Ed25519);
        assert_eq!(private_key.size(), 32);
        assert_eq!(public_key.size(), 32);
    }

    #[test]
    fn test_key_hex_conversion() {
        let private_key = PrivateKey::new_secp256k1().unwrap();
        let hex = private_key.to_hex();
        let restored = PrivateKey::from_hex(CryptoAlgorithm::Secp256k1, &hex).unwrap();
        
        assert_eq!(private_key.data, restored.data);
        assert_eq!(private_key.algorithm, restored.algorithm);
    }

    #[test]
    fn test_key_encryption() {
        let private_key = PrivateKey::new_secp256k1().unwrap();
        let password = b"test_password";
        
        let encrypted = private_key.encrypt(password).unwrap();
        let decrypted = PrivateKey::decrypt(&encrypted, password).unwrap();
        
        assert_eq!(private_key.data, decrypted.data);
        assert_eq!(private_key.algorithm, decrypted.algorithm);
    }

    #[test]
    fn test_public_key_hash() {
        let private_key = PrivateKey::new_secp256k1().unwrap();
        let public_key = private_key.public_key();
        let hash = public_key.to_hash();
        
        assert!(!hash.is_zero());
        
        // Same public key should produce same hash
        let hash2 = public_key.to_hash();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_key_validation() {
        // Valid key
        let private_key = PrivateKey::new_secp256k1().unwrap();
        assert!(private_key.verify());
        
        // Invalid key (wrong size)
        let invalid_private = PrivateKey::new(CryptoAlgorithm::Secp256k1, vec![1, 2, 3]);
        assert!(!invalid_private.verify());
    }
}
