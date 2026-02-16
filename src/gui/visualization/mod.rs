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

pub mod collective_viz;
pub mod emergence_viz;
pub mod entity_viz;
pub mod layer_visualization;
pub mod spectrum_viz;
pub mod structure_viz;
pub mod veil_visualization;
pub mod sacred_geometry_viz;

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
    SacredGeometryColors, FibonacciRenderPoint, FibonacciRenderConnection, FibonacciRenderData,
    SpiralRenderPoint, SpiralRenderData, PlatonicVertex, PlatonicEdge, PlatonicFace,
    PlatonicSolidRenderData, CircleRenderData, IntersectionPoint, VesicaPiscisRenderData,
    FlowerOfLifeRenderData, FrequencyBar, HarmonicResonanceRenderData, WavePoint,
    StandingWaveRenderData, SacredGeometryVisualization,
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
