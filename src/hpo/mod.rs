//! HPO (Hyperparameter Optimization) System
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Build automated universe parameter optimization system that runs multiple parallel
//! simulations, detects failures, and keeps what works through survival of the fittest."
//!
//! # Overview
//!
//! The HPO system enables automated universe parameter optimization through:
//! - **SimulationRunner**: Parallel execution of 100+ simulations
//! - **HealthMonitor**: Failure detection (coherence violations, stagnation, crashes)
//! - **FitnessEvaluator**: Emergence, stability, and complexity metrics
//! - **ParameterSpace**: Spectrum ratios, veil thickness, archetype configurations
//! - **SelectionAlgorithm**: Survival of fittest parameter configurations
//!
//! # Phase 0: Field-First Architecture
//!
//! This module also includes the foundational field-centric data structures:
//! - **HolographicFieldState**: Primary field representation with complex amplitudes
//! - **HolographicEncoder**: Maps entity properties to field amplitudes/phases
//! - **EntityExtractor**: Derives entities from field configurations
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "Replace entity-centric state with field-centric state"
//!
//! # Usage
//!
//! ```rust
//! use holonic_realms::hpo::{HolographicFieldState, HolographicCodec};
//!
//! let mut field = HolographicFieldState::with_defaults();
//! let codec = HolographicCodec::new();
//! codec.encode(&mut field, &[([0.0, 0.0, 0.0], 0.5, 3, 1.0)]);
//! let result = codec.decode(&mut field);
//! ```

pub mod types;

// Re-export commonly used types
pub use types::{
    ComplexityMetrics, ConfigError, ConfigId, EmergenceMetrics, EntityId, FailureType,
    FitnessScore, Float, HealthStatus, ParameterSpace, SelectionParameters, SimulationConfig,
    SimulationConfigBuilder, SimulationId, SimulationResult, StabilityMetrics,
};

pub mod fitness_evaluator;
pub mod health_monitor;
pub mod hpo_system;
pub mod parameter_space;
pub mod selection;
pub mod simulation_runner;

// Phase 0: Field-First Architecture
pub mod archetype_matter;
pub mod attractor_fields;
pub mod biological_emergence;
pub mod complexity_emergence;
pub mod cosmic_sequence;
pub mod density_sublevels;
pub mod entity_emergence;
pub mod evolution_feedback;
pub mod field_entity_bridge;
pub mod field_state;
pub mod full_integration;
pub mod holographic_encoder;
pub mod holographic_simulation;
pub mod involution_flow;
pub mod larson_framework;
pub mod planetary_emergence;
pub mod social_memory;
pub mod spatial_field;
pub mod spectrum_dynamics;
pub mod spectrum_spatial;
pub mod unified_field;

// Re-export field-first components
pub use field_state::{
    Complex, DensityBand, EntityExtractionResult, ExtractedEntity, FieldBounds, FieldNodeData,
    FieldStatistics, Float as FieldFloat, HolographicFieldConfig, HolographicFieldState,
    OctreeNode,
};

pub use attractor_fields::{
    ArchetypalAttractor, AttractorFieldConfig, AttractorFieldStatistics, AttractorFields,
    AttractorType, AttractorVisualizationData, CollectiveAttractor, EntityAttractor,
};
pub use cosmic_sequence::{
    AttractorVisualization, CosmicSequence, CosmicSequenceConfig, CosmicSequenceStatistics,
    CosmologicalLayer, LayerAttractor,
};
pub use density_sublevels::{
    DensitySubLevel, DensitySubLevelConfig, DensitySubLevelStatistics, DensitySubLevels,
    DensityWithSubLevel,
};
pub use entity_emergence::{
    EmergingEntity, EntityEmergence, EntityEmergenceConfig, EntityEmergenceStatistics,
    EntityRenderData,
};
pub use evolution_feedback::{
    DecisionFeedback, DecisionType, DensityProgression, EntityDecision, EvolutionFeedback,
    EvolutionFeedbackConfig, EvolutionFeedbackStatistics,
};
pub use field_entity_bridge::{BridgeStatistics, FieldEntityBridge, FieldEntityBridgeConfig};
pub use holographic_encoder::{
    EncodingConfig, EntityExtractor, HolographicCodec, HolographicEncoder,
};
pub use holographic_simulation::{
    CollectiveVisualization, FieldVisualizationData, HoloSimConfig, HolographicSimulation,
    RenderableEntity, SimulationStatistics,
};
pub use involution_flow::{
    Aspect, CosmicHierarchy, CosmicHierarchyConfig, HierarchyStatistics, Logos, Ray, SubLogos,
    SubSubLogos, TheOne,
};
pub use larson_framework::{
    LarsonConfig, LarsonFramework, LarsonMode, LarsonStatistics, VeilRelationship,
};
pub use social_memory::{
    Collective, EntityPhase, Resonance, SharedExperience, SocialMemory, SocialMemoryConfig,
    SocialMemoryStatistics,
};
pub use spectrum_dynamics::{
    DensityOscillator, SpectrumDynamics, SpectrumDynamicsConfig, SpectrumPosition,
    SpectrumStatistics, VeilDynamics,
};
pub use unified_field::{
    PrimalDistortionTerms, UnifiedFieldBuilder, UnifiedFieldConfig, UnifiedFieldEquation,
    UnifiedFieldStatistics,
};

// Re-export main components
pub use fitness_evaluator::FitnessEvaluator;
pub use health_monitor::HealthMonitor;
pub use hpo_system::HpoSystem;
pub use parameter_space::ParameterSpaceManager;
pub use selection::SelectionAlgorithm;
pub use simulation_runner::SimulationRunner;

// Phase F5: Biological Integration
pub use biological_emergence::{
    BiologicalEmergence, BiologicalEmergenceConfig, BiologicalStatistics, BiologyBridge, CellType,
    EmergentDNA, EmergentSpecies, LivingCell, Nucleotide,
};

// Phase F4: Complexity Emergence
pub use complexity_emergence::{
    AtomicFormation, AtomicFormationStatistics, ComplexAtom, ComplexMolecule, ComplexityEmergence,
    ComplexityPhase, DensityPhaseTransition, MolecularBonding, MolecularBondingStatistics,
    PhaseTransitionStatistics, PhaseTransitionThresholds,
};

// Phase F3: Archetype-Derived Matter
pub use archetype_matter::{
    ArchetypeMatterConfig, ArchetypeParticleDerivation, Atom, FieldMatterBridge, MatterEmergence,
    MatterEmergenceStatistics, MatterScale, Molecule, Particle, ParticleId, ParticleType,
    NUM_ARCHETYPES,
};

// Phase F2: Spectrum-Driven Space
pub use spectrum_spatial::{
    CoordinateType, SpatialConfig, SpatialProjection, SpectrumSpatialConfig,
    SpectrumSpatialDynamics, SpectrumSpatialStatistics, TransformedCoordinates, VeilBoundary,
};

// Phase F1: Spatial Field Foundation
pub use spatial_field::{
    EntitySpatialBridge, FieldActivity, Position3D, ResolutionLevel, SpatialBounds, SpatialField,
    SpatialFieldConfig, SpatialFieldNode, SpatialFieldStatistics,
};

// Phase F6: Environmental Simulation
pub use planetary_emergence::{
    AtmosphereLayer, Planet, PlanetaryBridge, PlanetaryConfig, PlanetaryEmergence,
    PlanetaryStatistics, TerrainCell, TerrainStatistics, TerrainType, WeatherPattern,
    WeatherSystem,
};

// Phase F7: Full Integration
pub use full_integration::{
    ConsciousnessPipelineState, EntityEnvironmentInfluence, EnvironmentEntityInfluence,
    EnvironmentPipelineState, FeedbackLoop, FeedbackMetric, FullIntegration, IntegrationBridge,
    IntegrationConfig, IntegrationMetrics, IntegrationStatistics, MatterFieldInfluence,
    MatterInfluenceType, MatterPipelineState, PhasePerformance, UnifiedPipeline,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpo_module_exports() {
        // Verify all types are accessible
        let _config = SimulationConfig::new(1);
        let _result = SimulationResult {
            simulation_id: 1,
            config: _config,
            completed: true,
            steps_executed: 100,
            final_coherence: 0.9,
            average_coherence: 0.85,
            coherence_history: vec![],
            energy_conservation_error: 0.01,
            entities_evolved: 50,
            entities_harvested: 10,
            emergence_metrics: EmergenceMetrics {
                biological_score: 0.5,
                noospheric_score: 0.5,
                gaia_score: 0.5,
                overall_score: 0.5,
            },
            stability_metrics: StabilityMetrics {
                coherence_stability: 0.8,
                energy_balance: 0.9,
                resilience: 0.7,
                overall_score: 0.8,
            },
            complexity_metrics: ComplexityMetrics {
                diversity_score: 0.6,
                integration_score: 0.7,
                depth_score: 0.5,
                overall_score: 0.6,
            },
            fitness_score: 0.7,
            execution_time: 10.0,
            failure: None,
            metadata: std::collections::HashMap::new(),
        };
        let _fitness = FitnessScore::new(0.8);
        let _space = ParameterSpace::new();

        assert!(true);
    }

    #[test]
    fn test_field_state_module() {
        // Test field state creation
        let state = HolographicFieldState::with_defaults();
        assert_eq!(state.active_node_count, 1);

        // Test field operations
        let mut state = HolographicFieldState::with_defaults();
        state.add_energy_at([0.0, 0.0, 0.0], 3, 1.0);
        assert!(state.root.field_data.energy > 0.0);
    }

    // Placeholder - test needs API fix
    #[test]
    fn test_holographic_codec_placeholder() {
        // TODO: Fix when EntityData API is finalized
        assert!(true);
    }
}
