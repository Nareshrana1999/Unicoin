use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gas management system for Ethereum compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasManager {
    /// Current gas price in wei
    pub gas_price: u64,
    /// Gas limit for blocks
    pub block_gas_limit: u64,
    /// Gas costs for different operations
    pub operation_costs: HashMap<String, u64>,
    /// Gas price history
    pub price_history: Vec<GasPricePoint>,
    /// Dynamic gas adjustment
    pub dynamic_adjustment: bool,
}

/// Gas price point for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPricePoint {
    /// Timestamp
    pub timestamp: u64,
    /// Gas price
    pub gas_price: u64,
    /// Block number
    pub block_number: u64,
}

/// Gas price configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPrice {
    /// Slow gas price (low priority)
    pub slow: u64,
    /// Standard gas price (normal priority)
    pub standard: u64,
    /// Fast gas price (high priority)
    pub fast: u64,
    /// Instant gas price (urgent priority)
    pub instant: u64,
}

/// Gas limit for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasLimit {
    /// Minimum gas limit
    pub minimum: u64,
    /// Maximum gas limit
    pub maximum: u64,
    /// Default gas limit
    pub default: u64,
    /// Gas limit for contract creation
    pub contract_creation: u64,
    /// Gas limit for simple transfers
    pub transfer: u64,
}

/// Gas estimation for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimation {
    /// Estimated gas usage
    pub estimated_gas: u64,
    /// Gas price recommendation
    pub recommended_gas_price: u64,
    /// Total transaction cost
    pub total_cost: u64,
    /// Confidence level (0-100)
    pub confidence: u8,
}

impl GasManager {
    pub fn new() -> Self {
        let mut manager = Self {
            gas_price: 20_000_000_000, // 20 gwei
            block_gas_limit: 10_000_000,
            operation_costs: HashMap::new(),
            price_history: Vec::new(),
            dynamic_adjustment: true,
        };
        
        manager.initialize_operation_costs();
        manager
    }

    /// Initialize operation costs
    fn initialize_operation_costs(&mut self) {
        // Basic operations
        self.operation_costs.insert("zero".to_string(), 0);
        self.operation_costs.insert("base".to_string(), 2);
        self.operation_costs.insert("very_low".to_string(), 3);
        self.operation_costs.insert("low".to_string(), 5);
        self.operation_costs.insert("mid".to_string(), 8);
        self.operation_costs.insert("high".to_string(), 10);
        
        // Memory operations
        self.operation_costs.insert("memory".to_string(), 3);
        self.operation_costs.insert("memory_expansion".to_string(), 3);
        
        // Storage operations
        self.operation_costs.insert("s_load".to_string(), 200);
        self.operation_costs.insert("s_store".to_string(), 20000);
        self.operation_costs.insert("sstore_set".to_string(), 20000);
        self.operation_costs.insert("sstore_reset".to_string(), 5000);
        self.operation_costs.insert("sstore_clear".to_string(), 15000);
        
        // Cryptographic operations
        self.operation_costs.insert("sha3".to_string(), 30);
        self.operation_costs.insert("sha3_word".to_string(), 6);
        self.operation_costs.insert("ecrecover".to_string(), 3000);
        
        // Contract operations
        self.operation_costs.insert("create".to_string(), 32000);
        self.operation_costs.insert("call".to_string(), 700);
        self.operation_costs.insert("call_value".to_string(), 9000);
        self.operation_costs.insert("call_stipend".to_string(), 2300);
        self.operation_costs.insert("new_account".to_string(), 25000);
        
        // Transaction costs
        self.operation_costs.insert("transaction".to_string(), 21000);
        self.operation_costs.insert("tx_data_zero".to_string(), 4);
        self.operation_costs.insert("tx_data_non_zero".to_string(), 68);
        self.operation_costs.insert("tx_create".to_string(), 53000);
        
        // Log operations
        self.operation_costs.insert("log".to_string(), 375);
        self.operation_costs.insert("log_data".to_string(), 8);
        self.operation_costs.insert("log_topic".to_string(), 375);
        
        // Other operations
        self.operation_costs.insert("copy".to_string(), 3);
        self.operation_costs.insert("suicide".to_string(), 5000);
        self.operation_costs.insert("suicide_refund".to_string(), 24000);
        self.operation_costs.insert("exp".to_string(), 10);
        self.operation_costs.insert("exp_byte".to_string(), 50);
        self.operation_costs.insert("balance".to_string(), 400);
        self.operation_costs.insert("block_hash".to_string(), 20);
        self.operation_costs.insert("ext_code".to_string(), 700);
        self.operation_costs.insert("jumpdest".to_string(), 1);
    }

    /// Get gas cost for operation
    pub fn get_operation_cost(&self, operation: &str) -> u64 {
        self.operation_costs.get(operation).copied().unwrap_or(0)
    }

    /// Calculate gas cost for transaction
    pub fn calculate_transaction_gas(&self, tx_data: &TransactionData) -> u64 {
        let mut gas = self.get_operation_cost("transaction");
        
        // Add gas for data
        for &byte in &tx_data.data {
            if byte == 0 {
                gas += self.get_operation_cost("tx_data_zero");
            } else {
                gas += self.get_operation_cost("tx_data_non_zero");
            }
        }
        
        // Add gas for contract creation
        if tx_data.is_contract_creation {
            gas += self.get_operation_cost("tx_create");
        }
        
        gas
    }

    /// Estimate gas for smart contract call
    pub fn estimate_contract_gas(&self, call_data: &ContractCallData) -> GasEstimation {
        let mut estimated_gas = self.get_operation_cost("call");
        
        // Add gas for contract execution (simplified estimation)
        estimated_gas += call_data.input_data.len() as u64 * 68; // Non-zero data cost
        
        // Add gas for storage operations (estimated)
        estimated_gas += call_data.estimated_storage_operations * self.get_operation_cost("s_store");
        
        // Add gas for memory operations (estimated)
        estimated_gas += call_data.estimated_memory_operations * self.get_operation_cost("memory");
        
        // Add 20% buffer for safety
        estimated_gas = (estimated_gas * 120) / 100;
        
        let recommended_gas_price = self.get_recommended_gas_price();
        let total_cost = estimated_gas * recommended_gas_price;
        
        GasEstimation {
            estimated_gas,
            recommended_gas_price,
            total_cost,
            confidence: 85, // 85% confidence
        }
    }

    /// Get recommended gas price based on network conditions
    pub fn get_recommended_gas_price(&self) -> u64 {
        if !self.dynamic_adjustment {
            return self.gas_price;
        }
        
        // Simple dynamic pricing based on recent history
        if self.price_history.len() < 10 {
            return self.gas_price;
        }
        
        let recent_prices: Vec<u64> = self.price_history
            .iter()
            .rev()
            .take(10)
            .map(|point| point.gas_price)
            .collect();
        
        let average_price: u64 = recent_prices.iter().sum::<u64>() / recent_prices.len() as u64;
        
        // Adjust based on network congestion
        let congestion_factor = self.calculate_network_congestion();
        
        if congestion_factor > 0.8 {
            // High congestion - increase price
            average_price * 150 / 100
        } else if congestion_factor < 0.3 {
            // Low congestion - decrease price
            average_price * 80 / 100
        } else {
            average_price
        }
    }

    /// Calculate network congestion factor (0.0 to 1.0)
    fn calculate_network_congestion(&self) -> f64 {
        // Simplified congestion calculation
        // In a real implementation, this would consider pending transactions,
        // block utilization, and other network metrics
        
        if self.price_history.len() < 5 {
            return 0.5; // Default moderate congestion
        }
        
        let recent_blocks: Vec<&GasPricePoint> = self.price_history
            .iter()
            .rev()
            .take(5)
            .collect();
        
        // Calculate price volatility as congestion indicator
        let prices: Vec<u64> = recent_blocks.iter().map(|p| p.gas_price).collect();
        let avg_price = prices.iter().sum::<u64>() as f64 / prices.len() as f64;
        
        let variance: f64 = prices.iter()
            .map(|&price| (price as f64 - avg_price).powi(2))
            .sum::<f64>() / prices.len() as f64;
        
        let volatility = variance.sqrt() / avg_price;
        
        // Convert volatility to congestion factor (0.0 to 1.0)
        volatility.min(1.0).max(0.0)
    }

    /// Update gas price based on network conditions
    pub fn update_gas_price(&mut self, block_number: u64) {
        let new_price = self.get_recommended_gas_price();
        
        self.price_history.push(GasPricePoint {
            timestamp: crate::utils::timestamp(),
            gas_price: new_price,
            block_number,
        });
        
        // Keep only recent history
        if self.price_history.len() > 1000 {
            self.price_history.remove(0);
        }
        
        self.gas_price = new_price;
    }

    /// Get gas price tiers
    pub fn get_gas_price_tiers(&self) -> GasPrice {
        let base_price = self.gas_price;
        
        GasPrice {
            slow: base_price * 80 / 100,      // 20% cheaper
            standard: base_price,              // Standard price
            fast: base_price * 120 / 100,     // 20% more expensive
            instant: base_price * 150 / 100,  // 50% more expensive
        }
    }

    /// Get gas limits for different operations
    pub fn get_gas_limits(&self) -> GasLimit {
        GasLimit {
            minimum: 21000,
            maximum: self.block_gas_limit,
            default: 100000,
            contract_creation: 2000000,
            transfer: 21000,
        }
    }

    /// Check if gas limit is valid
    pub fn is_valid_gas_limit(&self, gas_limit: u64) -> bool {
        let limits = self.get_gas_limits();
        gas_limit >= limits.minimum && gas_limit <= limits.maximum
    }

    /// Calculate total transaction cost
    pub fn calculate_total_cost(&self, gas_limit: u64, gas_price: Option<u64>) -> u64 {
        let price = gas_price.unwrap_or(self.gas_price);
        gas_limit * price
    }

    /// Get gas price statistics
    pub fn get_gas_price_stats(&self) -> GasPriceStats {
        if self.price_history.is_empty() {
            return GasPriceStats {
                current_price: self.gas_price,
                average_price: self.gas_price,
                min_price: self.gas_price,
                max_price: self.gas_price,
                price_change_24h: 0.0,
                price_change_7d: 0.0,
            };
        }
        
        let current_price = self.gas_price;
        let prices: Vec<u64> = self.price_history.iter().map(|p| p.gas_price).collect();
        
        let average_price = prices.iter().sum::<u64>() / prices.len() as u64;
        let min_price = *prices.iter().min().unwrap();
        let max_price = *prices.iter().max().unwrap();
        
        // Calculate price changes (simplified)
        let price_change_24h = if prices.len() > 24 {
            let old_price = prices[prices.len() - 25];
            ((current_price as f64 - old_price as f64) / old_price as f64) * 100.0
        } else {
            0.0
        };
        
        let price_change_7d = if prices.len() > 168 { // 7 days * 24 hours
            let old_price = prices[prices.len() - 169];
            ((current_price as f64 - old_price as f64) / old_price as f64) * 100.0
        } else {
            0.0
        };
        
        GasPriceStats {
            current_price,
            average_price,
            min_price,
            max_price,
            price_change_24h,
            price_change_7d,
        }
    }
}

/// Transaction data for gas calculation
#[derive(Debug, Clone)]
pub struct TransactionData {
    pub data: Vec<u8>,
    pub is_contract_creation: bool,
    pub to_address: Option<Vec<u8>>,
}

/// Contract call data for gas estimation
#[derive(Debug, Clone)]
pub struct ContractCallData {
    pub input_data: Vec<u8>,
    pub estimated_storage_operations: u64,
    pub estimated_memory_operations: u64,
    pub contract_complexity: u8, // 1-10 scale
}

/// Gas price statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPriceStats {
    pub current_price: u64,
    pub average_price: u64,
    pub min_price: u64,
    pub max_price: u64,
    pub price_change_24h: f64,
    pub price_change_7d: f64,
}

impl Default for GasManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GasPrice {
    fn default() -> Self {
        Self {
            slow: 10_000_000_000,    // 10 gwei
            standard: 20_000_000_000, // 20 gwei
            fast: 30_000_000_000,    // 30 gwei
            instant: 50_000_000_000, // 50 gwei
        }
    }
}

impl Default for GasLimit {
    fn default() -> Self {
        Self {
            minimum: 21000,
            maximum: 10_000_000,
            default: 100000,
            contract_creation: 2000000,
            transfer: 21000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_manager_creation() {
        let manager = GasManager::new();
        assert_eq!(manager.gas_price, 20_000_000_000);
        assert_eq!(manager.block_gas_limit, 10_000_000);
        assert!(!manager.operation_costs.is_empty());
    }

    #[test]
    fn test_operation_cost() {
        let manager = GasManager::new();
        assert_eq!(manager.get_operation_cost("transaction"), 21000);
        assert_eq!(manager.get_operation_cost("s_store"), 20000);
        assert_eq!(manager.get_operation_cost("unknown"), 0);
    }

    #[test]
    fn test_transaction_gas_calculation() {
        let manager = GasManager::new();
        let tx_data = TransactionData {
            data: vec![0, 1, 2, 0, 3],
            is_contract_creation: false,
            to_address: Some(vec![1, 2, 3, 4]),
        };
        
        let gas = manager.calculate_transaction_gas(&tx_data);
        assert!(gas > 21000); // More than base transaction cost
    }

    #[test]
    fn test_gas_limits() {
        let manager = GasManager::new();
        let limits = manager.get_gas_limits();
        
        assert_eq!(limits.minimum, 21000);
        assert_eq!(limits.maximum, 10_000_000);
        assert_eq!(limits.transfer, 21000);
    }

    #[test]
    fn test_gas_price_tiers() {
        let manager = GasManager::new();
        let tiers = manager.get_gas_price_tiers();
        
        assert!(tiers.slow < tiers.standard);
        assert!(tiers.standard < tiers.fast);
        assert!(tiers.fast < tiers.instant);
    }

    #[test]
    fn test_gas_limit_validation() {
        let manager = GasManager::new();
        
        assert!(manager.is_valid_gas_limit(50000));
        assert!(!manager.is_valid_gas_limit(10000)); // Too low
        assert!(!manager.is_valid_gas_limit(20_000_000)); // Too high
    }
}
