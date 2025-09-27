//! Staking rewards implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingReward {
    pub id: Hash,
    pub amount: u64,
    pub period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculator {
    pub base_rate: u16,
    pub bonus_rate: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingPosition {
    pub id: Hash,
    pub user: PublicKey,
    pub amount: u64,
    pub rewards: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub total_rewards: u64,
    pub distributed: u64,
    pub pending: u64,
}
