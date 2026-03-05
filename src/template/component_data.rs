//! # Component Data Types
//!
//! Component-specific data for use with UniversalTemplate<T>.
//!
//! Each component type defines only its specific data - all shared data
//! (field, spectrum, archetype activation, density, free will seed) is
//! handled by the UniversalTemplate<T> wrapper.
//!
//! ## Design Principles
//!
//! 1. **Minimal Data**: Only store what's unique to this component type
//! 2. **Derived Properties**: Properties should emerge from archetype activation
//! 3. **Serializable**: All data types must be serializable for caching
//! 4. **Clone-Friendly**: Cheap to clone (use Arc for heavy data)


use crate::entity_layer7::layer7::EntityId;
use crate::matter::particle::{Complex, Coordinate3D, Vector3D};
use crate::types::Float;

// =============================================================================
// PARTICLE DATA - Pilot Implementation
// =============================================================================

/// Particle-specific data for UniversalTemplate<ParticleData>
///
/// Properties (mass, charge, spin, lifetime) are DERIVED from the
/// archetype activation profile, not stored as fundamental attributes.
///
/// From R&D Roadmap Phase 2:
/// "Particles derive properties from archetype activation:
///  - Charge: Catalyst archetype (A3) determines charge
///  - Spin: Matrix + Great Way average
///  - Mass: Coherence × Planck mass"
#[derive(Debug, Clone)]
pub struct ParticleData {
    /// Unique particle identifier
    pub id: u64,

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Velocity vector
    pub velocity: Vector3D,

    /// Quantum wavefunction amplitude
    pub wavefunction: Complex,

    /// Creation timestamp
    pub creation_time: u64,

    /// Age in simulation steps
    pub age: u64,

    /// Optional reference to parent entity
    pub parent_entity: Option<EntityId>,

    /// Particle type classification (for rendering/optimization)
    pub particle_type: ParticleType,
}

/// Particle type classification derived from archetype activation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleType {
    /// Electron-like (Catalyst LOW → negative charge)
    Electron,
    /// Proton-like (Catalyst HIGH → positive charge)
    Proton,
    /// Neutron-like (Catalyst BALANCED → neutral)
    Neutron,
    /// Photon-like (pure light, no charge)
    Photon,
    /// Quark-like (fractional charge)
    Quark,
    /// Generic particle (custom archetype pattern)
    Generic,
}

impl ParticleData {
    /// Create new particle data
    pub fn new(id: u64, position: Coordinate3D, velocity: Vector3D) -> Self {
        Self {
            id,
            position,
            velocity,
            wavefunction: Complex::new(1.0, 0.0),
            creation_time: 0,
            age: 0,
            parent_entity: None,
            particle_type: ParticleType::Generic,
        }
    }

    /// Derive particle type from archetype activation
    ///
    /// The type is determined by the Catalyst archetype (A3):
    /// - A3 ≈ 0.0 → Electron (negative charge)
    /// - A3 ≈ 1.0 → Proton (positive charge)
    /// - A3 ≈ 0.5 → Neutron (neutral)
    pub fn derive_particle_type(archetype_activation: &[Float; 22]) -> ParticleType {
        let catalyst = archetype_activation[2]; // A3 = Catalyst

        if catalyst < 0.3 {
            ParticleType::Electron
        } else if catalyst > 0.7 {
            ParticleType::Proton
        } else if (catalyst - 0.5).abs() < 0.1 {
            ParticleType::Neutron
        } else {
            ParticleType::Generic
        }
    }

    /// Update age
    pub fn age(&mut self) {
        self.age += 1;
    }

    /// Move particle by velocity
    pub fn step(&mut self, dt: Float) {
        self.position.x += self.velocity.vx * dt;
        self.position.y += self.velocity.vy * dt;
        self.position.z += self.velocity.vz * dt;
    }
}

// =============================================================================
// ATOM DATA
// =============================================================================

/// Atom-specific data for UniversalTemplate<AtomData>
///
/// An atom is a collection of particles (protons, neutrons, electrons)
/// bound by archetype resonance patterns.
#[derive(Debug, Clone)]
pub struct AtomData {
    /// Unique atom identifier
    pub id: u64,

    /// Atomic number (derived from archetype coherence)
    pub atomic_number: u32,

    /// Mass number (protons + neutrons)
    pub mass_number: u32,

    /// Position of nucleus center
    pub position: Coordinate3D,

    /// Velocity of atom as whole
    pub velocity: Vector3D,

    /// Constituent particle IDs (owned by template system)
    pub protons: Vec<u64>,
    pub neutrons: Vec<u64>,
    pub electrons: Vec<u64>,

    /// Electron configuration (derived from archetype pattern)
    pub electron_configuration: String,

    /// Binding energy (derived from archetype resonance)
    pub binding_energy: Float,
}

impl AtomData {
    /// Create new atom data
    pub fn new(id: u64, atomic_number: u32, position: Coordinate3D) -> Self {
        Self {
            id,
            atomic_number,
            mass_number: atomic_number, // Default: no neutrons
            position,
            velocity: Vector3D::default(),
            protons: Vec::new(),
            neutrons: Vec::new(),
            electrons: Vec::new(),
            electron_configuration: String::new(),
            binding_energy: 0.0,
        }
    }

    /// Get number of electrons (neutral atom)
    pub fn electron_count(&self) -> u32 {
        self.atomic_number
    }
}

// =============================================================================
// MOLECULE DATA
// =============================================================================

/// Molecule-specific data for UniversalTemplate<MoleculeData>
///
/// A molecule is a collection of atoms bound by archetype resonance.
#[derive(Debug, Clone)]
pub struct MoleculeData {
    /// Unique molecule identifier
    pub id: u64,

    /// Molecular formula (derived from composition)
    pub formula: String,

    /// Constituent atom IDs
    pub atoms: Vec<u64>,

    /// Bond information (derived from archetype resonance)
    pub bonds: Vec<BondInfo>,

    /// Center of mass position
    pub position: Coordinate3D,

    /// Center of mass velocity
    pub velocity: Vector3D,

    /// Molecular geometry (derived from field interference)
    pub geometry: MolecularGeometry,
}

/// Bond information derived from archetype resonance
#[derive(Debug, Clone)]
pub struct BondInfo {
    /// Indices of bonded atoms
    pub atom_indices: (usize, usize),

    /// Bond strength (derived from harmonic ratio quality)
    pub strength: Float,

    /// Bond type (derived from archetype pattern)
    pub bond_type: BondType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Resonance,
    Hydrogen,
    Ionic,
}

/// Molecular geometry derived from field interference
#[derive(Debug, Clone)]
pub struct MolecularGeometry {
    /// Bond angles in radians
    pub bond_angles: Vec<Float>,

    /// Dihedral angles in radians
    pub dihedral_angles: Vec<Float>,

    /// Overall molecular shape
    pub shape: MolecularShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MolecularShape {
    Linear,
    Bent,
    TrigonalPlanar,
    Tetrahedral,
    TrigonalPyramidal,
    Octahedral,
    Complex,
}

// =============================================================================
// WORLD DATA
// =============================================================================

/// World/Planet-specific data for UniversalTemplate<WorldData>
///
/// A world is a Sub-Logos level entity with environmental systems.
#[derive(Debug, Clone)]
pub struct WorldData {
    /// Unique world identifier
    pub id: u64,

    /// World name
    pub name: String,

    /// Physical properties
    pub mass: Float,
    pub radius: Float,
    pub gravity: Float,

    /// Position in solar system
    pub position: Coordinate3D,

    /// Orbital properties
    pub orbital_radius: Float,
    pub orbital_velocity: Float,

    /// Environmental systems
    pub atmosphere: AtmosphereData,
    pub terrain: TerrainSummary,
    pub hydrology: HydrologySummary,
}

#[derive(Debug, Clone)]
pub struct AtmosphereData {
    pub composition: Vec<(String, Float)>, // (gas_name, percentage)
    pub pressure: Float,
    pub temperature_average: Float,
}

#[derive(Debug, Clone)]
pub struct TerrainSummary {
    pub dominant_type: String,
    pub elevation_range: (Float, Float),
    pub surface_area: Float,
}

#[derive(Debug, Clone)]
pub struct HydrologySummary {
    pub water_coverage: Float,
    pub ocean_depth_average: Float,
    pub river_count: u32,
}

impl Default for AtmosphereData {
    fn default() -> Self {
        Self {
            composition: vec![
                ("Nitrogen".to_string(), 78.0),
                ("Oxygen".to_string(), 21.0),
                ("Argon".to_string(), 0.93),
            ],
            pressure: 101.325,
            temperature_average: 288.0,
        }
    }
}

impl Default for TerrainSummary {
    fn default() -> Self {
        Self {
            dominant_type: "Rocky".to_string(),
            elevation_range: (-500.0, 8000.0),
            surface_area: 510_100_000.0,
        }
    }
}

impl Default for HydrologySummary {
    fn default() -> Self {
        Self {
            water_coverage: 0.71,
            ocean_depth_average: 3688.0,
            river_count: 0,
        }
    }
}

impl WorldData {
    pub fn new(id: u64, name: String, position: Coordinate3D) -> Self {
        Self {
            id,
            name,
            mass: 5.972e24,
            radius: 6.371e6,
            gravity: 9.81,
            position,
            orbital_radius: 1.496e11,
            orbital_velocity: 29.78,
            atmosphere: AtmosphereData::default(),
            terrain: TerrainSummary::default(),
            hydrology: HydrologySummary::default(),
        }
    }
}

// =============================================================================
// STAR DATA
// =============================================================================

/// Star-specific data for UniversalTemplate<StarData>
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Every element follows the SAME holographic template"
#[derive(Debug, Clone)]
pub struct StarData {
    /// Unique star identifier
    pub id: u64,

    /// Star name
    pub name: String,

    /// Mass in solar masses
    pub mass_solar: Float,

    /// Luminosity in solar luminosities
    pub luminosity_solar: Float,

    /// Effective temperature in Kelvin
    pub temperature: Float,

    /// Spectral class (O, B, A, F, G, K, M)
    pub spectral_class: SpectralClass,

    /// Position in galaxy
    pub position: Coordinate3D,

    /// Age in billions of years
    pub age_gyr: Float,

    /// Planetary system IDs
    pub planetary_systems: Vec<u64>,
}

/// Spectral classification derived from archetype activation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpectralClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl StarData {
    pub fn new(id: u64, name: String, position: Coordinate3D) -> Self {
        Self {
            id,
            name,
            mass_solar: 1.0,
            luminosity_solar: 1.0,
            temperature: 5778.0,
            spectral_class: SpectralClass::G,
            position,
            age_gyr: 4.6,
            planetary_systems: Vec::new(),
        }
    }

    /// Derive spectral class from archetype activation
    ///
    /// The spectral class is determined by archetype coherence:
    /// - Higher coherence → hotter stars (O, B)
    /// - Lower coherence → cooler stars (K, M)
    pub fn derive_spectral_class(archetype_activation: &[Float; 22]) -> SpectralClass {
        let coherence: Float = archetype_activation.iter().sum::<Float>() / 22.0;

        if coherence > 0.95 {
            SpectralClass::O
        } else if coherence > 0.85 {
            SpectralClass::B
        } else if coherence > 0.75 {
            SpectralClass::A
        } else if coherence > 0.65 {
            SpectralClass::F
        } else if coherence > 0.50 {
            SpectralClass::G
        } else if coherence > 0.35 {
            SpectralClass::K
        } else {
            SpectralClass::M
        }
    }
}

// =============================================================================
// GALAXY DATA
// =============================================================================

/// Galaxy-specific data for UniversalTemplate<GalaxyData>
///
/// A galaxy is a collection of stellar systems bound by archetype resonance.
#[derive(Debug, Clone)]
pub struct GalaxyData {
    /// Unique galaxy identifier
    pub id: u64,

    /// Galaxy name
    pub name: String,

    /// Galaxy type (spiral, elliptical, irregular)
    pub galaxy_type: GalaxyType,

    /// Mass in solar masses
    pub mass_solar: Float,

    /// Diameter in light-years
    pub diameter_ly: Float,

    /// Number of stars (approximate)
    pub star_count: u64,

    /// Position in cosmic web
    pub position: Coordinate3D,

    /// Central black hole mass in solar masses
    pub central_black_hole_mass: Float,

    /// Star system IDs
    pub star_systems: Vec<u64>,

    /// Consciousness level (for Law of One alignment)
    pub consciousness_level: Float,
}

/// Galaxy type classification derived from archetype patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GalaxyType {
    Spiral,
    BarredSpiral,
    Elliptical,
    Irregular,
    Lenticular,
}

impl GalaxyData {
    pub fn new(id: u64, name: String, position: Coordinate3D) -> Self {
        Self {
            id,
            name,
            galaxy_type: GalaxyType::Spiral,
            mass_solar: 1.5e12,
            diameter_ly: 100_000.0,
            star_count: 200_000_000_000,
            position,
            central_black_hole_mass: 4.0e6,
            star_systems: Vec::new(),
            consciousness_level: 0.5,
        }
    }

    /// Derive galaxy type from archetype activation
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Galactic Logoi configure the archetype system for their region"
    pub fn derive_galaxy_type(archetype_activation: &[Float; 22]) -> GalaxyType {
        let mind: Float = archetype_activation[0..7].iter().sum();
        let body: Float = archetype_activation[7..14].iter().sum();
        let spirit: Float = archetype_activation[14..21].iter().sum();

        let mind_ratio = mind / (mind + body + spirit + 0.001);
        let spirit_ratio = spirit / (mind + body + spirit + 0.001);

        if mind_ratio > 0.5 {
            GalaxyType::Spiral
        } else if spirit_ratio > 0.5 {
            GalaxyType::Elliptical
        } else if (mind_ratio - 0.4).abs() < 0.1 {
            GalaxyType::BarredSpiral
        } else if mind < 2.0 && spirit < 2.0 {
            GalaxyType::Irregular
        } else {
            GalaxyType::Lenticular
        }
    }
}

// =============================================================================
// CELL DATA
// =============================================================================

/// Cell-specific data for UniversalTemplate<CellData>
///
/// From Phase 10 of roadmap:
/// "DNA/RNA patterns UNFOLD from the pre-existing holographic blueprint"
#[derive(Debug, Clone)]
pub struct CellData {
    /// Unique cell identifier
    pub id: u64,

    /// Cell type
    pub cell_type: CellType,

    /// Position in organism
    pub position: Coordinate3D,

    /// Membrane integrity (0.0 to 1.0)
    pub membrane_integrity: Float,

    /// Energy level (ATP concentration proxy)
    pub energy_level: Float,

    /// Gene expression profile
    pub gene_expression: Vec<(u32, Float)>,

    /// Age in simulation steps
    pub age: u64,

    /// Division state
    pub division_state: DivisionState,

    /// Parent organism ID
    pub parent_organism: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellType {
    Prokaryote,
    Eukaryote,
    Stem,
    Muscle,
    Nerve,
    Epithelial,
    Blood,
    Immune,
    Reproductive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivisionState {
    Interphase,
    Prophase,
    Metaphase,
    Anaphase,
    Telophase,
    Cytokinesis,
}

impl CellData {
    pub fn new(id: u64, cell_type: CellType, position: Coordinate3D) -> Self {
        Self {
            id,
            cell_type,
            position,
            membrane_integrity: 1.0,
            energy_level: 1.0,
            gene_expression: Vec::new(),
            age: 0,
            division_state: DivisionState::Interphase,
            parent_organism: None,
        }
    }

    /// Derive cell type from archetype activation
    ///
    /// Cell types emerge from archetype patterns:
    /// - Mind-dominant → Nerve cells
    /// - Body-dominant → Muscle cells
    /// - Spirit-dominant → Stem cells
    pub fn derive_cell_type(archetype_activation: &[Float; 22]) -> CellType {
        let mind: Float = archetype_activation[0..7].iter().sum();
        let body: Float = archetype_activation[7..14].iter().sum();
        let spirit: Float = archetype_activation[14..21].iter().sum();

        let total = mind + body + spirit + 0.001;
        let mind_ratio = mind / total;
        let body_ratio = body / total;
        let spirit_ratio = spirit / total;

        if spirit_ratio > 0.5 {
            CellType::Stem
        } else if mind_ratio > 0.5 {
            CellType::Nerve
        } else if body_ratio > 0.5 {
            CellType::Muscle
        } else if (mind_ratio - 0.3).abs() < 0.1 {
            CellType::Epithelial
        } else if (body_ratio - 0.3).abs() < 0.1 {
            CellType::Blood
        } else {
            CellType::Eukaryote
        }
    }
}

// =============================================================================
// IMPLEMENTATION OF TemplateComponent TRAIT
// =============================================================================

/// Trait for types that can be used as component data in UniversalTemplate
pub trait TemplateComponent: Clone + std::fmt::Debug + Send + Sync {
    /// Create component data from a template configuration
    fn from_template_config(config: &super::TemplateConfig) -> Self;

    /// Get unique identifier for this component
    fn component_id(&self) -> u64;

    /// Check if this component is active/valid
    fn is_active(&self) -> bool;
}

impl TemplateComponent for ParticleData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        let particle_type = Self::derive_particle_type(&config.archetype_activation.coefficients);
        Self {
            id: config.free_will_seed, // Use seed as ID
            position: Coordinate3D::default(),
            velocity: Vector3D::default(),
            wavefunction: Complex::new(1.0, 0.0),
            creation_time: 0,
            age: 0,
            parent_entity: None,
            particle_type,
        }
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        true // Particles are always active once created
    }
}

impl TemplateComponent for AtomData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        // Atomic number derived from archetype coherence
        let coherence: Float = config
            .archetype_activation
            .coefficients
            .iter()
            .sum::<Float>()
            / 22.0;
        let atomic_number = ((coherence * 118.0).round() as u32).clamp(1, 118);

        Self::new(
            config.free_will_seed,
            atomic_number,
            Coordinate3D::default(),
        )
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        self.protons.len() == self.atomic_number as usize
    }
}

impl TemplateComponent for WorldData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        Self::new(
            config.free_will_seed,
            format!("World-{}", config.free_will_seed),
            Coordinate3D::default(),
        )
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        self.mass > 0.0 && self.radius > 0.0
    }
}

impl TemplateComponent for StarData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        let spectral_class = Self::derive_spectral_class(&config.archetype_activation.coefficients);
        let mut star = Self::new(
            config.free_will_seed,
            format!("Star-{}", config.free_will_seed),
            Coordinate3D::default(),
        );
        star.spectral_class = spectral_class;
        star
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        self.mass_solar > 0.0 && self.luminosity_solar > 0.0
    }
}

impl TemplateComponent for GalaxyData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        let galaxy_type = Self::derive_galaxy_type(&config.archetype_activation.coefficients);
        let mut galaxy = Self::new(
            config.free_will_seed,
            format!("Galaxy-{}", config.free_will_seed),
            Coordinate3D::default(),
        );
        galaxy.galaxy_type = galaxy_type;
        galaxy
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        self.mass_solar > 0.0 && self.star_count > 0
    }
}

impl TemplateComponent for CellData {
    fn from_template_config(config: &super::TemplateConfig) -> Self {
        let cell_type = Self::derive_cell_type(&config.archetype_activation.coefficients);
        let mut cell = Self::new(config.free_will_seed, cell_type, Coordinate3D::default());
        cell.cell_type = cell_type;
        cell
    }

    fn component_id(&self) -> u64 {
        self.id
    }

    fn is_active(&self) -> bool {
        self.membrane_integrity > 0.0 && self.energy_level > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_data_creation() {
        let particle = ParticleData::new(1, Coordinate3D::default(), Vector3D::default());
        assert_eq!(particle.id, 1);
        assert_eq!(particle.age, 0);
    }

    #[test]
    fn test_derive_particle_type_electron() {
        let mut activation = [0.5; 22];
        activation[2] = 0.1; // Low Catalyst
        assert_eq!(
            ParticleData::derive_particle_type(&activation),
            ParticleType::Electron
        );
    }

    #[test]
    fn test_derive_particle_type_proton() {
        let mut activation = [0.5; 22];
        activation[2] = 0.9; // High Catalyst
        assert_eq!(
            ParticleData::derive_particle_type(&activation),
            ParticleType::Proton
        );
    }

    #[test]
    fn test_derive_particle_type_neutron() {
        let mut activation = [0.5; 22];
        activation[2] = 0.5; // Balanced Catalyst
        assert_eq!(
            ParticleData::derive_particle_type(&activation),
            ParticleType::Neutron
        );
    }

    #[test]
    fn test_atom_data_creation() {
        let atom = AtomData::new(1, 6, Coordinate3D::default()); // Carbon
        assert_eq!(atom.atomic_number, 6);
        assert_eq!(atom.electron_count(), 6);
    }

    #[test]
    fn test_world_data_creation() {
        let world = WorldData::new(1, "Earth".to_string(), Coordinate3D::default());
        assert_eq!(world.id, 1);
        assert_eq!(world.name, "Earth");
        assert!(world.mass > 0.0);
        assert!(world.is_active());
    }

    #[test]
    fn test_star_data_creation() {
        let star = StarData::new(1, "Sol".to_string(), Coordinate3D::default());
        assert_eq!(star.id, 1);
        assert_eq!(star.spectral_class, SpectralClass::G);
    }

    #[test]
    fn test_derive_spectral_class() {
        let hot_activation = [0.97; 22];
        assert_eq!(
            StarData::derive_spectral_class(&hot_activation),
            SpectralClass::O
        );

        let cool_activation = [0.2; 22];
        assert_eq!(
            StarData::derive_spectral_class(&cool_activation),
            SpectralClass::M
        );

        let medium_activation = [0.55; 22];
        assert_eq!(
            StarData::derive_spectral_class(&medium_activation),
            SpectralClass::G
        );
    }

    #[test]
    fn test_galaxy_data_creation() {
        let galaxy = GalaxyData::new(1, "Milky Way".to_string(), Coordinate3D::default());
        assert_eq!(galaxy.id, 1);
        assert!(galaxy.star_count > 0);
        assert!(galaxy.is_active());
    }

    #[test]
    fn test_derive_galaxy_type() {
        let mut mind_dominant = [0.3; 22];
        for item in mind_dominant.iter_mut().take(7) {
            *item = 0.9;
        }
        assert_eq!(
            GalaxyData::derive_galaxy_type(&mind_dominant),
            GalaxyType::Spiral
        );

        let mut spirit_dominant = [0.3; 22];
        for item in spirit_dominant.iter_mut().skip(14).take(7) {
            *item = 0.9;
        }
        assert_eq!(
            GalaxyData::derive_galaxy_type(&spirit_dominant),
            GalaxyType::Elliptical
        );
    }

    #[test]
    fn test_cell_data_creation() {
        let cell = CellData::new(1, CellType::Nerve, Coordinate3D::default());
        assert_eq!(cell.id, 1);
        assert_eq!(cell.cell_type, CellType::Nerve);
        assert!(cell.is_active());
    }

    #[test]
    fn test_derive_cell_type() {
        let mut spirit_dominant = [0.3; 22];
        for item in spirit_dominant.iter_mut().skip(14).take(7) {
            *item = 0.9;
        }
        assert_eq!(CellData::derive_cell_type(&spirit_dominant), CellType::Stem);

        let mut mind_dominant = [0.3; 22];
        for item in mind_dominant.iter_mut().take(7) {
            *item = 0.9;
        }
        assert_eq!(CellData::derive_cell_type(&mind_dominant), CellType::Nerve);

        let mut body_dominant = [0.3; 22];
        for item in body_dominant.iter_mut().skip(7).take(7) {
            *item = 0.9;
        }
        assert_eq!(CellData::derive_cell_type(&body_dominant), CellType::Muscle);
    }
}
