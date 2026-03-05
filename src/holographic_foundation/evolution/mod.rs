//! Evolution Feedback - Bottom-up causal flow from entities to field
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 4:
//! "Implement bottom-up feedback where entity decisions modify the field."
//!
//! The Bidirectional Flow:
//! ```text
//! Top-Down (Involution): Field → Entity (constraint)
//! Bottom-Up (Evolution): Entity → Field (perturbation)
//! ```
//!
//! KEY PRINCIPLE: Entity choices create localized field modifications
//! - Amplitude = significance of decision
//! - Phase = nature of decision
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "All development flows from this moment" - entity choices shape the cosmos

mod collective_influence;
mod decision_feedback;
mod density_progression;
mod karmic_encoding;

pub use collective_influence::{CollectiveInfluence, InfluenceAggregation};
pub use decision_feedback::{DecisionFeedback, DecisionType, EntityDecision, FieldPerturbation};
pub use density_progression::{DensityProgression, ProgressionEvent, SpectrumShift};
pub use karmic_encoding::{KarmicEncoding, KarmicPattern, PatternSignature};

use crate::types::Float;

pub const MAX_DECISION_HISTORY: usize = 10000;
pub const FEEDBACK_DECAY_RATE: Float = 0.99;
pub const MIN_SIGNIFICANCE_THRESHOLD: Float = 0.01;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FeedbackMode {
    Individual,
    Collective,
    #[default]
    Hierarchical,
}

#[derive(Debug, Clone)]
pub struct EvolutionFeedbackConfig {
    pub feedback_strength: Float,
    pub perturbation_decay: Float,
    pub influence_range: Float,
    pub enable_density_progression: bool,
    pub progression_threshold: Float,
    pub collective_threshold: usize,
    pub time_step: Float,
}

impl Default for EvolutionFeedbackConfig {
    fn default() -> Self {
        Self {
            feedback_strength: 0.5,
            perturbation_decay: FEEDBACK_DECAY_RATE,
            influence_range: 50.0,
            enable_density_progression: true,
            progression_threshold: 0.3,
            collective_threshold: 3,
            time_step: 0.01,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_mode_default() {
        assert_eq!(FeedbackMode::default(), FeedbackMode::Hierarchical);
    }

    #[test]
    fn test_config_defaults() {
        let config = EvolutionFeedbackConfig::default();
        assert!(config.enable_density_progression);
        assert!(config.feedback_strength > 0.0);
    }
}
