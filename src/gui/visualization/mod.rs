//! Visualization Systems Module
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 3:
//! "Implement comprehensive visualization of simulation features"
//!
//! This module provides:
//! - Entity visualization with density, polarity, archetype coding
//! - Spectrum visualization overlay (space/time vs time/space)
//! - Emergence visualization (biological, noospheric, Gaia)
//! - Collective dynamics visualization
//! - Physical structure viewer
//! - 7-Layer architecture visualization (Phase 4)
//! - Veil visualization at v=1 (Phase 4)
//! - Sacred geometry visualization (Phase 5)
//! - Archetype visualization (Phase B.3)
//! - Cellular level visualization (Phase C.4)
//! - Consciousness visualization (Phase D.1): Free Will, Teleological Pull, Causal Inversion, Density Transition

pub mod archetype_viz;
pub mod atomic_viz;
pub mod cellular_viz;
pub mod collective_viz;
pub mod consciousness_viz;
pub mod emergence_viz;
pub mod entity_viz;
pub mod layer_visualization;
pub mod molecular_viz;
pub mod quantum_viz;
pub mod sacred_geometry_viz;
pub mod spectrum_viz;
pub mod structure_viz;
pub mod veil_visualization;

// Exports from layer_visualization (Phase 4: 7-Layer Architecture)
pub use layer_visualization::{
    EntityLayerMapping, LayerColors, LayerInteraction, LayerInteractionType, LayerView,
    LayerVisualization, LayerVisualizationData,
};

// Exports from veil_visualization (Phase 4: Veil at v=1)
pub use veil_visualization::{
    DensityTransition, EntityAccessRenderData, EntitySpectrumAccess, SpectrumSlider,
    SpectrumSliderRenderData, VeilData, VeilRenderData, VeilSide, VeilVisualization,
};

// Exports from sacred_geometry_viz (Phase 5: Sacred Geometry & Fractals)
pub use sacred_geometry_viz::{
    CircleRenderData, FibonacciRenderConnection, FibonacciRenderData, FibonacciRenderPoint,
    FlowerOfLifeRenderData, FrequencyBar, HarmonicResonanceRenderData, IntersectionPoint,
    PlatonicEdge, PlatonicFace, PlatonicSolidRenderData, PlatonicVertex, SacredGeometryColors,
    SacredGeometryVisualization, SpiralRenderData, SpiralRenderPoint, StandingWaveRenderData,
    VesicaPiscisRenderData, WavePoint,
};

// Exports from archetype_viz (Phase B.3: 22-Archetype System)
pub use archetype_viz::{
    get_archetype_color, get_complex_color, ArchetypeProfileData, ArchetypeRadarChart,
    ArchetypeRadarConfig, InterferenceDisplay, PropertyDerivation, ARCHETYPE_NAMES,
    ARCHETYPE_SHORT_NAMES, BODY_ARCHETYPES, CHOICE_ARCHETYPE, MIND_ARCHETYPES, NUM_ARCHETYPES,
    SPIRIT_ARCHETYPES,
};

// Exports from quantum_viz (Phase C.1: Quantum Level Visualization)
pub use quantum_viz::{
    CacheState, CollapseVisualization, EntanglementVisualization, ObservationResult,
    ObserverEffectVisualization, QuantumColors, QuantumNumberDerivationDisplay,
    QuantumNumberDerivationView, QuantumVisualizationPanel, WavefunctionRenderer,
};

// Exports from atomic_viz (Phase C.2: Atomic Level Visualization)
pub use atomic_viz::{
    AtomicColors, AtomicVisualizationPanel, AttractorFieldData, AttractorFieldVisualizer,
    ElementDisplayData, MassChargeDerivation, MassChargeDerivationData, OrbitalRenderer,
    OrbitalShape, ParticleSignatureData, ParticleSignatureView, PeriodicTableLandscape,
};

// Exports from molecular_viz (Phase C.3: Molecular Level Visualization)
pub use molecular_viz::{
    create_bond_render_data, create_functional_group_render_data, create_geometry_render_data,
    BondRenderData, BondRenderer, FunctionalGroupRenderData, FunctionalGroupVisualizer,
    GeometryRenderData, MolecularColors, MolecularManifestationView, ReactivityRenderData,
};

// Other exports (existing modules)
pub use collective_viz::{
    BehaviorPattern, CollectiveGroup, CollectiveLevel, CollectiveMetrics, CollectiveVisualizer,
    Entity, ResonanceFieldPoint, ResonanceType,
};

pub use emergence_viz::{
    BiologicalMetrics, EmergenceEvent, EmergenceEventType, EmergenceLevel, EmergenceMetricsData,
    EmergenceParticle, EmergenceParticleType, EmergenceVisualizer, GaiaMetrics, NoosphericMetrics,
};

pub use entity_viz::{EntityVisualizationData, EntityVisualizer, VisualizationStyle};

pub use spectrum_viz::{SpectrumOverlay, SpectrumPosition, SpectrumVisualizer};

pub use structure_viz::{
    Entity as StructureEntity, StructureLevel, StructureMetrics, StructureNode, StructureType,
    StructureVisualizer,
};

// Exports from cellular_viz (Phase C.4: Cellular Level Visualization)
pub use cellular_viz::{
    create_demo_blueprint, create_demo_cells, create_demo_planetary_field, create_demo_proteins,
    BlueprintColors, BlueprintRenderData, BlueprintRenderer, CellManifestationView,
    CellStateColors, CellViewData, CellularTab, CellularVisualizationPanel, DevelopmentalStageData,
    GaiaResonanceData, GaiaResonanceView, GeneCategoryColors, GeneExpressionView, GeneViewData,
    NetworkViewData, OrganelleColors, PlanetaryFieldData, ProteinFoldingView, ProteinViewData,
    SecondaryStructureColors,
};

// Exports from consciousness_viz (Phase D.1: Consciousness Integration - Free Will Visualization)
pub use consciousness_viz::{
    ActiveInfluence, AttractorField, CausalInversionView, ChoiceVisualization,
    ConsciousnessPosition, ConsciousnessVisualization, DensityTransitionView, FreeWillVisualizer,
    TeleologicalPullView, TrajectoryPoint,
};
