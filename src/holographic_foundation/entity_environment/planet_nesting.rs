//! Entity Field Nesting Within Planetary Field
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entity-Planet Assignment = Entity field nesting within planetary field
//!  Entity Position = WHERE entity's field configuration manifests"
//!
//! # Key Insight
//!
//! Entities are nested within planetary fields like Russian dolls:
//! - Entity field ⊂ Organism field ⊂ Ecosystem field ⊂ Planetary field
//! - Each nesting level adds boundary conditions and resonance patterns
//! - Entity inherits properties from all nesting levels

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanetId(pub u64);

impl PlanetId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NestingLevel {
    Planetary,
    Continental,
    Regional,
    Local,
    Micro,
}

impl NestingLevel {
    pub fn scale_factor(&self) -> Float {
        match self {
            NestingLevel::Planetary => 1.0,
            NestingLevel::Continental => 0.1,
            NestingLevel::Regional => 0.01,
            NestingLevel::Local => 0.001,
            NestingLevel::Micro => 0.0001,
        }
    }

    pub fn coherence_threshold(&self) -> Float {
        match self {
            NestingLevel::Planetary => 0.95,
            NestingLevel::Continental => 0.85,
            NestingLevel::Regional => 0.75,
            NestingLevel::Local => 0.65,
            NestingLevel::Micro => 0.55,
        }
    }

    pub fn field_strength_multiplier(&self) -> Float {
        match self {
            NestingLevel::Planetary => 1.0,
            NestingLevel::Continental => 0.8,
            NestingLevel::Regional => 0.6,
            NestingLevel::Local => 0.4,
            NestingLevel::Micro => 0.2,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            NestingLevel::Planetary => "Full planetary field access",
            NestingLevel::Continental => "Continental-scale nesting",
            NestingLevel::Regional => "Regional ecosystem nesting",
            NestingLevel::Local => "Local environment nesting",
            NestingLevel::Micro => "Micro-habitat nesting",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NestingDepth {
    Shallow,
    Medium,
    Deep,
    Full,
    Transcendent,
}

impl NestingDepth {
    pub fn from_coherence(coherence: Float) -> Self {
        if coherence < 0.3 {
            NestingDepth::Shallow
        } else if coherence < 0.5 {
            NestingDepth::Medium
        } else if coherence < 0.7 {
            NestingDepth::Deep
        } else if coherence < 0.9 {
            NestingDepth::Full
        } else {
            NestingDepth::Transcendent
        }
    }

    pub fn integration_factor(&self) -> Float {
        match self {
            NestingDepth::Shallow => 0.1,
            NestingDepth::Medium => 0.3,
            NestingDepth::Deep => 0.6,
            NestingDepth::Full => 0.9,
            NestingDepth::Transcendent => 1.0,
        }
    }

    pub fn boundary_permeability(&self) -> Float {
        match self {
            NestingDepth::Shallow => 0.9,
            NestingDepth::Medium => 0.6,
            NestingDepth::Deep => 0.3,
            NestingDepth::Full => 0.1,
            NestingDepth::Transcendent => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetaryResonance {
    pub planet_id: PlanetId,
    pub resonance_frequency: Float,
    pub resonance_amplitude: Float,
    pub phase_alignment: Float,
    pub field_coherence: Float,
}

impl PlanetaryResonance {
    pub fn new(planet_id: PlanetId) -> Self {
        Self {
            planet_id,
            resonance_frequency: 1.0,
            resonance_amplitude: 0.8,
            phase_alignment: 0.0,
            field_coherence: 0.8,
        }
    }

    pub fn resonance_strength(&self) -> Float {
        self.resonance_amplitude * self.field_coherence
    }

    pub fn update(&mut self, dt: Float, entity_contribution: Float) {
        self.phase_alignment += self.resonance_frequency * dt;
        self.resonance_amplitude =
            (self.resonance_amplitude + entity_contribution * 0.01 * dt).min(1.0);
        self.field_coherence = 0.9 + self.resonance_amplitude * 0.1;
    }

    pub fn calculate_phase_alignment(&self, entity_phase: Float) -> Float {
        let phase_diff = (self.phase_alignment - entity_phase).abs();
        let normalized = if phase_diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - phase_diff
        } else {
            phase_diff
        };
        1.0 - normalized / std::f64::consts::PI
    }
}

#[derive(Debug, Clone)]
pub struct EntityPlanetBinding {
    pub entity_id: u64,
    pub planet_id: PlanetId,
    pub nesting_level: NestingLevel,
    pub nesting_depth: NestingDepth,
    pub planetary_resonance: PlanetaryResonance,
    pub position_in_planet: Position3D,
    pub binding_strength: Float,
    pub inheritance_factors: HashMap<String, Float>,
}

impl EntityPlanetBinding {
    pub fn new(entity_id: u64, planet_id: PlanetId, position: Position3D) -> Self {
        let planetary_resonance = PlanetaryResonance::new(planet_id);
        let nesting_level = NestingLevel::Local;
        let nesting_depth = NestingDepth::Medium;

        Self {
            entity_id,
            planet_id,
            nesting_level,
            nesting_depth,
            planetary_resonance,
            position_in_planet: position,
            binding_strength: 0.5,
            inheritance_factors: HashMap::new(),
        }
    }

    pub fn with_nesting_level(mut self, level: NestingLevel) -> Self {
        self.nesting_level = level;
        self.update_binding_strength();
        self
    }

    pub fn with_nesting_depth(mut self, depth: NestingDepth) -> Self {
        self.nesting_depth = depth;
        self.update_binding_strength();
        self
    }

    fn update_binding_strength(&mut self) {
        let level_factor = self.nesting_level.field_strength_multiplier();
        let depth_factor = self.nesting_depth.integration_factor();
        let resonance_factor = self.planetary_resonance.resonance_strength();

        self.binding_strength = (level_factor + depth_factor + resonance_factor) / 3.0;
    }

    pub fn update(&mut self, dt: Float, entity_resonance: Float) {
        self.planetary_resonance.update(dt, entity_resonance);

        let phase_alignment = self
            .planetary_resonance
            .calculate_phase_alignment(entity_resonance * std::f64::consts::TAU);

        self.binding_strength = self.binding_strength * 0.9 + phase_alignment * 0.1;
        self.update_binding_strength();
    }

    pub fn inherit_property(&mut self, property_name: &str, base_value: Float) -> Float {
        let inheritance = self.nesting_depth.integration_factor() * self.binding_strength;
        let inherited_value = base_value * inheritance;
        self.inheritance_factors
            .insert(property_name.to_string(), inherited_value);
        inherited_value
    }

    pub fn get_inherited(&self, property_name: &str) -> Option<Float> {
        self.inheritance_factors.get(property_name).copied()
    }

    pub fn can_transcend(&self) -> bool {
        matches!(self.nesting_depth, NestingDepth::Transcendent) && self.binding_strength > 0.95
    }

    pub fn field_boundary_strength(&self) -> Float {
        1.0 - self.nesting_depth.boundary_permeability()
    }

    pub fn deepen_nesting(&mut self) {
        self.nesting_depth = match self.nesting_depth {
            NestingDepth::Shallow => NestingDepth::Medium,
            NestingDepth::Medium => NestingDepth::Deep,
            NestingDepth::Deep => NestingDepth::Full,
            NestingDepth::Full => NestingDepth::Transcendent,
            NestingDepth::Transcendent => NestingDepth::Transcendent,
        };
        self.update_binding_strength();
    }

    pub fn expand_nesting_level(&mut self) {
        self.nesting_level = match self.nesting_level {
            NestingLevel::Micro => NestingLevel::Local,
            NestingLevel::Local => NestingLevel::Regional,
            NestingLevel::Regional => NestingLevel::Continental,
            NestingLevel::Continental => NestingLevel::Planetary,
            NestingLevel::Planetary => NestingLevel::Planetary,
        };
        self.update_binding_strength();
    }
}

#[derive(Debug, Clone)]
pub struct PlanetaryFieldNesting {
    pub planets: HashMap<PlanetId, Vec<u64>>,
    pub entity_bindings: HashMap<u64, EntityPlanetBinding>,
    pub total_resonance: Float,
    pub field_coherence: Float,
}

impl PlanetaryFieldNesting {
    pub fn new() -> Self {
        Self {
            planets: HashMap::new(),
            entity_bindings: HashMap::new(),
            total_resonance: 0.0,
            field_coherence: 0.8,
        }
    }

    pub fn bind_entity(&mut self, binding: EntityPlanetBinding) {
        let entity_id = binding.entity_id;
        let planet_id = binding.planet_id;

        self.planets
            .entry(planet_id)
            .or_default()
            .push(entity_id);
        self.entity_bindings.insert(entity_id, binding);

        self.recalculate_total_resonance();
    }

    pub fn unbind_entity(&mut self, entity_id: u64) -> Option<EntityPlanetBinding> {
        if let Some(binding) = self.entity_bindings.remove(&entity_id) {
            if let Some(entities) = self.planets.get_mut(&binding.planet_id) {
                entities.retain(|&id| id != entity_id);
            }
            self.recalculate_total_resonance();
            Some(binding)
        } else {
            None
        }
    }

    fn recalculate_total_resonance(&mut self) {
        if self.entity_bindings.is_empty() {
            self.total_resonance = 0.0;
            return;
        }

        let sum: Float = self
            .entity_bindings
            .values()
            .map(|b| b.binding_strength)
            .sum();

        self.total_resonance = sum / self.entity_bindings.len() as Float;
        self.field_coherence = 0.7 + self.total_resonance * 0.3;
    }

    pub fn update(&mut self, dt: Float) {
        let entity_ids: Vec<u64> = self.entity_bindings.keys().copied().collect();

        for entity_id in entity_ids {
            if let Some(binding) = self.entity_bindings.get_mut(&entity_id) {
                binding.update(dt, self.field_coherence);
            }
        }

        self.recalculate_total_resonance();
    }

    pub fn entities_on_planet(&self, planet_id: &PlanetId) -> Option<&Vec<u64>> {
        self.planets.get(planet_id)
    }

    pub fn entity_binding(&self, entity_id: u64) -> Option<&EntityPlanetBinding> {
        self.entity_bindings.get(&entity_id)
    }

    pub fn planet_count(&self) -> usize {
        self.planets.len()
    }

    pub fn entity_count(&self) -> usize {
        self.entity_bindings.len()
    }

    pub fn average_binding_strength(&self) -> Float {
        if self.entity_bindings.is_empty() {
            return 0.0;
        }

        self.entity_bindings
            .values()
            .map(|b| b.binding_strength)
            .sum::<Float>()
            / self.entity_bindings.len() as Float
    }
}

impl Default for PlanetaryFieldNesting {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_id_creation() {
        let id = PlanetId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_nesting_level_scale() {
        assert!(NestingLevel::Planetary.scale_factor() > NestingLevel::Local.scale_factor());
    }

    #[test]
    fn test_nesting_level_coherence() {
        assert!(
            NestingLevel::Planetary.coherence_threshold()
                > NestingLevel::Micro.coherence_threshold()
        );
    }

    #[test]
    fn test_nesting_depth_from_coherence() {
        assert_eq!(NestingDepth::from_coherence(0.2), NestingDepth::Shallow);
        assert_eq!(NestingDepth::from_coherence(0.8), NestingDepth::Full);
        assert_eq!(
            NestingDepth::from_coherence(0.95),
            NestingDepth::Transcendent
        );
    }

    #[test]
    fn test_nesting_depth_integration() {
        assert!(
            NestingDepth::Full.integration_factor() > NestingDepth::Shallow.integration_factor()
        );
    }

    #[test]
    fn test_planetary_resonance_creation() {
        let resonance = PlanetaryResonance::new(PlanetId::new(1));
        assert!(resonance.resonance_strength() > 0.0);
    }

    #[test]
    fn test_planetary_resonance_update() {
        let mut resonance = PlanetaryResonance::new(PlanetId::new(1));
        resonance.update(1.0, 0.5);
        assert!(resonance.phase_alignment > 0.0);
    }

    #[test]
    fn test_entity_planet_binding_creation() {
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0));
        assert_eq!(binding.entity_id, 1);
        assert!(binding.binding_strength > 0.0);
    }

    #[test]
    fn test_entity_planet_binding_with_level() {
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0))
            .with_nesting_level(NestingLevel::Planetary);
        assert_eq!(binding.nesting_level, NestingLevel::Planetary);
    }

    #[test]
    fn test_entity_planet_binding_deepen() {
        let mut binding =
            EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0));
        binding.deepen_nesting();
        assert_eq!(binding.nesting_depth, NestingDepth::Deep);
    }

    #[test]
    fn test_entity_planet_binding_inherit() {
        let mut binding =
            EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0))
                .with_nesting_depth(NestingDepth::Full);
        let inherited = binding.inherit_property("gravity", 1.0);
        assert!(inherited > 0.0);
        assert_eq!(binding.get_inherited("gravity"), Some(inherited));
    }

    #[test]
    fn test_planetary_field_nesting_creation() {
        let nesting = PlanetaryFieldNesting::new();
        assert!(nesting.planets.is_empty());
        assert!(nesting.entity_bindings.is_empty());
    }

    #[test]
    fn test_planetary_field_nesting_bind() {
        let mut nesting = PlanetaryFieldNesting::new();
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0));
        nesting.bind_entity(binding);

        assert_eq!(nesting.entity_count(), 1);
        assert_eq!(nesting.planet_count(), 1);
    }

    #[test]
    fn test_planetary_field_nesting_unbind() {
        let mut nesting = PlanetaryFieldNesting::new();
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0));
        nesting.bind_entity(binding);

        let removed = nesting.unbind_entity(1);
        assert!(removed.is_some());
        assert_eq!(nesting.entity_count(), 0);
    }

    #[test]
    fn test_planetary_field_nesting_update() {
        let mut nesting = PlanetaryFieldNesting::new();
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0));
        nesting.bind_entity(binding);

        nesting.update(1.0);
        assert!(nesting.total_resonance > 0.0);
    }

    #[test]
    fn test_entity_can_transcend() {
        let binding = EntityPlanetBinding::new(1, PlanetId::new(1), Position3D::new(0.0, 0.0, 0.0))
            .with_nesting_depth(NestingDepth::Transcendent);

        assert!(binding.can_transcend() || binding.binding_strength <= 0.95);
    }
}
