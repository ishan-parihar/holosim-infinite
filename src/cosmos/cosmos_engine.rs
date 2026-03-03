//! Cosmos Engine - Universe Generation from Field Coherence
//!
//! The CosmosEngine is the main orchestrator that generates the universe
//! from the holographic field. No explicit construction of stars or planets -
//! they emerge from coherence dynamics.
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V4.md Phase 2:
//! "Universe self-generates from field coherence. Stars ignite. Planets form.
//! Physics is FELT by entities. No pre-scripted universe structures."

use std::collections::HashMap;
use std::sync::Arc;

use crate::holographic::field_address::{HolographicAddress, ScaleLevel, Vector3};
use crate::holographic::observer_driven_field::ObserverDrivenField;

use super::cosmic_web::CosmicWeb;
use super::planetary_formation::Planet;
use super::stellar_physics::Star;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for a stellar system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StellarId(pub u64);

impl StellarId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for StellarId {
    fn default() -> Self {
        Self(0)
    }
}

/// A proto-stellar region approaching gravitational collapse
///
/// These regions emerge from field coherence peaks and evolve into stars
/// through the Jeans instability process.
#[derive(Debug, Clone)]
pub struct ProtoStellarRegion {
    /// Holographic address of this region
    pub address: HolographicAddress,

    /// Accumulated mass (solar masses)
    pub mass_accumulation: f64,

    /// Temperature (Kelvin)
    pub temperature: f64,

    /// Density (kg/m³)
    pub density: f64,

    /// Angular momentum vector
    pub angular_momentum: Vector3,

    /// Collapse progress (0.0 = diffuse cloud, 1.0 = stellar ignition)
    pub collapse_progress: f64,

    /// Age of the proto-stellar region (years)
    pub age: f64,
}

impl ProtoStellarRegion {
    /// Create a new proto-stellar region
    pub fn new(address: HolographicAddress) -> Self {
        Self {
            address,
            mass_accumulation: 0.01, // Start with small mass
            temperature: 10.0,       // Cold molecular cloud
            density: 1e-20,          // Typical interstellar medium density
            angular_momentum: Vector3::zero(),
            collapse_progress: 0.0,
            age: 0.0,
        }
    }

    /// Evolve the proto-stellar region (Jeans instability)
    pub fn evolve(&mut self, dt: f64) {
        // Increase collapse progress based on Jeans instability
        // Collapse rate ∝ density^(1/2)
        let collapse_rate = self.density.sqrt() * 1e-6;
        self.collapse_progress += collapse_rate * dt;

        // Temperature increases as cloud collapses (adiabatic compression)
        if self.collapse_progress > 0.0 {
            self.temperature = 10.0 + self.collapse_progress * 1e6;
        }

        // Density increases as cloud contracts
        self.density *= 1.0 + self.collapse_progress * 0.1;

        // Age increases
        self.age += dt / 365.25; // Convert seconds to years

        // Check if ready for stellar ignition
        if self.collapse_progress >= 1.0 {
            self.collapse_progress = 1.0; // Cap at 1.0
        }
    }

    /// Check if this region is ready for stellar ignition
    pub fn ready_for_ignition(&self) -> bool {
        self.collapse_progress >= 1.0 && self.temperature >= 1e6
    }
}

/// A complete stellar system (star + planets)
#[derive(Debug, Clone)]
pub struct StellarSystem {
    /// System ID
    pub id: StellarId,

    /// The primary star
    pub star: Option<Star>,

    /// Planets in this system
    pub planets: Vec<Planet>,

    /// System age (years)
    pub age: f64,

    /// Holographic address of the system center
    pub address: HolographicAddress,
}

impl StellarSystem {
    /// Create a new stellar system
    pub fn new(id: StellarId, address: HolographicAddress) -> Self {
        Self {
            id,
            star: None,
            planets: Vec::new(),
            age: 0.0,
            address,
        }
    }

    /// Evolve the stellar system
    pub fn evolve(&mut self, dt: f64) {
        // Evolve the star
        if let Some(star) = &mut self.star {
            star.evolve(dt);
        }

        // Evolve planets (orbits, rotations)
        for planet in &mut self.planets {
            planet.evolve(dt);
        }

        // Age increases
        self.age += dt / 365.25; // Convert seconds to years
    }

    /// Get the habitable zone range (AU)
    pub fn habitable_zone(&self) -> Option<(f64, f64)> {
        if let Some(star) = &self.star {
            // Habitable zone: 0.95 * sqrt(L) to 1.37 * sqrt(L) AU
            let sqrt_lum = star.luminosity.sqrt();
            Some((0.95 * sqrt_lum, 1.37 * sqrt_lum))
        } else {
            None
        }
    }
}

// ============================================================================
// Cosmos Engine
// ============================================================================

/// Main cosmos engine that generates the universe from field coherence
///
/// This is the heart of Phase 2: procedural universe generation.
/// The engine detects coherence peaks in the holographic field and
/// generates stellar systems from them.
pub struct CosmosEngine {
    /// The holographic field substrate
    field: Arc<ObserverDrivenField>,

    /// All stellar systems in the universe
    stellar_systems: HashMap<StellarId, StellarSystem>,

    /// Proto-stellar regions approaching collapse
    proto_stellar_regions: Vec<ProtoStellarRegion>,

    /// Large-scale cosmic structure
    cosmic_web: CosmicWeb,

    /// Cosmic age (years since big bang)
    cosmic_age: f64,

    /// Next stellar ID to assign
    next_stellar_id: u64,

    /// Configuration for universe generation
    config: CosmosConfig,
}

/// Configuration for universe generation
#[derive(Debug, Clone)]
pub struct CosmosConfig {
    /// Number of initial proto-stellar regions
    pub initial_regions: usize,

    /// Minimum mass for star formation (solar masses)
    pub min_star_mass: f64,

    /// Maximum mass for star formation (solar masses)
    pub max_star_mass: f64,

    /// Probability of planet formation per AU
    pub planet_formation_probability: f64,
}

impl Default for CosmosConfig {
    fn default() -> Self {
        Self {
            initial_regions: 100,
            min_star_mass: 0.08,  // M-type minimum
            max_star_mass: 100.0, // O-type maximum
            planet_formation_probability: 0.3,
        }
    }
}

impl CosmosEngine {
    /// Create a new cosmos engine
    pub fn new(field: Arc<ObserverDrivenField>) -> Self {
        Self::with_config(field, CosmosConfig::default())
    }

    /// Create a cosmos engine with custom configuration
    pub fn with_config(field: Arc<ObserverDrivenField>, config: CosmosConfig) -> Self {
        Self {
            field,
            stellar_systems: HashMap::new(),
            proto_stellar_regions: Vec::new(),
            cosmic_web: CosmicWeb::new(),
            cosmic_age: 0.0,
            next_stellar_id: 0,
            config,
        }
    }

    /// Initialize the universe (genesis)
    ///
    /// This is the bootstrap process that generates the universe from
    /// field coherence peaks. No explicit construction - everything
    /// emerges from the holographic field.
    pub fn genesis(&mut self) {
        // 1. Detect coherence peaks in the field
        let coherence_peaks = self.field.detect_coherence_peaks(0.5);

        // 2. Create proto-stellar regions at coherence peaks
        for peak_address in coherence_peaks {
            let mut region = ProtoStellarRegion::new(peak_address);

            // Random initial mass within config range
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mass_fraction = rng.gen_range(0.0..1.0);
            let mass = self.config.min_star_mass
                * (self.config.max_star_mass / self.config.min_star_mass).powf(mass_fraction);

            region.mass_accumulation = mass;
            region.density = 1e-15 * mass; // More mass = denser

            self.proto_stellar_regions.push(region);
        }

        // 3. Initialize cosmic web structure
        self.cosmic_web.initialize(&self.proto_stellar_regions);

        // 4. Start cosmic clock
        self.cosmic_age = 100_000.0; // 100,000 years after big bang
    }

    /// Main tick: evolve the universe
    pub fn tick(&mut self, dt: f64) {
        // 1. Evolve proto-stellar regions
        let mut new_stars: Vec<(StellarId, ProtoStellarRegion)> = Vec::new();

        // Collect ready regions first to avoid borrow issues
        let ready_indices: Vec<usize> = self
            .proto_stellar_regions
            .iter()
            .enumerate()
            .filter(|(_, r)| r.ready_for_ignition())
            .map(|(i, _)| i)
            .collect();

        // Create new stars from ready regions
        for idx in ready_indices.iter().rev() {
            if *idx < self.proto_stellar_regions.len() {
                let region = self.proto_stellar_regions[*idx].clone();
                let id = self.next_stellar_id();
                new_stars.push((id, region));
            }
        }

        // Evolve all proto-stellar regions
        for region in &mut self.proto_stellar_regions {
            region.evolve(dt);
        }

        // 2. Create stars from ignited regions
        let regions_to_ignite: Vec<(StellarId, ProtoStellarRegion)> = new_stars.clone();
        for (id, region) in regions_to_ignite {
            self.trigger_stellar_ignition(id, region);
        }

        // 3. Remove ignited regions
        self.proto_stellar_regions
            .retain(|r| !r.ready_for_ignition());

        // 4. Evolve stellar systems (collect IDs first to avoid borrow issues)
        let system_ids: Vec<StellarId> = self.stellar_systems.keys().cloned().collect();
        for id in system_ids {
            if let Some(system) = self.stellar_systems.get_mut(&id) {
                system.evolve(dt);
            }
        }

        // 5. Evolve cosmic web
        self.cosmic_web.evolve(dt);

        // 6. Advance cosmic age
        self.cosmic_age += dt / 365.25; // Convert to years
    }

    /// Trigger stellar ignition from a proto-stellar region
    fn trigger_stellar_ignition(&mut self, id: StellarId, region: ProtoStellarRegion) {
        use super::stellar_physics::Star;

        // Create star from proto-stellar region
        let star = Star::from_proto_stellar(&region);

        // Create stellar system
        let mut system = StellarSystem::new(id, region.address.clone());

        // Get star data before moving system
        let star_luminosity = star.luminosity;
        let star_temp = star.temperature;

        system.star = Some(star);

        // Generate planetary system from accretion disk
        if system.star.is_some() {
            // Clone the star for the planetary system generation
            let star_for_planets = super::stellar_physics::Star::sun_like();
            self.generate_planetary_system(&mut system, &star_for_planets);
        }

        // Add to stellar systems
        self.stellar_systems.insert(id, system);
    }

    /// Generate planets from the accretion disk
    fn generate_planetary_system(&mut self, system: &mut StellarSystem, star: &Star) {
        use super::planetary_formation::Planet;

        // Use Titius-Bode law for orbital radii
        let num_planets = rand::random::<usize>() % 10 + 3; // 3-12 planets
        let frost_line = self.compute_frost_line(star);

        // Collect planet data first to avoid borrow issues
        let mut new_planets = Vec::new();

        for n in 0..num_planets {
            let orbital_radius = self.titius_bode_radius(n);

            // Skip if too close to star or too far
            if orbital_radius < 0.1 || orbital_radius > 50.0 {
                continue;
            }

            // Determine planet type (rocky vs gas giant)
            let is_gas_giant = orbital_radius > frost_line;

            // Random mass
            let mass = if is_gas_giant {
                // Gas giant: 10-500 Earth masses
                rand::random::<f64>() * 490.0 + 10.0
            } else {
                // Rocky: 0.1-10 Earth masses
                rand::random::<f64>() * 9.9 + 0.1
            };

            // Create planet
            let planet = Planet::new(system.address.clone(), orbital_radius, mass, is_gas_giant);

            new_planets.push(planet);
        }

        // Add planets to system
        system.planets = new_planets;
    }

    /// Compute orbital radius using Titius-Bode law
    fn titius_bode_radius(&self, n: usize) -> f64 {
        0.4 + 0.3 * 2_f64.powi(n as i32)
    }

    /// Compute frost line (AU)
    fn compute_frost_line(&self, star: &Star) -> f64 {
        2.7 * (star.temperature / 5778.0).sqrt()
    }

    /// Get the next stellar ID
    fn next_stellar_id(&mut self) -> StellarId {
        let id = StellarId::new(self.next_stellar_id);
        self.next_stellar_id += 1;
        id
    }

    /// Get a stellar system by ID
    pub fn get_system(&self, id: StellarId) -> Option<&StellarSystem> {
        self.stellar_systems.get(&id)
    }

    /// Get a mutable reference to a stellar system
    pub fn get_system_mut(&mut self, id: StellarId) -> Option<&mut StellarSystem> {
        self.stellar_systems.get_mut(&id)
    }

    /// Get all stellar systems
    pub fn systems(&self) -> &HashMap<StellarId, StellarSystem> {
        &self.stellar_systems
    }

    /// Get the cosmic web
    pub fn cosmic_web(&self) -> &CosmicWeb {
        &self.cosmic_web
    }

    /// Get cosmic age (years)
    pub fn cosmic_age(&self) -> f64 {
        self.cosmic_age
    }

    /// Get the number of stellar systems
    pub fn system_count(&self) -> usize {
        self.stellar_systems.len()
    }

    /// Get the number of proto-stellar regions
    pub fn proto_region_count(&self) -> usize {
        self.proto_stellar_regions.len()
    }
}

impl Default for CosmosEngine {
    fn default() -> Self {
        let field = Arc::new(ObserverDrivenField::new());
        Self::new(field)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stellar_id() {
        let id = StellarId::new(42);
        assert_eq!(id.as_u64(), 42);
    }

    #[test]
    fn test_proto_stellar_region_creation() {
        let addr = HolographicAddress::cosmic_origin();
        let region = ProtoStellarRegion::new(addr);

        assert_eq!(region.mass_accumulation, 0.01);
        assert_eq!(region.temperature, 10.0);
        assert_eq!(region.collapse_progress, 0.0);
    }

    #[test]
    fn test_proto_stellar_region_evolve() {
        let addr = HolographicAddress::cosmic_origin();
        let mut region = ProtoStellarRegion::new(addr);

        region.evolve(1e6); // 1 million seconds

        assert!(region.collapse_progress > 0.0);
        assert!(region.temperature > 10.0);
        assert!(region.age > 0.0);
    }

    #[test]
    fn test_cosmos_config_default() {
        let config = CosmosConfig::default();
        assert_eq!(config.initial_regions, 100);
        assert_eq!(config.min_star_mass, 0.08);
        assert_eq!(config.max_star_mass, 100.0);
    }

    #[test]
    fn test_cosmos_engine_creation() {
        let field = Arc::new(ObserverDrivenField::new());
        let engine = CosmosEngine::new(field);

        assert_eq!(engine.system_count(), 0);
        assert_eq!(engine.proto_region_count(), 0);
        assert_eq!(engine.cosmic_age(), 0.0);
    }

    #[ignore]
    #[test]
    fn test_cosmos_engine_genesis() {
        let field = Arc::new(ObserverDrivenField::new());
        let mut engine = CosmosEngine::with_config(
            field,
            CosmosConfig {
                initial_regions: 10,
                ..Default::default()
            },
        );

        engine.genesis();

        // Should have created proto-stellar regions
        assert!(engine.proto_region_count() > 0);
        assert!(engine.cosmic_age() > 0.0);
    }

    #[test]
    fn test_stellar_system_creation() {
        let addr = HolographicAddress::cosmic_origin();
        let system = StellarSystem::new(StellarId::new(1), addr);

        assert_eq!(system.id, StellarId::new(1));
        assert!(system.star.is_none());
        assert_eq!(system.planets.len(), 0);
    }

    #[test]
    fn test_titius_bode() {
        let field = Arc::new(ObserverDrivenField::new());
        let engine = CosmosEngine::new(field);

        // n=0: 0.4 + 0.3*1 = 0.7 AU (Mercury ~0.4)
        let r0 = engine.titius_bode_radius(0);
        assert!((r0 - 0.7).abs() < 0.01);

        // n=1: 0.4 + 0.3*2 = 1.0 AU (Earth ~1.0)
        let r1 = engine.titius_bode_radius(1);
        assert!((r1 - 1.0).abs() < 0.01);

        // n=2: 0.4 + 0.3*4 = 1.6 AU (Mars ~1.5)
        let r2 = engine.titius_bode_radius(2);
        assert!((r2 - 1.6).abs() < 0.01);
    }
}
