//! Planet Module - Living Planets (Phase 3)
//!
//! This module implements dynamic planetary systems including:
//! - Lithosphere: tectonic plates, volcanism, erosion
//! - Hydrosphere: water cycle, oceans, rivers
//! - Atmosphere: weather, storms, climate
//! - Energy Flow: star → planet → biology → entity
//!
//! From V4 Roadmap Phase 3: "Living Planets"
//! Planets are alive. Geology, hydrology, atmosphere, weather all run dynamically
//! each tick. The planet surface is a living system, not a static backdrop.

pub mod atmosphere;
pub mod energy_flow;
pub mod hydrosphere;
pub mod lithosphere;

// Re-export main types
pub use lithosphere::{
    BoundaryType, CrustType, Earthquake, EruptionState, Lithosphere, PlateId, SeismicZone,
    TectonicPlate, TerrainClass, VolcanicType, Volcano, VolcanoId,
};

pub use hydrosphere::{
    CurrentType, Glacier, Hydrosphere, Lake, OceanBasin, OceanCurrent, River, WaterCycle,
};

pub use atmosphere::{
    AtmosphericComposition, CloudFormation, CloudType, DynamicAtmosphere, FrontType, Storm,
    StormType, WeatherFront, WindVector,
};

pub use energy_flow::{
    EnergyFlowMetrics, EnergyFlowSystem, GeographicEnergyDistribution, ProductivityZone,
    ProductivityZoneType, SeasonalVariation,
};

// ============================================================================
// Module Documentation
// ============================================================================

// Living Planets Architecture
//
// The planet module implements a complete planetary system where:
//
// ### Lithosphere (Dynamic Geology)
// - Tectonic plates move at realistic rates
// - Volcanic eruptions occur at plate boundaries
// - Earthquakes track stress accumulation
// - Erosion shapes terrain over time
//
// ### Hydrosphere (Dynamic Water)
// - Oceans with realistic currents
// - Rivers flow based on precipitation
// - Lakes respond to climate
// - Glaciers advance/retreat with temperature
// - Water cycle connects all systems
//
// ### Atmosphere (Dynamic Weather)
// - Temperature driven by solar input
// - Wind from pressure gradients + Coriolis
// - Storms form from warm moist air
// - Fronts move across the planet
//
// ### Energy Flow (Star → Planet → Biology → Entity)
// - Stellar radiation powers everything
// - Albedo reflects some energy
// - Atmosphere transmits usable light
// - Photosynthesis captures energy
// - Food web transfers to entities
//
// ## Integration Points
// - **Cosmos**: Receives stellar radiation from Star
// - **Gaia**: Shares atmospheric and hydrological state
// - **Biology**: Receives energy flow for productivity
// - **Entity Experience**: Temperature, weather, seasons

// ============================================================================
// Convenience Traits
// ============================================================================

/// Trait for things that can exist on a planet surface
pub trait PlanetarySurface {
    fn position(&self) -> (f64, f64);
    fn elevation(&self) -> f64;
}

// ============================================================================
// Constants
// ============================================================================

/// Planet surface area (Earth in m²)
pub const EARTH_SURFACE_AREA: f64 = 5.1e14;

/// Ocean surface area (Earth in m²)
pub const OCEAN_SURFACE_AREA: f64 = 3.61e14;

/// Land surface area (Earth in m²)
pub const LAND_SURFACE_AREA: f64 = 1.49e14;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lithosphere_creation() {
        let litho = Lithosphere::new();
        assert!(!litho.tectonic_plates.is_empty());
    }

    #[test]
    fn test_hydrosphere_creation() {
        let hydro = Hydrosphere::new();
        assert!(!hydro.oceans.is_empty());
    }

    #[test]
    fn test_atmosphere_creation() {
        let atm = DynamicAtmosphere::new();
        assert!(!atm.temperature_field.is_empty());
    }

    #[test]
    fn test_energy_flow_creation() {
        let efs = EnergyFlowSystem::new();
        assert!(efs.metrics.stellar_input >= 0.0);
    }
}
