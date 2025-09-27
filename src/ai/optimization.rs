//! AI optimization engines for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioOptimizer {
    pub name: String,
    pub optimization_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasOptimizer {
    pub name: String,
    pub gas_savings: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeOptimizer {
    pub name: String,
    pub fee_reduction: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub improvement: f64,
    pub time_taken: u64,
    pub iterations: u32,
}
