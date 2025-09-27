//! AI risk management for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalyzer {
    pub name: String,
    pub risk_type: RiskType,
}

impl RiskAnalyzer {
    pub fn new(name: String, risk_type: RiskType) -> Self {
        Self { name, risk_type }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub overall_score: u8,
    pub market_risk: u8,
    pub liquidity_risk: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactors {
    pub factors: Vec<String>,
    pub severity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMitigation {
    pub strategy: String,
    pub effectiveness: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskType {
    Market,
    Liquidity,
    Operational,
}
