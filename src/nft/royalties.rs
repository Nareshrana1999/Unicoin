//! NFT royalties implementation for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyManager {
    pub name: String,
    pub version: String,
}

impl RoyaltyManager {
    pub fn new() -> Self {
        Self {
            name: "Unicoin Royalty Manager".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyInfo {
    pub recipient: PublicKey,
    pub percentage: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyDistribution {
    pub total_royalties: u64,
    pub distributed: u64,
    pub pending: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyRecipient {
    pub recipient: PublicKey,
    pub share_percentage: u16,
}
