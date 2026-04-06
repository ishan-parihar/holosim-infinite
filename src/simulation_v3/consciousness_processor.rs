//! Consciousness Processor - Archetype-Driven Behavior Emergence
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
//! "Behavior emerges from archetype interference (no behavior trees)"
//! "The 22-archetype cycle: Mind → Body → Spirit → Choice"
//! "Free Will choice mechanism"
//! "Polarity development from choices"
//!
//! This module implements:
//! - ConsciousnessProcessor: Unified processor integrating consciousness components
//! - ConsciousnessOutput: Result of processing a tick
//! - Catalyst generation from sensory fields
//! - Action execution on bodies
//! - Polarity tracking from choices

use crate::biology::archetype_processor::{
    BehaviorOutput, BehaviorType, CatalystEvent, CatalystPolarity, CatalystSource, CatalystType,
    EntityArchetypeProcessor, GrowthDirection,
};
use crate::consciousness::free_will::{ChoiceResult, PolarityPreference};
use crate::consciousness::kernel::{
    ConsciousnessKernel, ConsciousnessSignal, Experience, KernelChoiceContext,
};
use crate::entity_layer7::layer7::EntityId;
use crate::entity_layer7::layer7::EntityState;
use crate::simulation_v3::archetype_basis::ArchetypeActivationProfile;
use crate::simulation_v3::archetypical_interference_engine::{
    ActionVector, ArchetypicalInterferenceEngine,
};
use crate::simulation_v3::embodied_body::SensoryField;
use crate::simulation_v3::free_will_seed::FreeWillSeed;
use crate::types::Float;
use std::collections::VecDeque;

/// Maximum number of experiences to buffer for learning
const MAX_EXPERIENCE_BUFFER: usize = 100;

/// Output from consciousness processing for a single tick
///
/// This represents the result of one consciousness tick,
/// including any actions, choices, and state changes.
#[derive(Debug, Clone)]
pub struct ConsciousnessOutput {
    /// The action vector to execute (if any)
    pub action: Option<ActionVector>,
    /// The choice that was made (if any)
    pub choice_made: Option<ChoiceResult>,
    /// Shift in polarity this tick (-1.0 STS to 1.0 STO)
    pub polarity_shift: Float,
    /// Change in consciousness level
    pub consciousness_change: Float,
    /// Type of behavior that emerged
    pub behavior_type: BehaviorType,
    /// Whether the entity is harvest-ready after this tick
    pub is_harvest_ready: bool,
    /// The sensory catalysts that were processed
    pub catalysts_processed: usize,
}

impl Default for ConsciousnessOutput {
    fn default() -> Self {
        ConsciousnessOutput {
            action: None,
            choice_made: None,
            polarity_shift: 0.0,
            consciousness_change: 0.0,
            behavior_type: BehaviorType::Neutral,
            is_harvest_ready: false,
            catalysts_processed: 0,
        }
    }
}

impl ConsciousnessOutput {
    /// Check if any action was produced
    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    /// Check if a choice was made
    pub fn has_choice(&self) -> bool {
        self.choice_made.is_some()
    }

    /// Get the polarity direction of the choice (if any)
    pub fn polarity_direction(&self) -> Option<Float> {
        self.choice_made.as_ref().map(|c| c.choice.sto_alignment)
    }
}

/// Consciousness Processor - unified processor for entity consciousness
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
/// "Integrate archetype-driven consciousness into the causal simulation loop"
///
/// This processor coordinates:
/// 1. ConsciousnessKernel - core consciousness state and choices
/// 2. EntityArchetypeProcessor - catalyst processing through 22 archetypes
/// 3. ArchetypicalInterferenceEngine - behavior emergence from interference
///
/// The processing flow:
/// 1. Receive sensory field from body
/// 2. Generate catalysts from sensations
/// 3. Process catalysts through archetype processor
/// 4. Generate interference pattern
/// 5. Collapse to action via Free Will
/// 6. Execute action on body
/// 7. Update polarity and learning
#[derive(Debug, Clone)]
pub struct ConsciousnessProcessor {
    /// The entity this processor belongs to
    pub entity_id: EntityId,

    /// Core consciousness kernel (the "receiver" of consciousness)
    pub kernel: ConsciousnessKernel,

    /// Archetype processor for catalyst handling
    pub archetype_processor: EntityArchetypeProcessor,

    /// Interference engine for behavior emergence (NO BEHAVIOR TREES)
    pub interference_engine: ArchetypicalInterferenceEngine,

    /// Buffer of recent experiences for learning
    pub experience_buffer: VecDeque<Experience>,

    /// Current polarity accumulation (STO positive, STS negative)
    pub polarity_accumulation: Float,

    /// Number of choices made (for statistics)
    pub total_choices: usize,

    /// Number of STO choices
    pub sto_choices: usize,

    /// Number of STS choices
    pub sts_choices: usize,

    /// Current behavior output (for inspection)
    pub current_behavior: Option<BehaviorOutput>,

    /// Activation level (how active consciousness is)
    pub activation_level: Float,
}

impl ConsciousnessProcessor {
    /// Create a new consciousness processor for an entity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Every Entity can exercise Free Will because it's in the holographic seed"
    pub fn new(entity_id: EntityId) -> Self {
        // Create archetype processor
        let mut archetype_processor = EntityArchetypeProcessor::new(entity_id.as_u64());
        archetype_processor.init_free_will();

        // Create interference engine with archetype basis and free will seed
        let archetype_basis = crate::simulation_v3::archetype_basis::ArchetypeBasis::new(22);
        let free_will_seed = FreeWillSeed::random();
        let interference_engine =
            ArchetypicalInterferenceEngine::new(archetype_basis, free_will_seed);

        // Create consciousness kernel
        let kernel = ConsciousnessKernel::new();

        ConsciousnessProcessor {
            entity_id,
            kernel,
            archetype_processor,
            interference_engine,
            experience_buffer: VecDeque::with_capacity(MAX_EXPERIENCE_BUFFER),
            polarity_accumulation: 0.0,
            total_choices: 0,
            sto_choices: 0,
            sts_choices: 0,
            current_behavior: None,
            activation_level: 0.0,
        }
    }

    /// Create a consciousness processor with archetype activation profile
    pub fn with_archetype_activation(
        entity_id: EntityId,
        profile: ArchetypeActivationProfile,
    ) -> Self {
        let mut processor = Self::new(entity_id);
        // Set the archetype activation in the kernel
        // Note: The archetype processor will receive this through catalyst processing
        processor.kernel.archetype_activation = profile;

        processor
    }

    /// Main processing tick - process sensory field and produce output
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
    /// "Consciousness processing phase in tick"
    ///
    /// This method:
    /// 1. Generates catalysts from sensory field
    /// 2. Processes each catalyst through archetype processor
    /// 3. Generates interference pattern and behavior
    /// 4. Makes choice via Free Will if needed
    /// 5. Updates polarity and learning
    ///
    /// # Arguments
    /// * `sensory` - The sensory field from the body
    ///
    /// # Returns
    /// ConsciousnessOutput with actions and state changes
    pub fn process_tick(&mut self, sensory: &SensoryField) -> ConsciousnessOutput {
        let mut output = ConsciousnessOutput::default();

        // Update activation level based on sensory distress
        let distress = sensory.calculate_distress();
        self.activation_level = (self.activation_level * 0.9 + distress * 0.1).clamp(0.0, 1.0);

        // Generate catalysts from sensory field
        let catalysts = self.generate_catalysts_from_sensory(sensory);
        output.catalysts_processed = catalysts.len();

        // Process each catalyst
        let mut total_polarity_shift = 0.0;
        for catalyst in &catalysts {
            // Process through archetype processor
            let behavior = self.archetype_processor.process_catalyst(catalyst);
            self.current_behavior = Some(behavior.clone());

            // Update polarity based on behavior growth direction
            match behavior.growth_direction {
                GrowthDirection::Positive => total_polarity_shift += behavior.intensity * 0.1,
                GrowthDirection::Negative => total_polarity_shift -= behavior.intensity * 0.1,
                GrowthDirection::Neutral => {}
            }

            output.behavior_type = behavior.behavior_type;
        }

        // Generate interference pattern and emergent behavior
        if let Some(_behavior) = &self.current_behavior {
            // Use archetype activation profile from kernel
            let profile = self.kernel.archetype_activation.clone();

            // Evaluate emergent behavior through interference engine
            // This is where behavior emerges WITHOUT behavior trees
            match self.interference_engine.evaluate_behavior(
                &profile,
                self.kernel.time_space_ratio,
                3, // Third density
                Some(self.entity_id.as_u64()),
                Some(self.entity_id.as_u64()),
            ) {
                Ok(emergent) => {
                    output.action = Some(emergent.action_vector.clone());
                    output.consciousness_change = emergent.confidence * 0.01;
                }
                Err(_) => {
                    // If evaluation fails, still continue with archetype behavior
                }
            }
        }

        // Make choice through consciousness kernel if we have catalyst intensity
        if output.catalysts_processed > 0 {
            let catalyst_intensity = sensory.calculate_distress();
            let context = KernelChoiceContext {
                polarity_preference: self.determine_polarity_preference(),
                experience_bias: self.calculate_experience_bias(),
                environmental_pressure: sensory.threat_level,
                catalyst_intensity,
            };

            // Create entity state for choice
            let entity_state = self.create_entity_state_for_choice();

            let choice_result =
                self.kernel
                    .make_choice(&entity_state, &context, catalyst_intensity);
            output.choice_made = Some(choice_result.clone());

            // Update polarity tracking
            self.update_polarity_from_choice(&choice_result);

            // Record experience for learning
            let experience = Experience::new(
                self.total_choices as u64,
                catalyst_intensity,
                choice_result.effectiveness,
                choice_result.choice.sto_alignment as Float,
                "sensory_catalyst",
            );
            self.learn(&experience);

            output.polarity_shift =
                total_polarity_shift + choice_result.choice.sto_alignment as Float * 0.1;
        }

        // Check harvest readiness
        output.is_harvest_ready = self.kernel.is_harvest_ready();

        output
    }

    /// Generate catalysts from sensory field
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
    /// "Catalyst generation should map sensations to catalyst types"
    ///
    /// Mapping:
    /// - hunger > 0.7 → Challenge catalyst
    /// - pain > 0.5 → Health catalyst
    /// - threat_level > 0.5 → Challenge catalyst
    /// - heart_activation > 0.7 → Relationship catalyst (STO)
    pub fn generate_catalysts_from_sensory(&self, sensory: &SensoryField) -> Vec<CatalystEvent> {
        let mut catalysts = Vec::new();

        // Hunger catalyst
        if sensory.hunger > 0.7 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Body,
                    sensory.hunger,
                    CatalystType::Challenge,
                )
                .with_polarity(CatalystPolarity::Neutral),
            );
        }

        // Pain/Health catalyst
        if sensory.pain > 0.5 {
            catalysts.push(
                CatalystEvent::new(CatalystSource::Body, sensory.pain, CatalystType::Health)
                    .with_polarity(CatalystPolarity::Neutral),
            );
        }

        // Threat catalyst
        if sensory.threat_level > 0.5 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Environment,
                    sensory.threat_level,
                    CatalystType::Challenge,
                )
                .with_polarity(CatalystPolarity::Neutral),
            );
        }

        // Heart activation (relationship/love) catalyst - STO aligned
        if sensory.heart_activation > 0.7 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Social,
                    sensory.heart_activation,
                    CatalystType::Relationship,
                )
                .with_polarity(CatalystPolarity::ServiceToOthers),
            );
        }

        // Fatigue catalyst
        if sensory.fatigue > 0.6 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Body,
                    sensory.fatigue,
                    CatalystType::Challenge,
                )
                .with_polarity(CatalystPolarity::Neutral),
            );
        }

        // Stress catalyst
        if sensory.stress > 0.6 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Mind,
                    sensory.stress,
                    CatalystType::Challenge,
                )
                .with_polarity(CatalystPolarity::Neutral),
            );
        }

        // High vitality (opportunity for growth)
        if sensory.vitality > 0.8 {
            catalysts.push(
                CatalystEvent::new(
                    CatalystSource::Spirit,
                    sensory.vitality,
                    CatalystType::Opportunity,
                )
                .with_polarity(CatalystPolarity::ServiceToOthers),
            );
        }

        catalysts
    }

    /// Execute an action on the body
    ///
    /// This applies the action vector to modify body state.
    /// The action represents the collapsed behavioral intention.
    pub fn execute_action(
        &self,
        action: &ActionVector,
        body: &mut crate::simulation_v3::embodied_body::EmbodiedBody,
    ) {
        // The action magnitude affects energy consumption
        let energy_cost = action.magnitude * 0.1;
        body.organism.energy = (body.organism.energy - energy_cost).max(0.0);

        // Catalyst value from action can affect health/stress
        if action.catalyst_value > 0.5 {
            // High catalyst actions are transformative
            body.organism.health = (body.organism.health + action.catalyst_value * 0.01).min(1.0);
        }

        // Confidence affects vitality
        body.sensory_field.vitality =
            (body.sensory_field.vitality * 0.9 + action.confidence * 0.1).clamp(0.0, 1.0);
    }

    /// Update polarity from a choice result
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
    /// "Polarity updates: STO choices increase STO accumulation, STS choices increase STS"
    pub fn update_polarity_from_choice(&mut self, result: &ChoiceResult) {
        self.total_choices += 1;

        let sto_alignment = result.choice.sto_alignment;

        if sto_alignment > 0.0 {
            self.sto_choices += 1;
            self.polarity_accumulation += sto_alignment as Float * 0.1;
        } else if sto_alignment < 0.0 {
            self.sts_choices += 1;
            self.polarity_accumulation += sto_alignment as Float * 0.1;
        }

        // Clamp polarity accumulation
        self.polarity_accumulation = self.polarity_accumulation.clamp(-1.0, 1.0);
    }

    /// Learn from an experience
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Learning = improved spectrum_access_level"
    fn learn(&mut self, experience: &Experience) {
        // Add to buffer
        if self.experience_buffer.len() >= MAX_EXPERIENCE_BUFFER {
            self.experience_buffer.pop_front();
        }
        self.experience_buffer.push_back(experience.clone());

        // Kernel learning
        self.kernel.learn(experience);
    }

    /// Determine polarity preference based on accumulation
    fn determine_polarity_preference(&self) -> PolarityPreference {
        if self.polarity_accumulation > 0.3 {
            PolarityPreference::ServiceToOthers
        } else if self.polarity_accumulation < -0.3 {
            PolarityPreference::ServiceToSelf
        } else {
            PolarityPreference::Neutral
        }
    }

    /// Calculate experience bias from past choices
    fn calculate_experience_bias(&self) -> Float {
        if self.total_choices == 0 {
            return 0.0;
        }
        (self.sto_choices as Float - self.sts_choices as Float) / self.total_choices as Float
    }

    /// Create entity state for choice making
    fn create_entity_state_for_choice(&self) -> crate::entity_layer7::layer7::EntityState {
        use crate::entity_layer7::layer7::{PolarityState, VibrationalState};
        use crate::evolution_density_octave::density_octave::Density;

        let polarization_strength = self.polarity_accumulation.abs();

        EntityState {
            vibrational_state: VibrationalState {
                frequency: self.kernel.consciousness_level,
                amplitude: self.activation_level,
                coherence: self.kernel.source_resonance,
                density: Density::Third,
                potential_energy: self.kernel.spectrum_access_level,
                kinetic_energy: self.activation_level,
            },
            polarity_state: PolarityState {
                polarity_bias: self.polarity_accumulation,
                polarization_strength,
            },
            consciousness_level: self.kernel.consciousness_level,
            experience_accumulation: self.experience_buffer.len() as Float,
            learning_progress: self.kernel.spectrum_access_level,
        }
    }

    /// Get current polarity ratio (STO ratio)
    pub fn sto_ratio(&self) -> Float {
        if self.total_choices == 0 {
            return 0.5;
        }
        self.sto_choices as Float / self.total_choices as Float
    }

    /// Get consciousness signal for broadcast
    pub fn broadcast_signal(&self) -> ConsciousnessSignal {
        self.kernel.broadcast_signal()
    }

    /// Check if entity is harvest-ready
    pub fn is_harvest_ready(&self) -> bool {
        self.kernel.is_harvest_ready()
    }

    /// Get archetype activation profile
    pub fn archetype_profile(&self) -> &ArchetypeActivationProfile {
        &self.kernel.archetype_activation
    }

    /// Get current behavior if any
    pub fn current_behavior(&self) -> Option<&BehaviorOutput> {
        self.current_behavior.as_ref()
    }

    /// Get consciousness level
    pub fn consciousness_level(&self) -> Float {
        self.kernel.consciousness_level
    }

    /// Get polarization intensity
    pub fn polarization_intensity(&self) -> Float {
        self.kernel.polarization_intensity()
    }

    /// Get polarity accumulation
    pub fn polarity_accumulation(&self) -> Float {
        self.polarity_accumulation
    }

    /// Get total choices made
    pub fn total_choices(&self) -> usize {
        self.total_choices
    }

    /// Get activation level
    pub fn activation_level(&self) -> Float {
        self.activation_level
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_processor_creation() {
        let entity_id = EntityId::new("test-entity".to_string());
        let processor = ConsciousnessProcessor::new(entity_id.clone());

        assert_eq!(processor.entity_id, entity_id);
        assert_eq!(processor.total_choices, 0);
        assert_eq!(processor.polarity_accumulation, 0.0);
        assert!(processor.experience_buffer.is_empty());
    }

    #[test]
    fn test_consciousness_output_default() {
        let output = ConsciousnessOutput::default();

        assert!(output.action.is_none());
        assert!(output.choice_made.is_none());
        assert_eq!(output.polarity_shift, 0.0);
        assert_eq!(output.behavior_type, BehaviorType::Neutral);
        assert!(!output.is_harvest_ready);
    }

    #[test]
    fn test_generate_catalysts_from_sensory() {
        let entity_id = EntityId::new("test-entity".to_string());
        let processor = ConsciousnessProcessor::new(entity_id);

        // Create sensory field with various activations
        let sensory = SensoryField {
            hunger: 0.8,           // Should generate Challenge catalyst
            pain: 0.6,             // Should generate Health catalyst
            threat_level: 0.6,     // Should generate Challenge catalyst
            heart_activation: 0.8, // Should generate Relationship catalyst
            ..Default::default()
        };

        let catalysts = processor.generate_catalysts_from_sensory(&sensory);

        // Should have at least 4 catalysts
        assert!(catalysts.len() >= 4);

        // Check for hunger catalyst
        assert!(catalysts.iter().any(
            |c| c.source == CatalystSource::Body && c.catalyst_type == CatalystType::Challenge
        ));

        // Check for health catalyst
        assert!(catalysts
            .iter()
            .any(|c| c.catalyst_type == CatalystType::Health));

        // Check for STO relationship catalyst
        assert!(catalysts
            .iter()
            .any(|c| c.polarity == CatalystPolarity::ServiceToOthers
                && c.catalyst_type == CatalystType::Relationship));
    }

    #[test]
    fn test_process_tick_basic() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        let sensory = SensoryField::default();
        let _output = processor.process_tick(&sensory);

        // catalysts_processed is usize, always >= 0
    }

    #[test]
    fn test_process_tick_with_catalysts() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Create sensory field that will generate catalysts
        let sensory = SensoryField {
            hunger: 0.9,
            pain: 0.7,
            threat_level: 0.8,
            ..Default::default()
        };

        let output = processor.process_tick(&sensory);

        // Should have processed catalysts
        assert!(output.catalysts_processed > 0);
    }

    #[test]
    fn test_update_polarity_sto() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Create a choice result with STO alignment
        let choice_result = ChoiceResult {
            choice: crate::consciousness::free_will::Choice {
                selected_index: 0,
                confidence: 0.8,
                sto_alignment: 1.0, // STO
            },
            conscious_selection: None,
            possibility_space: crate::foundation::indigo_realm::PossibilitySpace::new(
                crate::foundation::indigo_realm::EntityConstraints::default(),
            ),
            record: crate::consciousness::free_will::ChoiceRecord {
                choice_id: 0,
                possibility_space_size: 3,
                selected_index: 0,
                confidence: 0.8,
                sto_alignment: 1.0,
                timestamp: 0,
            },
            effectiveness: 0.8,
        };

        processor.update_polarity_from_choice(&choice_result);

        assert_eq!(processor.sto_choices, 1);
        assert!(processor.polarity_accumulation > 0.0);
    }

    #[test]
    fn test_update_polarity_sts() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Create a choice result with STS alignment
        let choice_result = ChoiceResult {
            choice: crate::consciousness::free_will::Choice {
                selected_index: 0,
                confidence: 0.8,
                sto_alignment: -1.0, // STS
            },
            conscious_selection: None,
            possibility_space: crate::foundation::indigo_realm::PossibilitySpace::new(
                crate::foundation::indigo_realm::EntityConstraints::default(),
            ),
            record: crate::consciousness::free_will::ChoiceRecord {
                choice_id: 0,
                possibility_space_size: 3,
                selected_index: 0,
                confidence: 0.8,
                sto_alignment: -1.0,
                timestamp: 0,
            },
            effectiveness: 0.8,
        };

        processor.update_polarity_from_choice(&choice_result);

        assert_eq!(processor.sts_choices, 1);
        assert!(processor.polarity_accumulation < 0.0);
    }

    #[test]
    fn test_sto_ratio() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Initially 0.5 (neutral)
        assert_eq!(processor.sto_ratio(), 0.5);

        // After STO choice
        processor.sto_choices = 7;
        processor.total_choices = 10;
        assert!((processor.sto_ratio() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_determine_polarity_preference() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Neutral
        assert_eq!(
            processor.determine_polarity_preference(),
            PolarityPreference::Neutral
        );

        // STO
        processor.polarity_accumulation = 0.5;
        assert_eq!(
            processor.determine_polarity_preference(),
            PolarityPreference::ServiceToOthers
        );

        // STS
        processor.polarity_accumulation = -0.5;
        assert_eq!(
            processor.determine_polarity_preference(),
            PolarityPreference::ServiceToSelf
        );
    }

    #[test]
    fn test_broadcast_signal() {
        let entity_id = EntityId::new("test-entity".to_string());
        let processor = ConsciousnessProcessor::new(entity_id);

        let signal = processor.broadcast_signal();

        assert!(signal.consciousness_level > 0.0);
        assert!(signal.clarity >= 0.0);
    }

    #[test]
    fn test_experience_buffer() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut processor = ConsciousnessProcessor::new(entity_id);

        // Process multiple ticks to add to experience buffer
        for _ in 0..5 {
            let sensory = SensoryField {
                hunger: 0.8,
                ..Default::default()
            };
            processor.process_tick(&sensory);
        }

        // Should have experiences
        assert!(!processor.experience_buffer.is_empty());
    }

    #[test]
    fn test_with_archetype_activation() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(0, 0.8).unwrap();
        profile.set(7, 0.6).unwrap();

        let processor =
            ConsciousnessProcessor::with_archetype_activation(entity_id, profile.clone());

        // Should have matching profile
        assert_eq!(processor.kernel.archetype_activation.get(0), Some(0.8));
    }
}
