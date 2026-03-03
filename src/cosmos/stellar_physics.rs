//! Stellar Physics - Star Lifecycle Simulation
//!
//! Stars are living fusion reactors with realistic physics.
//! This module implements stellar evolution from protostar to final remnant.
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V4.md Phase 2:
//! "Stars are living fusion reactors with lifecycles: main sequence → red giant
//! → white dwarf / neutron star / black hole. Their radiation determines
//! habitability."

use crate::holographic::field_address::Vector3;

// ============================================================================
// Constants
// ============================================================================

const SOLAR_MASS: f64 = 1.989e30; // kg
const SOLAR_RADIUS: f64 = 6.957e8; // m
const SOLAR_LUMINOSITY: f64 = 3.828e26; // W
const STEFAN_BOLTZMANN: f64 = 5.670374e-8; // W/(m²·K⁴)
const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³/(kg·s²)
const SOLAR_TEMPERATURE: f64 = 5778.0; // K

// ============================================================================
// Enums
// ============================================================================

/// Spectral class of a star (O-B-A-F-G-K-M)
///
/// Each class has characteristic temperature, mass, and luminosity ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectralClass {
    /// O-type: >30,000K, >30 solar masses, rare, short-lived
    O,
    /// B-type: 10,000-30,000K, 2-16 solar masses
    B,
    /// A-type: 7,500-10,000K, 1.4-2.1 solar masses
    A,
    /// F-type: 6,000-7,500K, 1.04-1.4 solar masses
    F,
    /// G-type: 5,200-6,000K, 0.8-1.04 solar masses (Sun-like)
    G,
    /// K-type: 3,700-5,200K, 0.45-0.8 solar masses
    K,
    /// M-type: 2,400-3,700K, 0.08-0.45 solar masses, common
    M,
}

impl SpectralClass {
    /// Get spectral class from temperature (Kelvin)
    pub fn from_temperature(temperature: f64) -> Self {
        match temperature {
            t if t >= 30000.0 => SpectralClass::O,
            t if t >= 10000.0 => SpectralClass::B,
            t if t >= 7500.0 => SpectralClass::A,
            t if t >= 6000.0 => SpectralClass::F,
            t if t >= 5200.0 => SpectralClass::G,
            t if t >= 3700.0 => SpectralClass::K,
            _ => SpectralClass::M,
        }
    }

    /// Get characteristic temperature for this spectral class
    pub fn characteristic_temperature(&self) -> f64 {
        match self {
            SpectralClass::O => 35000.0,
            SpectralClass::B => 20000.0,
            SpectralClass::A => 8750.0,
            SpectralClass::F => 6750.0,
            SpectralClass::G => 5600.0,
            SpectralClass::K => 4450.0,
            SpectralClass::M => 3050.0,
        }
    }

    /// Get spectral class color (RGB approximation)
    pub fn color(&self) -> (f64, f64, f64) {
        match self {
            SpectralClass::O => (0.6, 0.7, 1.0), // Blue
            SpectralClass::B => (0.7, 0.8, 1.0), // Blue-white
            SpectralClass::A => (0.9, 0.9, 1.0), // White
            SpectralClass::F => (1.0, 1.0, 0.9), // Yellow-white
            SpectralClass::G => (1.0, 1.0, 0.8), // Yellow (Sun)
            SpectralClass::K => (1.0, 0.8, 0.6), // Orange
            SpectralClass::M => (1.0, 0.6, 0.5), // Red
        }
    }
}

/// Stellar evolution stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StellarEvolutionStage {
    /// Forming from collapsing cloud
    ProtoStar,
    /// Main sequence (hydrogen fusion)
    MainSequence,
    /// Exhausting core hydrogen, beginning to expand
    SubGiant,
    /// Red giant phase (shell fusion)
    RedGiant,
    /// Horizontal branch (helium fusion)
    HorizontalBranch,
    /// Asymptotic giant branch
    AsymptoticGiant,
    /// Planetary nebula shedding outer layers
    PlanetaryNebula,
    /// White dwarf remnant
    WhiteDwarf,
    /// Neutron star remnant
    NeutronStar,
    /// Black hole remnant
    BlackHole,
    /// Supernova explosion
    Supernova,
}

// ============================================================================
// Radiation Spectrum
// ============================================================================

/// Radiation spectrum output from a star
#[derive(Debug, Clone)]
pub struct RadiationSpectrum {
    /// Total luminosity (Watts)
    pub luminosity: f64,
    /// Surface temperature (Kelvin)
    pub temperature: f64,
    /// Peak wavelength (meters)
    pub peak_wavelength: f64,
    /// UV fraction (0.0-1.0)
    pub uv_fraction: f64,
    /// Visible fraction (0.0-1.0)
    pub visible_fraction: f64,
    /// Infrared fraction (0.0-1.0)
    pub ir_fraction: f64,
}

impl RadiationSpectrum {
    /// Create radiation spectrum from luminosity and temperature
    pub fn new(luminosity: f64, temperature: f64) -> Self {
        // Wien's displacement law: λ_max = b/T
        let b_wien = 2.898e-3; // m·K
        let peak_wavelength = b_wien / temperature;

        // Estimate spectral fractions based on temperature
        let (uv, vis, ir) = Self::spectral_fractions(temperature);

        Self {
            luminosity,
            temperature,
            peak_wavelength,
            uv_fraction: uv,
            visible_fraction: vis,
            ir_fraction: ir,
        }
    }

    /// Compute spectral fractions from temperature
    fn spectral_fractions(temperature: f64) -> (f64, f64, f64) {
        // Simplified Planck distribution approximation
        // Hotter stars have more UV, cooler stars have more IR

        let normalized_temp = (temperature / SOLAR_TEMPERATURE).ln_1p();

        let uv = normalized_temp.max(0.0).min(0.4);
        let vis = 0.4 * (1.0 - normalized_temp.abs()).max(0.0);
        let ir = 1.0 - uv - vis;

        (uv, vis, ir)
    }

    /// Compute radiation flux at a given distance (W/m²)
    pub fn flux_at_distance(&self, distance_meters: f64) -> f64 {
        // Inverse square law: F = L / (4πd²)
        let surface_area = 4.0 * std::f64::consts::PI * distance_meters.powi(2);
        self.luminosity / surface_area
    }

    /// Compute radiation flux at distance in AU (W/m²)
    pub fn flux_at_au(&self, distance_au: f64) -> f64 {
        const AU_IN_METERS: f64 = 1.496e11;
        self.flux_at_distance(distance_au * AU_IN_METERS)
    }
}

// ============================================================================
// Star Structure
// ============================================================================

/// A living star with realistic fusion physics
#[derive(Debug, Clone)]
pub struct Star {
    /// Unique identifier
    pub id: u64,

    /// Spectral class (O-B-A-F-G-K-M)
    pub spectral_class: SpectralClass,

    /// Mass (solar masses)
    pub mass: f64,

    /// Luminosity (solar luminosities)
    pub luminosity: f64,

    /// Surface temperature (Kelvin)
    pub temperature: f64,

    /// Radius (solar radii)
    pub radius: f64,

    /// Age (years)
    pub age: f64,

    /// Fuel remaining (0.0 = exhausted, 1.0 = full)
    pub fuel_remaining: f64,

    /// Radiation output spectrum
    pub radiation_output: RadiationSpectrum,

    /// Rotational velocity (km/s at equator)
    pub rotation_velocity: f64,

    /// Evolutionary stage
    pub evolutionary_stage: StellarEvolutionStage,

    /// Metallicity (relative to Sun)
    pub metallicity: f64,
}

impl Star {
    /// Create a star from a proto-stellar region
    pub fn from_proto_stellar(proto: &crate::cosmos::cosmos_engine::ProtoStellarRegion) -> Self {
        let mass = proto.mass_accumulation; // Already in solar masses

        // Compute stellar properties from mass
        let spectral_class = Self::mass_to_spectral_class(mass);
        let luminosity = Self::mass_to_luminosity(mass);
        let radius = Self::mass_to_radius(mass);
        let temperature = Self::luminosity_to_temperature(luminosity, radius);
        let lifetime = Self::mass_to_lifetime(mass);

        // Create radiation spectrum
        let luminosity_watts = luminosity * SOLAR_LUMINOSITY;
        let radiation_output = RadiationSpectrum::new(luminosity_watts, temperature);

        Self {
            id: 0,
            spectral_class,
            mass,
            luminosity,
            temperature,
            radius,
            age: 0.0,
            fuel_remaining: 1.0,
            radiation_output,
            rotation_velocity: 2.0, // Typical solar rotation
            evolutionary_stage: StellarEvolutionStage::MainSequence,
            metallicity: 1.0, // Solar metallicity
        }
    }

    /// Create a sun-like star
    pub fn sun_like() -> Self {
        Self {
            id: 0,
            spectral_class: SpectralClass::G,
            mass: 1.0,
            luminosity: 1.0,
            temperature: SOLAR_TEMPERATURE,
            radius: 1.0,
            age: 4.6e9, // 4.6 billion years
            fuel_remaining: 0.5,
            radiation_output: RadiationSpectrum::new(SOLAR_LUMINOSITY, SOLAR_TEMPERATURE),
            rotation_velocity: 2.0,
            evolutionary_stage: StellarEvolutionStage::MainSequence,
            metallicity: 1.0,
        }
    }

    /// Evolve the star forward in time
    pub fn evolve(&mut self, dt: f64) {
        // Convert dt (seconds) to years
        let dt_years = dt / 365.25 / 24.0 / 3600.0;

        self.age += dt_years;

        // Update fuel based on luminosity
        let lifetime = Self::mass_to_lifetime(self.mass);
        self.fuel_remaining = 1.0 - (self.age / lifetime);

        // Clamp fuel
        self.fuel_remaining = self.fuel_remaining.max(0.0);

        // Update luminosity based on fuel and evolutionary stage
        self.update_properties();

        // Check for evolution stage transitions
        self.check_evolution_stage();
    }

    /// Update stellar properties based on current state
    fn update_properties(&mut self) {
        match self.evolutionary_stage {
            StellarEvolutionStage::MainSequence => {
                // Main sequence: L increases slightly as star ages
                let age_factor = 1.0 + 0.3 * (self.age / Self::mass_to_lifetime(self.mass));
                self.luminosity = Self::mass_to_luminosity(self.mass) * age_factor;
            }
            StellarEvolutionStage::RedGiant => {
                // Red giant: Much higher luminosity, cooler temperature
                self.luminosity = 100.0 * self.mass.powf(2.0);
                self.temperature = 3500.0;
                self.radius = 50.0 * self.mass.powf(0.5);
            }
            _ => {
                // Other stages: simplified
                self.luminosity = Self::mass_to_luminosity(self.mass);
            }
        }

        // Recalculate temperature and radiation
        self.temperature = Self::luminosity_to_temperature(self.luminosity, self.radius);
        self.spectral_class = SpectralClass::from_temperature(self.temperature);

        let luminosity_watts = self.luminosity * SOLAR_LUMINOSITY;
        self.radiation_output = RadiationSpectrum::new(luminosity_watts, self.temperature);
    }

    /// Check for evolutionary stage transitions
    fn check_evolution_stage(&mut self) {
        let lifetime = Self::mass_to_lifetime(self.mass);
        let main_sequence_fraction = self.age / lifetime;

        match self.evolutionary_stage {
            StellarEvolutionStage::MainSequence => {
                // Transition to red giant at ~80% of main sequence lifetime
                if main_sequence_fraction > 0.8 && self.mass > 0.5 {
                    self.evolutionary_stage = StellarEvolutionStage::RedGiant;
                }
            }
            StellarEvolutionStage::RedGiant => {
                // End of red giant phase
                if main_sequence_fraction > 0.95 {
                    if self.mass > 8.0 {
                        self.evolutionary_stage = StellarEvolutionStage::Supernova;
                    } else {
                        self.evolutionary_stage = StellarEvolutionStage::WhiteDwarf;
                    }
                }
            }
            _ => {}
        }
    }

    /// Check if star is still alive (not a remnant)
    pub fn is_alive(&self) -> bool {
        matches!(
            self.evolutionary_stage,
            StellarEvolutionStage::ProtoStar
                | StellarEvolutionStage::MainSequence
                | StellarEvolutionStage::SubGiant
                | StellarEvolutionStage::RedGiant
                | StellarEvolutionStage::HorizontalBranch
                | StellarEvolutionStage::AsymptoticGiant
        )
    }

    /// Get habitable zone range (AU)
    pub fn habitable_zone(&self) -> (f64, f64) {
        // 0.95 * sqrt(L) to 1.37 * sqrt(L) AU
        let sqrt_lum = self.luminosity.sqrt();
        (0.95 * sqrt_lum, 1.37 * sqrt_lum)
    }

    // ============================================================================
    // Physics Relations
    // ============================================================================

    /// Mass-luminosity relation: L ∝ M^3.5
    fn mass_to_luminosity(mass: f64) -> f64 {
        mass.powf(3.5)
    }

    /// Mass-radius relation: R ∝ M^0.8 (main sequence)
    fn mass_to_radius(mass: f64) -> f64 {
        if mass < 1.0 {
            // Lower mass stars are more compact
            mass.powf(0.8)
        } else {
            // Higher mass stars expand
            mass.powf(0.6)
        }
    }

    /// Main sequence lifetime: τ ∝ M^-2.5 (years)
    fn mass_to_lifetime(mass: f64) -> f64 {
        1.0e10 * mass.powf(-2.5)
    }

    /// Luminosity to temperature: L = 4πR²σT⁴
    fn luminosity_to_temperature(luminosity: f64, radius: f64) -> f64 {
        // T = (L / (4πR²σ))^0.25
        let radius_meters = radius * SOLAR_RADIUS;
        let luminosity_watts = luminosity * SOLAR_LUMINOSITY;
        let surface_area = 4.0 * std::f64::consts::PI * radius_meters.powi(2);

        (luminosity_watts / (surface_area * STEFAN_BOLTZMANN)).powf(0.25)
    }

    /// Mass to spectral class
    fn mass_to_spectral_class(mass: f64) -> SpectralClass {
        match mass {
            m if m >= 16.0 => SpectralClass::O,
            m if m >= 2.1 => SpectralClass::B,
            m if m >= 1.4 => SpectralClass::A,
            m if m >= 1.04 => SpectralClass::F,
            m if m >= 0.8 => SpectralClass::G,
            m if m >= 0.45 => SpectralClass::K,
            _ => SpectralClass::M,
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
    fn test_spectral_class_from_temperature() {
        assert_eq!(SpectralClass::from_temperature(35000.0), SpectralClass::O);
        assert_eq!(SpectralClass::from_temperature(20000.0), SpectralClass::B);
        assert_eq!(SpectralClass::from_temperature(5778.0), SpectralClass::G);
        assert_eq!(SpectralClass::from_temperature(3000.0), SpectralClass::M);
    }

    #[test]
    fn test_mass_luminosity_relation() {
        let l_sun = Star::mass_to_luminosity(1.0);
        assert!((l_sun - 1.0).abs() < 0.01);

        let l_2sun = Star::mass_to_luminosity(2.0);
        // 2^3.5 ≈ 11.3
        assert!((l_2sun - 11.3).abs() < 0.5);
    }

    #[test]
    fn test_mass_lifetime_relation() {
        let t_sun = Star::mass_to_lifetime(1.0);
        assert!((t_sun - 1e10).abs() < 1e8);

        let t_2sun = Star::mass_to_lifetime(2.0);
        // 2^-2.5 ≈ 0.177 * 10^10 ≈ 1.77e9
        assert!((t_2sun - 1.77e9).abs() < 1e8);
    }

    #[test]
    fn test_sun_like_star() {
        let sun = Star::sun_like();

        assert_eq!(sun.spectral_class, SpectralClass::G);
        assert_eq!(sun.mass, 1.0);
        assert_eq!(sun.luminosity, 1.0);
        assert_eq!(sun.temperature, SOLAR_TEMPERATURE);
    }

    #[test]
    fn test_radiation_spectrum() {
        let spectrum = RadiationSpectrum::new(SOLAR_LUMINOSITY, SOLAR_TEMPERATURE);

        assert_eq!(spectrum.luminosity, SOLAR_LUMINOSITY);
        assert_eq!(spectrum.temperature, SOLAR_TEMPERATURE);

        // Wien's law: λ_max ≈ 2.9e-3 / 5778 ≈ 502 nm (green)
        assert!((spectrum.peak_wavelength - 502e-9).abs() < 50e-9);
    }

    #[test]
    fn test_flux_at_distance() {
        let spectrum = RadiationSpectrum::new(SOLAR_LUMINOSITY, SOLAR_TEMPERATURE);

        // Solar constant: ~1361 W/m² at 1 AU
        let flux = spectrum.flux_at_au(1.0);
        assert!((flux - 1361.0).abs() < 50.0);
    }

    #[test]
    fn test_star_evolution() {
        let mut star = Star::sun_like();
        let initial_age = star.age;

        // Evolve by 1 million years
        star.evolve(365.25 * 24.0 * 3600.0 * 1e6);

        assert!(star.age > initial_age);
        assert!(star.fuel_remaining < 1.0);
    }

    #[test]
    fn test_habitable_zone() {
        let sun = Star::sun_like();
        let (inner, outer) = sun.habitable_zone();

        // Earth's orbit at 1 AU should be in habitable zone
        assert!(inner <= 1.0 && outer >= 1.0);
    }

    #[test]
    fn test_spectral_colors() {
        let o_color = SpectralClass::O.color();
        assert!(o_color.2 > o_color.0); // More blue than red

        let m_color = SpectralClass::M.color();
        assert!(m_color.0 > m_color.2); // More red than blue
    }
}
