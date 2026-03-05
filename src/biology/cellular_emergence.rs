// Cellular Emergence - Emerge Prokaryotes and Eukaryotes from Blueprint
//
// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 3:
// "Cellular Emergence - Emerge prokaryotes and eukaryotes from blueprint,
// cell division dynamics, cellular consciousness"
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Prokaryotes and simple life emerge from cellular realm"

use crate::biology::dna_system::{Protein, DNA};
use crate::entity_layer7::dna_encoding::DNAPattern;
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::types::Float;
use rand::Rng;

/// Cellular Emergence - Emerges cells from holographic blueprint
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Emerge prokaryotes and eukaryotes from blueprint, cell division dynamics,
/// cellular consciousness"
#[derive(Debug, Clone)]
pub struct CellularEmergence {
    /// Prokaryote emerger
    prokaryote_emerger: ProkaryoteEmerger,

    /// Eukaryote emerger
    eukaryote_emerger: EukaryoteEmerger,

    /// Cell divider
    cell_divider: CellDivider,

    /// Consciousness calculator
    consciousness_calculator: CellularConsciousnessCalculator,

    /// System configuration
    config: CellularEmergenceConfig,
}

impl Default for CellularEmergence {
    fn default() -> Self {
        Self::new()
    }
}

impl CellularEmergence {
    /// Create a new cellular emergence system
    pub fn new() -> Self {
        Self {
            prokaryote_emerger: ProkaryoteEmerger::new(),
            eukaryote_emerger: EukaryoteEmerger::new(),
            cell_divider: CellDivider::new(),
            consciousness_calculator: CellularConsciousnessCalculator::new(),
            config: CellularEmergenceConfig::default(),
        }
    }

    /// Create a cellular emergence system with custom configuration
    pub fn with_config(config: CellularEmergenceConfig) -> Self {
        Self {
            prokaryote_emerger: ProkaryoteEmerger::new(),
            eukaryote_emerger: EukaryoteEmerger::new(),
            cell_divider: CellDivider::new(),
            consciousness_calculator: CellularConsciousnessCalculator::new(),
            config,
        }
    }

    /// Emerge prokaryote from holographic blueprint
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Emerge prokaryotes from blueprint"
    pub fn emerge_prokaryote(&self, blueprint: &HolographicBlueprint) -> Prokaryote {
        self.prokaryote_emerger.emerge(blueprint, &self.config)
    }

    /// Emerge eukaryote from holographic blueprint
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Emerge eukaryotes from blueprint"
    pub fn emerge_eukaryote(&self, blueprint: &HolographicBlueprint) -> Eukaryote {
        self.eukaryote_emerger.emerge(blueprint, &self.config)
    }

    /// Divide cell
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Cell division dynamics"
    pub fn divide_cell(&self, cell: &Cell) -> Vec<Cell> {
        self.cell_divider.divide(cell, &self.config)
    }

    /// Calculate cellular consciousness
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Calculate cellular consciousness"
    pub fn calculate_consciousness(&self, cell: &Cell) -> CellularConsciousness {
        self.consciousness_calculator.calculate(cell)
    }

    /// Get configuration
    pub fn config(&self) -> &CellularEmergenceConfig {
        &self.config
    }
}

/// Cellular Emergence Configuration
#[derive(Debug, Clone)]
pub struct CellularEmergenceConfig {
    /// Cell division rate (0.0 to 1.0)
    pub cell_division_rate: Float,

    /// Mutation rate during division (0.0 to 1.0)
    pub division_mutation_rate: Float,

    /// Consciousness threshold (0.0 to 1.0)
    pub consciousness_threshold: Float,

    /// Energy requirement for division (0.0 to 1.0)
    pub energy_requirement: Float,

    /// Endosymbiosis probability (prokaryote → eukaryote)
    pub endosymbiosis_probability: Float,
}

impl Default for CellularEmergenceConfig {
    fn default() -> Self {
        Self {
            cell_division_rate: 0.1,
            division_mutation_rate: 0.001,
            consciousness_threshold: 0.3,
            energy_requirement: 0.7,
            endosymbiosis_probability: 0.01,
        }
    }
}

/// Cell - Base cell type
#[derive(Debug, Clone)]
pub struct Cell {
    /// Cell identifier
    pub cell_id: String,

    /// Cell type
    pub cell_type: CellType,

    /// DNA
    pub dna: DNA,

    /// Proteins
    pub proteins: Vec<Protein>,

    /// Energy level (0.0 to 1.0)
    pub energy: Float,

    /// Age (in simulation steps)
    pub age: usize,

    /// Health (0.0 to 1.0)
    pub health: Float,

    /// Consciousness level
    pub consciousness: CellularConsciousness,

    /// Position in space
    pub position: (Float, Float, Float),
}

impl Cell {
    /// Create a new cell
    pub fn new(cell_type: CellType, dna: DNA) -> Self {
        Cell {
            cell_id: format!("cell-{}", uuid::Uuid::new_v4()),
            cell_type,
            dna,
            proteins: Vec::new(),
            energy: 1.0,
            age: 0,
            health: 1.0,
            consciousness: CellularConsciousness::default(),
            position: (0.0, 0.0, 0.0),
        }
    }

    /// Update cell state
    pub fn update(&mut self) {
        self.age += 1;
        // Energy naturally decreases
        self.energy = (self.energy - 0.01).max(0.0);
        // Health depends on energy
        self.health = self.energy;
    }

    /// Check if cell can divide
    pub fn can_divide(&self, config: &CellularEmergenceConfig) -> bool {
        self.energy >= config.energy_requirement && self.health > 0.8 && self.age > 10
    }
}

/// Cell Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    /// Prokaryotic cell (no nucleus)
    Prokaryotic,

    /// Eukaryotic cell (with nucleus)
    Eukaryotic,

    /// Specialized cell type
    Specialized,
}

/// Prokaryote - Simple cell without nucleus
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Prokaryotes and simple life"
#[derive(Debug, Clone)]
pub struct Prokaryote {
    /// Base cell structure
    pub cell: Cell,

    /// Cell wall type
    pub cell_wall: CellWallType,

    /// Metabolic pathway
    pub metabolic_pathway: MetabolicPathway,

    /// Reproduction method
    pub reproduction_method: ProkaryoteReproduction,
}

impl Prokaryote {
    /// Create a new prokaryote
    pub fn new(cell: Cell, cell_wall: CellWallType) -> Self {
        Prokaryote {
            cell,
            cell_wall,
            metabolic_pathway: MetabolicPathway::Glycolysis,
            reproduction_method: ProkaryoteReproduction::BinaryFission,
        }
    }

    /// Get cell reference
    pub fn cell(&self) -> &Cell {
        &self.cell
    }

    /// Get mutable cell reference
    pub fn cell_mut(&mut self) -> &mut Cell {
        &mut self.cell
    }

    /// Attempt endosymbiosis (become eukaryote)
    pub fn attempt_endosymbiosis(&self, config: &CellularEmergenceConfig) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<Float>() < config.endosymbiosis_probability
    }
}

/// Cell Wall Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellWallType {
    Peptidoglycan,
    None,
    Pseudopeptidoglycan,
    Glycoprotein,
}

/// Metabolic Pathway
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetabolicPathway {
    Glycolysis,
    Photosynthesis,
    Chemosynthesis,
    AnaerobicRespiration,
}

/// Prokaryote Reproduction Method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProkaryoteReproduction {
    BinaryFission,
    Budding,
    Fragmentation,
}

/// Eukaryote - Complex cell with nucleus and organelles
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Eukaryotes via endosymbiosis"
#[derive(Debug, Clone)]
pub struct Eukaryote {
    /// Base cell structure
    pub cell: Cell,

    /// Nucleus
    pub nucleus: Nucleus,

    /// Mitochondria
    pub mitochondria: Vec<Mitochondrion>,

    /// Other organelles
    pub organelles: Vec<Organelle>,

    /// Cytoskeleton
    pub cytoskeleton: Cytoskeleton,
}

impl Eukaryote {
    /// Create a new eukaryote
    pub fn new(cell: Cell, nucleus: Nucleus) -> Self {
        Eukaryote {
            cell,
            nucleus,
            mitochondria: vec![Mitochondrion::new()],
            organelles: Vec::new(),
            cytoskeleton: Cytoskeleton::new(),
        }
    }

    /// Get cell reference
    pub fn cell(&self) -> &Cell {
        &self.cell
    }

    /// Get mutable cell reference
    pub fn cell_mut(&mut self) -> &mut Cell {
        &mut self.cell
    }

    /// Add organelle
    pub fn add_organelle(&mut self, organelle: Organelle) {
        self.organelles.push(organelle);
    }

    /// Get total energy production
    pub fn energy_production(&self) -> Float {
        self.mitochondria
            .iter()
            .map(|m| m.energy_production())
            .sum()
    }
}

/// Nucleus - Contains DNA in eukaryotes
#[derive(Debug, Clone)]
pub struct Nucleus {
    /// Nuclear envelope (double membrane)
    pub has_envelope: bool,

    /// Nuclear pores
    pub pore_count: usize,

    /// Chromatin state
    pub chromatin_state: ChromatinState,
}

impl Nucleus {
    /// Create a new nucleus
    pub fn new() -> Self {
        Nucleus {
            has_envelope: true,
            pore_count: 100,
            chromatin_state: ChromatinState::Euchromatin,
        }
    }
}

/// Chromatin State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromatinState {
    Euchromatin,     // Active transcription
    Heterochromatin, // Inactive
}

/// Mitochondrion - Powerhouse of the cell
#[derive(Debug, Clone)]
pub struct Mitochondrion {
    /// Inner membrane folds (cristae)
    pub cristae_density: Float,

    /// DNA
    pub dna: Option<DNA>,

    /// Energy production rate
    pub production_rate: Float,
}

impl Mitochondrion {
    /// Create a new mitochondrion
    pub fn new() -> Self {
        Mitochondrion {
            cristae_density: 0.8,
            dna: None,
            production_rate: 1.0,
        }
    }

    /// Calculate energy production
    pub fn energy_production(&self) -> Float {
        self.production_rate * self.cristae_density * 10.0
    }
}

/// Organelle - Eukaryotic cell structure
#[derive(Debug, Clone)]
pub struct Organelle {
    /// Organelle type
    pub organelle_type: OrganelleType,

    /// Size
    pub size: Float,

    /// Function
    pub function: String,
}

/// Organelle Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganelleType {
    EndoplasmicReticulum,
    GolgiApparatus,
    Lysosome,
    Peroxisome,
    Vacuole,
    Chloroplast,
    Ribosome,
}

/// Cytoskeleton - Structural framework
#[derive(Debug, Clone)]
pub struct Cytoskeleton {
    /// Microfilaments
    pub microfilaments: Float,

    /// Microtubules
    pub microtubules: Float,

    /// Intermediate filaments
    pub intermediate_filaments: Float,
}

impl Cytoskeleton {
    /// Create a new cytoskeleton
    pub fn new() -> Self {
        Cytoskeleton {
            microfilaments: 1.0,
            microtubules: 1.0,
            intermediate_filaments: 1.0,
        }
    }
}

/// Cell Division Result
#[derive(Debug, Clone)]
pub struct CellDivisionResult {
    /// Parent cell
    pub parent: Cell,

    /// Daughter cells
    pub daughters: Vec<Cell>,

    /// Division success
    pub success: bool,

    /// Energy consumed
    pub energy_consumed: Float,
}

/// Cellular Consciousness
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Calculate cellular consciousness"
#[derive(Debug, Clone)]
pub struct CellularConsciousness {
    /// Consciousness level (0.0 to 1.0)
    pub level: Float,

    /// Awareness type
    pub awareness_type: AwarenessType,

    /// Response capability
    pub response_capability: Float,

    /// Memory capacity
    pub memory_capacity: Float,
}

impl Default for CellularConsciousness {
    fn default() -> Self {
        CellularConsciousness {
            level: 0.1,
            awareness_type: AwarenessType::BasicStimulusResponse,
            response_capability: 0.2,
            memory_capacity: 0.0,
        }
    }
}

/// Awareness Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwarenessType {
    BasicStimulusResponse,
    AdaptiveResponse,
    PatternRecognition,
    PrimitiveLearning,
}

// ============================================================================
// SUBSYSTEMS
// ============================================================================

/// Prokaryote Emerger
#[derive(Debug, Clone)]
struct ProkaryoteEmerger {
    #[allow(dead_code)]
    emergence_threshold: Float,
}

impl ProkaryoteEmerger {
    fn new() -> Self {
        Self {
            emergence_threshold: 0.5,
        }
    }

    fn emerge(
        &self,
        blueprint: &HolographicBlueprint,
        _config: &CellularEmergenceConfig,
    ) -> Prokaryote {
        let dna_pattern = DNAPattern::cellular_realm(&blueprint.spectrum_configuration);
        let dna = DNA::from_blueprint(dna_pattern);
        let cell = Cell::new(CellType::Prokaryotic, dna);
        let cell_wall = CellWallType::Peptidoglycan;

        Prokaryote::new(cell, cell_wall)
    }
}

/// Eukaryote Emerger
#[derive(Debug, Clone)]
struct EukaryoteEmerger {
    #[allow(dead_code)]
    emergence_threshold: Float,
}

impl EukaryoteEmerger {
    fn new() -> Self {
        Self {
            emergence_threshold: 0.7,
        }
    }

    fn emerge(
        &self,
        blueprint: &HolographicBlueprint,
        _config: &CellularEmergenceConfig,
    ) -> Eukaryote {
        let dna_pattern = DNAPattern::cellular_realm(&blueprint.spectrum_configuration);
        let dna = DNA::from_blueprint(dna_pattern);
        let cell = Cell::new(CellType::Eukaryotic, dna);
        let nucleus = Nucleus::new();

        Eukaryote::new(cell, nucleus)
    }
}

/// Cell Divider
#[derive(Debug, Clone)]
struct CellDivider {
    #[allow(dead_code)]
    division_speed: Float,
}

impl CellDivider {
    fn new() -> Self {
        Self {
            division_speed: 0.8,
        }
    }

    fn divide(&self, cell: &Cell, config: &CellularEmergenceConfig) -> Vec<Cell> {
        let mut rng = rand::thread_rng();

        // Check if division is possible
        if !cell.can_divide(config) {
            return Vec::new();
        }

        // Division succeeds with probability based on division rate
        if rng.gen::<Float>() > config.cell_division_rate {
            return Vec::new();
        }

        // Create two daughter cells
        let mut daughter1 = cell.clone();
        let mut daughter2 = cell.clone();

        // Set new IDs
        daughter1.cell_id = format!("cell-{}", uuid::Uuid::new_v4());
        daughter2.cell_id = format!("cell-{}", uuid::Uuid::new_v4());

        // Reset age
        daughter1.age = 0;
        daughter2.age = 0;

        // Split energy
        daughter1.energy = cell.energy / 2.0;
        daughter2.energy = cell.energy / 2.0;

        // Apply mutations with probability
        if rng.gen::<Float>() < config.division_mutation_rate {
            // Introduce mutation in daughter1
            if !daughter1.dna.sequence.is_empty() {
                let pos = rng.gen_range(0..daughter1.dna.sequence.len());
                daughter1.dna.sequence[pos] = match rng.gen_range(0..4) {
                    0 => crate::biology::dna_system::NucleotideBase::Adenine,
                    1 => crate::biology::dna_system::NucleotideBase::Thymine,
                    2 => crate::biology::dna_system::NucleotideBase::Cytosine,
                    _ => crate::biology::dna_system::NucleotideBase::Guanine,
                };
            }
        }

        vec![daughter1, daughter2]
    }
}

/// Cellular Consciousness Calculator
#[derive(Debug, Clone)]
struct CellularConsciousnessCalculator {
    #[allow(dead_code)]
    calculation_method: ConsciousnessMethod,
}

impl CellularConsciousnessCalculator {
    fn new() -> Self {
        Self {
            calculation_method: ConsciousnessMethod::Integrated,
        }
    }

    fn calculate(&self, cell: &Cell) -> CellularConsciousness {
        // Calculate consciousness based on cell properties
        let level = (cell.energy * 0.4 + cell.health * 0.3 + cell.dna.integrity * 0.3).min(1.0);

        let awareness_type = if level < 0.3 {
            AwarenessType::BasicStimulusResponse
        } else if level < 0.6 {
            AwarenessType::AdaptiveResponse
        } else if level < 0.8 {
            AwarenessType::PatternRecognition
        } else {
            AwarenessType::PrimitiveLearning
        };

        let response_capability = level * 0.8;
        let memory_capacity = if level > 0.5 {
            (level - 0.5) * 2.0
        } else {
            0.0
        };

        CellularConsciousness {
            level,
            awareness_type,
            response_capability,
            memory_capacity,
        }
    }
}

/// Consciousness Calculation Method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum ConsciousnessMethod {
    Integrated,
    EnergyBased,
    DnaBased,
    ComplexityBased,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::IndividualSpectrumConfiguration;
    use crate::spectrum::SpectrumRatio;

    fn create_test_blueprint() -> HolographicBlueprint {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let _config =
            crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            );

        // Create a minimal blueprint using from_spectrum_configuration which is public
        crate::entity_layer7::holographic_blueprint::HolographicBlueprint::from_spectrum_configuration(
            &spectrum_config,
            &crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::new_from_logos(),
        )
    }

    #[test]
    fn test_cellular_emergence_creation() {
        let emergence = CellularEmergence::new();
        assert_eq!(emergence.config().cell_division_rate, 0.1);
    }

    #[test]
    fn test_emerge_prokaryote() {
        let emergence = CellularEmergence::new();
        let blueprint = create_test_blueprint();

        let prokaryote = emergence.emerge_prokaryote(&blueprint);
        assert_eq!(prokaryote.cell.cell_type, CellType::Prokaryotic);
        assert!(!prokaryote.cell.dna.genes.is_empty());
    }

    #[test]
    fn test_emerge_eukaryote() {
        let emergence = CellularEmergence::new();
        let blueprint = create_test_blueprint();

        let eukaryote = emergence.emerge_eukaryote(&blueprint);
        assert_eq!(eukaryote.cell.cell_type, CellType::Eukaryotic);
        assert!(eukaryote.nucleus.has_envelope);
        assert!(!eukaryote.mitochondria.is_empty());
    }

    #[test]
    fn test_cell_division() {
        let config = CellularEmergenceConfig {
            cell_division_rate: 1.0, // 100% division rate for deterministic test
            ..Default::default()
        };
        let emergence = CellularEmergence::with_config(config);
        let blueprint = create_test_blueprint();

        let mut prokaryote = emergence.emerge_prokaryote(&blueprint);
        prokaryote.cell_mut().energy = 1.0;
        prokaryote.cell_mut().age = 20;

        let daughters = emergence.divide_cell(prokaryote.cell());
        assert_eq!(daughters.len(), 2);
        assert_ne!(daughters[0].cell_id, daughters[1].cell_id);
    }

    #[test]
    fn test_cellular_consciousness() {
        let emergence = CellularEmergence::new();
        let blueprint = create_test_blueprint();

        let prokaryote = emergence.emerge_prokaryote(&blueprint);
        let consciousness = emergence.calculate_consciousness(prokaryote.cell());

        assert!(consciousness.level >= 0.0 && consciousness.level <= 1.0);
        assert!(
            consciousness.response_capability >= 0.0 && consciousness.response_capability <= 1.0
        );
    }

    #[test]
    fn test_cell_update() {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::cellular_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );
        let dna = DNA::from_blueprint(dna_pattern);
        let mut cell = Cell::new(CellType::Prokaryotic, dna);

        let initial_age = cell.age;
        let initial_energy = cell.energy;

        cell.update();

        assert_eq!(cell.age, initial_age + 1);
        assert!(cell.energy < initial_energy);
    }

    #[test]
    fn test_cell_can_divide() {
        let config = CellularEmergenceConfig::default();
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::cellular_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );
        let dna = DNA::from_blueprint(dna_pattern);

        let mut cell = Cell::new(CellType::Prokaryotic, dna);
        cell.energy = 1.0;
        cell.age = 20;

        assert!(cell.can_divide(&config));

        cell.energy = 0.5;
        assert!(!cell.can_divide(&config));
    }

    #[test]
    fn test_endosymbiosis() {
        let emergence = CellularEmergence::new();
        let blueprint = create_test_blueprint();

        let prokaryote = emergence.emerge_prokaryote(&blueprint);
        let config = emergence.config();

        // Test with low probability
        let attempts = 100;
        let successes = (0..attempts)
            .filter(|_| prokaryote.attempt_endosymbiosis(config))
            .count();

        // Should have some successes but not all
        assert!(successes < attempts);
    }

    #[test]
    fn test_mitochondrion_energy_production() {
        let mito = Mitochondrion::new();
        let energy = mito.energy_production();

        assert!(energy > 0.0);
        assert_eq!(energy, mito.production_rate * mito.cristae_density * 10.0);
    }

    #[test]
    fn test_eukaryote_energy_production() {
        let emergence = CellularEmergence::new();
        let blueprint = create_test_blueprint();

        let eukaryote = emergence.emerge_eukaryote(&blueprint);
        let total_energy = eukaryote.energy_production();

        assert!(total_energy > 0.0);
    }
}
