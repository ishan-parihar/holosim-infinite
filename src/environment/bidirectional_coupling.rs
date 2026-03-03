//! Bidirectional Environmental Coupling
//!
//! From ROADMAP Phase 6.3: "Environmental Coupling: One-way → Bidirectional"
//! "Entities affect environment, environment affects entities"
//!
//! This module implements a two-way relationship between entities and environment:
//! - Environment affects entity state (catalyst, weather, terrain)
//! - Entities affect environment (actions, collective behavior, consciousness)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐     Entity Influence     ┌─────────────────┐
//! │    Entities     │ ───────────────────────► │   Environment   │
//! │ (SubSubLogos)   │                          │  (Gaia, Water)  │
//! └─────────────────┘ ◄─────────────────────── └─────────────────┘
//!                     Environmental Response
//! ```
//!
//! # Key Principles
//!
//! 1. **Bidirectional Flow**: Both entity→environment and environment→entity
//! 2. **Influence Accumulation**: Entity actions aggregate into environmental changes
//! 3. **Response Generation**: Environment responds to collective entity behavior
//! 4. **Gaia Integration**: Coupling affects planetary consciousness
//! 5. **Action Types**: Different entity actions have different environmental impacts

use crate::entity_layer7::layer7::EntityId;
use crate::environment::field_hydrology::FieldHydrology;
use crate::environment::unified_gaia::UnifiedGaia;
use crate::types::Float;

/// Bidirectional environmental coupling manager
///
/// This creates a two-way relationship between entities and environment:
/// - Environment affects entity state (catalyst, weather, terrain)
/// - Entities affect environment (actions, collective behavior, consciousness)
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Entities are not separate from their environment"
/// "The environment affects entity consciousness through field coherence"
pub struct BidirectionalCoupling {
    /// Gaia planetary consciousness
    pub gaia: UnifiedGaia,

    /// Field-derived hydrology
    pub hydrology: Option<FieldHydrology>,

    /// Entity influence accumulator
    pub entity_influences: Vec<EntityInfluence>,

    /// Environmental response accumulator
    pub environmental_responses: Vec<EnvironmentalResponse>,

    /// Coupling strength (0.0 to 1.0)
    pub coupling_strength: Float,

    /// Configuration
    pub config: CouplingConfig,
}

/// Configuration for bidirectional coupling
#[derive(Debug, Clone)]
pub struct CouplingConfig {
    /// How strongly entities affect environment
    pub entity_to_environment_factor: Float,
    /// How strongly environment affects entities
    pub environment_to_entity_factor: Float,
    /// Minimum collective influence to affect environment
    pub influence_threshold: Float,
    /// Enable entity modifications to terrain
    pub enable_terrain_modification: bool,
    /// Enable entity modifications to atmosphere
    pub enable_atmospheric_modification: bool,
    /// Enable entity modifications to hydrosphere
    pub enable_hydrosphere_modification: bool,
}

impl Default for CouplingConfig {
    fn default() -> Self {
        Self {
            entity_to_environment_factor: 0.1,
            environment_to_entity_factor: 0.3,
            influence_threshold: 0.1,
            enable_terrain_modification: true,
            enable_atmospheric_modification: true,
            enable_hydrosphere_modification: true,
        }
    }
}

/// Influence an entity has on the environment
#[derive(Debug, Clone)]
pub struct EntityInfluence {
    /// Source entity
    pub entity_id: EntityId,
    /// Type of influence
    pub influence_type: InfluenceType,
    /// Magnitude of influence (can be negative)
    pub magnitude: Float,
    /// Location of influence
    pub location: Location3D,
    /// Archetype signature of the influence
    pub archetype_signature: [Float; 22],
}

/// Types of entity influences on environment
#[derive(Debug, Clone, PartialEq)]
pub enum InfluenceType {
    /// Physical modification (building, erosion)
    TerrainModification,
    /// Atmospheric change (respiration, industry)
    AtmosphericChange,
    /// Water usage or modification
    HydrosphereChange,
    /// Consciousness field effect
    ConsciousnessFieldEffect,
    /// Biological activity
    BiologicalActivity,
    /// Catalyst creation
    CatalystCreation,
}

/// Response from environment to entity
#[derive(Debug, Clone)]
pub struct EnvironmentalResponse {
    /// Target entities (empty = broadcast to all)
    pub target_entities: Vec<EntityId>,
    /// Type of response
    pub response_type: ResponseType,
    /// Magnitude of response
    pub magnitude: Float,
    /// Human-readable description
    pub description: String,
}

/// Types of environmental responses
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseType {
    /// Weather pattern change
    WeatherChange,
    /// Terrain shift
    TerrainShift,
    /// Resource availability change
    ResourceAvailability,
    /// Catalyst provision from environment
    CatalystProvision,
    /// Healing energy from environment
    Healing,
    /// Environmental challenge
    Challenge,
}

/// 3D location in environment
#[derive(Debug, Clone, Default)]
pub struct Location3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Location3D {
    /// Create a new location
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
}

/// Result of a coupling cycle
#[derive(Debug, Clone)]
pub struct CouplingCycleResult {
    /// Total influence accumulated this cycle
    pub total_influence: Float,
    /// Number of modifications applied to environment
    pub modifications_applied: usize,
    /// Number of responses generated
    pub responses_generated: usize,
}

/// Environmental effects on an entity
#[derive(Debug, Clone)]
pub struct EnvironmentEffects {
    /// Gaia's collective consciousness level
    pub gaia_consciousness: Float,
    /// Planetary coherence level
    pub planetary_coherence: Float,
    /// Biosphere health (0.0 to 1.0)
    pub biosphere_health: Float,
    /// Local influence factor at entity's location
    pub local_influence_factor: Float,
}

/// Entity actions that affect environment
#[derive(Debug, Clone, PartialEq)]
pub enum EntityAction {
    /// Building or construction
    Build,
    /// Respiration (breathing)
    Respire,
    /// Consuming resources
    Consume,
    /// Meditative or spiritual activity
    Meditate,
    /// Creative or generative activity
    Create,
}

impl BidirectionalCoupling {
    /// Create new bidirectional coupling system
    ///
    /// Initializes with a UnifiedGaia planetary consciousness and default configuration.
    pub fn new(gaia: UnifiedGaia) -> Self {
        Self {
            gaia,
            hydrology: None,
            entity_influences: Vec::new(),
            environmental_responses: Vec::new(),
            coupling_strength: 0.5,
            config: CouplingConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(gaia: UnifiedGaia, config: CouplingConfig) -> Self {
        Self {
            gaia,
            hydrology: None,
            entity_influences: Vec::new(),
            environmental_responses: Vec::new(),
            coupling_strength: 0.5,
            config,
        }
    }

    /// Set hydrology system
    pub fn with_hydrology(mut self, hydrology: FieldHydrology) -> Self {
        self.hydrology = Some(hydrology);
        self
    }

    /// Register entity influence on environment
    ///
    /// Entity influences are accumulated and processed during coupling cycles.
    pub fn register_influence(&mut self, influence: EntityInfluence) {
        self.entity_influences.push(influence);
    }

    /// Process all influences and generate responses
    ///
    /// This is the main coupling cycle that:
    /// 1. Aggregates all entity influences
    /// 2. Applies influences to environment systems
    /// 3. Generates environmental responses
    /// 4. Updates Gaia based on coupling
    ///
    /// From ROADMAP: "Bidirectional entity-environment interaction"
    pub fn process_coupling_cycle(&mut self) -> CouplingCycleResult {
        // 1. Aggregate entity influences
        let total_influence = self.aggregate_influences();

        // 2. Apply influences to environment
        let modifications = self.apply_influences_to_environment();

        // 3. Generate environmental responses
        let responses = self.generate_responses(total_influence);

        // 4. Apply responses to entities (stored for retrieval)
        self.environmental_responses.extend(responses.clone());

        // 5. Update Gaia based on coupling
        self.update_gaia_from_coupling(total_influence);

        // 6. Update hydrology if present
        if let Some(ref mut hydrology) = self.hydrology {
            let _ = hydrology.evolve(0.1);
        }

        CouplingCycleResult {
            total_influence,
            modifications_applied: modifications,
            responses_generated: responses.len(),
        }
    }

    /// Aggregate all entity influences
    ///
    /// Sums all influence magnitudes and clears the influence buffer.
    fn aggregate_influences(&mut self) -> Float {
        let total: Float = self.entity_influences.iter().map(|i| i.magnitude).sum();

        // Clear processed influences
        self.entity_influences.clear();

        total
    }

    /// Apply influences to environment systems
    ///
    /// This modifies the actual environment based on entity actions.
    fn apply_influences_to_environment(&mut self) -> usize {
        // In a full implementation, this would modify actual terrain/atmosphere/hydrosphere
        // For now, we track the modifications via Gaia updates

        let mut count = 0;

        // Modifications would be applied to:
        // - Gaia's planetary data (biosphere health, etc.)
        // - Hydrosphere water bodies
        // - Atmospheric composition

        // Apply to Gaia's biosphere based on influence types
        // (This is tracked in update_gaia_from_coupling)

        count
    }

    /// Generate environmental responses based on influence
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Environment responds to collective entity behavior"
    fn generate_responses(&self, total_influence: Float) -> Vec<EnvironmentalResponse> {
        let mut responses = Vec::new();

        // If total influence exceeds threshold, environment responds
        if total_influence.abs() > self.config.influence_threshold {
            // Positive influence → healing/provision
            if total_influence > 0.0 {
                responses.push(EnvironmentalResponse {
                    target_entities: vec![], // Broadcast
                    response_type: ResponseType::Healing,
                    magnitude: total_influence * self.config.environment_to_entity_factor,
                    description: "Environment provides healing energy".to_string(),
                });

                // High positive influence may provide catalyst
                if total_influence > 0.5 {
                    responses.push(EnvironmentalResponse {
                        target_entities: vec![], // Broadcast
                        response_type: ResponseType::CatalystProvision,
                        magnitude: total_influence * 0.5,
                        description: "Environment offers evolution catalyst".to_string(),
                    });
                }
            }
            // Negative influence → challenge
            else {
                responses.push(EnvironmentalResponse {
                    target_entities: vec![], // Broadcast
                    response_type: ResponseType::Challenge,
                    magnitude: total_influence.abs() * self.config.environment_to_entity_factor,
                    description: "Environment presents challenge".to_string(),
                });
            }
        }

        // Check Gaia state and generate additional responses
        if self.gaia.collective_consciousness > 0.7 {
            responses.push(EnvironmentalResponse {
                target_entities: vec![],
                response_type: ResponseType::ResourceAvailability,
                magnitude: 0.3,
                description: "Abundant resources available due to collective harmony".to_string(),
            });
        }

        if self.gaia.template.component_data.biosphere_health < 0.3 {
            responses.push(EnvironmentalResponse {
                target_entities: vec![],
                response_type: ResponseType::WeatherChange,
                magnitude: 0.5,
                description: "Environmental instability due to low biosphere health".to_string(),
            });
        }

        responses
    }

    /// Update Gaia based on coupling results
    ///
    /// Entity activity affects Gaia's consciousness and planetary state.
    fn update_gaia_from_coupling(&mut self, total_influence: Float) {
        // Update collective consciousness based on entity activity
        let consciousness_delta = total_influence * 0.01 * self.config.entity_to_environment_factor;
        self.gaia.collective_consciousness =
            (self.gaia.collective_consciousness + consciousness_delta).clamp(0.0, 1.0);

        // Update biosphere health based on influence type
        if total_influence > 0.0 {
            // Positive influence improves biosphere
            self.gaia.template.component_data.biosphere_health =
                (self.gaia.template.component_data.biosphere_health + total_influence * 0.001)
                    .min(1.0);
        } else if total_influence < 0.0 {
            // Negative influence degrades biosphere
            self.gaia.template.component_data.biosphere_health =
                (self.gaia.template.component_data.biosphere_health + total_influence * 0.001)
                    .max(0.0);
        }

        // Evolve Gaia
        let _ = self.gaia.evolve(1.0);
    }

    /// Get pending responses for an entity
    ///
    /// Entities call this to retrieve environmental responses.
    /// Responses where target_entities is empty are broadcast to all.
    pub fn get_responses_for_entity(&mut self, entity_id: &EntityId) -> Vec<EnvironmentalResponse> {
        let (for_entity, remaining): (Vec<_>, Vec<_>) = self
            .environmental_responses
            .drain(..)
            .partition(|r| r.target_entities.is_empty() || r.target_entities.contains(entity_id));

        self.environmental_responses = remaining;
        for_entity
    }

    /// Get environment-to-entity effects (for consciousness tick)
    ///
    /// Returns environmental factors that affect entity consciousness.
    pub fn get_environment_effects(
        &self,
        _entity_id: &EntityId,
        location: &Location3D,
    ) -> EnvironmentEffects {
        EnvironmentEffects {
            gaia_consciousness: self.gaia.collective_consciousness,
            planetary_coherence: self.gaia.planetary_awareness.planetary_coherence,
            biosphere_health: self.gaia.template.component_data.biosphere_health,
            local_influence_factor: self.calculate_local_influence_factor(location),
        }
    }

    /// Calculate local influence factor based on location
    ///
    /// In a full implementation, this would consider:
    /// - Terrain type
    /// - Water proximity
    /// - Atmospheric conditions
    /// - Entity density
    fn calculate_local_influence_factor(&self, _location: &Location3D) -> Float {
        // For now, use coupling strength as base factor
        // Enhanced by Gaia's coherence
        self.coupling_strength * (0.5 + 0.5 * self.gaia.planetary_awareness.planetary_coherence)
    }

    /// Register entity action that affects environment
    ///
    /// Translates high-level entity actions into environmental influences.
    pub fn register_entity_action(
        &mut self,
        entity_id: EntityId,
        action: EntityAction,
        location: Location3D,
    ) {
        let influence = match action {
            EntityAction::Build => EntityInfluence {
                entity_id,
                influence_type: InfluenceType::TerrainModification,
                magnitude: 0.1,
                location,
                archetype_signature: [0.5; 22],
            },
            EntityAction::Respire => EntityInfluence {
                entity_id,
                influence_type: InfluenceType::AtmosphericChange,
                magnitude: 0.01,
                location,
                archetype_signature: [0.5; 22],
            },
            EntityAction::Consume => EntityInfluence {
                entity_id,
                influence_type: InfluenceType::BiologicalActivity,
                magnitude: 0.02,
                location,
                archetype_signature: [0.5; 22],
            },
            EntityAction::Meditate => EntityInfluence {
                entity_id,
                influence_type: InfluenceType::ConsciousnessFieldEffect,
                magnitude: 0.1,
                location,
                archetype_signature: [0.8; 22], // High spiritual activation
            },
            EntityAction::Create => EntityInfluence {
                entity_id,
                influence_type: InfluenceType::CatalystCreation,
                magnitude: 0.15,
                location,
                archetype_signature: [0.7; 22],
            },
        };

        self.register_influence(influence);
    }

    /// Apply environment effects to an entity's mutable state
    ///
    /// This is the environment→entity direction of the bidirectional coupling.
    pub fn apply_environment_to_entity(
        &self,
        entity_id: &EntityId,
        location: &Location3D,
        state: &mut EntityMutableEnvironmentState,
    ) {
        let effects = self.get_environment_effects(entity_id, location);

        // Apply Gaia consciousness influence
        state.gaia_influence =
            effects.gaia_consciousness * self.config.environment_to_entity_factor;

        // Apply planetary coherence
        state.coherence_factor = effects.planetary_coherence;

        // Apply biosphere health effects
        if effects.biosphere_health < 0.5 {
            // Low biosphere health causes stress
            state.stress_level =
                (state.stress_level + (0.5 - effects.biosphere_health) * 0.1).min(1.0);
        }

        // Apply local influence
        state.local_environment_factor = effects.local_influence_factor;
    }

    /// Get coupling statistics
    pub fn get_statistics(&self) -> CouplingStatistics {
        CouplingStatistics {
            total_influences_processed: self.entity_influences.len(),
            pending_responses: self.environmental_responses.len(),
            gaia_consciousness: self.gaia.collective_consciousness,
            planetary_coherence: self.gaia.planetary_awareness.planetary_coherence,
            biosphere_health: self.gaia.template.component_data.biosphere_health,
            entity_count: self.gaia.entity_count(),
        }
    }
}

/// Entity mutable environment state
///
/// Tracks how environment affects an entity.
#[derive(Debug, Clone, Default)]
pub struct EntityMutableEnvironmentState {
    /// Influence from Gaia consciousness
    pub gaia_influence: Float,
    /// Planetary coherence factor
    pub coherence_factor: Float,
    /// Stress level from environment
    pub stress_level: Float,
    /// Local environment influence factor
    pub local_environment_factor: Float,
}

/// Coupling statistics
#[derive(Debug, Clone)]
pub struct CouplingStatistics {
    /// Total influences processed (in current buffer)
    pub total_influences_processed: usize,
    /// Pending environmental responses
    pub pending_responses: usize,
    /// Gaia's consciousness level
    pub gaia_consciousness: Float,
    /// Planetary coherence
    pub planetary_coherence: Float,
    /// Biosphere health
    pub biosphere_health: Float,
    /// Number of entities in biosphere
    pub entity_count: usize,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create EntityId
    fn entity_id(n: u64) -> EntityId {
        EntityId::new(format!("entity-{}", n))
    }

    #[test]
    fn test_bidirectional_coupling_creation() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let coupling = BidirectionalCoupling::new(gaia);

        assert!(coupling.coupling_strength > 0.0);
        assert!(coupling.entity_influences.is_empty());
        assert!(coupling.environmental_responses.is_empty());
    }

    #[test]
    fn test_influence_registration() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::ConsciousnessFieldEffect,
            magnitude: 0.5,
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        assert_eq!(coupling.entity_influences.len(), 1);
    }

    #[test]
    fn test_coupling_cycle() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        // Add influence
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::ConsciousnessFieldEffect,
            magnitude: 0.5,
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        let result = coupling.process_coupling_cycle();

        assert!(result.total_influence > 0.0);
        // Influences should be cleared after processing
        assert!(coupling.entity_influences.is_empty());
    }

    #[test]
    fn test_entity_action_registration() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        coupling.register_entity_action(
            entity_id(1),
            EntityAction::Meditate,
            Location3D::default(),
        );

        assert_eq!(coupling.entity_influences.len(), 1);
        assert_eq!(
            coupling.entity_influences[0].influence_type,
            InfluenceType::ConsciousnessFieldEffect
        );
    }

    #[test]
    fn test_entity_action_build() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        coupling.register_entity_action(entity_id(1), EntityAction::Build, Location3D::default());

        assert_eq!(coupling.entity_influences.len(), 1);
        assert_eq!(
            coupling.entity_influences[0].influence_type,
            InfluenceType::TerrainModification
        );
    }

    #[test]
    fn test_entity_action_create() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        coupling.register_entity_action(entity_id(1), EntityAction::Create, Location3D::default());

        assert_eq!(coupling.entity_influences.len(), 1);
        assert_eq!(
            coupling.entity_influences[0].influence_type,
            InfluenceType::CatalystCreation
        );
        assert!(coupling.entity_influences[0].magnitude > 0.1);
    }

    #[test]
    fn test_environment_effects() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let coupling = BidirectionalCoupling::new(gaia);

        let effects = coupling.get_environment_effects(&entity_id(1), &Location3D::default());

        assert!(effects.gaia_consciousness >= 0.0);
        assert!(effects.planetary_coherence >= 0.0);
        assert!(effects.biosphere_health >= 0.0);
        assert!(effects.local_influence_factor >= 0.0);
    }

    #[test]
    fn test_positive_influence_generates_healing_response() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        // Add large positive influence
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::ConsciousnessFieldEffect,
            magnitude: 1.0,
            location: Location3D::default(),
            archetype_signature: [0.8; 22],
        });

        coupling.process_coupling_cycle();

        // Should have generated responses
        assert!(!coupling.environmental_responses.is_empty());

        // Should include healing response
        assert!(coupling
            .environmental_responses
            .iter()
            .any(|r| r.response_type == ResponseType::Healing));
    }

    #[test]
    fn test_negative_influence_generates_challenge_response() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        // Add large negative influence
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::TerrainModification,
            magnitude: -1.0,
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        coupling.process_coupling_cycle();

        // Should have generated challenge response
        assert!(coupling
            .environmental_responses
            .iter()
            .any(|r| r.response_type == ResponseType::Challenge));
    }

    #[test]
    fn test_get_responses_for_entity() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        // Add influence to generate response
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::ConsciousnessFieldEffect,
            magnitude: 0.5,
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        coupling.process_coupling_cycle();

        // Get responses for entity
        let responses = coupling.get_responses_for_entity(&entity_id(1));

        // Should have received broadcast responses
        assert!(!responses.is_empty());
    }

    #[test]
    fn test_apply_environment_to_entity() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let coupling = BidirectionalCoupling::new(gaia);

        let mut state = EntityMutableEnvironmentState::default();
        coupling.apply_environment_to_entity(&entity_id(1), &Location3D::default(), &mut state);

        assert!(state.gaia_influence >= 0.0);
        assert!(state.coherence_factor >= 0.0);
    }

    #[test]
    fn test_low_biosphere_causes_stress() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        gaia.template.component_data.biosphere_health = 0.2; // Low health

        let coupling = BidirectionalCoupling::new(gaia);

        let mut state = EntityMutableEnvironmentState::default();
        coupling.apply_environment_to_entity(&entity_id(1), &Location3D::default(), &mut state);

        // Should have increased stress
        assert!(state.stress_level > 0.0);
    }

    #[test]
    fn test_coupling_statistics() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        gaia.register_entity(entity_id(1));

        let coupling = BidirectionalCoupling::new(gaia);
        let stats = coupling.get_statistics();

        assert_eq!(stats.entity_count, 1);
        assert!(stats.gaia_consciousness >= 0.0);
    }

    #[test]
    fn test_coupling_config_default() {
        let config = CouplingConfig::default();

        assert!(config.entity_to_environment_factor > 0.0);
        assert!(config.environment_to_entity_factor > 0.0);
        assert!(config.influence_threshold > 0.0);
        assert!(config.enable_terrain_modification);
        assert!(config.enable_atmospheric_modification);
        assert!(config.enable_hydrosphere_modification);
    }

    #[test]
    fn test_location_3d() {
        let loc = Location3D::new(1.0, 2.0, 3.0);

        assert!((loc.x - 1.0).abs() < 0.001);
        assert!((loc.y - 2.0).abs() < 0.001);
        assert!((loc.z - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_gaia_consciousness_increases_with_positive_influence() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);
        let initial_consciousness = coupling.gaia.collective_consciousness;

        // Add positive influence
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::ConsciousnessFieldEffect,
            magnitude: 1.0,
            location: Location3D::default(),
            archetype_signature: [0.8; 22],
        });

        coupling.process_coupling_cycle();

        // Gaia consciousness should have increased
        assert!(
            coupling.gaia.collective_consciousness >= initial_consciousness,
            "Gaia consciousness should increase with positive influence"
        );
    }

    #[test]
    fn test_biosphere_health_affected_by_influence() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);
        let initial_health = coupling.gaia.template.component_data.biosphere_health;

        // Add negative influence
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::TerrainModification,
            magnitude: -10.0, // Large negative influence
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        coupling.process_coupling_cycle();

        // Biosphere health should have decreased
        assert!(
            coupling.gaia.template.component_data.biosphere_health <= initial_health,
            "Biosphere health should decrease with negative influence"
        );
    }

    #[test]
    fn test_with_config() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let config = CouplingConfig {
            entity_to_environment_factor: 0.5,
            environment_to_entity_factor: 0.7,
            influence_threshold: 0.05,
            enable_terrain_modification: false,
            enable_atmospheric_modification: true,
            enable_hydrosphere_modification: false,
        };

        let coupling = BidirectionalCoupling::with_config(gaia, config);

        assert!((coupling.config.entity_to_environment_factor - 0.5).abs() < 0.001);
        assert!(!coupling.config.enable_terrain_modification);
    }

    #[test]
    fn test_small_influence_below_threshold_no_response() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let mut coupling = BidirectionalCoupling::new(gaia);

        // Add very small influence (below default threshold of 0.1)
        coupling.register_influence(EntityInfluence {
            entity_id: entity_id(1),
            influence_type: InfluenceType::BiologicalActivity,
            magnitude: 0.01,
            location: Location3D::default(),
            archetype_signature: [0.5; 22],
        });

        coupling.process_coupling_cycle();

        // Should have no responses (influence below threshold)
        // Note: May still have responses from Gaia state checks
        let influence_responses: Vec<_> = coupling
            .environmental_responses
            .iter()
            .filter(|r| {
                r.response_type == ResponseType::Healing
                    || r.response_type == ResponseType::Challenge
            })
            .collect();
        assert!(
            influence_responses.is_empty(),
            "Small influence should not generate healing/challenge response"
        );
    }
}
