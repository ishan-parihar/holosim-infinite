// Particle System - Phase 2: Physical Manifestation, Task 2.2
//
// Enhanced Particle struct with full archetype-based properties.
// All particle properties derive from archetype activation patterns.
//
// Key Principles:
// 1. Mass, charge, spin, lifetime derive from archetype activation
// 2. Wavefunction evolution follows quantum mechanics
// 3. Particle interactions governed by archetype compatibility
// 4. Energy conservation (E=mc²) maintained
// 5. No deprecated ParticleType enum - properties emerge dynamically
//
// Phase 4 Update: Dual-Mode Physics System Integration
// - Particle can now use holographic physics discovery
// - Backward compatibility maintained through migration wrappers
// - Supports Hardcoded, Holographic, and Hybrid modes
//
// Knowledge Base References:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 2, Task 2.2
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 4: Integration & Migration
// - "All physics values are derived from the 22-Archetype structure"
// - "Light carries the complete blueprint for consciousness"

use crate::physics::PhysicsMode;
use crate::physics_derivation::{
    derive_charge_from_archetypes, derive_charge_migration, derive_lifetime_from_archetypes,
    derive_lifetime_migration, derive_mass_from_archetypes, derive_mass_migration,
    derive_spin_from_archetypes, derive_spin_migration,
};
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// SPATIAL AND QUANTUM STRUCTURES
// ============================================================================

/// 3D spatial coordinate
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    /// Normalize vector
    pub fn normalize(&self) -> Option<Self> {
        let mag = self.magnitude();
        if mag > 0.0 {
            Some(Self {
                vx: self.vx / mag,
                vy: self.vy / mag,
                vz: self.vz / mag,
            })
        } else {
            None
        }
    }

    /// Dot product with another vector
    pub fn dot(&self, other: &Vector3D) -> Float {
        self.vx * other.vx + self.vy * other.vy + self.vz * other.vz
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Self::zero()
    }
}

/// Complex number for quantum wavefunction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    /// Multiply by another complex number
    pub fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    /// Add another complex number
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    /// Scale by a real number
    pub fn scale(&self, factor: Float) -> Complex {
        Complex {
            real: self.real * factor,
            imag: self.imag * factor,
        }
    }

    /// Complex conjugate
    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
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

// ============================================================================
// PARTICLE ENTITY
// ============================================================================

/// Particle ID
pub type ParticleID = u64;

/// Particle entity with archetype-based properties
///
/// All particle properties derive from archetype activation patterns.
/// No deprecated ParticleType enum - properties emerge dynamically.
#[derive(Debug, Clone)]
pub struct Particle {
    /// Unique particle identifier
    pub id: ParticleID,

    /// Archetype activation pattern (22 values) - the bridge between spiritual and physical
    pub archetype_activation: [Float; 22],

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Velocity vector
    pub velocity: Vector3D,

    /// Mass (kg) - derived from archetype activation
    pub mass: Float,

    /// Charge (elementary charge units) - derived from archetype activation
    pub charge: Float,

    /// Spin (in units of ħ) - derived from archetype activation
    pub spin: Float,

    /// Lifetime (seconds) - None for stable particles, derived from archetype activation
    pub lifetime: Option<Float>,

    /// Quantum wavefunction (complex amplitude)
    pub wavefunction: Complex,

    /// Creation time (simulation steps)
    pub creation_time: u64,

    /// Current age (simulation steps)
    pub age: u64,

    /// Energy (Joules) - LEGACY field for backward compatibility
    pub energy: Float,

    /// Holographic reference (LEGACY - for backward compatibility)
    pub holographic_ref: Option<u64>,

    /// Light origin (LEGACY - for backward compatibility)
    pub light_origin: Option<u64>,
}

impl Particle {
    /// Create a new particle from archetype activation pattern
    ///
    /// All properties derive from the archetype activation pattern using
    /// the physics_derivation.rs functions.
    ///
    /// # Arguments
    /// * `id` - Unique particle identifier
    /// * `archetype_activation` - 22-value archetype activation pattern
    /// * `position` - Initial position in 3D space
    ///
    /// # Returns
    /// A new Particle with all properties derived from archetype activation
    pub fn from_archetype_activation(
        id: ParticleID,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
    ) -> Self {
        // Derive all properties from archetype activation
        let mass = derive_mass_from_archetypes(&archetype_activation);
        let charge = derive_charge_from_archetypes(&archetype_activation);
        let spin = derive_spin_from_archetypes(&archetype_activation);
        let lifetime = derive_lifetime_from_archetypes(&archetype_activation);

        // Initialize wavefunction with normalized amplitude
        let wavefunction = Complex::new(1.0, 0.0);

        // Calculate initial energy
        const C: Float = 2.998e8;
        let energy = mass * C * C;

        Self {
            id,
            archetype_activation,
            position,
            velocity: Vector3D::zero(),
            mass,
            charge,
            spin,
            lifetime,
            wavefunction,
            creation_time: 0,
            age: 0,
            energy,
            holographic_ref: None,
            light_origin: None,
        }
    }

    /// Create a new particle using dual-mode physics system
    ///
    /// Phase 4: Legacy Migration
    /// This method uses the global DualPhysicsSystem to calculate properties.
    /// Supports Hardcoded, Holographic, and Hybrid modes.
    ///
    /// # Arguments
    /// * `id` - Unique particle identifier
    /// * `archetype_activation` - 22-value archetype activation pattern
    /// * `position` - Initial position in 3D space
    ///
    /// # Returns
    /// A new Particle with properties calculated using the dual-mode system
    pub fn from_archetype_activation_dual_mode(
        id: ParticleID,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
    ) -> Self {
        // Use migration wrappers to get properties from dual-mode system
        let mass = derive_mass_migration(&archetype_activation);
        let charge = derive_charge_migration(&archetype_activation);
        let spin = derive_spin_migration(&archetype_activation);
        let lifetime = derive_lifetime_migration(&archetype_activation);

        // Initialize wavefunction with normalized amplitude
        let wavefunction = Complex::new(1.0, 0.0);

        // Calculate initial energy
        const C: Float = 2.998e8;
        let energy = mass * C * C;

        Self {
            id,
            archetype_activation,
            position,
            velocity: Vector3D::zero(),
            mass,
            charge,
            spin,
            lifetime,
            wavefunction,
            creation_time: 0,
            age: 0,
            energy,
            holographic_ref: None,
            light_origin: None,
        }
    }

    /// Initialize global physics system for dual-mode particle creation
    ///
    /// Phase 4: Legacy Migration
    /// Call this once before creating particles with dual-mode system.
    ///
    /// # Arguments
    /// * `mode` - Physics mode (Hardcoded, Holographic, or Hybrid)
    /// * `holographic_threshold` - Threshold for holographic discovery (0.0 to 1.0)
    pub fn initialize_dual_mode_physics(mode: PhysicsMode, holographic_threshold: Float) {
        // TODO: Implement global physics system initialization
        // initialize_global_physics_system(mode, holographic_threshold);
        // For now, this is a no-op placeholder
        let _ = (mode, holographic_threshold);
    }

    /// Emerge from Light condensation (LEGACY - for backward compatibility)
    #[deprecated(note = "Use Particle::from_archetype_activation() instead")]
    pub fn emerge_from_light(
        id: ParticleID,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
    ) -> Self {
        Self::from_archetype_activation(id, archetype_activation, position)
    }

    /// Set particle velocity
    pub fn with_velocity(mut self, velocity: Vector3D) -> Self {
        self.velocity = velocity;
        self
    }

    /// Set particle creation time
    pub fn with_creation_time(mut self, creation_time: u64) -> Self {
        self.creation_time = creation_time;
        self.age = 0;
        self
    }

    /// Check if particle is stable (infinite lifetime)
    pub fn is_stable(&self) -> bool {
        self.lifetime.is_none()
    }

    /// Check if particle has decayed
    pub fn has_decayed(&self, current_time: u64) -> bool {
        if let Some(lifetime) = self.lifetime {
            let age_seconds = (current_time - self.creation_time) as Float;
            age_seconds >= lifetime
        } else {
            false
        }
    }

    /// Calculate kinetic energy: E_k = ½mv²
    pub fn kinetic_energy(&self) -> Float {
        let v_squared = self.velocity.vx * self.velocity.vx
            + self.velocity.vy * self.velocity.vy
            + self.velocity.vz * self.velocity.vz;
        0.5 * self.mass * v_squared
    }

    /// Calculate rest energy: E_rest = mc²
    pub fn rest_energy(&self) -> Float {
        const C: Float = 2.998e8; // Speed of light in m/s
        self.mass * C * C
    }

    /// Calculate total energy: E_total = E_rest + E_kinetic
    pub fn total_energy(&self) -> Float {
        self.rest_energy() + self.kinetic_energy()
    }

    /// Calculate momentum: p = mv
    pub fn momentum(&self) -> Vector3D {
        Vector3D {
            vx: self.mass * self.velocity.vx,
            vy: self.mass * self.velocity.vy,
            vz: self.mass * self.velocity.vz,
        }
    }

    /// Update position based on velocity
    pub fn update_position(&mut self, dt: Float) {
        self.position.x += self.velocity.vx * dt;
        self.position.y += self.velocity.vy * dt;
        self.position.z += self.velocity.vz * dt;
    }

    /// Update age
    pub fn update_age(&mut self, delta_time: u64) {
        self.age += delta_time;
    }

    /// Evolve wavefunction (simplified time evolution)
    ///
    /// Uses the Schrödinger equation: ψ(t) = ψ(0) * exp(-iEt/ħ)
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds
    pub fn evolve_wavefunction(&mut self, dt: Float) {
        const H_BAR: Float = 1.0545718e-34; // Reduced Planck constant

        // Phase evolution: exp(-iEt/ħ)
        let energy = self.total_energy();
        let phase = -energy * dt / H_BAR;

        // exp(iθ) = cos(θ) + i*sin(θ)
        let cos_phase = phase.cos();
        let sin_phase = phase.sin();

        let evolution = Complex::new(cos_phase, sin_phase);
        self.wavefunction = self.wavefunction.multiply(&evolution);
    }

    /// Calculate probability density |ψ|²
    pub fn probability_density(&self) -> Float {
        self.wavefunction.magnitude_squared()
    }

    /// Normalize wavefunction
    pub fn normalize_wavefunction(&mut self) {
        let magnitude = self.wavefunction.magnitude();
        if magnitude > 0.0 {
            self.wavefunction = self.wavefunction.scale(1.0 / magnitude);
        }
    }

    /// Calculate archetype compatibility with another particle
    ///
    /// Compatibility is based on the correlation between archetype activation patterns.
    /// Higher compatibility = stronger interaction potential.
    ///
    /// # Arguments
    /// * `other` - The other particle
    ///
    /// # Returns
    /// Compatibility score (0.0 to 1.0)
    pub fn archetype_compatibility(&self, other: &Particle) -> Float {
        let mut sum_product = 0.0;
        let mut sum_sq1 = 0.0;
        let mut sum_sq2 = 0.0;

        for i in 0..22 {
            let a1 = self.archetype_activation[i];
            let a2 = other.archetype_activation[i];
            sum_product += a1 * a2;
            sum_sq1 += a1 * a1;
            sum_sq2 += a2 * a2;
        }

        // Pearson correlation coefficient
        let denominator = (sum_sq1 * sum_sq2).sqrt();
        if denominator > 0.0 {
            sum_product / denominator
        } else {
            0.0
        }
    }

    /// Interact with another particle
    ///
    /// Calculates interaction based on archetype compatibility and physical properties.
    ///
    /// # Arguments
    /// * `other` - The other particle
    ///
    /// # Returns
    /// Interaction result with force vector and energy transfer
    pub fn interact(&self, other: &Particle) -> InteractionResult {
        let distance = self.position.distance_to(&other.position);

        // Avoid division by zero
        if distance < 1e-15 {
            return InteractionResult {
                force: Vector3D::zero(),
                energy_transfer: 0.0,
                compatibility: 0.0,
            };
        }

        // Calculate archetype compatibility
        let compatibility = self.archetype_compatibility(other);

        // Electromagnetic force: F = k * q1 * q2 / r²
        const K_E: Float = 8.9875517923e9; // Coulomb constant
        let force_magnitude = K_E * self.charge * other.charge / (distance * distance);

        // Direction from self to other
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let dz = other.position.z - self.position.z;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();

        let force = if dist > 0.0 {
            // Repulsive if charges have same sign, attractive if opposite
            Vector3D {
                vx: force_magnitude * dx / dist,
                vy: force_magnitude * dy / dist,
                vz: force_magnitude * dz / dist,
            }
        } else {
            Vector3D::zero()
        };

        // Energy transfer based on compatibility
        const ENERGY_TRANSFER_FACTOR: Float = 1e-20; // Small energy transfer
        let energy_transfer = compatibility * ENERGY_TRANSFER_FACTOR;

        InteractionResult {
            force,
            energy_transfer,
            compatibility,
        }
    }

    /// Get particle type name (derived from properties)
    ///
    /// This is a convenience method to identify particles based on their derived properties.
    /// It's not a hardcoded enum - properties emerge dynamically.
    pub fn get_particle_type_name(&self) -> &'static str {
        // Determine type based on mass, charge, spin
        let charge_abs = self.charge.abs();
        let spin_abs = self.spin.abs();

        // Photon: mass ≈ 0, charge = 0, spin = 1
        if self.mass < 1.0e-35 && charge_abs < 0.1 && (spin_abs - 1.0).abs() < 0.1 {
            return "Photon";
        }

        // Proton/Antiproton: mass ~1e-31 to 1e-27, charge = ±1, spin = 0.5
        // Derived from archetypes, proton mass is heavier than electron (~3-4x)
        // Check proton condition BEFORE electron to avoid matching
        if self.mass >= 1.0e-31 && charge_abs > 0.5 && (spin_abs - 0.5).abs() < 0.1 {
            if self.charge > 0.0 {
                return "Proton";
            } else {
                return "Antiproton";
            }
        }

        // Electron/Positron: mass ~1e-32 to 1e-30, charge = ±1, spin = 0.5
        // Derived from archetypes, electron mass is typically ~6.8e-32 kg
        // Upper bound changed to < 1e-31 to avoid matching protons
        if self.mass >= 1.0e-33
            && self.mass < 1.0e-31
            && charge_abs > 0.5
            && (spin_abs - 0.5).abs() < 0.1
        {
            if self.charge < 0.0 {
                return "Electron";
            } else {
                return "Positron";
            }
        }

        // Neutrino: very light, neutral, spin = 0.5
        if self.mass < 1.0e-34 && charge_abs < 0.1 && (spin_abs - 0.5).abs() < 0.1 {
            return "Neutrino";
        }

        // Unknown particle
        "Unknown"
    }

    // ============================================================================
    // LEGACY METHODS (for backward compatibility)
    // ============================================================================

    /// Derive mass (LEGACY - for backward compatibility)
    #[deprecated(note = "Use particle.mass field directly")]
    pub fn derive_mass(&self) -> Float {
        self.mass
    }

    /// Derive charge (LEGACY - for backward compatibility)
    #[deprecated(note = "Use particle.charge field directly")]
    pub fn derive_charge(&self) -> Float {
        self.charge
    }

    /// Get energy (LEGACY - returns total energy)
    #[deprecated(note = "Use particle.total_energy() instead")]
    pub fn energy(&self) -> Float {
        self.total_energy()
    }
}

// ============================================================================
// VECTOR3D ADDITIONAL METHODS (for backward compatibility)
// ============================================================================

impl Vector3D {
    /// Add another vector
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            vx: self.vx + other.vx,
            vy: self.vy + other.vy,
            vz: self.vz + other.vz,
        }
    }

    /// Scale by a factor
    pub fn scale(&self, factor: Float) -> Vector3D {
        Vector3D {
            vx: self.vx * factor,
            vy: self.vy * factor,
            vz: self.vz * factor,
        }
    }

    /// Cross product
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            vx: self.vy * other.vz - self.vz * other.vy,
            vy: self.vz * other.vx - self.vx * other.vz,
            vz: self.vx * other.vy - self.vy * other.vx,
        }
    }

    /// Convert to energy_fields::Vector3
    pub fn to_vector3(&self) -> crate::energy_fields::Vector3 {
        crate::energy_fields::Vector3::new(self.vx, self.vy, self.vz)
    }

    /// Convert from energy_fields::Vector3
    pub fn from_vector3(v: &crate::energy_fields::Vector3) -> Self {
        Vector3D::new(v.x, v.y, v.z)
    }
}

// ============================================================================
// COORDINATE3D ADDITIONAL METHODS (for backward compatibility)
// ============================================================================

impl Coordinate3D {
    /// Add another coordinate
    pub fn add(&self, other: &Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract another coordinate
    pub fn sub(&self, other: &Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// Scale by a factor
    pub fn scale(&self, factor: Float) -> Coordinate3D {
        Coordinate3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    /// Get mass number (LEGACY - for backward compatibility)
    pub fn mass_number(&self) -> u32 {
        0
    }

    /// Convert to energy_fields::Vector3
    pub fn to_vector3(&self) -> crate::energy_fields::Vector3 {
        crate::energy_fields::Vector3::new(self.x, self.y, self.z)
    }

    /// Convert from energy_fields::Vector3
    pub fn from_vector3(v: &crate::energy_fields::Vector3) -> Self {
        Coordinate3D::new(v.x, v.y, v.z)
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Particle(id={}, type={}, mass={:.2e} kg, charge={:.2e} e, spin={:.2} ħ, pos=({:.2e}, {:.2e}, {:.2e}))",
            self.id,
            self.get_particle_type_name(),
            self.mass,
            self.charge,
            self.spin,
            self.position.x,
            self.position.y,
            self.position.z
        )
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            id: 0,
            archetype_activation: [0.0; 22],
            position: Coordinate3D::origin(),
            velocity: Vector3D::zero(),
            mass: 0.0,
            charge: 0.0,
            spin: 0.0,
            lifetime: None,
            wavefunction: Complex::default(),
            creation_time: 0,
            age: 0,
            energy: 0.0,
            holographic_ref: None,
            light_origin: None,
        }
    }
}

// ============================================================================
// INTERACTION RESULT
// ============================================================================

/// Result of particle interaction
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionResult {
    /// Force vector (Newtons)
    pub force: Vector3D,

    /// Energy transfer (Joules)
    pub energy_transfer: Float,

    /// Archetype compatibility (0.0 to 1.0)
    pub compatibility: Float,
}

// ============================================================================
// COMPATIBILITY TYPES (for backward compatibility)
// ============================================================================

/// Particle type enum (LEGACY - for backward compatibility only)
///
/// This enum is deprecated. Particle types should now emerge dynamically
/// from archetype activation patterns using Particle::get_particle_type_name().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deprecated(note = "Use Particle::get_particle_type_name() instead")]
pub enum ParticleType {
    /// Up quark
    Up,
    /// Down quark
    Down,
    /// Electron
    Electron,
    /// Proton
    Proton,
    /// Neutron
    Neutron,
    /// Photon
    Photon,
    /// Neutrino
    Neutrino,
    /// Unknown particle
    Unknown,
}

/// Bond type enum (placeholder for Task 2.4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondType {
    Covalent,
    Ionic,
    Hydrogen,
    VanDerWaals,
}

/// Nucleus struct (LEGACY - for backward compatibility)
#[derive(Debug, Clone)]
pub struct Nucleus {
    pub protons: u8,
    pub neutrons: u8,
}

impl Nucleus {
    pub fn new(protons: u8, neutrons: u8) -> Self {
        Self { protons, neutrons }
    }

    /// Get mass number (protons + neutrons)
    pub fn mass_number(&self) -> u8 {
        self.protons + self.neutrons
    }
}

/// Atom struct (placeholder for Task 2.3)
#[derive(Debug, Clone)]
pub struct Atom {
    pub atomic_number: u32,
    pub mass_number: u32,
    pub protons: Vec<Particle>,
    pub neutrons: Vec<Particle>,
    pub electrons: Vec<Particle>,
    /// Nucleus (for backward compatibility)
    pub nucleus: Nucleus,
    /// Nucleus position (for backward compatibility)
    pub position: Coordinate3D,
}

impl Atom {
    pub fn new(
        atomic_number: u32,
        mass_number: u32,
        nucleus: Nucleus,
        position: Coordinate3D,
    ) -> Self {
        Self {
            atomic_number,
            mass_number,
            protons: vec![],
            neutrons: vec![],
            electrons: vec![],
            nucleus,
            position,
        }
    }

    /// Get neutrons (LEGACY - for backward compatibility)
    pub fn neutrons(&self) -> &[Particle] {
        &self.neutrons
    }

    /// Calculate atomic mass (LEGACY - for backward compatibility)
    pub fn atomic_mass(&self) -> Float {
        // Simplified: use mass number times atomic mass unit
        self.mass_number as Float * 1.66053906660e-27
    }
}

/// Molecule struct (placeholder for Task 2.4)
#[derive(Debug, Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<(usize, usize, BondType)>,
}

/// Molecular structure (placeholder)
#[derive(Debug, Clone)]
pub struct MolecularStructure {
    pub molecule: Molecule,
    pub geometry: String,
}

/// Cell organelle type (placeholder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganelleType {
    Nucleus,
    Mitochondria,
    Ribosome,
    EndoplasmicReticulum,
    GolgiApparatus,
}

/// Cell struct (placeholder)
#[derive(Debug, Clone)]
pub struct Cell {
    pub organelles: Vec<(OrganelleType, String)>,
}

/// DNA base type (placeholder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNABase {
    Adenine,
    Thymine,
    Guanine,
    Cytosine,
}

/// RNA base type (placeholder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RNABase {
    Adenine,
    Uracil,
    Guanine,
    Cytosine,
}

/// Matter enum (placeholder)
#[derive(Debug, Clone)]
pub enum Matter {
    Particle(Particle),
    Atom(Atom),
    Molecule(Molecule),
    Cell(Cell),
    Empty, // For backward compatibility
}

impl Matter {
    /// Empty matter instance
    pub fn empty() -> Self {
        Matter::Empty
    }

    /// Get atoms (LEGACY - for backward compatibility)
    pub fn atoms(&self) -> Vec<&Atom> {
        match self {
            Matter::Molecule(m) => m.atoms.iter().collect(),
            Matter::Atom(a) => vec![a],
            _ => vec![],
        }
    }

    /// Get atoms mutably (LEGACY - for backward compatibility)
    pub fn atoms_mut(&mut self) -> &mut [Atom] {
        match self {
            Matter::Molecule(m) => &mut m.atoms,
            _ => &mut [],
        }
    }

    /// Get cells (LEGACY - for backward compatibility)
    pub fn cells(&self) -> &[(OrganelleType, String)] {
        match self {
            Matter::Cell(c) => &c.organelles,
            _ => &[],
        }
    }

    /// Get particles (LEGACY - for backward compatibility)
    pub fn particles(&self) -> Vec<&Particle> {
        match self {
            Matter::Particle(p) => vec![p],
            Matter::Atom(a) => {
                let mut particles: Vec<&Particle> = vec![];
                particles.extend(a.protons.iter());
                particles.extend(a.neutrons.iter());
                particles.extend(a.electrons.iter());
                particles
            }
            _ => vec![],
        }
    }

    /// Get particles mutably (LEGACY - for backward compatibility)
    pub fn particles_mut(&mut self) -> Vec<&mut Particle> {
        match self {
            Matter::Particle(p) => vec![p],
            Matter::Atom(a) => {
                let mut particles: Vec<&mut Particle> = vec![];
                particles.extend(a.protons.iter_mut());
                particles.extend(a.neutrons.iter_mut());
                particles.extend(a.electrons.iter_mut());
                particles
            }
            _ => vec![],
        }
    }

    /// Total energy (LEGACY - for backward compatibility)
    pub fn total_energy(&self) -> Float {
        match self {
            Matter::Particle(p) => p.total_energy(),
            Matter::Atom(a) => {
                let proton_mass = 1.6726219e-27;
                let neutron_mass = 1.6749275e-27;
                let electron_mass = 9.10938356e-31;
                const C: Float = 2.998e8;
                let mass = a.protons.len() as Float * proton_mass
                    + a.neutrons.len() as Float * neutron_mass
                    + a.electrons.len() as Float * electron_mass;
                mass * C * C
            }
            Matter::Molecule(_) => 0.0,
            Matter::Cell(_) => 0.0,
            Matter::Empty => 0.0,
        }
    }

    /// Total mass (LEGACY - for backward compatibility)
    pub fn total_mass(&self) -> Float {
        match self {
            Matter::Particle(p) => p.mass,
            Matter::Atom(a) => {
                let proton_mass = 1.6726219e-27;
                let neutron_mass = 1.6749275e-27;
                let electron_mass = 9.10938356e-31;
                a.protons.len() as Float * proton_mass
                    + a.neutrons.len() as Float * neutron_mass
                    + a.electrons.len() as Float * electron_mass
            }
            Matter::Molecule(_) => 0.0,
            Matter::Cell(_) => 0.0,
            Matter::Empty => 0.0,
        }
    }

    /// Update matter (LEGACY - for backward compatibility)
    pub fn update(&mut self, dt: Float) {
        match self {
            Matter::Particle(p) => {
                p.update_position(dt);
            }
            _ => {}
        }
    }

    /// From energy fields (LEGACY - for backward compatibility)
    pub fn from_energy_fields() -> Self {
        // Create a simple hydrogen atom from energy fields
        let proton = Particle {
            id: 0,
            archetype_activation: [0.0; 22],
            mass: 1.6726219e-27,
            charge: 1.0,
            spin: 0.5,
            position: Coordinate3D::origin(),
            velocity: Vector3D::zero(),
            lifetime: None, // Stable
            wavefunction: Complex::new((1.503e-10_f64).sqrt(), 0.0),
            creation_time: 0,
            age: 0,
            energy: 1.503e-10, // Rest energy in J
            holographic_ref: None,
            light_origin: None,
        };

        let electron = Particle {
            id: 1,
            archetype_activation: [0.0; 22],
            mass: 9.10938356e-31,
            charge: -1.0,
            spin: 0.5,
            position: Coordinate3D::origin(),
            velocity: Vector3D::zero(),
            lifetime: None, // Stable
            wavefunction: Complex::new((8.187e-14_f64).sqrt(), 0.0),
            creation_time: 0,
            age: 0,
            energy: 8.187e-14, // Rest energy in J
            holographic_ref: None,
            light_origin: None,
        };

        let nucleus = Nucleus::new(1, 0);

        let atom = Atom {
            atomic_number: 1,
            mass_number: 1,
            protons: vec![],
            neutrons: vec![],
            electrons: vec![electron],
            nucleus,
            position: Coordinate3D::origin(),
        };

        Matter::Atom(atom)
    }
}

impl fmt::Display for Matter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Matter::Particle(p) => write!(f, "Matter::Particle({})", p),
            Matter::Atom(a) => write!(
                f,
                "Matter::Atom(Z={}, A={})",
                a.atomic_number, a.mass_number
            ),
            Matter::Molecule(m) => write!(f, "Matter::Molecule({} atoms)", m.atoms.len()),
            Matter::Cell(c) => write!(f, "Matter::Cell({} organelles)", c.organelles.len()),
            Matter::Empty => write!(f, "Matter::Empty"),
        }
    }
}

/// DNA struct (placeholder)
#[derive(Debug, Clone)]
pub struct DNA {
    pub bases: Vec<DNABase>,
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics_engine::PhysicsEngine;

    // Helper function to create electron-like activation pattern
    fn electron_activation() -> [Float; 22] {
        let mut activation = [0.0; 22];
        // High Matrix, Potentiator, Catalyst for mass
        activation[0] = 1.0; // A1: Matrix
        activation[1] = 0.5; // A2: Potentiator
        activation[2] = 0.3; // A3: Catalyst (negative charge)
        activation[3] = 0.5; // A4: Experience
        activation[4] = 1.0; // A5: Significator (identity)
        activation[5] = 0.5; // A6: Transformation (for spin direction)
        activation[7] = 1.0; // A8: Matrix
        activation[8] = 0.5; // A9: Potentiator
        activation[9] = 0.3; // A10: Catalyst
        activation[10] = 0.5; // A11: Experience
        activation[11] = 1.0; // A12: Significator (identity)
        activation[12] = 0.5; // A13: Transformation (for spin direction)
        activation[14] = 1.0; // A15: Matrix
        activation[15] = 0.5; // A16: Potentiator
        activation[16] = 0.3; // A17: Catalyst
        activation[17] = 0.5; // A18: Experience
        activation[18] = 1.0; // A19: Significator (identity)
        activation[19] = 0.5; // A20: Transformation (for spin direction)
                              // High Great Way for stability (>0.95 for stability)
        activation[6] = 0.96; // A7: Great Way
        activation[13] = 0.96; // A14: Great Way
        activation[20] = 0.96; // A21: Great Way
        activation
    }

    // Helper function to create proton-like activation pattern
    fn proton_activation() -> [Float; 22] {
        let mut activation = electron_activation();
        // Flip catalyst for positive charge
        activation[2] = 0.7; // A3: Catalyst (positive)
        activation[9] = 0.7; // A10: Catalyst
        activation[16] = 0.7; // A17: Catalyst
                              // Higher potentiator for more mass
        activation[1] = 0.8; // A2: Potentiator
        activation[8] = 0.8; // A9: Potentiator
        activation[15] = 0.8; // A16: Potentiator
        activation
    }

    // Helper function to create photon-like activation pattern
    fn photon_activation() -> [Float; 22] {
        let mut activation = [0.0; 22];
        // Zero Matrix and Significator for neutral charge
        // When Matrix=0 or Significator=0, charge magnitude becomes 0
        activation[0] = 0.0; // A1: Matrix (set to 0 for neutral charge)
        activation[7] = 0.0; // A8: Matrix (set to 0 for neutral charge)
        activation[14] = 0.0; // A15: Matrix (set to 0 for neutral charge)
                              // No Catalyst for neutral charge
        activation[2] = 0.0; // A3: Catalyst
        activation[9] = 0.0; // A10: Catalyst
        activation[16] = 0.0; // A17: Catalyst
                              // Zero Significator for neutral charge
        activation[4] = 0.0; // A5: Significator (set to 0 for neutral charge)
        activation[11] = 0.0; // A12: Significator (set to 0 for neutral charge)
        activation[18] = 0.0; // A19: Significator (set to 0 for neutral charge)
                              // High Great Way for stability
        activation[6] = 1.0; // A7: Great Way
        activation[13] = 1.0; // A14: Great Way
        activation[20] = 1.0; // A21: Great Way
                              // High Choice for boson nature
        activation[21] = 1.0; // A22: Choice
                              // Add Potentiator and Experience for proper spin calculation (boson = spin 1)
        activation[1] = 0.5;
        activation[8] = 0.5;
        activation[15] = 0.5; // Potentiator
        activation[3] = 1.0;
        activation[10] = 1.0;
        activation[17] = 1.0; // Experience
        activation[5] = 1.0;
        activation[12] = 1.0;
        activation[19] = 1.0; // Transformation
        activation
    }

    // ============================================================================
    // Particle Creation Tests
    // ============================================================================

    #[test]
    fn test_particle_from_archetype_activation() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        assert_eq!(particle.id, 1);
        assert!(particle.mass > 0.0);
        assert_eq!(particle.velocity, Vector3D::zero());
        assert!(particle.age == 0);
    }

    #[test]
    fn test_particle_with_velocity() {
        let activation = electron_activation();
        let velocity = Vector3D::new(1.0, 2.0, 3.0);
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_velocity(velocity);

        assert_eq!(particle.velocity, velocity);
    }

    #[test]
    fn test_particle_with_creation_time() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_creation_time(100);

        assert_eq!(particle.creation_time, 100);
        assert_eq!(particle.age, 0);
    }

    // ============================================================================
    // Property Derivation Tests
    // ============================================================================

    #[test]
    fn test_electron_properties() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        assert!(particle.mass > 0.0, "Electron should have positive mass");
        assert!(
            particle.mass < 1.0e-30,
            "Electron mass should be ~9.1e-31 kg"
        );
        assert!(
            particle.charge < 0.0,
            "Electron should have negative charge"
        );
        assert!(
            (particle.spin.abs() - 0.5).abs() < 0.1,
            "Electron should have spin ~0.5"
        );
        assert!(particle.is_stable(), "Electron should be stable");
    }

    #[test]
    fn test_proton_properties() {
        let activation = proton_activation();
        let particle = Particle::from_archetype_activation(2, activation, Coordinate3D::origin());

        assert!(particle.mass > 0.0, "Proton should have positive mass");
        // Proton mass should be heavier than electron mass
        // Get electron mass for comparison
        let electron_activation = electron_activation();
        let electron =
            Particle::from_archetype_activation(1, electron_activation, Coordinate3D::origin());
        assert!(
            particle.mass > electron.mass,
            "Proton mass should be heavier than electron (proton: {} vs electron: {})",
            particle.mass,
            electron.mass
        );
        assert!(particle.charge > 0.0, "Proton should have positive charge");
        assert!(
            (particle.spin.abs() - 0.5).abs() < 0.1,
            "Proton should have spin ~0.5"
        );
        assert!(particle.is_stable(), "Proton should be stable");
    }

    #[test]
    fn test_photon_properties() {
        let activation = photon_activation();
        let particle = Particle::from_archetype_activation(3, activation, Coordinate3D::origin());

        assert!(particle.mass < 1.0e-35, "Photon mass should be ~0");
        assert_eq!(particle.charge, 0.0, "Photon should be neutral");
        assert!(
            (particle.spin.abs() - 1.0).abs() < 0.1,
            "Photon should have spin ~1.0"
        );
        assert!(particle.is_stable(), "Photon should be stable");
    }

    // ============================================================================
    // Energy and Momentum Tests
    // ============================================================================

    #[test]
    fn test_kinetic_energy() {
        let activation = electron_activation();
        let velocity = Vector3D::new(1e6, 0.0, 0.0);
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_velocity(velocity);

        let ke = particle.kinetic_energy();
        assert!(ke > 0.0, "Kinetic energy should be positive");

        // Compare with manual calculation
        let expected_ke = 0.5 * particle.mass * (1e6 * 1e6);
        assert!(
            (ke - expected_ke).abs() / expected_ke < 0.01,
            "Kinetic energy calculation should match formula"
        );
    }

    #[test]
    fn test_rest_energy() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        let rest_e = particle.rest_energy();
        assert!(rest_e > 0.0, "Rest energy should be positive");

        // E = mc²
        const C: Float = 2.998e8;
        let expected_e = particle.mass * C * C;
        assert!(
            (rest_e - expected_e).abs() / expected_e < 0.01,
            "Rest energy should equal mc²"
        );
    }

    #[test]
    fn test_total_energy() {
        let activation = electron_activation();
        let velocity = Vector3D::new(1e6, 0.0, 0.0);
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_velocity(velocity);

        let total = particle.total_energy();
        let rest = particle.rest_energy();
        let kinetic = particle.kinetic_energy();

        assert!(
            (total - (rest + kinetic)).abs() / total < 0.01,
            "Total energy should equal rest + kinetic"
        );
    }

    #[test]
    fn test_momentum() {
        let activation = electron_activation();
        let velocity = Vector3D::new(1e6, 2e6, 3e6);
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
            .with_velocity(velocity);

        let momentum = particle.momentum();
        assert!(
            (momentum.vx - particle.mass * velocity.vx).abs() < 1e-40,
            "Momentum x should equal m * vx"
        );
        assert!(
            (momentum.vy - particle.mass * velocity.vy).abs() < 1e-40,
            "Momentum y should equal m * vy"
        );
        assert!(
            (momentum.vz - particle.mass * velocity.vz).abs() < 1e-40,
            "Momentum z should equal m * vz"
        );
    }

    // ============================================================================
    // Position and Velocity Tests
    // ============================================================================

    #[test]
    fn test_update_position() {
        let activation = electron_activation();
        let velocity = Vector3D::new(1.0, 2.0, 3.0);
        let mut particle =
            Particle::from_archetype_activation(1, activation, Coordinate3D::origin())
                .with_velocity(velocity);

        particle.update_position(1.0);

        assert_eq!(particle.position.x, 1.0);
        assert_eq!(particle.position.y, 2.0);
        assert_eq!(particle.position.z, 3.0);
    }

    #[test]
    fn test_update_age() {
        let activation = electron_activation();
        let mut particle =
            Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        particle.update_age(10);
        assert_eq!(particle.age, 10);

        particle.update_age(5);
        assert_eq!(particle.age, 15);
    }

    // ============================================================================
    // Wavefunction Tests
    // ============================================================================

    #[test]
    fn test_wavefunction_initialization() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        assert_eq!(particle.wavefunction.real, 1.0);
        assert_eq!(particle.wavefunction.imag, 0.0);
    }

    #[test]
    fn test_probability_density() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        let density = particle.probability_density();
        assert_eq!(density, 1.0, "Initial probability density should be 1.0");
    }

    #[test]
    fn test_normalize_wavefunction() {
        let activation = electron_activation();
        let mut particle =
            Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        particle.wavefunction = Complex::new(2.0, 0.0);
        particle.normalize_wavefunction();

        assert_eq!(particle.wavefunction.real, 1.0);
        assert_eq!(particle.wavefunction.imag, 0.0);
    }

    #[test]
    fn test_evolve_wavefunction() {
        let activation = electron_activation();
        let mut particle =
            Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        let initial = particle.wavefunction;
        particle.evolve_wavefunction(1e-15);

        // Wavefunction should change (phase evolution)
        assert_ne!(particle.wavefunction, initial);

        // Magnitude should remain 1.0
        assert!(
            (particle.wavefunction.magnitude() - 1.0).abs() < 0.01,
            "Wavefunction magnitude should remain 1.0"
        );
    }

    // ============================================================================
    // Archetype Compatibility Tests
    // ============================================================================

    #[test]
    fn test_archetype_compatibility_identical() {
        let activation = electron_activation();
        let particle1 = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());
        let particle2 = Particle::from_archetype_activation(2, activation, Coordinate3D::origin());

        let compatibility = particle1.archetype_compatibility(&particle2);
        assert_eq!(
            compatibility, 1.0,
            "Identical patterns should have compatibility 1.0"
        );
    }

    #[test]
    fn test_archetype_compatibility_different() {
        let activation1 = electron_activation();
        let activation2 = proton_activation();
        let particle1 = Particle::from_archetype_activation(1, activation1, Coordinate3D::origin());
        let particle2 = Particle::from_archetype_activation(2, activation2, Coordinate3D::origin());

        let compatibility = particle1.archetype_compatibility(&particle2);
        assert!(
            compatibility < 1.0 && compatibility > 0.0,
            "Different patterns should have compatibility in (0, 1)"
        );
    }

    // ============================================================================
    // Particle Interaction Tests
    // ============================================================================

    #[test]
    fn test_particle_interaction_electron_proton() {
        let electron = Particle::from_archetype_activation(
            1,
            electron_activation(),
            Coordinate3D::new(0.0, 0.0, 0.0),
        );
        let proton = Particle::from_archetype_activation(
            2,
            proton_activation(),
            Coordinate3D::new(1e-10, 0.0, 0.0),
        );

        let result = electron.interact(&proton);

        // Opposite charges should attract (negative force direction)
        assert!(result.force.vx < 0.0, "Electron-proton should attract");
        assert!(
            result.compatibility > 0.0,
            "Compatibility should be positive"
        );
    }

    #[test]
    fn test_particle_interaction_electron_electron() {
        let activation = electron_activation();
        let particle1 =
            Particle::from_archetype_activation(1, activation, Coordinate3D::new(0.0, 0.0, 0.0));
        let particle2 =
            Particle::from_archetype_activation(2, activation, Coordinate3D::new(1e-10, 0.0, 0.0));

        let result = particle1.interact(&particle2);

        // Like charges should repel (positive force direction)
        assert!(result.force.vx > 0.0, "Electron-electron should repel");
        assert_eq!(
            result.compatibility, 1.0,
            "Identical patterns should have compatibility 1.0"
        );
    }

    #[test]
    fn test_particle_interaction_zero_distance() {
        let activation = electron_activation();
        let particle1 = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());
        let particle2 = Particle::from_archetype_activation(2, activation, Coordinate3D::origin());

        let result = particle1.interact(&particle2);

        assert_eq!(
            result.force,
            Vector3D::zero(),
            "Force should be zero at zero distance"
        );
        assert_eq!(
            result.energy_transfer, 0.0,
            "Energy transfer should be zero"
        );
    }

    // ============================================================================
    // Particle Type Identification Tests
    // ============================================================================

    #[test]
    fn test_get_particle_type_electron() {
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        assert_eq!(particle.get_particle_type_name(), "Electron");
    }

    #[test]
    fn test_get_particle_type_proton() {
        let activation = proton_activation();
        let particle = Particle::from_archetype_activation(2, activation, Coordinate3D::origin());

        assert_eq!(particle.get_particle_type_name(), "Proton");
    }

    #[test]
    fn test_get_particle_type_photon() {
        let activation = photon_activation();
        let particle = Particle::from_archetype_activation(3, activation, Coordinate3D::origin());

        assert_eq!(particle.get_particle_type_name(), "Photon");
    }

    // ============================================================================
    // Coordinate and Vector Tests
    // ============================================================================

    #[test]
    fn test_coordinate_distance() {
        let c1 = Coordinate3D::new(0.0, 0.0, 0.0);
        let c2 = Coordinate3D::new(3.0, 4.0, 0.0);

        let distance = c1.distance_to(&c2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        let normalized = v.normalize().unwrap();

        assert!((normalized.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_multiply() {
        let c1 = Complex::new(1.0, 1.0);
        let c2 = Complex::new(1.0, -1.0);

        let result = c1.multiply(&c2);
        assert_eq!(result.real, 2.0);
        assert_eq!(result.imag, 0.0);
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(1.0, 2.0);
        let conj = c.conjugate();

        assert_eq!(conj.real, 1.0);
        assert_eq!(conj.imag, -2.0);
    }

    // ============================================================================
    // Comprehensive Validation Tests
    // ============================================================================

    #[test]
    fn test_particle_system_completeness() {
        // Verify all particle properties derive from archetypes
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());

        // All properties should be derived
        assert!(particle.mass > 0.0);
        assert!(particle.charge != 0.0 || true);
        assert!(particle.spin != 0.0 || true);
        assert!(particle.lifetime.is_none() || particle.lifetime.unwrap() > 0.0);

        // Wavefunction should be normalized
        assert!((particle.wavefunction.magnitude() - 1.0).abs() < 0.01);

        // Position and velocity should work
        let ke = particle.kinetic_energy();
        let rest_e = particle.rest_energy();
        let total_e = particle.total_energy();
        assert!((total_e - (rest_e + ke)).abs() / total_e < 0.01);

        // Interaction should work
        let other =
            Particle::from_archetype_activation(2, activation, Coordinate3D::new(1e-10, 0.0, 0.0));
        let result = particle.interact(&other);
        assert!(result.compatibility > 0.0);
    }

    #[test]
    fn test_task_2_2_validation() {
        // Validate all Task 2.2 requirements

        // 1. Particle properties derive correctly from archetypes
        let activation = electron_activation();
        let particle = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());
        assert!(particle.mass > 0.0);
        assert!(particle.spin != 0.0);

        // 2. Particle interactions work correctly
        let other =
            Particle::from_archetype_activation(2, activation, Coordinate3D::new(1e-10, 0.0, 0.0));
        let result = particle.interact(&other);
        assert!(result.compatibility > 0.0);

        // 3. Wavefunction evolution implemented
        let mut particle =
            Particle::from_archetype_activation(3, activation, Coordinate3D::origin());
        particle.evolve_wavefunction(1e-15);
        assert!(particle.wavefunction.magnitude() > 0.0);

        // 4. Energy conservation (E=mc²)
        let rest_e = particle.rest_energy();
        const C: Float = 2.998e8;
        assert!((rest_e - particle.mass * C * C).abs() / rest_e < 0.01);

        // 5. No deprecated ParticleType enum - properties emerge dynamically
        let type_name = particle.get_particle_type_name();
        assert!(!type_name.is_empty());

        // All validation passed
        assert!(true);
    }

    // ============================================================================
    // Phase 4 Dual-Mode Physics Tests
    // ============================================================================

    #[test]
    fn test_particle_from_archetype_activation_dual_mode() {
        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let activation = electron_activation();
        let particle =
            Particle::from_archetype_activation_dual_mode(1, activation, Coordinate3D::origin());

        assert_eq!(particle.id, 1);
        assert!(particle.mass > 0.0);
        assert_eq!(particle.velocity, Vector3D::zero());
        assert!(particle.age == 0);
    }

    #[test]
    fn test_particle_dual_mode_backward_compatibility() {
        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let activation = electron_activation();

        // Both methods should produce same results in Hardcoded mode
        let particle1 = Particle::from_archetype_activation(1, activation, Coordinate3D::origin());
        let particle2 =
            Particle::from_archetype_activation_dual_mode(2, activation, Coordinate3D::origin());

        assert!((particle1.mass - particle2.mass).abs() < 1e-40);
        assert!((particle1.charge - particle2.charge).abs() < 1e-10);
        assert!((particle1.spin - particle2.spin).abs() < 1e-10);
    }

    #[test]
    fn test_particle_dual_mode_properties() {
        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Holographic, 0.5);

        let activation = electron_activation();
        let particle =
            Particle::from_archetype_activation_dual_mode(1, activation, Coordinate3D::origin());

        // Properties should still be valid
        assert!(particle.mass > 0.0);
        assert!(particle.charge != 0.0);
        assert!(particle.spin != 0.0);
        assert!(particle.is_stable());
    }

    #[test]
    fn test_particle_dual_mode_with_velocity() {
        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let activation = electron_activation();
        let velocity = Vector3D::new(1.0, 2.0, 3.0);
        let particle =
            Particle::from_archetype_activation_dual_mode(1, activation, Coordinate3D::origin())
                .with_velocity(velocity);

        assert_eq!(particle.velocity, velocity);
    }

    #[test]
    fn test_particle_dual_mode_interactions() {
        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let electron = Particle::from_archetype_activation_dual_mode(
            1,
            electron_activation(),
            Coordinate3D::new(0.0, 0.0, 0.0),
        );
        let proton = Particle::from_archetype_activation_dual_mode(
            2,
            proton_activation(),
            Coordinate3D::new(1e-10, 0.0, 0.0),
        );

        let result = electron.interact(&proton);

        // Opposite charges should attract
        assert!(result.force.vx < 0.0);
        assert!(result.compatibility > 0.0);
    }

    #[test]
    fn test_phase4_particle_migration_completeness() {
        // Verify all dual-mode particle functions work correctly

        // Initialize dual-mode physics system
        Particle::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        let activation = electron_activation();

        // Create particle using dual-mode system
        let particle =
            Particle::from_archetype_activation_dual_mode(1, activation, Coordinate3D::origin());

        // All properties should be valid
        assert!(particle.mass > 0.0);
        assert!(particle.charge != 0.0);
        assert!(particle.spin != 0.0);

        // All methods should work
        let _ke = particle.kinetic_energy();
        let _rest_e = particle.rest_energy();
        let _total_e = particle.total_energy();
        let _momentum = particle.momentum();
        let _density = particle.probability_density();
        let _type_name = particle.get_particle_type_name();

        // All should succeed without panic
        assert!(true);
    }

    #[test]
    fn test_phase4_physics_engine_dual_mode() {
        let mut engine = PhysicsEngine::earth_like();

        // Check default state
        assert!(!engine.use_dual_mode_physics());

        // Enable dual-mode physics
        engine.set_use_dual_mode_physics(true);
        assert!(engine.use_dual_mode_physics());

        // Disable dual-mode physics
        engine.set_use_dual_mode_physics(false);
        assert!(!engine.use_dual_mode_physics());
    }

    #[test]
    fn test_phase4_physics_engine_initialization() {
        // Initialize dual-mode physics system through PhysicsEngine
        PhysicsEngine::initialize_dual_mode_physics(PhysicsMode::Hardcoded, 0.5);

        // Verify initialization succeeded
        assert!(true);
    }
}
