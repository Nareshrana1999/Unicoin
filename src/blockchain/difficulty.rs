use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Mining difficulty target
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    /// Target value as 256-bit integer
    pub value: [u8; 32],
    /// Target difficulty (bits)
    pub bits: u32,
}

impl Target {
    pub fn new(bits: u32) -> Self {
        let mut value = [0u8; 32];
        
        // Convert compact representation to target value
        let exponent = (bits >> 24) as usize;
        let mantissa = bits & 0x00ffffff;
        
        if exponent <= 3 {
            // Target is very small
            value[31 - exponent] = (mantissa >> (8 * (3 - exponent))) as u8;
            if exponent < 3 {
                value[30 - exponent] = (mantissa >> (8 * (2 - exponent))) as u8;
            }
            if exponent < 2 {
                value[29 - exponent] = (mantissa >> (8 * (1 - exponent))) as u8;
            }
            if exponent < 1 {
                value[28 - exponent] = mantissa as u8;
            }
        } else {
            // Target is larger
            let shift = (exponent - 3) * 8;
            if shift < 256 {
                let byte_index = 31 - (shift / 8);
                let bit_shift = shift % 8;
                
                if byte_index < 32 {
                    value[byte_index] = (mantissa >> (24 - bit_shift)) as u8;
                }
                if byte_index > 0 && bit_shift > 0 {
                    value[byte_index - 1] = (mantissa << bit_shift) as u8;
                }
            }
        }
        
        Self { value, bits }
    }

    /// Create target from difficulty value
    pub fn from_difficulty(difficulty: f64) -> Self {
        // Maximum target (genesis block target)
        let max_target = 0x00000000ffff0000000000000000000000000000000000000000000000000000u128;
        
        // Calculate target value
        let target_value = (max_target as f64 / difficulty) as u128;
        
        // Convert to compact representation
        let mut target_bytes = [0u8; 32];
        target_bytes[16..32].copy_from_slice(&target_value.to_be_bytes());
        
        // Find compact representation
        let mut bits = 0u32;
        let mut exponent = 32usize;
        
        // Find first non-zero byte
        for (i, &byte) in target_bytes.iter().enumerate() {
            if byte != 0 {
                exponent = i;
                break;
            }
        }
        
        if exponent < 32 {
            let mantissa = if exponent <= 28 {
                u32::from_be_bytes([
                    target_bytes[exponent],
                    if exponent + 1 < 32 { target_bytes[exponent + 1] } else { 0 },
                    if exponent + 2 < 32 { target_bytes[exponent + 2] } else { 0 },
                    if exponent + 3 < 32 { target_bytes[exponent + 3] } else { 0 },
                ])
            } else {
                target_bytes[exponent] as u32
            };
            
            bits = ((32 - exponent) as u32) << 24 | mantissa;
        }
        
        Self { value: target_bytes, bits }
    }

    /// Check if hash meets target
    pub fn is_met_by(&self, hash: &Hash) -> bool {
        let hash_bytes = hash.to_bytes();
        
        // Compare byte by byte (big-endian)
        for i in 0..32 {
            if hash_bytes[i] < self.value[i] {
                return true;
            } else if hash_bytes[i] > self.value[i] {
                return false;
            }
        }
        
        // Hash equals target (very unlikely but valid)
        true
    }

    /// Get difficulty from target
    pub fn get_difficulty(&self) -> f64 {
        // Maximum target (genesis block target)
        let max_target = 0x00000000ffff0000000000000000000000000000000000000000000000000000u128;
        
        // Convert target to integer
        let mut target_value = 0u128;
        for i in 0..16 {
            target_value = (target_value << 8) | self.value[i] as u128;
        }
        
        if target_value == 0 {
            f64::INFINITY
        } else {
            max_target as f64 / target_value as f64
        }
    }

    /// Convert target to bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.value
    }

    /// Create target from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        // Convert back to compact representation
        let mut exponent = 32usize;
        
        // Find first non-zero byte
        for (i, &byte) in bytes.iter().enumerate() {
            if byte != 0 {
                exponent = i;
                break;
            }
        }
        
        let bits = if exponent < 32 {
            let mantissa = if exponent <= 28 {
                u32::from_be_bytes([
                    bytes[exponent],
                    if exponent + 1 < 32 { bytes[exponent + 1] } else { 0 },
                    if exponent + 2 < 32 { bytes[exponent + 2] } else { 0 },
                    if exponent + 3 < 32 { bytes[exponent + 3] } else { 0 },
                ])
            } else {
                bytes[exponent] as u32
            };
            
            ((32 - exponent) as u32) << 24 | mantissa
        } else {
            0
        };
        
        Self { value: bytes, bits }
    }
}

/// Mining difficulty adjustment algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyAdjustment {
    /// Current target
    pub current_target: Target,
    /// Target adjustment interval (blocks)
    pub adjustment_interval: u32,
    /// Target adjustment time (seconds)
    pub target_timespan: u64,
    /// Block time history for adjustment
    pub block_times: VecDeque<u64>,
    /// Block heights for adjustment
    pub block_heights: VecDeque<u64>,
    /// Minimum difficulty
    pub min_difficulty: f64,
    /// Maximum difficulty
    pub max_difficulty: f64,
    /// Difficulty adjustment factor limits
    pub min_adjustment_factor: f64,
    pub max_adjustment_factor: f64,
}

impl DifficultyAdjustment {
    pub fn new(initial_target: Target) -> Self {
        Self {
            current_target: initial_target,
            adjustment_interval: 2016, // Bitcoin's adjustment interval
            target_timespan: 1209600,  // 2 weeks in seconds
            block_times: VecDeque::new(),
            block_heights: VecDeque::new(),
            min_difficulty: 1.0,
            max_difficulty: 1e12,
            min_adjustment_factor: 0.25,
            max_adjustment_factor: 4.0,
        }
    }

    /// Add a new block for difficulty adjustment
    pub fn add_block(&mut self, block_height: u64, block_time: u64) {
        self.block_heights.push_back(block_height);
        self.block_times.push_back(block_time);
        
        // Keep only recent blocks for adjustment
        while self.block_heights.len() > self.adjustment_interval as usize {
            self.block_heights.pop_front();
            self.block_times.pop_front();
        }
    }

    /// Check if difficulty adjustment is needed
    pub fn should_adjust(&self) -> bool {
        self.block_heights.len() >= self.adjustment_interval as usize
    }

    /// Calculate new difficulty target
    pub fn calculate_new_target(&self) -> Result<Target, String> {
        if !self.should_adjust() {
            return Ok(self.current_target.clone());
        }

        let heights: Vec<u64> = self.block_heights.iter().cloned().collect();
        let times: Vec<u64> = self.block_times.iter().cloned().collect();

        // Get first and last blocks in adjustment period
        let first_height = heights[0];
        let last_height = heights[heights.len() - 1];
        let first_time = times[0];
        let last_time = times[times.len() - 1];

        // Calculate actual timespan
        let actual_timespan = last_time.saturating_sub(first_time);

        // Limit timespan to prevent extreme adjustments
        let limited_timespan = if actual_timespan < self.target_timespan / self.max_adjustment_factor as u64 {
            self.target_timespan / self.max_adjustment_factor as u64
        } else if actual_timespan > self.target_timespan * self.max_adjustment_factor as u64 {
            self.target_timespan * self.max_adjustment_factor as u64
        } else {
            actual_timespan
        };

        // Calculate adjustment factor
        let adjustment_factor = limited_timespan as f64 / self.target_timespan as f64;

        // Apply adjustment factor to current target
        let current_difficulty = self.current_target.get_difficulty();
        let new_difficulty = current_difficulty * adjustment_factor;

        // Limit difficulty to allowed range
        let clamped_difficulty = new_difficulty
            .max(self.min_difficulty)
            .min(self.max_difficulty);

        Ok(Target::from_difficulty(clamped_difficulty))
    }

    /// Update difficulty target
    pub fn update_target(&mut self) -> Result<Target, String> {
        let new_target = self.calculate_new_target()?;
        self.current_target = new_target.clone();
        Ok(new_target)
    }

    /// Get current target
    pub fn get_current_target(&self) -> &Target {
        &self.current_target
    }

    /// Get current difficulty
    pub fn get_current_difficulty(&self) -> f64 {
        self.current_target.get_difficulty()
    }

    /// Get target for next block
    pub fn get_next_target(&self) -> Target {
        if self.should_adjust() {
            self.calculate_new_target().unwrap_or(self.current_target.clone())
        } else {
            self.current_target.clone()
        }
    }

    /// Reset adjustment history
    pub fn reset(&mut self) {
        self.block_times.clear();
        self.block_heights.clear();
    }

    /// Set adjustment parameters
    pub fn set_parameters(
        &mut self,
        interval: u32,
        timespan: u64,
        min_factor: f64,
        max_factor: f64,
    ) {
        self.adjustment_interval = interval;
        self.target_timespan = timespan;
        self.min_adjustment_factor = min_factor;
        self.max_adjustment_factor = max_factor;
    }

    /// Get adjustment statistics
    pub fn get_stats(&self) -> DifficultyStats {
        let blocks_in_period = self.block_heights.len();
        let current_difficulty = self.get_current_difficulty();
        
        let actual_timespan = if blocks_in_period >= 2 {
            let first_time = self.block_times[0];
            let last_time = self.block_times[blocks_in_period - 1];
            last_time.saturating_sub(first_time)
        } else {
            0
        };

        let adjustment_factor = if actual_timespan > 0 {
            actual_timespan as f64 / self.target_timespan as f64
        } else {
            1.0
        };

        DifficultyStats {
            current_difficulty,
            current_target_bits: self.current_target.bits,
            blocks_in_period: blocks_in_period as u32,
            actual_timespan,
            target_timespan: self.target_timespan,
            adjustment_factor,
            should_adjust: self.should_adjust(),
        }
    }
}

/// Mining difficulty statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyStats {
    pub current_difficulty: f64,
    pub current_target_bits: u32,
    pub blocks_in_period: u32,
    pub actual_timespan: u64,
    pub target_timespan: u64,
    pub adjustment_factor: f64,
    pub should_adjust: bool,
}

/// Mining difficulty for different algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MiningDifficulty {
    /// Proof of Work difficulty
    ProofOfWork(Target),
    /// Proof of Stake difficulty (validator requirements)
    ProofOfStake {
        minimum_stake: u64,
        stake_weight: u64,
        validator_count: u32,
    },
    /// Hybrid difficulty (PoW + PoS)
    Hybrid {
        pow_target: Target,
        pos_requirements: ProofOfStake,
    },
}

impl MiningDifficulty {
    pub fn new_pow(initial_target: Target) -> Self {
        Self::ProofOfWork(initial_target)
    }

    pub fn new_pos(minimum_stake: u64, stake_weight: u64, validator_count: u32) -> Self {
        Self::ProofOfStake {
            minimum_stake,
            stake_weight,
            validator_count,
        }
    }

    pub fn new_hybrid(pow_target: Target, pos_requirements: ProofOfStake) -> Self {
        Self::Hybrid {
            pow_target,
            pos_requirements,
        }
    }

    pub fn get_pow_target(&self) -> Option<&Target> {
        match self {
            Self::ProofOfWork(target) => Some(target),
            Self::Hybrid { pow_target, .. } => Some(pow_target),
            _ => None,
        }
    }

    pub fn get_pos_requirements(&self) -> Option<&ProofOfStake> {
        match self {
            Self::ProofOfStake { .. } => {
                // Return self as ProofOfStake reference
                unsafe { std::mem::transmute(self) }
            }
            Self::Hybrid { pos_requirements, .. } => Some(pos_requirements),
            _ => None,
        }
    }

    pub fn is_pow(&self) -> bool {
        matches!(self, Self::ProofOfWork(_))
    }

    pub fn is_pos(&self) -> bool {
        matches!(self, Self::ProofOfStake { .. })
    }

    pub fn is_hybrid(&self) -> bool {
        matches!(self, Self::Hybrid { .. })
    }
}

/// Proof of Stake requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfStake {
    pub minimum_stake: u64,
    pub stake_weight: u64,
    pub validator_count: u32,
}

impl ProofOfStake {
    pub fn new(minimum_stake: u64, stake_weight: u64, validator_count: u32) -> Self {
        Self {
            minimum_stake,
            stake_weight,
            validator_count,
        }
    }

    pub fn calculate_stake_probability(&self, stake_amount: u64) -> f64 {
        if stake_amount < self.minimum_stake {
            return 0.0;
        }

        let total_stake = self.stake_weight;
        if total_stake == 0 {
            return 0.0;
        }

        stake_amount as f64 / total_stake as f64
    }

    pub fn is_eligible_validator(&self, stake_amount: u64) -> bool {
        stake_amount >= self.minimum_stake
    }

    pub fn get_validator_slot_probability(&self, stake_amount: u64) -> f64 {
        if !self.is_eligible_validator(stake_amount) {
            return 0.0;
        }

        self.calculate_stake_probability(stake_amount) / self.validator_count as f64
    }
}

impl Default for DifficultyAdjustment {
    fn default() -> Self {
        // Genesis block target
        let genesis_target = Target::new(0x1d00ffff);
        Self::new(genesis_target)
    }
}

impl Default for MiningDifficulty {
    fn default() -> Self {
        Self::new_pow(Target::new(0x1d00ffff))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_creation() {
        let bits = 0x1d00ffff;
        let target = Target::new(bits);
        assert_eq!(target.bits, bits);
    }

    #[test]
    fn test_target_difficulty() {
        let target = Target::new(0x1d00ffff);
        let difficulty = target.get_difficulty();
        assert!(difficulty > 0.0);
        assert!(difficulty.is_finite());
    }

    #[test]
    fn test_target_from_difficulty() {
        let difficulty = 1000.0;
        let target = Target::from_difficulty(difficulty);
        let calculated_difficulty = target.get_difficulty();
        
        // Allow for small floating point differences
        assert!((calculated_difficulty - difficulty).abs() < 1.0);
    }

    #[test]
    fn test_difficulty_adjustment() {
        let initial_target = Target::new(0x1d00ffff);
        let mut adjustment = DifficultyAdjustment::new(initial_target);
        
        // Add some blocks
        for i in 0..2016 {
            adjustment.add_block(i, i * 600); // 10 minutes per block
        }
        
        assert!(adjustment.should_adjust());
        
        let new_target = adjustment.update_target().unwrap();
        assert_ne!(new_target.bits, adjustment.current_target.bits);
    }

    #[test]
    fn test_proof_of_stake() {
        let pos = ProofOfStake::new(1000, 10000, 10);
        
        assert!(pos.is_eligible_validator(1000));
        assert!(!pos.is_eligible_validator(999));
        
        let probability = pos.calculate_stake_probability(2000);
        assert_eq!(probability, 0.2);
    }

    #[test]
    fn test_mining_difficulty_types() {
        let pow_target = Target::new(0x1d00ffff);
        let pow_difficulty = MiningDifficulty::new_pow(pow_target);
        assert!(pow_difficulty.is_pow());
        assert!(!pow_difficulty.is_pos());
        assert!(!pow_difficulty.is_hybrid());

        let pos_difficulty = MiningDifficulty::new_pos(1000, 10000, 10);
        assert!(!pos_difficulty.is_pow());
        assert!(pos_difficulty.is_pos());
        assert!(!pos_difficulty.is_hybrid());
    }
}
