//! Social Memory Complex Engine - Collective Consciousness Formation

use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

#[derive(Debug, Clone)]
pub struct SharedMemory {
    pub content: String,
    pub emotional_charge: Float,
    pub shared_by_keys: Vec<u64>,
    pub created_at: Float,
}

#[derive(Debug, Clone)]
pub struct SocialMemoryComplex {
    pub id: u64,
    pub member_entity_keys: Vec<u64>,
    pub shared_memories: Vec<SharedMemory>,
    pub collective_consciousness: Float,
    pub density: Density,
    pub polarity: Polarity,
    pub unified_purpose: String,
    pub formed_at: Float,
    pub telepathic_coherence: Float,
}

impl SocialMemoryComplex {
    pub fn new(id: u64, member_keys: Vec<u64>, density: Density, polarity: Polarity) -> Self {
        Self {
            id,
            member_entity_keys: member_keys,
            shared_memories: Vec::new(),
            collective_consciousness: 0.5,
            density,
            polarity,
            unified_purpose: "Collective evolution and service".to_string(),
            formed_at: 0.0,
            telepathic_coherence: 0.5,
        }
    }

    pub fn add_memory(
        &mut self,
        content: String,
        emotional_charge: Float,
        shared_by_keys: Vec<u64>,
    ) {
        self.shared_memories.push(SharedMemory {
            content,
            emotional_charge,
            shared_by_keys,
            created_at: 0.0,
        });

        self.collective_consciousness = (self.collective_consciousness + 0.01).min(1.0);
    }

    pub fn get_group_experience(&self) -> String {
        format!(
            "Collective consciousness: {:.1}%, {} members, coherence: {:.1}%",
            self.collective_consciousness * 100.0,
            self.member_entity_keys.len(),
            self.telepathic_coherence * 100.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct SocialMemoryComplexEngine {
    pub forming_complexes: Vec<SocialMemoryComplex>,
    pub active_complexes: HashMap<u64, SocialMemoryComplex>,
    pub next_id: u64,
}

impl Default for SocialMemoryComplexEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SocialMemoryComplexEngine {
    pub fn new() -> Self {
        SocialMemoryComplexEngine {
            forming_complexes: Vec::new(),
            active_complexes: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn tick(&mut self, _tick: u64, entity_ids: &[EntityId]) -> Vec<SmcEvent> {
        let mut events = Vec::new();

        // Simplified: form SMC if we have enough 4th+ density entities
        let fourth_density_count = entity_ids.iter().filter(|e| e.as_u64() % 8 == 3).count();

        if fourth_density_count >= 3 && self.active_complexes.is_empty() {
            let member_keys: Vec<u64> = entity_ids
                .iter()
                .filter(|e| e.as_u64() % 8 == 3)
                .take(5)
                .map(|e| e.as_u64())
                .collect();

            let smc = SocialMemoryComplex::new(
                self.next_id,
                member_keys.clone(),
                Density::Fourth,
                Polarity::ServiceToOthers,
            );

            events.push(SmcEvent::Formed(self.next_id));
            self.active_complexes.insert(self.next_id, smc);
            self.next_id += 1;
        }

        events
    }

    pub fn get_complexes(&self) -> Vec<&SocialMemoryComplex> {
        self.active_complexes.values().collect()
    }
}

#[derive(Debug, Clone)]
pub enum SmcEvent {
    Formed(u64),
    Dissolved(u64),
    MemberJoined(u64),
    MemberLeft(u64),
    ResonanceStrengthened,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smc_engine() {
        let engine = SocialMemoryComplexEngine::new();
        assert_eq!(engine.next_id, 1);
    }
}
