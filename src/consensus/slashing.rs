//! Slashing implementation
//!
//! This module provides slashing mechanisms for validator accountability.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Slashing condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlashingCondition {
    /// Double signing (signing two different blocks at the same height)
    DoubleSign,
    /// Validator downtime (not participating in consensus)
    Downtime,
    /// Invalid block production
    InvalidBlock,
    /// Unresponsive validator
    Unresponsive,
}

/// Slashing penalty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingPenalty {
    /// Validator address
    pub validator: crate::crypto::Hash,
    /// Penalty amount in satoshis
    pub penalty_amount: u64,
    /// Reason for slashing
    pub condition: SlashingCondition,
    /// Evidence data
    pub evidence: Vec<u8>,
    /// Timestamp of the slashing
    pub timestamp: u64,
    /// Block height when slashing occurred
    pub block_height: u64,
}

/// Slashing manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingManager {
    /// Slashing conditions and their penalties
    slashing_conditions: HashMap<SlashingCondition, u64>,
    /// Slashing history
    slashing_history: Vec<SlashingPenalty>,
    /// Validator slash counts
    validator_slash_count: HashMap<crate::crypto::Hash, u32>,
}

impl SlashingPenalty {
    /// Create a new slashing penalty
    pub fn new(
        validator: crate::crypto::Hash,
        penalty_amount: u64,
        condition: SlashingCondition,
        evidence: Vec<u8>,
    ) -> Self {
        Self {
            validator,
            penalty_amount,
            condition,
            evidence,
            timestamp: crate::utils::timestamp(),
            block_height: 0, // Will be set when penalty is applied
        }
    }

    /// Calculate the penalty amount based on condition and validator stake
    pub fn calculate_penalty(
        condition: SlashingCondition,
        validator_stake: u64,
        slash_count: u32,
    ) -> u64 {
        let base_penalty = match condition {
            SlashingCondition::DoubleSign => validator_stake / 2, // 50% penalty
            SlashingCondition::Downtime => validator_stake / 20,  // 5% penalty
            SlashingCondition::InvalidBlock => validator_stake / 10, // 10% penalty
            SlashingCondition::Unresponsive => validator_stake / 40, // 2.5% penalty
        };

        // Increase penalty for repeat offenders
        let multiplier = 1 + slash_count;
        base_penalty * multiplier as u64
    }

    /// Verify the slashing penalty is valid
    pub fn verify(&self, validator_stake: u64) -> Result<bool> {
        // Check if penalty doesn't exceed stake
        if self.penalty_amount > validator_stake {
            return Ok(false);
        }

        // Check if timestamp is reasonable
        let current_time = crate::utils::timestamp();
        if self.timestamp > current_time || current_time - self.timestamp > 86400 {
            return Ok(false);
        }

        // Check if evidence is not empty for certain conditions
        match self.condition {
            SlashingCondition::DoubleSign | SlashingCondition::InvalidBlock => {
                if self.evidence.is_empty() {
                    return Ok(false);
                }
            }
            _ => {}
        }

        Ok(true)
    }
}

impl SlashingManager {
    /// Create a new slashing manager
    pub fn new() -> Self {
        let mut slashing_conditions = HashMap::new();
        slashing_conditions.insert(SlashingCondition::DoubleSign, 50); // 50% penalty
        slashing_conditions.insert(SlashingCondition::Downtime, 5);    // 5% penalty
        slashing_conditions.insert(SlashingCondition::InvalidBlock, 10); // 10% penalty
        slashing_conditions.insert(SlashingCondition::Unresponsive, 2); // 2.5% penalty

        Self {
            slashing_conditions,
            slashing_history: Vec::new(),
            validator_slash_count: HashMap::new(),
        }
    }

    /// Apply a slashing penalty
    pub fn apply_slashing(
        &mut self,
        penalty: SlashingPenalty,
        current_block_height: u64,
    ) -> Result<()> {
        // Update block height
        let mut penalty = penalty;
        penalty.block_height = current_block_height;

        // Verify the penalty
        let validator_stake = 1000000; // This would come from validator set
        if !penalty.verify(validator_stake)? {
            return Err(UnicoinError::Consensus("Invalid slashing penalty".to_string()));
        }

        // Record the slashing
        self.slashing_history.push(penalty.clone());

        // Update slash count for validator
        let count = self.validator_slash_count.get(&penalty.validator).copied().unwrap_or(0);
        self.validator_slash_count.insert(penalty.validator, count + 1);

        Ok(())
    }

    /// Get slashing history for a validator
    pub fn get_validator_slashing_history(&self, validator: &crate::crypto::Hash) -> Vec<&SlashingPenalty> {
        self.slashing_history
            .iter()
            .filter(|penalty| penalty.validator == *validator)
            .collect()
    }

    /// Get total slashed amount for a validator
    pub fn get_validator_total_slashed(&self, validator: &crate::crypto::Hash) -> u64 {
        self.slashing_history
            .iter()
            .filter(|penalty| penalty.validator == *validator)
            .map(|penalty| penalty.penalty_amount)
            .sum()
    }

    /// Get validator slash count
    pub fn get_validator_slash_count(&self, validator: &crate::crypto::Hash) -> u32 {
        self.validator_slash_count.get(validator).copied().unwrap_or(0)
    }

    /// Check if validator should be jailed
    pub fn should_jail_validator(&self, validator: &crate::crypto::Hash) -> bool {
        let slash_count = self.get_validator_slash_count(validator);
        slash_count >= 3 // Jail after 3 slashing events
    }

    /// Get all slashing events
    pub fn get_all_slashing_events(&self) -> &Vec<SlashingPenalty> {
        &self.slashing_history
    }

    /// Get slashing statistics
    pub fn get_slashing_statistics(&self) -> SlashingStatistics {
        let total_slashed: u64 = self.slashing_history.iter().map(|p| p.penalty_amount).sum();
        let total_events = self.slashing_history.len();
        let unique_validators = self.validator_slash_count.len();

        SlashingStatistics {
            total_slashed,
            total_events,
            unique_validators,
            slashing_by_condition: self.get_slashing_by_condition(),
        }
    }

    /// Get slashing events grouped by condition
    fn get_slashing_by_condition(&self) -> HashMap<SlashingCondition, u32> {
        let mut counts = HashMap::new();
        
        for penalty in &self.slashing_history {
            let count = counts.get(&penalty.condition).copied().unwrap_or(0);
            counts.insert(penalty.condition, count + 1);
        }
        
        counts
    }

    /// Clear slashing history (for testing)
    #[cfg(test)]
    pub fn clear_history(&mut self) {
        self.slashing_history.clear();
        self.validator_slash_count.clear();
    }
}

/// Slashing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingStatistics {
    /// Total amount slashed across all validators
    pub total_slashed: u64,
    /// Total number of slashing events
    pub total_events: usize,
    /// Number of unique validators that have been slashed
    pub unique_validators: usize,
    /// Slashing events grouped by condition
    pub slashing_by_condition: HashMap<SlashingCondition, u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slashing_penalty_creation() {
        let validator = crate::crypto::Hash::random();
        let penalty = SlashingPenalty::new(
            validator,
            100000,
            SlashingCondition::DoubleSign,
            vec![1, 2, 3, 4],
        );

        assert_eq!(penalty.validator, validator);
        assert_eq!(penalty.penalty_amount, 100000);
        assert_eq!(penalty.condition, SlashingCondition::DoubleSign);
        assert!(!penalty.evidence.is_empty());
    }

    #[test]
    fn test_penalty_calculation() {
        let validator_stake = 1000000;
        let penalty_amount = SlashingPenalty::calculate_penalty(
            SlashingCondition::DoubleSign,
            validator_stake,
            0,
        );

        assert_eq!(penalty_amount, 500000); // 50% of stake

        let penalty_amount_downtime = SlashingPenalty::calculate_penalty(
            SlashingCondition::Downtime,
            validator_stake,
            0,
        );

        assert_eq!(penalty_amount_downtime, 50000); // 5% of stake
    }

    #[test]
    fn test_penalty_verification() {
        let validator = crate::crypto::Hash::random();
        let penalty = SlashingPenalty::new(
            validator,
            100000,
            SlashingCondition::DoubleSign,
            vec![1, 2, 3, 4],
        );

        assert!(penalty.verify(1000000).unwrap()); // Valid penalty
        assert!(!penalty.verify(50000).unwrap());  // Penalty exceeds stake
    }

    #[test]
    fn test_slashing_manager() {
        let mut manager = SlashingManager::new();
        let validator = crate::crypto::Hash::random();

        let penalty = SlashingPenalty::new(
            validator,
            100000,
            SlashingCondition::DoubleSign,
            vec![1, 2, 3, 4],
        );

        manager.apply_slashing(penalty, 100).unwrap();

        assert_eq!(manager.get_validator_slash_count(&validator), 1);
        assert_eq!(manager.get_validator_total_slashed(&validator), 100000);
        assert!(!manager.should_jail_validator(&validator)); // Not enough slashes yet
    }

    #[test]
    fn test_jail_condition() {
        let mut manager = SlashingManager::new();
        let validator = crate::crypto::Hash::random();

        // Apply 3 slashing penalties
        for _ in 0..3 {
            let penalty = SlashingPenalty::new(
                validator,
                100000,
                SlashingCondition::DoubleSign,
                vec![1, 2, 3, 4],
            );
            manager.apply_slashing(penalty, 100).unwrap();
        }

        assert_eq!(manager.get_validator_slash_count(&validator), 3);
        assert!(manager.should_jail_validator(&validator)); // Should be jailed
    }
}
