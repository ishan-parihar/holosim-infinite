//! Entity Emergence System - Field to Entity Pipeline
//!
//! This module manages the emergence of entities from the holographic field.
//! It creates field structure that enables entity extraction and scales to 10,000+ entities.

use super::cosmic_sequence::CosmologicalLayer;
use super::field_state::{Complex, DensityBand, Float, HolographicFieldState, OctreeNode};
use std::collections::HashMap;

/// Entity emergence configuration
#[derive(Debug, Clone)]
pub struct EntityEmergenceConfig {
    pub target_entity_count: usize,
    pub min_separation: Float,
    pub structure_scale: Float,
    pub multi_scale: bool,
    pub birth_rate: Float,
    pub death_rate: Float,
}

impl Default for EntityEmergenceConfig {
    fn default() -> Self {
        EntityEmergenceConfig {
            target_entity_count: 10000,
            min_separation: 5.0,
            structure_scale: 50.0,
            multi_scale: true,
            birth_rate: 0.01,
            death_rate: 0.001,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmergingEntity {
    pub id: usize,
    pub position: [Float; 3],
    pub target_position: [Float; 3],
    pub density_band: DensityBand,
    pub consciousness: Float,
    pub energy: Float,
    pub active: bool,
    pub age: usize,
}

impl EmergingEntity {
    pub fn new(id: usize, position: [Float; 3]) -> Self {
        EmergingEntity {
            id,
            position,
            target_position: position,
            density_band: DensityBand::Green,
            consciousness: 0.5,
            energy: 0.5,
            active: true,
            age: 0,
        }
    }

    pub fn update_position(&mut self, _dt: Float) {
        let speed = 0.1;
        self.position[0] += (self.target_position[0] - self.position[0]) * speed;
        self.position[1] += (self.target_position[1] - self.position[1]) * speed;
        self.position[2] += (self.target_position[2] - self.position[2]) * speed;
        self.age += 1;
    }
}

pub struct EntityEmergence {
    config: EntityEmergenceConfig,
    entities: HashMap<usize, EmergingEntity>,
    next_id: usize,
    structure_nodes: Vec<FieldStructureNode>,
    pub statistics: EntityEmergenceStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct EntityEmergenceStatistics {
    pub active_entity_count: usize,
    pub total_entity_count: usize,
    pub births: usize,
    pub deaths: usize,
    pub average_consciousness: Float,
    pub density_distribution: [usize; 8],
}

#[derive(Debug, Clone)]
pub struct FieldStructureNode {
    pub position: [Float; 3],
    pub energy: Float,
    pub coherence: Float,
    pub density_band: DensityBand,
    pub layer: CosmologicalLayer,
}

impl EntityEmergence {
    pub fn new(config: EntityEmergenceConfig) -> Self {
        EntityEmergence {
            config: config.clone(),
            entities: HashMap::new(),
            next_id: 0,
            structure_nodes: Vec::new(),
            statistics: EntityEmergenceStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(EntityEmergenceConfig::default())
    }

    pub fn initialize(&mut self, count: usize) {
        self.entities.clear();
        self.next_id = 0;
        let count = count.min(self.config.target_entity_count);

        for i in 0..count {
            let position = self.generate_entity_position(i, count);
            let entity = EmergingEntity::new(self.next_id, position);
            self.entities.insert(self.next_id, entity);
            self.next_id += 1;
        }

        self.create_field_structure();
        self.statistics.active_entity_count = self.entities.len();
        self.statistics.total_entity_count = self.entities.len();
    }

    fn generate_entity_position(&self, index: usize, total: usize) -> [Float; 3] {
        let golden_ratio = 1.618033988749895;
        let i = index as Float;
        let n = total as Float;
        let y = 1.0 - (2.0 * i + 1.0) / n;
        let radius = (1.0 - y * y).sqrt();
        let theta = 2.0 * std::f64::consts::PI * i / golden_ratio;
        let scale = self.config.structure_scale * 3.0;

        [
            radius * theta.cos() as Float * scale,
            y * scale,
            radius * theta.sin() as Float * scale,
        ]
    }

    fn create_field_structure(&mut self) {
        self.structure_nodes.clear();

        for entity in self.entities.values() {
            self.structure_nodes.push(FieldStructureNode {
                position: entity.position,
                energy: entity.energy,
                coherence: entity.consciousness,
                density_band: entity.density_band,
                layer: CosmologicalLayer::Layer7,
            });
        }

        if self.config.multi_scale {
            self.add_multi_scale_structure();
        }
    }

    fn add_multi_scale_structure(&mut self) {
        let coarse_count = self.entities.len() / 8;
        for i in 0..coarse_count {
            let position = self.generate_entity_position(i * 8, self.entities.len());
            self.structure_nodes.push(FieldStructureNode {
                position,
                energy: 0.3,
                coherence: 0.4,
                density_band: DensityBand::Yellow,
                layer: CosmologicalLayer::Orange,
            });
        }
    }

    pub fn apply_to_field(&self, field: &mut HolographicFieldState) {
        for node in &self.structure_nodes {
            self.apply_node_to_field(field, node);
        }
    }

    fn apply_node_to_field(&self, field: &mut HolographicFieldState, node: &FieldStructureNode) {
        let density_idx = node.density_band.index();

        // Add amplitude
        let current = field.root.field_data.density_amplitudes[density_idx];
        let add_amp = Complex::new(node.energy * 0.1, node.energy * 0.1);
        field.root.field_data.density_amplitudes[density_idx] = current.add(&add_amp);

        // Update coherence
        field.root.field_data.coherence =
            (field.root.field_data.coherence + node.coherence * 0.01).min(1.0);

        // Update energy
        field.root.field_data.energy += node.energy * 0.01;
    }

    pub fn update(&mut self, dt: Float) {
        for entity in self.entities.values_mut() {
            entity.update_position(dt);
        }

        if self.entities.len() < self.config.target_entity_count {
            if rand::random::<Float>() < self.config.birth_rate {
                self.spawn_entity();
            }
        }

        self.handle_deaths();
        self.update_statistics();
        self.create_field_structure();
    }

    fn spawn_entity(&mut self) {
        let position = self.generate_entity_position(self.next_id, self.config.target_entity_count);
        let entity = EmergingEntity::new(self.next_id, position);
        self.entities.insert(self.next_id, entity);
        self.next_id += 1;
        self.statistics.births += 1;
    }

    fn handle_deaths(&mut self) {
        let to_remove: Vec<usize> = self
            .entities
            .iter()
            .filter(|(_, e)| !e.active || rand::random::<Float>() < self.config.death_rate)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            self.entities.remove(&id);
            self.statistics.deaths += 1;
        }
    }

    fn update_statistics(&mut self) {
        self.statistics.active_entity_count = self.entities.len();

        let mut total_consciousness = 0.0;
        let mut density_counts = [0usize; 8];

        for entity in self.entities.values() {
            total_consciousness += entity.consciousness;
            density_counts[entity.density_band.index()] += 1;
        }

        let count = self.entities.len() as Float;
        if count > 0.0 {
            self.statistics.average_consciousness = total_consciousness / count;
        }
        self.statistics.density_distribution = density_counts;
    }

    pub fn get_entity_positions(&self) -> Vec<[Float; 3]> {
        self.entities
            .values()
            .filter(|e| e.active)
            .map(|e| e.position)
            .collect()
    }

    pub fn get_entities_for_rendering(&self) -> Vec<EntityRenderData> {
        self.entities
            .values()
            .filter(|e| e.active)
            .map(|e| EntityRenderData {
                id: e.id,
                position: e.position,
                density_band: e.density_band,
                consciousness: e.consciousness,
                energy: e.energy,
            })
            .collect()
    }

    pub fn get_statistics(&self) -> EntityEmergenceStatistics {
        self.statistics.clone()
    }
}

#[derive(Debug, Clone)]
pub struct EntityRenderData {
    pub id: usize,
    pub position: [Float; 3],
    pub density_band: DensityBand,
    pub consciousness: Float,
    pub energy: Float,
}
