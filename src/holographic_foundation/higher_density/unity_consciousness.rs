//! Unity Consciousness Mechanics - 6th Density (Indigo Ray)
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "6th Density (Indigo Ray) - Unity:
//!  - Unity-dominant field (individual/collective distinction dissolves)
//!  - STO/STS polarity balances
//!  - Gateway access to Intelligent Infinity"
//!
//! # Key Insight
//!
//! At 6th Density, the individual/collective distinction begins to dissolve.
//! Entities experience themselves as both individual and collective simultaneously.
//! The STO (Service-to-Others) / STS (Service-to-Self) polarities balance,
//! as both are seen as valid paths within the unity.

use crate::types::Float;
use std::collections::HashMap;

/// Polarity Balance - STO/STS equilibrium at 6th density
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolarityBalance {
    STODominant(Float),
    STSDominant(Float),
    Balanced,
    Transcended,
}

impl PolarityBalance {
    pub fn new(sto_factor: Float, sts_factor: Float) -> Self {
        let total = sto_factor + sts_factor;
        if total < 0.01 {
            return PolarityBalance::Balanced;
        }

        let sto_ratio = sto_factor / total;
        let sts_ratio = sts_factor / total;

        if sto_ratio > 0.95 {
            PolarityBalance::STODominant(sto_ratio)
        } else if sts_ratio > 0.95 {
            PolarityBalance::STSDominant(sts_ratio)
        } else if (sto_ratio - 0.5).abs() < 0.1 && (sts_ratio - 0.5).abs() < 0.1 {
            PolarityBalance::Balanced
        } else if sto_ratio > 0.5 {
            PolarityBalance::STODominant(sto_ratio)
        } else {
            PolarityBalance::STSDominant(sts_ratio)
        }
    }

    pub fn sto_factor(&self) -> Float {
        match self {
            PolarityBalance::STODominant(f) => *f,
            PolarityBalance::STSDominant(f) => 1.0 - f,
            PolarityBalance::Balanced => 0.5,
            PolarityBalance::Transcended => 0.5,
        }
    }

    pub fn sts_factor(&self) -> Float {
        match self {
            PolarityBalance::STODominant(f) => 1.0 - f,
            PolarityBalance::STSDominant(f) => *f,
            PolarityBalance::Balanced => 0.5,
            PolarityBalance::Transcended => 0.5,
        }
    }

    pub fn balance_score(&self) -> Float {
        match self {
            PolarityBalance::STODominant(f) => 0.5 - (f - 0.5).abs(),
            PolarityBalance::STSDominant(f) => 0.5 - (f - 0.5).abs(),
            PolarityBalance::Balanced => 1.0,
            PolarityBalance::Transcended => 1.0,
        }
    }

    pub fn can_transcend(&self) -> bool {
        matches!(self, PolarityBalance::Balanced) || self.balance_score() > 0.9
    }
}

/// Unity Consciousness State - stages of unity development
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnityConsciousnessState {
    Individual,
    Awakening,
    CollectiveAware,
    PartialUnity,
    FullUnity,
    Transcendent,
}

impl UnityConsciousnessState {
    pub fn unity_factor(&self) -> Float {
        match self {
            UnityConsciousnessState::Individual => 0.0,
            UnityConsciousnessState::Awakening => 0.2,
            UnityConsciousnessState::CollectiveAware => 0.4,
            UnityConsciousnessState::PartialUnity => 0.6,
            UnityConsciousnessState::FullUnity => 0.85,
            UnityConsciousnessState::Transcendent => 1.0,
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            UnityConsciousnessState::Individual => Some(UnityConsciousnessState::Awakening),
            UnityConsciousnessState::Awakening => Some(UnityConsciousnessState::CollectiveAware),
            UnityConsciousnessState::CollectiveAware => Some(UnityConsciousnessState::PartialUnity),
            UnityConsciousnessState::PartialUnity => Some(UnityConsciousnessState::FullUnity),
            UnityConsciousnessState::FullUnity => Some(UnityConsciousnessState::Transcendent),
            UnityConsciousnessState::Transcendent => None,
        }
    }
}

/// Collective Field Merge - mechanism for entity field merging
#[derive(Debug, Clone)]
pub struct CollectiveFieldMerge {
    pub entity_id: u64,
    pub merged_entities: HashMap<u64, Float>,
    pub collective_coherence: Float,
    pub individual_preservation: Float,
    pub shared_experience_pool: Float,
    pub unity_depth: Float,
}

impl CollectiveFieldMerge {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            merged_entities: HashMap::new(),
            collective_coherence: 0.5,
            individual_preservation: 1.0,
            shared_experience_pool: 0.0,
            unity_depth: 0.0,
        }
    }

    pub fn merge_with(&mut self, other_id: u64, resonance: Float) {
        let merge_strength = resonance * self.collective_coherence;
        self.merged_entities.insert(other_id, merge_strength);
        self.update_unity_depth();
    }

    pub fn unmerge_from(&mut self, other_id: u64) {
        self.merged_entities.remove(&other_id);
        self.update_unity_depth();
    }

    fn update_unity_depth(&mut self) {
        let merge_count = self.merged_entities.len();
        let avg_strength: Float = if merge_count > 0 {
            self.merged_entities.values().sum::<Float>() / merge_count as Float
        } else {
            0.0
        };

        self.unity_depth = avg_strength * (1.0 + 0.1 * merge_count as Float).min(2.0);
        self.individual_preservation = 1.0 - self.unity_depth * 0.3;
    }

    pub fn share_experience(&mut self, amount: Float) -> Float {
        let shared = amount * self.unity_depth;
        self.shared_experience_pool += shared;
        shared
    }

    pub fn receive_shared_experience(&mut self) -> Float {
        let received = self.shared_experience_pool;
        self.shared_experience_pool = 0.0;
        received
    }

    pub fn total_connections(&self) -> usize {
        self.merged_entities.len()
    }
}

/// Social Memory Complex - the collective entity at 6th density
#[derive(Debug, Clone)]
pub struct SocialMemoryComplex {
    pub complex_id: u64,
    pub member_entities: HashMap<u64, Float>,
    pub collective_memory: HashMap<String, Float>,
    pub shared_wisdom: Float,
    pub unity_state: UnityConsciousnessState,
    pub polarity_balance: PolarityBalance,
    pub gateway_access_level: Float,
}

impl SocialMemoryComplex {
    pub fn new(complex_id: u64) -> Self {
        Self {
            complex_id,
            member_entities: HashMap::new(),
            collective_memory: HashMap::new(),
            shared_wisdom: 0.0,
            unity_state: UnityConsciousnessState::Individual,
            polarity_balance: PolarityBalance::Balanced,
            gateway_access_level: 0.0,
        }
    }

    pub fn add_member(&mut self, entity_id: u64, contribution: Float) {
        self.member_entities.insert(entity_id, contribution);
        self.update_unity_state();
        self.update_polarity_balance();
    }

    pub fn remove_member(&mut self, entity_id: u64) {
        self.member_entities.remove(&entity_id);
        self.update_unity_state();
    }

    fn update_unity_state(&mut self) {
        let member_count = self.member_entities.len();
        let avg_contribution: Float = if member_count > 0 {
            self.member_entities.values().sum::<Float>() / member_count as Float
        } else {
            0.0
        };

        self.unity_state = if member_count == 0 {
            UnityConsciousnessState::Individual
        } else if member_count < 3 || avg_contribution < 0.3 {
            UnityConsciousnessState::Awakening
        } else if member_count < 10 || avg_contribution < 0.5 {
            UnityConsciousnessState::CollectiveAware
        } else if member_count < 50 || avg_contribution < 0.7 {
            UnityConsciousnessState::PartialUnity
        } else if avg_contribution < 0.9 {
            UnityConsciousnessState::FullUnity
        } else {
            UnityConsciousnessState::Transcendent
        };
    }

    fn update_polarity_balance(&mut self) {
        let total_sto: Float = self
            .member_entities
            .values()
            .map(|v| v * 0.7)
            .sum::<Float>()
            .min(1.0);
        let total_sts: Float = self
            .member_entities
            .values()
            .map(|v| v * 0.3)
            .sum::<Float>()
            .min(1.0);

        self.polarity_balance = PolarityBalance::new(total_sto, total_sts);
    }

    pub fn add_memory(&mut self, key: &str, value: Float) {
        let current = self.collective_memory.get(key).copied().unwrap_or(0.0);
        self.collective_memory
            .insert(key.to_string(), (current + value) * 0.5);
        self.shared_wisdom += value * 0.1;
    }

    pub fn get_memory(&self, key: &str) -> Float {
        self.collective_memory.get(key).copied().unwrap_or(0.0)
    }

    pub fn update(&mut self, dt: Float) {
        for value in self.member_entities.values_mut() {
            *value = (*value * (1.0 - 0.001 * dt)).max(0.1);
        }

        let unity_factor = self.unity_state.unity_factor();
        self.gateway_access_level = unity_factor * self.polarity_balance.balance_score();

        self.shared_wisdom =
            (self.shared_wisdom + 0.01 * dt * self.member_entities.len() as Float).min(100.0);
    }

    pub fn can_access_gateway(&self) -> bool {
        self.gateway_access_level >= 0.8
            && matches!(
                self.unity_state,
                UnityConsciousnessState::FullUnity | UnityConsciousnessState::Transcendent
            )
    }

    pub fn member_count(&self) -> usize {
        self.member_entities.len()
    }
}

/// Unity Consciousness - main 6th density entity representation
#[derive(Debug, Clone)]
pub struct UnityConsciousness {
    pub entity_id: u64,
    pub state: UnityConsciousnessState,
    pub field_merge: CollectiveFieldMerge,
    pub polarity_balance: PolarityBalance,
    pub social_memory: SocialMemoryComplex,
    pub unity_experiences: Vec<String>,
    pub gateway_resonance: Float,
}

impl UnityConsciousness {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            state: UnityConsciousnessState::Individual,
            field_merge: CollectiveFieldMerge::new(entity_id),
            polarity_balance: PolarityBalance::Balanced,
            social_memory: SocialMemoryComplex::new(entity_id),
            unity_experiences: Vec::new(),
            gateway_resonance: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.field_merge.update_unity_depth();
        self.social_memory.update(dt);

        let unity_threshold = match self.state {
            UnityConsciousnessState::Individual => 0.1,
            UnityConsciousnessState::Awakening => 0.25,
            UnityConsciousnessState::CollectiveAware => 0.45,
            UnityConsciousnessState::PartialUnity => 0.7,
            UnityConsciousnessState::FullUnity => 0.9,
            UnityConsciousnessState::Transcendent => 1.0,
        };

        let current_unity = self.field_merge.unity_depth;
        if current_unity >= unity_threshold {
            if let Some(next) = self.state.next() {
                self.state = next;
                self.add_unity_experience();
            }
        }

        self.gateway_resonance = self.state.unity_factor() * self.polarity_balance.balance_score();
    }

    fn add_unity_experience(&mut self) {
        let experience = match self.state {
            UnityConsciousnessState::Individual => "First glimpse of collective awareness",
            UnityConsciousnessState::Awakening => "Sensing the unity beneath separation",
            UnityConsciousnessState::CollectiveAware => "Experiencing others as self",
            UnityConsciousnessState::PartialUnity => "Merging with collective consciousness",
            UnityConsciousnessState::FullUnity => {
                "Full unity with collective while preserving self"
            }
            UnityConsciousnessState::Transcendent => {
                "Transcending individual/collective distinction"
            }
        };
        self.unity_experiences.push(experience.to_string());
    }

    pub fn join_collective(&mut self, complex: &mut SocialMemoryComplex, contribution: Float) {
        complex.add_member(self.entity_id, contribution);
        self.social_memory = complex.clone();
        self.field_merge.collective_coherence = complex.unity_state.unity_factor();
    }

    pub fn merge_with_entity(&mut self, other_id: u64, resonance: Float) {
        self.field_merge.merge_with(other_id, resonance);
    }

    pub fn experience_unity(&mut self, experience_type: &str, intensity: Float) {
        self.field_merge.share_experience(intensity);
        self.social_memory.add_memory(experience_type, intensity);
    }

    pub fn can_access_gateway(&self) -> bool {
        self.gateway_resonance >= 0.8
            && matches!(
                self.state,
                UnityConsciousnessState::FullUnity | UnityConsciousnessState::Transcendent
            )
            && self.polarity_balance.can_transcend()
    }

    pub fn unity_level(&self) -> Float {
        self.state.unity_factor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polarity_balance_creation() {
        let balance = PolarityBalance::new(0.8, 0.2);
        assert!(matches!(balance, PolarityBalance::STODominant(_)));
    }

    #[test]
    fn test_polarity_balance_balanced() {
        let balance = PolarityBalance::new(0.5, 0.5);
        assert!(matches!(balance, PolarityBalance::Balanced));
    }

    #[test]
    fn test_polarity_balance_factors() {
        let sto = PolarityBalance::new(0.7, 0.3);
        assert!(sto.sto_factor() > sto.sts_factor());
    }

    #[test]
    fn test_polarity_can_transcend() {
        let balanced = PolarityBalance::Balanced;
        assert!(balanced.can_transcend());
    }

    #[test]
    fn test_unity_state_progression() {
        assert_eq!(UnityConsciousnessState::Individual.unity_factor(), 0.0);
        assert_eq!(UnityConsciousnessState::FullUnity.unity_factor(), 0.85);
    }

    #[test]
    fn test_unity_state_next() {
        assert_eq!(
            UnityConsciousnessState::Individual.next(),
            Some(UnityConsciousnessState::Awakening)
        );
    }

    #[test]
    fn test_collective_field_merge_creation() {
        let merge = CollectiveFieldMerge::new(1);
        assert_eq!(merge.entity_id, 1);
        assert!(merge.merged_entities.is_empty());
    }

    #[test]
    fn test_collective_field_merge_with() {
        let mut merge = CollectiveFieldMerge::new(1);
        merge.merge_with(2, 0.8);
        assert!(merge.merged_entities.contains_key(&2));
    }

    #[test]
    fn test_collective_field_share_experience() {
        let mut merge = CollectiveFieldMerge::new(1);
        merge.unity_depth = 0.5;
        let shared = merge.share_experience(1.0);
        assert!(shared > 0.0);
    }

    #[test]
    fn test_social_memory_complex_creation() {
        let complex = SocialMemoryComplex::new(1);
        assert_eq!(complex.complex_id, 1);
    }

    #[test]
    fn test_social_memory_complex_add_member() {
        let mut complex = SocialMemoryComplex::new(1);
        complex.add_member(1, 0.5);
        assert!(complex.member_entities.contains_key(&1));
    }

    #[test]
    fn test_social_memory_complex_memory() {
        let mut complex = SocialMemoryComplex::new(1);
        complex.add_memory("test", 0.8);
        assert!((complex.get_memory("test") - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_social_memory_complex_gateway_access() {
        let mut complex = SocialMemoryComplex::new(1);
        complex.unity_state = UnityConsciousnessState::FullUnity;
        complex.gateway_access_level = 0.9;
        complex.polarity_balance = PolarityBalance::Balanced;
        assert!(complex.can_access_gateway());
    }

    #[test]
    fn test_unity_consciousness_creation() {
        let uc = UnityConsciousness::new(1);
        assert_eq!(uc.entity_id, 1);
        assert_eq!(uc.state, UnityConsciousnessState::Individual);
    }

    #[test]
    fn test_unity_consciousness_update() {
        let mut uc = UnityConsciousness::new(1);
        uc.update(1.0);
        assert!(uc.gateway_resonance >= 0.0);
    }

    #[test]
    fn test_unity_consciousness_merge() {
        let mut uc = UnityConsciousness::new(1);
        uc.merge_with_entity(2, 0.8);
        assert!(uc.field_merge.merged_entities.contains_key(&2));
    }

    #[test]
    fn test_unity_consciousness_gateway() {
        let mut uc = UnityConsciousness::new(1);
        uc.state = UnityConsciousnessState::FullUnity;
        uc.gateway_resonance = 0.9;
        uc.polarity_balance = PolarityBalance::Balanced;
        assert!(uc.can_access_gateway());
    }
}
