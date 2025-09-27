//! API module for Unicoin
//!
//! This module provides REST API, WebSocket API, and GraphQL API endpoints
//! for interacting with Unicoin nodes and blockchain data.

use crate::{config::ApiConfig, Result};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
};

pub mod rest;
pub mod websocket;
pub mod graphql;
pub mod handlers;
pub mod middleware;
pub mod types;

pub use rest::RestApiServer;
pub use websocket::WebSocketServer;
pub use graphql::GraphQLServer;
pub use handlers::ApiHandlers;
pub use middleware::ApiMiddleware;
pub use types::*;

/// Main API server that manages all API endpoints
pub struct ApiServer {
    config: ApiConfig,
    handlers: ApiHandlers,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(config: ApiConfig) -> Self {
        let handlers = ApiHandlers::new();
        Self { config, handlers }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<()> {
        if self.config.rest_api_enabled() {
            let rest_server = RestApiServer::new(
                self.config.clone(),
                self.handlers.clone(),
            );
            rest_server.start().await?;
        }

        if self.config.websocket_api_enabled() {
            let websocket_server = WebSocketServer::new(
                self.config.clone(),
                self.handlers.clone(),
            );
            websocket_server.start().await?;
        }

        if self.config.graphql_api_enabled() {
            let graphql_server = GraphQLServer::new(
                self.config.clone(),
                self.handlers.clone(),
            );
            graphql_server.start().await?;
        }

        Ok(())
    }

    /// Stop the API server
    pub async fn stop(&self) -> Result<()> {
        // Implementation for graceful shutdown
        Ok(())
    }
}

/// API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: u64,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    /// Create an error response
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    /// Create a success response with message
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message),
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
}

impl ApiError {
    /// Create a new API error
    pub fn new(code: u16, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
        }
    }

    /// Create a new API error with details
    pub fn with_details(code: u16, message: String, details: String) -> Self {
        Self {
            code,
            message,
            details: Some(details),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            offset: None,
        }
    }
}

impl PaginationParams {
    /// Get the effective limit
    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(20).min(100) // Max 100 items per page
    }

    /// Get the effective offset
    pub fn get_offset(&self) -> u32 {
        if let Some(offset) = self.offset {
            offset
        } else if let Some(page) = self.page {
            (page - 1) * self.get_limit()
        } else {
            0
        }
    }
}

/// Paginated response
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, total: u64, page: u32, limit: u32) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as u32;
        Self {
            items,
            total,
            page,
            limit,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
    pub timestamp: u64,
    pub uptime: u64,
    pub services: HashMap<String, String>,
}

impl HealthCheck {
    /// Create a new health check response
    pub fn new(services: HashMap<String, String>) -> Self {
        Self {
            status: "healthy".to_string(),
            version: crate::VERSION.to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            uptime: 0, // This should be calculated from start time
            services,
        }
    }
}

/// API metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub requests_per_second: f64,
    pub active_connections: u32,
    pub uptime_seconds: u64,
}

impl ApiMetrics {
    /// Create new API metrics
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            requests_per_second: 0.0,
            active_connections: 0,
            uptime_seconds: 0,
        }
    }
}

/// API configuration for runtime
#[derive(Debug, Clone)]
pub struct ApiRuntimeConfig {
    pub cors_origins: Vec<String>,
    pub rate_limit_per_minute: u32,
    pub max_request_size: usize,
    pub timeout_seconds: u64,
}

impl Default for ApiRuntimeConfig {
    fn default() -> Self {
        Self {
            cors_origins: vec!["*".to_string()],
            rate_limit_per_minute: 1000,
            max_request_size: 10 * 1024 * 1024, // 10MB
            timeout_seconds: 30,
        }
    }
}
