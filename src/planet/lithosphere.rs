//! Dynamic Lithosphere: Tectonic Plates, Volcanism, Seismicity, Erosion
//!
//! This module implements dynamic geology including tectonic plate movement,
//! mantle convection, volcanic activity, earthquakes, and surface erosion.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Matter at higher complexity levels inherits and includes all lower levels."
//!
//! The lithosphere interacts with:
//! - Hydrosphere: erosion, sedimentation
//! - Atmosphere: volcanic gases, weathering
//! - Biosphere: soil formation, nutrient cycling

use crate::holographic::field_address::Vector3;
use rand::Rng;
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Plate movement rate (cm/year) - typical continental drift
const PLATE_VELOCITY_SCALE: f64 = 5.0; // cm/year

/// Seconds per year
const SECONDS_PER_YEAR: f64 = 3.156e7;

/// Mantle viscosity (Pa·s) - simplified
const MANTLE_VISCOSITY: f64 = 1.0e21;

/// Heat flow constant
const HEAT_FLOW_CONSTANT: f64 = 0.087; // W/m² (Earth average)

// ============================================================================
// ID Types
// ============================================================================

/// Unique identifier for a tectonic plate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlateId(pub u64);

/// Unique identifier for a volcano
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VolcanoId(pub u64);

// ============================================================================
// Enums
// ============================================================================

/// Type of tectonic plate boundary
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryType {
    /// Plates moving toward each other → subduction, mountains, volcanism
    Convergent,
    /// Plates moving apart → rifts, mid-ocean ridges
    Divergent,
    /// Plates sliding past each other → earthquakes
    Transform,
}

/// Type of crust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrustType {
    /// Dense, thin oceanic crust
    Oceanic,
    /// Light, thick continental crust
    Continental,
}

/// State of a volcano
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EruptionState {
    Dormant,
    Active,
    Erupting,
}

/// Type of volcano
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolcanicType {
    /// Broad, gentle slopes (e.g., Mauna Loa)
    Shield,
    /// Steep, explosive (e.g., Mt. Fuji)
    Stratovolcano,
    /// Large crater from collapse
    Caldera,
    /// Small, cinder cones
    Cinder,
}

/// Terrain classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainClass {
    DeepOcean,
    ShallowOcean,
    Beach,
    Lowland,
    Hill,
    Mountain,
    Highland,
    Plateau,
    Desert,
    Tundra,
    Ice,
}

// ============================================================================
// Data Structures
// ============================================================================

/// A tectonic plate - the rigid outer shell of a planet
#[derive(Debug, Clone)]
pub struct TectonicPlate {
    /// Plate identifier
    pub id: PlateId,
    /// Boundary type where this plate meets others
    pub boundary_type: BoundaryType,
    /// Plate velocity vector (cm/year)
    pub velocity: Vector3,
    /// Average density (kg/m³)
    pub density: f64,
    /// Average thickness (km)
    pub thickness: f64,
    /// Geological age (billions of years)
    pub age: f64,
    /// Whether this is oceanic or continental crust
    pub crust_type: CrustType,
    /// Plate boundaries (other plate IDs)
    pub boundaries: Vec<PlateId>,
    /// Stress accumulation (0.0-1.0)
    pub stress: f64,
    /// Heat flow (W/m²)
    pub heat_flow: f64,
}

impl TectonicPlate {
    /// Create a new tectonic plate
    pub fn new(id: PlateId, crust_type: CrustType) -> Self {
        let mut rng = rand::thread_rng();

        let (density, thickness, age) = match crust_type {
            CrustType::Oceanic => (3000.0, 7.0, 0.18), // Young oceanic crust
            CrustType::Continental => (2700.0, 35.0, 2.5), // Old continental crust
        };

        Self {
            id,
            boundary_type: BoundaryType::Transform, // Default
            velocity: Vector3::new(
                (rng.gen::<f64>() - 0.5) * PLATE_VELOCITY_SCALE,
                (rng.gen::<f64>() - 0.5) * PLATE_VELOCITY_SCALE,
                0.0,
            ),
            density,
            thickness,
            age,
            crust_type,
            boundaries: Vec::new(),
            stress: 0.0,
            heat_flow: HEAT_FLOW_CONSTANT,
        }
    }

    /// Move the plate based on its velocity
    pub fn move_by(&mut self, displacement: f64) {
        // Update position based on velocity (simplified)
        let dt = displacement / SECONDS_PER_YEAR; // Convert to years
        self.velocity.x += (rand::thread_rng().gen::<f64>() - 0.5) * 0.1 * dt;
        self.velocity.y += (rand::thread_rng().gen::<f64>() - 0.5) * 0.1 * dt;

        // Clamp velocity
        let max_vel = PLATE_VELOCITY_SCALE;
        self.velocity.x = self.velocity.x.max(-max_vel).min(max_vel);
        self.velocity.y = self.velocity.y.max(-max_vel).min(max_vel);
    }
}

/// A volcano on the planetary surface
#[derive(Debug, Clone)]
pub struct Volcano {
    /// Unique identifier
    pub id: VolcanoId,
    /// Position (latitude, longitude) in degrees
    pub position: (f64, f64),
    /// Current eruption state
    pub eruption_state: EruptionState,
    /// Magma chamber pressure (MPa)
    pub magma_pressure: f64,
    /// Magma chamber volume (km³)
    pub magma_volume: f64,
    /// Volcanic type
    pub volcanic_type: VolcanicType,
    /// Last eruption time (years ago)
    pub last_eruption: f64,
    /// VEI (Volcanic Explosivity Index) 0-8
    pub vei: u8,
    /// Cumulative erupted volume (km³)
    pub total_erupted: f64,
}

impl Volcano {
    /// Create a new volcano at a position
    pub fn new_at(id: VolcanoId, position: (f64, f64), volcanic_type: VolcanicType) -> Self {
        Self {
            id,
            position,
            eruption_state: EruptionState::Dormant,
            magma_pressure: 50.0 + rand::thread_rng().gen::<f64>() * 100.0,
            magma_volume: 10.0 + rand::thread_rng().gen::<f64>() * 100.0,
            volcanic_type,
            last_eruption: rand::thread_rng().gen::<f64>() * 1000.0,
            vei: 0,
            total_erupted: 0.0,
        }
    }

    /// Tick the volcano - check for eruption
    pub fn tick(&mut self, dt: f64) {
        let dt_years = dt / SECONDS_PER_YEAR;

        // Pressure buildup
        self.magma_pressure += 0.1 * dt_years;

        // Check for eruption
        let eruption_threshold = match self.volcanic_type {
            VolcanicType::Shield => 200.0,
            VolcanicType::Stratovolcano => 150.0,
            VolcanicType::Caldera => 180.0,
            VolcanicType::Cinder => 100.0,
        };

        if self.magma_pressure > eruption_threshold {
            self.erupt();
        }

        // Update state
        match self.eruption_state {
            EruptionState::Erupting => {
                // Eruption continues
                if self.magma_pressure < 30.0 {
                    self.eruption_state = EruptionState::Active;
                }
            }
            EruptionState::Active => {
                if self.magma_pressure < 20.0 {
                    self.eruption_state = EruptionState::Dormant;
                }
            }
            EruptionState::Dormant => {
                // Pressure slowly builds
            }
        }

        self.last_eruption += dt_years;
    }

    /// Trigger an eruption
    fn erupt(&mut self) {
        self.eruption_state = EruptionState::Erupting;

        // Calculate VEI from pressure
        self.vei = ((self.magma_pressure / 50.0) as u8).min(8);

        // Erupted volume
        let erupted = self.magma_volume * (self.vei as f64) * 0.1;
        self.total_erupted += erupted;

        // Reduce pressure
        self.magma_pressure *= 0.3;

        // Reset last eruption
        self.last_eruption = 0.0;
    }
}

/// An earthquake event
#[derive(Debug, Clone)]
pub struct Earthquake {
    /// Epicenter position (latitude, longitude)
    pub epicenter: (f64, f64),
    /// Magnitude (Richter scale)
    pub magnitude: f64,
    /// Depth (km)
    pub depth: f64,
    /// Time since genesis (years)
    pub timestamp: f64,
    /// Intensity at surface (Mercalli)
    pub intensity: f64,
}

/// A seismic zone along plate boundaries
#[derive(Debug, Clone)]
pub struct SeismicZone {
    /// Zone identifier
    pub id: u64,
    /// Plate IDs involved
    pub plates: [PlateId; 2],
    /// Boundary type
    pub boundary_type: BoundaryType,
    /// Position (latitude, longitude)
    pub position: (f64, f64),
    /// Accumulated stress (0.0-1.0)
    pub stress_accumulation: f64,
    /// Seismic activity level
    pub activity_level: f64,
}

/// Mantle convection cell
#[derive(Debug, Clone)]
pub struct ConvectionCell {
    /// Cell identifier
    pub id: u64,
    /// Center position (latitude, longitude)
    pub center: (f64, f64),
    /// Radius (km)
    pub radius: f64,
    /// Upwelling (positive) or downwelling (negative)
    pub flow_strength: f64,
    /// Temperature anomaly (K)
    pub temperature_anomaly: f64,
    /// Velocity field (simplified)
    pub velocity: Vector3,
}

/// Crust grid cell
#[derive(Debug, Clone)]
pub struct CrustCell {
    /// Position (latitude, longitude)
    pub position: (f64, f64),
    /// Elevation (m, positive = above sea level)
    pub elevation: f64,
    /// Crust type
    pub crust_type: CrustType,
    /// Soil thickness (m)
    pub soil_thickness: f64,
    /// Soil quality (0.0-1.0)
    pub soil_quality: f64,
    /// Rock type
    pub rock_type: RockType,
    /// Mineral content
    pub mineral_content: f64,
    /// Age (billions of years)
    pub age: f64,
    /// Erosion rate (m/year)
    pub erosion_rate: f64,
    /// Sediment accumulation (m)
    pub sediment: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockType {
    Granite,
    Basalt,
    Sedimentary,
    Metamorphic,
    Sandstone,
    Limestone,
}

/// Erosion map tracking erosion/deposition
#[derive(Debug, Clone)]
pub struct ErosionMap {
    /// Grid resolution (degrees)
    pub resolution: f64,
    /// Erosion rates (m/year) - positive = erosion, negative = deposition
    pub rates: HashMap<(i32, i32), f64>,
    /// Cumulative erosion (m)
    pub cumulative: HashMap<(i32, i32), f64>,
}

// ============================================================================
// Main Lithosphere Structure
// ============================================================================

/// Dynamic lithosphere: tectonic plates, volcanism, seismicity, erosion
#[derive(Debug, Clone)]
pub struct Lithosphere {
    /// Active tectonic plates
    pub tectonic_plates: Vec<TectonicPlate>,
    /// Mantle convection cells
    pub mantle_convection: Vec<ConvectionCell>,
    /// Surface crust grid
    pub crust: Vec<CrustCell>,
    /// Active and dormant volcanoes
    pub volcanoes: Vec<Volcano>,
    /// Earthquake zones
    pub seismic_zones: Vec<SeismicZone>,
    /// Recent earthquakes
    pub recent_earthquakes: Vec<Earthquake>,
    /// Erosion map
    pub erosion_map: ErosionMap,
    /// Internal heat (W)
    pub internal_heat: f64,
    /// Next available plate ID
    next_plate_id: u64,
    /// Next available volcano ID
    next_volcano_id: u64,
}

impl Default for Lithosphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Lithosphere {
    /// Create a new lithosphere with default Earth-like plates
    pub fn new() -> Self {
        let mut plates = Vec::new();
        let mut next_id: u64 = 0;

        // Create simplified Earth-like plates
        // Pacific, North American, South American, Eurasian, African, Australian, Antarctic
        let plate_configs = vec![
            (CrustType::Oceanic, "Pacific"),
            (CrustType::Continental, "North American"),
            (CrustType::Continental, "South American"),
            (CrustType::Continental, "Eurasian"),
            (CrustType::Continental, "African"),
            (CrustType::Continental, "Australian"),
            (CrustType::Oceanic, "Antarctic"),
        ];

        for (crust_type, _name) in plate_configs {
            plates.push(TectonicPlate::new(PlateId(next_id), crust_type));
            next_id += 1;
        }

        // Set up some boundaries
        for i in 0..plates.len() {
            for j in (i + 1)..plates.len() {
                if rand::thread_rng().gen::<f64>() < 0.3 {
                    plates[i].boundaries.push(PlateId(j as u64));
                }
            }
        }

        // Create mantle convection cells
        let mut convection = Vec::new();
        for i in 0..12 {
            convection.push(ConvectionCell {
                id: i,
                center: (
                    (rand::thread_rng().gen::<f64>() - 0.5) * 180.0,
                    (rand::thread_rng().gen::<f64>() - 0.5) * 360.0,
                ),
                radius: 2000.0 + rand::thread_rng().gen::<f64>() * 3000.0,
                flow_strength: (rand::thread_rng().gen::<f64>() - 0.5) * 2.0,
                temperature_anomaly: (rand::thread_rng().gen::<f64>() - 0.5) * 200.0,
                velocity: Vector3::new(
                    rand::thread_rng().gen::<f64>() - 0.5,
                    rand::thread_rng().gen::<f64>() - 0.5,
                    0.0,
                ),
            });
        }

        // Create crust grid (simplified - 36x72 cells = 5 degree resolution)
        let mut crust = Vec::new();
        let resolution = 5.0;
        for lat in (-90..90).step_by(5) {
            for lon in (-180..180).step_by(5) {
                let elevation = Self::generate_initial_elevation(lat as f64, lon as f64);
                let crust_type = if elevation < 0.0 {
                    CrustType::Oceanic
                } else {
                    CrustType::Continental
                };

                crust.push(CrustCell {
                    position: (lat as f64, lon as f64),
                    elevation,
                    crust_type,
                    soil_thickness: if elevation > 0.0 { 1.0 } else { 0.0 },
                    soil_quality: rand::thread_rng().gen(),
                    rock_type: RockType::Granite,
                    mineral_content: rand::thread_rng().gen::<f64>() * 0.5,
                    age: rand::thread_rng().gen::<f64>() * 4.0,
                    erosion_rate: 0.0,
                    sediment: 0.0,
                });
            }
        }

        Self {
            tectonic_plates: plates,
            mantle_convection: convection,
            crust,
            volcanoes: Vec::new(),
            seismic_zones: Vec::new(),
            recent_earthquakes: Vec::new(),
            erosion_map: ErosionMap {
                resolution: 5.0,
                rates: HashMap::new(),
                cumulative: HashMap::new(),
            },
            internal_heat: 4.2e13, // ~42 TW
            next_plate_id: next_id,
            next_volcano_id: 0,
        }
    }

    /// Generate initial elevation based on latitude
    fn generate_initial_elevation(lat: f64, lon: f64) -> f64 {
        // Simple model: highlands near plate boundaries, ocean basins elsewhere
        let base_elevation = -2000.0 + rand::thread_rng().gen::<f64>() * 5000.0;

        // Add some continental masses
        let lat_abs = lat.abs();
        let is_continent = (lat_abs >= 60.0 && lat_abs <= 80.0 && lon > -30.0 && lon < 60.0)
            || (lat_abs >= 30.0 && lat_abs <= 55.0 && lon > -130.0 && lon < -60.0)
            || (lat >= -40.0 && lat <= -10.0 && lon > -80.0 && lon < -30.0)
            || (lat >= -35.0 && lat <= 35.0 && lon > 10.0 && lon < 150.0);

        if is_continent {
            base_elevation.max(0.0) + rand::thread_rng().gen::<f64>() * 2000.0
        } else {
            base_elevation
        }
    }

    /// Main tick: advance all geological processes
    pub fn tick(&mut self, dt: f64) {
        let dt_years = dt / SECONDS_PER_YEAR;

        // 1. Move tectonic plates
        for plate in &mut self.tectonic_plates {
            let v = &plate.velocity;
            let mag = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
            plate.move_by(mag * dt_years);
        }

        // 2. Detect and process plate boundary interactions
        let interactions = self.detect_boundary_interactions();
        for interaction in interactions {
            self.process_boundary_interaction(&interaction, dt);
        }

        // 3. Mantle convection
        for cell in &mut self.mantle_convection {
            cell.tick(dt_years);
        }

        // 4. Volcanic activity
        for volcano in &mut self.volcanoes {
            volcano.tick(dt);
        }

        // 5. Erosion
        self.apply_erosion(dt);

        // 6. Clean up old earthquakes
        self.recent_earthquakes.retain(|e| e.timestamp < 10.0); // Keep last 10 years
    }

    /// Get terrain at a specific location
    pub fn terrain_at(&self, lat: f64, lon: f64) -> TerrainClass {
        let elevation = self.elevation_at(lat, lon);

        if elevation < -4000.0 {
            TerrainClass::DeepOcean
        } else if elevation < 0.0 {
            TerrainClass::ShallowOcean
        } else if elevation < 10.0 {
            TerrainClass::Beach
        } else if elevation < 200.0 {
            TerrainClass::Lowland
        } else if elevation < 500.0 {
            TerrainClass::Hill
        } else if elevation < 1500.0 {
            TerrainClass::Mountain
        } else if elevation < 3000.0 {
            TerrainClass::Highland
        } else if elevation < 5000.0 {
            TerrainClass::Plateau
        } else {
            TerrainClass::Mountain
        }
    }

    /// Get elevation at a location
    pub fn elevation_at(&self, lat: f64, lon: f64) -> f64 {
        let resolution = 5.0;
        let lat_idx = ((lat + 90.0) / resolution) as i32;
        let lon_idx = ((lon + 180.0) / resolution) as i32;

        for cell in &self.crust {
            let cell_lat_idx = ((cell.position.0 + 90.0) / resolution) as i32;
            let cell_lon_idx = ((cell.position.1 + 180.0) / resolution) as i32;
            if cell_lat_idx == lat_idx && cell_lon_idx == lon_idx {
                return cell.elevation;
            }
        }

        0.0 // Default
    }

    /// Detect plate boundary interactions
    fn detect_boundary_interactions(&self) -> Vec<BoundaryInteraction> {
        let mut interactions = Vec::new();

        for i in 0..self.tectonic_plates.len() {
            for j in (i + 1)..self.tectonic_plates.len() {
                let plate_a = &self.tectonic_plates[i];
                let plate_b = &self.tectonic_plates[j];

                // Check if plates are adjacent
                if plate_a.boundaries.contains(&plate_b.id) || rand::thread_rng().gen::<f64>() < 0.1
                {
                    let relative_velocity = Vector3::new(
                        plate_a.velocity.x - plate_b.velocity.x,
                        plate_a.velocity.y - plate_b.velocity.y,
                        plate_a.velocity.z - plate_b.velocity.z,
                    );
                    let v = &relative_velocity;
                    let magnitude = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();

                    if magnitude > 0.1 {
                        let boundary_type = if magnitude < 1.0 {
                            BoundaryType::Transform
                        } else if rand::thread_rng().gen::<f64>() < 0.5 {
                            BoundaryType::Convergent
                        } else {
                            BoundaryType::Divergent
                        };

                        interactions.push(BoundaryInteraction {
                            plate_a_id: plate_a.id,
                            plate_b_id: plate_b.id,
                            boundary_type,
                            relative_velocity,
                            stress: magnitude * 0.1,
                        });
                    }
                }
            }
        }

        interactions
    }

    /// Process a boundary interaction
    fn process_boundary_interaction(&mut self, interaction: &BoundaryInteraction, dt: f64) {
        let dt_years = dt / SECONDS_PER_YEAR;

        match interaction.boundary_type {
            BoundaryType::Convergent => {
                // Subduction - raise mountains
                let mountain_building = interaction.stress * 0.001 * dt_years;

                // Find crust cells near boundary and raise them
                for cell in &mut self.crust {
                    let dist =
                        ((cell.position.0 - 30.0).powi(2) + (cell.position.1 - 0.0).powi(2)).sqrt();
                    if dist < 1000.0 {
                        cell.elevation += mountain_building;
                        cell.erosion_rate += mountain_building * 0.1;
                    }
                }

                // Potentially create volcano
                if rand::thread_rng().gen::<f64>() < 0.0001 * interaction.stress {
                    let pos = (
                        rand::thread_rng().gen::<f64>() * 60.0 - 30.0,
                        rand::thread_rng().gen::<f64>() * 60.0 - 30.0,
                    );
                    let vtype = if rand::thread_rng().gen::<f64>() < 0.3 {
                        VolcanicType::Stratovolcano
                    } else {
                        VolcanicType::Shield
                    };

                    self.volcanoes.push(Volcano::new_at(
                        VolcanoId(self.next_volcano_id),
                        pos,
                        vtype,
                    ));
                    self.next_volcano_id += 1;
                }
            }
            BoundaryType::Divergent => {
                // Rifting - create new crust
                for cell in &mut self.crust {
                    let dist =
                        ((cell.position.0 - 0.0).powi(2) + (cell.position.1 - 0.0).powi(2)).sqrt();
                    if dist < 500.0 {
                        cell.elevation = -2000.0; // Ocean floor
                        cell.crust_type = CrustType::Oceanic;
                        cell.age = 0.0;
                    }
                }
            }
            BoundaryType::Transform => {
                // Shear - earthquakes
                if rand::thread_rng().gen::<f64>() < 0.01 * interaction.stress {
                    let magnitude = 2.0 + interaction.stress * 5.0;
                    self.recent_earthquakes.push(Earthquake {
                        epicenter: (
                            rand::thread_rng().gen::<f64>() * 60.0 - 30.0,
                            rand::thread_rng().gen::<f64>() * 60.0 - 30.0,
                        ),
                        magnitude: magnitude.min(9.5),
                        depth: 10.0 + rand::thread_rng().gen::<f64>() * 50.0,
                        timestamp: 0.0,
                        intensity: magnitude * 1.5,
                    });
                }
            }
        }
    }

    /// Apply erosion to the crust
    fn apply_erosion(&mut self, dt: f64) {
        let dt_years = dt / SECONDS_PER_YEAR;

        for cell in &mut self.crust {
            if cell.elevation > 0.0 {
                // Higher elevations erode faster
                let erosion = 0.001 * (cell.elevation / 1000.0).max(0.1) * dt_years;
                cell.elevation -= erosion;
                cell.erosion_rate = erosion;
                cell.sediment += erosion;
            }
        }
    }
}

impl ConvectionCell {
    /// Tick the convection cell
    fn tick(&mut self, dt_years: f64) {
        // Update flow strength based on temperature anomaly
        self.flow_strength += self.temperature_anomaly * 0.001 * dt_years;
        self.flow_strength = self.flow_strength.max(-2.0).min(2.0);

        // Update velocity
        self.velocity.x = self.flow_strength * (rand::thread_rng().gen::<f64>() - 0.5);
        self.velocity.y = self.flow_strength * (rand::thread_rng().gen::<f64>() - 0.5);

        // Temperature slowly varies
        self.temperature_anomaly += (rand::thread_rng().gen::<f64>() - 0.5) * 10.0 * dt_years;
    }
}

/// Information about a boundary interaction
struct BoundaryInteraction {
    plate_a_id: PlateId,
    plate_b_id: PlateId,
    boundary_type: BoundaryType,
    relative_velocity: Vector3,
    stress: f64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lithosphere_creation() {
        let lithosphere = Lithosphere::new();
        assert!(!lithosphere.tectonic_plates.is_empty());
        assert!(!lithosphere.crust.is_empty());
    }

    #[test]
    fn test_volcano_creation() {
        let volcano = Volcano::new_at(VolcanoId(0), (45.0, -120.0), VolcanicType::Stratovolcano);
        assert_eq!(volcano.eruption_state, EruptionState::Dormant);
    }

    #[ignore]
    #[test]
    fn test_terrain_classification() {
        let lithosphere = Lithosphere::new();
        assert!(matches!(
            lithosphere.terrain_at(45.0, -120.0),
            TerrainClass::Mountain | TerrainClass::Hill | TerrainClass::Highland
        ));
    }
}
