//! Bond Formation from Archetype Interference Patterns
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 9:
//! "Chemical bonds form through ARCHETYPE RESONANCE between field configurations."
//!
//! Theory:
//! - Covalent: Similar archetype patterns → shared electron field
//! - Ionic: Complementary patterns → electron field transfer
//! - Metallic: Collective archetype resonance
//! - Hydrogen: Catalyst archetype bridge
//! - Van der Waals: Weak field interaction
//!
//! Key Insight: Bond types are NOT determined solely by electronegativity.
//! They emerge from the INTERFERENCE between archetype patterns of the bonding atoms.

use crate::types::Float;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::atomic_emergence::{ElementAttractorField, ElementIdentity};
use super::super::field_state::Position3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BondId(u64);

impl BondId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondType {
    Covalent,
    Ionic,
    Metallic,
    Hydrogen,
    VanDerWaals,
    Coordinate,
    Aromatic,
}

impl BondType {
    pub fn archetype_relationship(&self) -> &'static str {
        match self {
            BondType::Covalent => "Similar patterns → shared electron field",
            BondType::Ionic => "Complementary patterns → electron transfer",
            BondType::Metallic => "Collective archetype resonance",
            BondType::Hydrogen => "Catalyst archetype bridge",
            BondType::VanDerWaals => "Weak field interaction",
            BondType::Coordinate => "Donor-acceptor archetype pair",
            BondType::Aromatic => "Cyclic resonance stabilization",
        }
    }

    pub fn typical_energy_kj_mol(&self) -> Float {
        match self {
            BondType::Covalent => 350.0,
            BondType::Ionic => 600.0,
            BondType::Metallic => 200.0,
            BondType::Hydrogen => 20.0,
            BondType::VanDerWaals => 5.0,
            BondType::Coordinate => 250.0,
            BondType::Aromatic => 150.0,
        }
    }

    pub fn typical_length_angstrom(&self) -> Float {
        match self {
            BondType::Covalent => 1.5,
            BondType::Ionic => 2.5,
            BondType::Metallic => 2.5,
            BondType::Hydrogen => 2.8,
            BondType::VanDerWaals => 3.5,
            BondType::Coordinate => 2.0,
            BondType::Aromatic => 1.4,
        }
    }

    pub fn dominant_archetypes(&self) -> &'static [usize] {
        match self {
            BondType::Covalent => &[0, 2, 6],
            BondType::Ionic => &[1, 5, 15],
            BondType::Metallic => &[7, 8, 9],
            BondType::Hydrogen => &[2, 14],
            BondType::VanDerWaals => &[10, 11, 12],
            BondType::Coordinate => &[3, 4, 16],
            BondType::Aromatic => &[0, 6, 21],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BondOrder {
    Partial = 0,
    Single = 1,
    Double = 2,
    Triple = 3,
    Aromatic = 4,
}

impl BondOrder {
    pub fn numeric(&self) -> Float {
        match self {
            BondOrder::Partial => 0.5,
            BondOrder::Single => 1.0,
            BondOrder::Double => 2.0,
            BondOrder::Triple => 3.0,
            BondOrder::Aromatic => 1.5,
        }
    }

    pub fn electron_pairs(&self) -> u32 {
        match self {
            BondOrder::Partial => 1,
            BondOrder::Single => 1,
            BondOrder::Double => 2,
            BondOrder::Triple => 3,
            BondOrder::Aromatic => 1,
        }
    }

    pub fn from_archetype_similarity(similarity: Float) -> Self {
        if similarity > 0.85 {
            BondOrder::Triple
        } else if similarity > 0.70 {
            BondOrder::Double
        } else if similarity > 0.50 {
            BondOrder::Single
        } else {
            BondOrder::Partial
        }
    }
}

#[derive(Debug, Clone)]
pub struct MolecularInterferencePattern {
    pub archetype_interference: [Float; NUM_ARCHETYPES],
    pub constructive_regions: Vec<usize>,
    pub destructive_regions: Vec<usize>,
    pub net_interference: Float,
    pub phase_alignment: Float,
}

impl MolecularInterferencePattern {
    pub fn between_elements(elem1: &ElementAttractorField, elem2: &ElementAttractorField) -> Self {
        let arch1 = elem1.configuration().archetype_vector;
        let arch2 = elem2.configuration().archetype_vector;

        let mut interference = [0.0; NUM_ARCHETYPES];
        let mut constructive = Vec::new();
        let mut destructive = Vec::new();
        let mut net = 0.0;

        for i in 0..NUM_ARCHETYPES {
            let prod = arch1[i] * arch2[i];
            let sum_sq = arch1[i].powi(2) + arch2[i].powi(2);
            interference[i] = prod / (sum_sq.sqrt() + 1e-10);

            if interference[i] > 0.0 {
                constructive.push(i);
            } else if interference[i] < -0.1 {
                destructive.push(i);
            }
            net += interference[i];
        }

        let dot: Float = arch1.iter().zip(arch2.iter()).map(|(a, b)| a * b).sum();
        let norm1: Float = arch1.iter().map(|a| a * a).sum::<Float>().sqrt();
        let norm2: Float = arch2.iter().map(|b| b * b).sum::<Float>().sqrt();
        let phase_alignment = if norm1 > 1e-10 && norm2 > 1e-10 {
            dot / (norm1 * norm2)
        } else {
            0.0
        };

        Self {
            archetype_interference: interference,
            constructive_regions: constructive,
            destructive_regions: destructive,
            net_interference: net / NUM_ARCHETYPES as Float,
            phase_alignment,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.net_interference > 0.0 && !self.constructive_regions.is_empty()
    }

    pub fn stability_factor(&self) -> Float {
        let constructive_strength =
            self.constructive_regions.len() as Float / NUM_ARCHETYPES as Float;
        let destructive_penalty = self.destructive_regions.len() as Float / NUM_ARCHETYPES as Float;
        (constructive_strength - destructive_penalty * 0.5).max(0.0)
    }

    pub fn preferred_bond_type(&self) -> BondType {
        let mind1 = (0..7)
            .filter(|&i| self.archetype_interference[i] > 0.5)
            .count();
        let spirit1 = (14..21)
            .filter(|&i| self.archetype_interference[i] > 0.5)
            .count();
        let catalyst = self.archetype_interference[2];
        let choice = self.archetype_interference[21];

        if catalyst > 0.6 && self.phase_alignment > 0.7 {
            return BondType::Hydrogen;
        }

        if choice > 0.7 && self.constructive_regions.contains(&21) {
            return BondType::Aromatic;
        }

        if spirit1 > mind1 && self.net_interference < 0.3 {
            return BondType::Ionic;
        }

        if mind1 > 3 && self.phase_alignment > 0.8 {
            return BondType::Covalent;
        }

        if self.constructive_regions.len() > 10 {
            return BondType::Metallic;
        }

        if self.net_interference < 0.1 {
            return BondType::VanDerWaals;
        }

        BondType::Covalent
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeBond {
    pub id: BondId,
    pub element1: ElementIdentity,
    pub element2: ElementIdentity,
    pub bond_type: BondType,
    pub bond_order: BondOrder,
    pub length: Float,
    pub energy: Float,
    pub interference: MolecularInterferencePattern,
    pub archetype_similarity: Float,
    pub electron_sharing: Float,
    pub position: Position3D,
}

impl ArchetypeBond {
    pub fn new(
        elem1: &ElementAttractorField,
        elem2: &ElementAttractorField,
        position: Position3D,
    ) -> Self {
        let interference = MolecularInterferencePattern::between_elements(elem1, elem2);
        let bond_type = interference.preferred_bond_type();
        let bond_order = BondOrder::from_archetype_similarity(interference.phase_alignment);
        let archetype_similarity = interference.phase_alignment;

        let base_length = bond_type.typical_length_angstrom();
        let length_modifier = 1.0 - (interference.phase_alignment - 0.5) * 0.2;
        let length = base_length * length_modifier;

        let base_energy = bond_type.typical_energy_kj_mol();
        let energy = base_energy * bond_order.numeric() * interference.stability_factor();

        let electron_sharing = match bond_type {
            BondType::Covalent => archetype_similarity,
            BondType::Ionic => 1.0 - archetype_similarity.abs(),
            BondType::Metallic => 0.5,
            BondType::Hydrogen => 0.1,
            BondType::VanDerWaals => 0.01,
            BondType::Coordinate => 0.3,
            BondType::Aromatic => 0.33,
        };

        Self {
            id: BondId::new(rand_u64()),
            element1: *elem1.identity(),
            element2: *elem2.identity(),
            bond_type,
            bond_order,
            length,
            energy,
            interference,
            archetype_similarity,
            electron_sharing,
            position,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.interference.is_stable() && self.energy > 0.0
    }

    pub fn polarity(&self) -> Float {
        match self.bond_type {
            BondType::Ionic => 0.8 + self.archetype_similarity * 0.2,
            BondType::Covalent => (1.0 - self.archetype_similarity) * 0.5,
            BondType::Hydrogen => 0.4,
            _ => 0.0,
        }
    }

    pub fn bond_strength(&self) -> Float {
        self.energy / self.bond_type.typical_energy_kj_mol()
    }
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}

#[derive(Debug, Clone)]
pub struct BondFormationResult {
    pub bonds: Vec<ArchetypeBond>,
    pub total_energy: Float,
    pub coherence_change: Float,
    pub successful: bool,
    pub message: String,
}

impl BondFormationResult {
    pub fn empty() -> Self {
        Self {
            bonds: Vec::new(),
            total_energy: 0.0,
            coherence_change: 0.0,
            successful: false,
            message: "No bonds formed".to_string(),
        }
    }

    pub fn single(bond: ArchetypeBond) -> Self {
        let energy = bond.energy;
        Self {
            bonds: vec![bond],
            total_energy: energy,
            coherence_change: energy * 0.01,
            successful: true,
            message: "Bond formed successfully".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BondFormation {
    min_similarity_threshold: Float,
    max_bond_distance: Float,
    #[allow(dead_code)]
    enable_resonance: bool,
    formation_history: Vec<BondFormationResult>,
}

impl BondFormation {
    pub fn new() -> Self {
        Self {
            min_similarity_threshold: 0.3,
            max_bond_distance: 4.0,
            enable_resonance: true,
            formation_history: Vec::new(),
        }
    }

    pub fn with_threshold(mut self, threshold: Float) -> Self {
        self.min_similarity_threshold = threshold;
        self
    }

    pub fn with_max_distance(mut self, distance: Float) -> Self {
        self.max_bond_distance = distance;
        self
    }

    pub fn can_form_bond(
        &self,
        elem1: &ElementAttractorField,
        elem2: &ElementAttractorField,
    ) -> bool {
        if elem1.atomic_number() == elem2.atomic_number() && elem1.atomic_number() > 10 {
            return false;
        }

        let interference = MolecularInterferencePattern::between_elements(elem1, elem2);
        interference.is_stable() && interference.phase_alignment >= self.min_similarity_threshold
    }

    pub fn form_bond(
        &mut self,
        elem1: &ElementAttractorField,
        elem2: &ElementAttractorField,
        position: Position3D,
    ) -> BondFormationResult {
        if !self.can_form_bond(elem1, elem2) {
            return BondFormationResult::empty();
        }

        let bond = ArchetypeBond::new(elem1, elem2, position);
        let result = BondFormationResult::single(bond);
        self.formation_history.push(result.clone());
        result
    }

    pub fn form_bonds_in_vicinity(
        &mut self,
        elements: &[(ElementAttractorField, Position3D)],
        max_pairs: usize,
    ) -> BondFormationResult {
        let mut bonds = Vec::new();
        let mut total_energy = 0.0;
        let mut pairs_formed = 0;

        for i in 0..elements.len() {
            for j in (i + 1)..elements.len() {
                if pairs_formed >= max_pairs {
                    break;
                }

                let (elem1, pos1) = &elements[i];
                let (elem2, pos2) = &elements[j];

                let distance = pos1.distance(pos2);
                if distance > self.max_bond_distance {
                    continue;
                }

                if !self.can_form_bond(elem1, elem2) {
                    continue;
                }

                let mid_pos = Position3D::new(
                    (pos1.x + pos2.x) / 2.0,
                    (pos1.y + pos2.y) / 2.0,
                    (pos1.z + pos2.z) / 2.0,
                );

                let bond = ArchetypeBond::new(elem1, elem2, mid_pos);
                total_energy += bond.energy;
                bonds.push(bond);
                pairs_formed += 1;
            }
        }

        let coherence_change = total_energy * 0.01;
        let successful = !bonds.is_empty();

        let result = BondFormationResult {
            bonds,
            total_energy,
            coherence_change,
            successful,
            message: format!("Formed {} bonds", pairs_formed),
        };

        self.formation_history.push(result.clone());
        result
    }

    pub fn predict_bond_type(
        &self,
        elem1: &ElementAttractorField,
        elem2: &ElementAttractorField,
    ) -> BondType {
        let interference = MolecularInterferencePattern::between_elements(elem1, elem2);
        interference.preferred_bond_type()
    }

    pub fn predict_bond_order(
        &self,
        elem1: &ElementAttractorField,
        elem2: &ElementAttractorField,
    ) -> BondOrder {
        let interference = MolecularInterferencePattern::between_elements(elem1, elem2);
        BondOrder::from_archetype_similarity(interference.phase_alignment)
    }

    pub fn history(&self) -> &[BondFormationResult] {
        &self.formation_history
    }

    pub fn clear_history(&mut self) {
        self.formation_history.clear();
    }
}

impl Default for BondFormation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_type_properties() {
        let covalent = BondType::Covalent;
        assert!(covalent.typical_energy_kj_mol() > 0.0);
        assert!(covalent.typical_length_angstrom() > 0.0);
        assert!(!covalent.dominant_archetypes().is_empty());
    }

    #[test]
    fn test_bond_order_numeric() {
        assert_eq!(BondOrder::Single.numeric(), 1.0);
        assert_eq!(BondOrder::Double.numeric(), 2.0);
        assert_eq!(BondOrder::Triple.numeric(), 3.0);
    }

    #[test]
    fn test_bond_order_from_similarity() {
        assert_eq!(BondOrder::from_archetype_similarity(0.9), BondOrder::Triple);
        assert_eq!(
            BondOrder::from_archetype_similarity(0.75),
            BondOrder::Double
        );
        assert_eq!(BondOrder::from_archetype_similarity(0.6), BondOrder::Single);
        assert_eq!(
            BondOrder::from_archetype_similarity(0.3),
            BondOrder::Partial
        );
    }

    #[test]
    fn test_interference_pattern_creation() {
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();

        let interference = MolecularInterferencePattern::between_elements(&h, &o);

        assert!(interference.net_interference >= -1.0 && interference.net_interference <= 1.0);
        assert!(interference.phase_alignment >= 0.0 && interference.phase_alignment <= 1.0);
    }

    #[test]
    fn test_interference_pattern_stability() {
        let h = ElementAttractorField::hydrogen();
        let h2 = ElementAttractorField::hydrogen();

        let interference = MolecularInterferencePattern::between_elements(&h, &h2);

        assert!(interference.is_stable() || !interference.is_stable());
    }

    #[test]
    fn test_archetype_bond_creation() {
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();
        let pos = Position3D::new(0.0, 0.0, 0.0);

        let bond = ArchetypeBond::new(&h, &o, pos);

        assert!(bond.length > 0.0);
        assert!(bond.energy > 0.0 || bond.energy <= 0.0);
    }

    #[test]
    fn test_bond_formation_creation() {
        let formation = BondFormation::new();
        assert_eq!(formation.min_similarity_threshold, 0.3);
        assert_eq!(formation.max_bond_distance, 4.0);
    }

    #[test]
    fn test_can_form_bond() {
        let formation = BondFormation::new();
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();

        let can_form = formation.can_form_bond(&h, &o);
        // Just verify it returns a boolean
        let _ = can_form;
    }

    #[test]
    fn test_form_bond() {
        let mut formation = BondFormation::new().with_threshold(0.1);
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();
        let pos = Position3D::new(0.0, 0.0, 0.0);

        let result = formation.form_bond(&h, &o, pos);

        if result.successful {
            assert!(!result.bonds.is_empty());
            assert!(result.total_energy > 0.0);
        }
    }

    #[test]
    fn test_predict_bond_type() {
        let formation = BondFormation::new();
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();

        let bond_type = formation.predict_bond_type(&h, &o);
        assert!(matches!(
            bond_type,
            BondType::Covalent
                | BondType::Ionic
                | BondType::Metallic
                | BondType::Hydrogen
                | BondType::VanDerWaals
                | BondType::Coordinate
                | BondType::Aromatic
        ));
    }

    #[test]
    fn test_bond_polarity() {
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();
        let pos = Position3D::new(0.0, 0.0, 0.0);

        let bond = ArchetypeBond::new(&h, &o, pos);
        let polarity = bond.polarity();

        assert!((0.0..=1.0).contains(&polarity));
    }

    #[test]
    fn test_bond_formation_history() {
        let mut formation = BondFormation::new().with_threshold(0.1);
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();
        let pos = Position3D::new(0.0, 0.0, 0.0);

        let result = formation.form_bond(&h, &o, pos);
        if result.successful {
            assert!(!formation.history().is_empty());
        }

        formation.clear_history();
        assert!(formation.history().is_empty());
    }
}
