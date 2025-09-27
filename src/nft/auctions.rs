//! NFT auctions implementation for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionManager {
    pub name: String,
    pub version: String,
}

impl AuctionManager {
    pub fn new() -> Self {
        Self {
            name: "Unicoin Auction Manager".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuctionType {
    English,
    Dutch,
    SealedBid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuctionState {
    Active,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub winner: Option<PublicKey>,
    pub winning_bid: u64,
    pub reserve_met: bool,
}
