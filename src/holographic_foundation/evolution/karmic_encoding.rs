//! Karmic Pattern Field Encoding - How entity experiences encode into field patterns
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Karmic pattern field encoding"
//!
//! KEY INSIGHT: Karmic patterns are not abstract records but actual field
//! configurations that influence future probability distributions.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Law of Karma is the Law of Cause and Effect operating through the field"

use super::decision_feedback::{DecisionType, EntityDecision};
use super::{EvolutionFeedbackConfig, MIN_SIGNIFICANCE_THRESHOLD};
use crate::holographic_foundation::distortions::{DensityAmplitude, FieldState};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternType {
    Creative,
    Destructive,
    Balancing,
    Transformative,
    Repetitive,
    Breaking,
}

impl PatternType {
    pub fn from_decision_sequence(decisions: &[DecisionType]) -> Self {
        if decisions.is_empty() {
            return PatternType::Balancing;
        }

        let growth_count = decisions
            .iter()
            .filter(|&&d| d == DecisionType::Growth)
            .count();
        let service_count = decisions
            .iter()
            .filter(|&&d| d == DecisionType::Service)
            .count();
        let control_count = decisions
            .iter()
            .filter(|&&d| d == DecisionType::Control)
            .count();
        let isolation_count = decisions
            .iter()
            .filter(|&&d| d == DecisionType::Isolation)
            .count();

        let positive = growth_count + service_count;
        let negative = control_count + isolation_count;

        if positive > negative * 2 {
            PatternType::Creative
        } else if negative > positive * 2 {
            PatternType::Destructive
        } else if decisions.len() > 5 {
            let unique_count = decisions
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len();
            if unique_count == 1 {
                PatternType::Repetitive
            } else if unique_count > decisions.len() / 2 {
                PatternType::Breaking
            } else {
                PatternType::Transformative
            }
        } else {
            PatternType::Balancing
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PatternType::Creative => "Creative Pattern",
            PatternType::Destructive => "Destructive Pattern",
            PatternType::Balancing => "Balancing Pattern",
            PatternType::Transformative => "Transformative Pattern",
            PatternType::Repetitive => "Repetitive Pattern",
            PatternType::Breaking => "Pattern Breaking",
        }
    }

    pub fn karma_multiplier(&self) -> Float {
        match self {
            PatternType::Creative => 1.2,
            PatternType::Destructive => 1.5,
            PatternType::Balancing => 0.8,
            PatternType::Transformative => 1.0,
            PatternType::Repetitive => 1.3,
            PatternType::Breaking => 0.9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternSignature {
    pub density_weights: [Float; 8],
    pub phase_pattern: [Float; 8],
    pub intensity: Float,
    pub resolution_tendency: Float,
}

impl PatternSignature {
    pub fn new(density_weights: [Float; 8], phase_pattern: [Float; 8], intensity: Float) -> Self {
        Self {
            density_weights,
            phase_pattern,
            intensity: intensity.clamp(0.0, 1.0),
            resolution_tendency: 0.5,
        }
    }

    pub fn from_decisions(decisions: &[EntityDecision]) -> Self {
        if decisions.is_empty() {
            return Self::neutral();
        }

        let mut density_accum = [0.0; 8];
        let mut phase_accum = [0.0; 8];
        let mut total_significance = 0.0;

        for decision in decisions {
            let perturbation = decision.decision_type.perturbation_signature();
            for (i, &modulation) in perturbation.density_modulations.iter().enumerate() {
                density_accum[i] += modulation * decision.significance;
                phase_accum[i] += decision.phase * decision.significance;
            }
            total_significance += decision.significance;
        }

        if total_significance > 0.0 {
            for i in 0..8 {
                density_accum[i] /= total_significance;
                phase_accum[i] /= total_significance;
            }
        }

        Self::new(
            density_accum,
            phase_accum,
            total_significance / decisions.len() as Float,
        )
    }

    pub fn neutral() -> Self {
        Self::new([0.125; 8], [0.0; 8], 0.0)
    }

    pub fn is_significant(&self) -> bool {
        self.intensity >= MIN_SIGNIFICANCE_THRESHOLD
    }

    pub fn to_field_state(&self) -> FieldState {
        let mut state = FieldState::new();
        for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
            let magnitude = self.density_weights[i];
            let phase = self.phase_pattern[i];
            *amp = DensityAmplitude::from_polar(magnitude, phase);
        }
        state.coherence = self.intensity;
        state
    }

    pub fn combine(&self, other: &PatternSignature) -> Self {
        let combined_weights =
            std::array::from_fn(|i| (self.density_weights[i] + other.density_weights[i]) / 2.0);
        let combined_phases = std::array::from_fn(|i| {
            let diff = other.phase_pattern[i] - self.phase_pattern[i];
            self.phase_pattern[i] + diff / 2.0
        });

        Self::new(
            combined_weights,
            combined_phases,
            (self.intensity + other.intensity) / 2.0,
        )
    }
}

#[derive(Debug, Clone)]
pub struct KarmicPattern {
    pub pattern_id: u64,
    pub entity_id: u64,
    pub pattern_type: PatternType,
    pub signature: PatternSignature,
    pub resolution_status: Float,
    pub creation_time: Float,
    pub last_activation: Float,
    pub activation_count: usize,
    pub contributing_decisions: Vec<DecisionType>,
}

impl KarmicPattern {
    pub fn new(
        pattern_id: u64,
        entity_id: u64,
        pattern_type: PatternType,
        signature: PatternSignature,
        time: Float,
    ) -> Self {
        Self {
            pattern_id,
            entity_id,
            pattern_type,
            signature,
            resolution_status: 0.0,
            creation_time: time,
            last_activation: time,
            activation_count: 1,
            contributing_decisions: Vec::new(),
        }
    }

    pub fn from_decisions(
        pattern_id: u64,
        entity_id: u64,
        decisions: &[EntityDecision],
        time: Float,
    ) -> Self {
        let decision_types: Vec<DecisionType> = decisions.iter().map(|d| d.decision_type).collect();
        let pattern_type = PatternType::from_decision_sequence(&decision_types);
        let signature = PatternSignature::from_decisions(decisions);

        Self {
            pattern_id,
            entity_id,
            pattern_type,
            signature,
            resolution_status: 0.0,
            creation_time: time,
            last_activation: time,
            activation_count: 1,
            contributing_decisions: decision_types,
        }
    }

    pub fn activate(&mut self, time: Float, significance: Float) {
        self.last_activation = time;
        self.activation_count += 1;
        self.resolution_status = (self.resolution_status + significance * 0.1).min(1.0);
    }

    pub fn is_resolved(&self) -> bool {
        self.resolution_status >= 0.999
    }

    pub fn intensity(&self) -> Float {
        self.signature.intensity * self.pattern_type.karma_multiplier()
    }

    pub fn time_active(&self, current_time: Float) -> Float {
        current_time - self.creation_time
    }

    pub fn influence_on_field(&self) -> FieldState {
        let mut state = self.signature.to_field_state();
        let unresolved_factor = 1.0 - self.resolution_status;
        for amp in state.density_amplitudes.iter_mut() {
            *amp = amp.scale(unresolved_factor * self.intensity());
        }
        state
    }
}

pub struct KarmicEncoding {
    patterns: HashMap<u64, KarmicPattern>,
    entity_patterns: HashMap<u64, Vec<u64>>,
    next_pattern_id: u64,
    config: EvolutionFeedbackConfig,
    total_patterns_created: u64,
    total_patterns_resolved: u64,
}

impl KarmicEncoding {
    pub fn new(config: EvolutionFeedbackConfig) -> Self {
        Self {
            patterns: HashMap::new(),
            entity_patterns: HashMap::new(),
            next_pattern_id: 1,
            config,
            total_patterns_created: 0,
            total_patterns_resolved: 0,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(EvolutionFeedbackConfig::default())
    }

    pub fn create_pattern(
        &mut self,
        entity_id: u64,
        decisions: &[EntityDecision],
        time: Float,
    ) -> u64 {
        if decisions.is_empty() {
            return 0;
        }

        let pattern_id = self.next_pattern_id;
        self.next_pattern_id += 1;

        let pattern = KarmicPattern::from_decisions(pattern_id, entity_id, decisions, time);

        self.patterns.insert(pattern_id, pattern);
        self.entity_patterns
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(pattern_id);
        self.total_patterns_created += 1;

        pattern_id
    }

    pub fn get_pattern(&self, pattern_id: u64) -> Option<&KarmicPattern> {
        self.patterns.get(&pattern_id)
    }

    pub fn get_pattern_mut(&mut self, pattern_id: u64) -> Option<&mut KarmicPattern> {
        self.patterns.get_mut(&pattern_id)
    }

    pub fn get_entity_patterns(&self, entity_id: u64) -> Vec<&KarmicPattern> {
        self.entity_patterns
            .get(&entity_id)
            .map(|ids| ids.iter().filter_map(|id| self.patterns.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_unresolved_patterns(&self, entity_id: u64) -> Vec<&KarmicPattern> {
        self.get_entity_patterns(entity_id)
            .into_iter()
            .filter(|p| !p.is_resolved())
            .collect()
    }

    pub fn activate_pattern(&mut self, pattern_id: u64, time: Float, significance: Float) {
        if let Some(pattern) = self.patterns.get_mut(&pattern_id) {
            pattern.activate(time, significance);

            if pattern.is_resolved() {
                self.total_patterns_resolved += 1;
            }
        }
    }

    pub fn compute_karmic_field(&self, entity_id: u64) -> FieldState {
        let mut field = FieldState::new();

        for pattern in self.get_unresolved_patterns(entity_id) {
            let pattern_field = pattern.influence_on_field();
            for (i, amp) in field.density_amplitudes.iter_mut().enumerate() {
                *amp = amp.add(&pattern_field.density_amplitudes[i]);
            }
            field.coherence += pattern_field.coherence;
            field.energy += pattern_field.energy;
        }

        field
    }

    pub fn resolve_pattern(&mut self, pattern_id: u64) -> bool {
        if let Some(pattern) = self.patterns.get_mut(&pattern_id) {
            if !pattern.is_resolved() {
                pattern.resolution_status = 1.0;
                self.total_patterns_resolved += 1;
                return true;
            }
        }
        false
    }

    pub fn total_karmic_load(&self, entity_id: u64) -> Float {
        self.get_unresolved_patterns(entity_id)
            .iter()
            .map(|p| p.intensity())
            .sum()
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn unresolved_count(&self) -> usize {
        self.patterns.values().filter(|p| !p.is_resolved()).count()
    }

    pub fn statistics(&self) -> KarmicStatistics {
        KarmicStatistics {
            total_created: self.total_patterns_created,
            total_resolved: self.total_patterns_resolved,
            currently_active: self.unresolved_count(),
            by_type: self.count_by_type(),
        }
    }

    fn count_by_type(&self) -> HashMap<PatternType, usize> {
        let mut counts = HashMap::new();
        for pattern in self.patterns.values() {
            *counts.entry(pattern.pattern_type).or_insert(0) += 1;
        }
        counts
    }

    pub fn remove_entity(&mut self, entity_id: u64) {
        if let Some(pattern_ids) = self.entity_patterns.remove(&entity_id) {
            for id in pattern_ids {
                self.patterns.remove(&id);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct KarmicStatistics {
    pub total_created: u64,
    pub total_resolved: u64,
    pub currently_active: usize,
    pub by_type: HashMap<PatternType, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_type_from_sequence() {
        let creative = PatternType::from_decision_sequence(&[
            DecisionType::Service,
            DecisionType::Service,
            DecisionType::Service,
            DecisionType::Growth,
        ]);
        assert_eq!(creative, PatternType::Creative);

        let destructive = PatternType::from_decision_sequence(&[
            DecisionType::Control,
            DecisionType::Control,
            DecisionType::Control,
            DecisionType::Isolation,
        ]);
        assert_eq!(destructive, PatternType::Destructive);
    }

    #[test]
    fn test_pattern_signature_from_decisions() {
        let decisions = vec![
            EntityDecision::new(1, DecisionType::Growth, 0.5, [0.0, 0.0, 0.0]),
            EntityDecision::new(2, DecisionType::Service, 0.7, [1.0, 0.0, 0.0]),
        ];

        let sig = PatternSignature::from_decisions(&decisions);

        assert!(sig.intensity > 0.0);
        assert!(sig.is_significant());
    }

    #[test]
    fn test_karmic_pattern_creation() {
        let decisions = vec![EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        )];

        let pattern = KarmicPattern::from_decisions(1, 1, &decisions, 0.0);

        assert_eq!(pattern.pattern_id, 1);
        assert!(!pattern.is_resolved());
    }

    #[test]
    fn test_karmic_encoding_creation() {
        let encoding = KarmicEncoding::with_defaults();
        assert_eq!(encoding.pattern_count(), 0);
    }

    #[test]
    fn test_create_pattern() {
        let mut encoding = KarmicEncoding::with_defaults();
        let decisions = vec![EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        )];

        let pattern_id = encoding.create_pattern(1, &decisions, 0.0);

        assert!(pattern_id > 0);
        assert_eq!(encoding.pattern_count(), 1);
    }

    #[test]
    fn test_get_entity_patterns() {
        let mut encoding = KarmicEncoding::with_defaults();
        let decisions = vec![EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        )];

        encoding.create_pattern(1, &decisions, 0.0);

        let patterns = encoding.get_entity_patterns(1);
        assert_eq!(patterns.len(), 1);
    }

    #[test]
    fn test_pattern_resolution() {
        let mut encoding = KarmicEncoding::with_defaults();
        let decisions = vec![EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        )];

        let pattern_id = encoding.create_pattern(1, &decisions, 0.0);
        assert!(pattern_id > 0);

        for t in 1..=10 {
            encoding.activate_pattern(pattern_id, t as Float, 1.0);
        }

        let pattern = encoding.get_pattern(pattern_id).unwrap();
        assert!(
            pattern.is_resolved(),
            "Pattern should be resolved after 10 activations, resolution_status = {}",
            pattern.resolution_status
        );
    }

    #[test]
    fn test_karmic_field_computation() {
        let mut encoding = KarmicEncoding::with_defaults();
        let decisions = vec![EntityDecision::new(
            1,
            DecisionType::Service,
            0.8,
            [0.0, 0.0, 0.0],
        )];

        encoding.create_pattern(1, &decisions, 0.0);

        let field = encoding.compute_karmic_field(1);

        assert!(field.coherence > 0.0);
    }

    #[test]
    fn test_total_karmic_load() {
        let mut encoding = KarmicEncoding::with_defaults();

        encoding.create_pattern(
            1,
            &[EntityDecision::new(
                1,
                DecisionType::Service,
                0.8,
                [0.0, 0.0, 0.0],
            )],
            0.0,
        );
        encoding.create_pattern(
            1,
            &[EntityDecision::new(
                1,
                DecisionType::Growth,
                0.6,
                [0.0, 0.0, 0.0],
            )],
            0.0,
        );

        let load = encoding.total_karmic_load(1);
        assert!(load > 0.0);
    }

    #[test]
    fn test_karma_multiplier() {
        assert!(
            PatternType::Destructive.karma_multiplier() > PatternType::Balancing.karma_multiplier()
        );
        assert!(
            PatternType::Creative.karma_multiplier() > PatternType::Balancing.karma_multiplier()
        );
    }
}
