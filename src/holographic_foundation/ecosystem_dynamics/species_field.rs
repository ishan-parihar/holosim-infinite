//! Species as Field Configuration Patterns
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Species = specific field configuration pattern"
//!
//! # Key Insight
//!
//! A species is NOT a collection of individuals, but a stable field pattern
//! that manifests as individuals. The species field pattern determines:
//! - Morphology (physical form)
//! - Behavior (action patterns)
//! - Ecological niche (field resonance with environment)
//! - Evolutionary potential (field pattern plasticity)

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::Position3D;
use crate::holographic_foundation::organism_physiology::organism_field::OrganismField;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u64);

impl SpeciesId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesType {
    Producer,
    Herbivore,
    Carnivore,
    Omnivore,
    Decomposer,
    Parasite,
    Mutualist,
    Keystone,
}

impl SpeciesType {
    pub fn default_trophic_level(&self) -> usize {
        match self {
            SpeciesType::Producer => 0,
            SpeciesType::Herbivore => 1,
            SpeciesType::Omnivore => 2,
            SpeciesType::Carnivore => 2,
            SpeciesType::Decomposer => 3,
            SpeciesType::Parasite => 2,
            SpeciesType::Mutualist => 1,
            SpeciesType::Keystone => 2,
        }
    }

    pub fn archetype_dominance(&self) -> usize {
        match self {
            SpeciesType::Producer => 3,
            SpeciesType::Herbivore => 4,
            SpeciesType::Carnivore => 5,
            SpeciesType::Omnivore => 2,
            SpeciesType::Decomposer => 6,
            SpeciesType::Parasite => 11,
            SpeciesType::Mutualist => 2,
            SpeciesType::Keystone => 0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SpeciesType::Producer => "Creates biomass from inorganic sources",
            SpeciesType::Herbivore => "Consumes producers",
            SpeciesType::Carnivore => "Consumes other consumers",
            SpeciesType::Omnivore => "Consumes multiple trophic levels",
            SpeciesType::Decomposer => "Breaks down dead organic matter",
            SpeciesType::Parasite => "Lives on/in host organism",
            SpeciesType::Mutualist => "Provides benefit to host",
            SpeciesType::Keystone => "Disproportionate ecosystem impact",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesInteraction {
    Predation,
    Competition,
    Mutualism,
    Commensalism,
    Parasitism,
    Amensalism,
    Neutralism,
}

impl SpeciesInteraction {
    pub fn field_coupling_sign(&self) -> Float {
        match self {
            SpeciesInteraction::Predation => -0.8,
            SpeciesInteraction::Competition => -0.5,
            SpeciesInteraction::Mutualism => 0.7,
            SpeciesInteraction::Commensalism => 0.3,
            SpeciesInteraction::Parasitism => -0.6,
            SpeciesInteraction::Amensalism => -0.4,
            SpeciesInteraction::Neutralism => 0.0,
        }
    }

    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            SpeciesInteraction::Competition | SpeciesInteraction::Mutualism
        )
    }
}

#[derive(Debug, Clone)]
pub struct SpeciesFieldPattern {
    archetype_pattern: [Float; NUM_ARCHETYPES],
    morphological_encoding: [Float; 16],
    behavioral_encoding: [Float; 8],
    niche_signature: [Float; 12],
    stability: Float,
    plasticity: Float,
}

impl SpeciesFieldPattern {
    pub fn new(species_type: SpeciesType) -> Self {
        let mut archetype_pattern = [0.5; NUM_ARCHETYPES];
        let dominant = species_type.archetype_dominance();
        archetype_pattern[dominant] = 0.95;

        for i in 0..7 {
            if i != dominant % 7 {
                archetype_pattern[i] = 0.6 + (dominant as Float * 0.02);
            }
        }
        for i in 7..14 {
            archetype_pattern[i] = archetype_pattern[i - 7] * 0.7;
        }
        for i in 14..21 {
            archetype_pattern[i] = archetype_pattern[i - 14] * 0.5;
        }
        archetype_pattern[21] = 0.5;

        Self {
            archetype_pattern,
            morphological_encoding: Self::generate_morphological_encoding(species_type),
            behavioral_encoding: Self::generate_behavioral_encoding(species_type),
            niche_signature: Self::generate_niche_signature(species_type),
            stability: 0.8,
            plasticity: match species_type {
                SpeciesType::Keystone => 0.3,
                SpeciesType::Producer => 0.5,
                _ => 0.7,
            },
        }
    }

    fn generate_morphological_encoding(species_type: SpeciesType) -> [Float; 16] {
        let mut encoding = [0.5; 16];
        match species_type {
            SpeciesType::Producer => {
                encoding[0] = 0.8;
                encoding[4] = 0.9;
            }
            SpeciesType::Herbivore => {
                encoding[1] = 0.8;
                encoding[5] = 0.6;
            }
            SpeciesType::Carnivore => {
                encoding[2] = 0.9;
                encoding[6] = 0.8;
            }
            SpeciesType::Omnivore => {
                encoding[1] = 0.6;
                encoding[2] = 0.6;
            }
            _ => {}
        }
        encoding
    }

    fn generate_behavioral_encoding(species_type: SpeciesType) -> [Float; 8] {
        let mut encoding = [0.5; 8];
        match species_type {
            SpeciesType::Producer => {
                encoding[0] = 0.2;
                encoding[1] = 0.3;
            }
            SpeciesType::Carnivore => {
                encoding[0] = 0.9;
                encoding[1] = 0.8;
                encoding[2] = 0.7;
            }
            SpeciesType::Keystone => {
                encoding[0] = 0.8;
                encoding[3] = 0.9;
            }
            _ => {}
        }
        encoding
    }

    fn generate_niche_signature(species_type: SpeciesType) -> [Float; 12] {
        let mut signature = [0.5; 12];
        let level = species_type.default_trophic_level();
        signature[level] = 0.9;
        signature[6 + level] = 0.8;
        signature
    }

    pub fn archetype_pattern(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_pattern
    }

    pub fn morphological_encoding(&self) -> &[Float; 16] {
        &self.morphological_encoding
    }

    pub fn behavioral_encoding(&self) -> &[Float; 8] {
        &self.behavioral_encoding
    }

    pub fn niche_signature(&self) -> &[Float; 12] {
        &self.niche_signature
    }

    pub fn stability(&self) -> Float {
        self.stability
    }

    pub fn plasticity(&self) -> Float {
        self.plasticity
    }

    pub fn resonance_with(&self, other: &SpeciesFieldPattern) -> Float {
        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..NUM_ARCHETYPES {
            dot += self.archetype_pattern[i] * other.archetype_pattern[i];
            norm1 += self.archetype_pattern[i] * self.archetype_pattern[i];
            norm2 += other.archetype_pattern[i] * other.archetype_pattern[i];
        }

        if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1.sqrt() * norm2.sqrt())
        } else {
            0.0
        }
    }

    pub fn niche_overlap(&self, other: &SpeciesFieldPattern) -> Float {
        let mut overlap = 0.0;
        for i in 0..12 {
            overlap += (self.niche_signature[i] - other.niche_signature[i]).abs();
        }
        1.0 - (overlap / 12.0)
    }
}

#[derive(Debug, Clone)]
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub species_type: SpeciesType,
    pub field_pattern: SpeciesFieldPattern,
    pub trophic_level: usize,
    pub population_ids:
        Vec<crate::holographic_foundation::ecosystem_dynamics::population_dynamics::PopulationId>,
    pub interaction_strengths: HashMap<SpeciesId, Float>,
    pub generation_time: Float,
    pub reproductive_rate: Float,
    pub mortality_rate: Float,
}

impl Species {
    pub fn new(name: String, species_type: SpeciesType) -> Self {
        let field_pattern = SpeciesFieldPattern::new(species_type);
        let trophic_level = species_type.default_trophic_level();

        let (generation_time, reproductive_rate, mortality_rate) = match species_type {
            SpeciesType::Producer => (1.0, 0.5, 0.1),
            SpeciesType::Herbivore => (2.0, 0.3, 0.15),
            SpeciesType::Carnivore => (3.0, 0.2, 0.12),
            SpeciesType::Omnivore => (2.5, 0.25, 0.13),
            SpeciesType::Decomposer => (0.5, 0.6, 0.2),
            SpeciesType::Parasite => (1.0, 0.4, 0.25),
            SpeciesType::Mutualist => (1.5, 0.35, 0.1),
            SpeciesType::Keystone => (4.0, 0.15, 0.08),
        };

        Self {
            id: SpeciesId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            name,
            species_type,
            field_pattern,
            trophic_level,
            population_ids: Vec::new(),
            interaction_strengths: HashMap::new(),
            generation_time,
            reproductive_rate,
            mortality_rate,
        }
    }

    pub fn add_interaction(&mut self, other_id: SpeciesId, strength: Float) {
        self.interaction_strengths.insert(other_id, strength);
    }

    pub fn interaction_with(&self, other_id: &SpeciesId) -> Float {
        *self.interaction_strengths.get(other_id).unwrap_or(&0.0)
    }

    pub fn total_interaction_strength(&self) -> Float {
        self.interaction_strengths.values().sum()
    }

    pub fn competition_intensity(&self, other: &Species) -> Float {
        self.field_pattern.niche_overlap(&other.field_pattern)
    }

    pub fn potential_growth(&self, current_population: Float, carrying_capacity: Float) -> Float {
        if carrying_capacity <= 0.0 {
            return 0.0;
        }
        let intrinsic_rate = self.reproductive_rate - self.mortality_rate;
        intrinsic_rate * current_population * (1.0 - current_population / carrying_capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_id_creation() {
        let id = SpeciesId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_species_type_trophic_level() {
        assert_eq!(SpeciesType::Producer.default_trophic_level(), 0);
        assert_eq!(SpeciesType::Herbivore.default_trophic_level(), 1);
        assert_eq!(SpeciesType::Carnivore.default_trophic_level(), 2);
    }

    #[test]
    fn test_species_field_pattern_creation() {
        let pattern = SpeciesFieldPattern::new(SpeciesType::Producer);
        assert!(pattern.stability() > 0.0);
        assert!(pattern.plasticity() > 0.0);
    }

    #[test]
    fn test_species_field_pattern_resonance() {
        let pattern1 = SpeciesFieldPattern::new(SpeciesType::Producer);
        let pattern2 = SpeciesFieldPattern::new(SpeciesType::Producer);
        let resonance = pattern1.resonance_with(&pattern2);
        assert!(resonance >= 0.0 && resonance <= 1.0);
    }

    #[test]
    fn test_species_field_pattern_niche_overlap() {
        let pattern1 = SpeciesFieldPattern::new(SpeciesType::Herbivore);
        let pattern2 = SpeciesFieldPattern::new(SpeciesType::Carnivore);
        let overlap = pattern1.niche_overlap(&pattern2);
        assert!(overlap >= 0.0 && overlap <= 1.0);
    }

    #[test]
    fn test_species_creation() {
        let species = Species::new("Test Species".to_string(), SpeciesType::Herbivore);
        assert_eq!(species.name, "Test Species");
        assert_eq!(species.species_type, SpeciesType::Herbivore);
    }

    #[test]
    fn test_species_add_interaction() {
        let mut species = Species::new("Test".to_string(), SpeciesType::Carnivore);
        let other_id = SpeciesId::new(99);
        species.add_interaction(other_id, -0.5);
        assert_eq!(species.interaction_with(&other_id), -0.5);
    }

    #[test]
    fn test_species_potential_growth() {
        let species = Species::new("Test".to_string(), SpeciesType::Producer);
        let growth = species.potential_growth(100.0, 1000.0);
        assert!(growth > 0.0);
    }

    #[test]
    fn test_species_interaction_field_coupling() {
        assert!(SpeciesInteraction::Predation.field_coupling_sign() < 0.0);
        assert!(SpeciesInteraction::Mutualism.field_coupling_sign() > 0.0);
    }

    #[test]
    fn test_species_interaction_symmetry() {
        assert!(SpeciesInteraction::Mutualism.is_symmetric());
        assert!(!SpeciesInteraction::Predation.is_symmetric());
    }

    #[test]
    fn test_species_competition_intensity() {
        let species1 = Species::new("A".to_string(), SpeciesType::Herbivore);
        let species2 = Species::new("B".to_string(), SpeciesType::Herbivore);
        let competition = species1.competition_intensity(&species2);
        assert!(competition >= 0.0 && competition <= 1.0);
    }

    #[test]
    fn test_keystone_species_characteristics() {
        let keystone = Species::new("Keystone".to_string(), SpeciesType::Keystone);
        assert!(keystone.generation_time > 2.0);
        assert!(keystone.reproductive_rate < 0.2);
        assert!(keystone.field_pattern.plasticity() < 0.5);
    }
}
