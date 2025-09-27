//! Yield farming implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldFarm {
    pub name: String,
    pub total_staked: u64,
}

impl YieldFarm {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_staked: 0,
        }
    }

    pub fn get_total_staked(&self) -> u64 {
        self.total_staked
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmReward {
    pub id: Hash,
    pub amount: u64,
    pub token: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmingPosition {
    pub id: Hash,
    pub user: PublicKey,
    pub staked_amount: u64,
    pub rewards: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmStrategy {
    pub name: String,
    pub apy: u16,
    pub risk_level: u8,
}
