//! Dynamic Hydrosphere: Water Cycle, Oceans, Rivers, Lakes, Glaciers
//!
//! This module implements the dynamic water system including evaporation,
//! precipitation, river flow, ocean currents, groundwater, and glaciers.
//!
//! The hydrosphere interacts with:
//! - Atmosphere: evaporation, precipitation
//! - Lithosphere: erosion, sediment transport, groundwater
//! - Biosphere: water needed for life

use rand::Rng;

// ============================================================================
// Constants
// ============================================================================

/// Seconds per day
const SECONDS_PER_DAY: f64 = 86400.0;

/// Earth's ocean volume (m³)
/// Note: Used for global water budget calculations
const OCEAN_VOLUME: f64 = 1.332e18;

/// Average ocean depth (m)
const AVG_OCEAN_DEPTH: f64 = 3682.0;

/// Sea level (reference)
const SEA_LEVEL: f64 = 0.0;

/// Base evaporation rate at 20°C (kg/m²/s)
const BASE_EVAPORATION: f64 = 0.001;

/// Snow line altitude (m)
/// Note: Used for glacial formation calculations
const SNOW_LINE: f64 = 3000.0;

// ============================================================================
// Data Structures
// ============================================================================

/// A river system
#[derive(Debug, Clone)]
pub struct River {
    /// River ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Source position (lat, lon)
    pub source: (f64, f64),
    /// Mouth position (lat, lon)
    pub mouth: (f64, f64),
    /// Flow rate (m³/s)
    pub flow_rate: f64,
    /// Length (km)
    pub length: f64,
    /// Basin area (km²)
    pub basin_area: f64,
    /// Current water level (0.0-1.0)
    pub water_level: f64,
    /// Pollution level (0.0-1.0)
    pub pollution: f64,
    /// Active (not dried up)
    pub is_active: bool,
}

impl River {
    /// Create a new river
    pub fn new(id: u64, name: &str, source: (f64, f64), mouth: (f64, f64)) -> Self {
        let length = ((source.0 - mouth.0).powi(2) + (source.1 - mouth.1).powi(2)).sqrt() * 111.0; // Approx km per degree
        Self {
            id,
            name: name.to_string(),
            source,
            mouth,
            flow_rate: 100.0 + rand::thread_rng().gen::<f64>() * 500.0,
            length,
            basin_area: length * 1000.0,
            water_level: 0.8,
            pollution: 0.1,
            is_active: true,
        }
    }

    /// Update river flow based on precipitation
    pub fn flow(&mut self, precipitation: f64, dt: f64) {
        if !self.is_active {
            return;
        }

        // Flow increases with precipitation
        let precip_factor = precipitation / 10.0; // Normalize
        self.flow_rate += precip_factor * self.basin_area * 0.001 * dt;
        self.flow_rate *= 0.999; // Natural decrease

        // Update water level
        self.water_level = (self.flow_rate / 1000.0).min(1.0);

        // Check for drying
        if self.flow_rate < 1.0 {
            self.is_active = false;
        }
    }
}

/// A lake
#[derive(Debug, Clone)]
pub struct Lake {
    /// Lake ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Center position (lat, lon)
    pub position: (f64, f64),
    /// Surface area (km²)
    pub area: f64,
    /// Volume (km³)
    pub volume: f64,
    /// Average depth (m)
    pub avg_depth: f64,
    /// Water level (m above/below reference)
    pub water_level: f64,
    /// Salinity (parts per thousand)
    pub salinity: f64,
    /// Frozen (ice covered)
    pub frozen: bool,
    /// Pollution level (0.0-1.0)
    pub pollution: f64,
}

impl Lake {
    /// Create a new lake
    pub fn new(id: u64, name: &str, position: (f64, f64), area: f64) -> Self {
        let avg_depth = (area / 10.0).sqrt() * 10.0;
        Self {
            id,
            name: name.to_string(),
            position,
            area,
            volume: area * avg_depth / 1000.0, // Convert to km³
            avg_depth,
            water_level: 0.0,
            salinity: 0.5,
            frozen: false,
            pollution: 0.05,
        }
    }

    /// Update lake state
    pub fn tick(&mut self, temperature: f64, precipitation: f64, dt: f64) {
        // Evaporation
        if temperature > 0.0 {
            let evap_rate = BASE_EVAPORATION * (temperature / 293.0) * self.area * 1e6 * dt;
            self.volume -= evap_rate / 1e9; // Convert to km³
        }

        // Precipitation inflow
        self.volume += precipitation * self.area * 1e6 * dt / 1e9;

        // Ice formation
        if temperature < 273.15 {
            self.frozen = true;
        } else if temperature > 275.15 {
            self.frozen = false;
        }

        // Clamp volume
        self.volume = self.volume.max(0.001);
    }
}

/// A glacier
#[derive(Debug, Clone)]
pub struct Glacier {
    /// Glacier ID
    pub id: u64,
    /// Name
    pub name: String,
    /// Position (lat, lon)
    pub position: (f64, f64),
    /// Area (km²)
    pub area: f64,
    /// Volume (km³)
    pub volume: f64,
    /// Length (km)
    pub length: f64,
    /// Elevation range (m)
    pub elevation_range: (f64, f64),
    /// Melt rate (m/year)
    pub melt_rate: f64,
    /// Accumulation rate (m/year)
    pub accumulation_rate: f64,
    /// Advance (+) or retreat (-) rate (m/year)
    pub advance_rate: f64,
}

impl Glacier {
    /// Create a new glacier
    pub fn new(
        id: u64,
        name: &str,
        position: (f64, f64),
        area: f64,
        elevation_range: (f64, f64),
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            position,
            area,
            volume: area * 100.0 / 1000.0, // Assume 100m avg thickness
            length: (area / 10.0).sqrt(),
            elevation_range,
            melt_rate: 0.5,
            accumulation_rate: 1.0,
            advance_rate: 0.5,
        }
    }

    /// Update glacier state
    pub fn update(&mut self, temperature: f64, precipitation: f64, dt: f64) {
        let dt_years = dt / (365.0 * SECONDS_PER_DAY);

        // Temperature effect on melt
        if temperature > 273.15 {
            let melt_factor = (temperature - 273.15) / 10.0;
            self.melt_rate = melt_factor.max(0.1);
        } else {
            self.melt_rate = 0.0;
        }

        // Precipitation effect on accumulation (snow)
        if temperature < 275.15 {
            self.accumulation_rate = precipitation * 0.1;
        }

        // Net advance/retreat
        self.advance_rate = self.accumulation_rate - self.melt_rate;

        // Update volume
        let volume_change = (self.accumulation_rate - self.melt_rate) * self.area * dt_years;
        self.volume = (self.volume + volume_change).max(0.001);
    }
}

/// An ocean current
#[derive(Debug, Clone)]
pub struct OceanCurrent {
    /// Current name
    pub name: String,
    /// Path waypoints (lat, lon)
    pub path: Vec<(f64, f64)>,
    /// Velocity (m/s)
    pub velocity: f64,
    /// Temperature (°C)
    pub temperature: f64,
    /// Salinity (ppt)
    pub salinity: f64,
    /// Type
    pub current_type: CurrentType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentType {
    /// Wind-driven surface current
    Surface,
    /// Thermohaline deep current
    DeepWater,
    /// Cold water rising
    Upwelling,
    /// Warm water sinking
    Downwelling,
}

impl OceanCurrent {
    /// Create a major ocean current
    pub fn new_major(name: &str, path: Vec<(f64, f64)>, current_type: CurrentType) -> Self {
        let (velocity, temperature) = match current_type {
            CurrentType::Surface => (
                0.5 + rand::thread_rng().gen::<f64>() * 1.5,
                15.0 + rand::thread_rng().gen::<f64>() * 15.0,
            ),
            CurrentType::DeepWater => (
                0.1 + rand::thread_rng().gen::<f64>() * 0.2,
                2.0 + rand::thread_rng().gen::<f64>() * 3.0,
            ),
            CurrentType::Upwelling => (0.05, 5.0),
            CurrentType::Downwelling => (0.05, 20.0),
        };

        Self {
            name: name.to_string(),
            path,
            velocity,
            temperature,
            salinity: 35.0,
            current_type,
        }
    }

    /// Update current based on temperature
    pub fn update(&mut self, temperature: f64, _tidal_force: f64, _dt: f64) {
        // Temperature affects velocity
        let temp_factor = 1.0 + (temperature - 288.0) / 50.0;
        self.velocity *= temp_factor;
        self.velocity = self.velocity.clamp(0.01, 2.0);
    }
}

/// Water depth map cell
#[derive(Debug, Clone)]
pub struct DepthCell {
    pub position: (f64, f64),
    pub depth: f64, // meters, negative = above sea level
    pub salinity: f64,
    pub temperature: f64,
}

/// Water cycle state
#[derive(Debug, Clone)]
pub struct WaterCycle {
    /// Global evaporation rate (kg/m²/s)
    pub evaporation_rate: f64,
    /// Global precipitation rate (mm/hour)
    pub precipitation_rate: f64,
    /// Total river runoff (m³/s)
    pub runoff_rate: f64,
    /// Groundwater level (m below surface)
    pub groundwater_level: f64,
    /// Cloud water content (kg/m³)
    pub cloud_water_content: f64,
    /// Ice/snow coverage (fraction)
    pub ice_coverage: f64,
    /// Global water inventory (km³)
    pub total_water: f64,
}

impl Default for WaterCycle {
    fn default() -> Self {
        Self::new()
    }
}

impl WaterCycle {
    /// Create default water cycle
    pub fn new() -> Self {
        Self {
            evaporation_rate: BASE_EVAPORATION,
            precipitation_rate: 2.5,
            runoff_rate: 1.2e5, // ~1.2e5 m³/s global river discharge
            groundwater_level: 10.0,
            cloud_water_content: 0.5,
            ice_coverage: 0.1,
            total_water: 1.386e9, // Global water in km³
        }
    }
}

// ============================================================================
// Main Hydrosphere Structure
// ============================================================================

/// Dynamic hydrosphere: oceans, rivers, lakes, glaciers, water cycle
#[derive(Debug, Clone)]
pub struct Hydrosphere {
    /// Ocean basins
    pub oceans: Vec<OceanBasin>,
    /// Rivers
    pub rivers: Vec<River>,
    /// Lakes
    pub lakes: Vec<Lake>,
    /// Glaciers
    pub glaciers: Vec<Glacier>,
    /// Ocean currents
    pub ocean_currents: Vec<OceanCurrent>,
    /// Water cycle state
    pub water_cycle: WaterCycle,
    /// Depth map
    pub depth_map: Vec<DepthCell>,
    /// Sea level relative to reference (m)
    pub sea_level: f64,
}

impl Default for Hydrosphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Hydrosphere {
    /// Create a new hydrosphere with Earth-like water systems
    pub fn new() -> Self {
        // Create ocean basins
        let oceans = vec![
            OceanBasin::new(0, "Pacific", -15.0, -130.0, 165e6),
            OceanBasin::new(1, "Atlantic", 0.0, -30.0, 85e6),
            OceanBasin::new(2, "Indian", -20.0, 80.0, 70e6),
            OceanBasin::new(3, "Arctic", 75.0, 0.0, 14e6),
            OceanBasin::new(4, "Southern", -60.0, 0.0, 20e6),
        ];

        // Create some major rivers
        let rivers = vec![
            River::new(0, "Nile", (0.0, 32.0), (31.0, 30.0)),
            River::new(1, "Amazon", (-4.0, -73.0), (0.0, -50.0)),
            River::new(2, "Mississippi", (47.0, -95.0), (29.0, -90.0)),
            River::new(3, "Yangtze", (33.0, 91.0), (31.0, 121.0)),
            River::new(4, "Ganges", (28.0, 77.0), (22.0, 90.0)),
        ];

        // Create some major lakes
        let lakes = vec![
            Lake::new(0, "Caspian Sea", (42.0, 51.0), 371000.0),
            Lake::new(1, "Lake Superior", (47.0, -87.0), 82100.0),
            Lake::new(2, "Lake Victoria", (-1.0, 33.0), 68870.0),
            Lake::new(3, "Aral Sea", (45.0, 60.0), 68000.0),
        ];

        // Create glaciers
        let glaciers = vec![
            Glacier::new(
                0,
                "Greenland Ice Sheet",
                (72.0, -40.0),
                1.7e6,
                (0.0, 3200.0),
            ),
            Glacier::new(1, "Antarctic Ice Sheet", (-80.0, 0.0), 1.4e7, (0.0, 4500.0)),
            Glacier::new(
                2,
                "Himalayan Glaciers",
                (28.0, 86.0),
                33000.0,
                (4000.0, 8000.0),
            ),
        ];

        // Create ocean currents
        let ocean_currents = vec![
            OceanCurrent::new_major(
                "Gulf Stream",
                vec![(25.0, -80.0), (30.0, -70.0), (40.0, -50.0), (50.0, -20.0)],
                CurrentType::Surface,
            ),
            OceanCurrent::new_major(
                "Kuroshio Current",
                vec![(25.0, 125.0), (30.0, 135.0), (35.0, 145.0), (40.0, 155.0)],
                CurrentType::Surface,
            ),
            OceanCurrent::new_major(
                "Antarctic Circumpolar",
                vec![
                    (-60.0, -180.0),
                    (-60.0, -90.0),
                    (-60.0, 0.0),
                    (-60.0, 90.0),
                    (-60.0, 180.0),
                ],
                CurrentType::Surface,
            ),
        ];

        // Create depth map (simplified 5-degree grid)
        let mut depth_map = Vec::new();
        for lat in (-90..90).step_by(5) {
            for lon in (-180..180).step_by(5) {
                let depth = Self::generate_ocean_depth(lat as f64, lon as f64);
                depth_map.push(DepthCell {
                    position: (lat as f64, lon as f64),
                    depth,
                    salinity: 35.0,
                    temperature: 5.0,
                });
            }
        }

        Self {
            oceans,
            rivers,
            lakes,
            glaciers,
            ocean_currents,
            water_cycle: WaterCycle::new(),
            depth_map,
            sea_level: SEA_LEVEL,
        }
    }

    /// Generate ocean depth based on position
    fn generate_ocean_depth(lat: f64, lon: f64) -> f64 {
        // Simple model: deeper in center of ocean basins, shallow near edges
        let basin_depth = match lon {
            -180.0..=-100.0 => -4000.0,            // Pacific
            -100.0..=-20.0 => -3500.0,             // Atlantic
            -20.0..=60.0 => -3000.0,               // Indian
            60.0..=180.0 if lat > 60.0 => -1500.0, // Arctic
            _ => -3000.0,
        };

        // Add variation
        basin_depth + rand::thread_rng().gen::<f64>() * 2000.0
    }

    /// Main tick: advance all hydrological processes
    pub fn tick(&mut self, temperature: f64, precipitation: f64, tidal_force: f64, dt: f64) {
        // 1. Evaporation (temperature-driven)
        let evaporation = self.compute_evaporation(temperature);
        self.water_cycle.evaporation_rate = evaporation;
        self.water_cycle.cloud_water_content += evaporation * dt;

        // 2. Precipitation
        self.water_cycle.precipitation_rate = precipitation;
        self.water_cycle.cloud_water_content -= precipitation * 0.001 * dt;

        // 3. River flow
        let total_runoff = self.rivers.iter().map(|r| r.flow_rate).sum::<f64>();
        self.water_cycle.runoff_rate = total_runoff;

        for river in &mut self.rivers {
            river.flow(precipitation, dt);
        }

        // 4. Lake updates
        for lake in &mut self.lakes {
            lake.tick(temperature, precipitation, dt);
        }

        // 5. Glacier updates
        for glacier in &mut self.glaciers {
            glacier.update(temperature, precipitation, dt);
        }

        // 6. Ocean currents
        for current in &mut self.ocean_currents {
            current.update(temperature, tidal_force, dt);
        }

        // 7. Sea level (simplified)
        self.update_sea_level(temperature, dt);

        // 8. Update depth map temperatures
        for cell in &mut self.depth_map {
            if cell.depth < 0.0 {
                // Ocean
                cell.temperature = temperature - 50.0; // Simplified
                cell.temperature = cell.temperature.clamp(-2.0, 30.0);
            }
        }
    }

    /// Compute evaporation rate
    fn compute_evaporation(&self, temperature: f64) -> f64 {
        // Clausius-Clapeyron: evaporation roughly doubles per 10°C
        let base_rate = BASE_EVAPORATION;
        if temperature > 273.15 {
            base_rate * 2.0_f64.powf((temperature - 293.0) / 10.0)
        } else {
            0.0 // No evaporation below freezing
        }
    }

    /// Update sea level
    fn update_sea_level(&mut self, temperature: f64, dt: f64) {
        // Thermal expansion + glacial melt
        let temp_anomaly = temperature - 288.0; // Relative to 15°C

        let expansion = if temp_anomaly > 0.0 {
            temp_anomaly * 0.01 * dt / (365.0 * SECONDS_PER_DAY)
        } else {
            0.0
        };

        // Glacial melt contribution
        let glacial_melt = self
            .glaciers
            .iter()
            .map(|g| g.melt_rate * g.area * 1e-3)
            .sum::<f64>()
            * dt
            / (365.0 * SECONDS_PER_DAY);

        self.sea_level += expansion + glacial_melt;

        // Clamp
        self.sea_level = self.sea_level.clamp(-100.0, 100.0);
    }

    /// Get water depth at a location
    pub fn depth_at(&self, lat: f64, lon: f64) -> f64 {
        for cell in &self.depth_map {
            if (cell.position.0 - lat).abs() < 3.0 && (cell.position.1 - lon).abs() < 3.0 {
                return cell.depth + self.sea_level;
            }
        }
        0.0
    }

    /// Check if a location is underwater
    pub fn is_underwater(&self, lat: f64, lon: f64) -> bool {
        self.depth_at(lat, lon) < 0.0
    }
}

/// An ocean basin
#[derive(Debug, Clone)]
pub struct OceanBasin {
    pub id: u64,
    pub name: String,
    pub center: (f64, f64),
    pub area: f64,
    pub avg_depth: f64,
    pub max_depth: f64,
}

impl OceanBasin {
    pub fn new(id: u64, name: &str, center_lat: f64, center_lon: f64, area: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            center: (center_lat, center_lon),
            area,
            avg_depth: AVG_OCEAN_DEPTH,
            max_depth: AVG_OCEAN_DEPTH * 2.0,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrosphere_creation() {
        let hydro = Hydrosphere::new();
        assert!(!hydro.oceans.is_empty());
        assert!(!hydro.rivers.is_empty());
    }

    #[test]
    fn test_water_depth() {
        let hydro = Hydrosphere::new();
        let depth = hydro.depth_at(0.0, -50.0);
        assert!(depth < 0.0); // Should be ocean
    }

    #[test]
    fn test_river_flow() {
        let mut river = River::new(0, "Test", (45.0, -120.0), (40.0, -110.0));
        river.flow(5.0, 1000.0);
        assert!(river.flow_rate > 0.0);
    }
}
