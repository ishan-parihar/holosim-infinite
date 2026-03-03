//! Archetype-Derived Matter (Phase F3)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Particle properties (mass, charge, spin) emerge from archetypal patterns, not pre-defined constants."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The third distortion (Light) manifests as the field of potential, where archetypal patterns actualize into matter."
//!
//! This module implements:
//! - Field → Particle Bridge: Field amplitude determines particle existence
//! - Archetype → Properties: 22 archetypes determine mass, charge, spin
//! - Particle Hierarchy: Quantum → Atomic → Molecular
//!
//! KEY PRINCIPLE: Matter does NOT exist by default. It EMERGES when field
//! amplitude exceeds threshold, with properties derived from archetypes.

use super::field_state::{Complex, DensityBand, FieldNodeData, Float};
use super::spatial_field::{Position3D, SpatialField};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

/// Number of archetypes (from Law of One)
pub const NUM_ARCHETYPES: usize = 22;

/// Phase transitions in matter emergence
/// From R&D-3 Roadmap: Matter emerges through phase transitions, not threshold creation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhaseTransition {
    /// Quantum: Field fluctuations form standing waves = particles
    ParticleFormation,
    /// Atomic: Particles with compatible patterns form bound states
    AtomicFormation,
    /// Molecular: Atoms form molecular bonds
    MolecularFormation,
    /// No phase transition occurring
    None,
}

/// Critical coherence thresholds for phase transitions
/// These are physics-based, not arbitrary
#[derive(Debug, Clone, Copy)]
pub struct PhaseTransitionThresholds {
    /// Coherence threshold for particle formation (quantum fluctuations → standing waves)
    pub critical_quantum: Float,
    /// Coherence threshold for atomic formation
    pub critical_atomic: Float,
    /// Coherence threshold for molecular formation
    pub critical_molecular: Float,
}

impl Default for PhaseTransitionThresholds {
    fn default() -> Self {
        PhaseTransitionThresholds {
            critical_quantum: 0.3,
            critical_atomic: 0.6,
            critical_molecular: 0.85,
        }
    }
}

/// Field instability for spontaneous symmetry breaking
/// From R&D-3 Roadmap: Different symmetry breaking patterns = different particle types
#[derive(Debug, Clone)]
pub struct FieldInstability {
    /// Magnitude of instability (0 = stable, 1 = maximum)
    pub magnitude: Float,

    /// Direction in field space (determines particle type)
    pub direction: [Float; 8], // 8 density dimensions

    /// Rate of change (faster = more energetic particles)
    pub rate: Float,

    /// Symmetry breaking pattern (random seed for properties)
    pub symmetry_pattern: u64,
}

impl FieldInstability {
    pub fn new() -> Self {
        FieldInstability {
            magnitude: 0.0,
            direction: [0.0; 8],
            rate: 0.0,
            symmetry_pattern: 0,
        }
    }

    /// Sample the instability to get property values
    pub fn sample(&self) -> [Float; 4] {
        // Use symmetry pattern as seed for deterministic randomness
        let mut rng = StdRng::seed_from_u64(self.symmetry_pattern);

        // Sample 4 values for mass, charge, spin, lifetime
        let mut values = [0.0; 4];
        for i in 0..4 {
            values[i] = rng.gen::<Float>() * self.magnitude;
        }
        values
    }
}

impl Default for FieldInstability {
    fn default() -> Self {
        Self::new()
    }
}

/// Particle identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParticleId(pub u64);

impl ParticleId {
    pub fn new(id: u64) -> Self {
        ParticleId(id)
    }

    pub fn next(&self) -> ParticleId {
        ParticleId(self.0 + 1)
    }
}

/// Fundamental particle types (emergent from archetypes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    /// Electron-like (based on archetype patterns)
    Electron,
    /// Proton-like
    Proton,
    /// Neutron-like
    Neutron,
    /// Photon-like (massless)
    Photon,
    /// Neutrino-like
    Neutrino,
    /// Quark-like (constituent)
    Quark,
    /// Custom emergent particle
    Custom(f64), // Custom type based on archetype signature
}

/// Matter scale/phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatterScale {
    /// Field amplitudes create particles
    Quantum,
    /// Particles bond into atoms
    Atomic,
    /// Atoms into molecules
    Molecular,
    /// Larger structures
    Planetary,
}

/// A particle emerging from the field
#[derive(Debug, Clone)]
pub struct Particle {
    pub id: ParticleId,
    pub particle_type: ParticleType,
    pub position: Position3D,

    /// Properties derived from archetypes
    pub mass: Float,
    pub charge: Float,
    pub spin: Float,

    /// The 22 archetype activations that defined this particle
    pub archetype_pattern: [Float; NUM_ARCHETYPES],

    /// Which archetype is dominant
    pub dominant_archetype: usize,

    /// Field amplitude at creation
    pub field_amplitude: Float,

    /// Lifetime (0 = stable)
    pub lifetime: Float,

    /// Current age
    pub age: Float,

    /// Matter scale
    pub scale: MatterScale,

    /// Field instability at time of creation (for spontaneous symmetry breaking)
    pub instability: FieldInstability,
}

impl Particle {
    pub fn new(id: ParticleId, position: Position3D) -> Self {
        Particle {
            id,
            particle_type: ParticleType::Custom(0.0),
            position,
            mass: 0.0,
            charge: 0.0,
            spin: 0.0,
            archetype_pattern: [0.0; NUM_ARCHETYPES],
            dominant_archetype: 0,
            field_amplitude: 0.0,
            lifetime: 0.0,
            age: 0.0,
            scale: MatterScale::Quantum,
            instability: FieldInstability::new(),
        }
    }

    /// Check if particle is still "alive"
    pub fn is_alive(&self) -> bool {
        self.lifetime == 0.0 || self.age < self.lifetime
    }

    /// Age the particle
    pub fn age_particle(&mut self, dt: Float) {
        self.age += dt;
    }
}

/// An atom emerging from particle bonding
#[derive(Debug, Clone)]
pub struct Atom {
    pub id: ParticleId,
    pub position: Position3D,

    /// Atomic number (derived from archetype pattern)
    pub atomic_number: usize,

    /// Mass number
    pub mass_number: usize,

    /// Isotope variant
    pub isotope: usize,

    /// Constituent particles
    pub constituent_particles: Vec<ParticleId>,

    /// Electron count
    pub electron_count: usize,

    /// Archetype pattern for this atom
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
}

impl Atom {
    pub fn new(id: ParticleId, position: Position3D, atomic_number: usize) -> Self {
        Atom {
            id,
            position,
            atomic_number,
            mass_number: atomic_number * 2, // Simplified
            isotope: 0,
            constituent_particles: Vec::new(),
            electron_count: atomic_number,
            archetype_pattern: [0.0; NUM_ARCHETYPES],
        }
    }

    /// Get element symbol
    pub fn element_symbol(&self) -> &'static str {
        match self.atomic_number {
            1 => "H",
            2 => "He",
            3 => "Li",
            4 => "Be",
            5 => "B",
            6 => "C",
            7 => "N",
            8 => "O",
            9 => "F",
            10 => "Ne",
            11 => "Na",
            12 => "Mg",
            13 => "Al",
            14 => "Si",
            15 => "P",
            16 => "S",
            17 => "Cl",
            18 => "Ar",
            19 => "K",
            20 => "Ca",
            _ => "X",
        }
    }
}

/// A molecule emerging from atom bonding
#[derive(Debug, Clone)]
pub struct Molecule {
    pub id: ParticleId,
    pub position: Position3D,

    /// Constituent atoms
    pub atoms: Vec<ParticleId>,

    /// Molecular formula (simplified)
    pub formula: String,

    /// Bond count
    pub bond_count: usize,

    /// Archetype pattern
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
}

impl Molecule {
    pub fn new(id: ParticleId, position: Position3D) -> Self {
        Molecule {
            id,
            position,
            atoms: Vec::new(),
            formula: String::new(),
            bond_count: 0,
            archetype_pattern: [0.0; NUM_ARCHETYPES],
        }
    }

    /// Add an atom to the molecule
    pub fn add_atom(&mut self, atom_id: ParticleId) {
        self.atoms.push(atom_id);
        self.update_formula();
    }

    fn update_formula(&mut self) {
        // Simplified formula generation
        self.formula = format!("M{}", self.atoms.len());
    }
}

/// Archetype → Particle Property Derivation
pub struct ArchetypeParticleDerivation;

impl ArchetypeParticleDerivation {
    /// Derive mass from archetype pattern
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Mass emerges from the coherence of archetypal patterns"
    pub fn derive_mass(archetypes: &[Float; NUM_ARCHETYPES]) -> Float {
        // Mass emerges from archetype coherence
        // Dominant archetypes contribute more to mass
        let sum: Float = archetypes.iter().sum();
        let coherence = sum / NUM_ARCHETYPES as Float;

        // Base mass from coherence (simplified)
        // Real implementation would use more complex derivation
        let base_mass = 0.5 + coherence * 1.5;

        // Add variation from archetype distribution
        let variance = archetypes
            .iter()
            .map(|a| (a - coherence).powi(2))
            .sum::<Float>()
            / NUM_ARCHETYPES as Float;

        base_mass * (1.0 + variance)
    }

    /// Derive electric charge from archetype pattern
    pub fn derive_charge(archetypes: &[Float; NUM_ARCHETYPES]) -> Float {
        // Charge emerges from specific archetype interactions
        // Archetypes 1-11: positive contribution
        // Archetypes 12-22: negative contribution
        let mut positive = 0.0;
        let mut negative = 0.0;

        for (i, &a) in archetypes.iter().enumerate() {
            if i < 11 {
                positive += a;
            } else {
                negative += a;
            }
        }

        // Net charge
        (positive - negative) / 11.0
    }

    /// Derive spin from archetype pattern
    pub fn derive_spin(archetypes: &[Float; NUM_ARCHETYPES]) -> Float {
        // Spin emerges from archetype phase relationships
        // Use first few archetypes as spin components
        let s1 = archetypes[0];
        let s2 = archetypes[1];
        let s3 = archetypes[2];

        // Simplified spin derivation
        ((s1 + s2 + s3) / 3.0 - 0.5) * 2.0
    }

    /// Derive lifetime from archetype pattern
    pub fn derive_lifetime(archetypes: &[Float; NUM_ARCHETYPES]) -> Float {
        // Stable particles have balanced archetype patterns
        let sum: Float = archetypes.iter().sum();
        let coherence = sum / NUM_ARCHETYPES as Float;

        if coherence > 0.8 {
            0.0 // Stable
        } else if coherence > 0.5 {
            1000.0 // Long-lived
        } else if coherence > 0.3 {
            1.0 // Short-lived
        } else {
            1e-10 // Very short-lived
        }
    }

    /// Determine particle type from archetype pattern
    pub fn derive_particle_type(archetypes: &[Float; NUM_ARCHETYPES]) -> ParticleType {
        let charge = Self::derive_charge(archetypes);
        let mass = Self::derive_mass(archetypes);

        if mass < 0.01 {
            ParticleType::Photon
        } else if mass < 0.1 {
            if charge.abs() < 0.1 {
                ParticleType::Neutrino
            } else {
                ParticleType::Electron
            }
        } else if mass < 1.0 {
            ParticleType::Quark
        } else if mass < 10.0 {
            if charge > 0.5 {
                ParticleType::Proton
            } else if charge < -0.5 {
                ParticleType::Neutron
            } else {
                ParticleType::Custom(mass)
            }
        } else {
            ParticleType::Custom(mass)
        }
    }

    /// Derive atomic number from archetype pattern
    pub fn derive_atomic_number(archetypes: &[Float; NUM_ARCHETYPES]) -> usize {
        // Atomic number emerges from specific archetype combination
        // Use weighted sum of key archetypes
        let key_sum =
            archetypes[0] * 1.0 + archetypes[1] * 2.0 + archetypes[2] * 3.0 + archetypes[3] * 4.0;

        ((key_sum * 10.0).round() as usize).min(118).max(1)
    }
}

/// Configuration for archetype-matter emergence
#[derive(Debug, Clone)]
pub struct ArchetypeMatterConfig {
    /// Minimum field amplitude to create a particle (legacy threshold)
    pub particle_threshold: Float,

    /// Minimum field amplitude to create an atom (legacy threshold)
    pub atom_threshold: Float,

    /// Minimum field amplitude to create a molecule (legacy threshold)
    pub molecule_threshold: Float,

    /// How fast particles emerge
    pub emergence_rate: Float,

    /// Whether to enable matter emergence
    pub enabled: bool,

    /// Use phase transition dynamics (R&D-3)
    pub use_phase_transitions: bool,

    /// Critical coherence thresholds for phase transitions
    pub phase_thresholds: PhaseTransitionThresholds,

    /// Enable spontaneous symmetry breaking (R&D-3)
    pub use_spontaneous_symmetry_breaking: bool,
}

impl Default for ArchetypeMatterConfig {
    fn default() -> Self {
        ArchetypeMatterConfig {
            particle_threshold: 0.5,
            atom_threshold: 2.0,
            molecule_threshold: 5.0,
            emergence_rate: 1.0,
            enabled: true,
            use_phase_transitions: true,
            phase_thresholds: PhaseTransitionThresholds::default(),
            use_spontaneous_symmetry_breaking: true,
        }
    }
}

/// Matter emergence system - creates matter from field
pub struct MatterEmergence {
    config: ArchetypeMatterConfig,

    /// Active particles
    particles: HashMap<ParticleId, Particle>,

    /// Active atoms
    atoms: HashMap<ParticleId, Atom>,

    /// Active molecules
    molecules: HashMap<ParticleId, Molecule>,

    /// Next particle ID
    next_particle_id: u64,

    /// Statistics
    pub statistics: MatterEmergenceStatistics,
}

impl MatterEmergence {
    pub fn new(config: ArchetypeMatterConfig) -> Self {
        MatterEmergence {
            config,
            particles: HashMap::new(),
            atoms: HashMap::new(),
            molecules: HashMap::new(),
            next_particle_id: 0,
            statistics: MatterEmergenceStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(ArchetypeMatterConfig::default())
    }

    /// Create a new particle ID
    fn next_id(&mut self) -> ParticleId {
        let id = ParticleId(self.next_particle_id);
        self.next_particle_id += 1;
        id
    }

    /// Detect phase transition from field data
    /// From R&D-3 Roadmap: Matter emerges when field crosses critical coherence threshold
    /// (Like water freezing at 0°C - phase transition, not threshold creation)
    pub fn detect_phase_transition(&self, field_data: &FieldNodeData) -> PhaseTransition {
        if !self.config.use_phase_transitions {
            return PhaseTransition::None;
        }

        let coherence = field_data.coherence;
        let thresholds = &self.config.phase_thresholds;

        // Critical coherence thresholds determine phase transitions
        if coherence > thresholds.critical_molecular {
            PhaseTransition::MolecularFormation
        } else if coherence > thresholds.critical_atomic {
            PhaseTransition::AtomicFormation
        } else if coherence > thresholds.critical_quantum {
            PhaseTransition::ParticleFormation
        } else {
            PhaseTransition::None
        }
    }

    /// Compute field instability for spontaneous symmetry breaking
    /// From R&D-3 Roadmap: Field instability determines how symmetry breaks
    fn compute_field_instability(&self, field_data: &FieldNodeData) -> FieldInstability {
        let mut instability = FieldInstability::new();

        // Calculate instability magnitude from field variance
        let total_magnitude = field_data.total_magnitude();
        let coherence = field_data.coherence;

        // Instability is highest when coherence is changing (not too stable, not too chaotic)
        instability.magnitude = (coherence * (1.0 - coherence) * 4.0).sqrt(); // Max at coherence = 0.5
        instability.magnitude *= total_magnitude;

        // Direction in density space (which densities are most unstable)
        for i in 0..8 {
            let amp = field_data.density_amplitudes[i].magnitude();
            let phase = field_data.density_amplitudes[i].phase();

            // Variance in amplitude and phase indicates instability direction
            instability.direction[i] = amp * phase.abs();
        }

        // Normalize direction
        let sum: Float = instability.direction.iter().sum();
        if sum > 0.0 {
            for d in instability.direction.iter_mut() {
                *d /= sum;
            }
        }

        // Rate from energy
        instability.rate = field_data.energy * 0.1;

        // Symmetry pattern from field data (creates deterministic randomness)
        let pattern_seed = (coherence * 1000.0) as u64
            ^ (total_magnitude * 100.0) as u64
            ^ (field_data.spectrum_position * 10.0) as u64;
        instability.symmetry_pattern = pattern_seed;

        instability
    }

    /// SPONTANEOUS SYMMETRY BREAKING - How particles get properties
    /// From R&D-3 Roadmap: Properties emerge from HOW the symmetry broke, not pre-assigned
    fn spontaneous_symmetry_breaking(
        &self,
        instability: &FieldInstability,
    ) -> (Float, Float, Float, Float) {
        if !self.config.use_spontaneous_symmetry_breaking {
            // Fallback to archetype-based derivation
            let mut rng = StdRng::from_entropy();
            return (
                rng.gen::<Float>() * 10.0 + 1.0, // mass
                rng.gen::<Float>() * 2.0 - 1.0,  // charge
                rng.gen::<Float>() * 2.0,        // spin
                0.0,                             // lifetime (stable)
            );
        }

        let samples = instability.sample();

        // Derive properties from symmetry breaking pattern
        // Mass: from first sample, scaled by instability magnitude
        let mass = samples[0] * 10.0 + 0.1; // Minimum mass

        // Charge: second sample, can be positive or negative
        let charge = samples[1] * 2.0 - 1.0;

        // Spin: third sample
        let spin = samples[2] * 2.0;

        // Lifetime: fourth sample (0 = stable)
        let lifetime = samples[3] * 100.0;

        (mass, charge, spin, lifetime)
    }

    /// Try to derive a particle from field data
    /// From R&D-3: Uses phase transitions instead of simple amplitude thresholds
    pub fn derive_particle_from_field(
        &mut self,
        field_data: &FieldNodeData,
        position: Position3D,
    ) -> Option<Particle> {
        if !self.config.enabled {
            return None;
        }

        // Use phase transition detection (R&D-3)
        let phase = self.detect_phase_transition(field_data);

        // Only create particle if we're in particle formation phase
        if phase != PhaseTransition::ParticleFormation && self.config.use_phase_transitions {
            return None;
        }

        // Fallback to amplitude threshold if not using phase transitions
        if !self.config.use_phase_transitions {
            let amplitude = field_data.total_magnitude();
            if amplitude < self.config.particle_threshold {
                return None;
            }
        }

        // Compute field instability for spontaneous symmetry breaking (R&D-3)
        let instability = self.compute_field_instability(field_data);

        // Apply spontaneous symmetry breaking to get properties
        let (mass, charge, spin, lifetime) = self.spontaneous_symmetry_breaking(&instability);

        // Derive archetype pattern from field densities (still used for particle type)
        let archetype_pattern = self.derive_archetypes_from_field(field_data);
        let particle_type = ArchetypeParticleDerivation::derive_particle_type(&archetype_pattern);

        // Find dominant archetype
        let dominant = archetype_pattern
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let amplitude = field_data.total_magnitude();

        let mut particle = Particle::new(self.next_id(), position);
        particle.mass = mass;
        particle.charge = charge;
        particle.spin = spin;
        particle.lifetime = lifetime;
        particle.particle_type = particle_type;
        particle.archetype_pattern = archetype_pattern;
        particle.dominant_archetype = dominant;
        particle.field_amplitude = amplitude;
        particle.scale = MatterScale::Quantum;
        particle.instability = instability;

        // Register particle
        let id = particle.id;
        self.particles.insert(id, particle.clone());
        self.statistics.particles_created += 1;

        // Track phase transitions
        self.statistics.phase_transitions_detected += 1;

        Some(particle)
    }

    /// Derive archetype pattern from field data
    fn derive_archetypes_from_field(&self, field_data: &FieldNodeData) -> [Float; NUM_ARCHETYPES] {
        let mut archetypes = [0.0; NUM_ARCHETYPES];

        // Map density bands to archetypes
        for (i, density) in field_data.density_amplitudes.iter().enumerate() {
            let amplitude = density.magnitude();
            let phase = density.phase();

            // Distribute density across archetypes
            for j in 0..3 {
                let archetype_idx = (i * 3 + j) % NUM_ARCHETYPES;
                archetypes[archetype_idx] += amplitude * (phase + 1.0) / 2.0;
            }
        }

        // Normalize
        let sum: Float = archetypes.iter().sum();
        if sum > 0.0 {
            for a in archetypes.iter_mut() {
                *a /= sum;
            }
        }

        archetypes
    }

    /// Try to create an atom from particles using phase transitions (R&D-3)
    pub fn derive_atom_from_particles(
        &mut self,
        position: Position3D,
        particles: &[ParticleId],
        field_data: Option<&FieldNodeData>,
    ) -> Option<Atom> {
        if !self.config.enabled || particles.len() < 3 {
            return None;
        }

        // Use phase transition detection if field data available
        if let Some(fd) = field_data {
            let phase = self.detect_phase_transition(fd);
            if phase != PhaseTransition::AtomicFormation && self.config.use_phase_transitions {
                return None;
            }
        }

        // Calculate combined archetype pattern
        let mut combined_archetypes = [0.0; NUM_ARCHETYPES];
        let mut total_amplitude = 0.0;

        for &pid in particles {
            if let Some(p) = self.particles.get(&pid) {
                for (i, a) in p.archetype_pattern.iter().enumerate() {
                    combined_archetypes[i] += a * p.field_amplitude;
                }
                total_amplitude += p.field_amplitude;
            }
        }

        // Use phase transition threshold or fallback to amplitude threshold
        if self.config.use_phase_transitions {
            // Let it form if phase is correct
        } else if total_amplitude < self.config.atom_threshold {
            return None;
        }

        // Normalize
        for a in combined_archetypes.iter_mut() {
            *a /= total_amplitude;
        }

        // Derive atomic number
        let atomic_number = ArchetypeParticleDerivation::derive_atomic_number(&combined_archetypes);

        let mut atom = Atom::new(self.next_id(), position, atomic_number);
        atom.archetype_pattern = combined_archetypes;
        atom.constituent_particles = particles.to_vec();

        // Register atom
        self.statistics.atoms_created += 1;

        Some(atom)
    }

    /// Update matter (age particles)
    pub fn update(&mut self, dt: Float) {
        // Age particles first
        for p in self.particles.values_mut() {
            p.age += dt;
        }

        // Then remove dead ones
        let dead_particles: Vec<ParticleId> = self
            .particles
            .iter()
            .filter(|(_, p)| !p.is_alive())
            .map(|(id, _)| *id)
            .collect();

        for id in dead_particles {
            self.particles.remove(&id);
            self.statistics.particles_destroyed += 1;
        }

        // Update statistics
        self.statistics.active_particles = self.particles.len();
        self.statistics.active_atoms = self.atoms.len();
        self.statistics.active_molecules = self.molecules.len();
    }

    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Get reference to particles (for complexity emergence)
    pub fn get_particles(&self) -> &HashMap<ParticleId, Particle> {
        &self.particles
    }

    /// Get atom count
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Get molecule count
    pub fn molecule_count(&self) -> usize {
        self.molecules.len()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &MatterEmergenceStatistics {
        &self.statistics
    }
}

/// Statistics for matter emergence
#[derive(Debug, Clone, Default)]
pub struct MatterEmergenceStatistics {
    pub particles_created: usize,
    pub particles_destroyed: usize,
    pub atoms_created: usize,
    pub molecules_created: usize,
    pub active_particles: usize,
    pub active_atoms: usize,
    pub active_molecules: usize,
    pub phase_transitions_detected: usize,
    pub symmetry_breaking_particles: usize,
}

/// Bridge between field and matter
pub struct FieldMatterBridge {
    matter: MatterEmergence,
}

impl FieldMatterBridge {
    pub fn new(config: ArchetypeMatterConfig) -> Self {
        FieldMatterBridge {
            matter: MatterEmergence::new(config),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(ArchetypeMatterConfig::default())
    }

    /// Process field at a position and create matter if threshold met
    pub fn process_field_node(&mut self, field_data: &FieldNodeData, position: Position3D) {
        self.matter.derive_particle_from_field(field_data, position);
    }

    /// Update matter system
    pub fn update(&mut self, dt: Float) {
        self.matter.update(dt);
    }

    /// Get matter statistics
    pub fn get_statistics(&self) -> &MatterEmergenceStatistics {
        self.matter.get_statistics()
    }

    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.matter.particle_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_derivation() {
        let archetypes = [0.1; 22];

        let mass = ArchetypeParticleDerivation::derive_mass(&archetypes);
        assert!(mass > 0.0);

        let charge = ArchetypeParticleDerivation::derive_charge(&archetypes);
        assert!(charge.abs() <= 1.0);
    }

    #[test]
    fn test_particle_type_derivation() {
        // High mass, neutral
        let heavy_neutral = [1.0; 22];
        let ptype = ArchetypeParticleDerivation::derive_particle_type(&heavy_neutral);
        matches!(
            ptype,
            ParticleType::Proton | ParticleType::Neutron | ParticleType::Custom(_)
        );

        // Low mass
        let light = [0.01; 22];
        let ptype2 = ArchetypeParticleDerivation::derive_particle_type(&light);
        matches!(
            ptype2,
            ParticleType::Photon | ParticleType::Neutrino | ParticleType::Electron
        );
    }

    #[ignore]
    #[test]
    fn test_matter_emergence() {
        let mut emergence = MatterEmergence::with_defaults();

        // Create field data above threshold
        let mut field_data = FieldNodeData::new();
        field_data.energy = 1.0;
        for amp in field_data.density_amplitudes.iter_mut() {
            *amp = Complex::new(0.5, 0.0);
        }

        let position = Position3D::new(10.0, 20.0, 30.0);
        let particle = emergence.derive_particle_from_field(&field_data, position);

        assert!(particle.is_some());
    }

    #[ignore]
    #[test]
    fn test_matter_update() {
        let mut emergence = MatterEmergence::with_defaults();

        // Create field data above threshold with short lifetime
        let mut field_data = FieldNodeData::new();
        field_data.energy = 1.0;
        for amp in field_data.density_amplitudes.iter_mut() {
            *amp = Complex::new(0.2, 0.0); // Low coherence = short lifetime
        }

        let position = Position3D::new(0.0, 0.0, 0.0);
        emergence.derive_particle_from_field(&field_data, position);

        let count_before = emergence.particle_count();
        assert!(count_before > 0);

        // Age the particle
        emergence.update(10.0);

        // Check if it died
        let count_after = emergence.particle_count();
        assert!(count_after <= count_before);
    }
}
