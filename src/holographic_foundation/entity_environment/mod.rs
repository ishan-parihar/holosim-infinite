//! Phase 13: Entity-Environment Binding
//!
//! This module implements the grounding of entities in their environment,
//! where entity position emerges from field manifestation and entities
//! are nested within planetary fields.
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entities are GROUNDING POINTS of the holographic field in environmental space.
//!  Entity Position = WHERE entity's field configuration manifests
//!  Entity-Planet Assignment = Entity field nesting within planetary field
//!  Terrain Interaction = Field energy exchange with landscape field
//!  Weather Interaction = Atmospheric field dynamics affecting consciousness
//!  Resource Extraction = Field amplitude modification at location"
//!
//! # Key Insight
//!
//! The Veil creates the ILLUSION of separation from environment:
//! - Entities appear separate but are fundamentally connected through field
//! - "External" environment is actually part of unified consciousness field
//! - Breaking Veil transparency reveals underlying unity
//!
//! # Key Components
//!
//! - `field_manifestation`: Entity position from field manifestation
//! - `planet_nesting`: Entity field nesting within planetary field
//! - `terrain_coupling`: Terrain-type metabolism coupling
//! - `weather_consciousness`: Weather-consciousness state coupling
//! - `resource_dynamics`: Resource extraction with regeneration
//! - `veil_separation`: Veil creates "external" environment illusion
//! - `unified_position`: Unified position system (Phase 9)

pub mod field_manifestation;
pub mod planet_nesting;
pub mod resource_dynamics;
pub mod terrain_coupling;
pub mod unified_position;
pub mod veil_separation;
pub mod weather_consciousness;

pub use field_manifestation::{
    EntityGrounding, FieldManifestationPoint, ManifestationConditions, ManifestationResult,
    PositionStability,
};
pub use planet_nesting::{
    EntityPlanetBinding, NestingDepth, NestingLevel, PlanetaryFieldNesting, PlanetaryResonance,
};
pub use resource_dynamics::{
    ExtractionImpact, ResourceAvailability, ResourceExtraction, ResourceNode, ResourcePool,
    ResourceType,
};
pub use terrain_coupling::{
    TerrainInfluence, TerrainMetabolismCoupling, TerrainResonance, TerrainType,
};
pub use unified_position::{
    GeoCoordinates, PositionSourceType, PositionTransition, UnifiedPosition,
};
pub use veil_separation::{
    SeparationIllusion, UnityPerception, VeilPerceptionLevel, VeilSeparationMechanics,
};
pub use weather_consciousness::{
    AtmosphericInfluence, ConsciousnessWeatherCoupling, WeatherField, WeatherState,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _terrain = TerrainType::Forest;
        let _weather = WeatherState::Clear;
        let _resource = ResourceType::Organic;
        // Test passes if types can be created
    }
}
