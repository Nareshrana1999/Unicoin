//! Flash loans implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoan {
    pub id: Hash,
    pub borrower: PublicKey,
    pub amount: u64,
    pub asset: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoanProvider {
    pub name: String,
    pub available_liquidity: u64,
}

impl FlashLoanProvider {
    pub fn new(name: String) -> Self {
        Self {
            name,
            available_liquidity: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoanFee {
    pub fee_percentage: u16,
    pub minimum_fee: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashLoanValidator {
    pub validator: PublicKey,
    pub reputation: u8,
}
