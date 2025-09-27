//! Performance configuration for Unicoin
//!
//! This module defines performance-related configuration options including
//! parallel processing, caching, memory management, and optimization settings.

use serde::{Deserialize, Serialize};
use crate::Result;

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum transactions per second
    pub max_tps: u32,
    /// Enable parallel processing
    pub enable_parallel_processing: bool,
    /// Number of worker threads
    pub worker_threads: usize,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable transaction batching
    pub enable_transaction_batching: bool,
    /// Batch size for transactions
    pub transaction_batch_size: usize,
    /// Enable block pre-validation
    pub enable_block_prevalidation: bool,
    /// Enable transaction pre-validation
    pub enable_transaction_prevalidation: bool,
    /// Enable memory pooling
    pub enable_memory_pooling: bool,
    /// Memory pool size in MB
    pub memory_pool_size_mb: usize,
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Connection pool size
    pub connection_pool_size: usize,
    /// Enable database optimization
    pub enable_db_optimization: bool,
    /// Database cache size in MB
    pub db_cache_size_mb: usize,
    /// Enable compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u8,
    /// Enable async I/O
    pub enable_async_io: bool,
    /// I/O buffer size in KB
    pub io_buffer_size_kb: usize,
    /// Enable prefetching
    pub enable_prefetching: bool,
    /// Prefetch size in KB
    pub prefetch_size_kb: usize,
    /// Enable JIT compilation
    pub enable_jit: bool,
    /// Enable CPU optimization
    pub enable_cpu_optimization: bool,
    /// Enable memory optimization
    pub enable_memory_optimization: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_tps: 10000,
            enable_parallel_processing: true,
            worker_threads: num_cpus::get(),
            cache_size_mb: 512,
            enable_transaction_batching: true,
            transaction_batch_size: 1000,
            enable_block_prevalidation: true,
            enable_transaction_prevalidation: true,
            enable_memory_pooling: true,
            memory_pool_size_mb: 256,
            enable_connection_pooling: true,
            connection_pool_size: 100,
            enable_db_optimization: true,
            db_cache_size_mb: 256,
            enable_compression: true,
            compression_level: 6,
            enable_async_io: true,
            io_buffer_size_kb: 64,
            enable_prefetching: true,
            prefetch_size_kb: 32,
            enable_jit: true,
            enable_cpu_optimization: true,
            enable_memory_optimization: true,
        }
    }
}

impl PerformanceConfig {
    /// Create a new performance configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the performance configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_tps == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_tps must be greater than 0".to_string()));
        }

        if self.worker_threads == 0 {
            return Err(crate::UnicoinError::InvalidConfig("worker_threads must be greater than 0".to_string()));
        }

        if self.cache_size_mb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("cache_size_mb must be greater than 0".to_string()));
        }

        if self.transaction_batch_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("transaction_batch_size must be greater than 0".to_string()));
        }

        if self.memory_pool_size_mb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("memory_pool_size_mb must be greater than 0".to_string()));
        }

        if self.connection_pool_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("connection_pool_size must be greater than 0".to_string()));
        }

        if self.db_cache_size_mb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("db_cache_size_mb must be greater than 0".to_string()));
        }

        if self.compression_level == 0 || self.compression_level > 9 {
            return Err(crate::UnicoinError::InvalidConfig("compression_level must be between 1 and 9".to_string()));
        }

        if self.io_buffer_size_kb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("io_buffer_size_kb must be greater than 0".to_string()));
        }

        if self.prefetch_size_kb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("prefetch_size_kb must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Check if parallel processing is enabled
    pub fn parallel_processing_enabled(&self) -> bool {
        self.enable_parallel_processing
    }

    /// Check if transaction batching is enabled
    pub fn transaction_batching_enabled(&self) -> bool {
        self.enable_transaction_batching
    }

    /// Check if block pre-validation is enabled
    pub fn block_prevalidation_enabled(&self) -> bool {
        self.enable_block_prevalidation
    }

    /// Check if transaction pre-validation is enabled
    pub fn transaction_prevalidation_enabled(&self) -> bool {
        self.enable_transaction_prevalidation
    }

    /// Check if memory pooling is enabled
    pub fn memory_pooling_enabled(&self) -> bool {
        self.enable_memory_pooling
    }

    /// Check if connection pooling is enabled
    pub fn connection_pooling_enabled(&self) -> bool {
        self.enable_connection_pooling
    }

    /// Check if database optimization is enabled
    pub fn db_optimization_enabled(&self) -> bool {
        self.enable_db_optimization
    }

    /// Check if compression is enabled
    pub fn compression_enabled(&self) -> bool {
        self.enable_compression
    }

    /// Check if async I/O is enabled
    pub fn async_io_enabled(&self) -> bool {
        self.enable_async_io
    }

    /// Check if prefetching is enabled
    pub fn prefetching_enabled(&self) -> bool {
        self.enable_prefetching
    }

    /// Check if JIT compilation is enabled
    pub fn jit_enabled(&self) -> bool {
        self.enable_jit
    }

    /// Check if CPU optimization is enabled
    pub fn cpu_optimization_enabled(&self) -> bool {
        self.enable_cpu_optimization
    }

    /// Check if memory optimization is enabled
    pub fn memory_optimization_enabled(&self) -> bool {
        self.enable_memory_optimization
    }

    /// Get the cache size in bytes
    pub fn cache_size_bytes(&self) -> usize {
        self.cache_size_mb * 1024 * 1024
    }

    /// Get the memory pool size in bytes
    pub fn memory_pool_size_bytes(&self) -> usize {
        self.memory_pool_size_mb * 1024 * 1024
    }

    /// Get the database cache size in bytes
    pub fn db_cache_size_bytes(&self) -> usize {
        self.db_cache_size_mb * 1024 * 1024
    }

    /// Get the I/O buffer size in bytes
    pub fn io_buffer_size_bytes(&self) -> usize {
        self.io_buffer_size_kb * 1024
    }

    /// Get the prefetch size in bytes
    pub fn prefetch_size_bytes(&self) -> usize {
        self.prefetch_size_kb * 1024
    }

    /// Get the optimal number of worker threads
    pub fn optimal_worker_threads(&self) -> usize {
        if self.enable_parallel_processing {
            self.worker_threads
        } else {
            1
        }
    }
}
