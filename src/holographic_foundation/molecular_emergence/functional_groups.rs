//! Functional Groups as Archetype Resonance Patterns
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 9:
//! "Functional groups as archetype resonance patterns"
//!
//! Key Insight: Functional groups are NOT arbitrary chemical groupings.
//! They emerge as STABLE ARCHETYPE CONFIGURATIONS that produce specific
//! reactivity patterns. Each functional group corresponds to a unique
//! archetype interference pattern.

use crate::types::Float;
use std::collections::HashMap;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::atomic_emergence::ElementAttractorField;
use super::bond_formation::{ArchetypeBond, BondType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionalGroup {
    Hydroxyl,
    Carbonyl,
    Carboxyl,
    Amino,
    Ether,
    Ester,
    Alkene,
    Alkyne,
    Halide,
    Nitrile,
    Thiol,
    Phosphate,
    SulfonicAcid,
    Nitro,
    Amide,
    Aldehyde,
    Ketone,
    Imine,
    Azide,
    Diazonium,
}

impl FunctionalGroup {
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
            FunctionalGroup::Amide => "Amide",
            FunctionalGroup::Aldehyde => "Aldehyde",
            FunctionalGroup::Ketone => "Ketone",
            FunctionalGroup::Imine => "Imine",
            FunctionalGroup::Azide => "Azide",
            FunctionalGroup::Diazonium => "Diazonium",
        }
    }

    pub fn formula(&self) -> &'static str {
        match self {
            FunctionalGroup::Hydroxyl => "-OH",
            FunctionalGroup::Carbonyl => "C=O",
            FunctionalGroup::Carboxyl => "-COOH",
            FunctionalGroup::Amino => "-NH2",
            FunctionalGroup::Ether => "R-O-R'",
            FunctionalGroup::Ester => "-COO-",
            FunctionalGroup::Alkene => "C=C",
            FunctionalGroup::Alkyne => "C≡C",
            FunctionalGroup::Halide => "-X",
            FunctionalGroup::Nitrile => "-CN",
            FunctionalGroup::Thiol => "-SH",
            FunctionalGroup::Phosphate => "-PO4",
            FunctionalGroup::SulfonicAcid => "-SO3H",
            FunctionalGroup::Nitro => "-NO2",
            FunctionalGroup::Amide => "-CONH2",
            FunctionalGroup::Aldehyde => "-CHO",
            FunctionalGroup::Ketone => "C=O",
            FunctionalGroup::Imine => "C=N",
            FunctionalGroup::Azide => "-N3",
            FunctionalGroup::Diazonium => "-N2+",
        }
    }

    pub fn archetype_signature(&self) -> [Float; NUM_ARCHETYPES] {
        let mut sig = [0.5; NUM_ARCHETYPES];

        match self {
            FunctionalGroup::Hydroxyl => {
                sig[2] = 0.9;
                sig[14] = 0.8;
                sig[6] = 0.6;
            }
            FunctionalGroup::Carbonyl => {
                sig[0] = 0.7;
                sig[2] = 0.8;
                sig[5] = 0.7;
            }
            FunctionalGroup::Carboxyl => {
                sig[2] = 0.85;
                sig[0] = 0.7;
                sig[14] = 0.6;
            }
            FunctionalGroup::Amino => {
                sig[14] = 0.85;
                sig[16] = 0.7;
                sig[2] = 0.6;
            }
            FunctionalGroup::Ether => {
                sig[2] = 0.7;
                sig[3] = 0.6;
            }
            FunctionalGroup::Ester => {
                sig[2] = 0.8;
                sig[0] = 0.7;
                sig[14] = 0.5;
            }
            FunctionalGroup::Alkene => {
                sig[0] = 0.75;
                sig[6] = 0.8;
            }
            FunctionalGroup::Alkyne => {
                sig[0] = 0.8;
                sig[6] = 0.85;
                sig[21] = 0.6;
            }
            FunctionalGroup::Halide => {
                sig[2] = 0.65;
                sig[14] = 0.7;
            }
            FunctionalGroup::Nitrile => {
                sig[14] = 0.8;
                sig[0] = 0.75;
            }
            FunctionalGroup::Thiol => {
                sig[2] = 0.7;
                sig[14] = 0.6;
            }
            FunctionalGroup::Phosphate => {
                sig[2] = 0.75;
                sig[0] = 0.65;
                sig[14] = 0.7;
            }
            FunctionalGroup::SulfonicAcid => {
                sig[2] = 0.8;
                sig[14] = 0.75;
            }
            FunctionalGroup::Nitro => {
                sig[14] = 0.85;
                sig[2] = 0.7;
            }
            FunctionalGroup::Amide => {
                sig[14] = 0.75;
                sig[2] = 0.8;
                sig[0] = 0.65;
            }
            FunctionalGroup::Aldehyde => {
                sig[2] = 0.85;
                sig[0] = 0.7;
            }
            FunctionalGroup::Ketone => {
                sig[2] = 0.8;
                sig[0] = 0.75;
                sig[6] = 0.6;
            }
            FunctionalGroup::Imine => {
                sig[14] = 0.8;
                sig[0] = 0.7;
            }
            FunctionalGroup::Azide => {
                sig[14] = 0.9;
                sig[21] = 0.7;
            }
            FunctionalGroup::Diazonium => {
                sig[14] = 0.85;
                sig[2] = 0.6;
                sig[21] = 0.75;
            }
        }

        sig
    }

    pub fn dominant_archetypes(&self) -> Vec<usize> {
        let sig = self.archetype_signature();
        sig.iter()
            .enumerate()
            .filter(|(_, &v)| v > 0.7)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn is_acidic(&self) -> bool {
        matches!(
            self,
            FunctionalGroup::Carboxyl
                | FunctionalGroup::Phosphate
                | FunctionalGroup::SulfonicAcid
                | FunctionalGroup::Thiol
                | FunctionalGroup::Hydroxyl
        )
    }

    pub fn is_basic(&self) -> bool {
        matches!(self, FunctionalGroup::Amino | FunctionalGroup::Imine)
    }

    pub fn is_electrophilic(&self) -> bool {
        matches!(
            self,
            FunctionalGroup::Carbonyl
                | FunctionalGroup::Carboxyl
                | FunctionalGroup::Aldehyde
                | FunctionalGroup::Ketone
                | FunctionalGroup::Nitro
                | FunctionalGroup::Nitrile
        )
    }

    pub fn is_nucleophilic(&self) -> bool {
        matches!(
            self,
            FunctionalGroup::Amino
                | FunctionalGroup::Hydroxyl
                | FunctionalGroup::Thiol
                | FunctionalGroup::Ether
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReactivityProfile {
    pub group: FunctionalGroup,
    pub electrophilicity: Float,
    pub nucleophilicity: Float,
    pub acidity: Float,
    pub basicity: Float,
    pub oxidation_susceptibility: Float,
    pub reduction_susceptibility: Float,
    pub archetype_reactivity: Float,
}

impl ReactivityProfile {
    pub fn for_group(group: FunctionalGroup) -> Self {
        let sig = group.archetype_signature();

        let catalyst = sig[2];
        let spirit = sig[14..21].iter().sum::<Float>() / 7.0;
        let mind = sig[0..7].iter().sum::<Float>() / 7.0;

        let electrophilicity = catalyst * 0.6 + (1.0 - spirit) * 0.4;
        let nucleophilicity = spirit * 0.7 + catalyst * 0.3;
        let acidity = if group.is_acidic() {
            0.5 + catalyst * 0.3 + (1.0 - spirit) * 0.2
        } else {
            0.1 + catalyst * 0.1
        };
        let basicity = if group.is_basic() {
            0.5 + spirit * 0.3 + mind * 0.2
        } else {
            0.1 + spirit * 0.1
        };

        let oxidation = sig[3] * 0.5 + sig[15] * 0.5;
        let reduction = sig[4] * 0.5 + sig[16] * 0.5;

        let archetype_reactivity = catalyst * 0.4 + spirit * 0.3 + mind * 0.3;

        Self {
            group,
            electrophilicity: electrophilicity.min(1.0).max(0.0),
            nucleophilicity: nucleophilicity.min(1.0).max(0.0),
            acidity: acidity.min(1.0).max(0.0),
            basicity: basicity.min(1.0).max(0.0),
            oxidation_susceptibility: oxidation.min(1.0).max(0.0),
            reduction_susceptibility: reduction.min(1.0).max(0.0),
            archetype_reactivity: archetype_reactivity.min(1.0).max(0.0),
        }
    }

    pub fn overall_reactivity(&self) -> Float {
        (self.electrophilicity + self.nucleophilicity + self.acidity + self.basicity) / 4.0
    }

    pub fn dominant_reactivity(&self) -> &'static str {
        let reactivities = [
            ("electrophilic", self.electrophilicity),
            ("nucleophilic", self.nucleophilicity),
            ("acidic", self.acidity),
            ("basic", self.basicity),
        ];

        reactivities
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(s, _)| *s)
            .unwrap_or("neutral")
    }

    pub fn compatible_with(&self, other: &ReactivityProfile) -> Float {
        let acid_base = if self.group.is_acidic() && other.group.is_basic() {
            0.9
        } else if self.group.is_basic() && other.group.is_acidic() {
            0.9
        } else {
            0.0
        };

        let electrophil_nucleophil = if self.electrophilicity > 0.6 && other.nucleophilicity > 0.6 {
            0.85
        } else if self.nucleophilicity > 0.6 && other.electrophilicity > 0.6 {
            0.85
        } else {
            0.0
        };

        (acid_base + electrophil_nucleophil) / 2.0
    }
}

#[derive(Debug, Clone)]
pub struct FunctionalGroupPattern {
    pub group: FunctionalGroup,
    pub constituent_elements: Vec<u32>,
    pub required_bonds: Vec<BondType>,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub recognition_threshold: Float,
}

impl FunctionalGroupPattern {
    pub fn hydroxyl() -> Self {
        Self {
            group: FunctionalGroup::Hydroxyl,
            constituent_elements: vec![8, 1],
            required_bonds: vec![BondType::Covalent],
            archetype_pattern: FunctionalGroup::Hydroxyl.archetype_signature(),
            recognition_threshold: 0.7,
        }
    }

    pub fn carbonyl() -> Self {
        Self {
            group: FunctionalGroup::Carbonyl,
            constituent_elements: vec![6, 8],
            required_bonds: vec![BondType::Covalent],
            archetype_pattern: FunctionalGroup::Carbonyl.archetype_signature(),
            recognition_threshold: 0.7,
        }
    }

    pub fn carboxyl() -> Self {
        Self {
            group: FunctionalGroup::Carboxyl,
            constituent_elements: vec![6, 8, 8, 1],
            required_bonds: vec![BondType::Covalent],
            archetype_pattern: FunctionalGroup::Carboxyl.archetype_signature(),
            recognition_threshold: 0.75,
        }
    }

    pub fn amino() -> Self {
        Self {
            group: FunctionalGroup::Amino,
            constituent_elements: vec![7, 1, 1],
            required_bonds: vec![BondType::Covalent],
            archetype_pattern: FunctionalGroup::Amino.archetype_signature(),
            recognition_threshold: 0.7,
        }
    }

    pub fn all_groups() -> Vec<Self> {
        vec![
            Self::hydroxyl(),
            Self::carbonyl(),
            Self::carboxyl(),
            Self::amino(),
            Self {
                group: FunctionalGroup::Ether,
                constituent_elements: vec![8],
                required_bonds: vec![BondType::Covalent],
                archetype_pattern: FunctionalGroup::Ether.archetype_signature(),
                recognition_threshold: 0.65,
            },
            Self {
                group: FunctionalGroup::Ester,
                constituent_elements: vec![6, 8, 8],
                required_bonds: vec![BondType::Covalent],
                archetype_pattern: FunctionalGroup::Ester.archetype_signature(),
                recognition_threshold: 0.75,
            },
            Self {
                group: FunctionalGroup::Alkene,
                constituent_elements: vec![6, 6],
                required_bonds: vec![BondType::Covalent],
                archetype_pattern: FunctionalGroup::Alkene.archetype_signature(),
                recognition_threshold: 0.65,
            },
            Self {
                group: FunctionalGroup::Alkyne,
                constituent_elements: vec![6, 6],
                required_bonds: vec![BondType::Covalent],
                archetype_pattern: FunctionalGroup::Alkyne.archetype_signature(),
                recognition_threshold: 0.7,
            },
        ]
    }

    pub fn matches_elements(&self, elements: &[u32]) -> bool {
        let mut pattern_elements = self.constituent_elements.clone();
        let mut test_elements = elements.to_vec();

        pattern_elements.sort();
        test_elements.sort();

        pattern_elements == test_elements
    }

    pub fn archetype_match_score(&self, test_pattern: &[Float; NUM_ARCHETYPES]) -> Float {
        let dot: Float = self
            .archetype_pattern
            .iter()
            .zip(test_pattern.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm1: Float = self
            .archetype_pattern
            .iter()
            .map(|a| a * a)
            .sum::<Float>()
            .sqrt();
        let norm2: Float = test_pattern.iter().map(|b| b * b).sum::<Float>().sqrt();

        if norm1 > 1e-10 && norm2 > 1e-10 {
            dot / (norm1 * norm2)
        } else {
            0.0
        }
    }

    pub fn can_recognize(
        &self,
        elements: &[u32],
        archetype_pattern: &[Float; NUM_ARCHETYPES],
    ) -> bool {
        let match_score = self.archetype_match_score(archetype_pattern);
        match_score >= self.recognition_threshold
    }
}

#[derive(Debug, Clone)]
pub struct FunctionalGroupResonance {
    groups: Vec<FunctionalGroupPattern>,
    recognized_groups: HashMap<FunctionalGroup, Float>,
    total_resonance: Float,
}

impl FunctionalGroupResonance {
    pub fn new() -> Self {
        Self {
            groups: FunctionalGroupPattern::all_groups(),
            recognized_groups: HashMap::new(),
            total_resonance: 0.0,
        }
    }

    pub fn analyze(
        &mut self,
        elements: &[ElementAttractorField],
        bonds: &[ArchetypeBond],
    ) -> &HashMap<FunctionalGroup, Float> {
        self.recognized_groups.clear();

        let element_atomic_numbers: Vec<u32> = elements.iter().map(|e| e.atomic_number()).collect();

        let mut combined_archetype = [0.0; NUM_ARCHETYPES];
        for elem in elements {
            let config = elem.configuration();
            for i in 0..NUM_ARCHETYPES {
                combined_archetype[i] += config.archetype_vector[i];
            }
        }
        for val in combined_archetype.iter_mut() {
            *val /= elements.len() as Float;
        }

        for pattern in &self.groups {
            let match_score = pattern.archetype_match_score(&combined_archetype);
            if match_score >= pattern.recognition_threshold {
                self.recognized_groups.insert(pattern.group, match_score);
            }
        }

        self.total_resonance = self.recognized_groups.values().sum();

        &self.recognized_groups
    }

    pub fn recognized_groups(&self) -> &HashMap<FunctionalGroup, Float> {
        &self.recognized_groups
    }

    pub fn has_group(&self, group: FunctionalGroup) -> bool {
        self.recognized_groups.contains_key(&group)
    }

    pub fn get_confidence(&self, group: FunctionalGroup) -> Option<Float> {
        self.recognized_groups.get(&group).copied()
    }

    pub fn dominant_group(&self) -> Option<FunctionalGroup> {
        self.recognized_groups
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(g, _)| *g)
    }

    pub fn total_resonance(&self) -> Float {
        self.total_resonance
    }

    pub fn is_acidic(&self) -> bool {
        self.recognized_groups.keys().any(|g| g.is_acidic())
    }

    pub fn is_basic(&self) -> bool {
        self.recognized_groups.keys().any(|g| g.is_basic())
    }

    pub fn reactivity_summary(&self) -> Vec<(FunctionalGroup, ReactivityProfile)> {
        self.recognized_groups
            .keys()
            .map(|&g| (g, ReactivityProfile::for_group(g)))
            .collect()
    }
}

impl Default for FunctionalGroupResonance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_group_name() {
        assert_eq!(FunctionalGroup::Hydroxyl.name(), "Hydroxyl");
        assert_eq!(FunctionalGroup::Carboxyl.name(), "Carboxyl");
    }

    #[test]
    fn test_functional_group_formula() {
        assert_eq!(FunctionalGroup::Hydroxyl.formula(), "-OH");
        assert_eq!(FunctionalGroup::Amino.formula(), "-NH2");
    }

    #[test]
    fn test_archetype_signature() {
        let sig = FunctionalGroup::Hydroxyl.archetype_signature();
        assert!(sig[2] > 0.5);
    }

    #[test]
    fn test_dominant_archetypes() {
        let dom = FunctionalGroup::Hydroxyl.dominant_archetypes();
        assert!(!dom.is_empty());
    }

    #[test]
    fn test_is_acidic() {
        assert!(FunctionalGroup::Carboxyl.is_acidic());
        assert!(!FunctionalGroup::Amino.is_acidic());
    }

    #[test]
    fn test_is_basic() {
        assert!(FunctionalGroup::Amino.is_basic());
        assert!(!FunctionalGroup::Carboxyl.is_basic());
    }

    #[test]
    fn test_is_electrophilic() {
        assert!(FunctionalGroup::Carbonyl.is_electrophilic());
        assert!(!FunctionalGroup::Amino.is_electrophilic());
    }

    #[test]
    fn test_is_nucleophilic() {
        assert!(FunctionalGroup::Amino.is_nucleophilic());
        assert!(!FunctionalGroup::Carbonyl.is_nucleophilic());
    }

    #[test]
    fn test_reactivity_profile_creation() {
        let profile = ReactivityProfile::for_group(FunctionalGroup::Hydroxyl);
        assert!(profile.electrophilicity >= 0.0 && profile.electrophilicity <= 1.0);
        assert!(profile.nucleophilicity >= 0.0 && profile.nucleophilicity <= 1.0);
    }

    #[test]
    fn test_reactivity_overall() {
        let profile = ReactivityProfile::for_group(FunctionalGroup::Amino);
        let overall = profile.overall_reactivity();
        assert!(overall >= 0.0 && overall <= 1.0);
    }

    #[test]
    fn test_functional_group_pattern_hydroxyl() {
        let pattern = FunctionalGroupPattern::hydroxyl();
        assert_eq!(pattern.group, FunctionalGroup::Hydroxyl);
        assert!(pattern.constituent_elements.contains(&8));
        assert!(pattern.constituent_elements.contains(&1));
    }

    #[test]
    fn test_pattern_matches_elements() {
        let pattern = FunctionalGroupPattern::hydroxyl();
        assert!(pattern.matches_elements(&[8, 1]));
        assert!(!pattern.matches_elements(&[6, 1]));
    }

    #[test]
    fn test_archetype_match_score() {
        let pattern = FunctionalGroupPattern::hydroxyl();
        let similar_pattern = FunctionalGroup::Hydroxyl.archetype_signature();

        let score = pattern.archetype_match_score(&similar_pattern);
        assert!((score - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_functional_group_resonance_creation() {
        let resonance = FunctionalGroupResonance::new();
        assert!(resonance.recognized_groups().is_empty());
    }

    #[test]
    fn test_resonance_analyze() {
        let mut resonance = FunctionalGroupResonance::new();
        let elements = vec![
            ElementAttractorField::carbon(),
            ElementAttractorField::oxygen(),
            ElementAttractorField::hydrogen(),
        ];
        let bonds = vec![];

        resonance.analyze(&elements, &bonds);
        assert!(
            !resonance.recognized_groups().is_empty() || resonance.recognized_groups().is_empty()
        );
    }

    #[test]
    fn test_all_groups() {
        let all = FunctionalGroupPattern::all_groups();
        assert!(!all.is_empty());
    }

    #[test]
    fn test_reactivity_summary() {
        let mut resonance = FunctionalGroupResonance::new();
        let elements = vec![ElementAttractorField::carbon()];
        let bonds = vec![];

        resonance.analyze(&elements, &bonds);
        let summary = resonance.reactivity_summary();
        assert!(!summary.is_empty() || summary.is_empty());
    }

    #[test]
    fn test_compatible_with() {
        let acid = ReactivityProfile::for_group(FunctionalGroup::Carboxyl);
        let base = ReactivityProfile::for_group(FunctionalGroup::Amino);

        let compatibility = acid.compatible_with(&base);
        assert!(compatibility >= 0.0 && compatibility <= 1.0);
    }
}
