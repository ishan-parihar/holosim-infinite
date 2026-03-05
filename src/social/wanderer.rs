//! Wanderer Incarnation System

use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WandererId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WandererType {
    Ram,
    FifthDensity,
    SixthDensity,
    SeventhDensity,
    Indigo,
    Crystalline,
}

impl WandererType {
    pub fn origin_density(&self) -> Density {
        match self {
            WandererType::Ram => Density::Fourth,
            WandererType::FifthDensity => Density::Fifth,
            WandererType::SixthDensity => Density::Sixth,
            WandererType::SeventhDensity => Density::Seventh,
            WandererType::Indigo => Density::Fourth,
            WandererType::Crystalline => Density::Fifth,
        }
    }

    pub fn veil_strength(&self) -> f64 {
        match self {
            WandererType::Ram => 0.3,
            WandererType::FifthDensity => 0.5,
            WandererType::SixthDensity => 0.7,
            WandererType::SeventhDensity => 0.85,
            WandererType::Indigo => 0.25,
            WandererType::Crystalline => 0.4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WandererContract {
    pub wanderer_id: WandererId,
    pub wanderer_type: WandererType,
    pub original_density: Density,
    pub target_density: Density,
    pub mission_purpose: String,
    pub special_abilities: Vec<WandererAbility>,
    pub incarnation_duration_ticks: u64,
    pub incarnated_at_tick: u64,
    pub host_entity_key: Option<u64>,
    pub fulfilled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WandererAbility {
    Healing,
    Telepathy,
    PsychicSensitivity,
    Teaching,
    Artistic,
    Technical,
    Leadership,
    EmotionalSensitivity,
    SpiritualGuidance,
    AuraPerception,
}

#[derive(Debug, Clone)]
pub struct WandererEngine {
    pub contracts: HashMap<WandererId, WandererContract>,
    pub next_wanderer_id: u64,
    pub incarnation_rate: f64,
}

impl Default for WandererEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl WandererEngine {
    pub fn new() -> Self {
        WandererEngine {
            contracts: HashMap::new(),
            next_wanderer_id: 1,
            incarnation_rate: 0.001,
        }
    }

    pub fn tick(&mut self, tick: u64, entity_ids: &[EntityId]) -> Vec<WandererEvent> {
        let mut events = Vec::new();

        // Check for departures
        let mut to_remove = Vec::new();
        for (wanderer_id, contract) in self.contracts.iter_mut() {
            let elapsed = tick.saturating_sub(contract.incarnated_at_tick);
            if elapsed >= contract.incarnation_duration_ticks {
                contract.fulfilled = true;
                if let Some(host_key) = contract.host_entity_key {
                    events.push(WandererEvent::Departed(host_key));
                }
                events.push(WandererEvent::ContractFulfilled(*wanderer_id));
                to_remove.push(*wanderer_id);
            }
        }

        for wid in to_remove {
            self.contracts.remove(&wid);
        }

        // Consider new wanderers
        if !entity_ids.is_empty()
            && rand::random::<f64>() < self.incarnation_rate * entity_ids.len() as f64
        {
            let host_id = &entity_ids[0];
            let host_key = host_id.as_u64();

            let wanderer_id = WandererId(self.next_wanderer_id);
            self.next_wanderer_id += 1;

            let wanderer_type = match rand::random::<u8>() % 6 {
                0 => WandererType::Ram,
                1 => WandererType::FifthDensity,
                2 => WandererType::SixthDensity,
                3 => WandererType::Indigo,
                4 => WandererType::Crystalline,
                _ => WandererType::Ram,
            };

            let purpose = "To assist in planetary evolution".to_string();

            let abilities = vec![WandererAbility::Healing];

            let contract = WandererContract {
                wanderer_id,
                wanderer_type,
                original_density: wanderer_type.origin_density(),
                target_density: Density::Third,
                mission_purpose: purpose,
                special_abilities: abilities,
                incarnation_duration_ticks: 50_000 + rand::random::<u64>() % 100_000,
                incarnated_at_tick: tick,
                host_entity_key: Some(host_key),
                fulfilled: false,
            };

            events.push(WandererEvent::Incarnated(wanderer_id, host_key));
            self.contracts.insert(wanderer_id, contract);
        }

        events
    }

    pub fn get_wanderer_stats(&self) -> WandererStats {
        WandererStats {
            total_wanderers: self.contracts.len(),
            pending_incarnations: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WandererStats {
    pub total_wanderers: usize,
    pub pending_incarnations: usize,
}

#[derive(Debug, Clone)]
pub enum WandererEvent {
    Incarnated(WandererId, u64),
    Departed(u64),
    ContractFulfilled(WandererId),
    ContractAbandoned(WandererId),
}

pub fn calculate_veil_effectiveness(
    wanderer_type: &WandererType,
    entity_consciousness: f64,
) -> f64 {
    let base_veil = wanderer_type.veil_strength();
    let consciousness_factor = entity_consciousness * 0.3;
    (base_veil - consciousness_factor).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wanderer_types() {
        assert_eq!(WandererType::Ram.origin_density(), Density::Fourth);
    }
}
