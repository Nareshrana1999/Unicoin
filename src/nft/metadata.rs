//! NFT metadata management for Unicoin

use serde::{Deserialize, Serialize};
use crate::crypto::{hash::Hash, keys::PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataManager {
    pub name: String,
    pub version: String,
}

impl MetadataManager {
    pub fn new() -> Self {
        Self {
            name: "Unicoin Metadata Manager".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSchema {
    pub schema_type: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub file_type: String,
    pub url: String,
    pub hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSStorage {
    pub gateway: String,
    pub pin_service: String,
}
