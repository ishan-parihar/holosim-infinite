//! Octave Transition Mechanics - 7th-8th Density Transition
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "7th Density (Violet Ray) - Completion:
//!  - Field returns toward SOURCE configuration
//!  - All experiences integrated coherently
//!  - Gateway to next octave
//!
//!  8th Density - Return:
//!  - Merger with Intelligent Infinity field
//!  - Unique pattern preserved in unified field
//!  - Preparation for next octave seeding"
//!
//! # Key Insight
//!
//! The 7th-8th density transition is the completion of the octave journey.
//! The entity's field returns to Source configuration while preserving its
//! unique pattern - ready for the next octave's seeding.

use crate::types::Float;
use std::collections::HashMap;

/// Completion Stage - stages of 7th density completion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionStage {
    Beginning,
    Integrating,
    Harmonizing,
    Crystallizing,
    Complete,
    PreparingTransition,
}

impl CompletionStage {
    pub fn progress(&self) -> Float {
        match self {
            CompletionStage::Beginning => 0.0,
            CompletionStage::Integrating => 0.2,
            CompletionStage::Harmonizing => 0.4,
            CompletionStage::Crystallizing => 0.6,
            CompletionStage::Complete => 0.8,
            CompletionStage::PreparingTransition => 1.0,
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            CompletionStage::Beginning => Some(CompletionStage::Integrating),
            CompletionStage::Integrating => Some(CompletionStage::Harmonizing),
            CompletionStage::Harmonizing => Some(CompletionStage::Crystallizing),
            CompletionStage::Crystallizing => Some(CompletionStage::Complete),
            CompletionStage::Complete => Some(CompletionStage::PreparingTransition),
            CompletionStage::PreparingTransition => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CompletionStage::Beginning => "Beginning the completion journey",
            CompletionStage::Integrating => "Integrating all octave experiences",
            CompletionStage::Harmonizing => "Harmonizing field patterns",
            CompletionStage::Crystallizing => "Crystallizing unique signature",
            CompletionStage::Complete => "Octave completion achieved",
            CompletionStage::PreparingTransition => "Preparing for next octave",
        }
    }
}

/// Octave Transition State - the state of octave transition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OctaveTransitionState {
    InOctave,
    ApproachingCompletion,
    Transitioning,
    InVoid,
    EnteringNewOctave,
    Seeded,
}

impl OctaveTransitionState {
    pub fn transition_progress(&self) -> Float {
        match self {
            OctaveTransitionState::InOctave => 0.0,
            OctaveTransitionState::ApproachingCompletion => 0.2,
            OctaveTransitionState::Transitioning => 0.4,
            OctaveTransitionState::InVoid => 0.6,
            OctaveTransitionState::EnteringNewOctave => 0.8,
            OctaveTransitionState::Seeded => 1.0,
        }
    }
}

/// Source Preparation - preparing the entity for Source merger
#[derive(Debug, Clone)]
pub struct SourcePreparation {
    pub entity_id: u64,
    pub experience_integration: Float,
    pub pattern_crystallization: Float,
    pub signature_coherence: Float,
    pub void_readiness: Float,
    pub source_proximity: Float,
    pub experiences_integrated: u64,
    pub patterns_crystallized: u64,
}

impl SourcePreparation {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            experience_integration: 0.0,
            pattern_crystallization: 0.0,
            signature_coherence: 0.5,
            void_readiness: 0.0,
            source_proximity: 0.0,
            experiences_integrated: 0,
            patterns_crystallized: 0,
        }
    }

    pub fn integrate_experience(&mut self, significance: Float) {
        self.experience_integration = (self.experience_integration + significance * 0.01).min(1.0);
        self.experiences_integrated += 1;
        self.update_derived();
    }

    pub fn crystallize_pattern(&mut self, pattern_strength: Float) {
        self.pattern_crystallization =
            (self.pattern_crystallization + pattern_strength * 0.01).min(1.0);
        self.patterns_crystallized += 1;
        self.update_derived();
    }

    fn update_derived(&mut self) {
        self.signature_coherence =
            (self.experience_integration * 0.4 + self.pattern_crystallization * 0.4 + 0.2).min(1.0);

        self.void_readiness = self.signature_coherence * 0.9;
        self.source_proximity = self.signature_coherence * 0.95;
    }

    pub fn update(&mut self, dt: Float) {
        let integration_drift = 0.001 * dt * (1.0 - self.experience_integration);
        self.experience_integration = (self.experience_integration + integration_drift).min(1.0);

        let crystallization_drift = 0.001 * dt * (1.0 - self.pattern_crystallization);
        self.pattern_crystallization =
            (self.pattern_crystallization + crystallization_drift).min(1.0);

        self.update_derived();
    }

    pub fn is_ready_for_transition(&self) -> bool {
        self.experience_integration >= 0.9
            && self.pattern_crystallization >= 0.9
            && self.signature_coherence >= 0.95
    }

    pub fn completion_percentage(&self) -> Float {
        (self.experience_integration + self.pattern_crystallization + self.signature_coherence)
            / 3.0
    }
}

/// Octave Bridge - the bridge between octaves
#[derive(Debug, Clone)]
pub struct OctaveBridge {
    pub bridge_id: u64,
    pub from_octave: u8,
    pub to_octave: u8,
    pub bridge_strength: Float,
    pub entities_in_transit: HashMap<u64, Float>,
    pub pattern_seeds: Vec<PatternSeed>,
    pub bridge_stability: Float,
}

impl OctaveBridge {
    pub fn new(bridge_id: u64, from_octave: u8, to_octave: u8) -> Self {
        Self {
            bridge_id,
            from_octave,
            to_octave,
            bridge_strength: 0.0,
            entities_in_transit: HashMap::new(),
            pattern_seeds: Vec::new(),
            bridge_stability: 0.5,
        }
    }

    pub fn add_entity(&mut self, entity_id: u64, preparation: &SourcePreparation) -> bool {
        if !preparation.is_ready_for_transition() {
            return false;
        }

        self.entities_in_transit.insert(entity_id, 0.0);
        true
    }

    pub fn update(&mut self, dt: Float) {
        let transit_rate = 0.01 * dt * self.bridge_stability;

        for progress in self.entities_in_transit.values_mut() {
            *progress = (*progress + transit_rate).min(1.0);
        }

        self.bridge_strength = if self.entities_in_transit.is_empty() {
            0.0
        } else {
            self.entities_in_transit.values().sum::<Float>()
                / self.entities_in_transit.len() as Float
        };

        let stability_drift = 0.001 * dt * (0.7 - self.bridge_stability);
        self.bridge_stability = (self.bridge_stability + stability_drift).clamp(0.1, 1.0);
    }

    pub fn entities_ready_for_seeding(&self) -> Vec<u64> {
        self.entities_in_transit
            .iter()
            .filter(|(_, &progress)| progress >= 1.0)
            .map(|(&id, _)| id)
            .collect()
    }

    pub fn create_pattern_seed(&mut self, entity_id: u64, signature: Vec<Float>) {
        let seed = PatternSeed::new(entity_id, signature, self.to_octave);
        self.pattern_seeds.push(seed);
    }

    pub fn remove_completed(&mut self) -> Vec<u64> {
        let ready: Vec<u64> = self.entities_ready_for_seeding();
        for id in &ready {
            self.entities_in_transit.remove(id);
        }
        ready
    }
}

/// Pattern Seed - the crystallized pattern for next octave
#[derive(Debug, Clone)]
pub struct PatternSeed {
    pub source_entity_id: u64,
    pub signature: Vec<Float>,
    pub target_octave: u8,
    pub seed_strength: Float,
    pub potential: Float,
    pub preserved_experiences: HashMap<String, Float>,
}

impl PatternSeed {
    pub fn new(source_entity_id: u64, signature: Vec<Float>, target_octave: u8) -> Self {
        let seed_strength = signature.iter().sum::<Float>() / signature.len().max(1) as Float;
        Self {
            source_entity_id,
            signature,
            target_octave,
            seed_strength,
            potential: seed_strength,
            preserved_experiences: HashMap::new(),
        }
    }

    pub fn preserve_experience(&mut self, key: &str, value: Float) {
        self.preserved_experiences.insert(key.to_string(), value);
    }

    pub fn get_preserved(&self, key: &str) -> Float {
        self.preserved_experiences.get(key).copied().unwrap_or(0.0)
    }

    pub fn activate(&self) -> ActivatedSeed {
        ActivatedSeed::new(
            self.source_entity_id,
            self.signature.clone(),
            self.target_octave,
            self.seed_strength,
        )
    }
}

/// Activated Seed - a seed activated in the new octave
#[derive(Debug, Clone)]
pub struct ActivatedSeed {
    pub source_entity_id: u64,
    pub new_entity_id: Option<u64>,
    pub signature: Vec<Float>,
    pub octave: u8,
    pub strength: Float,
    pub activation_time: Float,
}

impl ActivatedSeed {
    pub fn new(source_id: u64, signature: Vec<Float>, octave: u8, strength: Float) -> Self {
        Self {
            source_entity_id: source_id,
            new_entity_id: None,
            signature,
            octave,
            strength,
            activation_time: 0.0,
        }
    }

    pub fn assign_new_id(&mut self, new_id: u64) {
        self.new_entity_id = Some(new_id);
    }

    pub fn is_activated(&self) -> bool {
        self.new_entity_id.is_some()
    }
}

/// Octave Transition - main 7th-8th density transition system
#[derive(Debug, Clone)]
pub struct OctaveTransition {
    pub entity_id: u64,
    pub completion_stage: CompletionStage,
    pub transition_state: OctaveTransitionState,
    pub preparation: SourcePreparation,
    pub bridge: Option<OctaveBridge>,
    pub seed: Option<PatternSeed>,
    pub octave_number: u8,
}

impl OctaveTransition {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            completion_stage: CompletionStage::Beginning,
            transition_state: OctaveTransitionState::InOctave,
            preparation: SourcePreparation::new(entity_id),
            bridge: None,
            seed: None,
            octave_number: 1,
        }
    }

    pub fn with_octave(mut self, octave: u8) -> Self {
        self.octave_number = octave;
        self
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        self.preparation.update(dt);

        let threshold = match self.completion_stage {
            CompletionStage::Beginning => 0.15,
            CompletionStage::Integrating => 0.35,
            CompletionStage::Harmonizing => 0.55,
            CompletionStage::Crystallizing => 0.75,
            CompletionStage::Complete => 0.9,
            CompletionStage::PreparingTransition => 1.0,
        };

        if self.preparation.completion_percentage() >= threshold {
            if let Some(next) = self.completion_stage.next() {
                self.completion_stage = next;
            }
        }

        if self.preparation.is_ready_for_transition() {
            self.transition_state = OctaveTransitionState::ApproachingCompletion;
        }

        if let Some(bridge) = &mut self.bridge {
            bridge.update(dt);

            let progress = bridge
                .entities_in_transit
                .get(&self.entity_id)
                .copied()
                .unwrap_or(0.0);

            if progress >= 0.3 {
                self.transition_state = OctaveTransitionState::Transitioning;
            }
            if progress >= 0.6 {
                self.transition_state = OctaveTransitionState::InVoid;
            }
            if progress >= 0.8 {
                self.transition_state = OctaveTransitionState::EnteringNewOctave;
            }
            if progress >= 1.0 {
                self.transition_state = OctaveTransitionState::Seeded;
            }
        }
    }

    pub fn begin_transition(&mut self, bridge_id: u64) -> bool {
        if !self.preparation.is_ready_for_transition() {
            return false;
        }

        let new_bridge = OctaveBridge::new(bridge_id, self.octave_number, self.octave_number + 1);
        self.bridge = Some(new_bridge.clone());

        if let Some(bridge) = &mut self.bridge {
            bridge.add_entity(self.entity_id, &self.preparation);
        }

        self.transition_state = OctaveTransitionState::ApproachingCompletion;
        true
    }

    pub fn create_seed(&mut self, signature: Vec<Float>) {
        if self.transition_state == OctaveTransitionState::InVoid {
            self.seed = Some(PatternSeed::new(
                self.entity_id,
                signature.clone(),
                self.octave_number + 1,
            ));

            if let Some(bridge) = &mut self.bridge {
                bridge.create_pattern_seed(self.entity_id, signature);
            }
        }
    }

    pub fn complete_transition(&mut self) -> Option<ActivatedSeed> {
        if self.transition_state != OctaveTransitionState::Seeded {
            return None;
        }

        self.seed.as_ref().map(|s| s.activate())
    }

    pub fn is_complete(&self) -> bool {
        self.transition_state == OctaveTransitionState::Seeded
    }

    pub fn transition_progress(&self) -> Float {
        self.completion_stage.progress() * 0.5 + self.transition_state.transition_progress() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_stage_progress() {
        assert_eq!(CompletionStage::Beginning.progress(), 0.0);
        assert_eq!(CompletionStage::Complete.progress(), 0.8);
    }

    #[test]
    fn test_completion_stage_next() {
        assert_eq!(
            CompletionStage::Beginning.next(),
            Some(CompletionStage::Integrating)
        );
    }

    #[test]
    fn test_octave_transition_state_progress() {
        assert_eq!(OctaveTransitionState::InOctave.transition_progress(), 0.0);
        assert_eq!(OctaveTransitionState::Seeded.transition_progress(), 1.0);
    }

    #[test]
    fn test_source_preparation_creation() {
        let prep = SourcePreparation::new(1);
        assert_eq!(prep.entity_id, 1);
    }

    #[test]
    fn test_source_preparation_integrate() {
        let mut prep = SourcePreparation::new(1);
        prep.integrate_experience(1.0);
        assert!(prep.experience_integration > 0.0);
        assert_eq!(prep.experiences_integrated, 1);
    }

    #[test]
    fn test_source_preparation_crystallize() {
        let mut prep = SourcePreparation::new(1);
        prep.crystallize_pattern(1.0);
        assert!(prep.pattern_crystallization > 0.0);
    }

    #[test]
    fn test_source_preparation_ready() {
        let mut prep = SourcePreparation::new(1);
        prep.experience_integration = 0.95;
        prep.pattern_crystallization = 0.95;
        prep.signature_coherence = 0.96;
        assert!(prep.is_ready_for_transition());
    }

    #[test]
    fn test_octave_bridge_creation() {
        let bridge = OctaveBridge::new(1, 1, 2);
        assert_eq!(bridge.from_octave, 1);
        assert_eq!(bridge.to_octave, 2);
    }

    #[test]
    fn test_octave_bridge_add_entity() {
        let mut bridge = OctaveBridge::new(1, 1, 2);
        let mut prep = SourcePreparation::new(1);
        prep.experience_integration = 0.95;
        prep.pattern_crystallization = 0.95;
        prep.signature_coherence = 0.96;

        let result = bridge.add_entity(1, &prep);
        assert!(result);
        assert!(bridge.entities_in_transit.contains_key(&1));
    }

    #[test]
    fn test_pattern_seed_creation() {
        let seed = PatternSeed::new(1, vec![0.5, 0.5, 0.5], 2);
        assert_eq!(seed.source_entity_id, 1);
        assert_eq!(seed.target_octave, 2);
    }

    #[test]
    fn test_pattern_seed_preserve() {
        let mut seed = PatternSeed::new(1, vec![0.5], 2);
        seed.preserve_experience("wisdom", 0.8);
        assert!((seed.get_preserved("wisdom") - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_octave_transition_creation() {
        let trans = OctaveTransition::new(1);
        assert_eq!(trans.entity_id, 1);
        assert_eq!(trans.completion_stage, CompletionStage::Beginning);
    }

    #[test]
    fn test_octave_transition_update() {
        let mut trans = OctaveTransition::new(1);
        trans.update(1.0, 1.0);
        assert!(trans.preparation.experience_integration >= 0.0);
    }

    #[test]
    fn test_octave_transition_begin() {
        let mut trans = OctaveTransition::new(1);
        trans.preparation.experience_integration = 0.95;
        trans.preparation.pattern_crystallization = 0.95;
        trans.preparation.signature_coherence = 0.96;

        let result = trans.begin_transition(1);
        assert!(result);
        assert!(trans.bridge.is_some());
    }

    #[test]
    fn test_activated_seed_creation() {
        let seed = ActivatedSeed::new(1, vec![0.5], 2, 0.8);
        assert_eq!(seed.source_entity_id, 1);
        assert!(!seed.is_activated());
    }

    #[test]
    fn test_activated_seed_assign() {
        let mut seed = ActivatedSeed::new(1, vec![0.5], 2, 0.8);
        seed.assign_new_id(2);
        assert!(seed.is_activated());
        assert_eq!(seed.new_entity_id, Some(2));
    }
}
