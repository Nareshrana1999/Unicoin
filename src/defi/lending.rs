//! Lending protocol implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LendingProtocol {
    pub name: String,
    pub total_supplied: u64,
    pub total_borrowed: u64,
}

impl LendingProtocol {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_supplied: 0,
            total_borrowed: 0,
        }
    }

    pub fn get_total_supplied(&self) -> u64 {
        self.total_supplied
    }

    pub fn get_total_borrowed(&self) -> u64 {
        self.total_borrowed
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LendingPool {
    pub id: Hash,
    pub asset: Hash,
    pub total_supply: u64,
    pub total_borrow: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowRequest {
    pub id: Hash,
    pub borrower: PublicKey,
    pub amount: u64,
    pub collateral: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LendingPosition {
    pub id: Hash,
    pub user: PublicKey,
    pub supplied_amount: u64,
    pub borrowed_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRate {
    pub rate: u16,
    pub utilization_rate: u16,
}
