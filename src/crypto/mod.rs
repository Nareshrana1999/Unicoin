//! Cryptographic functions and data structures
//!
//! This module provides all cryptographic functionality for Unicoin including
//! hashing, digital signatures, key generation, and quantum-resistant algorithms.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use blake3::Hasher as Blake3Hasher;
use secp256k1::{Secp256k1, SecretKey, PublicKey as Secp256k1PublicKey, Message, Signature as Secp256k1Signature};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature as Ed25519Signature, Signer, Verifier};
use ring::digest;
use std::fmt;

pub mod hash;
pub mod keys;
pub mod signatures;
pub mod quantum_resistant;

pub use hash::Hash;
pub use keys::{PublicKey, PrivateKey};
pub use signatures::Signature;
pub use quantum_resistant::{PostQuantumSignature, PostQuantumKeyPair};

/// Cryptographic algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    /// Secp256k1 (Bitcoin/Ethereum compatible)
    Secp256k1,
    /// Ed25519 (fast, secure)
    Ed25519,
    /// Post-quantum resistant algorithm
    PostQuantum,
}

/// Generate a cryptographically secure random hash
pub fn random_hash() -> Hash {
    Hash::random()
}

/// Generate a random private key using the specified algorithm
pub fn generate_private_key(algorithm: CryptoAlgorithm) -> Result<PrivateKey> {
    match algorithm {
        CryptoAlgorithm::Secp256k1 => PrivateKey::new_secp256k1(),
        CryptoAlgorithm::Ed25519 => PrivateKey::new_ed25519(),
        CryptoAlgorithm::PostQuantum => PrivateKey::new_post_quantum(),
    }
}

/// Generate a key pair using the specified algorithm
pub fn generate_key_pair(algorithm: CryptoAlgorithm) -> Result<(PrivateKey, PublicKey)> {
    let private_key = generate_private_key(algorithm)?;
    let public_key = private_key.public_key();
    Ok((private_key, public_key))
}

/// Verify a digital signature
pub fn verify_signature(
    message: &[u8],
    signature: &Signature,
    public_key: &PublicKey,
) -> Result<bool> {
    match (&signature.algorithm, &public_key.algorithm) {
        (CryptoAlgorithm::Secp256k1, CryptoAlgorithm::Secp256k1) => {
            let secp = Secp256k1::new();
            let message_hash = Message::from_slice(&Sha256::digest(message).to_vec())?;
            let secp_sig = Secp256k1Signature::from_compact(&signature.data)?;
            let secp_pubkey = Secp256k1PublicKey::from_slice(&public_key.data)?;
            
            Ok(secp.verify_ecdsa(&message_hash, &secp_sig, &secp_pubkey).is_ok())
        }
        (CryptoAlgorithm::Ed25519, CryptoAlgorithm::Ed25519) => {
            let verifying_key = VerifyingKey::from_bytes(&public_key.data.try_into()?)?;
            let ed25519_sig = Ed25519Signature::from_bytes(&signature.data.try_into()?)?;
            
            Ok(verifying_key.verify(message, &ed25519_sig).is_ok())
        }
        (CryptoAlgorithm::PostQuantum, CryptoAlgorithm::PostQuantum) => {
            // Post-quantum signature verification would be implemented here
            // For now, return true as placeholder
            Ok(true)
        }
        _ => Err(UnicoinError::Crypto(
            "Signature and public key algorithm mismatch".to_string()
        )),
    }
}

/// Create a digital signature
pub fn sign_message(
    message: &[u8],
    private_key: &PrivateKey,
) -> Result<Signature> {
    match private_key.algorithm {
        CryptoAlgorithm::Secp256k1 => {
            let secp = Secp256k1::new();
            let message_hash = Message::from_slice(&Sha256::digest(message).to_vec())?;
            let secret_key = SecretKey::from_slice(&private_key.data)?;
            let secp_sig = secp.sign_ecdsa(&message_hash, &secret_key);
            
            Ok(Signature {
                algorithm: CryptoAlgorithm::Secp256k1,
                data: secp_sig.serialize_compact().to_vec(),
            })
        }
        CryptoAlgorithm::Ed25519 => {
            let signing_key = SigningKey::from_bytes(&private_key.data.try_into()?)?;
            let ed25519_sig = signing_key.sign(message);
            
            Ok(Signature {
                algorithm: CryptoAlgorithm::Ed25519,
                data: ed25519_sig.to_bytes().to_vec(),
            })
        }
        CryptoAlgorithm::PostQuantum => {
            // Post-quantum signature creation would be implemented here
            Ok(Signature {
                algorithm: CryptoAlgorithm::PostQuantum,
                data: vec![0u8; 64], // Placeholder
            })
        }
    }
}

/// Compute SHA-256 hash
pub fn sha256(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    Hash::from_bytes(&hasher.finalize())
}

/// Compute BLAKE3 hash
pub fn blake3(data: &[u8]) -> Hash {
    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    Hash::from_bytes(&hasher.finalize())
}

/// Compute double SHA-256 hash (Bitcoin style)
pub fn double_sha256(data: &[u8]) -> Hash {
    sha256(&sha256(data).as_bytes())
}

/// Compute RIPEMD-160 hash
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    use ripemd::Ripemd160;
    use sha2::Digest;
    
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Derive a key from a password using PBKDF2
pub fn derive_key_from_password(password: &[u8], salt: &[u8], iterations: u32) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(iterations).ok_or_else(|| {
            UnicoinError::Crypto("Invalid iteration count".to_string())
        })?,
        salt,
        password,
        &mut key,
    );
    Ok(key)
}

/// Generate a cryptographically secure random salt
pub fn generate_salt() -> [u8; 32] {
    use ring::rand::{SecureRandom, SystemRandom};
    
    let rng = SystemRandom::new();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt).expect("Failed to generate random salt");
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_functions() {
        let data = b"Hello, Unicoin!";
        
        let sha256_hash = sha256(data);
        let blake3_hash = blake3(data);
        let double_sha256_hash = double_sha256(data);
        
        // Hashes should be different
        assert_ne!(sha256_hash, blake3_hash);
        assert_ne!(sha256_hash, double_sha256_hash);
        assert_ne!(blake3_hash, double_sha256_hash);
        
        // Hashes should be deterministic
        assert_eq!(sha256(data), sha256_hash);
        assert_eq!(blake3(data), blake3_hash);
        assert_eq!(double_sha256(data), double_sha256_hash);
    }

    #[test]
    fn test_key_generation() {
        let (private_key, public_key) = generate_key_pair(CryptoAlgorithm::Secp256k1).unwrap();
        
        assert_eq!(private_key.algorithm, CryptoAlgorithm::Secp256k1);
        assert_eq!(public_key.algorithm, CryptoAlgorithm::Secp256k1);
        assert_eq!(public_key, private_key.public_key());
    }

    #[test]
    fn test_signature_verification() {
        let (private_key, public_key) = generate_key_pair(CryptoAlgorithm::Secp256k1).unwrap();
        let message = b"Test message for signing";
        
        let signature = sign_message(message, &private_key).unwrap();
        let is_valid = verify_signature(message, &signature, &public_key).unwrap();
        
        assert!(is_valid);
        
        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_valid_wrong = verify_signature(wrong_message, &signature, &public_key).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_ed25519_signatures() {
        let (private_key, public_key) = generate_key_pair(CryptoAlgorithm::Ed25519).unwrap();
        let message = b"Test message for Ed25519";
        
        let signature = sign_message(message, &private_key).unwrap();
        let is_valid = verify_signature(message, &signature, &public_key).unwrap();
        
        assert!(is_valid);
        assert_eq!(signature.algorithm, CryptoAlgorithm::Ed25519);
    }

    #[test]
    fn test_key_derivation() {
        let password = b"test_password";
        let salt = generate_salt();
        let iterations = 10000;
        
        let key1 = derive_key_from_password(password, &salt, iterations).unwrap();
        let key2 = derive_key_from_password(password, &salt, iterations).unwrap();
        
        // Same password, salt, and iterations should produce same key
        assert_eq!(key1, key2);
        
        // Different salt should produce different key
        let different_salt = generate_salt();
        let key3 = derive_key_from_password(password, &different_salt, iterations).unwrap();
        assert_ne!(key1, key3);
    }
}
