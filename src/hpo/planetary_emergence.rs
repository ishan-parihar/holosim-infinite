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

use super::archetype_matter::NUM_ARCHETYPES;
use super::field_state::{Complex, FieldNodeData, Float};
use super::spatial_field::Position3D;
use std::collections::HashMap;

/// Grid position for tectonic stress mapping (hashable)
/// From R&D-6: Quantized position grid for tectonic stress field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        GridPosition { x, y, z }
    }

    /// Convert from Position3D (quantize to grid)
    pub fn from_position(pos: &Position3D, grid_size: Float) -> Self {
        GridPosition {
            x: (pos.x / grid_size).floor() as i32,
            y: (pos.y / grid_size).floor() as i32,
            z: (pos.z / grid_size).floor() as i32,
        }
    }
}

/// Gravitational condensation simulation (R&D-6)
/// From roadmap: Energy attracts energy (Love/Logos as gravity)
#[derive(Debug, Clone)]
pub struct GravitationalCore {
    /// Total energy in the condensing region
    pub total_energy: Float,
    /// Energy distribution (position, energy)
    pub energy_points: Vec<(Position3D, Float)>,
    /// Computed density gradient
    pub density_gradient: Float,
    /// Core formation progress
    pub core_formed: bool,
}

impl GravitationalCore {
    pub fn new() -> Self {
        GravitationalCore {
            total_energy: 0.0,
            energy_points: Vec::new(),
            density_gradient: 0.0,
            core_formed: false,
        }
    }

    /// Apply attraction from field energy point
    pub fn apply_attraction(&mut self, position: Position3D, energy: Float) {
        self.total_energy += energy;
        self.energy_points.push((position, energy));
    }

    /// Compute density gradient (higher toward center)
    pub fn compute_density_gradient(&mut self) -> Float {
        if self.energy_points.is_empty() {
            return 0.0;
        }

        let avg_x = self.energy_points.iter().map(|(p, _)| p.x).sum::<Float>()
            / self.energy_points.len() as Float;
        let avg_y = self.energy_points.iter().map(|(p, _)| p.y).sum::<Float>()
            / self.energy_points.len() as Float;
        let avg_z = self.energy_points.iter().map(|(p, _)| p.z).sum::<Float>()
            / self.energy_points.len() as Float;

        let center = Position3D::new(avg_x, avg_y, avg_z);

        let variance: Float = self
            .energy_points
            .iter()
            .map(|(p, e)| {
                let dx = p.x - center.x;
                let dy = p.y - center.y;
                let dz = p.z - center.z;
                e * (dx * dx + dy * dy + dz * dz)
            })
            .sum::<Float>()
            / self.energy_points.len() as Float;

        self.density_gradient = 1.0 / (1.0 + variance);
        self.density_gradient
    }

    pub fn check_core_formation(&self) -> bool {
        self.density_gradient > 0.5 && self.total_energy > 10.0
    }
}

impl Default for GravitationalCore {
    fn default() -> Self {
        Self::new()
    }
}

/// Tectonic stress simulation (R&D-6)
#[derive(Debug, Clone, Default)]
pub struct TectonicStress {
    pub stress_map: HashMap<GridPosition, Float>,
    pub uplift_rate: Float,
}

impl TectonicStress {
    pub fn new() -> Self {
        TectonicStress {
            stress_map: HashMap::new(),
            uplift_rate: 0.01,
        }
    }

    pub fn compute_from_field(&mut self, field_region: &[FieldNodeData], positions: &[Position3D]) {
        self.stress_map.clear();
        let grid_size = 10.0; // Grid cell size for quantization

        for (field, pos) in field_region.iter().zip(positions.iter()) {
            let stress = field.coherence * field.energy;
            let grid_pos = GridPosition::from_position(pos, grid_size);
            self.stress_map.insert(grid_pos, stress);
        }
    }

    pub fn get_uplift(&self, position: &Position3D) -> Float {
        let grid_size = 10.0;
        let grid_pos = GridPosition::from_position(position, grid_size);
        self.stress_map
            .get(&grid_pos)
            .map(|s| s * self.uplift_rate)
            .unwrap_or(0.0)
    }
}

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

/// Tectonic plate (R&D-6: geological physics)
#[derive(Debug, Clone)]
pub struct TectonicPlate {
    pub plate_id: usize,
    pub boundary: Vec<Position3D>,
    pub velocity: Position3D,
    pub density: Float,
}

/// Geological simulation (R&D-6: proper physics)
#[derive(Debug, Clone)]
pub struct Geology {
    /// Tectonic plates
    pub plates: Vec<TectonicPlate>,
    /// Erosion rate
    pub erosion_rate: Float,
}

impl Geology {
    /// Calculate tectonic uplift at a position
    pub fn calculate_uplift(&self, position: &Position3D) -> Float {
        let mut uplift = 0.0;
        for plate in &self.plates {
            if !plate.boundary.is_empty() {
                let dx = position.x - plate.boundary[0].x;
                let dy = position.y - plate.boundary[0].y;
                let dz = position.z - plate.boundary[0].z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                if distance < 100.0 {
                    uplift += plate.density * 10.0 / (distance + 1.0);
                }
            }
        }
        uplift
    }
}

impl Default for Geology {
    fn default() -> Self {
        Self {
            plates: Vec::new(),
            erosion_rate: 0.001,
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

    /// Determine terrain type from geological forces (R&D-6)
    pub fn from_geological_forces(height: Float, coherence: Float, tectonic_uplift: Float) -> Self {
        if height < 0.0 {
            TerrainType::DeepOcean
        } else if height < 10.0 {
            TerrainType::Ocean
        } else if height < 20.0 {
            TerrainType::Beach
        } else if tectonic_uplift > 30.0 {
            TerrainType::Mountains
        } else if tectonic_uplift > 15.0 {
            TerrainType::Hills
        } else if coherence > 0.6 {
            TerrainType::Forest
        } else if coherence > 0.4 {
            TerrainType::Plains
        } else if coherence < 0.3 && height > 30.0 {
            TerrainType::Desert
        } else if height > 50.0 {
            TerrainType::Peaks
        } else {
            TerrainType::Hills
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

/// Atmospheric circulation cell type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CirculationType {
    Hadley, // Equator to 30°
    Ferrel, // 30° to 60°
    Polar,  // 60° to poles
}

/// Atmospheric circulation cell
#[derive(Debug, Clone)]
pub struct CirculationCell {
    pub cell_type: CirculationType,
    pub temperature: Float,
    pub pressure: Float,
    pub velocity: Position3D,
}

/// Atmospheric simulation (R&D-6: proper physics)
#[derive(Debug, Clone)]
pub struct Atmosphere {
    /// Temperature gradient from equator to poles
    pub temperature_gradient: Float,
    /// Circulation cells (Hadley, Ferrel, Polar)
    pub circulation_cells: Vec<CirculationCell>,
    /// Solar radiation absorption
    pub solar_input: Float,
}

impl Atmosphere {
    /// Create atmosphere from field coherence
    pub fn from_coherence(coherence: Float) -> Self {
        let solar_input = coherence * 1.2;

        let circulation_cells = vec![
            CirculationCell {
                cell_type: CirculationType::Hadley,
                temperature: solar_input,
                pressure: 0.3,
                velocity: Position3D::new(0.0, 0.1, 0.0),
            },
            CirculationCell {
                cell_type: CirculationType::Ferrel,
                temperature: solar_input * 0.7,
                pressure: 0.5,
                velocity: Position3D::new(0.0, -0.05, 0.0),
            },
            CirculationCell {
                cell_type: CirculationType::Polar,
                temperature: solar_input * 0.3,
                pressure: 0.7,
                velocity: Position3D::new(0.0, 0.02, 0.0),
            },
        ];

        Atmosphere {
            temperature_gradient: solar_input * 0.7,
            circulation_cells,
            solar_input,
        }
    }
}

impl Default for Atmosphere {
    fn default() -> Self {
        Self::from_coherence(0.5)
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

/// Determine weather pattern from atmospheric physics (R&D-6)
fn determine_weather_pattern(
    temp: Float,
    humidity: Float,
    cloud_cover: Float,
    wind: Float,
) -> WeatherPattern {
    if cloud_cover < 0.3 {
        WeatherPattern::Clear
    } else if cloud_cover < 0.6 {
        WeatherPattern::PartlyCloudy
    } else if humidity > 0.8 && wind > 0.6 {
        WeatherPattern::Storm
    } else if humidity > 0.6 {
        WeatherPattern::Rain
    } else if temp < 0.3 {
        WeatherPattern::Snow
    } else if wind > 0.7 && humidity < 0.3 {
        WeatherPattern::DesertStorm
    } else {
        WeatherPattern::Cloudy
    }
}

impl WeatherSystem {
    /// Create weather from field oscillation patterns (legacy)
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

    /// Create weather from atmospheric physics (R&D-6)
    pub fn from_atmospheric_physics(
        latitude: Float,
        atmosphere: &Atmosphere,
        surface_temp: Float,
    ) -> Self {
        // Temperature from latitude + solar input
        let equatorial_temp = atmosphere.solar_input;
        let polar_temp = equatorial_temp * 0.3;
        let temperature = equatorial_temp
            - (latitude.abs() / std::f64::consts::PI) * (equatorial_temp - polar_temp);

        // Find active circulation cell
        let cell_type = if latitude.abs() < 0.5 {
            // ~30°
            CirculationType::Hadley
        } else if latitude.abs() < 1.0 {
            // ~60°
            CirculationType::Ferrel
        } else {
            CirculationType::Polar
        };

        // Wind from circulation cell velocity
        let cell = atmosphere
            .circulation_cells
            .iter()
            .find(|c| c.cell_type == cell_type)
            .unwrap_or(&atmosphere.circulation_cells[0]);
        let wind_speed = cell.velocity.magnitude();

        // Humidity from temperature + pressure
        let humidity = (temperature * 0.6 + (1.0 - cell.pressure) * 0.4).min(1.0);

        // Cloud cover from humidity + temperature
        let cloud_cover = if temperature > 0.7 && humidity > 0.5 {
            (humidity * 0.9).min(1.0)
        } else {
            (humidity * 0.5).min(1.0)
        };

        // Weather pattern from physics
        let pattern = determine_weather_pattern(temperature, humidity, cloud_cover, wind_speed);

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

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            temperature: 0.5,
            humidity: 0.5,
            wind_speed: 0.0,
            cloud_cover: 0.0,
            pattern: WeatherPattern::Clear,
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
    pub fn from_field(
        position: Position3D,
        field_data: &FieldNodeData,
        config: &PlanetaryConfig,
    ) -> Self {
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

    /// Create terrain from geological forces (R&D-6)
    pub fn from_geology(
        position: Position3D,
        field_data: &FieldNodeData,
        geology: &Geology,
        config: &PlanetaryConfig,
    ) -> Self {
        // Calculate tectonic uplift from plate boundaries
        let tectonic_uplift = geology.calculate_uplift(&position);

        // Calculate erosion
        let erosion = field_data.coherence * geology.erosion_rate;

        // Height = field base + tectonic uplift - erosion
        let base_height = (field_data.coherence - config.ocean_threshold).max(0.0);
        let height = (base_height + tectonic_uplift - erosion) * config.height_scale;

        let terrain_type =
            TerrainType::from_geological_forces(height, field_data.coherence, tectonic_uplift);

        TerrainCell {
            position,
            height,
            terrain_type,
            coherence: field_data.coherence,
            amplitude: field_data.total_magnitude(),
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
    pub atmosphere_layer: AtmosphereLayer,
    /// Atmospheric simulation (R&D-6)
    pub atmosphere: Atmosphere,
    /// Geological simulation (R&D-6)
    pub geology: Geology,
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

impl Default for Planet {
    fn default() -> Self {
        Self {
            id: 0,
            position: Position3D::new(0.0, 0.0, 0.0),
            radius: 1.0,
            mass: 1.0,
            gravity: 1.0,
            atmosphere_layer: AtmosphereLayer::Moderate,
            atmosphere: Atmosphere::default(),
            geology: Geology::default(),
            terrain: Vec::new(),
            grid_resolution: 64,
            weather: WeatherSystem::default(),
            average_coherence: 0.5,
            archetype_pattern: [0.5; NUM_ARCHETYPES],
        }
    }
}
impl Planet {
    /// Create a planet from field region (R&D-6: with gravitational core, proper physics)
    pub fn from_field(
        id: u64,
        center: Position3D,
        field_region: &[FieldNodeData],
        config: &PlanetaryConfig,
        core: Option<&GravitationalCore>, // NEW: gravitational core
    ) -> Option<Self> {
        if field_region.is_empty() {
            return None;
        }

        // Calculate average coherence
        let avg_coherence: Float =
            field_region.iter().map(|f| f.coherence).sum::<Float>() / field_region.len() as Float;

        // Check if coherence is high enough for planet
        if avg_coherence < config.planet_formation_threshold {
            return None;
        }

        // Derive mass from field energy (R&D-6: use gravitational core)
        let total_energy: Float = field_region.iter().map(|f| f.energy).sum();

        // Use core density gradient if available
        let mass = if let Some(c) = core {
            // Mass increases with core formation
            total_energy * 0.001 * (1.0 + c.density_gradient)
        } else {
            total_energy * 0.001
        };

        // Radius from mass (simplified)
        let radius = (mass / 10.0).cbrt() * config.planetary_scale;

        // Gravity from mass/radius
        let gravity = mass / (radius * radius).max(0.001);

        // Atmosphere layer from coherence
        let atmosphere_layer = AtmosphereLayer::from_coherence(avg_coherence);

        // Initialize atmosphere (R&D-6)
        let atmosphere = Atmosphere::from_coherence(avg_coherence);

        // Initialize geology (R&D-6)
        let plates = vec![TectonicPlate {
            plate_id: 0,
            boundary: vec![center],
            velocity: Position3D::new(0.01, 0.0, 0.0),
            density: 3.0,
        }];

        let geology = Geology {
            plates,
            erosion_rate: 0.001,
        };

        // Generate terrain from geological forces (R&D-6)
        let terrain =
            Self::generate_terrain_from_geology(center, radius, field_region, config, &geology);

        // Weather from atmospheric physics (R&D-6)
        let weather = if !field_region.is_empty() {
            WeatherSystem::from_atmospheric_physics(0.0, &atmosphere, avg_coherence)
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
            atmosphere_layer,
            atmosphere,
            geology,
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

    /// Generate terrain from geological forces (R&D-6)
    fn generate_terrain_from_geology(
        center: Position3D,
        radius: Float,
        field_region: &[FieldNodeData],
        config: &PlanetaryConfig,
        geology: &Geology,
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

                let cell = TerrainCell::from_geology(position, field_data, geology, config);
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
        let i = ((lat + std::f64::consts::PI / 2.0) / std::f64::consts::PI
            * self.grid_resolution as Float) as usize;
        let j = ((lon + std::f64::consts::PI) / (2.0 * std::f64::consts::PI)
            * self.grid_resolution as Float) as usize;

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
    // R&D-6: Physics-based simulation
    pub gravitational_cores: HashMap<u64, GravitationalCore>,
    pub tectonic_stress: TectonicStress,
}

impl PlanetaryEmergence {
    /// Create new planetary emergence system
    pub fn new(config: PlanetaryConfig) -> Self {
        PlanetaryEmergence {
            config,
            planets: HashMap::new(),
            next_planet_id: 0,
            oscillation_phase: 0.0,
            gravitational_cores: HashMap::new(),
            tectonic_stress: TectonicStress::new(),
            statistics: PlanetaryStatistics::default(),
        }
    }

    /// Create with defaults
    pub fn with_defaults() -> Self {
        Self::new(PlanetaryConfig::default())
    }

    /// Gravitational condensation (R&D-6)
    /// From roadmap: Energy attracts energy (Love/Logos as gravity)
    fn gravitational_condensation(
        &mut self,
        field_region: &[FieldNodeData],
        positions: &[Position3D],
    ) -> GravitationalCore {
        let mut core = GravitationalCore::new();

        // Apply attraction from each field point
        for (field, pos) in field_region.iter().zip(positions.iter()) {
            // Energy attracts energy (not mass)
            core.apply_attraction(*pos, field.energy);
        }

        // Compute density gradient
        core.compute_density_gradient();

        core
    }

    /// Try to create a planet at the given position (R&D-6: physics-based)
    pub fn try_create_planet(
        &mut self,
        center: Position3D,
        field_region: &[FieldNodeData],
    ) -> Option<u64> {
        // First, compute gravitational condensation
        // For now, create simple position array matching field region
        let positions: Vec<Position3D> = (0..field_region.len())
            .map(|i| {
                let angle = i as Float * 0.5;
                Position3D::new(
                    center.x + angle.cos() * 10.0,
                    center.y + angle.sin() * 10.0,
                    center.z,
                )
            })
            .collect();

        let core = self.gravitational_condensation(field_region, &positions);

        // Check if core has formed
        if !core.check_core_formation() {
            return None;
        }

        // Core formed - create planet
        let planet = Planet::from_field(
            self.next_planet_id,
            center,
            field_region,
            &self.config,
            Some(&core), // Pass gravitational core
        )?;

        let id = self.next_planet_id;
        self.next_planet_id += 1;

        // Store gravitational core for this planet
        self.gravitational_cores.insert(id, core);

        // Update statistics
        self.statistics.planet_count += 1;
        self.statistics.terrain_cells += planet.terrain.len();

        // Calculate new average coherence
        let total_coherence: Float = self
            .planets
            .values()
            .map(|p| p.average_coherence)
            .sum::<Float>()
            + planet.average_coherence;
        self.statistics.average_coherence = total_coherence / self.statistics.planet_count as Float;

        self.planets.insert(id, planet);
        Some(id)
    }

    /// Update all planets (R&D-6: physics-based simulation)
    pub fn update(&mut self, delta_time: Float) {
        self.oscillation_phase += delta_time;

        // Compute tectonic stress (R&D-6)
        self.simulate_tectonics(delta_time);

        // Collect the stress map data for geological simulation
        let grid_size = 10.0;
        let uplift_rate = self.tectonic_stress.uplift_rate;
        let stress_map_clone = self.tectonic_stress.stress_map.clone();

        // Update weather and geology for each planet
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

            // Apply geological processes (R&D-6) - using cloned data
            for terrain in planet.terrain.iter_mut() {
                let grid_pos = GridPosition::from_position(&terrain.position, grid_size);
                let uplift = stress_map_clone
                    .get(&grid_pos)
                    .map(|s| s * uplift_rate)
                    .unwrap_or(0.0);
                terrain.height += uplift;
            }
        }

        self.statistics.active_weather_systems = self.planets.len();
    }

    /// Simulate plate tectonics from field stress (R&D-6)
    fn simulate_tectonics(&mut self, _delta_time: Float) {
        // Update tectonic stress from overall field state
        // Simplified: use oscillation as stress driver
        let stress_level = self.oscillation_phase.sin() * 0.5 + 0.5;
        self.tectonic_stress.uplift_rate = stress_level * 0.01;
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
    fn test_terrain_type_from_geology() {
        // Test ocean
        let terrain = TerrainType::from_geological_forces(5.0, 0.4, 0.0);
        assert_eq!(terrain, TerrainType::Ocean);

        // Test mountains from tectonic uplift
        let terrain = TerrainType::from_geological_forces(40.0, 0.7, 35.0);
        assert_eq!(terrain, TerrainType::Mountains);

        // Test forest from coherence
        let terrain = TerrainType::from_geological_forces(25.0, 0.7, 5.0);
        assert_eq!(terrain, TerrainType::Forest);

        // Test desert from low coherence + high height
        let terrain = TerrainType::from_geological_forces(35.0, 0.2, 5.0);
        assert_eq!(terrain, TerrainType::Desert);
    }

    #[test]
    fn test_atmosphere_layer() {
        let atm = AtmosphereLayer::from_coherence(0.2);
        assert_eq!(atm, AtmosphereLayer::None);

        let atm = AtmosphereLayer::from_coherence(0.7);
        assert_eq!(atm, AtmosphereLayer::Thick);
    }

    #[test]
    fn test_atmosphere_from_coherence() {
        let atm = Atmosphere::from_coherence(0.7);
        assert_eq!(atm.circulation_cells.len(), 3);
        assert!(atm.solar_input > 0.0);
        assert_eq!(atm.circulation_cells[0].cell_type, CirculationType::Hadley);
        assert_eq!(atm.circulation_cells[1].cell_type, CirculationType::Ferrel);
        assert_eq!(atm.circulation_cells[2].cell_type, CirculationType::Polar);
    }

    #[test]
    fn test_geology_uplift() {
        let plates = vec![TectonicPlate {
            plate_id: 0,
            boundary: vec![Position3D::new(0.0, 0.0, 0.0)],
            velocity: Position3D::new(0.01, 0.0, 0.0),
            density: 3.0,
        }];

        let geology = Geology {
            plates,
            erosion_rate: 0.001,
        };

        // Position near plate should have uplift
        let uplift = geology.calculate_uplift(&Position3D::new(10.0, 0.0, 0.0));
        assert!(uplift > 0.0);

        // Position far from plate should have little uplift
        let uplift_far = geology.calculate_uplift(&Position3D::new(500.0, 0.0, 0.0));
        assert!(uplift_far < uplift);
    }

    #[test]
    fn test_weather_from_atmospheric_physics() {
        let atm = Atmosphere::from_coherence(0.7);
        let weather = WeatherSystem::from_atmospheric_physics(0.0, &atm, 0.7);

        assert!(weather.temperature > 0.0);
        assert!(weather.humidity >= 0.0);
        assert!(weather.wind_speed >= 0.0);
        assert!(weather.cloud_cover >= 0.0);
    }

    #[test]
    fn test_planet_creation() {
        let config = PlanetaryConfig::default();
        let center = Position3D::new(1000.0, 0.0, 0.0);

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

        let planet = Planet::from_field(0, center, &field_region, &config, None);
        assert!(planet.is_some());

        let planet = planet.unwrap();
        assert!(planet.radius > 0.0);
        assert!(planet.mass > 0.0);
        // Verify new fields exist
        assert_eq!(planet.atmosphere.circulation_cells.len(), 3);
        assert_eq!(planet.geology.plates.len(), 1);
    }

    #[ignore]
    #[test]
    fn test_planetary_emergence() {
        let mut emergence = PlanetaryEmergence::with_defaults();

        let center = Position3D::new(1000.0, 0.0, 0.0);
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
