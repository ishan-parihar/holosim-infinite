//! Collective Influence - Aggregation of entity choices affecting cosmic structure
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Collective entity influence on cosmic structure
//!  How do individual entity choices aggregate to affect cosmic structures?"
//!
//! KEY INSIGHT: When many entities make similar choices, they create
//! cumulative field modifications that can reshape local cosmic conditions.

use super::decision_feedback::{DecisionType, EntityDecision, FieldPerturbation};
use super::{EvolutionFeedbackConfig, FeedbackMode};
use crate::holographic_foundation::distortions::FieldState;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InfluenceAggregation {
    pub decision_type: DecisionType,
    pub total_significance: Float,
    pub entity_count: usize,
    pub average_significance: Float,
    pub collective_perturbation: FieldPerturbation,
    pub position_centroid: [Float; 3],
}

impl InfluenceAggregation {
    pub fn new(decision_type: DecisionType) -> Self {
        Self {
            decision_type,
            total_significance: 0.0,
            entity_count: 0,
            average_significance: 0.0,
            collective_perturbation: FieldPerturbation::zero(),
            position_centroid: [0.0, 0.0, 0.0],
        }
    }

    pub fn add_decision(&mut self, decision: &EntityDecision) {
        self.total_significance += decision.significance;
        self.entity_count += 1;
        self.average_significance = self.total_significance / self.entity_count as Float;

        let perturbation = decision.perturbation();
        self.collective_perturbation = self.collective_perturbation.combine(&perturbation);

        let weight = 1.0 / self.entity_count as Float;
        for i in 0..3 {
            self.position_centroid[i] =
                self.position_centroid[i] * (1.0 - weight) + decision.position[i] * weight;
        }
    }

    pub fn is_significant(&self) -> bool {
        self.entity_count >= 3 && self.average_significance > 0.1
    }

    pub fn collective_strength(&self) -> Float {
        self.total_significance * (self.entity_count as Float).sqrt() / 10.0
    }
}

pub struct CollectiveInfluence {
    aggregations: HashMap<DecisionType, InfluenceAggregation>,
    recent_decisions: Vec<EntityDecision>,
    field_modifications: FieldState,
    config: EvolutionFeedbackConfig,
    mode: FeedbackMode,
    total_influence_applied: Float,
}

impl CollectiveInfluence {
    pub fn new(config: EvolutionFeedbackConfig) -> Self {
        let mut aggregations = HashMap::new();
        for dt in DecisionType::all() {
            aggregations.insert(dt, InfluenceAggregation::new(dt));
        }

        Self {
            aggregations,
            recent_decisions: Vec::new(),
            field_modifications: FieldState::new(),
            config,
            mode: FeedbackMode::default(),
            total_influence_applied: 0.0,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(EvolutionFeedbackConfig::default())
    }

    pub fn with_mode(mut self, mode: FeedbackMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn add_decision(&mut self, decision: EntityDecision) {
        if let Some(aggregation) = self.aggregations.get_mut(&decision.decision_type) {
            aggregation.add_decision(&decision);
        }
        self.recent_decisions.push(decision);

        if self.recent_decisions.len() > 1000 {
            self.recent_decisions.remove(0);
        }
    }

    pub fn add_decisions(&mut self, decisions: &[EntityDecision]) {
        for decision in decisions {
            self.add_decision(decision.clone());
        }
    }

    pub fn compute_collective_perturbation(&self) -> FieldPerturbation {
        let mut total = FieldPerturbation::zero();

        for aggregation in self.aggregations.values() {
            if aggregation.is_significant() {
                let scaled = aggregation
                    .collective_perturbation
                    .scale(aggregation.collective_strength());
                total = total.combine(&scaled);
            }
        }

        total
    }

    pub fn apply_to_field(&mut self, field: &mut FieldState) {
        let perturbation = self.compute_collective_perturbation();

        for (i, &modulation) in perturbation.density_modulations.iter().enumerate() {
            if i < 8 {
                field.density_amplitudes[i] = field.density_amplitudes[i]
                    .with_added_magnitude(modulation * self.config.feedback_strength * 0.1);
            }
        }

        field.coherence = (field.coherence + perturbation.coherence_effect * 0.05).clamp(0.0, 1.0);
        field.energy += perturbation.energy_effect * 0.1;

        self.total_influence_applied += perturbation.coherence_effect.abs();
    }

    pub fn get_aggregation(&self, decision_type: DecisionType) -> Option<&InfluenceAggregation> {
        self.aggregations.get(&decision_type)
    }

    pub fn dominant_influence(&self) -> Option<DecisionType> {
        self.aggregations
            .values()
            .filter(|a| a.is_significant())
            .max_by(|a, b| {
                a.collective_strength()
                    .partial_cmp(&b.collective_strength())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|a| a.decision_type)
    }

    pub fn total_entity_count(&self) -> usize {
        self.aggregations.values().map(|a| a.entity_count).sum()
    }

    pub fn total_significance(&self) -> Float {
        self.aggregations
            .values()
            .map(|a| a.total_significance)
            .sum()
    }

    pub fn polarity_balance(&self) -> Float {
        let sto_count = self
            .aggregations
            .get(&DecisionType::Service)
            .map(|a| a.entity_count)
            .unwrap_or(0);
        let sts_count = self
            .aggregations
            .get(&DecisionType::Control)
            .map(|a| a.entity_count)
            .unwrap_or(0);
        let connection_count = self
            .aggregations
            .get(&DecisionType::Connection)
            .map(|a| a.entity_count)
            .unwrap_or(0);
        let isolation_count = self
            .aggregations
            .get(&DecisionType::Isolation)
            .map(|a| a.entity_count)
            .unwrap_or(0);

        let positive = (sto_count + connection_count) as Float;
        let negative = (sts_count + isolation_count) as Float;

        if positive + negative > 0.0 {
            (positive - negative) / (positive + negative)
        } else {
            0.0
        }
    }

    pub fn reset_aggregations(&mut self) {
        for dt in DecisionType::all() {
            self.aggregations.insert(dt, InfluenceAggregation::new(dt));
        }
        self.recent_decisions.clear();
    }

    pub fn total_influence_applied(&self) -> Float {
        self.total_influence_applied
    }

    pub fn get_field_modifications(&self) -> &FieldState {
        &self.field_modifications
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_influence_aggregation_creation() {
        let agg = InfluenceAggregation::new(DecisionType::Growth);
        assert_eq!(agg.entity_count, 0);
        assert_eq!(agg.total_significance, 0.0);
    }

    #[test]
    fn test_aggregation_add_decision() {
        let mut agg = InfluenceAggregation::new(DecisionType::Growth);
        let decision = EntityDecision::new(1, DecisionType::Growth, 0.5, [1.0, 2.0, 3.0]);

        agg.add_decision(&decision);

        assert_eq!(agg.entity_count, 1);
        assert_eq!(agg.total_significance, 0.5);
    }

    #[test]
    fn test_aggregation_significance() {
        let mut agg = InfluenceAggregation::new(DecisionType::Growth);

        for i in 0..3 {
            agg.add_decision(&EntityDecision::new(
                i,
                DecisionType::Growth,
                0.5,
                [0.0, 0.0, 0.0],
            ));
        }

        assert!(agg.is_significant());
    }

    #[test]
    fn test_collective_influence_creation() {
        let influence = CollectiveInfluence::with_defaults();
        assert_eq!(influence.total_entity_count(), 0);
    }

    #[test]
    fn test_add_decisions_to_influence() {
        let mut influence = CollectiveInfluence::with_defaults();

        influence.add_decision(EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        ));
        influence.add_decision(EntityDecision::new(
            2,
            DecisionType::Service,
            0.7,
            [1.0, 0.0, 0.0],
        ));

        assert_eq!(influence.total_entity_count(), 2);
    }

    #[test]
    fn test_polarity_balance() {
        let mut influence = CollectiveInfluence::with_defaults();

        for i in 0..5 {
            influence.add_decision(EntityDecision::new(
                i,
                DecisionType::Service,
                0.5,
                [0.0, 0.0, 0.0],
            ));
        }
        for i in 5..7 {
            influence.add_decision(EntityDecision::new(
                i,
                DecisionType::Control,
                0.5,
                [0.0, 0.0, 0.0],
            ));
        }

        let balance = influence.polarity_balance();
        assert!(balance > 0.0);
    }

    #[test]
    fn test_dominant_influence() {
        let mut influence = CollectiveInfluence::with_defaults();

        for i in 0..10 {
            influence.add_decision(EntityDecision::new(
                i,
                DecisionType::Service,
                0.7,
                [0.0, 0.0, 0.0],
            ));
        }
        for i in 10..12 {
            influence.add_decision(EntityDecision::new(
                i,
                DecisionType::Growth,
                0.5,
                [0.0, 0.0, 0.0],
            ));
        }

        let dominant = influence.dominant_influence();
        assert_eq!(dominant, Some(DecisionType::Service));
    }

    #[test]
    fn test_collective_perturbation() {
        let mut influence = CollectiveInfluence::with_defaults();

        for i in 0..5 {
            influence.add_decision(EntityDecision::new(
                i,
                DecisionType::Growth,
                0.6,
                [0.0, 0.0, 0.0],
            ));
        }

        let perturbation = influence.compute_collective_perturbation();

        assert!(perturbation.coherence_effect > 0.0);
    }

    #[test]
    fn test_collective_strength_calculation() {
        let mut agg = InfluenceAggregation::new(DecisionType::Growth);

        for i in 0..4 {
            agg.add_decision(&EntityDecision::new(
                i,
                DecisionType::Growth,
                0.5,
                [0.0, 0.0, 0.0],
            ));
        }

        let strength = agg.collective_strength();
        assert!(strength > 0.0);
    }
}
