//! Logging configuration for Unicoin
//!
//! This module defines logging-related configuration options including
//! log levels, output destinations, formatting, and rotation settings.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::Result;

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: LogLevel,
    /// Enable console logging
    pub enable_console_logging: bool,
    /// Enable file logging
    pub enable_file_logging: bool,
    /// Log file path
    pub log_file_path: PathBuf,
    /// Enable structured logging
    pub enable_structured_logging: bool,
    /// Log format
    pub log_format: LogFormat,
    /// Enable log rotation
    pub enable_log_rotation: bool,
    /// Maximum log file size in MB
    pub max_log_file_size_mb: u64,
    /// Maximum number of log files
    pub max_log_files: u32,
    /// Log retention days
    pub log_retention_days: u32,
    /// Enable colored output
    pub enable_colored_output: bool,
    /// Enable timestamps
    pub enable_timestamps: bool,
    /// Enable thread IDs
    pub enable_thread_ids: bool,
    /// Enable module paths
    pub enable_module_paths: bool,
    /// Enable target filtering
    pub enable_target_filtering: bool,
    /// Target filters
    pub target_filters: Vec<String>,
    /// Enable performance logging
    pub enable_performance_logging: bool,
    /// Enable security logging
    pub enable_security_logging: bool,
    /// Enable audit logging
    pub enable_audit_logging: bool,
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

/// Log formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogFormat {
    /// Plain text format
    Plain,
    /// JSON format
    Json,
    /// Compact format
    Compact,
    /// Pretty format
    Pretty,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Compact
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: LogLevel::default(),
            enable_console_logging: true,
            enable_file_logging: true,
            log_file_path: PathBuf::from("logs/unicoin.log"),
            enable_structured_logging: true,
            log_format: LogFormat::default(),
            enable_log_rotation: true,
            max_log_file_size_mb: 100,
            max_log_files: 10,
            log_retention_days: 30,
            enable_colored_output: true,
            enable_timestamps: true,
            enable_thread_ids: false,
            enable_module_paths: true,
            enable_target_filtering: false,
            target_filters: vec![],
            enable_performance_logging: true,
            enable_security_logging: true,
            enable_audit_logging: true,
        }
    }
}

impl LoggingConfig {
    /// Create a new logging configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the logging configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_log_file_size_mb == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_log_file_size_mb must be greater than 0".to_string()));
        }

        if self.max_log_files == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_log_files must be greater than 0".to_string()));
        }

        if self.log_retention_days == 0 {
            return Err(crate::UnicoinError::InvalidConfig("log_retention_days must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Check if logging is enabled
    pub fn logging_enabled(&self) -> bool {
        self.enable_logging
    }

    /// Check if console logging is enabled
    pub fn console_logging_enabled(&self) -> bool {
        self.enable_console_logging && self.enable_logging
    }

    /// Check if file logging is enabled
    pub fn file_logging_enabled(&self) -> bool {
        self.enable_file_logging && self.enable_logging
    }

    /// Check if structured logging is enabled
    pub fn structured_logging_enabled(&self) -> bool {
        self.enable_structured_logging
    }

    /// Check if log rotation is enabled
    pub fn log_rotation_enabled(&self) -> bool {
        self.enable_log_rotation
    }

    /// Check if colored output is enabled
    pub fn colored_output_enabled(&self) -> bool {
        self.enable_colored_output
    }

    /// Check if timestamps are enabled
    pub fn timestamps_enabled(&self) -> bool {
        self.enable_timestamps
    }

    /// Check if thread IDs are enabled
    pub fn thread_ids_enabled(&self) -> bool {
        self.enable_thread_ids
    }

    /// Check if module paths are enabled
    pub fn module_paths_enabled(&self) -> bool {
        self.enable_module_paths
    }

    /// Check if target filtering is enabled
    pub fn target_filtering_enabled(&self) -> bool {
        self.enable_target_filtering
    }

    /// Check if performance logging is enabled
    pub fn performance_logging_enabled(&self) -> bool {
        self.enable_performance_logging
    }

    /// Check if security logging is enabled
    pub fn security_logging_enabled(&self) -> bool {
        self.enable_security_logging
    }

    /// Check if audit logging is enabled
    pub fn audit_logging_enabled(&self) -> bool {
        self.enable_audit_logging
    }

    /// Get the maximum log file size in bytes
    pub fn max_log_file_size_bytes(&self) -> u64 {
        self.max_log_file_size_mb * 1024 * 1024
    }

    /// Add a target filter
    pub fn add_target_filter(&mut self, filter: String) {
        if !self.target_filters.contains(&filter) {
            self.target_filters.push(filter);
        }
    }

    /// Remove a target filter
    pub fn remove_target_filter(&mut self, filter: &str) {
        self.target_filters.retain(|f| f != filter);
    }

    /// Set the log level
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// Set the log format
    pub fn set_log_format(&mut self, format: LogFormat) {
        self.log_format = format;
    }

    /// Set the log file path
    pub fn set_log_file_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.log_file_path = path.into();
    }

    /// Get the log level as string
    pub fn log_level_string(&self) -> &'static str {
        match self.log_level {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }

    /// Get the log format as string
    pub fn log_format_string(&self) -> &'static str {
        match self.log_format {
            LogFormat::Plain => "plain",
            LogFormat::Json => "json",
            LogFormat::Compact => "compact",
            LogFormat::Pretty => "pretty",
        }
    }
}
