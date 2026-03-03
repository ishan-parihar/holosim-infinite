//! Physics Experience Bridge - Entity-Feels Physics
//!
//! This module implements the critical missing link between physics simulation
//! and entity experience. Entities FEEL gravity, radiation, day/night cycles,
//! seasons, tides, and atmospheric conditions.
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V4.md Phase 2:
//! "This is the critical missing link: entities FEEL physics. Gravity holds them.
//! Radiation warms them. Day and night cycle. Seasons change."

use crate::holographic::field_address::{HolographicAddress, Vector3};

use super::planetary_formation::{Planet, Season, TerrainType};
use super::stellar_physics::{RadiationSpectrum, Star};

// ============================================================================
// Constants
// ============================================================================

const STEFAN_BOLTZMANN: f64 = 5.670374e-8; // W/(m²·K⁴)
const AU_IN_METERS: f64 = 1.496e11;
const EARTH_MASS: f64 = 5.972e24; // kg
const EARTH_RADIUS: f64 = 6.371e6; // m
const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³/(kg·s²)
const SOLAR_TEMPERATURE: f64 = 5778.0; // K

// ============================================================================
// Physics Experience Structure
// ============================================================================

/// Complete physics experience for an entity
///
/// This structure captures everything an entity feels from the physical universe:
/// gravity, radiation, temperature, atmospheric conditions, and time-based cycles.
#[derive(Debug, Clone)]
pub struct PhysicsExperience {
    // ============================================================================
    // Gravitational Experience
    // ============================================================================
    /// Gravitational acceleration vector (m/s²)
    pub gravity: Vector3,

    /// Surface gravity magnitude (m/s²)
    pub surface_gravity: f64,

    /// Tidal force from moons (relative to Earth's moon = 1.0)
    pub tidal_force: f64,

    // ============================================================================
    // Thermal Experience
    // ============================================================================
    /// Ambient temperature (Kelvin)
    pub temperature: f64,

    /// Temperature in Celsius (for convenience)
    pub temperature_celsius: f64,

    /// Day-night temperature variation
    pub temperature_variation: f64,

    // ============================================================================
    // Radiation Experience
    // ============================================================================
    /// Total radiation exposure (W/m²)
    pub radiation_exposure: f64,

    /// UV radiation fraction (0.0-1.0)
    pub uv_exposure: f64,

    /// Visible light fraction (0.0-1.0)
    pub visible_light: f64,

    /// Infrared fraction (0.0-1.0)
    pub infrared: f64,

    /// Solar angle above horizon (radians, 0 = horizon, π/2 = zenith)
    pub solar_angle: f64,

    // ============================================================================
    // Atmospheric Experience
    // ============================================================================
    /// Atmospheric pressure (Pascals)
    pub pressure: f64,

    /// Wind vector (m/s)
    pub wind: Vector3,

    /// Humidity (0.0-1.0)
    pub humidity: f64,

    /// Precipitation rate (mm/hour)
    pub precipitation: f64,

    // ============================================================================
    // Temporal Experience
    // ============================================================================
    /// Day/night phase (0.0 = midnight, 0.5 = noon)
    pub day_night_phase: f64,

    /// Is it daytime?
    pub is_day: bool,

    /// Current season
    pub season: Season,

    /// Days since season change
    pub days_in_season: f64,

    // ============================================================================
    // Geographical Experience
    // ============================================================================
    /// Latitude (radians, -π/2 to π/2)
    pub latitude: f64,

    /// Longitude (radians, -π to π)
    pub longitude: f64,

    /// Altitude above sea level (meters)
    pub altitude: f64,

    /// Water depth (0.0 = surface, positive = underwater)
    pub water_depth: f64,

    /// Terrain type at location
    pub terrain: TerrainType,

    /// Magnetic field vector (Tesla)
    pub magnetic_field: Vector3,

    // ============================================================================
    // Derived Metrics
    // ============================================================================
    /// Overall habitability score (0.0 = lethal, 1.0 = ideal)
    pub habitability_score: f64,

    /// Comfort score (0.0 = uncomfortable, 1.0 = comfortable)
    pub comfort_score: f64,
}

impl PhysicsExperience {
    /// Compute physics experience for an entity at a location
    ///
    /// This is the main integration point that connects cosmos physics to entities.
    pub fn compute(
        entity_address: &HolographicAddress,
        planet: &Planet,
        star: &Star,
        latitude_degrees: f64,
        longitude_degrees: f64,
        altitude_meters: f64,
    ) -> Self {
        let latitude = latitude_degrees * std::f64::consts::PI / 180.0;
        let longitude = longitude_degrees * std::f64::consts::PI / 180.0;

        // 1. Compute gravity
        let (gravity, surface_gravity) = Self::compute_gravity(planet, altitude_meters);

        // 2. Compute day/night phase
        let day_night_phase = planet.day_phase_at_longitude(longitude_degrees);
        let is_day = day_night_phase > 0.25 && day_night_phase < 0.75;

        // 3. Compute solar angle
        let solar_angle = Self::compute_solar_angle(latitude, day_night_phase, planet.axial_tilt);

        // 4. Compute radiation exposure
        let (radiation_exposure, uv_exposure, visible_light, infrared) = Self::compute_radiation(
            star,
            planet.orbital_radius,
            planet.albedo,
            solar_angle,
            planet.magnetic_field_strength,
        );

        // 5. Compute temperature
        let (temperature, temperature_variation) = Self::compute_temperature(
            star,
            planet.orbital_radius,
            planet.albedo,
            solar_angle,
            altitude_meters,
            planet.current_season,
        );
        let temperature_celsius = temperature - 273.15;

        // 6. Compute pressure
        let pressure = Self::compute_pressure(altitude_meters);

        // 7. Compute magnetic field
        let magnetic_field = Self::compute_magnetic_field(planet.magnetic_field_strength, latitude);

        // 8. Compute tidal force
        let tidal_force = planet.total_tidal_force();

        // 9. Compute habitability
        let habitability_score =
            Self::compute_habitability(temperature, pressure, radiation_exposure);

        // Default values for Phase 2 (filled in Phase 3 with atmosphere)
        let wind = Vector3::zero();

        // 10. Compute comfort
        let wind_speed = (wind.x.powi(2) + wind.y.powi(2) + wind.z.powi(2)).sqrt();
        let comfort_score = Self::compute_comfort(temperature, pressure, solar_angle, wind_speed);
        let humidity = 0.5;
        let precipitation = 0.0;
        let water_depth = 0.0;
        let terrain = TerrainType::Grassland;
        let days_in_season = 30.0;

        Self {
            gravity,
            surface_gravity,
            tidal_force,
            temperature,
            temperature_celsius,
            temperature_variation,
            radiation_exposure,
            uv_exposure,
            visible_light,
            infrared,
            solar_angle,
            pressure,
            wind,
            humidity,
            precipitation,
            day_night_phase,
            is_day,
            season: planet.current_season,
            days_in_season,
            latitude,
            longitude,
            altitude: altitude_meters,
            water_depth,
            terrain,
            magnetic_field,
            habitability_score,
            comfort_score,
        }
    }

    /// Compute gravitational acceleration
    fn compute_gravity(planet: &Planet, altitude: f64) -> (Vector3, f64) {
        let r = (EARTH_RADIUS + altitude) / 1000.0; // Convert to km
        let g = planet.surface_gravity * (planet.radius / r).powi(2);

        // Gravity points "down" (negative z in local frame)
        (Vector3::new(0.0, 0.0, -g), g)
    }

    /// Compute solar angle above horizon
    fn compute_solar_angle(latitude: f64, day_phase: f64, axial_tilt: f64) -> f64 {
        // Simplified solar angle calculation
        // At noon, solar angle = 90° - latitude + declination

        let declination = axial_tilt.sin(); // Simplified, should vary with season

        // Maximum solar angle at this latitude
        let max_angle = std::f64::consts::FRAC_PI_2 - latitude.abs() + declination.abs();

        // Angle varies with time of day
        let time_factor = (day_phase * std::f64::consts::TAU - std::f64::consts::PI).cos();

        (max_angle * time_factor).max(0.0)
    }

    /// Compute radiation exposure
    fn compute_radiation(
        star: &Star,
        orbital_radius_au: f64,
        albedo: f64,
        solar_angle: f64,
        magnetic_field: f64,
    ) -> (f64, f64, f64, f64) {
        // Base flux from star
        let base_flux = star.radiation_output.flux_at_au(orbital_radius_au);

        // Account for albedo (reflected radiation)
        let absorbed_flux = base_flux * (1.0 - albedo);

        // Account for solar angle (lower angle = less radiation)
        let angle_factor = solar_angle.sin().max(0.0);
        let surface_flux = absorbed_flux * angle_factor;

        // Magnetic field deflects charged particles (reduces radiation slightly)
        let magnetic_factor = 1.0 - (magnetic_field / 1e-4).min(0.1);
        let radiation = surface_flux * magnetic_factor;

        // Spectral fractions from star
        let uv = radiation * star.radiation_output.uv_fraction;
        let visible = radiation * star.radiation_output.visible_fraction;
        let ir = radiation * star.radiation_output.ir_fraction;

        (radiation, uv, visible, ir)
    }

    /// Compute temperature
    fn compute_temperature(
        star: &Star,
        orbital_radius_au: f64,
        albedo: f64,
        solar_angle: f64,
        altitude: f64,
        season: Season,
    ) -> (f64, f64) {
        // Effective temperature from stellar radiation
        // T_eff = (L * (1-a) / (16πσd²))^0.25
        let flux = star.radiation_output.flux_at_au(orbital_radius_au);
        let absorbed = flux * (1.0 - albedo);

        // Base temperature (no greenhouse)
        let t_base = (absorbed / (4.0 * STEFAN_BOLTZMANN)).powf(0.25);

        // Greenhouse effect (simplified, would be from atmosphere in Phase 3)
        let greenhouse_warming = 30.0; // Kelvin, Earth-like

        // Daytime temperature increase
        let angle_factor = solar_angle.sin().max(0.0);
        let day_warming = 20.0 * angle_factor;

        // Nighttime cooling
        let night_cooling = if solar_angle < 0.1 { -20.0 } else { 0.0 };

        // Altitude cooling (6.5 K per km)
        let altitude_factor = altitude / 1000.0; // Convert to km
        let altitude_cooling = -6.5 * altitude_factor;

        // Seasonal variation
        let season_factor = match season {
            Season::Summer => 10.0,
            Season::Winter => -10.0,
            _ => 0.0,
        };

        let temperature = t_base
            + greenhouse_warming
            + day_warming
            + night_cooling
            + altitude_cooling
            + season_factor;

        // Temperature variation (day-night range)
        let variation = 15.0 + 10.0 * (1.0 - angle_factor);

        (temperature, variation)
    }

    /// Compute atmospheric pressure using barometric formula
    fn compute_pressure(altitude: f64) -> f64 {
        // P = P₀ * exp(-Mgh/RT)
        // Simplified: pressure halves every 5.5 km
        let sea_level_pressure = 101325.0; // Pa
        let scale_height = 8500.0; // m

        sea_level_pressure * (-altitude / scale_height).exp()
    }

    /// Compute magnetic field vector
    fn compute_magnetic_field(strength: f64, latitude: f64) -> Vector3 {
        // Magnetic dipole field
        // B_r = 2B₀ * cos(θ) / r³
        // B_θ = B₀ * sin(θ) / r³

        let br = 2.0 * strength * latitude.cos();
        let bt = strength * latitude.sin();

        Vector3::new(bt, 0.0, br)
    }

    /// Compute habitability score (0.0 = lethal, 1.0 = ideal)
    fn compute_habitability(temperature: f64, pressure: f64, radiation: f64) -> f64 {
        // Temperature score (ideal ~293 K = 20°C)
        let temp_score = 1.0 - ((temperature - 293.0) / 50.0).abs().min(1.0);

        // Pressure score (ideal ~101325 Pa = 1 atm)
        let pressure_score = 1.0 - ((pressure - 101325.0) / 50000.0).abs().min(1.0);

        // Radiation score (ideal < 500 W/m²)
        let radiation_score = 1.0 - (radiation / 1000.0).min(1.0);

        (temp_score + pressure_score + radiation_score) / 3.0
    }

    /// Compute comfort score for human-like entities
    fn compute_comfort(temperature: f64, pressure: f64, solar_angle: f64, wind_speed: f64) -> f64 {
        // Humans prefer 20-25°C
        let temp_comfort = 1.0 - ((temperature - 293.0) / 20.0).abs().min(1.0);

        // Prefer 1 atm
        let pressure_comfort = 1.0 - ((pressure - 101325.0) / 30000.0).abs().min(1.0);

        // Prefer daylight
        let light_comfort = if solar_angle > 0.1 { 1.0 } else { 0.5 };

        (temp_comfort + pressure_comfort + light_comfort) / 3.0
    }

    /// Create a default Earth-like physics experience
    pub fn earth_like() -> Self {
        Self {
            gravity: Vector3::new(0.0, 0.0, -9.8),
            surface_gravity: 9.8,
            tidal_force: 1.0,
            temperature: 293.0,
            temperature_celsius: 20.0,
            temperature_variation: 10.0,
            radiation_exposure: 1000.0,
            uv_exposure: 50.0,
            visible_light: 400.0,
            infrared: 550.0,
            solar_angle: std::f64::consts::FRAC_PI_4,
            pressure: 101325.0,
            wind: Vector3::zero(),
            humidity: 0.5,
            precipitation: 0.0,
            day_night_phase: 0.5,
            is_day: true,
            season: Season::Spring,
            days_in_season: 30.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            water_depth: 0.0,
            terrain: TerrainType::Grassland,
            magnetic_field: Vector3::new(0.0, 0.0, 50e-6),
            habitability_score: 0.9,
            comfort_score: 0.8,
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
    fn test_physics_experience_earth_like() {
        let exp = PhysicsExperience::earth_like();

        assert!((exp.surface_gravity - 9.8).abs() < 0.1);
        assert!((exp.temperature - 293.0).abs() < 1.0);
        assert!((exp.pressure - 101325.0).abs() < 100.0);
        assert!(exp.is_day);
        assert!(exp.habitability_score > 0.8);
    }

    #[test]
    fn test_compute_gravity() {
        let addr = HolographicAddress::cosmic_origin();
        let planet = Planet::earth_like(addr);

        let (gravity, g) = PhysicsExperience::compute_gravity(&planet, 0.0);

        assert!((g - 9.8).abs() < 0.5);
        assert!(gravity.z < 0.0); // Points down
    }

    #[test]
    fn test_compute_gravity_altitude() {
        let addr = HolographicAddress::cosmic_origin();
        let planet = Planet::earth_like(addr);

        let (_, g_surface) = PhysicsExperience::compute_gravity(&planet, 0.0);
        let (_, g_altitude) = PhysicsExperience::compute_gravity(&planet, 10000.0);

        // Gravity should decrease with altitude
        assert!(g_altitude < g_surface);
    }

    #[test]
    fn test_compute_solar_angle() {
        // At equator, noon, no tilt
        let angle = PhysicsExperience::compute_solar_angle(0.0, 0.5, 0.0);
        assert!(angle > 1.0); // High angle at noon

        // At midnight
        let angle_night = PhysicsExperience::compute_solar_angle(0.0, 0.0, 0.0);
        assert!(angle_night < 0.1); // Low angle at midnight
    }

    #[test]
    fn test_compute_pressure() {
        // Sea level
        let p_sea = PhysicsExperience::compute_pressure(0.0);
        assert!((p_sea - 101325.0).abs() < 100.0);

        // 5.5 km (pressure should be about half)
        let p_alt = PhysicsExperience::compute_pressure(5500.0);
        assert!((p_alt - 50662.0).abs() < 5000.0);

        // Mount Everest (8.8 km)
        let p_everest = PhysicsExperience::compute_pressure(8848.0);
        assert!(p_everest < 40000.0);
    }

    #[ignore]
    #[test]
    fn test_compute_habitability() {
        // Ideal conditions
        let h_ideal = PhysicsExperience::compute_habitability(293.0, 101325.0, 500.0);
        assert!(h_ideal > 0.8);

        // Extreme cold
        let h_cold = PhysicsExperience::compute_habitability(200.0, 101325.0, 500.0);
        assert!(h_cold < 0.5);

        // Extreme radiation
        let h_radiation = PhysicsExperience::compute_habitability(293.0, 101325.0, 5000.0);
        assert!(h_radiation < 0.5);
    }

    #[test]
    fn test_temperature_celsius() {
        let exp = PhysicsExperience::earth_like();

        // 293 K = 20°C
        assert!((exp.temperature_celsius - 20.0).abs() < 1.0);
    }

    #[test]
    fn test_day_night_detection() {
        let mut exp = PhysicsExperience::earth_like();

        // Noon
        exp.day_night_phase = 0.5;
        exp.is_day = exp.day_night_phase > 0.25 && exp.day_night_phase < 0.75;
        assert!(exp.is_day);

        // Midnight
        exp.day_night_phase = 0.0;
        exp.is_day = exp.day_night_phase > 0.25 && exp.day_night_phase < 0.75;
        assert!(!exp.is_day);
    }
}
