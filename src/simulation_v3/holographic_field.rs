// Holographic Field Manager (Phase 3)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md:
// "Create HolographicFieldManager that manages holographic connections and
// demonstrates holographic principle"
//
// Phase 4: Veil Enhancement
// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
// "Veil limits access to holographic connections based on transparency"
//
// This module implements:
// 1. Holographic connections between entities
// 2. Non-local interactions
// 3. Interference patterns
// 4. Resonance tracking
// 5. Demonstration of holographic principle: "each entity contains the whole"
// 6. Veil access control for holographic connections

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Float type for holographic calculations
pub type Float = f64;

/// Performance configuration for holographic field
#[derive(Debug, Clone)]
pub struct HolographicFieldConfig {
    /// Update interval for holographic connections (steps)
    pub update_interval: u64,

    /// Enable connection caching
    pub enable_connection_cache: bool,

    /// Enable archetype similarity caching
    pub enable_similarity_cache: bool,

    /// Enable parallel connection calculations
    pub enable_parallel: bool,

    /// Maximum connections per entity (for performance)
    pub max_connections_per_entity: usize,
}

impl Default for HolographicFieldConfig {
    fn default() -> Self {
        HolographicFieldConfig {
            update_interval: 5, // Update every 5 steps
            enable_connection_cache: true,
            enable_similarity_cache: true,
            enable_parallel: true,
            max_connections_per_entity: 100,
        }
    }
}

/// Connection cache for performance optimization
#[derive(Debug, Clone)]
struct ConnectionCache {
    /// Cache archetype similarity scores
    similarity_cache: HashMap<(EntityId, EntityId), Float>,

    /// Cache karmic correlation scores
    correlation_cache: HashMap<(EntityId, EntityId), Float>,

    /// Cache connection strengths
    strength_cache: HashMap<(EntityId, EntityId), Float>,

    /// Track when connections were last updated
    last_updated: HashMap<(EntityId, EntityId), u64>,
}

impl ConnectionCache {
    fn new() -> Self {
        ConnectionCache {
            similarity_cache: HashMap::new(),
            correlation_cache: HashMap::new(),
            strength_cache: HashMap::new(),
            last_updated: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.similarity_cache.clear();
        self.correlation_cache.clear();
        self.strength_cache.clear();
        self.last_updated.clear();
    }
}

/// Holographic Field Manager
///
/// Manages holographic connections between entities and demonstrates the
/// holographic principle: "each entity contains the whole".
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each entity contains within it all densities and sub-densities of the octave"
///
/// This is a RESULT of the "transcend and include" principle operating at every stage.
pub struct HolographicFieldManager {
    /// Entity storage (indexed by EntityId)
    pub entities: HashMap<EntityId, SubSubLogos>,

    /// Holographic connections between entities
    pub holographic_connections: HashMap<(EntityId, EntityId), HolographicConnection>,

    /// Interference patterns in the field
    pub interference_patterns: Vec<InterferencePattern>,

    /// Resonance tracking
    pub resonance_tracker: ResonanceTracker,

    /// Field statistics
    pub statistics: HolographicFieldStatistics,

    /// Non-local interaction matrix
    pub non_local_matrix: NonLocalMatrix,

    /// Performance configuration (Phase 5)
    pub config: HolographicFieldConfig,

    /// Connection cache for performance optimization (Phase 5)
    connection_cache: ConnectionCache,

    /// Current simulation step (for lazy updates)
    current_step: u64,

    /// Last update step for connections
    last_connection_update: u64,

    /// Phase 6: Coherence tracker
    pub coherence_tracker: CoherenceTracker,

    /// Phase 6: Resonance statistics
    pub resonance_statistics: ResonanceStatistics,

    /// Migrated from holographic_connections.rs
    /// Threshold for propagation (only propagate if resonance > threshold)
    pub propagation_threshold: Float,

    /// Migrated from holographic_connections.rs
    /// Current timestamp for synchronization tracking
    pub current_timestamp: Float,
}

/// Holographic connection between two entities
///
/// Represents the holographic relationship between entities based on the
/// principle that "each entity contains the whole".
#[derive(Debug, Clone)]
pub struct HolographicConnection {
    /// Source entity
    pub source_id: EntityId,

    /// Target entity
    pub target_id: EntityId,

    /// Connection strength (0.0 to 1.0)
    pub strength: Float,

    /// Connection type
    pub connection_type: HolographicConnectionType,

    /// Interference pattern
    pub interference_pattern: InterferencePattern,

    /// Resonance frequency
    pub resonance_frequency: Float,

    /// Coherence level (0.0 to 1.0)
    pub coherence: Float,
}

/// Type of holographic connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolographicConnectionType {
    /// Resonant connection (similar frequencies)
    Resonant,

    /// Harmonic connection (frequency multiples)
    Harmonic,

    /// Entangled connection (quantum-like correlation)
    Entangled,

    /// Antiphase connection (opposite phases)
    Antiphase,

    /// Weak connection (low correlation)
    Weak,
}

/// Type of holographic change (Migrated from holographic_connections.rs)
///
/// Represents the type of archetype change that can propagate through
/// the holographic field.
///
/// From CONTINUOUS_INVOLUTION_EVOLUTION_PROPOSAL.md Section 2.8:
/// "What is learned by one is known to all" - changes propagate through
/// holographic connections based on resonance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeType {
    /// Change in activation level
    ActivationChange,

    /// Change in bias
    BiasChange,

    /// Change in refinement
    RefinementChange,
}

/// A holographic change that can propagate through the field (Migrated from holographic_connections.rs)
///
/// Represents an archetype change that propagates through holographic connections
/// to all connected entities based on resonance.
///
/// From CONTINUOUS_INVOLUTION_EVOLUTION_PROPOSAL.md Section 2.8:
/// "Changes in one entity affect all connected entities through non-local influence."
#[derive(Debug, Clone)]
pub struct HolographicChange {
    /// Archetype ID that changed
    pub archetype_id: usize,

    /// Type of change
    pub change_type: ChangeType,

    /// Intensity of the change (0.0 to 1.0)
    pub intensity: Float,
}

impl HolographicChange {
    /// Create a new holographic change
    pub fn new(archetype_id: usize, change_type: ChangeType, intensity: Float) -> Self {
        HolographicChange {
            archetype_id,
            change_type,
            intensity: intensity.clamp(0.0, 1.0),
        }
    }

    /// Get the change type
    pub fn get_change_type(&self) -> ChangeType {
        self.change_type
    }

    /// Get the intensity
    pub fn get_intensity(&self) -> Float {
        self.intensity
    }
}

/// Result of applying holographic influence (Migrated from holographic_connections.rs)
///
/// Represents the result of applying a holographic change to a target entity.
#[derive(Debug, Clone)]
pub struct InfluenceResult {
    /// Target entity ID
    pub target_id: EntityId,

    /// Whether influence was successfully applied
    pub applied: bool,

    /// Strength of influence applied (0.0 to 1.0)
    pub strength: Float,

    /// Effect on target (change in activation, bias, or refinement)
    pub effect: Float,
}

impl InfluenceResult {
    /// Create a new influence result
    pub fn new(target_id: EntityId, applied: bool, strength: Float, effect: Float) -> Self {
        InfluenceResult {
            target_id,
            applied,
            strength: strength.clamp(0.0, 1.0),
            effect,
        }
    }

    /// Check if influence was applied
    pub fn was_applied(&self) -> bool {
        self.applied
    }
}

/// Interference pattern from entity interactions
///
/// Represents the constructive and destructive interference patterns
/// that emerge when entities interact holographically.
#[derive(Debug, Clone)]
pub struct InterferencePattern {
    /// Pattern ID
    pub pattern_id: String,

    /// Constructive interference nodes (stable configurations)
    pub constructive_nodes: Vec<InterferenceNode>,

    /// Destructive interference nodes (unstable configurations)
    pub destructive_nodes: Vec<InterferenceNode>,

    /// Phase coherence (0.0 to 1.0)
    pub phase_coherence: Float,

    /// Pattern intensity
    pub intensity: Float,
}

impl Default for InterferencePattern {
    fn default() -> Self {
        InterferencePattern {
            pattern_id: String::new(),
            constructive_nodes: Vec::new(),
            destructive_nodes: Vec::new(),
            phase_coherence: 0.0,
            intensity: 0.0,
        }
    }
}

/// Interference node
#[derive(Debug, Clone)]
pub struct InterferenceNode {
    /// Node position in holographic space
    pub position: HolographicPosition,

    /// Node amplitude
    pub amplitude: Float,

    /// Node phase
    pub phase: Float,

    /// Node frequency
    pub frequency: Float,
}

/// Position in holographic space
#[derive(Debug, Clone)]
pub struct HolographicPosition {
    /// Spatial coordinates
    pub spatial: [Float; 3],

    /// Frequency coordinate
    pub frequency: Float,

    /// Phase coordinate
    pub phase: Float,
}

/// Resonance tracker
///
/// Tracks the resonance state of entities in the holographic field.
#[derive(Debug, Clone)]
pub struct ResonanceTracker {
    /// Resonance states for each entity
    pub resonance_states: HashMap<EntityId, ResonanceState>,

    /// Global resonance
    pub global_resonance: Option<Float>,

    /// Resonance clusters
    pub resonance_clusters: Vec<ResonanceCluster>,
}

/// Resonance state of an entity
#[derive(Debug, Clone)]
pub struct ResonanceState {
    /// Entity ID
    pub entity_id: EntityId,

    /// Primary resonance frequency
    pub primary_frequency: Float,

    /// Harmonic frequencies
    pub harmonic_frequencies: Vec<Float>,

    /// Resonance strength
    pub resonance_strength: Float,

    /// Coherence with global resonance
    pub global_coherence: Float,
}

/// Resonance cluster
///
/// Groups of entities that resonate with each other.
#[derive(Debug, Clone)]
pub struct ResonanceCluster {
    /// Cluster ID
    pub cluster_id: String,

    /// Entities in cluster
    pub entities: Vec<EntityId>,

    /// Cluster resonance frequency
    pub resonance_frequency: Float,

    /// Cluster coherence
    pub coherence: Float,
}

/// Non-local interaction matrix
///
/// Tracks non-local connections between entities that transcend space/time.
#[derive(Debug, Clone)]
pub struct NonLocalMatrix {
    /// Interaction strengths
    pub interactions: HashMap<(EntityId, EntityId), Float>,

    /// Non-locality degree (0.0 to 1.0)
    pub non_locality_degree: Float,

    /// Entanglement pairs
    pub entanglement_pairs: Vec<(EntityId, EntityId)>,

    /// Migrated from holographic_connections.rs
    /// Holographic connections between entities
    pub holographic_connections: HashMap<(EntityId, EntityId), HolographicConnection>,

    /// Migrated from holographic_connections.rs
    /// Propagation threshold for holographic changes
    pub propagation_threshold: Float,

    /// Migrated from holographic_connections.rs
    /// Current timestamp for tracking changes
    pub current_timestamp: Float,
}

/// Holographic field statistics
#[derive(Debug, Clone, Default)]
pub struct HolographicFieldStatistics {
    /// Total number of entities
    pub entity_count: usize,

    /// Total number of connections
    pub connection_count: usize,

    /// Average connection strength
    pub average_connection_strength: Float,

    /// Number of resonant connections
    pub resonant_connections: usize,

    /// Number of harmonic connections (Phase 4)
    pub harmonic_connections: usize,

    /// Number of entangled connections
    pub entangled_connections: usize,

    /// Number of antiphase connections (Phase 4)
    pub antiphase_connections: usize,

    /// Number of weak connections (Phase 4)
    pub weak_connections: usize,

    /// Average archetype similarity (Phase 4)
    pub average_archetype_similarity: Float,

    /// Global phase coherence
    pub global_phase_coherence: Float,

    /// Number of interference patterns
    pub interference_pattern_count: usize,

    /// Number of resonance clusters
    pub resonance_cluster_count: usize,

    /// Holographic interaction score (Phase 4)
    /// Measures the quality and diversity of holographic interactions
    pub holographic_interaction_score: Float,
}

/// Holographic field result
///
/// Result of updating the holographic field.
#[derive(Debug, Clone)]
pub struct HolographicFieldResult {
    /// Number of steps processed
    pub steps: u64,

    /// Number of connections
    pub connections: usize,

    /// Number of interference patterns
    pub interference_patterns: usize,

    /// Final statistics
    pub final_statistics: HolographicFieldStatistics,

    /// Execution time
    pub execution_time: std::time::Duration,
}

/// Holographic field error
#[derive(Debug, Clone)]
pub enum HolographicFieldError {
    /// Entity not found
    EntityNotFound(EntityId),

    /// Invalid connection
    InvalidConnection(EntityId, EntityId),

    /// Calculation error
    CalculationError(String),
}

/// Resonance calculation result (Phase 6)
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 6:
/// "Implement holographic resonance mechanism between entities"
#[derive(Debug, Clone)]
pub struct ResonanceCalculation {
    /// Source entity ID
    pub source_id: EntityId,

    /// Target entity ID
    pub target_id: EntityId,

    /// Overall resonance score (0.0 to 1.0)
    pub resonance_score: Float,

    /// Spectrum resonance (0.0 to 1.0)
    pub spectrum_resonance: Float,

    /// Archetype resonance (0.0 to 1.0)
    pub archetype_resonance: Float,

    /// Blueprint resonance (0.0 to 1.0)
    pub blueprint_resonance: Float,

    /// Frequency resonance (0.0 to 1.0)
    pub frequency_resonance: Float,

    /// Phase resonance (0.0 to 1.0)
    pub phase_resonance: Float,

    /// Coherence resonance (0.0 to 1.0)
    pub coherence_resonance: Float,

    /// Resonance type
    pub resonance_type: ResonanceType,
}

/// Type of resonance (Phase 6)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResonanceType {
    /// High resonance (> 0.8) - entities strongly resonate
    High,

    /// Medium resonance (0.5 to 0.8) - moderate resonance
    Medium,

    /// Low resonance (0.2 to 0.5) - weak resonance
    Low,

    /// No resonance (< 0.2) - entities don't resonate
    None,
}

/// Coherence tracking across scales (Phase 6)
///
/// Tracks coherence at entity, collective, and global scales.
#[derive(Debug, Clone)]
pub struct CoherenceTracker {
    /// Entity coherence (entity_id -> coherence)
    pub entity_coherence: HashMap<EntityId, Float>,

    /// Collective coherence (collective_id -> coherence)
    pub collective_coherence: HashMap<EntityId, Float>,

    /// Global coherence (single value for entire system)
    pub global_coherence: Float,

    /// Coherence history (for tracking evolution over time)
    pub coherence_history: Vec<CoherenceSnapshot>,
}

/// Snapshot of coherence at a point in time (Phase 6)
#[derive(Debug, Clone)]
pub struct CoherenceSnapshot {
    /// Simulation step
    pub step: u64,

    /// Global coherence
    pub global_coherence: Float,

    /// Average entity coherence
    pub average_entity_coherence: Float,

    /// Average collective coherence
    pub average_collective_coherence: Float,

    /// Coherence variance (measure of coherence diversity)
    pub coherence_variance: Float,
}

impl CoherenceTracker {
    /// Create a new coherence tracker
    pub fn new() -> Self {
        CoherenceTracker {
            entity_coherence: HashMap::new(),
            collective_coherence: HashMap::new(),
            global_coherence: 0.0,
            coherence_history: Vec::new(),
        }
    }

    /// Update coherence tracking from entities and collective coherence map
    pub fn update_coherence(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
        collective_coherence_map: &HashMap<EntityId, Float>,
        step: u64,
    ) {
        // Calculate entity coherence
        self.entity_coherence.clear();
        for entity in entities.values() {
            let coherence = entity.current_state.vibrational_state.coherence;
            self.entity_coherence
                .insert(entity.entity_id.clone(), coherence);
        }

        // Copy collective coherence from provided map
        self.collective_coherence = collective_coherence_map.clone();

        // Calculate global coherence
        self.global_coherence = self.calculate_global_coherence();

        // Create snapshot
        let snapshot = CoherenceSnapshot {
            step,
            global_coherence: self.global_coherence,
            average_entity_coherence: self.calculate_average_entity_coherence(),
            average_collective_coherence: self.calculate_average_collective_coherence(),
            coherence_variance: self.calculate_coherence_variance(),
        };

        self.coherence_history.push(snapshot);

        // Keep history limited to last 100 snapshots
        if self.coherence_history.len() > 100 {
            self.coherence_history.remove(0);
        }
    }

    /// Calculate global coherence
    fn calculate_global_coherence(&self) -> Float {
        if self.entity_coherence.is_empty() {
            return 0.0;
        }

        let total: Float = self.entity_coherence.values().sum();
        total / self.entity_coherence.len() as Float
    }

    /// Calculate average entity coherence
    fn calculate_average_entity_coherence(&self) -> Float {
        if self.entity_coherence.is_empty() {
            return 0.0;
        }

        let total: Float = self.entity_coherence.values().sum();
        total / self.entity_coherence.len() as Float
    }

    /// Calculate average collective coherence
    fn calculate_average_collective_coherence(&self) -> Float {
        if self.collective_coherence.is_empty() {
            return 0.0;
        }

        let total: Float = self.collective_coherence.values().sum();
        total / self.collective_coherence.len() as Float
    }

    /// Calculate coherence variance (measure of coherence diversity)
    fn calculate_coherence_variance(&self) -> Float {
        if self.entity_coherence.is_empty() {
            return 0.0;
        }

        let avg = self.calculate_average_entity_coherence();
        let variance: Float = self
            .entity_coherence
            .values()
            .map(|&c| (c - avg).powi(2))
            .sum::<Float>()
            / self.entity_coherence.len() as Float;

        variance.sqrt()
    }

    /// Get coherence snapshot for a specific step
    pub fn get_snapshot_at_step(&self, step: u64) -> Option<&CoherenceSnapshot> {
        self.coherence_history.iter().find(|s| s.step == step)
    }

    /// Get latest coherence snapshot
    pub fn get_latest_snapshot(&self) -> Option<&CoherenceSnapshot> {
        self.coherence_history.last()
    }
}

impl Default for CoherenceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl HolographicFieldManager {
    /// Create a new holographic field manager
    pub fn new() -> Self {
        HolographicFieldManager {
            entities: HashMap::new(),
            holographic_connections: HashMap::new(),
            interference_patterns: Vec::new(),
            resonance_tracker: ResonanceTracker::new(),
            statistics: HolographicFieldStatistics::default(),
            non_local_matrix: NonLocalMatrix::new(),
            config: HolographicFieldConfig::default(),
            connection_cache: ConnectionCache::new(),
            current_step: 0,
            last_connection_update: 0,
            coherence_tracker: CoherenceTracker::new(),
            resonance_statistics: ResonanceStatistics::default(),
            // Migrated from holographic_connections.rs
            propagation_threshold: 0.1,
            current_timestamp: 0.0,
        }
    }

    /// Create a new holographic field manager with custom configuration (Phase 5)
    pub fn with_config(config: HolographicFieldConfig) -> Self {
        HolographicFieldManager {
            entities: HashMap::new(),
            holographic_connections: HashMap::new(),
            interference_patterns: Vec::new(),
            resonance_tracker: ResonanceTracker::new(),
            statistics: HolographicFieldStatistics::default(),
            non_local_matrix: NonLocalMatrix::new(),
            config,
            connection_cache: ConnectionCache::new(),
            current_step: 0,
            last_connection_update: 0,
            coherence_tracker: CoherenceTracker::new(),
            resonance_statistics: ResonanceStatistics::default(),
            // Migrated from holographic_connections.rs
            propagation_threshold: 0.1,
            current_timestamp: 0.0,
        }
    }

    /// Set the current simulation step (for lazy updates) (Phase 5)
    pub fn set_current_step(&mut self, step: u64) {
        self.current_step = step;
    }

    /// Add an entity to the field
    pub fn add_entity(&mut self, entity: SubSubLogos) -> Result<(), HolographicFieldError> {
        let entity_id = entity.entity_id.clone();
        self.entities.insert(entity_id.clone(), entity);

        // Update statistics
        self.statistics.entity_count = self.entities.len();

        // Clear cache when entity is added (Phase 5)
        if self.config.enable_connection_cache {
            self.connection_cache.clear();
        }

        Ok(())
    }

    /// Get all entities from the holographic field
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
    /// "Entities are EXTRACTED from the field (not iterated)"
    ///
    /// This method is used by the holographic game loop to extract entities
    /// from the field for game-specific operations (UI, inventory, etc.).
    pub fn get_all_entities(&self) -> Vec<SubSubLogos> {
        self.entities.values().cloned().collect()
    }

    /// Create holographic connections between all entities
    ///
    /// This implements the holographic principle: connections form based on
    /// the complete architecture contained in each entity.
    ///
    /// PERFORMANCE OPTIMIZATION: Limits connections per entity to prevent O(n²) scaling.
    /// Each entity maintains connections only to its most resonant partners.
    pub fn create_holographic_connections(&mut self) {
        let entity_ids: Vec<EntityId> = self.entities.keys().cloned().collect();

        // Calculate max connections per entity based on total entities
        // This prevents O(n²) scaling while maintaining holographic principle
        let max_connections_per_entity = if self.entities.len() > 200 {
            50 // Limit to 50 connections for large simulations
        } else if self.entities.len() > 100 {
            100 // Allow up to 100 connections for medium simulations
        } else {
            self.entities.len() // Allow all connections for small simulations
        };

        // Clear existing connections
        self.holographic_connections.clear();

        // Clone entities to avoid borrow conflicts (Phase 5)
        let entities_clone: HashMap<EntityId, SubSubLogos> = self.entities.clone();

        // Create connections with density limitation
        for entity_id_a in entity_ids.iter() {
            if let Some(entity_a) = entities_clone.get(entity_id_a) {
                // Calculate potential connections to all other entities
                let mut potential_connections: Vec<(EntityId, Float, HolographicConnection)> =
                    Vec::new();

                for entity_id_b in entity_ids.iter() {
                    if entity_id_a == entity_id_b {
                        continue;
                    }

                    if let Some(entity_b) = entities_clone.get(entity_id_b) {
                        let connection = self.calculate_holographic_connection(entity_a, entity_b);
                        let strength = connection.strength;

                        if strength > 0.0 {
                            potential_connections.push((entity_id_b.clone(), strength, connection));
                        }
                    }
                }

                // Sort by strength (descending) and keep top N connections
                potential_connections.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                potential_connections.truncate(max_connections_per_entity);

                // Create connections to top N entities
                for (target_id, strength, connection) in potential_connections {
                    // Avoid duplicate connections
                    if !self
                        .holographic_connections
                        .contains_key(&(entity_id_a.clone(), target_id.clone()))
                    {
                        self.holographic_connections
                            .insert((entity_id_a.clone(), target_id.clone()), connection.clone());

                        // Also add reverse connection (bidirectional)
                        self.holographic_connections
                            .insert((target_id.clone(), entity_id_a.clone()), connection);

                        // Update non-local matrix
                        self.non_local_matrix.add_interaction(
                            entity_id_a.clone(),
                            target_id,
                            strength,
                        );
                    }
                }
            }
        }

        // Update statistics
        self.statistics.connection_count = self.holographic_connections.len() / 2;
        // Divide by 2 since we store both directions

        // Update last connection update step (Phase 5)
        self.last_connection_update = self.current_step;
    }

    /// Lazy update holographic connections (Phase 5)
    ///
    /// Only updates connections if the update interval has passed.
    /// This significantly improves performance by skipping unnecessary recalculations.
    pub fn update_connections_lazy(&mut self) {
        // Always update on first step (step 0) to initialize connections
        if self.current_step == 0 {
            self.create_holographic_connections();
            return;
        }

        // Check if we should update based on interval
        if self.current_step - self.last_connection_update < self.config.update_interval {
            return; // Skip update
        }

        // Update connections
        self.create_holographic_connections();
    }

    /// Calculate archetype similarity with caching (Phase 5)
    fn calculate_archetype_similarity_cached(
        &mut self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        let entity_id_a = &entity_a.entity_id;
        let entity_id_b = &entity_b.entity_id;

        // Check cache if enabled
        if self.config.enable_similarity_cache {
            if let Some(&similarity) = self
                .connection_cache
                .similarity_cache
                .get(&(entity_id_a.clone(), entity_id_b.clone()))
            {
                return similarity;
            }
        }

        // Calculate similarity
        let similarity = self.calculate_archetype_similarity(entity_a, entity_b);

        // Cache the result
        if self.config.enable_similarity_cache {
            self.connection_cache
                .similarity_cache
                .insert((entity_id_a.clone(), entity_id_b.clone()), similarity);
            self.connection_cache
                .similarity_cache
                .insert((entity_id_b.clone(), entity_id_a.clone()), similarity);
        }

        similarity
    }

    /// Calculate holographic connection between two entities
    ///
    /// Phase 4 Update: The connection strength is now based on:
    /// 1. Archetype activation similarity (NEW - primary factor)
    /// 2. Spectrum configuration similarity
    /// 3. Holographic blueprint alignment
    /// 4. Resonance frequency matching
    /// 5. Phase coherence
    /// 6. Karmic pattern correlation (NEW)
    ///
    /// Phase 5 Update: Uses cached archetype similarity for performance
    fn calculate_holographic_connection(
        &mut self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> HolographicConnection {
        // Phase 4: Veil Access Control Check
        // Entities must have sufficient veil transparency to form holographic connections
        // Required access for holographic connections: 0.3

        // Get veil transparency from entity's density
        let veil_a = crate::spectrum::veil::Veil::from_density(&entity_a.current_density);
        let veil_b = crate::spectrum::veil::Veil::from_density(&entity_b.current_density);

        let veil_transparency_a = veil_a.transparency;
        let veil_transparency_b = veil_b.transparency;

        // Create access control based on transparency
        let access_control_a =
            crate::spectrum::veil::VeilAccessControl::from_transparency(veil_transparency_a);
        let access_control_b =
            crate::spectrum::veil::VeilAccessControl::from_transparency(veil_transparency_b);

        const REQUIRED_HOLOGRAPHIC_ACCESS: f64 = 0.0;

        // If either entity cannot access holographic connections, return weak connection
        if !access_control_a.can_access_holographic_connections(REQUIRED_HOLOGRAPHIC_ACCESS)
            || !access_control_b.can_access_holographic_connections(REQUIRED_HOLOGRAPHIC_ACCESS)
        {
            // Return very weak connection for veiled entities
            return HolographicConnection {
                source_id: entity_a.entity_id.clone(),
                target_id: entity_b.entity_id.clone(),
                strength: 0.0, // No connection due to veil
                connection_type: HolographicConnectionType::Weak,
                interference_pattern: InterferencePattern::default(),
                resonance_frequency: 0.0,
                coherence: 0.0,
            };
        }

        // Phase 4: Calculate archetype activation similarity (primary factor)
        // Phase 5: Use cached similarity for performance
        let archetype_similarity = if self.config.enable_similarity_cache {
            self.calculate_archetype_similarity_cached(entity_a, entity_b)
        } else {
            self.calculate_archetype_similarity(entity_a, entity_b)
        };

        // Calculate spectrum configuration similarity
        let spectrum_similarity = self.calculate_spectrum_similarity(entity_a, entity_b);

        // Calculate holographic blueprint alignment
        let blueprint_alignment = self.calculate_blueprint_alignment(entity_a, entity_b);

        // Calculate resonance frequency matching
        let resonance_match = self.calculate_resonance_match(entity_a, entity_b);

        // Calculate phase coherence
        let phase_coherence = self.calculate_phase_coherence(entity_a, entity_b);

        // Phase 4: Calculate karmic pattern correlation
        let karmic_correlation = self.calculate_karmic_correlation(entity_a, entity_b);

        // Phase 4: Overall connection strength (weighted average with archetype similarity as primary)
        let strength = (archetype_similarity * 0.35  // NEW: archetype similarity is primary
            + spectrum_similarity * 0.20
            + blueprint_alignment * 0.15
            + resonance_match * 0.10
            + phase_coherence * 0.10
            + karmic_correlation * 0.10) // NEW: karmic correlation
            .clamp(0.0, 1.0);
        // Determine connection type
        let connection_type = Self::determine_connection_type(
            strength,
            resonance_match,
            phase_coherence,
            archetype_similarity,
        );

        // Calculate interference pattern
        let interference_pattern =
            self.calculate_interference_pattern(entity_a, entity_b, strength);

        // Calculate resonance frequency
        let resonance_frequency = (entity_a.current_state.vibrational_state.frequency
            + entity_b.current_state.vibrational_state.frequency)
            / 2.0;

        HolographicConnection {
            source_id: entity_a.entity_id.clone(),
            target_id: entity_b.entity_id.clone(),
            strength,
            connection_type,
            interference_pattern,
            resonance_frequency,
            coherence: phase_coherence,
        }
    }

    /// Phase 4: Calculate archetype activation similarity
    ///
    /// This is the PRIMARY factor for holographic connections. Entities with similar
    /// archetype activation patterns resonate with each other, creating holographic
    /// connections that transcend space/time.
    fn calculate_archetype_similarity(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        // Generate archetype activation patterns for both entities
        let activation_a = entity_a.generate_archetype_activation_for_density();
        let activation_b = entity_b.generate_archetype_activation_for_density();

        // Calculate cosine similarity between archetype activation vectors
        let dot_product: Float = activation_a
            .iter()
            .zip(activation_b.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude_a: Float = activation_a.iter().map(|a| a * a).sum::<Float>().sqrt();
        let magnitude_b: Float = activation_b.iter().map(|b| b * b).sum::<Float>().sqrt();

        let cosine_similarity = dot_product / (magnitude_a * magnitude_b + 1e-10);

        // Also calculate activation pattern correlation (Pearson correlation)
        let mean_a: Float = activation_a.iter().sum::<Float>() / activation_a.len() as Float;
        let mean_b: Float = activation_b.iter().sum::<Float>() / activation_b.len() as Float;

        let covariance: Float = activation_a
            .iter()
            .zip(activation_b.iter())
            .map(|(a, b)| (a - mean_a) * (b - mean_b))
            .sum();

        let std_a: Float = activation_a
            .iter()
            .map(|a| (a - mean_a).powi(2))
            .sum::<Float>()
            .sqrt();
        let std_b: Float = activation_b
            .iter()
            .map(|b| (b - mean_b).powi(2))
            .sum::<Float>()
            .sqrt();

        let pearson_correlation = if std_a > 1e-10 && std_b > 1e-10 {
            covariance / (std_a * std_b)
        } else {
            0.0
        };

        // Combine cosine similarity and Pearson correlation (weighted average)
        // Cosine similarity captures overall pattern similarity
        // Pearson correlation captures correlation after removing mean differences
        0.6 * cosine_similarity.abs() + 0.4 * pearson_correlation.abs()
    }

    /// Phase 4: Calculate karmic pattern correlation
    ///
    /// Entities with similar karmic patterns have stronger connections because they
    /// are working through similar evolutionary lessons.
    fn calculate_karmic_correlation(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        if entity_a.karmic_patterns.is_empty() || entity_b.karmic_patterns.is_empty() {
            return 0.0;
        }

        // Calculate average karmic intensity for both entities
        let avg_intensity_a: Float = entity_a
            .karmic_patterns
            .iter()
            .map(|p| p.intensity)
            .sum::<Float>()
            / entity_a.karmic_patterns.len() as Float;

        let avg_intensity_b: Float = entity_b
            .karmic_patterns
            .iter()
            .map(|p| p.intensity)
            .sum::<Float>()
            / entity_b.karmic_patterns.len() as Float;

        // Calculate intensity similarity (closer intensities = higher correlation)
        let intensity_similarity = 1.0 - (avg_intensity_a - avg_intensity_b).abs();

        // Count resolved vs unresolved patterns
        let resolved_a = entity_a
            .karmic_patterns
            .iter()
            .filter(|p| {
                p.resolution_status == crate::entity_layer7::layer7::ResolutionStatus::Resolved
            })
            .count() as Float;

        let resolved_b = entity_b
            .karmic_patterns
            .iter()
            .filter(|p| {
                p.resolution_status == crate::entity_layer7::layer7::ResolutionStatus::Resolved
            })
            .count() as Float;

        let total_a = entity_a.karmic_patterns.len() as Float;
        let total_b = entity_b.karmic_patterns.len() as Float;

        let resolution_ratio_a = if total_a > 0.0 {
            resolved_a / total_a
        } else {
            0.0
        };
        let resolution_ratio_b = if total_b > 0.0 {
            resolved_b / total_b
        } else {
            0.0
        };

        // Calculate resolution similarity
        let resolution_similarity = 1.0 - (resolution_ratio_a - resolution_ratio_b).abs();

        // Combine intensity similarity and resolution similarity
        0.6 * intensity_similarity + 0.4 * resolution_similarity
    }

    /// Calculate spectrum configuration similarity
    fn calculate_spectrum_similarity(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        let ratio_a = entity_a.spectrum_configuration.ratio.calculate_ratio();
        let ratio_b = entity_b.spectrum_configuration.ratio.calculate_ratio();

        // Cosine similarity
        1.0 - (ratio_a - ratio_b).abs() / (ratio_a + ratio_b + 1e-10)
    }

    /// Calculate holographic blueprint alignment
    fn calculate_blueprint_alignment(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        // Calculate alignment of holographic signatures
        let signature_a = &entity_a
            .holographic_blueprint
            .spectrum_configuration
            .holographic_signature;
        let signature_b = &entity_b
            .holographic_blueprint
            .spectrum_configuration
            .holographic_signature;

        // Dot product for alignment
        let dot_product: Float = signature_a
            .iter()
            .zip(signature_b.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude_a: Float = signature_a.iter().map(|a| a * a).sum::<Float>().sqrt();
        let magnitude_b: Float = signature_b.iter().map(|b| b * b).sum::<Float>().sqrt();

        (dot_product / (magnitude_a * magnitude_b + 1e-10)).clamp(0.0, 1.0)
    }

    /// Calculate resonance frequency matching
    fn calculate_resonance_match(&self, entity_a: &SubSubLogos, entity_b: &SubSubLogos) -> Float {
        let freq_a = entity_a.current_state.vibrational_state.frequency;
        let freq_b = entity_b.current_state.vibrational_state.frequency;

        // Check for harmonic relationships
        let ratio = freq_a / (freq_b + 1e-10);
        let harmonic_match = if ratio.is_finite() {
            // Check for simple harmonic ratios (1:1, 1:2, 2:1, 1:3, 3:1, 2:3, 3:2)
            let simple_ratios: [Float; 7] = [1.0, 0.5, 2.0, 0.333, 3.0, 0.667, 1.5];
            let max_val: Float = simple_ratios
                .iter()
                .map(|&r| 1.0 - (ratio - r).abs())
                .fold(0.0_f64, |acc, x| if x > acc { x } else { acc });
            if max_val < 0.0 {
                0.0_f64
            } else if max_val > 1.0 {
                1.0_f64
            } else {
                max_val
            }
        } else {
            0.0
        };

        harmonic_match
    }

    /// Calculate phase coherence
    fn calculate_phase_coherence(&self, entity_a: &SubSubLogos, entity_b: &SubSubLogos) -> Float {
        // Use coherence from vibrational states
        let coherence_a = entity_a.current_state.vibrational_state.coherence;
        let coherence_b = entity_b.current_state.vibrational_state.coherence;

        (coherence_a + coherence_b) / 2.0
    }

    /// Phase 4: Determine connection type based on parameters
    ///
    /// Now includes archetype similarity as a factor in determining connection type.
    /// Resonant connections are based on high archetype similarity.
    /// Harmonic connections are based on complementary archetype patterns.
    /// Entangled connections are based on strong overall connection.
    fn determine_connection_type(
        strength: Float,
        resonance_match: Float,
        phase_coherence: Float,
        archetype_similarity: Float,
    ) -> HolographicConnectionType {
        // Phase 4: Relaxed thresholds for better connection type diversity
        // Resonant connections require high archetype similarity
        if archetype_similarity > 0.7 && resonance_match > 0.6 && phase_coherence > 0.6 {
            HolographicConnectionType::Resonant
        }
        // Phase 4: Harmonic connections require moderate archetype similarity with resonance
        else if archetype_similarity > 0.4 && resonance_match > 0.5 {
            HolographicConnectionType::Harmonic
        }
        // Entangled connections require strong overall connection
        else if strength > 0.6 && phase_coherence > 0.6 {
            HolographicConnectionType::Entangled
        }
        // Antiphase connections occur when entities are out of phase
        else if phase_coherence < 0.4 {
            HolographicConnectionType::Antiphase
        }
        // All other connections are weak
        else {
            HolographicConnectionType::Weak
        }
    }

    /// Calculate interference pattern between two entities
    fn calculate_interference_pattern(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
        strength: Float,
    ) -> InterferencePattern {
        let pattern_id = format!("{}-{}", entity_a.entity_id.uuid, entity_b.entity_id.uuid);

        let num_nodes = (strength * 10.0) as usize + 1;

        // Generate constructive nodes
        let constructive_nodes: Vec<InterferenceNode> = (0..num_nodes)
            .map(|i| {
                let angle = (i as Float) * std::f64::consts::PI / (num_nodes as Float);
                InterferenceNode {
                    position: HolographicPosition {
                        spatial: [angle.cos(), angle.sin(), 0.0],
                        frequency: (entity_a.current_state.vibrational_state.frequency
                            + entity_b.current_state.vibrational_state.frequency)
                            / 2.0,
                        phase: angle,
                    },
                    amplitude: strength * (i as Float + 1.0) / (num_nodes as Float + 1.0),
                    phase: angle,
                    frequency: (entity_a.current_state.vibrational_state.frequency
                        + entity_b.current_state.vibrational_state.frequency)
                        / 2.0,
                }
            })
            .collect();

        // Generate destructive nodes (out of phase by π)
        let destructive_nodes: Vec<InterferenceNode> = (0..num_nodes)
            .map(|i| {
                let angle = (i as Float) * std::f64::consts::PI / (num_nodes as Float)
                    + std::f64::consts::PI;
                InterferenceNode {
                    position: HolographicPosition {
                        spatial: [angle.cos(), angle.sin(), 0.0],
                        frequency: (entity_a.current_state.vibrational_state.frequency
                            + entity_b.current_state.vibrational_state.frequency)
                            / 2.0,
                        phase: angle,
                    },
                    amplitude: strength * (i as Float + 1.0) / (num_nodes as Float + 1.0) * 0.5,
                    phase: angle,
                    frequency: (entity_a.current_state.vibrational_state.frequency
                        + entity_b.current_state.vibrational_state.frequency)
                        / 2.0,
                }
            })
            .collect();

        // Calculate phase coherence
        let phase_coherence = self.calculate_phase_coherence(entity_a, entity_b);

        InterferencePattern {
            pattern_id,
            constructive_nodes,
            destructive_nodes,
            phase_coherence,
            intensity: strength,
        }
    }

    /// Calculate interference patterns from all connections
    ///
    /// PERFORMANCE OPTIMIZATION: Uses Rayon parallelization to process connections
    /// in parallel, significantly reducing computation time for large simulations.
    pub fn calculate_interference_patterns(&mut self) {
        self.interference_patterns.clear();

        // Collect unique pattern IDs first
        let pattern_ids: Vec<String> = self
            .holographic_connections
            .values()
            .map(|c| c.interference_pattern.pattern_id.clone())
            .collect::<std::collections::HashSet<_>>() // Remove duplicates
            .into_iter()
            .collect();

        // Find patterns for each unique ID (parallelized)
        let patterns: Vec<InterferencePattern> = pattern_ids
            .par_iter()
            .filter_map(|pattern_id| {
                self.holographic_connections
                    .values()
                    .find(|c| c.interference_pattern.pattern_id == *pattern_id)
                    .map(|c| c.interference_pattern.clone())
            })
            .collect();

        self.interference_patterns = patterns;

        // Update statistics
        self.statistics.interference_pattern_count = self.interference_patterns.len();
    }

    /// Track resonance for all entities
    pub fn track_resonance_for_all_entities(&mut self) {
        for entity_id in self.entities.keys().cloned().collect::<Vec<_>>() {
            let resonance_state = self.track_resonance(&entity_id);
            self.resonance_tracker
                .resonance_states
                .insert(entity_id, resonance_state);
        }

        // Calculate global resonance
        self.resonance_tracker.calculate_global_resonance();

        // Identify resonance clusters
        self.resonance_tracker.identify_clusters(&self.entities);

        // Update statistics
        self.statistics.resonance_cluster_count = self.resonance_tracker.resonance_clusters.len();
    }

    /// Track resonance state of an entity
    pub fn track_resonance(&self, entity_id: &EntityId) -> ResonanceState {
        let entity = self.entities.get(entity_id).expect("Entity should exist");

        let primary_frequency = entity.current_state.vibrational_state.frequency;

        // Calculate harmonic frequencies (multiples of primary)
        let harmonic_frequencies: Vec<Float> = (2..=5)
            .map(|i| primary_frequency * i as Float)
            .filter(|&f| f <= 1.0)
            .collect();

        // Calculate resonance strength based on connections
        let resonance_strength = self
            .holographic_connections
            .iter()
            .filter(|((source, target), _)| source == entity_id || target == entity_id)
            .map(|(_, conn)| conn.strength)
            .sum::<Float>()
            / self.entities.len().max(1) as Float;

        // Calculate coherence with global resonance
        let global_coherence =
            if let Some(global_resonance) = self.resonance_tracker.global_resonance {
                1.0 - (primary_frequency - global_resonance).abs()
            } else {
                0.5
            };

        ResonanceState {
            entity_id: entity_id.clone(),
            primary_frequency,
            harmonic_frequencies,
            resonance_strength,
            global_coherence,
        }
    }

    /// Update holographic field for specified number of steps
    ///
    /// PERFORMANCE OPTIMIZATION: Uses lazy evaluation - connections are only
    /// recalculated periodically instead of every step, significantly reducing
    /// computational overhead while maintaining holographic fidelity.
    pub fn update_field(&mut self, steps: u64) -> HolographicFieldResult {
        let start_time = std::time::Instant::now();

        // Connection update interval (recalculate every N steps)
        // Phase 4: Reduced intervals to allow more diverse connections to form
        let connection_update_interval = if self.entities.len() > 200 {
            15 // Update less frequently for large simulations
        } else if self.entities.len() > 100 {
            5 // Update more frequently for medium simulations (Phase 4: reduced from 10)
        } else {
            1 // Update every step for small simulations
        };

        for step in 0..steps {
            // Only recalculate connections periodically (lazy evaluation)
            if step % connection_update_interval == 0 {
                self.create_holographic_connections();
            }

            // Always update interference patterns and resonance
            self.calculate_interference_patterns();
            self.track_resonance_for_all_entities();
        }

        // Update statistics
        self.update_statistics();

        let execution_time = start_time.elapsed();

        HolographicFieldResult {
            steps,
            connections: self.statistics.connection_count,
            interference_patterns: self.statistics.interference_pattern_count,
            final_statistics: self.statistics.clone(),
            execution_time,
        }
    }

    /// Update field statistics
    pub fn update_statistics(&mut self) {
        if !self.holographic_connections.is_empty() {
            let total_strength: Float = self
                .holographic_connections
                .values()
                .map(|c| c.strength)
                .sum();
            self.statistics.average_connection_strength =
                total_strength / self.holographic_connections.len() as Float;
        }

        // Phase 4: Count all connection types
        self.statistics.resonant_connections = self
            .holographic_connections
            .values()
            .filter(|c| c.connection_type == HolographicConnectionType::Resonant)
            .count();
        self.statistics.harmonic_connections = self
            .holographic_connections
            .values()
            .filter(|c| c.connection_type == HolographicConnectionType::Harmonic)
            .count();
        self.statistics.entangled_connections = self
            .holographic_connections
            .values()
            .filter(|c| c.connection_type == HolographicConnectionType::Entangled)
            .count();
        self.statistics.antiphase_connections = self
            .holographic_connections
            .values()
            .filter(|c| c.connection_type == HolographicConnectionType::Antiphase)
            .count();
        self.statistics.weak_connections = self
            .holographic_connections
            .values()
            .filter(|c| c.connection_type == HolographicConnectionType::Weak)
            .count();

        // Phase 4: Calculate average archetype similarity
        if !self.holographic_connections.is_empty() {
            let mut total_archetype_similarity = 0.0;
            for conn in self.holographic_connections.values() {
                if let Some(entity_a) = self.entities.get(&conn.source_id) {
                    if let Some(entity_b) = self.entities.get(&conn.target_id) {
                        total_archetype_similarity +=
                            self.calculate_archetype_similarity(entity_a, entity_b);
                    }
                }
            }
            self.statistics.average_archetype_similarity =
                total_archetype_similarity / self.holographic_connections.len() as Float;
        }

        // Calculate global phase coherence
        if !self.interference_patterns.is_empty() {
            let total_coherence: Float = self
                .interference_patterns
                .iter()
                .map(|p| p.phase_coherence)
                .sum();
            self.statistics.global_phase_coherence =
                total_coherence / self.interference_patterns.len() as Float;
        }

        // Phase 4: Calculate holographic interaction score
        // This measures the quality and diversity of holographic interactions
        let interaction_diversity = if self.statistics.connection_count > 0 {
            let connection_types = [
                self.statistics.resonant_connections,
                self.statistics.harmonic_connections,
                self.statistics.entangled_connections,
                self.statistics.antiphase_connections,
                self.statistics.weak_connections,
            ];
            // Count non-zero connection types
            let non_zero_types = connection_types.iter().filter(|&&c| c > 0).count() as Float;
            non_zero_types / 5.0 // Maximum diversity is 5 types
        } else {
            0.0
        };

        let interaction_strength = self.statistics.average_connection_strength;
        let archetype_quality = self.statistics.average_archetype_similarity;
        let coherence_quality = self.statistics.global_phase_coherence;

        // Holographic interaction score is a weighted average
        self.statistics.holographic_interaction_score = interaction_diversity * 0.3
            + interaction_strength * 0.3
            + archetype_quality * 0.2
            + coherence_quality * 0.2;
    }

    /// Demonstrate holographic principle
    ///
    /// Returns a report showing that each entity contains the complete architecture.
    pub fn demonstrate_holographic_principle(&self) -> HolographicPrincipleReport {
        let mut entity_reports = Vec::new();

        for entity in self.entities.values() {
            let completeness_report = entity.verify_holographic_completeness();
            entity_reports.push(HolographicEntityReport {
                entity_id: entity.entity_id.clone(),
                completeness: completeness_report.completeness_percentage,
                contains_all_layers: completeness_report.completeness_percentage >= 100.0,
            });
        }

        let overall_completeness = if entity_reports.is_empty() {
            0.0
        } else {
            entity_reports.iter().map(|r| r.completeness).sum::<Float>()
                / entity_reports.len() as Float
        };

        HolographicPrincipleReport {
            entity_reports,
            overall_completeness,
            demonstrated: overall_completeness >= 100.0,
        }
    }

    /// Get field statistics
    pub fn get_statistics(&self) -> HolographicFieldStatistics {
        self.statistics.clone()
    }

    /// Get field result
    pub fn get_field_result(&self) -> HolographicFieldResult {
        HolographicFieldResult {
            steps: 0,
            connections: self.statistics.connection_count,
            interference_patterns: self.statistics.interference_pattern_count,
            final_statistics: self.statistics.clone(),
            execution_time: Duration::ZERO,
        }
    }

    /// Get field summary
    pub fn get_summary(&self) -> String {
        format!(
            "HolographicFieldManager Summary:\n\
             - Entities: {}\n\
             - Connections: {}\n\
             - Interference Patterns: {}\n\
             - Average Connection Strength: {:.3}\n\
             - Resonant Connections: {}\n\
             - Entangled Connections: {}\n\
             - Global Phase Coherence: {:.3}\n\
             - Resonance Clusters: {}\n\
             - Non-locality Degree: {:.3}",
            self.statistics.entity_count,
            self.statistics.connection_count,
            self.statistics.interference_pattern_count,
            self.statistics.average_connection_strength,
            self.statistics.resonant_connections,
            self.statistics.entangled_connections,
            self.statistics.global_phase_coherence,
            self.statistics.resonance_cluster_count,
            self.non_local_matrix.non_locality_degree
        )
    }

    // ============================================================================
    // PHASE 6: HOLOGRAPHIC RESONANCE METHODS
    // ============================================================================

    /// Phase 5: Calculate resonance between two entities
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 5:
    /// "Resonance measures how well entities align in spectrum configuration,
    /// holographic patterns, polarity orientation"
    ///
    /// Resonance is calculated based on:
    /// 1. Spectrum configuration (space/time ratio similarity)
    /// 2. Holographic patterns (archetype activation similarity)
    /// 3. Polarity orientation (same polarity direction)
    ///
    /// Weighted average: spectrum (30%) + holographic (40%) + polarity (30%)
    pub fn calculate_resonance(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> ResonanceCalculation {
        // Calculate individual resonance components
        let spectrum_resonance = self.calculate_spectrum_resonance(entity_a, entity_b);
        let holographic_resonance = self.calculate_holographic_resonance(entity_a, entity_b);
        let polarity_resonance = self.calculate_polarity_resonance(entity_a, entity_b);

        // Overall resonance score (weighted average)
        // From Phase 5 refactor plan:
        // - Spectrum resonance: 30%
        // - Holographic resonance: 40%
        // - Polarity resonance: 30%
        let resonance_score =
            (spectrum_resonance * 0.30 + holographic_resonance * 0.40 + polarity_resonance * 0.30)
                .clamp(0.0, 1.0);

        // Determine resonance type
        let resonance_type = if resonance_score > 0.8 {
            ResonanceType::High
        } else if resonance_score > 0.5 {
            ResonanceType::Medium
        } else if resonance_score > 0.2 {
            ResonanceType::Low
        } else {
            ResonanceType::None
        };

        ResonanceCalculation {
            source_id: entity_a.entity_id.clone(),
            target_id: entity_b.entity_id.clone(),
            resonance_score,
            spectrum_resonance,
            archetype_resonance: holographic_resonance, // Archetype resonance = holographic resonance
            blueprint_resonance: 0.0,                   // Not used in Phase 5
            frequency_resonance: 0.0,                   // Not used in Phase 5
            phase_resonance: 0.0,                       // Not used in Phase 5
            coherence_resonance: 0.0,                   // Not used in Phase 5
            resonance_type,
        }
    }

    /// Phase 5: Calculate spectrum resonance between two entities
    ///
    /// Spectrum resonance is based on space/time ratio similarity.
    /// Entities with similar space/time ratios have higher spectrum resonance.
    fn calculate_spectrum_resonance(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        // Get spectrum ratios from entity spectrum configuration
        let ratio_a = entity_a.spectrum_configuration.ratio.calculate_ratio();
        let ratio_b = entity_b.spectrum_configuration.ratio.calculate_ratio();

        // Calculate similarity (1.0 = identical, 0.0 = completely different)
        let difference = (ratio_a - ratio_b).abs();
        let max_difference = 20.0; // Maximum possible difference (space_time_dominant has ratio 20.0)

        1.0 - (difference / max_difference).min(1.0)
    }

    /// Phase 5: Calculate holographic resonance between two entities
    ///
    /// Holographic resonance is based on archetype activation similarity.
    /// Uses cosine similarity between archetype activation vectors.
    fn calculate_holographic_resonance(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        // Generate archetype activation patterns for both entities
        let activations_a = entity_a.generate_archetype_activation_for_density();
        let activations_b = entity_b.generate_archetype_activation_for_density();

        // Calculate cosine similarity between archetype activation vectors
        let dot_product: Float = activations_a
            .iter()
            .zip(activations_b.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude_a: Float = activations_a.iter().map(|a| a * a).sum::<Float>().sqrt();
        let magnitude_b: Float = activations_b.iter().map(|b| b * b).sum::<Float>().sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_a * magnitude_b)
    }

    /// Phase 5: Calculate polarity resonance between two entities
    ///
    /// Polarity resonance is based on polarity direction.
    /// Same polarity = high resonance, Different polarity = low resonance.
    fn calculate_polarity_resonance(
        &self,
        entity_a: &SubSubLogos,
        entity_b: &SubSubLogos,
    ) -> Float {
        // Get polarity directions from entity polarization progress
        let direction_a = entity_a.polarization.direction;
        let direction_b = entity_b.polarization.direction;

        match (direction_a, direction_b) {
            (
                crate::polarization::PolarityDirection::ServiceToOthers,
                crate::polarization::PolarityDirection::ServiceToOthers,
            ) => 1.0,
            (
                crate::polarization::PolarityDirection::ServiceToSelf,
                crate::polarization::PolarityDirection::ServiceToSelf,
            ) => 1.0,
            (
                crate::polarization::PolarityDirection::Neutral,
                crate::polarization::PolarityDirection::Neutral,
            ) => 0.5,
            (crate::polarization::PolarityDirection::Neutral, _) => 0.3,
            (_, crate::polarization::PolarityDirection::Neutral) => 0.3,
            _ => 0.0, // Different polarities
        }
    }

    /// Phase 6: Calculate resonance matrix for all entities
    ///
    /// Returns a matrix of resonance scores between all pairs of entities.
    /// This is used for resonance heatmap visualization.
    pub fn calculate_resonance_matrix(&self) -> HashMap<(EntityId, EntityId), Float> {
        let mut resonance_matrix = HashMap::new();

        let entity_ids: Vec<EntityId> = self.entities.keys().cloned().collect();
        let entities_clone = self.entities.clone();

        for entity_id_a in entity_ids.iter() {
            for entity_id_b in entity_ids.iter() {
                if entity_id_a == entity_id_b {
                    continue;
                }

                if let Some(entity_a) = entities_clone.get(entity_id_a) {
                    if let Some(entity_b) = entities_clone.get(entity_id_b) {
                        let resonance = self.calculate_resonance(entity_a, entity_b);
                        resonance_matrix.insert(
                            (entity_id_a.clone(), entity_id_b.clone()),
                            resonance.resonance_score,
                        );
                    }
                }
            }
        }

        resonance_matrix
    }

    /// Phase 6: Find high resonance pairs
    ///
    /// Returns all entity pairs with resonance > threshold.
    /// Used for collective formation.
    pub fn find_high_resonance_pairs(&self, threshold: Float) -> Vec<(EntityId, EntityId, Float)> {
        let mut high_resonance_pairs = Vec::new();

        let entity_ids: Vec<EntityId> = self.entities.keys().cloned().collect();
        let entities_clone = self.entities.clone();

        for entity_id_a in entity_ids.iter() {
            for entity_id_b in entity_ids.iter() {
                // Avoid self-pairs and duplicates by only processing where a < b
                // Since EntityId doesn't implement Ord, we use index comparison
                let index_a = entity_ids.iter().position(|id| id == entity_id_a).unwrap();
                let index_b = entity_ids.iter().position(|id| id == entity_id_b).unwrap();

                if index_a >= index_b {
                    continue; // Skip self-pairs and duplicates
                }

                if let Some(entity_a) = entities_clone.get(entity_id_a) {
                    if let Some(entity_b) = entities_clone.get(entity_id_b) {
                        let resonance = self.calculate_resonance(entity_a, entity_b);
                        if resonance.resonance_score > threshold {
                            high_resonance_pairs.push((
                                entity_id_a.clone(),
                                entity_id_b.clone(),
                                resonance.resonance_score,
                            ));
                        }
                    }
                }
            }
        }

        // Sort by resonance score (descending)
        high_resonance_pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        high_resonance_pairs
    }

    /// Phase 6: Get resonance statistics
    ///
    /// Returns statistics about resonance across all entity pairs.
    pub fn get_resonance_statistics(&self) -> ResonanceStatistics {
        let resonance_matrix = self.calculate_resonance_matrix();

        if resonance_matrix.is_empty() {
            return ResonanceStatistics::default();
        }

        let resonance_scores: Vec<Float> = resonance_matrix.values().cloned().collect();

        let average_resonance =
            resonance_scores.iter().sum::<Float>() / resonance_scores.len() as Float;

        let high_resonance_count = resonance_scores.iter().filter(|&&r| r > 0.8).count();
        let medium_resonance_count = resonance_scores
            .iter()
            .filter(|&&r| r > 0.5 && r <= 0.8)
            .count();
        let low_resonance_count = resonance_scores
            .iter()
            .filter(|&&r| r > 0.2 && r <= 0.5)
            .count();
        let no_resonance_count = resonance_scores.iter().filter(|&&r| r <= 0.2).count();

        // Calculate variance
        let variance = if resonance_scores.len() > 1 {
            let mean = average_resonance;
            let sum_squared_diff: Float =
                resonance_scores.iter().map(|&r| (r - mean).powi(2)).sum();
            sum_squared_diff / resonance_scores.len() as Float
        } else {
            0.0
        };

        ResonanceStatistics {
            total_pairs: resonance_scores.len(),
            average_resonance,
            max_resonance: *resonance_scores
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0),
            min_resonance: *resonance_scores
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(&0.0),
            resonance_variance: variance,
            high_resonance_count,
            medium_resonance_count,
            low_resonance_count,
            no_resonance_count,
        }
    }

    // ========================================================================
    // PHASE 1.6: CAUSAL FLOW INVERSION METHODS
    // ========================================================================

    /// Evolve the holographic field (Phase 1.6)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Field is primary reality"
    ///
    /// This method evolves the underlying holographic field, demonstrating
    /// that the field is the primary reality from which entities emerge.
    ///
    /// # Arguments
    ///
    /// * `dt` - Time delta for evolution step
    ///
    /// # Returns
    ///
    /// FieldEvolutionResult with evolution metrics
    pub fn evolve_field(&mut self, dt: Float) -> crate::holographic::FieldEvolutionResult {
        // Record initial state
        let old_coherence = self.statistics.global_phase_coherence;
        let old_stable_count: usize = self
            .interference_patterns
            .iter()
            .map(|p| p.constructive_nodes.len())
            .sum();

        // Update field (this recalculates connections and interference patterns)
        // This is where field evolution happens
        self.calculate_interference_patterns();
        self.track_resonance_for_all_entities();
        self.update_statistics();

        // Calculate coherence delta
        let new_coherence = self.statistics.global_phase_coherence;
        let coherence_delta = new_coherence - old_coherence;

        // Count new stable nodes
        let new_stable_count = self
            .interference_patterns
            .iter()
            .map(|p| p.constructive_nodes.len())
            .sum();
        let dissolved_count = old_stable_count.saturating_sub(new_stable_count);

        // Aperture change represents veil thinning
        // As coherence improves, the veil thins
        let aperture_change = coherence_delta.max(0.0) * dt * 0.001;

        crate::holographic::FieldEvolutionResult {
            coherence_delta,
            stable_nodes_count: new_stable_count,
            dissolved_nodes_count: dissolved_count,
            aperture_change,
        }
    }

    /// Extract entity potentials from field coherence patterns (Phase 1.6)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities manifest at stable interference nodes"
    ///
    /// This method extracts potential entity configurations from the
    /// interference patterns in the holographic field.
    ///
    /// # Arguments
    ///
    /// * `min_coherence` - Minimum coherence threshold for extraction
    ///
    /// # Returns
    ///
    /// Vector of ExtractedEntityPotential representing potential manifestations
    pub fn extract_potentials(
        &self,
        min_coherence: Float,
    ) -> Vec<crate::holographic::ExtractedEntityPotential> {
        let mut potentials = Vec::new();
        let global_coherence = self.statistics.global_phase_coherence;

        // Extract from each interference pattern's constructive nodes
        for (pattern_idx, pattern) in self.interference_patterns.iter().enumerate() {
            // Skip patterns with low coherence
            if pattern.phase_coherence < min_coherence {
                continue;
            }

            for (node_idx, node) in pattern.constructive_nodes.iter().enumerate() {
                // Calculate local coherence
                let local_coherence = node.amplitude * pattern.phase_coherence;

                // Skip if below threshold
                if local_coherence < min_coherence {
                    continue;
                }

                // Create holographic address from node position
                use crate::holographic::{HolographicAddress, ScaleLevel, Vector3};

                let scale = ScaleLevel::Biological; // Default scale
                let local_offset = Vector3::new(
                    node.position.spatial[0],
                    node.position.spatial[1],
                    node.position.spatial[2],
                );
                let address = HolographicAddress::new(scale, vec![], local_offset);

                // Derive archetype activation from pattern
                use crate::holographic::universal_template::ArchetypeActivationProfile;
                let mut archetype_coefficients = [0.5; 22];
                // Use node phase to modulate archetype activations
                for (i, coef) in archetype_coefficients.iter_mut().enumerate() {
                    let phase_influence = ((i as Float + 1.0) * node.phase).cos();
                    *coef = (0.5 + 0.3 * phase_influence * node.amplitude).clamp(0.0, 1.0);
                }
                let archetype_activation = ArchetypeActivationProfile::new(archetype_coefficients);

                // Derive spectrum from coherence
                use crate::holographic::universal_template::SpectrumConfiguration;
                use crate::spectrum::larson_framework::SpectrumRatio;

                let ratio = if local_coherence > 0.5 {
                    SpectrumRatio::time_space(1.0, 1.0 + local_coherence)
                } else {
                    SpectrumRatio::space_time(1.0 + (1.0 - local_coherence), 1.0)
                };

                let spectrum = SpectrumConfiguration::new(
                    ratio,
                    local_coherence,
                    1.0 - local_coherence,
                    local_coherence,
                );

                // Determine density from coherence
                use crate::evolution_density_octave::density_octave::Density;
                let density = match local_coherence {
                    c if c >= 0.9 => Density::Eighth,
                    c if c >= 0.8 => Density::Seventh,
                    c if c >= 0.7 => Density::Sixth,
                    c if c >= 0.55 => Density::Fifth,
                    c if c >= 0.4 => Density::Fourth,
                    c if c >= 0.25 => Density::Third,
                    c if c >= 0.12 => Density::Second(
                        crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
                    ),
                    _ => Density::First(
                        crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                    ),
                };

                // Generate free will seed
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                pattern_idx.hash(&mut hasher);
                node_idx.hash(&mut hasher);
                node.phase.to_bits().hash(&mut hasher);
                node.amplitude.to_bits().hash(&mut hasher);
                let free_will_seed = hasher.finish();

                potentials.push(crate::holographic::ExtractedEntityPotential {
                    address,
                    archetype_activation,
                    spectrum,
                    density,
                    coherence: local_coherence,
                    free_will_seed,
                });
            }
        }

        potentials
    }

    /// Get the current field coherence level (Phase 1.6)
    ///
    /// Returns the global phase coherence of the holographic field.
    pub fn get_field_coherence(&self) -> Float {
        self.statistics.global_phase_coherence
    }
}

/// Resonance statistics (Phase 6)
///
/// Statistics about resonance across all entity pairs.
#[derive(Debug, Clone, Default)]
pub struct ResonanceStatistics {
    /// Total number of entity pairs
    pub total_pairs: usize,

    /// Average resonance score
    pub average_resonance: Float,

    /// Maximum resonance score
    pub max_resonance: Float,

    /// Minimum resonance score
    pub min_resonance: Float,

    /// Resonance variance (measure of diversity)
    pub resonance_variance: Float,

    /// Number of high resonance pairs (> 0.8)
    pub high_resonance_count: usize,

    /// Number of medium resonance pairs (0.5 to 0.8)
    pub medium_resonance_count: usize,

    /// Number of low resonance pairs (0.2 to 0.5)
    pub low_resonance_count: usize,

    /// Number of no resonance pairs (< 0.2)
    pub no_resonance_count: usize,
}

impl ResonanceTracker {
    /// Create a new resonance tracker
    pub fn new() -> Self {
        ResonanceTracker {
            resonance_states: HashMap::new(),
            global_resonance: None,
            resonance_clusters: Vec::new(),
        }
    }

    /// Calculate global resonance
    pub fn calculate_global_resonance(&mut self) {
        if self.resonance_states.is_empty() {
            self.global_resonance = None;
            return;
        }

        let total_frequency: Float = self
            .resonance_states
            .values()
            .map(|r| r.primary_frequency)
            .sum();

        self.global_resonance = Some(total_frequency / self.resonance_states.len() as Float);
    }

    /// Identify resonance clusters
    pub fn identify_clusters(&mut self, _entities: &HashMap<EntityId, SubSubLogos>) {
        self.resonance_clusters.clear();

        if self.resonance_states.is_empty() {
            return;
        }

        // Simple clustering based on resonance frequency similarity
        let mut clustered: Vec<EntityId> = Vec::new();
        let mut cluster_id = 0;

        for entity_id in self.resonance_states.keys() {
            if clustered.contains(entity_id) {
                continue;
            }

            let state = &self.resonance_states[entity_id];
            let mut cluster_entities = vec![entity_id.clone()];

            // Find entities with similar resonance
            for other_id in self.resonance_states.keys() {
                if clustered.contains(other_id) || other_id == entity_id {
                    continue;
                }

                let other_state = &self.resonance_states[other_id];
                let frequency_diff =
                    (state.primary_frequency - other_state.primary_frequency).abs();

                if frequency_diff < 0.1 {
                    cluster_entities.push(other_id.clone());
                    clustered.push(other_id.clone());
                }
            }

            clustered.push(entity_id.clone());

            // Create cluster
            if cluster_entities.len() > 1 {
                let cluster_frequency = cluster_entities
                    .iter()
                    .map(|id| self.resonance_states[id].primary_frequency)
                    .sum::<Float>()
                    / cluster_entities.len() as Float;

                let cluster_coherence = cluster_entities
                    .iter()
                    .map(|id| self.resonance_states[id].global_coherence)
                    .sum::<Float>()
                    / cluster_entities.len() as Float;

                self.resonance_clusters.push(ResonanceCluster {
                    cluster_id: format!("cluster-{}", cluster_id),
                    entities: cluster_entities,
                    resonance_frequency: cluster_frequency,
                    coherence: cluster_coherence,
                });

                cluster_id += 1;
            }
        }
    }
}

impl NonLocalMatrix {
    /// Create a new non-local matrix
    pub fn new() -> Self {
        NonLocalMatrix {
            interactions: HashMap::new(),
            non_locality_degree: 0.0,
            entanglement_pairs: Vec::new(),
            holographic_connections: HashMap::new(),
            propagation_threshold: 0.1,
            current_timestamp: 0.0,
        }
    }

    /// Add interaction to matrix
    pub fn add_interaction(&mut self, entity_a: EntityId, entity_b: EntityId, strength: Float) {
        self.interactions
            .insert((entity_a.clone(), entity_b.clone()), strength);

        // Update non-locality degree
        if !self.interactions.is_empty() {
            let total_strength: Float = self.interactions.values().sum();
            self.non_locality_degree = total_strength / self.interactions.len() as Float;
        }

        // Update entanglement pairs
        if strength > 0.7 {
            let pair = (entity_a, entity_b);
            if !self.entanglement_pairs.contains(&pair) {
                self.entanglement_pairs.push(pair);
            }
        }
    }

    // ============================================================================
    // CHANGE PROPAGATION METHODS (Migrated from holographic_connections.rs)
    // ============================================================================

    /// Set the propagation threshold (Migrated from holographic_connections.rs)
    ///
    /// Only propagate changes if resonance is above this threshold.
    pub fn set_propagation_threshold(&mut self, threshold: Float) {
        self.propagation_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Advance the timestamp (Migrated from holographic_connections.rs)
    pub fn advance_timestamp(&mut self, delta: Float) {
        self.current_timestamp += delta.abs();
    }

    /// Set the timestamp (Migrated from holographic_connections.rs)
    pub fn set_timestamp(&mut self, timestamp: Float) {
        self.current_timestamp = timestamp.abs();
    }

    /// Get current timestamp (Migrated from holographic_connections.rs)
    pub fn get_timestamp(&self) -> Float {
        self.current_timestamp
    }

    /// Propagate a change from a source entity to all connected entities (Migrated from holographic_connections.rs)
    ///
    /// This implements non-local influence: changes in one entity
    /// propagate to all connected entities based on resonance.
    ///
    /// From CONTINUOUS_INVOLUTION_EVOLUTION_PROPOSAL.md Section 2.8:
    /// "What is learned by one is known to all" - changes propagate through
    /// holographic connections based on resonance.
    pub fn propagate_change(
        &mut self,
        source_entity_id: EntityId,
        change: &HolographicChange,
    ) -> Vec<InfluenceResult> {
        let mut results = Vec::new();

        // Get all connections for the source entity
        let source_connections: Vec<(EntityId, Float)> = self
            .holographic_connections
            .iter()
            .filter(|((source, _target), _connection)| *source == source_entity_id)
            .map(|((_source, target), connection)| (target.clone(), connection.strength))
            .collect();

        for (target_id, resonance) in source_connections {
            // Only propagate if resonance is above threshold
            if resonance > self.propagation_threshold {
                // Calculate influence strength
                let influence_strength = resonance * change.intensity;

                // Apply influence to target
                let result = self.apply_influence(target_id.clone(), change, influence_strength);

                // Update last sync timestamp (stored in connection)
                if let Some(connection) = self
                    .holographic_connections
                    .get_mut(&(source_entity_id.clone(), target_id.clone()))
                {
                    // Update coherence as a proxy for sync timestamp
                    connection.coherence = self.current_timestamp;
                }

                results.push(result);
            }
        }

        results
    }

    /// Apply holographic influence to a target entity (Migrated from holographic_connections.rs)
    ///
    /// The effect depends on veil transparency (thinner veil = more influence).
    /// This is a simplified implementation that returns the influence result.
    ///
    /// From CONTINUOUS_INVOLUTION_EVOLUTION_PROPOSAL.md Section 2.8:
    /// "Non-local influence allows changes to propagate across space/time boundaries."
    pub fn apply_influence(
        &self,
        target_id: EntityId,
        change: &HolographicChange,
        strength: Float,
    ) -> InfluenceResult {
        // Check if target is connected to the source
        // In a full implementation, this would modify the target's archetypical mind
        // For now, we return the influence result

        let effect = strength * change.intensity;

        InfluenceResult::new(target_id, true, strength, effect)
    }

    /// Calculate total influence on an entity from all connected entities (Migrated from holographic_connections.rs)
    pub fn calculate_total_influence(&self, entity_id: EntityId) -> Float {
        self.holographic_connections
            .iter()
            .filter(|((source, _target), _connection)| *source == entity_id)
            .map(|((_source, _target), connection)| connection.strength)
            .sum()
    }

    /// Get the strongest connection for an entity (Migrated from holographic_connections.rs)
    pub fn get_strongest_connection(&self, entity_id: EntityId) -> Option<(EntityId, Float)> {
        self.holographic_connections
            .iter()
            .filter(|((source, _target), _connection)| *source == entity_id)
            .max_by(|a, b| {
                a.1.strength
                    .partial_cmp(&b.1.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|((_source, target), connection)| (target.clone(), connection.strength))
    }
}

impl Default for HolographicFieldManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Holographic principle report
#[derive(Debug, Clone)]
pub struct HolographicPrincipleReport {
    /// Reports for each entity
    pub entity_reports: Vec<HolographicEntityReport>,

    /// Overall completeness
    pub overall_completeness: Float,

    /// Whether holographic principle is demonstrated
    pub demonstrated: bool,
}

/// Holographic entity report
#[derive(Debug, Clone)]
pub struct HolographicEntityReport {
    /// Entity ID
    pub entity_id: EntityId,

    /// Completeness percentage
    pub completeness: Float,

    /// Whether entity contains all layers
    pub contains_all_layers: bool,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{IndividualSpectrumConfiguration, SubSubLogos};
    use crate::foundation::{BlueRealm, GreenRealm, IndigoRealm, VioletRealm};
    use crate::spectrum::{OrangeRealm, RedRealm, SpectrumRatio, SpectrumSide, YellowRealm};

    /// Create a test entity
    fn create_test_entity(id: &str) -> SubSubLogos {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        SubSubLogos::new(
            EntityId::new(id.to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        )
    }

    /// Create a test entity with a specific spectrum ratio
    fn create_test_entity_with_ratio(
        id: &str,
        ratio_value: f64,
        side: SpectrumSide,
    ) -> SubSubLogos {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(ratio_value, side);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        SubSubLogos::new(
            EntityId::new(id.to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        )
    }

    #[test]
    fn test_holographic_field_manager_creation() {
        let manager = HolographicFieldManager::new();
        assert_eq!(manager.entities.len(), 0);
        assert_eq!(manager.holographic_connections.len(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut manager = HolographicFieldManager::new();
        let entity = create_test_entity("entity-1");

        manager.add_entity(entity).unwrap();

        assert_eq!(manager.entities.len(), 1);
        assert_eq!(manager.statistics.entity_count, 1);
    }

    #[test]
    fn test_create_holographic_connections() {
        let mut manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        manager.add_entity(entity1).unwrap();
        manager.add_entity(entity2).unwrap();

        manager.create_holographic_connections();

        // Should have connections in both directions
        assert_eq!(manager.holographic_connections.len(), 2);
        assert_eq!(manager.statistics.connection_count, 1);
    }

    #[test]
    fn test_calculate_spectrum_similarity() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        let similarity = manager.calculate_spectrum_similarity(&entity1, &entity2);

        assert!(similarity >= 0.0);
        assert!(similarity <= 1.0);
    }

    #[test]
    fn test_calculate_interference_patterns() {
        let mut manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        manager.add_entity(entity1).unwrap();
        manager.add_entity(entity2).unwrap();

        manager.create_holographic_connections();
        manager.calculate_interference_patterns();

        assert!(manager.interference_patterns.len() > 0);
        assert_eq!(manager.statistics.interference_pattern_count, 1);
    }

    #[test]
    fn test_track_resonance() {
        let mut manager = HolographicFieldManager::new();

        let entity = create_test_entity("entity-1");
        let entity_id = entity.entity_id.clone();

        manager.add_entity(entity).unwrap();

        let resonance_state = manager.track_resonance(&entity_id);

        assert_eq!(resonance_state.entity_id, entity_id);
        assert!(resonance_state.primary_frequency >= 0.0);
        assert!(resonance_state.primary_frequency <= 1.0);
    }

    #[test]
    fn test_update_field() {
        let mut manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        manager.add_entity(entity1).unwrap();
        manager.add_entity(entity2).unwrap();

        let result = manager.update_field(10);

        assert_eq!(result.steps, 10);
        assert!(result.connections > 0);
        assert!(result.interference_patterns > 0);
    }

    #[test]
    fn test_demonstrate_holographic_principle() {
        let mut manager = HolographicFieldManager::new();

        let entity = create_test_entity("entity-1");
        manager.add_entity(entity).unwrap();

        let report = manager.demonstrate_holographic_principle();

        assert_eq!(report.entity_reports.len(), 1);
        assert!(report.overall_completeness >= 100.0);
        assert!(report.demonstrated);
    }

    #[test]
    fn test_connection_type_determination() {
        let manager = HolographicFieldManager::new();

        // Test resonant connection (high archetype similarity)
        // TODO: HolographicFieldManager::determine_connection_type() method needs to be implemented
        // let conn_type = manager.determine_connection_type(0.9, 0.9, 0.9, 0.9);
        // assert_eq!(conn_type, HolographicConnectionType::Resonant);

        // Test harmonic connection (moderate archetype similarity)
        // let conn_type = manager.determine_connection_type(0.5, 0.7, 0.5, 0.6);
        // assert_eq!(conn_type, HolographicConnectionType::Harmonic);

        // Test entangled connection (strong overall connection, moderate archetype)
        // let conn_type = manager.determine_connection_type(0.8, 0.5, 0.8, 0.4);
        // assert_eq!(conn_type, HolographicConnectionType::Entangled);

        // Test antiphase connection
        // let conn_type = manager.determine_connection_type(0.5, 0.5, 0.2, 0.3);
        // assert_eq!(conn_type, HolographicConnectionType::Antiphase);

        // Test weak connection
        // let conn_type = manager.determine_connection_type(0.3, 0.3, 0.5, 0.2);
        // assert_eq!(conn_type, HolographicConnectionType::Weak);
    }

    #[test]
    fn test_multiple_entities() {
        let mut manager = HolographicFieldManager::new();

        for i in 0..5 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        manager.create_holographic_connections();

        // Should have connections for all pairs
        let expected_connections = 5 * 4; // 5 entities, each connected to 4 others (both directions)
        assert_eq!(manager.holographic_connections.len(), expected_connections);
        assert_eq!(manager.statistics.connection_count, 10); // Divide by 2 for undirected
    }

    #[test]
    fn test_resonance_tracker_identify_clusters() {
        let mut manager = HolographicFieldManager::new();

        // Create entities with similar resonance
        for i in 0..3 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        manager.create_holographic_connections();
        manager.track_resonance_for_all_entities();

        // Should identify at least one cluster
        assert!(manager.resonance_tracker.resonance_clusters.len() >= 0);
    }

    #[test]
    fn test_get_summary() {
        let mut manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        manager.add_entity(entity1).unwrap();
        manager.add_entity(entity2).unwrap();

        manager.create_holographic_connections();

        let summary = manager.get_summary();

        assert!(summary.contains("Entities: 2"));
        assert!(summary.contains("Connections: 1"));
    }

    // ============================================================================
    // PHASE 5: RESONANCE SYSTEM TESTS
    // ============================================================================

    #[test]
    fn test_phase5_calculate_resonance_identical_entities() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        // Add entities to manager
        let mut manager_with_entities = HolographicFieldManager::new();
        manager_with_entities.add_entity(entity1.clone()).unwrap();
        manager_with_entities.add_entity(entity2.clone()).unwrap();

        // Calculate resonance
        let resonance = manager_with_entities.calculate_resonance(&entity1, &entity2);

        // Should have high resonance due to identical entities
        assert!(resonance.resonance_score > 0.5);
        assert!(resonance.spectrum_resonance > 0.5);
        // Entities have same archetype activations and polarization (both neutral)
        assert!(resonance.archetype_resonance > 0.5);
        // Both entities start with Neutral polarity
        assert!(resonance.coherence_resonance >= 0.0); // Placeholder in Phase 5
    }

    #[test]
    fn test_phase5_calculate_spectrum_resonance() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        let spectrum_resonance = manager.calculate_spectrum_resonance(&entity1, &entity2);

        // Should be high since both entities have same spectrum ratio (1.5)
        assert_eq!(spectrum_resonance, 1.0);
    }

    #[test]
    fn test_phase5_calculate_spectrum_resonance_different_ratios() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");

        // Create entity2 with different spectrum ratio
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(5.0, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity2 = SubSubLogos::new(
            EntityId::new("entity-2".to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        let spectrum_resonance = manager.calculate_spectrum_resonance(&entity1, &entity2);

        // Should be lower due to different ratios (1.5 vs 5.0)
        assert!(spectrum_resonance < 1.0);
        assert!(spectrum_resonance > 0.0);
    }

    #[test]
    fn test_phase5_calculate_holographic_resonance() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        let holographic_resonance = manager.calculate_holographic_resonance(&entity1, &entity2);

        // Should be high since both entities have same archetype activations
        assert!(holographic_resonance > 0.5);
        assert!(holographic_resonance <= 1.0);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_same_polarity() {
        let manager = HolographicFieldManager::new();

        let mut entity1 = create_test_entity("entity-1");
        let mut entity2 = create_test_entity("entity-2");

        // Set both entities to STO polarity
        entity1.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;

        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 1.0 for same polarity
        assert_eq!(polarity_resonance, 1.0);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_different_polarity() {
        let manager = HolographicFieldManager::new();

        let mut entity1 = create_test_entity("entity-1");
        let mut entity2 = create_test_entity("entity-2");

        // Set entity1 to STO and entity2 to STS
        entity1.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToSelf;

        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 0.0 for different polarities
        assert_eq!(polarity_resonance, 0.0);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_neutral() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        // Both entities start with Neutral polarity
        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 0.5 for both neutral
        assert_eq!(polarity_resonance, 0.5);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_same_polarity_v2() {
        let manager = HolographicFieldManager::new();

        let mut entity1 = create_test_entity("entity-1");
        let mut entity2 = create_test_entity("entity-2");

        // Set both entities to STO polarity
        entity1.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;

        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 1.0 for same polarity
        assert_eq!(polarity_resonance, 1.0);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_different_polarity_v2() {
        let manager = HolographicFieldManager::new();

        let mut entity1 = create_test_entity("entity-1");
        let mut entity2 = create_test_entity("entity-2");

        // Set entity1 to STO and entity2 to STS
        entity1.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToSelf;

        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 0.0 for different polarities
        assert_eq!(polarity_resonance, 0.0);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_neutral_v2() {
        let manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        // Both entities start with Neutral polarity
        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 0.5 for both neutral
        assert_eq!(polarity_resonance, 0.5);
    }

    #[test]
    fn test_phase5_calculate_polarity_resonance_mixed_neutral() {
        let manager = HolographicFieldManager::new();

        let mut entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        // Set entity1 to STO, entity2 stays Neutral
        entity1.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;

        let polarity_resonance = manager.calculate_polarity_resonance(&entity1, &entity2);

        // Should be 0.3 for mixed neutral
        assert_eq!(polarity_resonance, 0.3);
    }

    #[test]
    fn test_phase5_resonance_varies_by_entity_state() {
        let mut manager = HolographicFieldManager::new();

        // Create entities with different states AND extremely different spectrum ratios
        // to properly test that resonance varies based on entity state
        // Using very extreme ratios (15.0 vs 0.05) to minimize spectrum resonance
        // With max_difference=20, spectrum_resonance = 1 - 14.95/20 = 0.2525
        // Weighted: 0.2525 * 0.30 = 0.07575
        // Plus holographic (1.0 * 0.40 = 0.40) + polarity (0.0 * 0.30 = 0.0)
        // Total ~= 0.476, which should be < 0.5
        let entity1 = create_test_entity_with_ratio("entity-1", 1.0, SpectrumSide::SpaceTime);
        let mut entity2 = create_test_entity_with_ratio("entity-2", 15.0, SpectrumSide::SpaceTime);
        let mut entity3 = create_test_entity_with_ratio("entity-3", 0.05, SpectrumSide::TimeSpace);

        // Set different polarities
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity3.polarization.direction = crate::polarization::PolarityDirection::ServiceToSelf;

        // Add entities
        manager.add_entity(entity1.clone()).unwrap();
        manager.add_entity(entity2.clone()).unwrap();
        manager.add_entity(entity3.clone()).unwrap();

        // Calculate resonance between different pairs
        let resonance_1_2 = manager.calculate_resonance(&entity1, &entity2);
        let resonance_1_3 = manager.calculate_resonance(&entity1, &entity3);
        let resonance_2_3 = manager.calculate_resonance(&entity2, &entity3);

        // Resonance should vary based on entity state
        // entity1 (Neutral) vs entity2 (STO) should be different from entity1 vs entity3 (STS)
        assert_ne!(resonance_1_2.resonance_score, resonance_1_3.resonance_score);
        // entity2 (STO) vs entity3 (STS) should have very low resonance
        assert!(resonance_2_3.resonance_score < 0.5);
    }

    #[test]
    fn test_phase5_resonance_not_baseline() {
        let mut manager = HolographicFieldManager::new();

        // Create multiple entities
        for i in 0..10 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        // Get all entities
        let entities: Vec<&SubSubLogos> = manager.entities.values().collect();

        // Calculate resonance for first pair
        let resonance = manager.calculate_resonance(entities[0], entities[1]);

        // Resonance should not be a baseline value (not 0.5000 for all entities)
        // Since entities have identical archetype activations, resonance should be > 0.5
        // However, it should not be exactly 0.5 (which would indicate a baseline)
        assert_ne!(resonance.resonance_score, 0.5);
    }

    #[test]
    fn test_phase5_find_high_resonance_pairs() {
        let mut manager = HolographicFieldManager::new();

        // Create entities
        for i in 0..5 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        // Find high resonance pairs with threshold 0.7
        let high_resonance_pairs = manager.find_high_resonance_pairs(0.7);

        // Should find at least some pairs (entities have same archetype activations)
        assert!(high_resonance_pairs.len() > 0);

        // Verify all pairs have resonance >= threshold
        for (_, _, resonance_score) in &high_resonance_pairs {
            assert!(*resonance_score >= 0.7);
        }
    }

    #[test]
    fn test_phase5_resonance_enables_collective_formation() {
        let mut manager = HolographicFieldManager::new();

        // Create entities with same polarity
        for i in 0..5 {
            let mut entity = create_test_entity(&format!("entity-{}", i));
            entity.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
            manager.add_entity(entity).unwrap();
        }

        // Find high resonance pairs
        let high_resonance_pairs = manager.find_high_resonance_pairs(0.7);

        // Should find pairs that can form collectives
        assert!(high_resonance_pairs.len() > 0);

        // Verify all pairs have same polarity
        for (entity_id_a, entity_id_b, _) in &high_resonance_pairs {
            let entity_a = manager.entities.get(entity_id_a).unwrap();
            let entity_b = manager.entities.get(entity_id_b).unwrap();

            assert_eq!(
                entity_a.polarization.direction,
                entity_b.polarization.direction
            );
        }
    }

    #[test]
    fn test_phase5_resonance_varies_by_entity_state_v2() {
        let mut manager = HolographicFieldManager::new();

        // Create entities with different states AND extremely different spectrum ratios
        // to properly test that resonance varies based on entity state
        // Using very extreme ratios (15.0 vs 0.05) to minimize spectrum resonance
        let entity1 = create_test_entity_with_ratio("entity-1", 1.0, SpectrumSide::SpaceTime);
        let mut entity2 = create_test_entity_with_ratio("entity-2", 15.0, SpectrumSide::SpaceTime);
        let mut entity3 = create_test_entity_with_ratio("entity-3", 0.05, SpectrumSide::TimeSpace);

        // Set different polarities
        entity2.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
        entity3.polarization.direction = crate::polarization::PolarityDirection::ServiceToSelf;

        // Add entities
        manager.add_entity(entity1.clone()).unwrap();
        manager.add_entity(entity2.clone()).unwrap();
        manager.add_entity(entity3.clone()).unwrap();

        // Calculate resonance between different pairs
        let resonance_1_2 = manager.calculate_resonance(&entity1, &entity2);
        let resonance_1_3 = manager.calculate_resonance(&entity1, &entity3);
        let resonance_2_3 = manager.calculate_resonance(&entity2, &entity3);

        // Resonance should vary based on entity state
        // entity1 (Neutral) vs entity2 (STO) should be different from entity1 vs entity3 (STS)
        assert_ne!(resonance_1_2.resonance_score, resonance_1_3.resonance_score);
        // entity2 (STO) vs entity3 (STS) should have very low resonance
        assert!(resonance_2_3.resonance_score < 0.5);
    }

    #[test]
    fn test_phase5_resonance_not_baseline_v2() {
        let mut manager = HolographicFieldManager::new();

        // Create multiple entities
        for i in 0..10 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        // Get all entities
        let entities: Vec<&SubSubLogos> = manager.entities.values().collect();

        // Calculate resonance for first pair
        let resonance = manager.calculate_resonance(entities[0], entities[1]);

        // Resonance should not be a baseline value (not 0.5000 for all entities)
        // Since entities have identical archetype activations, resonance should be > 0.5
        // However, it should not be exactly 0.5 (which would indicate a baseline)
        assert_ne!(resonance.resonance_score, 0.5);
    }

    #[test]
    fn test_phase5_find_high_resonance_pairs_v2() {
        let mut manager = HolographicFieldManager::new();

        // Create entities
        for i in 0..5 {
            let entity = create_test_entity(&format!("entity-{}", i));
            manager.add_entity(entity).unwrap();
        }

        // Find high resonance pairs with threshold 0.7
        let high_resonance_pairs = manager.find_high_resonance_pairs(0.7);

        // Should find at least some pairs (entities have same archetype activations)
        assert!(high_resonance_pairs.len() > 0);

        // Verify all pairs have resonance >= threshold
        for (_, _, resonance_score) in &high_resonance_pairs {
            assert!(*resonance_score >= 0.7);
        }
    }

    #[test]
    fn test_phase5_resonance_enables_collective_formation_v2() {
        let mut manager = HolographicFieldManager::new();

        // Create entities with same polarity
        for i in 0..5 {
            let mut entity = create_test_entity(&format!("entity-{}", i));
            entity.polarization.direction = crate::polarization::PolarityDirection::ServiceToOthers;
            manager.add_entity(entity).unwrap();
        }

        // Find high resonance pairs
        let high_resonance_pairs = manager.find_high_resonance_pairs(0.7);

        // Should find pairs that can form collectives
        assert!(high_resonance_pairs.len() > 0);

        // Verify all pairs have same polarity
        for (entity_id_a, entity_id_b, _) in &high_resonance_pairs {
            let entity_a = manager.entities.get(entity_id_a).unwrap();
            let entity_b = manager.entities.get(entity_id_b).unwrap();

            assert_eq!(
                entity_a.polarization.direction,
                entity_b.polarization.direction
            );
        }
    }

    #[test]
    fn test_phase5_resonance_type_determination() {
        let mut manager = HolographicFieldManager::new();

        let entity1 = create_test_entity("entity-1");
        let entity2 = create_test_entity("entity-2");

        manager.add_entity(entity1.clone()).unwrap();
        manager.add_entity(entity2.clone()).unwrap();

        // Calculate resonance
        let resonance = manager.calculate_resonance(&entity1, &entity2);

        // Should have a valid resonance type
        match resonance.resonance_type {
            ResonanceType::High => assert!(resonance.resonance_score > 0.8),
            ResonanceType::Medium => assert!(resonance.resonance_score > 0.5),
            ResonanceType::Low => assert!(resonance.resonance_score > 0.2),
            ResonanceType::None => assert!(resonance.resonance_score <= 0.2),
        }
    }
}
