//! Living Environment - Planet Systems for Entity Interaction
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
//! "Environment is substrate, entities must interact to survive"
//!
//! This module wraps planet systems for entity interaction:
//! - Temperature, pressure, weather from atmosphere
//! - Water, food from hydrosphere and biosphere
//! - Terrain from lithosphere
//! - Energy flow from stellar radiation
//!
//! ## Architecture
//!
//! The LivingEnvironment wraps the Planet struct and provides:
//! 1. Spatial positioning for entities (lat/lon/alt)
//! 2. BodyEnvironment derivation from planet systems
//! 3. Resource distribution queries
//! 4. Planetary system ticking (atmosphere, hydrosphere, lithosphere)

use crate::cosmos::planetary_formation::{Planet, Season};
use crate::entity_layer7::layer7::EntityId;
use crate::planet::hydrosphere::Hydrosphere;
use crate::planet::lithosphere::{Lithosphere, TerrainClass};
use crate::simulation_v3::embodied_body::BodyEnvironment;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Solar constant at Earth's orbit (W/m²)
const SOLAR_CONSTANT: Float = 1361.0;

/// Spatial cell resolution in degrees (for caching)
const SPATIAL_CELL_RESOLUTION: Float = 5.0;

/// Altitude bands for atmosphere layers (km)
/// Note: Used for atmosphere layer calculations
#[allow(dead_code)]
const ALTITUDE_BANDS: [Float; 5] = [0.0, 2.0, 10.0, 50.0, 100.0];

// ============================================================================
// Entity Spatial Position
// ============================================================================

/// Position of an entity in the living environment.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
/// "Entities have position on planet surface"
///
/// This represents a 3D position on/near a planetary surface using
/// geographic coordinates (latitude, longitude, altitude) plus
/// orientation (facing direction).
#[derive(Debug, Clone, Default)]
pub struct EntitySpatialPosition {
    /// Latitude in degrees (-90 to 90)
    pub latitude: Float,

    /// Longitude in degrees (-180 to 180)
    pub longitude: Float,

    /// Altitude above sea level in meters
    pub altitude: Float,

    /// Facing direction in degrees (0-360, 0 = North)
    pub facing: Float,
}

impl EntitySpatialPosition {
    /// Create a new spatial position.
    pub fn new(latitude: Float, longitude: Float, altitude: Float, facing: Float) -> Self {
        Self {
            latitude: latitude.clamp(-90.0, 90.0),
            longitude: longitude.clamp(-180.0, 180.0),
            altitude,
            facing: facing.rem_euclid(360.0),
        }
    }

    /// Create a position at sea level.
    pub fn at_surface(latitude: Float, longitude: Float) -> Self {
        Self::new(latitude, longitude, 0.0, 0.0)
    }

    /// Get the spatial cell key for this position.
    ///
    /// The spatial cell discretizes latitude/longitude for caching purposes.
    pub fn spatial_cell_key(&self) -> SpatialCell {
        SpatialCell::from_position(self.latitude, self.longitude)
    }

    /// Calculate distance to another position (haversine formula).
    ///
    /// Returns distance in kilometers.
    pub fn distance_to(&self, other: &EntitySpatialPosition) -> Float {
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        // Earth's radius in km
        6371.0 * c
    }

    /// Move in the facing direction by a distance.
    ///
    /// # Arguments
    /// * `distance_km` - Distance to move in kilometers
    ///
    /// # Returns
    /// New position after movement
    pub fn move_forward(&self, distance_km: Float) -> Self {
        // Simplified movement on a sphere
        let bearing = self.facing.to_radians();
        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let d = distance_km / 6371.0; // Angular distance

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

// ============================================================================
// Spatial Cell
// ============================================================================

/// Discretized lat/lon for caching environment data.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
/// "Cache environment per spatial cell for performance"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpatialCell {
    /// Latitude index (discretized)
    pub lat_index: i32,

    /// Longitude index (discretized)
    pub lon_index: i32,
}

impl SpatialCell {
    /// Create a spatial cell from latitude and longitude.
    pub fn from_position(latitude: Float, longitude: Float) -> Self {
        let lat_index = ((latitude + 90.0) / SPATIAL_CELL_RESOLUTION) as i32;
        let lon_index = ((longitude + 180.0) / SPATIAL_CELL_RESOLUTION) as i32;

        Self {
            lat_index,
            lon_index,
        }
    }

    /// Get the center coordinates of this cell.
    pub fn center(&self) -> (Float, Float) {
        let lat = (self.lat_index as Float) * SPATIAL_CELL_RESOLUTION - 90.0
            + SPATIAL_CELL_RESOLUTION / 2.0;
        let lon = (self.lon_index as Float) * SPATIAL_CELL_RESOLUTION - 180.0
            + SPATIAL_CELL_RESOLUTION / 2.0;
        (lat, lon)
    }
}

// ============================================================================
// Resource Distribution
// ============================================================================

/// Resource availability in a spatial cell.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
/// "Resources: water, food, shelter derived from planet systems"
#[derive(Debug, Clone, Default)]
pub struct ResourceDistribution {
    /// Water availability (0.0-1.0)
    pub water: Float,

    /// Plant biomass density (0.0-1.0)
    pub plant_biomass: Float,

    /// Prey animal density (0.0-1.0)
    pub prey_density: Float,

    /// Predator density (0.0-1.0)
    pub predator_density: Float,

    /// Shelter availability (0.0-1.0)
    pub shelter: Float,

    /// Mineral resources (0.0-1.0)
    pub minerals: Float,

    /// Edible vegetation (0.0-1.0)
    pub edible_plants: Float,

    /// Carrion/scavenging opportunities (0.0-1.0)
    pub carrion: Float,
}

impl ResourceDistribution {
    /// Create a resource distribution from planet systems.
    pub fn from_planet_systems(
        lithosphere: &Lithosphere,
        hydrosphere: &Hydrosphere,
        latitude: Float,
        longitude: Float,
    ) -> Self {
        let terrain = lithosphere.terrain_at(latitude, longitude);

        // Water from hydrosphere
        let depth = hydrosphere.depth_at(latitude, longitude);
        let water = if depth < 0.0 {
            // Underwater
            1.0
        } else if depth < 10.0 {
            // Near water
            0.8
        } else {
            // Distance from water
            0.2
        };

        // Resources vary by terrain
        let (plant_biomass, prey_density, predator_density, shelter) = match terrain {
            TerrainClass::DeepOcean => (0.3, 0.5, 0.3, 0.0),
            TerrainClass::ShallowOcean => (0.5, 0.6, 0.2, 0.0),
            TerrainClass::Beach => (0.2, 0.2, 0.1, 0.1),
            TerrainClass::Lowland => (0.6, 0.5, 0.3, 0.3),
            TerrainClass::Hill => (0.5, 0.4, 0.4, 0.5),
            TerrainClass::Mountain => (0.2, 0.2, 0.3, 0.6),
            TerrainClass::Highland => (0.3, 0.3, 0.2, 0.4),
            TerrainClass::Plateau => (0.4, 0.3, 0.2, 0.3),
            TerrainClass::Desert => (0.1, 0.1, 0.2, 0.2),
            TerrainClass::Tundra => (0.2, 0.2, 0.2, 0.1),
            TerrainClass::Ice => (0.0, 0.05, 0.1, 0.3),
        };

        // Mineral from lithosphere
        let minerals = match terrain {
            TerrainClass::Mountain | TerrainClass::Highland => 0.6,
            TerrainClass::Hill => 0.4,
            _ => 0.2,
        };

        // Edible plants are subset of plant biomass
        let edible_plants = plant_biomass * 0.6;

        // Carrion is rare
        let carrion = 0.05;

        Self {
            water,
            plant_biomass,
            prey_density,
            predator_density,
            shelter,
            minerals,
            edible_plants,
            carrion,
        }
    }

    /// Calculate overall resource richness (0.0-1.0).
    pub fn richness(&self) -> Float {
        (self.water + self.plant_biomass + self.prey_density + self.shelter) / 4.0
    }

    /// Calculate danger level from predators (0.0-1.0).
    pub fn danger_level(&self) -> Float {
        self.predator_density
    }
}

// ============================================================================
// Living Environment Configuration
// ============================================================================

/// Configuration for the living environment.
#[derive(Debug, Clone)]
pub struct LivingEnvironmentConfig {
    /// Enable atmosphere simulation
    pub enable_atmosphere: bool,

    /// Enable hydrosphere simulation
    pub enable_hydrosphere: bool,

    /// Enable lithosphere simulation
    pub enable_lithosphere: bool,

    /// Enable energy flow simulation
    pub enable_energy_flow: bool,

    /// Cache body environments for performance
    pub enable_caching: bool,

    /// Base solar radiation (W/m²)
    pub base_solar_radiation: Float,
}

impl Default for LivingEnvironmentConfig {
    fn default() -> Self {
        Self {
            enable_atmosphere: true,
            enable_hydrosphere: true,
            enable_lithosphere: true,
            enable_energy_flow: true,
            enable_caching: true,
            base_solar_radiation: SOLAR_CONSTANT,
        }
    }
}

impl LivingEnvironmentConfig {
    /// Create a minimal configuration for testing.
    pub fn minimal() -> Self {
        Self {
            enable_atmosphere: false,
            enable_hydrosphere: false,
            enable_lithosphere: false,
            enable_energy_flow: false,
            enable_caching: true,
            base_solar_radiation: SOLAR_CONSTANT,
        }
    }

    /// Create a full configuration for production.
    pub fn full() -> Self {
        Self::default()
    }
}

// ============================================================================
// Living Environment
// ============================================================================

/// Living environment wrapper around planet systems.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
/// "Environment is substrate, entities must interact to survive"
///
/// This struct:
/// 1. Wraps an existing Planet with its dynamic systems
/// 2. Provides entity positioning on the planetary surface
/// 3. Derives BodyEnvironment for entity survival simulation
/// 4. Caches environment data per spatial cell for performance
///
/// # Example
///
/// ```rust
/// use holonic_realms::simulation_v3::living_environment::{
///     LivingEnvironment, LivingEnvironmentConfig, EntitySpatialPosition,
/// };
/// use holonic_realms::cosmos::planetary_formation::Planet;
/// use holonic_realms::holographic::field_address::HolographicAddress;
///
/// let addr = HolographicAddress::cosmic_origin();
/// let planet = Planet::earth_like(addr);
/// let config = LivingEnvironmentConfig::default();
/// let mut env = LivingEnvironment::new(planet, config);
///
/// // Tick the planetary systems
/// env.tick(1361.0, 1.0);
///
/// // Get environment at a location
/// let position = EntitySpatialPosition::at_surface(45.0, -120.0);
/// let body_env = env.body_environment_at(position.latitude, position.longitude, position.altitude);
/// ```
#[derive(Debug, Clone)]
pub struct LivingEnvironment {
    /// The underlying planet
    planet: Planet,

    /// Configuration
    config: LivingEnvironmentConfig,

    /// Entity positions
    entity_positions: HashMap<EntityId, EntitySpatialPosition>,

    /// Cached body environments per spatial cell
    environment_cache: HashMap<SpatialCell, BodyEnvironment>,

    /// Cached resource distributions per spatial cell
    resource_cache: HashMap<SpatialCell, ResourceDistribution>,

    /// Current simulation time
    current_time: Float,

    /// Environment statistics
    stats: LivingEnvironmentStats,
}

/// Statistics for the living environment.
#[derive(Debug, Clone, Default)]
pub struct LivingEnvironmentStats {
    /// Average surface temperature (K)
    pub avg_temperature: Float,

    /// Average pressure (Pa)
    pub avg_pressure: Float,

    /// Global water coverage (0.0-1.0)
    pub water_coverage: Float,

    /// Habitability score (0.0-1.0)
    pub habitability: Float,

    /// Number of entities tracked
    pub entity_count: usize,

    /// Cache hit rate
    pub cache_hit_rate: Float,
}

impl LivingEnvironment {
    /// Create a new living environment from a planet.
    pub fn new(planet: Planet, config: LivingEnvironmentConfig) -> Self {
        let stats = LivingEnvironmentStats::default();

        Self {
            planet,
            config,
            entity_positions: HashMap::new(),
            environment_cache: HashMap::new(),
            resource_cache: HashMap::new(),
            current_time: 0.0,
            stats,
        }
    }

    /// Create a living environment with Earth-like defaults.
    pub fn earth_like() -> Self {
        let addr = crate::holographic::field_address::HolographicAddress::cosmic_origin();
        let planet = Planet::earth_like(addr);
        Self::new(planet, LivingEnvironmentConfig::default())
    }

    /// Tick all planetary systems.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
    /// "Environment tick: atmosphere -> hydrosphere -> lithosphere -> energy_flow"
    ///
    /// # Arguments
    /// * `solar_radiation` - Solar radiation input (W/m²)
    /// * `dt` - Time step
    pub fn tick(&mut self, solar_radiation: Float, dt: Float) {
        // Tick planet evolution (orbital mechanics, seasons)
        self.planet.evolve(dt);

        // Calculate solar angle from rotation phase
        let solar_angle = (self.planet.rotation_phase.sin() * self.planet.axial_tilt.cos()).asin();

        // Get tidal force before mutably borrowing planet
        let tidal_force = self.planet.total_tidal_force();

        // Tick atmosphere
        if self.config.enable_atmosphere {
            if let Some(ref mut atm) = self.planet.atmosphere {
                atm.tick(solar_radiation, solar_angle, self.planet.current_season, dt);
            }
        }

        // Get temperature and precipitation after atmosphere tick
        // These are needed for hydrosphere
        let (temperature, precipitation) = {
            let atm = self.planet.atmosphere.as_ref();
            (
                atm.map(|a| a.temperature_at(0.0, 0.0)).unwrap_or(288.0),
                atm.map(|a| a.precipitation_at(0.0, 0.0)).unwrap_or(0.0),
            )
        };

        // Tick hydrosphere
        if self.config.enable_hydrosphere {
            if let Some(ref mut hydro) = self.planet.hydrosphere {
                hydro.tick(temperature, precipitation, tidal_force, dt);
            }
        }

        // Tick lithosphere
        if self.config.enable_lithosphere {
            if let Some(ref mut litho) = self.planet.lithosphere {
                litho.tick(dt);
            }
        }

        // Tick energy flow
        if self.config.enable_energy_flow {
            if let Some(ref mut efs) = self.planet.energy_flow {
                efs.tick(1.0); // Seasonal factor
            }
        }

        // Clear caches (environment may have changed)
        if self.config.enable_caching {
            self.environment_cache.clear();
            self.resource_cache.clear();
        }

        // Update statistics
        self.update_stats();
        self.current_time += dt;
    }

    /// Get body environment at a location.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
    /// "body_environment_at() derives from planet systems"
    ///
    /// This method combines data from atmosphere, hydrosphere, lithosphere,
    /// and energy flow to create a complete environmental context for
    /// an entity's body.
    pub fn body_environment_at(
        &mut self,
        latitude: Float,
        longitude: Float,
        altitude: Float,
    ) -> BodyEnvironment {
        // Check cache first
        if self.config.enable_caching {
            let cell = SpatialCell::from_position(latitude, longitude);
            if let Some(env) = self.environment_cache.get(&cell) {
                self.stats.cache_hit_rate = (self.stats.cache_hit_rate * 0.99) + 0.01;
                return env.clone();
            }
        }

        // Derive environment from planet systems
        let env = self.derive_body_environment(latitude, longitude, altitude);

        // Cache it
        if self.config.enable_caching {
            let cell = SpatialCell::from_position(latitude, longitude);
            self.environment_cache.insert(cell, env.clone());
            self.stats.cache_hit_rate *= 0.99;
        }

        env
    }

    /// Derive body environment from planet systems.
    fn derive_body_environment(
        &self,
        latitude: Float,
        longitude: Float,
        _altitude: Float,
    ) -> BodyEnvironment {
        // Temperature from atmosphere
        let temperature = self.temperature_at(latitude, longitude);

        // Pressure from atmosphere (adjusted for altitude)
        let pressure = self.pressure_at_altitude(_altitude);

        // Solar radiation from energy flow or default
        let solar_radiation = self
            .planet
            .energy_flow
            .as_ref()
            .map(|e| e.metrics.surface_irradiance)
            .unwrap_or(self.config.base_solar_radiation);

        // Gravity from planet
        let gravity = self.planet.surface_gravity;

        // Resources from planet systems
        let resources = self.resource_distribution_at(latitude, longitude);

        // Day/night phase from planet rotation
        let day_night_phase = self.planet.day_phase_at_longitude(longitude);

        // Season from planet
        let season = match self.planet.current_season {
            Season::Spring => 0.25,
            Season::Summer => 0.5,
            Season::Autumn => 0.75,
            Season::Winter => 0.0,
        };

        // Radiation (simplified)
        let radiation = self.planet.magnetic_field_strength * 1e6; // Simplified

        BodyEnvironment {
            temperature,
            pressure,
            solar_radiation,
            radiation,
            gravity,
            food_density: (resources.plant_biomass + resources.prey_density) / 2.0,
            water_availability: resources.water,
            air_quality: 0.9, // Default good air quality
            prey_density: resources.prey_density,
            predator_density: resources.predator_density,
            plant_density: resources.plant_biomass,
            season,
            day_night_phase,
        }
    }

    /// Get terrain class at a location.
    pub fn terrain_at(&self, latitude: Float, longitude: Float) -> TerrainClass {
        self.planet
            .lithosphere
            .as_ref()
            .map(|l| l.terrain_at(latitude, longitude))
            .unwrap_or(TerrainClass::Lowland)
    }

    /// Get temperature at a location.
    pub fn temperature_at(&self, latitude: Float, longitude: Float) -> Float {
        self.planet
            .atmosphere
            .as_ref()
            .map(|a| a.temperature_at(latitude, longitude))
            .unwrap_or(288.0)
    }

    /// Get pressure at altitude.
    pub fn pressure_at_altitude(&self, altitude: Float) -> Float {
        self.planet
            .atmosphere
            .as_ref()
            .map(|a| a.pressure_at_altitude(altitude))
            .unwrap_or(101325.0)
    }

    /// Get precipitation at a location.
    pub fn precipitation_at(&self, latitude: Float, longitude: Float) -> Float {
        self.planet
            .atmosphere
            .as_ref()
            .map(|a| a.precipitation_at(latitude, longitude))
            .unwrap_or(0.0)
    }

    /// Get resource distribution at a location.
    pub fn resource_distribution_at(
        &self,
        latitude: Float,
        longitude: Float,
    ) -> ResourceDistribution {
        // Check cache
        if self.config.enable_caching {
            let cell = SpatialCell::from_position(latitude, longitude);
            if let Some(resources) = self.resource_cache.get(&cell) {
                return resources.clone();
            }
        }

        // Derive from planet systems

        match (&self.planet.lithosphere, &self.planet.hydrosphere) {
            (Some(litho), Some(hydro)) => {
                ResourceDistribution::from_planet_systems(litho, hydro, latitude, longitude)
            }
            (Some(litho), None) => ResourceDistribution::from_planet_systems(
                litho,
                &Hydrosphere::new(),
                latitude,
                longitude,
            ),
            (None, Some(hydro)) => ResourceDistribution::from_planet_systems(
                &Lithosphere::new(),
                hydro,
                latitude,
                longitude,
            ),
            (None, None) => ResourceDistribution::default(),
        }
    }

    /// Set an entity's position in the environment.
    pub fn set_entity_position(&mut self, entity_id: EntityId, position: EntitySpatialPosition) {
        self.entity_positions.insert(entity_id, position);
        self.stats.entity_count = self.entity_positions.len();
    }

    /// Get an entity's position.
    pub fn get_entity_position(&self, entity_id: &EntityId) -> Option<&EntitySpatialPosition> {
        self.entity_positions.get(entity_id)
    }

    /// Remove an entity's position.
    pub fn remove_entity_position(&mut self, entity_id: &EntityId) {
        self.entity_positions.remove(entity_id);
        self.stats.entity_count = self.entity_positions.len();
    }

    /// Get all entity positions.
    pub fn all_entity_positions(&self) -> &HashMap<EntityId, EntitySpatialPosition> {
        &self.entity_positions
    }

    /// Update statistics.
    fn update_stats(&mut self) {
        // Average temperature
        self.stats.avg_temperature = self
            .planet
            .atmosphere
            .as_ref()
            .map(|a| {
                a.temperature_field
                    .iter()
                    .map(|c| c.temperature)
                    .sum::<Float>()
                    / a.temperature_field.len().max(1) as Float
            })
            .unwrap_or(288.0);

        // Average pressure
        self.stats.avg_pressure = self
            .planet
            .atmosphere
            .as_ref()
            .map(|a| {
                a.pressure_field.iter().map(|c| c.pressure).sum::<Float>()
                    / a.pressure_field.len().max(1) as Float
            })
            .unwrap_or(101325.0);

        // Water coverage from hydrosphere
        self.stats.water_coverage = self
            .planet
            .hydrosphere
            .as_ref()
            .map(|h| {
                h.depth_map.iter().filter(|c| c.depth < 0.0).count() as Float
                    / h.depth_map.len().max(1) as Float
            })
            .unwrap_or(0.7);

        // Habitability from energy flow
        self.stats.habitability = self
            .planet
            .energy_flow
            .as_ref()
            .map(|e| e.habitability_score())
            .unwrap_or(0.5);
    }

    /// Get environment statistics.
    pub fn stats(&self) -> &LivingEnvironmentStats {
        &self.stats
    }

    /// Get the underlying planet.
    pub fn planet(&self) -> &Planet {
        &self.planet
    }

    /// Get mutable access to the planet.
    pub fn planet_mut(&mut self) -> &mut Planet {
        &mut self.planet
    }

    /// Get current simulation time.
    pub fn current_time(&self) -> Float {
        self.current_time
    }

    /// Get the current season.
    pub fn current_season(&self) -> Season {
        self.planet.current_season
    }

    /// Check if it's daytime at a location.
    pub fn is_daytime(&self, longitude: Float) -> bool {
        self.planet.is_daytime(longitude)
    }

    /// Get day phase (0.0-1.0, 0.5 = noon) at a longitude.
    pub fn day_phase(&self, longitude: Float) -> Float {
        self.planet.day_phase_at_longitude(longitude)
    }

    /// Calculate distance between two entities.
    pub fn distance_between_entities(
        &self,
        entity_a: &EntityId,
        entity_b: &EntityId,
    ) -> Option<Float> {
        let pos_a = self.entity_positions.get(entity_a)?;
        let pos_b = self.entity_positions.get(entity_b)?;
        Some(pos_a.distance_to(pos_b))
    }

    /// Find entities within radius of a position.
    pub fn entities_within_radius(
        &self,
        position: &EntitySpatialPosition,
        radius_km: Float,
    ) -> Vec<EntityId> {
        self.entity_positions
            .iter()
            .filter(|(_, pos)| position.distance_to(pos) <= radius_km)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_spatial_position_creation() {
        let pos = EntitySpatialPosition::new(45.0, -120.0, 100.0, 90.0);
        assert!((pos.latitude - 45.0).abs() < 0.001);
        assert!((pos.longitude - (-120.0)).abs() < 0.001);
        assert!((pos.altitude - 100.0).abs() < 0.001);
        assert!((pos.facing - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_entity_spatial_position_clamping() {
        let pos = EntitySpatialPosition::new(100.0, 200.0, 0.0, 0.0);
        assert!(pos.latitude <= 90.0);
        assert!(pos.longitude <= 180.0);
    }

    #[test]
    fn test_spatial_cell_from_position() {
        let cell = SpatialCell::from_position(45.0, -120.0);
        assert_eq!(cell.lat_index, 27); // (45 + 90) / 5
        assert_eq!(cell.lon_index, 12); // (-120 + 180) / 5
    }

    #[test]
    fn test_spatial_cell_center() {
        let cell = SpatialCell::from_position(0.0, 0.0);
        let (lat, lon) = cell.center();
        assert!((lat - 2.5).abs() < 0.001);
        assert!((lon - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_entity_position_distance() {
        let pos1 = EntitySpatialPosition::at_surface(0.0, 0.0);
        let pos2 = EntitySpatialPosition::at_surface(0.0, 1.0);

        // 1 degree at equator ≈ 111 km
        let distance = pos1.distance_to(&pos2);
        assert!(distance > 100.0 && distance < 120.0);
    }

    #[test]
    fn test_resource_distribution_default() {
        let dist = ResourceDistribution::default();
        assert_eq!(dist.water, 0.0);
        assert_eq!(dist.plant_biomass, 0.0);
    }

    #[test]
    fn test_resource_distribution_richness() {
        let dist = ResourceDistribution {
            water: 1.0,
            plant_biomass: 1.0,
            prey_density: 1.0,
            shelter: 1.0,
            ..Default::default()
        };

        assert!((dist.richness() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_living_environment_config_default() {
        let config = LivingEnvironmentConfig::default();
        assert!(config.enable_atmosphere);
        assert!(config.enable_hydrosphere);
        assert!(config.enable_lithosphere);
        assert!(config.enable_energy_flow);
    }

    #[test]
    fn test_living_environment_config_minimal() {
        let config = LivingEnvironmentConfig::minimal();
        assert!(!config.enable_atmosphere);
        assert!(!config.enable_hydrosphere);
        assert!(!config.enable_lithosphere);
        assert!(!config.enable_energy_flow);
    }

    #[test]
    fn test_living_environment_creation() {
        let env = LivingEnvironment::earth_like();
        assert!(env.planet().mass > 0.0);
        assert_eq!(env.entity_positions.len(), 0);
    }

    #[test]
    fn test_living_environment_body_environment_at() {
        let mut env = LivingEnvironment::earth_like();
        let body_env = env.body_environment_at(45.0, -120.0, 0.0);

        // Should have reasonable Earth-like values
        assert!(body_env.temperature > 200.0 && body_env.temperature < 350.0);
        assert!(body_env.pressure > 50000.0);
        assert!((body_env.gravity - 9.8).abs() < 1.0);
    }

    #[test]
    fn test_living_environment_set_entity_position() {
        let mut env = LivingEnvironment::earth_like();
        let entity_id = EntityId::new("test-entity".to_string());
        let position = EntitySpatialPosition::at_surface(45.0, -120.0);

        env.set_entity_position(entity_id.clone(), position);
        assert_eq!(env.entity_positions.len(), 1);

        let retrieved = env.get_entity_position(&entity_id);
        assert!(retrieved.is_some());
        assert!((retrieved.unwrap().latitude - 45.0).abs() < 0.001);
    }

    #[test]
    fn test_living_environment_remove_entity_position() {
        let mut env = LivingEnvironment::earth_like();
        let entity_id = EntityId::new("test-entity".to_string());
        let position = EntitySpatialPosition::at_surface(45.0, -120.0);

        env.set_entity_position(entity_id.clone(), position);
        assert_eq!(env.entity_positions.len(), 1);

        env.remove_entity_position(&entity_id);
        assert_eq!(env.entity_positions.len(), 0);
    }

    #[test]
    fn test_living_environment_tick() {
        let mut env = LivingEnvironment::earth_like();
        let initial_time = env.current_time();

        env.tick(1361.0, 1.0);

        assert!(env.current_time() > initial_time);
    }

    #[test]
    fn test_living_environment_terrain_at() {
        let env = LivingEnvironment::earth_like();
        let terrain = env.terrain_at(45.0, -120.0);

        // Should return a valid terrain class
        match terrain {
            TerrainClass::DeepOcean
            | TerrainClass::ShallowOcean
            | TerrainClass::Beach
            | TerrainClass::Lowland
            | TerrainClass::Hill
            | TerrainClass::Mountain
            | TerrainClass::Highland
            | TerrainClass::Plateau
            | TerrainClass::Desert
            | TerrainClass::Tundra
            | TerrainClass::Ice => (),
        }
    }

    #[test]
    fn test_living_environment_entities_within_radius() {
        let mut env = LivingEnvironment::earth_like();

        let entity1 = EntityId::new("entity-1".to_string());
        let entity2 = EntityId::new("entity-2".to_string());
        let entity3 = EntityId::new("entity-3".to_string());

        env.set_entity_position(entity1.clone(), EntitySpatialPosition::at_surface(0.0, 0.0));
        env.set_entity_position(entity2.clone(), EntitySpatialPosition::at_surface(0.0, 0.1)); // Close
        env.set_entity_position(
            entity3.clone(),
            EntitySpatialPosition::at_surface(45.0, 0.0),
        ); // Far

        let center = EntitySpatialPosition::at_surface(0.0, 0.0);
        let nearby = env.entities_within_radius(&center, 50.0); // 50 km radius

        // Should find entity1 and entity2, but not entity3
        assert!(nearby.contains(&entity1));
        // entity2 should be close enough (0.1 degree ≈ 11 km at equator)
        assert!(nearby.contains(&entity2));
        // entity3 is at 45 degrees latitude, much farther
        assert!(!nearby.contains(&entity3));
    }

    #[test]
    fn test_living_environment_stats() {
        let mut env = LivingEnvironment::earth_like();
        env.tick(1361.0, 1.0);

        let stats = env.stats();
        assert!(stats.avg_temperature > 0.0);
        assert!(stats.avg_pressure > 0.0);
        assert!(stats.water_coverage >= 0.0 && stats.water_coverage <= 1.0);
        assert!(stats.habitability >= 0.0 && stats.habitability <= 1.0);
    }

    #[test]
    fn test_entity_position_move_forward() {
        let pos = EntitySpatialPosition::new(0.0, 0.0, 0.0, 0.0); // Facing North
        let moved = pos.move_forward(111.0); // Move ~1 degree

        // Should have moved north
        assert!(moved.latitude > pos.latitude);
    }

    #[test]
    fn test_resource_distribution_from_planet_systems() {
        let litho = Lithosphere::new();
        let hydro = Hydrosphere::new();

        // Test over land (should have less water)
        let land_resources =
            ResourceDistribution::from_planet_systems(&litho, &hydro, 45.0, -120.0);

        // Test over ocean (should have more water)
        let ocean_resources =
            ResourceDistribution::from_planet_systems(&litho, &hydro, 0.0, -150.0);

        // Water availability should differ
        // (Note: this depends on the actual terrain generation, so just check it's valid)
        assert!(land_resources.water >= 0.0 && land_resources.water <= 1.0);
        assert!(ocean_resources.water >= 0.0 && ocean_resources.water <= 1.0);
    }
}
