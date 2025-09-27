//! Network protocol implementation
//!
//! This module defines the network protocol for Unicoin communication.

use crate::{Result, UnicoinError};
use serde::{Deserialize, Serialize};

/// Network protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolMessage {
    /// New block message
    NewBlock(crate::blockchain::Block),
    /// New transaction message
    NewTransaction(crate::blockchain::Transaction),
    /// Consensus vote message
    ConsensusVote(crate::network::ConsensusVote),
    /// Peer discovery message
    PeerDiscovery(PeerDiscoveryMessage),
    /// Sync request message
    SyncRequest(SyncRequestMessage),
    /// Sync response message
    SyncResponse(SyncResponseMessage),
}

/// Peer discovery message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDiscoveryMessage {
    /// Peer address
    pub address: String,
    /// Peer port
    pub port: u16,
    /// Peer public key
    pub public_key: crate::crypto::PublicKey,
    /// Timestamp
    pub timestamp: u64,
}

/// Sync request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequestMessage {
    /// Starting block height
    pub start_height: u64,
    /// Number of blocks to sync
    pub count: u64,
    /// Requesting peer
    pub requester: crate::crypto::PublicKey,
}

/// Sync response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponseMessage {
    /// Blocks being sent
    pub blocks: Vec<crate::blockchain::Block>,
    /// Response timestamp
    pub timestamp: u64,
}

/// Network protocol implementation
pub struct NetworkProtocol;

impl NetworkProtocol {
    /// Serialize a protocol message
    pub fn serialize(message: &ProtocolMessage) -> Result<Vec<u8>> {
        bincode::serialize(message)
            .map_err(|e| UnicoinError::Network(format!("Serialization failed: {}", e)))
    }

    /// Deserialize a protocol message
    pub fn deserialize(data: &[u8]) -> Result<ProtocolMessage> {
        bincode::deserialize(data)
            .map_err(|e| UnicoinError::Network(format!("Deserialization failed: {}", e)))
    }

    /// Create a peer discovery message
    pub fn create_peer_discovery(
        address: String,
        port: u16,
        public_key: crate::crypto::PublicKey,
    ) -> ProtocolMessage {
        ProtocolMessage::PeerDiscovery(PeerDiscoveryMessage {
            address,
            port,
            public_key,
            timestamp: crate::utils::timestamp(),
        })
    }

    /// Create a sync request message
    pub fn create_sync_request(
        start_height: u64,
        count: u64,
        requester: crate::crypto::PublicKey,
    ) -> ProtocolMessage {
        ProtocolMessage::SyncRequest(SyncRequestMessage {
            start_height,
            count,
            requester,
        })
    }

    /// Create a sync response message
    pub fn create_sync_response(blocks: Vec<crate::blockchain::Block>) -> ProtocolMessage {
        ProtocolMessage::SyncResponse(SyncResponseMessage {
            blocks,
            timestamp: crate::utils::timestamp(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let message = NetworkProtocol::create_peer_discovery(
            "127.0.0.1".to_string(),
            30303,
            crate::crypto::PublicKey::random(),
        );

        let serialized = NetworkProtocol::serialize(&message).unwrap();
        let deserialized = NetworkProtocol::deserialize(&serialized).unwrap();

        match (message, deserialized) {
            (ProtocolMessage::PeerDiscovery(orig), ProtocolMessage::PeerDiscovery(deser)) => {
                assert_eq!(orig.address, deser.address);
                assert_eq!(orig.port, deser.port);
            }
            _ => panic!("Message types don't match"),
        }
    }

    #[test]
    fn test_sync_messages() {
        let requester = crate::crypto::PublicKey::random();
        let sync_request = NetworkProtocol::create_sync_request(100, 50, requester);

        match sync_request {
            ProtocolMessage::SyncRequest(request) => {
                assert_eq!(request.start_height, 100);
                assert_eq!(request.count, 50);
                assert_eq!(request.requester, requester);
            }
            _ => panic!("Expected sync request message"),
        }
    }
}
