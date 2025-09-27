//! Security configuration for Unicoin
//!
//! This module defines security-related configuration options including
//! encryption, authentication, access control, and security policies.

use serde::{Deserialize, Serialize};
use crate::Result;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable encryption
    pub enable_encryption: bool,
    /// Encryption algorithm
    pub encryption_algorithm: EncryptionAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Enable authentication
    pub enable_authentication: bool,
    /// Authentication method
    pub authentication_method: AuthenticationMethod,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Account lockout duration in seconds
    pub lockout_duration: u64,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Rate limit requests per minute
    pub rate_limit_per_minute: u32,
    /// Rate limit burst size
    pub rate_limit_burst: u32,
    /// Enable IP whitelisting
    pub enable_ip_whitelist: bool,
    /// Whitelisted IP addresses
    pub whitelisted_ips: Vec<String>,
    /// Enable IP blacklisting
    pub enable_ip_blacklist: bool,
    /// Blacklisted IP addresses
    pub blacklisted_ips: Vec<String>,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention days
    pub audit_log_retention_days: u32,
    /// Enable intrusion detection
    pub enable_intrusion_detection: bool,
    /// Intrusion detection sensitivity
    pub intrusion_detection_sensitivity: u8,
    /// Enable malware scanning
    pub enable_malware_scanning: bool,
    /// Enable DDoS protection
    pub enable_ddos_protection: bool,
    /// DDoS protection threshold
    pub ddos_protection_threshold: u32,
    /// Enable firewall
    pub enable_firewall: bool,
    /// Firewall rules
    pub firewall_rules: Vec<FirewallRule>,
    /// Enable secure headers
    pub enable_secure_headers: bool,
    /// Enable CSRF protection
    pub enable_csrf_protection: bool,
    /// Enable XSS protection
    pub enable_xss_protection: bool,
    /// Enable content security policy
    pub enable_csp: bool,
    /// Content security policy rules
    pub csp_rules: Vec<String>,
}

/// Encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM
    Aes256Gcm,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
    /// Unicoin's proprietary encryption
    UniEnc,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::UniEnc
    }
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthenticationMethod {
    /// Public key authentication
    PublicKey,
    /// Password authentication
    Password,
    /// Multi-factor authentication
    MultiFactor,
    /// Biometric authentication
    Biometric,
    /// Hardware token
    HardwareToken,
}

impl Default for AuthenticationMethod {
    fn default() -> Self {
        Self::PublicKey
    }
}

/// Firewall rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    /// Rule name
    pub name: String,
    /// Rule action (allow/deny)
    pub action: FirewallAction,
    /// Source IP or CIDR
    pub source: String,
    /// Destination IP or CIDR
    pub destination: String,
    /// Port or port range
    pub port: String,
    /// Protocol (TCP/UDP/ICMP)
    pub protocol: String,
    /// Rule priority
    pub priority: u32,
}

/// Firewall actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FirewallAction {
    /// Allow traffic
    Allow,
    /// Deny traffic
    Deny,
    /// Log and allow
    LogAndAllow,
    /// Log and deny
    LogAndDeny,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            encryption_algorithm: EncryptionAlgorithm::default(),
            key_size: 256,
            enable_authentication: true,
            authentication_method: AuthenticationMethod::default(),
            session_timeout: 3600, // 1 hour
            max_login_attempts: 5,
            lockout_duration: 300, // 5 minutes
            enable_rate_limiting: true,
            rate_limit_per_minute: 1000,
            rate_limit_burst: 100,
            enable_ip_whitelist: false,
            whitelisted_ips: vec![],
            enable_ip_blacklist: true,
            blacklisted_ips: vec![
                "127.0.0.1".to_string(), // Localhost
                "::1".to_string(), // IPv6 localhost
            ],
            enable_audit_logging: true,
            audit_log_retention_days: 90,
            enable_intrusion_detection: true,
            intrusion_detection_sensitivity: 5,
            enable_malware_scanning: true,
            enable_ddos_protection: true,
            ddos_protection_threshold: 1000,
            enable_firewall: true,
            firewall_rules: vec![],
            enable_secure_headers: true,
            enable_csrf_protection: true,
            enable_xss_protection: true,
            enable_csp: true,
            csp_rules: vec![
                "default-src 'self'".to_string(),
                "script-src 'self' 'unsafe-inline'".to_string(),
                "style-src 'self' 'unsafe-inline'".to_string(),
                "img-src 'self' data: https:".to_string(),
            ],
        }
    }
}

impl SecurityConfig {
    /// Create a new security configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the security configuration
    pub fn validate(&self) -> Result<()> {
        if self.key_size == 0 {
            return Err(crate::UnicoinError::InvalidConfig("key_size must be greater than 0".to_string()));
        }

        if self.session_timeout == 0 {
            return Err(crate::UnicoinError::InvalidConfig("session_timeout must be greater than 0".to_string()));
        }

        if self.max_login_attempts == 0 {
            return Err(crate::UnicoinError::InvalidConfig("max_login_attempts must be greater than 0".to_string()));
        }

        if self.lockout_duration == 0 {
            return Err(crate::UnicoinError::InvalidConfig("lockout_duration must be greater than 0".to_string()));
        }

        if self.rate_limit_per_minute == 0 {
            return Err(crate::UnicoinError::InvalidConfig("rate_limit_per_minute must be greater than 0".to_string()));
        }

        if self.rate_limit_burst == 0 {
            return Err(crate::UnicoinError::InvalidConfig("rate_limit_burst must be greater than 0".to_string()));
        }

        if self.audit_log_retention_days == 0 {
            return Err(crate::UnicoinError::InvalidConfig("audit_log_retention_days must be greater than 0".to_string()));
        }

        if self.intrusion_detection_sensitivity == 0 || self.intrusion_detection_sensitivity > 10 {
            return Err(crate::UnicoinError::InvalidConfig("intrusion_detection_sensitivity must be between 1 and 10".to_string()));
        }

        if self.ddos_protection_threshold == 0 {
            return Err(crate::UnicoinError::InvalidConfig("ddos_protection_threshold must be greater than 0".to_string()));
        }

        Ok(())
    }

    /// Check if encryption is enabled
    pub fn encryption_enabled(&self) -> bool {
        self.enable_encryption
    }

    /// Check if authentication is enabled
    pub fn authentication_enabled(&self) -> bool {
        self.enable_authentication
    }

    /// Check if rate limiting is enabled
    pub fn rate_limiting_enabled(&self) -> bool {
        self.enable_rate_limiting
    }

    /// Check if IP whitelisting is enabled
    pub fn ip_whitelist_enabled(&self) -> bool {
        self.enable_ip_whitelist
    }

    /// Check if IP blacklisting is enabled
    pub fn ip_blacklist_enabled(&self) -> bool {
        self.enable_ip_blacklist
    }

    /// Check if audit logging is enabled
    pub fn audit_logging_enabled(&self) -> bool {
        self.enable_audit_logging
    }

    /// Check if intrusion detection is enabled
    pub fn intrusion_detection_enabled(&self) -> bool {
        self.enable_intrusion_detection
    }

    /// Check if malware scanning is enabled
    pub fn malware_scanning_enabled(&self) -> bool {
        self.enable_malware_scanning
    }

    /// Check if DDoS protection is enabled
    pub fn ddos_protection_enabled(&self) -> bool {
        self.enable_ddos_protection
    }

    /// Check if firewall is enabled
    pub fn firewall_enabled(&self) -> bool {
        self.enable_firewall
    }

    /// Check if secure headers are enabled
    pub fn secure_headers_enabled(&self) -> bool {
        self.enable_secure_headers
    }

    /// Check if CSRF protection is enabled
    pub fn csrf_protection_enabled(&self) -> bool {
        self.enable_csrf_protection
    }

    /// Check if XSS protection is enabled
    pub fn xss_protection_enabled(&self) -> bool {
        self.enable_xss_protection
    }

    /// Check if content security policy is enabled
    pub fn csp_enabled(&self) -> bool {
        self.enable_csp
    }

    /// Add a whitelisted IP address
    pub fn add_whitelisted_ip(&mut self, ip: String) {
        if !self.whitelisted_ips.contains(&ip) {
            self.whitelisted_ips.push(ip);
        }
    }

    /// Remove a whitelisted IP address
    pub fn remove_whitelisted_ip(&mut self, ip: &str) {
        self.whitelisted_ips.retain(|i| i != ip);
    }

    /// Add a blacklisted IP address
    pub fn add_blacklisted_ip(&mut self, ip: String) {
        if !self.blacklisted_ips.contains(&ip) {
            self.blacklisted_ips.push(ip);
        }
    }

    /// Remove a blacklisted IP address
    pub fn remove_blacklisted_ip(&mut self, ip: &str) {
        self.blacklisted_ips.retain(|i| i != ip);
    }

    /// Add a firewall rule
    pub fn add_firewall_rule(&mut self, rule: FirewallRule) {
        self.firewall_rules.push(rule);
    }

    /// Remove a firewall rule by name
    pub fn remove_firewall_rule(&mut self, name: &str) {
        self.firewall_rules.retain(|r| r.name != name);
    }

    /// Add a CSP rule
    pub fn add_csp_rule(&mut self, rule: String) {
        if !self.csp_rules.contains(&rule) {
            self.csp_rules.push(rule);
        }
    }

    /// Remove a CSP rule
    pub fn remove_csp_rule(&mut self, rule: &str) {
        self.csp_rules.retain(|r| r != rule);
    }
}
