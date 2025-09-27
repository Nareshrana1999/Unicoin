//! Validator implementation
//!
//! This module provides validator management for the consensus mechanism.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    /// Validator's public key
    pub public_key: crate::crypto::PublicKey,
    /// Stake amount
    pub stake: u64,
    /// Commission rate (percentage)
    pub commission_rate: u8,
    /// Delegation address
    pub delegation_address: crate::crypto::Hash,
    /// Registration timestamp
    pub registered_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Validator status
    pub status: ValidatorStatus,
}

/// Validator status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatorStatus {
    /// Active validator
    Active,
    /// Inactive validator
    Inactive,
    /// Jailed validator
    Jailed,
    /// Slashed validator
    Slashed,
}

/// Validator set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    /// Map of validator addresses to validator info
    validators: HashMap<crate::crypto::Hash, Validator>,
    /// Maximum number of validators
    max_validators: usize,
}

impl Validator {
    /// Create a new validator
    pub fn new(
        public_key: crate::crypto::PublicKey,
        stake: u64,
        commission_rate: u8,
    ) -> Self {
        Self {
            delegation_address: public_key.to_hash(),
            registered_at: crate::utils::timestamp(),
            last_activity: crate::utils::timestamp(),
            status: ValidatorStatus::Active,
            public_key,
            stake,
            commission_rate,
        }
    }

    /// Update validator activity
    pub fn update_activity(&mut self) {
        self.last_activity = crate::utils::timestamp();
    }

    /// Check if validator is active
    pub fn is_active(&self) -> bool {
        self.status == ValidatorStatus::Active
    }

    /// Jail the validator
    pub fn jail(&mut self) {
        self.status = ValidatorStatus::Jailed;
    }

    /// Unjail the validator
    pub fn unjail(&mut self) {
        if self.status == ValidatorStatus::Jailed {
            self.status = ValidatorStatus::Active;
        }
    }

    /// Slash the validator
    pub fn slash(&mut self) {
        self.status = ValidatorStatus::Slashed;
    }

    /// Get validator address
    pub fn get_address(&self) -> crate::crypto::Hash {
        self.delegation_address
    }
}

impl ValidatorSet {
    /// Create a new validator set
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            max_validators: 1000,
        }
    }

    /// Create a new validator set with maximum limit
    pub fn with_max_validators(max_validators: usize) -> Self {
        Self {
            validators: HashMap::new(),
            max_validators,
        }
    }

    /// Add a validator to the set
    pub fn add_validator(&mut self, validator: Validator) -> Result<()> {
        if self.validators.len() >= self.max_validators {
            return Err(UnicoinError::Consensus("Maximum validators reached".to_string()));
        }

        let address = validator.get_address();
        self.validators.insert(address, validator);
        Ok(())
    }

    /// Remove a validator from the set
    pub fn remove_validator(&mut self, address: &crate::crypto::Hash) -> Result<()> {
        self.validators.remove(address)
            .ok_or_else(|| UnicoinError::Consensus("Validator not found".to_string()))?;
        Ok(())
    }

    /// Get a validator by address
    pub fn get_validator(&self, address: &crate::crypto::Hash) -> Option<&Validator> {
        self.validators.get(address)
    }

    /// Get a mutable validator by address
    pub fn get_validator_mut(&mut self, address: &crate::crypto::Hash) -> Option<&mut Validator> {
        self.validators.get_mut(address)
    }

    /// Get all validators
    pub fn get_all_validators(&self) -> Vec<&Validator> {
        self.validators.values().collect()
    }

    /// Get active validators only
    pub fn get_active_validators(&self) -> Vec<&Validator> {
        self.validators.values()
            .filter(|v| v.is_active())
            .collect()
    }

    /// Get validator count
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Check if validator set is empty
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Clear all validators
    pub fn clear(&mut self) {
        self.validators.clear();
    }

    /// Get total stake
    pub fn get_total_stake(&self) -> u64 {
        self.validators.values().map(|v| v.stake).sum()
    }

    /// Get stake distribution
    pub fn get_stake_distribution(&self) -> HashMap<crate::crypto::Hash, u64> {
        self.validators.iter()
            .map(|(address, validator)| (*address, validator.stake))
            .collect()
    }

    /// Update validator stake
    pub fn update_stake(&mut self, address: &crate::crypto::Hash, new_stake: u64) -> Result<()> {
        if let Some(validator) = self.validators.get_mut(address) {
            validator.stake = new_stake;
            validator.update_activity();
            Ok(())
        } else {
            Err(UnicoinError::Consensus("Validator not found".to_string()))
        }
    }

    /// Select validator for block production
    pub fn select_block_producer(&self, block_height: u64) -> Option<crate::crypto::Hash> {
        let active_validators: Vec<_> = self.get_active_validators();
        if active_validators.is_empty() {
            return None;
        }

        // Simple round-robin selection
        let index = (block_height as usize) % active_validators.len();
        Some(active_validators[index].get_address())
    }

    /// Get top validators by stake
    pub fn get_top_validators(&self, count: usize) -> Vec<&Validator> {
        let mut validators: Vec<_> = self.validators.values().collect();
        validators.sort_by(|a, b| b.stake.cmp(&a.stake));
        validators.into_iter().take(count).collect()
    }

    /// Check if address is a validator
    pub fn is_validator(&self, address: &crate::crypto::Hash) -> bool {
        self.validators.contains_key(address)
    }
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let public_key = crate::crypto::PublicKey::random();
        let stake = 1000000;
        let commission_rate = 5;

        let validator = Validator::new(public_key, stake, commission_rate);

        assert_eq!(validator.stake, stake);
        assert_eq!(validator.commission_rate, commission_rate);
        assert!(validator.is_active());
        assert_eq!(validator.status, ValidatorStatus::Active);
    }

    #[test]
    fn test_validator_set_management() {
        let mut validator_set = ValidatorSet::new();
        let public_key = crate::crypto::PublicKey::random();
        let validator = Validator::new(public_key, 1000000, 5);

        let address = validator.get_address();
        validator_set.add_validator(validator).unwrap();

        assert_eq!(validator_set.len(), 1);
        assert!(validator_set.is_validator(&address));
        assert!(!validator_set.is_empty());

        let retrieved_validator = validator_set.get_validator(&address).unwrap();
        assert_eq!(retrieved_validator.stake, 1000000);

        validator_set.remove_validator(&address).unwrap();
        assert_eq!(validator_set.len(), 0);
        assert!(validator_set.is_empty());
    }

    #[test]
    fn test_validator_status() {
        let public_key = crate::crypto::PublicKey::random();
        let mut validator = Validator::new(public_key, 1000000, 5);

        assert!(validator.is_active());
        assert_eq!(validator.status, ValidatorStatus::Active);

        validator.jail();
        assert!(!validator.is_active());
        assert_eq!(validator.status, ValidatorStatus::Jailed);

        validator.unjail();
        assert!(validator.is_active());
        assert_eq!(validator.status, ValidatorStatus::Active);

        validator.slash();
        assert!(!validator.is_active());
        assert_eq!(validator.status, ValidatorStatus::Slashed);
    }

    #[test]
    fn test_validator_selection() {
        let mut validator_set = ValidatorSet::new();
        
        // Add multiple validators
        for i in 0..5 {
            let public_key = crate::crypto::PublicKey::random();
            let validator = Validator::new(public_key, 1000000 + i as u64 * 100000, 5);
            validator_set.add_validator(validator).unwrap();
        }

        let active_validators = validator_set.get_active_validators();
        assert_eq!(active_validators.len(), 5);

        let selected = validator_set.select_block_producer(0);
        assert!(selected.is_some());
    }
}
