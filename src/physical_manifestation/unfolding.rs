// Physical Unfolding Sequence
//
// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 3:
// "Implement Physical Unfolding Sequence"
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "At each stage, the more subtle level creates the conditions for the next more dense level"
// "Quantum → Atomic → Molecular → Cellular → Life → Societal"
//
// This module demonstrates the unfolding of physical forms from holographic blueprint patterns.
// Each stage unfolds from the previous stage, guided by pre-existing spectrum configurations.
//
// This is the key evidence for CONSCIOUSNESS-FIRST COSMOLOGY:
// 1. Holographic blueprint contains spectrum configurations (pre-existing patterns)
// 2. Physical forms unfold from these patterns in sequence
// 3. Each stage creates conditions for the next stage

use crate::physical_manifestation::blueprint::PhysicalBlueprintEncoding;

/// Physical unfolding sequence
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md:
/// "Unfolding from quantum to societal"
///
/// This structure manages the sequential unfolding of physical forms
/// from holographic blueprint patterns. Each stage unfolds from the
/// previous stage, guided by pre-existing spectrum configurations.
#[derive(Debug, Clone)]
pub struct PhysicalUnfolding {
    /// Physical blueprint encoding (pre-existing patterns)
    blueprint_encoding: PhysicalBlueprintEncoding,

    /// Current unfolding stage
    current_stage: UnfoldingStage,

    /// Unfolding progress (0.0 to 1.0)
    progress: f64,
}

impl PhysicalUnfolding {
    /// Create new physical unfolding
    ///
    /// Starts at the quantum realm, the most subtle level of physical manifestation.
    pub fn new() -> Self {
        PhysicalUnfolding {
            blueprint_encoding: PhysicalBlueprintEncoding::default(),
            current_stage: UnfoldingStage::QuantumRealm,
            progress: 0.0,
        }
    }

    /// Initialize from holographic blueprint
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The holographic blueprint for ALL physical existence is encoded BEFORE physical atoms exist"
    ///
    /// This initializes the unfolding sequence from pre-existing spectrum configurations.
    pub fn from_blueprint(blueprint: &PhysicalBlueprintEncoding) -> Self {
        PhysicalUnfolding {
            blueprint_encoding: blueprint.clone(),
            current_stage: UnfoldingStage::QuantumRealm,
            progress: 0.0,
        }
    }

    /// Advance to next unfolding stage
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md:
    /// "Advance through unfolding stages"
    ///
    /// This advances the unfolding sequence to the next stage, using
    /// pre-existing patterns from the holographic blueprint.
    ///
    /// Each stage demonstrates that physical forms unfold from
    /// pre-existing consciousness patterns.
    pub fn advance_stage(&mut self) -> Result<UnfoldingResult, UnfoldingError> {
        match self.current_stage {
            UnfoldingStage::QuantumRealm => {
                // Unfold from quantum particles to atomic structures
                let atoms = self.unfold_atoms_from_quantum()?;
                self.current_stage = UnfoldingStage::AtomicRealm;
                self.progress = 0.125;
                Ok(UnfoldingResult::AtomsFormed(atoms))
            }
            UnfoldingStage::AtomicRealm => {
                // Unfold from atoms to molecules
                let molecules = self.unfold_molecules_from_atoms()?;
                self.current_stage = UnfoldingStage::MolecularRealm;
                self.progress = 0.25;
                Ok(UnfoldingResult::MoleculesFormed(molecules))
            }
            UnfoldingStage::MolecularRealm => {
                // Unfold from molecules to cells
                let cells = self.unfold_cells_from_molecules()?;
                self.current_stage = UnfoldingStage::CellularRealm;
                self.progress = 0.375;
                Ok(UnfoldingResult::CellsFormed(cells))
            }
            UnfoldingStage::CellularRealm => {
                // Unfold from cells to simple life
                let life = self.unfold_simple_life_from_cells()?;
                self.current_stage = UnfoldingStage::SimpleLifeRealm;
                self.progress = 0.5;
                Ok(UnfoldingResult::SimpleLifeFormed(life))
            }
            UnfoldingStage::SimpleLifeRealm => {
                // Unfold from simple life to complex life
                let complex_life = self.unfold_complex_life_from_simple()?;
                self.current_stage = UnfoldingStage::ComplexLifeRealm;
                self.progress = 0.625;
                Ok(UnfoldingResult::ComplexLifeFormed(complex_life))
            }
            UnfoldingStage::ComplexLifeRealm => {
                // Unfold from complex life to conscious life
                let conscious_life = self.unfold_conscious_life_from_complex()?;
                self.current_stage = UnfoldingStage::ConsciousLifeRealm;
                self.progress = 0.75;
                Ok(UnfoldingResult::ConsciousLifeFormed(conscious_life))
            }
            UnfoldingStage::ConsciousLifeRealm => {
                // Unfold from conscious beings to societies
                let societies = self.unfold_societies_from_beings()?;
                self.current_stage = UnfoldingStage::SocietalRealm;
                self.progress = 1.0;
                Ok(UnfoldingResult::SocietiesFormed(societies))
            }
            UnfoldingStage::SocietalRealm => {
                // Unfolding complete
                Err(UnfoldingError::AlreadyComplete)
            }
        }
    }

    /// Unfold atoms from quantum particles
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Atoms form according to pre-existing spectrum configurations"
    ///
    /// This demonstrates consciousness-first: Atomic patterns exist in
    /// holographic blueprint BEFORE physical atoms exist.
    fn unfold_atoms_from_quantum(&self) -> Result<Vec<Atom>, UnfoldingError> {
        // Use atomic patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.atomic_patterns;

        // Create atoms based on pre-existing patterns
        let mut atoms = Vec::new();
        for (i, pattern) in patterns.iter().enumerate() {
            atoms.push(Atom {
                atomic_number: (i + 1) as u32,
                element_name: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                is_stable: pattern.is_stable(),
            });
        }

        Ok(atoms)
    }

    /// Unfold molecules from atoms
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Molecules form according to pre-existing spectrum configurations"
    ///
    /// This demonstrates that molecular patterns exist in holographic
    /// blueprint BEFORE physical molecules exist.
    fn unfold_molecules_from_atoms(&self) -> Result<Vec<Molecule>, UnfoldingError> {
        // Use molecular patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.molecular_patterns;

        // Create molecules based on pre-existing patterns
        let mut molecules = Vec::new();
        for (i, pattern) in patterns.iter().enumerate() {
            molecules.push(Molecule {
                molecule_id: i as u32,
                formula: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                complexity: (i + 1) as u32,
            });
        }

        Ok(molecules)
    }

    /// Unfold cells from molecules
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Cells form according to holographic blueprint"
    ///
    /// This demonstrates that cellular patterns exist in holographic
    /// blueprint BEFORE physical cells exist.
    fn unfold_cells_from_molecules(&self) -> Result<Vec<Cell>, UnfoldingError> {
        // Use cellular patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.cellular_patterns;

        // Create cells based on pre-existing patterns
        let mut cells = Vec::new();
        for (i, pattern) in patterns.iter().enumerate() {
            cells.push(Cell {
                cell_id: i as u32,
                cell_type: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                has_nucleus: i > 0, // First cell is prokaryote
            });
        }

        Ok(cells)
    }

    /// Unfold simple life from cells
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Simple life forms according to holographic blueprint"
    ///
    /// This demonstrates that life patterns exist in holographic
    /// blueprint BEFORE physical life exists.
    fn unfold_simple_life_from_cells(&self) -> Result<Vec<Organism>, UnfoldingError> {
        // Use organismic patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.organismic_patterns;

        // Create simple life forms based on pre-existing patterns
        let mut organisms = Vec::new();
        for (i, pattern) in patterns.iter().enumerate().take(2) {
            // First two are simple life
            organisms.push(Organism {
                organism_id: i as u32,
                species: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                is_conscious: false, // Simple life is not self-aware
            });
        }

        Ok(organisms)
    }

    /// Unfold complex life from simple life
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Complex life forms according to holographic blueprint"
    ///
    /// This demonstrates that complex life patterns exist in holographic
    /// blueprint BEFORE physical complex life exists.
    fn unfold_complex_life_from_simple(&self) -> Result<Vec<Organism>, UnfoldingError> {
        // Use organismic patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.organismic_patterns;

        // Create complex life forms based on pre-existing patterns
        let mut organisms = Vec::new();
        for (i, pattern) in patterns.iter().enumerate().skip(2).take(2) {
            // Next two are complex life
            organisms.push(Organism {
                organism_id: (i + 2) as u32,
                species: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                is_conscious: false, // Complex life is not self-aware
            });
        }

        Ok(organisms)
    }

    /// Unfold conscious life from complex life
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Self-aware beings incarnate"
    ///
    /// This demonstrates that conscious life patterns exist in holographic
    /// blueprint BEFORE physical conscious life exists.
    fn unfold_conscious_life_from_complex(&self) -> Result<Vec<Organism>, UnfoldingError> {
        // Use organismic patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.organismic_patterns;

        // Create conscious life forms based on pre-existing patterns
        let mut organisms = Vec::new();
        for (i, pattern) in patterns.iter().enumerate().skip(4).take(1) {
            // Last one is conscious life
            organisms.push(Organism {
                organism_id: (i + 4) as u32,
                species: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                is_conscious: true, // Conscious life is self-aware
            });
        }

        Ok(organisms)
    }

    /// Unfold societies from conscious beings
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Societies form according to holographic blueprint"
    ///
    /// This demonstrates that societal patterns exist in holographic
    /// blueprint BEFORE physical societies exist.
    fn unfold_societies_from_beings(&self) -> Result<Vec<Society>, UnfoldingError> {
        // Use societal patterns from holographic blueprint
        let patterns = &self.blueprint_encoding.societal_patterns;

        // Create societies based on pre-existing patterns
        let mut societies = Vec::new();
        for (i, pattern) in patterns.iter().enumerate() {
            societies.push(Society {
                society_id: i as u32,
                society_type: pattern.description.clone(),
                spectrum_ratio: pattern.get_ratio(),
                population: 10_i32.pow(i as u32 + 1),
            });
        }

        Ok(societies)
    }

    /// Get current unfolding stage
    pub fn get_current_stage(&self) -> &UnfoldingStage {
        &self.current_stage
    }

    /// Get unfolding progress
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    /// Check if unfolding is complete
    pub fn is_complete(&self) -> bool {
        self.current_stage == UnfoldingStage::SocietalRealm && self.progress >= 1.0
    }
}

impl Default for PhysicalUnfolding {
    fn default() -> Self {
        Self::new()
    }
}

/// Unfolding Stage
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Quantum → Atomic → Molecular → Cellular → Life → Societal"
///
/// Each stage represents a level of physical manifestation that unfolds
/// from the previous stage, guided by pre-existing spectrum configurations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnfoldingStage {
    /// Quantum Realm: Particles and fields
    ///
    /// The most subtle level of physical manifestation.
    /// Quantum particles and fields emerge from spectrum patterns.
    QuantumRealm,

    /// Atomic Realm: Atoms and galaxies
    ///
    /// Atoms and galaxies form according to pre-existing spectrum configurations.
    AtomicRealm,

    /// Molecular Realm: Molecules and planets
    ///
    /// Molecules and planets form according to pre-existing spectrum configurations.
    MolecularRealm,

    /// Cellular Realm: Prokaryotes and simple life
    ///
    /// Cells form according to holographic blueprint.
    CellularRealm,

    /// Simple Life Realm: Plants and simple animals
    ///
    /// Simple life forms according to holographic blueprint.
    SimpleLifeRealm,

    /// Complex Life Realm: Eukaryotes and complex animals
    ///
    /// Complex life forms according to holographic blueprint.
    ComplexLifeRealm,

    /// Conscious Life Realm: Self-aware beings
    ///
    /// Self-aware beings incarnate, capable of choice and polarization.
    ConsciousLifeRealm,

    /// Societal Realm: Societies and civilizations
    ///
    /// Societies form according to holographic blueprint.
    SocietalRealm,
}

/// Result of unfolding operation
#[derive(Debug, Clone)]
pub enum UnfoldingResult {
    /// Atoms formed from quantum particles
    AtomsFormed(Vec<Atom>),

    /// Molecules formed from atoms
    MoleculesFormed(Vec<Molecule>),

    /// Cells formed from molecules
    CellsFormed(Vec<Cell>),

    /// Simple life formed from cells
    SimpleLifeFormed(Vec<Organism>),

    /// Complex life formed from simple life
    ComplexLifeFormed(Vec<Organism>),

    /// Conscious life formed from complex life
    ConsciousLifeFormed(Vec<Organism>),

    /// Societies formed from conscious beings
    SocietiesFormed(Vec<Society>),
}

/// Atom
///
/// Represents an atom that unfolded from quantum particles.
#[derive(Debug, Clone)]
pub struct Atom {
    /// Atomic number
    pub atomic_number: u32,

    /// Element name
    pub element_name: String,

    /// Spectrum ratio (space-time / time-space)
    pub spectrum_ratio: f64,

    /// Whether this atom is stable
    pub is_stable: bool,
}

/// Molecule
///
/// Represents a molecule that unfolded from atoms.
#[derive(Debug, Clone)]
pub struct Molecule {
    /// Molecule ID
    pub molecule_id: u32,

    /// Chemical formula
    pub formula: String,

    /// Spectrum ratio (space-time / time-space)
    pub spectrum_ratio: f64,

    /// Complexity level
    pub complexity: u32,
}

/// Cell
///
/// Represents a cell that unfolded from molecules.
#[derive(Debug, Clone)]
pub struct Cell {
    /// Cell ID
    pub cell_id: u32,

    /// Cell type
    pub cell_type: String,

    /// Spectrum ratio (space-time / time-space)
    pub spectrum_ratio: f64,

    /// Whether this cell has a nucleus (eukaryote)
    pub has_nucleus: bool,
}

/// Organism
///
/// Represents an organism that unfolded from cells.
#[derive(Debug, Clone)]
pub struct Organism {
    /// Organism ID
    pub organism_id: u32,

    /// Species name
    pub species: String,

    /// Spectrum ratio (space-time / time-space)
    pub spectrum_ratio: f64,

    /// Whether this organism is self-aware (conscious)
    pub is_conscious: bool,
}

/// Society
///
/// Represents a society that unfolded from conscious beings.
#[derive(Debug, Clone)]
pub struct Society {
    /// Society ID
    pub society_id: u32,

    /// Society type
    pub society_type: String,

    /// Spectrum ratio (space-time / time-space)
    pub spectrum_ratio: f64,

    /// Population size
    pub population: i32,
}

/// Error in unfolding operation
#[derive(Debug, Clone, PartialEq)]
pub enum UnfoldingError {
    /// Unfolding is already complete
    AlreadyComplete,

    /// Cannot advance (missing prerequisites)
    CannotAdvance(String),

    /// Blueprint encoding error
    BlueprintError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfolding_creation() {
        let unfolding = PhysicalUnfolding::new();

        assert_eq!(unfolding.get_current_stage(), &UnfoldingStage::QuantumRealm);
        assert_eq!(unfolding.get_progress(), 0.0);
    }

    #[test]
    fn test_advance_to_atomic() {
        let mut unfolding = PhysicalUnfolding::new();

        let result = unfolding.advance_stage();
        assert!(result.is_ok());

        assert_eq!(unfolding.get_current_stage(), &UnfoldingStage::AtomicRealm);
        assert_eq!(unfolding.get_progress(), 0.125);

        if let Ok(UnfoldingResult::AtomsFormed(atoms)) = result {
            assert!(!atoms.is_empty());
        } else {
            panic!("Expected AtomsFormed result");
        }
    }

    #[test]
    fn test_complete_unfolding() {
        let mut unfolding = PhysicalUnfolding::new();

        // Advance through all stages
        let mut stages = 0;
        while !unfolding.is_complete() && stages < 10 {
            if unfolding.advance_stage().is_ok() {
                stages += 1;
            } else {
                break;
            }
        }

        assert!(unfolding.is_complete());
        assert_eq!(unfolding.get_progress(), 1.0);
    }

    #[test]
    fn test_unfolding_from_blueprint() {
        let blueprint = PhysicalBlueprintEncoding::default();
        let unfolding = PhysicalUnfolding::from_blueprint(&blueprint);

        assert_eq!(unfolding.get_current_stage(), &UnfoldingStage::QuantumRealm);
    }
}
