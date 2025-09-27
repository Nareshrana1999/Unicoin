//! Non-Fungible Token (NFT) implementation for Unicoin
//! 
//! This module provides comprehensive NFT functionality including
//! ERC-721 and ERC-1155 compatible token standards, metadata handling,
//! and marketplace integration.

pub mod standards;
pub mod metadata;
pub mod marketplace;
pub mod collections;
pub mod royalties;
pub mod auctions;

pub use standards::{ERC721, ERC1155, NFTStandard, TokenStandard, TokenMetadata, TokenTransfer};
pub use metadata::{MetadataManager, MetadataSchema, Attribute, MediaFile, IPFSStorage};
pub use marketplace::{NFTMarketplace, Listing, Sale, Auction, Bid};
pub use collections::{NFTCollection, CollectionMetadata, CollectionStats, CollectionCreator};
pub use royalties::{RoyaltyManager, RoyaltyInfo, RoyaltyDistribution, RoyaltyRecipient};
pub use auctions::{AuctionManager, AuctionType, AuctionState, AuctionResult};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::crypto::{hash::Hash, keys::PublicKey};

/// NFT Manager for Unicoin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTManager {
    /// NFT collections
    pub collections: HashMap<Hash, NFTCollection>,
    /// Individual NFTs
    pub nfts: HashMap<Hash, NFT>,
    /// NFT marketplace
    pub marketplace: NFTMarketplace,
    /// Metadata manager
    pub metadata_manager: MetadataManager,
    /// Royalty manager
    pub royalty_manager: RoyaltyManager,
    /// Auction manager
    pub auction_manager: AuctionManager,
    /// NFT statistics
    pub stats: NFTStats,
    /// Configuration
    pub config: NFTConfig,
}

/// NFT Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFT {
    /// Token ID
    pub id: Hash,
    /// Collection ID
    pub collection_id: Hash,
    /// Token standard (ERC-721, ERC-1155)
    pub standard: TokenStandard,
    /// Owner address
    pub owner: PublicKey,
    /// Creator address
    pub creator: PublicKey,
    /// Token metadata
    pub metadata: TokenMetadata,
    /// Token URI
    pub token_uri: String,
    /// Mint timestamp
    pub minted_at: u64,
    /// Last transfer timestamp
    pub last_transfer: u64,
    /// Transfer count
    pub transfer_count: u64,
    /// Current price (if listed)
    pub current_price: Option<u64>,
    /// Is locked (for auctions, etc.)
    pub is_locked: bool,
    /// Lock reason
    pub lock_reason: Option<String>,
    /// Royalty information
    pub royalty_info: Option<RoyaltyInfo>,
}

/// Token Standard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenStandard {
    /// ERC-721 (Non-Fungible Token)
    ERC721,
    /// ERC-1155 (Multi-Token Standard)
    ERC1155,
    /// ERC-4907 (Rental NFT)
    ERC4907,
    /// Custom Unicoin standard
    UnicoinNFT,
}

/// Token Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// Token name
    pub name: String,
    /// Token description
    pub description: String,
    /// Token image URL
    pub image: String,
    /// Token animation URL
    pub animation_url: Option<String>,
    /// Token attributes
    pub attributes: Vec<Attribute>,
    /// External URL
    pub external_url: Option<String>,
    /// Background color
    pub background_color: Option<String>,
    /// YouTube URL
    pub youtube_url: Option<String>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Trait type
    pub trait_type: String,
    /// Value
    pub value: String,
    /// Display type
    pub display_type: Option<String>,
    /// Max value (for numeric attributes)
    pub max_value: Option<u64>,
}

/// Token Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransfer {
    /// Transfer ID
    pub id: Hash,
    /// Token ID
    pub token_id: Hash,
    /// From address
    pub from: Option<PublicKey>,
    /// To address
    pub to: PublicKey,
    /// Transfer amount (for ERC-1155)
    pub amount: Option<u64>,
    /// Transfer timestamp
    pub timestamp: u64,
    /// Transaction hash
    pub tx_hash: Hash,
    /// Transfer type
    pub transfer_type: TransferType,
}

/// Transfer type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferType {
    /// Mint (creation)
    Mint,
    /// Regular transfer
    Transfer,
    /// Burn (destruction)
    Burn,
    /// Sale transfer
    Sale,
    /// Auction transfer
    Auction,
    /// Gift transfer
    Gift,
}

/// NFT Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTStats {
    /// Total number of NFTs
    pub total_nfts: u64,
    /// Total number of collections
    pub total_collections: u64,
    /// Total trading volume
    pub total_volume: u64,
    /// 24h trading volume
    pub volume_24h: u64,
    /// 7d trading volume
    pub volume_7d: u64,
    /// Total sales count
    pub total_sales: u64,
    /// Average sale price
    pub average_sale_price: u64,
    /// Floor price
    pub floor_price: u64,
    /// Number of unique owners
    pub unique_owners: u64,
    /// Number of unique creators
    pub unique_creators: u64,
    /// Market cap
    pub market_cap: u64,
}

/// NFT Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTConfig {
    /// Default royalty percentage (basis points)
    pub default_royalty: u16,
    /// Maximum royalty percentage (basis points)
    pub max_royalty: u16,
    /// Platform fee percentage (basis points)
    pub platform_fee: u16,
    /// Minimum listing duration (seconds)
    pub min_listing_duration: u64,
    /// Maximum listing duration (seconds)
    pub max_listing_duration: u64,
    /// Enable lazy minting
    pub enable_lazy_minting: bool,
    /// Enable batch operations
    pub enable_batch_operations: bool,
    /// Enable auctions
    pub enable_auctions: bool,
    /// Enable rentals
    pub enable_rentals: bool,
    /// Metadata storage type
    pub metadata_storage: MetadataStorageType,
}

/// Metadata storage types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataStorageType {
    /// IPFS storage
    IPFS,
    /// Arweave storage
    Arweave,
    /// Centralized storage
    Centralized,
    /// On-chain storage
    OnChain,
}

impl NFTManager {
    /// Create a new NFT Manager
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            nfts: HashMap::new(),
            marketplace: NFTMarketplace::new(),
            metadata_manager: MetadataManager::new(),
            royalty_manager: RoyaltyManager::new(),
            auction_manager: AuctionManager::new(),
            stats: NFTStats::new(),
            config: NFTConfig::default(),
        }
    }

    /// Create a new NFT collection
    pub fn create_collection(
        &mut self,
        name: String,
        symbol: String,
        description: String,
        creator: PublicKey,
        standard: TokenStandard,
    ) -> Result<Hash, String> {
        // Validate inputs
        if name.is_empty() || symbol.is_empty() {
            return Err("Collection name and symbol cannot be empty".to_string());
        }

        // Generate collection ID
        let collection_id = Hash::sha256(&format!("{}-{}-{}", name, symbol, creator.to_bytes()).as_bytes());

        // Create collection
        let collection = NFTCollection {
            id: collection_id,
            name,
            symbol,
            description,
            creator,
            standard,
            total_supply: 0,
            current_supply: 0,
            metadata: CollectionMetadata::new(),
            stats: CollectionStats::new(),
            created_at: crate::utils::timestamp(),
            is_active: true,
        };

        // Add collection
        self.collections.insert(collection_id, collection);

        // Update stats
        self.stats.total_collections += 1;

        Ok(collection_id)
    }

    /// Mint a new NFT
    pub fn mint_nft(
        &mut self,
        collection_id: Hash,
        owner: PublicKey,
        metadata: TokenMetadata,
        token_uri: String,
    ) -> Result<Hash, String> {
        // Validate collection exists
        let collection = self.collections.get_mut(&collection_id)
            .ok_or("Collection not found")?;

        if !collection.is_active {
            return Err("Collection is not active".to_string());
        }

        // Generate token ID
        let token_id = Hash::sha256(&format!("{}-{}-{}", collection_id, owner.to_bytes(), crate::utils::timestamp()).as_bytes());

        // Create NFT
        let nft = NFT {
            id: token_id,
            collection_id,
            standard: collection.standard.clone(),
            owner,
            creator: collection.creator,
            metadata,
            token_uri,
            minted_at: crate::utils::timestamp(),
            last_transfer: crate::utils::timestamp(),
            transfer_count: 0,
            current_price: None,
            is_locked: false,
            lock_reason: None,
            royalty_info: Some(RoyaltyInfo {
                recipient: collection.creator,
                percentage: self.config.default_royalty,
            }),
        };

        // Add NFT
        self.nfts.insert(token_id, nft);

        // Update collection stats
        collection.current_supply += 1;
        collection.total_supply += 1;

        // Update global stats
        self.stats.total_nfts += 1;

        // Create transfer record
        let transfer = TokenTransfer {
            id: Hash::random(),
            token_id,
            from: None,
            to: owner,
            amount: Some(1),
            timestamp: crate::utils::timestamp(),
            tx_hash: Hash::random(),
            transfer_type: TransferType::Mint,
        };

        // Store transfer record
        self.record_transfer(transfer);

        Ok(token_id)
    }

    /// Transfer NFT
    pub fn transfer_nft(
        &mut self,
        token_id: Hash,
        from: PublicKey,
        to: PublicKey,
        amount: Option<u64>,
    ) -> Result<(), String> {
        // Validate NFT exists
        let nft = self.nfts.get_mut(&token_id)
            .ok_or("NFT not found")?;

        // Validate ownership
        if nft.owner != from {
            return Err("Not the owner of this NFT".to_string());
        }

        // Check if NFT is locked
        if nft.is_locked {
            return Err("NFT is locked and cannot be transferred".to_string());
        }

        // Update NFT
        nft.owner = to;
        nft.last_transfer = crate::utils::timestamp();
        nft.transfer_count += 1;

        // Create transfer record
        let transfer = TokenTransfer {
            id: Hash::random(),
            token_id,
            from: Some(from),
            to,
            amount,
            timestamp: crate::utils::timestamp(),
            tx_hash: Hash::random(),
            transfer_type: TransferType::Transfer,
        };

        // Store transfer record
        self.record_transfer(transfer);

        Ok(())
    }

    /// List NFT for sale
    pub fn list_nft(
        &mut self,
        token_id: Hash,
        price: u64,
        duration: u64,
        seller: PublicKey,
    ) -> Result<Hash, String> {
        // Validate NFT exists and ownership
        let nft = self.nfts.get(&token_id)
            .ok_or("NFT not found")?;

        if nft.owner != seller {
            return Err("Not the owner of this NFT".to_string());
        }

        if nft.is_locked {
            return Err("NFT is locked and cannot be listed".to_string());
        }

        // Validate listing duration
        if duration < self.config.min_listing_duration || duration > self.config.max_listing_duration {
            return Err("Invalid listing duration".to_string());
        }

        // Create listing
        let listing_id = self.marketplace.create_listing(
            token_id,
            seller,
            price,
            duration,
        )?;

        // Update NFT
        if let Some(nft) = self.nfts.get_mut(&token_id) {
            nft.current_price = Some(price);
            nft.is_locked = true;
            nft.lock_reason = Some("Listed for sale".to_string());
        }

        Ok(listing_id)
    }

    /// Buy NFT
    pub fn buy_nft(
        &mut self,
        token_id: Hash,
        buyer: PublicKey,
        price: u64,
    ) -> Result<(), String> {
        // Validate NFT exists
        let nft = self.nfts.get(&token_id)
            .ok_or("NFT not found")?;

        if !nft.is_locked || nft.current_price.is_none() {
            return Err("NFT is not listed for sale".to_string());
        }

        let listing_price = nft.current_price.unwrap();
        if price < listing_price {
            return Err("Insufficient payment".to_string());
        }

        // Get listing
        let listing = self.marketplace.get_listing_by_token(token_id)
            .ok_or("Listing not found")?;

        // Execute sale
        let sale_result = self.marketplace.execute_sale(
            listing.id,
            buyer,
            price,
        )?;

        // Update NFT ownership
        if let Some(nft) = self.nfts.get_mut(&token_id) {
            nft.owner = buyer;
            nft.current_price = None;
            nft.is_locked = false;
            nft.lock_reason = None;
            nft.last_transfer = crate::utils::timestamp();
            nft.transfer_count += 1;
        }

        // Create transfer record
        let transfer = TokenTransfer {
            id: Hash::random(),
            token_id,
            from: Some(nft.owner),
            to: buyer,
            amount: Some(1),
            timestamp: crate::utils::timestamp(),
            tx_hash: Hash::random(),
            transfer_type: TransferType::Sale,
        };

        self.record_transfer(transfer);

        // Update stats
        self.stats.total_sales += 1;
        self.stats.total_volume += price;
        self.stats.volume_24h += price;

        Ok(())
    }

    /// Burn NFT
    pub fn burn_nft(
        &mut self,
        token_id: Hash,
        owner: PublicKey,
    ) -> Result<(), String> {
        // Validate NFT exists and ownership
        let nft = self.nfts.get(&token_id)
            .ok_or("NFT not found")?;

        if nft.owner != owner {
            return Err("Not the owner of this NFT".to_string());
        }

        // Remove NFT
        self.nfts.remove(&token_id);

        // Update collection stats
        if let Some(collection) = self.collections.get_mut(&nft.collection_id) {
            collection.current_supply -= 1;
        }

        // Update global stats
        self.stats.total_nfts -= 1;

        // Create transfer record
        let transfer = TokenTransfer {
            id: Hash::random(),
            token_id,
            from: Some(owner),
            to: PublicKey::zero(), // Zero address for burned tokens
            amount: Some(1),
            timestamp: crate::utils::timestamp(),
            tx_hash: Hash::random(),
            transfer_type: TransferType::Burn,
        };

        self.record_transfer(transfer);

        Ok(())
    }

    /// Get NFT by ID
    pub fn get_nft(&self, token_id: Hash) -> Option<&NFT> {
        self.nfts.get(&token_id)
    }

    /// Get collection by ID
    pub fn get_collection(&self, collection_id: Hash) -> Option<&NFTCollection> {
        self.collections.get(&collection_id)
    }

    /// Get NFTs by owner
    pub fn get_nfts_by_owner(&self, owner: PublicKey) -> Vec<&NFT> {
        self.nfts.values()
            .filter(|nft| nft.owner == owner)
            .collect()
    }

    /// Get NFTs by collection
    pub fn get_nfts_by_collection(&self, collection_id: Hash) -> Vec<&NFT> {
        self.nfts.values()
            .filter(|nft| nft.collection_id == collection_id)
            .collect()
    }

    /// Get marketplace listings
    pub fn get_listings(&self) -> Vec<&Listing> {
        self.marketplace.get_active_listings()
    }

    /// Get NFT statistics
    pub fn get_stats(&self) -> &NFTStats {
        &self.stats
    }

    /// Record transfer
    fn record_transfer(&mut self, transfer: TokenTransfer) {
        // In a real implementation, this would store the transfer in a database
        // For now, we'll just update the NFT's transfer count
        if let Some(nft) = self.nfts.get_mut(&transfer.token_id) {
            nft.transfer_count += 1;
        }
    }

    /// Update statistics
    pub fn update_stats(&mut self) {
        // Calculate floor price
        let mut floor_price = u64::MAX;
        for nft in self.nfts.values() {
            if let Some(price) = nft.current_price {
                if price < floor_price {
                    floor_price = price;
                }
            }
        }
        if floor_price == u64::MAX {
            floor_price = 0;
        }
        self.stats.floor_price = floor_price;

        // Calculate unique owners
        let mut unique_owners = std::collections::HashSet::new();
        for nft in self.nfts.values() {
            unique_owners.insert(nft.owner);
        }
        self.stats.unique_owners = unique_owners.len() as u64;

        // Calculate unique creators
        let mut unique_creators = std::collections::HashSet::new();
        for collection in self.collections.values() {
            unique_creators.insert(collection.creator);
        }
        self.stats.unique_creators = unique_creators.len() as u64;

        // Calculate average sale price
        if self.stats.total_sales > 0 {
            self.stats.average_sale_price = self.stats.total_volume / self.stats.total_sales;
        }

        // Calculate market cap (simplified)
        self.stats.market_cap = self.stats.total_volume * 10; // Simplified calculation
    }
}

impl Default for NFTConfig {
    fn default() -> Self {
        Self {
            default_royalty: 250, // 2.5%
            max_royalty: 1000, // 10%
            platform_fee: 25, // 0.25%
            min_listing_duration: 3600, // 1 hour
            max_listing_duration: 31536000, // 1 year
            enable_lazy_minting: true,
            enable_batch_operations: true,
            enable_auctions: true,
            enable_rentals: true,
            metadata_storage: MetadataStorageType::IPFS,
        }
    }
}

impl NFTStats {
    pub fn new() -> Self {
        Self {
            total_nfts: 0,
            total_collections: 0,
            total_volume: 0,
            volume_24h: 0,
            volume_7d: 0,
            total_sales: 0,
            average_sale_price: 0,
            floor_price: 0,
            unique_owners: 0,
            unique_creators: 0,
            market_cap: 0,
        }
    }
}

impl Default for NFTManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nft_manager_creation() {
        let manager = NFTManager::new();
        assert_eq!(manager.collections.len(), 0);
        assert_eq!(manager.nfts.len(), 0);
        assert_eq!(manager.stats.total_nfts, 0);
    }

    #[test]
    fn test_collection_creation() {
        let mut manager = NFTManager::new();
        let creator = PublicKey::random();
        
        let result = manager.create_collection(
            "Test Collection".to_string(),
            "TEST".to_string(),
            "A test NFT collection".to_string(),
            creator,
            TokenStandard::ERC721,
        );
        
        assert!(result.is_ok());
        let collection_id = result.unwrap();
        assert!(manager.collections.contains_key(&collection_id));
        assert_eq!(manager.stats.total_collections, 1);
    }

    #[test]
    fn test_nft_minting() {
        let mut manager = NFTManager::new();
        let creator = PublicKey::random();
        let owner = PublicKey::random();
        
        // Create collection first
        let collection_id = manager.create_collection(
            "Test Collection".to_string(),
            "TEST".to_string(),
            "A test NFT collection".to_string(),
            creator,
            TokenStandard::ERC721,
        ).unwrap();
        
        // Create metadata
        let metadata = TokenMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            image: "https://example.com/image.png".to_string(),
            animation_url: None,
            attributes: vec![
                Attribute {
                    trait_type: "Color".to_string(),
                    value: "Blue".to_string(),
                    display_type: None,
                    max_value: None,
                }
            ],
            external_url: None,
            background_color: None,
            youtube_url: None,
            properties: HashMap::new(),
        };
        
        // Mint NFT
        let result = manager.mint_nft(
            collection_id,
            owner,
            metadata,
            "https://example.com/metadata.json".to_string(),
        );
        
        assert!(result.is_ok());
        let token_id = result.unwrap();
        assert!(manager.nfts.contains_key(&token_id));
        assert_eq!(manager.stats.total_nfts, 1);
        
        // Verify NFT properties
        let nft = manager.get_nft(token_id).unwrap();
        assert_eq!(nft.owner, owner);
        assert_eq!(nft.creator, creator);
        assert_eq!(nft.collection_id, collection_id);
    }

    #[test]
    fn test_nft_transfer() {
        let mut manager = NFTManager::new();
        let creator = PublicKey::random();
        let owner = PublicKey::random();
        let new_owner = PublicKey::random();
        
        // Create collection and mint NFT
        let collection_id = manager.create_collection(
            "Test Collection".to_string(),
            "TEST".to_string(),
            "A test NFT collection".to_string(),
            creator,
            TokenStandard::ERC721,
        ).unwrap();
        
        let metadata = TokenMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            image: "https://example.com/image.png".to_string(),
            animation_url: None,
            attributes: Vec::new(),
            external_url: None,
            background_color: None,
            youtube_url: None,
            properties: HashMap::new(),
        };
        
        let token_id = manager.mint_nft(
            collection_id,
            owner,
            metadata,
            "https://example.com/metadata.json".to_string(),
        ).unwrap();
        
        // Transfer NFT
        let result = manager.transfer_nft(token_id, owner, new_owner, Some(1));
        assert!(result.is_ok());
        
        // Verify ownership change
        let nft = manager.get_nft(token_id).unwrap();
        assert_eq!(nft.owner, new_owner);
        assert_eq!(nft.transfer_count, 1);
    }

    #[test]
    fn test_nft_listing() {
        let mut manager = NFTManager::new();
        let creator = PublicKey::random();
        let owner = PublicKey::random();
        
        // Create collection and mint NFT
        let collection_id = manager.create_collection(
            "Test Collection".to_string(),
            "TEST".to_string(),
            "A test NFT collection".to_string(),
            creator,
            TokenStandard::ERC721,
        ).unwrap();
        
        let metadata = TokenMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            image: "https://example.com/image.png".to_string(),
            animation_url: None,
            attributes: Vec::new(),
            external_url: None,
            background_color: None,
            youtube_url: None,
            properties: HashMap::new(),
        };
        
        let token_id = manager.mint_nft(
            collection_id,
            owner,
            metadata,
            "https://example.com/metadata.json".to_string(),
        ).unwrap();
        
        // List NFT
        let result = manager.list_nft(token_id, 1000, 3600, owner);
        assert!(result.is_ok());
        
        // Verify listing
        let nft = manager.get_nft(token_id).unwrap();
        assert!(nft.is_locked);
        assert_eq!(nft.current_price, Some(1000));
        assert_eq!(nft.lock_reason, Some("Listed for sale".to_string()));
    }

    #[test]
    fn test_nft_burning() {
        let mut manager = NFTManager::new();
        let creator = PublicKey::random();
        let owner = PublicKey::random();
        
        // Create collection and mint NFT
        let collection_id = manager.create_collection(
            "Test Collection".to_string(),
            "TEST".to_string(),
            "A test NFT collection".to_string(),
            creator,
            TokenStandard::ERC721,
        ).unwrap();
        
        let metadata = TokenMetadata {
            name: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            image: "https://example.com/image.png".to_string(),
            animation_url: None,
            attributes: Vec::new(),
            external_url: None,
            background_color: None,
            youtube_url: None,
            properties: HashMap::new(),
        };
        
        let token_id = manager.mint_nft(
            collection_id,
            owner,
            metadata,
            "https://example.com/metadata.json".to_string(),
        ).unwrap();
        
        // Burn NFT
        let result = manager.burn_nft(token_id, owner);
        assert!(result.is_ok());
        
        // Verify NFT is removed
        assert!(!manager.nfts.contains_key(&token_id));
        assert_eq!(manager.stats.total_nfts, 0);
    }

    #[test]
    fn test_nft_config() {
        let config = NFTConfig::default();
        assert_eq!(config.default_royalty, 250); // 2.5%
        assert_eq!(config.max_royalty, 1000); // 10%
        assert_eq!(config.platform_fee, 25); // 0.25%
        assert!(config.enable_lazy_minting);
        assert!(config.enable_batch_operations);
        assert!(config.enable_auctions);
        assert_eq!(config.metadata_storage, MetadataStorageType::IPFS);
    }
}
