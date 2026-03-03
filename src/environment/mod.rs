//! Environment Module - Phase 4: Entity-Environment Coupling
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md:
//! "Implement entity-planet coupling"
//! "Entities should be assigned to planets and experience weather/terrain"
//!
//! This module implements:
//! - EntityBridge: Connect entities to planetary environments
//! - EnvironmentalEffects: Weather/terrain effects on entities
//! - PlanetaryCoupling: Bidirectional entity-environment interaction
//!
//! Phase 6.1 additions:
//! - FieldHydrology: Water systems emerging from holographic field coherence
//!
//! Phase 6.2 additions:
//! - UnifiedGaia: Planetary consciousness as Sub-Logos entity with bidirectional entity connection
//!
//! Phase 6.3 additions:
//! - BidirectionalCoupling: Two-way entity-environment influence system

/// Re-export for convenience
pub mod bidirectional_coupling;
pub mod entity_bridge;
pub mod environmental_effects;
pub mod field_hydrology;
pub mod unified_gaia;

pub use bidirectional_coupling::*;
pub use entity_bridge::*;
pub use environmental_effects::*;
pub use field_hydrology::*;
pub use unified_gaia::*;

// ============================================================================
// ENVIRONMENT TYPES
// ============================================================================

use std::collections::HashMap;

/// Unique planet identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanetId(pub u64);

/// Unique environment identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvironmentId(pub u64);

/// Entity's environmental state
#[derive(Debug, Clone)]
pub struct EntityEnvironmentState {
    /// Current planet (if any)
    pub planet_id: Option<PlanetId>,

    /// Environment at current position
    pub environment_id: Option<EnvironmentId>,

    /// Local temperature (affects metabolism)
    pub temperature: f64,

    /// Oxygen level (affects breathing)
    pub oxygen_level: f64,

    /// Current weather pattern
    pub weather: WeatherPattern,

    /// Terrain type
    pub terrain: EnvironmentTerrain,

    /// Movement cost modifier
    pub movement_cost: f64,

    /// Energy expenditure rate
    pub energy_expenditure: f64,
}

/// Weather patterns affecting entities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherPattern {
    Clear,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
}

/// Terrain types for environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentTerrain {
    Ocean,
    Plains,
    Hills,
    Mountains,
    Forest,
    Desert,
    Tundra,
    Volcanic,
}

impl Default for EntityEnvironmentState {
    fn default() -> Self {
        EntityEnvironmentState {
            planet_id: None,
            environment_id: None,
            temperature: 20.0,  // Celsius
            oxygen_level: 0.21, // 21%
            weather: WeatherPattern::Clear,
            terrain: EnvironmentTerrain::Plains,
            movement_cost: 1.0,
            energy_expenditure: 1.0,
        }
    }
}
