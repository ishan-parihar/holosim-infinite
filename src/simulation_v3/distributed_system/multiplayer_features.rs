//! Multiplayer Features - Week 109-112
//!
//! This module implements advanced multiplayer features for the distributed holographic field:
//! - Collective manifestation: Multiple players contribute resonance to create structures
//! - Density sharing: Simultaneous exploration of multiple densities
//! - Scale sharing: Simultaneous exploration of multiple scales
//! - Network optimization: Bandwidth and latency optimization
//!
//! From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Week 109-112:
//! "Multiplayer Features (Density/Scale Sharing) - Finalizes the multiplayer experience
//! with density and scale interactions"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The holographic principle states that each entity contains the whole. In a distributed
//! system, this principle extends to peer instances, where each peer contains the entire
//! field state in a compressed form."

use crate::simulation_v3::collective_manifestation::holographic_structure::{
    HolographicStructure, StructureProperties, StructureType,
};
use crate::simulation_v3::distributed_system::{
    distributed_holographic_field::ResonancePattern, PeerId, Timestamp,
};
use crate::simulation_v3::holographic_physics::SpectrumRatio;
use crate::types::Density;
use crate::types::Float;
use std::collections::{HashMap, VecDeque};
use std::fmt;

// ============================================================================
// Type Aliases and Identifiers
// ============================================================================

/// Unique identifier for a collective manifestation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManifestationId(pub u64);

impl ManifestationId {
    pub fn new(id: u64) -> Self {
        ManifestationId(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ManifestationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manifestation({})", self.0)
    }
}

impl Default for ManifestationId {
    fn default() -> Self {
        ManifestationId(0)
    }
}

impl From<u64> for ManifestationId {
    fn from(id: u64) -> Self {
        ManifestationId(id)
    }
}

/// Entity identifier for multiplayer features
pub type EntityId = u64;

/// Catalyst amount for density sharing
pub type CatalystAmount = Float;

// ============================================================================
// Collective Manifestation System
// ============================================================================

/// State of a collective manifestation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestationState {
    /// Gathering participants
    Gathering,
    /// Building collective resonance
    ResonanceBuilding,
    /// Structure is actively manifesting
    ManifestationActive,
    /// Structure fully manifested
    Complete,
    /// Manifestation failed
    Failed,
}

impl fmt::Display for ManifestationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifestationState::Gathering => write!(f, "Gathering"),
            ManifestationState::ResonanceBuilding => write!(f, "Resonance Building"),
            ManifestationState::ManifestationActive => write!(f, "Manifestation Active"),
            ManifestationState::Complete => write!(f, "Complete"),
            ManifestationState::Failed => write!(f, "Failed"),
        }
    }
}

impl Default for ManifestationState {
    fn default() -> Self {
        ManifestationState::Gathering
    }
}

/// Type of contribution to collective manifestation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContributionType {
    /// Direct resonance contribution
    DirectResonance,
    /// Catalyst contribution
    Catalyst,
    /// Architectural contribution (design)
    Architectural,
    /// Energetic contribution
    Energetic,
    /// Spiritual contribution
    Spiritual,
}

impl fmt::Display for ContributionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContributionType::DirectResonance => write!(f, "Direct Resonance"),
            ContributionType::Catalyst => write!(f, "Catalyst"),
            ContributionType::Architectural => write!(f, "Architectural"),
            ContributionType::Energetic => write!(f, "Energetic"),
            ContributionType::Spiritual => write!(f, "Spiritual"),
        }
    }
}

impl Default for ContributionType {
    fn default() -> Self {
        ContributionType::DirectResonance
    }
}

/// Player's contribution to collective manifestation
#[derive(Debug, Clone, PartialEq)]
pub struct ResonanceContribution {
    /// Player who made the contribution
    pub player_id: PeerId,
    /// Amount contributed (0.0 to 1.0)
    pub contribution_amount: Float,
    /// Type of contribution
    pub contribution_type: ContributionType,
    /// Resonance pattern of contribution
    pub resonance_pattern: ResonancePattern,
    /// When the contribution was made
    pub contribution_timestamp: Timestamp,
}

impl ResonanceContribution {
    pub fn new(
        player_id: PeerId,
        contribution_amount: Float,
        contribution_type: ContributionType,
        resonance_pattern: ResonancePattern,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            player_id,
            contribution_amount: contribution_amount.clamp(0.0, 1.0),
            contribution_type,
            resonance_pattern,
            contribution_timestamp: timestamp,
        }
    }
}

impl Default for ResonanceContribution {
    fn default() -> Self {
        Self {
            player_id: PeerId::default(),
            contribution_amount: 0.0,
            contribution_type: ContributionType::default(),
            resonance_pattern: ResonancePattern::default(),
            contribution_timestamp: 0.0,
        }
    }
}

/// A collective manifestation being built by multiple players
#[derive(Debug, Clone, PartialEq)]
pub struct CollectiveManifestation {
    /// Unique identifier for this manifestation
    pub manifestation_id: ManifestationId,
    /// Players participating in the manifestation
    pub participating_players: Vec<PeerId>,
    /// Type of structure being manifested
    pub target_structure_type: StructureType,
    /// Current collective resonance
    pub collective_resonance: ResonancePattern,
    /// Required resonance for completion
    pub required_resonance: ResonancePattern,
    /// Progress towards completion (0.0 to 1.0)
    pub current_progress: Float,
    /// Track each player's contribution
    pub contribution_tracker: HashMap<PeerId, ResonanceContribution>,
    /// Current state of manifestation
    pub manifestation_state: ManifestationState,
    /// When the manifestation was created
    pub creation_timestamp: Timestamp,
}

impl CollectiveManifestation {
    /// Create a new collective manifestation
    pub fn new(
        manifestation_id: ManifestationId,
        players: Vec<PeerId>,
        structure_type: StructureType,
        required_resonance: ResonancePattern,
    ) -> Self {
        Self {
            manifestation_id,
            participating_players: players,
            target_structure_type: structure_type,
            collective_resonance: ResonancePattern::default(),
            required_resonance,
            current_progress: 0.0,
            contribution_tracker: HashMap::new(),
            manifestation_state: ManifestationState::Gathering,
            creation_timestamp: Self::get_current_timestamp(),
        }
    }

    /// Add a participant to the manifestation
    pub fn add_participant(&mut self, player_id: PeerId) -> Result<(), MultiplayerFeaturesError> {
        if self.participating_players.contains(&player_id) {
            return Err(MultiplayerFeaturesError::PlayerNotFound);
        }
        self.participating_players.push(player_id);
        Ok(())
    }

    /// Remove a participant from the manifestation
    pub fn remove_participant(
        &mut self,
        player_id: PeerId,
    ) -> Result<(), MultiplayerFeaturesError> {
        if let Some(pos) = self
            .participating_players
            .iter()
            .position(|&id| id == player_id)
        {
            self.participating_players.remove(pos);
            self.contribution_tracker.remove(&player_id);
            Ok(())
        } else {
            Err(MultiplayerFeaturesError::PlayerNotFound)
        }
    }

    /// Contribute resonance to the manifestation
    pub fn contribute_resonance(
        &mut self,
        player_id: PeerId,
        contribution: ResonanceContribution,
    ) -> Result<(), MultiplayerFeaturesError> {
        if !self.participating_players.contains(&player_id) {
            return Err(MultiplayerFeaturesError::PlayerNotFound);
        }

        // Update contribution tracker
        self.contribution_tracker
            .insert(player_id, contribution.clone());

        // Update collective resonance by blending with contribution
        self.collective_resonance = self.blend_resonance_patterns(
            &self.collective_resonance,
            &contribution.resonance_pattern,
            contribution.contribution_amount,
        );

        // Update progress
        self.update_progress()?;

        // Update state based on progress
        if self.current_progress >= 1.0 {
            self.manifestation_state = ManifestationState::Complete;
        } else if self.current_progress > 0.0 {
            if self.manifestation_state == ManifestationState::Gathering {
                self.manifestation_state = ManifestationState::ResonanceBuilding;
            }
            if self.current_progress > 0.5 {
                self.manifestation_state = ManifestationState::ManifestationActive;
            }
        }

        Ok(())
    }

    /// Update progress based on collective resonance vs required resonance
    pub fn update_progress(&mut self) -> Result<(), MultiplayerFeaturesError> {
        let resonance_progress = self.calculate_resonance_progress();
        self.current_progress = resonance_progress;
        Ok(())
    }

    /// Check if manifestation is complete
    pub fn is_complete(&self) -> bool {
        self.manifestation_state == ManifestationState::Complete && self.current_progress >= 1.0
    }

    /// Get the manifested structure if complete
    pub fn get_manifestation_result(&self) -> Option<HolographicStructure> {
        if self.is_complete() {
            // Create a simple holographic structure
            // In a full implementation, this would use the collective resonance result
            let structure = HolographicStructure {
                structure_id: self.manifestation_id.as_u64(),
                structure_type: self.target_structure_type,
                contributors: self
                    .participating_players
                    .iter()
                    .map(|p| p.as_u64())
                    .collect(),
                holographic_signature: [0.5; 22],
                spectrum_ratio: SpectrumRatio {
                    space_time_ratio: 1.0,
                    time_space_ratio: 1.0,
                    spectrum_position: 0.5,
                },
                structure_properties: StructureProperties::default(),
                build_progress: 1.0,
                required_resonance: self.required_resonance.coherence,
                current_resonance: self.collective_resonance.coherence,
                stability: 1.0,
                position: [0.0, 0.0, 0.0],
                memory_key: None,
            };
            Some(structure)
        } else {
            None
        }
    }

    /// Calculate progress based on resonance matching
    fn calculate_resonance_progress(&self) -> Float {
        let freq_ratio = (self.collective_resonance.frequency / self.required_resonance.frequency)
            .clamp(0.0, 1.0);
        let amp_ratio = (self.collective_resonance.amplitude / self.required_resonance.amplitude)
            .clamp(0.0, 1.0);
        let coh_ratio = (self.collective_resonance.coherence / self.required_resonance.coherence)
            .clamp(0.0, 1.0);

        (freq_ratio + amp_ratio + coh_ratio) / 3.0
    }

    /// Blend two resonance patterns
    fn blend_resonance_patterns(
        &self,
        base: &ResonancePattern,
        addition: &ResonancePattern,
        weight: Float,
    ) -> ResonancePattern {
        ResonancePattern {
            frequency: base.frequency + (addition.frequency - base.frequency) * weight,
            amplitude: base.amplitude + (addition.amplitude - base.amplitude) * weight,
            phase: base.phase + (addition.phase - base.phase) * weight,
            coherence: base.coherence + (addition.coherence - base.coherence) * weight,
        }
    }

    fn get_current_timestamp() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

impl Default for CollectiveManifestation {
    fn default() -> Self {
        Self {
            manifestation_id: ManifestationId::default(),
            participating_players: Vec::new(),
            target_structure_type: StructureType::CommunityCenter,
            collective_resonance: ResonancePattern::default(),
            required_resonance: ResonancePattern::default(),
            current_progress: 0.0,
            contribution_tracker: HashMap::new(),
            manifestation_state: ManifestationState::default(),
            creation_timestamp: 0.0,
        }
    }
}

/// Manager for all collective manifestations
#[derive(Debug, Clone, PartialEq)]
pub struct CollectiveManifestationManager {
    /// Active manifestations
    pub active_manifestations: HashMap<ManifestationId, CollectiveManifestation>,
    /// Completed manifestations
    pub completed_manifestations: Vec<CollectiveManifestation>,
}

impl CollectiveManifestationManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self {
            active_manifestations: HashMap::new(),
            completed_manifestations: Vec::new(),
        }
    }

    /// Create a new manifestation
    pub fn create_manifestation(
        &mut self,
        players: Vec<PeerId>,
        structure_type: StructureType,
    ) -> Result<ManifestationId, MultiplayerFeaturesError> {
        if players.is_empty() {
            return Err(MultiplayerFeaturesError::InsufficientResources);
        }

        let id = ManifestationId::new(self.active_manifestations.len() as u64 + 1);
        let required_resonance = ResonancePattern::new(
            structure_type.base_resonance_requirement() / 100.0,
            0.8,
            0.0,
            0.9,
        );

        let manifestation =
            CollectiveManifestation::new(id, players, structure_type, required_resonance);
        self.active_manifestations.insert(id, manifestation);
        Ok(id)
    }

    /// Get a manifestation by ID
    pub fn get_manifestation(&self, id: ManifestationId) -> Option<&CollectiveManifestation> {
        self.active_manifestations.get(&id)
    }

    /// Update all manifestations
    pub fn update_all_manifestations(&mut self) -> Result<(), MultiplayerFeaturesError> {
        let mut completed = Vec::new();
        for (id, manifestation) in self.active_manifestations.iter_mut() {
            manifestation.update_progress()?;
            if manifestation.is_complete() {
                completed.push(*id);
            }
        }

        // Move completed manifestations
        for id in completed {
            if let Some(manifestation) = self.active_manifestations.remove(&id) {
                self.completed_manifestations.push(manifestation);
            }
        }

        Ok(())
    }

    /// Complete a manifestation and get the structure
    pub fn complete_manifestation(
        &mut self,
        id: ManifestationId,
    ) -> Result<HolographicStructure, MultiplayerFeaturesError> {
        if let Some(manifestation) = self.active_manifestations.remove(&id) {
            if let Some(structure) = manifestation.get_manifestation_result() {
                self.completed_manifestations.push(manifestation);
                Ok(structure)
            } else {
                Err(MultiplayerFeaturesError::ManifestationFailed)
            }
        } else {
            Err(MultiplayerFeaturesError::PlayerNotFound)
        }
    }
}

impl Default for CollectiveManifestationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Density Sharing System
// ============================================================================

/// State of a shared density
#[derive(Debug, Clone, PartialEq)]
pub struct DensityState {
    /// Entities in this density
    pub entities: Vec<EntityId>,
    /// Resonance field of this density
    pub resonance_field: ResonancePattern,
    /// Collective catalyst amount
    pub collective_catalyst: CatalystAmount,
    /// Evolution rate
    pub evolution_rate: Float,
}

impl DensityState {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            resonance_field: ResonancePattern::default(),
            collective_catalyst: 0.0,
            evolution_rate: 1.0,
        }
    }
}

impl Default for DensityState {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared density information
#[derive(Debug, Clone, PartialEq)]
pub struct DensityShare {
    /// Density being shared
    pub density: Density,
    /// Players sharing this density
    pub sharing_players: Vec<PeerId>,
    /// Current state of the density
    pub density_state: DensityState,
    /// Resonance signature
    pub resonance_signature: ResonancePattern,
    /// Last update timestamp
    pub last_update: Timestamp,
}

impl DensityShare {
    pub fn new(density: Density) -> Self {
        Self {
            density,
            sharing_players: Vec::new(),
            density_state: DensityState::new(),
            resonance_signature: ResonancePattern::default(),
            last_update: 0.0,
        }
    }
}

impl Default for DensityShare {
    fn default() -> Self {
        Self::new(Density::First)
    }
}

/// Type of density update
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensityUpdateType {
    /// Entity added
    EntityAdded,
    /// Entity removed
    EntityRemoved,
    /// Resonance field updated
    ResonanceUpdated,
    /// Catalyst updated
    CatalystUpdated,
    /// Full state sync
    FullSync,
}

impl fmt::Display for DensityUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DensityUpdateType::EntityAdded => write!(f, "Entity Added"),
            DensityUpdateType::EntityRemoved => write!(f, "Entity Removed"),
            DensityUpdateType::ResonanceUpdated => write!(f, "Resonance Updated"),
            DensityUpdateType::CatalystUpdated => write!(f, "Catalyst Updated"),
            DensityUpdateType::FullSync => write!(f, "Full Sync"),
        }
    }
}

impl Default for DensityUpdateType {
    fn default() -> Self {
        DensityUpdateType::FullSync
    }
}

/// Data for a density update
#[derive(Debug, Clone, PartialEq)]
pub enum DensityUpdateData {
    EntityAdded { entity_id: EntityId },
    EntityRemoved { entity_id: EntityId },
    ResonanceUpdated { new_pattern: ResonancePattern },
    CatalystUpdated { new_amount: CatalystAmount },
    FullSync { state: DensityState },
}

impl Default for DensityUpdateData {
    fn default() -> Self {
        DensityUpdateData::FullSync {
            state: DensityState::default(),
        }
    }
}

/// Update to a density state
#[derive(Debug, Clone, PartialEq)]
pub struct DensityUpdate {
    /// Density being updated
    pub density: Density,
    /// Type of update
    pub update_type: DensityUpdateType,
    /// When the update occurred
    pub timestamp: Timestamp,
    /// Update data
    pub update_data: DensityUpdateData,
}

impl DensityUpdate {
    pub fn new(density: Density, update_type: DensityUpdateType, data: DensityUpdateData) -> Self {
        Self {
            density,
            update_type,
            timestamp: Self::get_current_timestamp(),
            update_data: data,
        }
    }

    fn get_current_timestamp() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

/// Status of density synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    /// Synchronized
    Synchronized,
    /// Synchronizing
    Synchronizing,
    /// Out of sync
    OutOfSync,
    /// Error during sync
    Error,
}

impl fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncStatus::Synchronized => write!(f, "Synchronized"),
            SyncStatus::Synchronizing => write!(f, "Synchronizing"),
            SyncStatus::OutOfSync => write!(f, "Out of Sync"),
            SyncStatus::Error => write!(f, "Error"),
        }
    }
}

impl Default for SyncStatus {
    fn default() -> Self {
        SyncStatus::Synchronized
    }
}

/// Synchronizes density states across players
#[derive(Debug, Clone, PartialEq)]
pub struct DensitySynchronization {
    /// Queue of pending updates
    pub sync_queue: VecDeque<DensityUpdate>,
}

impl DensitySynchronization {
    pub fn new() -> Self {
        Self {
            sync_queue: VecDeque::new(),
        }
    }

    /// Add an update to the sync queue
    pub fn enqueue_update(&mut self, update: DensityUpdate) {
        self.sync_queue.push_back(update);
    }

    /// Process all updates in the queue
    pub fn process_updates(&mut self) -> Result<(), MultiplayerFeaturesError> {
        // Process all updates (in a real implementation, this would send them to peers)
        while let Some(_update) = self.sync_queue.pop_front() {
            // Process update
        }
        Ok(())
    }

    /// Get current sync status
    pub fn get_sync_status(&self) -> SyncStatus {
        if self.sync_queue.is_empty() {
            SyncStatus::Synchronized
        } else {
            SyncStatus::Synchronizing
        }
    }
}

impl Default for DensitySynchronization {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for density sharing
#[derive(Debug, Clone, PartialEq)]
pub struct DensityStatistics {
    /// Total shared densities
    pub total_shared: usize,
    /// Total players across all densities
    pub total_players: usize,
    /// Average resonance across densities
    pub average_resonance: Float,
    /// Total catalyst across all densities
    pub total_catalyst: Float,
}

impl DensityStatistics {
    pub fn new() -> Self {
        Self {
            total_shared: 0,
            total_players: 0,
            average_resonance: 0.0,
            total_catalyst: 0.0,
        }
    }
}

impl Default for DensityStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages density sharing across players
#[derive(Debug, Clone, PartialEq)]
pub struct DensitySharing {
    /// Shared densities
    pub shared_densities: HashMap<Density, DensityShare>,
    /// Active density views per player
    pub active_density_views: HashMap<PeerId, Vec<Density>>,
    /// Synchronization manager
    pub density_synchronization: DensitySynchronization,
}

impl DensitySharing {
    pub fn new() -> Self {
        Self {
            shared_densities: HashMap::new(),
            active_density_views: HashMap::new(),
            density_synchronization: DensitySynchronization::new(),
        }
    }

    /// Enable density sharing for specific densities
    pub fn enable_density_sharing(
        &mut self,
        densities: Vec<Density>,
    ) -> Result<(), MultiplayerFeaturesError> {
        for density in densities {
            if !self.shared_densities.contains_key(&density) {
                self.shared_densities
                    .insert(density, DensityShare::new(density));
            }
        }
        Ok(())
    }

    /// Disable density sharing for specific densities
    pub fn disable_density_sharing(
        &mut self,
        densities: Vec<Density>,
    ) -> Result<(), MultiplayerFeaturesError> {
        for density in densities {
            self.shared_densities.remove(&density);
            // Remove from all player views
            for view in self.active_density_views.values_mut() {
                view.retain(|d| d != &density);
            }
        }
        Ok(())
    }

    /// Get a player's current density view
    pub fn get_player_density_view(&self, player_id: PeerId) -> Vec<Density> {
        self.active_density_views
            .get(&player_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Set a player's density view
    pub fn set_player_density_view(
        &mut self,
        player_id: PeerId,
        densities: Vec<Density>,
    ) -> Result<(), MultiplayerFeaturesError> {
        // Validate densities are shared
        for density in &densities {
            if !self.shared_densities.contains_key(density) {
                return Err(MultiplayerFeaturesError::DensitySharingFailed);
            }
        }
        self.active_density_views.insert(player_id, densities);
        Ok(())
    }

    /// Synchronize all density views
    pub fn sync_density_views(&mut self) -> Result<SyncResult, MultiplayerFeaturesError> {
        self.density_synchronization.process_updates()?;
        Ok(SyncResult {
            success: true,
            synced_players: self.active_density_views.len(),
            message: "Density views synchronized".to_string(),
        })
    }

    /// Get density sharing statistics
    pub fn get_density_statistics(&self) -> DensityStatistics {
        let total_shared = self.shared_densities.len();
        let total_players: usize = self
            .shared_densities
            .values()
            .map(|s| s.sharing_players.len())
            .sum();

        let total_resonance: Float = self
            .shared_densities
            .values()
            .map(|s| s.resonance_signature.coherence)
            .sum();

        let average_resonance = if total_shared > 0 {
            total_resonance / total_shared as Float
        } else {
            0.0
        };

        let total_catalyst: Float = self
            .shared_densities
            .values()
            .map(|s| s.density_state.collective_catalyst)
            .sum();

        DensityStatistics {
            total_shared,
            total_players,
            average_resonance,
            total_catalyst,
        }
    }
}

impl Default for DensitySharing {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a synchronization operation
#[derive(Debug, Clone, PartialEq)]
pub struct SyncResult {
    /// Whether the sync succeeded
    pub success: bool,
    /// Number of players synced
    pub synced_players: usize,
    /// Status message
    pub message: String,
}

impl Default for SyncResult {
    fn default() -> Self {
        Self {
            success: false,
            synced_players: 0,
            message: String::new(),
        }
    }
}

// ============================================================================
// Scale Sharing System
// ============================================================================

/// Scale level in the holographic universe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaleLevel {
    /// Planck scale (10^-35 m)
    Quantum,
    /// Atomic scale (10^-10 m)
    Atomic,
    /// Molecular scale (10^-9 m)
    Molecular,
    /// Cellular scale (10^-6 m)
    Cellular,
    /// Planetary scale (10^7 m)
    Planetary,
    /// Solar system scale (10^13 m)
    Solar,
    /// Galactic scale (10^21 m)
    Galactic,
    /// Cosmic scale (10^26 m)
    Cosmic,
}

impl fmt::Display for ScaleLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaleLevel::Quantum => write!(f, "Quantum"),
            ScaleLevel::Atomic => write!(f, "Atomic"),
            ScaleLevel::Molecular => write!(f, "Molecular"),
            ScaleLevel::Cellular => write!(f, "Cellular"),
            ScaleLevel::Planetary => write!(f, "Planetary"),
            ScaleLevel::Solar => write!(f, "Solar"),
            ScaleLevel::Galactic => write!(f, "Galactic"),
            ScaleLevel::Cosmic => write!(f, "Cosmic"),
        }
    }
}

impl Default for ScaleLevel {
    fn default() -> Self {
        ScaleLevel::Cellular
    }
}

impl ScaleLevel {
    /// Get the magnitude order of this scale
    pub fn magnitude(&self) -> i32 {
        match self {
            ScaleLevel::Quantum => -35,
            ScaleLevel::Atomic => -10,
            ScaleLevel::Molecular => -9,
            ScaleLevel::Cellular => -6,
            ScaleLevel::Planetary => 7,
            ScaleLevel::Solar => 13,
            ScaleLevel::Galactic => 21,
            ScaleLevel::Cosmic => 26,
        }
    }
}

/// Scale identifier
pub type Scale = ScaleLevel;

/// Spatial dimensions
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialDimensions {
    /// Width
    pub width: Float,
    /// Height
    pub height: Float,
    /// Depth
    pub depth: Float,
}

impl SpatialDimensions {
    pub fn new(width: Float, height: Float, depth: Float) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn volume(&self) -> Float {
        self.width * self.height * self.depth
    }
}

impl Default for SpatialDimensions {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

/// Temporal dimensions
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalDimensions {
    /// Time dilation factor
    pub time_dilation: Float,
    /// Temporal coherence
    pub temporal_coherence: Float,
}

impl TemporalDimensions {
    pub fn new(time_dilation: Float, temporal_coherence: Float) -> Self {
        Self {
            time_dilation: time_dilation.clamp(0.0, 1.0),
            temporal_coherence: temporal_coherence.clamp(0.0, 1.0),
        }
    }
}

impl Default for TemporalDimensions {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

/// Holographic field for a scale
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicField {
    /// Field intensity
    pub intensity: Float,
    /// Field coherence
    pub coherence: Float,
    /// Field complexity
    pub complexity: Float,
}

impl HolographicField {
    pub fn new(intensity: Float, coherence: Float, complexity: Float) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            coherence: coherence.clamp(0.0, 1.0),
            complexity: complexity.clamp(0.0, 1.0),
        }
    }
}

impl Default for HolographicField {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

/// State of a shared scale
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleState {
    /// Scale level
    pub scale_level: ScaleLevel,
    /// Entities at this scale
    pub entities: Vec<EntityId>,
    /// Spatial dimensions
    pub spatial_dimensions: SpatialDimensions,
    /// Temporal dimensions
    pub temporal_dimensions: TemporalDimensions,
    /// Resonance signature
    pub resonance_signature: ResonancePattern,
}

impl ScaleState {
    pub fn new(scale_level: ScaleLevel) -> Self {
        Self {
            scale_level,
            entities: Vec::new(),
            spatial_dimensions: SpatialDimensions::default(),
            temporal_dimensions: TemporalDimensions::default(),
            resonance_signature: ResonancePattern::default(),
        }
    }
}

impl Default for ScaleState {
    fn default() -> Self {
        Self::new(ScaleLevel::default())
    }
}

/// Shared scale information
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleShare {
    /// Scale being shared
    pub scale: Scale,
    /// Players sharing this scale
    pub sharing_players: Vec<PeerId>,
    /// Current state of the scale
    pub scale_state: ScaleState,
    /// Holographic field
    pub holographic_field: HolographicField,
    /// Last update timestamp
    pub last_update: Timestamp,
}

impl ScaleShare {
    pub fn new(scale: Scale) -> Self {
        Self {
            scale,
            sharing_players: Vec::new(),
            scale_state: ScaleState::new(scale),
            holographic_field: HolographicField::default(),
            last_update: 0.0,
        }
    }
}

impl Default for ScaleShare {
    fn default() -> Self {
        Self::new(ScaleLevel::default())
    }
}

/// Synchronizes scale states across players
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleSynchronization {
    /// Queue of pending updates
    pub sync_queue: VecDeque<ScaleUpdate>,
}

impl ScaleSynchronization {
    pub fn new() -> Self {
        Self {
            sync_queue: VecDeque::new(),
        }
    }

    /// Add an update to the sync queue
    pub fn enqueue_update(&mut self, update: ScaleUpdate) {
        self.sync_queue.push_back(update);
    }

    /// Process all updates in the queue
    pub fn process_updates(&mut self) -> Result<(), MultiplayerFeaturesError> {
        while let Some(_update) = self.sync_queue.pop_front() {
            // Process update
        }
        Ok(())
    }

    /// Get current sync status
    pub fn get_sync_status(&self) -> SyncStatus {
        if self.sync_queue.is_empty() {
            SyncStatus::Synchronized
        } else {
            SyncStatus::Synchronizing
        }
    }
}

impl Default for ScaleSynchronization {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of scale update
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleUpdateType {
    /// Entity added
    EntityAdded,
    /// Entity removed
    EntityRemoved,
    /// Spatial dimensions updated
    SpatialUpdated,
    /// Temporal dimensions updated
    TemporalUpdated,
    /// Full state sync
    FullSync,
}

impl fmt::Display for ScaleUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaleUpdateType::EntityAdded => write!(f, "Entity Added"),
            ScaleUpdateType::EntityRemoved => write!(f, "Entity Removed"),
            ScaleUpdateType::SpatialUpdated => write!(f, "Spatial Updated"),
            ScaleUpdateType::TemporalUpdated => write!(f, "Temporal Updated"),
            ScaleUpdateType::FullSync => write!(f, "Full Sync"),
        }
    }
}

impl Default for ScaleUpdateType {
    fn default() -> Self {
        ScaleUpdateType::FullSync
    }
}

/// Data for a scale update
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleUpdateData {
    EntityAdded { entity_id: EntityId },
    EntityRemoved { entity_id: EntityId },
    SpatialUpdated { dimensions: SpatialDimensions },
    TemporalUpdated { dimensions: TemporalDimensions },
    FullSync { state: ScaleState },
}

impl Default for ScaleUpdateData {
    fn default() -> Self {
        ScaleUpdateData::FullSync {
            state: ScaleState::default(),
        }
    }
}

/// Update to a scale state
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleUpdate {
    /// Scale being updated
    pub scale: Scale,
    /// Type of update
    pub update_type: ScaleUpdateType,
    /// When the update occurred
    pub timestamp: Timestamp,
    /// Update data
    pub update_data: ScaleUpdateData,
}

impl ScaleUpdate {
    pub fn new(scale: Scale, update_type: ScaleUpdateType, data: ScaleUpdateData) -> Self {
        Self {
            scale,
            update_type,
            timestamp: Self::get_current_timestamp(),
            update_data: data,
        }
    }

    fn get_current_timestamp() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

/// Statistics for scale sharing
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleStatistics {
    /// Total shared scales
    pub total_shared: usize,
    /// Total players across all scales
    pub total_players: usize,
    /// Average field intensity
    pub average_intensity: Float,
    /// Total entities across all scales
    pub total_entities: usize,
}

impl ScaleStatistics {
    pub fn new() -> Self {
        Self {
            total_shared: 0,
            total_players: 0,
            average_intensity: 0.0,
            total_entities: 0,
        }
    }
}

impl Default for ScaleStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages scale sharing across players
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleSharing {
    /// Shared scales
    pub shared_scales: HashMap<Scale, ScaleShare>,
    /// Active scale views per player
    pub active_scale_views: HashMap<PeerId, Vec<Scale>>,
    /// Synchronization manager
    pub scale_synchronization: ScaleSynchronization,
}

impl ScaleSharing {
    pub fn new() -> Self {
        Self {
            shared_scales: HashMap::new(),
            active_scale_views: HashMap::new(),
            scale_synchronization: ScaleSynchronization::new(),
        }
    }

    /// Enable scale sharing for specific scales
    pub fn enable_scale_sharing(
        &mut self,
        scales: Vec<Scale>,
    ) -> Result<(), MultiplayerFeaturesError> {
        for scale in scales {
            if !self.shared_scales.contains_key(&scale) {
                self.shared_scales.insert(scale, ScaleShare::new(scale));
            }
        }
        Ok(())
    }

    /// Disable scale sharing for specific scales
    pub fn disable_scale_sharing(
        &mut self,
        scales: Vec<Scale>,
    ) -> Result<(), MultiplayerFeaturesError> {
        for scale in scales {
            self.shared_scales.remove(&scale);
            // Remove from all player views
            for view in self.active_scale_views.values_mut() {
                view.retain(|s| s != &scale);
            }
        }
        Ok(())
    }

    /// Get a player's current scale view
    pub fn get_player_scale_view(&self, player_id: PeerId) -> Vec<Scale> {
        self.active_scale_views
            .get(&player_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Set a player's scale view
    pub fn set_player_scale_view(
        &mut self,
        player_id: PeerId,
        scales: Vec<Scale>,
    ) -> Result<(), MultiplayerFeaturesError> {
        // Validate scales are shared
        for scale in &scales {
            if !self.shared_scales.contains_key(scale) {
                return Err(MultiplayerFeaturesError::ScaleSharingFailed);
            }
        }
        self.active_scale_views.insert(player_id, scales);
        Ok(())
    }

    /// Synchronize all scale views
    pub fn sync_scale_views(&mut self) -> Result<SyncResult, MultiplayerFeaturesError> {
        self.scale_synchronization.process_updates()?;
        Ok(SyncResult {
            success: true,
            synced_players: self.active_scale_views.len(),
            message: "Scale views synchronized".to_string(),
        })
    }

    /// Get scale sharing statistics
    pub fn get_scale_statistics(&self) -> ScaleStatistics {
        let total_shared = self.shared_scales.len();
        let total_players: usize = self
            .shared_scales
            .values()
            .map(|s| s.sharing_players.len())
            .sum();

        let total_intensity: Float = self
            .shared_scales
            .values()
            .map(|s| s.holographic_field.intensity)
            .sum();

        let average_intensity = if total_shared > 0 {
            total_intensity / total_shared as Float
        } else {
            0.0
        };

        let total_entities: usize = self
            .shared_scales
            .values()
            .map(|s| s.scale_state.entities.len())
            .sum();

        ScaleStatistics {
            total_shared,
            total_players,
            average_intensity,
            total_entities,
        }
    }
}

impl Default for ScaleSharing {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Network Optimization System
// ============================================================================

/// Network message
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkMessage {
    /// Message ID
    pub message_id: u64,
    /// Source peer
    pub source: PeerId,
    /// Destination peer
    pub destination: PeerId,
    /// Message type
    pub message_type: String,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message priority
    pub priority: crate::simulation_v3::distributed_system::MessagePriority,
    /// Timestamp
    pub timestamp: Timestamp,
}

impl NetworkMessage {
    pub fn size(&self) -> usize {
        self.payload.len()
    }
}

impl Default for NetworkMessage {
    fn default() -> Self {
        Self {
            message_id: 0,
            source: PeerId::default(),
            destination: PeerId::default(),
            message_type: String::new(),
            payload: Vec::new(),
            priority: crate::simulation_v3::distributed_system::MessagePriority::Normal,
            timestamp: 0.0,
        }
    }
}

/// Priority queue for network messages
#[derive(Debug, Clone, PartialEq)]
pub struct PriorityQueue<T> {
    items: Vec<(crate::simulation_v3::distributed_system::MessagePriority, T)>,
}

impl<T: Clone + PartialEq> PriorityQueue<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(
        &mut self,
        priority: crate::simulation_v3::distributed_system::MessagePriority,
        item: T,
    ) {
        self.items.push((priority, item));
        self.items.sort_by(|a, b| b.0.cmp(&a.0));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop().map(|(_, item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T: Clone + PartialEq> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimized network message
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizedMessage {
    /// Original message
    pub original_message: NetworkMessage,
    /// Optimized payload
    pub optimized_payload: Vec<u8>,
    /// Compression ratio
    pub compression_ratio: Float,
    /// Estimated bandwidth saved
    pub bandwidth_saved: usize,
}

impl Default for OptimizedMessage {
    fn default() -> Self {
        Self {
            original_message: NetworkMessage::default(),
            optimized_payload: Vec::new(),
            compression_ratio: 1.0,
            bandwidth_saved: 0,
        }
    }
}

/// Compressed message
#[derive(Debug, Clone, PartialEq)]
pub struct CompressedMessage {
    /// Original size
    pub original_size: usize,
    /// Compressed data
    pub compressed_data: Vec<u8>,
    /// Compression algorithm used
    pub algorithm: String,
}

impl CompressedMessage {
    pub fn size(&self) -> usize {
        self.compressed_data.len()
    }
}

impl Default for CompressedMessage {
    fn default() -> Self {
        Self {
            original_size: 0,
            compressed_data: Vec::new(),
            algorithm: "none".to_string(),
        }
    }
}

/// Batched message
#[derive(Debug, Clone, PartialEq)]
pub struct BatchMessage {
    /// Messages in the batch
    pub messages: Vec<NetworkMessage>,
    /// Batch ID
    pub batch_id: u64,
    /// Total size
    pub total_size: usize,
}

impl Default for BatchMessage {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            batch_id: 0,
            total_size: 0,
        }
    }
}

/// Bandwidth allocation
#[derive(Debug, Clone, PartialEq)]
pub struct BandwidthAllocation {
    /// Allocated bandwidth
    pub allocated: Float,
    /// Message ID
    pub message_id: u64,
    /// Timestamp
    pub timestamp: Timestamp,
}

impl Default for BandwidthAllocation {
    fn default() -> Self {
        Self {
            allocated: 0.0,
            message_id: 0,
            timestamp: 0.0,
        }
    }
}

/// Bandwidth usage information
#[derive(Debug, Clone, PartialEq)]
pub struct BandwidthUsage {
    /// Total used bandwidth
    pub used_bandwidth: Float,
    /// Available bandwidth
    pub available_bandwidth: Float,
    /// Usage percentage
    pub usage_percentage: Float,
}

impl Default for BandwidthUsage {
    fn default() -> Self {
        Self {
            used_bandwidth: 0.0,
            available_bandwidth: 100.0,
            usage_percentage: 0.0,
        }
    }
}

/// Manages bandwidth usage
#[derive(Debug, Clone, PartialEq)]
pub struct BandwidthManager {
    /// Total available bandwidth
    pub total_bandwidth: Float,
    /// Currently used bandwidth
    pub used_bandwidth: Float,
    /// Bandwidth limit
    pub bandwidth_limit: Float,
}

impl BandwidthManager {
    pub fn new(total_bandwidth: Float) -> Self {
        Self {
            total_bandwidth,
            used_bandwidth: 0.0,
            bandwidth_limit: total_bandwidth * 0.9, // Use 90% as limit
        }
    }

    /// Allocate bandwidth for a message
    pub fn allocate_bandwidth(
        &mut self,
        message: &NetworkMessage,
    ) -> Result<BandwidthAllocation, MultiplayerFeaturesError> {
        let required = message.size() as Float;
        let available = self.bandwidth_limit - self.used_bandwidth;

        if required > available {
            return Err(MultiplayerFeaturesError::InsufficientResources);
        }

        self.used_bandwidth += required;

        Ok(BandwidthAllocation {
            allocated: required,
            message_id: message.message_id,
            timestamp: Self::get_current_timestamp(),
        })
    }

    /// Release allocated bandwidth
    pub fn release_bandwidth(&mut self, allocation: BandwidthAllocation) {
        self.used_bandwidth -= allocation.allocated;
        self.used_bandwidth = self.used_bandwidth.max(0.0);
    }

    /// Get current bandwidth usage
    pub fn get_bandwidth_usage(&self) -> BandwidthUsage {
        let available = self.bandwidth_limit - self.used_bandwidth;
        let usage_percentage = if self.bandwidth_limit > 0.0 {
            (self.used_bandwidth / self.bandwidth_limit) * 100.0
        } else {
            0.0
        };

        BandwidthUsage {
            used_bandwidth: self.used_bandwidth,
            available_bandwidth: available,
            usage_percentage,
        }
    }

    fn get_current_timestamp() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

impl Default for BandwidthManager {
    fn default() -> Self {
        Self::new(1_000_000.0) // 1 Mbps default
    }
}

/// Latency measurement
#[derive(Debug, Clone, PartialEq)]
pub struct LatencyMeasurement {
    /// Peer ID
    pub peer_id: PeerId,
    /// Measured latency
    pub latency: Float,
    /// Timestamp
    pub timestamp: Timestamp,
}

impl Default for LatencyMeasurement {
    fn default() -> Self {
        Self {
            peer_id: PeerId::default(),
            latency: 0.0,
            timestamp: 0.0,
        }
    }
}

/// Latency prediction model
#[derive(Debug, Clone, PartialEq)]
pub struct LatencyPredictionModel {
    /// Base latency
    pub base_latency: Float,
    /// Latency variance
    pub variance: Float,
    /// Trend (positive = increasing, negative = decreasing)
    pub trend: Float,
}

impl LatencyPredictionModel {
    pub fn predict(&self, _peer_id: PeerId) -> Float {
        self.base_latency + self.trend
    }
}

impl Default for LatencyPredictionModel {
    fn default() -> Self {
        Self {
            base_latency: 50.0, // 50ms default
            variance: 10.0,
            trend: 0.0,
        }
    }
}

/// Routing result
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingResult {
    /// Target peer
    pub target_peer: PeerId,
    /// Optimal route
    pub route: Vec<PeerId>,
    /// Expected latency
    pub expected_latency: Float,
}

impl Default for RoutingResult {
    fn default() -> Self {
        Self {
            target_peer: PeerId::default(),
            route: Vec::new(),
            expected_latency: 0.0,
        }
    }
}

/// Optimizes network latency
#[derive(Debug, Clone, PartialEq)]
pub struct LatencyOptimizer {
    /// History of latency measurements
    pub latency_history: VecDeque<LatencyMeasurement>,
    /// Prediction model
    pub prediction_model: LatencyPredictionModel,
}

impl LatencyOptimizer {
    pub fn new() -> Self {
        Self {
            latency_history: VecDeque::with_capacity(100),
            prediction_model: LatencyPredictionModel::default(),
        }
    }

    /// Measure latency to a peer
    pub fn measure_latency(&mut self, peer_id: PeerId) -> Float {
        let latency = 50.0 + (peer_id.as_u64() % 100) as Float; // Simulated measurement

        let measurement = LatencyMeasurement {
            peer_id,
            latency,
            timestamp: Self::get_current_timestamp(),
        };

        self.latency_history.push_back(measurement);

        // Keep only last 100 measurements
        while self.latency_history.len() > 100 {
            self.latency_history.pop_front();
        }

        latency
    }

    /// Predict latency to a peer
    pub fn predict_latency(&self, peer_id: PeerId) -> Float {
        self.prediction_model.predict(peer_id)
    }

    /// Optimize routing to a target peer
    pub fn optimize_routing(&mut self, target_peer: PeerId) -> RoutingResult {
        let expected_latency = self.predict_latency(target_peer);

        RoutingResult {
            target_peer,
            route: vec![target_peer],
            expected_latency,
        }
    }

    fn get_current_timestamp() -> Timestamp {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
    }
}

impl Default for LatencyOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Network statistics
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkStatistics {
    /// Total messages sent
    pub total_messages: usize,
    /// Total bytes sent
    pub total_bytes: usize,
    /// Average latency
    pub average_latency: Float,
    /// Bandwidth usage
    pub bandwidth_usage: BandwidthUsage,
    /// Compression ratio
    pub compression_ratio: Float,
}

impl NetworkStatistics {
    pub fn new() -> Self {
        Self {
            total_messages: 0,
            total_bytes: 0,
            average_latency: 0.0,
            bandwidth_usage: BandwidthUsage::default(),
            compression_ratio: 1.0,
        }
    }
}

impl Default for NetworkStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Network conditions
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkConditions {
    /// Available bandwidth
    pub available_bandwidth: Float,
    /// Average latency
    pub average_latency: Float,
    /// Packet loss rate
    pub packet_loss_rate: Float,
}

impl Default for NetworkConditions {
    fn default() -> Self {
        Self {
            available_bandwidth: 1_000_000.0,
            average_latency: 50.0,
            packet_loss_rate: 0.01,
        }
    }
}

/// Optimizes network traffic
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkOptimizer {
    /// Is compression enabled
    pub compression_enabled: bool,
    /// Priority queue for messages
    pub priority_queue: PriorityQueue<NetworkMessage>,
    /// Bandwidth manager
    pub bandwidth_manager: BandwidthManager,
    /// Latency optimizer
    pub latency_optimizer: LatencyOptimizer,
}

impl NetworkOptimizer {
    pub fn new(total_bandwidth: Float) -> Self {
        Self {
            compression_enabled: true,
            priority_queue: PriorityQueue::new(),
            bandwidth_manager: BandwidthManager::new(total_bandwidth),
            latency_optimizer: LatencyOptimizer::new(),
        }
    }

    /// Optimize a message
    pub fn optimize_message(
        &self,
        message: NetworkMessage,
    ) -> Result<OptimizedMessage, MultiplayerFeaturesError> {
        let optimized_payload = if self.compression_enabled {
            self.compress_payload(&message.payload)?
        } else {
            message.payload.clone()
        };

        let compression_ratio = if message.size() > 0 {
            optimized_payload.len() as Float / message.size() as Float
        } else {
            1.0
        };

        let bandwidth_saved = if message.size() > optimized_payload.len() {
            message.size() - optimized_payload.len()
        } else {
            0
        };

        Ok(OptimizedMessage {
            original_message: message,
            optimized_payload,
            compression_ratio,
            bandwidth_saved,
        })
    }

    /// Batch multiple messages
    pub fn batch_messages(
        &mut self,
        messages: Vec<NetworkMessage>,
    ) -> Result<BatchMessage, MultiplayerFeaturesError> {
        let total_size: usize = messages.iter().map(|m| m.size()).sum();
        let batch_id = messages.len() as u64;

        Ok(BatchMessage {
            messages,
            batch_id,
            total_size,
        })
    }

    /// Compress a message
    pub fn compress_message(
        &self,
        message: NetworkMessage,
    ) -> Result<CompressedMessage, MultiplayerFeaturesError> {
        let compressed_data = if self.compression_enabled {
            self.compress_payload(&message.payload)?
        } else {
            message.payload.clone()
        };

        Ok(CompressedMessage {
            original_size: message.size(),
            compressed_data,
            algorithm: if self.compression_enabled {
                "zlib".to_string()
            } else {
                "none".to_string()
            },
        })
    }

    /// Decompress a message
    pub fn decompress_message(
        &self,
        message: CompressedMessage,
    ) -> Result<NetworkMessage, MultiplayerFeaturesError> {
        let payload = if message.algorithm != "none" {
            self.decompress_payload(&message.compressed_data, message.original_size)?
        } else {
            message.compressed_data.clone()
        };

        Ok(NetworkMessage {
            payload,
            ..Default::default()
        })
    }

    /// Get network statistics
    pub fn get_network_statistics(&self) -> NetworkStatistics {
        let bandwidth_usage = self.bandwidth_manager.get_bandwidth_usage();

        let avg_latency = if !self.latency_optimizer.latency_history.is_empty() {
            let sum: Float = self
                .latency_optimizer
                .latency_history
                .iter()
                .map(|m| m.latency)
                .sum();
            sum / self.latency_optimizer.latency_history.len() as Float
        } else {
            0.0
        };

        NetworkStatistics {
            total_messages: self.priority_queue.len(),
            total_bytes: self.bandwidth_manager.used_bandwidth as usize,
            average_latency: avg_latency,
            bandwidth_usage,
            compression_ratio: 0.8, // Average compression ratio
        }
    }

    /// Tune parameters based on network conditions
    pub fn tune_parameters(&mut self, conditions: NetworkConditions) {
        // Adjust compression based on bandwidth
        self.compression_enabled = conditions.available_bandwidth < 500_000.0;

        // Adjust bandwidth manager
        self.bandwidth_manager.bandwidth_limit = conditions.available_bandwidth * 0.9;

        // Update latency prediction model
        self.latency_optimizer.prediction_model.base_latency = conditions.average_latency;
    }

    /// Simple payload compression (simulation)
    fn compress_payload(&self, payload: &[u8]) -> Result<Vec<u8>, MultiplayerFeaturesError> {
        // In a real implementation, this would use actual compression
        // For simulation, we just return a copy
        Ok(payload.to_vec())
    }

    /// Simple payload decompression (simulation)
    fn decompress_payload(
        &self,
        compressed: &[u8],
        _original_size: usize,
    ) -> Result<Vec<u8>, MultiplayerFeaturesError> {
        // In a real implementation, this would use actual decompression
        Ok(compressed.to_vec())
    }
}

impl Default for NetworkOptimizer {
    fn default() -> Self {
        Self::new(1_000_000.0)
    }
}

// ============================================================================
// Main Multiplayer Features System
// ============================================================================

/// Player experience in multiplayer features
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerExperience {
    /// Player ID
    pub player_id: PeerId,
    /// Active densities
    pub active_densities: Vec<Density>,
    /// Active scales
    pub active_scales: Vec<Scale>,
    /// Participating manifestations
    pub participating_manifestations: Vec<ManifestationId>,
    /// Network quality (0.0 to 1.0)
    pub network_quality: Float,
    /// Current latency
    pub latency: Float,
    /// Available bandwidth
    pub bandwidth: Float,
}

impl PlayerExperience {
    pub fn new(player_id: PeerId) -> Self {
        Self {
            player_id,
            active_densities: Vec::new(),
            active_scales: Vec::new(),
            participating_manifestations: Vec::new(),
            network_quality: 1.0,
            latency: 0.0,
            bandwidth: 0.0,
        }
    }
}

impl Default for PlayerExperience {
    fn default() -> Self {
        Self::new(PeerId::default())
    }
}

/// System status
#[derive(Debug, Clone, PartialEq)]
pub struct SystemStatus {
    /// Is the system running
    pub is_running: bool,
    /// Number of active manifestations
    pub active_manifestations: usize,
    /// Number of shared densities
    pub shared_densities: usize,
    /// Number of shared scales
    pub shared_scales: usize,
    /// Network quality
    pub network_quality: Float,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            active_manifestations: 0,
            shared_densities: 0,
            shared_scales: 0,
            network_quality: 1.0,
        }
    }
}

/// Main multiplayer features system
#[derive(Debug, Clone, PartialEq)]
pub struct MultiplayerFeaturesSystem {
    /// Collective manifestation manager
    pub collective_manifestation: CollectiveManifestationManager,
    /// Density sharing system
    pub density_sharing: DensitySharing,
    /// Scale sharing system
    pub scale_sharing: ScaleSharing,
    /// Network optimizer
    pub network_optimizer: NetworkOptimizer,
}

impl MultiplayerFeaturesSystem {
    pub fn new() -> Self {
        Self {
            collective_manifestation: CollectiveManifestationManager::new(),
            density_sharing: DensitySharing::new(),
            scale_sharing: ScaleSharing::new(),
            network_optimizer: NetworkOptimizer::new(1_000_000.0),
        }
    }

    /// Initialize the system
    pub fn initialize(&mut self) -> Result<(), MultiplayerFeaturesError> {
        // Initialize all subsystems
        Ok(())
    }

    /// Process collective manifestations
    pub fn process_collective_manifestation(&mut self) -> Result<(), MultiplayerFeaturesError> {
        self.collective_manifestation.update_all_manifestations()?;
        Ok(())
    }

    /// Process density sharing
    pub fn process_density_sharing(&mut self) -> Result<(), MultiplayerFeaturesError> {
        self.density_sharing.sync_density_views()?;
        Ok(())
    }

    /// Process scale sharing
    pub fn process_scale_sharing(&mut self) -> Result<(), MultiplayerFeaturesError> {
        self.scale_sharing.sync_scale_views()?;
        Ok(())
    }

    /// Optimize network
    pub fn optimize_network(&mut self) -> Result<(), MultiplayerFeaturesError> {
        // Process priority queue
        while let Some(_message) = self.network_optimizer.priority_queue.pop() {
            // Process message
        }
        Ok(())
    }

    /// Get system status
    pub fn get_system_status(&self) -> SystemStatus {
        let network_stats = self.network_optimizer.get_network_statistics();
        let network_quality = 1.0 - (network_stats.average_latency / 1000.0).min(1.0);

        SystemStatus {
            is_running: true,
            active_manifestations: self.collective_manifestation.active_manifestations.len(),
            shared_densities: self.density_sharing.shared_densities.len(),
            shared_scales: self.scale_sharing.shared_scales.len(),
            network_quality,
        }
    }

    /// Get player experience
    pub fn get_player_experience(&self, player_id: PeerId) -> PlayerExperience {
        let active_densities = self.density_sharing.get_player_density_view(player_id);
        let active_scales = self.scale_sharing.get_player_scale_view(player_id);
        let participating_manifestations: Vec<ManifestationId> = self
            .collective_manifestation
            .active_manifestations
            .iter()
            .filter(|(_, m)| m.participating_players.contains(&player_id))
            .map(|(id, _)| *id)
            .collect();

        let latency = self
            .network_optimizer
            .latency_optimizer
            .predict_latency(player_id);
        let bandwidth_usage = self
            .network_optimizer
            .bandwidth_manager
            .get_bandwidth_usage();
        let network_quality = 1.0 - (latency / 1000.0).min(1.0);

        PlayerExperience {
            player_id,
            active_densities,
            active_scales,
            participating_manifestations,
            network_quality,
            latency,
            bandwidth: bandwidth_usage.available_bandwidth,
        }
    }
}

impl Default for MultiplayerFeaturesSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Multiplayer features error types
#[derive(Debug, Clone, PartialEq)]
pub enum MultiplayerFeaturesError {
    /// Manifestation failed
    ManifestationFailed,
    /// Density sharing failed
    DensitySharingFailed,
    /// Scale sharing failed
    ScaleSharingFailed,
    /// Network optimization failed
    NetworkOptimizationFailed,
    /// Player not found
    PlayerNotFound,
    /// Insufficient resources
    InsufficientResources,
    /// Synchronization failed
    SynchronizationFailed,
}

impl fmt::Display for MultiplayerFeaturesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiplayerFeaturesError::ManifestationFailed => write!(f, "Manifestation failed"),
            MultiplayerFeaturesError::DensitySharingFailed => write!(f, "Density sharing failed"),
            MultiplayerFeaturesError::ScaleSharingFailed => write!(f, "Scale sharing failed"),
            MultiplayerFeaturesError::NetworkOptimizationFailed => {
                write!(f, "Network optimization failed")
            }
            MultiplayerFeaturesError::PlayerNotFound => write!(f, "Player not found"),
            MultiplayerFeaturesError::InsufficientResources => write!(f, "Insufficient resources"),
            MultiplayerFeaturesError::SynchronizationFailed => write!(f, "Synchronization failed"),
        }
    }
}

impl std::error::Error for MultiplayerFeaturesError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::distributed_system::MessagePriority;

    // -------------------------------------------------------------------------
    // Collective Manifestation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_manifestation_id_creation() {
        let id = ManifestationId::new(123);
        assert_eq!(id.as_u64(), 123);
    }

    #[test]
    fn test_manifestation_state_display() {
        assert_eq!(ManifestationState::Gathering.to_string(), "Gathering");
        assert_eq!(ManifestationState::Complete.to_string(), "Complete");
    }

    #[test]
    fn test_contribution_type_default() {
        let ctype = ContributionType::default();
        assert_eq!(ctype, ContributionType::DirectResonance);
    }

    #[test]
    fn test_resonance_contribution_creation() {
        let player_id = PeerId::new(1);
        let contribution = ResonanceContribution::new(
            player_id,
            0.5,
            ContributionType::Catalyst,
            ResonancePattern::default(),
            100.0,
        );

        assert_eq!(contribution.player_id, player_id);
        assert_eq!(contribution.contribution_amount, 0.5);
        assert_eq!(contribution.contribution_type, ContributionType::Catalyst);
    }

    #[test]
    fn test_collective_manifestation_creation() {
        let players = vec![PeerId::new(1), PeerId::new(2)];
        let manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            players.clone(),
            StructureType::Temple,
            ResonancePattern::new(0.8, 0.9, 0.0, 0.95),
        );

        assert_eq!(manifestation.participating_players, players);
        assert_eq!(
            manifestation.manifestation_state,
            ManifestationState::Gathering
        );
        assert_eq!(manifestation.current_progress, 0.0);
    }

    #[test]
    fn test_collective_manifestation_add_participant() {
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![PeerId::new(1)],
            StructureType::Temple,
            ResonancePattern::default(),
        );

        manifestation.add_participant(PeerId::new(2)).unwrap();
        assert!(manifestation
            .participating_players
            .contains(&PeerId::new(2)));
    }

    #[test]
    fn test_collective_manifestation_remove_participant() {
        let player_id = PeerId::new(1);
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![player_id],
            StructureType::Temple,
            ResonancePattern::default(),
        );

        manifestation.remove_participant(player_id).unwrap();
        assert!(!manifestation.participating_players.contains(&player_id));
    }

    #[test]
    fn test_collective_manifestation_contribute_resonance() {
        let player_id = PeerId::new(1);
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![player_id],
            StructureType::Temple,
            ResonancePattern::new(0.8, 0.9, 0.0, 0.95),
        );

        let contribution = ResonanceContribution::new(
            player_id,
            0.5,
            ContributionType::DirectResonance,
            ResonancePattern::new(0.6, 0.7, 0.0, 0.8),
            100.0,
        );

        manifestation
            .contribute_resonance(player_id, contribution)
            .unwrap();
        assert!(manifestation.current_progress > 0.0);
        assert_eq!(
            manifestation.manifestation_state,
            ManifestationState::ResonanceBuilding
        );
    }

    #[test]
    fn test_collective_manifestation_update_progress() {
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![PeerId::new(1)],
            StructureType::CommunityCenter,
            ResonancePattern::new(0.3, 0.4, 0.0, 0.5),
        );

        manifestation.collective_resonance = ResonancePattern::new(0.15, 0.2, 0.0, 0.25);
        manifestation.update_progress().unwrap();
        assert!(manifestation.current_progress > 0.0);
    }

    #[test]
    fn test_collective_manifestation_is_complete() {
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![PeerId::new(1)],
            StructureType::Temple,
            ResonancePattern::new(0.5, 0.5, 0.0, 0.5),
        );

        manifestation.current_progress = 1.0;
        manifestation.manifestation_state = ManifestationState::Complete;
        assert!(manifestation.is_complete());
    }

    #[test]
    fn test_collective_manifestation_manager_create() {
        let mut manager = CollectiveManifestationManager::new();
        let players = vec![PeerId::new(1), PeerId::new(2)];

        let id = manager
            .create_manifestation(players, StructureType::Temple)
            .unwrap();

        assert!(manager.active_manifestations.contains_key(&id));
    }

    #[test]
    fn test_collective_manifestation_manager_get() {
        let mut manager = CollectiveManifestationManager::new();
        let players = vec![PeerId::new(1)];

        let id = manager
            .create_manifestation(players, StructureType::Temple)
            .unwrap();

        let manifestation = manager.get_manifestation(id);
        assert!(manifestation.is_some());
    }

    #[test]
    fn test_collective_manifestation_manager_update_all() {
        let mut manager = CollectiveManifestationManager::new();
        let players = vec![PeerId::new(1)];

        manager
            .create_manifestation(players, StructureType::Temple)
            .unwrap();

        manager.update_all_manifestations().unwrap();
    }

    // -------------------------------------------------------------------------
    // Density Sharing Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_density_state_creation() {
        let state = DensityState::new();
        assert_eq!(state.entities.len(), 0);
        assert_eq!(state.collective_catalyst, 0.0);
    }

    #[test]
    fn test_density_share_creation() {
        let share = DensityShare::new(Density::Third);
        assert_eq!(share.density, Density::Third);
        assert_eq!(share.sharing_players.len(), 0);
    }

    #[test]
    fn test_density_sharing_enable() {
        let mut sharing = DensitySharing::new();
        let densities = vec![Density::First, Density::Second];

        sharing.enable_density_sharing(densities).unwrap();
        assert_eq!(sharing.shared_densities.len(), 2);
    }

    #[test]
    fn test_density_sharing_disable() {
        let mut sharing = DensitySharing::new();
        sharing
            .enable_density_sharing(vec![Density::First])
            .unwrap();

        sharing
            .disable_density_sharing(vec![Density::First])
            .unwrap();
        assert_eq!(sharing.shared_densities.len(), 0);
    }

    #[test]
    fn test_density_sharing_get_player_view() {
        let mut sharing = DensitySharing::new();
        let player_id = PeerId::new(1);

        let view = sharing.get_player_density_view(player_id);
        assert_eq!(view.len(), 0);
    }

    #[test]
    fn test_density_sharing_set_player_view() {
        let mut sharing = DensitySharing::new();
        let player_id = PeerId::new(1);
        sharing
            .enable_density_sharing(vec![Density::First])
            .unwrap();

        sharing
            .set_player_density_view(player_id, vec![Density::First])
            .unwrap();

        let view = sharing.get_player_density_view(player_id);
        assert_eq!(view.len(), 1);
    }

    #[test]
    fn test_density_sharing_sync() {
        let mut sharing = DensitySharing::new();
        let result = sharing.sync_density_views().unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_density_statistics() {
        let mut sharing = DensitySharing::new();
        sharing
            .enable_density_sharing(vec![Density::First, Density::Second])
            .unwrap();

        let stats = sharing.get_density_statistics();
        assert_eq!(stats.total_shared, 2);
    }

    #[test]
    fn test_density_synchronization_enqueue() {
        let mut sync = DensitySynchronization::new();
        let update = DensityUpdate::new(
            Density::First,
            DensityUpdateType::EntityAdded,
            DensityUpdateData::EntityAdded { entity_id: 1 },
        );

        sync.enqueue_update(update);
        assert_eq!(sync.sync_queue.len(), 1);
    }

    #[test]
    fn test_density_synchronization_process() {
        let mut sync = DensitySynchronization::new();
        sync.enqueue_update(DensityUpdate::new(
            Density::First,
            DensityUpdateType::FullSync,
            DensityUpdateData::FullSync {
                state: DensityState::new(),
            },
        ));

        sync.process_updates().unwrap();
        assert_eq!(sync.sync_queue.len(), 0);
    }

    #[test]
    fn test_density_synchronization_status() {
        let sync = DensitySynchronization::new();
        assert_eq!(sync.get_sync_status(), SyncStatus::Synchronized);
    }

    // -------------------------------------------------------------------------
    // Scale Sharing Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_scale_level_magnitude() {
        assert_eq!(ScaleLevel::Quantum.magnitude(), -35);
        assert_eq!(ScaleLevel::Cosmic.magnitude(), 26);
    }

    #[test]
    fn test_scale_level_display() {
        assert_eq!(ScaleLevel::Quantum.to_string(), "Quantum");
        assert_eq!(ScaleLevel::Galactic.to_string(), "Galactic");
    }

    #[test]
    fn test_spatial_dimensions_creation() {
        let dims = SpatialDimensions::new(10.0, 20.0, 30.0);
        assert_eq!(dims.width, 10.0);
        assert_eq!(dims.volume(), 6000.0);
    }

    #[test]
    fn test_temporal_dimensions_creation() {
        let dims = TemporalDimensions::new(0.8, 0.9);
        assert_eq!(dims.time_dilation, 0.8);
        assert_eq!(dims.temporal_coherence, 0.9);
    }

    #[test]
    fn test_holographic_field_creation() {
        let field = HolographicField::new(0.7, 0.8, 0.9);
        assert_eq!(field.intensity, 0.7);
        assert_eq!(field.coherence, 0.8);
    }

    #[test]
    fn test_scale_state_creation() {
        let state = ScaleState::new(ScaleLevel::Cellular);
        assert_eq!(state.scale_level, ScaleLevel::Cellular);
    }

    #[test]
    fn test_scale_share_creation() {
        let share = ScaleShare::new(ScaleLevel::Planetary);
        assert_eq!(share.scale, ScaleLevel::Planetary);
    }

    #[test]
    fn test_scale_sharing_enable() {
        let mut sharing = ScaleSharing::new();
        let scales = vec![ScaleLevel::Cellular, ScaleLevel::Planetary];

        sharing.enable_scale_sharing(scales).unwrap();
        assert_eq!(sharing.shared_scales.len(), 2);
    }

    #[test]
    fn test_scale_sharing_disable() {
        let mut sharing = ScaleSharing::new();
        sharing
            .enable_scale_sharing(vec![ScaleLevel::Cellular])
            .unwrap();

        sharing
            .disable_scale_sharing(vec![ScaleLevel::Cellular])
            .unwrap();
        assert_eq!(sharing.shared_scales.len(), 0);
    }

    #[test]
    fn test_scale_sharing_get_player_view() {
        let mut sharing = ScaleSharing::new();
        let player_id = PeerId::new(1);

        let view = sharing.get_player_scale_view(player_id);
        assert_eq!(view.len(), 0);
    }

    #[test]
    fn test_scale_sharing_set_player_view() {
        let mut sharing = ScaleSharing::new();
        let player_id = PeerId::new(1);
        sharing
            .enable_scale_sharing(vec![ScaleLevel::Cellular])
            .unwrap();

        sharing
            .set_player_scale_view(player_id, vec![ScaleLevel::Cellular])
            .unwrap();

        let view = sharing.get_player_scale_view(player_id);
        assert_eq!(view.len(), 1);
    }

    #[test]
    fn test_scale_sharing_sync() {
        let mut sharing = ScaleSharing::new();
        let result = sharing.sync_scale_views().unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_scale_statistics() {
        let mut sharing = ScaleSharing::new();
        sharing
            .enable_scale_sharing(vec![ScaleLevel::Cellular, ScaleLevel::Planetary])
            .unwrap();

        let stats = sharing.get_scale_statistics();
        assert_eq!(stats.total_shared, 2);
    }

    #[test]
    fn test_scale_synchronization_status() {
        let sync = ScaleSynchronization::new();
        assert_eq!(sync.get_sync_status(), SyncStatus::Synchronized);
    }

    // -------------------------------------------------------------------------
    // Network Optimization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_network_message_creation() {
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1, 2, 3],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        assert_eq!(message.size(), 3);
    }

    #[test]
    fn test_priority_queue_push_pop() {
        let mut queue: PriorityQueue<NetworkMessage> = PriorityQueue::new();

        let msg1 = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "low".to_string(),
            payload: vec![1],
            priority: MessagePriority::Low,
            timestamp: 100.0,
        };

        let msg2 = NetworkMessage {
            message_id: 2,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "high".to_string(),
            payload: vec![2],
            priority: MessagePriority::High,
            timestamp: 100.0,
        };

        queue.push(MessagePriority::Low, msg1);
        queue.push(MessagePriority::High, msg2);

        let popped = queue.pop();
        assert!(popped.is_some());
    }

    #[test]
    fn test_bandwidth_manager_creation() {
        let manager = BandwidthManager::new(1_000_000.0);
        assert_eq!(manager.total_bandwidth, 1_000_000.0);
        assert_eq!(manager.bandwidth_limit, 900_000.0);
    }

    #[test]
    fn test_bandwidth_manager_allocate() {
        let mut manager = BandwidthManager::new(1_000_000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 1000],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        let allocation = manager.allocate_bandwidth(&message).unwrap();
        assert_eq!(allocation.allocated, 1000.0);
    }

    #[test]
    fn test_bandwidth_manager_release() {
        let mut manager = BandwidthManager::new(1_000_000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 1000],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        let allocation = manager.allocate_bandwidth(&message).unwrap();
        manager.release_bandwidth(allocation);

        assert_eq!(manager.used_bandwidth, 0.0);
    }

    #[test]
    fn test_bandwidth_manager_usage() {
        let mut manager = BandwidthManager::new(1_000_000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 100_000],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        manager.allocate_bandwidth(&message).unwrap();
        let usage = manager.get_bandwidth_usage();

        assert!(usage.used_bandwidth > 0.0);
    }

    #[test]
    fn test_latency_optimizer_measure() {
        let mut optimizer = LatencyOptimizer::new();
        let peer_id = PeerId::new(1);

        let latency = optimizer.measure_latency(peer_id);
        assert!(latency > 0.0);
    }

    #[test]
    fn test_latency_optimizer_predict() {
        let optimizer = LatencyOptimizer::new();
        let peer_id = PeerId::new(1);

        let latency = optimizer.predict_latency(peer_id);
        assert!(latency > 0.0);
    }

    #[test]
    fn test_latency_optimizer_routing() {
        let mut optimizer = LatencyOptimizer::new();
        let target_peer = PeerId::new(1);

        let result = optimizer.optimize_routing(target_peer);
        assert_eq!(result.target_peer, target_peer);
    }

    #[test]
    fn test_network_optimizer_creation() {
        let optimizer = NetworkOptimizer::new(1_000_000.0);
        assert!(optimizer.compression_enabled);
    }

    #[test]
    fn test_network_optimizer_optimize_message() {
        let optimizer = NetworkOptimizer::new(1_000_000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 100],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        let optimized = optimizer.optimize_message(message).unwrap();
        assert_eq!(optimized.original_message.message_id, 1);
    }

    #[test]
    fn test_network_optimizer_batch_messages() {
        let mut optimizer = NetworkOptimizer::new(1_000_000.0);
        let messages = vec![
            NetworkMessage {
                message_id: 1,
                source: PeerId::new(1),
                destination: PeerId::new(2),
                message_type: "test1".to_string(),
                payload: vec![1],
                priority: MessagePriority::Normal,
                timestamp: 100.0,
            },
            NetworkMessage {
                message_id: 2,
                source: PeerId::new(1),
                destination: PeerId::new(2),
                message_type: "test2".to_string(),
                payload: vec![2],
                priority: MessagePriority::Normal,
                timestamp: 100.0,
            },
        ];

        let batch = optimizer.batch_messages(messages).unwrap();
        assert_eq!(batch.messages.len(), 2);
    }

    #[test]
    fn test_network_optimizer_compress() {
        let optimizer = NetworkOptimizer::new(1_000_000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 100],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        let compressed = optimizer.compress_message(message).unwrap();
        assert_eq!(compressed.original_size, 100);
    }

    #[test]
    fn test_network_optimizer_decompress() {
        let optimizer = NetworkOptimizer::new(1_000_000.0);
        let compressed = CompressedMessage {
            original_size: 100,
            compressed_data: vec![1; 100],
            algorithm: "none".to_string(),
        };

        let decompressed = optimizer.decompress_message(compressed).unwrap();
        assert_eq!(decompressed.payload.len(), 100);
    }

    #[test]
    fn test_network_optimizer_tune_parameters() {
        let mut optimizer = NetworkOptimizer::new(1_000_000.0);
        let conditions = NetworkConditions {
            available_bandwidth: 500_000.0,
            average_latency: 100.0,
            packet_loss_rate: 0.05,
        };

        optimizer.tune_parameters(conditions);
        assert!(optimizer.compression_enabled);
    }

    // -------------------------------------------------------------------------
    // Multiplayer Features System Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_multiplayer_features_system_creation() {
        let system = MultiplayerFeaturesSystem::new();
        assert_eq!(
            system.collective_manifestation.active_manifestations.len(),
            0
        );
    }

    #[test]
    fn test_multiplayer_features_system_initialize() {
        let mut system = MultiplayerFeaturesSystem::new();
        system.initialize().unwrap();
    }

    #[test]
    fn test_multiplayer_features_system_status() {
        let system = MultiplayerFeaturesSystem::new();
        let status = system.get_system_status();

        assert!(status.is_running);
        assert_eq!(status.active_manifestations, 0);
    }

    #[test]
    fn test_multiplayer_features_system_player_experience() {
        let system = MultiplayerFeaturesSystem::new();
        let player_id = PeerId::new(1);

        let experience = system.get_player_experience(player_id);
        assert_eq!(experience.player_id, player_id);
    }

    #[test]
    fn test_player_experience_creation() {
        let player_id = PeerId::new(1);
        let experience = PlayerExperience::new(player_id);

        assert_eq!(experience.player_id, player_id);
        assert_eq!(experience.network_quality, 1.0);
    }

    // -------------------------------------------------------------------------
    // Error Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_multiplayer_features_error_display() {
        let err = MultiplayerFeaturesError::ManifestationFailed;
        assert_eq!(err.to_string(), "Manifestation failed");

        let err = MultiplayerFeaturesError::PlayerNotFound;
        assert_eq!(err.to_string(), "Player not found");

        let err = MultiplayerFeaturesError::InsufficientResources;
        assert_eq!(err.to_string(), "Insufficient resources");
    }

    #[test]
    fn test_collective_manifestation_add_duplicate_player() {
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![PeerId::new(1)],
            StructureType::Temple,
            ResonancePattern::default(),
        );

        let result = manifestation.add_participant(PeerId::new(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_collective_manifestation_contribute_nonexistent_player() {
        let mut manifestation = CollectiveManifestation::new(
            ManifestationId::new(1),
            vec![PeerId::new(1)],
            StructureType::Temple,
            ResonancePattern::default(),
        );

        let contribution = ResonanceContribution::new(
            PeerId::new(999),
            0.5,
            ContributionType::DirectResonance,
            ResonancePattern::default(),
            100.0,
        );

        let result = manifestation.contribute_resonance(PeerId::new(999), contribution);
        assert!(result.is_err());
    }

    #[test]
    fn test_collective_manifestation_manager_create_empty_players() {
        let mut manager = CollectiveManifestationManager::new();
        let result = manager.create_manifestation(vec![], StructureType::Temple);
        assert!(result.is_err());
    }

    #[test]
    fn test_density_sharing_set_invalid_view() {
        let mut sharing = DensitySharing::new();
        let player_id = PeerId::new(1);

        let result = sharing.set_player_density_view(player_id, vec![Density::First]);
        assert!(result.is_err());
    }

    #[test]
    fn test_scale_sharing_set_invalid_view() {
        let mut sharing = ScaleSharing::new();
        let player_id = PeerId::new(1);

        let result = sharing.set_player_scale_view(player_id, vec![ScaleLevel::Cellular]);
        assert!(result.is_err());
    }

    #[test]
    fn test_bandwidth_manager_insufficient() {
        let mut manager = BandwidthManager::new(1000.0);
        let message = NetworkMessage {
            message_id: 1,
            source: PeerId::new(1),
            destination: PeerId::new(2),
            message_type: "test".to_string(),
            payload: vec![1; 1_000_000],
            priority: MessagePriority::Normal,
            timestamp: 100.0,
        };

        let result = manager.allocate_bandwidth(&message);
        assert!(result.is_err());
    }
}
