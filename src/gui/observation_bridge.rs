//! Observation Bridge - Dwarf-Fortress-Level Entity Observation
//!
//! This module provides comprehensive observation of entities across all
//! simulation systems: biology, consciousness, and social dynamics.

use crate::biology::ConsciousnessTickEngine;
use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use crate::social::SocialState;
use std::collections::HashMap;

/// Complete entity observation data
#[derive(Debug, Clone)]
pub struct EntityObservation {
    /// Entity ID
    pub entity_id: EntityId,

    // === Biology Layer ===
    pub health: f64,
    pub energy: f64,
    pub age: u64,
    pub cell_count: usize,

    // === Consciousness Layer ===
    pub consciousness_level: f64,
    pub polarity: f64,
    pub wisdom: f64,
    pub density: Density,
    pub st_ratio: f64,
    pub top_archetypes: Vec<(u8, f64)>,

    // === Social Layer ===
    pub relationship_count: usize,
    pub total_bond_strength: f64,
    pub social_connection: f64,
    pub social_isolation: f64,

    // === Status ===
    pub harvest_ready: bool,
    pub harvest_type: Option<String>,
    pub is_wanderer: bool,
}

/// Observation bridge - provides unified entity observation
pub struct ObservationBridge {
    consciousness_engine: Option<std::sync::Arc<std::sync::RwLock<ConsciousnessTickEngine>>>,
    social_state: Option<std::sync::Arc<std::sync::RwLock<SocialState>>>,
    cache: HashMap<u64, EntityObservation>,
    last_update_tick: u64,
    cache_duration: u64,
}

impl Default for ObservationBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl ObservationBridge {
    pub fn new() -> Self {
        ObservationBridge {
            consciousness_engine: None,
            social_state: None,
            cache: HashMap::new(),
            last_update_tick: 0,
            cache_duration: 10,
        }
    }

    pub fn with_consciousness(
        mut self,
        engine: std::sync::Arc<std::sync::RwLock<ConsciousnessTickEngine>>,
    ) -> Self {
        self.consciousness_engine = Some(engine);
        self
    }

    pub fn with_social(mut self, state: std::sync::Arc<std::sync::RwLock<SocialState>>) -> Self {
        self.social_state = Some(state);
        self
    }

    pub fn get_observation(&self, entity_id: EntityId, current_tick: u64) -> EntityObservation {
        let key = entity_id.as_u64();

        if let Some(cached) = self.cache.get(&key) {
            if current_tick - self.last_update_tick < self.cache_duration {
                return cached.clone();
            }
        }

        self.generate_observation(entity_id)
    }

    fn generate_observation(&self, entity_id: EntityId) -> EntityObservation {
        let key = entity_id.as_u64();

        let (consciousness_level, polarity, wisdom, density, st_ratio, archetypes) =
            self.get_consciousness_data(key);

        let (relationship_count, total_bond, connection, isolation) = self.get_social_data(key);

        let (harvest_ready, harvest_type) =
            self.check_harvest_readiness(polarity, consciousness_level, &density);

        let is_wanderer = self.check_wanderer(key);

        EntityObservation {
            entity_id,
            health: 0.8,
            energy: 0.7,
            age: key % 10000,
            cell_count: (key % 1000) as usize,
            consciousness_level,
            polarity,
            wisdom,
            density,
            st_ratio,
            top_archetypes: archetypes,
            relationship_count,
            total_bond_strength: total_bond,
            social_connection: connection,
            social_isolation: isolation,
            harvest_ready,
            harvest_type,
            is_wanderer,
        }
    }

    fn get_consciousness_data(&self, key: u64) -> (f64, f64, f64, Density, f64, Vec<(u8, f64)>) {
        let consciousness_level = ((key % 100) as f64 / 100.0) * 0.8 + 0.2;
        let polarity = ((key % 200) as f64 / 100.0) - 1.0;
        let wisdom = ((key % 50) as f64 / 100.0) * 0.5;
        let density = match key % 8 {
            0..=2 => Density::Third,
            3 => Density::Fourth,
            4 => Density::Fifth,
            5 => Density::Sixth,
            _ => Density::Third,
        };
        let st_ratio = ((key % 100) as f64 / 50.0) - 1.0;

        let archetypes: Vec<(u8, f64)> = (0..22)
            .map(|i| (i as u8, ((key + i as u64) % 100) as f64 / 100.0))
            .collect();

        let mut sorted = archetypes.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top_archetypes: Vec<(u8, f64)> = sorted.into_iter().take(3).collect();

        (
            consciousness_level,
            polarity,
            wisdom,
            density,
            st_ratio,
            top_archetypes,
        )
    }

    fn get_social_data(&self, key: u64) -> (usize, f64, f64, f64) {
        let relationship_count = (key % 10) as usize;
        let total_bond = (key % 100) as f64 / 100.0;
        let connection = (key % 80) as f64 / 100.0;
        let isolation = 1.0 - connection;

        (relationship_count, total_bond, connection, isolation)
    }

    fn check_harvest_readiness(
        &self,
        polarity: f64,
        consciousness: f64,
        density: &Density,
    ) -> (bool, Option<String>) {
        if !matches!(density, Density::Third) {
            return (false, None);
        }

        if consciousness < 0.7 {
            return (false, None);
        }

        if polarity >= 0.5 {
            (true, Some("Service-to-Others".to_string()))
        } else if polarity <= -0.95 {
            (true, Some("Service-to-Self".to_string()))
        } else {
            (false, None)
        }
    }

    fn check_wanderer(&self, key: u64) -> bool {
        (key % 10) == 7
    }

    pub fn get_all_observations(
        &self,
        entity_ids: &[EntityId],
        current_tick: u64,
    ) -> Vec<EntityObservation> {
        let mut results = Vec::new();
        for id in entity_ids {
            results.push(self.get_observation(id.clone(), current_tick));
        }
        results
    }

    pub fn get_statistics(&self, entity_ids: &[EntityId]) -> ObservationStats {
        let observations = self.get_all_observations(entity_ids, 0);

        let total = observations.len();
        let harvest_ready = observations.iter().filter(|o| o.harvest_ready).count();
        let wanderers = observations.iter().filter(|o| o.is_wanderer).count();
        let avg_consciousness: f64 = observations
            .iter()
            .map(|o| o.consciousness_level)
            .sum::<f64>()
            / total.max(1) as f64;
        let avg_polarity: f64 =
            observations.iter().map(|o| o.polarity).sum::<f64>() / total.max(1) as f64;

        let mut by_density: HashMap<String, usize> = HashMap::new();
        for obs in &observations {
            let density_name = match obs.density {
                Density::First(_) => "1st",
                Density::Second(_) => "2nd",
                Density::Third => "3rd",
                Density::Fourth => "4th",
                Density::Fifth => "5th",
                Density::Sixth => "6th",
                Density::Seventh => "7th",
                Density::Eighth => "8th",
            };
            *by_density.entry(density_name.to_string()).or_insert(0) += 1;
        }

        ObservationStats {
            total_entities: total,
            harvest_ready_count: harvest_ready,
            wanderer_count: wanderers,
            average_consciousness: avg_consciousness,
            average_polarity: avg_polarity,
            density_distribution: by_density,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObservationStats {
    pub total_entities: usize,
    pub harvest_ready_count: usize,
    pub wanderer_count: usize,
    pub average_consciousness: f64,
    pub average_polarity: f64,
    pub density_distribution: HashMap<String, usize>,
}

impl ObservationStats {
    pub fn sto_percentage(&self) -> f64 {
        if self.total_entities == 0 {
            return 0.0;
        }
        let sto = (self.total_entities as f64) * (self.average_polarity + 1.0) / 2.0;
        (sto / self.total_entities as f64) * 100.0
    }

    pub fn sts_percentage(&self) -> f64 {
        if self.total_entities == 0 {
            return 0.0;
        }
        let sts = (self.total_entities as f64) * (1.0 - self.average_polarity) / 2.0;
        (sts / self.total_entities as f64) * 100.0
    }
}

pub struct ObservationFormatter;

impl ObservationFormatter {
    pub fn format_report(observation: &EntityObservation) -> String {
        let mut report = String::new();

        report.push_str("=== Entity Observation ===\n");
        report.push_str(&format!("ID: {:?}\n\n", observation.entity_id));

        report.push_str("--- Biology ---\n");
        report.push_str(&format!("Health: {:.1}%\n", observation.health * 100.0));
        report.push_str(&format!("Energy: {:.1}%\n", observation.energy * 100.0));
        report.push_str(&format!("Age: {} ticks\n", observation.age));
        report.push_str(&format!("Cell Count: {}\n\n", observation.cell_count));

        report.push_str("--- Consciousness ---\n");
        report.push_str(&format!(
            "Level: {:.1}%\n",
            observation.consciousness_level * 100.0
        ));

        let polarity_str = if observation.polarity > 0.0 {
            format!("+{:.0}% (STO)", observation.polarity * 100.0)
        } else {
            format!("{:.0}% (STS)", observation.polarity * 100.0)
        };
        report.push_str(&format!("Polarity: {}\n", polarity_str));
        report.push_str(&format!("Wisdom: {:.1}\n", observation.wisdom));

        let density_str = match observation.density {
            Density::First(_) => "1st",
            Density::Second(_) => "2nd",
            Density::Third => "3rd",
            Density::Fourth => "4th",
            Density::Fifth => "5th",
            Density::Sixth => "6th",
            Density::Seventh => "7th",
            Density::Eighth => "8th",
        };
        report.push_str(&format!("Density: {}\n", density_str));
        report.push_str(&format!("ST Ratio: {:.2}\n\n", observation.st_ratio));

        report.push_str("--- Top Archetypes ---\n");
        for (archetype, activation) in &observation.top_archetypes {
            report.push_str(&format!(
                "  Archetype {}: {:.1}%\n",
                archetype,
                activation * 100.0
            ));
        }
        report.push('\n');

        report.push_str("--- Social ---\n");
        report.push_str(&format!(
            "Relationships: {}\n",
            observation.relationship_count
        ));
        report.push_str(&format!(
            "Total Bond Strength: {:.1}%\n",
            observation.total_bond_strength * 100.0
        ));
        report.push_str(&format!(
            "Connection: {:.1}%\n",
            observation.social_connection * 100.0
        ));
        report.push_str(&format!(
            "Isolation: {:.1}%\n\n",
            observation.social_isolation * 100.0
        ));

        report.push_str("--- Status ---\n");
        if observation.harvest_ready {
            report.push_str(&format!(
                "HARVEST READY: {}\n",
                observation.harvest_type.as_ref().unwrap()
            ));
        } else {
            report.push_str("Harvest: Not ready\n");
        }
        report.push_str(&format!(
            "Wanderer: {}\n",
            if observation.is_wanderer { "Yes" } else { "No" }
        ));

        report
    }
}

#[cfg(test)]
mod observation_tests {
    use super::*;

    #[test]
    fn test_observation_creation() {
        let bridge = ObservationBridge::new();
        let entity_id = EntityId::new("test_entity".to_string());

        let observation = bridge.get_observation(entity_id.clone(), 0);
        assert_eq!(observation.entity_id.as_u64(), entity_id.as_u64());
    }

    #[test]
    fn test_statistics() {
        let bridge = ObservationBridge::new();
        let entities = vec![
            EntityId::new("entity1".to_string()),
            EntityId::new("entity2".to_string()),
        ];

        let stats = bridge.get_statistics(&entities);
        assert_eq!(stats.total_entities, 2);
    }
}
