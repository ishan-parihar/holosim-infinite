//! Terrain-Type Metabolism Coupling
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Terrain Interaction = Field energy exchange with landscape field"
//!
//! # Key Insight
//!
//! Terrain types have different field resonance patterns that affect
//! entity metabolism and consciousness. Different terrains provide
//! different field energy signatures.

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Ocean,
    Coast,
    Wetland,
    Forest,
    Grassland,
    Savanna,
    Desert,
    Tundra,
    Mountain,
    Volcanic,
    Cave,
    Urban,
    Ruins,
    Sacred,
}

impl TerrainType {
    pub fn field_coherence(&self) -> Float {
        match self {
            TerrainType::Sacred => 0.95,
            TerrainType::Forest => 0.85,
            TerrainType::Ocean => 0.82,
            TerrainType::Wetland => 0.80,
            TerrainType::Grassland => 0.75,
            TerrainType::Mountain => 0.72,
            TerrainType::Savanna => 0.70,
            TerrainType::Coast => 0.68,
            TerrainType::Tundra => 0.65,
            TerrainType::Cave => 0.60,
            TerrainType::Urban => 0.45,
            TerrainType::Desert => 0.40,
            TerrainType::Volcanic => 0.35,
            TerrainType::Ruins => 0.30,
        }
    }

    pub fn metabolic_modifier(&self) -> Float {
        match self {
            TerrainType::Ocean => 0.9,
            TerrainType::Coast => 1.0,
            TerrainType::Wetland => 1.1,
            TerrainType::Forest => 1.2,
            TerrainType::Grassland => 1.1,
            TerrainType::Savanna => 1.0,
            TerrainType::Desert => 0.7,
            TerrainType::Tundra => 0.8,
            TerrainType::Mountain => 0.9,
            TerrainType::Volcanic => 0.5,
            TerrainType::Cave => 0.85,
            TerrainType::Urban => 0.95,
            TerrainType::Ruins => 0.9,
            TerrainType::Sacred => 1.5,
        }
    }

    pub fn consciousness_modifier(&self) -> Float {
        match self {
            TerrainType::Sacred => 1.5,
            TerrainType::Forest => 1.2,
            TerrainType::Ocean => 1.15,
            TerrainType::Mountain => 1.1,
            TerrainType::Cave => 1.05,
            TerrainType::Wetland => 1.0,
            TerrainType::Grassland => 1.0,
            TerrainType::Savanna => 0.95,
            TerrainType::Coast => 1.0,
            TerrainType::Tundra => 0.9,
            TerrainType::Urban => 0.85,
            TerrainType::Desert => 0.8,
            TerrainType::Volcanic => 0.7,
            TerrainType::Ruins => 0.75,
        }
    }

    pub fn energy_renewal_rate(&self) -> Float {
        match self {
            TerrainType::Sacred => 0.2,
            TerrainType::Forest => 0.15,
            TerrainType::Wetland => 0.12,
            TerrainType::Ocean => 0.1,
            TerrainType::Grassland => 0.08,
            TerrainType::Savanna => 0.06,
            TerrainType::Coast => 0.09,
            TerrainType::Mountain => 0.05,
            TerrainType::Tundra => 0.04,
            TerrainType::Cave => 0.03,
            TerrainType::Urban => 0.07,
            TerrainType::Desert => 0.02,
            TerrainType::Volcanic => 0.01,
            TerrainType::Ruins => 0.05,
        }
    }

    pub fn dominant_archetype(&self) -> usize {
        match self {
            TerrainType::Ocean => 2,
            TerrainType::Coast => 2,
            TerrainType::Wetland => 3,
            TerrainType::Forest => 4,
            TerrainType::Grassland => 4,
            TerrainType::Savanna => 4,
            TerrainType::Desert => 6,
            TerrainType::Tundra => 6,
            TerrainType::Mountain => 10,
            TerrainType::Volcanic => 5,
            TerrainType::Cave => 11,
            TerrainType::Urban => 0,
            TerrainType::Ruins => 11,
            TerrainType::Sacred => 0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TerrainType::Ocean => "Deep waters with strong field coherence",
            TerrainType::Coast => "Transition zone between land and sea",
            TerrainType::Wetland => "Water-saturated lands of high fertility",
            TerrainType::Forest => "Dense vegetation with rich field patterns",
            TerrainType::Grassland => "Open fields with moderate coherence",
            TerrainType::Savanna => "Tropical grasslands with scattered trees",
            TerrainType::Desert => "Arid lands with sparse field energy",
            TerrainType::Tundra => "Frozen plains with dormant fields",
            TerrainType::Mountain => "Elevated terrain with thin field density",
            TerrainType::Volcanic => "Geologically active, chaotic field energy",
            TerrainType::Cave => "Underground spaces with concentrated fields",
            TerrainType::Urban => "Settlement areas with fragmented fields",
            TerrainType::Ruins => "Abandoned places with residual field patterns",
            TerrainType::Sacred => "Sites of high spiritual field density",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerrainResonance {
    pub terrain_type: TerrainType,
    pub resonance_frequency: Float,
    pub resonance_amplitude: Float,
    pub phase: Float,
    pub coherence: Float,
}

impl TerrainResonance {
    pub fn new(terrain_type: TerrainType) -> Self {
        let coherence = terrain_type.field_coherence();
        let resonance_frequency = 1.0 + terrain_type.dominant_archetype() as Float * 0.1;

        Self {
            terrain_type,
            resonance_frequency,
            resonance_amplitude: coherence,
            phase: 0.0,
            coherence,
        }
    }

    pub fn resonance_strength(&self) -> Float {
        self.resonance_amplitude * self.coherence
    }

    pub fn update(&mut self, dt: Float) {
        self.phase += self.resonance_frequency * dt;
        self.resonance_amplitude =
            self.terrain_type.field_coherence() * (1.0 + 0.1 * (self.phase * 2.0).sin());
    }

    pub fn entity_coupling(&self, entity_phase: Float) -> Float {
        let phase_diff = (self.phase - entity_phase).abs();
        let normalized = if phase_diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - phase_diff
        } else {
            phase_diff
        };
        (1.0 - normalized / std::f64::consts::PI) * self.resonance_strength()
    }
}

#[derive(Debug, Clone)]
pub struct TerrainInfluence {
    pub terrain_type: TerrainType,
    pub influence_radius: Float,
    pub influence_strength: Float,
    pub affected_entities: HashMap<u64, Float>,
    pub field_gradient: Float,
}

impl TerrainInfluence {
    pub fn new(terrain_type: TerrainType, radius: Float) -> Self {
        Self {
            terrain_type,
            influence_radius: radius,
            influence_strength: terrain_type.field_coherence(),
            affected_entities: HashMap::new(),
            field_gradient: 0.0,
        }
    }

    pub fn calculate_influence_at(&self, distance: Float) -> Float {
        if distance > self.influence_radius {
            return 0.0;
        }

        let falloff = 1.0 - (distance / self.influence_radius).powi(2);
        self.influence_strength * falloff
    }

    pub fn add_affected_entity(&mut self, entity_id: u64, distance: Float) {
        let influence = self.calculate_influence_at(distance);
        if influence > 0.0 {
            self.affected_entities.insert(entity_id, influence);
        }
    }

    pub fn remove_affected_entity(&mut self, entity_id: &u64) {
        self.affected_entities.remove(entity_id);
    }

    pub fn total_influence(&self) -> Float {
        self.affected_entities.values().sum()
    }

    pub fn average_influence(&self) -> Float {
        if self.affected_entities.is_empty() {
            return 0.0;
        }
        self.total_influence() / self.affected_entities.len() as Float
    }
}

#[derive(Debug, Clone)]
pub struct TerrainMetabolismCoupling {
    pub entity_id: u64,
    pub current_terrain: TerrainType,
    pub terrain_resonance: TerrainResonance,
    pub metabolic_rate: Float,
    pub consciousness_modifier: Float,
    pub energy_renewal: Float,
    pub adaptation_level: Float,
    pub terrain_history: Vec<(TerrainType, Float)>,
}

impl TerrainMetabolismCoupling {
    pub fn new(entity_id: u64, terrain_type: TerrainType) -> Self {
        let terrain_resonance = TerrainResonance::new(terrain_type);

        Self {
            entity_id,
            current_terrain: terrain_type,
            terrain_resonance,
            metabolic_rate: terrain_type.metabolic_modifier(),
            consciousness_modifier: terrain_type.consciousness_modifier(),
            energy_renewal: terrain_type.energy_renewal_rate(),
            adaptation_level: 0.5,
            terrain_history: vec![(terrain_type, 0.0)],
        }
    }

    pub fn update(&mut self, dt: Float, entity_phase: Float) {
        self.terrain_resonance.update(dt);

        let coupling = self.terrain_resonance.entity_coupling(entity_phase);
        self.metabolic_rate = self.current_terrain.metabolic_modifier()
            * (1.0 + coupling * self.adaptation_level * 0.2);

        self.consciousness_modifier =
            self.current_terrain.consciousness_modifier() * (1.0 + coupling * 0.1);

        self.adaptation_level = (self.adaptation_level + 0.001 * dt).min(1.0);
    }

    pub fn change_terrain(&mut self, new_terrain: TerrainType, time: Float) {
        self.current_terrain = new_terrain;
        self.terrain_resonance = TerrainResonance::new(new_terrain);
        self.metabolic_rate = new_terrain.metabolic_modifier() * 0.8;
        self.consciousness_modifier = new_terrain.consciousness_modifier() * 0.9;
        self.energy_renewal = new_terrain.energy_renewal_rate();
        self.adaptation_level *= 0.5;
        self.terrain_history.push((new_terrain, time));
    }

    pub fn energy_from_terrain(&self, dt: Float) -> Float {
        self.energy_renewal * self.adaptation_level * dt
    }

    pub fn consciousness_boost(&self) -> Float {
        self.consciousness_modifier * self.adaptation_level
    }

    pub fn terrain_preference(&self) -> Option<TerrainType> {
        self.terrain_history
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(t, _)| *t)
    }

    pub fn time_in_current_terrain(&self, current_time: Float) -> Float {
        if let Some(last) = self.terrain_history.last() {
            current_time - last.1
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_type_coherence() {
        assert!(TerrainType::Sacred.field_coherence() > TerrainType::Desert.field_coherence());
    }

    #[test]
    fn test_terrain_type_metabolic_modifier() {
        assert!(
            TerrainType::Forest.metabolic_modifier() > TerrainType::Desert.metabolic_modifier()
        );
    }

    #[test]
    fn test_terrain_type_consciousness_modifier() {
        assert!(TerrainType::Sacred.consciousness_modifier() > 1.0);
    }

    #[test]
    fn test_terrain_resonance_creation() {
        let resonance = TerrainResonance::new(TerrainType::Forest);
        assert!(resonance.resonance_strength() > 0.0);
    }

    #[test]
    fn test_terrain_resonance_update() {
        let mut resonance = TerrainResonance::new(TerrainType::Forest);
        resonance.update(1.0);
        assert!(resonance.phase > 0.0);
    }

    #[test]
    fn test_terrain_resonance_entity_coupling() {
        let resonance = TerrainResonance::new(TerrainType::Forest);
        let coupling = resonance.entity_coupling(0.0);
        assert!(coupling >= 0.0 && coupling <= 1.0);
    }

    #[test]
    fn test_terrain_influence_creation() {
        let influence = TerrainInfluence::new(TerrainType::Forest, 100.0);
        assert_eq!(influence.terrain_type, TerrainType::Forest);
        assert_eq!(influence.influence_radius, 100.0);
    }

    #[test]
    fn test_terrain_influence_calculate() {
        let influence = TerrainInfluence::new(TerrainType::Forest, 100.0);
        let at_center = influence.calculate_influence_at(0.0);
        let at_edge = influence.calculate_influence_at(100.0);
        let outside = influence.calculate_influence_at(150.0);

        assert!(at_center > at_edge);
        assert_eq!(outside, 0.0);
    }

    #[test]
    fn test_terrain_influence_add_entity() {
        let mut influence = TerrainInfluence::new(TerrainType::Forest, 100.0);
        influence.add_affected_entity(1, 50.0);
        assert_eq!(influence.affected_entities.len(), 1);
    }

    #[test]
    fn test_terrain_metabolism_coupling_creation() {
        let coupling = TerrainMetabolismCoupling::new(1, TerrainType::Forest);
        assert_eq!(coupling.entity_id, 1);
        assert!(coupling.metabolic_rate > 0.0);
    }

    #[test]
    fn test_terrain_metabolism_coupling_update() {
        let mut coupling = TerrainMetabolismCoupling::new(1, TerrainType::Forest);
        coupling.update(1.0, 0.5);
        assert!(coupling.terrain_resonance.phase > 0.0);
    }

    #[test]
    fn test_terrain_metabolism_coupling_change() {
        let mut coupling = TerrainMetabolismCoupling::new(1, TerrainType::Forest);
        coupling.change_terrain(TerrainType::Desert, 1.0);

        assert_eq!(coupling.current_terrain, TerrainType::Desert);
        assert!(coupling.adaptation_level < 0.5);
    }

    #[test]
    fn test_terrain_metabolism_energy() {
        let coupling = TerrainMetabolismCoupling::new(1, TerrainType::Forest);
        let energy = coupling.energy_from_terrain(1.0);
        assert!(energy > 0.0);
    }

    #[test]
    fn test_terrain_metabolism_consciousness_boost() {
        let coupling = TerrainMetabolismCoupling::new(1, TerrainType::Sacred);
        let boost = coupling.consciousness_boost();
        assert!(boost > 0.0);
    }

    #[test]
    fn test_terrain_dominant_archetype() {
        assert_eq!(TerrainType::Forest.dominant_archetype(), 4);
        assert_eq!(TerrainType::Ocean.dominant_archetype(), 2);
    }
}
