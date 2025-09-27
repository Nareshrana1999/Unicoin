//! Decentralized Exchange (DEX) implementation for Unicoin
//! 
//! This module provides a complete DEX implementation with liquidity pools,
//! automated market makers, and trading functionality.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use crate::crypto::{hash::Hash, keys::PublicKey};

/// Decentralized Exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEX {
    /// DEX name
    pub name: String,
    /// Trading fee (basis points)
    pub trading_fee: u16,
    /// Liquidity pools
    pub pools: HashMap<Hash, LiquidityPool>,
    /// Order books for each trading pair
    pub order_books: HashMap<TradingPair, OrderBook>,
    /// DEX statistics
    pub stats: DEXStats,
    /// Protocol fees
    pub protocol_fees: ProtocolFeeStructure,
}

/// Liquidity Pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    /// Pool ID
    pub id: Hash,
    /// Trading pair
    pub pair: TradingPair,
    /// Token A reserves
    pub token_a_reserves: u64,
    /// Token B reserves
    pub token_b_reserves: u64,
    /// Liquidity tokens
    pub liquidity_tokens: u64,
    /// Pool fee (basis points)
    pub fee: u16,
    /// Pool creation timestamp
    pub created_at: u64,
    /// Last update timestamp
    pub last_updated: u64,
    /// Pool state
    pub state: PoolState,
    /// Liquidity providers
    pub providers: HashMap<PublicKey, LiquidityProvider>,
    /// Pool metrics
    pub metrics: PoolMetrics,
}

/// Trading Pair
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    /// Token A address
    pub token_a: Hash,
    /// Token B address
    pub token_b: Hash,
    /// Pair symbol
    pub symbol: String,
}

/// Order Book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Trading pair
    pub pair: TradingPair,
    /// Buy orders (price -> amount)
    pub buy_orders: BTreeMap<u64, u64>,
    /// Sell orders (price -> amount)
    pub sell_orders: BTreeMap<u64, u64>,
    /// Order history
    pub order_history: Vec<Order>,
    /// Last trade price
    pub last_trade_price: Option<u64>,
    /// 24h volume
    pub volume_24h: u64,
    /// 24h high
    pub high_24h: Option<u64>,
    /// 24h low
    pub low_24h: Option<u64>,
}

/// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: Hash,
    /// Order type
    pub order_type: OrderType,
    /// Trading pair
    pub pair: TradingPair,
    /// Order side
    pub side: OrderSide,
    /// Price per token
    pub price: u64,
    /// Amount
    pub amount: u64,
    /// Filled amount
    pub filled_amount: u64,
    /// Order status
    pub status: OrderStatus,
    /// Creator
    pub creator: PublicKey,
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp
    pub expires_at: Option<u64>,
}

/// Order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order (execute immediately)
    Market,
    /// Limit order (execute at specific price)
    Limit,
    /// Stop order (trigger when price reached)
    Stop,
    /// Stop-limit order
    StopLimit,
}

/// Order side
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order is pending
    Pending,
    /// Order is partially filled
    PartiallyFilled,
    /// Order is fully filled
    Filled,
    /// Order is cancelled
    Cancelled,
    /// Order has expired
    Expired,
}

/// Pool state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolState {
    /// Pool is active
    Active,
    /// Pool is paused
    Paused,
    /// Pool is deprecated
    Deprecated,
    /// Pool is closed
    Closed,
}

/// Liquidity Provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityProvider {
    /// Provider's public key
    pub provider: PublicKey,
    /// Liquidity token amount
    pub liquidity_tokens: u64,
    /// Token A contribution
    pub token_a_contribution: u64,
    /// Token B contribution
    pub token_b_contribution: u64,
    /// Entry timestamp
    pub entry_timestamp: u64,
    /// Last claim timestamp
    pub last_claim_timestamp: u64,
    /// Unclaimed fees
    pub unclaimed_fees: u64,
}

/// Pool Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    /// Total trading volume
    pub total_volume: u64,
    /// 24h trading volume
    pub volume_24h: u64,
    /// 7d trading volume
    pub volume_7d: u64,
    /// Total fees collected
    pub total_fees: u64,
    /// 24h fees collected
    pub fees_24h: u64,
    /// Number of transactions
    pub transaction_count: u64,
    /// Number of unique traders
    pub unique_traders: u64,
    /// Price change 24h
    pub price_change_24h: f64,
    /// Price change 7d
    pub price_change_7d: f64,
}

/// DEX Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEXStats {
    /// Total trading volume
    pub total_volume: u64,
    /// 24h trading volume
    pub volume_24h: u64,
    /// Total number of trades
    pub total_trades: u64,
    /// Number of active pools
    pub active_pools: u32,
    /// Total liquidity
    pub total_liquidity: u64,
    /// Number of active users
    pub active_users: u64,
    /// Protocol revenue
    pub protocol_revenue: u64,
}

/// Protocol Fee Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolFeeStructure {
    /// Trading fee percentage
    pub trading_fee_percentage: u16,
    /// Liquidity provider fee percentage
    pub lp_fee_percentage: u16,
    /// Protocol fee percentage
    pub protocol_fee_percentage: u16,
    /// Fee recipient
    pub fee_recipient: PublicKey,
    /// Fee collection timestamp
    pub last_fee_collection: u64,
    /// Total fees collected
    pub total_fees_collected: u64,
}

/// Swap Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapTransaction {
    /// Transaction ID
    pub id: Hash,
    /// Trading pair
    pub pair: TradingPair,
    /// Input token
    pub token_in: Hash,
    /// Output token
    pub token_out: Hash,
    /// Input amount
    pub amount_in: u64,
    /// Expected output amount
    pub expected_amount_out: u64,
    /// Minimum output amount (slippage protection)
    pub min_amount_out: u64,
    /// Slippage tolerance (basis points)
    pub slippage_tolerance: u16,
    /// Recipient
    pub recipient: PublicKey,
    /// Deadline
    pub deadline: u64,
    /// Transaction fee
    pub fee: u64,
    /// Executor
    pub executor: PublicKey,
    /// Execution timestamp
    pub executed_at: u64,
}

/// Swap Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResult {
    /// Transaction ID
    pub transaction_id: Hash,
    /// Actual output amount
    pub amount_out: u64,
    /// Price impact
    pub price_impact: f64,
    /// Slippage
    pub slippage: f64,
    /// Trading fee paid
    pub trading_fee: u64,
    /// Protocol fee paid
    pub protocol_fee: u64,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error_message: Option<String>,
}

impl DEX {
    /// Create a new DEX
    pub fn new(name: String, trading_fee: u16) -> Self {
        Self {
            name,
            trading_fee,
            pools: HashMap::new(),
            order_books: HashMap::new(),
            stats: DEXStats::new(),
            protocol_fees: ProtocolFeeStructure::default(),
        }
    }

    /// Create a new liquidity pool
    pub fn create_pool(
        &mut self,
        pair: TradingPair,
        initial_token_a: u64,
        initial_token_b: u64,
        creator: PublicKey,
    ) -> Result<Hash, String> {
        // Validate inputs
        if initial_token_a == 0 || initial_token_b == 0 {
            return Err("Initial amounts must be greater than zero".to_string());
        }

        // Generate pool ID
        let pool_id = Hash::sha256(&format!("{}-{}", pair.token_a, pair.token_b).as_bytes());

        // Create liquidity pool
        let mut pool = LiquidityPool {
            id: pool_id,
            pair: pair.clone(),
            token_a_reserves: initial_token_a,
            token_b_reserves: initial_token_b,
            liquidity_tokens: (initial_token_a * initial_token_b).sqrt(),
            fee: self.trading_fee,
            created_at: crate::utils::timestamp(),
            last_updated: crate::utils::timestamp(),
            state: PoolState::Active,
            providers: HashMap::new(),
            metrics: PoolMetrics::new(),
        };

        // Add initial liquidity provider
        let initial_lp = LiquidityProvider {
            provider: creator,
            liquidity_tokens: pool.liquidity_tokens,
            token_a_contribution: initial_token_a,
            token_b_contribution: initial_token_b,
            entry_timestamp: crate::utils::timestamp(),
            last_claim_timestamp: crate::utils::timestamp(),
            unclaimed_fees: 0,
        };

        pool.providers.insert(creator, initial_lp);

        // Add pool to DEX
        self.pools.insert(pool_id, pool);

        // Create order book for the pair
        let order_book = OrderBook::new(pair);
        self.order_books.insert(order_book.pair.clone(), order_book);

        // Update stats
        self.stats.active_pools += 1;
        self.stats.total_liquidity += initial_token_a + initial_token_b;

        Ok(pool_id)
    }

    /// Add liquidity to a pool
    pub fn add_liquidity(
        &mut self,
        pool_id: Hash,
        token_a_amount: u64,
        token_b_amount: u64,
        provider: PublicKey,
    ) -> Result<u64, String> {
        let pool = self.pools.get_mut(&pool_id)
            .ok_or("Pool not found")?;

        if pool.state != PoolState::Active {
            return Err("Pool is not active".to_string());
        }

        // Calculate liquidity tokens to mint
        let liquidity_tokens = if pool.liquidity_tokens == 0 {
            (token_a_amount * token_b_amount).sqrt()
        } else {
            let token_a_ratio = token_a_amount as f64 / pool.token_a_reserves as f64;
            let token_b_ratio = token_b_amount as f64 / pool.token_b_reserves as f64;
            
            // Check if ratios are within acceptable range (0.5% tolerance)
            let ratio_diff = (token_a_ratio - token_b_ratio).abs();
            if ratio_diff > 0.005 {
                return Err("Token ratios don't match pool reserves".to_string());
            }

            (token_a_ratio * pool.liquidity_tokens as f64) as u64
        };

        // Update pool reserves
        pool.token_a_reserves += token_a_amount;
        pool.token_b_reserves += token_b_amount;
        pool.liquidity_tokens += liquidity_tokens;
        pool.last_updated = crate::utils::timestamp();

        // Update or add liquidity provider
        if let Some(existing_lp) = pool.providers.get_mut(&provider) {
            existing_lp.liquidity_tokens += liquidity_tokens;
            existing_lp.token_a_contribution += token_a_amount;
            existing_lp.token_b_contribution += token_b_amount;
        } else {
            let new_lp = LiquidityProvider {
                provider,
                liquidity_tokens,
                token_a_contribution: token_a_amount,
                token_b_contribution: token_b_amount,
                entry_timestamp: crate::utils::timestamp(),
                last_claim_timestamp: crate::utils::timestamp(),
                unclaimed_fees: 0,
            };
            pool.providers.insert(provider, new_lp);
        }

        // Update DEX stats
        self.stats.total_liquidity += token_a_amount + token_b_amount;

        Ok(liquidity_tokens)
    }

    /// Remove liquidity from a pool
    pub fn remove_liquidity(
        &mut self,
        pool_id: Hash,
        liquidity_tokens: u64,
        provider: PublicKey,
    ) -> Result<(u64, u64), String> {
        let pool = self.pools.get_mut(&pool_id)
            .ok_or("Pool not found")?;

        if pool.state != PoolState::Active {
            return Err("Pool is not active".to_string());
        }

        let lp = pool.providers.get_mut(&provider)
            .ok_or("Liquidity provider not found")?;

        if lp.liquidity_tokens < liquidity_tokens {
            return Err("Insufficient liquidity tokens".to_string());
        }

        // Calculate token amounts to return
        let token_a_amount = (liquidity_tokens as f64 / pool.liquidity_tokens as f64 * pool.token_a_reserves as f64) as u64;
        let token_b_amount = (liquidity_tokens as f64 / pool.liquidity_tokens as f64 * pool.token_b_reserves as f64) as u64;

        // Update pool reserves
        pool.token_a_reserves -= token_a_amount;
        pool.token_b_reserves -= token_b_amount;
        pool.liquidity_tokens -= liquidity_tokens;
        pool.last_updated = crate::utils::timestamp();

        // Update liquidity provider
        lp.liquidity_tokens -= liquidity_tokens;
        lp.token_a_contribution -= token_a_amount;
        lp.token_b_contribution -= token_b_amount;

        // If liquidity provider has no more tokens, remove them
        if lp.liquidity_tokens == 0 {
            pool.providers.remove(&provider);
        }

        // Update DEX stats
        self.stats.total_liquidity -= token_a_amount + token_b_amount;

        Ok((token_a_amount, token_b_amount))
    }

    /// Execute a swap
    pub fn swap(&mut self, swap_tx: SwapTransaction) -> Result<SwapResult, String> {
        let pool_id = Hash::sha256(&format!("{}-{}", swap_tx.pair.token_a, swap_tx.pair.token_b).as_bytes());
        
        let pool = self.pools.get_mut(&pool_id)
            .ok_or("Pool not found")?;

        if pool.state != PoolState::Active {
            return Err("Pool is not active".to_string());
        }

        // Calculate output amount using constant product formula
        let (token_in_reserves, token_out_reserves) = if swap_tx.token_in == swap_tx.pair.token_a {
            (pool.token_a_reserves, pool.token_b_reserves)
        } else {
            (pool.token_b_reserves, pool.token_a_reserves)
        };

        // Apply trading fee
        let fee_amount = (swap_tx.amount_in * self.trading_fee as u64) / 10000;
        let amount_in_after_fee = swap_tx.amount_in - fee_amount;

        // Calculate output amount (constant product formula)
        let amount_out = (amount_in_after_fee * token_out_reserves) / (token_in_reserves + amount_in_after_fee);

        // Check slippage tolerance
        let slippage = if swap_tx.expected_amount_out > amount_out {
            ((swap_tx.expected_amount_out - amount_out) as f64 / swap_tx.expected_amount_out as f64) * 100.0
        } else {
            0.0
        };

        if slippage > swap_tx.slippage_tolerance as f64 / 100.0 {
            return Err(format!("Slippage too high: {:.2}%", slippage));
        }

        if amount_out < swap_tx.min_amount_out {
            return Err("Output amount below minimum".to_string());
        }

        // Update pool reserves
        if swap_tx.token_in == swap_tx.pair.token_a {
            pool.token_a_reserves += swap_tx.amount_in;
            pool.token_b_reserves -= amount_out;
        } else {
            pool.token_b_reserves += swap_tx.amount_in;
            pool.token_a_reserves -= amount_out;
        }

        pool.last_updated = crate::utils::timestamp();

        // Update pool metrics
        pool.metrics.total_volume += swap_tx.amount_in;
        pool.metrics.volume_24h += swap_tx.amount_in;
        pool.metrics.transaction_count += 1;
        pool.metrics.total_fees += fee_amount;
        pool.metrics.fees_24h += fee_amount;

        // Update DEX stats
        self.stats.total_volume += swap_tx.amount_in;
        self.stats.volume_24h += swap_tx.amount_in;
        self.stats.total_trades += 1;
        self.stats.protocol_revenue += (fee_amount * self.protocol_fees.protocol_fee_percentage as u64) / 10000;

        // Calculate price impact
        let price_impact = (amount_out as f64 / swap_tx.expected_amount_out as f64 - 1.0) * 100.0;

        Ok(SwapResult {
            transaction_id: swap_tx.id,
            amount_out,
            price_impact,
            slippage,
            trading_fee: fee_amount,
            protocol_fee: (fee_amount * self.protocol_fees.protocol_fee_percentage as u64) / 10000,
            success: true,
            error_message: None,
        })
    }

    /// Get pool information
    pub fn get_pool(&self, pool_id: Hash) -> Option<&LiquidityPool> {
        self.pools.get(&pool_id)
    }

    /// Get all pools
    pub fn get_all_pools(&self) -> Vec<&LiquidityPool> {
        self.pools.values().collect()
    }

    /// Get pool price
    pub fn get_pool_price(&self, pool_id: Hash) -> Result<f64, String> {
        let pool = self.pools.get(&pool_id)
            .ok_or("Pool not found")?;

        Ok(pool.token_b_reserves as f64 / pool.token_a_reserves as f64)
    }

    /// Get total liquidity
    pub fn get_total_liquidity(&self) -> u64 {
        self.stats.total_liquidity
    }

    /// Update pool metrics
    pub fn update_pool_metrics(&mut self, pool_id: Hash) -> Result<(), String> {
        let pool = self.pools.get_mut(&pool_id)
            .ok_or("Pool not found")?;

        // Update 24h metrics
        let current_time = crate::utils::timestamp();
        if current_time - pool.last_updated > 86400 { // 24 hours
            pool.metrics.volume_24h = 0;
            pool.metrics.fees_24h = 0;
        }

        Ok(())
    }
}

impl OrderBook {
    pub fn new(pair: TradingPair) -> Self {
        Self {
            pair,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            order_history: Vec::new(),
            last_trade_price: None,
            volume_24h: 0,
            high_24h: None,
            low_24h: None,
        }
    }

    /// Add an order to the order book
    pub fn add_order(&mut self, order: Order) -> Result<(), String> {
        match order.side {
            OrderSide::Buy => {
                self.buy_orders.insert(order.price, order.amount);
            }
            OrderSide::Sell => {
                self.sell_orders.insert(order.price, order.amount);
            }
        }

        self.order_history.push(order);
        Ok(())
    }

    /// Get best buy price
    pub fn get_best_buy_price(&self) -> Option<u64> {
        self.buy_orders.keys().next_back().copied()
    }

    /// Get best sell price
    pub fn get_best_sell_price(&self) -> Option<u64> {
        self.sell_orders.keys().next().copied()
    }

    /// Get spread
    pub fn get_spread(&self) -> Option<f64> {
        match (self.get_best_buy_price(), self.get_best_sell_price()) {
            (Some(buy_price), Some(sell_price)) => {
                Some((sell_price as f64 - buy_price as f64) / buy_price as f64 * 100.0)
            }
            _ => None,
        }
    }
}

impl LiquidityPool {
    /// Get total liquidity value
    pub fn get_total_liquidity(&self) -> u64 {
        self.token_a_reserves + self.token_b_reserves
    }

    /// Get used liquidity
    pub fn get_used_liquidity(&self) -> u64 {
        // Simplified: assume 50% utilization
        self.get_total_liquidity() / 2
    }

    /// Calculate impermanent loss
    pub fn calculate_impermanent_loss(&self, current_price: f64, entry_price: f64) -> f64 {
        let price_ratio = current_price / entry_price;
        let sqrt_price_ratio = price_ratio.sqrt();
        
        let hodl_value = 1.0 + price_ratio;
        let lp_value = 2.0 * sqrt_price_ratio;
        
        ((hodl_value - lp_value) / hodl_value) * 100.0
    }
}

impl Default for ProtocolFeeStructure {
    fn default() -> Self {
        Self {
            trading_fee_percentage: 30,     // 0.3%
            lp_fee_percentage: 25,          // 0.25%
            protocol_fee_percentage: 5,     // 0.05%
            fee_recipient: PublicKey::random(),
            last_fee_collection: 0,
            total_fees_collected: 0,
        }
    }
}

impl DEXStats {
    pub fn new() -> Self {
        Self {
            total_volume: 0,
            volume_24h: 0,
            total_trades: 0,
            active_pools: 0,
            total_liquidity: 0,
            active_users: 0,
            protocol_revenue: 0,
        }
    }
}

impl PoolMetrics {
    pub fn new() -> Self {
        Self {
            total_volume: 0,
            volume_24h: 0,
            volume_7d: 0,
            total_fees: 0,
            fees_24h: 0,
            transaction_count: 0,
            unique_traders: 0,
            price_change_24h: 0.0,
            price_change_7d: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dex_creation() {
        let dex = DEX::new("TestDEX".to_string(), 30);
        assert_eq!(dex.name, "TestDEX");
        assert_eq!(dex.trading_fee, 30);
        assert_eq!(dex.pools.len(), 0);
    }

    #[test]
    fn test_pool_creation() {
        let mut dex = DEX::new("TestDEX".to_string(), 30);
        let token_a = Hash::new([1u8; 32]);
        let token_b = Hash::new([2u8; 32]);
        let pair = TradingPair {
            token_a,
            token_b,
            symbol: "TOKENA/TOKENB".to_string(),
        };
        let creator = PublicKey::random();

        let result = dex.create_pool(pair, 1000, 2000, creator);
        assert!(result.is_ok());

        let pool_id = result.unwrap();
        assert!(dex.pools.contains_key(&pool_id));
    }

    #[test]
    fn test_liquidity_addition() {
        let mut dex = DEX::new("TestDEX".to_string(), 30);
        let token_a = Hash::new([1u8; 32]);
        let token_b = Hash::new([2u8; 32]);
        let pair = TradingPair {
            token_a,
            token_b,
            symbol: "TOKENA/TOKENB".to_string(),
        };
        let creator = PublicKey::random();

        let pool_id = dex.create_pool(pair, 1000, 2000, creator).unwrap();
        let result = dex.add_liquidity(pool_id, 500, 1000, creator);

        assert!(result.is_ok());
        let liquidity_tokens = result.unwrap();
        assert!(liquidity_tokens > 0);
    }

    #[test]
    fn test_swap_execution() {
        let mut dex = DEX::new("TestDEX".to_string(), 30);
        let token_a = Hash::new([1u8; 32]);
        let token_b = Hash::new([2u8; 32]);
        let pair = TradingPair {
            token_a: token_a,
            token_b: token_b,
            symbol: "TOKENA/TOKENB".to_string(),
        };
        let creator = PublicKey::random();

        let pool_id = dex.create_pool(pair.clone(), 1000, 2000, creator).unwrap();

        let swap_tx = SwapTransaction {
            id: Hash::random(),
            pair: pair.clone(),
            token_in: token_a,
            token_out: token_b,
            amount_in: 100,
            expected_amount_out: 180,
            min_amount_out: 170,
            slippage_tolerance: 500, // 5%
            recipient: creator,
            deadline: crate::utils::timestamp() + 3600,
            fee: 30,
            executor: creator,
            executed_at: crate::utils::timestamp(),
        };

        let result = dex.swap(swap_tx);
        assert!(result.is_ok());

        let swap_result = result.unwrap();
        assert!(swap_result.success);
        assert!(swap_result.amount_out > 0);
    }
}
