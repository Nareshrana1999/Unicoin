//! Unicoin Core Integration Layer
//! 
//! This module provides the integration layer that connects all
//! Unicoin components and manages their interactions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Unicoin Core Integration Manager
#[derive(Debug, Clone)]
pub struct UnicoinCore {
    /// Component integrations
    pub integrations: Arc<RwLock<HashMap<String, ComponentIntegration>>>,
    /// Event bus for component communication
    pub event_bus: Arc<RwLock<EventBus>>,
    /// Integration configuration
    pub config: IntegrationConfig,
    /// Integration metrics
    pub metrics: Arc<RwLock<IntegrationMetrics>>,
}

/// Component Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentIntegration {
    /// Component name
    pub component_name: String,
    /// Integration type
    pub integration_type: IntegrationType,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Configuration
    pub config: HashMap<String, String>,
    /// Status
    pub status: IntegrationStatus,
    /// Last update
    pub last_update: u64,
}

/// Integration Type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationType {
    /// Direct integration
    Direct,
    /// Event-driven integration
    EventDriven,
    /// API integration
    ApiIntegration,
    /// Database integration
    DatabaseIntegration,
    /// Message queue integration
    MessageQueue,
}

/// Integration Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {
    /// Not connected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Error
    Error(String),
}

/// Event Bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBus {
    /// Event handlers
    pub handlers: HashMap<String, Vec<EventHandler>>,
    /// Event queue
    pub event_queue: Vec<Event>,
    /// Event history
    pub event_history: Vec<Event>,
    /// Max history size
    pub max_history_size: usize,
}

/// Event Handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandler {
    /// Handler ID
    pub id: String,
    /// Component name
    pub component: String,
    /// Event type
    pub event_type: String,
    /// Handler function (placeholder)
    pub handler_function: String,
    /// Priority
    pub priority: u8,
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Source component
    pub source: String,
    /// Target component
    pub target: Option<String>,
    /// Event data
    pub data: HashMap<String, String>,
    /// Timestamp
    pub timestamp: u64,
    /// Priority
    pub priority: EventPriority,
}

/// Event Priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Integration Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable event bus
    pub enable_event_bus: bool,
    /// Max event queue size
    pub max_event_queue_size: usize,
    /// Event processing timeout
    pub event_processing_timeout: u64,
    /// Enable event persistence
    pub enable_event_persistence: bool,
    /// Integration retry attempts
    pub retry_attempts: u32,
    /// Integration timeout
    pub integration_timeout: u64,
}

/// Integration Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetrics {
    /// Total events processed
    pub events_processed: u64,
    /// Events per second
    pub events_per_second: f64,
    /// Average processing time
    pub avg_processing_time: f64,
    /// Integration success rate
    pub success_rate: f64,
    /// Active integrations
    pub active_integrations: u32,
    /// Failed integrations
    pub failed_integrations: u32,
}

impl UnicoinCore {
    /// Create a new Unicoin Core
    pub fn new() -> Self {
        Self {
            integrations: Arc::new(RwLock::new(HashMap::new())),
            event_bus: Arc::new(RwLock::new(EventBus::new())),
            config: IntegrationConfig::default(),
            metrics: Arc::new(RwLock::new(IntegrationMetrics::new())),
        }
    }

    /// Register a component integration
    pub async fn register_integration(
        &self,
        component_name: String,
        integration_type: IntegrationType,
        dependencies: Vec<String>,
        config: HashMap<String, String>,
    ) -> Result<(), String> {
        let integration = ComponentIntegration {
            component_name: component_name.clone(),
            integration_type,
            dependencies,
            config,
            status: IntegrationStatus::Disconnected,
            last_update: crate::utils::timestamp(),
        };

        let mut integrations = self.integrations.write().await;
        integrations.insert(component_name, integration);

        Ok(())
    }

    /// Connect a component integration
    pub async fn connect_integration(&self, component_name: &str) -> Result<(), String> {
        let mut integrations = self.integrations.write().await;
        
        if let Some(integration) = integrations.get_mut(component_name) {
            integration.status = IntegrationStatus::Connecting;
            integration.last_update = crate::utils::timestamp();
            
            // Simulate connection process
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            integration.status = IntegrationStatus::Connected;
            integration.last_update = crate::utils::timestamp();
            
            // Update metrics
            let mut metrics = self.metrics.write().await;
            metrics.active_integrations += 1;
            
            Ok(())
        } else {
            Err(format!("Integration for component '{}' not found", component_name))
        }
    }

    /// Disconnect a component integration
    pub async fn disconnect_integration(&self, component_name: &str) -> Result<(), String> {
        let mut integrations = self.integrations.write().await;
        
        if let Some(integration) = integrations.get_mut(component_name) {
            integration.status = IntegrationStatus::Disconnected;
            integration.last_update = crate::utils::timestamp();
            
            // Update metrics
            let mut metrics = self.metrics.write().await;
            metrics.active_integrations = metrics.active_integrations.saturating_sub(1);
            
            Ok(())
        } else {
            Err(format!("Integration for component '{}' not found", component_name))
        }
    }

    /// Register an event handler
    pub async fn register_event_handler(
        &self,
        handler_id: String,
        component: String,
        event_type: String,
        handler_function: String,
        priority: u8,
    ) -> Result<(), String> {
        let handler = EventHandler {
            id: handler_id,
            component,
            event_type: event_type.clone(),
            handler_function,
            priority,
        };

        let mut event_bus = self.event_bus.write().await;
        event_bus.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);

        Ok(())
    }

    /// Publish an event
    pub async fn publish_event(
        &self,
        event_type: String,
        source: String,
        target: Option<String>,
        data: HashMap<String, String>,
        priority: EventPriority,
    ) -> Result<(), String> {
        let event = Event {
            id: Self::generate_event_id(),
            event_type: event_type.clone(),
            source,
            target,
            data,
            timestamp: crate::utils::timestamp(),
            priority,
        };

        let mut event_bus = self.event_bus.write().await;
        
        // Add to event queue
        event_bus.event_queue.push(event.clone());
        
        // Add to event history
        event_bus.event_history.push(event.clone());
        
        // Limit history size
        if event_bus.event_history.len() > event_bus.max_history_size {
            event_bus.event_history.remove(0);
        }

        // Process event
        self.process_event(event).await?;

        Ok(())
    }

    /// Process an event
    async fn process_event(&self, event: Event) -> Result<(), String> {
        let start_time = std::time::Instant::now();
        
        let event_bus = self.event_bus.read().await;
        let handlers = event_bus.handlers.get(&event.event_type);
        
        if let Some(handlers) = handlers {
            // Sort handlers by priority
            let mut sorted_handlers = handlers.clone();
            sorted_handlers.sort_by(|a, b| b.priority.cmp(&a.priority));
            
            // Execute handlers
            for handler in sorted_handlers {
                self.execute_handler(&handler, &event).await?;
            }
        }

        // Update metrics
        let processing_time = start_time.elapsed().as_millis() as f64;
        let mut metrics = self.metrics.write().await;
        metrics.events_processed += 1;
        metrics.avg_processing_time = 
            (metrics.avg_processing_time * (metrics.events_processed - 1) as f64 + processing_time) / metrics.events_processed as f64;

        Ok(())
    }

    /// Execute an event handler
    async fn execute_handler(&self, handler: &EventHandler, event: &Event) -> Result<(), String> {
        // Simulate handler execution
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        // In a real implementation, this would execute the actual handler function
        println!("Executing handler {} for event {} from component {}", 
                 handler.id, event.event_type, handler.component);
        
        Ok(())
    }

    /// Generate event ID
    fn generate_event_id() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        let timestamp = crate::utils::timestamp();
        let random = rand::random::<u64>();
        (timestamp ^ random).hash(&mut hasher);
        
        format!("event_{}", hasher.finish())
    }

    /// Get integration status
    pub async fn get_integration_status(&self, component_name: &str) -> Result<IntegrationStatus, String> {
        let integrations = self.integrations.read().await;
        
        if let Some(integration) = integrations.get(component_name) {
            Ok(integration.status.clone())
        } else {
            Err(format!("Integration for component '{}' not found", component_name))
        }
    }

    /// Get all integrations
    pub async fn get_all_integrations(&self) -> Result<HashMap<String, ComponentIntegration>, String> {
        let integrations = self.integrations.read().await;
        Ok(integrations.clone())
    }

    /// Get integration metrics
    pub async fn get_integration_metrics(&self) -> Result<IntegrationMetrics, String> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Get event history
    pub async fn get_event_history(&self, limit: Option<usize>) -> Result<Vec<Event>, String> {
        let event_bus = self.event_bus.read().await;
        
        let mut history = event_bus.event_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            history.truncate(limit);
        }
        
        Ok(history)
    }

    /// Clear event queue
    pub async fn clear_event_queue(&self) -> Result<(), String> {
        let mut event_bus = self.event_bus.write().await;
        event_bus.event_queue.clear();
        Ok(())
    }

    /// Get event queue size
    pub async fn get_event_queue_size(&self) -> Result<usize, String> {
        let event_bus = self.event_bus.read().await;
        Ok(event_bus.event_queue.len())
    }

    /// Health check for all integrations
    pub async fn health_check(&self) -> Result<HashMap<String, bool>, String> {
        let integrations = self.integrations.read().await;
        let mut health_status = HashMap::new();
        
        for (component_name, integration) in integrations.iter() {
            let is_healthy = match integration.status {
                IntegrationStatus::Connected => true,
                _ => false,
            };
            health_status.insert(component_name.clone(), is_healthy);
        }
        
        Ok(health_status)
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            event_queue: Vec::new(),
            event_history: Vec::new(),
            max_history_size: 10000,
        }
    }
}

impl IntegrationMetrics {
    pub fn new() -> Self {
        Self {
            events_processed: 0,
            events_per_second: 0.0,
            avg_processing_time: 0.0,
            success_rate: 100.0,
            active_integrations: 0,
            failed_integrations: 0,
        }
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enable_event_bus: true,
            max_event_queue_size: 100000,
            event_processing_timeout: 5000, // 5 seconds
            enable_event_persistence: true,
            retry_attempts: 3,
            integration_timeout: 30000, // 30 seconds
        }
    }
}

impl Default for UnicoinCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unicoin_core_creation() {
        let core = UnicoinCore::new();
        assert!(core.config.enable_event_bus);
        assert_eq!(core.config.max_event_queue_size, 100000);
    }

    #[tokio::test]
    async fn test_integration_registration() {
        let core = UnicoinCore::new();
        let config = HashMap::new();
        
        let result = core.register_integration(
            "test_component".to_string(),
            IntegrationType::Direct,
            vec![],
            config,
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_integration_connection() {
        let core = UnicoinCore::new();
        let config = HashMap::new();
        
        // Register integration first
        core.register_integration(
            "test_component".to_string(),
            IntegrationType::Direct,
            vec![],
            config,
        ).await.unwrap();
        
        // Connect integration
        let result = core.connect_integration("test_component").await;
        assert!(result.is_ok());
        
        // Check status
        let status = core.get_integration_status("test_component").await;
        assert!(status.is_ok());
        assert_eq!(status.unwrap(), IntegrationStatus::Connected);
    }

    #[tokio::test]
    async fn test_event_handler_registration() {
        let core = UnicoinCore::new();
        
        let result = core.register_event_handler(
            "test_handler".to_string(),
            "test_component".to_string(),
            "test_event".to_string(),
            "test_function".to_string(),
            1,
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_event_publishing() {
        let core = UnicoinCore::new();
        
        // Register event handler first
        core.register_event_handler(
            "test_handler".to_string(),
            "test_component".to_string(),
            "test_event".to_string(),
            "test_function".to_string(),
            1,
        ).await.unwrap();
        
        // Publish event
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        
        let result = core.publish_event(
            "test_event".to_string(),
            "source_component".to_string(),
            Some("target_component".to_string()),
            data,
            EventPriority::Normal,
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_integration_metrics() {
        let core = UnicoinCore::new();
        let metrics = core.get_integration_metrics().await;
        
        assert!(metrics.is_ok());
        let metrics = metrics.unwrap();
        assert_eq!(metrics.events_processed, 0);
        assert_eq!(metrics.active_integrations, 0);
    }

    #[tokio::test]
    async fn test_health_check() {
        let core = UnicoinCore::new();
        let health = core.health_check().await;
        
        assert!(health.is_ok());
        let health = health.unwrap();
        assert!(health.is_empty()); // No integrations registered yet
    }

    #[tokio::test]
    async fn test_event_history() {
        let core = UnicoinCore::new();
        let history = core.get_event_history(Some(10)).await;
        
        assert!(history.is_ok());
        let history = history.unwrap();
        assert_eq!(history.len(), 0); // No events published yet
    }
}
