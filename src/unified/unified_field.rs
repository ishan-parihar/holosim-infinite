//! Unified Field - Single Field for Physics + Biology + Consciousness

use crate::hpo::field_state::HolographicFieldState;
use crate::spectrum::larson_framework::SpectrumRatio;
use std::sync::Arc;

/// The Unified Field - primary reality structure
#[derive(Debug, Clone)]
pub struct UnifiedField {
    pub id: String,
    pub holographic_state: HolographicFieldState,
    pub spectrum_ratio: SpectrumRatio,
    pub awareness_level: f64,
}

impl UnifiedField {
    pub fn new(id: String) -> Self {
        Self {
            id: id.clone(),
            holographic_state: HolographicFieldState::with_defaults(),
            spectrum_ratio: SpectrumRatio::space_time(1.0, 1.0),
            awareness_level: 0.5,
        }
    }

    pub fn sample_coherence(&self, position: [f64; 3]) -> f64 {
        self.holographic_state.root.field_data.coherence
    }

    pub fn sample_energy(&self, position: [f64; 3]) -> f64 {
        self.holographic_state.root.field_data.energy
    }

    pub fn get_archetypes(&self, position: [f64; 3]) -> [f64; 22] {
        [0.5; 22]
    }

    pub fn matter_emerges(&self, position: [f64; 3]) -> bool {
        self.sample_coherence(position) > 0.7
    }

    pub fn life_emerges(&self, position: [f64; 3]) -> bool {
        let coherence = self.sample_coherence(position);
        let energy = self.sample_energy(position);
        coherence > 0.8 && energy > 0.5
    }

    pub fn consciousness_emerges(&self, position: [f64; 3]) -> bool {
        self.sample_coherence(position) > 0.9
    }

    pub fn get_awareness(&self, position: [f64; 3]) -> f64 {
        if self.consciousness_emerges(position) {
            self.awareness_level * self.sample_coherence(position)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnifiedFieldStatistics {
    pub total_energy: f64,
    pub average_coherence: f64,
    pub matter_regions: usize,
    pub life_regions: usize,
    pub consciousness_regions: usize,
    pub active_nodes: usize,
}

impl UnifiedField {
    pub fn calculate_statistics(&self) -> UnifiedFieldStatistics {
        UnifiedFieldStatistics {
            total_energy: self.holographic_state.root.field_data.energy,
            average_coherence: self.holographic_state.root.field_data.coherence,
            matter_regions: 0,
            life_regions: 0,
            consciousness_regions: 0,
            active_nodes: self.holographic_state.active_node_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_field_creation() {
        let field = UnifiedField::new("test".to_string());
        assert_eq!(field.id, "test");
    }

    #[test]
    fn test_matter_emergence() {
        let field = UnifiedField::new("test".to_string());
        assert!(!field.matter_emerges([0.0, 0.0, 0.0]));
    }
}
