//! Functional groups from archetype patterns
//!
//! From ROADMAP: "Functional groups emerge from archetype resonance"
//!
//! This module implements functional group recognition and properties:
//! - Functional groups as archetype-derived molecular patterns
//! - Reactivity predictions from archetype compatibility
//! - Functional group effects on molecular properties

use crate::types::Float;

/// Functional group types in organic chemistry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionalGroup {
    /// Hydroxyl group (-OH)
    Hydroxyl,
    /// Carbonyl group (C=O)
    Carbonyl,
    /// Carboxyl group (-COOH)
    Carboxyl,
    /// Amino group (-NH2)
    Amino,
    /// Ether group (R-O-R')
    Ether,
    /// Ester group (-COO-)
    Ester,
    /// Alkene (C=C)
    Alkene,
    /// Alkyne (C≡C)
    Alkyne,
    /// Halide (-X where X = F, Cl, Br, I)
    Halide,
    /// Nitrile (-CN)
    Nitrile,
    /// Thiol (-SH)
    Thiol,
    /// Phosphate (-PO4)
    Phosphate,
    /// Sulfonic acid (-SO3H)
    SulfonicAcid,
    /// Nitro group (-NO2)
    Nitro,
}

impl FunctionalGroup {
    /// Get the name of this functional group
    pub fn name(&self) -> &'static str {
        match self {
            FunctionalGroup::Hydroxyl => "Hydroxyl",
            FunctionalGroup::Carbonyl => "Carbonyl",
            FunctionalGroup::Carboxyl => "Carboxyl",
            FunctionalGroup::Amino => "Amino",
            FunctionalGroup::Ether => "Ether",
            FunctionalGroup::Ester => "Ester",
            FunctionalGroup::Alkene => "Alkene",
            FunctionalGroup::Alkyne => "Alkyne",
            FunctionalGroup::Halide => "Halide",
            FunctionalGroup::Nitrile => "Nitrile",
            FunctionalGroup::Thiol => "Thiol",
            FunctionalGroup::Phosphate => "Phosphate",
            FunctionalGroup::SulfonicAcid => "Sulfonic acid",
            FunctionalGroup::Nitro => "Nitro",
        }
    }

    /// Get typical reactivity score (0.0 to 1.0)
    /// Higher = more reactive
    pub fn reactivity(&self) -> Float {
        match self {
            FunctionalGroup::Hydroxyl => 0.6,
            FunctionalGroup::Carbonyl => 0.7,
            FunctionalGroup::Carboxyl => 0.5,
            FunctionalGroup::Amino => 0.6,
            FunctionalGroup::Ether => 0.2,
            FunctionalGroup::Ester => 0.5,
            FunctionalGroup::Alkene => 0.7,
            FunctionalGroup::Alkyne => 0.6,
            FunctionalGroup::Halide => 0.8,
            FunctionalGroup::Nitrile => 0.4,
            FunctionalGroup::Thiol => 0.5,
            FunctionalGroup::Phosphate => 0.3,
            FunctionalGroup::SulfonicAcid => 0.4,
            FunctionalGroup::Nitro => 0.6,
        }
    }

    /// Check if this functional group is acidic
    pub fn is_acidic(&self) -> bool {
        matches!(
            self,
            FunctionalGroup::Carboxyl
                | FunctionalGroup::Phosphate
                | FunctionalGroup::SulfonicAcid
                | FunctionalGroup::Thiol
        )
    }

    /// Check if this functional group is basic
    pub fn is_basic(&self) -> bool {
        matches!(self, FunctionalGroup::Amino)
    }

    /// Check if this functional group can hydrogen bond
    pub fn can_hydrogen_bond(&self) -> bool {
        matches!(
            self,
            FunctionalGroup::Hydroxyl
                | FunctionalGroup::Carbonyl
                | FunctionalGroup::Carboxyl
                | FunctionalGroup::Amino
                | FunctionalGroup::Ether
                | FunctionalGroup::Ester
        )
    }
}

/// Functional group instance in a molecule
#[derive(Debug, Clone)]
pub struct FunctionalGroupInstance {
    /// The functional group type
    pub group: FunctionalGroup,

    /// Position in molecule (atom indices)
    pub atom_indices: Vec<usize>,

    /// Local reactivity modifier (from archetype influence)
    pub reactivity_modifier: Float,
}

impl FunctionalGroupInstance {
    /// Create a new functional group instance
    pub fn new(group: FunctionalGroup, atom_indices: Vec<usize>) -> Self {
        Self {
            group,
            atom_indices,
            reactivity_modifier: 1.0,
        }
    }

    /// Get effective reactivity
    pub fn effective_reactivity(&self) -> Float {
        self.group.reactivity() * self.reactivity_modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_group_names() {
        assert_eq!(FunctionalGroup::Hydroxyl.name(), "Hydroxyl");
        assert_eq!(FunctionalGroup::Carboxyl.name(), "Carboxyl");
    }

    #[test]
    fn test_functional_group_acidity() {
        assert!(FunctionalGroup::Carboxyl.is_acidic());
        assert!(!FunctionalGroup::Amino.is_acidic());
    }

    #[test]
    fn test_functional_group_basicity() {
        assert!(FunctionalGroup::Amino.is_basic());
        assert!(!FunctionalGroup::Carboxyl.is_basic());
    }

    #[test]
    fn test_hydrogen_bonding() {
        assert!(FunctionalGroup::Hydroxyl.can_hydrogen_bond());
        assert!(FunctionalGroup::Amino.can_hydrogen_bond());
        assert!(!FunctionalGroup::Alkene.can_hydrogen_bond());
    }

    #[test]
    fn test_reactivity() {
        // Halides are very reactive
        assert!(FunctionalGroup::Halide.reactivity() > 0.7);

        // Ethers are relatively unreactive
        assert!(FunctionalGroup::Ether.reactivity() < 0.3);
    }
}
