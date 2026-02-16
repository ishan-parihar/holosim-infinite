//! Distributed System - Phase 7: Multiplayer with Distributed Holographic Field
//!
//! This module implements a multiplayer simulation system that allows multiple entities
//! to interact through a shared holographic field while preserving free will autonomy
//! and implementing quantum observation effects.
//!
//! ## Sub-Phases
//!
//! ### Week 97-100: Distributed Holographic Field (Foundation)
//! Creates the base infrastructure for a shared holographic field across multiple
//! simulation instances. This includes:
//! - Field state synchronization protocols
//! - Merkle-based conflict resolution
//! - Signature verification for authenticity
//! - Base networking layer with peer discovery
//!
//! ### Week 101-104: Free Will Replication (Player Choices)
//! Enables replication of entity free will choices across the distributed field:
//! - Choice encoding and broadcasting
//! - Causality tracking across peers
//! - Deterministic vs non-deterministic state handling
//! - Choice persistence and replay capabilities
//!
//! ### Week 105-108: Observer Effect Synchronization (Quantum Observation)
//! Implements quantum observation mechanics across multiple observers:
//! - Wave function collapse coordination
//! - Observer effect propagation
//! - Entanglement state sharing
//! - Measurement consensus protocols
//!
//! ### Week 109-112: Multiplayer Features (Density/Scale Sharing)
//! Finalizes the multiplayer experience with density and scale interactions:
//! - Density octave progression sharing
//! - Multi-scale entity interactions
//! - Shared involution state transitions
//! - Cooperative consciousness evolution
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The holographic principle states that each entity
//! contains the whole. In a distributed system, this principle extends to peer instances,
//! where each peer contains the entire field state in a compressed form."

use crate::types::Float;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// Module Exports (to be enabled as each submodule is completed)
// ============================================================================

pub mod distributed_holographic_field;
pub mod free_will_replication;
pub mod multiplayer_features;
pub mod observer_effect_sync;

// ============================================================================
// Shared Newtype Wrappers
// ============================================================================

/// Unique identifier for a peer in the distributed system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId(pub u64);

impl PeerId {
    /// Creates a new PeerId
    pub fn new(id: u64) -> Self {
        PeerId(id)
    }

    /// Returns the underlying u64 value
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Peer({})", self.0)
    }
}

impl Default for PeerId {
    fn default() -> Self {
        PeerId(0)
    }
}

impl From<u64> for PeerId {
    fn from(id: u64) -> Self {
        PeerId(id)
    }
}

/// Unique identifier for a field state update
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateId(pub u64);

impl UpdateId {
    /// Creates a new UpdateId
    pub fn new(id: u64) -> Self {
        UpdateId(id)
    }

    /// Returns the underlying u64 value
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Generates a unique UpdateId from system entropy
    pub fn generate() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        duration.as_nanos().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        UpdateId(hasher.finish())
    }
}

impl fmt::Display for UpdateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Update({})", self.0)
    }
}

impl Default for UpdateId {
    fn default() -> Self {
        UpdateId(0)
    }
}

impl From<u64> for UpdateId {
    fn from(id: u64) -> Self {
        UpdateId(id)
    }
}

/// Unique identifier for a free will choice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChoiceId(pub u64);

impl ChoiceId {
    /// Creates a new ChoiceId
    pub fn new(id: u64) -> Self {
        ChoiceId(id)
    }

    /// Returns the underlying u64 value
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Returns the underlying u64 as bytes (little-endian)
    pub fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl fmt::Display for ChoiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ChoiceId {
    fn default() -> Self {
        ChoiceId(0)
    }
}

impl From<u64> for ChoiceId {
    fn from(id: u64) -> Self {
        ChoiceId(id)
    }
}

impl std::ops::AddAssign<u64> for ChoiceId {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

/// Unique identifier for a quantum observation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObservationId(pub u64);

impl ObservationId {
    /// Creates a new ObservationId
    pub fn new(id: u64) -> Self {
        ObservationId(id)
    }

    /// Returns the underlying u64 value
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ObservationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Observation({})", self.0)
    }
}

impl Default for ObservationId {
    fn default() -> Self {
        ObservationId(0)
    }
}

impl From<u64> for ObservationId {
    fn from(id: u64) -> Self {
        ObservationId(id)
    }
}

/// Unique identifier for a multiplayer session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);

impl SessionId {
    /// Creates a new SessionId
    pub fn new(id: u64) -> Self {
        SessionId(id)
    }

    /// Returns the underlying u64 value
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Generates a unique SessionId from system entropy
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        duration.as_nanos().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        SessionId(hasher.finish())
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Session({})", self.0)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId(0)
    }
}

impl From<u64> for SessionId {
    fn from(id: u64) -> Self {
        SessionId(id)
    }
}

// ============================================================================
// Type Aliases
// ============================================================================

/// Timestamp for network events and updates
pub type Timestamp = Float;

/// Version number for field state synchronization
pub type Version = u64;

/// Cryptographic signature for field state verification
pub type FieldSignature = Vec<u8>;

/// Network latency measurement
pub type Latency = Float;

/// Common result type for distributed system operations
pub type Result<T> = std::result::Result<T, NetworkError>;

// ============================================================================
// Network-Related Types
// ============================================================================

/// Connection status for a peer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Peer is disconnected
    Disconnected,
    /// Establishing connection
    Connecting,
    /// Connected and ready
    Connected,
    /// Connection error occurred
    Error,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::Connecting => write!(f, "Connecting"),
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Error => write!(f, "Error"),
        }
    }
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Disconnected
    }
}

/// Network error types
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkError {
    /// Failed to establish connection
    ConnectionFailed(String),
    /// Operation timed out
    Timeout(String),
    /// Invalid message format or content
    InvalidMessage(String),
    /// Cryptographic signature verification failed
    SignatureInvalid(String),
    /// Peer not found or unknown
    PeerNotFound(PeerId),
    /// Version mismatch between peers
    VersionMismatch {
        expected: Version,
        received: Version,
    },
    /// Field state conflict that couldn't be resolved
    ConflictResolutionFailed(String),
    /// Buffer overflow or capacity exceeded
    BufferOverflow(String),
    /// Serialization or deserialization error
    SerializationError(String),
    /// IO error
    IoError(String),
    /// Other network errors
    Other(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            NetworkError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            NetworkError::SignatureInvalid(msg) => write!(f, "Signature invalid: {}", msg),
            NetworkError::PeerNotFound(peer_id) => write!(f, "Peer not found: {}", peer_id),
            NetworkError::VersionMismatch { expected, received } => {
                write!(
                    f,
                    "Version mismatch: expected {}, got {}",
                    expected, received
                )
            }
            NetworkError::ConflictResolutionFailed(msg) => {
                write!(f, "Conflict resolution failed: {}", msg)
            }
            NetworkError::BufferOverflow(msg) => write!(f, "Buffer overflow: {}", msg),
            NetworkError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            NetworkError::IoError(msg) => write!(f, "IO error: {}", msg),
            NetworkError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for NetworkError {}

// Forward declaration for DistributedFieldError
// This will be implemented in distributed_holographic_field.rs

/// Network configuration settings
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkConfig {
    /// Maximum number of peers allowed
    pub max_peers: usize,
    /// Timeout for connection attempts (seconds)
    pub connection_timeout: Float,
    /// Timeout for message acknowledgments (seconds)
    pub ack_timeout: Float,
    /// Maximum buffer size for messages (bytes)
    pub max_buffer_size: usize,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval: Float,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Enable signature verification
    pub verify_signatures: bool,
    /// Enable field compression
    pub enable_compression: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            max_peers: 16,
            connection_timeout: 10.0,
            ack_timeout: 5.0,
            max_buffer_size: 1_048_576, // 1 MB
            heartbeat_interval: 30.0,
            max_retries: 3,
            verify_signatures: true,
            enable_compression: true,
        }
    }
}

impl NetworkConfig {
    /// Creates a new NetworkConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder for NetworkConfig
    pub fn builder() -> NetworkConfigBuilder {
        NetworkConfigBuilder::new()
    }
}

/// Builder for NetworkConfig
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkConfigBuilder {
    config: NetworkConfig,
}

impl NetworkConfigBuilder {
    fn new() -> Self {
        NetworkConfigBuilder {
            config: NetworkConfig::default(),
        }
    }

    pub fn max_peers(mut self, max_peers: usize) -> Self {
        self.config.max_peers = max_peers;
        self
    }

    pub fn connection_timeout(mut self, timeout: Float) -> Self {
        self.config.connection_timeout = timeout;
        self
    }

    pub fn ack_timeout(mut self, timeout: Float) -> Self {
        self.config.ack_timeout = timeout;
        self
    }

    pub fn max_buffer_size(mut self, size: usize) -> Self {
        self.config.max_buffer_size = size;
        self
    }

    pub fn heartbeat_interval(mut self, interval: Float) -> Self {
        self.config.heartbeat_interval = interval;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn verify_signatures(mut self, verify: bool) -> Self {
        self.config.verify_signatures = verify;
        self
    }

    pub fn enable_compression(mut self, enable: bool) -> Self {
        self.config.enable_compression = enable;
        self
    }

    pub fn build(self) -> Result<NetworkConfig> {
        // Validate configuration
        if self.config.max_peers == 0 {
            return Err(NetworkError::InvalidMessage(
                "max_peers must be greater than 0".to_string(),
            ));
        }
        if self.config.connection_timeout <= 0.0 {
            return Err(NetworkError::InvalidMessage(
                "connection_timeout must be positive".to_string(),
            ));
        }
        if self.config.max_buffer_size == 0 {
            return Err(NetworkError::InvalidMessage(
                "max_buffer_size must be greater than 0".to_string(),
            ));
        }
        Ok(self.config)
    }
}

impl Default for NetworkConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    /// Low priority - can be delayed or dropped
    Low = 0,
    /// Normal priority - standard processing
    Normal = 1,
    /// High priority - immediate processing
    High = 2,
}

impl fmt::Display for MessagePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagePriority::Low => write!(f, "Low"),
            MessagePriority::Normal => write!(f, "Normal"),
            MessagePriority::High => write!(f, "High"),
        }
    }
}

impl Default for MessagePriority {
    fn default() -> Self {
        MessagePriority::Normal
    }
}

impl From<u8> for MessagePriority {
    fn from(value: u8) -> Self {
        match value {
            0 => MessagePriority::Low,
            1 => MessagePriority::Normal,
            2 => MessagePriority::High,
            _ => MessagePriority::Normal,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_id_creation() {
        let peer_id = PeerId::new(123);
        assert_eq!(peer_id.as_u64(), 123);
    }

    #[test]
    fn test_peer_id_display() {
        let peer_id = PeerId::new(456);
        assert_eq!(peer_id.to_string(), "Peer(456)");
    }

    #[test]
    fn test_peer_id_from_u64() {
        let peer_id: PeerId = 789.into();
        assert_eq!(peer_id.as_u64(), 789);
    }

    #[test]
    fn test_update_id_equality() {
        let id1 = UpdateId::new(100);
        let id2 = UpdateId::new(100);
        let id3 = UpdateId::new(200);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_choice_id_hash() {
        let id1 = ChoiceId::new(50);
        let id2 = ChoiceId::new(50);
        assert_eq!(id1, id2);
        // Verify Hash implementation compiles
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2));
    }

    #[test]
    fn test_observation_id_default() {
        let obs_id = ObservationId::default();
        assert_eq!(obs_id.as_u64(), 0);
    }

    #[test]
    fn test_session_id_generation() {
        let session_id = SessionId::generate();
        assert_ne!(session_id.as_u64(), 0, "Generated ID should not be zero");
    }

    #[test]
    fn test_connection_status_display() {
        assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
        assert_eq!(ConnectionStatus::Disconnected.to_string(), "Disconnected");
    }

    #[test]
    fn test_network_error_display() {
        let err = NetworkError::ConnectionFailed("test error".to_string());
        assert_eq!(err.to_string(), "Connection failed: test error");

        let err = NetworkError::PeerNotFound(PeerId::new(42));
        assert_eq!(err.to_string(), "Peer not found: Peer(42)");
    }

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.max_peers, 16);
        assert_eq!(config.connection_timeout, 10.0);
        assert_eq!(config.ack_timeout, 5.0);
        assert!(config.verify_signatures);
    }

    #[test]
    fn test_network_config_builder() {
        let config = NetworkConfig::builder()
            .max_peers(32)
            .connection_timeout(20.0)
            .verify_signatures(false)
            .build()
            .unwrap();

        assert_eq!(config.max_peers, 32);
        assert_eq!(config.connection_timeout, 20.0);
        assert!(!config.verify_signatures);
    }

    #[test]
    fn test_network_config_builder_validation() {
        let result = NetworkConfig::builder().max_peers(0).build();
        assert!(result.is_err());

        let result = NetworkConfig::builder().connection_timeout(-5.0).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_priority_ordering() {
        assert!(MessagePriority::High > MessagePriority::Normal);
        assert!(MessagePriority::Normal > MessagePriority::Low);
        assert!(MessagePriority::High >= MessagePriority::High);
    }

    #[test]
    fn test_message_priority_from_u8() {
        assert_eq!(MessagePriority::from(0), MessagePriority::Low);
        assert_eq!(MessagePriority::from(1), MessagePriority::Normal);
        assert_eq!(MessagePriority::from(2), MessagePriority::High);
        assert_eq!(MessagePriority::from(99), MessagePriority::Normal); // invalid -> default
    }

    #[test]
    fn test_newtype_traits() {
        let peer1 = PeerId::new(1);
        let peer2 = PeerId::new(1);
        let peer3 = PeerId::new(2);

        // Clone
        let peer1_clone = peer1.clone();
        assert_eq!(peer1, peer1_clone);

        // Copy
        let peer1_copy = peer1;
        assert_eq!(peer1_copy, peer1_clone);

        // PartialEq
        assert_eq!(peer1, peer2);
        assert_ne!(peer1, peer3);

        // Hash
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(peer1);
        set.insert(peer2);
        set.insert(peer3);
        assert_eq!(set.len(), 2); // Only 1 and 2
    }

    #[test]
    fn test_network_error_version_mismatch() {
        let err = NetworkError::VersionMismatch {
            expected: 5,
            received: 3,
        };
        assert_eq!(err.to_string(), "Version mismatch: expected 5, got 3");
    }
}
