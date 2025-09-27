//! Network module - P2P networking for Unicoin
//!
//! This module provides the networking layer for Unicoin nodes to communicate.

use crate::{
    blockchain::{Block, Transaction},
    consensus::ConsensusEngine,
    Result, UnicoinError,
};
use libp2p::{
    identity, Multiaddr, PeerId, Swarm, SwarmBuilder,
    NetworkBehaviour, NetworkInfo, ConnectionInfo,
    ping::{Ping, PingConfig},
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    kad::{Kademlia, KademliaEvent, Record, record::store::MemoryStore},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn, error};

pub mod protocol;
pub mod peer;
pub mod discovery;
pub mod routing;

pub use protocol::{NetworkProtocol, ProtocolMessage};
pub use peer::{Peer, PeerInfo, PeerManager};
pub use discovery::PeerDiscovery;
pub use routing::MessageRouting;

/// Network node for Unicoin
pub struct NetworkNode {
    /// P2P swarm
    swarm: Swarm<UnicoinBehaviour>,
    /// Peer manager
    peer_manager: PeerManager,
    /// Message sender for consensus
    consensus_sender: mpsc::UnboundedSender<ProtocolMessage>,
    /// Message receiver from consensus
    consensus_receiver: mpsc::UnboundedReceiver<ProtocolMessage>,
    /// Network statistics
    stats: RwLock<NetworkStats>,
}

/// Network behavior combining multiple protocols
#[derive(NetworkBehaviour)]
struct UnicoinBehaviour {
    /// Ping protocol for connectivity testing
    ping: Ping,
    /// Floodsub for message broadcasting
    floodsub: Floodsub,
    /// mDNS for local peer discovery
    mdns: Mdns,
    /// Kademlia for peer routing and storage
    kademlia: Kademlia<MemoryStore>,
}

/// Network statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    /// Number of connected peers
    pub connected_peers: usize,
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Network latency (average)
    pub average_latency: f64,
    /// Bandwidth usage (bytes)
    pub bandwidth_usage: u64,
}

impl NetworkNode {
    /// Create a new network node
    pub fn new(consensus: ConsensusEngine) -> Result<Self> {
        // Create local identity
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        // Create ping protocol
        let ping = Ping::new(PingConfig::new().with_keep_alive(true));

        // Create floodsub protocol
        let mut floodsub = Floodsub::new(local_peer_id);
        floodsub.subscribe("unicoin-blocks".into());
        floodsub.subscribe("unicoin-transactions".into());
        floodsub.subscribe("unicoin-consensus".into());

        // Create mDNS protocol
        let mdns = Mdns::new(MdnsConfig::default())?;

        // Create Kademlia protocol
        let store = MemoryStore::new(local_peer_id);
        let mut kademlia = Kademlia::new(local_peer_id, store);
        kademlia.set_mode(Some(libp2p::kad::Mode::Server));

        // Create behavior
        let behaviour = UnicoinBehaviour {
            ping,
            floodsub,
            mdns,
            kademlia,
        };

        // Create swarm
        let swarm = SwarmBuilder::new(
            libp2p::development_transport(local_key)?,
            behaviour,
            local_peer_id,
        )
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

        // Create channels for consensus communication
        let (consensus_sender, consensus_receiver) = mpsc::unbounded_channel();

        Ok(Self {
            swarm,
            peer_manager: PeerManager::new(),
            consensus_sender,
            consensus_receiver,
            stats: RwLock::new(NetworkStats::default()),
        })
    }

    /// Start the network node
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Unicoin network node");

        // Listen on all interfaces
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Start network loop
        self.network_loop().await
    }

    /// Main network loop
    async fn network_loop(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                // Handle swarm events
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await?;
                }
                
                // Handle consensus messages
                message = self.consensus_receiver.recv() => {
                    if let Some(msg) = message {
                        self.handle_consensus_message(msg).await?;
                    }
                }
            }
        }
    }

    /// Handle swarm events
    async fn handle_swarm_event(&mut self, event: libp2p::swarm::SwarmEvent<libp2p::swarm::behaviour::FromSwarm<libp2p::swarm::behaviour::ConnectionEstablished, libp2p::swarm::behaviour::ConnectionClosed, libp2p::swarm::behaviour::OutgoingConnectionError, libp2p::swarm::behaviour::IncomingConnectionError, libp2p::swarm::behaviour::Dialing, libp2p::swarm::behaviour::ListenFailure, libp2p::swarm::behaviour::NewListener, libp2p::swarm::behaviour::ExpiredListenAddr, libp2p::swarm::behaviour::ListenerClosed, libp2p::swarm::behaviour::ListenerError, libp2p::swarm::behaviour::IncomingConnection, libp2p::swarm::behaviour::OutgoingConnection, libp2p::swarm::behaviour::BannedPeer, libp2p::swarm::behaviour::UnbannedPeer>, libp2p::swarm::behaviour::BehaviourEvent<UnicoinBehaviour>>) -> Result<()> {
        match event {
            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("Connected to peer {}", peer_id);
                self.peer_manager.add_peer(peer_id).await;
                self.update_connected_peers().await;
            }
            libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("Disconnected from peer {}", peer_id);
                self.peer_manager.remove_peer(peer_id).await;
                self.update_connected_peers().await;
            }
            libp2p::swarm::SwarmEvent::Behaviour(event) => {
                self.handle_behaviour_event(event).await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle behavior events
    async fn handle_behaviour_event(&mut self, event: libp2p::swarm::behaviour::BehaviourEvent<UnicoinBehaviour>) -> Result<()> {
        match event {
            libp2p::swarm::behaviour::BehaviourEvent::Ping(ping_event) => {
                self.handle_ping_event(ping_event).await?;
            }
            libp2p::swarm::behaviour::BehaviourEvent::Floodsub(floodsub_event) => {
                self.handle_floodsub_event(floodsub_event).await?;
            }
            libp2p::swarm::behaviour::BehaviourEvent::Mdns(mdns_event) => {
                self.handle_mdns_event(mdns_event).await?;
            }
            libp2p::swarm::behaviour::BehaviourEvent::Kademlia(kademlia_event) => {
                self.handle_kademlia_event(kademlia_event).await?;
            }
        }
        Ok(())
    }

    /// Handle ping events
    async fn handle_ping_event(&mut self, _event: libp2p::ping::PingEvent) -> Result<()> {
        // Update latency statistics
        // Implementation would measure actual ping times
        Ok(())
    }

    /// Handle floodsub events
    async fn handle_floodsub_event(&mut self, event: FloodsubEvent) -> Result<()> {
        match event {
            FloodsubEvent::Message(message) => {
                self.handle_floodsub_message(message).await?;
            }
            FloodsubEvent::Subscribed { peer_id, topic } => {
                info!("Peer {} subscribed to topic {}", peer_id, topic);
            }
            FloodsubEvent::Unsubscribed { peer_id, topic } => {
                info!("Peer {} unsubscribed from topic {}", peer_id, topic);
            }
        }
        Ok(())
    }

    /// Handle floodsub messages
    async fn handle_floodsub_message(&mut self, message: libp2p::floodsub::FloodsubMessage) -> Result<()> {
        let topic = message.topic();
        let data = message.data();

        match topic.as_str() {
            "unicoin-blocks" => {
                self.handle_block_message(data).await?;
            }
            "unicoin-transactions" => {
                self.handle_transaction_message(data).await?;
            }
            "unicoin-consensus" => {
                self.handle_consensus_message_data(data).await?;
            }
            _ => {
                warn!("Unknown topic: {}", topic);
            }
        }

        self.increment_messages_received().await;
        Ok(())
    }

    /// Handle block messages
    async fn handle_block_message(&mut self, data: &[u8]) -> Result<()> {
        match bincode::deserialize::<Block>(data) {
            Ok(block) => {
                info!("Received block at height {}", block.header.height);
                // Forward to consensus engine
                self.consensus_sender.send(ProtocolMessage::NewBlock(block))?;
            }
            Err(e) => {
                warn!("Failed to deserialize block: {}", e);
            }
        }
        Ok(())
    }

    /// Handle transaction messages
    async fn handle_transaction_message(&mut self, data: &[u8]) -> Result<()> {
        match bincode::deserialize::<Transaction>(data) {
            Ok(transaction) => {
                info!("Received transaction: {}", transaction.hash());
                // Forward to consensus engine
                self.consensus_sender.send(ProtocolMessage::NewTransaction(transaction))?;
            }
            Err(e) => {
                warn!("Failed to deserialize transaction: {}", e);
            }
        }
        Ok(())
    }

    /// Handle consensus messages
    async fn handle_consensus_message(&mut self, message: ProtocolMessage) -> Result<()> {
        match message {
            ProtocolMessage::NewBlock(block) => {
                self.broadcast_block(&block).await?;
            }
            ProtocolMessage::NewTransaction(transaction) => {
                self.broadcast_transaction(&transaction).await?;
            }
            ProtocolMessage::ConsensusVote(vote) => {
                self.broadcast_consensus_vote(&vote).await?;
            }
        }
        Ok(())
    }

    /// Handle consensus message data
    async fn handle_consensus_message_data(&mut self, data: &[u8]) -> Result<()> {
        match bincode::deserialize::<ProtocolMessage>(data) {
            Ok(message) => {
                self.handle_consensus_message(message).await?;
            }
            Err(e) => {
                warn!("Failed to deserialize consensus message: {}", e);
            }
        }
        Ok(())
    }

    /// Handle mDNS events
    async fn handle_mdns_event(&mut self, event: MdnsEvent) -> Result<()> {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, multiaddr) in list {
                    info!("Discovered peer {} at {}", peer_id, multiaddr);
                    self.swarm.behaviour_mut().mdns.add_peer_to_discovery(&peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, multiaddr) in list {
                    info!("Expired peer {} at {}", peer_id, multiaddr);
                }
            }
        }
        Ok(())
    }

    /// Handle Kademlia events
    async fn handle_kademlia_event(&mut self, event: KademliaEvent) -> Result<()> {
        match event {
            KademliaEvent::RoutingUpdated { peer, .. } => {
                info!("Kademlia routing updated for peer {}", peer);
            }
            KademliaEvent::RoutablePeer { peer, .. } => {
                info!("Found routable peer {}", peer);
                self.swarm.behaviour_mut().kademlia.add_address(&peer, "/ip4/127.0.0.1/tcp/8080".parse()?);
            }
            _ => {}
        }
        Ok(())
    }

    /// Broadcast a block to all connected peers
    async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let data = bincode::serialize(block)?;
        self.swarm.behaviour_mut().floodsub.publish("unicoin-blocks".into(), data);
        self.increment_messages_sent().await;
        Ok(())
    }

    /// Broadcast a transaction to all connected peers
    async fn broadcast_transaction(&mut self, transaction: &Transaction) -> Result<()> {
        let data = bincode::serialize(transaction)?;
        self.swarm.behaviour_mut().floodsub.publish("unicoin-transactions".into(), data);
        self.increment_messages_sent().await;
        Ok(())
    }

    /// Broadcast a consensus vote
    async fn broadcast_consensus_vote(&mut self, vote: &ConsensusVote) -> Result<()> {
        let message = ProtocolMessage::ConsensusVote(vote.clone());
        let data = bincode::serialize(&message)?;
        self.swarm.behaviour_mut().floodsub.publish("unicoin-consensus".into(), data);
        self.increment_messages_sent().await;
        Ok(())
    }

    /// Update connected peers count
    async fn update_connected_peers(&self) {
        let mut stats = self.stats.write().await;
        stats.connected_peers = self.peer_manager.get_peer_count().await;
    }

    /// Increment messages sent counter
    async fn increment_messages_sent(&self) {
        let mut stats = self.stats.write().await;
        stats.messages_sent += 1;
    }

    /// Increment messages received counter
    async fn increment_messages_received(&self) {
        let mut stats = self.stats.write().await;
        stats.messages_received += 1;
    }

    /// Get network statistics
    pub async fn get_stats(&self) -> NetworkStats {
        self.stats.read().await.clone()
    }

    /// Get peer information
    pub async fn get_peer_info(&self, peer_id: PeerId) -> Option<PeerInfo> {
        self.peer_manager.get_peer_info(peer_id).await
    }

    /// Connect to a peer
    pub fn connect_to_peer(&mut self, peer_id: PeerId, address: Multiaddr) -> Result<()> {
        self.swarm.behaviour_mut().kademlia.add_address(&peer_id, address.clone());
        self.swarm.dial((peer_id, address))?;
        Ok(())
    }
}

/// Consensus vote message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    /// Block hash being voted on
    pub block_hash: crate::crypto::Hash,
    /// Vote type
    pub vote_type: VoteType,
    /// Validator public key
    pub validator: crate::crypto::PublicKey,
    /// Signature of the vote
    pub signature: crate::crypto::Signature,
}

/// Types of consensus votes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    /// Approve the block
    Approve,
    /// Reject the block
    Reject,
    /// Abstain from voting
    Abstain,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Blockchain;

    #[tokio::test]
    async fn test_network_node_creation() {
        let blockchain = Blockchain::new().unwrap();
        let consensus = ConsensusEngine::new(blockchain).unwrap();
        let network = NetworkNode::new(consensus).unwrap();
        
        let stats = network.get_stats().await;
        assert_eq!(stats.connected_peers, 0);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
    }

    #[test]
    fn test_consensus_vote() {
        let vote = ConsensusVote {
            block_hash: crate::crypto::Hash::random(),
            vote_type: VoteType::Approve,
            validator: crate::crypto::PublicKey::random(),
            signature: crate::crypto::Signature::new_secp256k1(vec![1u8; 64]),
        };
        
        assert_eq!(vote.vote_type, VoteType::Approve);
    }
}
