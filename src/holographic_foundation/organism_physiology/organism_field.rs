//! Organism as Unified Organ Field
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The organism is a UNIFIED FIELD of organs - not a collection of parts,
//!  but a coherent whole where each organ is a node in a single field."
//!
//! # Key Insight
//!
//! An organism is a unified field configuration:
//! - All organs resonate together as one field
//! - Body consciousness emerges from field coherence
//! - Health = field coherence across all organ systems
//! - Disease = field distortion that disrupts unity

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::Position3D;
use crate::holographic_foundation::organism_physiology::disease_healing::{
    DiseaseState, HealingSystem,
};
use crate::holographic_foundation::organism_physiology::organ_field::{
    Organ, OrganFieldNode, OrganHealth, OrganId, OrganState, OrganType, OrganVitality,
};
use crate::holographic_foundation::organism_physiology::organ_systems::{
    OrganSystemCoordinator, OrganSystemId, OrganSystemType,
};
use crate::holographic_foundation::organism_physiology::physiology_engine::{
    FieldWave, OrganCommunication, PhysiologyEngine, PhysiologySignal, SignalType,
};
use crate::holographic_foundation::organism_physiology::tissue_coherence::{
    Tissue, TissueCoherence, TissueId, TissueVitality,
};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganismId(pub u64);

impl OrganismId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganismState {
    Embryonic,
    Developing,
    Mature,
    Aging,
    Degenerating,
    Deceased,
}

impl OrganismState {
    pub fn from_vitality_and_age(vitality: Float, age_years: Float) -> Self {
        if vitality < 0.1 {
            OrganismState::Deceased
        } else if vitality < 0.3 {
            OrganismState::Degenerating
        } else if age_years < 0.5 {
            OrganismState::Embryonic
        } else if age_years < 20.0 {
            OrganismState::Developing
        } else if age_years < 60.0 && vitality > 0.7 {
            OrganismState::Mature
        } else {
            OrganismState::Aging
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrganismVitality {
    pub overall_coherence: Float,
    pub energy_level: Float,
    pub metabolic_balance: Float,
    pub immune_strength: Float,
    pub nervous_integration: Float,
    pub hormonal_balance: Float,
    pub structural_integrity: Float,
}

impl Default for OrganismVitality {
    fn default() -> Self {
        Self {
            overall_coherence: 0.9,
            energy_level: 0.85,
            metabolic_balance: 0.9,
            immune_strength: 0.85,
            nervous_integration: 0.9,
            hormonal_balance: 0.88,
            structural_integrity: 0.92,
        }
    }
}

impl OrganismVitality {
    pub fn overall_health(&self) -> Float {
        (self.overall_coherence
            + self.energy_level
            + self.metabolic_balance
            + self.immune_strength
            + self.nervous_integration
            + self.hormonal_balance
            + self.structural_integrity)
            / 7.0
    }

    pub fn is_critical(&self) -> bool {
        self.overall_health() < 0.3
    }

    pub fn weakest_system(&self) -> &'static str {
        let values = [
            ("coherence", self.overall_coherence),
            ("energy", self.energy_level),
            ("metabolic", self.metabolic_balance),
            ("immune", self.immune_strength),
            ("nervous", self.nervous_integration),
            ("hormonal", self.hormonal_balance),
            ("structural", self.structural_integrity),
        ];

        values
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|v| v.0)
            .unwrap_or("unknown")
    }
}

#[derive(Debug, Clone)]
pub struct BodyConsciousness {
    pub awareness_level: Float,
    pub self_recognition: Float,
    pub organ_awareness: HashMap<OrganId, Float>,
    pub system_integration: HashMap<OrganSystemType, Float>,
    pub homeostatic_drive: Float,
    pub healing_intelligence: Float,
}

impl BodyConsciousness {
    pub fn new() -> Self {
        Self {
            awareness_level: 0.5,
            self_recognition: 0.5,
            organ_awareness: HashMap::new(),
            system_integration: HashMap::new(),
            homeostatic_drive: 0.8,
            healing_intelligence: 0.7,
        }
    }

    pub fn update_organ_awareness(&mut self, organ_id: OrganId, coherence: Float) {
        self.organ_awareness.insert(organ_id, coherence);
    }

    pub fn update_system_integration(&mut self, system_type: OrganSystemType, coherence: Float) {
        self.system_integration.insert(system_type, coherence);
    }

    pub fn overall_consciousness(&self) -> Float {
        let base = (self.awareness_level + self.self_recognition) / 2.0;

        let organ_avg = if self.organ_awareness.is_empty() {
            0.5
        } else {
            self.organ_awareness.values().sum::<Float>() / self.organ_awareness.len() as Float
        };

        let system_avg = if self.system_integration.is_empty() {
            0.5
        } else {
            self.system_integration.values().sum::<Float>() / self.system_integration.len() as Float
        };

        (base * 0.4 + organ_avg * 0.3 + system_avg * 0.3) * self.homeostatic_drive
    }

    pub fn healing_capacity(&self) -> Float {
        self.healing_intelligence * self.overall_consciousness()
    }
}

impl Default for BodyConsciousness {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct OrganismField {
    pub id: OrganismId,
    pub state: OrganismState,
    pub age: Float,
    pub vitality: OrganismVitality,
    pub consciousness: BodyConsciousness,
    pub organs: HashMap<OrganId, Organ>,
    pub organ_systems: OrganSystemCoordinator,
    pub tissues: HashMap<TissueId, Tissue>,
    pub unified_field_pattern: [Float; NUM_ARCHETYPES],
    pub field_coherence: Float,
    pub resonance_frequency: Float,
}

impl OrganismField {
    pub fn new() -> Self {
        Self {
            id: OrganismId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            state: OrganismState::Embryonic,
            age: 0.0,
            vitality: OrganismVitality::default(),
            consciousness: BodyConsciousness::new(),
            organs: HashMap::new(),
            organ_systems: OrganSystemCoordinator::new(),
            tissues: HashMap::new(),
            unified_field_pattern: [0.5; NUM_ARCHETYPES],
            field_coherence: 0.9,
            resonance_frequency: 1.0,
        }
    }

    pub fn add_organ(&mut self, mut organ: Organ) {
        self.consciousness
            .update_organ_awareness(organ.id(), organ.coherence());
        self.organs.insert(organ.id(), organ);
        self.recalculate_unified_field();
    }

    pub fn remove_organ(&mut self, organ_id: &OrganId) -> Option<Organ> {
        let organ = self.organs.remove(organ_id);
        if organ.is_some() {
            self.consciousness.organ_awareness.remove(organ_id);
            self.recalculate_unified_field();
        }
        organ
    }

    pub fn add_tissue(&mut self, tissue: Tissue) {
        self.tissues.insert(tissue.id, tissue);
    }

    fn recalculate_unified_field(&mut self) {
        if self.organs.is_empty() {
            self.unified_field_pattern = [0.5; NUM_ARCHETYPES];
            return;
        }

        let mut pattern = [0.0; NUM_ARCHETYPES];
        let mut total_weight = 0.0;

        for organ in self.organs.values() {
            let weight = organ.mass() * organ.coherence();
            for i in 0..NUM_ARCHETYPES {
                pattern[i] += organ.node.archetype_pattern[i] * weight;
            }
            total_weight += weight;
        }

        if total_weight > 0.0 {
            for i in 0..NUM_ARCHETYPES {
                pattern[i] /= total_weight;
            }
        } else {
            pattern = [0.5; NUM_ARCHETYPES];
        }

        self.unified_field_pattern = pattern;
    }

    pub fn calculate_field_coherence(&self) -> Float {
        if self.organs.is_empty() {
            return 0.5;
        }

        let organ_coherence: Float =
            self.organs.values().map(|o| o.coherence()).sum::<Float>() / self.organs.len() as Float;

        let tissue_coherence: Float = if self.tissues.is_empty() {
            1.0
        } else {
            self.tissues
                .values()
                .map(|t| t.coherence.coherence())
                .sum::<Float>()
                / self.tissues.len() as Float
        };

        let system_coherence = self.organ_systems.overall_coherence();

        (organ_coherence * 0.4 + tissue_coherence * 0.3 + system_coherence * 0.3).clamp(0.0, 1.0)
    }

    pub fn update(&mut self, dt: Float) {
        self.age += dt;

        for organ in self.organs.values_mut() {
            organ.update(dt);
        }

        for tissue in self.tissues.values_mut() {
            tissue.update(dt);
        }

        self.organ_systems.update(dt);

        self.recalculate_unified_field();
        self.field_coherence = self.calculate_field_coherence();

        for organ in &self.organs {
            self.consciousness
                .update_organ_awareness(*organ.0, organ.1.coherence());
        }

        self.vitality.overall_coherence = self.field_coherence;
        self.vitality.energy_level = self.calculate_energy_level();
        self.vitality.metabolic_balance = self.calculate_metabolic_balance();
        self.vitality.immune_strength = self.calculate_immune_strength();
        self.vitality.nervous_integration = self.calculate_nervous_integration();
        self.vitality.hormonal_balance = self.calculate_hormonal_balance();
        self.vitality.structural_integrity = self.calculate_structural_integrity();

        self.state = OrganismState::from_vitality_and_age(self.vitality.overall_health(), self.age);

        self.consciousness.awareness_level = self.vitality.nervous_integration * 0.8 + 0.1;
        self.consciousness.homeostatic_drive = self.field_coherence * 0.9 + 0.1;
        self.consciousness.healing_intelligence =
            self.vitality.immune_strength * 0.7 + self.vitality.nervous_integration * 0.3;
    }

    fn calculate_energy_level(&self) -> Float {
        let heart_output = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Heart)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        let lung_output = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Lungs)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        (heart_output * 0.6 + lung_output * 0.4) * self.field_coherence
    }

    fn calculate_metabolic_balance(&self) -> Float {
        let liver = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Liver)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        let digestive = self
            .organs
            .values()
            .filter(|o| {
                o.organ_type() == OrganType::Stomach
                    || o.organ_type() == OrganType::Intestines
                    || o.organ_type() == OrganType::Pancreas
            })
            .map(|o| o.coherence())
            .sum::<Float>()
            / 3.0;

        (liver * 0.5 + digestive * 0.5) * self.field_coherence
    }

    fn calculate_immune_strength(&self) -> Float {
        let spleen = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Spleen)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        let lymph = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::LymphNodes)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        (spleen * 0.4 + lymph * 0.4 + self.field_coherence * 0.2)
    }

    fn calculate_nervous_integration(&self) -> Float {
        self.organs
            .values()
            .find(|o| o.organ_type() == OrganType::Brain)
            .map(|o| o.coherence())
            .unwrap_or(0.5)
    }

    fn calculate_hormonal_balance(&self) -> Float {
        let endocrine_organs: Vec<Float> = self
            .organs
            .values()
            .filter(|o| {
                o.organ_type() == OrganType::Thyroid
                    || o.organ_type() == OrganType::Adrenals
                    || o.organ_type() == OrganType::Pancreas
            })
            .map(|o| o.coherence())
            .collect();

        if endocrine_organs.is_empty() {
            0.5
        } else {
            endocrine_organs.iter().sum::<Float>() / endocrine_organs.len() as Float
        }
    }

    fn calculate_structural_integrity(&self) -> Float {
        let bones = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Bones)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        let muscles = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Muscles)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        let skin = self
            .organs
            .values()
            .find(|o| o.organ_type() == OrganType::Skin)
            .map(|o| o.coherence())
            .unwrap_or(0.5);

        (bones * 0.5 + muscles * 0.3 + skin * 0.2) * self.field_coherence
    }

    pub fn organ_count(&self) -> usize {
        self.organs.len()
    }

    pub fn tissue_count(&self) -> usize {
        self.tissues.len()
    }

    pub fn vital_organs_healthy(&self) -> bool {
        self.organs
            .values()
            .filter(|o| o.node.organ_type.vital())
            .all(|o| o.coherence() > 0.5)
    }

    pub fn apply_stress(&mut self, amount: Float, target_system: Option<OrganSystemType>) {
        match target_system {
            Some(system) => {
                for organ in self.organs.values_mut() {
                    if organ.node.organ_type.organ_system() == Self::system_to_str(&system) {
                        organ.health_mut().apply_stress(amount);
                    }
                }
            }
            None => {
                for organ in self.organs.values_mut() {
                    organ.health_mut().apply_stress(amount * 0.5);
                }
            }
        }
    }

    fn system_to_str(system: &OrganSystemType) -> &'static str {
        match system {
            OrganSystemType::Nervous => "nervous",
            OrganSystemType::Circulatory => "circulatory",
            OrganSystemType::Respiratory => "respiratory",
            OrganSystemType::Digestive => "digestive",
            OrganSystemType::Immune => "immune",
            OrganSystemType::Endocrine => "endocrine",
            OrganSystemType::Reproductive => "reproductive",
            OrganSystemType::Excretory => "excretory",
            OrganSystemType::Integumentary => "integumentary",
            OrganSystemType::Muscular => "muscular",
            OrganSystemType::Skeletal => "skeletal",
        }
    }

    pub fn heal(&mut self, amount: Float) {
        for organ in self.organs.values_mut() {
            organ.health_mut().heal(amount);
        }

        self.consciousness.healing_intelligence =
            (self.consciousness.healing_intelligence + amount * 0.1).min(1.0);
    }

    pub fn resonance_with(&self, other: &OrganismField) -> Float {
        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..NUM_ARCHETYPES {
            dot += self.unified_field_pattern[i] * other.unified_field_pattern[i];
            norm1 += self.unified_field_pattern[i] * self.unified_field_pattern[i];
            norm2 += other.unified_field_pattern[i] * other.unified_field_pattern[i];
        }

        if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1.sqrt() * norm2.sqrt())
        } else {
            0.0
        }
    }
}

impl Default for OrganismField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organism_id_creation() {
        let id = OrganismId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_organism_state_from_vitality() {
        assert_eq!(
            OrganismState::from_vitality_and_age(0.05, 30.0),
            OrganismState::Deceased
        );
        assert_eq!(
            OrganismState::from_vitality_and_age(0.2, 30.0),
            OrganismState::Degenerating
        );
        assert_eq!(
            OrganismState::from_vitality_and_age(0.8, 0.3),
            OrganismState::Embryonic
        );
        assert_eq!(
            OrganismState::from_vitality_and_age(0.8, 10.0),
            OrganismState::Developing
        );
        assert_eq!(
            OrganismState::from_vitality_and_age(0.8, 40.0),
            OrganismState::Mature
        );
        assert_eq!(
            OrganismState::from_vitality_and_age(0.5, 70.0),
            OrganismState::Aging
        );
    }

    #[test]
    fn test_organism_vitality_default() {
        let vit = OrganismVitality::default();
        assert!(vit.overall_health() > 0.8);
    }

    #[test]
    fn test_organism_vitality_weakest_system() {
        let mut vit = OrganismVitality::default();
        vit.immune_strength = 0.3;
        assert_eq!(vit.weakest_system(), "immune");
    }

    #[test]
    fn test_body_consciousness_new() {
        let bc = BodyConsciousness::new();
        assert!(bc.awareness_level > 0.0);
        assert!(bc.homeostatic_drive > 0.0);
    }

    #[test]
    fn test_body_consciousness_overall() {
        let bc = BodyConsciousness::new();
        assert!(bc.overall_consciousness() > 0.0);
    }

    #[test]
    fn test_body_consciousness_healing_capacity() {
        let bc = BodyConsciousness::new();
        assert!(bc.healing_capacity() > 0.0);
    }

    #[test]
    fn test_organism_field_new() {
        let org = OrganismField::new();
        assert!(org.organs.is_empty());
        assert!(org.field_coherence > 0.0);
    }

    #[test]
    fn test_organism_field_add_organ() {
        let mut org = OrganismField::new();
        let organ = Organ::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0), 0.3);
        let organ_id = organ.id();

        org.add_organ(organ);
        assert_eq!(org.organs.len(), 1);
        assert!(org.organs.contains_key(&organ_id));
        assert!(org.consciousness.organ_awareness.contains_key(&organ_id));
    }

    #[test]
    fn test_organism_field_remove_organ() {
        let mut org = OrganismField::new();
        let organ = Organ::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0), 0.3);
        let organ_id = organ.id();

        org.add_organ(organ);
        let removed = org.remove_organ(&organ_id);

        assert!(removed.is_some());
        assert!(org.organs.is_empty());
        assert!(!org.consciousness.organ_awareness.contains_key(&organ_id));
    }

    #[test]
    fn test_organism_field_update() {
        let mut org = OrganismField::new();
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));
        org.add_organ(Organ::new(
            OrganType::Brain,
            Position3D::new(0.0, 0.1, 0.0),
            1.4,
        ));

        let initial_age = org.age;
        org.update(1.0);

        assert!(org.age > initial_age);
        assert!(org.field_coherence > 0.0);
    }

    #[test]
    fn test_organism_field_vital_organs_healthy() {
        let mut org = OrganismField::new();
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));
        org.add_organ(Organ::new(
            OrganType::Brain,
            Position3D::new(0.0, 0.1, 0.0),
            1.4,
        ));

        assert!(org.vital_organs_healthy());
    }

    #[test]
    fn test_organism_field_apply_stress() {
        let mut org = OrganismField::new();
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));

        let initial_coherence = org.organs.values().next().unwrap().coherence();
        org.apply_stress(0.5, None);
        let after_stress = org.organs.values().next().unwrap().coherence();

        assert!(after_stress < initial_coherence);
    }

    #[test]
    fn test_organism_field_heal() {
        let mut org = OrganismField::new();
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));

        org.apply_stress(0.5, None);
        let stressed_coherence = org.organs.values().next().unwrap().coherence();

        org.heal(0.3);
        let healed_coherence = org.organs.values().next().unwrap().coherence();

        assert!(healed_coherence > stressed_coherence);
    }

    #[test]
    fn test_organism_field_resonance() {
        let mut org1 = OrganismField::new();
        let mut org2 = OrganismField::new();

        org1.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));
        org2.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));

        let resonance = org1.resonance_with(&org2);
        assert!(resonance >= 0.0 && resonance <= 1.0);
    }

    #[test]
    fn test_organism_field_recalculate_unified() {
        let mut org = OrganismField::new();
        org.add_organ(Organ::new(
            OrganType::Brain,
            Position3D::new(0.0, 0.0, 0.0),
            1.4,
        ));
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.1, 0.0),
            0.3,
        ));

        let pattern = &org.unified_field_pattern;
        let has_variation = pattern.iter().any(|&v| v != 0.5);
        assert!(has_variation);
    }

    #[test]
    fn test_organism_complete_body() {
        let mut org = OrganismField::new();

        org.add_organ(Organ::new(
            OrganType::Brain,
            Position3D::new(0.0, 0.5, 0.0),
            1.4,
        ));
        org.add_organ(Organ::new(
            OrganType::Heart,
            Position3D::new(0.0, 0.0, 0.0),
            0.3,
        ));
        org.add_organ(Organ::new(
            OrganType::Lungs,
            Position3D::new(0.0, 0.1, 0.0),
            0.5,
        ));
        org.add_organ(Organ::new(
            OrganType::Liver,
            Position3D::new(0.1, -0.1, 0.0),
            1.5,
        ));
        org.add_organ(Organ::new(
            OrganType::Kidneys,
            Position3D::new(0.0, -0.2, 0.0),
            0.15,
        ));
        org.add_organ(Organ::new(
            OrganType::Stomach,
            Position3D::new(-0.05, -0.1, 0.0),
            0.2,
        ));

        assert_eq!(org.organ_count(), 6);
        assert!(org.vital_organs_healthy());

        org.update(1.0);

        assert!(org.vitality.overall_health() > 0.5);
        assert!(org.consciousness.overall_consciousness() > 0.0);
    }
}
