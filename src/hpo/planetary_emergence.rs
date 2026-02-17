//! Environmental Simulation (Phase F6)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Goal: Planets, weather, terrain emerge from field patterns, not pre-defined structures."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The third distortion (Light) manifests as the field of potential, where archetypal 
//! patterns actualize into matter."
//!
//! This module implements:
//! - Planetary Emergence: Planets emerge from high-coherence field regions
//! - Terrain Generation: Heightmap derives from field amplitude patterns
//! - Atmosphere Dynamics: Weather emerges from field coherence oscillations
//! - Environmental Layers: Surface, ocean, atmosphere all from field
//!
//! KEY PRINCIPLE: Environments are not pre-defined - they EMERGE from the field
//! just like matter and biology. The planet is a natural consequence of
//! field coherence at the appropriate scale.

use super::spatial_field::Position3D;
use super::field_state::{Complex, FieldNodeData, Float};
use super::complexity_emergence::ComplexityPhase;
use super::archetype_matter::NUM_ARCHETYPES;
use std::collections::HashMap;

/// Configuration for planetary emergence
#[derive(Debug, Clone)]
pub struct PlanetaryConfig {
    /// Minimum field coherence required for planet formation
    pub planet_formation_threshold: Float,
    /// Minimum coherence for water (oceans)
    pub ocean_threshold: Float,
    /// Minimum coherence for mountains
    pub mountain_threshold: Float,
    /// Scale at which planetary features emerge (field coherence radius)
    pub planetary_scale: Float,
    /// Height scaling factor
    pub height_scale: Float,
    /// Weather update interval
    pub weather_interval: usize,
}

impl Default for PlanetaryConfig {
    fn default() -> Self {
        PlanetaryConfig {
            planet_formation_threshold: 0.75,
            ocean_threshold: 0.5,
            mountain_threshold: 0.7,
            planetary_scale: 1000.0,
            height_scale: 100.0,
            weather_interval: 10,
        }
    }
}

/// Type of planetary terrain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    /// Deep ocean (lowest)
    DeepOcean,
    /// Shallow ocean
    Ocean,
    /// Beach/coast
    Beach,
    /// Plains/grassland
    Plains,
    /// Forest
    Forest,
    /// Hills
    Hills,
    /// Mountains
    Mountains,
    /// Peaks/snow cap
    Peaks,
    /// Desert (low coherence but specific pattern)
    Desert,
}

impl TerrainType {
    /// Determine terrain type from field coherence and amplitude
    pub fn from_field(coherence: Float, amplitude: Float) -> Self {
        if coherence < 0.3 {
            TerrainType::DeepOcean
        } else if coherence < 0.5 {
            TerrainType::Ocean
        } else if coherence < 0.55 {
            TerrainType::Beach
        } else if coherence < 0.65 {
            if amplitude < 0.4 {
                TerrainType::Plains
            } else {
                TerrainType::Forest
            }
        } else if coherence < 0.75 {
            TerrainType::Hills
        } else if coherence < 0.85 {
            TerrainType::Mountains
        } else {
            TerrainType::Peaks
        }
    }
    
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::DeepOcean => "Deep Ocean",
            TerrainType::Ocean => "Ocean",
            TerrainType::Beach => "Beach",
            TerrainType::Plains => "Plains",
            TerrainType::Forest => "Forest",
            TerrainType::Hills => "Hills",
            TerrainType::Mountains => "Mountains",
            TerrainType::Peaks => "Snowy Peaks",
            TerrainType::Desert => "Desert",
        }
    }
    
    /// Get terrain color for visualization
    pub fn color(&self) -> [Float; 3] {
        match self {
            TerrainType::DeepOcean => [0.0, 0.0, 0.4],
            TerrainType::Ocean => [0.0, 0.2, 0.6],
            TerrainType::Beach => [0.76, 0.7, 0.5],
            TerrainType::Plains => [0.3, 0.7, 0.2],
            TerrainType::Forest => [0.1, 0.4, 0.1],
            TerrainType::Hills => [0.5, 0.4, 0.2],
            TerrainType::Mountains => [0.4, 0.35, 0.3],
            TerrainType::Peaks => [0.9, 0.9, 1.0],
            TerrainType::Desert => [0.9, 0.8, 0.5],
        }
    }
}

/// Atmosphere layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtmosphereLayer {
    /// No atmosphere
    None,
    /// Thin atmosphere (Mercury-like)
    Thin,
    /// Earth-like atmosphere
    Moderate,
    /// Thick atmosphere (Venus-like)
    Thick,
    /// Gas giant atmosphere
    GasGiant,
}

impl AtmosphereLayer {
    /// Determine atmosphere from field coherence
    pub fn from_coherence(coherence: Float) -> Self {
        if coherence < 0.3 {
            AtmosphereLayer::None
        } else if coherence < 0.45 {
            AtmosphereLayer::Thin
        } else if coherence < 0.65 {
            AtmosphereLayer::Moderate
        } else if coherence < 0.8 {
            AtmosphereLayer::Thick
        } else {
            AtmosphereLayer::GasGiant
        }
    }
}

/// Weather system
#[derive(Debug, Clone)]
pub struct WeatherSystem {
    /// Temperature (0.0 - 1.0, cold to hot)
    pub temperature: Float,
    /// Humidity (0.0 - 1.0, dry to humid)
    pub humidity: Float,
    /// Wind speed (0.0 - 1.0)
    pub wind_speed: Float,
    /// Cloud cover (0.0 - 1.0)
    pub cloud_cover: Float,
    /// Weather pattern type
    pub pattern: WeatherPattern,
}

/// Weather pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherPattern {
    Clear,
    PartlyCloudy,
    Cloudy,
    Rain,
    Storm,
    Snow,
    DesertStorm,
}

impl WeatherSystem {
    /// Create weather from field oscillation patterns
    pub fn from_field(field_data: &FieldNodeData, oscillation_phase: Float) -> Self {
        let coherence = field_data.coherence;
        let amplitude = field_data.total_magnitude();
        
        // Temperature from coherence (higher coherence = more energy = warmer)
        let temperature = coherence;
        
        // Humidity from specific archetype patterns
        // (simplified - real implementation would use archetype activations)
        let humidity = (amplitude * 0.5 + coherence * 0.3).min(1.0);
        
        // Wind from oscillation phase differences
        let wind_speed = (oscillation_phase.sin().abs() * 0.3 + 0.1).min(1.0);
        
        // Cloud cover from coherence + humidity
        let cloud_cover = (humidity * 0.8 + coherence * 0.2).min(1.0);
        
        // Weather pattern from combination
        let pattern = if cloud_cover < 0.3 {
            if temperature > 0.8 {
                WeatherPattern::Clear
            } else {
                WeatherPattern::Clear
            }
        } else if cloud_cover < 0.6 {
            WeatherPattern::PartlyCloudy
        } else if cloud_cover < 0.8 {
            if humidity > 0.6 {
                WeatherPattern::Rain
            } else {
                WeatherPattern::Cloudy
            }
        } else {
            if temperature < 0.3 {
                WeatherPattern::Snow
            } else if humidity > 0.7 {
                WeatherPattern::Storm
            } else {
                WeatherPattern::Cloudy
            }
        };
        
        WeatherSystem {
            temperature,
            humidity,
            wind_speed,
            cloud_cover,
            pattern,
        }
    }
    
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self.pattern {
            WeatherPattern::Clear => "Clear",
            WeatherPattern::PartlyCloudy => "Partly Cloudy",
            WeatherPattern::Cloudy => "Cloudy",
            WeatherPattern::Rain => "Rain",
            WeatherPattern::Storm => "Storm",
            WeatherPattern::Snow => "Snow",
            WeatherPattern::DesertStorm => "Dust Storm",
        }
    }
}

/// Terrain heightmap cell
#[derive(Debug, Clone)]
pub struct TerrainCell {
    /// Position in terrain grid
    pub position: Position3D,
    /// Height above sea level
    pub height: Float,
    /// Terrain type
    pub terrain_type: TerrainType,
    /// Field coherence at this point
    pub coherence: Float,
    /// Field amplitude at this point
    pub amplitude: Float,
}

impl TerrainCell {
    /// Create terrain cell from field data
    pub fn from_field(position: Position3D, field_data: &FieldNodeData, config: &PlanetaryConfig) -> Self {
        let coherence = field_data.coherence;
        let amplitude = field_data.total_magnitude();
        
        // Height from coherence + amplitude
        let height = (coherence - config.ocean_threshold).max(0.0) * config.height_scale;
        
        let terrain_type = TerrainType::from_field(coherence, amplitude);
        
        TerrainCell {
            position,
            height,
            terrain_type,
            coherence,
            amplitude,
        }
    }
}

/// Planet structure emerging from field
#[derive(Debug, Clone)]
pub struct Planet {
    /// Unique ID
    pub id: u64,
    /// Center position in space
    pub position: Position3D,
    /// Planet radius (in simulation units)
    pub radius: Float,
    /// Mass derived from field
    pub mass: Float,
    /// Surface gravity
    pub gravity: Float,
    /// Atmosphere layer type
    pub atmosphere: AtmosphereLayer,
    /// Terrain grid (heightmap)
    pub terrain: Vec<TerrainCell>,
    /// Grid resolution
    pub grid_resolution: usize,
    /// Current weather
    pub weather: WeatherSystem,
    /// Field coherence average
    pub average_coherence: Float,
    /// Archetype pattern (defines planet type)
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
}

impl Planet {
    /// Create a planet from field region
    pub fn from_field(
        id: u64,
        center: Position3D,
        field_region: &[FieldNodeData],
        config: &PlanetaryConfig,
    ) -> Option<Self> {
        if field_region.is_empty() {
            return None;
        }
        
        // Calculate average coherence
        let avg_coherence: Float = field_region.iter()
            .map(|f| f.coherence)
            .sum::<Float>() / field_region.len() as Float;
        
        // Check if coherence is high enough for planet
        if avg_coherence < config.planet_formation_threshold {
            return None;
        }
        
        // Derive mass from field energy
        let total_energy: Float = field_region.iter()
            .map(|f| f.energy)
            .sum();
        let mass = total_energy * 0.001; // Scale factor
        
        // Radius from mass (simplified)
        let radius = (mass / 10.0).cbrt() * config.planetary_scale;
        
        // Gravity from mass/radius
        let gravity = mass / (radius * radius).max(0.001);
        
        // Atmosphere from coherence
        let atmosphere = AtmosphereLayer::from_coherence(avg_coherence);
        
        // Generate terrain from field patterns
        let terrain = Self::generate_terrain(center, radius, field_region, config);
        
        // Weather from field oscillations
        let weather = if !field_region.is_empty() {
            WeatherSystem::from_field(&field_region[0], 0.0)
        } else {
            WeatherSystem {
                temperature: 0.5,
                humidity: 0.5,
                wind_speed: 0.0,
                cloud_cover: 0.0,
                pattern: WeatherPattern::Clear,
            }
        };
        
        // Extract archetype pattern
        let mut archetype_pattern = [0.0; NUM_ARCHETYPES];
        for (i, field) in field_region.iter().take(NUM_ARCHETYPES).enumerate() {
            archetype_pattern[i] = field.coherence;
        }
        
        Some(Planet {
            id,
            position: center,
            radius,
            mass,
            gravity,
            atmosphere,
            terrain,
            grid_resolution: 32,
            weather,
            average_coherence: avg_coherence,
            archetype_pattern,
        })
    }
    
    /// Generate terrain from field patterns
    fn generate_terrain(
        center: Position3D,
        radius: Float,
        field_region: &[FieldNodeData],
        config: &PlanetaryConfig,
    ) -> Vec<TerrainCell> {
        let resolution = 32;
        let mut terrain = Vec::with_capacity(resolution * resolution);
        
        // Generate terrain grid
        for i in 0..resolution {
            for j in 0..resolution {
                let lat = (i as Float / resolution as Float - 0.5) * std::f64::consts::PI;
                let lon = (j as Float / resolution as Float - 0.5) * 2.0 * std::f64::consts::PI;
                
                // Convert to 3D position on sphere
                let x = center.x + radius * lat.cos() * lon.cos();
                let y = center.y + radius * lat.sin();
                let z = center.z + radius * lat.cos() * lon.sin();
                
                let position = Position3D::new(x, y, z);
                
                // Get field data (sample from region)
                let field_idx = (i * resolution + j) % field_region.len();
                let field_data = &field_region[field_idx];
                
                let cell = TerrainCell::from_field(position, field_data, config);
                terrain.push(cell);
            }
        }
        
        terrain
    }
    
    /// Update weather based on field oscillations
    pub fn update_weather(&mut self, field_data: &FieldNodeData, time_step: Float) {
        let oscillation = (time_step * 0.1).sin();
        self.weather = WeatherSystem::from_field(field_data, oscillation);
    }
    
    /// Get terrain at specific coordinates
    pub fn get_terrain_at(&self, lat: Float, lon: Float) -> Option<&TerrainCell> {
        let i = ((lat + std::f64::consts::PI / 2.0) / std::f64::consts::PI * self.grid_resolution as Float) as usize;
        let j = ((lon + std::f64::consts::PI) / (2.0 * std::f64::consts::PI) * self.grid_resolution as Float) as usize;
        
        let idx = i * self.grid_resolution + j;
        self.terrain.get(idx)
    }
    
    /// Get terrain statistics
    pub fn get_terrain_statistics(&self) -> TerrainStatistics {
        let mut terrain_counts = [0usize; 9];
        
        for cell in &self.terrain {
            let idx = match cell.terrain_type {
                TerrainType::DeepOcean => 0,
                TerrainType::Ocean => 1,
                TerrainType::Beach => 2,
                TerrainType::Plains => 3,
                TerrainType::Forest => 4,
                TerrainType::Hills => 5,
                TerrainType::Mountains => 6,
                TerrainType::Peaks => 7,
                TerrainType::Desert => 8,
            };
            terrain_counts[idx] += 1;
        }
        
        let total = self.terrain.len();
        
        TerrainStatistics {
            total_cells: total,
            deep_ocean_pct: terrain_counts[0] as Float / total as Float,
            ocean_pct: terrain_counts[1] as Float / total as Float,
            beach_pct: terrain_counts[2] as Float / total as Float,
            plains_pct: terrain_counts[3] as Float / total as Float,
            forest_pct: terrain_counts[4] as Float / total as Float,
            hills_pct: terrain_counts[5] as Float / total as Float,
            mountains_pct: terrain_counts[6] as Float / total as Float,
            peaks_pct: terrain_counts[7] as Float / total as Float,
            desert_pct: terrain_counts[8] as Float / total as Float,
        }
    }
}

/// Terrain statistics
#[derive(Debug, Clone, Default)]
pub struct TerrainStatistics {
    pub total_cells: usize,
    pub deep_ocean_pct: Float,
    pub ocean_pct: Float,
    pub beach_pct: Float,
    pub plains_pct: Float,
    pub forest_pct: Float,
    pub hills_pct: Float,
    pub mountains_pct: Float,
    pub peaks_pct: Float,
    pub desert_pct: Float,
}

/// Statistics for planetary emergence
#[derive(Debug, Clone, Default)]
pub struct PlanetaryStatistics {
    /// Total planets formed
    pub planet_count: usize,
    /// Total terrain cells
    pub terrain_cells: usize,
    /// Average planet coherence
    pub average_coherence: Float,
    /// Active weather systems
    pub active_weather_systems: usize,
}

/// Main planetary emergence system
pub struct PlanetaryEmergence {
    config: PlanetaryConfig,
    planets: HashMap<u64, Planet>,
    next_planet_id: u64,
    statistics: PlanetaryStatistics,
    /// Field oscillation phase for weather
    oscillation_phase: Float,
}

impl PlanetaryEmergence {
    /// Create new planetary emergence system
    pub fn new(config: PlanetaryConfig) -> Self {
        PlanetaryEmergence {
            config,
            planets: HashMap::new(),
            next_planet_id: 0,
            statistics: PlanetaryStatistics::default(),
            oscillation_phase: 0.0,
        }
    }
    
    /// Create with defaults
    pub fn with_defaults() -> Self {
        Self::new(PlanetaryConfig::default())
    }
    
    /// Try to create a planet at the given position
    pub fn try_create_planet(&mut self, center: Position3D, field_region: &[FieldNodeData]) -> Option<u64> {
        let planet = Planet::from_field(
            self.next_planet_id,
            center,
            field_region,
            &self.config,
        )?;
        
        let id = self.next_planet_id;
        self.next_planet_id += 1;
        
        // Update statistics
        self.statistics.planet_count += 1;
        self.statistics.terrain_cells += planet.terrain.len();
        
        // Calculate new average coherence
        let total_coherence: Float = self.planets.values()
            .map(|p| p.average_coherence)
            .sum::<Float>() + planet.average_coherence;
        self.statistics.average_coherence = total_coherence / self.statistics.planet_count as Float;
        
        self.planets.insert(id, planet);
        Some(id)
    }
    
    /// Update all planets
    pub fn update(&mut self, delta_time: Float) {
        self.oscillation_phase += delta_time;
        
        // Update weather for each planet
        for planet in self.planets.values_mut() {
            // Get sample field data (simplified)
            let sample_field = FieldNodeData {
                density_amplitudes: [Complex::default(); 8],
                coherence: planet.average_coherence,
                spectrum_position: 0.5,
                veil_transparency: 0.5,
                energy: planet.mass * 0.001,
            };
            
            planet.update_weather(&sample_field, self.oscillation_phase);
        }
        
        self.statistics.active_weather_systems = self.planets.len();
    }
    
    /// Get planet by ID
    pub fn get_planet(&self, id: u64) -> Option<&Planet> {
        self.planets.get(&id)
    }
    
    /// Get planet by ID (mutable)
    pub fn get_planet_mut(&mut self, id: u64) -> Option<&mut Planet> {
        self.planets.get_mut(&id)
    }
    
    /// Get all planets
    pub fn get_all_planets(&self) -> &HashMap<u64, Planet> {
        &self.planets
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &PlanetaryStatistics {
        &self.statistics
    }
    
    /// Check if position is on any planet surface
    pub fn get_planet_at_position(&self, position: &Position3D) -> Option<&Planet> {
        for planet in self.planets.values() {
            let dx = position.x - planet.position.x;
            let dy = position.y - planet.position.y;
            let dz = position.z - planet.position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            
            if (distance - planet.radius).abs() < 10.0 {
                return Some(planet);
            }
        }
        None
    }
    
    /// Get total planet count
    pub fn planet_count(&self) -> usize {
        self.planets.len()
    }
    
    /// Get total terrain cells
    pub fn terrain_cell_count(&self) -> usize {
        self.planets.values().map(|p| p.terrain.len()).sum()
    }
}

/// Bridge to integrate planetary emergence with the main simulation
pub struct PlanetaryBridge {
    emergence: PlanetaryEmergence,
    config: PlanetaryConfig,
}

impl PlanetaryBridge {
    /// Create new bridge
    pub fn new(config: PlanetaryConfig) -> Self {
        PlanetaryBridge {
            emergence: PlanetaryEmergence::new(config.clone()),
            config,
        }
    }
    
    /// Create with defaults
    pub fn with_defaults() -> Self {
        Self::new(PlanetaryConfig::default())
    }
    
    /// Process field positions and create planets where coherence is high
    pub fn process_field_positions(&mut self, positions: &[(Position3D, FieldNodeData)]) {
        // Group positions by region
        // Simplified: just check each position's coherence
        
        for (position, field_data) in positions {
            if field_data.coherence >= self.config.planet_formation_threshold {
                // Check if we already have a planet nearby
                let has_nearby = self.emergence.get_planet_at_position(position).is_some();
                
                if !has_nearby {
                    // Try to create planet with surrounding field data
                    let field_region = vec![field_data.clone(); 16]; // Simplified
                    self.emergence.try_create_planet(*position, &field_region);
                }
            }
        }
    }
    
    /// Update planetary systems
    pub fn update(&mut self, delta_time: Float) {
        self.emergence.update(delta_time);
    }
    
    /// Get planet count
    pub fn planet_count(&self) -> usize {
        self.emergence.planet_count()
    }
    
    /// Get terrain cell count
    pub fn terrain_cell_count(&self) -> usize {
        self.emergence.terrain_cell_count()
    }
    
    /// Get all planets
    pub fn get_all_planets(&self) -> &HashMap<u64, Planet> {
        self.emergence.get_all_planets()
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &PlanetaryStatistics {
        self.emergence.get_statistics()
    }
    
    /// Get planet at position
    pub fn get_planet_at(&self, position: &Position3D) -> Option<&Planet> {
        self.emergence.get_planet_at_position(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_type_from_field() {
        let terrain = TerrainType::from_field(0.2, 0.3);
        assert_eq!(terrain, TerrainType::DeepOcean);
        
        let terrain = TerrainType::from_field(0.8, 0.8);
        assert_eq!(terrain, TerrainType::Mountains);
    }
    
    #[test]
    fn test_atmosphere_layer() {
        let atm = AtmosphereLayer::from_coherence(0.2);
        assert_eq!(atm, AtmosphereLayer::None);
        
        let atm = AtmosphereLayer::from_coherence(0.7);
        assert_eq!(atm, AtmosphereLayer::Thick);
    }
    
    #[test]
    fn test_planet_creation() {
        let config = PlanetaryConfig::default();
        let center = Position3D::new([1000.0, 0.0, 0.0]);
        
        let field_region = vec![
            FieldNodeData {
                density_amplitudes: [Complex::default(); 8],
                coherence: 0.8,
                spectrum_position: 0.5,
                veil_transparency: 0.5,
                energy: 1000.0,
            };
            16
        ];
        
        let planet = Planet::from_field(0, center, &field_region, &config);
        assert!(planet.is_some());
        
        let planet = planet.unwrap();
        assert!(planet.radius > 0.0);
        assert!(planet.mass > 0.0);
    }
    
    #[test]
    fn test_planetary_emergence() {
        let mut emergence = PlanetaryEmergence::with_defaults();
        
        let center = Position3D::new([1000.0, 0.0, 0.0]);
        let field_region = vec![
            FieldNodeData {
                density_amplitudes: [Complex::default(); 8],
                coherence: 0.8,
                spectrum_position: 0.5,
                veil_transparency: 0.5,
                energy: 1000.0,
            };
            16
        ];
        
        let id = emergence.try_create_planet(center, &field_region);
        assert!(id.is_some());
        
        emergence.update(0.1);
        
        assert_eq!(emergence.planet_count(), 1);
    }
    
    #[test]
    fn test_weather_system() {
        let field_data = FieldNodeData {
            density_amplitudes: [Complex::default(); 8],
            coherence: 0.7,
            spectrum_position: 0.5,
            veil_transparency: 0.5,
            energy: 100.0,
        };
        
        let weather = WeatherSystem::from_field(&field_data, 0.0);
        assert!(weather.temperature > 0.0);
        assert!(weather.humidity >= 0.0);
    }
}
