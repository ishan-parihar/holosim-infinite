// Physical Structure Definitions - Phase 3: Physical Manifestation System
//
// This module defines the physical structures that manifest at each density sub-level.
// These structures represent the hierarchical composition of matter from quantum particles
// to galaxies, following the Law of One cosmological principles.
//
// Key Principles:
// 1. Consciousness-First: Spectrum patterns exist BEFORE physical matter
// 2. Simultaneous Emergence: Individual and collective emerge together
//    - Atoms and galaxies form simultaneously (1st Density - Atomic)
//    - Molecules and planets form simultaneously (1st Density - Molecular)
//    - Cells and Gaia-system form simultaneously (2nd Density - Cellular)
// 3. Hierarchical Composition: Each structure is composed of smaller structures
//    - Atoms → Molecules → Cells → Organisms → Beings
//    - Quantum Particles → Atoms → Stars → Galaxies
// 4. Each physical structure has a corresponding consciousness entity
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Each density has multiple sub-levels with distinct physical manifestations"
// "The individual and the collective emerge together at each sub-level"

use crate::types::Float;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// PHYSICAL STRUCTURE TYPES
// ============================================================================

/// Physical structure types across all density sub-levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicalStructureType {
    // 1st Density - Quantum Realm
    QuantumParticle,

    // 1st Density - Atomic Realm
    Atom,
    Star, // Collective form at atomic level

    // 1st Density - Molecular Realm
    Molecule,
    Planet, // Collective form at molecular level

    // 1st Density - Planetary Realm
    SolarSystem, // Collective form at planetary level

    // 2nd Density - Cellular Realm
    Cell,
    GaiaSystem, // Collective form at cellular level (planetary consciousness)

    // 2nd Density - Simple Life Realm
    SimpleOrganism,
    Community, // Collective form at simple life level

    // 2nd Density - Complex Life Realm
    ComplexOrganism,
    Ecosystem, // Collective form at complex life level

    // 3rd Density - Conscious Life Realm
    ConsciousOrganism,
    Society, // Collective form at conscious life level

    // Higher densities (4th-8th) - Light-based structures
    LightBody,
    SocialMemoryComplex,
    PlanetaryLogos,
    SolarLogos,
    GalacticLogos,
}

// ============================================================================
// PHYSICAL STRUCTURE BASE
// ============================================================================

/// Base structure for all physical manifestations
#[derive(Debug, Clone)]
pub struct PhysicalStructure {
    /// Unique identifier for this physical structure
    pub id: u64,

    /// Type of physical structure
    pub structure_type: PhysicalStructureType,

    /// Name/description of this structure
    pub name: String,

    /// Density level (1-8)
    pub density_level: u8,

    /// Sub-level within the density
    pub sub_level: String,

    /// Position in space/time coordinates (3D)
    pub position: [Float; 3],

    /// Size/dimensions of the structure
    pub size: [Float; 3],

    /// Mass of the structure (in kg for matter-based, in energy units for light-based)
    pub mass: Float,

    /// Composition: IDs of component structures
    /// For example: Atom is composed of protons, neutrons, electrons
    /// Molecule is composed of atoms
    /// Cell is composed of molecules
    pub composition: Vec<u64>,

    /// Associated consciousness entity ID
    pub consciousness_entity_id: Option<u64>,

    /// Creation timestamp
    pub creation_time: u64,

    /// Current age
    pub age: u64,

    /// Stability (0.0 to 1.0) - how stable is this structure
    pub stability: Float,

    /// Coherence (0.0 to 1.0) - how coherent is this structure internally
    pub coherence: Float,

    /// Energy level of this structure
    pub energy_level: Float,

    /// Metadata about this structure
    pub metadata: HashMap<String, String>,
}

impl PhysicalStructure {
    /// Create a new physical structure
    pub fn new(
        structure_type: PhysicalStructureType,
        name: String,
        density_level: u8,
        sub_level: String,
        position: [Float; 3],
        size: [Float; 3],
        mass: Float,
    ) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let creation_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        PhysicalStructure {
            id,
            structure_type,
            name,
            density_level,
            sub_level,
            position,
            size,
            mass,
            composition: Vec::new(),
            consciousness_entity_id: None,
            creation_time,
            age: 0,
            stability: 1.0,    // Start stable
            coherence: 1.0,    // Start coherent
            energy_level: 1.0, // Start at baseline energy
            metadata: HashMap::new(),
        }
    }

    /// Add a component to this structure
    pub fn add_component(&mut self, component_id: u64) {
        if !self.composition.contains(&component_id) {
            self.composition.push(component_id);
        }
    }

    /// Update the age of this structure
    pub fn update_age(&mut self, current_time: u64) {
        self.age = current_time.saturating_sub(self.creation_time);
    }

    /// Update stability based on internal coherence and external factors
    pub fn update_stability(&mut self, external_factor: Float) {
        // Stability is influenced by internal coherence and external factors
        self.stability = (self.coherence * 0.7 + external_factor * 0.3).clamp(0.0, 1.0);
    }

    /// Check if this structure is stable
    pub fn is_stable(&self) -> bool {
        self.stability > 0.5
    }

    /// Get the number of components in this structure
    pub fn component_count(&self) -> usize {
        self.composition.len()
    }

    /// Get the scale of this structure (in meters)
    pub fn get_scale(&self) -> Float {
        // Return the maximum dimension
        self.size.iter().fold(Float::NAN, |a, &b| a.max(b))
    }
}

// ============================================================================
// QUANTUM STRUCTURES
// ============================================================================

/// Quantum particle structure
#[derive(Debug, Clone)]
pub struct QuantumParticleStructure {
    pub base: PhysicalStructure,
    pub particle_type: String, // "proton", "neutron", "electron", etc.
    pub charge: Float,         // Electric charge
    pub spin: Float,           // Spin quantum number
    pub mass_me_v: Float,      // Mass in MeV/c²
}

impl QuantumParticleStructure {
    /// Create a new quantum particle
    pub fn new(
        particle_type: String,
        charge: Float,
        spin: Float,
        mass_me_v: Float,
        position: [Float; 3],
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::QuantumParticle,
            format!("QuantumParticle_{}", particle_type),
            1,
            "Quantum".to_string(),
            position,
            [1e-15, 1e-15, 1e-15], // ~1 femtometer
            mass_me_v * 1.78e-30,  // Convert MeV to kg
        );

        base.metadata
            .insert("particle_type".to_string(), particle_type.clone());
        base.metadata
            .insert("charge".to_string(), charge.to_string());
        base.metadata.insert("spin".to_string(), spin.to_string());

        QuantumParticleStructure {
            base,
            particle_type,
            charge,
            spin,
            mass_me_v,
        }
    }
}

// ============================================================================
// ATOMIC STRUCTURES
// ============================================================================

/// Atomic structure
#[derive(Debug, Clone)]
pub struct AtomicStructure {
    pub base: PhysicalStructure,
    pub atomic_number: u8,
    pub mass_number: u16,
    pub electron_configuration: String,
    pub chemical_family: String,
}

impl AtomicStructure {
    /// Create a new atomic structure
    pub fn new(
        atomic_number: u8,
        mass_number: u16,
        position: [Float; 3],
        electron_configuration: String,
        chemical_family: String,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::Atom,
            format!("Atom_Z{}", atomic_number),
            1,
            "Atomic".to_string(),
            position,
            [1e-10, 1e-10, 1e-10],           // ~1 angstrom
            mass_number as Float * 1.66e-27, // ~1 amu
        );

        base.metadata
            .insert("atomic_number".to_string(), atomic_number.to_string());
        base.metadata
            .insert("mass_number".to_string(), mass_number.to_string());
        base.metadata
            .insert("chemical_family".to_string(), chemical_family.clone());

        AtomicStructure {
            base,
            atomic_number,
            mass_number,
            electron_configuration,
            chemical_family,
        }
    }
}

// ============================================================================
// STELLAR STRUCTURES
// ============================================================================

/// Star structure (collective form at atomic level)
#[derive(Debug, Clone)]
pub struct StellarStructure {
    pub base: PhysicalStructure,
    pub star_type: String, // "main_sequence", "red_giant", "white_dwarf", etc.
    pub spectral_class: String, // "O", "B", "A", "F", "G", "K", "M"
    pub temperature: Float, // Surface temperature in Kelvin
    pub luminosity: Float, // Luminosity in Solar luminosities
    pub age_billion_years: Float, // Age in billion years
}

impl StellarStructure {
    /// Create a new star
    pub fn new(
        name: String,
        position: [Float; 3],
        mass_solar_masses: Float,
        star_type: String,
        spectral_class: String,
        temperature: Float,
        luminosity: Float,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::Star,
            name,
            1,
            "Atomic".to_string(), // Stars form at Atomic sub-level (simultaneous with atoms)
            position,
            [
                mass_solar_masses * 6.96e8,
                mass_solar_masses * 6.96e8,
                mass_solar_masses * 6.96e8,
            ], // Solar radius scaled
            mass_solar_masses * 1.989e30, // Solar mass
        );

        base.metadata
            .insert("star_type".to_string(), star_type.clone());
        base.metadata
            .insert("spectral_class".to_string(), spectral_class.clone());
        base.metadata
            .insert("temperature".to_string(), temperature.to_string());
        base.metadata
            .insert("luminosity".to_string(), luminosity.to_string());

        StellarStructure {
            base,
            star_type,
            spectral_class,
            temperature,
            luminosity,
            age_billion_years: 0.0,
        }
    }
}

// ============================================================================
// MOLECULAR STRUCTURES
// ============================================================================

/// Molecular structure
#[derive(Debug, Clone)]
pub struct MolecularStructure {
    pub base: PhysicalStructure,
    pub chemical_formula: String,
    pub molecular_weight: Float,
    pub bond_types: Vec<String>, // List of bond types
    pub polarity: Float,         // 0.0 (non-polar) to 1.0 (highly polar)
}

impl MolecularStructure {
    /// Create a new molecular structure
    pub fn new(
        chemical_formula: String,
        position: [Float; 3],
        molecular_weight: Float,
        bond_types: Vec<String>,
        polarity: Float,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::Molecule,
            format!("Molecule_{}", chemical_formula),
            1,
            "Molecular".to_string(),
            position,
            [1e-9, 1e-9, 1e-9],          // ~1 nanometer
            molecular_weight * 1.66e-27, // Convert amu to kg
        );

        base.metadata
            .insert("chemical_formula".to_string(), chemical_formula.clone());
        base.metadata
            .insert("molecular_weight".to_string(), molecular_weight.to_string());
        base.metadata
            .insert("polarity".to_string(), polarity.to_string());

        MolecularStructure {
            base,
            chemical_formula,
            molecular_weight,
            bond_types,
            polarity,
        }
    }
}

// ============================================================================
// PLANETARY STRUCTURES
// ============================================================================

/// Planet structure (collective form at molecular level)
#[derive(Debug, Clone)]
pub struct PlanetaryStructure {
    pub base: PhysicalStructure,
    pub planet_type: String, // "terrestrial", "gas_giant", "ice_giant", "dwarf"
    pub atmosphere_composition: String, // Composition of atmosphere
    pub surface_temperature: Float, // Average surface temperature in Kelvin
    pub has_magnetic_field: bool,
    pub orbital_period_days: Float,
    pub rotation_period_hours: Float,
}

impl PlanetaryStructure {
    /// Create a new planet
    pub fn new(
        name: String,
        position: [Float; 3],
        radius_km: Float,
        mass_earth_masses: Float,
        planet_type: String,
        atmosphere_composition: String,
        surface_temperature: Float,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::Planet,
            name,
            1,
            "Molecular".to_string(), // Planets form at Molecular sub-level (simultaneous with molecules)
            position,
            [radius_km * 1000.0, radius_km * 1000.0, radius_km * 1000.0], // Convert km to meters
            mass_earth_masses * 5.972e24,                                 // Earth mass
        );

        base.metadata
            .insert("planet_type".to_string(), planet_type.clone());
        base.metadata.insert(
            "atmosphere_composition".to_string(),
            atmosphere_composition.clone(),
        );
        base.metadata.insert(
            "surface_temperature".to_string(),
            surface_temperature.to_string(),
        );

        PlanetaryStructure {
            base,
            planet_type,
            atmosphere_composition,
            surface_temperature,
            has_magnetic_field: false,
            orbital_period_days: 0.0,
            rotation_period_hours: 0.0,
        }
    }
}

// ============================================================================
// CELLULAR STRUCTURES
// ============================================================================

/// Cell structure
#[derive(Debug, Clone)]
pub struct CellularStructure {
    pub base: PhysicalStructure,
    pub cell_type: String, // "prokaryotic", "eukaryotic", "plant", "animal"
    pub organelle_types: Vec<String>, // List of organelles present
    pub metabolic_rate: Float, // Relative metabolic rate
    pub division_rate: Float, // Division rate (divisions per hour)
}

impl CellularStructure {
    /// Create a new cell
    pub fn new(
        cell_type: String,
        position: [Float; 3],
        organelle_types: Vec<String>,
        metabolic_rate: Float,
        division_rate: Float,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::Cell,
            format!("Cell_{}", cell_type),
            2,
            "Cellular".to_string(),
            position,
            [1e-5, 1e-5, 1e-5], // ~10 micrometers
            1e-15,              // ~1 picogram
        );

        base.metadata
            .insert("cell_type".to_string(), cell_type.clone());
        base.metadata
            .insert("metabolic_rate".to_string(), metabolic_rate.to_string());
        base.metadata
            .insert("division_rate".to_string(), division_rate.to_string());

        CellularStructure {
            base,
            cell_type,
            organelle_types,
            metabolic_rate,
            division_rate,
        }
    }
}

// ============================================================================
// GAIA SYSTEM STRUCTURES
// ============================================================================

/// Gaia system structure (collective form at cellular level)
#[derive(Debug, Clone)]
pub struct GaiaSystemStructure {
    pub base: PhysicalStructure,
    pub biosphere_stage: String, // "pre-biotic", "primitive", "developing", "mature"
    pub biodiversity_index: Float, // 0.0 to 1.0
    pub consciousness_level: Float, // 0.0 to 1.0 (planetary consciousness)
    pub homeostasis_stability: Float, // 0.0 to 1.0
}

impl GaiaSystemStructure {
    /// Create a new Gaia system
    pub fn new(planet_id: u64, biosphere_stage: String, biodiversity_index: Float) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::GaiaSystem,
            format!("GaiaSystem_{}", planet_id),
            2,
            "Cellular".to_string(), // Gaia emerges at Cellular sub-level (simultaneous with cells)
            [0.0, 0.0, 0.0],        // Position is same as planet
            [0.0, 0.0, 0.0],        // Size is same as planet
            0.0,                    // Mass not applicable (consciousness field)
        );

        base.consciousness_entity_id = Some(planet_id); // Associated with planet
        base.metadata
            .insert("biosphere_stage".to_string(), biosphere_stage.clone());
        base.metadata.insert(
            "biodiversity_index".to_string(),
            biodiversity_index.to_string(),
        );

        GaiaSystemStructure {
            base,
            biosphere_stage,
            biodiversity_index,
            consciousness_level: 0.0,
            homeostasis_stability: 0.5,
        }
    }
}

// ============================================================================
// SIMPLE LIFE STRUCTURES
// ============================================================================

/// Simple organism structure
#[derive(Debug, Clone)]
pub struct SimpleOrganismStructure {
    pub base: PhysicalStructure,
    pub organism_type: String, // "plant", "fungi", "simple_animal", etc.
    pub cell_count: usize,
    pub growth_rate: Float,            // Relative growth rate
    pub reproductive_strategy: String, // "asexual", "sexual"
}

impl SimpleOrganismStructure {
    /// Create a new simple organism
    pub fn new(
        organism_type: String,
        position: [Float; 3],
        cell_count: usize,
        growth_rate: Float,
        reproductive_strategy: String,
    ) -> Self {
        let size = (cell_count as Float).cbrt() * 1e-5; // Scale with cell count

        let mut base = PhysicalStructure::new(
            PhysicalStructureType::SimpleOrganism,
            format!("SimpleOrganism_{}", organism_type),
            2,
            "SimpleLife".to_string(),
            position,
            [size, size, size],
            cell_count as Float * 1e-12, // ~1 trillionth of a gram per cell
        );

        base.metadata
            .insert("organism_type".to_string(), organism_type.clone());
        base.metadata
            .insert("cell_count".to_string(), cell_count.to_string());
        base.metadata
            .insert("growth_rate".to_string(), growth_rate.to_string());

        SimpleOrganismStructure {
            base,
            organism_type,
            cell_count,
            growth_rate,
            reproductive_strategy,
        }
    }
}

// ============================================================================
// COMPLEX LIFE STRUCTURES
// ============================================================================

/// Complex organism structure
#[derive(Debug, Clone)]
pub struct ComplexOrganismStructure {
    pub base: PhysicalStructure,
    pub organism_type: String, // "mammal", "bird", "reptile", "fish", etc.
    pub organ_systems: Vec<String>, // List of organ systems present
    pub nervous_system_complexity: Float, // 0.0 to 1.0
    pub social_behavior: Float, // 0.0 (solitary) to 1.0 (highly social)
}

impl ComplexOrganismStructure {
    /// Create a new complex organism
    pub fn new(
        organism_type: String,
        position: [Float; 3],
        organ_systems: Vec<String>,
        nervous_system_complexity: Float,
        social_behavior: Float,
    ) -> Self {
        let size = match organism_type.as_str() {
            "mammal" => 1.0, // ~1 meter
            "bird" => 0.3,
            "reptile" => 0.5,
            "fish" => 0.4,
            _ => 0.5,
        };

        let mut base = PhysicalStructure::new(
            PhysicalStructureType::ComplexOrganism,
            format!("ComplexOrganism_{}", organism_type),
            2,
            "ComplexLife".to_string(),
            position,
            [size, size, size],
            size * 10.0, // ~10 kg per meter
        );

        base.metadata
            .insert("organism_type".to_string(), organism_type.clone());
        base.metadata.insert(
            "nervous_system_complexity".to_string(),
            nervous_system_complexity.to_string(),
        );
        base.metadata
            .insert("social_behavior".to_string(), social_behavior.to_string());

        ComplexOrganismStructure {
            base,
            organism_type,
            organ_systems,
            nervous_system_complexity,
            social_behavior,
        }
    }
}

// ============================================================================
// CONSCIOUS LIFE STRUCTURES
// ============================================================================

/// Conscious organism structure (3rd density)
#[derive(Debug, Clone)]
pub struct ConsciousOrganismStructure {
    pub base: PhysicalStructure,
    pub species_type: String,       // "human", "humanoid", etc.
    pub consciousness_level: Float, // 0.0 to 1.0 (self-awareness)
    pub free_will_capacity: Float,  // 0.0 to 1.0
    pub polarization_bias: Float,   // -1.0 (STS) to 1.0 (STO)
    pub spiritual_potential: Float, // 0.0 to 1.0
}

impl ConsciousOrganismStructure {
    /// Create a new conscious organism
    pub fn new(
        species_type: String,
        position: [Float; 3],
        consciousness_level: Float,
        free_will_capacity: Float,
    ) -> Self {
        let size = 1.7; // ~1.7 meters (human average)

        let mut base = PhysicalStructure::new(
            PhysicalStructureType::ConsciousOrganism,
            format!("ConsciousOrganism_{}", species_type),
            3,
            "ConsciousLife".to_string(),
            position,
            [size, size, size],
            70.0, // ~70 kg (human average)
        );

        base.metadata
            .insert("species_type".to_string(), species_type.clone());
        base.metadata.insert(
            "consciousness_level".to_string(),
            consciousness_level.to_string(),
        );
        base.metadata.insert(
            "free_will_capacity".to_string(),
            free_will_capacity.to_string(),
        );

        ConsciousOrganismStructure {
            base,
            species_type,
            consciousness_level,
            free_will_capacity,
            polarization_bias: 0.0,   // Start unpolarized
            spiritual_potential: 0.5, // Average potential
        }
    }
}

// ============================================================================
// GALACTIC STRUCTURES
// ============================================================================

/// Galaxy structure (collective form at atomic level - largest scale)
#[derive(Debug, Clone)]
pub struct GalacticStructure {
    pub base: PhysicalStructure,
    pub galaxy_type: String, // "spiral", "elliptical", "irregular"
    pub star_count: u64,
    pub diameter_light_years: Float,
    pub mass_solar_masses: Float,
    pub rotation_period_million_years: Float,
}

impl GalacticStructure {
    /// Create a new galaxy
    pub fn new(
        name: String,
        galaxy_type: String,
        diameter_light_years: Float,
        mass_solar_masses: Float,
    ) -> Self {
        let mut base = PhysicalStructure::new(
            PhysicalStructureType::GalacticLogos,
            name,
            1,
            "Atomic".to_string(), // Galaxies form at Atomic sub-level
            [0.0, 0.0, 0.0],      // Center position
            [
                diameter_light_years * 9.461e15,
                diameter_light_years * 9.461e15,
                diameter_light_years * 9.461e15,
            ], // Convert light years to meters
            mass_solar_masses * 1.989e30, // Solar mass
        );

        base.metadata
            .insert("galaxy_type".to_string(), galaxy_type.clone());
        base.metadata.insert(
            "diameter_light_years".to_string(),
            diameter_light_years.to_string(),
        );

        GalacticStructure {
            base,
            galaxy_type,
            star_count: 100_000_000_000, // ~100 billion stars
            diameter_light_years,
            mass_solar_masses,
            rotation_period_million_years: 200.0,
        }
    }
}

// ============================================================================
// PHYSICAL STRUCTURE MANAGER
// ============================================================================

/// Manages all physical structures in the simulation
#[derive(Debug, Clone)]
pub struct PhysicalStructureManager {
    /// All physical structures indexed by ID
    pub structures: HashMap<u64, PhysicalStructure>,

    /// Quantum particles
    pub quantum_particles: HashMap<u64, QuantumParticleStructure>,

    /// Atomic structures
    pub atomic_structures: HashMap<u64, AtomicStructure>,

    /// Stellar structures
    pub stellar_structures: HashMap<u64, StellarStructure>,

    /// Molecular structures
    pub molecular_structures: HashMap<u64, MolecularStructure>,

    /// Planetary structures
    pub planetary_structures: HashMap<u64, PlanetaryStructure>,

    /// Cellular structures
    pub cellular_structures: HashMap<u64, CellularStructure>,

    /// Gaia systems
    pub gaia_systems: HashMap<u64, GaiaSystemStructure>,

    /// Simple organisms
    pub simple_organisms: HashMap<u64, SimpleOrganismStructure>,

    /// Complex organisms
    pub complex_organisms: HashMap<u64, ComplexOrganismStructure>,

    /// Conscious organisms
    pub conscious_organisms: HashMap<u64, ConsciousOrganismStructure>,

    /// Galactic structures
    pub galactic_structures: HashMap<u64, GalacticStructure>,
}

impl PhysicalStructureManager {
    /// Create a new physical structure manager
    pub fn new() -> Self {
        PhysicalStructureManager {
            structures: HashMap::new(),
            quantum_particles: HashMap::new(),
            atomic_structures: HashMap::new(),
            stellar_structures: HashMap::new(),
            molecular_structures: HashMap::new(),
            planetary_structures: HashMap::new(),
            cellular_structures: HashMap::new(),
            gaia_systems: HashMap::new(),
            simple_organisms: HashMap::new(),
            complex_organisms: HashMap::new(),
            conscious_organisms: HashMap::new(),
            galactic_structures: HashMap::new(),
        }
    }

    /// Add a physical structure to the manager
    pub fn add_structure(&mut self, structure: PhysicalStructure) {
        let id = structure.id;
        self.structures.insert(id, structure.clone());
    }

    /// Get a physical structure by ID
    pub fn get_structure(&self, id: u64) -> Option<&PhysicalStructure> {
        self.structures.get(&id)
    }

    /// Get all structures of a specific type
    pub fn get_structures_by_type(
        &self,
        structure_type: PhysicalStructureType,
    ) -> Vec<&PhysicalStructure> {
        self.structures
            .values()
            .filter(|s| s.structure_type == structure_type)
            .collect()
    }

    /// Get all structures at a specific density level
    pub fn get_structures_by_density(&self, density_level: u8) -> Vec<&PhysicalStructure> {
        self.structures
            .values()
            .filter(|s| s.density_level == density_level)
            .collect()
    }

    /// Get all structures at a specific sub-level
    pub fn get_structures_by_sublevel(&self, sub_level: &str) -> Vec<&PhysicalStructure> {
        self.structures
            .values()
            .filter(|s| s.sub_level == sub_level)
            .collect()
    }

    /// Get statistics about physical structures
    pub fn get_statistics(&self) -> PhysicalStructureStatistics {
        let mut stats = PhysicalStructureStatistics::default();

        for structure in self.structures.values() {
            match structure.structure_type {
                PhysicalStructureType::QuantumParticle => stats.quantum_particle_count += 1,
                PhysicalStructureType::Atom => stats.atom_count += 1,
                PhysicalStructureType::Star => stats.star_count += 1,
                PhysicalStructureType::Molecule => stats.molecule_count += 1,
                PhysicalStructureType::Planet => stats.planet_count += 1,
                PhysicalStructureType::SolarSystem => stats.solar_system_count += 1,
                PhysicalStructureType::Cell => stats.cell_count += 1,
                PhysicalStructureType::GaiaSystem => stats.gaia_system_count += 1,
                PhysicalStructureType::SimpleOrganism => stats.simple_organism_count += 1,
                PhysicalStructureType::Community => stats.community_count += 1,
                PhysicalStructureType::ComplexOrganism => stats.complex_organism_count += 1,
                PhysicalStructureType::Ecosystem => stats.ecosystem_count += 1,
                PhysicalStructureType::ConsciousOrganism => stats.conscious_organism_count += 1,
                PhysicalStructureType::Society => stats.society_count += 1,
                PhysicalStructureType::LightBody => stats.light_body_count += 1,
                PhysicalStructureType::SocialMemoryComplex => {
                    stats.social_memory_complex_count += 1
                }
                PhysicalStructureType::PlanetaryLogos => stats.planetary_logos_count += 1,
                PhysicalStructureType::SolarLogos => stats.solar_logos_count += 1,
                PhysicalStructureType::GalacticLogos => stats.galactic_logos_count += 1,
            }

            stats.total_mass += structure.mass;
        }

        stats.total_structure_count = self.structures.len() as u64;
        stats
    }
}

impl Default for PhysicalStructureManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Statistics about physical structures
#[derive(Debug, Clone, Default)]
pub struct PhysicalStructureStatistics {
    pub total_structure_count: u64,
    pub quantum_particle_count: u64,
    pub atom_count: u64,
    pub star_count: u64,
    pub molecule_count: u64,
    pub planet_count: u64,
    pub solar_system_count: u64,
    pub cell_count: u64,
    pub gaia_system_count: u64,
    pub simple_organism_count: u64,
    pub community_count: u64,
    pub complex_organism_count: u64,
    pub ecosystem_count: u64,
    pub conscious_organism_count: u64,
    pub society_count: u64,
    pub light_body_count: u64,
    pub social_memory_complex_count: u64,
    pub planetary_logos_count: u64,
    pub solar_logos_count: u64,
    pub galactic_logos_count: u64,
    pub total_mass: Float,
}
