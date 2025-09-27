//! Liquidity pools implementation for Unicoin DeFi

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityProvider {
    pub provider: PublicKey,
    pub liquidity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolToken {
    pub id: Hash,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub volume_24h: u64,
    pub fees_24h: u64,
    pub apy: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpermanentLoss {
    pub loss_percentage: f64,
    pub timestamp: u64,
}
