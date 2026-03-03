//! Chemical bonding from archetype patterns
//!
//! From ROADMAP: "Chemical bonding from archetype attraction"
//!
//! This module implements chemical bonding based on:
//! - Archetype attraction patterns between atoms
//! - Bond order, length, and energy from archetype compatibility
//! - Bond type classification (covalent, ionic, metallic, hydrogen)
//!
//! # Key Concepts
//!
//! - Bonds form between atoms with compatible archetype patterns
//! - Bond strength derived from archetype resonance
//! - Bond type determined by archetype pattern matching

use crate::chemistry::ElementAttractor;
use crate::types::Float;

/// Chemical bond types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondType {
    /// Covalent bond (shared electrons)
    Covalent,
    /// Ionic bond (electron transfer)
    Ionic,
    /// Metallic bond (delocalized electrons)
    Metallic,
    /// Hydrogen bond (dipole-dipole)
    HydrogenBond,
    /// Van der Waals interaction
    VanDerWaals,
}

impl BondType {
    /// Get typical bond energy in kJ/mol
    pub fn typical_energy(&self) -> Float {
        match self {
            BondType::Covalent => 350.0,
            BondType::Ionic => 600.0,
            BondType::Metallic => 200.0,
            BondType::HydrogenBond => 20.0,
            BondType::VanDerWaals => 5.0,
        }
    }

    /// Get typical bond length in Angstroms
    pub fn typical_length(&self) -> Float {
        match self {
            BondType::Covalent => 1.5,
            BondType::Ionic => 2.5,
            BondType::Metallic => 2.5,
            BondType::HydrogenBond => 2.8,
            BondType::VanDerWaals => 3.5,
        }
    }
}

/// Bond order (single, double, triple, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BondOrder {
    /// Single bond (1 electron pair)
    Single = 1,
    /// Double bond (2 electron pairs)
    Double = 2,
    /// Triple bond (3 electron pairs)
    Triple = 3,
    /// Aromatic bond (1.5 order)
    Aromatic = 4,
}

impl BondOrder {
    /// Get numeric bond order
    pub fn numeric(&self) -> Float {
        match self {
            BondOrder::Single => 1.0,
            BondOrder::Double => 2.0,
            BondOrder::Triple => 3.0,
            BondOrder::Aromatic => 1.5,
        }
    }
}

/// Chemical bond between two atoms
#[derive(Debug, Clone)]
pub struct ChemicalBond {
    /// Atom 1 index
    pub atom1: usize,

    /// Atom 2 index
    pub atom2: usize,

    /// Bond type
    pub bond_type: BondType,

    /// Bond order
    pub order: BondOrder,

    /// Bond length in Angstroms
    pub length: Float,

    /// Bond energy in kJ/mol
    pub energy: Float,

    /// Archetype compatibility score (0.0 to 1.0)
    pub compatibility: Float,
}

impl ChemicalBond {
    /// Create a new chemical bond
    pub fn new(atom1: usize, atom2: usize, bond_type: BondType, order: BondOrder) -> Self {
        Self {
            atom1,
            atom2,
            bond_type,
            order,
            length: bond_type.typical_length(),
            energy: bond_type.typical_energy() * order.numeric(),
            compatibility: 1.0,
        }
    }

    /// Create bond from archetype compatibility
    pub fn from_archetypes(
        atom1: usize,
        atom2: usize,
        elem1: &ElementAttractor,
        elem2: &ElementAttractor,
    ) -> Self {
        // Calculate archetype compatibility
        let compatibility = calculate_archetype_compatibility(
            &elem1.archetype_signature,
            &elem2.archetype_signature,
        );

        // Determine bond type from electronegativity difference
        let en_diff = (elem1.electronegativity - elem2.electronegativity).abs();
        let bond_type = if en_diff > 1.7 {
            BondType::Ionic
        } else if en_diff > 0.4 {
            BondType::Covalent
        } else {
            BondType::Covalent
        };

        // Determine bond order from compatibility
        let order = if compatibility > 0.8 {
            BondOrder::Triple
        } else if compatibility > 0.6 {
            BondOrder::Double
        } else {
            BondOrder::Single
        };

        // Calculate bond energy based on compatibility
        let base_energy = bond_type.typical_energy();
        let energy = base_energy * order.numeric() * compatibility;

        // Calculate bond length based on elements and order
        // Shorter bonds for higher order
        let base_length = bond_type.typical_length();
        let length = base_length / (order.numeric() * 0.3 + 0.7);

        Self {
            atom1,
            atom2,
            bond_type,
            order,
            length,
            energy,
            compatibility,
        }
    }
}

/// Calculate archetype compatibility between two signatures
///
/// Uses correlation coefficient to measure alignment between
/// two archetype activation patterns.
pub fn calculate_archetype_compatibility(sig1: &[Float; 22], sig2: &[Float; 22]) -> Float {
    let mean1: Float = sig1.iter().sum::<Float>() / 22.0;
    let mean2: Float = sig2.iter().sum::<Float>() / 22.0;

    // Check if both arrays are identical (exact match)
    if sig1 == sig2 {
        return 1.0;
    }

    let mut covariance = 0.0;
    let mut variance1 = 0.0;
    let mut variance2 = 0.0;

    for i in 0..22 {
        let diff1 = sig1[i] - mean1;
        let diff2 = sig2[i] - mean2;

        covariance += diff1 * diff2;
        variance1 += diff1 * diff1;
        variance2 += diff2 * diff2;
    }

    // Pearson correlation coefficient
    let denominator = (variance1 * variance2).sqrt();
    if denominator > 1e-10 {
        (covariance / denominator).abs()
    } else {
        // If both have near-zero variance, they're very similar
        1.0
    }
}

/// Predict bond type from element properties
pub fn predict_bond_type(elem1: &ElementAttractor, elem2: &ElementAttractor) -> BondType {
    let en_diff = (elem1.electronegativity - elem2.electronegativity).abs();

    // Electronegativity difference determines bond character
    if en_diff > 1.7 {
        BondType::Ionic
    } else if en_diff > 0.4 {
        // Polar covalent (classified as covalent)
        BondType::Covalent
    } else {
        // Nonpolar covalent
        BondType::Covalent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_type_energy() {
        assert!(BondType::Covalent.typical_energy() > BondType::HydrogenBond.typical_energy());
        assert!(BondType::Ionic.typical_energy() > BondType::Covalent.typical_energy());
    }

    #[test]
    fn test_bond_order_numeric() {
        assert_eq!(BondOrder::Single.numeric(), 1.0);
        assert_eq!(BondOrder::Double.numeric(), 2.0);
        assert_eq!(BondOrder::Triple.numeric(), 3.0);
        assert_eq!(BondOrder::Aromatic.numeric(), 1.5);
    }

    #[test]
    fn test_archetype_compatibility_identical() {
        let sig = [0.5; 22];
        let compat = calculate_archetype_compatibility(&sig, &sig);
        assert!(
            (compat - 1.0).abs() < 1e-10,
            "Identical signatures should have compatibility 1.0"
        );
    }

    #[test]
    fn test_archetype_compatibility_different() {
        let sig1 = [0.3; 22];
        let sig2 = [0.7; 22];
        let compat = calculate_archetype_compatibility(&sig1, &sig2);
        // Different constant values should still have high correlation
        assert!(compat > 0.9);
    }

    #[test]
    fn test_chemical_bond_creation() {
        let bond = ChemicalBond::new(0, 1, BondType::Covalent, BondOrder::Single);
        assert_eq!(bond.atom1, 0);
        assert_eq!(bond.atom2, 1);
        assert_eq!(bond.bond_type, BondType::Covalent);
        assert_eq!(bond.order, BondOrder::Single);
    }
}
