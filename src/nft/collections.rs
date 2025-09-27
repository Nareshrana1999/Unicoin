//! NFT collections implementation for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTCollection {
    pub id: Hash,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub creator: PublicKey,
    pub standard: crate::nft::standards::NFTStandard,
    pub total_supply: u64,
    pub current_supply: u64,
    pub metadata: CollectionMetadata,
    pub stats: CollectionStats,
    pub created_at: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMetadata {
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
}

impl CollectionMetadata {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            image: String::new(),
            external_url: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub volume: u64,
    pub floor_price: u64,
    pub owners: u64,
}

impl CollectionStats {
    pub fn new() -> Self {
        Self {
            volume: 0,
            floor_price: 0,
            owners: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionCreator {
    pub creator: PublicKey,
    pub royalty_percentage: u16,
}
