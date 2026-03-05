//! Organs as Field Nodes with Archetype Resonance
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each organ as field node with archetype resonance - organs are NOT just
//!  anatomical structures, but field configurations with specific archetype patterns."
//!
//! # Key Insight
//!
//! Organs are specialized field nodes:
//! - Each organ type has a dominant archetype pattern
//! - Organ health = field coherence at that node
//! - Organ function = field dynamics at that resolution

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganId(pub u64);

impl OrganId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganType {
    Brain,
    Heart,
    Lungs,
    Liver,
    Stomach,
    Intestines,
    Kidneys,
    Pancreas,
    Spleen,
    Bladder,
    Skin,
    Muscles,
    Bones,
    Blood,
    LymphNodes,
    Thyroid,
    Adrenals,
    Gonads,
}

impl OrganType {
    pub fn dominant_archetype(&self) -> usize {
        match self {
            OrganType::Brain => 0,
            OrganType::Heart => 2,
            OrganType::Lungs => 6,
            OrganType::Liver => 3,
            OrganType::Stomach => 4,
            OrganType::Intestines => 4,
            OrganType::Kidneys => 5,
            OrganType::Pancreas => 2,
            OrganType::Spleen => 5,
            OrganType::Bladder => 11,
            OrganType::Skin => 8,
            OrganType::Muscles => 9,
            OrganType::Bones => 10,
            OrganType::Blood => 2,
            OrganType::LymphNodes => 5,
            OrganType::Thyroid => 2,
            OrganType::Adrenals => 2,
            OrganType::Gonads => 7,
        }
    }

    pub fn archetype_pattern(&self) -> [Float; NUM_ARCHETYPES] {
        let mut pattern = [0.5; NUM_ARCHETYPES];
        let dominant = self.dominant_archetype();

        pattern[dominant] = 0.9;

        for (i, item) in pattern[0..7].iter_mut().enumerate() {
            if i != dominant % 7 {
                *item = 0.6 + (dominant as Float * 0.02);
            }
        }
        for i in 7..14 {
            pattern[i] = pattern[i - 7] * 0.8;
        }
        for i in 14..21 {
            pattern[i] = pattern[i - 14] * 0.7;
        }
        pattern[21] = 0.5;

        pattern
    }

    pub fn organ_system(&self) -> &'static str {
        match self {
            OrganType::Brain => "nervous",
            OrganType::Heart | OrganType::Blood => "circulatory",
            OrganType::Lungs => "respiratory",
            OrganType::Liver | OrganType::Stomach | OrganType::Intestines | OrganType::Pancreas => {
                "digestive"
            }
            OrganType::Kidneys | OrganType::Bladder => "excretory",
            OrganType::Spleen | OrganType::LymphNodes => "immune",
            OrganType::Skin => "integumentary",
            OrganType::Muscles => "muscular",
            OrganType::Bones => "skeletal",
            OrganType::Thyroid | OrganType::Adrenals => "endocrine",
            OrganType::Gonads => "reproductive",
        }
    }

    pub fn vital(&self) -> bool {
        matches!(
            self,
            OrganType::Brain
                | OrganType::Heart
                | OrganType::Lungs
                | OrganType::Liver
                | OrganType::Kidneys
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganState {
    Healthy,
    Stressed,
    Inflamed,
    Damaged,
    Failing,
    Regenerating,
}

impl OrganState {
    pub fn from_coherence(coherence: Float) -> Self {
        if coherence > 0.85 {
            OrganState::Healthy
        } else if coherence > 0.7 {
            OrganState::Stressed
        } else if coherence > 0.5 {
            OrganState::Inflamed
        } else if coherence > 0.3 {
            OrganState::Damaged
        } else if coherence > 0.15 {
            OrganState::Failing
        } else {
            OrganState::Regenerating
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrganVitality {
    pub coherence: Float,
    pub energy_level: Float,
    pub blood_flow: Float,
    pub oxygenation: Float,
    pub nutrient_level: Float,
    pub waste_accumulation: Float,
}

impl Default for OrganVitality {
    fn default() -> Self {
        Self {
            coherence: 0.9,
            energy_level: 0.8,
            blood_flow: 0.85,
            oxygenation: 0.9,
            nutrient_level: 0.8,
            waste_accumulation: 0.1,
        }
    }
}

impl OrganVitality {
    pub fn overall_health(&self) -> Float {
        let positive = (self.coherence
            + self.energy_level
            + self.blood_flow
            + self.oxygenation
            + self.nutrient_level)
            / 5.0;
        let negative = self.waste_accumulation;
        (positive - negative * 0.5).clamp(0.0, 1.0)
    }

    pub fn is_critical(&self) -> bool {
        self.overall_health() < 0.3
    }
}

#[derive(Debug, Clone)]
pub struct OrganHealth {
    state: OrganState,
    vitality: OrganVitality,
    damage_accumulated: Float,
    regeneration_rate: Float,
    archetype_alignment: Float,
}

impl OrganHealth {
    pub fn new() -> Self {
        Self {
            state: OrganState::Healthy,
            vitality: OrganVitality::default(),
            damage_accumulated: 0.0,
            regeneration_rate: 0.01,
            archetype_alignment: 0.9,
        }
    }

    pub fn state(&self) -> OrganState {
        self.state
    }

    pub fn vitality(&self) -> &OrganVitality {
        &self.vitality
    }

    pub fn damage(&self) -> Float {
        self.damage_accumulated
    }

    pub fn alignment(&self) -> Float {
        self.archetype_alignment
    }

    pub fn apply_stress(&mut self, amount: Float) {
        self.vitality.coherence -= amount * 0.1;
        self.vitality.energy_level -= amount * 0.05;
        self.damage_accumulated += amount * 0.01;
        self.state = OrganState::from_coherence(self.vitality.coherence);
    }

    pub fn regenerate(&mut self, dt: Float) {
        let regen = self.regeneration_rate * dt;
        self.vitality.coherence = (self.vitality.coherence + regen).min(1.0);
        self.vitality.energy_level = (self.vitality.energy_level + regen * 0.5).min(1.0);
        self.damage_accumulated = (self.damage_accumulated - regen).max(0.0);
        self.state = OrganState::from_coherence(self.vitality.coherence);
    }

    pub fn heal(&mut self, amount: Float) {
        self.vitality.coherence = (self.vitality.coherence + amount).min(1.0);
        self.damage_accumulated = (self.damage_accumulated - amount * 0.5).max(0.0);
        self.state = OrganState::from_coherence(self.vitality.coherence);
    }
}

impl Default for OrganHealth {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct OrganFieldNode {
    pub id: OrganId,
    pub organ_type: OrganType,
    pub position: Position3D,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub health: OrganHealth,
    pub connected_organs: Vec<OrganId>,
    pub field_strength: Float,
    pub resonance_frequency: Float,
}

impl OrganFieldNode {
    pub fn new(organ_type: OrganType, position: Position3D) -> Self {
        let archetype_pattern = organ_type.archetype_pattern();
        let field_strength = 0.8;
        let resonance_frequency = 1.0 + organ_type.dominant_archetype() as Float * 0.1;

        Self {
            id: OrganId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            organ_type,
            position,
            archetype_pattern,
            health: OrganHealth::new(),
            connected_organs: Vec::new(),
            field_strength,
            resonance_frequency,
        }
    }

    pub fn connect_to(&mut self, other_id: OrganId) {
        if !self.connected_organs.contains(&other_id) {
            self.connected_organs.push(other_id);
        }
    }

    pub fn coherence(&self) -> Float {
        self.health.vitality().coherence
    }

    pub fn resonance_with(&self, other: &OrganFieldNode) -> Float {
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

    pub fn field_output(&self) -> Float {
        self.field_strength * self.coherence()
    }

    pub fn update(&mut self, dt: Float) {
        self.health.regenerate(dt);

        let coherence = self.coherence();
        self.field_strength = 0.5 + coherence * 0.5;
    }
}

#[derive(Debug, Clone)]
pub struct Organ {
    pub node: OrganFieldNode,
    pub cells: Vec<crate::holographic_foundation::cellular_emergence::CellId>,
    pub mass: Float,
    pub metabolic_rate: Float,
}

impl Organ {
    pub fn new(organ_type: OrganType, position: Position3D, mass: Float) -> Self {
        let node = OrganFieldNode::new(organ_type, position);
        let metabolic_rate = Self::calculate_metabolic_rate(&organ_type, mass);

        Self {
            node,
            cells: Vec::new(),
            mass,
            metabolic_rate,
        }
    }

    fn calculate_metabolic_rate(organ_type: &OrganType, mass: Float) -> Float {
        let base_rate = match organ_type {
            OrganType::Brain => 20.0,
            OrganType::Heart => 10.0,
            OrganType::Liver => 15.0,
            OrganType::Kidneys => 8.0,
            OrganType::Lungs => 5.0,
            _ => 2.0,
        };
        base_rate * mass.powf(0.75)
    }

    pub fn id(&self) -> OrganId {
        self.node.id
    }

    pub fn organ_type(&self) -> OrganType {
        self.node.organ_type
    }

    pub fn coherence(&self) -> Float {
        self.node.coherence()
    }

    pub fn health(&self) -> &OrganHealth {
        &self.node.health
    }

    pub fn health_mut(&mut self) -> &mut OrganHealth {
        &mut self.node.health
    }

    pub fn mass(&self) -> Float {
        self.mass
    }

    pub fn metabolic_demand(&self) -> Float {
        self.metabolic_rate * self.coherence()
    }

    pub fn update(&mut self, dt: Float) {
        self.node.update(dt);
        self.metabolic_rate = Self::calculate_metabolic_rate(&self.node.organ_type, self.mass);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organ_id_creation() {
        let id = OrganId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_organ_type_dominant_archetype() {
        assert_eq!(OrganType::Brain.dominant_archetype(), 0);
        assert_eq!(OrganType::Heart.dominant_archetype(), 2);
        assert_eq!(OrganType::Lungs.dominant_archetype(), 6);
    }

    #[test]
    fn test_organ_type_archetype_pattern() {
        let pattern = OrganType::Brain.archetype_pattern();
        assert!(pattern[0] > 0.8);
    }

    #[test]
    fn test_organ_type_vital() {
        assert!(OrganType::Brain.vital());
        assert!(OrganType::Heart.vital());
        assert!(!OrganType::Skin.vital());
    }

    #[test]
    fn test_organ_state_from_coherence() {
        assert_eq!(OrganState::from_coherence(0.9), OrganState::Healthy);
        assert_eq!(OrganState::from_coherence(0.6), OrganState::Inflamed);
        assert_eq!(OrganState::from_coherence(0.2), OrganState::Failing);
    }

    #[test]
    fn test_organ_vitality_default() {
        let vit = OrganVitality::default();
        assert!(vit.coherence > 0.8);
    }

    #[test]
    fn test_organ_vitality_overall_health() {
        let vit = OrganVitality::default();
        assert!(vit.overall_health() > 0.7);
    }

    #[test]
    fn test_organ_health_new() {
        let health = OrganHealth::new();
        assert_eq!(health.state(), OrganState::Healthy);
    }

    #[test]
    fn test_organ_health_apply_stress() {
        let mut health = OrganHealth::new();
        health.apply_stress(0.5);
        assert!(health.vitality().coherence < 0.9);
    }

    #[test]
    fn test_organ_field_node_creation() {
        let node = OrganFieldNode::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0));
        assert_eq!(node.organ_type, OrganType::Heart);
    }

    #[test]
    fn test_organ_field_node_resonance() {
        let node1 = OrganFieldNode::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0));
        let node2 = OrganFieldNode::new(OrganType::Brain, Position3D::new(0.0, 0.0, 0.0));
        let resonance = node1.resonance_with(&node2);
        assert!((0.0..=1.0).contains(&resonance));
    }

    #[test]
    fn test_organ_creation() {
        let organ = Organ::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0), 0.3);
        assert_eq!(organ.organ_type(), OrganType::Heart);
        assert!(organ.mass() > 0.0);
    }

    #[test]
    fn test_organ_metabolic_demand() {
        let organ = Organ::new(OrganType::Brain, Position3D::new(0.0, 0.0, 0.0), 1.0);
        assert!(organ.metabolic_demand() > 0.0);
    }

    #[test]
    fn test_organ_connection() {
        let mut node = OrganFieldNode::new(OrganType::Heart, Position3D::new(0.0, 0.0, 0.0));
        node.connect_to(OrganId::new(1));
        assert!(node.connected_organs.contains(&OrganId::new(1)));
    }
}
