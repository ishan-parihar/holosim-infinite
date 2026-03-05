//! Harvest Mechanics - Density Transition System
//!
//! This module implements the harvest mechanics from the Ra Material cosmology.

use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HarvestEngine {
    pub harvest_cycle_ticks: u64,
    pub sto_threshold: f64,
    pub sts_threshold: f64,
    pub consciousness_threshold: f64,
    pub harvested_entities: HashMap<u64, HarvestEvent>,
}

impl Default for HarvestEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HarvestEngine {
    pub fn new() -> Self {
        HarvestEngine {
            harvest_cycle_ticks: 100_000,
            sto_threshold: 0.50,
            sts_threshold: 0.95,
            consciousness_threshold: 0.70,
            harvested_entities: HashMap::new(),
        }
    }

    pub fn process_harvest(&mut self, tick: u64, entity_ids: &[EntityId]) -> Vec<HarvestResult> {
        let mut results = Vec::new();

        for entity_id in entity_ids {
            let entity_key = entity_id.as_u64();

            // Simulate polarity and consciousness
            let polarity = ((entity_key % 200) as f64 / 100.0) - 1.0;
            let consciousness_level = 0.5 + ((entity_key % 50) as f64 / 100.0);

            // Only harvest at 3rd density
            if entity_key % 8 > 2 {
                continue;
            }

            let harvest_type = self.determine_harvest_type(polarity, consciousness_level);

            if harvest_type == HarvestType::NotReady {
                continue;
            }

            let event = HarvestEvent {
                entity_key,
                tick,
                harvest_type: harvest_type.clone(),
                polarity,
                consciousness_level,
                current_density: Density::Third,
                next_density: Density::Fourth,
            };

            let result = HarvestResult {
                entity_key,
                harvest_type,
                polarity,
                consciousness_level,
                ready_for_transition: true,
                event: event.clone(),
            };

            self.harvested_entities.insert(entity_key, event);
            results.push(result);
        }

        results
    }

    fn determine_harvest_type(&self, polarity: f64, consciousness_level: f64) -> HarvestType {
        if consciousness_level < self.consciousness_threshold {
            return HarvestType::NotReady;
        }

        if polarity >= self.sts_threshold {
            HarvestType::ServiceToSelf
        } else if polarity >= self.sto_threshold {
            HarvestType::ServiceToOthers
        } else if polarity <= -self.sto_threshold {
            HarvestType::ServiceToSelf
        } else {
            HarvestType::NotReady
        }
    }

    pub fn get_harvest_stats(&self) -> HarvestStats {
        let mut sto_count = 0;
        let mut sts_count = 0;
        let mut not_ready_count = 0;

        for event in self.harvested_entities.values() {
            match event.harvest_type {
                HarvestType::ServiceToOthers => sto_count += 1,
                HarvestType::ServiceToSelf => sts_count += 1,
                HarvestType::NotReady | HarvestType::Remain => not_ready_count += 1,
            }
        }

        HarvestStats {
            total_harvested: self.harvested_entities.len(),
            sto_harvested: sto_count,
            sts_harvested: sts_count,
            not_ready: not_ready_count,
        }
    }

    /// Get the count of pending harvests (entities awaiting transition).
    pub fn pending_harvests(&self) -> usize {
        self.harvested_entities
            .values()
            .filter(|e| {
                e.harvest_type == HarvestType::ServiceToOthers
                    || e.harvest_type == HarvestType::ServiceToSelf
            })
            .count()
    }

    pub fn reset_cycle(&mut self) {
        self.harvested_entities.clear();
    }
}

#[derive(Debug, Clone)]
pub struct HarvestResult {
    pub entity_key: u64,
    pub harvest_type: HarvestType,
    pub polarity: f64,
    pub consciousness_level: f64,
    pub ready_for_transition: bool,
    pub event: HarvestEvent,
}

#[derive(Debug, Clone)]
pub struct HarvestEvent {
    pub entity_key: u64,
    pub tick: u64,
    pub harvest_type: HarvestType,
    pub polarity: f64,
    pub consciousness_level: f64,
    pub current_density: Density,
    pub next_density: Density,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum HarvestType {
    ServiceToOthers,
    ServiceToSelf,
    #[default]
    NotReady,
    Remain,
}


#[derive(Debug, Clone, Default)]
pub struct HarvestStats {
    pub total_harvested: usize,
    pub sto_harvested: usize,
    pub sts_harvested: usize,
    pub not_ready: usize,
}

impl HarvestStats {
    pub fn harvest_percentage(&self) -> f64 {
        if self.total_harvested == 0 {
            return 0.0;
        }
        let ready = self.sto_harvested + self.sts_harvested;
        (ready as f64 / self.total_harvested as f64) * 100.0
    }

    pub fn sto_to_sts_ratio(&self) -> f64 {
        if self.sts_harvested == 0 {
            return f64::INFINITY;
        }
        self.sto_harvested as f64 / self.sts_harvested as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvest_engine() {
        let engine = HarvestEngine::new();
        assert_eq!(engine.harvest_cycle_ticks, 100_000);
    }
}
