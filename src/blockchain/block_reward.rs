use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Block reward structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockReward {
    /// Base block reward (before halving)
    pub base_reward: u64,
    /// Current block reward (after halving)
    pub current_reward: u64,
    /// Block height
    pub block_height: u64,
    /// Reward type
    pub reward_type: RewardType,
    /// Fee rewards from transactions
    pub fee_rewards: u64,
    /// Total reward (current + fees)
    pub total_reward: u64,
    /// Reward recipients
    pub recipients: Vec<RewardRecipient>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RewardType {
    /// Mining reward (Proof of Work)
    Mining,
    /// Staking reward (Proof of Stake)
    Staking,
    /// Validator reward
    Validation,
    /// Governance reward
    Governance,
    /// Development fund reward
    Development,
    /// Community reward
    Community,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewardRecipient {
    /// Recipient address
    pub address: Vec<u8>,
    /// Reward amount
    pub amount: u64,
    /// Reward type
    pub reward_type: RewardType,
    /// Recipient role
    pub role: RecipientRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipientRole {
    /// Block producer/miner
    BlockProducer,
    /// Validator/staker
    Validator,
    /// Governance participant
    Governance,
    /// Development team
    Developer,
    /// Community fund
    Community,
    /// Treasury
    Treasury,
}

/// Halving schedule for block rewards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HalvingSchedule {
    /// Initial block reward
    pub initial_reward: u64,
    /// Halving interval (blocks)
    pub halving_interval: u64,
    /// Current halving period
    pub current_period: u64,
    /// Minimum reward (never goes below this)
    pub minimum_reward: u64,
    /// Maximum halvings
    pub max_halvings: u64,
    /// Total supply limit
    pub total_supply_limit: u64,
    /// Current total supply
    pub current_supply: u64,
}

impl HalvingSchedule {
    pub fn new(
        initial_reward: u64,
        halving_interval: u64,
        minimum_reward: u64,
        max_halvings: u64,
        total_supply_limit: u64,
    ) -> Self {
        Self {
            initial_reward,
            halving_interval,
            current_period: 0,
            minimum_reward,
            max_halvings,
            total_supply_limit,
            current_supply: 0,
        }
    }

    /// Calculate reward for given block height
    pub fn calculate_reward(&self, block_height: u64) -> u64 {
        let halving_period = block_height / self.halving_interval;
        
        if halving_period >= self.max_halvings {
            return self.minimum_reward;
        }

        let reward = self.initial_reward >> halving_period.min(63); // Prevent overflow
        
        reward.max(self.minimum_reward)
    }

    /// Get current halving period
    pub fn get_current_period(&self, block_height: u64) -> u64 {
        block_height / self.halving_interval
    }

    /// Check if halving occurs at given block
    pub fn is_halving_block(&self, block_height: u64) -> bool {
        block_height > 0 && block_height % self.halving_interval == 0
    }

    /// Get blocks until next halving
    pub fn blocks_until_halving(&self, current_height: u64) -> u64 {
        let next_halving = ((current_height / self.halving_interval) + 1) * self.halving_interval;
        next_halving - current_height
    }

    /// Calculate total supply at given block height
    pub fn calculate_total_supply(&self, block_height: u64) -> u64 {
        let mut total_supply = 0u64;
        let mut current_height = 0u64;
        let mut current_reward = self.initial_reward;
        let mut halving_period = 0u64;

        while current_height <= block_height {
            let blocks_in_period = if halving_period < self.max_halvings {
                self.halving_interval.min(block_height - current_height + 1)
            } else {
                block_height - current_height + 1
            };

            total_supply += current_reward * blocks_in_period;
            current_height += blocks_in_period;
            
            if halving_period < self.max_halvings {
                current_reward = current_reward >> 1;
                current_reward = current_reward.max(self.minimum_reward);
                halving_period += 1;
            }
        }

        total_supply.min(self.total_supply_limit)
    }

    /// Update current supply
    pub fn update_supply(&mut self, block_height: u64) {
        self.current_supply = self.calculate_total_supply(block_height);
    }

    /// Get inflation rate at given block height
    pub fn get_inflation_rate(&self, block_height: u64) -> f64 {
        if block_height == 0 {
            return 0.0;
        }

        let current_supply = self.calculate_total_supply(block_height - 1);
        let new_reward = self.calculate_reward(block_height);
        
        if current_supply == 0 {
            0.0
        } else {
            (new_reward as f64 / current_supply as f64) * 100.0
        }
    }
}

/// Inflation control system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationControl {
    /// Target inflation rate (percentage)
    pub target_inflation_rate: f64,
    /// Current inflation rate
    pub current_inflation_rate: f64,
    /// Adjustment sensitivity
    pub adjustment_sensitivity: f64,
    /// Minimum inflation rate
    pub min_inflation_rate: f64,
    /// Maximum inflation rate
    pub max_inflation_rate: f64,
    /// Inflation history
    pub inflation_history: Vec<f64>,
    /// Adjustment history
    pub adjustment_history: Vec<InflationAdjustment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationAdjustment {
    pub block_height: u64,
    pub old_rate: f64,
    pub new_rate: f64,
    pub adjustment_factor: f64,
    pub reason: String,
}

impl InflationControl {
    pub fn new(
        target_inflation_rate: f64,
        adjustment_sensitivity: f64,
        min_inflation_rate: f64,
        max_inflation_rate: f64,
    ) -> Self {
        Self {
            target_inflation_rate,
            current_inflation_rate: target_inflation_rate,
            adjustment_sensitivity,
            min_inflation_rate,
            max_inflation_rate,
            inflation_history: Vec::new(),
            adjustment_history: Vec::new(),
        }
    }

    /// Update inflation rate
    pub fn update_inflation_rate(&mut self, block_height: u64, new_rate: f64) {
        self.inflation_history.push(new_rate);
        self.current_inflation_rate = new_rate;
        
        // Keep only recent history
        if self.inflation_history.len() > 1000 {
            self.inflation_history.remove(0);
        }
    }

    /// Check if adjustment is needed
    pub fn should_adjust(&self) -> bool {
        let deviation = (self.current_inflation_rate - self.target_inflation_rate).abs();
        deviation > self.adjustment_sensitivity
    }

    /// Calculate adjustment factor
    pub fn calculate_adjustment_factor(&self) -> f64 {
        let deviation = self.current_inflation_rate - self.target_inflation_rate;
        let adjustment_factor = 1.0 - (deviation * self.adjustment_sensitivity);
        
        adjustment_factor
            .max(self.min_inflation_rate / self.target_inflation_rate)
            .min(self.max_inflation_rate / self.target_inflation_rate)
    }

    /// Apply inflation adjustment
    pub fn apply_adjustment(&mut self, block_height: u64, reason: String) -> f64 {
        let old_rate = self.current_inflation_rate;
        let adjustment_factor = self.calculate_adjustment_factor();
        let new_rate = self.target_inflation_rate * adjustment_factor;
        
        self.current_inflation_rate = new_rate;
        
        self.adjustment_history.push(InflationAdjustment {
            block_height,
            old_rate,
            new_rate,
            adjustment_factor,
            reason,
        });
        
        // Keep only recent adjustments
        if self.adjustment_history.len() > 100 {
            self.adjustment_history.remove(0);
        }
        
        adjustment_factor
    }

    /// Get average inflation rate over period
    pub fn get_average_inflation_rate(&self, period: usize) -> f64 {
        if self.inflation_history.is_empty() {
            return 0.0;
        }
        
        let start = if self.inflation_history.len() > period {
            self.inflation_history.len() - period
        } else {
            0
        };
        
        let sum: f64 = self.inflation_history[start..].iter().sum();
        sum / (self.inflation_history.len() - start) as f64
    }

    /// Get inflation statistics
    pub fn get_stats(&self) -> InflationStats {
        InflationStats {
            target_inflation_rate: self.target_inflation_rate,
            current_inflation_rate: self.current_inflation_rate,
            average_inflation_rate: self.get_average_inflation_rate(100),
            min_inflation_rate: self.min_inflation_rate,
            max_inflation_rate: self.max_inflation_rate,
            adjustment_count: self.adjustment_history.len(),
            last_adjustment: self.adjustment_history.last().cloned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationStats {
    pub target_inflation_rate: f64,
    pub current_inflation_rate: f64,
    pub average_inflation_rate: f64,
    pub min_inflation_rate: f64,
    pub max_inflation_rate: f64,
    pub adjustment_count: usize,
    pub last_adjustment: Option<InflationAdjustment>,
}

/// Block reward manager
#[derive(Debug, Clone)]
pub struct BlockRewardManager {
    /// Halving schedule
    pub halving_schedule: HalvingSchedule,
    /// Inflation control
    pub inflation_control: InflationControl,
    /// Reward recipients configuration
    pub reward_distribution: RewardDistribution,
    /// Block reward history
    pub reward_history: Vec<BlockReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    /// Block producer reward percentage
    pub block_producer_percentage: f64,
    /// Validator reward percentage
    pub validator_percentage: f64,
    /// Governance reward percentage
    pub governance_percentage: f64,
    /// Development fund percentage
    pub development_percentage: f64,
    /// Community fund percentage
    pub community_percentage: f64,
    /// Treasury percentage
    pub treasury_percentage: f64,
}

impl RewardDistribution {
    pub fn new() -> Self {
        Self {
            block_producer_percentage: 50.0,
            validator_percentage: 30.0,
            governance_percentage: 10.0,
            development_percentage: 5.0,
            community_percentage: 3.0,
            treasury_percentage: 2.0,
        }
    }

    /// Validate that percentages sum to 100%
    pub fn validate(&self) -> Result<(), String> {
        let total = self.block_producer_percentage +
                   self.validator_percentage +
                   self.governance_percentage +
                   self.development_percentage +
                   self.community_percentage +
                   self.treasury_percentage;

        if (total - 100.0).abs() > 0.01 {
            Err(format!("Reward distribution percentages must sum to 100%, got {}", total))
        } else {
            Ok(())
        }
    }
}

impl BlockRewardManager {
    pub fn new() -> Self {
        let halving_schedule = HalvingSchedule::new(
            50_000_000_000, // 50 UNI (in satoshis)
            210_000,        // Halving every 210,000 blocks
            1_000_000,      // Minimum reward 0.01 UNI
            64,             // Maximum 64 halvings
            21_000_000_000_000_000, // 21M UNI total supply
        );

        let inflation_control = InflationControl::new(
            2.0,    // Target 2% annual inflation
            0.1,    // 0.1% sensitivity
            0.5,    // Minimum 0.5% inflation
            5.0,    // Maximum 5% inflation
        );

        Self {
            halving_schedule,
            inflation_control,
            reward_distribution: RewardDistribution::new(),
            reward_history: Vec::new(),
        }
    }

    /// Calculate block reward for given height
    pub fn calculate_block_reward(&self, block_height: u64, fee_rewards: u64) -> BlockReward {
        let base_reward = self.halving_schedule.calculate_reward(block_height);
        let total_reward = base_reward + fee_rewards;

        let mut recipients = Vec::new();
        
        // Block producer reward
        let block_producer_amount = (total_reward as f64 * self.reward_distribution.block_producer_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![0u8; 20], // Placeholder address
            amount: block_producer_amount,
            reward_type: RewardType::Mining,
            role: RecipientRole::BlockProducer,
        });

        // Validator rewards
        let validator_amount = (total_reward as f64 * self.reward_distribution.validator_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![1u8; 20], // Placeholder address
            amount: validator_amount,
            reward_type: RewardType::Staking,
            role: RecipientRole::Validator,
        });

        // Governance rewards
        let governance_amount = (total_reward as f64 * self.reward_distribution.governance_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![2u8; 20], // Placeholder address
            amount: governance_amount,
            reward_type: RewardType::Governance,
            role: RecipientRole::Governance,
        });

        // Development fund
        let development_amount = (total_reward as f64 * self.reward_distribution.development_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![3u8; 20], // Placeholder address
            amount: development_amount,
            reward_type: RewardType::Development,
            role: RecipientRole::Developer,
        });

        // Community fund
        let community_amount = (total_reward as f64 * self.reward_distribution.community_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![4u8; 20], // Placeholder address
            amount: community_amount,
            reward_type: RewardType::Community,
            role: RecipientRole::Community,
        });

        // Treasury
        let treasury_amount = (total_reward as f64 * self.reward_distribution.treasury_percentage / 100.0) as u64;
        recipients.push(RewardRecipient {
            address: vec![5u8; 20], // Placeholder address
            amount: treasury_amount,
            reward_type: RewardType::Governance,
            role: RecipientRole::Treasury,
        });

        BlockReward {
            base_reward: self.halving_schedule.initial_reward,
            current_reward: base_reward,
            block_height,
            reward_type: RewardType::Mining,
            fee_rewards,
            total_reward,
            recipients,
        }
    }

    /// Process block reward
    pub fn process_block_reward(&mut self, block_height: u64, fee_rewards: u64) -> BlockReward {
        let reward = self.calculate_block_reward(block_height, fee_rewards);
        
        // Update halving schedule
        self.halving_schedule.update_supply(block_height);
        
        // Update inflation control
        let inflation_rate = self.halving_schedule.get_inflation_rate(block_height);
        self.inflation_control.update_inflation_rate(block_height, inflation_rate);
        
        // Check for inflation adjustment
        if self.inflation_control.should_adjust() {
            self.inflation_control.apply_adjustment(
                block_height,
                "Automatic inflation adjustment".to_string(),
            );
        }
        
        // Store reward history
        self.reward_history.push(reward.clone());
        
        // Keep only recent history
        if self.reward_history.len() > 10000 {
            self.reward_history.remove(0);
        }
        
        reward
    }

    /// Get reward statistics
    pub fn get_stats(&self, block_height: u64) -> RewardStats {
        RewardStats {
            current_reward: self.halving_schedule.calculate_reward(block_height),
            blocks_until_halving: self.halving_schedule.blocks_until_halving(block_height),
            current_halving_period: self.halving_schedule.get_current_period(block_height),
            total_supply: self.halving_schedule.calculate_total_supply(block_height),
            inflation_rate: self.halving_schedule.get_inflation_rate(block_height),
            inflation_stats: self.inflation_control.get_stats(),
        }
    }

    /// Update reward distribution
    pub fn update_reward_distribution(&mut self, distribution: RewardDistribution) -> Result<(), String> {
        distribution.validate()?;
        self.reward_distribution = distribution;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardStats {
    pub current_reward: u64,
    pub blocks_until_halving: u64,
    pub current_halving_period: u64,
    pub total_supply: u64,
    pub inflation_rate: f64,
    pub inflation_stats: InflationStats,
}

impl Default for BlockRewardManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RewardDistribution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halving_schedule() {
        let schedule = HalvingSchedule::new(
            50_000_000_000, // 50 UNI
            210_000,        // Halving every 210,000 blocks
            1_000_000,      // Minimum 0.01 UNI
            64,             // Max 64 halvings
            21_000_000_000_000_000, // 21M UNI total
        );

        assert_eq!(schedule.calculate_reward(0), 50_000_000_000);
        assert_eq!(schedule.calculate_reward(209_999), 50_000_000_000);
        assert_eq!(schedule.calculate_reward(210_000), 25_000_000_000);
        assert_eq!(schedule.calculate_reward(420_000), 12_500_000_000);
    }

    #[test]
    fn test_halving_detection() {
        let schedule = HalvingSchedule::new(50_000_000_000, 210_000, 1_000_000, 64, 21_000_000_000_000_000);
        
        assert!(!schedule.is_halving_block(0));
        assert!(!schedule.is_halving_block(209_999));
        assert!(schedule.is_halving_block(210_000));
        assert!(schedule.is_halving_block(420_000));
    }

    #[test]
    fn test_inflation_control() {
        let mut control = InflationControl::new(2.0, 0.1, 0.5, 5.0);
        
        control.update_inflation_rate(100, 2.5);
        assert_eq!(control.current_inflation_rate, 2.5);
        
        assert!(control.should_adjust());
        
        let factor = control.apply_adjustment(101, "Test adjustment".to_string());
        assert!(factor != 1.0);
    }

    #[test]
    fn test_reward_distribution() {
        let distribution = RewardDistribution::new();
        assert!(distribution.validate().is_ok());
        
        let mut invalid_distribution = RewardDistribution::new();
        invalid_distribution.block_producer_percentage = 60.0;
        invalid_distribution.validator_percentage = 50.0;
        
        assert!(invalid_distribution.validate().is_err());
    }

    #[test]
    fn test_block_reward_manager() {
        let manager = BlockRewardManager::new();
        let reward = manager.calculate_block_reward(1000, 1000000);
        
        assert_eq!(reward.block_height, 1000);
        assert_eq!(reward.fee_rewards, 1000000);
        assert!(!reward.recipients.is_empty());
        
        let stats = manager.get_stats(1000);
        assert!(stats.current_reward > 0);
        assert!(stats.total_supply > 0);
    }
}
