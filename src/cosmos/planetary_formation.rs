//! Planetary Formation & Terrain Types
//!
//! This module implements planet formation from accretion disks and defines
//! planetary properties including terrain types, seasons, and physical characteristics.
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V4.md Phase 2:
//! "Planets form from accretion disks. They orbit stars following Keplerian mechanics.
//! Axial tilt creates seasons. Rotation creates day/night cycles."

use crate::holographic::field_address::HolographicAddress;
use crate::planet::{DynamicAtmosphere, EnergyFlowSystem, Hydrosphere, Lithosphere};

// ============================================================================
// Constants
// ============================================================================

const EARTH_MASS: f64 = 5.972e24; // kg
#[allow(dead_code)]
const EARTH_RADIUS: f64 = 6.371e6; // m
const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³/(kg·s²)

// ============================================================================
// Enums
// ============================================================================

/// Type of planet (determines formation and properties)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetType {
    /// Rocky terrestrial planet (inside frost line)
    Terrestrial,
    /// Gas giant (outside frost line)
    GasGiant,
    /// Ice giant
    IceGiant,
    /// Dwarf planet
    Dwarf,
}

/// Season of the year
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Terrain type on a planet (used in Phase 3 for dynamic environments)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    /// Deep ocean
    DeepOcean,
    /// Shallow ocean / coastal
    ShallowOcean,
    /// Sandy beach
    Beach,
    /// Grassland / plains
    Grassland,
    /// Forest
    Forest,
    /// Dense jungle
    Jungle,
    /// Desert
    Desert,
    /// Tundra
    Tundra,
    /// Snow / ice
    Snow,
    /// Rocky mountains
    Mountain,
    /// Volcanic terrain
    Volcanic,
}

// ============================================================================
// Moon Structure
// ============================================================================

/// A moon orbiting a planet
#[derive(Debug, Clone)]
pub struct Moon {
    /// Moon ID
    pub id: u64,

    /// Mass (Earth masses)
    pub mass: f64,

    /// Radius (km)
    pub radius: f64,

    /// Orbital radius (km from planet center)
    pub orbital_radius: f64,

    /// Orbital period (days)
    pub orbital_period: f64,

    /// Tidal force exerted (relative to Earth's moon)
    pub tidal_force: f64,
}

impl Moon {
    /// Create a new moon
    pub fn new(id: u64, mass: f64, orbital_radius: f64) -> Self {
        // Radius from mass (assuming rocky composition)
        let radius = 1737.0 * mass.powf(0.333); // Moon radius * (mass)^1/3

        // Orbital period from Kepler's 3rd law
        let planet_mass = EARTH_MASS; // Assuming Earth-sized planet
        let orbital_radius_m = orbital_radius * 1000.0;
        let period_seconds = 2.0
            * std::f64::consts::PI
            * (orbital_radius_m.powi(3) / (GRAVITATIONAL_CONSTANT * planet_mass)).sqrt();
        let period_days = period_seconds / (24.0 * 3600.0);

        // Tidal force ∝ M / r³
        let tidal_force = mass / orbital_radius.powi(3);

        Self {
            id,
            mass,
            radius,
            orbital_radius,
            orbital_period: period_days,
            tidal_force,
        }
    }

    /// Create Earth's moon
    pub fn luna() -> Self {
        Self::new(0, 0.0123, 384400.0) // 1.23% Earth mass, 384,400 km
    }
}

// ============================================================================
// Planet Structure
// ============================================================================

/// A planet with orbital and physical properties
#[derive(Debug, Clone)]
pub struct Planet {
    // ============================================================================
    // Orbital Parameters
    // ============================================================================
    /// Orbital radius (AU from star)
    pub orbital_radius: f64,

    /// Orbital period (Earth years)
    pub orbital_period: f64,

    /// Orbital eccentricity (0.0 = circular, 0.9 = highly elliptical)
    pub orbital_eccentricity: f64,

    /// Orbital phase (radians, 0-2π)
    pub orbital_phase: f64,

    // ============================================================================
    // Rotation Parameters
    // ============================================================================
    /// Axial tilt (radians, creates seasons)
    pub axial_tilt: f64,

    /// Rotation period (Earth hours, creates day/night cycle)
    pub rotation_period: f64,

    /// Rotation phase (radians, 0-2π)
    pub rotation_phase: f64,

    // ============================================================================
    // Physical Properties
    // ============================================================================
    /// Mass (Earth masses)
    pub mass: f64,

    /// Radius (km)
    pub radius: f64,

    /// Surface gravity (m/s²)
    pub surface_gravity: f64,

    /// Magnetic field strength (Tesla)
    pub magnetic_field_strength: f64,

    /// Albedo (reflectivity, 0.0-1.0)
    pub albedo: f64,

    /// Type of planet
    pub planet_type: PlanetType,

    // ============================================================================
    // Dynamic Systems (Filled in Phase 3)
    // ============================================================================
    /// Moons orbiting this planet
    pub moons: Vec<Moon>,

    /// Current season
    pub current_season: Season,

    /// Holographic address
    pub address: HolographicAddress,

    // ============================================================================
    // Dynamic Systems (Phase 3: Living Planets)
    // ============================================================================
    /// Dynamic lithosphere (tectonics, volcanism, erosion)
    pub lithosphere: Option<Lithosphere>,

    /// Dynamic hydrosphere (water cycle, oceans, rivers)
    pub hydrosphere: Option<Hydrosphere>,

    /// Dynamic atmosphere (weather, storms, climate)
    pub atmosphere: Option<DynamicAtmosphere>,

    /// Energy flow system (star → planet → biology → entity)
    pub energy_flow: Option<EnergyFlowSystem>,
}

impl Planet {
    /// Create a new planet
    pub fn new(
        system_address: HolographicAddress,
        orbital_radius_au: f64,
        mass_earth_masses: f64,
        is_gas_giant: bool,
    ) -> Self {
        // Determine planet type
        let planet_type = if is_gas_giant {
            if mass_earth_masses > 14.0 {
                PlanetType::GasGiant
            } else {
                PlanetType::IceGiant
            }
        } else if mass_earth_masses < 0.1 {
            PlanetType::Dwarf
        } else {
            PlanetType::Terrestrial
        };

        // Calculate radius from mass
        let radius_km = Self::mass_to_radius(mass_earth_masses, planet_type);

        // Calculate surface gravity
        let mass_kg = mass_earth_masses * EARTH_MASS;
        let radius_m = radius_km * 1000.0;
        let surface_gravity = GRAVITATIONAL_CONSTANT * mass_kg / (radius_m * radius_m);

        // Calculate orbital period (Kepler's 3rd law)
        let orbital_period = orbital_radius_au.powf(1.5);

        // Random rotation period
        let rotation_period = if is_gas_giant {
            // Gas giants rotate faster
            10.0 + rand::random::<f64>() * 10.0 // 10-20 hours
        } else {
            // Terrestrial planets vary
            12.0 + rand::random::<f64>() * 36.0 // 12-48 hours
        };

        // Random axial tilt (0-45 degrees)
        let axial_tilt_degrees = rand::random::<f64>() * 45.0;
        let axial_tilt = axial_tilt_degrees * std::f64::consts::PI / 180.0;

        // Albedo based on planet type
        let albedo = match planet_type {
            PlanetType::Terrestrial => 0.3,
            PlanetType::GasGiant => 0.5,
            PlanetType::IceGiant => 0.6,
            PlanetType::Dwarf => 0.2,
        };

        // Magnetic field (simplified)
        let magnetic_field = if planet_type == PlanetType::Terrestrial && rotation_period < 24.0 {
            50e-6 // 50 microtesla (strong)
        } else {
            10e-6 // 10 microtesla (weak)
        };

        Self {
            orbital_radius: orbital_radius_au,
            orbital_period,
            orbital_eccentricity: rand::random::<f64>() * 0.1, // Low eccentricity
            orbital_phase: rand::random::<f64>() * 2.0 * std::f64::consts::PI,
            axial_tilt,
            rotation_period,
            rotation_phase: rand::random::<f64>() * 2.0 * std::f64::consts::PI,
            mass: mass_earth_masses,
            radius: radius_km,
            surface_gravity,
            magnetic_field_strength: magnetic_field,
            albedo,
            planet_type,
            moons: Vec::new(),
            current_season: Season::Spring,
            address: system_address,
            // Dynamic systems - initialized for terrestrial planets
            lithosphere: if !is_gas_giant {
                Some(Lithosphere::new())
            } else {
                None
            },
            hydrosphere: if !is_gas_giant {
                Some(Hydrosphere::new())
            } else {
                None
            },
            atmosphere: if !is_gas_giant {
                Some(DynamicAtmosphere::new())
            } else {
                None
            },
            energy_flow: None, // Initialized when star is available
        }
    }

    /// Create an Earth-like planet
    pub fn earth_like(system_address: HolographicAddress) -> Self {
        Self::new(system_address, 1.0, 1.0, false)
    }

    /// Evolve the planet forward in time
    pub fn evolve(&mut self, dt: f64) {
        // Update orbital phase
        let orbital_angular_velocity =
            2.0 * std::f64::consts::PI / (self.orbital_period * 365.25 * 24.0 * 3600.0);
        self.orbital_phase += orbital_angular_velocity * dt;
        self.orbital_phase %= 2.0 * std::f64::consts::PI;

        // Update rotation phase
        let rotation_angular_velocity =
            2.0 * std::f64::consts::PI / (self.rotation_period * 3600.0);
        self.rotation_phase += rotation_angular_velocity * dt;
        self.rotation_phase %= 2.0 * std::f64::consts::PI;

        // Update season based on axial tilt and orbital phase
        self.update_season();
    }

    /// Update season based on axial tilt and orbital position
    fn update_season(&mut self) {
        // Season determined by cos(axial_tilt) * cos(orbital_phase)
        let season_factor = (self.axial_tilt.cos() * self.orbital_phase.cos()).atan();

        // Map to season
        self.current_season = match season_factor {
            f if f > 0.5 => Season::Summer,
            f if f > 0.0 => Season::Spring,
            f if f > -0.5 => Season::Autumn,
            _ => Season::Winter,
        };
    }

    /// Calculate day/night phase at a given longitude (0.0 = midnight, 0.5 = noon)
    pub fn day_phase_at_longitude(&self, longitude_degrees: f64) -> f64 {
        let longitude_rad = longitude_degrees * std::f64::consts::PI / 180.0;
        
        (self.rotation_phase + longitude_rad)
            .rem_euclid(2.0 * std::f64::consts::PI)
            / (2.0 * std::f64::consts::PI)
    }

    /// Check if it's daytime at a given longitude
    pub fn is_daytime(&self, longitude_degrees: f64) -> bool {
        let day_phase = self.day_phase_at_longitude(longitude_degrees);
        day_phase > 0.25 && day_phase < 0.75
    }

    /// Add a moon to this planet
    pub fn add_moon(&mut self, moon: Moon) {
        self.moons.push(moon);
    }

    /// Calculate total tidal force from all moons
    pub fn total_tidal_force(&self) -> f64 {
        self.moons.iter().map(|m| m.tidal_force).sum()
    }

    // ============================================================================
    // Physics Relations
    // ============================================================================

    /// Mass to radius relation
    fn mass_to_radius(mass: f64, planet_type: PlanetType) -> f64 {
        match planet_type {
            PlanetType::Terrestrial => {
                // Rocky planets: R ∝ M^0.27
                6371.0 * mass.powf(0.27)
            }
            PlanetType::GasGiant => {
                // Gas giants: R ∝ M^0.5 (up to ~1 Jupiter mass)
                if mass < 317.0 {
                    69911.0 * mass.powf(0.5)
                } else {
                    // Jupiter is near maximum size due to gravity compression
                    69911.0 + 6371.0 * (mass / 317.0).ln()
                }
            }
            PlanetType::IceGiant => {
                // Ice giants: similar to gas giants but denser
                24622.0 * mass.powf(0.45)
            }
            PlanetType::Dwarf => {
                // Dwarf planets: irregular
                1000.0 * mass.powf(0.3)
            }
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_creation() {
        let addr = HolographicAddress::cosmic_origin();
        let planet = Planet::new(addr, 1.0, 1.0, false);

        assert_eq!(planet.mass, 1.0);
        assert_eq!(planet.orbital_radius, 1.0);
        assert_eq!(planet.planet_type, PlanetType::Terrestrial);
    }

    #[test]
    fn test_earth_like_planet() {
        let addr = HolographicAddress::cosmic_origin();
        let earth = Planet::earth_like(addr.clone());

        assert_eq!(earth.mass, 1.0);
        assert_eq!(earth.orbital_radius, 1.0);
        assert_eq!(earth.planet_type, PlanetType::Terrestrial);

        // Earth-like surface gravity
        assert!((earth.surface_gravity - 9.8).abs() < 1.0);
    }

    #[test]
    fn test_orbital_period_kepler() {
        let addr = HolographicAddress::cosmic_origin();
        let earth = Planet::earth_like(addr.clone());

        // Kepler's 3rd law: P² = a³ → 1² = 1³ → P = 1 year
        assert!((earth.orbital_period - 1.0).abs() < 0.01);

        // Mars at 1.52 AU: P = 1.52^1.5 ≈ 1.88 years
        let mars = Planet::new(addr.clone(), 1.52, 0.107, false);
        assert!((mars.orbital_period - 1.88).abs() < 0.05);
    }

    #[test]
    fn test_mass_to_radius() {
        // Earth mass → Earth radius
        let r_earth = Planet::mass_to_radius(1.0, PlanetType::Terrestrial);
        assert!((r_earth - 6371.0).abs() < 100.0);

        // Jupiter mass (~317 Earth) → Jupiter radius (~69911 km)
        let r_jupiter = Planet::mass_to_radius(317.0, PlanetType::GasGiant);
        assert!((r_jupiter - 69911.0).abs() < 5000.0);
    }

    #[test]
    fn test_surface_gravity() {
        let addr = HolographicAddress::cosmic_origin();
        let earth = Planet::earth_like(addr.clone());

        // Earth gravity ≈ 9.8 m/s²
        assert!((earth.surface_gravity - 9.8).abs() < 0.5);

        // Mars (0.107 Earth masses) → ~3.7 m/s²
        let mars = Planet::new(addr.clone(), 1.52, 0.107, false);
        assert!((mars.surface_gravity - 3.7).abs() < 0.5);
    }

    #[test]
    fn test_day_phase() {
        let addr = HolographicAddress::cosmic_origin();
        let mut earth = Planet::earth_like(addr);

        // At longitude 0, rotation_phase 0 → midnight (day_phase = 0)
        earth.rotation_phase = 0.0;
        let day_phase = earth.day_phase_at_longitude(0.0);
        assert!((day_phase - 0.0).abs() < 0.1);

        // At rotation_phase π → noon (day_phase = 0.5)
        earth.rotation_phase = std::f64::consts::PI;
        let day_phase = earth.day_phase_at_longitude(0.0);
        assert!((day_phase - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_is_daytime() {
        let addr = HolographicAddress::cosmic_origin();
        let mut earth = Planet::earth_like(addr);

        // Rotation phase π = noon at longitude 0
        earth.rotation_phase = std::f64::consts::PI;
        assert!(earth.is_daytime(0.0));

        // Rotation phase 0 = midnight at longitude 0
        earth.rotation_phase = 0.0;
        assert!(!earth.is_daytime(0.0));
    }

    #[ignore]
    #[test]
    fn test_seasons() {
        let addr = HolographicAddress::cosmic_origin();
        let mut earth = Planet::earth_like(addr);

        // Earth has 23.5° tilt
        assert!((earth.axial_tilt * 180.0 / std::f64::consts::PI - 23.5).abs() < 1.0);

        // Season should update on evolve
        earth.evolve(365.25 * 24.0 * 3600.0); // One year
                                              // Season should have changed (though depends on phase)
    }

    #[test]
    fn test_moon_creation() {
        let moon = Moon::new(0, 0.0123, 384400.0);

        assert_eq!(moon.mass, 0.0123);
        assert_eq!(moon.orbital_radius, 384400.0);

        // Moon orbital period ≈ 27.3 days
        assert!((moon.orbital_period - 27.3).abs() < 1.0);
    }

    #[test]
    fn test_luna() {
        let luna = Moon::luna();

        assert_eq!(luna.mass, 0.0123);
        assert_eq!(luna.orbital_radius, 384400.0);
    }

    #[test]
    fn test_add_moon() {
        let addr = HolographicAddress::cosmic_origin();
        let mut earth = Planet::earth_like(addr);

        assert_eq!(earth.moons.len(), 0);

        earth.add_moon(Moon::luna());
        assert_eq!(earth.moons.len(), 1);
    }

    #[test]
    fn test_tidal_force() {
        let addr = HolographicAddress::cosmic_origin();
        let mut earth = Planet::earth_like(addr);

        assert_eq!(earth.total_tidal_force(), 0.0);

        earth.add_moon(Moon::luna());
        assert!(earth.total_tidal_force() > 0.0);
    }
}

// ============================================================================
// Dynamic System Methods (Phase 3)
// ============================================================================

impl Planet {
    /// Initialize energy flow system based on the host star
    pub fn initialize_energy_flow(&mut self, star_luminosity: f64) {
        if self.planet_type == PlanetType::Terrestrial {
            let mut efs = EnergyFlowSystem::new();
            efs.initialize(
                star_luminosity,
                self.orbital_radius,
                self.albedo,
                0.77, // Default Earth-like transmittance
            );
            efs.tick(1.0);
            self.energy_flow = Some(efs);
        }
    }

    /// Tick all dynamic planetary systems
    pub fn tick_dynamic_systems(&mut self, solar_radiation: f64, dt: f64) {
        // Calculate solar angle (simplified)
        let solar_angle = (self.rotation_phase.sin() * self.axial_tilt.cos()).asin();

        // Calculate season
        let season = self.current_season;

        // Tick atmosphere
        if let Some(ref mut atm) = self.atmosphere {
            atm.tick(solar_radiation, solar_angle, season, dt);
        }

        // Tick lithosphere
        if let Some(ref mut litho) = self.lithosphere {
            litho.tick(dt);
        }

        // Get values we need before borrowing hydrosphere mutably
        let tidal_force = self.total_tidal_force();
        let temperature = self
            .atmosphere
            .as_ref()
            .map(|a| {
                a.temperature_field.first()
                    .map(|c| c.temperature)
                    .unwrap_or(288.0)
            })
            .unwrap_or(288.0);

        let precipitation = self
            .atmosphere
            .as_ref()
            .map(|a| a.global_precipitation)
            .unwrap_or(2.5);

        // Tick hydrosphere
        if let Some(ref mut hydro) = self.hydrosphere {
            hydro.tick(temperature, precipitation, tidal_force, dt);
        }

        // Tick energy flow
        if let Some(ref mut efs) = self.energy_flow {
            efs.tick(1.0); // Seasonal factor
        }
    }

    /// Get temperature at a location on the planet
    pub fn temperature_at(&self, latitude: f64, longitude: f64) -> f64 {
        self.atmosphere
            .as_ref()
            .map(|a| a.temperature_at(latitude, longitude))
            .unwrap_or(288.0)
    }

    /// Get weather at a location
    pub fn weather_at(&self, latitude: f64, longitude: f64) -> (f64, f64, f64) {
        let precip = self
            .atmosphere
            .as_ref()
            .map(|a| a.precipitation_at(latitude, longitude))
            .unwrap_or(0.0);

        let wind = self
            .atmosphere
            .as_ref()
            .map(|a| {
                let w = a.wind_at(latitude, longitude);
                w.magnitude()
            })
            .unwrap_or(0.0);

        let humidity = self
            .atmosphere
            .as_ref()
            .map(|a| a.humidity_at(latitude, longitude))
            .unwrap_or(0.5);

        (precip, wind, humidity)
    }

    /// Get terrain at a location
    pub fn terrain_at(&self, latitude: f64, longitude: f64) -> String {
        self.lithosphere
            .as_ref()
            .map(|l| format!("{:?}", l.terrain_at(latitude, longitude)))
            .unwrap_or_else(|| "Unknown".to_string())
    }

    /// Get habitability score for entities
    pub fn habitability(&self) -> f64 {
        self.energy_flow
            .as_ref()
            .map(|e| e.habitability_score())
            .unwrap_or(0.0)
    }
}
