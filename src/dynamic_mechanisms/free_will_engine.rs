//! Free Will Engine (Dynamic Mechanism 3)
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
//! "Enable free will decisions affecting entity trajectories"
//!
//! This module implements free will decision-making that:
//! - Generates non-deterministic choices from possibility space
//! - Affects entity trajectories and polarity progression
//! - Integrates with catalysts and archetype processing
//! - Tracks free will evolution and learning

use crate::consciousness::free_will::{
    Choice, ChoiceContext, ChoiceRecord, FreeWillKernel, FreeWillStatistics, PolarityPreference,
};
use crate::entity_layer7::layer7::{EntityId, EntityState};
use crate::simulation_v3::catalyst_system::{Catalyst, CatalystVariety, PolarityChoice};
use crate::types::{Float, Polarity};
use rand::Rng;
use std::collections::HashMap;

/// Free Will Engine
///
/// Enables free will decisions that affect entity trajectories.
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Success Criteria:
/// "Free will choices are non-deterministic"
pub struct FreeWillEngine {
    /// Free will kernels for each entity
    free_will_kernels: HashMap<EntityId, FreeWillKernel>,

    /// Decision history
    decision_history: Vec<FreeWillDecision>,

    /// Simulation time
    simulation_time: u64,
}

/// Free Will Decision
///
/// A free will decision made by an entity.
#[derive(Debug, Clone)]
pub struct FreeWillDecision {
    /// Decision ID
    pub decision_id: u64,

    /// Entity ID
    pub entity_id: EntityId,

    /// Timestamp
    pub timestamp: u64,

    /// Catalyst that triggered this decision
    pub catalyst: Option<Catalyst>,

    /// Choice made
    pub choice: Choice,

    /// Polarity choice (STO vs STS)
    pub polarity_choice: PolarityChoice,

    /// Decision confidence (0.0 to 1.0)
    pub confidence: Float,

    /// Intentionality (alignment with chosen polarity)
    pub intentionality: Float,

    /// Effect on entity trajectory
    pub trajectory_effect: TrajectoryEffect,
}

/// Trajectory Effect
///
/// How a free will decision affects entity trajectory.
#[derive(Debug, Clone)]
pub struct TrajectoryEffect {
    /// Polarity change (-1.0 to 1.0)
    pub polarity_change: Float,

    /// Consciousness change (0.0 to 1.0)
    pub consciousness_change: Float,

    /// Experience gained (0.0 to 1.0)
    pub experience_gain: Float,

    /// Density progression (0.0 to 1.0, where 1.0 = next density)
    pub density_progression: Float,
}

/// Free Will Engine State
///
/// Current state of the free will engine.
#[derive(Debug, Clone)]
pub struct FreeWillEngineState {
    /// Total decisions made
    pub total_decisions: usize,

    /// STO choices
    pub sto_choices: usize,

    /// STS choices
    pub sts_choices: usize,

    /// Average confidence
    pub avg_confidence: Float,

    /// Average intentionality
    pub avg_intentionality: Float,
}

impl Default for FreeWillEngineState {
    fn default() -> Self {
        Self {
            total_decisions: 0,
            sto_choices: 0,
            sts_choices: 0,
            avg_confidence: 0.0,
            avg_intentionality: 0.0,
        }
    }
}

impl FreeWillEngine {
    /// Create a new free will engine
    pub fn new() -> Self {
        Self {
            free_will_kernels: HashMap::new(),
            decision_history: Vec::new(),
            simulation_time: 0,
        }
    }

    /// Initialize free will kernel for an entity
    pub fn initialize_entity(&mut self, entity_id: EntityId, free_will_kernel: FreeWillKernel) {
        self.free_will_kernels.insert(entity_id, free_will_kernel);
    }

    /// Make a free will decision for an entity
    ///
    /// This is the main entry point for free will decision-making.
    /// It generates a non-deterministic choice from possibility space.
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Free will choices are non-deterministic"
    pub fn make_decision(
        &mut self,
        entity_id: &EntityId,
        entity_state: &EntityState,
        catalyst: Option<&Catalyst>,
    ) -> Option<FreeWillDecision> {
        // Get free will kernel for this entity
        let free_will_kernel = self.free_will_kernels.get_mut(entity_id)?;

        // Calculate catalyst intensity (default to 0.5 if no catalyst)
        let catalyst_intensity = catalyst.map(|c| c.intensity).unwrap_or(0.5);

        // Calculate veil transparency (default to 0.0)
        let veil_transparency = 0.0;

        // Determine polarity preference based on entity state
        let polarity_preference = Self::determine_polarity_preference(entity_state);

        // Create choice context
        let context = ChoiceContext {
            polarity_preference,
            environmental_constraints: catalyst
                .map(|c| {
                    vec![crate::consciousness::free_will::EnvironmentalConstraint {
                        constraint_type: match c.variety {
                            CatalystVariety::Physical => {
                                crate::consciousness::free_will::ConstraintType::Physical
                            }
                            CatalystVariety::Emotional => {
                                crate::consciousness::free_will::ConstraintType::Emotional
                            }
                            CatalystVariety::Intellectual => {
                                crate::consciousness::free_will::ConstraintType::Mental
                            }
                            CatalystVariety::Spiritual => {
                                crate::consciousness::free_will::ConstraintType::Spiritual
                            }
                        },
                        severity: c.intensity,
                    }]
                })
                .unwrap_or_default(),
            experience_bias: entity_state.experience_accumulation / 100.0,
        };

        // Exercise free will
        let choice_result = free_will_kernel.exercise_free_will(
            entity_state,
            &context,
            catalyst_intensity,
            veil_transparency,
        );

        // Convert choice to polarity choice
        let polarity_choice = self.choice_to_polarity_choice(&choice_result.choice);

        // Calculate intentionality
        let intentionality =
            self.calculate_intentionality(entity_state, &polarity_choice, &choice_result.choice);

        // Calculate trajectory effect
        let trajectory_effect = self.calculate_trajectory_effect(
            entity_state,
            &polarity_choice,
            catalyst_intensity,
            &choice_result.choice,
        );

        // Create decision record
        let decision = FreeWillDecision {
            decision_id: self.decision_history.len() as u64,
            entity_id: entity_id.clone(),
            timestamp: self.simulation_time,
            catalyst: catalyst.cloned(),
            choice: choice_result.choice.clone(),
            polarity_choice,
            confidence: choice_result.choice.confidence,
            intentionality,
            trajectory_effect,
        };

        // Record decision
        self.decision_history.push(decision.clone());

        self.simulation_time += 1;

        Some(decision)
    }

    /// Determine polarity preference based on entity state
    fn determine_polarity_preference(entity_state: &EntityState) -> PolarityPreference {
        let polarity_bias = entity_state.polarity_state.polarity_bias;

        if polarity_bias > 0.05 {
            PolarityPreference::ServiceToOthers
        } else if polarity_bias < -0.05 {
            PolarityPreference::ServiceToSelf
        } else {
            PolarityPreference::Neutral
        }
    }

    /// Convert Choice to PolarityChoice
    fn choice_to_polarity_choice(&self, choice: &Choice) -> PolarityChoice {
        if choice.sto_alignment > 0.3 {
            PolarityChoice::ServiceToOthers
        } else if choice.sto_alignment < -0.3 {
            PolarityChoice::ServiceToSelf
        } else {
            PolarityChoice::Unpolarized
        }
    }

    /// Calculate intentionality of a choice
    ///
    /// Intentionality measures how aligned the choice is with the chosen polarity.
    fn calculate_intentionality(
        &self,
        entity_state: &EntityState,
        polarity_choice: &PolarityChoice,
        choice: &Choice,
    ) -> Float {
        let base_intentionality = choice.confidence;

        // Check alignment with existing polarity bias
        let alignment = match polarity_choice {
            PolarityChoice::ServiceToOthers => {
                if entity_state.polarity_state.polarity_bias > 0.0 {
                    1.0 // Aligned
                } else {
                    0.5 // Misaligned
                }
            }
            PolarityChoice::ServiceToSelf => {
                if entity_state.polarity_state.polarity_bias < 0.0 {
                    1.0 // Aligned
                } else {
                    0.5 // Misaligned
                }
            }
            PolarityChoice::Unpolarized => {
                if entity_state.polarity_state.polarity_bias.abs() < 0.1 {
                    1.0 // Aligned (truly neutral)
                } else {
                    0.3 // Misaligned (should choose a polarity)
                }
            }
        };

        base_intentionality * alignment
    }

    /// Calculate trajectory effect of a choice
    ///
    /// Trajectory effect measures how the choice affects entity development.
    fn calculate_trajectory_effect(
        &self,
        entity_state: &EntityState,
        polarity_choice: &PolarityChoice,
        catalyst_intensity: Float,
        choice: &Choice,
    ) -> TrajectoryEffect {
        // Polarity change
        let polarity_change = match polarity_choice {
            PolarityChoice::ServiceToOthers => catalyst_intensity * 0.1,
            PolarityChoice::ServiceToSelf => -catalyst_intensity * 0.1,
            PolarityChoice::Unpolarized => 0.0,
        };

        // Consciousness change (higher for polarized choices)
        let consciousness_change = match polarity_choice {
            PolarityChoice::ServiceToOthers | PolarityChoice::ServiceToSelf => {
                catalyst_intensity * choice.confidence * 0.05
            }
            PolarityChoice::Unpolarized => catalyst_intensity * 0.01,
        };

        // Experience gain
        let experience_gain = catalyst_intensity * choice.confidence * 0.1;

        // Density progression (accumulates over time)
        let density_progression = catalyst_intensity * choice.confidence * 0.01;

        TrajectoryEffect {
            polarity_change,
            consciousness_change,
            experience_gain,
            density_progression,
        }
    }

    /// Apply trajectory effect to entity state
    pub fn apply_trajectory_effect(
        &self,
        entity_state: &mut EntityState,
        trajectory_effect: &TrajectoryEffect,
    ) {
        // Apply polarity change
        entity_state.polarity_state.polarity_bias = (entity_state.polarity_state.polarity_bias
            + trajectory_effect.polarity_change)
            .clamp(-1.0, 1.0);

        // Apply polarization strength
        entity_state.polarity_state.polarization_strength =
            (entity_state.polarity_state.polarization_strength
                + trajectory_effect.polarity_change.abs() * 0.5)
                .min(1.0);

        // Apply consciousness change
        entity_state.consciousness_level =
            (entity_state.consciousness_level + trajectory_effect.consciousness_change).min(1.0);

        // Apply experience gain
        entity_state.experience_accumulation = (entity_state.experience_accumulation
            + trajectory_effect.experience_gain * 10.0)
            .min(100.0);
    }

    /// Get free will kernel for an entity
    pub fn get_free_will_kernel(&self, entity_id: &EntityId) -> Option<&FreeWillKernel> {
        self.free_will_kernels.get(entity_id)
    }

    /// Get mutable free will kernel for an entity
    pub fn get_free_will_kernel_mut(
        &mut self,
        entity_id: &EntityId,
    ) -> Option<&mut FreeWillKernel> {
        self.free_will_kernels.get_mut(entity_id)
    }

    /// Get engine state
    pub fn get_state(&self) -> FreeWillEngineState {
        let total_decisions = self.decision_history.len();

        let sto_choices = self
            .decision_history
            .iter()
            .filter(|d| matches!(d.polarity_choice, PolarityChoice::ServiceToOthers))
            .count();

        let sts_choices = self
            .decision_history
            .iter()
            .filter(|d| matches!(d.polarity_choice, PolarityChoice::ServiceToSelf))
            .count();

        let avg_confidence = if total_decisions > 0 {
            let total: Float = self.decision_history.iter().map(|d| d.confidence).sum();
            total / total_decisions as Float
        } else {
            0.0
        };

        let avg_intentionality = if total_decisions > 0 {
            let total: Float = self.decision_history.iter().map(|d| d.intentionality).sum();
            total / total_decisions as Float
        } else {
            0.0
        };

        FreeWillEngineState {
            total_decisions,
            sto_choices,
            sts_choices,
            avg_confidence,
            avg_intentionality,
        }
    }

    /// Get decision history for an entity
    pub fn get_entity_decisions(&self, entity_id: &EntityId) -> Vec<&FreeWillDecision> {
        self.decision_history
            .iter()
            .filter(|d| &d.entity_id == entity_id)
            .collect()
    }

    /// Get recent decisions (last N)
    pub fn get_recent_decisions(&self, count: usize) -> Vec<&FreeWillDecision> {
        let start = self.decision_history.len().saturating_sub(count);
        self.decision_history[start..].iter().collect()
    }
}

impl Default for FreeWillEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consciousness::free_will::Choice;
    use crate::entity_layer7::layer7::{PolarityState, VibrationalState};
    use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
    use crate::foundation::indigo_realm::{
        Archetype22 as FoundationArchetype22, IntelligentInfinity,
    };

    fn create_test_entity_state() -> EntityState {
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.7,
                density: Density::First(Density1SubLevel::Quantum),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level: 0.3,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        }
    }

    fn create_test_free_will_kernel() -> FreeWillKernel {
        let intelligent_infinity = IntelligentInfinity::new();
        let archetype22 = FoundationArchetype22::new();
        FreeWillKernel::new(archetype22)
    }

    #[test]
    fn test_free_will_engine_creation() {
        let engine = FreeWillEngine::new();
        assert_eq!(engine.simulation_time, 0);
        assert!(engine.decision_history.is_empty());
    }

    #[test]
    fn test_initialize_entity() {
        let mut engine = FreeWillEngine::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let kernel = create_test_free_will_kernel();

        engine.initialize_entity(entity_id.clone(), kernel);

        assert!(engine.free_will_kernels.contains_key(&entity_id));
    }

    #[test]
    fn test_make_decision() {
        let mut engine = FreeWillEngine::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let kernel = create_test_free_will_kernel();

        engine.initialize_entity(entity_id.clone(), kernel);

        let entity_state = create_test_entity_state();

        let decision = engine.make_decision(&entity_id, &entity_state, None);

        assert!(decision.is_some());
        let decision = decision.unwrap();
        assert_eq!(decision.entity_id, entity_id);
    }

    #[test]
    fn test_trajectory_effect_calculation() {
        let engine = FreeWillEngine::new();
        let entity_state = create_test_entity_state();

        let polarity_choice = PolarityChoice::ServiceToOthers;
        let catalyst_intensity = 0.7;
        let choice = Choice {
            selected_index: 0,
            confidence: 0.8,
            sto_alignment: 1.0,
        };

        let effect = engine.calculate_trajectory_effect(
            &entity_state,
            &polarity_choice,
            catalyst_intensity,
            &choice,
        );

        assert!(effect.polarity_change > 0.0);
        assert!(effect.consciousness_change > 0.0);
        assert!(effect.experience_gain > 0.0);
    }

    #[test]
    fn test_apply_trajectory_effect() {
        let engine = FreeWillEngine::new();
        let mut entity_state = create_test_entity_state();

        let effect = TrajectoryEffect {
            polarity_change: 0.1,
            consciousness_change: 0.05,
            experience_gain: 0.1,
            density_progression: 0.01,
        };

        let initial_polarity = entity_state.polarity_state.polarity_bias;
        let initial_consciousness = entity_state.consciousness_level;
        let initial_experience = entity_state.experience_accumulation;

        engine.apply_trajectory_effect(&mut entity_state, &effect);

        assert!(entity_state.polarity_state.polarity_bias > initial_polarity);
        assert!(entity_state.consciousness_level > initial_consciousness);
        assert!(entity_state.experience_accumulation > initial_experience);
    }

    #[test]
    fn test_get_state() {
        let mut engine = FreeWillEngine::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let kernel = create_test_free_will_kernel();

        engine.initialize_entity(entity_id.clone(), kernel);

        let entity_state = create_test_entity_state();

        // Make several decisions
        for _ in 0..5 {
            engine.make_decision(&entity_id, &entity_state, None);
        }

        let state = engine.get_state();
        assert_eq!(state.total_decisions, 5);
    }
}
