//! Organ System Implementations
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Organ Systems as Archetype Specializations - each system has dominant
//!  archetype patterns that determine its function and dynamics."
//!
//! # Key Insight
//!
//! Organ systems are specialized field configurations:
//! - Each system has a dominant archetype pattern
//! - System coherence = integration of organ fields
//! - System function = coordinated field dynamics

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::organism_physiology::organ_field::{Organ, OrganId, OrganType};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganSystemId(pub u64);

impl OrganSystemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganSystemType {
    Nervous,
    Circulatory,
    Respiratory,
    Digestive,
    Immune,
    Endocrine,
    Reproductive,
    Excretory,
    Integumentary,
    Muscular,
    Skeletal,
}

impl OrganSystemType {
    pub fn dominant_archetype(&self) -> usize {
        match self {
            OrganSystemType::Nervous => 0,
            OrganSystemType::Circulatory => 2,
            OrganSystemType::Respiratory => 6,
            OrganSystemType::Digestive => 4,
            OrganSystemType::Immune => 5,
            OrganSystemType::Endocrine => 2,
            OrganSystemType::Reproductive => 7,
            OrganSystemType::Excretory => 11,
            OrganSystemType::Integumentary => 8,
            OrganSystemType::Muscular => 9,
            OrganSystemType::Skeletal => 10,
        }
    }

    pub fn archetype_pattern(&self) -> [Float; NUM_ARCHETYPES] {
        let mut pattern = [0.5; NUM_ARCHETYPES];
        let dominant = self.dominant_archetype();

        pattern[dominant] = 0.95;

        match self {
            OrganSystemType::Nervous => {
                pattern[0] = 0.9;
                pattern[1] = 0.85;
                pattern[2] = 0.75;
                pattern[14] = 0.7;
            }
            OrganSystemType::Circulatory => {
                pattern[2] = 0.9;
                pattern[3] = 0.75;
                pattern[8] = 0.7;
            }
            OrganSystemType::Respiratory => {
                pattern[6] = 0.9;
                pattern[2] = 0.7;
            }
            OrganSystemType::Digestive => {
                pattern[4] = 0.9;
                pattern[3] = 0.8;
                pattern[10] = 0.7;
            }
            OrganSystemType::Immune => {
                pattern[5] = 0.9;
                pattern[11] = 0.8;
                pattern[0] = 0.65;
            }
            OrganSystemType::Endocrine => {
                pattern[2] = 0.85;
                pattern[1] = 0.8;
                pattern[14] = 0.75;
            }
            OrganSystemType::Reproductive => {
                pattern[7] = 0.9;
                pattern[6] = 0.8;
                pattern[14] = 0.7;
            }
            OrganSystemType::Excretory => {
                pattern[11] = 0.85;
                pattern[5] = 0.7;
            }
            OrganSystemType::Integumentary => {
                pattern[8] = 0.8;
                pattern[5] = 0.65;
            }
            OrganSystemType::Muscular => {
                pattern[9] = 0.9;
                pattern[2] = 0.75;
            }
            OrganSystemType::Skeletal => {
                pattern[10] = 0.9;
                pattern[8] = 0.7;
            }
        }

        for i in 14..21 {
            if pattern[i] == 0.5 {
                pattern[i] = pattern[i - 14] * 0.6;
            }
        }
        pattern[21] = 0.5;

        pattern
    }

    pub fn description(&self) -> &'static str {
        match self {
            OrganSystemType::Nervous => "Information processing and coordination",
            OrganSystemType::Circulatory => "Blood circulation and nutrient transport",
            OrganSystemType::Respiratory => "Gas exchange and breathing",
            OrganSystemType::Digestive => "Food processing and nutrient absorption",
            OrganSystemType::Immune => "Defense against pathogens",
            OrganSystemType::Endocrine => "Hormone production and regulation",
            OrganSystemType::Reproductive => "Reproduction and species continuation",
            OrganSystemType::Excretory => "Waste removal and filtration",
            OrganSystemType::Integumentary => "Protection and sensation",
            OrganSystemType::Muscular => "Movement and force generation",
            OrganSystemType::Skeletal => "Structural support and protection",
        }
    }

    pub fn organ_types(&self) -> Vec<OrganType> {
        match self {
            OrganSystemType::Nervous => vec![OrganType::Brain],
            OrganSystemType::Circulatory => vec![OrganType::Heart, OrganType::Blood],
            OrganSystemType::Respiratory => vec![OrganType::Lungs],
            OrganSystemType::Digestive => {
                vec![
                    OrganType::Stomach,
                    OrganType::Intestines,
                    OrganType::Liver,
                    OrganType::Pancreas,
                ]
            }
            OrganSystemType::Immune => vec![OrganType::Spleen, OrganType::LymphNodes],
            OrganSystemType::Endocrine => {
                vec![OrganType::Thyroid, OrganType::Adrenals, OrganType::Pancreas]
            }
            OrganSystemType::Reproductive => vec![OrganType::Gonads],
            OrganSystemType::Excretory => vec![OrganType::Kidneys, OrganType::Bladder],
            OrganSystemType::Integumentary => vec![OrganType::Skin],
            OrganSystemType::Muscular => vec![OrganType::Muscles],
            OrganSystemType::Skeletal => vec![OrganType::Bones],
        }
    }
}

#[derive(Debug, Clone)]
pub struct NervousSystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    signal_speed: Float,
    processing_capacity: Float,
}

impl NervousSystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            signal_speed: 100.0,
            processing_capacity: 1.0,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn processing_capacity(&self) -> Float {
        self.processing_capacity * self.coherence
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for NervousSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CirculatorySystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    blood_volume: Float,
    cardiac_output: Float,
    distribution_efficiency: Float,
}

impl CirculatorySystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            blood_volume: 5.0,
            cardiac_output: 5.0,
            distribution_efficiency: 0.85,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
        self.cardiac_output = 5.0 * self.coherence;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn cardiac_output(&self) -> Float {
        self.cardiac_output
    }

    pub fn distribution_efficiency(&self) -> Float {
        self.distribution_efficiency * self.coherence
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for CirculatorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct RespiratorySystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    oxygen_uptake_rate: Float,
    gas_exchange_efficiency: Float,
}

impl RespiratorySystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            oxygen_uptake_rate: 0.25,
            gas_exchange_efficiency: 0.9,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
        self.gas_exchange_efficiency = 0.9 * self.coherence;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn gas_exchange_efficiency(&self) -> Float {
        self.gas_exchange_efficiency
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for RespiratorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct DigestiveSystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    digestion_rate: Float,
    nutrient_absorption: Float,
}

impl DigestiveSystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            digestion_rate: 0.1,
            nutrient_absorption: 0.85,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
        self.nutrient_absorption = 0.85 * self.coherence;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn nutrient_absorption(&self) -> Float {
        self.nutrient_absorption
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for DigestiveSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ImmuneSystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    defense_strength: Float,
    response_time: Float,
}

impl ImmuneSystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            defense_strength: 0.85,
            response_time: 0.5,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
        self.defense_strength = 0.85 * self.coherence;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn defense_strength(&self) -> Float {
        self.defense_strength
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for ImmuneSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EndocrineSystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    hormone_balance: Float,
    regulation_accuracy: Float,
}

impl EndocrineSystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            hormone_balance: 0.85,
            regulation_accuracy: 0.9,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn hormone_balance(&self) -> Float {
        self.hormone_balance * self.coherence
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for EndocrineSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ReproductiveSystem {
    organs: HashMap<OrganId, Organ>,
    coherence: Float,
    fertility: Float,
    genetic_integrity: Float,
}

impl ReproductiveSystem {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            coherence: 0.9,
            fertility: 0.8,
            genetic_integrity: 0.95,
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        self.organs.insert(organ.id(), organ);
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        if self.organs.is_empty() {
            self.coherence = 0.9;
            return;
        }
        let total: Float = self.organs.values().map(|o| o.coherence()).sum();
        self.coherence = total / self.organs.len() as Float;
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn update(&mut self, dt: Float) {
        for organ in self.organs.values_mut() {
            organ.update(dt);
        }
        self.recalculate_coherence();
    }
}

impl Default for ReproductiveSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct OrganSystemCoordinator {
    nervous: NervousSystem,
    circulatory: CirculatorySystem,
    respiratory: RespiratorySystem,
    digestive: DigestiveSystem,
    immune: ImmuneSystem,
    endocrine: EndocrineSystem,
    reproductive: ReproductiveSystem,
}

impl OrganSystemCoordinator {
    pub fn new() -> Self {
        Self {
            nervous: NervousSystem::new(),
            circulatory: CirculatorySystem::new(),
            respiratory: RespiratorySystem::new(),
            digestive: DigestiveSystem::new(),
            immune: ImmuneSystem::new(),
            endocrine: EndocrineSystem::new(),
            reproductive: ReproductiveSystem::new(),
        }
    }

    pub fn add_organ(&mut self, organ: Organ) {
        match organ.organ_type() {
            OrganType::Brain => self.nervous.add_organ(organ),
            OrganType::Heart | OrganType::Blood => self.circulatory.add_organ(organ),
            OrganType::Lungs => self.respiratory.add_organ(organ),
            OrganType::Stomach | OrganType::Intestines | OrganType::Liver | OrganType::Pancreas => {
                self.digestive.add_organ(organ)
            }
            OrganType::Spleen | OrganType::LymphNodes => self.immune.add_organ(organ),
            OrganType::Thyroid | OrganType::Adrenals => self.endocrine.add_organ(organ),
            OrganType::Gonads => self.reproductive.add_organ(organ),
            OrganType::Kidneys | OrganType::Bladder => {}
            OrganType::Skin => {}
            OrganType::Muscles => {}
            OrganType::Bones => {}
        }
    }

    pub fn overall_coherence(&self) -> Float {
        let systems = [
            self.nervous.coherence(),
            self.circulatory.coherence(),
            self.respiratory.coherence(),
            self.digestive.coherence(),
            self.immune.coherence(),
            self.endocrine.coherence(),
            self.reproductive.coherence(),
        ];
        systems.iter().sum::<Float>() / systems.len() as Float
    }

    pub fn nervous(&self) -> &NervousSystem {
        &self.nervous
    }

    pub fn circulatory(&self) -> &CirculatorySystem {
        &self.circulatory
    }

    pub fn respiratory(&self) -> &RespiratorySystem {
        &self.respiratory
    }

    pub fn digestive(&self) -> &DigestiveSystem {
        &self.digestive
    }

    pub fn immune(&self) -> &ImmuneSystem {
        &self.immune
    }

    pub fn endocrine(&self) -> &EndocrineSystem {
        &self.endocrine
    }

    pub fn reproductive(&self) -> &ReproductiveSystem {
        &self.reproductive
    }

    pub fn update(&mut self, dt: Float) {
        self.nervous.update(dt);
        self.circulatory.update(dt);
        self.respiratory.update(dt);
        self.digestive.update(dt);
        self.immune.update(dt);
        self.endocrine.update(dt);
        self.reproductive.update(dt);
    }
}

impl Default for OrganSystemCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic_foundation::field_state::Position3D;

    #[test]
    fn test_organ_system_id_creation() {
        let id = OrganSystemId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_organ_system_type_dominant_archetype() {
        assert_eq!(OrganSystemType::Nervous.dominant_archetype(), 0);
        assert_eq!(OrganSystemType::Circulatory.dominant_archetype(), 2);
    }

    #[test]
    fn test_organ_system_type_archetype_pattern() {
        let pattern = OrganSystemType::Nervous.archetype_pattern();
        assert!(pattern[0] > 0.8);
    }

    #[test]
    fn test_nervous_system_creation() {
        let sys = NervousSystem::new();
        assert!(sys.coherence() > 0.8);
    }

    #[test]
    fn test_circulatory_system_creation() {
        let sys = CirculatorySystem::new();
        assert!(sys.coherence() > 0.8);
    }

    #[test]
    fn test_respiratory_system_creation() {
        let sys = RespiratorySystem::new();
        assert!(sys.coherence() > 0.8);
    }

    #[test]
    fn test_organ_system_coordinator_creation() {
        let coord = OrganSystemCoordinator::new();
        assert!(coord.overall_coherence() > 0.8);
    }

    #[test]
    fn test_organ_system_coordinator_add_organ() {
        let mut coord = OrganSystemCoordinator::new();
        let organ = Organ::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0), 0.3);
        coord.add_organ(organ);
        assert!(coord.circulatory().coherence() > 0.0);
    }

    #[test]
    fn test_organ_system_type_organ_types() {
        let types = OrganSystemType::Digestive.organ_types();
        assert!(types.contains(&OrganType::Stomach));
        assert!(types.contains(&OrganType::Liver));
    }

    #[test]
    fn test_immune_system_defense() {
        let sys = ImmuneSystem::new();
        assert!(sys.defense_strength() > 0.5);
    }

    #[test]
    fn test_endocrine_system_hormone_balance() {
        let sys = EndocrineSystem::new();
        assert!(sys.hormone_balance() > 0.5);
    }
}
