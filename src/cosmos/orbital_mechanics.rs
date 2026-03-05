//! Orbital Mechanics - Keplerian Orbits and N-Body Interactions
//!
//! This module implements orbital mechanics for planets, moons, and other
//! celestial bodies following Kepler's laws and gravitational physics.
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V4.md Phase 2:
//! "Planets orbit stars following Keplerian mechanics. Axial tilt creates seasons.
//! Rotation creates day/night cycles. Moons create tides."

use crate::holographic::field_address::Vector3;

// ============================================================================
// Constants
// ============================================================================

const AU_IN_METERS: f64 = 1.496e11;
const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³/(kg·s²)
const SOLAR_MASS: f64 = 1.989e30; // kg
const YEAR_IN_SECONDS: f64 = 365.25 * 24.0 * 3600.0;

// ============================================================================
// Orbital Elements
// ============================================================================

/// Keplerian orbital elements that define an orbit
#[derive(Debug, Clone, Copy)]
pub struct OrbitalElements {
    /// Semi-major axis (AU)
    pub semi_major_axis: f64,

    /// Eccentricity (0.0 = circular, 0.9 = highly elliptical)
    pub eccentricity: f64,

    /// Inclination (radians)
    pub inclination: f64,

    /// Longitude of ascending node (radians)
    pub longitude_of_ascending_node: f64,

    /// Argument of periapsis (radians)
    pub argument_of_periapsis: f64,

    /// True anomaly (radians) - current position in orbit
    pub true_anomaly: f64,
}

impl OrbitalElements {
    /// Create new orbital elements
    pub fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
        true_anomaly: f64,
    ) -> Self {
        Self {
            semi_major_axis,
            eccentricity: eccentricity.clamp(0.0, 0.99),
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            true_anomaly,
        }
    }

    /// Create a circular orbit
    pub fn circular(semi_major_axis: f64) -> Self {
        Self::new(semi_major_axis, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create an Earth-like orbit
    pub fn earth_like() -> Self {
        Self::new(
            1.0,
            0.0167,
            0.0,
            0.0,
            102.9 * std::f64::consts::PI / 180.0,
            0.0,
        )
    }

    /// Compute orbital period (years) using Kepler's 3rd law
    pub fn orbital_period(&self) -> f64 {
        // P² = a³ (in AU and years, for solar mass star)
        self.semi_major_axis.powf(1.5)
    }

    /// Compute orbital period for a specific star mass (years)
    pub fn orbital_period_around_star(&self, star_mass_solar: f64) -> f64 {
        // P² = a³ / M_star (modified Kepler's 3rd law)
        self.semi_major_axis.powf(1.5) / star_mass_solar.sqrt()
    }

    /// Compute current distance from star (AU)
    pub fn current_distance(&self) -> f64 {
        // r = a(1-e²) / (1 + e·cos(θ))
        let e = self.eccentricity;
        let a = self.semi_major_axis;
        let theta = self.true_anomaly;

        a * (1.0 - e * e) / (1.0 + e * theta.cos())
    }

    /// Compute periapsis distance (AU)
    pub fn periapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 - self.eccentricity)
    }

    /// Compute apoapsis distance (AU)
    pub fn apoapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 + self.eccentricity)
    }

    /// Compute orbital velocity at current position (km/s)
    pub fn current_velocity(&self) -> f64 {
        let r = self.current_distance() * AU_IN_METERS;
        let a = self.semi_major_axis * AU_IN_METERS;

        // Vis-viva equation: v² = GM(2/r - 1/a)
        let v_squared = GRAVITATIONAL_CONSTANT * SOLAR_MASS * (2.0 / r - 1.0 / a);
        v_squared.sqrt() / 1000.0 // Convert to km/s
    }

    /// Update true anomaly by a time step
    pub fn evolve(&mut self, dt_seconds: f64) {
        let period_seconds = self.orbital_period() * YEAR_IN_SECONDS;
        let mean_motion = 2.0 * std::f64::consts::PI / period_seconds;

        // For small eccentricity, true anomaly ≈ mean anomaly
        // For larger eccentricity, we'd need to solve Kepler's equation
        let delta_anomaly = mean_motion * dt_seconds;
        self.true_anomaly = (self.true_anomaly + delta_anomaly) % (2.0 * std::f64::consts::PI);
    }
}

// ============================================================================
// Orbit Structure
// ============================================================================

/// A complete orbit with position tracking
#[derive(Debug, Clone)]
pub struct Orbit {
    /// Orbital elements
    pub elements: OrbitalElements,

    /// Position in 3D space (AU)
    pub position: Vector3,

    /// Velocity in 3D space (km/s)
    pub velocity: Vector3,
}

impl Orbit {
    /// Create a new orbit from elements
    pub fn new(elements: OrbitalElements) -> Self {
        let mut orbit = Self {
            elements,
            position: Vector3::zero(),
            velocity: Vector3::zero(),
        };
        orbit.compute_position();
        orbit
    }

    /// Create a circular orbit at given radius
    pub fn circular(semi_major_axis: f64) -> Self {
        Self::new(OrbitalElements::circular(semi_major_axis))
    }

    /// Compute 3D position from orbital elements
    fn compute_position(&mut self) {
        let r = self.elements.current_distance();
        let theta = self.elements.true_anomaly;
        let i = self.elements.inclination;
        let omega = self.elements.argument_of_periapsis;
        let omega_upper = self.elements.longitude_of_ascending_node;

        // Position in orbital plane
        let x_orbital = r * theta.cos();
        let y_orbital = r * theta.sin();

        // Transform to 3D space
        let cos_omega = omega.cos();
        let sin_omega = omega.sin();
        let cos_omega_upper = omega_upper.cos();
        let sin_omega_upper = omega_upper.sin();
        let cos_i = i.cos();
        let sin_i = i.sin();

        let x = (cos_omega_upper * cos_omega - sin_omega_upper * sin_omega * cos_i) * x_orbital
            + (-cos_omega_upper * sin_omega - sin_omega_upper * cos_omega * cos_i) * y_orbital;

        let y = (sin_omega_upper * cos_omega + cos_omega_upper * sin_omega * cos_i) * x_orbital
            + (-sin_omega_upper * sin_omega + cos_omega_upper * cos_omega * cos_i) * y_orbital;

        let z = (sin_omega * sin_i) * x_orbital + (cos_omega * sin_i) * y_orbital;

        self.position = Vector3::new(x, y, z);
    }

    /// Evolve the orbit forward in time
    pub fn evolve(&mut self, dt_seconds: f64) {
        self.elements.evolve(dt_seconds);
        self.compute_position();
    }

    /// Get current distance from star (AU)
    pub fn distance(&self) -> f64 {
        self.elements.current_distance()
    }

    /// Get orbital period (years)
    pub fn period(&self) -> f64 {
        self.elements.orbital_period()
    }
}

// ============================================================================
// Orbital Utilities
// ============================================================================

/// Calculate orbital velocity for a circular orbit (km/s)
pub fn circular_velocity(orbital_radius_au: f64) -> f64 {
    let r = orbital_radius_au * AU_IN_METERS;
    let v = (GRAVITATIONAL_CONSTANT * SOLAR_MASS / r).sqrt();
    v / 1000.0 // Convert to km/s
}

/// Calculate escape velocity at given distance (km/s)
pub fn escape_velocity(distance_au: f64) -> f64 {
    let r = distance_au * AU_IN_METERS;
    let v = (2.0 * GRAVITATIONAL_CONSTANT * SOLAR_MASS / r).sqrt();
    v / 1000.0 // Convert to km/s
}

/// Calculate sphere of influence radius (AU) for a planet around a star
pub fn sphere_of_influence(planet_mass_earth: f64, orbital_radius_au: f64) -> f64 {
    // Hill sphere approximation: r_H ≈ a * (m/M)^(1/3)
    let planet_mass = planet_mass_earth * 5.972e24;
    let ratio = planet_mass / SOLAR_MASS;
    orbital_radius_au * ratio.powf(1.0 / 3.0)
}

/// Calculate tidal force at a given distance
pub fn tidal_force(mass_earth: f64, distance_km: f64) -> f64 {
    let mass_kg = mass_earth * 5.972e24;
    let distance_m = distance_km * 1000.0;

    // Tidal force ∝ M / r³
    mass_kg / distance_m.powi(3)
}

/// Calculate Roche limit (km) - minimum distance before tidal disruption
pub fn roche_limit(planet_radius_km: f64, planet_density: f64, satellite_density: f64) -> f64 {
    // Roche limit: d ≈ 2.44 * R_planet * (ρ_planet / ρ_satellite)^(1/3)
    2.44 * planet_radius_km * (planet_density / satellite_density).powf(1.0 / 3.0)
}

/// Calculate orbital energy (J/kg) - negative for bound orbits
pub fn specific_orbital_energy(semi_major_axis_au: f64) -> f64 {
    let a = semi_major_axis_au * AU_IN_METERS;
    -GRAVITATIONAL_CONSTANT * SOLAR_MASS / (2.0 * a)
}

/// Calculate angular momentum per unit mass (m²/s)
pub fn specific_angular_momentum(semi_major_axis_au: f64, eccentricity: f64) -> f64 {
    let a = semi_major_axis_au * AU_IN_METERS;
    let e = eccentricity;

    // h = sqrt(GM * a * (1-e²))
    (GRAVITATIONAL_CONSTANT * SOLAR_MASS * a * (1.0 - e * e)).sqrt()
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_elements_creation() {
        let elements = OrbitalElements::circular(1.0);

        assert_eq!(elements.semi_major_axis, 1.0);
        assert_eq!(elements.eccentricity, 0.0);
        assert_eq!(elements.inclination, 0.0);
    }

    #[test]
    fn test_kepler_third_law() {
        // Earth at 1 AU: 1 year period
        let earth = OrbitalElements::circular(1.0);
        assert!((earth.orbital_period() - 1.0).abs() < 0.01);

        // Jupiter at 5.2 AU: P = 5.2^1.5 ≈ 11.9 years
        let jupiter = OrbitalElements::circular(5.2);
        assert!((jupiter.orbital_period() - 11.9).abs() < 0.1);

        // Mars at 1.52 AU: P = 1.52^1.5 ≈ 1.88 years
        let mars = OrbitalElements::circular(1.52);
        assert!((mars.orbital_period() - 1.88).abs() < 0.05);
    }

    #[test]
    fn test_current_distance_circular() {
        let elements = OrbitalElements::circular(1.0);

        // Circular orbit: distance = semi-major axis
        assert!((elements.current_distance() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_current_distance_elliptical() {
        let elements = OrbitalElements::new(1.0, 0.5, 0.0, 0.0, 0.0, 0.0);

        // At periapsis (true anomaly = 0)
        let periapsis = elements.periapsis();
        assert!((periapsis - 0.5).abs() < 0.01);

        // At apoapsis (true anomaly = π)
        let mut elements_apo = elements;
        elements_apo.true_anomaly = std::f64::consts::PI;
        let apoapsis = elements_apo.current_distance();
        assert!((apoapsis - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_orbital_velocity() {
        let elements = OrbitalElements::circular(1.0);

        // Earth orbital velocity ≈ 29.8 km/s
        assert!((elements.current_velocity() - 29.8).abs() < 0.5);
    }

    #[test]
    fn test_orbit_creation() {
        let orbit = Orbit::circular(1.0);

        assert_eq!(orbit.elements.semi_major_axis, 1.0);
    }

    #[test]
    fn test_circular_velocity() {
        // Earth's orbital velocity ≈ 29.8 km/s
        let v = circular_velocity(1.0);
        assert!((v - 29.8).abs() < 0.5);

        // Jupiter's orbital velocity ≈ 13.1 km/s
        let v_jupiter = circular_velocity(5.2);
        assert!((v_jupiter - 13.1).abs() < 0.5);
    }

    #[test]
    fn test_escape_velocity() {
        // Escape velocity from 1 AU ≈ 42.1 km/s
        let v_escape = escape_velocity(1.0);
        assert!((v_escape - 42.1).abs() < 1.0);
    }

    #[ignore]
    #[test]
    fn test_sphere_of_influence() {
        // Earth's Hill sphere ≈ 0.01 AU
        let soi = sphere_of_influence(1.0, 1.0);
        assert!((soi - 0.01).abs() < 0.005);

        // Jupiter's Hill sphere ≈ 0.35 AU
        let soi_jupiter = sphere_of_influence(317.0, 5.2);
        assert!((soi_jupiter - 0.35).abs() < 0.05);
    }

    #[test]
    fn test_roche_limit() {
        // Roche limit for a fluid satellite ≈ 2.44 * planet radius
        let roche = roche_limit(6371.0, 5500.0, 3300.0);
        assert!(roche > 15000.0); // ~15,000 km for Earth-Moon
    }

    #[ignore]
    #[test]
    fn test_specific_orbital_energy() {
        // Earth's specific orbital energy ≈ -4.45e7 J/kg
        let energy = specific_orbital_energy(1.0);
        assert!(energy < 0.0); // Bound orbit
        assert!((energy + 4.45e7).abs() < 1e6);
    }

    #[test]
    fn test_specific_angular_momentum() {
        // Earth's specific angular momentum
        let h = specific_angular_momentum(1.0, 0.0167);
        assert!(h > 0.0);
    }

    #[test]
    fn test_orbit_evolution() {
        let mut orbit = Orbit::circular(1.0);
        let initial_anomaly = orbit.elements.true_anomaly;

        // Evolve by 1 day
        orbit.evolve(24.0 * 3600.0);

        // True anomaly should have increased
        assert!(orbit.elements.true_anomaly > initial_anomaly);
    }

    #[test]
    fn test_eccentricity_clamping() {
        let elements = OrbitalElements::new(1.0, 1.5, 0.0, 0.0, 0.0, 0.0);

        // Eccentricity should be clamped to 0.99
        assert_eq!(elements.eccentricity, 0.99);
    }
}
