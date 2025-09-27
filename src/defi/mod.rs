//! Decentralized Finance (DeFi) implementation for Unicoin
//! 
//! This module provides comprehensive DeFi functionality including
//! decentralized exchanges, lending protocols, yield farming, and more.

pub mod dex;
pub mod lending;
pub mod yield_farming;
pub mod liquidity_pools;
pub mod staking_rewards;
pub mod governance_tokens;
pub mod flash_loans;
pub mod derivatives;

pub use dex::{DEX, LiquidityPool, SwapTransaction, SwapResult, TradingPair, OrderBook};
pub use lending::{LendingProtocol, LendingPool, BorrowRequest, LendingPosition, InterestRate};
pub use yield_farming::{YieldFarm, FarmReward, FarmingPosition, FarmStrategy};
pub use liquidity_pools::{LiquidityProvider, PoolToken, PoolMetrics, ImpermanentLoss};
pub use staking_rewards::{StakingReward, RewardCalculator, StakingPosition, RewardDistribution};
pub use governance_tokens::{GovernanceToken, VotingPower, Proposal, Vote, Delegation};
pub use flash_loans::{FlashLoan, FlashLoanProvider, FlashLoanFee, FlashLoanValidator};
pub use derivatives::{Derivative, OptionsContract, FuturesContract, PerpetualSwap, MarginTrading};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::crypto::{hash::Hash, keys::PublicKey};

/// DeFi Protocol Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiManager {
    /// Active DEX protocols
    pub dex_protocols: HashMap<String, DEX>,
    /// Active lending protocols
    pub lending_protocols: HashMap<String, LendingProtocol>,
    /// Active yield farms
    pub yield_farms: HashMap<String, YieldFarm>,
    /// Liquidity pools
    pub liquidity_pools: HashMap<Hash, LiquidityPool>,
    /// Governance tokens
    pub governance_tokens: HashMap<Hash, GovernanceToken>,
    /// Flash loan providers
    pub flash_loan_providers: HashMap<String, FlashLoanProvider>,
    /// Derivatives markets
    pub derivatives_markets: HashMap<String, Derivative>,
    /// Protocol fees
    pub protocol_fees: ProtocolFees,
    /// DeFi metrics
    pub metrics: DeFiMetrics,
}

/// Protocol fees structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolFees {
    /// Trading fees (basis points)
    pub trading_fees: u16,
    /// Lending fees (basis points)
    pub lending_fees: u16,
    /// Yield farming fees (basis points)
    pub farming_fees: u16,
    /// Flash loan fees (basis points)
    pub flash_loan_fees: u16,
    /// Governance fees (basis points)
    pub governance_fees: u16,
    /// Protocol treasury
    pub treasury_balance: u64,
}

/// DeFi metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiMetrics {
    /// Total value locked (TVL)
    pub total_value_locked: u64,
    /// Total trading volume (24h)
    pub trading_volume_24h: u64,
    /// Number of active users
    pub active_users: u64,
    /// Number of transactions (24h)
    pub transactions_24h: u64,
    /// Average transaction fee
    pub average_fee: u64,
    /// Protocol revenue (24h)
    pub revenue_24h: u64,
    /// Liquidity utilization rate
    pub liquidity_utilization: f64,
    /// Lending utilization rate
    pub lending_utilization: f64,
}

impl DeFiManager {
    pub fn new() -> Self {
        Self {
            dex_protocols: HashMap::new(),
            lending_protocols: HashMap::new(),
            yield_farms: HashMap::new(),
            liquidity_pools: HashMap::new(),
            governance_tokens: HashMap::new(),
            flash_loan_providers: HashMap::new(),
            derivatives_markets: HashMap::new(),
            protocol_fees: ProtocolFees::default(),
            metrics: DeFiMetrics::new(),
        }
    }

    /// Initialize default DeFi protocols
    pub fn initialize_default_protocols(&mut self) {
        // Initialize Uniswap-like DEX
        let dex = DEX::new("UniSwap".to_string(), 30); // 0.3% fee
        self.dex_protocols.insert("uniswap".to_string(), dex);

        // Initialize Compound-like lending
        let lending = LendingProtocol::new("UniLend".to_string());
        self.lending_protocols.insert("unilend".to_string(), lending);

        // Initialize yield farming
        let farm = YieldFarm::new("UniFarm".to_string());
        self.yield_farms.insert("unifarm".to_string(), farm);

        // Initialize flash loan provider
        let flash_provider = FlashLoanProvider::new("UniFlash".to_string());
        self.flash_loan_providers.insert("uniflash".to_string(), flash_provider);
    }

    /// Get total value locked across all protocols
    pub fn get_total_value_locked(&self) -> u64 {
        let mut tvl = 0u64;
        
        // Add DEX TVL
        for dex in self.dex_protocols.values() {
            tvl += dex.get_total_liquidity();
        }
        
        // Add lending TVL
        for lending in self.lending_protocols.values() {
            tvl += lending.get_total_supplied();
        }
        
        // Add yield farming TVL
        for farm in self.yield_farms.values() {
            tvl += farm.get_total_staked();
        }
        
        tvl
    }

    /// Update DeFi metrics
    pub fn update_metrics(&mut self) {
        self.metrics.total_value_locked = self.get_total_value_locked();
        self.metrics.liquidity_utilization = self.calculate_liquidity_utilization();
        self.metrics.lending_utilization = self.calculate_lending_utilization();
    }

    /// Calculate liquidity utilization rate
    fn calculate_liquidity_utilization(&self) -> f64 {
        let mut total_liquidity = 0u64;
        let mut used_liquidity = 0u64;
        
        for pool in self.liquidity_pools.values() {
            total_liquidity += pool.get_total_liquidity();
            used_liquidity += pool.get_used_liquidity();
        }
        
        if total_liquidity == 0 {
            0.0
        } else {
            used_liquidity as f64 / total_liquidity as f64
        }
    }

    /// Calculate lending utilization rate
    fn calculate_lending_utilization(&self) -> f64 {
        let mut total_supplied = 0u64;
        let mut total_borrowed = 0u64;
        
        for lending in self.lending_protocols.values() {
            total_supplied += lending.get_total_supplied();
            total_borrowed += lending.get_total_borrowed();
        }
        
        if total_supplied == 0 {
            0.0
        } else {
            total_borrowed as f64 / total_supplied as f64
        }
    }
}

impl Default for DeFiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ProtocolFees {
    fn default() -> Self {
        Self {
            trading_fees: 30,      // 0.3%
            lending_fees: 10,      // 0.1%
            farming_fees: 20,      // 0.2%
            flash_loan_fees: 5,    // 0.05%
            governance_fees: 15,   // 0.15%
            treasury_balance: 0,
        }
    }
}

impl DeFiMetrics {
    pub fn new() -> Self {
        Self {
            total_value_locked: 0,
            trading_volume_24h: 0,
            active_users: 0,
            transactions_24h: 0,
            average_fee: 0,
            revenue_24h: 0,
            liquidity_utilization: 0.0,
            lending_utilization: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defi_manager_creation() {
        let manager = DeFiManager::new();
        assert_eq!(manager.dex_protocols.len(), 0);
        assert_eq!(manager.lending_protocols.len(), 0);
        assert_eq!(manager.yield_farms.len(), 0);
    }

    #[test]
    fn test_default_protocols_initialization() {
        let mut manager = DeFiManager::new();
        manager.initialize_default_protocols();
        
        assert_eq!(manager.dex_protocols.len(), 1);
        assert_eq!(manager.lending_protocols.len(), 1);
        assert_eq!(manager.yield_farms.len(), 1);
        assert_eq!(manager.flash_loan_providers.len(), 1);
    }

    #[test]
    fn test_total_value_locked() {
        let manager = DeFiManager::new();
        let tvl = manager.get_total_value_locked();
        assert_eq!(tvl, 0);
    }

    #[test]
    fn test_metrics_update() {
        let mut manager = DeFiManager::new();
        manager.update_metrics();
        
        assert_eq!(manager.metrics.total_value_locked, 0);
        assert_eq!(manager.metrics.liquidity_utilization, 0.0);
        assert_eq!(manager.metrics.lending_utilization, 0.0);
    }
}
