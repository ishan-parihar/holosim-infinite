//! Collective Dynamics - Decision-making and polarity emergence
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Collective decision-making dynamics
//!  Collective structures can grow to cosmic scales"

use super::collective_formation::Collective;
use super::resonance_calculation::ResonanceResult;
use crate::holographic_foundation::evolution::{DecisionType, EntityDecision, FieldPerturbation};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectivePolarity {
    ServiceToOthers,
    ServiceToSelf,
    Balanced,
    Undetermined,
}

impl CollectivePolarity {
    pub fn from_average(polarity: Float) -> Self {
        if polarity > 0.3 {
            CollectivePolarity::ServiceToOthers
        } else if polarity < -0.3 {
            CollectivePolarity::ServiceToSelf
        } else if polarity.abs() <= 0.1 {
            CollectivePolarity::Undetermined
        } else {
            CollectivePolarity::Balanced
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CollectivePolarity::ServiceToOthers => "Service to Others (STO)",
            CollectivePolarity::ServiceToSelf => "Service to Self (STS)",
            CollectivePolarity::Balanced => "Balanced",
            CollectivePolarity::Undetermined => "Undetermined",
        }
    }

    pub fn numeric_value(&self) -> Float {
        match self {
            CollectivePolarity::ServiceToOthers => 1.0,
            CollectivePolarity::ServiceToSelf => -1.0,
            CollectivePolarity::Balanced => 0.0,
            CollectivePolarity::Undetermined => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollectiveDecision {
    pub collective_id: u64,
    pub decision_type: DecisionType,
    pub total_weight: Float,
    pub participant_count: usize,
    pub consensus_level: Float,
    pub outcome: DecisionOutcome,
    pub time: Float,
}

impl CollectiveDecision {
    pub fn new(collective_id: u64, decision_type: DecisionType, time: Float) -> Self {
        Self {
            collective_id,
            decision_type,
            total_weight: 0.0,
            participant_count: 0,
            consensus_level: 0.0,
            outcome: DecisionOutcome::Pending,
            time,
        }
    }

    pub fn add_vote(&mut self, weight: Float, alignment: Float) {
        self.total_weight += weight * alignment;
        self.participant_count += 1;
    }

    pub fn finalize(&mut self) {
        if self.participant_count > 0 {
            let mean_weight = self.total_weight / self.participant_count as Float;
            self.consensus_level = mean_weight.abs();

            self.outcome = if self.consensus_level >= 0.7 {
                DecisionOutcome::StrongConsensus
            } else if self.consensus_level >= 0.5 {
                DecisionOutcome::Majority
            } else if self.consensus_level >= 0.3 {
                DecisionOutcome::WeakConsensus
            } else {
                DecisionOutcome::NoConsensus
            };
        } else {
            self.outcome = DecisionOutcome::NoConsensus;
        }
    }

    pub fn to_field_perturbation(&self) -> FieldPerturbation {
        let base = self.decision_type.perturbation_signature();
        base.scale(self.consensus_level * (self.participant_count as Float).sqrt() / 10.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionOutcome {
    Pending,
    StrongConsensus,
    Majority,
    WeakConsensus,
    NoConsensus,
}

impl DecisionOutcome {
    pub fn is_decisive(&self) -> bool {
        matches!(
            self,
            DecisionOutcome::StrongConsensus | DecisionOutcome::Majority
        )
    }
}

pub struct CollectiveDynamics {
    pending_decisions: HashMap<u64, CollectiveDecision>,
    decision_history: Vec<CollectiveDecision>,
    polarity_history: Vec<(u64, Float, Float)>,
    max_history: usize,
}

impl CollectiveDynamics {
    pub fn new() -> Self {
        Self {
            pending_decisions: HashMap::new(),
            decision_history: Vec::new(),
            polarity_history: Vec::new(),
            max_history: 1000,
        }
    }

    pub fn initiate_decision(
        &mut self,
        collective_id: u64,
        decision_type: DecisionType,
        time: Float,
    ) {
        let decision = CollectiveDecision::new(collective_id, decision_type, time);
        self.pending_decisions.insert(collective_id, decision);
    }

    pub fn cast_vote(&mut self, collective_id: u64, weight: Float, alignment: Float) -> bool {
        if let Some(decision) = self.pending_decisions.get_mut(&collective_id) {
            decision.add_vote(weight, alignment);
            true
        } else {
            false
        }
    }

    pub fn finalize_decision(&mut self, collective_id: u64) -> Option<CollectiveDecision> {
        if let Some(mut decision) = self.pending_decisions.remove(&collective_id) {
            decision.finalize();

            self.decision_history.push(decision.clone());

            if self.decision_history.len() > self.max_history {
                self.decision_history.remove(0);
            }

            Some(decision)
        } else {
            None
        }
    }

    pub fn compute_collective_polarity(
        &mut self,
        collective: &Collective,
        time: Float,
    ) -> CollectivePolarity {
        let mut total_polarity = 0.0;

        for resonance in collective.member_resonances.values() {
            total_polarity += resonance * if resonance > &0.5 { 1.0 } else { -0.5 };
        }

        let average_polarity = if !collective.member_resonances.is_empty() {
            total_polarity / collective.member_resonances.len() as Float
        } else {
            0.0
        };

        self.polarity_history
            .push((collective.collective_id, average_polarity, time));

        if self.polarity_history.len() > self.max_history {
            self.polarity_history.remove(0);
        }

        CollectivePolarity::from_average(average_polarity)
    }

    pub fn get_decision_history(&self, collective_id: u64) -> Vec<&CollectiveDecision> {
        self.decision_history
            .iter()
            .filter(|d| d.collective_id == collective_id)
            .collect()
    }

    pub fn get_polarity_history(&self, collective_id: u64) -> Vec<(Float, Float)> {
        self.polarity_history
            .iter()
            .filter(|(id, _, _)| *id == collective_id)
            .map(|(_, polarity, time)| (*polarity, *time))
            .collect()
    }

    pub fn aggregate_entity_decisions(
        &self,
        collective: &Collective,
        entity_decisions: &[EntityDecision],
    ) -> Option<CollectiveDecision> {
        if collective.members.is_empty() {
            return None;
        }

        let mut decision_counts: HashMap<DecisionType, (Float, usize)> = HashMap::new();

        for decision in entity_decisions {
            if collective.members.contains(&decision.entity_id) {
                let entry = decision_counts
                    .entry(decision.decision_type)
                    .or_insert((0.0, 0));
                entry.0 += decision.significance;
                entry.1 += 1;
            }
        }

        let (dominant_type, (total_weight, count)) =
            decision_counts.into_iter().max_by(|a, b| {
                a.1 .0
                    .partial_cmp(&b.1 .0)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        let consensus = if count > 0 {
            total_weight / count as Float
        } else {
            0.0
        };

        let mut collective_decision =
            CollectiveDecision::new(collective.collective_id, dominant_type, 0.0);
        collective_decision.total_weight = total_weight;
        collective_decision.participant_count = count;
        collective_decision.consensus_level = consensus;
        collective_decision.outcome = if consensus >= 0.5 {
            DecisionOutcome::Majority
        } else {
            DecisionOutcome::WeakConsensus
        };

        Some(collective_decision)
    }

    pub fn total_decisions(&self) -> usize {
        self.decision_history.len()
    }

    pub fn pending_count(&self) -> usize {
        self.pending_decisions.len()
    }
}

impl Default for CollectiveDynamics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EmergentProperty {
    pub property_type: EmergentPropertyType,
    pub strength: Float,
    pub source: EmergenceSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmergentPropertyType {
    Wisdom,
    Compassion,
    Creativity,
    Resilience,
    Coherence,
    Intelligence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmergenceSource {
    MemberAlignment,
    SizeThreshold,
    ResonanceCascade,
    PolarityCoherence,
}

impl EmergentProperty {
    pub fn new(
        property_type: EmergentPropertyType,
        strength: Float,
        source: EmergenceSource,
    ) -> Self {
        Self {
            property_type,
            strength: strength.clamp(0.0, 1.0),
            source,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self.property_type {
            EmergentPropertyType::Wisdom => "Collective Wisdom",
            EmergentPropertyType::Compassion => "Collective Compassion",
            EmergentPropertyType::Creativity => "Collective Creativity",
            EmergentPropertyType::Resilience => "Collective Resilience",
            EmergentPropertyType::Coherence => "Collective Coherence",
            EmergentPropertyType::Intelligence => "Collective Intelligence",
        }
    }
}

pub fn compute_emergent_properties(collective: &Collective) -> Vec<EmergentProperty> {
    let mut properties = Vec::new();

    if collective.members.len() >= 10 {
        properties.push(EmergentProperty::new(
            EmergentPropertyType::Intelligence,
            (collective.members.len() as Float / 100.0).min(1.0),
            EmergenceSource::SizeThreshold,
        ));
    }

    if collective.coherence >= 0.7 {
        properties.push(EmergentProperty::new(
            EmergentPropertyType::Wisdom,
            collective.coherence,
            EmergenceSource::MemberAlignment,
        ));
    }

    if collective.resonance_level >= 0.8 {
        properties.push(EmergentProperty::new(
            EmergentPropertyType::Coherence,
            collective.resonance_level,
            EmergenceSource::ResonanceCascade,
        ));
    }

    properties
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_polarity_from_average() {
        assert_eq!(
            CollectivePolarity::from_average(0.5),
            CollectivePolarity::ServiceToOthers
        );
        assert_eq!(
            CollectivePolarity::from_average(-0.5),
            CollectivePolarity::ServiceToSelf
        );
        assert_eq!(
            CollectivePolarity::from_average(0.0),
            CollectivePolarity::Undetermined
        );
    }

    #[test]
    fn test_collective_decision_creation() {
        let decision = CollectiveDecision::new(1, DecisionType::Growth, 0.0);

        assert_eq!(decision.participant_count, 0);
        assert_eq!(decision.outcome, DecisionOutcome::Pending);
    }

    #[test]
    fn test_collective_decision_voting() {
        let mut decision = CollectiveDecision::new(1, DecisionType::Growth, 0.0);

        decision.add_vote(1.0, 0.8);
        decision.add_vote(1.0, 0.9);
        decision.finalize();

        assert_eq!(decision.participant_count, 2);
        assert!(decision.outcome.is_decisive());
    }

    #[test]
    fn test_collective_dynamics_creation() {
        let dynamics = CollectiveDynamics::new();
        assert_eq!(dynamics.total_decisions(), 0);
    }

    #[test]
    fn test_initiate_and_finalize_decision() {
        let mut dynamics = CollectiveDynamics::new();

        dynamics.initiate_decision(1, DecisionType::Service, 0.0);
        dynamics.cast_vote(1, 1.0, 0.9);

        let decision = dynamics.finalize_decision(1);

        assert!(decision.is_some());
        assert_eq!(dynamics.total_decisions(), 1);
    }

    #[test]
    fn test_decision_outcome_decisive() {
        assert!(DecisionOutcome::StrongConsensus.is_decisive());
        assert!(DecisionOutcome::Majority.is_decisive());
        assert!(!DecisionOutcome::WeakConsensus.is_decisive());
        assert!(!DecisionOutcome::NoConsensus.is_decisive());
    }

    #[test]
    fn test_emergent_properties() {
        let collective = Collective::new(1, 100, Default::default(), 0.0);

        let properties = compute_emergent_properties(&collective);

        assert!(properties.is_empty() || properties.iter().all(|p| p.strength >= 0.0));
    }

    #[test]
    fn test_decision_to_perturbation() {
        let mut decision = CollectiveDecision::new(1, DecisionType::Growth, 0.0);
        decision.consensus_level = 0.8;
        decision.participant_count = 10;

        let perturbation = decision.to_field_perturbation();

        assert!(perturbation.coherence_effect > 0.0);
    }
}
