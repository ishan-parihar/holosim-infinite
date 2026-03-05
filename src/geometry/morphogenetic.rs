//! Morphogenetic Templates
//!
//! From HOLOSIM_INFINITE_RnD_ROADMAP_V5.md Phase 8.3:
//! "Templates for atom->molecule->cell->organism"
//! "Standing wave stability conditions"
//!
//! ## Overview
//!
//! Morphogenetic templates define the standing wave patterns that guide the
//! emergence of physical forms from the holographic field. These templates
//! are "molds" that Intelligent-Infinity uses to shape matter.
//!
//! ## Key Concepts
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Matter condenses from field coherence according to morphogenetic templates"
//!
//! The templates operate at multiple scales:
//! - **Atom**: Standing wave patterns for electron shells
//! - **Molecule**: Templates for molecular geometry
//! - **Cell**: Templates for cellular structure
//! - **Organism**: Templates for body plans
//!
//! Each template includes:
//! - Standing wave configuration
//! - Stability conditions (what keeps the form coherent)
//! - Developmental stages (how the form emerges)

use crate::sacred_geometry::{
    constants, InterferenceType, SacredGeometrySystem, StandingWave,
};
use crate::types::Float;
use std::collections::HashMap;

/// Element types for atomic templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementType {
    /// Hydrogen (Z=1)
    Hydrogen,
    /// Helium (Z=2)
    Helium,
    /// Carbon (Z=6)
    Carbon,
    /// Nitrogen (Z=7)
    Nitrogen,
    /// Oxygen (Z=8)
    Oxygen,
    /// Silicon (Z=14)
    Silicon,
    /// Iron (Z=26)
    Iron,
    /// Gold (Z=79)
    Gold,
    /// Custom element
    Custom(u8),
}

impl ElementType {
    /// Get atomic number
    pub fn atomic_number(&self) -> u8 {
        match self {
            ElementType::Hydrogen => 1,
            ElementType::Helium => 2,
            ElementType::Carbon => 6,
            ElementType::Nitrogen => 7,
            ElementType::Oxygen => 8,
            ElementType::Silicon => 14,
            ElementType::Iron => 26,
            ElementType::Gold => 79,
            ElementType::Custom(z) => *z,
        }
    }

    /// Get base frequency for this element
    /// Higher atomic number = higher base frequency
    pub fn base_frequency(&self) -> Float {
        432.0 * (self.atomic_number() as Float).sqrt()
    }

    /// Get number of electron shells
    pub fn electron_shells(&self) -> u8 {
        let z = self.atomic_number();
        match z {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 7,
        }
    }

    /// Check if this is a noble gas
    pub fn is_noble_gas(&self) -> bool {
        matches!(self, ElementType::Helium)
            || matches!(self.atomic_number(), 10 | 18 | 36 | 54 | 86 | 118)
    }
}

/// Cell types for cellular templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellType {
    /// Prokaryotic cell (no nucleus)
    Prokaryote,
    /// Eukaryotic cell (with nucleus)
    Eukaryote,
    /// Plant cell
    Plant,
    /// Animal cell
    Animal,
    /// Neuron
    Neuron,
    /// Muscle cell
    Muscle,
    /// Stem cell
    Stem,
    /// Red blood cell
    RedBloodCell,
    /// White blood cell
    WhiteBloodCell,
}

impl CellType {
    /// Get complexity level (1-10)
    pub fn complexity(&self) -> u8 {
        match self {
            CellType::Prokaryote => 2,
            CellType::Eukaryote => 5,
            CellType::Plant => 6,
            CellType::Animal => 6,
            CellType::Neuron => 8,
            CellType::Muscle => 5,
            CellType::Stem => 7,
            CellType::RedBloodCell => 3,
            CellType::WhiteBloodCell => 6,
        }
    }

    /// Get base frequency for this cell type
    pub fn base_frequency(&self) -> Float {
        432.0 * (self.complexity() as Float)
    }
}

/// Body plan types for organism templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyPlan {
    /// Single cell
    Unicellular,
    /// Simple multicellular (no tissues)
    SimpleMulticellular,
    /// Radial symmetry (jellyfish, sea anemone)
    RadialSymmetry,
    /// Bilateral symmetry (most animals)
    BilateralSymmetry,
    /// Segmented body (annelids, arthropods)
    Segmented,
    /// Vertebrate body plan
    Vertebrate,
    /// Human body plan
    Human,
}

impl BodyPlan {
    /// Get complexity level (1-10)
    pub fn complexity(&self) -> u8 {
        match self {
            BodyPlan::Unicellular => 1,
            BodyPlan::SimpleMulticellular => 3,
            BodyPlan::RadialSymmetry => 5,
            BodyPlan::BilateralSymmetry => 6,
            BodyPlan::Segmented => 7,
            BodyPlan::Vertebrate => 8,
            BodyPlan::Human => 10,
        }
    }

    /// Get number of developmental stages
    pub fn developmental_stages(&self) -> usize {
        match self {
            BodyPlan::Unicellular => 2,
            BodyPlan::SimpleMulticellular => 4,
            BodyPlan::RadialSymmetry => 6,
            BodyPlan::BilateralSymmetry => 8,
            BodyPlan::Segmented => 10,
            BodyPlan::Vertebrate => 12,
            BodyPlan::Human => 14,
        }
    }
}

/// Stability conditions for a morphogenetic template
#[derive(Debug, Clone)]
pub struct StabilityConditions {
    /// Minimum coherence required (0.0 to 1.0)
    pub min_coherence: Float,

    /// Maximum entropy allowed (0.0 to 1.0)
    pub max_entropy: Float,

    /// Required archetype alignment (0.0 to 1.0)
    pub archetype_alignment: Float,

    /// Temperature range (K)
    pub temperature_range: (Float, Float),

    /// Pressure range (atm)
    pub pressure_range: (Float, Float),

    /// Energy stability threshold
    pub energy_threshold: Float,

    /// Golden ratio alignment requirement
    pub phi_alignment: Float,
}

impl Default for StabilityConditions {
    fn default() -> Self {
        Self {
            min_coherence: 0.5,
            max_entropy: 0.5,
            archetype_alignment: 0.5,
            temperature_range: (0.0, 1000.0),
            pressure_range: (0.0, 100.0),
            energy_threshold: 0.1,
            phi_alignment: 0.618, // Golden ratio inverse
        }
    }
}

impl StabilityConditions {
    /// Create stability conditions for an atom
    pub fn for_element(element: ElementType) -> Self {
        let base_coherence = if element.is_noble_gas() {
            0.95 // Noble gases are very stable
        } else {
            0.5 + 0.1 * (element.electron_shells() as Float)
        };

        Self {
            min_coherence: base_coherence,
            max_entropy: 0.3,
            archetype_alignment: 0.7,
            temperature_range: (0.0, 10000.0),
            pressure_range: (0.0, 1000.0),
            energy_threshold: element.atomic_number() as Float * 0.1,
            phi_alignment: constants::GOLDEN_RATIO_INVERSE,
        }
    }

    /// Create stability conditions for a cell
    pub fn for_cell(cell_type: CellType) -> Self {
        Self {
            min_coherence: 0.6 + 0.03 * cell_type.complexity() as Float,
            max_entropy: 0.4,
            archetype_alignment: 0.6,
            temperature_range: (273.0, 315.0), // Biological temperature range
            pressure_range: (0.8, 1.2),        // Near atmospheric pressure
            energy_threshold: cell_type.complexity() as Float * 0.5,
            phi_alignment: constants::GOLDEN_RATIO_INVERSE,
        }
    }

    /// Create stability conditions for an organism
    pub fn for_body_plan(body_plan: BodyPlan) -> Self {
        Self {
            min_coherence: 0.7 + 0.02 * body_plan.complexity() as Float,
            max_entropy: 0.3,
            archetype_alignment: 0.7,
            temperature_range: (273.0, 315.0),
            pressure_range: (0.8, 1.2),
            energy_threshold: body_plan.complexity() as Float,
            phi_alignment: constants::GOLDEN_RATIO_INVERSE,
        }
    }

    /// Check if conditions are met
    pub fn is_stable(
        &self,
        coherence: Float,
        entropy: Float,
        temperature: Float,
        pressure: Float,
    ) -> bool {
        coherence >= self.min_coherence
            && entropy <= self.max_entropy
            && temperature >= self.temperature_range.0
            && temperature <= self.temperature_range.1
            && pressure >= self.pressure_range.0
            && pressure <= self.pressure_range.1
    }
}

/// Developmental stage of a morphogenetic form
#[derive(Debug, Clone)]
pub struct DevelopmentalStage {
    /// Stage number
    pub stage: usize,

    /// Stage name
    pub name: String,

    /// Required coherence at this stage
    pub required_coherence: Float,

    /// Energy level at this stage
    pub energy_level: Float,

    /// Pattern coefficients at this stage
    pub pattern_coefficients: Vec<Float>,

    /// Duration in simulation ticks
    pub duration: u64,
}

impl DevelopmentalStage {
    /// Create a new developmental stage
    pub fn new(stage: usize, name: &str, coherence: Float, energy: Float) -> Self {
        let mut coefficients = Vec::new();
        for i in 0..22 {
            // Each stage has different archetype coefficients
            coefficients.push((1.0 / (stage + 1) as Float + i as Float * 0.01).min(1.0));
        }

        Self {
            stage,
            name: name.to_string(),
            required_coherence: coherence,
            energy_level: energy,
            pattern_coefficients: coefficients,
            duration: (stage as u64 + 1) * 100,
        }
    }
}

/// Morphogenetic Template
///
/// From ROADMAP Phase 8.3:
/// "Templates for atom->molecule->cell->organism"
///
/// A morphogenetic template defines the standing wave pattern that guides
/// the emergence of a physical form from the holographic field.
#[derive(Debug, Clone)]
pub struct MorphogeneticTemplate {
    /// Template name
    pub name: String,

    /// Template type
    pub template_type: TemplateType,

    /// Standing wave configuration
    pub standing_wave: StandingWave,

    /// Stability conditions
    pub stability_conditions: StabilityConditions,

    /// Developmental stages
    pub stages: Vec<DevelopmentalStage>,

    /// Sacred geometry integration
    pub geometry: SacredGeometrySystem,

    /// Pattern coefficients (archetype alignment)
    pub pattern_coefficients: Vec<Float>,

    /// Template energy
    pub template_energy: Float,

    /// Creation timestamp
    pub created_at: u64,
}

/// Types of morphogenetic templates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateType {
    /// Atomic template
    Atom(ElementType),

    /// Molecular template
    Molecule(String), // Formula like "H2O", "C6H12O6"

    /// Cellular template
    Cell(CellType),

    /// Organism template
    Organism(BodyPlan),

    /// Custom template
    Custom(String),
}

impl MorphogeneticTemplate {
    /// Create a morphogenetic template for an element
    pub fn for_atom(element: ElementType) -> Self {
        let geometry = SacredGeometrySystem::new();

        // Create standing wave based on electron shells
        let primary_freq = element.base_frequency();
        let secondary_freq = primary_freq * constants::GOLDEN_RATIO;
        let standing_wave = StandingWave::calculate(primary_freq, secondary_freq);

        // Create stability conditions
        let stability_conditions = StabilityConditions::for_element(element);

        // Create developmental stages (electron shell filling)
        let shells = element.electron_shells() as usize;
        let stages: Vec<DevelopmentalStage> = (0..shells)
            .map(|i| {
                DevelopmentalStage::new(
                    i,
                    &format!("Shell {} filling", i + 1),
                    0.5 + i as Float * 0.1,
                    element.atomic_number() as Float * (i + 1) as Float * 0.1,
                )
            })
            .collect();

        // Create pattern coefficients from atomic number
        let z = element.atomic_number();
        let pattern_coefficients: Vec<Float> = (0..22)
            .map(|i| {
                ((z as Float * constants::GOLDEN_RATIO.powi(i)) % 1.0)
                    .clamp(0.3, 1.0)
            })
            .collect();

        let name = format!("{:?} atom template", element);

        Self {
            name,
            template_type: TemplateType::Atom(element),
            standing_wave,
            stability_conditions,
            stages,
            geometry,
            pattern_coefficients,
            template_energy: element.atomic_number() as Float * 10.0,
            created_at: 0,
        }
    }

    /// Create a morphogenetic template for a molecule
    pub fn for_molecule(formula: &str) -> Self {
        let geometry = SacredGeometrySystem::new();

        // Parse simple formulas to get atom count
        let atom_count = Self::count_atoms(formula);

        // Create standing wave
        let primary_freq = 432.0 * (atom_count as Float).sqrt();
        let secondary_freq = primary_freq * constants::GOLDEN_RATIO;
        let standing_wave = StandingWave::calculate(primary_freq, secondary_freq);

        // Create stability conditions
        let stability_conditions = StabilityConditions {
            min_coherence: 0.6,
            max_entropy: 0.4,
            archetype_alignment: 0.5,
            temperature_range: (200.0, 500.0),
            pressure_range: (0.5, 10.0),
            energy_threshold: atom_count as Float * 0.5,
            phi_alignment: constants::GOLDEN_RATIO_INVERSE,
        };

        // Create developmental stages (bond formation)
        let stages = vec![
            DevelopmentalStage::new(0, "Atom alignment", 0.5, atom_count as Float * 0.5),
            DevelopmentalStage::new(1, "Bond formation", 0.6, atom_count as Float * 0.8),
            DevelopmentalStage::new(2, "Molecule stabilization", 0.7, atom_count as Float * 1.0),
        ];

        // Pattern coefficients for molecule
        let pattern_coefficients: Vec<Float> = (0..22)
            .map(|i| {
                let base = (atom_count as Float + i as Float) / 30.0;
                (base * constants::GOLDEN_RATIO).min(1.0)
            })
            .collect();

        let name = format!("{} molecule template", formula);

        Self {
            name,
            template_type: TemplateType::Molecule(formula.to_string()),
            standing_wave,
            stability_conditions,
            stages,
            geometry,
            pattern_coefficients,
            template_energy: atom_count as Float * 50.0,
            created_at: 0,
        }
    }

    /// Create a morphogenetic template for a cell
    pub fn for_cell(cell_type: CellType) -> Self {
        let geometry = SacredGeometrySystem::new();

        // Create standing wave based on cell complexity
        let primary_freq = cell_type.base_frequency();
        let secondary_freq = primary_freq * constants::GOLDEN_RATIO;
        let standing_wave = StandingWave::calculate(primary_freq, secondary_freq);

        // Create stability conditions
        let stability_conditions = StabilityConditions::for_cell(cell_type);

        // Create developmental stages
        let stages = match cell_type {
            CellType::Prokaryote => vec![
                DevelopmentalStage::new(0, "Membrane formation", 0.5, 1.0),
                DevelopmentalStage::new(1, "DNA organization", 0.6, 1.5),
            ],
            CellType::Eukaryote => vec![
                DevelopmentalStage::new(0, "Membrane formation", 0.5, 2.0),
                DevelopmentalStage::new(1, "Nucleus formation", 0.6, 3.0),
                DevelopmentalStage::new(2, "Organelle emergence", 0.7, 4.0),
                DevelopmentalStage::new(3, "Cell specialization", 0.8, 5.0),
            ],
            _ => vec![
                DevelopmentalStage::new(0, "Membrane formation", 0.5, 2.0),
                DevelopmentalStage::new(1, "Nucleus formation", 0.6, 3.0),
                DevelopmentalStage::new(2, "Organelle emergence", 0.7, 4.0),
                DevelopmentalStage::new(3, "Specialization", 0.8, 5.0),
            ],
        };

        // Pattern coefficients for cell
        let complexity = cell_type.complexity() as Float;
        let pattern_coefficients: Vec<Float> = (0..22)
            .map(|i| {
                let base = complexity / 10.0 + i as Float * 0.02;
                (base * constants::GOLDEN_RATIO).min(1.0)
            })
            .collect();

        let name = format!("{:?} cell template", cell_type);

        Self {
            name,
            template_type: TemplateType::Cell(cell_type),
            standing_wave,
            stability_conditions,
            stages,
            geometry,
            pattern_coefficients,
            template_energy: complexity * 100.0,
            created_at: 0,
        }
    }

    /// Create a morphogenetic template for an organism
    pub fn for_organism(body_plan: BodyPlan) -> Self {
        let geometry = SacredGeometrySystem::new();

        // Create standing wave based on body plan complexity
        let complexity = body_plan.complexity() as Float;
        let primary_freq = 432.0 * complexity;
        let secondary_freq = primary_freq * constants::GOLDEN_RATIO;
        let standing_wave = StandingWave::calculate(primary_freq, secondary_freq);

        // Create stability conditions
        let stability_conditions = StabilityConditions::for_body_plan(body_plan);

        // Create developmental stages
        let num_stages = body_plan.developmental_stages();
        let stages: Vec<DevelopmentalStage> = (0..num_stages)
            .map(|i| {
                let names = match body_plan {
                    BodyPlan::Human => vec![
                        "Zygote",
                        "Blastula",
                        "Gastrula",
                        "Neurula",
                        "Embryo",
                        "Fetus",
                        "Infant",
                        "Child",
                        "Adolescent",
                        "Adult",
                        "Mature",
                        "Senior",
                        "Wisdom",
                        "Integration",
                    ],
                    _ => vec![
                        "Stage 1", "Stage 2", "Stage 3", "Stage 4", "Stage 5", "Stage 6",
                        "Stage 7", "Stage 8", "Stage 9", "Stage 10", "Stage 11", "Stage 12",
                        "Stage 13", "Stage 14",
                    ],
                };
                let name = names.get(i).unwrap_or(&"Stage").to_string();
                DevelopmentalStage::new(
                    i,
                    &name,
                    0.5 + i as Float * 0.03,
                    complexity * (i + 1) as Float * 10.0,
                )
            })
            .collect();

        // Pattern coefficients for organism
        let pattern_coefficients: Vec<Float> = (0..22)
            .map(|i| {
                let base = complexity / 10.0 + i as Float * 0.03;
                (base * constants::GOLDEN_RATIO).min(1.0)
            })
            .collect();

        let name = format!("{:?} organism template", body_plan);

        Self {
            name,
            template_type: TemplateType::Organism(body_plan),
            standing_wave,
            stability_conditions,
            stages,
            geometry,
            pattern_coefficients,
            template_energy: complexity * 1000.0,
            created_at: 0,
        }
    }

    /// Count atoms in a simple chemical formula
    fn count_atoms(formula: &str) -> usize {
        let mut count = 0;
        let mut current_num = String::new();
        let mut last_was_element = false;

        for c in formula.chars() {
            if c.is_ascii_uppercase() {
                // Process any number from previous element
                if last_was_element && !current_num.is_empty() {
                    count += current_num.parse::<usize>().unwrap_or(1) - 1; // -1 because we already counted 1
                    current_num.clear();
                }
                count += 1; // Count this element (at least 1)
                last_was_element = true;
            } else if c.is_ascii_digit() {
                current_num.push(c);
            } else if c.is_ascii_lowercase() {
                // Part of element symbol (e.g., "Na", "Ca")
                // Don't reset last_was_element
            } else {
                last_was_element = false;
            }
        }

        // Process trailing number
        if last_was_element && !current_num.is_empty() {
            count += current_num.parse::<usize>().unwrap_or(1) - 1;
        }

        count.max(1)
    }

    /// Check if the template is stable under given conditions
    pub fn is_stable(
        &self,
        coherence: Float,
        entropy: Float,
        temperature: Float,
        pressure: Float,
    ) -> bool {
        self.stability_conditions
            .is_stable(coherence, entropy, temperature, pressure)
    }

    /// Get current developmental stage
    pub fn get_stage(&self, stage: usize) -> Option<&DevelopmentalStage> {
        self.stages.get(stage)
    }

    /// Calculate coherence contribution from this template
    pub fn coherence_contribution(&self) -> Float {
        let wave_coherence = match self.standing_wave.interference_type {
            InterferenceType::Constructive => 0.9,
            InterferenceType::Partial => 0.5,
            InterferenceType::Destructive => 0.1,
        };

        let pattern_coherence: Float = self.pattern_coefficients.iter().sum::<Float>()
            / self.pattern_coefficients.len() as Float;

        (wave_coherence + pattern_coherence) / 2.0
    }

    /// Calculate golden ratio scaling for the template
    pub fn phi_scale(&self, level: i32) -> Float {
        self.geometry.phi_scale(self.template_energy, level)
    }

    /// Get archetype signature
    pub fn archetype_signature(&self) -> [Float; 22] {
        let mut signature = [0.5; 22];
        for (i, sig) in self.pattern_coefficients.iter().enumerate() {
            if i < 22 {
                signature[i] = *sig;
            }
        }
        signature
    }
}

/// Morphogenetic Template Library
///
/// A collection of templates organized by type.
#[derive(Debug, Clone)]
pub struct MorphogeneticTemplateLibrary {
    /// Atomic templates
    pub atoms: HashMap<ElementType, MorphogeneticTemplate>,

    /// Molecular templates
    pub molecules: HashMap<String, MorphogeneticTemplate>,

    /// Cellular templates
    pub cells: HashMap<CellType, MorphogeneticTemplate>,

    /// Organism templates
    pub organisms: HashMap<BodyPlan, MorphogeneticTemplate>,
}

impl MorphogeneticTemplateLibrary {
    /// Create a new template library
    pub fn new() -> Self {
        Self {
            atoms: HashMap::new(),
            molecules: HashMap::new(),
            cells: HashMap::new(),
            organisms: HashMap::new(),
        }
    }

    /// Create with common templates pre-loaded
    pub fn with_defaults() -> Self {
        let mut library = Self::new();

        // Add common elements
        for element in [
            ElementType::Hydrogen,
            ElementType::Helium,
            ElementType::Carbon,
            ElementType::Nitrogen,
            ElementType::Oxygen,
            ElementType::Silicon,
            ElementType::Iron,
        ] {
            library
                .atoms
                .insert(element, MorphogeneticTemplate::for_atom(element));
        }

        // Add common molecules
        for formula in ["H2O", "CO2", "O2", "N2", "CH4"] {
            library.molecules.insert(
                formula.to_string(),
                MorphogeneticTemplate::for_molecule(formula),
            );
        }

        // Add common cell types
        for cell_type in [
            CellType::Prokaryote,
            CellType::Eukaryote,
            CellType::Neuron,
            CellType::Muscle,
            CellType::Stem,
        ] {
            library
                .cells
                .insert(cell_type, MorphogeneticTemplate::for_cell(cell_type));
        }

        // Add body plans
        for body_plan in [
            BodyPlan::Unicellular,
            BodyPlan::BilateralSymmetry,
            BodyPlan::Vertebrate,
            BodyPlan::Human,
        ] {
            library
                .organisms
                .insert(body_plan, MorphogeneticTemplate::for_organism(body_plan));
        }

        library
    }

    /// Get a template by type
    pub fn get(&self, template_type: &TemplateType) -> Option<&MorphogeneticTemplate> {
        match template_type {
            TemplateType::Atom(element) => self.atoms.get(element),
            TemplateType::Molecule(formula) => self.molecules.get(formula),
            TemplateType::Cell(cell_type) => self.cells.get(cell_type),
            TemplateType::Organism(body_plan) => self.organisms.get(body_plan),
            TemplateType::Custom(_) => None,
        }
    }

    /// Add a template to the library
    pub fn add(&mut self, template: MorphogeneticTemplate) {
        match &template.template_type {
            TemplateType::Atom(element) => {
                self.atoms.insert(*element, template);
            }
            TemplateType::Molecule(formula) => {
                self.molecules.insert(formula.clone(), template);
            }
            TemplateType::Cell(cell_type) => {
                self.cells.insert(*cell_type, template);
            }
            TemplateType::Organism(body_plan) => {
                self.organisms.insert(*body_plan, template);
            }
            TemplateType::Custom(_) => {}
        }
    }

    /// Get total number of templates
    pub fn total_templates(&self) -> usize {
        self.atoms.len() + self.molecules.len() + self.cells.len() + self.organisms.len()
    }
}

impl Default for MorphogeneticTemplateLibrary {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_type_atomic_number() {
        assert_eq!(ElementType::Hydrogen.atomic_number(), 1);
        assert_eq!(ElementType::Carbon.atomic_number(), 6);
        assert_eq!(ElementType::Oxygen.atomic_number(), 8);
        assert_eq!(ElementType::Custom(42).atomic_number(), 42);
    }

    #[test]
    fn test_element_type_noble_gas() {
        assert!(ElementType::Helium.is_noble_gas());
        assert!(!ElementType::Hydrogen.is_noble_gas());
    }

    #[test]
    fn test_atom_template_creation() {
        let template = MorphogeneticTemplate::for_atom(ElementType::Carbon);

        assert!(template.name.contains("Carbon"));
        assert_eq!(
            template.template_type,
            TemplateType::Atom(ElementType::Carbon)
        );
        assert!(template.template_energy > 0.0);
        assert!(!template.stages.is_empty());
    }

    #[test]
    fn test_molecule_template_creation() {
        let template = MorphogeneticTemplate::for_molecule("H2O");

        assert!(template.name.contains("H2O"));
        assert_eq!(
            template.template_type,
            TemplateType::Molecule("H2O".to_string())
        );
        assert!(template.template_energy > 0.0);
    }

    #[test]
    fn test_cell_template_creation() {
        let template = MorphogeneticTemplate::for_cell(CellType::Neuron);

        assert!(template.name.contains("Neuron"));
        assert_eq!(template.template_type, TemplateType::Cell(CellType::Neuron));
        assert!(template.template_energy > 0.0);
    }

    #[test]
    fn test_organism_template_creation() {
        let template = MorphogeneticTemplate::for_organism(BodyPlan::Human);

        assert!(template.name.contains("Human"));
        assert_eq!(
            template.template_type,
            TemplateType::Organism(BodyPlan::Human)
        );
        assert!(template.stages.len() >= 10); // Human has many stages
    }

    #[test]
    fn test_stability_conditions_default() {
        let conditions = StabilityConditions::default();

        assert!(conditions.min_coherence >= 0.0 && conditions.min_coherence <= 1.0);
        assert!(conditions.max_entropy >= 0.0 && conditions.max_entropy <= 1.0);
    }

    #[test]
    fn test_stability_is_stable() {
        let conditions = StabilityConditions::default();

        // Should be stable under default conditions
        assert!(conditions.is_stable(0.6, 0.4, 500.0, 1.0));

        // Should not be stable with low coherence
        assert!(!conditions.is_stable(0.3, 0.4, 500.0, 1.0));

        // Should not be stable with high entropy
        assert!(!conditions.is_stable(0.6, 0.7, 500.0, 1.0));
    }

    #[test]
    fn test_coherence_contribution() {
        let template = MorphogeneticTemplate::for_atom(ElementType::Helium);

        let contribution = template.coherence_contribution();
        assert!((0.0..=1.0).contains(&contribution));
    }

    #[test]
    fn test_phi_scale() {
        let template = MorphogeneticTemplate::for_atom(ElementType::Carbon);

        let scaled = template.phi_scale(1);
        assert!(scaled > template.template_energy); // φ^1 > 1
    }

    #[test]
    fn test_archetype_signature() {
        let template = MorphogeneticTemplate::for_atom(ElementType::Oxygen);

        let signature = template.archetype_signature();
        assert_eq!(signature.len(), 22);

        // All values should be valid
        for &value in &signature {
            assert!((0.0..=1.0).contains(&value));
        }
    }

    #[test]
    fn test_count_atoms() {
        assert_eq!(MorphogeneticTemplate::count_atoms("H2O"), 3);
        assert_eq!(MorphogeneticTemplate::count_atoms("CO2"), 3);
        assert_eq!(MorphogeneticTemplate::count_atoms("C6H12O6"), 24);
        assert_eq!(MorphogeneticTemplate::count_atoms("O2"), 2);
    }

    #[test]
    fn test_template_library_creation() {
        let library = MorphogeneticTemplateLibrary::with_defaults();

        // Should have default templates
        assert!(library.atoms.contains_key(&ElementType::Hydrogen));
        assert!(library.molecules.contains_key("H2O"));
        assert!(library.cells.contains_key(&CellType::Prokaryote));
        assert!(library.organisms.contains_key(&BodyPlan::Human));
    }

    #[test]
    fn test_template_library_get() {
        let library = MorphogeneticTemplateLibrary::with_defaults();

        let template = library.get(&TemplateType::Atom(ElementType::Carbon));
        assert!(template.is_some());

        let missing = library.get(&TemplateType::Atom(ElementType::Custom(200)));
        assert!(missing.is_none());
    }

    #[test]
    fn test_template_library_add() {
        let mut library = MorphogeneticTemplateLibrary::new();

        let template = MorphogeneticTemplate::for_atom(ElementType::Gold);
        library.add(template);

        assert!(library.atoms.contains_key(&ElementType::Gold));
    }

    #[test]
    fn test_body_plan_complexity() {
        assert!(BodyPlan::Human.complexity() > BodyPlan::Unicellular.complexity());
        assert!(BodyPlan::Vertebrate.complexity() > BodyPlan::RadialSymmetry.complexity());
    }

    #[test]
    fn test_developmental_stage_creation() {
        let stage = DevelopmentalStage::new(0, "Test Stage", 0.5, 1.0);

        assert_eq!(stage.stage, 0);
        assert_eq!(stage.name, "Test Stage");
        assert!((stage.required_coherence - 0.5).abs() < 0.001);
        assert!(stage.duration > 0);
    }
}
