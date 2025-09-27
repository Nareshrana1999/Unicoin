//! API configuration for Unicoin
//!
//! This module defines API-related configuration options including
//! REST API settings, WebSocket configuration, and API endpoints.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use crate::Result;

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Enable REST API
    pub enable_rest_api: bool,
    /// REST API listening address
    pub rest_api_address: IpAddr,
    /// REST API listening port
    pub rest_api_port: u16,
    /// Enable WebSocket API
    pub enable_websocket_api: bool,
    /// WebSocket API listening address
    pub websocket_api_address: IpAddr,
    /// WebSocket API listening port
    pub websocket_api_port: u16,
    /// Enable GraphQL API
    pub enable_graphql_api: bool,
    /// GraphQL API listening address
    pub graphql_api_address: IpAddr,
    /// GraphQL API listening port
    pub graphql_api_port: u16,
    /// API version
    pub api_version: String,
    /// Enable API documentation
    pub enable_api_docs: bool,
    /// API documentation path
    pub api_docs_path: String,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_allowed_origins: Vec<String>,
    /// Enable API authentication
    pub enable_api_auth: bool,
    /// API authentication token
    pub api_auth_token: Option<String>,
    /// Enable API rate limiting
    pub enable_api_rate_limiting: bool,
    /// API rate limit per minute
    pub api_rate_limit_per_minute: u32,
    /// Enable API caching
    pub enable_api_caching: bool,
    /// API cache TTL in seconds
    pub api_cache_ttl: u64,
    /// Enable API compression
    pub enable_api_compression: bool,
    /// Enable API logging
    pub enable_api_logging: bool,
    /// API log level
    pub api_log_level: LogLevel,
    /// Enable API metrics
    pub enable_api_metrics: bool,
    /// Enable API health checks
    pub enable_api_health_checks: bool,
    /// API health check interval in seconds
    pub api_health_check_interval: u64,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    /// Error level
    Error,
    /// Warning level
    Warning,
    /// Info level
    Info,
    /// Debug level
    Debug,
    /// Trace level
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enable_rest_api: true,
            rest_api_address: "127.0.0.1".parse().unwrap(),
            rest_api_port: 8080,
            enable_websocket_api: true,
            websocket_api_address: "127.0.0.1".parse().unwrap(),
            websocket_api_port: 8081,
            enable_graphql_api: true,
            graphql_api_address: "127.0.0.1".parse().unwrap(),
            graphql_api_port: 8082,
            api_version: "v1".to_string(),
            enable_api_docs: true,
            api_docs_path: "/docs".to_string(),
            enable_cors: true,
            cors_allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:3001".to_string(),
            ],
            enable_api_auth: false,
            api_auth_token: None,
            enable_api_rate_limiting: true,
            api_rate_limit_per_minute: 1000,
            enable_api_caching: true,
            api_cache_ttl: 300, // 5 minutes
            enable_api_compression: true,
            enable_api_logging: true,
            api_log_level: LogLevel::default(),
            enable_api_metrics: true,
            enable_api_health_checks: true,
            api_health_check_interval: 30,
        }
    }
}

impl ApiConfig {
    /// Create a new API configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the API configuration
    pub fn validate(&self) -> Result<()> {
        if self.rest_api_port == 0 {
            return Err(crate::UnicoinError::InvalidConfig("rest_api_port must be greater than 0".to_string()));
        }

        if self.websocket_api_port == 0 {
            return Err(crate::UnicoinError::InvalidConfig("websocket_api_port must be greater than 0".to_string()));
        }

        if self.graphql_api_port == 0 {
            return Err(crate::UnicoinError::InvalidConfig("graphql_api_port must be greater than 0".to_string()));
        }

        if self.api_rate_limit_per_minute == 0 {
            return Err(crate::UnicoinError::InvalidConfig("api_rate_limit_per_minute must be greater than 0".to_string()));
        }

        if self.api_cache_ttl == 0 {
            return Err(crate::UnicoinError::InvalidConfig("api_cache_ttl must be greater than 0".to_string()));
        }

        if self.api_health_check_interval == 0 {
            return Err(crate::UnicoinError::InvalidConfig("api_health_check_interval must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Check if REST API is enabled
    pub fn rest_api_enabled(&self) -> bool {
        self.enable_rest_api
    }

    /// Check if WebSocket API is enabled
    pub fn websocket_api_enabled(&self) -> bool {
        self.enable_websocket_api
    }

    /// Check if GraphQL API is enabled
    pub fn graphql_api_enabled(&self) -> bool {
        self.enable_graphql_api
    }

    /// Check if API documentation is enabled
    pub fn api_docs_enabled(&self) -> bool {
        self.enable_api_docs
    }

    /// Check if CORS is enabled
    pub fn cors_enabled(&self) -> bool {
        self.enable_cors
    }

    /// Check if API authentication is enabled
    pub fn api_auth_enabled(&self) -> bool {
        self.enable_api_auth
    }

    /// Check if API rate limiting is enabled
    pub fn api_rate_limiting_enabled(&self) -> bool {
        self.enable_api_rate_limiting
    }

    /// Check if API caching is enabled
    pub fn api_caching_enabled(&self) -> bool {
        self.enable_api_caching
    }

    /// Check if API compression is enabled
    pub fn api_compression_enabled(&self) -> bool {
        self.enable_api_compression
    }

    /// Check if API logging is enabled
    pub fn api_logging_enabled(&self) -> bool {
        self.enable_api_logging
    }

    /// Check if API metrics are enabled
    pub fn api_metrics_enabled(&self) -> bool {
        self.enable_api_metrics
    }

    /// Check if API health checks are enabled
    pub fn api_health_checks_enabled(&self) -> bool {
        self.enable_api_health_checks
    }

    /// Get the REST API socket address
    pub fn rest_api_addr(&self) -> SocketAddr {
        SocketAddr::new(self.rest_api_address, self.rest_api_port)
    }

    /// Get the WebSocket API socket address
    pub fn websocket_api_addr(&self) -> SocketAddr {
        SocketAddr::new(self.websocket_api_address, self.websocket_api_port)
    }

    /// Get the GraphQL API socket address
    pub fn graphql_api_addr(&self) -> SocketAddr {
        SocketAddr::new(self.graphql_api_address, self.graphql_api_port)
    }

    /// Add a CORS allowed origin
    pub fn add_cors_origin(&mut self, origin: String) {
        if !self.cors_allowed_origins.contains(&origin) {
            self.cors_allowed_origins.push(origin);
        }
    }

    /// Remove a CORS allowed origin
    pub fn remove_cors_origin(&mut self, origin: &str) {
        self.cors_allowed_origins.retain(|o| o != origin);
    }

    /// Set the API authentication token
    pub fn set_api_auth_token(&mut self, token: String) {
        self.api_auth_token = Some(token);
    }

    /// Remove the API authentication token
    pub fn remove_api_auth_token(&mut self) {
        self.api_auth_token = None;
    }

    /// Get the API base URL
    pub fn get_api_base_url(&self) -> String {
        format!("http://{}:{}", self.rest_api_address, self.rest_api_port)
    }

    /// Get the WebSocket URL
    pub fn get_websocket_url(&self) -> String {
        format!("ws://{}:{}", self.websocket_api_address, self.websocket_api_port)
    }

    /// Get the GraphQL URL
    pub fn get_graphql_url(&self) -> String {
        format!("http://{}:{}", self.graphql_api_address, self.graphql_api_port)
    }
}
