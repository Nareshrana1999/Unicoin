//! Governance implementation
//!
//! This module provides governance functionality for Unicoin including
//! on-chain voting, proposal management, and DAO operations.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID
    pub id: String,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposal creator
    pub creator: crate::crypto::PublicKey,
    /// Creation timestamp
    pub created_at: u64,
    /// Voting start timestamp
    pub voting_start: u64,
    /// Voting end timestamp
    pub voting_end: u64,
    /// Proposal status
    pub status: ProposalStatus,
    /// Votes
    pub votes: HashMap<String, Vote>,
}

/// Proposal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    /// Proposal is pending
    Pending,
    /// Voting is active
    Voting,
    /// Voting has ended
    Ended,
    /// Proposal was executed
    Executed,
    /// Proposal was rejected
    Rejected,
}

/// Vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voter's public key
    pub voter: crate::crypto::PublicKey,
    /// Vote choice
    pub choice: VoteChoice,
    /// Vote weight (based on stake)
    pub weight: u64,
    /// Vote timestamp
    pub timestamp: u64,
    /// Vote signature
    pub signature: crate::crypto::Signature,
}

/// Vote choice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteChoice {
    /// Vote for the proposal
    For,
    /// Vote against the proposal
    Against,
    /// Abstain from voting
    Abstain,
}

impl Proposal {
    /// Create a new proposal
    pub fn new(
        title: String,
        description: String,
        creator: crate::crypto::PublicKey,
        voting_duration: u64,
    ) -> Self {
        let now = crate::utils::timestamp();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            creator,
            created_at: now,
            voting_start: now,
            voting_end: now + voting_duration,
            status: ProposalStatus::Voting,
            votes: HashMap::new(),
        }
    }

    /// Add a vote to the proposal
    pub fn add_vote(&mut self, vote: Vote) -> Result<()> {
        if self.status != ProposalStatus::Voting {
            return Err(UnicoinError::Governance("Voting is not active".to_string()));
        }

        if crate::utils::timestamp() > self.voting_end {
            return Err(UnicoinError::Governance("Voting period has ended".to_string()));
        }

        let voter_key = vote.voter.to_hash().to_hex();
        self.votes.insert(voter_key, vote);
        Ok(())
    }

    /// Get vote results
    pub fn get_results(&self) -> VoteResults {
        let mut for_votes = 0u64;
        let mut against_votes = 0u64;
        let mut abstain_votes = 0u64;

        for vote in self.votes.values() {
            match vote.choice {
                VoteChoice::For => for_votes += vote.weight,
                VoteChoice::Against => against_votes += vote.weight,
                VoteChoice::Abstain => abstain_votes += vote.weight,
            }
        }

        VoteResults {
            for_votes,
            against_votes,
            abstain_votes,
            total_votes: for_votes + against_votes + abstain_votes,
        }
    }

    /// Check if proposal is passed
    pub fn is_passed(&self, threshold: f64) -> bool {
        let results = self.get_results();
        if results.total_votes == 0 {
            return false;
        }

        let for_percentage = results.for_votes as f64 / results.total_votes as f64;
        for_percentage >= threshold
    }
}

/// Vote results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResults {
    /// Votes for
    pub for_votes: u64,
    /// Votes against
    pub against_votes: u64,
    /// Abstain votes
    pub abstain_votes: u64,
    /// Total votes
    pub total_votes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_creation() {
        let creator = crate::crypto::PublicKey::random();
        let proposal = Proposal::new(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            creator,
            86400, // 1 day
        );

        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.status, ProposalStatus::Voting);
        assert!(!proposal.id.is_empty());
    }

    #[test]
    fn test_vote_results() {
        let creator = crate::crypto::PublicKey::random();
        let mut proposal = Proposal::new(
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            creator,
            86400,
        );

        let results = proposal.get_results();
        assert_eq!(results.total_votes, 0);
        assert_eq!(results.for_votes, 0);
    }
}
