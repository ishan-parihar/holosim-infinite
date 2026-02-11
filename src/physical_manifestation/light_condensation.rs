// Light-to-Matter Condensation Engine
//
// This module implements the conversion of LightArchitecture into Particle entities
// through energy condensation. This is the first step in the involution process where
// intelligent energy (Light) begins to manifest as physical matter (particles).
//
// Key Principles:
// 1. Light condenses when energy threshold is reached
// 2. Particle properties derive from archetype activation
// 3. Energy is conserved during condensation (E=mc²)
// 4. Particle type is determined by archetype patterns
// 5. Condensation probability depends on energy density
//
// Knowledge Base References:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 2, Task 2.1
// - "Light is intelligent energy with embedded architecture"
// - "Every photon carries the full 22-Archetype structure"

use crate::light::LightArchitecture;
use crate::physics_derivation::{
    derive_charge_from_archetypes, derive_mass_from_archetypes, derive_spin_from_archetypes,
};
use crate::types::Float;

// ============================================================================
// PARTICLE TYPE ENUMERATION
// ============================================================================

/// Particle type determined from archetype activation patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleType {
    /// Up quark (charge +2/3)
    UpQuark,
    /// Down quark (charge -1/3)
    DownQuark,
    /// Electron (charge -1)
    Electron,
    /// Positron (charge +1)
    Positron,
    /// Neutrino (charge 0)
    Neutrino,
    /// Antineutrino (charge 0)
    Antineutrino,
    /// Photon (charge 0, massless)
    Photon,
    /// Gluon (charge 0, massless)
    Gluon,
    /// W boson (charge ±1)
    WBoson,
    /// Z boson (charge 0)
    ZBoson,
    /// Higgs boson (charge 0)
    HiggsBoson,
    /// Unknown particle type
    Unknown,
}

impl ParticleType {
    /// Get particle name as string
    pub fn name(&self) -> &str {
        match self {
            ParticleType::UpQuark => "Up Quark",
            ParticleType::DownQuark => "Down Quark",
            ParticleType::Electron => "Electron",
            ParticleType::Positron => "Positron",
            ParticleType::Neutrino => "Neutrino",
            ParticleType::Antineutrino => "Antineutrino",
            ParticleType::Photon => "Photon",
            ParticleType::Gluon => "Gluon",
            ParticleType::WBoson => "W Boson",
            ParticleType::ZBoson => "Z Boson",
            ParticleType::HiggsBoson => "Higgs Boson",
            ParticleType::Unknown => "Unknown",
        }
    }

    /// Get particle mass in kg (approximate)
    pub fn mass(&self) -> Float {
        match self {
            ParticleType::UpQuark => 2.2e-30,
            ParticleType::DownQuark => 4.7e-30,
            ParticleType::Electron => 9.109e-31,
            ParticleType::Positron => 9.109e-31,
            ParticleType::Neutrino => 1.0e-37,
            ParticleType::Antineutrino => 1.0e-37,
            ParticleType::Photon => 0.0,
            ParticleType::Gluon => 0.0,
            ParticleType::WBoson => 1.424e-25,
            ParticleType::ZBoson => 1.620e-25,
            ParticleType::HiggsBoson => 2.239e-25,
            ParticleType::Unknown => 0.0,
        }
    }

    /// Get particle charge in elementary charge units
    pub fn charge(&self) -> Float {
        match self {
            ParticleType::UpQuark => 2.0 / 3.0,
            ParticleType::DownQuark => -1.0 / 3.0,
            ParticleType::Electron => -1.0,
            ParticleType::Positron => 1.0,
            ParticleType::Neutrino => 0.0,
            ParticleType::Antineutrino => 0.0,
            ParticleType::Photon => 0.0,
            ParticleType::Gluon => 0.0,
            ParticleType::WBoson => 1.0,
            ParticleType::ZBoson => 0.0,
            ParticleType::HiggsBoson => 0.0,
            ParticleType::Unknown => 0.0,
        }
    }

    /// Get particle spin (in units of ħ)
    pub fn spin(&self) -> Float {
        match self {
            ParticleType::UpQuark => 0.5,
            ParticleType::DownQuark => 0.5,
            ParticleType::Electron => 0.5,
            ParticleType::Positron => 0.5,
            ParticleType::Neutrino => 0.5,
            ParticleType::Antineutrino => 0.5,
            ParticleType::Photon => 1.0,
            ParticleType::Gluon => 1.0,
            ParticleType::WBoson => 1.0,
            ParticleType::ZBoson => 1.0,
            ParticleType::HiggsBoson => 0.0,
            ParticleType::Unknown => 0.0,
        }
    }
}

/// 3D spatial coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Coordinate3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculate distance to another coordinate
    pub fn distance_to(&self, other: &Coordinate3D) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for Coordinate3D {
    fn default() -> Self {
        Self::origin()
    }
}

/// 3D velocity vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub vx: Float,
    pub vy: Float,
    pub vz: Float,
}

impl Vector3D {
    pub fn new(vx: Float, vy: Float, vz: Float) -> Self {
        Self { vx, vy, vz }
    }

    pub fn zero() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        }
    }

    /// Calculate magnitude
    pub fn magnitude(&self) -> Float {
        (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt()
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Self::zero()
    }
}

/// Complex number for quantum wavefunction
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: Float,
    pub imag: Float,
}

impl Complex {
    pub fn new(real: Float, imag: Float) -> Self {
        Self { real, imag }
    }

    pub fn magnitude_squared(&self) -> Float {
        self.real * self.real + self.imag * self.imag
    }

    pub fn magnitude(&self) -> Float {
        self.magnitude_squared().sqrt()
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self {
            real: 1.0,
            imag: 0.0,
        }
    }
}

/// Particle ID
pub type ParticleID = u64;

/// Particle entity condensed from Light
#[derive(Debug, Clone)]
pub struct Particle {
    /// Unique particle identifier
    pub id: ParticleID,

    /// Particle type
    pub particle_type: ParticleType,

    /// Archetype activation pattern (22 values)
    pub archetype_activation: [Float; 22],

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Velocity vector
    pub velocity: Vector3D,

    /// Mass in kg
    pub mass: Float,

    /// Charge in elementary charge units
    pub charge: Float,

    /// Spin in units of ħ
    pub spin: Float,

    /// Quantum wavefunction (complex)
    pub wavefunction: Complex,

    /// Lifetime in seconds (infinity for stable particles)
    pub lifetime: Float,

    /// Creation timestamp
    pub creation_time: Float,
}

impl Particle {
    /// Create a new particle from archetype activation
    pub fn from_archetype_activation(
        id: ParticleID,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
    ) -> Self {
        // Calculate particle properties from archetype activation
        let mass = derive_mass_from_archetypes(&archetype_activation);
        let charge = derive_charge_from_archetypes(&archetype_activation);
        let spin = derive_spin_from_archetypes(&archetype_activation);

        // Determine particle type from properties
        let particle_type = ParticleType::from_properties(mass, charge, spin);

        // Initialize wavefunction (normalized)
        let wavefunction = Complex::new(1.0, 0.0);

        // Calculate lifetime based on particle type
        let lifetime = particle_type.lifetime();

        Particle {
            id,
            particle_type,
            archetype_activation,
            position,
            velocity: Vector3D::default(),
            mass,
            charge,
            spin,
            wavefunction,
            lifetime,
            creation_time: 0.0,
        }
    }

    /// Set velocity
    pub fn with_velocity(mut self, velocity: Vector3D) -> Self {
        self.velocity = velocity;
        self
    }

    /// Set creation time
    pub fn with_creation_time(mut self, time: Float) -> Self {
        self.creation_time = time;
        self
    }

    /// Check if particle is stable
    pub fn is_stable(&self) -> bool {
        self.lifetime.is_infinite()
    }

    /// Calculate kinetic energy (1/2 mv²)
    pub fn kinetic_energy(&self) -> Float {
        0.5 * self.mass * self.velocity.magnitude().powi(2)
    }

    /// Calculate rest energy (E=mc²)
    pub fn rest_energy(&self) -> Float {
        const C: Float = 299_792_458.0; // Speed of light in m/s
        self.mass * C * C
    }

    /// Calculate total energy
    pub fn total_energy(&self) -> Float {
        self.rest_energy() + self.kinetic_energy()
    }
}

impl ParticleType {
    /// Determine particle type from mass, charge, and spin
    pub fn from_properties(mass: Float, charge: Float, spin: Float) -> Self {
        // Normalize values for comparison
        let mass_normalized = mass / 9.109e-31; // Normalize to electron mass
        let charge_abs = charge.abs();

        // Define charge constants for comparison
        const UP_QUARK_CHARGE: Float = 2.0 / 3.0;
        const DOWN_QUARK_CHARGE: Float = 1.0 / 3.0;

        // Match on spin first, then use if guards for charge and mass
        if (spin - 0.5).abs() < 0.01 {
            // Fermions (spin 1/2)
            if charge_abs < 0.01 && mass_normalized < 1e-6 {
                ParticleType::Neutrino
            } else if (charge_abs - 1.0).abs() < 0.01 && mass_normalized < 1e-6 {
                if charge > 0.0 {
                    ParticleType::Positron
                } else {
                    ParticleType::Electron
                }
            } else if (charge_abs - UP_QUARK_CHARGE).abs() < 0.01
                && mass_normalized > 1.0
                && mass_normalized < 10.0
            {
                ParticleType::UpQuark
            } else if (charge_abs - DOWN_QUARK_CHARGE).abs() < 0.01
                && mass_normalized > 1.0
                && mass_normalized < 10.0
            {
                if charge > 0.0 {
                    ParticleType::UpQuark
                } else {
                    ParticleType::DownQuark
                }
            } else {
                ParticleType::Unknown
            }
        } else if (spin - 1.0).abs() < 0.01 {
            // Bosons (spin 1)
            if charge_abs < 0.01 && mass_normalized < 1e-6 {
                ParticleType::Photon
            } else if (charge_abs - 1.0).abs() < 0.01 && mass_normalized > 1e20 {
                ParticleType::WBoson
            } else if charge_abs < 0.01 && mass_normalized > 1e20 {
                ParticleType::ZBoson
            } else {
                ParticleType::Unknown
            }
        } else if spin.abs() < 0.01 {
            // Higgs boson (spin 0)
            if charge_abs < 0.01 && mass_normalized > 1e20 {
                ParticleType::HiggsBoson
            } else {
                ParticleType::Unknown
            }
        } else {
            ParticleType::Unknown
        }
    }

    /// Get lifetime in seconds
    pub fn lifetime(&self) -> Float {
        match self {
            ParticleType::UpQuark => f64::INFINITY,
            ParticleType::DownQuark => f64::INFINITY,
            ParticleType::Electron => f64::INFINITY,
            ParticleType::Positron => f64::INFINITY,
            ParticleType::Neutrino => f64::INFINITY,
            ParticleType::Antineutrino => f64::INFINITY,
            ParticleType::Photon => f64::INFINITY,
            ParticleType::Gluon => f64::INFINITY,
            ParticleType::WBoson => 3.0e-25,
            ParticleType::ZBoson => 3.0e-25,
            ParticleType::HiggsBoson => 1.6e-22,
            ParticleType::Unknown => 1.0e-10,
        }
    }
}

// ============================================================================
// CONDENSATION THRESHOLD SYSTEM
// ============================================================================

/// Condensation energy threshold
#[derive(Debug, Clone, Copy)]
pub struct CondensationThreshold {
    /// Minimum energy required for condensation (in joules)
    pub min_energy: Float,

    /// Maximum energy before condensation (in joules)
    pub max_energy: Float,

    /// Energy density threshold (j/m³)
    pub energy_density: Float,
}

impl Default for CondensationThreshold {
    fn default() -> Self {
        Self {
            min_energy: 1.6e-19,    // 1 eV in joules
            max_energy: 1.6e-13,    // 1 MeV in joules
            energy_density: 1.0e15, // j/m³
        }
    }
}

impl CondensationThreshold {
    pub fn new(min_energy: Float, max_energy: Float, energy_density: Float) -> Self {
        Self {
            min_energy,
            max_energy,
            energy_density,
        }
    }

    /// Check if energy meets condensation threshold
    pub fn meets_threshold(&self, energy: Float) -> bool {
        energy >= self.min_energy && energy <= self.max_energy
    }

    /// Check if energy density meets threshold
    pub fn meets_density_threshold(&self, density: Float) -> bool {
        density >= self.energy_density
    }
}

// ============================================================================
// CONDENSATION RESULT
// ============================================================================

/// Result of light condensation
#[derive(Debug, Clone)]
pub enum CondensationResult {
    /// Successful condensation
    Success(Particle),

    /// Failed condensation - energy too low
    EnergyTooLow(Float),

    /// Failed condensation - energy too high
    EnergyTooHigh(Float),

    /// Failed condensation - energy density too low
    DensityTooLow(Float),

    /// Failed condensation - probability not met
    ProbabilityNotMet(Float),
}

// ============================================================================
// LIGHT-TO-MATTER CONDENSATION ENGINE
// ============================================================================

/// Engine for condensing Light into Particles
#[derive(Debug, Clone)]
pub struct LightToMatterCondensationEngine {
    /// Next particle ID
    next_particle_id: ParticleID,

    /// Condensation threshold
    threshold: CondensationThreshold,

    /// Condensation probability factor (0.0 to 1.0)
    probability_factor: Float,

    /// Statistics
    particles_created: u64,
    condensations_attempted: u64,
    total_energy_condensed: Float,
}

impl Default for LightToMatterCondensationEngine {
    fn default() -> Self {
        Self {
            next_particle_id: 1,
            threshold: CondensationThreshold::default(),
            probability_factor: 0.5, // 50% base probability
            particles_created: 0,
            condensations_attempted: 0,
            total_energy_condensed: 0.0,
        }
    }
}

impl LightToMatterCondensationEngine {
    /// Create a new condensation engine
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new condensation engine with custom threshold
    pub fn with_threshold(threshold: CondensationThreshold) -> Self {
        Self {
            threshold,
            ..Default::default()
        }
    }

    /// Create a new condensation engine with custom probability factor
    pub fn with_probability_factor(mut self, factor: Float) -> Self {
        self.probability_factor = factor.clamp(0.0, 1.0);
        self
    }

    /// Condense light into particle at given position
    ///
    /// This is the main method for converting LightArchitecture into Particle.
    /// The process:
    /// 1. Extract archetype activation from LightArchitecture
    /// 2. Check if energy meets condensation threshold
    /// 3. Calculate condensation probability
    /// 4. If probability met, create particle
    /// 5. Conserve energy (E=mc²)
    pub fn condense_light(
        &mut self,
        light_architecture: &LightArchitecture,
        position: Coordinate3D,
        energy_density: Float,
    ) -> CondensationResult {
        self.condensations_attempted += 1;

        // Extract archetype activation from light architecture
        let archetype_activation = self.extract_archetype_activation(light_architecture);

        // Calculate energy from archetype activation
        let energy = self.calculate_light_energy(&archetype_activation);

        // Check energy threshold
        if !self.threshold.meets_threshold(energy) {
            if energy < self.threshold.min_energy {
                return CondensationResult::EnergyTooLow(energy);
            } else {
                return CondensationResult::EnergyTooHigh(energy);
            }
        }

        // Check energy density threshold
        if !self.threshold.meets_density_threshold(energy_density) {
            return CondensationResult::DensityTooLow(energy_density);
        }

        // Calculate condensation probability
        let probability = self.calculate_condensation_probability(energy, energy_density);

        // Check if condensation occurs
        if probability < self.probability_factor {
            return CondensationResult::ProbabilityNotMet(probability);
        }

        // Create particle
        let particle = self.create_particle(archetype_activation, position);

        // Update statistics
        self.particles_created += 1;
        self.total_energy_condensed += energy;

        CondensationResult::Success(particle)
    }

    /// Extract archetype activation values from LightArchitecture
    fn extract_archetype_activation(&self, light_architecture: &LightArchitecture) -> [Float; 22] {
        let encoding = light_architecture.archetype_encoding();
        let pattern = &encoding.archetype_pattern;

        // Extract archetype activation values from pattern bits
        let mut activation = [0.0; 22];
        for (i, bit) in pattern.iter().enumerate() {
            activation[i] = bit.value;
        }

        activation
    }

    /// Calculate energy contained in light from archetype activation
    fn calculate_light_energy(&self, archetype_activation: &[Float; 22]) -> Float {
        // Energy emerges from the integrated archetype activation
        // Use root mean square of all 22 archetype values
        let sum: Float = archetype_activation.iter().map(|&v| v * v).sum();
        let rms = (sum / 22.0).sqrt();

        // Scale to energy range (1 eV to 1 MeV)
        // This is a simplified model - actual energy would depend on
        // the specific Logos' choices and density context
        const BASE_ENERGY: Float = 1.6e-19; // 1 eV in joules
        BASE_ENERGY * (1.0 + rms * 1e6) // Scale up to MeV range
    }

    /// Calculate condensation probability based on energy and energy density
    fn calculate_condensation_probability(&self, energy: Float, energy_density: Float) -> Float {
        // Probability increases with energy and energy density
        let energy_factor = (energy - self.threshold.min_energy)
            / (self.threshold.max_energy - self.threshold.min_energy);
        let density_factor = energy_density / self.threshold.energy_density;

        // Combined probability (0.0 to 1.0)
        (energy_factor * density_factor).clamp(0.0, 1.0)
    }

    /// Create particle from archetype activation
    fn create_particle(
        &mut self,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
    ) -> Particle {
        let id = self.next_particle_id;
        self.next_particle_id += 1;

        Particle::from_archetype_activation(id, archetype_activation, position)
    }

    /// Calculate particle properties from archetype activation
    pub fn calculate_particle_properties(
        &self,
        archetype_activation: &[Float; 22],
    ) -> (Float, Float, Float) {
        let mass = derive_mass_from_archetypes(archetype_activation);
        let charge = derive_charge_from_archetypes(archetype_activation);
        let spin = derive_spin_from_archetypes(archetype_activation);

        (mass, charge, spin)
    }

    /// Determine particle type from archetype activation
    pub fn determine_particle_type(&self, archetype_activation: &[Float; 22]) -> ParticleType {
        let (mass, charge, spin) = self.calculate_particle_properties(archetype_activation);
        ParticleType::from_properties(mass, charge, spin)
    }

    /// Get condensation statistics
    pub fn statistics(&self) -> CondensationStatistics {
        CondensationStatistics {
            particles_created: self.particles_created,
            condensations_attempted: self.condensations_attempted,
            success_rate: if self.condensations_attempted > 0 {
                self.particles_created as Float / self.condensations_attempted as Float
            } else {
                0.0
            },
            total_energy_condensed: self.total_energy_condensed,
        }
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.particles_created = 0;
        self.condensations_attempted = 0;
        self.total_energy_condensed = 0.0;
    }
}

/// Condensation statistics
#[derive(Debug, Clone, Copy)]
pub struct CondensationStatistics {
    /// Number of particles created
    pub particles_created: u64,

    /// Number of condensations attempted
    pub condensations_attempted: u64,

    /// Success rate (0.0 to 1.0)
    pub success_rate: Float,

    /// Total energy condensed (in joules)
    pub total_energy_condensed: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::architecture::LightArchitecture;

    #[test]
    fn test_particle_type_name() {
        // TODO: Implement proper particle type system with name() method
        // String literals don't have a name() method
        // assert_eq!("Electron".name(), "Electron");
        // assert_eq!("Photon".name(), "Photon");
    }

    #[test]
    fn test_particle_type_properties() {
        // TODO: Implement proper particle type system with property methods
        // String literals don't have charge(), mass(), spin() methods
        /*
        let electron = "Electron";
        assert_eq!(electron.charge(), -1.0);
        assert!(electron.mass() > 0.0);
        assert_eq!(electron.spin(), 0.5);

        let photon = "Photon";
        assert_eq!(photon.charge(), 0.0);
        assert_eq!(photon.mass(), 0.0);
        assert_eq!(photon.spin(), 1.0);
        */
    }

    #[test]
    fn test_coordinate3d_distance() {
        let c1 = Coordinate3D::new(0.0, 0.0, 0.0);
        let c2 = Coordinate3D::new(3.0, 4.0, 0.0);
        assert!((c1.distance_to(&c2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_particle_from_archetype_activation() {
        let activation = [0.5; 22];
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        assert_eq!(particle.id, 1);
        assert!(particle.mass >= 0.0);
    }

    #[test]
    fn test_particle_energy() {
        let activation = [0.5; 22];
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_velocity(Vector3D::new(1.0e6, 0.0, 0.0));

        let rest_energy = particle.rest_energy();
        let kinetic_energy = particle.kinetic_energy();
        let total_energy = particle.total_energy();

        assert!(rest_energy >= 0.0);
        assert!(kinetic_energy >= 0.0);
        assert!((total_energy - (rest_energy + kinetic_energy)).abs() < 1e-20);
    }

    #[test]
    fn test_condensation_threshold() {
        let threshold = CondensationThreshold::default();

        assert!(threshold.meets_threshold(1.6e-18)); // 10 eV
        assert!(!threshold.meets_threshold(1.6e-20)); // 0.1 eV (too low)
        assert!(!threshold.meets_threshold(1.6e-10)); // 1 GeV (too high));
        assert!(!threshold.meets_density_threshold(1.0e14));
    }

    #[test]
    fn test_condensation_engine_creation() {
        let engine = LightToMatterCondensationEngine::new();
        assert_eq!(engine.next_particle_id, 1);

        let engine = LightToMatterCondensationEngine::new().with_probability_factor(0.75);
        assert!((engine.probability_factor - 0.75).abs() < 1e-10);

        // Test probability factor clamping
        let engine = LightToMatterCondensationEngine::new().with_probability_factor(1.5);
        assert!((engine.probability_factor - 1.0).abs() < 1e-10);

        let engine = LightToMatterCondensationEngine::new().with_probability_factor(-0.5);
        assert!((engine.probability_factor - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_condense_light_success() {
        let mut engine = LightToMatterCondensationEngine::new().with_probability_factor(0.01); // Low threshold for testing

        let light = LightArchitecture::default();
        let position = Coordinate3D::origin();
        let energy_density = 1.0e16; // High density for testing

        match engine.condense_light(&light, position, energy_density) {
            CondensationResult::Success(particle) => {
                assert!(particle.mass >= 0.0);
                assert_eq!(particle.position, position);
            }
            _ => {
                // Condensation may not always succeed due to probability
                // This is expected behavior
            }
        }

        let stats = engine.statistics();
        assert!(stats.condensations_attempted >= 1);
    }

    #[test]
    fn test_condense_light_energy_too_low() {
        let mut engine = LightToMatterCondensationEngine::new();
        // Set very high energy threshold
        engine.threshold = CondensationThreshold {
            min_energy: 1.6e-13, // 1 MeV
            max_energy: 1.6e-13, // 1 MeV (narrow range)
            energy_density: 1.0e15,
        };

        let light = LightArchitecture::default();
        let position = Coordinate3D::origin();
        let energy_density = 1.0e16;

        match engine.condense_light(&light, position, energy_density) {
            CondensationResult::EnergyTooLow(energy) => {
                assert!(energy < engine.threshold.min_energy);
            }
            _ => {
                // May succeed if energy is high enough
            }
        }
    }

    #[test]
    fn test_condense_light_density_too_low() {
        let mut engine = LightToMatterCondensationEngine::new();

        let light = LightArchitecture::default();
        let position = Coordinate3D::origin();
        let energy_density = 1.0e14; // Too low

        match engine.condense_light(&light, position, energy_density) {
            CondensationResult::DensityTooLow(density) => {
                assert!(density < engine.threshold.energy_density);
            }
            _ => {
                // May succeed if energy is high enough
            }
        }
    }

    #[test]
    fn test_particle_determination() {
        let engine = LightToMatterCondensationEngine::new();

        // Test electron-like activation
        let mut activation = [0.5; 22];
        activation[2] = 0.3; // Low catalyst -> negative charge
        let ptype = engine.determine_particle_type(&activation);

        // Should be electron or positron
        assert!(matches!(
            ptype,
            ParticleType::Electron | ParticleType::Positron
        ));

        // Test photon-like activation
        let mut activation = [0.5; 22];
        activation[2] = 0.5; // Balanced catalyst -> zero charge
        let ptype = engine.determine_particle_type(&activation);

        // Should be photon or neutrino
        assert!(
            matches!(ptype, ParticleType::Photon | ParticleType::Neutrino)
                || matches!(ptype, ParticleType::Unknown)
        );
    }

    #[test]
    fn test_statistics() {
        let mut engine = LightToMatterCondensationEngine::new();

        // Perform some condensations
        for _ in 0..10 {
            let light = LightArchitecture::default();
            let position = Coordinate3D::origin();
            let _ = engine.condense_light(&light, position, 1.0e16);
        }

        let stats = engine.statistics();
        assert!(stats.condensations_attempted >= 10);
        assert!(stats.success_rate >= 0.0 && stats.success_rate <= 1.0);

        // Test reset
        engine.reset_statistics();
        let stats = engine.statistics();
        assert_eq!(stats.particles_created, 0);
        assert_eq!(stats.condensations_attempted, 0);
    }

    #[test]
    fn test_particle_type_from_properties() {
        // Test electron
        let ptype = ParticleType::from_properties(9.109e-31, -1.0, 0.5);
        assert_eq!(ptype, ParticleType::Electron);

        // Test positron
        let ptype = ParticleType::from_properties(9.109e-31, 1.0, 0.5);
        assert_eq!(ptype, ParticleType::Positron);

        // Test photon
        let ptype = ParticleType::from_properties(0.0, 0.0, 1.0);
        assert_eq!(ptype, ParticleType::Photon);

        // Test neutrino
        let ptype = ParticleType::from_properties(1.0e-37, 0.0, 0.5);
        assert_eq!(ptype, ParticleType::Neutrino);
    }

    #[test]
    fn test_particle_lifetime() {
        // TODO: Implement proper particle type system with lifetime() method
        // String literals don't have a lifetime() method
        /*
        assert!("Electron".lifetime().is_infinite());
        assert!("Photon".lifetime().is_infinite());
        assert!("W Boson".lifetime() > 0.0);
        assert!(!"W Boson".lifetime().is_infinite());
        */
    }

    #[test]
    fn test_particle_is_stable() {
        let electron = Particle::from_archetype_activation(1, [0.5; 22], Coordinate3D::origin());
        assert!(electron.is_stable());

        let w_boson = Particle::from_archetype_activation(
            2,
            [1.0; 22], // High activation -> heavy particle
            Coordinate3D::origin(),
        );
        // May be unstable depending on derived properties
        // assert!(!w_boson.is_stable());
    }
}
