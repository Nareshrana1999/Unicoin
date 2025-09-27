//! REST API server for Unicoin
//!
//! This module provides a comprehensive REST API server with endpoints
//! for blockchain data, wallet operations, network management, and more.

use crate::{
    api::{
        ApiConfig, ApiHandlers, ApiResponse, HealthCheck,
        types::*,
    },
    Result,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
};

/// REST API server
pub struct RestApiServer {
    config: ApiConfig,
    handlers: ApiHandlers,
}

impl RestApiServer {
    /// Create a new REST API server
    pub fn new(config: ApiConfig, handlers: ApiHandlers) -> Self {
        Self { config, handlers }
    }

    /// Start the REST API server
    pub async fn start(&self) -> Result<()> {
        let app = self.create_app();
        let addr = self.config.rest_api_addr();
        
        tracing::info!("Starting REST API server on {}", addr);
        
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }

    /// Create the Axum application with all routes
    fn create_app(&self) -> Router {
        let handlers = Arc::new(self.handlers.clone());

        Router::new()
            // Health and system endpoints
            .route("/health", get(Self::health_check_handler))
            .route("/metrics", get(Self::metrics_handler))
            .route("/info", get(Self::info_handler))
            
            // Blockchain endpoints
            .route("/blockchain/info", get(Self::blockchain_info_handler))
            .route("/blockchain/block/hash/:hash", get(Self::block_by_hash_handler))
            .route("/blockchain/block/height/:height", get(Self::block_by_height_handler))
            .route("/blockchain/blocks", get(Self::latest_blocks_handler))
            .route("/blockchain/transaction/:hash", get(Self::transaction_by_hash_handler))
            .route("/blockchain/transaction/broadcast", post(Self::broadcast_transaction_handler))
            
            // Network endpoints
            .route("/network/info", get(Self::network_info_handler))
            .route("/network/peers", get(Self::peers_handler))
            .route("/network/node", get(Self::node_info_handler))
            
            // Wallet endpoints
            .route("/wallet/info", get(Self::wallet_info_handler))
            .route("/wallet/address/generate", post(Self::generate_address_handler))
            .route("/wallet/address/:address/balance", get(Self::address_balance_handler))
            .route("/wallet/balances", post(Self::query_balances_handler))
            
            // Mempool endpoints
            .route("/mempool/info", get(Self::mempool_info_handler))
            .route("/mempool/transactions", get(Self::pending_transactions_handler))
            
            // Mining endpoints
            .route("/mining/info", get(Self::mining_info_handler))
            
            // Smart Contract endpoints
            .route("/contract/:address", get(Self::smart_contract_handler))
            
            // DeFi endpoints
            .route("/defi/protocols", get(Self::defi_protocols_handler))
            
            // NFT endpoints
            .route("/nft/:contract_address/:token_id", get(Self::nft_handler))
            .route("/nft/owner/:owner", get(Self::nfts_by_owner_handler))
            
            // Search endpoint
            .route("/search", get(Self::search_handler))
            
            // Add middleware
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CompressionLayer::new())
                    .layer(CorsLayer::permissive())
            )
            .with_state(handlers)
    }

    // Handler implementations

    async fn health_check_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<HealthCheck>>, StatusCode> {
        handlers.health_check().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn metrics_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<ApiMetrics>>, StatusCode> {
        handlers.get_api_metrics().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<NodeInfo>>, StatusCode> {
        handlers.get_node_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn blockchain_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<NetworkInfo>>, StatusCode> {
        handlers.get_blockchain_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn block_by_hash_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(hash): Path<String>,
    ) -> Result<Json<ApiResponse<BlockInfo>>, StatusCode> {
        handlers.get_block_by_hash(hash).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn block_by_height_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(height): Path<u64>,
    ) -> Result<Json<ApiResponse<BlockInfo>>, StatusCode> {
        handlers.get_block_by_height(height).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn latest_blocks_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Query(params): Query<PaginationParams>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<BlockInfo>>>, StatusCode> {
        handlers.get_latest_blocks(params).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn transaction_by_hash_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(hash): Path<String>,
    ) -> Result<Json<ApiResponse<TransactionInfo>>, StatusCode> {
        handlers.get_transaction_by_hash(hash).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn broadcast_transaction_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Json(request): Json<BroadcastTransactionRequest>,
    ) -> Result<Json<ApiResponse<BroadcastTransactionResponse>>, StatusCode> {
        handlers.broadcast_transaction(request).await.map_err(|_| StatusCode::BAD_REQUEST)
    }

    async fn network_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<NetworkInfo>>, StatusCode> {
        handlers.get_network_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn peers_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Query(params): Query<PaginationParams>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<PeerInfo>>>, StatusCode> {
        handlers.get_peers(params).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn node_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<NodeInfo>>, StatusCode> {
        handlers.get_node_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn wallet_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<WalletInfo>>, StatusCode> {
        handlers.get_wallet_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn generate_address_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Json(request): Json<GenerateAddressRequest>,
    ) -> Result<Json<ApiResponse<GenerateAddressResponse>>, StatusCode> {
        handlers.generate_address(request).await.map_err(|_| StatusCode::BAD_REQUEST)
    }

    async fn address_balance_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(address): Path<String>,
    ) -> Result<Json<ApiResponse<AddressInfo>>, StatusCode> {
        handlers.get_address_balance(address).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn query_balances_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Json(request): Json<BalanceQueryRequest>,
    ) -> Result<Json<ApiResponse<BalanceQueryResponse>>, StatusCode> {
        handlers.query_balances(request).await.map_err(|_| StatusCode::BAD_REQUEST)
    }

    async fn mempool_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<MempoolInfo>>, StatusCode> {
        handlers.get_mempool_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn pending_transactions_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Query(params): Query<PaginationParams>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<TransactionInfo>>>, StatusCode> {
        handlers.get_pending_transactions(params).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn mining_info_handler(
        State(handlers): State<Arc<ApiHandlers>>,
    ) -> Result<Json<ApiResponse<MiningInfo>>, StatusCode> {
        handlers.get_mining_info().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn smart_contract_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(address): Path<String>,
    ) -> Result<Json<ApiResponse<SmartContractInfo>>, StatusCode> {
        handlers.get_smart_contract(address).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn defi_protocols_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Query(params): Query<PaginationParams>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<DeFiProtocolInfo>>>, StatusCode> {
        handlers.get_defi_protocols(params).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn nft_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path((contract_address, token_id)): Path<(String, String)>,
    ) -> Result<Json<ApiResponse<NFTInfo>>, StatusCode> {
        handlers.get_nft(token_id, contract_address).await.map_err(|_| StatusCode::NOT_FOUND)
    }

    async fn nfts_by_owner_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Path(owner): Path<String>,
        Query(params): Query<PaginationParams>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<NFTInfo>>>, StatusCode> {
        handlers.get_nfts_by_owner(owner, params).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn search_handler(
        State(handlers): State<Arc<ApiHandlers>>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<ApiResponse<SearchResult>>, StatusCode> {
        handlers.search(params).await.map_err(|_| StatusCode::BAD_REQUEST)
    }
}
