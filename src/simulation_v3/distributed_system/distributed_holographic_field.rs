//! Week 97-100: Distributed Holographic Field
//!
//! This module implements the distributed holographic field system where multiple peers
//! maintain local views of the holographic field that converge via consensus.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The holographic principle states that each entity
//! contains the whole. In a distributed system, this principle extends to peer instances,
//! where each peer contains the entire field state in a compressed form."
//!
//! # Key Components
//!
//! - **DistributedHolographicField**: Main distributed field system managing peer views
//! - **FieldUpdate**: Signed changes to the holographic field
//! - **ConsensusAlgorithm**: Conflict resolution mechanisms
//! - **PeerDiscovery**: Finding and managing peer connections
//! - **FieldSynchronizer**: Managing field synchronization
//! - **NetworkMessage**: Protocol for network communication

use crate::entity_layer7::layer7::EntityId;
use crate::holographic::holographic_field::HolographicField;
use crate::holographic::Position;
use crate::simulation_v3::distributed_system::{
    ConnectionStatus, FieldSignature, Latency, MessagePriority, NetworkError, PeerId, Result,
    Timestamp, UpdateId, Version,
};
use crate::spectrum::SpectrumRatio;
use crate::types::Float;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// Core Identifiers
// ============================================================================

/// Unique identifier for a holographic field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FieldId(pub u64);

impl FieldId {
    pub fn new(id: u64) -> Self {
        FieldId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn generate() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        duration.as_nanos().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        FieldId(hasher.finish())
    }
}

impl fmt::Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field({})", self.0)
    }
}

impl From<u64> for FieldId {
    fn from(id: u64) -> Self {
        FieldId(id)
    }
}

/// Unique identifier for a network message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MessageId(pub u64);

impl MessageId {
    pub fn new(id: u64) -> Self {
        MessageId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn generate() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        duration.as_nanos().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        MessageId(hasher.finish())
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Msg({})", self.0)
    }
}

impl From<u64> for MessageId {
    fn from(id: u64) -> Self {
        MessageId(id)
    }
}

/// Unique identifier for a consensus proposal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ProposalId(pub u64);

impl ProposalId {
    pub fn new(id: u64) -> Self {
        ProposalId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ProposalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Proposal({})", self.0)
    }
}

impl From<u64> for ProposalId {
    fn from(id: u64) -> Self {
        ProposalId(id)
    }
}

/// Unique identifier for a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConflictId(pub u64);

impl ConflictId {
    pub fn new(id: u64) -> Self {
        ConflictId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Generates a unique ConflictId from system entropy
    pub fn generate() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        duration.as_nanos().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        ConflictId(hasher.finish())
    }
}

impl fmt::Display for ConflictId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conflict({})", self.0)
    }
}

impl From<u64> for ConflictId {
    fn from(id: u64) -> Self {
        ConflictId(id)
    }
}

// ============================================================================
// Field Change Types
// ============================================================================

/// Type of change to the holographic field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ChangeType {
    /// A new entity was created in the field
    EntityCreated,
    /// An existing entity was modified
    #[default]
    EntityModified,
    /// An entity was deleted from the field
    EntityDeleted,
    /// Resonance pattern changed
    ResonanceChanged,
    /// Spectrum ratio changed
    SpectrumChanged,
    /// Field collapsed due to observer effect
    FieldCollapsed,
}

impl fmt::Display for ChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChangeType::EntityCreated => write!(f, "EntityCreated"),
            ChangeType::EntityModified => write!(f, "EntityModified"),
            ChangeType::EntityDeleted => write!(f, "EntityDeleted"),
            ChangeType::ResonanceChanged => write!(f, "ResonanceChanged"),
            ChangeType::SpectrumChanged => write!(f, "SpectrumChanged"),
            ChangeType::FieldCollapsed => write!(f, "FieldCollapsed"),
        }
    }
}

/// Resonance pattern for entities in the field
#[derive(Debug, Clone, PartialEq)]
pub struct ResonancePattern {
    /// Resonance frequency (0.0 to 1.0)
    pub frequency: Float,
    /// Resonance amplitude (0.0 to 1.0)
    pub amplitude: Float,
    /// Resonance phase (0.0 to 2π)
    pub phase: Float,
    /// Coherence level (0.0 to 1.0)
    pub coherence: Float,
}

impl ResonancePattern {
    pub fn new(frequency: Float, amplitude: Float, phase: Float, coherence: Float) -> Self {
        Self {
            frequency: frequency.clamp(0.0, 1.0),
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: phase.clamp(0.0, 2.0 * std::f64::consts::PI),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    pub fn initial() -> Self {
        Self {
            frequency: 0.5,
            amplitude: 0.5,
            phase: 0.0,
            coherence: 0.5,
        }
    }
}

impl Default for ResonancePattern {
    fn default() -> Self {
        Self::initial()
    }
}

impl Eq for ResonancePattern {}

impl std::hash::Hash for ResonancePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert f64 to u64 using to_bits() for hashing
        self.frequency.to_bits().hash(state);
        self.amplitude.to_bits().hash(state);
        self.phase.to_bits().hash(state);
        self.coherence.to_bits().hash(state);
    }
}

/// 3D coordinate in the holographic field
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinate3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Coordinate3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance_to(&self, other: &Coordinate3D) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Eq for Coordinate3D {}

impl std::hash::Hash for Coordinate3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert f64 to u64 using to_bits() for hashing
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl From<Position> for Coordinate3D {
    fn from(pos: Position) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }
}

impl From<Coordinate3D> for Position {
    fn from(coord: Coordinate3D) -> Self {
        Position {
            x: coord.x,
            y: coord.y,
            z: coord.z,
        }
    }
}

/// Individual change to the holographic field
#[derive(Debug, Clone)]
pub struct FieldChange {
    /// Type of change
    pub change_type: ChangeType,
    /// Entity affected by this change (if any)
    pub entity_id: Option<EntityId>,
    /// New resonance pattern (if applicable)
    pub resonance_pattern: Option<ResonancePattern>,
    /// New position (if applicable)
    pub position: Option<Coordinate3D>,
    /// New spectrum ratio (if applicable)
    pub spectrum_ratio: Option<SpectrumRatio>,
}

impl FieldChange {
    /// Apply this change to a holographic field
    pub fn apply_to(&self, _field: &mut HolographicField) -> Result<()> {
        // In a full implementation, this would modify the field state
        // For now, we just validate the change
        match self.change_type {
            ChangeType::EntityCreated => {
                // Create entity in field
            }
            ChangeType::EntityModified => {
                // Modify entity in field
            }
            ChangeType::EntityDeleted => {
                // Delete entity from field
            }
            ChangeType::ResonanceChanged => {
                // Update resonance pattern
            }
            ChangeType::SpectrumChanged => {
                // Update spectrum ratio
            }
            ChangeType::FieldCollapsed => {
                // Apply observer effect collapse
            }
        }
        Ok(())
    }

    /// Check if this change is compatible with another change
    pub fn is_compatible_with(&self, other: &FieldChange) -> bool {
        // Changes to different entities are compatible
        if self.entity_id.is_some()
            && other.entity_id.is_some()
            && self.entity_id != other.entity_id
        {
            return true;
        }

        // Same entity modified concurrently - check if changes conflict
        if self.change_type == ChangeType::EntityModified
            && other.change_type == ChangeType::EntityModified
        {
            // Check if they modify different aspects
            let self_changes = [
                self.resonance_pattern.is_some(),
                self.position.is_some(),
                self.spectrum_ratio.is_some(),
            ];
            let other_changes = [
                other.resonance_pattern.is_some(),
                other.position.is_some(),
                other.spectrum_ratio.is_some(),
            ];

            // Count how many aspects each modifies
            let self_count = self_changes.iter().filter(|&&x| x).count();
            let other_count = other_changes.iter().filter(|&&x| x).count();

            // If both modify only one aspect and they're different, they're compatible
            if self_count == 1 && other_count == 1 {
                return !(self_changes[0] && other_changes[0]
                    || self_changes[1] && other_changes[1]
                    || self_changes[2] && other_changes[2]);
            }
        }

        // Otherwise, assume incompatible (will require conflict resolution)
        false
    }
}

// ============================================================================
// Field Update
// ============================================================================

/// Changes to the holographic field from a peer
#[derive(Debug, Clone)]
pub struct FieldUpdate {
    /// Unique identifier for this update
    pub update_id: UpdateId,
    /// Peer that created this update
    pub from_peer: PeerId,
    /// Individual changes included in this update
    pub changes: Vec<FieldChange>,
    /// When this update was created
    pub timestamp: Timestamp,
    /// Version of the field this update applies to
    pub version: Version,
    /// Cryptographic signature for verification
    pub signature: FieldSignature,
}

impl FieldUpdate {
    /// Create a new field update
    pub fn new(
        from_peer: PeerId,
        changes: Vec<FieldChange>,
        version: Version,
        signature: FieldSignature,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let update_id = UpdateId::generate();

        Self {
            update_id,
            from_peer,
            changes,
            timestamp,
            version,
            signature,
        }
    }

    /// Validate the signature of this update
    pub fn validate(&self) -> bool {
        // In a real implementation, this would verify cryptographic signature
        // For now, we just check that signature is not empty
        !self.signature.is_empty()
    }

    /// Get the number of changes in this update
    pub fn get_change_count(&self) -> usize {
        self.changes.len()
    }

    /// Merge this update with another update
    pub fn merge_with(&self, other: &FieldUpdate) -> Option<FieldUpdate> {
        // Can only merge updates from the same peer
        if self.from_peer != other.from_peer {
            return None;
        }

        // Check if changes are compatible
        for change1 in &self.changes {
            for change2 in &other.changes {
                if !change1.is_compatible_with(change2) {
                    return None;
                }
            }
        }

        // Merge changes
        let mut merged_changes = self.changes.clone();
        merged_changes.extend(other.changes.clone());

        // Use the later timestamp and higher version
        let timestamp = self.timestamp.max(other.timestamp);
        let version = self.version.max(other.version);

        // In a real implementation, we'd recompute the signature
        let signature = self.signature.clone();

        Some(FieldUpdate {
            update_id: UpdateId::generate(),
            from_peer: self.from_peer,
            changes: merged_changes,
            timestamp,
            version,
            signature,
        })
    }
}

// ============================================================================
// Consensus Types
// ============================================================================

/// Type of consensus algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConsensusType {
    /// Higher resonance priority wins
    #[default]
    ResonancePriority,
    /// Latest timestamp wins
    LastWriteWins,
    /// Majority vote required
    Voting,
    /// Combination of algorithms
    Hybrid,
}

impl fmt::Display for ConsensusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsensusType::ResonancePriority => write!(f, "ResonancePriority"),
            ConsensusType::LastWriteWins => write!(f, "LastWriteWins"),
            ConsensusType::Voting => write!(f, "Voting"),
            ConsensusType::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// Vote in consensus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Vote {
    /// Vote in favor
    Approve,
    /// Vote against
    Reject,
    /// Abstain from voting
    #[default]
    Abstain,
}

impl fmt::Display for Vote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vote::Approve => write!(f, "Approve"),
            Vote::Reject => write!(f, "Reject"),
            Vote::Abstain => write!(f, "Abstain"),
        }
    }
}

/// Status of a consensus proposal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProposalStatus {
    /// Proposal is being voted on
    #[default]
    Pending,
    /// Proposal was approved
    Approved,
    /// Proposal was rejected
    Rejected,
    /// Proposal expired
    Expired,
}

impl fmt::Display for ProposalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProposalStatus::Pending => write!(f, "Pending"),
            ProposalStatus::Approved => write!(f, "Approved"),
            ProposalStatus::Rejected => write!(f, "Rejected"),
            ProposalStatus::Expired => write!(f, "Expired"),
        }
    }
}

/// Consensus proposal
#[derive(Debug, Clone)]
pub struct Proposal {
    /// Unique identifier for this proposal
    pub proposal_id: ProposalId,
    /// Update being proposed
    pub update: FieldUpdate,
    /// Votes from peers
    pub votes: HashMap<PeerId, Vote>,
    /// When this proposal was created
    pub created_at: Timestamp,
    /// Current status
    pub status: ProposalStatus,
}

impl Proposal {
    pub fn new(proposal_id: ProposalId, update: FieldUpdate) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        Self {
            proposal_id,
            update,
            votes: HashMap::new(),
            created_at,
            status: ProposalStatus::Pending,
        }
    }

    /// Add a vote to this proposal
    pub fn add_vote(&mut self, peer_id: PeerId, vote: Vote) {
        self.votes.insert(peer_id, vote);
    }

    /// Count votes by type
    pub fn count_votes(&self) -> (usize, usize, usize) {
        let approve = self.votes.values().filter(|&&v| v == Vote::Approve).count();
        let reject = self.votes.values().filter(|&&v| v == Vote::Reject).count();
        let abstain = self.votes.values().filter(|&&v| v == Vote::Abstain).count();
        (approve, reject, abstain)
    }

    /// Check if this proposal has majority approval
    pub fn has_majority_approval(&self, total_peers: usize) -> bool {
        let (approve, reject, _) = self.count_votes();
        let required = (total_peers / 2) + 1;
        approve >= required && approve > reject
    }
}

/// Consensus algorithm for conflict resolution
#[derive(Debug, Clone)]
pub struct ConsensusAlgorithm {
    /// Type of consensus algorithm
    pub algorithm_type: ConsensusType,
    /// Active proposals
    pub proposals: HashMap<ProposalId, Proposal>,
    /// Next proposal ID
    pub next_proposal_id: u64,
}

impl ConsensusAlgorithm {
    pub fn new(algorithm_type: ConsensusType) -> Self {
        Self {
            algorithm_type,
            proposals: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    /// Propose an update for consensus
    pub fn propose_update(&mut self, update: FieldUpdate) -> Result<ProposalId> {
        let proposal_id = ProposalId::new(self.next_proposal_id);
        self.next_proposal_id += 1;

        let proposal = Proposal::new(proposal_id, update);
        self.proposals.insert(proposal_id, proposal);

        Ok(proposal_id)
    }

    /// Vote on a proposal
    pub fn vote_on_update(&mut self, proposal_id: ProposalId, vote: Vote) -> Result<()> {
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .ok_or_else(|| NetworkError::Other(format!("Proposal not found: {}", proposal_id)))?;

        proposal.add_vote(PeerId::new(0), vote); // Use placeholder peer ID
        Ok(())
    }

    /// Resolve a conflict using the consensus algorithm
    pub fn resolve_conflict(&mut self, conflict: Conflict) -> Result<Resolution> {
        match self.algorithm_type {
            ConsensusType::LastWriteWins => {
                // Choose the update with the latest timestamp
                let chosen_update = conflict
                    .conflicting_updates
                    .iter()
                    .max_by(|a, b| {
                        a.timestamp
                            .partial_cmp(&b.timestamp)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .cloned();

                Ok(Resolution {
                    resolution_type: ResolutionType::ChosenUpdate,
                    chosen_update,
                    merged_update: None,
                    rationale: "Chose update with latest timestamp".to_string(),
                })
            }
            ConsensusType::ResonancePriority => {
                // Choose the update with higher resonance
                let chosen_update = conflict
                    .conflicting_updates
                    .iter()
                    .max_by(|a, b| {
                        // Find resonance change and compare
                        let a_resonance = a
                            .changes
                            .iter()
                            .find_map(|c| c.resonance_pattern.as_ref())
                            .map(|r| r.frequency)
                            .unwrap_or(0.0);
                        let b_resonance = b
                            .changes
                            .iter()
                            .find_map(|c| c.resonance_pattern.as_ref())
                            .map(|r| r.frequency)
                            .unwrap_or(0.0);
                        a_resonance
                            .partial_cmp(&b_resonance)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .cloned();

                Ok(Resolution {
                    resolution_type: ResolutionType::ChosenUpdate,
                    chosen_update,
                    merged_update: None,
                    rationale: "Chose update with higher resonance frequency".to_string(),
                })
            }
            ConsensusType::Voting => {
                // For voting, we need a proposal
                let proposal_id = ProposalId::new(self.next_proposal_id);
                self.next_proposal_id += 1;

                // Create a proposal from the conflict
                if let Some(first_update) = conflict.conflicting_updates.first() {
                    let proposal = Proposal::new(proposal_id, first_update.clone());
                    self.proposals.insert(proposal_id, proposal);
                }

                Ok(Resolution {
                    resolution_type: ResolutionType::ConsensusReached,
                    chosen_update: None,
                    merged_update: None,
                    rationale: "Initiated voting process for conflict resolution".to_string(),
                })
            }
            ConsensusType::Hybrid => {
                // Hybrid: try to merge, fall back to last-write-wins
                let merged = Self::try_merge_updates(&conflict.conflicting_updates);

                if let Some(merged_update) = merged {
                    Ok(Resolution {
                        resolution_type: ResolutionType::MergedUpdate,
                        chosen_update: None,
                        merged_update: Some(merged_update),
                        rationale: "Successfully merged conflicting updates".to_string(),
                    })
                } else {
                    // Fall back to last-write-wins
                    let chosen_update = conflict
                        .conflicting_updates
                        .iter()
                        .max_by(|a, b| {
                            a.timestamp
                                .partial_cmp(&b.timestamp)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .cloned();

                    Ok(Resolution {
                        resolution_type: ResolutionType::ChosenUpdate,
                        chosen_update,
                        merged_update: None,
                        rationale: "Merge failed, chose update with latest timestamp".to_string(),
                    })
                }
            }
        }
    }

    /// Try to merge multiple updates
    fn try_merge_updates(updates: &[FieldUpdate]) -> Option<FieldUpdate> {
        if updates.is_empty() {
            return None;
        }

        let mut merged = updates[0].clone();
        for update in &updates[1..] {
            if let Some(result) = merged.merge_with(update) {
                merged = result;
            } else {
                return None;
            }
        }
        Some(merged)
    }

    /// Get the status of a proposal
    pub fn get_proposal_status(&self, proposal_id: ProposalId) -> Option<ProposalStatus> {
        self.proposals.get(&proposal_id).map(|p| p.status)
    }
}

impl Default for ConsensusAlgorithm {
    fn default() -> Self {
        Self::new(ConsensusType::default())
    }
}

// ============================================================================
// Conflict Types
// ============================================================================

/// Type of conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConflictType {
    /// Same entity modified concurrently
    #[default]
    ConcurrentModification,
    /// Version mismatch between updates
    VersionConflict,
    /// Incompatible resonance patterns
    ResonanceConflict,
    /// Different observers with conflicting observations
    ObserverConflict,
}

impl fmt::Display for ConflictType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConflictType::ConcurrentModification => write!(f, "ConcurrentModification"),
            ConflictType::VersionConflict => write!(f, "VersionConflict"),
            ConflictType::ResonanceConflict => write!(f, "ResonanceConflict"),
            ConflictType::ObserverConflict => write!(f, "ObserverConflict"),
        }
    }
}

/// Type of resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResolutionType {
    /// One update was chosen over others
    #[default]
    ChosenUpdate,
    /// Updates were merged
    MergedUpdate,
    /// Consensus was reached through voting
    ConsensusReached,
    /// All updates were discarded
    DiscardedAll,
}

impl fmt::Display for ResolutionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolutionType::ChosenUpdate => write!(f, "ChosenUpdate"),
            ResolutionType::MergedUpdate => write!(f, "MergedUpdate"),
            ResolutionType::ConsensusReached => write!(f, "ConsensusReached"),
            ResolutionType::DiscardedAll => write!(f, "DiscardedAll"),
        }
    }
}

/// Conflict between field updates
#[derive(Debug, Clone)]
pub struct Conflict {
    /// Unique identifier for this conflict
    pub conflict_id: ConflictId,
    /// Updates that are in conflict
    pub conflicting_updates: Vec<FieldUpdate>,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Whether manual resolution is required
    pub resolution_required: bool,
}

impl Conflict {
    pub fn new(conflict_type: ConflictType, conflicting_updates: Vec<FieldUpdate>) -> Self {
        let resolution_required = conflicting_updates.len() > 1;
        let conflict_id = ConflictId::generate();

        Self {
            conflict_id,
            conflicting_updates,
            conflict_type,
            resolution_required,
        }
    }
}

/// Resolution of a conflict
#[derive(Debug, Clone)]
pub struct Resolution {
    /// Type of resolution
    pub resolution_type: ResolutionType,
    /// Update that was chosen (if applicable)
    pub chosen_update: Option<FieldUpdate>,
    /// Merged update (if applicable)
    pub merged_update: Option<FieldUpdate>,
    /// Rationale for the resolution
    pub rationale: String,
}

impl Resolution {
    pub fn new(
        resolution_type: ResolutionType,
        chosen_update: Option<FieldUpdate>,
        merged_update: Option<FieldUpdate>,
        rationale: String,
    ) -> Self {
        Self {
            resolution_type,
            chosen_update,
            merged_update,
            rationale,
        }
    }
}

// ============================================================================
// Peer Types
// ============================================================================

/// Information about a peer
#[derive(Debug, Clone, PartialEq)]
pub struct PeerInfo {
    /// Unique identifier for this peer
    pub peer_id: PeerId,
    /// Network address
    pub address: String,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// Latency to this peer
    pub latency: Latency,
    /// Peer's current field version
    pub field_version: Version,
    /// When this peer was last seen
    pub last_seen: Timestamp,
}

impl PeerInfo {
    pub fn new(peer_id: PeerId, address: String) -> Self {
        Self {
            peer_id,
            address,
            connection_status: ConnectionStatus::Disconnected,
            latency: 0.0,
            field_version: 0,
            last_seen: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connection_status == ConnectionStatus::Connected
    }
}

/// Peer discovery and management
#[derive(Debug, Clone)]
pub struct PeerDiscovery {
    /// List of known peer IDs
    pub peer_list: Vec<PeerId>,
    /// Detailed information about known peers
    pub known_peers: HashMap<PeerId, PeerInfo>,
    /// Whether discovery is enabled
    pub discovery_enabled: bool,
}

impl PeerDiscovery {
    pub fn new() -> Self {
        Self {
            peer_list: Vec::new(),
            known_peers: HashMap::new(),
            discovery_enabled: true,
        }
    }

    /// Discover new peers
    pub fn discover_peers(&mut self) -> Vec<PeerId> {
        if !self.discovery_enabled {
            return Vec::new();
        }

        // In a real implementation, this would scan the network
        // For now, return an empty list
        Vec::new()
    }

    /// Connect to a peer
    pub fn connect_to_peer(&mut self, peer_id: PeerId) -> Result<()> {
        let peer_info = self
            .known_peers
            .get_mut(&peer_id)
            .ok_or(NetworkError::PeerNotFound(peer_id))?;

        peer_info.connection_status = ConnectionStatus::Connecting;

        // Simulate connection delay
        peer_info.connection_status = ConnectionStatus::Connected;
        peer_info.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        Ok(())
    }

    /// Disconnect from a peer
    pub fn disconnect_from_peer(&mut self, peer_id: PeerId) -> Result<()> {
        let peer_info = self
            .known_peers
            .get_mut(&peer_id)
            .ok_or(NetworkError::PeerNotFound(peer_id))?;

        peer_info.connection_status = ConnectionStatus::Disconnected;

        Ok(())
    }

    /// Get information about a peer
    pub fn get_peer_info(&self, peer_id: PeerId) -> Option<&PeerInfo> {
        self.known_peers.get(&peer_id)
    }

    /// Get list of connected peers
    pub fn get_connected_peers(&self) -> Vec<PeerId> {
        self.peer_list
            .iter()
            .filter(|&peer_id| {
                self.known_peers
                    .get(peer_id)
                    .map(|info| info.is_connected())
                    .unwrap_or(false)
            })
            .copied()
            .collect()
    }

    /// Add a new peer
    pub fn add_peer(&mut self, peer_id: PeerId, address: String) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.known_peers.entry(peer_id) {
            let peer_info = PeerInfo::new(peer_id, address);
            e.insert(peer_info);
            self.peer_list.push(peer_id);
        }
    }

    /// Remove a peer
    pub fn remove_peer(&mut self, peer_id: PeerId) -> Result<()> {
        self.known_peers
            .remove(&peer_id)
            .ok_or(NetworkError::PeerNotFound(peer_id))?;

        self.peer_list.retain(|&id| id != peer_id);
        Ok(())
    }
}

impl Default for PeerDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Synchronization Types
// ============================================================================

/// A peer's view of the holographic field
#[derive(Debug, Clone)]
pub struct PeerFieldState {
    /// Peer that owns this view
    pub peer_id: PeerId,
    /// Snapshot of the field from this peer's perspective
    pub field_snapshot: HolographicField,
    /// Version of this field state
    pub version: Version,
    /// When this state was last synchronized
    pub last_sync: Timestamp,
    /// Number of updates received from this peer
    pub update_count: usize,
}

impl PeerFieldState {
    pub fn new(peer_id: PeerId, field_snapshot: HolographicField) -> Self {
        Self {
            peer_id,
            field_snapshot,
            version: 0,
            last_sync: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
            update_count: 0,
        }
    }
}

/// Result of a synchronization operation
#[derive(Debug, Clone, PartialEq)]
pub struct SyncResult {
    /// Updates that were applied
    pub updates_applied: Vec<UpdateId>,
    /// Conflicts that were resolved
    pub conflicts_resolved: Vec<ConflictId>,
    /// New field version after sync
    pub new_field_version: Version,
    /// Duration of the sync operation
    pub sync_duration: Timestamp,
}

impl SyncResult {
    pub fn new(
        updates_applied: Vec<UpdateId>,
        conflicts_resolved: Vec<ConflictId>,
        new_field_version: Version,
        sync_duration: Timestamp,
    ) -> Self {
        Self {
            updates_applied,
            conflicts_resolved,
            new_field_version,
            sync_duration,
        }
    }
}

impl Default for SyncResult {
    fn default() -> Self {
        Self {
            updates_applied: Vec::new(),
            conflicts_resolved: Vec::new(),
            new_field_version: 0,
            sync_duration: 0.0,
        }
    }
}

/// Field synchronizer managing update queue and processing
#[derive(Debug, Clone)]
pub struct FieldSynchronizer {
    /// Queue of updates to process
    pub sync_queue: VecDeque<FieldUpdate>,
    /// Updates currently being processed
    pub pending_updates: HashMap<UpdateId, FieldUpdate>,
    /// Audit trail of all updates
    pub update_log: Vec<FieldUpdate>,
    /// Maximum queue size
    pub max_queue_size: usize,
}

impl FieldSynchronizer {
    pub fn new() -> Self {
        Self {
            sync_queue: VecDeque::new(),
            pending_updates: HashMap::new(),
            update_log: Vec::new(),
            max_queue_size: 1000,
        }
    }

    /// Add an update to the sync queue
    pub fn enqueue_update(&mut self, update: FieldUpdate) -> Result<()> {
        if self.sync_queue.len() >= self.max_queue_size {
            return Err(NetworkError::BufferOverflow(
                "Sync queue is full".to_string(),
            ));
        }

        self.sync_queue.push_back(update);
        Ok(())
    }

    /// Process all updates in the queue
    pub fn process_queue(&mut self) -> Result<Vec<SyncResult>> {
        let mut results = Vec::new();
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        while let Some(update) = self.sync_queue.pop_front() {
            // Move to pending
            let update_id = update.update_id;
            self.pending_updates.insert(update_id, update.clone());

            // Add to log
            self.update_log.push(update.clone());

            // Create sync result
            let result = SyncResult {
                updates_applied: vec![update_id],
                conflicts_resolved: Vec::new(),
                new_field_version: update.version,
                sync_duration: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64()
                    - start_time,
            };

            results.push(result);

            // Remove from pending
            self.pending_updates.remove(&update_id);
        }

        Ok(results)
    }

    /// Get peers that should receive this update
    pub fn broadcast_update(&self, _update: &FieldUpdate) -> Vec<PeerId> {
        // In a real implementation, this would determine which peers need the update
        // For now, return empty list
        Vec::new()
    }

    /// Get pending updates
    pub fn get_pending_updates(&self) -> Vec<&FieldUpdate> {
        self.pending_updates.values().collect()
    }

    /// Get update history
    pub fn get_update_history(&self) -> Vec<&FieldUpdate> {
        self.update_log.iter().collect()
    }
}

impl Default for FieldSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Message Types
// ============================================================================

/// Type of network message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MessageType {
    /// Field update message
    FieldUpdateMsg,
    /// Synchronization request
    SyncRequest,
    /// Synchronization response
    SyncResponse,
    /// Consensus proposal
    ConsensusProposal,
    /// Consensus vote
    ConsensusVote,
    /// Peer discovery
    PeerDiscovery,
    /// Peer announcement
    PeerAnnouncement,
    /// Keep-alive message
    #[default]
    KeepAlive,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::FieldUpdateMsg => write!(f, "FieldUpdateMsg"),
            MessageType::SyncRequest => write!(f, "SyncRequest"),
            MessageType::SyncResponse => write!(f, "SyncResponse"),
            MessageType::ConsensusProposal => write!(f, "ConsensusProposal"),
            MessageType::ConsensusVote => write!(f, "ConsensusVote"),
            MessageType::PeerDiscovery => write!(f, "PeerDiscovery"),
            MessageType::PeerAnnouncement => write!(f, "PeerAnnouncement"),
            MessageType::KeepAlive => write!(f, "KeepAlive"),
        }
    }
}

/// Payload of a network message
#[derive(Debug, Clone)]
pub enum MessagePayload {
    /// Field update payload
    FieldUpdatePayload(FieldUpdate),
    /// Sync request payload
    SyncRequestPayload { version: Version },
    /// Sync response payload
    SyncResponsePayload { updates: Vec<FieldUpdate> },
    /// Consensus proposal payload
    ConsensusProposalPayload(Proposal),
    /// Consensus vote payload
    ConsensusVotePayload { proposal_id: ProposalId, vote: Vote },
    /// Peer discovery payload
    PeerDiscoveryPayload { peer_list: Vec<PeerId> },
    /// Peer announcement payload
    PeerAnnouncementPayload(PeerInfo),
    /// Keep-alive payload
    KeepAlivePayload,
}

impl MessagePayload {
    /// Get the message type for this payload
    pub fn message_type(&self) -> MessageType {
        match self {
            MessagePayload::FieldUpdatePayload(_) => MessageType::FieldUpdateMsg,
            MessagePayload::SyncRequestPayload { .. } => MessageType::SyncRequest,
            MessagePayload::SyncResponsePayload { .. } => MessageType::SyncResponse,
            MessagePayload::ConsensusProposalPayload(_) => MessageType::ConsensusProposal,
            MessagePayload::ConsensusVotePayload { .. } => MessageType::ConsensusVote,
            MessagePayload::PeerDiscoveryPayload { .. } => MessageType::PeerDiscovery,
            MessagePayload::PeerAnnouncementPayload(_) => MessageType::PeerAnnouncement,
            MessagePayload::KeepAlivePayload => MessageType::KeepAlive,
        }
    }
}

/// Network message for peer communication
#[derive(Debug, Clone)]
pub struct NetworkMessage {
    /// Unique identifier for this message
    pub message_id: MessageId,
    /// Type of message
    pub message_type: MessageType,
    /// Sender of this message
    pub sender: PeerId,
    /// Receiver of this message (None for broadcast)
    pub receiver: Option<PeerId>,
    /// Message payload
    pub payload: MessagePayload,
    /// When this message was sent
    pub timestamp: Timestamp,
    /// Message priority
    pub priority: MessagePriority,
    /// Cryptographic signature
    pub signature: FieldSignature,
}

impl NetworkMessage {
    pub fn new(
        message_type: MessageType,
        sender: PeerId,
        receiver: Option<PeerId>,
        payload: MessagePayload,
        priority: MessagePriority,
        signature: FieldSignature,
    ) -> Self {
        let message_id = MessageId::generate();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        Self {
            message_id,
            message_type,
            sender,
            receiver,
            payload,
            timestamp,
            priority,
            signature,
        }
    }

    /// Validate the message signature
    pub fn validate(&self) -> bool {
        !self.signature.is_empty()
    }

    /// Check if this message is a broadcast
    pub fn is_broadcast(&self) -> bool {
        self.receiver.is_none()
    }

    /// Check if this message is intended for a specific peer
    pub fn is_for(&self, peer_id: PeerId) -> bool {
        self.receiver == Some(peer_id)
    }
}

// ============================================================================
// Distributed Field Error
// ============================================================================

/// Errors specific to distributed holographic field operations
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedFieldError {
    /// Invalid field update
    InvalidUpdate(String),
    /// Update signature verification failed
    UpdateSignatureInvalid(String),
    /// Peer not found
    PeerNotFound(PeerId),
    /// Conflict resolution failed
    ConflictResolutionFailed(String),
    /// Synchronization failed
    SyncFailed(String),
    /// Consensus algorithm failed
    ConsensusFailed(String),
    /// Version mismatch
    VersionMismatch { expected: Version, actual: Version },
    /// Network error
    NetworkError(NetworkError),
}

impl From<DistributedFieldError> for NetworkError {
    fn from(err: DistributedFieldError) -> Self {
        match err {
            DistributedFieldError::NetworkError(e) => e,
            _ => NetworkError::Other(err.to_string()),
        }
    }
}

impl fmt::Display for DistributedFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistributedFieldError::InvalidUpdate(msg) => {
                write!(f, "Invalid update: {}", msg)
            }
            DistributedFieldError::UpdateSignatureInvalid(msg) => {
                write!(f, "Update signature invalid: {}", msg)
            }
            DistributedFieldError::PeerNotFound(peer_id) => {
                write!(f, "Peer not found: {}", peer_id)
            }
            DistributedFieldError::ConflictResolutionFailed(msg) => {
                write!(f, "Conflict resolution failed: {}", msg)
            }
            DistributedFieldError::SyncFailed(msg) => {
                write!(f, "Sync failed: {}", msg)
            }
            DistributedFieldError::ConsensusFailed(msg) => {
                write!(f, "Consensus failed: {}", msg)
            }
            DistributedFieldError::VersionMismatch { expected, actual } => {
                write!(f, "Version mismatch: expected {}, got {}", expected, actual)
            }
            DistributedFieldError::NetworkError(err) => {
                write!(f, "Network error: {}", err)
            }
        }
    }
}

impl std::error::Error for DistributedFieldError {}

impl From<NetworkError> for DistributedFieldError {
    fn from(err: NetworkError) -> Self {
        DistributedFieldError::NetworkError(err)
    }
}

// ============================================================================
// Distributed Holographic Field
// ============================================================================

/// State of the holographic field
#[derive(Debug, Clone)]
pub struct FieldState {
    /// Current version of the field
    pub version: Version,
    /// Number of entities in the field
    pub entity_count: usize,
    /// Field coherence level (0.0 to 1.0)
    pub coherence: Float,
    /// Field stability (0.0 to 1.0)
    pub stability: Float,
}

impl FieldState {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            entity_count: 0,
            coherence: 1.0,
            stability: 1.0,
        }
    }
}

impl Default for FieldState {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Distributed holographic field system
///
/// Manages local and peer views of the holographic field with consensus-based
/// synchronization and conflict resolution.
#[derive(Debug, Clone)]
pub struct DistributedHolographicField {
    /// Unique identifier for this field
    pub field_id: FieldId,
    /// This peer's local view of the field
    pub local_field: HolographicField,
    /// Other peers' views of the field
    pub peer_fields: HashMap<PeerId, PeerFieldState>,
    /// Consensus algorithm for conflict resolution
    pub consensus: ConsensusAlgorithm,
    /// Current version of the field
    pub field_version: Version,
    /// Field synchronizer managing updates
    pub synchronizer: FieldSynchronizer,
}

impl DistributedHolographicField {
    /// Create a new distributed holographic field
    pub fn new(field_id: FieldId, local_field: HolographicField) -> Self {
        Self {
            field_id,
            local_field,
            peer_fields: HashMap::new(),
            consensus: ConsensusAlgorithm::default(),
            field_version: 0,
            synchronizer: FieldSynchronizer::default(),
        }
    }

    /// Synchronize with incoming field updates
    pub fn sync(&mut self, updates: Vec<FieldUpdate>) -> Result<SyncResult> {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let mut updates_applied = Vec::new();
        let mut conflicts_resolved = Vec::new();

        // Enqueue all updates
        for update in updates {
            // Validate update
            if !update.validate() {
                return Err(DistributedFieldError::UpdateSignatureInvalid(
                    "Invalid signature".to_string(),
                )
                .into());
            }

            // Check version compatibility
            if update.version != self.field_version {
                // Try to resolve version conflict
                let conflict = Conflict::new(ConflictType::VersionConflict, vec![update.clone()]);
                let resolution = self.consensus.resolve_conflict(conflict)?;

                if let Some(chosen) = resolution.chosen_update {
                    self.synchronizer.enqueue_update(chosen)?;
                } else if let Some(merged) = resolution.merged_update {
                    self.synchronizer.enqueue_update(merged)?;
                }
                conflicts_resolved.push(ConflictId::generate());
            } else {
                self.synchronizer.enqueue_update(update.clone())?;
                updates_applied.push(update.update_id);
            }
        }

        // Process queue
        let sync_results = self.synchronizer.process_queue()?;

        // Update field version
        if let Some(result) = sync_results.first() {
            self.field_version = result.new_field_version;
        }

        let sync_duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            - start_time;

        Ok(SyncResult {
            updates_applied,
            conflicts_resolved,
            new_field_version: self.field_version,
            sync_duration,
        })
    }

    /// Apply a single update to the field
    pub fn apply_update(&mut self, update: FieldUpdate) -> Result<()> {
        // Validate update
        if !update.validate() {
            return Err(DistributedFieldError::UpdateSignatureInvalid(
                "Invalid signature".to_string(),
            )
            .into());
        }

        // Apply changes
        for change in &update.changes {
            change.apply_to(&mut self.local_field)?;
        }

        // Update version
        self.field_version = update.version;

        Ok(())
    }

    /// Resolve a conflict between updates
    pub fn resolve_conflict(&mut self, conflict: Conflict) -> Result<Resolution> {
        self.consensus.resolve_conflict(conflict)
    }

    /// Get the current field state
    pub fn get_field_state(&self) -> FieldState {
        FieldState {
            version: self.field_version,
            entity_count: 0, // Would need to count from local_field
            coherence: 0.9,
            stability: 0.9,
        }
    }

    /// Get a peer's view of the field
    pub fn get_peer_view(&self, peer_id: PeerId) -> Option<&PeerFieldState> {
        self.peer_fields.get(&peer_id)
    }

    /// Get the local view of the field
    pub fn get_local_view(&self) -> &HolographicField {
        &self.local_field
    }

    /// Add a peer to the distributed field
    pub fn add_peer(&mut self, peer_id: PeerId) -> Result<()> {
        if self.peer_fields.contains_key(&peer_id) {
            return Err(
                DistributedFieldError::InvalidUpdate("Peer already exists".to_string()).into(),
            );
        }

        let peer_state = PeerFieldState::new(peer_id, self.local_field.clone());
        self.peer_fields.insert(peer_id, peer_state);

        Ok(())
    }

    /// Remove a peer from the distributed field
    pub fn remove_peer(&mut self, peer_id: PeerId) -> Result<()> {
        self.peer_fields
            .remove(&peer_id)
            .ok_or(DistributedFieldError::PeerNotFound(peer_id))?;

        Ok(())
    }

    /// Get the number of peers
    pub fn peer_count(&self) -> usize {
        self.peer_fields.len()
    }
}

// ============================================================================
// Message Handler
// ============================================================================

/// Handles network messages for the distributed system
#[derive(Debug, Clone)]
pub struct MessageHandler {
    /// This peer's ID
    pub peer_id: PeerId,
    /// Messages received
    pub received_messages: Vec<NetworkMessage>,
    /// Messages sent
    pub sent_messages: Vec<NetworkMessage>,
}

impl MessageHandler {
    pub fn new(peer_id: PeerId) -> Self {
        Self {
            peer_id,
            received_messages: Vec::new(),
            sent_messages: Vec::new(),
        }
    }

    /// Handle an incoming message
    pub fn handle_message(&mut self, message: NetworkMessage) -> Result<Option<NetworkMessage>> {
        // Validate message
        if !message.validate() {
            return Err(NetworkError::SignatureInvalid(
                "Invalid signature".to_string(),
            ));
        }

        // Check if message is for this peer
        if !message.is_broadcast() && !message.is_for(self.peer_id) {
            return Ok(None);
        }

        self.received_messages.push(message.clone());

        // Process message based on type
        match message.message_type {
            MessageType::KeepAlive => {
                // Respond with keep-alive
                let response = NetworkMessage::new(
                    MessageType::KeepAlive,
                    self.peer_id,
                    Some(message.sender),
                    MessagePayload::KeepAlivePayload,
                    MessagePriority::Normal,
                    vec![],
                );
                self.sent_messages.push(response.clone());
                Ok(Some(response))
            }
            MessageType::SyncRequest => {
                // Would respond with sync data
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

impl Default for MessageHandler {
    fn default() -> Self {
        Self::new(PeerId::default())
    }
}

// ============================================================================
// Distributed Holographic Field System
// ============================================================================

/// Main system wrapper for the distributed holographic field
#[derive(Debug, Clone)]
pub struct DistributedHolographicFieldSystem {
    /// The distributed field
    pub distributed_field: DistributedHolographicField,
    /// Peer discovery
    pub peer_discovery: PeerDiscovery,
    /// Message handler
    pub message_handler: MessageHandler,
}

impl DistributedHolographicFieldSystem {
    /// Create a new distributed holographic field system
    pub fn new(field_id: FieldId, local_field: HolographicField, peer_id: PeerId) -> Self {
        Self {
            distributed_field: DistributedHolographicField::new(field_id, local_field),
            peer_discovery: PeerDiscovery::new(),
            message_handler: MessageHandler::new(peer_id),
        }
    }

    /// Initialize the system
    pub fn initialize(&mut self) -> Result<()> {
        // Discover peers
        let discovered_peers = self.peer_discovery.discover_peers();

        // Add discovered peers
        for peer_id in discovered_peers {
            self.peer_discovery
                .add_peer(peer_id, format!("peer:{}", peer_id.as_u64()));
            self.distributed_field.add_peer(peer_id)?;
        }

        Ok(())
    }

    /// Process incoming messages
    pub fn process_messages(
        &mut self,
        messages: Vec<NetworkMessage>,
    ) -> Result<Vec<NetworkMessage>> {
        let mut responses = Vec::new();

        for message in messages {
            if let Some(response) = self.message_handler.handle_message(message)? {
                responses.push(response);
            }
        }

        Ok(responses)
    }

    /// Synchronize with all peers
    pub fn synchronize_with_peers(&mut self) -> Result<SyncResult> {
        let _connected_peers = self.peer_discovery.get_connected_peers();
        let updates: Vec<FieldUpdate> = Vec::new(); // Would fetch from peers

        self.distributed_field.sync(updates)
    }

    /// Broadcast a field update to all peers
    pub fn broadcast_field_update(&mut self, update: FieldUpdate) -> Result<()> {
        let peers = self.peer_discovery.get_connected_peers();

        for peer_id in peers {
            let message = NetworkMessage::new(
                MessageType::FieldUpdateMsg,
                self.message_handler.peer_id,
                Some(peer_id),
                MessagePayload::FieldUpdatePayload(update.clone()),
                MessagePriority::High,
                update.signature.clone(),
            );
            self.message_handler.sent_messages.push(message);
        }

        Ok(())
    }

    /// Handle peer disconnection
    pub fn handle_peer_disconnection(&mut self, peer_id: PeerId) -> Result<()> {
        self.peer_discovery.disconnect_from_peer(peer_id)?;
        self.distributed_field.remove_peer(peer_id)?;
        Ok(())
    }

    /// Get system status
    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus {
            peer_count: self.peer_discovery.peer_list.len(),
            connected_peers: self.peer_discovery.get_connected_peers().len(),
            field_version: self.distributed_field.field_version,
            field_coherence: self.distributed_field.get_field_state().coherence,
            last_sync_time: 0.0,
        }
    }
}

/// System status information
#[derive(Debug, Clone, PartialEq)]
pub struct SystemStatus {
    /// Total number of peers
    pub peer_count: usize,
    /// Number of connected peers
    pub connected_peers: usize,
    /// Current field version
    pub field_version: Version,
    /// Field coherence level
    pub field_coherence: Float,
    /// Last synchronization time
    pub last_sync_time: Timestamp,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic::complex_vectors::ComplexArchetype;
    use crate::holographic::holographic_field::InvolutionLayer;

    /// Helper function to create a test holographic field
    fn create_test_field() -> HolographicField {
        let archetypes = [ComplexArchetype {
            amplitude: 0.5,
            phase: 0.0,
        }; 22];
        HolographicField::new(InvolutionLayer::Green, archetypes)
    }

    // -------------------------------------------------------------------------
    // Tests for Core Identifiers
    // -------------------------------------------------------------------------

    #[test]
    fn test_field_id_creation() {
        let field_id = FieldId::new(123);
        assert_eq!(field_id.as_u64(), 123);
    }

    #[test]
    fn test_field_id_generate() {
        let field_id = FieldId::generate();
        assert_ne!(field_id.as_u64(), 0);
    }

    #[test]
    fn test_message_id_equality() {
        let id1 = MessageId::new(100);
        let id2 = MessageId::new(100);
        let id3 = MessageId::new(200);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_proposal_id_from_u64() {
        let proposal_id: ProposalId = 42.into();
        assert_eq!(proposal_id.as_u64(), 42);
    }

    #[test]
    fn test_conflict_id_display() {
        let conflict_id = ConflictId::new(999);
        assert_eq!(conflict_id.to_string(), "Conflict(999)");
    }

    // -------------------------------------------------------------------------
    // Tests for Change Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_change_type_display() {
        assert_eq!(ChangeType::EntityCreated.to_string(), "EntityCreated");
        assert_eq!(ChangeType::FieldCollapsed.to_string(), "FieldCollapsed");
    }

    #[test]
    fn test_resonance_pattern_creation() {
        let pattern = ResonancePattern::new(0.7, 0.8, 1.0, 0.9);
        assert_eq!(pattern.frequency, 0.7);
        assert_eq!(pattern.amplitude, 0.8);
    }

    #[test]
    fn test_resonance_pattern_clamping() {
        let pattern = ResonancePattern::new(1.5, -0.5, 10.0, 1.5);
        assert_eq!(pattern.frequency, 1.0); // Clamped to 1.0
        assert_eq!(pattern.amplitude, 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_coordinate3d_creation() {
        let coord = Coordinate3D::new(1.0, 2.0, 3.0);
        assert_eq!(coord.x, 1.0);
        assert_eq!(coord.y, 2.0);
        assert_eq!(coord.z, 3.0);
    }

    #[test]
    fn test_coordinate3d_distance() {
        let p1 = Coordinate3D::new(0.0, 0.0, 0.0);
        let p2 = Coordinate3D::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_field_change_apply_to() {
        let change = FieldChange {
            change_type: ChangeType::EntityCreated,
            entity_id: None,
            resonance_pattern: None,
            position: None,
            spectrum_ratio: None,
        };

        let mut field = create_test_field();
        let result = change.apply_to(&mut field);
        assert!(result.is_ok());
    }

    #[test]
    fn test_field_change_compatible_with() {
        let change1 = FieldChange {
            change_type: ChangeType::EntityModified,
            entity_id: Some(EntityId::new("entity-1".to_string())),
            resonance_pattern: Some(ResonancePattern::initial()),
            position: None,
            spectrum_ratio: None,
        };

        let change2 = FieldChange {
            change_type: ChangeType::EntityModified,
            entity_id: Some(EntityId::new("entity-2".to_string())),
            resonance_pattern: Some(ResonancePattern::initial()),
            position: None,
            spectrum_ratio: None,
        };

        assert!(change1.is_compatible_with(&change2));
    }

    // -------------------------------------------------------------------------
    // Tests for Field Update
    // -------------------------------------------------------------------------

    #[test]
    fn test_field_update_creation() {
        let signature = vec![1u8, 2u8, 3u8];
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, signature.clone());

        assert_eq!(update.from_peer.as_u64(), 1);
        assert_eq!(update.version, 1);
        assert_eq!(update.signature, signature);
    }

    #[test]
    fn test_field_update_validate() {
        let valid_update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8, 2u8, 3u8]);
        assert!(valid_update.validate());

        let invalid_update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, Vec::new());
        assert!(!invalid_update.validate());
    }

    #[test]
    fn test_field_update_get_change_count() {
        let changes = vec![
            FieldChange {
                change_type: ChangeType::EntityCreated,
                entity_id: None,
                resonance_pattern: None,
                position: None,
                spectrum_ratio: None,
            },
            FieldChange {
                change_type: ChangeType::EntityModified,
                entity_id: None,
                resonance_pattern: None,
                position: None,
                spectrum_ratio: None,
            },
        ];

        let update = FieldUpdate::new(PeerId::new(1), changes, 1, vec![1u8]);

        assert_eq!(update.get_change_count(), 2);
    }

    #[test]
    fn test_field_update_merge_with() {
        let change1 = FieldChange {
            change_type: ChangeType::EntityModified,
            entity_id: Some(EntityId::new("entity-1".to_string())),
            resonance_pattern: Some(ResonancePattern::initial()),
            position: None,
            spectrum_ratio: None,
        };

        let change2 = FieldChange {
            change_type: ChangeType::EntityModified,
            entity_id: Some(EntityId::new("entity-2".to_string())),
            resonance_pattern: None,
            position: Some(Coordinate3D::origin()),
            spectrum_ratio: None,
        };

        let update1 = FieldUpdate::new(PeerId::new(1), vec![change1], 1, vec![1u8]);

        let update2 = FieldUpdate::new(PeerId::new(1), vec![change2], 1, vec![2u8]);

        let merged = update1.merge_with(&update2);
        assert!(merged.is_some());
        assert_eq!(merged.unwrap().changes.len(), 2);
    }

    // -------------------------------------------------------------------------
    // Tests for Consensus Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_consensus_type_display() {
        assert_eq!(
            ConsensusType::ResonancePriority.to_string(),
            "ResonancePriority"
        );
        assert_eq!(ConsensusType::Voting.to_string(), "Voting");
    }

    #[test]
    fn test_vote_display() {
        assert_eq!(Vote::Approve.to_string(), "Approve");
        assert_eq!(Vote::Reject.to_string(), "Reject");
    }

    #[test]
    fn test_consensus_algorithm_creation() {
        let consensus = ConsensusAlgorithm::new(ConsensusType::LastWriteWins);
        assert_eq!(consensus.algorithm_type, ConsensusType::LastWriteWins);
    }

    #[test]
    fn test_consensus_algorithm_propose_update() {
        let mut consensus = ConsensusAlgorithm::new(ConsensusType::Voting);
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        let proposal_id = consensus.propose_update(update);
        assert!(proposal_id.is_ok());
    }

    #[test]
    fn test_consensus_algorithm_vote_on_update() {
        let mut consensus = ConsensusAlgorithm::new(ConsensusType::Voting);
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        let proposal_id = consensus.propose_update(update).unwrap();
        let result = consensus.vote_on_update(proposal_id, Vote::Approve);
        assert!(result.is_ok());
    }

    #[test]
    fn test_consensus_algorithm_resolve_conflict() {
        let mut consensus = ConsensusAlgorithm::new(ConsensusType::LastWriteWins);

        let conflict = Conflict::new(
            ConflictType::ConcurrentModification,
            vec![FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8])],
        );

        let resolution = consensus.resolve_conflict(conflict);
        assert!(resolution.is_ok());
        assert_eq!(
            resolution.unwrap().resolution_type,
            ResolutionType::ChosenUpdate
        );
    }

    // -------------------------------------------------------------------------
    // Tests for Conflict Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_conflict_type_display() {
        assert_eq!(
            ConflictType::ConcurrentModification.to_string(),
            "ConcurrentModification"
        );
        assert_eq!(
            ConflictType::ObserverConflict.to_string(),
            "ObserverConflict"
        );
    }

    #[test]
    fn test_conflict_creation() {
        let conflict = Conflict::new(ConflictType::VersionConflict, Vec::new());

        assert_eq!(conflict.conflict_type, ConflictType::VersionConflict);
        assert!(!conflict.resolution_required);
    }

    #[test]
    fn test_resolution_creation() {
        let resolution = Resolution::new(
            ResolutionType::MergedUpdate,
            None,
            None,
            "Test resolution".to_string(),
        );

        assert_eq!(resolution.resolution_type, ResolutionType::MergedUpdate);
        assert_eq!(resolution.rationale, "Test resolution");
    }

    // -------------------------------------------------------------------------
    // Tests for Peer Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_peer_info_creation() {
        let peer_info = PeerInfo::new(PeerId::new(1), "localhost:8080".to_string());
        assert_eq!(peer_info.peer_id.as_u64(), 1);
        assert_eq!(peer_info.address, "localhost:8080");
    }

    #[test]
    fn test_peer_info_is_connected() {
        let mut peer_info = PeerInfo::new(PeerId::new(1), "localhost:8080".to_string());
        assert!(!peer_info.is_connected());

        peer_info.connection_status = ConnectionStatus::Connected;
        assert!(peer_info.is_connected());
    }

    #[test]
    fn test_peer_discovery_creation() {
        let discovery = PeerDiscovery::new();
        assert!(discovery.discovery_enabled);
        assert_eq!(discovery.peer_list.len(), 0);
    }

    #[test]
    fn test_peer_discovery_add_peer() {
        let mut discovery = PeerDiscovery::new();
        discovery.add_peer(PeerId::new(1), "localhost:8080".to_string());

        assert_eq!(discovery.peer_list.len(), 1);
        assert!(discovery.known_peers.contains_key(&PeerId::new(1)));
    }

    #[test]
    fn test_peer_discovery_connect_to_peer() {
        let mut discovery = PeerDiscovery::new();
        discovery.add_peer(PeerId::new(1), "localhost:8080".to_string());

        let result = discovery.connect_to_peer(PeerId::new(1));
        assert!(result.is_ok());

        let peer_info = discovery.get_peer_info(PeerId::new(1)).unwrap();
        assert!(peer_info.is_connected());
    }

    #[test]
    fn test_peer_discovery_get_connected_peers() {
        let mut discovery = PeerDiscovery::new();
        discovery.add_peer(PeerId::new(1), "localhost:8080".to_string());
        discovery.add_peer(PeerId::new(2), "localhost:8081".to_string());

        discovery.connect_to_peer(PeerId::new(1)).unwrap();

        let connected = discovery.get_connected_peers();
        assert_eq!(connected.len(), 1);
        assert!(connected.contains(&PeerId::new(1)));
    }

    #[test]
    fn test_peer_discovery_remove_peer() {
        let mut discovery = PeerDiscovery::new();
        discovery.add_peer(PeerId::new(1), "localhost:8080".to_string());

        let result = discovery.remove_peer(PeerId::new(1));
        assert!(result.is_ok());
        assert_eq!(discovery.peer_list.len(), 0);
    }

    // -------------------------------------------------------------------------
    // Tests for Synchronization Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_peer_field_state_creation() {
        let field = create_test_field();
        let peer_state = PeerFieldState::new(PeerId::new(1), field);

        assert_eq!(peer_state.peer_id.as_u64(), 1);
        assert_eq!(peer_state.version, 0);
    }

    #[test]
    fn test_sync_result_creation() {
        let result = SyncResult::new(
            vec![UpdateId::new(1), UpdateId::new(2)],
            vec![ConflictId::new(1)],
            5,
            1.5,
        );

        assert_eq!(result.updates_applied.len(), 2);
        assert_eq!(result.conflicts_resolved.len(), 1);
        assert_eq!(result.new_field_version, 5);
        assert_eq!(result.sync_duration, 1.5);
    }

    #[test]
    fn test_field_synchronizer_enqueue_update() {
        let mut synchronizer = FieldSynchronizer::new();
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        let result = synchronizer.enqueue_update(update);
        assert!(result.is_ok());
        assert_eq!(synchronizer.sync_queue.len(), 1);
    }

    #[test]
    fn test_field_synchronizer_process_queue() {
        let mut synchronizer = FieldSynchronizer::new();
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        synchronizer.enqueue_update(update).unwrap();

        let results = synchronizer.process_queue();
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 1);
    }

    // -------------------------------------------------------------------------
    // Tests for Message Types
    // -------------------------------------------------------------------------

    #[test]
    fn test_message_type_display() {
        assert_eq!(MessageType::FieldUpdateMsg.to_string(), "FieldUpdateMsg");
        assert_eq!(MessageType::KeepAlive.to_string(), "KeepAlive");
    }

    #[test]
    fn test_message_payload_type() {
        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        let payload = MessagePayload::FieldUpdatePayload(update);
        assert_eq!(payload.message_type(), MessageType::FieldUpdateMsg);
    }

    #[test]
    fn test_network_message_creation() {
        let message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            Some(PeerId::new(2)),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );

        assert_eq!(message.sender.as_u64(), 1);
        assert_eq!(message.receiver, Some(PeerId::new(2)));
        assert_eq!(message.message_type, MessageType::KeepAlive);
    }

    #[test]
    fn test_network_message_validate() {
        let valid_message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            None,
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );
        assert!(valid_message.validate());

        let invalid_message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            None,
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            Vec::new(),
        );
        assert!(!invalid_message.validate());
    }

    #[test]
    fn test_network_message_is_broadcast() {
        let broadcast_message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            None,
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );
        assert!(broadcast_message.is_broadcast());

        let direct_message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            Some(PeerId::new(2)),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );
        assert!(!direct_message.is_broadcast());
    }

    #[test]
    fn test_network_message_is_for() {
        let message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(1),
            Some(PeerId::new(2)),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );

        assert!(message.is_for(PeerId::new(2)));
        assert!(!message.is_for(PeerId::new(3)));
    }

    // -------------------------------------------------------------------------
    // Tests for Distributed Field Error
    // -------------------------------------------------------------------------

    #[test]
    fn test_distributed_field_error_display() {
        let err = DistributedFieldError::InvalidUpdate("Test error".to_string());
        assert_eq!(err.to_string(), "Invalid update: Test error");

        let err = DistributedFieldError::PeerNotFound(PeerId::new(42));
        assert_eq!(err.to_string(), "Peer not found: Peer(42)");
    }

    #[test]
    fn test_distributed_field_error_version_mismatch() {
        let err = DistributedFieldError::VersionMismatch {
            expected: 5,
            actual: 3,
        };
        assert_eq!(err.to_string(), "Version mismatch: expected 5, got 3");
    }

    // -------------------------------------------------------------------------
    // Tests for Distributed Holographic Field
    // -------------------------------------------------------------------------

    #[test]
    fn test_distributed_holographic_field_creation() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();

        let distributed_field = DistributedHolographicField::new(field_id, local_field);

        assert_eq!(distributed_field.field_id.as_u64(), 1);
        assert_eq!(distributed_field.field_version, 0);
    }

    #[test]
    fn test_distributed_holographic_field_add_peer() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        let result = distributed_field.add_peer(PeerId::new(1));
        assert!(result.is_ok());
        assert_eq!(distributed_field.peer_count(), 1);
    }

    #[test]
    fn test_distributed_holographic_field_remove_peer() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        distributed_field.add_peer(PeerId::new(1)).unwrap();

        let result = distributed_field.remove_peer(PeerId::new(1));
        assert!(result.is_ok());
        assert_eq!(distributed_field.peer_count(), 0);
    }

    #[test]
    fn test_distributed_holographic_field_get_peer_view() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        distributed_field.add_peer(PeerId::new(1)).unwrap();

        let peer_view = distributed_field.get_peer_view(PeerId::new(1));
        assert!(peer_view.is_some());
    }

    #[test]
    fn test_distributed_holographic_field_get_local_view() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let spatial_frequency = local_field.spatial_frequency;
        let distributed_field = DistributedHolographicField::new(field_id, local_field);

        let local_view = distributed_field.get_local_view();
        assert_eq!(local_view.spatial_frequency, spatial_frequency);
    }

    #[test]
    fn test_distributed_holographic_field_get_field_state() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let distributed_field = DistributedHolographicField::new(field_id, local_field);

        let state = distributed_field.get_field_state();
        assert_eq!(state.version, 0);
        assert_eq!(state.coherence, 0.9);
    }

    #[test]
    fn test_distributed_holographic_field_apply_update() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 1, vec![1u8]);

        let result = distributed_field.apply_update(update);
        assert!(result.is_ok());
        assert_eq!(distributed_field.field_version, 1);
    }

    #[test]
    fn test_distributed_holographic_field_sync() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        let update = FieldUpdate::new(PeerId::new(1), Vec::new(), 0, vec![1u8]);

        let result = distributed_field.sync(vec![update]);
        assert!(result.is_ok());
        assert!(result.unwrap().updates_applied.len() == 1);
    }

    #[test]
    fn test_distributed_holographic_field_sync_invalid_signature() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let mut distributed_field = DistributedHolographicField::new(field_id, local_field);

        let update = FieldUpdate::new(
            PeerId::new(1),
            Vec::new(),
            0,
            Vec::new(), // Empty signature
        );

        let result = distributed_field.sync(vec![update]);
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Tests for Message Handler
    // -------------------------------------------------------------------------

    #[test]
    fn test_message_handler_creation() {
        let handler = MessageHandler::new(PeerId::new(1));
        assert_eq!(handler.peer_id.as_u64(), 1);
    }

    #[test]
    fn test_message_handler_handle_keep_alive() {
        let mut handler = MessageHandler::new(PeerId::new(1));

        let message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(2),
            Some(PeerId::new(1)),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );

        let result = handler.handle_message(message);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_message_handler_handle_invalid_signature() {
        let mut handler = MessageHandler::new(PeerId::new(1));

        let message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(2),
            Some(PeerId::new(1)),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            Vec::new(),
        );

        let result = handler.handle_message(message);
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Tests for Distributed Holographic Field System
    // -------------------------------------------------------------------------

    #[test]
    fn test_distributed_holographic_field_system_creation() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);

        let system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        assert_eq!(system.message_handler.peer_id.as_u64(), 1);
        assert_eq!(system.distributed_field.field_id.as_u64(), 1);
    }

    #[test]
    fn test_distributed_holographic_field_system_initialize() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);
        let mut system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        let result = system.initialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_distributed_holographic_field_system_process_messages() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);
        let mut system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        let message = NetworkMessage::new(
            MessageType::KeepAlive,
            PeerId::new(2),
            Some(peer_id),
            MessagePayload::KeepAlivePayload,
            MessagePriority::Normal,
            vec![1u8],
        );

        let result = system.process_messages(vec![message]);
        assert!(result.is_ok());
        assert!(result.unwrap().len() == 1);
    }

    #[test]
    fn test_distributed_holographic_field_system_broadcast_update() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);
        let mut system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        // Add a connected peer
        system
            .peer_discovery
            .add_peer(PeerId::new(2), "localhost:8080".to_string());
        system
            .peer_discovery
            .connect_to_peer(PeerId::new(2))
            .unwrap();

        let update = FieldUpdate::new(peer_id, Vec::new(), 1, vec![1u8]);

        let result = system.broadcast_field_update(update);
        assert!(result.is_ok());
    }

    #[test]
    fn test_distributed_holographic_field_system_handle_peer_disconnection() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);
        let mut system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        // Add a peer
        system
            .peer_discovery
            .add_peer(PeerId::new(2), "localhost:8080".to_string());
        system.distributed_field.add_peer(PeerId::new(2)).unwrap();

        let result = system.handle_peer_disconnection(PeerId::new(2));
        assert!(result.is_ok());
        assert_eq!(system.distributed_field.peer_count(), 0);
    }

    #[test]
    fn test_distributed_holographic_field_system_get_system_status() {
        let field_id = FieldId::new(1);
        let local_field = create_test_field();
        let peer_id = PeerId::new(1);
        let system = DistributedHolographicFieldSystem::new(field_id, local_field, peer_id);

        let status = system.get_system_status();
        assert_eq!(status.peer_count, 0);
        assert_eq!(status.connected_peers, 0);
        assert_eq!(status.field_version, 0);
    }
}
