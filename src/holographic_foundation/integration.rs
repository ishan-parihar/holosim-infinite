//! Holographic Foundation Integration
//!
//! This module provides the unified entry point for the Phase 0 holographic foundation,
//! integrating UniversalTemplate, HolographicFieldState, and the MERA compression system.
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The holographic principle IS compression - self-similarity enables efficient storage.
//!  Emergence IS performance - procedural generation avoids storing pre-defined content.
//!  Science and optimization are one - implementing holographic science gives efficiency 'for free'."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Reality is not constructed; it is Unfolded from a Pre-Existing Whole.
//!  The Entity does not 'become' something it was not—it REMEMBERS what it always was."

use std::sync::Arc;

use super::field_state::{CoherencePeak, EntityPotential, HolographicFieldState, Position3D};
use super::scale_level::{PhysicsMode, ScaleLevel};
use super::template::{
    Density, HolographicFieldReference, TemplateConfig, TemplateFactory, UniversalTemplate,
};
use crate::types::Float;

#[derive(Debug, Clone)]
pub struct FoundationConfig {
    pub field_size: Float,
    pub initial_coherence: Float,
    pub subdivision_threshold: Float,
    pub max_depth: u8,
}

impl Default for FoundationConfig {
    fn default() -> Self {
        Self {
            field_size: 1.0,
            initial_coherence: 0.5,
            subdivision_threshold: 0.1,
            max_depth: 12,
        }
    }
}

pub struct HolographicFoundation {
    field_state: HolographicFieldState,
    template_factory: TemplateFactory,
    config: FoundationConfig,
    entity_potentials: Vec<EntityPotential>,
    resonance_network: ResonanceNetwork,
}

impl HolographicFoundation {
    pub fn new(config: FoundationConfig) -> Self {
        let field_state = HolographicFieldState::new(config.field_size);
        let field_ref = Arc::new(HolographicFieldReference::new(
            1,
            config.initial_coherence,
            ScaleLevel::Biological,
        ));
        let template_factory = TemplateFactory::new(field_ref);

        Self {
            field_state,
            template_factory,
            config,
            entity_potentials: Vec::new(),
            resonance_network: ResonanceNetwork::new(),
        }
    }

    pub fn unity() -> Self {
        let config = FoundationConfig {
            initial_coherence: 1.0,
            ..Default::default()
        };
        let mut foundation = Self::new(config);
        foundation.field_state = HolographicFieldState::unity();
        foundation
    }

    pub fn field_state(&self) -> &HolographicFieldState {
        &self.field_state
    }

    pub fn field_state_mut(&mut self) -> &mut HolographicFieldState {
        &mut self.field_state
    }

    pub fn global_coherence(&self) -> Float {
        self.field_state.global_coherence()
    }

    pub fn evolve(&mut self, dt: Float) {
        self.field_state.evolve(dt);
        self.update_entity_potentials();
        self.resonance_network.update_resonances();
    }

    fn update_entity_potentials(&mut self) {
        let peaks = self
            .field_state
            .extract_coherence_peaks(self.config.subdivision_threshold);
        self.entity_potentials = peaks
            .iter()
            .map(|peak| peak.to_entity_potential())
            .collect();
    }

    pub fn entity_potentials(&self) -> &[EntityPotential] {
        &self.entity_potentials
    }

    pub fn entity_count(&self) -> usize {
        self.entity_potentials.len()
    }

    pub fn create_template<T: Clone + 'static>(
        &mut self,
        config: &TemplateConfig,
        component_data: T,
    ) -> UniversalTemplate<T> {
        self.template_factory.instantiate(config, component_data)
    }

    pub fn derive_archetype_at(&self, position: &Position3D) -> Option<[Float; 22]> {
        self.field_state
            .get_node_at(position)
            .map(|node| node.archetype_vector)
    }

    pub fn derive_density_at(&self, position: &Position3D) -> Option<Density> {
        self.field_state.get_node_at(position).map(|node| {
            let scale = node.dominant_scale();
            match scale {
                ScaleLevel::Quantum => Density::First(1),
                ScaleLevel::Atomic => Density::First(2),
                ScaleLevel::Molecular => Density::First(3),
                ScaleLevel::Cellular => Density::Second(1),
                ScaleLevel::Biological => Density::Third,
                ScaleLevel::Planetary => Density::Fourth,
                ScaleLevel::Stellar => Density::Fifth,
                ScaleLevel::Cosmic => Density::Eighth,
            }
        })
    }

    pub fn field_statistics(&self) -> FieldStatistics {
        self.field_state.field_statistics()
    }

    pub fn resonance_network(&self) -> &ResonanceNetwork {
        &self.resonance_network
    }

    pub fn compute_resonance(&self, pos_a: &Position3D, pos_b: &Position3D) -> Float {
        let node_a = match self.field_state.get_node_at(pos_a) {
            Some(n) => n,
            None => return 0.0,
        };
        let node_b = match self.field_state.get_node_at(pos_b) {
            Some(n) => n,
            None => return 0.0,
        };

        let mut resonance = 1.0;

        for i in 0..22 {
            let diff = (node_a.archetype_vector[i] - node_b.archetype_vector[i]).abs();
            resonance *= 1.0 - diff;
        }

        resonance *= node_a.coherence * node_b.coherence;

        let scale_alignment = if node_a.dominant_scale() == node_b.dominant_scale() {
            1.2
        } else {
            0.8
        };

        resonance * scale_alignment
    }

    pub fn find_resonant_pairs(&self, threshold: Float) -> Vec<(Position3D, Position3D, Float)> {
        let peaks = self.field_state.extract_coherence_peaks(threshold);
        let mut pairs = Vec::new();

        for i in 0..peaks.len() {
            for j in (i + 1)..peaks.len() {
                let resonance = self.compute_resonance(&peaks[i].position, &peaks[j].position);
                if resonance >= threshold {
                    pairs.push((peaks[i].position, peaks[j].position, resonance));
                }
            }
        }

        pairs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        pairs
    }

    pub fn apply_free_will_perturbation(
        &mut self,
        position: &Position3D,
        archetype_shift: [Float; 22],
        magnitude: Float,
    ) {
        if let Some(node) = self.field_state.get_or_create_node_mut(position) {
            for (node_i, &shift_i) in node.archetype_vector.iter_mut().zip(archetype_shift.iter()) {
                *node_i = (*node_i + shift_i * magnitude).clamp(0.0, 1.0);
            }
            node.recalculate_coherence();
        }
    }

    pub fn extract_view_at_scale(&self, scale: ScaleLevel) -> ScaleView {
        let peaks = self.field_state.extract_coherence_peaks(0.1);
        let mut scale_peaks = Vec::new();

        for peak in peaks {
            if peak.dominant_scale == scale {
                scale_peaks.push(peak);
            }
        }

        ScaleView {
            scale,
            peaks: scale_peaks,
            physics_mode: scale.physics_mode(),
        }
    }

    pub fn simultaneous_emergence_check(&self) -> Vec<SimultaneousEmergence> {
        let mut emergences = Vec::new();
        let peaks = self.field_state.extract_coherence_peaks(0.5);

        let mut by_density: std::collections::HashMap<u8, Vec<&CoherencePeak>> =
            std::collections::HashMap::new();

        for peak in &peaks {
            let density = peak.dominant_scale.to_density();
            by_density.entry(density).or_default().push(peak);
        }

        for (density, density_peaks) in by_density {
            if density_peaks.len() > 1 {
                let scales: std::collections::HashSet<ScaleLevel> =
                    density_peaks.iter().map(|p| p.dominant_scale).collect();

                if scales.len() > 1 {
                    emergences.push(SimultaneousEmergence {
                        density,
                        scales: scales.into_iter().collect(),
                        positions: density_peaks.iter().map(|p| p.position).collect(),
                    });
                }
            }
        }

        emergences
    }
}

impl std::fmt::Debug for HolographicFoundation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HolographicFoundation")
            .field("config", &self.config)
            .field("entity_count", &self.entity_count())
            .field("global_coherence", &self.global_coherence())
            .finish()
    }
}

pub struct ResonanceNetwork {
    connections: Vec<ResonanceConnection>,
    total_resonance: Float,
}

impl ResonanceNetwork {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            total_resonance: 0.0,
        }
    }

    pub fn add_connection(&mut self, from: Position3D, to: Position3D, strength: Float) {
        self.connections
            .push(ResonanceConnection { from, to, strength });
        self.total_resonance += strength;
    }

    pub fn connections(&self) -> &[ResonanceConnection] {
        &self.connections
    }

    pub fn total_resonance(&self) -> Float {
        self.total_resonance
    }

    pub fn average_resonance(&self) -> Float {
        if self.connections.is_empty() {
            0.0
        } else {
            self.total_resonance / self.connections.len() as Float
        }
    }

    pub fn clear(&mut self) {
        self.connections.clear();
        self.total_resonance = 0.0;
    }

    fn update_resonances(&mut self) {
        for connection in &mut self.connections {
            connection.strength *= 0.99;
        }
        self.connections.retain(|c| c.strength > 0.01);
        self.total_resonance = self.connections.iter().map(|c| c.strength).sum();
    }
}

impl Default for ResonanceNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ResonanceConnection {
    pub from: Position3D,
    pub to: Position3D,
    pub strength: Float,
}

#[derive(Debug, Clone)]
pub struct ScaleView {
    pub scale: ScaleLevel,
    pub peaks: Vec<CoherencePeak>,
    pub physics_mode: PhysicsMode,
}

impl ScaleView {
    pub fn entity_count(&self) -> usize {
        self.peaks.len()
    }

    pub fn average_coherence(&self) -> Float {
        if self.peaks.is_empty() {
            return 0.0;
        }
        self.peaks.iter().map(|p| p.coherence).sum::<Float>() / self.peaks.len() as Float
    }

    pub fn total_amplitude(&self) -> Float {
        self.peaks.iter().map(|p| p.amplitude).sum()
    }
}

#[derive(Debug, Clone)]
pub struct SimultaneousEmergence {
    pub density: u8,
    pub scales: Vec<ScaleLevel>,
    pub positions: Vec<Position3D>,
}

impl SimultaneousEmergence {
    pub fn description(&self) -> String {
        match self.density {
            1 => format!(
                "1st Density: {} atoms emerging with {} galaxies",
                self.positions.len() / 2,
                self.positions.len() / 2
            ),
            2 => format!(
                "2nd Density: {} cells emerging with Gaia consciousness",
                self.positions.len()
            ),
            3 => format!(
                "3rd Density: {} organisms emerging with {} societies",
                self.positions.len() / 2,
                self.positions.len() / 2
            ),
            _ => format!(
                "{}th Density: {} simultaneous manifestations",
                self.density,
                self.positions.len()
            ),
        }
    }
}

pub use super::field_state::FieldStatistics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_creation() {
        let foundation = HolographicFoundation::new(FoundationConfig::default());
        assert_eq!(foundation.entity_count(), 0);
    }

    #[test]
    fn test_unity_foundation() {
        let foundation = HolographicFoundation::unity();
        assert!((foundation.global_coherence() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_evolution() {
        let mut foundation = HolographicFoundation::new(FoundationConfig::default());
        let initial_coherence = foundation.global_coherence();
        foundation.evolve(1.0);
        assert!(foundation.global_coherence() != initial_coherence || initial_coherence == 0.0);
    }

    #[test]
    fn test_resonance_calculation() {
        let foundation = HolographicFoundation::new(FoundationConfig {
            initial_coherence: 0.8,
            ..Default::default()
        });

        let pos_a = Position3D::new(0.5, 0.5, 0.5);
        let pos_b = Position3D::new(0.5, 0.5, 0.5);

        let resonance = foundation.compute_resonance(&pos_a, &pos_b);
        assert!(resonance > 0.0);
    }

    #[test]
    fn test_free_will_perturbation() {
        let mut foundation = HolographicFoundation::new(FoundationConfig::default());
        let pos = Position3D::new(0.5, 0.5, 0.5);

        let shift = [0.1; 22];
        foundation.apply_free_will_perturbation(&pos, shift, 1.0);

        let archetype = foundation.derive_archetype_at(&pos);
        assert!(archetype.is_some());
    }

    #[test]
    fn test_scale_view() {
        let foundation = HolographicFoundation::new(FoundationConfig::default());
        let view = foundation.extract_view_at_scale(ScaleLevel::Biological);
        assert_eq!(view.scale, ScaleLevel::Biological);
        assert_eq!(view.physics_mode, PhysicsMode::SpaceTime);
    }

    #[test]
    fn test_resonance_network() {
        let mut network = ResonanceNetwork::new();
        network.add_connection(Position3D::zero(), Position3D::new(1.0, 1.0, 1.0), 0.8);

        assert_eq!(network.connections().len(), 1);
        assert!((network.total_resonance() - 0.8).abs() < 1e-10);
    }
}
