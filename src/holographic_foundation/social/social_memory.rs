//! Social Memory - Shared experiences and collective learning
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Shared experiences and learning"
//!
//! KEY INSIGHT: Social memory transcends individual entity memory,
//! creating collective knowledge that persists and evolves.

use super::MEMORY_CAPACITY;
use crate::holographic_foundation::evolution::DecisionType;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SharedExperience {
    pub experience_id: u64,
    pub collective_id: u64,
    pub experience_type: ExperienceType,
    pub participants: Vec<u64>,
    pub significance: Float,
    pub emotional_charge: Float,
    pub learning_value: Float,
    pub time: Float,
    pub encoding: ExperienceEncoding,
}

impl SharedExperience {
    pub fn new(
        experience_id: u64,
        collective_id: u64,
        experience_type: ExperienceType,
        participants: Vec<u64>,
        time: Float,
    ) -> Self {
        Self {
            experience_id,
            collective_id,
            experience_type,
            participants,
            significance: 0.5,
            emotional_charge: 0.0,
            learning_value: 0.0,
            time,
            encoding: ExperienceEncoding::new(),
        }
    }

    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    pub fn collective_significance(&self) -> Float {
        self.significance * (self.participants.len() as Float).sqrt() / 10.0
    }

    pub fn decay(&mut self, rate: Float) {
        self.significance *= 1.0 - rate;
        self.emotional_charge *= 1.0 - rate * 0.5;
        self.learning_value = (self.learning_value + 0.01).min(1.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExperienceType {
    CollectiveDecision,
    CatalystEvent,
    DensityTransition,
    Formation,
    Dissolution,
    Healing,
    Learning,
    Teaching,
    Creative,
    Transformative,
}

impl ExperienceType {
    pub fn default_significance(&self) -> Float {
        match self {
            ExperienceType::CollectiveDecision => 0.3,
            ExperienceType::CatalystEvent => 0.7,
            ExperienceType::DensityTransition => 0.9,
            ExperienceType::Formation => 0.5,
            ExperienceType::Dissolution => 0.6,
            ExperienceType::Healing => 0.4,
            ExperienceType::Learning => 0.5,
            ExperienceType::Teaching => 0.5,
            ExperienceType::Creative => 0.6,
            ExperienceType::Transformative => 0.8,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ExperienceType::CollectiveDecision => "Collective Decision",
            ExperienceType::CatalystEvent => "Catalyst Event",
            ExperienceType::DensityTransition => "Density Transition",
            ExperienceType::Formation => "Formation",
            ExperienceType::Dissolution => "Dissolution",
            ExperienceType::Healing => "Healing",
            ExperienceType::Learning => "Learning",
            ExperienceType::Teaching => "Teaching",
            ExperienceType::Creative => "Creative",
            ExperienceType::Transformative => "Transformative",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExperienceEncoding {
    pub pattern: [Float; 8],
    pub archetype_impressions: [Float; 22],
    pub emotional_signature: Float,
}

impl ExperienceEncoding {
    pub fn new() -> Self {
        Self {
            pattern: [0.0; 8],
            archetype_impressions: [0.0; 22],
            emotional_signature: 0.0,
        }
    }

    pub fn from_decision(decision_type: &DecisionType, significance: Float) -> Self {
        let base_pattern = decision_type.perturbation_signature();

        Self {
            pattern: base_pattern.density_modulations,
            archetype_impressions: [significance / 22.0; 22],
            emotional_signature: significance,
        }
    }

    pub fn similarity(&self, other: &ExperienceEncoding) -> Float {
        let pattern_sim: Float = self
            .pattern
            .iter()
            .zip(other.pattern.iter())
            .map(|(a, b)| 1.0 - (a - b).abs())
            .sum::<Float>()
            / 8.0;

        let archetype_sim: Float = self
            .archetype_impressions
            .iter()
            .zip(other.archetype_impressions.iter())
            .map(|(a, b)| 1.0 - (a - b).abs())
            .sum::<Float>()
            / 22.0;

        (pattern_sim * 0.6 + archetype_sim * 0.4).clamp(0.0, 1.0)
    }

    pub fn merge(&mut self, other: &ExperienceEncoding, weight: Float) {
        for i in 0..8 {
            self.pattern[i] = self.pattern[i] * (1.0 - weight) + other.pattern[i] * weight;
        }

        for i in 0..22 {
            self.archetype_impressions[i] = self.archetype_impressions[i] * (1.0 - weight)
                + other.archetype_impressions[i] * weight;
        }

        self.emotional_signature =
            self.emotional_signature * (1.0 - weight) + other.emotional_signature * weight;
    }
}

impl Default for ExperienceEncoding {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SocialMemoryEntry {
    pub entity_id: u64,
    pub collective_id: Option<u64>,
    pub experiences: Vec<u64>,
    pub total_significance: Float,
    pub dominant_experience_type: Option<ExperienceType>,
    pub wisdom_accumulated: Float,
}

impl SocialMemoryEntry {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            collective_id: None,
            experiences: Vec::new(),
            total_significance: 0.0,
            dominant_experience_type: None,
            wisdom_accumulated: 0.0,
        }
    }

    pub fn add_experience(
        &mut self,
        experience_id: u64,
        significance: Float,
        _experience_type: ExperienceType,
    ) {
        self.experiences.push(experience_id);
        self.total_significance += significance;
        self.wisdom_accumulated = (self.wisdom_accumulated + significance * 0.1).min(1.0);
    }

    pub fn experience_count(&self) -> usize {
        self.experiences.len()
    }
}

pub struct SocialMemory {
    experiences: HashMap<u64, SharedExperience>,
    entity_memories: HashMap<u64, SocialMemoryEntry>,
    collective_memories: HashMap<u64, SocialMemoryEntry>,
    experience_index: HashMap<ExperienceType, Vec<u64>>,
    next_experience_id: u64,
    decay_rate: Float,
}

impl SocialMemory {
    pub fn new() -> Self {
        Self {
            experiences: HashMap::new(),
            entity_memories: HashMap::new(),
            collective_memories: HashMap::new(),
            experience_index: HashMap::new(),
            next_experience_id: 1,
            decay_rate: 0.01,
        }
    }

    pub fn record_experience(
        &mut self,
        collective_id: u64,
        experience_type: ExperienceType,
        participants: Vec<u64>,
        time: Float,
    ) -> u64 {
        let experience_id = self.next_experience_id;
        self.next_experience_id += 1;

        let mut experience = SharedExperience::new(
            experience_id,
            collective_id,
            experience_type,
            participants.clone(),
            time,
        );
        experience.significance = experience_type.default_significance();
        experience.encoding =
            ExperienceEncoding::from_decision(&DecisionType::Growth, experience.significance);

        for entity_id in &participants {
            let entry = self
                .entity_memories
                .entry(*entity_id)
                .or_insert_with(|| SocialMemoryEntry::new(*entity_id));
            entry.add_experience(experience_id, experience.significance, experience_type);
        }

        let collective_entry = self
            .collective_memories
            .entry(collective_id)
            .or_insert_with(|| SocialMemoryEntry::new(collective_id));
        collective_entry.add_experience(experience_id, experience.significance, experience_type);

        self.experience_index
            .entry(experience_type)
            .or_default()
            .push(experience_id);

        self.experiences.insert(experience_id, experience);

        self.prune_if_needed();

        experience_id
    }

    pub fn get_experience(&self, experience_id: u64) -> Option<&SharedExperience> {
        self.experiences.get(&experience_id)
    }

    pub fn get_entity_memory(&self, entity_id: u64) -> Option<&SocialMemoryEntry> {
        self.entity_memories.get(&entity_id)
    }

    pub fn get_collective_memory(&self, collective_id: u64) -> Option<&SocialMemoryEntry> {
        self.collective_memories.get(&collective_id)
    }

    pub fn get_experiences_by_type(
        &self,
        experience_type: ExperienceType,
    ) -> Vec<&SharedExperience> {
        self.experience_index
            .get(&experience_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.experiences.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn find_similar_experiences(
        &self,
        encoding: &ExperienceEncoding,
        threshold: Float,
    ) -> Vec<&SharedExperience> {
        self.experiences
            .values()
            .filter(|e| e.encoding.similarity(encoding) >= threshold)
            .collect()
    }

    pub fn compute_wisdom(&self, entity_id: u64) -> Float {
        self.entity_memories
            .get(&entity_id)
            .map(|e| e.wisdom_accumulated)
            .unwrap_or(0.0)
    }

    pub fn compute_collective_wisdom(&self, collective_id: u64) -> Float {
        self.collective_memories
            .get(&collective_id)
            .map(|e| e.wisdom_accumulated)
            .unwrap_or(0.0)
    }

    pub fn update(&mut self, dt: Float) {
        for experience in self.experiences.values_mut() {
            experience.decay(self.decay_rate * dt);
        }

        let threshold = 0.01;
        let to_remove: Vec<u64> = self
            .experiences
            .iter()
            .filter(|(_, e)| e.significance < threshold)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            self.remove_experience(id);
        }
    }

    fn remove_experience(&mut self, experience_id: u64) {
        if let Some(experience) = self.experiences.remove(&experience_id) {
            for entity_id in &experience.participants {
                if let Some(entry) = self.entity_memories.get_mut(entity_id) {
                    entry.experiences.retain(|&id| id != experience_id);
                }
            }

            if let Some(entries) = self.experience_index.get_mut(&experience.experience_type) {
                entries.retain(|&id| id != experience_id);
            }
        }
    }

    fn prune_if_needed(&mut self) {
        if self.experiences.len() > MEMORY_CAPACITY {
            let mut sorted: Vec<_> = self.experiences.iter().collect();
            sorted.sort_by(|a, b| {
                a.1.significance
                    .partial_cmp(&b.1.significance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let to_remove = sorted
                .iter()
                .take(self.experiences.len() - MEMORY_CAPACITY)
                .map(|(id, _)| **id)
                .collect::<Vec<_>>();

            for id in to_remove {
                self.remove_experience(id);
            }
        }
    }

    pub fn total_experiences(&self) -> usize {
        self.experiences.len()
    }

    pub fn entity_count(&self) -> usize {
        self.entity_memories.len()
    }

    pub fn collective_count(&self) -> usize {
        self.collective_memories.len()
    }

    pub fn transfer_memory(&mut self, from_entity: u64, to_entity: u64, transfer_ratio: Float) {
        if let Some(from_entry) = self.entity_memories.get(&from_entity) {
            let transfer_amount = from_entry.wisdom_accumulated * transfer_ratio;

            let to_entry = self
                .entity_memories
                .entry(to_entity)
                .or_insert_with(|| SocialMemoryEntry::new(to_entity));
            to_entry.wisdom_accumulated += transfer_amount;
        }
    }
}

impl Default for SocialMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_encoding_creation() {
        let encoding = ExperienceEncoding::new();
        assert_eq!(encoding.pattern, [0.0; 8]);
    }

    #[test]
    fn test_experience_encoding_from_decision() {
        let encoding = ExperienceEncoding::from_decision(&DecisionType::Growth, 0.8);

        assert!(encoding.emotional_signature > 0.0);
    }

    #[test]
    fn test_experience_encoding_similarity() {
        let enc1 = ExperienceEncoding::from_decision(&DecisionType::Growth, 0.5);
        let enc2 = ExperienceEncoding::from_decision(&DecisionType::Growth, 0.5);

        let similarity = enc1.similarity(&enc2);
        assert!((similarity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_shared_experience_creation() {
        let exp = SharedExperience::new(1, 100, ExperienceType::CatalystEvent, vec![1, 2, 3], 0.0);

        assert_eq!(exp.participant_count(), 3);
    }

    #[test]
    fn test_social_memory_creation() {
        let memory = SocialMemory::new();
        assert_eq!(memory.total_experiences(), 0);
    }

    #[test]
    fn test_record_experience() {
        let mut memory = SocialMemory::new();

        let exp_id =
            memory.record_experience(100, ExperienceType::CollectiveDecision, vec![1, 2, 3], 0.0);

        assert!(exp_id > 0);
        assert_eq!(memory.total_experiences(), 1);
    }

    #[test]
    fn test_get_entity_memory() {
        let mut memory = SocialMemory::new();

        memory.record_experience(100, ExperienceType::CollectiveDecision, vec![1, 2], 0.0);

        let entry = memory.get_entity_memory(1);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().experience_count(), 1);
    }

    #[test]
    fn test_compute_wisdom() {
        let mut memory = SocialMemory::new();

        memory.record_experience(100, ExperienceType::DensityTransition, vec![1], 0.0);

        let wisdom = memory.compute_wisdom(1);
        assert!(wisdom > 0.0);
    }

    #[test]
    fn test_find_similar_experiences() {
        let mut memory = SocialMemory::new();

        memory.record_experience(100, ExperienceType::CollectiveDecision, vec![1, 2], 0.0);

        let encoding = ExperienceEncoding::from_decision(&DecisionType::Growth, 0.5);
        let similar = memory.find_similar_experiences(&encoding, 0.0);

        assert!(!similar.is_empty());
    }

    #[test]
    fn test_experience_type_significance() {
        assert!(
            ExperienceType::DensityTransition.default_significance()
                > ExperienceType::CollectiveDecision.default_significance()
        );
    }
}
