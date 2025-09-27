//! Proof-of-Stake implementation
//!
//! This module provides the Proof-of-Stake consensus mechanism for Unicoin.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Proof-of-Stake consensus mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfStake {
    /// Validator stakes
    stakes: HashMap<crate::crypto::Hash, u64>,
    /// Total stake
    total_stake: u64,
    /// Slashing conditions
    slashing_conditions: Vec<SlashingCondition>,
}

/// Slashing condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlashingCondition {
    /// Double signing
    DoubleSign,
    /// Validator downtime
    Downtime,
    /// Invalid block production
    InvalidBlock,
}

/// Slashing penalty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingPenalty {
    /// Validator address
    pub validator: crate::crypto::Hash,
    /// Penalty amount
    pub penalty: u64,
    /// Reason for slashing
    pub reason: SlashingCondition,
    /// Timestamp
    pub timestamp: u64,
}

impl ProofOfStake {
    /// Create a new Proof-of-Stake instance
    pub fn new() -> Self {
        Self {
            stakes: HashMap::new(),
            total_stake: 0,
            slashing_conditions: vec![
                SlashingCondition::DoubleSign,
                SlashingCondition::Downtime,
                SlashingCondition::InvalidBlock,
            ],
        }
    }

    /// Add stake for a validator
    pub fn add_stake(&mut self, validator: crate::crypto::Hash, amount: u64) -> Result<()> {
        let current_stake = self.stakes.get(&validator).copied().unwrap_or(0);
        self.stakes.insert(validator, current_stake + amount);
        self.total_stake += amount;
        Ok(())
    }

    /// Remove stake for a validator
    pub fn remove_stake(&mut self, validator: &crate::crypto::Hash, amount: u64) -> Result<()> {
        let current_stake = self.stakes.get(validator).copied().unwrap_or(0);
        if current_stake < amount {
            return Err(UnicoinError::Consensus("Insufficient stake".to_string()));
        }
        
        let new_stake = current_stake - amount;
        if new_stake == 0 {
            self.stakes.remove(validator);
        } else {
            self.stakes.insert(*validator, new_stake);
        }
        self.total_stake -= amount;
        Ok(())
    }

    /// Get stake for a validator
    pub fn get_stake(&self, validator: &crate::crypto::Hash) -> u64 {
        self.stakes.get(validator).copied().unwrap_or(0)
    }

    /// Get total stake
    pub fn get_total_stake(&self) -> u64 {
        self.total_stake
    }

    /// Select validator for block production
    pub fn select_validator(&self, block_height: u64) -> Option<crate::crypto::Hash> {
        if self.stakes.is_empty() {
            return None;
        }

        // Simple round-robin selection based on stake
        let mut cumulative_stake = 0u64;
        let selection_value = block_height % self.total_stake;

        for (validator, stake) in &self.stakes {
            cumulative_stake += stake;
            if selection_value < cumulative_stake {
                return Some(*validator);
            }
        }

        None
    }

    /// Apply slashing penalty
    pub fn apply_slashing(&mut self, penalty: SlashingPenalty) -> Result<()> {
        let current_stake = self.get_stake(&penalty.validator);
        if current_stake < penalty.penalty {
            return Err(UnicoinError::Consensus("Penalty exceeds stake".to_string()));
        }

        self.remove_stake(&penalty.validator, penalty.penalty)?;
        Ok(())
    }

    /// Check if validator meets minimum stake requirement
    pub fn meets_minimum_stake(&self, validator: &crate::crypto::Hash, minimum: u64) -> bool {
        self.get_stake(validator) >= minimum
    }

    /// Get validator set
    pub fn get_validators(&self) -> Vec<crate::crypto::Hash> {
        self.stakes.keys().copied().collect()
    }

    /// Get stake distribution
    pub fn get_stake_distribution(&self) -> HashMap<crate::crypto::Hash, f64> {
        let mut distribution = HashMap::new();
        
        for (validator, stake) in &self.stakes {
            let percentage = (*stake as f64 / self.total_stake as f64) * 100.0;
            distribution.insert(*validator, percentage);
        }
        
        distribution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_of_stake_creation() {
        let pos = ProofOfStake::new();
        assert_eq!(pos.get_total_stake(), 0);
        assert!(pos.get_validators().is_empty());
    }

    #[test]
    fn test_stake_management() {
        let mut pos = ProofOfStake::new();
        let validator = crate::crypto::Hash::random();
        let stake_amount = 1000000;

        pos.add_stake(validator, stake_amount).unwrap();
        assert_eq!(pos.get_stake(&validator), stake_amount);
        assert_eq!(pos.get_total_stake(), stake_amount);

        pos.remove_stake(&validator, stake_amount / 2).unwrap();
        assert_eq!(pos.get_stake(&validator), stake_amount / 2);
        assert_eq!(pos.get_total_stake(), stake_amount / 2);
    }

    #[test]
    fn test_validator_selection() {
        let mut pos = ProofOfStake::new();
        let validator1 = crate::crypto::Hash::random();
        let validator2 = crate::crypto::Hash::random();

        pos.add_stake(validator1, 1000).unwrap();
        pos.add_stake(validator2, 2000).unwrap();

        // Should select validator based on stake weight
        let selected = pos.select_validator(0);
        assert!(selected.is_some());
    }

    #[test]
    fn test_slashing() {
        let mut pos = ProofOfStake::new();
        let validator = crate::crypto::Hash::random();
        let stake_amount = 1000000;

        pos.add_stake(validator, stake_amount).unwrap();

        let penalty = SlashingPenalty {
            validator,
            penalty: 100000,
            reason: SlashingCondition::DoubleSign,
            timestamp: crate::utils::timestamp(),
        };

        pos.apply_slashing(penalty).unwrap();
        assert_eq!(pos.get_stake(&validator), stake_amount - 100000);
    }
}
