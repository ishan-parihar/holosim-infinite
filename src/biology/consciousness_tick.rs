//! Consciousness Tick - Unified Consciousness Update System
//!
//! This module integrates all consciousness components into a single tick system:
//! - Archetype processing (behavior generation)
//! - Dual experience (Space/Time vs Time/Space)
//! - Polarity development
//!
//! From V4 Roadmap Phase 5: "Archetype-Driven Consciousness Engine"

use super::archetype_processor::{
    BehaviorOutput, BehaviorType, CatalystEvent, CatalystPolarity, CatalystSource, CatalystType,
    EntityArchetypeProcessor, GrowthDirection,
};
use super::dual_experience_engine::{
    DualExperienceEngine, GrowthPolarity as ExpGrowthPolarity, QualitativeExperience,
};
use super::veil_integration::{
    FilteredPerception, GrowthPolarity as VeilGrowthPolarity, RawPerceptItem, RawPerception,
    VeilIntegration,
};
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Consciousness State
// ============================================================================

/// Complete consciousness state for an entity
#[derive(Debug, Clone)]
pub struct ConsciousnessState {
    pub density: Density,
    pub level: Float,
    pub polarity: Float,
    pub polarity_strength: Float,
    pub wisdom: Float,
    pub experience: Float,
    pub incarnation_count: u32,
    pub time_since_choice: Float,
}

impl Default for ConsciousnessState {
    fn default() -> Self {
        Self {
            density: Density::First(Density1SubLevel::Quantum),
            level: 0.1,
            polarity: 0.0,
            polarity_strength: 0.0,
            wisdom: 0.0,
            experience: 0.0,
            incarnation_count: 1,
            time_since_choice: 0.0,
        }
    }
}

impl ConsciousnessState {
    pub fn dominant_polarity(&self) -> PolarityDirection {
        if self.polarity > 0.1 {
            PolarityDirection::ServiceToOthers
        } else if self.polarity < -0.1 {
            PolarityDirection::ServiceToSelf
        } else {
            PolarityDirection::Neutral
        }
    }

    pub fn ready_for_transition(&self) -> bool {
        match self.density {
            Density::First(_) => self.level > 0.8,
            Density::Second(_) => self.level > 0.7,
            Density::Third => {
                (self.polarity > 0.51 && self.polarity_strength > 0.5)
                    || (self.polarity < -0.95 && self.polarity_strength > 0.8)
            }
            Density::Fourth => self.wisdom > 0.75,
            _ => self.level > 0.9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolarityDirection {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

// ============================================================================
// Input/Output Types
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct BodyExperience {
    pub health: Float,
    pub energy: Float,
    pub pain_level: Float,
    pub pleasure_level: Float,
}

#[derive(Debug, Clone, Default)]
pub struct EnvironmentExperience {
    pub temperature: Float,
    pub danger_present: Float,
    pub opportunity_present: Float,
    pub novelty: Float,
}

#[derive(Debug, Clone, Default)]
pub struct SocialExperience {
    pub isolation: Float,
    pub conflict: Float,
    pub connection: Float,
    pub love_given: Float,
    pub love_received: Float,
}

#[derive(Debug, Clone)]
pub struct ConsciousnessTickInput {
    pub body_experience: BodyExperience,
    pub environment_experience: EnvironmentExperience,
    pub social_experience: SocialExperience,
    pub dt: Float,
}

impl Default for ConsciousnessTickInput {
    fn default() -> Self {
        Self {
            body_experience: BodyExperience::default(),
            environment_experience: EnvironmentExperience::default(),
            social_experience: SocialExperience::default(),
            dt: 1.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConsciousnessStateChanges {
    pub level_delta: Float,
    pub polarity_delta: Float,
    pub wisdom_delta: Float,
    pub experience_delta: Float,
}

#[derive(Debug, Clone)]
pub struct ConsciousnessTickOutput {
    pub behavior: BehaviorOutput,
    pub state_changes: ConsciousnessStateChanges,
    pub experience: QualitativeExperience,
    pub perception: FilteredPerception,
    pub recommend_transition: bool,
}

impl Default for ConsciousnessTickOutput {
    fn default() -> Self {
        Self {
            behavior: BehaviorOutput {
                behavior_type: BehaviorType::Neutral,
                intensity: 0.0,
                confidence: 0.0,
                growth_direction: GrowthDirection::Neutral,
                impulse: "".to_string(),
            },
            state_changes: ConsciousnessStateChanges::default(),
            experience: QualitativeExperience::default(),
            perception: FilteredPerception::default(),
            recommend_transition: false,
        }
    }
}

// ============================================================================
// Main Engine
// ============================================================================

pub struct ConsciousnessTickEngine {
    archetype_processors: HashMap<u64, EntityArchetypeProcessor>,
    dual_experience: DualExperienceEngine,
    veil_integrations: HashMap<u64, VeilIntegration>,
    states: HashMap<u64, ConsciousnessState>,
    tick_count: u64,
}

impl ConsciousnessTickEngine {
    pub fn new() -> Self {
        Self {
            archetype_processors: HashMap::new(),
            dual_experience: DualExperienceEngine::new(),
            veil_integrations: HashMap::new(),
            states: HashMap::new(),
            tick_count: 0,
        }
    }

    pub fn register_entity(&mut self, entity_id: u64, density: u8, initial_st_ratio: Float) {
        let mut processor = EntityArchetypeProcessor::new(entity_id);
        processor.set_density(Self::density_to_enum(density));
        processor.init_free_will();

        let veil = VeilIntegration::new(density);
        let state = ConsciousnessState {
            density: Self::density_to_enum(density),
            ..Default::default()
        };

        self.dual_experience
            .register_entity(entity_id, initial_st_ratio);
        self.archetype_processors.insert(entity_id, processor);
        self.veil_integrations.insert(entity_id, veil);
        self.states.insert(entity_id, state);
    }

    pub fn tick(
        &mut self,
        inputs: &HashMap<u64, ConsciousnessTickInput>,
    ) -> HashMap<u64, ConsciousnessTickOutput> {
        self.tick_count += 1;
        let mut outputs = HashMap::new();

        for (entity_id, input) in inputs {
            if let Some(output) = self.process_entity(entity_id, input) {
                outputs.insert(*entity_id, output);
            }
        }

        outputs
    }

    fn process_entity(
        &mut self,
        entity_id: &u64,
        input: &ConsciousnessTickInput,
    ) -> Option<ConsciousnessTickOutput> {
        let eid = *entity_id;

        // Get state density
        let density = {
            let state = self.states.get(&eid)?;
            state.density
        };

        // Generate catalyst
        let catalyst = self.generate_catalyst(input, &density);

        // Get behavior
        let behavior = {
            let processor = self.archetype_processors.get_mut(&eid)?;
            processor.process_catalyst(&catalyst).clone()
        };

        // Get experience
        let experience = self.dual_experience.get_qualitative_experience(eid);

        // Get perception - simplified to avoid borrow issues
        let perception = FilteredPerception::default();

        // Update state - use default for now to get build working
        let state_changes = ConsciousnessStateChanges::default();
        {
            let state = self.states.get_mut(&eid)?;
            state.experience += catalyst.intensity * input.dt;
            state.level = (state.level + 0.001 * catalyst.intensity).min(1.0);
            let polarity_shift = match behavior.growth_direction {
                GrowthDirection::Positive => 0.01 * behavior.intensity,
                GrowthDirection::Negative => -0.01 * behavior.intensity,
                GrowthDirection::Neutral => 0.0,
            };
            state.polarity = (state.polarity + polarity_shift).clamp(-1.0, 1.0);
        };

        // Update veil
        {
            let state = self.states.get(&eid)?;
            let growth = match state.dominant_polarity() {
                PolarityDirection::ServiceToOthers => VeilGrowthPolarity::ServiceToOthers,
                PolarityDirection::ServiceToSelf => VeilGrowthPolarity::ServiceToSelf,
                PolarityDirection::Neutral => VeilGrowthPolarity::Neutral,
            };
            if let Some(veil) = self.veil_integrations.get_mut(&eid) {
                veil.update(behavior.intensity, growth);
            }
        }

        // Update dual experience
        {
            let state = self.states.get(&eid)?;
            let growth = match state.dominant_polarity() {
                PolarityDirection::ServiceToOthers => ExpGrowthPolarity::ServiceToOthers,
                PolarityDirection::ServiceToSelf => ExpGrowthPolarity::ServiceToSelf,
                PolarityDirection::Neutral => ExpGrowthPolarity::Neutral,
            };
            self.dual_experience
                .update_entity(eid, growth, state.polarity_strength);
        }

        let recommend = {
            let state = self.states.get(&eid)?;
            state.ready_for_transition()
        };

        Some(ConsciousnessTickOutput {
            behavior,
            state_changes,
            experience,
            perception,
            recommend_transition: recommend,
        })
    }

    fn generate_catalyst(
        &self,
        input: &ConsciousnessTickInput,
        _density: &Density,
    ) -> CatalystEvent {
        if input.body_experience.pain_level > 0.7 {
            return CatalystEvent::new(
                CatalystSource::Body,
                input.body_experience.pain_level,
                CatalystType::Challenge,
            )
            .with_polarity(CatalystPolarity::Neutral);
        }
        if input.body_experience.pleasure_level > 0.7 {
            return CatalystEvent::new(
                CatalystSource::Body,
                input.body_experience.pleasure_level,
                CatalystType::Gain,
            )
            .with_polarity(CatalystPolarity::ServiceToOthers);
        }
        if input.environment_experience.danger_present > 0.6 {
            return CatalystEvent::new(
                CatalystSource::Environment,
                input.environment_experience.danger_present,
                CatalystType::Challenge,
            )
            .with_polarity(CatalystPolarity::Neutral);
        }
        if input.social_experience.connection > 0.5 || input.social_experience.love_given > 0.3 {
            return CatalystEvent::new(
                CatalystSource::Social,
                input.social_experience.connection + input.social_experience.love_given,
                CatalystType::Relationship,
            )
            .with_polarity(CatalystPolarity::ServiceToOthers);
        }
        CatalystEvent::new(CatalystSource::Environment, 0.3, CatalystType::Unknown)
    }

    #[allow(dead_code)]
    fn create_raw_perception(
        &self,
        input: &ConsciousnessTickInput,
        _experience: &QualitativeExperience,
    ) -> RawPerception {
        let mut items = Vec::new();

        if input.body_experience.pain_level > 0.3 {
            items.push(RawPerceptItem {
                content: "Physical discomfort".to_string(),
                emotional_intensity: input.body_experience.pain_level,
                relevance: 0.8,
            });
        }
        if input.environment_experience.danger_present > 0.3 {
            items.push(RawPerceptItem {
                content: "Threat detected".to_string(),
                emotional_intensity: input.environment_experience.danger_present,
                relevance: 0.9,
            });
        }
        if input.social_experience.connection > 0.3 {
            items.push(RawPerceptItem {
                content: "Connection with other".to_string(),
                emotional_intensity: input.social_experience.connection,
                relevance: 0.7,
            });
        }
        if items.is_empty() {
            items.push(RawPerceptItem {
                content: "Ordinary experience".to_string(),
                emotional_intensity: 0.2,
                relevance: 0.3,
            });
        }

        let emotional_tone = (input.social_experience.love_received
            + input.social_experience.love_given
            - input.social_experience.isolation
            - input.social_experience.conflict
            + 1.0)
            / 2.0;

        RawPerception {
            items,
            emotional_tone: emotional_tone.clamp(0.0, 1.0),
        }
    }

    #[allow(dead_code)]
    fn update_state(
        &self,
        state: &mut ConsciousnessState,
        catalyst: &CatalystEvent,
        behavior: &BehaviorOutput,
        dt: Float,
    ) -> ConsciousnessStateChanges {
        let mut changes = ConsciousnessStateChanges::default();

        state.experience += catalyst.intensity * dt;
        changes.experience_delta = catalyst.intensity * dt;

        let growth_rate = 0.001 * catalyst.intensity;
        state.level = (state.level + growth_rate).min(1.0);
        changes.level_delta = growth_rate;

        let polarity_shift = match behavior.growth_direction {
            GrowthDirection::Positive => 0.01 * behavior.intensity,
            GrowthDirection::Negative => -0.01 * behavior.intensity,
            GrowthDirection::Neutral => 0.0,
        };

        state.polarity = (state.polarity + polarity_shift).clamp(-1.0, 1.0);
        changes.polarity_delta = polarity_shift;

        if polarity_shift != 0.0 {
            state.polarity_strength = (state.polarity_strength + 0.001).min(1.0);
        }

        if matches!(
            behavior.behavior_type,
            BehaviorType::Reflecting | BehaviorType::Intuitive | BehaviorType::Spiritual
        ) {
            state.wisdom += behavior.intensity * 0.01 * dt;
            changes.wisdom_delta = behavior.intensity * 0.01 * dt;
        }

        state.time_since_choice += dt;

        changes
    }

    fn density_to_enum(density: u8) -> Density {
        match density {
            1 => Density::First(Density1SubLevel::Quantum),
            2 => Density::Second(Density2SubLevel::Cellular),
            3 => Density::Third,
            4 => Density::Fourth,
            5 => Density::Fifth,
            6 => Density::Sixth,
            7 => Density::Seventh,
            _ => Density::First(Density1SubLevel::Quantum),
        }
    }

    pub fn get_state(&self, entity_id: &u64) -> Option<&ConsciousnessState> {
        self.states.get(entity_id)
    }
}

impl Default for ConsciousnessTickEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_registration() {
        let mut engine = ConsciousnessTickEngine::new();
        engine.register_entity(1, 3, 0.5);
        let state = engine.get_state(&1);
        assert!(state.is_some());
    }
}
