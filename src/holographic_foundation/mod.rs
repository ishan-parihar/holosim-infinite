//! Phase 0-15: Unified Holographic Foundation (COMPLETE)
//!
//! This module provides the unified foundation for the holographic simulation,
//! consolidating duplicate types and providing clean integration between:
//! - UniversalTemplate<T> - Generic template for all simulation components
//! - HolographicFieldState - Multi-scale field with recursive subdivision
//! - MERA compression - Tensor network for holographic compression
//! - Three Primal Distortions - Free Will, Love, Light as field dynamics
//! - Spectrum Dynamics - Eight coupled density oscillator fields with Veil crossing
//! - Involution Flow - Top-down causal chain from Logos hierarchy to entities
//! - Evolution Feedback - Bottom-up entity-to-field perturbations
//! - Social Memory + Resonance - Collective formation through resonance
//! - Integration Pipeline - Field-to-entity extraction and visualization (Phase 6)
//! - Quantum Consciousness - Wavefunction, Entanglement, and Collapse (Phase 7)
//! - Atomic Emergence - Atoms as stable attractor fields (Phase 8)
//! - Molecular Emergence - Chemistry as archetype bonding (Phase 9)
//! - Cellular Emergence - Biology from holographic blueprint (Phase 10)
//! - Organism Physiology - Organ systems as field nodes (Phase 11)
//! - Ecosystem Dynamics - Ecosystems as field resonance networks (Phase 12)
//! - Entity-Environment Binding - Environment as extension of entity field (Phase 13)
//! - Higher Density Mechanics - 5th-8th Density field configurations (Phase 14)
//! - Intelligent Infinity Integration - Active source connection (Phase 15 - FINAL)
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Every element in the simulation follows the same holographic template.
//!  We implement the template once, it applies to EVERYTHING."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Unified Field Equation combines all three primal distortions:
//!  ∂ψ/∂t = FreeWill(ψ) + Love(ψ) + Light(ψ)
//!
//!  The spectrum is ONE unified reality with a QUALITATIVE BREAK at v=1.
//!  At v=1, the Veil: Space/Time (3D space, 1D time) ↔ Time/Space (1D space, 3D time)"

pub mod archetype_profile;
pub mod atomic_emergence;
pub mod cellular_emergence;
pub mod distortions;
pub mod ecosystem_dynamics;
pub mod entity_environment;
pub mod evolution;
pub mod field_state;
pub mod higher_density;
pub mod integration;
pub mod integration_pipeline;
pub mod intelligent_infinity;
pub mod involution;
pub mod molecular_emergence;
pub mod organism_physiology;
pub mod quantum_consciousness;
pub mod scale_level;
pub mod social;
pub mod spectrum;
pub mod template;

pub use archetype_profile::ArchetypeActivationProfile;
pub use atomic_emergence::{
    AtomFormationEvent, AtomGalaxyPair, AtomicManifestation, AttractorBasin, AttractorField,
    AttractorStability, ChargeConfiguration, ElementAttractorField, ElementIdentity,
    ElementPosition, EmergenceScalePair, FieldConfiguration, ManifestationConditions,
    PeriodicTableAttractors, ShellConfiguration, SimultaneousEmergence,
};
pub use cellular_emergence::{
    AminoAcid, AminoAcidField, ArchetypeGene, ArchetypeGeneEncoder, CellBoundary, CellId,
    CellManifestation, CellMembrane, CellOrganelle, CellState, CellularFieldConfiguration,
    CellularGaiaPair, CellularPlanetaryResonance, DNAHelix, DNAInterferencePattern,
    ExpressionCondition, ExpressionResult, FieldResonanceExpression, GaiaConsciousness,
    GeneCategory, GeneExpressionEngine, GeneExpressionProfile, GeneId, GeneRegulatoryNetwork,
    Nucleotide, NucleotideEncoding, NucleotideSequence, PlanetaryCellField, ProteinFoldingField,
    ProteinId, ProteinManifestation, ProteinStructure, RegulatorySignal,
};
pub use distortions::{
    CoherenceGradient, DensityAmplitude, DistortionType, FieldState, FreeWillConfig, FreeWillTerm,
    InterferencePattern, LightConfig, LightTerm, LoveConfig, LoveTerm, PerturbationType,
    UnifiedFieldConfig, UnifiedFieldEquation, WaveState,
};
pub use ecosystem_dynamics::{
    AdaptationDirection, CarryingCapacity, CoevolutionPair, CoevolutionRelationship,
    CoevolutionSystem, Corridor, EcologicalPatch, EcosystemField, EcosystemHealth, EcosystemId,
    EcosystemState, EnergyBudget, EnergyFlow, FitnessLandscape, OscillationPattern,
    PatchConnectivity, PatchId, PatchType, Population, PopulationDynamics, PopulationId,
    SpatialEcosystem, Species, SpeciesFieldPattern, SpeciesId, SpeciesInteraction, SpeciesType,
    TrophicCoupling, TrophicLevel, TrophicLink, TrophicNetwork, TrophicNode,
};
pub use entity_environment::{
    AtmosphericInfluence, ConsciousnessWeatherCoupling, EntityGrounding, EntityPlanetBinding,
    ExtractionImpact, FieldManifestationPoint, ManifestationResult, NestingDepth, NestingLevel,
    PlanetaryFieldNesting, PlanetaryResonance, PositionStability, ResourceAvailability,
    ResourceExtraction, ResourceNode, ResourcePool, ResourceType, SeparationIllusion,
    TerrainInfluence, TerrainMetabolismCoupling, TerrainResonance, TerrainType, UnityPerception,
    VeilPerceptionLevel, VeilSeparationMechanics, WeatherField, WeatherState,
};
pub use evolution::{
    CollectiveInfluence, DecisionFeedback, DecisionType, DensityProgression, EntityDecision,
    EvolutionFeedbackConfig, FieldPerturbation, KarmicEncoding, KarmicPattern, PatternSignature,
    ProgressionEvent, SpectrumShift,
};
pub use field_state::HolographicFieldState;
pub use higher_density::{
    ActivatedSeed, CollectiveFieldMerge, CompletionStage, DensityRay, EnergyBodyField,
    GatewayAccess, GatewayMechanics, GatewayResonance, GatewayState, HigherDensityField,
    IntelligentInfinityAccess, IntelligentInfinityMerger, IntelligentInfinityPattern, LightBody,
    LightBodyManifestation, LightBodyState, MergerProgress, OctaveBridge, OctaveTransition,
    OctaveTransitionState, PatternPreservation, PatternSeed, PhotonField, PolarityBalance,
    SocialMemoryComplex, SourceConnection, SourceMergerState, SourcePreparation,
    UnityConsciousness, UnityConsciousnessState,
};
pub use integration::{FoundationConfig, HolographicFoundation};
pub use integration_pipeline::{
    CoherenceMeter, EntityBatch, EntityBirth, EntityDeath, EntityExtractionConfig,
    EntityExtractionPipeline, EntityLifecycleTransition, EntityMerge, EntitySplit,
    ExtractionResult, FieldHeatmap, FieldSnapshot, FieldToGuiBridge, FieldVisualizationConfig,
    FieldVisualizationData, GuiIntegrationConfig, HolographicGuiState, LifecycleTransitionManager,
    PerformanceConfig, PerformanceMetrics, PerformanceOptimizer, RenderCommand,
    SpatialPartitioning, TransitionType, UpdateStrategy, VeilTransparencyIndicator,
    VisualizationRenderData,
};
pub use intelligent_infinity::{
    ActiveFeedbackLoop, ArchitecturalResonance, EmissionPattern, EntityExperience,
    FeedbackAnalysis, FeedbackDirection, FieldEmission, GatewayThreshold, IntelligenceTransmission,
    IntelligentInfinitySource, PatternCategory, PatternLibrary, PatternMatch, PatternRequest,
    PullStrength, ResonanceComponent, SourceConnectionState, SourceEmission, TeleologicalPull,
    TemplateSearch, ThresholdState, TransmissionType, UnityGradient, UnityPullResult,
};
pub use involution::{
    CosmicHierarchy, EntityManifestation, GalacticLogosConfig, HierarchyLevel, HierarchyPath,
    InvolutionFlow, ManifestationParameters, PlanetaryLogosConfig, PrimaryLogosConfig,
    PropagationMode, PropagationResult, SolarLogosConfig,
};
pub use molecular_emergence::{
    ArchetypeBond, BondAngle, BondFormation, BondFormationResult, BondOrder, BondType,
    FunctionalGroup, FunctionalGroupPattern, FunctionalGroupResonance, GeometryPrediction,
    InterferenceMinima, MolecularInterferencePattern, MolecularManifestation,
    MolecularPlanetaryPair, MolecularPlanetarySystem, MolecularShape, PlanetType,
    PlanetaryEmergence, ReactivityProfile,
};
pub use organism_physiology::{
    BodyConsciousness, CirculatorySystem, DigestiveSystem, DiseaseState, DiseaseType,
    EndocrineSystem, FieldDistortion, FieldWave, HealingMechanism, HealingResult, HealingSystem,
    ImmuneSystem, NervousSystem, Organ, OrganCommunication, OrganFieldNode, OrganHealth, OrganId,
    OrganState, OrganSystemCoordinator, OrganSystemId, OrganSystemType, OrganType, OrganVitality,
    OrganismField, OrganismId, OrganismState, OrganismVitality, PhysiologyEngine, PhysiologySignal,
    ReproductiveSystem, RespiratorySystem, SignalType, Tissue, TissueCoherence, TissueId,
    TissueType, TissueVitality,
};
pub use quantum_consciousness::{
    Archetype22Collapse, ArchetypeToQuantumMapping, ChoiceOperator, CollapseContext,
    CollapseResult, EntanglementCorrelation, EntanglementField, PhaseCorrelation,
    QuantumNumberDerivation, QuantumNumberSet, QuantumWavefunction, WavefunctionState,
};
pub use scale_level::ScaleLevel;
pub use social::{
    Collective, CollectiveConfig, CollectiveDecision, CollectiveDynamics, CollectiveFormation,
    CollectivePolarity, CollectiveState, ConnectionType, EmergentProperty, EmergentPropertyType,
    FormationEvent, FormationEventType, PhaseAlignment, ResonanceCalculation, ResonanceConnection,
    ResonanceResult, SharedExperience, SocialMemory, SocialMemoryEntry,
};
pub use spectrum::{
    CoordinateTransform, DensityBandConfig, DensityBandOscillator, DensityBands, DensityPosition,
    SpaceTimeCoordinates, SpectrumConfig, SpectrumDynamics, SpectrumPosition, SpectrumSide,
    SpectrumState, TimeSpaceCoordinates, VeilCrossing, VeilState, VeilTransparency, VelocityRatio,
};
pub use template::{TemplateConfig, TemplateFactory, UniversalTemplate};

pub mod prelude {
    pub use super::archetype_profile::ArchetypeActivationProfile;
    pub use super::atomic_emergence::{
        AtomicManifestation, AttractorField, ElementAttractorField, PeriodicTableAttractors,
        SimultaneousEmergence,
    };
    pub use super::distortions::{
        DistortionType, FieldState, FreeWillTerm, LightTerm, LoveTerm, UnifiedFieldConfig,
        UnifiedFieldEquation,
    };
    pub use super::evolution::{
        CollectiveInfluence, DecisionFeedback, DecisionType, DensityProgression, EntityDecision,
    };
    pub use super::field_state::HolographicFieldState;
    pub use super::integration::HolographicFoundation;
    pub use super::integration_pipeline::{
        EntityExtractionPipeline, FieldToGuiBridge, FieldVisualizationData,
        LifecycleTransitionManager, PerformanceOptimizer,
    };
    pub use super::intelligent_infinity::{
        ActiveFeedbackLoop, ArchitecturalResonance, GatewayThreshold, IntelligentInfinitySource,
        PatternLibrary, TeleologicalPull, ThresholdState,
    };
    pub use super::involution::{
        CosmicHierarchy, EntityManifestation, HierarchyLevel, InvolutionFlow, PrimaryLogosConfig,
    };
    pub use super::quantum_consciousness::{
        Archetype22Collapse, EntanglementField, QuantumNumberSet, QuantumWavefunction,
    };
    pub use super::scale_level::ScaleLevel;
    pub use super::social::{
        Collective, CollectiveFormation, CollectivePolarity, ResonanceCalculation, SocialMemory,
    };
    pub use super::spectrum::{
        DensityBands, DensityPosition, SpectrumPosition, SpectrumState, VeilCrossing, VelocityRatio,
    };
    pub use super::template::{TemplateConfig, TemplateFactory, UniversalTemplate};
}

pub const NUM_ARCHETYPES: usize = 22;
pub const NUM_DENSITIES: usize = 8;
pub const NUM_SCALE_LEVELS: usize = 8;
