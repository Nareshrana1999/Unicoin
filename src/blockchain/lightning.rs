use crate::crypto::{hash::Hash, keys::{PublicKey, PrivateKey}, signatures::Signature};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Lightning Network payment channel
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentChannel {
    /// Channel ID
    pub channel_id: Hash,
    /// Channel participants
    pub node_a: PublicKey,
    pub node_b: PublicKey,
    /// Channel capacity (total amount locked in channel)
    pub capacity: u64,
    /// Current balance for node A
    pub balance_a: u64,
    /// Current balance for node B
    pub balance_b: u64,
    /// Channel state (Open, Active, Closing, Closed)
    pub state: ChannelState,
    /// Channel funding transaction
    pub funding_tx_id: Hash,
    /// Channel funding output index
    pub funding_output_index: u32,
    /// Channel closing transaction (if closing)
    pub closing_tx_id: Option<Hash>,
    /// Channel creation timestamp
    pub created_at: u64,
    /// Channel last update timestamp
    pub updated_at: u64,
    /// Commitment transactions
    pub commitments: HashMap<u64, CommitmentTransaction>,
    /// Channel configuration
    pub config: ChannelConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelState {
    Opening,
    Open,
    Active,
    Closing,
    Closed,
    Disputed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Minimum HTLC value
    pub min_htlc_value: u64,
    /// Maximum HTLC value
    pub max_htlc_value: u64,
    /// Channel reserve amount
    pub channel_reserve: u64,
    /// Dust limit
    pub dust_limit: u64,
    /// Maximum number of HTLCs
    pub max_accepted_htlcs: u16,
    /// Channel fee rate
    pub fee_rate: u32,
    /// Time lock delta
    pub time_lock_delta: u32,
}

/// Commitment transaction for payment channels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitmentTransaction {
    /// Commitment number
    pub commitment_number: u64,
    /// Transaction ID
    pub tx_id: Hash,
    /// Transaction outputs
    pub outputs: Vec<ChannelOutput>,
    /// HTLCs in this commitment
    pub htlcs: Vec<HTLC>,
    /// Transaction lock time
    pub lock_time: u32,
    /// Transaction signature
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelOutput {
    /// Output amount
    pub amount: u64,
    /// Output script (channel or HTLC script)
    pub script_pubkey: Vec<u8>,
    /// Whether this is a to_remote output
    pub is_to_remote: bool,
}

/// Hash Time Locked Contract (HTLC)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HTLC {
    /// HTLC ID
    pub htlc_id: u64,
    /// Payment hash (preimage hash)
    pub payment_hash: Hash,
    /// Payment preimage (revealed when payment is made)
    pub payment_preimage: Option<Hash>,
    /// HTLC amount
    pub amount: u64,
    /// HTLC expiry time
    pub expiry_time: u64,
    /// HTLC state
    pub state: HTLCState,
    /// HTLC direction (incoming/outgoing)
    pub direction: HTLCDirection,
    /// HTLC signature
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HTLCState {
    Offered,
    Accepted,
    Settled,
    Failed,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HTLCDirection {
    Incoming,
    Outgoing,
}

/// Lightning Network node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightningNode {
    /// Node public key
    pub public_key: PublicKey,
    /// Node private key (for signing)
    pub private_key: PrivateKey,
    /// Node alias
    pub alias: String,
    /// Node color (for visualization)
    pub color: String,
    /// Node features
    pub features: NodeFeatures,
    /// Active channels
    pub channels: HashMap<Hash, PaymentChannel>,
    /// Node routing table
    pub routing_table: RoutingTable,
    /// Node balance
    pub balance: u64,
    /// Node fees
    pub fees: FeeSchedule,
    /// Node last seen timestamp
    pub last_seen: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeFeatures {
    /// Supports TLV records
    pub supports_tlv: bool,
    /// Supports payment secrets
    pub supports_payment_secrets: bool,
    /// Supports multi-path payments
    pub supports_mpp: bool,
    /// Supports keysend
    pub supports_keysend: bool,
    /// Supports onion messages
    pub supports_onion_messages: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingTable {
    /// Known nodes
    pub nodes: HashMap<PublicKey, NodeInfo>,
    /// Channel announcements
    pub channels: HashMap<Hash, ChannelAnnouncement>,
    /// Node updates
    pub node_updates: HashMap<PublicKey, NodeUpdate>,
    /// Channel updates
    pub channel_updates: HashMap<Hash, ChannelUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node public key
    pub public_key: PublicKey,
    /// Node address
    pub address: String,
    /// Node features
    pub features: NodeFeatures,
    /// Last update timestamp
    pub last_update: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelAnnouncement {
    /// Channel ID
    pub channel_id: Hash,
    /// Node A public key
    pub node_a: PublicKey,
    /// Node B public key
    pub node_b: PublicKey,
    /// Channel capacity
    pub capacity: u64,
    /// Channel funding transaction
    pub funding_tx_id: Hash,
    /// Channel funding output index
    pub funding_output_index: u32,
    /// Announcement signature
    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeUpdate {
    /// Node public key
    pub public_key: PublicKey,
    /// Node timestamp
    pub timestamp: u64,
    /// Node features
    pub features: NodeFeatures,
    /// Update signature
    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelUpdate {
    /// Channel ID
    pub channel_id: Hash,
    /// Channel timestamp
    pub timestamp: u64,
    /// Channel flags
    pub flags: u16,
    /// Time lock delta
    pub time_lock_delta: u16,
    /// HTLC minimum
    pub htlc_minimum_msat: u64,
    /// Fee base
    pub fee_base_msat: u32,
    /// Fee rate
    pub fee_rate: u32,
    /// Update signature
    pub signature: Signature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeeSchedule {
    /// Base fee in millisatoshis
    pub base_fee_msat: u32,
    /// Fee rate in parts per million
    pub fee_rate_ppm: u32,
    /// Minimum fee
    pub min_fee_msat: u32,
    /// Maximum fee
    pub max_fee_msat: u32,
}

/// Lightning Network payment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightningPayment {
    /// Payment hash
    pub payment_hash: Hash,
    /// Payment preimage
    pub payment_preimage: Hash,
    /// Payment amount in millisatoshis
    pub amount_msat: u64,
    /// Payment destination
    pub destination: PublicKey,
    /// Payment route
    pub route: PaymentRoute,
    /// Payment state
    pub state: PaymentState,
    /// Payment creation timestamp
    pub created_at: u64,
    /// Payment completion timestamp
    pub completed_at: Option<u64>,
    /// Payment fees paid
    pub fees_paid_msat: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentRoute {
    /// Route hops
    pub hops: Vec<RouteHop>,
    /// Total route length
    pub total_length: u8,
    /// Total route fees
    pub total_fees_msat: u64,
    /// Total route time lock
    pub total_time_lock: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHop {
    /// Hop node public key
    pub node_id: PublicKey,
    /// Hop channel ID
    pub channel_id: Hash,
    /// Hop fee in millisatoshis
    pub fee_msat: u64,
    /// Hop time lock delta
    pub time_lock_delta: u16,
    /// Hop amount in millisatoshis
    pub amount_msat: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentState {
    Pending,
    InFlight,
    Succeeded,
    Failed,
    Expired,
}

/// Lightning Network implementation
#[derive(Debug, Clone)]
pub struct LightningNetwork {
    /// Network nodes
    nodes: HashMap<PublicKey, LightningNode>,
    /// Network channels
    channels: HashMap<Hash, PaymentChannel>,
    /// Network routing table
    routing_table: RoutingTable,
    /// Network statistics
    stats: NetworkStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Total number of channels
    pub total_channels: usize,
    /// Total network capacity
    pub total_capacity: u64,
    /// Average channel size
    pub average_channel_size: u64,
    /// Network diameter (longest shortest path)
    pub network_diameter: u8,
    /// Average path length
    pub average_path_length: f64,
}

impl LightningNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            channels: HashMap::new(),
            routing_table: RoutingTable {
                nodes: HashMap::new(),
                channels: HashMap::new(),
                node_updates: HashMap::new(),
                channel_updates: HashMap::new(),
            },
            stats: NetworkStats {
                total_nodes: 0,
                total_channels: 0,
                total_capacity: 0,
                average_channel_size: 0,
                network_diameter: 0,
                average_path_length: 0.0,
            },
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: LightningNode) {
        self.nodes.insert(node.public_key, node);
        self.update_stats();
    }

    /// Remove a node from the network
    pub fn remove_node(&mut self, node_id: &PublicKey) {
        self.nodes.remove(node_id);
        self.update_stats();
    }

    /// Open a payment channel between two nodes
    pub fn open_channel(
        &mut self,
        node_a: &PublicKey,
        node_b: &PublicKey,
        capacity: u64,
        config: ChannelConfig,
    ) -> Result<Hash, String> {
        // Check if both nodes exist
        if !self.nodes.contains_key(node_a) || !self.nodes.contains_key(node_b) {
            return Err("One or both nodes do not exist".to_string());
        }

        // Create channel
        let channel_id = Hash::sha256(&format!("{}{}{}", node_a.to_bytes(), node_b.to_bytes(), capacity).as_bytes());
        let funding_tx_id = Hash::new([0u8; 32]); // Simplified
        
        let channel = PaymentChannel {
            channel_id,
            node_a: *node_a,
            node_b: *node_b,
            capacity,
            balance_a: capacity / 2, // Equal initial balance
            balance_b: capacity / 2,
            state: ChannelState::Opening,
            funding_tx_id,
            funding_output_index: 0,
            closing_tx_id: None,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            commitments: HashMap::new(),
            config,
        };

        self.channels.insert(channel_id, channel);
        self.update_stats();

        Ok(channel_id)
    }

    /// Close a payment channel
    pub fn close_channel(&mut self, channel_id: &Hash) -> Result<(), String> {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            channel.state = ChannelState::Closing;
            channel.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            Ok(())
        } else {
            Err("Channel not found".to_string())
        }
    }

    /// Send a payment through the Lightning Network
    pub fn send_payment(
        &mut self,
        sender: &PublicKey,
        recipient: &PublicKey,
        amount_msat: u64,
        payment_hash: Hash,
    ) -> Result<LightningPayment, String> {
        // Find route
        let route = self.find_route(sender, recipient, amount_msat)?;
        
        // Create payment
        let payment_preimage = Hash::new([1u8; 32]); // Simplified
        let payment = LightningPayment {
            payment_hash,
            payment_preimage,
            amount_msat,
            destination: *recipient,
            route: route.clone(),
            state: PaymentState::Pending,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            completed_at: None,
            fees_paid_msat: route.total_fees_msat,
        };

        // Process payment through route
        self.process_payment(&payment)?;

        Ok(payment)
    }

    /// Find a route for payment
    fn find_route(
        &self,
        sender: &PublicKey,
        recipient: &PublicKey,
        amount_msat: u64,
    ) -> Result<PaymentRoute, String> {
        // Simplified routing algorithm
        // In a real implementation, this would use Dijkstra's algorithm
        
        let mut hops = Vec::new();
        let mut total_fees = 0u64;
        let mut total_time_lock = 0u32;

        // Find direct channel if exists
        for (channel_id, channel) in &self.channels {
            if (channel.node_a == *sender && channel.node_b == *recipient) ||
               (channel.node_b == *sender && channel.node_a == *recipient) {
                
                // Check if channel has sufficient capacity
                if amount_msat <= channel.capacity * 1000 {
                    let hop = RouteHop {
                        node_id: *recipient,
                        channel_id: *channel_id,
                        fee_msat: 1000, // Simplified fee
                        time_lock_delta: 144, // 24 hours
                        amount_msat,
                    };
                    
                    hops.push(hop);
                    total_fees += 1000;
                    total_time_lock += 144;
                    
                    return Ok(PaymentRoute {
                        hops,
                        total_length: 1,
                        total_fees_msat: total_fees,
                        total_time_lock,
                    });
                }
            }
        }

        // No direct channel found - would need multi-hop routing
        Err("No route found".to_string())
    }

    /// Process payment through the network
    fn process_payment(&mut self, payment: &LightningPayment) -> Result<(), String> {
        // Process each hop in the route
        for hop in &payment.route.hops {
            // Update channel balances
            self.update_channel_balance(&hop.channel_id, payment.amount_msat, &hop.node_id)?;
        }
        
        Ok(())
    }

    /// Update channel balance after payment
    fn update_channel_balance(
        &mut self,
        channel_id: &Hash,
        amount_msat: u64,
        recipient: &PublicKey,
    ) -> Result<(), String> {
        if let Some(channel) = self.channels.get_mut(channel_id) {
            let amount = amount_msat / 1000; // Convert to satoshis
            
            if channel.node_a == *recipient {
                channel.balance_a += amount;
                channel.balance_b = channel.balance_b.saturating_sub(amount);
            } else if channel.node_b == *recipient {
                channel.balance_b += amount;
                channel.balance_a = channel.balance_a.saturating_sub(amount);
            } else {
                return Err("Invalid recipient for channel".to_string());
            }
            
            channel.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
        
        Ok(())
    }

    /// Get network statistics
    pub fn get_stats(&self) -> &NetworkStats {
        &self.stats
    }

    /// Update network statistics
    fn update_stats(&mut self) {
        self.stats.total_nodes = self.nodes.len();
        self.stats.total_channels = self.channels.len();
        self.stats.total_capacity = self.channels.values().map(|c| c.capacity).sum();
        
        if !self.channels.is_empty() {
            self.stats.average_channel_size = self.stats.total_capacity / self.channels.len() as u64;
        }
    }

    /// Get node information
    pub fn get_node(&self, node_id: &PublicKey) -> Option<&LightningNode> {
        self.nodes.get(node_id)
    }

    /// Get channel information
    pub fn get_channel(&self, channel_id: &Hash) -> Option<&PaymentChannel> {
        self.channels.get(channel_id)
    }

    /// Get all channels for a node
    pub fn get_node_channels(&self, node_id: &PublicKey) -> Vec<&PaymentChannel> {
        self.channels
            .values()
            .filter(|channel| channel.node_a == *node_id || channel.node_b == *node_id)
            .collect()
    }

    /// Calculate channel liquidity
    pub fn get_channel_liquidity(&self, channel_id: &Hash, direction: HTLCDirection) -> Option<u64> {
        if let Some(channel) = self.channels.get(channel_id) {
            match direction {
                HTLCDirection::Incoming => Some(channel.balance_b),
                HTLCDirection::Outgoing => Some(channel.balance_a),
            }
        } else {
            None
        }
    }

    /// Validate HTLC
    pub fn validate_htlc(&self, htlc: &HTLC) -> Result<(), String> {
        // Check expiry time
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time > htlc.expiry_time {
            return Err("HTLC expired".to_string());
        }

        // Check amount
        if htlc.amount == 0 {
            return Err("HTLC amount cannot be zero".to_string());
        }

        // Check payment hash
        if htlc.payment_hash.to_bytes() == [0u8; 32] {
            return Err("Invalid payment hash".to_string());
        }

        Ok(())
    }

    /// Settle HTLC with preimage
    pub fn settle_htlc(&mut self, htlc: &mut HTLC, preimage: Hash) -> Result<(), String> {
        // Verify preimage matches payment hash
        let expected_hash = crate::crypto::hash::sha256(&preimage.to_bytes());
        if expected_hash != htlc.payment_hash {
            return Err("Invalid payment preimage".to_string());
        }

        htlc.payment_preimage = Some(preimage);
        htlc.state = HTLCState::Settled;

        Ok(())
    }

    /// Fail HTLC
    pub fn fail_htlc(&mut self, htlc: &mut HTLC, reason: &str) -> Result<(), String> {
        htlc.state = HTLCState::Failed;
        // Log failure reason
        println!("HTLC failed: {}", reason);
        Ok(())
    }
}

impl Default for LightningNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            min_htlc_value: 1000,
            max_htlc_value: 1000000000,
            channel_reserve: 10000,
            dust_limit: 546,
            max_accepted_htlcs: 483,
            fee_rate: 1000,
            time_lock_delta: 144,
        }
    }
}

impl Default for FeeSchedule {
    fn default() -> Self {
        Self {
            base_fee_msat: 1000,
            fee_rate_ppm: 1,
            min_fee_msat: 1000,
            max_fee_msat: 100000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keys::{PublicKey, PrivateKey};

    #[test]
    fn test_lightning_network_creation() {
        let network = LightningNetwork::new();
        assert_eq!(network.get_stats().total_nodes, 0);
        assert_eq!(network.get_stats().total_channels, 0);
    }

    #[test]
    fn test_payment_channel_creation() {
        let node_a = PublicKey::new([1u8; 33]);
        let node_b = PublicKey::new([2u8; 33]);
        let capacity = 1000000;
        let config = ChannelConfig::default();

        let channel = PaymentChannel {
            channel_id: Hash::new([0u8; 32]),
            node_a,
            node_b,
            capacity,
            balance_a: capacity / 2,
            balance_b: capacity / 2,
            state: ChannelState::Opening,
            funding_tx_id: Hash::new([0u8; 32]),
            funding_output_index: 0,
            closing_tx_id: None,
            created_at: 0,
            updated_at: 0,
            commitments: HashMap::new(),
            config,
        };

        assert_eq!(channel.capacity, capacity);
        assert_eq!(channel.balance_a, capacity / 2);
        assert_eq!(channel.balance_b, capacity / 2);
    }

    #[test]
    fn test_htlc_creation() {
        let payment_hash = Hash::new([1u8; 32]);
        let htlc = HTLC {
            htlc_id: 1,
            payment_hash,
            payment_preimage: None,
            amount: 100000,
            expiry_time: 1000000,
            state: HTLCState::Offered,
            direction: HTLCDirection::Outgoing,
            signature: None,
        };

        assert_eq!(htlc.amount, 100000);
        assert_eq!(htlc.state, HTLCState::Offered);
    }

    #[test]
    fn test_channel_config_default() {
        let config = ChannelConfig::default();
        assert_eq!(config.min_htlc_value, 1000);
        assert_eq!(config.max_htlc_value, 1000000000);
        assert_eq!(config.channel_reserve, 10000);
    }

    #[test]
    fn test_fee_schedule_default() {
        let fees = FeeSchedule::default();
        assert_eq!(fees.base_fee_msat, 1000);
        assert_eq!(fees.fee_rate_ppm, 1);
        assert_eq!(fees.min_fee_msat, 1000);
        assert_eq!(fees.max_fee_msat, 100000);
    }
}
