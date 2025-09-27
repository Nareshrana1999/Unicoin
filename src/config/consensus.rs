//! Consensus configuration for Unicoin
//!
//! This module defines consensus-related configuration options including
//! validator settings, slashing parameters, and consensus algorithm settings.

use serde::{Deserialize, Serialize};
use crate::{consensus::ValidatorSpecialization, Result};

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Consensus algorithm to use
    pub algorithm: ConsensusAlgorithm,
    /// Minimum number of validators
    pub min_validators: u32,
    /// Maximum number of validators
    pub max_validators: u32,
    /// Minimum stake required to become a validator
    pub min_stake: u64,
    /// Maximum stake per validator
    pub max_stake: u64,
    /// Enable slashing
    pub enable_slashing: bool,
    /// Slashing percentage for double signing
    pub double_sign_slashing: f64,
    /// Slashing percentage for downtime
    pub downtime_slashing: f64,
    /// Unbonding period (blocks)
    pub unbonding_period: u64,
    /// Block finality threshold
    pub finality_threshold: u32,
    /// Enable geographic distribution
    pub enable_geographic_distribution: bool,
    /// Minimum validators per region
    pub min_validators_per_region: u32,
    /// Enable validator specialization
    pub enable_specialization: bool,
    /// Consensus timeout (seconds)
    pub consensus_timeout: u64,
    /// Proposal timeout (seconds)
    pub proposal_timeout: u64,
    /// Vote timeout (seconds)
    pub vote_timeout: u64,
    /// Enable fast finality
    pub enable_fast_finality: bool,
    /// Fast finality threshold (votes)
    pub fast_finality_threshold: u32,
    /// Enable BFT (Byzantine Fault Tolerance)
    pub enable_bft: bool,
    /// BFT threshold (percentage)
    pub bft_threshold: f64,
}

/// Consensus algorithm types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsensusAlgorithm {
    /// Unicoin's proprietary consensus algorithm
    UniConsensus,
    /// Standard Proof of Stake
    ProofOfStake,
    /// Delegated Proof of Stake
    DelegatedProofOfStake,
    /// Practical Byzantine Fault Tolerance
    PBFT,
    /// Tendermint consensus
    Tendermint,
}

impl Default for ConsensusAlgorithm {
    fn default() -> Self {
        Self::UniConsensus
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            algorithm: ConsensusAlgorithm::default(),
            min_validators: 21,
            max_validators: 100,
            min_stake: 1_000_000 * 100_000_000, // 1M Unicoin
            max_stake: 100_000_000 * 100_000_000, // 100M Unicoin
            enable_slashing: true,
            double_sign_slashing: 0.05, // 5%
            downtime_slashing: 0.01, // 1%
            unbonding_period: 7 * 24 * 3600 / 10, // 7 days in blocks (10s block time)
            finality_threshold: 2, // 2/3 + 1
            enable_geographic_distribution: true,
            min_validators_per_region: 3,
            enable_specialization: true,
            consensus_timeout: 30,
            proposal_timeout: 10,
            vote_timeout: 15,
            enable_fast_finality: true,
            fast_finality_threshold: 15, // 15 out of 21 validators
            enable_bft: true,
            bft_threshold: 0.67, // 67%
        }
    }
}

impl ConsensusConfig {
    /// Create a new consensus configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the consensus configuration
    pub fn validate(&self) -> Result<()> {
        if self.min_validators == 0 {
            return Err(crate::UnicoinError::InvalidConfig("min_validators must be greater than 0".to_string()));
        }

        if self.max_validators == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_validators must be greater than 0".to_string()));
        }

        if self.min_validators > self.max_validators {
            return Err(crate::UnicoinError::InvalidConfig("min_validators must be less than or equal to max_validators".to_string()));
        }

        if self.min_stake == 0 {
            return Err(crate::UnicoinError::InvalidConfig("min_stake must be greater than 0".to_string()));
        }

        if self.max_stake == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_stake must be greater than 0".to_string()));
        }

        if self.min_stake > self.max_stake {
            return Err(crate::UnicoinError::InvalidConfig("min_stake must be less than or equal to max_stake".to_string()));
        }

        if self.double_sign_slashing < 0.0 || self.double_sign_slashing > 1.0 {
            return Err(crate::UnicoinError::InvalidConfig("double_sign_slashing must be between 0.0 and 1.0".to_string()));
        }

        if self.downtime_slashing < 0.0 || self.downtime_slashing > 1.0 {
            return Err(crate::UnicoinError::InvalidConfig("downtime_slashing must be between 0.0 and 1.0".to_string()));
        }

        if self.unbonding_period == 0 {
            return Err(crate::UnicoinError::InvalidConfig("unbonding_period must be greater than 0".to_string()));
        }

        if self.finality_threshold == 0 {
            return Err(crate::UnicoinError::InvalidConfig("finality_threshold must be greater than 0".to_string()));
        }

        if self.consensus_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("consensus_timeout must be greater than 0".to_string()));
        }

        if self.proposal_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("proposal_timeout must be greater than 0".to_string()));
        }

        if self.vote_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("vote_timeout must be greater than 0".to_string()));
        }

        if self.bft_threshold < 0.0 || self.bft_threshold > 1.0 {
            return Err(crate::UnicoinError::InvalidConfig("bft_threshold must be between 0.0 and 1.0".to_string()));
        }

        Ok(())
    }

    /// Check if slashing is enabled
    pub fn slashing_enabled(&self) -> bool {
        self.enable_slashing
    }

    /// Check if geographic distribution is enabled
    pub fn geographic_distribution_enabled(&self) -> bool {
        self.enable_geographic_distribution
    }

    /// Check if specialization is enabled
    pub fn specialization_enabled(&self) -> bool {
        self.enable_specialization
    }

    /// Check if fast finality is enabled
    pub fn fast_finality_enabled(&self) -> bool {
        self.enable_fast_finality
    }

    /// Check if BFT is enabled
    pub fn bft_enabled(&self) -> bool {
        self.enable_bft
    }

    /// Get the minimum number of validators required for consensus
    pub fn min_consensus_validators(&self) -> u32 {
        if self.bft_enabled() {
            ((self.min_validators as f64) * self.bft_threshold).ceil() as u32
        } else {
            self.min_validators / 2 + 1
        }
    }

    /// Calculate the slashing amount for double signing
    pub fn calculate_double_sign_slashing(&self, stake: u64) -> u64 {
        (stake as f64 * self.double_sign_slashing) as u64
    }

    /// Calculate the slashing amount for downtime
    pub fn calculate_downtime_slashing(&self, stake: u64) -> u64 {
        (stake as f64 * self.downtime_slashing) as u64
    }

    /// Check if a stake amount is valid
    pub fn is_valid_stake(&self, stake: u64) -> bool {
        stake >= self.min_stake && stake <= self.max_stake
    }

    /// Check if the number of validators is within limits
    pub fn is_valid_validator_count(&self, count: u32) -> bool {
        count >= self.min_validators && count <= self.max_validators
    }

    /// Get the consensus algorithm
    pub fn algorithm(&self) -> &ConsensusAlgorithm {
        &self.algorithm
    }

    /// Set the consensus algorithm
    pub fn set_algorithm(&mut self, algorithm: ConsensusAlgorithm) {
        self.algorithm = algorithm;
    }
}
