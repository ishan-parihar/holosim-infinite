// Scale-Specific Entity Types - Multi-Scale Entity System
//
// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
// "Quantum-Scale (Stage 0): Sub-Atomic particles"
// "Atomic-Scale (Stage 1): Atoms"

use crate::energy_fields::Vector3;
use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use crate::matter::BondType;
use crate::multi_scale::scales::{EntityId as ScaleEntityId, EntityScale, MultiScaleEntity};
use crate::types::Float;

/// Quantum Particle - sub-atomic scale entity
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Quantum-Scale (Stage 0): Sub-Atomic particles"
#[derive(Debug, Clone)]
pub struct QuantumParticle {
    /// Unique ID
    pub id: ParticleID,

    /// Particle type name (e.g., String::from("Electron"), String::from("Proton"), "Neutron", "Photon")
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Particle types: quarks, leptons, bosons"
    ///
    /// Note: This replaces the deprecated ParticleType enum. Particle types now
    /// emerge dynamically from archetype activation patterns.
    pub particle_type: String,

    /// Quantum position (probability cloud)
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Quantum properties: spin, charge, mass"
    pub position: QuantumPosition,

    /// Quantum state
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Quantum state: superposition, entangled, collapsed"
    pub quantum_state: MultiScaleQuantumState,

    /// Holographic seed
    ///
    /// From Cosmology.json:
    /// "Every portion contains the whole holographically"
    pub seed: HolographicSeed,
}

impl QuantumParticle {
    /// Create new quantum particle
    pub fn new(id: ParticleID, particle_type: String, seed: HolographicSeed) -> Self {
        QuantumParticle {
            id,
            particle_type,
            position: QuantumPosition::new(),
            quantum_state: MultiScaleQuantumState::Superposition,
            seed,
        }
    }

    /// Create quantum particle with specific position
    pub fn with_position(
        id: ParticleID,
        particle_type: String,
        position: QuantumPosition,
        seed: HolographicSeed,
    ) -> Self {
        QuantumParticle {
            id,
            particle_type,
            position,
            quantum_state: MultiScaleQuantumState::Superposition,
            seed,
        }
    }

    /// Collapse quantum state
    pub fn collapse_state(&mut self) {
        self.quantum_state = MultiScaleQuantumState::Collapsed;
    }

    /// Entangle with another particle
    pub fn entangle_with(&mut self, other: &mut QuantumParticle) {
        self.quantum_state = MultiScaleQuantumState::Entangled;
        other.quantum_state = MultiScaleQuantumState::Entangled;
    }

    /// Get mass based on particle type name
    pub fn mass(&self) -> Float {
        match self.particle_type.as_str() {
            "Electron" => 9.10938356e-31,
            "Proton" => 1.6726219e-27,
            "Neutron" => 1.6749275e-27,
            "Photon" => 0.0,
            "Neutrino" => 1.0e-36,
            "Up" => 2.2e-30,
            "Down" => 4.7e-30,
            _ => 0.0,
        }
    }

    /// Get charge based on particle type name
    pub fn charge(&self) -> Float {
        match self.particle_type.as_str() {
            "Electron" => -1.0,
            "Proton" => 1.0,
            "Neutron" => 0.0,
            "Photon" => 0.0,
            "Neutrino" => 0.0,
            "Up" => 2.0 / 3.0,
            "Down" => -1.0 / 3.0,
            _ => 0.0,
        }
    }
}

impl MultiScaleEntity for QuantumParticle {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Quantum
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        // Quantum particles are the smallest scale
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        // Every particle contains reference to complete architecture
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Particle ID type
pub type ParticleID = u64;

/// Quantum position (probability cloud)
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumPosition {
    /// Position vector
    pub position: Vector3,

    /// Probability amplitude (squared gives probability)
    pub amplitude: Float,

    /// Phase angle
    pub phase: Float,
}

impl QuantumPosition {
    /// Create new quantum position
    pub fn new() -> Self {
        QuantumPosition {
            position: Vector3::new(0.0, 0.0, 0.0),
            amplitude: 1.0,
            phase: 0.0,
        }
    }

    /// Create quantum position with specific values
    pub fn with_values(position: Vector3, amplitude: Float, phase: Float) -> Self {
        QuantumPosition {
            position,
            amplitude: amplitude.max(0.0),
            phase,
        }
    }

    /// Get probability (amplitude squared)
    pub fn probability(&self) -> Float {
        self.amplitude * self.amplitude
    }
}

impl Default for QuantumPosition {
    fn default() -> Self {
        Self::new()
    }
}

/// Quantum state
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Quantum state: superposition, entangled, collapsed"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiScaleQuantumState {
    /// Superposition of multiple states
    Superposition,

    /// Entangled with another particle
    Entangled,

    /// Collapsed to definite state
    Collapsed,
}

/// MultiScaleAtom - atomic scale entity (renamed to avoid conflict with matter::MultiScaleAtom)
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Atomic-Scale (Stage 1): Atoms"
#[derive(Debug, Clone)]
pub struct MultiScaleAtom {
    /// Unique ID
    pub id: AtomID,

    /// Atomic number (protons)
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Atomic structure: protons, neutrons, electrons"
    pub atomic_number: u8,

    /// Neutrons
    pub neutrons: u8,

    /// Electrons
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Electron configurations (1s, 2s, 2p, 3s, 3p, 3d, etc.)"
    pub electrons: Vec<Electron>,

    /// Electron configuration
    pub electron_configuration: Vec<u8>,

    /// Atomic position
    pub position: AtomicPosition,

    /// Holographic seed
    pub seed: HolographicSeed,

    /// Contained quantum particles
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Each entity contains within it all densities and sub-densities"
    pub contained_particles: Vec<QuantumParticle>,
}

impl MultiScaleAtom {
    /// Create new atom
    pub fn new(id: AtomID, atomic_number: u8, neutrons: u8, seed: HolographicSeed) -> Self {
        MultiScaleAtom {
            id,
            atomic_number,
            neutrons,
            electrons: Vec::new(),
            electron_configuration: Vec::new(),
            position: AtomicPosition::new(),
            seed,
            contained_particles: Vec::new(),
        }
    }

    /// Create atom with electrons
    pub fn with_electrons(
        id: AtomID,
        atomic_number: u8,
        neutrons: u8,
        electrons: Vec<Electron>,
        seed: HolographicSeed,
    ) -> Self {
        let mut atom = Self::new(id, atomic_number, neutrons, seed);
        atom.electrons = electrons;
        atom.calculate_electron_configuration();
        atom
    }

    /// Calculate electron configuration
    fn calculate_electron_configuration(&mut self) {
        // Simplified electron configuration calculation
        // In a full implementation, this would follow the Aufbau principle
        self.electron_configuration = self.electrons.iter().map(|e| e.shell).collect();
    }

    /// Get mass number
    pub fn mass_number(&self) -> u8 {
        self.atomic_number + self.neutrons
    }

    /// Add contained particle
    pub fn add_contained_particle(&mut self, particle: QuantumParticle) {
        self.contained_particles.push(particle);
    }

    /// Get total mass from contained particles
    pub fn particle_mass(&self) -> Float {
        self.contained_particles.iter().map(|p| p.mass()).sum()
    }

    /// Get total charge from contained particles
    pub fn particle_charge(&self) -> Float {
        self.contained_particles.iter().map(|p| p.charge()).sum()
    }
}

impl MultiScaleEntity for MultiScaleAtom {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Atomic
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        self.contained_particles
            .iter()
            .map(|p| p as &dyn MultiScaleEntity)
            .collect()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// MultiScaleAtom ID type
pub type AtomID = u64;

/// Electron in atomic orbit
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Electron configurations (1s, 2s, 2p, 3s, 3p, 3d, etc.)"
#[derive(Debug, Clone, PartialEq)]
pub struct Electron {
    /// Shell number (1, 2, 3, etc.)
    pub shell: u8,

    /// Subshell (s, p, d, f)
    pub subshell: char,

    /// Energy level within subshell
    pub energy_level: u8,

    /// Spin (+1/2 or -1/2)
    pub spin: i8,
}

impl Electron {
    /// Create new electron
    pub fn new(shell: u8, subshell: char, energy_level: u8, spin: i8) -> Self {
        Electron {
            shell,
            subshell,
            energy_level,
            spin,
        }
    }
}

/// Atomic position
#[derive(Debug, Clone, PartialEq)]
pub struct AtomicPosition {
    /// Position vector
    pub position: Vector3,

    /// Thermal velocity
    pub thermal_velocity: Vector3,

    /// Temperature
    pub temperature: Float,
}

impl AtomicPosition {
    /// Create new atomic position
    pub fn new() -> Self {
        AtomicPosition {
            position: Vector3::new(0.0, 0.0, 0.0),
            thermal_velocity: Vector3::new(0.0, 0.0, 0.0),
            temperature: 0.0,
        }
    }

    /// Create atomic position with specific values
    pub fn with_values(position: Vector3, thermal_velocity: Vector3, temperature: Float) -> Self {
        AtomicPosition {
            position,
            thermal_velocity,
            temperature: temperature.max(0.0),
        }
    }
}

impl Default for AtomicPosition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== QuantumParticle Tests =====

    #[test]
    fn test_quantum_particle_new() {
        let seed = HolographicSeed::new_from_source();
        let particle = QuantumParticle::new(1, String::from("Electron"), seed);
        assert_eq!(particle.id, 1);
        assert_eq!(
            particle.quantum_state,
            MultiScaleQuantumState::Superposition
        );
    }

    #[test]
    fn test_quantum_particle_collapse_state() {
        let seed = HolographicSeed::new_from_source();
        let mut particle = QuantumParticle::new(1, String::from("Electron"), seed);
        particle.collapse_state();
        assert_eq!(particle.quantum_state, MultiScaleQuantumState::Collapsed);
    }

    #[test]
    fn test_quantum_particle_entangle() {
        let seed = HolographicSeed::new_from_source();
        let mut particle1 = QuantumParticle::new(1, String::from("Electron"), seed.clone());
        let mut particle2 = QuantumParticle::new(2, String::from("Electron"), seed);

        particle1.entangle_with(&mut particle2);

        assert_eq!(particle1.quantum_state, MultiScaleQuantumState::Entangled);
        assert_eq!(particle2.quantum_state, MultiScaleQuantumState::Entangled);
    }

    #[test]
    fn test_quantum_particle_mass() {
        let seed = HolographicSeed::new_from_source();
        let particle = QuantumParticle::new(1, String::from("Electron"), seed);
        assert!(particle.mass() > 0.0);
    }

    #[test]
    fn test_quantum_particle_charge() {
        let seed = HolographicSeed::new_from_source();
        let particle = QuantumParticle::new(1, String::from("Electron"), seed);
        assert!(particle.charge() < 0.0); // Electron has negative charge
    }

    #[test]
    fn test_quantum_particle_multi_scale() {
        let seed = HolographicSeed::new_from_source();
        let particle = QuantumParticle::new(1, String::from("Electron"), seed);

        assert_eq!(particle.get_scale(), EntityScale::Quantum);
        assert!(particle.contains_holographic_whole());
        assert_eq!(particle.contains_subscale_entities().len(), 0);
    }

    // ===== QuantumPosition Tests =====

    #[test]
    fn test_quantum_position_new() {
        let pos = QuantumPosition::new();
        assert_eq!(pos.amplitude, 1.0);
        assert_eq!(pos.phase, 0.0);
    }

    #[test]
    fn test_quantum_position_probability() {
        let pos = QuantumPosition::with_values(Vector3::new(1.0, 2.0, 3.0), 0.5, 0.0);
        assert!((pos.probability() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_quantum_position_default() {
        let pos = QuantumPosition::default();
        assert_eq!(pos.amplitude, 1.0);
    }

    // ===== MultiScaleAtom Tests =====

    #[test]
    fn test_atom_new() {
        let seed = HolographicSeed::new_from_source();
        let atom = MultiScaleAtom::new(1, 6, 6, seed); // Carbon-12
        assert_eq!(atom.id, 1);
        assert_eq!(atom.atomic_number, 6);
        assert_eq!(atom.neutrons, 6);
        assert_eq!(atom.mass_number(), 12);
    }

    #[test]
    fn test_atom_with_electrons() {
        let seed = HolographicSeed::new_from_source();
        let electrons = vec![
            Electron::new(1, 's', 1, 1),
            Electron::new(1, 's', 1, -1),
            Electron::new(2, 's', 1, 1),
        ];
        let atom = MultiScaleAtom::with_electrons(1, 3, 4, electrons, seed); // Lithium-7
        assert_eq!(atom.electrons.len(), 3);
    }

    #[test]
    fn test_atom_mass_number() {
        let seed = HolographicSeed::new_from_source();
        let atom = MultiScaleAtom::new(1, 6, 6, seed.clone()); // Carbon-12
        assert_eq!(atom.mass_number(), 12);

        let atom = MultiScaleAtom::new(1, 6, 7, seed); // Carbon-13
        assert_eq!(atom.mass_number(), 13);
    }

    #[test]
    fn test_atom_add_contained_particle() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 1, 0, seed.clone()); // Hydrogen-1

        let proton = QuantumParticle::new(1, String::from("Proton"), seed.clone());
        atom.add_contained_particle(proton);

        let electron = QuantumParticle::new(2, String::from("Electron"), seed);
        atom.add_contained_particle(electron);

        assert_eq!(atom.contained_particles.len(), 2);
    }

    #[test]
    fn test_atom_particle_mass() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 1, 0, seed.clone());

        let proton = QuantumParticle::new(1, String::from("Proton"), seed);
        atom.add_contained_particle(proton);

        let mass = atom.particle_mass();
        assert!(mass > 0.0);
    }

    #[test]
    fn test_atom_particle_charge() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 1, 0, seed.clone());

        let proton = QuantumParticle::new(1, String::from("Proton"), seed.clone());
        atom.add_contained_particle(proton);

        let electron = QuantumParticle::new(2, String::from("Electron"), seed);
        atom.add_contained_particle(electron);

        let charge = atom.particle_charge();
        // Proton (+1) + Electron (-1) = 0
        assert!((charge).abs() < 0.001);
    }

    #[test]
    fn test_atom_multi_scale() {
        let seed = HolographicSeed::new_from_source();
        let atom = MultiScaleAtom::new(1, 6, 6, seed);

        assert_eq!(atom.get_scale(), EntityScale::Atomic);
        assert!(atom.contains_holographic_whole());
        assert_eq!(atom.contains_subscale_entities().len(), 0); // No particles yet
    }

    #[test]
    fn test_atom_multi_scale_with_particles() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 1, 0, seed.clone());

        let proton = QuantumParticle::new(1, String::from("Proton"), seed);
        atom.add_contained_particle(proton);

        assert_eq!(atom.contains_subscale_entities().len(), 1);
    }

    // ===== Electron Tests =====

    #[test]
    fn test_electron_new() {
        let electron = Electron::new(1, 's', 1, 1);
        assert_eq!(electron.shell, 1);
        assert_eq!(electron.subshell, 's');
        assert_eq!(electron.energy_level, 1);
        assert_eq!(electron.spin, 1);
    }

    // ===== AtomicPosition Tests =====

    #[test]
    fn test_atomic_position_new() {
        let pos = AtomicPosition::new();
        assert_eq!(pos.temperature, 0.0);
    }

    #[test]
    fn test_atomic_position_with_values() {
        let position = Vector3::new(1.0, 2.0, 3.0);
        let velocity = Vector3::new(0.5, 0.6, 0.7);
        let pos = AtomicPosition::with_values(position, velocity, 300.0);
        assert_eq!(pos.temperature, 300.0);
    }

    #[test]
    fn test_atomic_position_default() {
        let pos = AtomicPosition::default();
        assert_eq!(pos.temperature, 0.0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_hydrogen_atom_structure() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 1, 0, seed.clone()); // Hydrogen-1

        // Add proton
        let proton = QuantumParticle::new(1, String::from("Proton"), seed.clone());
        atom.add_contained_particle(proton);

        // Add electron
        let electron = QuantumParticle::new(2, String::from("Electron"), seed);
        atom.add_contained_particle(electron);

        assert_eq!(atom.mass_number(), 1);
        assert_eq!(atom.atomic_number, 1);
        assert!((atom.particle_charge()).abs() < 0.001); // Neutral atom
    }

    #[test]
    fn test_carbon_atom_structure() {
        let seed = HolographicSeed::new_from_source();
        let mut atom = MultiScaleAtom::new(1, 6, 6, seed.clone()); // Carbon-12

        // Add 6 protons
        for i in 1..=6 {
            let proton = QuantumParticle::new(i, String::from("Proton"), seed.clone());
            atom.add_contained_particle(proton);
        }

        // Add 6 neutrons
        for i in 7..=12 {
            let neutron = QuantumParticle::new(i, String::from("Neutron"), seed.clone());
            atom.add_contained_particle(neutron);
        }

        // Add 6 electrons
        for i in 13..=18 {
            let electron = QuantumParticle::new(i, String::from("Electron"), seed.clone());
            atom.add_contained_particle(electron);
        }

        assert_eq!(atom.mass_number(), 12);
        assert_eq!(atom.atomic_number, 6);
        assert_eq!(atom.contained_particles.len(), 18);
        assert!((atom.particle_charge()).abs() < 0.001); // Neutral atom
    }

    #[test]
    fn test_scale_hierarchy() {
        let seed = HolographicSeed::new_from_source();

        // Create atom
        let mut atom = MultiScaleAtom::new(1, 6, 6, seed.clone());

        // Add quantum particles
        for i in 1..=18 {
            let particle = QuantumParticle::new(i, String::from("Electron"), seed.clone());
            atom.add_contained_particle(particle);
        }

        // Verify scale hierarchy
        assert_eq!(atom.get_scale(), EntityScale::Atomic);
        assert!(atom.contains_holographic_whole());

        let subscale = atom.contains_subscale_entities();
        assert_eq!(subscale.len(), 18);

        for particle in subscale {
            assert_eq!(particle.get_scale(), EntityScale::Quantum);
        }
    }

    // ===== Additional Scale Tests =====

    #[test]
    fn test_molecular_entity() {
        let seed = HolographicSeed::new_from_source();
        let mut molecule = MultiScaleMolecule::new(1, "H2O".to_string(), seed.clone());

        // Add atoms
        for i in 0..3 {
            let atom = MultiScaleAtom::new(i, 1, 0, seed.clone());
            molecule.add_atom(atom);
        }

        assert_eq!(molecule.get_scale(), EntityScale::Molecular);
        assert!(molecule.contains_holographic_whole());
        assert_eq!(molecule.count_atoms(), 3);
    }

    #[test]
    fn test_cellular_entity() {
        let seed = HolographicSeed::new_from_source();
        let cell = MultiScaleCell::new(1, CellType::Eukaryotic, seed);

        assert_eq!(cell.get_scale(), EntityScale::Cellular);
        assert!(cell.contains_holographic_whole());
    }

    #[test]
    fn test_organism_entity() {
        let seed = HolographicSeed::new_from_source();
        let organism = MultiScaleOrganism::new(1, OrganismType::Human, seed);

        assert_eq!(organism.get_scale(), EntityScale::Organism);
        assert!(organism.contains_holographic_whole());
    }

    #[test]
    fn test_collective_entity() {
        let seed = HolographicSeed::new_from_source();
        let collective = MultiScaleCollective::new(1, CollectiveType::SocialGroup, seed);

        assert_eq!(collective.get_scale(), EntityScale::Collective);
        assert!(collective.contains_holographic_whole());
    }

    #[test]
    fn test_planetary_entity() {
        let seed = HolographicSeed::new_from_source();
        let planet = MultiScalePlanetary::new(1, PlanetaryType::Terrestrial, seed);

        assert_eq!(planet.get_scale(), EntityScale::Planetary);
        assert!(planet.contains_holographic_whole());
    }

    #[test]
    fn test_solar_entity() {
        let seed = HolographicSeed::new_from_source();
        let solar = MultiScaleSolar::new(1, SolarType::YellowDwarf, seed);

        assert_eq!(solar.get_scale(), EntityScale::Solar);
        assert!(solar.contains_holographic_whole());
    }

    #[test]
    fn test_galactic_entity() {
        let seed = HolographicSeed::new_from_source();
        let galaxy = MultiScaleGalactic::new(1, GalacticType::Spiral, seed);

        assert_eq!(galaxy.get_scale(), EntityScale::Galactic);
        assert!(galaxy.contains_holographic_whole());
    }

    #[test]
    fn test_cosmic_entity() {
        let seed = HolographicSeed::new_from_source();
        let cosmos = MultiScaleCosmic::new(1, seed);

        assert_eq!(cosmos.get_scale(), EntityScale::Cosmic);
        assert!(cosmos.contains_holographic_whole());
    }
}

// ===== Additional Scale-Specific Entity Types =====

/// MultiScaleMolecule - molecular scale entity
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Molecular-Scale (Stage 2): Molecules"
#[derive(Debug, Clone)]
pub struct MultiScaleMolecule {
    /// Unique ID
    pub id: AtomID,

    /// Molecular formula
    pub formula: String,

    /// Contained atoms
    pub atoms: Vec<MultiScaleAtom>,

    /// Molecular bonds
    pub bonds: Vec<MolecularBond>,

    /// Holographic seed
    pub seed: HolographicSeed,
}

#[derive(Debug, Clone)]
pub struct MolecularBond {
    /// First atom index
    pub atom1: usize,

    /// Second atom index
    pub atom2: usize,

    /// Bond type
    pub bond_type: BondType,

    /// Bond length
    pub length: Float,
}

impl MultiScaleMolecule {
    pub fn new(id: AtomID, formula: String, seed: HolographicSeed) -> Self {
        MultiScaleMolecule {
            id,
            formula,
            atoms: Vec::new(),
            bonds: Vec::new(),
            seed,
        }
    }

    pub fn add_atom(&mut self, atom: MultiScaleAtom) {
        self.atoms.push(atom);
    }

    pub fn add_bond(&mut self, bond: MolecularBond) {
        self.bonds.push(bond);
    }

    pub fn count_atoms(&self) -> usize {
        self.atoms.len()
    }
}

impl MultiScaleEntity for MultiScaleMolecule {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Molecular
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        self.atoms
            .iter()
            .map(|a| a as &dyn MultiScaleEntity)
            .collect()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Cell type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Prokaryotic,
    Eukaryotic,
}

/// MultiScaleCell - cellular scale entity
#[derive(Debug, Clone)]
pub struct MultiScaleCell {
    pub id: AtomID,
    pub cell_type: CellType,
    pub organelles: Vec<String>,
    pub seed: HolographicSeed,
}

impl MultiScaleCell {
    pub fn new(id: AtomID, cell_type: CellType, seed: HolographicSeed) -> Self {
        MultiScaleCell {
            id,
            cell_type,
            organelles: Vec::new(),
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScaleCell {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Cellular
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Organism type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganismType {
    Plant,
    Animal,
    Human,
}

/// MultiScaleOrganism - organism scale entity
#[derive(Debug, Clone)]
pub struct MultiScaleOrganism {
    pub id: AtomID,
    pub organism_type: OrganismType,
    pub seed: HolographicSeed,
}

impl MultiScaleOrganism {
    pub fn new(id: AtomID, organism_type: OrganismType, seed: HolographicSeed) -> Self {
        MultiScaleOrganism {
            id,
            organism_type,
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScaleOrganism {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Organism
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Collective type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectiveType {
    SocialGroup,
    Society,
    Civilization,
}

/// MultiScaleCollective - collective scale entity
#[derive(Debug, Clone)]
pub struct MultiScaleCollective {
    pub id: AtomID,
    pub collective_type: CollectiveType,
    pub seed: HolographicSeed,
}

impl MultiScaleCollective {
    pub fn new(id: AtomID, collective_type: CollectiveType, seed: HolographicSeed) -> Self {
        MultiScaleCollective {
            id,
            collective_type,
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScaleCollective {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Collective
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Planetary type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetaryType {
    Terrestrial,
    GasGiant,
    IceGiant,
}

/// MultiScalePlanetary - planetary scale entity
#[derive(Debug, Clone)]
pub struct MultiScalePlanetary {
    pub id: AtomID,
    pub planetary_type: PlanetaryType,
    pub seed: HolographicSeed,
}

impl MultiScalePlanetary {
    pub fn new(id: AtomID, planetary_type: PlanetaryType, seed: HolographicSeed) -> Self {
        MultiScalePlanetary {
            id,
            planetary_type,
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScalePlanetary {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Planetary
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Solar type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarType {
    RedDwarf,
    YellowDwarf,
    BlueGiant,
}

/// MultiScaleSolar - solar scale entity
#[derive(Debug, Clone)]
pub struct MultiScaleSolar {
    pub id: AtomID,
    pub solar_type: SolarType,
    pub seed: HolographicSeed,
}

impl MultiScaleSolar {
    pub fn new(id: AtomID, solar_type: SolarType, seed: HolographicSeed) -> Self {
        MultiScaleSolar {
            id,
            solar_type,
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScaleSolar {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Solar
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// Galactic type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalacticType {
    Spiral,
    Elliptical,
    Irregular,
}

/// MultiScaleGalactic - galactic scale entity
#[derive(Debug, Clone)]
pub struct MultiScaleGalactic {
    pub id: AtomID,
    pub galactic_type: GalacticType,
    pub seed: HolographicSeed,
}

impl MultiScaleGalactic {
    pub fn new(id: AtomID, galactic_type: GalacticType, seed: HolographicSeed) -> Self {
        MultiScaleGalactic {
            id,
            galactic_type,
            seed,
        }
    }
}

impl MultiScaleEntity for MultiScaleGalactic {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Galactic
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}

/// MultiScaleCosmic - cosmic scale entity (entire creation)
#[derive(Debug, Clone)]
pub struct MultiScaleCosmic {
    pub id: AtomID,
    pub seed: HolographicSeed,
}

impl MultiScaleCosmic {
    pub fn new(id: AtomID, seed: HolographicSeed) -> Self {
        MultiScaleCosmic { id, seed }
    }
}

impl MultiScaleEntity for MultiScaleCosmic {
    fn get_scale(&self) -> EntityScale {
        EntityScale::Cosmic
    }

    fn get_holographic_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity> {
        Vec::new()
    }

    fn contains_holographic_whole(&self) -> bool {
        true
    }

    fn get_id(&self) -> ScaleEntityId {
        self.id
    }
}
