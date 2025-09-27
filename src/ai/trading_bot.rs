//! AI trading bots for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingBot {
    pub name: String,
    pub strategy: TradingStrategy,
}

impl TradingBot {
    pub fn new(name: String, strategy: TradingStrategy) -> Self {
        Self { name, strategy }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradingStrategy {
    Momentum,
    MeanReversion,
    Arbitrage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    pub risk_level: u8,
    pub max_position_size: u64,
    pub stop_loss: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub signal_type: SignalType,
    pub confidence: f64,
    pub price_target: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
}
