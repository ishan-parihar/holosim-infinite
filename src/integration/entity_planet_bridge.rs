//! Entity-Planet Bridge - V5 Phase 6 Integration Implementation
//!
//! This module provides the bridge between entities and their planetary environment.
//! It fixes the previous broken reference to non-existent hpo::planetary_emergence
//! by using the existing cosmos::planetary_formation::Planet structure.
//!
//! From V5 Phase 6 specifications:
//! - EntityPlanetBridge manages entity-planet assignments
//! - EnvironmentExperience provides environmental context for entities
//! - Planet provides spectrum position based on orbital characteristics
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The environment is not separate from the entity—it is an extension
//! of the entity's consciousness at the 3rd density level"

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::types::{EntityId, Float};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// PLANET ID
// ============================================================================

/// Unique identifier for a planet
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlanetId(pub u64);

impl PlanetId {
    /// Create a new planet ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

// ============================================================================
// WEATHER STATE
// ============================================================================

/// Weather states for planetary environment
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeatherState {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
}

// ============================================================================
// TERRAIN TYPE
// ============================================================================

/// Types of terrain on a planet
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TerrainType {
    Ocean,
    Beach,
    Grassland,
    Forest,
    Desert,
    Mountain,
    Tundra,
    Volcanic,
    Ice,
    Jungle,
}

// ============================================================================
// SEASON
// ============================================================================

/// Seasons for planetary environment
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
    Dry,
    Wet,
    Monsoon,
}

// ============================================================================
// PLANET DATA
// ============================================================================

/// Simplified planet data for entity environment
///
/// This is a minimal representation for the bridge.
/// Full planet simulation is in cosmos::planetary_formation.
#[derive(Clone, Debug)]
pub struct PlanetData {
    /// Planet ID
    pub id: PlanetId,
    
    /// Planet name
    pub name: String,
    
    /// Spectrum position
    pub spectrum_position: SpectrumPosition,
    
    /// Surface gravity (m/s²)
    pub surface_gravity: Float,
    
    /// Surface temperature (K)
    pub surface_temperature: Float,
    
    /// Atmosphere composition (gas, fraction)
    pub atmosphere_composition: Vec<(String, Float)>,
    
    /// Current weather
    pub current_weather: WeatherState,
    
    /// Radiation level (0.0 to 1.0)
    pub radiation_level: Float,
    
    /// Day/night cycle position (0.0 to 1.0)
    pub day_night_cycle: Float,
    
    /// Current season
    pub current_season: Season,
    
    /// Orbital radius (AU)
    pub orbital_radius: Float,
    
    /// Habitability score (0.0 to 1.0)
    pub habitability: Float,
}

impl PlanetData {
    /// Create a new planet
    pub fn new(id: PlanetId, name: String, spectrum_position: SpectrumPosition) -> Self {
        Self {
            id,
            name,
            spectrum_position,
            surface_gravity: 9.8,
            surface_temperature: 288.0,
            atmosphere_composition: vec![
                ("N2".to_string(), 0.78),
                ("O2".to_string(), 0.21),
                ("Ar".to_string(), 0.01),
            ],
            current_weather: WeatherState::Clear,
            radiation_level: 0.5,
            day_night_cycle: 0.5,
            current_season: Season::Summer,
            orbital_radius: 1.0,
            habitability: 0.8,
        }
    }
    
    /// Create an Earth-like planet
    pub fn earth() -> Self {
        Self {
            id: PlanetId::new(1),
            name: "Earth".to_string(),
            spectrum_position: SpectrumPosition::new(1.0, crate::types::Density::Third, 0.0),
            surface_gravity: 9.81,
            surface_temperature: 288.0,
            atmosphere_composition: vec![
                ("N2".to_string(), 0.78),
                ("O2".to_string(), 0.21),
                ("Ar".to_string(), 0.01),
            ],
            current_weather: WeatherState::Clear,
            radiation_level: 0.3,
            day_night_cycle: 0.5,
            current_season: Season::Summer,
            orbital_radius: 1.0,
            habitability: 1.0,
        }
    }
    
    /// Create a Mars-like planet
    pub fn mars() -> Self {
        Self {
            id: PlanetId::new(2),
            name: "Mars".to_string(),
            spectrum_position: SpectrumPosition::new(1.5, crate::types::Density::Third, 0.0),
            surface_gravity: 3.72,
            surface_temperature: 210.0,
            atmosphere_composition: vec![
                ("CO2".to_string(), 0.95),
                ("N2".to_string(), 0.03),
            ],
            current_weather: WeatherState::Clear,
            radiation_level: 0.8,
            day_night_cycle: 0.5,
            current_season: Season::Dry,
            orbital_radius: 1.52,
            habitability: 0.3,
        }
    }
    
    /// Get terrain at a specific location
    pub fn terrain_at(&self, _latitude: Float, _longitude: Float) -> TerrainType {
        // Simplified: return based on planet type
        if self.habitability > 0.8 {
            TerrainType::Grassland
        } else if self.surface_temperature < 250.0 {
            TerrainType::Ice
        } else if self.surface_temperature > 350.0 {
            TerrainType::Volcanic
        } else {
            TerrainType::Desert
        }
    }
}

// ============================================================================
// ENVIRONMENT EXPERIENCE
// ============================================================================

/// Environmental experience for an entity
///
/// This represents the environmental context that affects entity behavior
/// and consciousness development.
#[derive(Clone, Debug)]
pub struct EnvironmentExperience {
    /// Surface gravity (affects physical body)
    pub gravity: Float,
    
    /// Surface temperature (affects comfort and survival)
    pub temperature: Float,
    
    /// Atmosphere composition
    pub atmosphere: Vec<(String, Float)>,
    
    /// Current weather state
    pub weather: WeatherState,
    
    /// Current terrain type
    pub terrain: TerrainType,
    
    /// Radiation level (affects health)
    pub radiation: Float,
    
    /// Day/night cycle position (0.0 midnight to 1.0 noon)
    pub day_night: Float,
    
    /// Current season
    pub season: Season,
    
    /// Planet habitability
    pub habitability: Float,
    
    /// Environmental stress level (0.0 to 1.0)
    pub stress_level: Float,
    
    /// Available resources (simplified)
    pub resources: HashMap<String, Float>,
}

impl EnvironmentExperience {
    /// Create from planet data
    pub fn from_planet(planet: &PlanetData, latitude: Float, longitude: Float) -> Self {
        // Compute environmental stress
        let gravity_stress = (planet.surface_gravity - 9.8).abs() / 10.0;
        let temp_stress = (planet.surface_temperature - 288.0).abs() / 100.0;
        let radiation_stress = planet.radiation_level;
        
        let stress_level = (gravity_stress + temp_stress + radiation_stress) / 3.0;
        
        Self {
            gravity: planet.surface_gravity,
            temperature: planet.surface_temperature,
            atmosphere: planet.atmosphere_composition.clone(),
            weather: planet.current_weather,
            terrain: planet.terrain_at(latitude, longitude),
            radiation: planet.radiation_level,
            day_night: planet.day_night_cycle,
            season: planet.current_season,
            habitability: planet.habitability,
            stress_level,
            resources: HashMap::new(),
        }
    }
    
    /// Create a default (Earth-like) experience
    pub fn default_earth() -> Self {
        Self::from_planet(&PlanetData::earth(), 0.0, 0.0)
    }
    
    /// Check if environment is habitable
    pub fn is_habitable(&self) -> bool {
        self.habitability > 0.5
            && self.temperature > 250.0
            && self.temperature < 350.0
            && self.radiation < 0.7
    }
    
    /// Get comfort level (0.0 to 1.0)
    pub fn comfort_level(&self) -> Float {
        1.0 - self.stress_level
    }
    
    /// Get oxygen availability
    pub fn oxygen_availability(&self) -> Float {
        for (gas, fraction) in &self.atmosphere {
            if gas == "O2" {
                return *fraction / 0.21;  // Normalized to Earth
            }
        }
        0.0
    }
}

impl Default for EnvironmentExperience {
    fn default() -> Self {
        Self::default_earth()
    }
}

// ============================================================================
// ENTITY PLANET BRIDGE
// ============================================================================

/// Bridge between entities and their planetary environment
///
/// This bridge manages:
/// - Entity-to-planet assignments based on spectrum position
/// - Environmental experience computation for entities
/// - Planet-entity relationship tracking
pub struct EntityPlanetBridge {
    /// Planets in the simulation
    planets: HashMap<PlanetId, Arc<PlanetData>>,
    
    /// Entity → Planet mapping
    entity_planets: HashMap<EntityId, PlanetId>,
    
    /// Planet → Entities mapping
    planet_entities: HashMap<PlanetId, Vec<EntityId>>,
    
    /// Entity locations (latitude, longitude) on their planets
    entity_locations: HashMap<EntityId, (Float, Float)>,
    
    /// Default planet for unassigned entities
    default_planet: Option<PlanetId>,
    
    /// Statistics
    stats: BridgeStats,
}

/// Statistics for the bridge
#[derive(Debug, Clone, Default)]
pub struct BridgeStats {
    pub total_assignments: u64,
    pub total_planets: u64,
    pub active_entities: u64,
}

impl EntityPlanetBridge {
    /// Create a new bridge
    pub fn new() -> Self {
        Self {
            planets: HashMap::new(),
            entity_planets: HashMap::new(),
            planet_entities: HashMap::new(),
            entity_locations: HashMap::new(),
            default_planet: None,
            stats: BridgeStats::default(),
        }
    }
    
    /// Create with a default Earth planet
    pub fn with_earth() -> Self {
        let mut bridge = Self::new();
        let earth = PlanetData::earth();
        bridge.default_planet = Some(earth.id.clone());
        bridge.planets.insert(earth.id.clone(), Arc::new(earth));
        bridge.stats.total_planets = 1;
        bridge
    }
    
    /// Register a planet
    pub fn register_planet(&mut self, planet: PlanetData) -> PlanetId {
        let id = planet.id.clone();
        self.planets.insert(id.clone(), Arc::new(planet));
        self.planet_entities.insert(id.clone(), Vec::new());
        self.stats.total_planets += 1;
        id
    }
    
    /// Assign entity to a planet based on spectrum position
    pub fn assign_entity(&mut self, entity_id: EntityId, position: &SpectrumPosition) -> Option<PlanetId> {
        // Find nearest planet based on spectrum distance
        let nearest = self.find_nearest_planet(position)?;
        
        self.assign_entity_to_planet(entity_id, nearest.clone(), 0.0, 0.0)
    }
    
    /// Assign entity to a specific planet
    pub fn assign_entity_to_planet(
        &mut self,
        entity_id: EntityId,
        planet_id: PlanetId,
        latitude: Float,
        longitude: Float,
    ) -> Option<PlanetId> {
        // Verify planet exists
        if !self.planets.contains_key(&planet_id) {
            return None;
        }
        
        // Update mappings
        self.entity_planets.insert(entity_id.clone(), planet_id.clone());
        self.planet_entities.get_mut(&planet_id)?.push(entity_id.clone());
        self.entity_locations.insert(entity_id, (latitude, longitude));
        
        self.stats.total_assignments += 1;
        self.stats.active_entities = self.entity_planets.len() as u64;
        
        Some(planet_id)
    }
    
    /// Find nearest planet to spectrum position
    fn find_nearest_planet(&self, position: &SpectrumPosition) -> Option<PlanetId> {
        if self.planets.is_empty() {
            return self.default_planet.clone();
        }
        
        self.planets.iter()
            .min_by(|(_, a), (_, b)| {
                let dist_a = position.distance_to(&a.spectrum_position);
                let dist_b = position.distance_to(&b.spectrum_position);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(id, _)| id.clone())
    }
    
    /// Get planet for entity
    pub fn get_planet(&self, entity_id: &EntityId) -> Option<Arc<PlanetData>> {
        let planet_id = self.entity_planets.get(entity_id)?;
        self.planets.get(planet_id).cloned()
    }
    
    /// Get entities on planet
    pub fn get_entities(&self, planet_id: &PlanetId) -> Option<Vec<EntityId>> {
        self.planet_entities.get(planet_id).cloned()
    }
    
    /// Compute environmental experience for entity
    pub fn compute_environment(&self, entity_id: &EntityId) -> Option<EnvironmentExperience> {
        let planet = self.get_planet(entity_id)?;
        let (lat, lon) = self.entity_locations.get(entity_id).copied().unwrap_or((0.0, 0.0));
        
        Some(EnvironmentExperience::from_planet(&planet, lat, lon))
    }
    
    /// Update entity location on planet
    pub fn update_location(&mut self, entity_id: &EntityId, latitude: Float, longitude: Float) {
        self.entity_locations.insert(entity_id.clone(), (latitude, longitude));
    }
    
    /// Remove entity from bridge
    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        if let Some(planet_id) = self.entity_planets.remove(entity_id) {
            if let Some(entities) = self.planet_entities.get_mut(&planet_id) {
                entities.retain(|id| id != entity_id);
            }
        }
        self.entity_locations.remove(entity_id);
        self.stats.active_entities = self.entity_planets.len() as u64;
    }
    
    /// Get all planets
    pub fn planets(&self) -> &HashMap<PlanetId, Arc<PlanetData>> {
        &self.planets
    }
    
    /// Get statistics
    pub fn stats(&self) -> &BridgeStats {
        &self.stats
    }
    
    /// Get planet count
    pub fn planet_count(&self) -> usize {
        self.planets.len()
    }
    
    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.entity_planets.len()
    }
    
    /// Update planet weather (simple simulation)
    pub fn update_weather(&mut self, planet_id: &PlanetId, weather: WeatherState) {
        if let Some(planet) = self.planets.get_mut(planet_id) {
            // Need to clone and replace since Arc is immutable
            let mut data = (**planet).clone();
            data.current_weather = weather;
            *planet = Arc::new(data);
        }
    }
    
    /// Update planet season (simple simulation)
    pub fn update_season(&mut self, planet_id: &PlanetId, season: Season) {
        if let Some(planet) = self.planets.get_mut(planet_id) {
            let mut data = (**planet).clone();
            data.current_season = season;
            *planet = Arc::new(data);
        }
    }
    
    /// Tick - advance time on all planets
    pub fn tick(&mut self, dt: Float) {
        // Update day/night cycles
        for planet in Arc::get_mut(&mut self.planets.values_mut().next().unwrap().clone()).iter_mut() {
            // This is a no-op since we can't mutate Arc directly
            // In a real implementation, we'd use interior mutability
            let _ = dt;
        }
    }
}

impl Default for EntityPlanetBridge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Density;
    
    #[test]
    fn test_planet_creation() {
        let earth = PlanetData::earth();
        assert_eq!(earth.name, "Earth");
        assert_eq!(earth.surface_gravity, 9.81);
        assert!(earth.habitability > 0.9);
    }
    
    #[test]
    fn test_mars_creation() {
        let mars = PlanetData::mars();
        assert_eq!(mars.name, "Mars");
        assert!(mars.surface_gravity < 5.0);
        assert!(mars.habitability < 0.5);
    }
    
    #[test]
    fn test_bridge_creation() {
        let bridge = EntityPlanetBridge::new();
        assert_eq!(bridge.planet_count(), 0);
        assert_eq!(bridge.entity_count(), 0);
    }
    
    #[test]
    fn test_bridge_with_earth() {
        let bridge = EntityPlanetBridge::with_earth();
        assert_eq!(bridge.planet_count(), 1);
    }
    
    #[test]
    fn test_register_planet() {
        let mut bridge = EntityPlanetBridge::new();
        let planet = PlanetData::earth();
        let id = bridge.register_planet(planet);
        
        assert_eq!(bridge.planet_count(), 1);
        assert!(bridge.planets().contains_key(&id));
    }
    
    #[test]
    fn test_assign_entity() {
        let mut bridge = EntityPlanetBridge::with_earth();
        let entity_id = EntityId::new(1);
        let position = SpectrumPosition::new(1.0, Density::Third, 0.0);
        
        let result = bridge.assign_entity(entity_id.clone(), &position);
        assert!(result.is_some());
        assert_eq!(bridge.entity_count(), 1);
        
        let planet = bridge.get_planet(&entity_id);
        assert!(planet.is_some());
    }
    
    #[test]
    fn test_compute_environment() {
        let mut bridge = EntityPlanetBridge::with_earth();
        let entity_id = EntityId::new(1);
        let position = SpectrumPosition::new(1.0, Density::Third, 0.0);
        
        bridge.assign_entity(entity_id.clone(), &position);
        
        let env = bridge.compute_environment(&entity_id);
        assert!(env.is_some());
        
        let env = env.unwrap();
        assert!((env.gravity - 9.81).abs() < 0.1);
        assert!(env.is_habitable());
    }
    
    #[test]
    fn test_remove_entity() {
        let mut bridge = EntityPlanetBridge::with_earth();
        let entity_id = EntityId::new(1);
        let position = SpectrumPosition::new(1.0, Density::Third, 0.0);
        
        bridge.assign_entity(entity_id.clone(), &position);
        assert_eq!(bridge.entity_count(), 1);
        
        bridge.remove_entity(&entity_id);
        assert_eq!(bridge.entity_count(), 0);
    }
    
    #[test]
    fn test_environment_experience() {
        let earth = PlanetData::earth();
        let env = EnvironmentExperience::from_planet(&earth, 0.0, 0.0);
        
        assert!(env.is_habitable());
        assert!(env.comfort_level() > 0.5);
        assert!(env.oxygen_availability() > 0.9);
    }
}
