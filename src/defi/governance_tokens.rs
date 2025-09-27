//! Governance tokens implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceToken {
    pub id: Hash,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPower {
    pub user: PublicKey,
    pub power: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Hash,
    pub proposer: PublicKey,
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub id: Hash,
    pub voter: PublicKey,
    pub proposal_id: Hash,
    pub vote: bool,
    pub weight: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub delegator: PublicKey,
    pub delegate: PublicKey,
    pub amount: u64,
}
