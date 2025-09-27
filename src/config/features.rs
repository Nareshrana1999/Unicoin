//! Features configuration for Unicoin
//!
//! This module defines feature flags and configuration options for
//! various Unicoin features including AI, DeFi, NFTs, and privacy.

use serde::{Deserialize, Serialize};
use crate::Result;

/// Features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// Enable AI features
    pub enable_ai: bool,
    /// Enable DeFi features
    pub enable_defi: bool,
    /// Enable NFT features
    pub enable_nft: bool,
    /// Enable privacy features
    pub enable_privacy: bool,
    /// Enable smart contracts
    pub enable_smart_contracts: bool,
    /// Enable cross-chain bridges
    pub enable_bridges: bool,
    /// Enable governance
    pub enable_governance: bool,
    /// Enable staking
    pub enable_staking: bool,
    /// Enable yield farming
    pub enable_yield_farming: bool,
    /// Enable flash loans
    pub enable_flash_loans: bool,
    /// Enable derivatives
    pub enable_derivatives: bool,
    /// Enable lending protocols
    pub enable_lending: bool,
    /// Enable DEX functionality
    pub enable_dex: bool,
    /// Enable liquidity pools
    pub enable_liquidity_pools: bool,
    /// Enable NFT marketplace
    pub enable_nft_marketplace: bool,
    /// Enable NFT auctions
    pub enable_nft_auctions: bool,
    /// Enable NFT royalties
    pub enable_nft_royalties: bool,
    /// Enable zero-knowledge proofs
    pub enable_zk_proofs: bool,
    /// Enable ring signatures
    pub enable_ring_signatures: bool,
    /// Enable stealth addresses
    pub enable_stealth_addresses: bool,
    /// Enable confidential transactions
    pub enable_confidential_transactions: bool,
    /// Enable AI prediction
    pub enable_ai_prediction: bool,
    /// Enable AI optimization
    pub enable_ai_optimization: bool,
    /// Enable AI risk management
    pub enable_ai_risk_management: bool,
    /// Enable AI trading bots
    pub enable_ai_trading_bots: bool,
    /// Enable AI governance
    pub enable_ai_governance: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enable_ai: true,
            enable_defi: true,
            enable_nft: true,
            enable_privacy: true,
            enable_smart_contracts: true,
            enable_bridges: true,
            enable_governance: true,
            enable_staking: true,
            enable_yield_farming: true,
            enable_flash_loans: true,
            enable_derivatives: true,
            enable_lending: true,
            enable_dex: true,
            enable_liquidity_pools: true,
            enable_nft_marketplace: true,
            enable_nft_auctions: true,
            enable_nft_royalties: true,
            enable_zk_proofs: true,
            enable_ring_signatures: true,
            enable_stealth_addresses: true,
            enable_confidential_transactions: true,
            enable_ai_prediction: true,
            enable_ai_optimization: true,
            enable_ai_risk_management: true,
            enable_ai_trading_bots: true,
            enable_ai_governance: true,
        }
    }
}

impl FeaturesConfig {
    /// Create a new features configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the features configuration
    pub fn validate(&self) -> Result<()> {
        // Check AI feature dependencies
        if self.enable_ai_prediction && !self.enable_ai {
            return Err(crate::UnicoinError::InvalidConfig("enable_ai_prediction requires enable_ai".to_string()));
        }

        if self.enable_ai_optimization && !self.enable_ai {
            return Err(crate::UnicoinError::InvalidConfig("enable_ai_optimization requires enable_ai".to_string()));
        }

        if self.enable_ai_risk_management && !self.enable_ai {
            return Err(crate::UnicoinError::InvalidConfig("enable_ai_risk_management requires enable_ai".to_string()));
        }

        if self.enable_ai_trading_bots && !self.enable_ai {
            return Err(crate::UnicoinError::InvalidConfig("enable_ai_trading_bots requires enable_ai".to_string()));
        }

        if self.enable_ai_governance && !self.enable_ai {
            return Err(crate::UnicoinError::InvalidConfig("enable_ai_governance requires enable_ai".to_string()));
        }

        // Check DeFi feature dependencies
        if self.enable_yield_farming && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_yield_farming requires enable_defi".to_string()));
        }

        if self.enable_flash_loans && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_flash_loans requires enable_defi".to_string()));
        }

        if self.enable_derivatives && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_derivatives requires enable_defi".to_string()));
        }

        if self.enable_lending && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_lending requires enable_defi".to_string()));
        }

        if self.enable_dex && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_dex requires enable_defi".to_string()));
        }

        if self.enable_liquidity_pools && !self.enable_defi {
            return Err(crate::UnicoinError::InvalidConfig("enable_liquidity_pools requires enable_defi".to_string()));
        }

        // Check NFT feature dependencies
        if self.enable_nft_marketplace && !self.enable_nft {
            return Err(crate::UnicoinError::InvalidConfig("enable_nft_marketplace requires enable_nft".to_string()));
        }

        if self.enable_nft_auctions && !self.enable_nft {
            return Err(crate::UnicoinError::InvalidConfig("enable_nft_auctions requires enable_nft".to_string()));
        }

        if self.enable_nft_royalties && !self.enable_nft {
            return Err(crate::UnicoinError::InvalidConfig("enable_nft_royalties requires enable_nft".to_string()));
        }

        // Check privacy feature dependencies
        if self.enable_zk_proofs && !self.enable_privacy {
            return Err(crate::UnicoinError::InvalidConfig("enable_zk_proofs requires enable_privacy".to_string()));
        }

        if self.enable_ring_signatures && !self.enable_privacy {
            return Err(crate::UnicoinError::InvalidConfig("enable_ring_signatures requires enable_privacy".to_string()));
        }

        if self.enable_stealth_addresses && !self.enable_privacy {
            return Err(crate::UnicoinError::InvalidConfig("enable_stealth_addresses requires enable_privacy".to_string()));
        }

        if self.enable_confidential_transactions && !self.enable_privacy {
            return Err(crate::UnicoinError::InvalidConfig("enable_confidential_transactions requires enable_privacy".to_string()));
        }

        Ok(())
    }

    /// Check if AI features are enabled
    pub fn ai_enabled(&self) -> bool {
        self.enable_ai
    }

    /// Check if DeFi features are enabled
    pub fn defi_enabled(&self) -> bool {
        self.enable_defi
    }

    /// Check if NFT features are enabled
    pub fn nft_enabled(&self) -> bool {
        self.enable_nft
    }

    /// Check if privacy features are enabled
    pub fn privacy_enabled(&self) -> bool {
        self.enable_privacy
    }

    /// Check if smart contracts are enabled
    pub fn smart_contracts_enabled(&self) -> bool {
        self.enable_smart_contracts
    }

    /// Check if bridges are enabled
    pub fn bridges_enabled(&self) -> bool {
        self.enable_bridges
    }

    /// Check if governance is enabled
    pub fn governance_enabled(&self) -> bool {
        self.enable_governance
    }

    /// Check if staking is enabled
    pub fn staking_enabled(&self) -> bool {
        self.enable_staking
    }

    /// Check if yield farming is enabled
    pub fn yield_farming_enabled(&self) -> bool {
        self.enable_yield_farming && self.enable_defi
    }

    /// Check if flash loans are enabled
    pub fn flash_loans_enabled(&self) -> bool {
        self.enable_flash_loans && self.enable_defi
    }

    /// Check if derivatives are enabled
    pub fn derivatives_enabled(&self) -> bool {
        self.enable_derivatives && self.enable_defi
    }

    /// Check if lending is enabled
    pub fn lending_enabled(&self) -> bool {
        self.enable_lending && self.enable_defi
    }

    /// Check if DEX is enabled
    pub fn dex_enabled(&self) -> bool {
        self.enable_dex && self.enable_defi
    }

    /// Check if liquidity pools are enabled
    pub fn liquidity_pools_enabled(&self) -> bool {
        self.enable_liquidity_pools && self.enable_defi
    }

    /// Check if NFT marketplace is enabled
    pub fn nft_marketplace_enabled(&self) -> bool {
        self.enable_nft_marketplace && self.enable_nft
    }

    /// Check if NFT auctions are enabled
    pub fn nft_auctions_enabled(&self) -> bool {
        self.enable_nft_auctions && self.enable_nft
    }

    /// Check if NFT royalties are enabled
    pub fn nft_royalties_enabled(&self) -> bool {
        self.enable_nft_royalties && self.enable_nft
    }

    /// Check if zero-knowledge proofs are enabled
    pub fn zk_proofs_enabled(&self) -> bool {
        self.enable_zk_proofs && self.enable_privacy
    }

    /// Check if ring signatures are enabled
    pub fn ring_signatures_enabled(&self) -> bool {
        self.enable_ring_signatures && self.enable_privacy
    }

    /// Check if stealth addresses are enabled
    pub fn stealth_addresses_enabled(&self) -> bool {
        self.enable_stealth_addresses && self.enable_privacy
    }

    /// Check if confidential transactions are enabled
    pub fn confidential_transactions_enabled(&self) -> bool {
        self.enable_confidential_transactions && self.enable_privacy
    }

    /// Check if AI prediction is enabled
    pub fn ai_prediction_enabled(&self) -> bool {
        self.enable_ai_prediction && self.enable_ai
    }

    /// Check if AI optimization is enabled
    pub fn ai_optimization_enabled(&self) -> bool {
        self.enable_ai_optimization && self.enable_ai
    }

    /// Check if AI risk management is enabled
    pub fn ai_risk_management_enabled(&self) -> bool {
        self.enable_ai_risk_management && self.enable_ai
    }

    /// Check if AI trading bots are enabled
    pub fn ai_trading_bots_enabled(&self) -> bool {
        self.enable_ai_trading_bots && self.enable_ai
    }

    /// Check if AI governance is enabled
    pub fn ai_governance_enabled(&self) -> bool {
        self.enable_ai_governance && self.enable_ai
    }

    /// Get all enabled features as a list
    pub fn get_enabled_features(&self) -> Vec<String> {
        let mut features = Vec::new();

        if self.enable_ai { features.push("AI".to_string()); }
        if self.enable_defi { features.push("DeFi".to_string()); }
        if self.enable_nft { features.push("NFT".to_string()); }
        if self.enable_privacy { features.push("Privacy".to_string()); }
        if self.enable_smart_contracts { features.push("Smart Contracts".to_string()); }
        if self.enable_bridges { features.push("Bridges".to_string()); }
        if self.enable_governance { features.push("Governance".to_string()); }
        if self.enable_staking { features.push("Staking".to_string()); }

        features
    }
}
