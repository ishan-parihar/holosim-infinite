//! Molecular Field - V5 Phase 5 Biology Implementation
//!
//! This module implements molecular bonding from atomic attractor interactions.
//! Molecules emerge when atomic attractor fields interact and form stable
//! configurations based on their quantum properties.
//!
//! From V5 Phase 5 specifications:
//! - MolecularField manages atoms and computes bonding potentials
//! - Molecules form when bonding potentials exceed threshold
//! - BondType determined by electronegativity differences
//! - Molecular formula tracks element composition
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Matter IS consciousness at the densest resolution"
//! "Molecules are stable configurations of atomic attractor fields"

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::physics::matter_emergence::Atom;
use crate::physics::quantum_field::{Element, HolographicBlueprint};
use crate::types::{Density, Float};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// BOND TYPE
// ============================================================================

/// Types of chemical bonds between atoms
///
/// Determined by electronegativity differences between atoms
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BondType {
    /// Covalent bond - shared electrons (small EN difference)
    Covalent,

    /// Ionic bond - electron transfer (large EN difference)
    Ionic,

    /// Hydrogen bond - H bonded to N, O, or F
    Hydrogen,

    /// Van der Waals - weak attraction
    VanDerWaals,

    /// Metallic bond - electron sea
    Metallic,
}

impl BondType {
    /// Determine bond type from electronegativity difference
    pub fn from_electronegativity_diff(diff: Float) -> Self {
        if diff > 1.7 {
            BondType::Ionic
        } else if diff > 0.4 {
            BondType::Covalent
        } else {
            BondType::Covalent // Nonpolar covalent
        }
    }

    /// Get typical bond strength for this type
    pub fn typical_strength(&self) -> Float {
        match self {
            BondType::Covalent => 0.8,
            BondType::Ionic => 0.9,
            BondType::Hydrogen => 0.2,
            BondType::VanDerWaals => 0.05,
            BondType::Metallic => 0.7,
        }
    }
}

// ============================================================================
// BOND
// ============================================================================

/// A bond between two atoms in a molecule
#[derive(Clone, Debug)]
pub struct Bond {
    /// Index of first atom in molecule's atom list
    pub atom_a: usize,

    /// Index of second atom in molecule's atom list
    pub atom_b: usize,

    /// Type of bond
    pub bond_type: BondType,

    /// Bond strength (0.0 to 1.0)
    pub strength: Float,

    /// Bond length in spectrum distance
    pub length: Float,
}

impl Bond {
    /// Create a new bond
    pub fn new(atom_a: usize, atom_b: usize, bond_type: BondType, strength: Float) -> Self {
        Self {
            atom_a,
            atom_b,
            bond_type,
            strength,
            length: 1.0, // Default length
        }
    }
}

// ============================================================================
// MOLECULAR FORMULA
// ============================================================================

/// Molecular formula tracking element composition
#[derive(Clone, Debug)]
pub struct MolecularFormula {
    /// Map from element atomic number to count
    elements: HashMap<u32, u32>,
}

impl MolecularFormula {
    /// Create an empty formula
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    /// Create formula from atoms
    pub fn from_atoms(atoms: &[Atom]) -> Self {
        let mut elements = HashMap::new();
        for atom in atoms {
            *elements.entry(atom.element.atomic_number()).or_insert(0) += 1;
        }
        Self { elements }
    }

    /// Water: H2O
    pub fn water() -> Self {
        let mut elements = HashMap::new();
        elements.insert(1, 2); // Hydrogen
        elements.insert(8, 1); // Oxygen
        Self { elements }
    }

    /// Carbon dioxide: CO2
    pub fn carbon_dioxide() -> Self {
        let mut elements = HashMap::new();
        elements.insert(6, 1); // Carbon
        elements.insert(8, 2); // Oxygen
        Self { elements }
    }

    /// Methane: CH4
    pub fn methane() -> Self {
        let mut elements = HashMap::new();
        elements.insert(6, 1); // Carbon
        elements.insert(1, 4); // Hydrogen
        Self { elements }
    }

    /// Glucose: C6H12O6
    pub fn glucose() -> Self {
        let mut elements = HashMap::new();
        elements.insert(6, 6); // Carbon
        elements.insert(1, 12); // Hydrogen
        elements.insert(8, 6); // Oxygen
        Self { elements }
    }

    /// Get count for element by atomic number
    pub fn count(&self, atomic_number: u32) -> u32 {
        self.elements.get(&atomic_number).copied().unwrap_or(0)
    }

    /// Get total atom count
    pub fn total_atoms(&self) -> u32 {
        self.elements.values().sum()
    }

    /// Get element map
    pub fn elements(&self) -> &HashMap<u32, u32> {
        &self.elements
    }

    /// Format as string (simplified)
    pub fn to_string_simple(&self) -> String {
        let mut parts = Vec::new();

        // Standard order: C, H, then others by atomic number
        if let Some(&c) = self.elements.get(&6) {
            parts.push(format!(
                "C{}",
                if c > 1 { c.to_string() } else { "".to_string() }
            ));
        }
        if let Some(&h) = self.elements.get(&1) {
            parts.push(format!(
                "H{}",
                if h > 1 { h.to_string() } else { "".to_string() }
            ));
        }

        // Add other elements in atomic number order
        let mut others: Vec<_> = self
            .elements
            .iter()
            .filter(|(z, _)| **z != 1 && **z != 6)
            .collect();
        others.sort_by_key(|(z, _)| *z);

        for (z, &count) in others {
            let symbol = match z {
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
                26 => "Fe",
                29 => "Cu",
                30 => "Zn",
                _ => "??",
            };
            parts.push(format!(
                "{}{}",
                symbol,
                if count > 1 {
                    count.to_string()
                } else {
                    "".to_string()
                }
            ));
        }

        parts.join("")
    }

    /// Check equality with another formula
    pub fn matches(&self, other: &MolecularFormula) -> bool {
        self.elements == other.elements
    }
}

impl Default for MolecularFormula {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for MolecularFormula {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

// ============================================================================
// MOLECULE
// ============================================================================

/// A molecule - stable configuration of atoms
#[derive(Clone, Debug)]
pub struct Molecule {
    /// Unique identifier
    pub id: u64,

    /// Atoms in this molecule
    pub atoms: Vec<Atom>,

    /// Bonds between atoms
    pub bonds: Vec<Bond>,

    /// Molecular formula
    pub formula: MolecularFormula,

    /// Position on spectrum (center of mass)
    pub position: SpectrumPosition,

    /// Stability (0.0 to 1.0)
    pub stability: Float,

    /// Energy stored in bonds
    pub bond_energy: Float,

    /// Molecular mass (sum of atomic masses)
    pub mass: Float,

    /// Common name if known
    pub name: Option<String>,
}

impl Molecule {
    /// Create a new molecule
    pub fn new(id: u64, atoms: Vec<Atom>, bonds: Vec<Bond>, position: SpectrumPosition) -> Self {
        let formula = MolecularFormula::from_atoms(&atoms);
        let stability = Self::compute_stability(&bonds);
        let bond_energy = Self::compute_bond_energy(&bonds);
        let mass = atoms
            .iter()
            .map(|a| a.element.atomic_number() as Float)
            .sum();

        Self {
            id,
            atoms,
            bonds,
            formula,
            position,
            stability,
            bond_energy,
            mass,
            name: None,
        }
    }

    /// Compute overall stability from bonds
    fn compute_stability(bonds: &[Bond]) -> Float {
        if bonds.is_empty() {
            return 0.1; // Isolated atoms are unstable
        }

        let total_strength: Float = bonds.iter().map(|b| b.strength).sum();
        total_strength / bonds.len() as Float
    }

    /// Compute total bond energy
    fn compute_bond_energy(bonds: &[Bond]) -> Float {
        bonds
            .iter()
            .map(|b| b.strength * b.bond_type.typical_strength() * 100.0)
            .sum()
    }

    /// Check if this molecule matches a known formula
    pub fn matches_formula(&self, formula: &MolecularFormula) -> bool {
        self.formula.matches(formula)
    }

    /// Get center of mass position
    pub fn center_of_mass(&self) -> SpectrumPosition {
        if self.atoms.is_empty() {
            return self.position.clone();
        }

        let total_mass: Float = self
            .atoms
            .iter()
            .map(|a| a.element.atomic_number() as Float)
            .sum();

        if total_mass == 0.0 {
            return self.position.clone();
        }

        // Weighted average of positions (using velocity_ratio)
        let mut weighted_velocity = 0.0;
        for atom in &self.atoms {
            let weight = atom.element.atomic_number() as Float / total_mass;
            weighted_velocity += weight * atom.position.velocity_ratio;
        }

        SpectrumPosition::new(
            weighted_velocity,
            self.position.density,
            self.position.phase,
        )
    }
}

// ============================================================================
// MOLECULAR FIELD
// ============================================================================

/// Molecular Field - atoms combine to form molecules
///
/// Molecules emerge when atomic attractor fields interact
/// and form stable configurations.
pub struct MolecularField {
    /// Atoms in the field
    atoms: Vec<Atom>,

    /// Bonding potential matrix
    /// bonding_potential[i][j] = strength of potential bond between atom i and j
    bonding_potential: Vec<Vec<Float>>,

    /// Molecules formed
    molecules: Vec<Molecule>,

    /// Reference to holographic blueprint (for pattern matching)
    blueprint: Option<Arc<HolographicBlueprint>>,

    /// Next molecule ID
    next_molecule_id: u64,

    /// Bond threshold for formation
    bond_threshold: Float,

    /// Statistics
    stats: MolecularFieldStats,
}

/// Statistics for the molecular field
#[derive(Debug, Clone, Default)]
pub struct MolecularFieldStats {
    pub atoms_processed: u64,
    pub molecules_formed: u64,
    pub bonds_created: u64,
    pub average_molecule_size: Float,
    pub most_common_formula: Option<MolecularFormula>,
}

impl MolecularField {
    /// Create a new molecular field
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonding_potential: Vec::new(),
            molecules: Vec::new(),
            blueprint: None,
            next_molecule_id: 0,
            bond_threshold: 0.3,
            stats: MolecularFieldStats::default(),
        }
    }

    /// Create with blueprint reference
    pub fn with_blueprint(blueprint: Arc<HolographicBlueprint>) -> Self {
        Self {
            blueprint: Some(blueprint),
            ..Self::new()
        }
    }

    /// Set bond threshold
    pub fn set_bond_threshold(&mut self, threshold: Float) {
        self.bond_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Add atoms to the field
    pub fn add_atoms(&mut self, atoms: Vec<Atom>) {
        let start_idx = self.atoms.len();
        self.atoms.extend(atoms);
        self.stats.atoms_processed += start_idx as u64;

        // Expand bonding potential matrix
        let new_count = self.atoms.len();
        for row in &mut self.bonding_potential {
            row.resize(new_count, 0.0);
        }
        for _ in start_idx..new_count {
            self.bonding_potential.push(vec![0.0; new_count]);
        }

        // Compute bonding potentials for new atoms
        self.compute_bonding_potentials(start_idx);
    }

    /// Compute bonding potentials between atoms
    fn compute_bonding_potentials(&mut self, start_idx: usize) {
        for i in start_idx..self.atoms.len() {
            for j in 0..self.atoms.len() {
                if i != j {
                    let potential = self.compute_bond_potential(i, j);
                    self.bonding_potential[i][j] = potential;
                }
            }
        }
    }

    /// Compute bond potential between two atoms
    fn compute_bond_potential(&self, i: usize, j: usize) -> Float {
        let atom_a = &self.atoms[i];
        let atom_b = &self.atoms[j];

        // Distance on spectrum (affects bonding probability)
        let distance = atom_a.position.distance_to(&atom_b.position);
        let distance_factor = (-distance * 2.0).exp();

        // Electronegativity difference (determines bond strength/type)
        let en_a = Self::electronegativity(&atom_a.element);
        let en_b = Self::electronegativity(&atom_b.element);
        let en_diff = (en_a - en_b).abs();

        // Valence compatibility
        let val_a = Self::valence(&atom_a.element);
        let val_b = Self::valence(&atom_b.element);
        let valence_compat = if val_a > 0 && val_b > 0 { 1.0 } else { 0.5 };

        // Stability of both atoms
        let stability_factor = (atom_a.stability + atom_b.stability) / 2.0;

        // Combined potential
        distance_factor * (1.0 - en_diff * 0.1) * valence_compat * stability_factor
    }

    /// Get electronegativity for element (Pauling scale)
    fn electronegativity(element: &Element) -> Float {
        match element {
            Element::Hydrogen => 2.20,
            Element::Helium => 0.0, // Noble gas, doesn't bond
            Element::Lithium => 0.98,
            Element::Custom { protons } => {
                match *protons {
                    4 => 1.57,  // Be
                    5 => 2.04,  // B
                    6 => 2.55,  // C
                    7 => 3.04,  // N
                    8 => 3.44,  // O
                    9 => 3.98,  // F
                    10 => 0.0,  // Ne
                    11 => 0.93, // Na
                    12 => 1.31, // Mg
                    13 => 1.61, // Al
                    14 => 1.90, // Si
                    15 => 2.19, // P
                    16 => 2.58, // S
                    17 => 3.16, // Cl
                    18 => 0.0,  // Ar
                    19 => 0.82, // K
                    20 => 1.00, // Ca
                    26 => 1.83, // Fe
                    29 => 1.90, // Cu
                    30 => 1.65, // Zn
                    _ => 2.0,   // Default
                }
            }
        }
    }

    /// Get valence electrons
    fn valence(element: &Element) -> u32 {
        match element {
            Element::Hydrogen => 1,
            Element::Helium => 0, // Doesn't bond
            Element::Lithium => 1,
            Element::Custom { protons } => {
                // Simplified valence based on group
                let protons = *protons;
                let group = if protons <= 2 {
                    protons
                } else if protons <= 18 {
                    let adjusted = protons - 2;
                    if adjusted <= 8 {
                        adjusted
                    } else {
                        adjusted - 8
                    }
                } else {
                    let adjusted = protons - 18;
                    if adjusted <= 2 {
                        adjusted
                    } else {
                        (adjusted - 2) % 18
                    }
                };

                match group {
                    1 | 2 => group as u32,
                    13 => 3,
                    14 => 4,
                    15 => 5,
                    16 => 6,
                    17 => 7,
                    18 => 0, // Noble gas
                    _ => (group % 8) as u32,
                }
            }
        }
    }

    /// Form molecules from atoms
    pub fn form_molecules(&mut self) -> Vec<Molecule> {
        let mut new_molecules = Vec::new();
        let mut used = vec![false; self.atoms.len()];

        for i in 0..self.atoms.len() {
            if used[i] || self.atoms[i].element == Element::Helium {
                continue; // Skip noble gases
            }

            // Start a new potential molecule
            let mut molecule_atoms = vec![i];
            let mut bonds = Vec::new();
            used[i] = true;

            // Find atoms to bond with
            for j in (i + 1)..self.atoms.len() {
                if used[j] {
                    continue;
                }

                // Check bonding potential
                if self.bonding_potential[i][j] > self.bond_threshold {
                    molecule_atoms.push(j);

                    let en_i = Self::electronegativity(&self.atoms[i].element);
                    let en_j = Self::electronegativity(&self.atoms[j].element);
                    let bond_type = BondType::from_electronegativity_diff((en_i - en_j).abs());

                    bonds.push(Bond::new(
                        0,
                        molecule_atoms.len() - 1,
                        bond_type,
                        self.bonding_potential[i][j],
                    ));
                    used[j] = true;
                }
            }

            // Only create molecule if we have multiple atoms
            if molecule_atoms.len() > 1 {
                let atoms: Vec<Atom> = molecule_atoms
                    .iter()
                    .map(|&idx| self.atoms[idx].clone())
                    .collect();

                let position = atoms[0].position.clone();
                let molecule = Molecule::new(self.next_molecule_id, atoms, bonds, position);
                self.next_molecule_id += 1;

                self.stats.bonds_created += molecule.bonds.len() as u64;
                new_molecules.push(molecule);
            }
        }

        self.stats.molecules_formed += new_molecules.len() as u64;

        // Update average molecule size
        if !new_molecules.is_empty() {
            let total_atoms: Float = new_molecules.iter().map(|m| m.atoms.len() as Float).sum();
            self.stats.average_molecule_size = total_atoms / new_molecules.len() as Float;
        }

        self.molecules.extend(new_molecules.clone());
        new_molecules
    }

    /// Check if molecule matches blueprint pattern
    pub fn matches_blueprint(&self, molecule: &Molecule) -> bool {
        if let Some(ref _blueprint) = self.blueprint {
            // Would check if blueprint contains this molecular pattern
            // For now, return true for stable molecules
            molecule.stability > 0.5
        } else {
            true
        }
    }

    /// Get all molecules
    pub fn molecules(&self) -> &[Molecule] {
        &self.molecules
    }

    /// Get all atoms (unbonded)
    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    /// Get statistics
    pub fn stats(&self) -> &MolecularFieldStats {
        &self.stats
    }

    /// Clear the field
    pub fn clear(&mut self) {
        self.atoms.clear();
        self.bonding_potential.clear();
        self.molecules.clear();
    }

    /// Tick - evolve molecular field
    pub fn tick(&mut self, dt: Float) {
        // Apply thermal motion to atoms
        for atom in &mut self.atoms {
            // Random perturbation based on temperature
            let perturbation = (rand::random::<Float>() - 0.5) * dt * 0.1;
            atom.energy += perturbation;
        }

        // Recompute bonding potentials (dynamic)
        self.compute_bonding_potentials(0);

        // Check for molecule dissociation
        self.check_dissociation(dt);
    }

    /// Check for molecule dissociation
    fn check_dissociation(&mut self, dt: Float) {
        let mut to_remove = Vec::new();

        for (i, molecule) in self.molecules.iter_mut().enumerate() {
            // Probability of dissociation based on stability and energy
            let dissociation_prob = (1.0 - molecule.stability) * dt * 0.01;

            if rand::random::<Float>() < dissociation_prob {
                to_remove.push(i);
            }
        }

        // Remove dissociated molecules (in reverse order)
        for i in to_remove.into_iter().rev() {
            // Return atoms to the field
            let molecule = self.molecules.remove(i);
            self.atoms.extend(molecule.atoms);
        }
    }

    /// Find molecules matching a formula
    pub fn find_by_formula(&self, formula: &MolecularFormula) -> Vec<&Molecule> {
        self.molecules
            .iter()
            .filter(|m| m.formula.matches(formula))
            .collect()
    }

    /// Get count of molecules by formula
    pub fn count_formula(&self, formula: &MolecularFormula) -> usize {
        self.molecules
            .iter()
            .filter(|m| m.formula.matches(formula))
            .count()
    }
}

impl Default for MolecularField {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_atom(element: Element, id: u64) -> Atom {
        Atom {
            element,
            position: SpectrumPosition::new(1.0, Density::Third, 0.0),
            energy: 0.0,
            configuration: "1s1".to_string(),
            formation_coherence: 0.8,
            stability: 0.9,
            atom_id: id,
        }
    }

    #[test]
    fn test_bond_type_from_electronegativity() {
        // Small difference -> covalent
        assert_eq!(
            BondType::from_electronegativity_diff(0.2),
            BondType::Covalent
        );

        // Medium difference -> covalent (polar)
        assert_eq!(
            BondType::from_electronegativity_diff(0.8),
            BondType::Covalent
        );

        // Large difference -> ionic
        assert_eq!(BondType::from_electronegativity_diff(2.0), BondType::Ionic);
    }

    #[test]
    fn test_molecular_formula() {
        let h2o = MolecularFormula::water();
        assert_eq!(h2o.count(1), 2); // Hydrogen
        assert_eq!(h2o.count(8), 1); // Oxygen
        assert_eq!(h2o.total_atoms(), 3);
    }

    #[test]
    fn test_molecular_field_creation() {
        let field = MolecularField::new();
        assert!(field.atoms().is_empty());
        assert!(field.molecules().is_empty());
    }

    #[test]
    fn test_add_atoms() {
        let mut field = MolecularField::new();
        let atoms = vec![
            create_test_atom(Element::Hydrogen, 1),
            create_test_atom(Element::Hydrogen, 2),
        ];

        field.add_atoms(atoms);
        assert_eq!(field.atoms().len(), 2);
    }

    #[test]
    fn test_electronegativity_values() {
        // Hydrogen has known electronegativity
        let en_h = MolecularField::electronegativity(&Element::Hydrogen);
        assert!((en_h - 2.20).abs() < 0.01);

        // Oxygen has higher electronegativity
        let en_o = MolecularField::electronegativity(&Element::Custom { protons: 8 });
        assert!(en_o > en_h);
    }

    #[test]
    fn test_valence_values() {
        assert_eq!(MolecularField::valence(&Element::Hydrogen), 1);
        assert_eq!(MolecularField::valence(&Element::Helium), 0);
        assert_eq!(MolecularField::valence(&Element::Custom { protons: 6 }), 4); // Carbon
        assert_eq!(MolecularField::valence(&Element::Custom { protons: 8 }), 6);
        // Oxygen
    }

    #[test]
    fn test_molecule_creation() {
        let atoms = vec![
            create_test_atom(Element::Hydrogen, 1),
            create_test_atom(Element::Hydrogen, 2),
        ];

        let bond = Bond::new(0, 1, BondType::Covalent, 0.8);
        let molecule = Molecule::new(
            0,
            atoms,
            vec![bond],
            SpectrumPosition::new(1.0, Density::Third, 0.0),
        );

        assert_eq!(molecule.atoms.len(), 2);
        assert_eq!(molecule.bonds.len(), 1);
        assert!(molecule.stability > 0.0);
    }

    #[test]
    fn test_formula_matching() {
        let formula1 = MolecularFormula::water();
        let formula2 = MolecularFormula::water();
        let formula3 = MolecularFormula::carbon_dioxide();

        assert!(formula1.matches(&formula2));
        assert!(!formula1.matches(&formula3));
    }
}
