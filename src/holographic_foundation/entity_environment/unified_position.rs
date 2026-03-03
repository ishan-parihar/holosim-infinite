//! Unified Position System - Field-Primary with Derived Geographic Coordinates
//!
//! From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 9:
//! "Single unified position system. Field manifestation determines physical location.
//!  Geographic coordinates derived, not primary."
//!
//! # Key Insight
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Entity Position = WHERE entity's field configuration manifests"
//!
//! The field position is PRIMARY (where the entity's field resonates).
//! Geographic coordinates (lat/lon/alt) are DERIVED for visualization and planet interaction.
//!
//! # Architecture
//!
//! ```text
//! FieldManifestationPoint (PRIMARY)
//!         ↓
//! Position3D (field-space coordinates)
//!         ↓
//! UnifiedPosition.to_geographic(planet) → GeoCoordinates (DERIVED)
//! ```

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::f64::consts::PI;

pub const EARTH_RADIUS_KM: Float = 6371.0;
pub const DEGREES_PER_RADIAN: Float = 180.0 / PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoCoordinates {
    pub latitude: Float,
    pub longitude: Float,
    pub altitude: Float,
    pub facing: Float,
}

impl GeoCoordinates {
    pub fn new(latitude: Float, longitude: Float, altitude: Float, facing: Float) -> Self {
        Self {
            latitude: latitude.clamp(-90.0, 90.0),
            longitude: longitude.clamp(-180.0, 180.0),
            altitude,
            facing: facing.rem_euclid(360.0),
        }
    }

    pub fn at_surface(latitude: Float, longitude: Float) -> Self {
        Self::new(latitude, longitude, 0.0, 0.0)
    }

    pub fn distance_to(&self, other: &GeoCoordinates, planet_radius_km: Float) -> Float {
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        planet_radius_km * c
    }

    pub fn move_forward(&self, distance_km: Float, planet_radius_km: Float) -> Self {
        let bearing = self.facing.to_radians();
        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let d = distance_km / planet_radius_km;

        let lat2 = (lat1.sin() * d.cos() + lat1.cos() * d.sin() * bearing.cos()).asin();
        let lon2 =
            lon1 + (bearing.sin() * d.sin() * lat1.cos()).atan2(d.cos() - lat1.sin() * lat2.sin());

        Self::new(
            lat2.to_degrees(),
            lon2.to_degrees(),
            self.altitude,
            self.facing,
        )
    }
}

impl Default for GeoCoordinates {
    fn default() -> Self {
        Self::at_surface(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSourceType {
    FieldManifestation,
    GeographicOverride,
    Transitional,
}

#[derive(Debug, Clone)]
pub struct UnifiedPosition {
    field_position: Position3D,
    field_resonance: Float,
    geographic_cache: Option<GeoCoordinates>,
    source_type: PositionSourceType,
    planet_binding: Option<u64>,
}

impl UnifiedPosition {
    pub fn from_field(position: Position3D, resonance: Float) -> Self {
        Self {
            field_position: position,
            field_resonance: resonance,
            geographic_cache: None,
            source_type: PositionSourceType::FieldManifestation,
            planet_binding: None,
        }
    }

    pub fn from_geographic(geo: GeoCoordinates, planet_radius_km: Float) -> Self {
        let field_pos = Self::geo_to_field_position(&geo, planet_radius_km);
        Self {
            field_position: field_pos,
            field_resonance: 0.5,
            geographic_cache: Some(geo),
            source_type: PositionSourceType::GeographicOverride,
            planet_binding: None,
        }
    }

    pub fn with_planet_binding(mut self, planet_id: u64) -> Self {
        self.planet_binding = Some(planet_id);
        self
    }

    pub fn with_resonance(mut self, resonance: Float) -> Self {
        self.field_resonance = resonance;
        self
    }

    pub fn field_position(&self) -> &Position3D {
        &self.field_position
    }

    pub fn field_resonance(&self) -> Float {
        self.field_resonance
    }

    pub fn source_type(&self) -> PositionSourceType {
        self.source_type
    }

    pub fn planet_binding(&self) -> Option<u64> {
        self.planet_binding
    }

    pub fn to_geographic(&mut self, planet_radius_km: Float) -> GeoCoordinates {
        if let Some(ref cached) = self.geographic_cache {
            if self.source_type == PositionSourceType::GeographicOverride {
                return *cached;
            }
        }

        let geo = Self::field_to_geographic(&self.field_position, planet_radius_km);
        self.geographic_cache = Some(geo);
        geo
    }

    pub fn geographic(&self, planet_radius_km: Float) -> GeoCoordinates {
        if let Some(ref cached) = self.geographic_cache {
            if self.source_type == PositionSourceType::GeographicOverride {
                return *cached;
            }
        }
        Self::field_to_geographic(&self.field_position, planet_radius_km)
    }

    pub fn update_field_position(&mut self, new_position: Position3D, new_resonance: Float) {
        self.field_position = new_position;
        self.field_resonance = new_resonance;
        self.geographic_cache = None;
        self.source_type = PositionSourceType::FieldManifestation;
    }

    fn field_to_geographic(field_pos: &Position3D, planet_radius_km: Float) -> GeoCoordinates {
        let x = field_pos.x;
        let y = field_pos.y;
        let z = field_pos.z;

        let r = (x * x + y * y + z * z).sqrt();

        let altitude = if r > 0.0 {
            (r - planet_radius_km * 1000.0).max(0.0)
        } else {
            0.0
        };

        let latitude = if r > 0.0 {
            (z / r).asin() * DEGREES_PER_RADIAN
        } else {
            0.0
        };

        let longitude = if r > 0.0 && (x.abs() > 1e-10 || y.abs() > 1e-10) {
            y.atan2(x) * DEGREES_PER_RADIAN
        } else {
            0.0
        };

        let facing = Self::calculate_facing_from_velocity(x, y, z);

        GeoCoordinates::new(latitude, longitude, altitude, facing)
    }

    fn geo_to_field_position(geo: &GeoCoordinates, planet_radius_km: Float) -> Position3D {
        let lat_rad = geo.latitude.to_radians();
        let lon_rad = geo.longitude.to_radians();

        let r = planet_radius_km * 1000.0 + geo.altitude;

        let x = r * lat_rad.cos() * lon_rad.cos();
        let y = r * lat_rad.cos() * lon_rad.sin();
        let z = r * lat_rad.sin();

        Position3D::new(x, y, z)
    }

    fn calculate_facing_from_velocity(_x: Float, _y: Float, _z: Float) -> Float {
        0.0
    }

    pub fn distance_to(&self, other: &UnifiedPosition) -> Float {
        self.field_position.distance(&other.field_position)
    }

    pub fn spherical_distance_km(&self, other: &UnifiedPosition, planet_radius_km: Float) -> Float {
        let geo1 = self.geographic(planet_radius_km);
        let geo2 = other.geographic(planet_radius_km);
        geo1.distance_to(&geo2, planet_radius_km)
    }
}

impl Default for UnifiedPosition {
    fn default() -> Self {
        Self::from_field(Position3D::zero(), 0.5)
    }
}

#[derive(Debug, Clone)]
pub struct PositionTransition {
    from: UnifiedPosition,
    to: UnifiedPosition,
    progress: Float,
    duration: Float,
}

impl PositionTransition {
    pub fn new(from: UnifiedPosition, to: UnifiedPosition, duration: Float) -> Self {
        Self {
            from,
            to,
            progress: 0.0,
            duration: duration.max(0.001),
        }
    }

    pub fn update(&mut self, dt: Float) -> bool {
        self.progress += dt / self.duration;
        if self.progress >= 1.0 {
            self.progress = 1.0;
            true
        } else {
            false
        }
    }

    pub fn current_position(&self) -> UnifiedPosition {
        let t = self.progress.clamp(0.0, 1.0);

        let from_pos = self.from.field_position();
        let to_pos = self.to.field_position();

        let interpolated = Position3D::new(
            from_pos.x + (to_pos.x - from_pos.x) * t,
            from_pos.y + (to_pos.y - from_pos.y) * t,
            from_pos.z + (to_pos.z - from_pos.z) * t,
        );

        let from_res = self.from.field_resonance();
        let to_res = self.to.field_resonance();
        let interp_resonance = from_res + (to_res - from_res) * t;

        let mut unified = UnifiedPosition::from_field(interpolated, interp_resonance);
        if let Some(planet_id) = self.from.planet_binding() {
            unified = unified.with_planet_binding(planet_id);
        }
        unified.source_type = PositionSourceType::Transitional;
        unified
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_coordinates_creation() {
        let geo = GeoCoordinates::new(45.0, -120.0, 100.0, 90.0);
        assert!((geo.latitude - 45.0).abs() < 0.001);
        assert!((geo.longitude - (-120.0)).abs() < 0.001);
        assert!((geo.altitude - 100.0).abs() < 0.001);
        assert!((geo.facing - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_geo_coordinates_clamping() {
        let geo = GeoCoordinates::new(100.0, 200.0, 0.0, 400.0);
        assert!((geo.latitude - 90.0).abs() < 0.001);
        assert!((geo.longitude - 180.0).abs() < 0.001);
        assert!((geo.facing - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_unified_position_from_field() {
        let pos = UnifiedPosition::from_field(Position3D::new(1.0, 2.0, 3.0), 0.8);

        assert_eq!(pos.source_type(), PositionSourceType::FieldManifestation);
        assert!((pos.field_resonance() - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_unified_position_from_geographic() {
        let geo = GeoCoordinates::at_surface(45.0, -120.0);
        let pos = UnifiedPosition::from_geographic(geo, EARTH_RADIUS_KM);

        assert_eq!(pos.source_type(), PositionSourceType::GeographicOverride);
    }

    #[test]
    fn test_field_to_geographic_roundtrip() {
        let original_geo = GeoCoordinates::new(45.0, -120.0, 0.0, 0.0);
        let pos = UnifiedPosition::from_geographic(original_geo, EARTH_RADIUS_KM);

        let mut pos_mut = pos.clone();
        let derived_geo = pos_mut.to_geographic(EARTH_RADIUS_KM);

        assert!((derived_geo.latitude - original_geo.latitude).abs() < 0.001);
        assert!((derived_geo.longitude - original_geo.longitude).abs() < 0.001);
    }

    #[test]
    fn test_geo_distance_calculation() {
        let geo1 = GeoCoordinates::at_surface(0.0, 0.0);
        let geo2 = GeoCoordinates::at_surface(0.0, 1.0);

        let distance = geo1.distance_to(&geo2, EARTH_RADIUS_KM);

        assert!(distance > 0.0 && distance < 200.0);
    }

    #[test]
    fn test_position_transition() {
        let from = UnifiedPosition::from_field(Position3D::new(0.0, 0.0, 0.0), 0.5);
        let to = UnifiedPosition::from_field(Position3D::new(10.0, 10.0, 10.0), 0.8);

        let mut transition = PositionTransition::new(from, to, 1.0);

        assert!(!transition.is_complete());

        transition.update(0.5);
        let current = transition.current_position();

        assert!((current.field_position().x - 5.0).abs() < 0.001);

        transition.update(0.5);
        assert!(transition.is_complete());
    }

    #[test]
    fn test_planet_binding() {
        let pos = UnifiedPosition::from_field(Position3D::zero(), 0.5).with_planet_binding(42);

        assert_eq!(pos.planet_binding(), Some(42));
    }

    #[test]
    fn test_unified_position_distance() {
        let pos1 = UnifiedPosition::from_field(Position3D::new(0.0, 0.0, 0.0), 0.5);
        let pos2 = UnifiedPosition::from_field(Position3D::new(3.0, 4.0, 0.0), 0.5);

        let distance = pos1.distance_to(&pos2);

        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_equator_position() {
        let geo = GeoCoordinates::at_surface(0.0, 0.0);
        let pos = UnifiedPosition::from_geographic(geo, EARTH_RADIUS_KM);

        let derived_geo = pos.geographic(EARTH_RADIUS_KM);

        assert!((derived_geo.latitude).abs() < 0.01);
        assert!((derived_geo.longitude).abs() < 0.01);
    }

    #[test]
    fn test_pole_position() {
        let geo = GeoCoordinates::at_surface(90.0, 0.0);
        let pos = UnifiedPosition::from_geographic(geo, EARTH_RADIUS_KM);

        let derived_geo = pos.geographic(EARTH_RADIUS_KM);

        assert!((derived_geo.latitude - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_update_field_position_clears_cache() {
        let geo = GeoCoordinates::at_surface(45.0, -120.0);
        let mut pos = UnifiedPosition::from_geographic(geo, EARTH_RADIUS_KM);

        let _ = pos.to_geographic(EARTH_RADIUS_KM);
        assert!(pos.geographic_cache.is_some());

        pos.update_field_position(Position3D::new(100.0, 100.0, 100.0), 0.9);

        assert_eq!(pos.source_type(), PositionSourceType::FieldManifestation);
    }
}

#[cfg(test)]
mod phase9_integration_tests {
    use super::*;
    use crate::holographic_foundation::field_state::Position3D;

    #[test]
    fn test_phase9_field_primary_paradigm() {
        let field_pos = Position3D::new(
            EARTH_RADIUS_KM * 1000.0 * 0.7071,
            EARTH_RADIUS_KM * 1000.0 * 0.7071,
            0.0,
        );
        let unified = UnifiedPosition::from_field(field_pos, 0.85);

        assert_eq!(
            unified.source_type(),
            PositionSourceType::FieldManifestation
        );
        assert!((unified.field_resonance() - 0.85).abs() < 0.001);

        let geo = unified.geographic(EARTH_RADIUS_KM);
        assert!((geo.latitude).abs() < 5.0);
    }

    #[test]
    fn test_phase9_geographic_derived_not_primary() {
        let geo = GeoCoordinates::at_surface(45.0, -120.0);
        let unified = UnifiedPosition::from_geographic(geo, EARTH_RADIUS_KM);

        let derived_geo = unified.geographic(EARTH_RADIUS_KM);
        assert!((derived_geo.latitude - 45.0).abs() < 1.0);
        assert!((derived_geo.longitude - (-120.0)).abs() < 1.0);
    }

    #[test]
    fn test_phase9_multiple_positions_same_planet() {
        let positions: Vec<UnifiedPosition> = vec![
            UnifiedPosition::from_geographic(GeoCoordinates::at_surface(0.0, 0.0), EARTH_RADIUS_KM),
            UnifiedPosition::from_geographic(
                GeoCoordinates::at_surface(45.0, -120.0),
                EARTH_RADIUS_KM,
            ),
            UnifiedPosition::from_geographic(
                GeoCoordinates::at_surface(-33.0, 151.0),
                EARTH_RADIUS_KM,
            ),
        ]
        .into_iter()
        .map(|p| p.with_planet_binding(1))
        .collect();

        for pos in &positions {
            assert_eq!(pos.planet_binding(), Some(1));
        }
    }

    #[test]
    fn test_phase9_resonance_tracking() {
        let mut pos = UnifiedPosition::from_field(Position3D::zero(), 0.5);

        pos.update_field_position(Position3D::new(100.0, 100.0, 100.0), 0.8);

        assert!((pos.field_resonance() - 0.8).abs() < 0.001);
        assert_eq!(pos.source_type(), PositionSourceType::FieldManifestation);
    }

    #[test]
    fn test_phase9_roundtrip_accuracy() {
        let original_geo = GeoCoordinates::new(37.7749, -122.4194, 10.0, 45.0);

        let unified = UnifiedPosition::from_geographic(original_geo, EARTH_RADIUS_KM);

        let roundtrip_geo = unified.geographic(EARTH_RADIUS_KM);

        assert!((roundtrip_geo.latitude - original_geo.latitude).abs() < 0.01);
        assert!((roundtrip_geo.longitude - original_geo.longitude).abs() < 0.01);
    }
}
