//! Decision Feedback - Entity decisions as field perturbations
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entity choice creates localized field modification
//!  Amplitude = significance, Phase = nature of decision"

use super::{
    EvolutionFeedbackConfig, FeedbackMode, MAX_DECISION_HISTORY, MIN_SIGNIFICANCE_THRESHOLD,
};
use crate::holographic_foundation::distortions::FieldState;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionType {
    Growth,
    Service,
    Control,
    Connection,
    Isolation,
    Learning,
    Expression,
    Reception,
}

impl DecisionType {
    pub fn all() -> [DecisionType; 8] {
        [
            DecisionType::Growth,
            DecisionType::Service,
            DecisionType::Control,
            DecisionType::Connection,
            DecisionType::Isolation,
            DecisionType::Learning,
            DecisionType::Expression,
            DecisionType::Reception,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DecisionType::Growth => "Growth (Seeking Evolution)",
            DecisionType::Service => "Service (Service to Others)",
            DecisionType::Control => "Control (Service to Self)",
            DecisionType::Connection => "Connection (Seeking Unity)",
            DecisionType::Isolation => "Isolation (Seeking Separation)",
            DecisionType::Learning => "Learning (Catalyst Processing)",
            DecisionType::Expression => "Expression (Creative Output)",
            DecisionType::Reception => "Reception (Accepting Input)",
        }
    }

    pub fn cosmological_description(&self) -> &'static str {
        match self {
            DecisionType::Growth => "Evolutionary impulse toward higher density",
            DecisionType::Service => "STO polarization - strengthening Green ray",
            DecisionType::Control => "STS polarization - strengthening Orange ray",
            DecisionType::Connection => "Resonance seeking - unity consciousness",
            DecisionType::Isolation => "Separation seeking - individual identity",
            DecisionType::Learning => "Catalyst processing - Yellow ray wisdom",
            DecisionType::Expression => "Creative manifestation - Orange ray energy",
            DecisionType::Reception => "Acceptance - Blue ray understanding",
        }
    }

    pub fn perturbation_signature(&self) -> FieldPerturbation {
        match self {
            DecisionType::Growth => FieldPerturbation::growth(),
            DecisionType::Service => FieldPerturbation::service(),
            DecisionType::Control => FieldPerturbation::control(),
            DecisionType::Connection => FieldPerturbation::connection(),
            DecisionType::Isolation => FieldPerturbation::isolation(),
            DecisionType::Learning => FieldPerturbation::learning(),
            DecisionType::Expression => FieldPerturbation::expression(),
            DecisionType::Reception => FieldPerturbation::reception(),
        }
    }

    pub fn polarity(&self) -> Float {
        match self {
            DecisionType::Service => 1.0,
            DecisionType::Control => -1.0,
            DecisionType::Connection => 0.5,
            DecisionType::Isolation => -0.5,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldPerturbation {
    pub density_modulations: [Float; 8],
    pub coherence_effect: Float,
    pub energy_effect: Float,
    pub spectrum_shift: Float,
}

impl FieldPerturbation {
    pub fn new(
        density_modulations: [Float; 8],
        coherence: Float,
        energy: Float,
        spectrum: Float,
    ) -> Self {
        Self {
            density_modulations,
            coherence_effect: coherence,
            energy_effect: energy,
            spectrum_shift: spectrum,
        }
    }

    pub fn growth() -> Self {
        Self::new([0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7], 0.1, 0.5, -0.05)
    }

    pub fn service() -> Self {
        Self::new([0.0, 0.0, 0.0, 0.8, 0.2, 0.1, 0.0, 0.0], 0.2, 0.3, -0.02)
    }

    pub fn control() -> Self {
        Self::new([0.0, 0.0, 0.0, 0.1, 0.2, 0.3, 0.8, 0.0], -0.1, 0.4, 0.05)
    }

    pub fn connection() -> Self {
        Self::new(
            [0.15, 0.2, 0.25, 0.3, 0.25, 0.2, 0.15, 0.1],
            0.3,
            0.2,
            -0.03,
        )
    }

    pub fn isolation() -> Self {
        Self::new([0.4, 0.3, 0.2, 0.1, 0.0, 0.0, 0.0, 0.0], -0.2, 0.1, 0.08)
    }

    pub fn learning() -> Self {
        Self::new([0.0, 0.0, 0.1, 0.2, 0.7, 0.3, 0.1, 0.0], 0.15, 0.3, -0.02)
    }

    pub fn expression() -> Self {
        Self::new([0.0, 0.0, 0.0, 0.1, 0.2, 0.7, 0.3, 0.0], 0.1, 0.4, 0.0)
    }

    pub fn reception() -> Self {
        Self::new(
            [0.1, 0.15, 0.2, 0.25, 0.2, 0.15, 0.1, 0.05],
            0.2,
            0.2,
            -0.01,
        )
    }

    pub fn zero() -> Self {
        Self::new([0.0; 8], 0.0, 0.0, 0.0)
    }

    pub fn scale(&self, factor: Float) -> Self {
        Self::new(
            self.density_modulations.map(|d| d * factor),
            self.coherence_effect * factor,
            self.energy_effect * factor,
            self.spectrum_shift * factor,
        )
    }

    pub fn combine(&self, other: &FieldPerturbation) -> Self {
        let combined_density =
            std::array::from_fn(|i| self.density_modulations[i] + other.density_modulations[i]);
        Self::new(
            combined_density,
            self.coherence_effect + other.coherence_effect,
            self.energy_effect + other.energy_effect,
            self.spectrum_shift + other.spectrum_shift,
        )
    }
}

#[derive(Debug, Clone)]
pub struct EntityDecision {
    pub entity_id: u64,
    pub decision_type: DecisionType,
    pub significance: Float,
    pub phase: Float,
    pub position: [Float; 3],
    pub time: Float,
    pub applied: bool,
}

impl EntityDecision {
    pub fn new(
        entity_id: u64,
        decision_type: DecisionType,
        significance: Float,
        position: [Float; 3],
    ) -> Self {
        Self {
            entity_id,
            decision_type,
            significance: significance.clamp(0.0, 1.0),
            phase: 0.0,
            position,
            time: 0.0,
            applied: false,
        }
    }

    pub fn with_phase(mut self, phase: Float) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_time(mut self, time: Float) -> Self {
        self.time = time;
        self
    }

    pub fn is_significant(&self) -> bool {
        self.significance >= MIN_SIGNIFICANCE_THRESHOLD
    }

    pub fn perturbation(&self) -> FieldPerturbation {
        self.decision_type
            .perturbation_signature()
            .scale(self.significance)
    }
}

pub struct DecisionFeedback {
    pending_decisions: Vec<EntityDecision>,
    decision_history: Vec<EntityDecision>,
    accumulated_perturbation: FieldPerturbation,
    config: EvolutionFeedbackConfig,
    mode: FeedbackMode,
    decision_counts: HashMap<DecisionType, usize>,
}

impl DecisionFeedback {
    pub fn new(config: EvolutionFeedbackConfig) -> Self {
        Self {
            pending_decisions: Vec::new(),
            decision_history: Vec::new(),
            accumulated_perturbation: FieldPerturbation::zero(),
            config,
            mode: FeedbackMode::default(),
            decision_counts: HashMap::new(),
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
        if decision.is_significant() {
            *self
                .decision_counts
                .entry(decision.decision_type)
                .or_insert(0) += 1;
            self.pending_decisions.push(decision.clone());
            self.decision_history.push(decision);

            if self.decision_history.len() > MAX_DECISION_HISTORY {
                self.decision_history.remove(0);
            }
        }
    }

    pub fn process_pending(&mut self) -> FieldPerturbation {
        let mut total = FieldPerturbation::zero();

        for decision in self.pending_decisions.drain(..) {
            let perturbation = decision.perturbation();
            total = total.combine(&perturbation);
            self.accumulated_perturbation = self.accumulated_perturbation.combine(&perturbation);
        }

        total
    }

    pub fn apply_to_field(&self, field: &mut FieldState, position_influence: Float) {
        for (i, &modulation) in self
            .accumulated_perturbation
            .density_modulations
            .iter()
            .enumerate()
        {
            if i < 8 {
                field.density_amplitudes[i] = field.density_amplitudes[i].with_added_magnitude(
                    modulation * self.config.feedback_strength * position_influence,
                );
            }
        }

        field.coherence = (field.coherence + self.accumulated_perturbation.coherence_effect * 0.1)
            .clamp(0.0, 1.0);
        field.energy = (field.energy + self.accumulated_perturbation.energy_effect * 0.5).max(0.0);
        field.spectrum_position = (field.spectrum_position
            + self.accumulated_perturbation.spectrum_shift * 0.01)
            .clamp(0.0, 10.0);
    }

    pub fn decay(&mut self, decay_rate: Float) {
        self.accumulated_perturbation = self.accumulated_perturbation.scale(decay_rate);
    }

    pub fn get_history(&self) -> &[EntityDecision] {
        &self.decision_history
    }

    pub fn decision_count(&self, decision_type: DecisionType) -> usize {
        self.decision_counts
            .get(&decision_type)
            .copied()
            .unwrap_or(0)
    }

    pub fn total_decisions(&self) -> usize {
        self.pending_decisions.len() + self.decision_history.len()
    }

    pub fn clear_pending(&mut self) {
        self.pending_decisions.clear();
    }

    pub fn reset(&mut self) {
        self.pending_decisions.clear();
        self.decision_history.clear();
        self.accumulated_perturbation = FieldPerturbation::zero();
        self.decision_counts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_type_signatures() {
        for dt in DecisionType::all() {
            let sig = dt.perturbation_signature();
            assert!(sig.coherence_effect.abs() <= 1.0);
            assert!(sig.energy_effect >= 0.0);
        }
    }

    #[test]
    fn test_entity_decision_creation() {
        let decision = EntityDecision::new(1, DecisionType::Growth, 0.8, [0.0, 0.0, 0.0]);
        assert!(decision.is_significant());
        assert_eq!(decision.entity_id, 1);
    }

    #[test]
    fn test_insignificant_decision_filtered() {
        let decision = EntityDecision::new(1, DecisionType::Growth, 0.001, [0.0, 0.0, 0.0]);
        assert!(!decision.is_significant());
    }

    #[test]
    fn test_decision_feedback_processing() {
        let mut feedback = DecisionFeedback::with_defaults();

        feedback.add_decision(EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        ));
        feedback.add_decision(EntityDecision::new(
            2,
            DecisionType::Service,
            0.7,
            [1.0, 0.0, 0.0],
        ));

        let perturbation = feedback.process_pending();

        assert!(perturbation.coherence_effect > 0.0);
    }

    #[test]
    fn test_field_perturbation_scaling() {
        let base = FieldPerturbation::growth();
        let scaled = base.scale(0.5);

        assert!(scaled.coherence_effect < base.coherence_effect);
    }

    #[test]
    fn test_field_perturbation_combination() {
        let a = FieldPerturbation::growth();
        let b = FieldPerturbation::service();
        let combined = a.combine(&b);

        assert!(combined.coherence_effect > a.coherence_effect);
    }

    #[test]
    fn test_decision_counting() {
        let mut feedback = DecisionFeedback::with_defaults();

        feedback.add_decision(EntityDecision::new(
            1,
            DecisionType::Growth,
            0.5,
            [0.0, 0.0, 0.0],
        ));
        feedback.add_decision(EntityDecision::new(
            2,
            DecisionType::Growth,
            0.6,
            [1.0, 0.0, 0.0],
        ));
        feedback.add_decision(EntityDecision::new(
            3,
            DecisionType::Service,
            0.7,
            [2.0, 0.0, 0.0],
        ));

        assert_eq!(feedback.decision_count(DecisionType::Growth), 2);
        assert_eq!(feedback.decision_count(DecisionType::Service), 1);
    }

    #[test]
    fn test_polarity_values() {
        assert!(DecisionType::Service.polarity() > 0.0);
        assert!(DecisionType::Control.polarity() < 0.0);
        assert_eq!(DecisionType::Growth.polarity(), 0.0);
    }
}
