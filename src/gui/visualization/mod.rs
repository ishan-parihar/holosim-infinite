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

pub mod collective_viz;
pub mod emergence_viz;
pub mod entity_viz;
pub mod spectrum_viz;
pub mod structure_viz;

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
