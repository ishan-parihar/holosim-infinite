//! Integration Adapter for LivingEnvironment with UnifiedPosition
//!
//! From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 9:
//! "Single unified position system. Field manifestation determines physical location.
//!  Geographic coordinates derived, not primary."
//!
//! This module provides adapters between:
//! - `EntitySpatialPosition` (legacy geographic-only system)
//! - `UnifiedPosition` (new field-primary system)
//!
//! The adapter enables gradual migration while maintaining backward compatibility.

use crate::cosmos::planetary_formation::Planet;
use crate::holographic_foundation::entity_environment::{GeoCoordinates, UnifiedPosition};
use crate::holographic_foundation::field_state::Position3D;
use crate::simulation_v3::living_environment::EntitySpatialPosition;
use crate::types::Float;

pub const EARTH_RADIUS_KM: Float = 6371.0;

pub trait SpatialPositionAdapter {
    fn to_unified(&self, planet_radius_km: Float) -> UnifiedPosition;
    fn to_geo(&self) -> GeoCoordinates;
}

impl SpatialPositionAdapter for EntitySpatialPosition {
    fn to_unified(&self, planet_radius_km: Float) -> UnifiedPosition {
        let geo = self.to_geo();
        UnifiedPosition::from_geographic(geo, planet_radius_km)
    }

    fn to_geo(&self) -> GeoCoordinates {
        GeoCoordinates::new(self.latitude, self.longitude, self.altitude, self.facing)
    }
}

pub trait UnifiedPositionAdapter {
    fn to_spatial_position(&self, planet_radius_km: Float) -> EntitySpatialPosition;
}

impl UnifiedPositionAdapter for UnifiedPosition {
    fn to_spatial_position(&self, planet_radius_km: Float) -> EntitySpatialPosition {
        let geo = self.geographic(planet_radius_km);
        EntitySpatialPosition::new(geo.latitude, geo.longitude, geo.altitude, geo.facing)
    }
}

pub struct PositionIntegration {
    planet_radius_km: Float,
}

impl PositionIntegration {
    pub fn new(planet: &Planet) -> Self {
        Self {
            planet_radius_km: planet.radius,
        }
    }

    pub fn with_radius(planet_radius_km: Float) -> Self {
        Self { planet_radius_km }
    }

    pub fn planet_radius(&self) -> Float {
        self.planet_radius_km
    }

    pub fn spatial_to_unified(&self, spatial: &EntitySpatialPosition) -> UnifiedPosition {
        spatial.to_unified(self.planet_radius_km)
    }

    pub fn unified_to_spatial(&self, unified: &UnifiedPosition) -> EntitySpatialPosition {
        unified.to_spatial_position(self.planet_radius_km)
    }

    pub fn geo_to_unified(&self, geo: &GeoCoordinates) -> UnifiedPosition {
        UnifiedPosition::from_geographic(*geo, self.planet_radius_km)
    }

    pub fn field_to_geo(&self, field_pos: &Position3D) -> GeoCoordinates {
        let unified = UnifiedPosition::from_field(*field_pos, 0.5);
        unified.geographic(self.planet_radius_km)
    }

    pub fn distance_between(&self, pos1: &UnifiedPosition, pos2: &UnifiedPosition) -> Float {
        pos1.spherical_distance_km(pos2, self.planet_radius_km)
    }
}

pub fn convert_environment_positions(
    positions: &std::collections::HashMap<
        crate::entity_layer7::layer7::EntityId,
        EntitySpatialPosition,
    >,
    planet_radius_km: Float,
) -> std::collections::HashMap<crate::entity_layer7::layer7::EntityId, UnifiedPosition> {
    positions
        .iter()
        .map(|(id, spatial)| (id.clone(), spatial.to_unified(planet_radius_km)))
        .collect()
}

pub fn convert_unified_positions(
    positions: &std::collections::HashMap<crate::entity_layer7::layer7::EntityId, UnifiedPosition>,
    planet_radius_km: Float,
) -> std::collections::HashMap<crate::entity_layer7::layer7::EntityId, EntitySpatialPosition> {
    positions
        .iter()
        .map(|(id, unified)| (id.clone(), unified.to_spatial_position(planet_radius_km)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_to_unified_roundtrip() {
        let spatial = EntitySpatialPosition::new(45.0, -120.0, 100.0, 90.0);
        let unified = spatial.to_unified(EARTH_RADIUS_KM);
        let back = unified.to_spatial_position(EARTH_RADIUS_KM);

        assert!((back.latitude - spatial.latitude).abs() < 0.01);
        assert!((back.longitude - spatial.longitude).abs() < 0.01);
        assert!((back.altitude - spatial.altitude).abs() < 1.0);
    }

    #[test]
    fn test_position_integration_creation() {
        let integration = PositionIntegration::with_radius(EARTH_RADIUS_KM);
        assert!((integration.planet_radius() - EARTH_RADIUS_KM).abs() < 0.001);
    }

    #[test]
    fn test_geo_to_unified() {
        let integration = PositionIntegration::with_radius(EARTH_RADIUS_KM);
        let geo = GeoCoordinates::at_surface(45.0, -120.0);
        let unified = integration.geo_to_unified(&geo);

        let derived_geo = unified.geographic(EARTH_RADIUS_KM);
        assert!((derived_geo.latitude - 45.0).abs() < 0.01);
    }

    #[test]
    fn test_field_to_geo_conversion() {
        let integration = PositionIntegration::with_radius(EARTH_RADIUS_KM);

        let field_pos = Position3D::new(EARTH_RADIUS_KM * 1000.0, 0.0, 0.0);

        let geo = integration.field_to_geo(&field_pos);

        assert!((geo.longitude).abs() < 1.0 || (geo.longitude - 180.0).abs() < 1.0);
        assert!((geo.latitude).abs() < 1.0);
    }

    #[test]
    fn test_distance_calculation() {
        let integration = PositionIntegration::with_radius(EARTH_RADIUS_KM);

        let pos1 =
            UnifiedPosition::from_geographic(GeoCoordinates::at_surface(0.0, 0.0), EARTH_RADIUS_KM);
        let pos2 =
            UnifiedPosition::from_geographic(GeoCoordinates::at_surface(0.0, 1.0), EARTH_RADIUS_KM);

        let distance = integration.distance_between(&pos1, &pos2);
        assert!(distance > 0.0 && distance < 200.0);
    }
}
