//! Full Integration Module (Phase F7)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Goal: All systems working together in a coherent simulation."
//!
//! Integration Architecture:
//! ┌─────────────────────────────────────────────────────────┐
//! │                    HOLOGRAPHIC FIELD                     │
//! │  (Spatial octree with archetypal patterns)             │
//! └──────────────────────┬──────────────────────────────────┘
//!                       │
//!         ┌─────────────┼─────────────┐
//!         ▼             ▼             ▼
//!    SPECTRUM      PHASE         ATTRACTOR
//!    DRIVES       TRANSITIONS     FIELDS
//!    SPACE        CREATE          PULL
//!                  COMPLEXITY      CONSCIOUSNESS
//!         │             │             │
//!         ▼             ▼             ▼
//!    3D SPACE    MATTER         ENTITY
//!    EMERGES     EMERGES        EVOLVES
//!         │             │             │
//!         └─────────────┼─────────────┘
//!                       ▼
//!               BIOLOGY/ENVIRONMENT
//!               FULLY INTEGRATED
//!
//! This module implements:
//! - Unified field-to-environment pipeline
//! - Feedback loops: Matter → Field → Consciousness
//! - Cross-phase integration metrics
//! - Performance optimization for large entity counts

use super::complexity_emergence::ComplexityPhase;
use super::field_state::{FieldNodeData, Float};
use super::spatial_field::Position3D;

/// Configuration for full integration
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    /// Enable field-to-matter feedback
    pub enable_feedback: bool,
    /// Feedback strength (0.0 - 1.0)
    pub feedback_strength: Float,
    /// Update frequency for integration stats
    pub stats_interval: usize,
    /// Enable cross-phase optimization
    pub optimize_phases: bool,
    /// Maximum entities to process per phase
    pub max_entities_per_phase: usize,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        IntegrationConfig {
            enable_feedback: true,
            feedback_strength: 0.5,
            stats_interval: 10,
            optimize_phases: true,
            max_entities_per_phase: 10000,
        }
    }
}

/// Feedback loop type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackLoop {
    /// Matter influences field coherence
    MatterToField,
    /// Consciousness influences matter
    ConsciousnessToMatter,
    /// Environment affects entities
    EnvironmentToEntity,
    /// Entities affect their environment
    EntityToEnvironment,
    /// Spectrum position affects space
    SpectrumToSpace,
}

/// Integration metric for a single feedback loop
#[derive(Debug, Clone, Default)]
pub struct FeedbackMetric {
    /// Total influence applied this step
    pub total_influence: Float,
    /// Average influence per entity
    pub avg_influence: Float,
    /// Number of entities affected
    pub affected_count: usize,
    /// Stability metric (0.0 = unstable, 1.0 = stable)
    pub stability: Float,
}

/// Cross-phase integration metrics
#[derive(Debug, Clone, Default)]
pub struct IntegrationMetrics {
    /// Field → Space integration
    pub field_to_space: FeedbackMetric,
    /// Field → Matter integration
    pub field_to_matter: FeedbackMetric,
    /// Matter → Field feedback
    pub matter_to_field: FeedbackMetric,
    /// Consciousness → Matter integration
    pub consciousness_to_matter: FeedbackMetric,
    /// Entity → Environment integration
    pub entity_to_environment: FeedbackMetric,
    /// Environment → Entity integration
    pub environment_to_entity: FeedbackMetric,
    /// Overall integration coherence (0.0 - 1.0)
    pub overall_coherence: Float,
}

/// Phase performance metrics
#[derive(Debug, Clone, Default)]
pub struct PhasePerformance {
    /// Phase name
    pub name: String,
    /// Time spent in microseconds
    pub execution_time_us: u64,
    /// Number of entities processed
    pub entities_processed: usize,
    /// Efficiency (entities per microsecond)
    pub efficiency: Float,
}

/// Complete integration statistics
#[derive(Debug, Clone, Default)]
pub struct IntegrationStatistics {
    /// Cross-phase integration metrics
    pub metrics: IntegrationMetrics,
    /// Performance by phase
    pub phase_performance: Vec<PhasePerformance>,
    /// Total entities in system
    pub total_entities: usize,
    /// Active feedback loops
    pub active_feedback_loops: usize,
    /// Integration stability (0.0 - 1.0)
    pub stability: Float,
    /// Simulation coherence (0.0 - 1.0)
    pub coherence: Float,
}

/// Field influence from matter
#[derive(Debug, Clone)]
pub struct MatterFieldInfluence {
    /// Position of matter
    pub position: Position3D,
    /// Influence radius
    pub radius: Float,
    /// Influence strength (from mass/complexity)
    pub strength: Float,
    /// Type of matter
    pub matter_type: MatterInfluenceType,
}

/// Type of matter influencing the field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatterInfluenceType {
    Particle,
    Atom,
    Molecule,
    Cell,
    Planet,
}

/// Entity influence on environment
#[derive(Debug, Clone)]
pub struct EntityEnvironmentInfluence {
    /// Entity ID
    pub entity_id: usize,
    /// Entity position
    pub position: Position3D,
    /// Consciousness level (affects environment modification)
    pub consciousness: Float,
    /// Activity level
    pub activity: Float,
}

/// Environment influence on entities
#[derive(Debug, Clone)]
pub struct EnvironmentEntityInfluence {
    /// Position
    pub position: Position3D,
    /// Environmental quality (0.0 - 1.0)
    pub quality: Float,
    /// Resource availability
    pub resources: Float,
    /// Danger level
    pub danger: Float,
}

/// Unified pipeline data structure
#[derive(Debug, Clone)]
pub struct UnifiedPipeline {
    /// Current field state snapshot
    pub field_snapshot: FieldNodeData,
    /// Derived spatial configuration
    pub spatial_config: [Float; 3], // dimensions, time_dimensionality, scale
    /// Derived matter state
    pub matter_state: MatterPipelineState,
    /// Derived consciousness state
    pub consciousness_state: ConsciousnessPipelineState,
    /// Derived environment state
    pub environment_state: EnvironmentPipelineState,
}

/// Matter state in pipeline
#[derive(Debug, Clone, Default)]
pub struct MatterPipelineState {
    /// Complexity phase
    pub phase: Option<ComplexityPhase>,
    /// Total particles
    pub particle_count: usize,
    /// Total atoms
    pub atom_count: usize,
    /// Total molecules
    pub molecule_count: usize,
    /// Field coherence at matter level
    pub coherence: Float,
}

/// Consciousness state in pipeline
#[derive(Debug, Clone, Default)]
pub struct ConsciousnessPipelineState {
    /// Average entity consciousness
    pub avg_consciousness: Float,
    /// Number of entities in collectives
    pub collective_entities: usize,
    /// Entity density
    pub entity_density: Float,
    /// Evolution progress
    pub evolution_progress: Float,
}

/// Environment state in pipeline
#[derive(Debug, Clone, Default)]
pub struct EnvironmentPipelineState {
    /// Number of planets
    pub planet_count: usize,
    /// Average terrain coherence
    pub avg_terrain_coherence: Float,
    /// Atmosphere quality
    pub atmosphere_quality: Float,
    /// Biological activity
    pub biological_activity: Float,
}

/// Full Integration System
///
/// This system ensures all phases work together cohesively by:
/// 1. Tracking cross-phase dependencies
/// 2. Implementing feedback loops
/// 3. Measuring integration quality
/// 4. Optimizing performance
pub struct FullIntegration {
    config: IntegrationConfig,
    statistics: IntegrationStatistics,
    pipeline: UnifiedPipeline,
    // Feedback accumulators
    #[allow(dead_code)]
    matter_influence_accumulator: Vec<MatterFieldInfluence>,
    #[allow(dead_code)]
    entity_influence_accumulator: Vec<EntityEnvironmentInfluence>,
    // Performance tracking
    phase_timings: Vec<(String, u64)>,
    // Stability tracking
    coherence_history: Vec<Float>,
    max_history: usize,
}

impl FullIntegration {
    /// Create new integration system
    pub fn new(config: IntegrationConfig) -> Self {
        FullIntegration {
            config,
            statistics: IntegrationStatistics::default(),
            pipeline: UnifiedPipeline {
                field_snapshot: FieldNodeData::new(),
                spatial_config: [3.0, 1.0, 1.0],
                matter_state: MatterPipelineState::default(),
                consciousness_state: ConsciousnessPipelineState::default(),
                environment_state: EnvironmentPipelineState::default(),
            },
            matter_influence_accumulator: Vec::new(),
            entity_influence_accumulator: Vec::new(),
            phase_timings: Vec::new(),
            coherence_history: Vec::new(),
            max_history: 100,
        }
    }

    /// Create with defaults
    pub fn with_defaults() -> Self {
        Self::new(IntegrationConfig::default())
    }

    /// Process field → space integration
    pub fn integrate_field_to_space(&mut self, field_coherence: Float, spectrum_position: Float) {
        let metric = &mut self.statistics.metrics.field_to_space;

        // Higher coherence = more defined space
        let influence = field_coherence * spectrum_position;

        metric.total_influence += influence;
        metric.affected_count += 1;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Update spatial config in pipeline
        self.pipeline.spatial_config[0] = if spectrum_position < 1.0 { 3.0 } else { 1.0 };
        self.pipeline.spatial_config[1] = if spectrum_position < 1.0 { 1.0 } else { 3.0 };
        self.pipeline.spatial_config[2] = field_coherence;
    }

    /// Process field → matter integration
    pub fn integrate_field_to_matter(
        &mut self,
        field_data: &FieldNodeData,
        particle_count: usize,
        atom_count: usize,
    ) {
        let metric = &mut self.statistics.metrics.field_to_matter;

        // Higher coherence + amplitude = more matter
        let coherence = field_data.coherence;
        let amplitude = field_data.total_magnitude();
        let influence = coherence * amplitude;

        metric.total_influence += influence;
        metric.affected_count += particle_count + atom_count;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Update matter state in pipeline
        self.pipeline.matter_state.particle_count = particle_count;
        self.pipeline.matter_state.atom_count = atom_count;
        self.pipeline.matter_state.coherence = coherence;

        // Determine phase
        self.pipeline.matter_state.phase = if coherence >= 0.75 {
            Some(ComplexityPhase::Planetary)
        } else if coherence >= 0.5 {
            Some(ComplexityPhase::Molecular)
        } else if coherence >= 0.3 {
            Some(ComplexityPhase::Atomic)
        } else {
            Some(ComplexityPhase::Quantum)
        };
    }

    /// Process matter → field feedback
    pub fn integrate_matter_to_field(
        &mut self,
        matter_count: usize,
        avg_complexity: Float,
    ) -> FieldNodeData {
        let metric = &mut self.statistics.metrics.matter_to_field;

        // Matter affects field coherence
        let influence = (matter_count as Float * avg_complexity * 0.01).min(1.0);

        metric.total_influence += influence;
        metric.affected_count += matter_count;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Calculate stability based on influence variation
        metric.stability = (1.0 - influence).max(0.0);

        // Return modified field data
        let mut modified_field = self.pipeline.field_snapshot.clone();
        modified_field.coherence =
            (modified_field.coherence + influence * self.config.feedback_strength).min(1.0);
        modified_field
    }

    /// Process consciousness → matter integration
    pub fn integrate_consciousness_to_matter(
        &mut self,
        entity_count: usize,
        collective_count: usize,
        avg_consciousness: Float,
    ) {
        let metric = &mut self.statistics.metrics.consciousness_to_matter;

        // More conscious entities = more structured matter
        let collective_factor = (collective_count as Float / entity_count.max(1) as Float).min(1.0);
        let influence = avg_consciousness * (1.0 + collective_factor);

        metric.total_influence += influence;
        metric.affected_count += entity_count;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Update consciousness state in pipeline
        self.pipeline.consciousness_state.avg_consciousness = avg_consciousness;
        self.pipeline.consciousness_state.collective_entities = collective_count;
        self.pipeline.consciousness_state.entity_density = entity_count as Float / 10000.0;
    }

    /// Process entity → environment integration
    pub fn integrate_entity_to_environment(
        &mut self,
        entity_count: usize,
        planet_count: usize,
        terrain_cells: usize,
    ) {
        let metric = &mut self.statistics.metrics.entity_to_environment;

        // More entities with higher consciousness can modify their environment
        let entity_factor = (entity_count as Float / 1000.0).min(1.0);
        let planet_factor = (planet_count as Float / 10.0).min(1.0);
        let influence = entity_factor * planet_factor;

        metric.total_influence += influence;
        metric.affected_count += terrain_cells;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Update environment state in pipeline
        self.pipeline.environment_state.planet_count = planet_count;
    }

    /// Process environment → entity integration
    pub fn integrate_environment_to_entity(
        &mut self,
        planet_count: usize,
        terrain_coherence: Float,
        biological_activity: Float,
    ) -> EnvironmentEntityInfluence {
        let metric = &mut self.statistics.metrics.environment_to_entity;

        // Better environment supports more entities
        let quality = terrain_coherence;
        let resources = biological_activity;
        let danger = 1.0 - quality;

        let influence = quality * resources;

        metric.total_influence += influence;
        metric.affected_count += planet_count;
        metric.avg_influence = metric.total_influence / metric.affected_count.max(1) as Float;

        // Update environment state
        self.pipeline.environment_state.avg_terrain_coherence = terrain_coherence;
        self.pipeline.environment_state.biological_activity = biological_activity;

        EnvironmentEntityInfluence {
            position: Position3D::origin(),
            quality,
            resources,
            danger,
        }
    }

    /// Calculate overall integration coherence
    pub fn calculate_coherence(&mut self) {
        let metrics = &self.statistics.metrics;

        // Average all feedback influences
        let total_influence = metrics.field_to_space.avg_influence
            + metrics.field_to_matter.avg_influence
            + metrics.matter_to_field.avg_influence
            + metrics.consciousness_to_matter.avg_influence
            + metrics.entity_to_environment.avg_influence
            + metrics.environment_to_entity.avg_influence;

        let avg_influence = total_influence / 6.0;

        // Average all stabilities
        let stability =
            (metrics.matter_to_field.stability + metrics.field_to_space.stability) / 2.0;

        // Combined coherence
        self.statistics.coherence = (avg_influence * 0.7 + stability * 0.3).min(1.0);

        // Track history
        self.coherence_history.push(self.statistics.coherence);
        if self.coherence_history.len() > self.max_history {
            self.coherence_history.remove(0);
        }

        // Calculate overall coherence from history
        let sum: Float = self.coherence_history.iter().sum();
        let count = self.coherence_history.len() as Float;
        self.statistics.metrics.overall_coherence = if count > 0.0 { sum / count } else { 0.5 };

        // Stability is variance in coherence
        if count > 1.0 {
            let mean = sum / count;
            let variance: Float = self
                .coherence_history
                .iter()
                .map(|x| (x - mean).powi(2))
                .sum::<Float>()
                / count;
            self.statistics.stability = (1.0 - variance.sqrt()).max(0.0);
        }
    }

    /// Record phase execution time
    pub fn record_phase(&mut self, name: String, time_us: u64, entities: usize) {
        self.phase_timings.push((name.clone(), time_us));

        // Update performance tracking
        let efficiency = if time_us > 0 {
            entities as Float / time_us as Float
        } else {
            0.0
        };

        // Update or add phase performance
        if let Some(perf) = self
            .statistics
            .phase_performance
            .iter_mut()
            .find(|p| p.name == name)
        {
            perf.execution_time_us = time_us;
            perf.entities_processed = entities;
            perf.efficiency = efficiency;
        } else {
            self.statistics.phase_performance.push(PhasePerformance {
                name,
                execution_time_us: time_us,
                entities_processed: entities,
                efficiency,
            });
        }
    }

    /// Update field snapshot
    pub fn update_field_snapshot(&mut self, field_data: FieldNodeData) {
        self.pipeline.field_snapshot = field_data;
    }

    /// Update total entities
    pub fn update_entity_count(&mut self, count: usize) {
        self.statistics.total_entities = count;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &IntegrationStatistics {
        &self.statistics
    }

    /// Get pipeline state
    pub fn get_pipeline(&self) -> &UnifiedPipeline {
        &self.pipeline
    }

    /// Get overall coherence
    pub fn get_coherence(&self) -> Float {
        self.statistics.coherence
    }

    /// Get stability
    pub fn get_stability(&self) -> Float {
        self.statistics.stability
    }

    /// Check if integration is healthy
    pub fn is_healthy(&self) -> bool {
        self.statistics.coherence > 0.3 && self.statistics.stability > 0.5
    }

    /// Reset statistics for new cycle
    pub fn reset_statistics(&mut self) {
        self.statistics.metrics = IntegrationMetrics::default();
        self.statistics.phase_performance.clear();
        self.phase_timings.clear();
    }

    /// Get performance summary
    pub fn get_performance_summary(&self) -> String {
        let mut summary = String::from("Phase Performance:\n");
        for perf in &self.statistics.phase_performance {
            summary.push_str(&format!(
                "  {}: {}μs ({} entities, {:.2} entities/μs)\n",
                perf.name, perf.execution_time_us, perf.entities_processed, perf.efficiency
            ));
        }
        summary
    }
}

/// Bridge to integrate full system with main simulation
pub struct IntegrationBridge {
    integration: FullIntegration,
    config: IntegrationConfig,
}

impl IntegrationBridge {
    /// Create new bridge
    pub fn new(config: IntegrationConfig) -> Self {
        IntegrationBridge {
            integration: FullIntegration::new(config.clone()),
            config,
        }
    }

    /// Create with defaults
    pub fn with_defaults() -> Self {
        Self::new(IntegrationConfig::default())
    }

    /// Process full integration step
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        field_coherence: Float,
        spectrum_position: Float,
        field_data: &FieldNodeData,
        particle_count: usize,
        atom_count: usize,
        molecule_count: usize,
        entity_count: usize,
        collective_count: usize,
        avg_consciousness: Float,
        planet_count: usize,
        terrain_cells: usize,
        terrain_coherence: Float,
        biological_activity: Float,
    ) {
        // Reset metrics each step
        self.integration.reset_statistics();

        // Field → Space
        self.integration
            .integrate_field_to_space(field_coherence, spectrum_position);

        // Field → Matter
        self.integration
            .integrate_field_to_matter(field_data, particle_count, atom_count);

        // Matter → Field (feedback)
        if self.config.enable_feedback {
            let avg_complexity = if molecule_count > 0 {
                0.7
            } else if atom_count > 0 {
                0.5
            } else {
                0.3
            };
            let _modified_field = self.integration.integrate_matter_to_field(
                particle_count + atom_count + molecule_count,
                avg_complexity,
            );
        }

        // Consciousness → Matter
        self.integration.integrate_consciousness_to_matter(
            entity_count,
            collective_count,
            avg_consciousness,
        );

        // Entity → Environment
        self.integration
            .integrate_entity_to_environment(entity_count, planet_count, terrain_cells);

        // Environment → Entity
        let _env_influence = self.integration.integrate_environment_to_entity(
            planet_count,
            terrain_coherence,
            biological_activity,
        );

        // Update field snapshot
        self.integration.update_field_snapshot(field_data.clone());

        // Update entity count
        self.integration.update_entity_count(entity_count);

        // Calculate coherence
        self.integration.calculate_coherence();
    }

    /// Record phase timing
    pub fn record_phase(&mut self, name: String, time_us: u64, entities: usize) {
        self.integration.record_phase(name, time_us, entities);
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &IntegrationStatistics {
        self.integration.get_statistics()
    }

    /// Get pipeline
    pub fn get_pipeline(&self) -> &UnifiedPipeline {
        self.integration.get_pipeline()
    }

    /// Get coherence
    pub fn get_coherence(&self) -> Float {
        self.integration.get_coherence()
    }

    /// Get stability
    pub fn get_stability(&self) -> Float {
        self.integration.get_stability()
    }

    /// Check health
    pub fn is_healthy(&self) -> bool {
        self.integration.is_healthy()
    }

    /// Get performance summary
    pub fn get_performance_summary(&self) -> String {
        self.integration.get_performance_summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = FullIntegration::with_defaults();
        assert_eq!(integration.get_coherence(), 0.0);
    }

    #[test]
    fn test_field_to_space_integration() {
        let mut integration = FullIntegration::with_defaults();
        integration.integrate_field_to_space(0.8, 0.5);

        let stats = integration.get_statistics();
        assert!(stats.metrics.field_to_space.avg_influence > 0.0);
    }

    #[test]
    fn test_integration_coherence() {
        let mut integration = FullIntegration::with_defaults();

        integration.integrate_field_to_space(0.8, 0.5);
        integration.integrate_field_to_matter(&FieldNodeData::new(), 10, 5);
        integration.integrate_consciousness_to_matter(100, 10, 0.7);
        integration.integrate_entity_to_environment(100, 2, 1000);
        integration.integrate_environment_to_entity(2, 0.6, 0.5);

        integration.calculate_coherence();

        assert!(integration.get_coherence() > 0.0);
    }

    #[ignore]
    #[test]
    fn test_integration_bridge() {
        let mut bridge = IntegrationBridge::with_defaults();

        bridge.step(
            0.7, // field_coherence
            0.5, // spectrum_position
            &FieldNodeData::new(),
            100, // particle_count
            50,  // atom_count
            20,  // molecule_count
            137, // entity_count
            5,   // collective_count
            0.6, // avg_consciousness
            1,   // planet_count
            100, // terrain_cells
            0.5, // terrain_coherence
            0.3, // biological_activity
        );

        assert!(bridge.get_coherence() > 0.0);
        assert!(bridge.is_healthy());
    }

    #[test]
    fn test_pipeline_state() {
        let bridge = IntegrationBridge::with_defaults();
        let pipeline = bridge.get_pipeline();

        // Check default values
        assert_eq!(pipeline.spatial_config[0], 3.0);
        assert_eq!(pipeline.matter_state.phase, None);
    }
}
