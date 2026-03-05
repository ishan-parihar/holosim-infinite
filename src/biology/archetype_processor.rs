//! Archetype Processor - Entity Behavior from Archetype Interference
//!
//! This module integrates the 22-archetype system with embodied entities
//! so that archetype processing drives entity behavior each tick.
//!
//! From V4 Roadmap Phase 5: "Archetype-Driven Consciousness Engine"
//! Gap #12 resolution: "Archetype system not driving entity behavior"

use crate::archetypes::ArchetypeSystem;
use crate::consciousness::free_will::FreeWillKernel;
use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
use crate::types::Float;
use rand::Rng;

// ============================================================================
// Archetype Activation State
// ============================================================================

/// Current activation levels for all 22 archetypes in an entity
#[derive(Debug, Clone, Default)]
pub struct ArchetypeActivationState {
    /// Mind Complex (A1-A7) activations
    pub mind: [Float; 7],
    /// Body Complex (A8-A14) activations
    pub body: [Float; 7],
    /// Spirit Complex (A15-A21) activations
    pub spirit: [Float; 7],
    /// Choice (A22) activation
    pub choice: Float,
}

impl ArchetypeActivationState {
    /// Get activation for a specific archetype
    pub fn get(&self, archetype_id: u8) -> Float {
        match archetype_id {
            1..=7 => self.mind[(archetype_id - 1) as usize],
            8..=14 => self.body[(archetype_id - 8) as usize],
            15..=21 => self.spirit[(archetype_id - 15) as usize],
            22 => self.choice,
            _ => 0.0,
        }
    }

    /// Set activation for a specific archetype
    pub fn set(&mut self, archetype_id: u8, value: Float) {
        match archetype_id {
            1..=7 => self.mind[(archetype_id - 1) as usize] = value.clamp(0.0, 1.0),
            8..=14 => self.body[(archetype_id - 8) as usize] = value.clamp(0.0, 1.0),
            15..=21 => self.spirit[(archetype_id - 15) as usize] = value.clamp(0.0, 1.0),
            22 => self.choice = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    /// Get all 22 activations as a flat array
    pub fn to_array(&self) -> [Float; 22] {
        let mut result = [0.0; 22];
        for i in 0..7 {
            result[i] = self.mind[i];
            result[i + 7] = self.body[i];
            result[i + 14] = self.spirit[i];
        }
        result[21] = self.choice;
        result
    }
}

// ============================================================================
// Catalyst Event
// ============================================================================

/// External or internal catalyst that triggers archetype processing
#[derive(Debug, Clone)]
pub struct CatalystEvent {
    /// Source of the catalyst
    pub source: CatalystSource,
    /// Intensity of the catalyst (0.0-1.0)
    pub intensity: Float,
    /// Type of catalyst
    pub catalyst_type: CatalystType,
    /// Whether this is a positive or negative catalyst
    pub polarity: CatalystPolarity,
}

/// Where the catalyst originated
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatalystSource {
    /// From the physical body (hunger, pain, pleasure)
    Body,
    /// From the environment (encounters, situations)
    Environment,
    /// From other entities (relationships)
    Social,
    /// From the mind itself (thoughts, memories)
    Mind,
    /// From spirit (intuition, download)
    Spirit,
}

/// Classification of catalyst
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatalystType {
    /// Challenge that requires growth
    Challenge,
    /// Opportunity for service or growth
    Opportunity,
    /// Loss or ending
    Loss,
    /// Gain or beginning
    Gain,
    /// Identity question
    Identity,
    /// Relationship issue
    Relationship,
    /// Purpose question
    Purpose,
    /// Health crisis
    Health,
    /// Unknown/miscellaneous
    Unknown,
}

/// Polarity of the catalyst
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatalystPolarity {
    /// Catalyst that moves toward service-to-others
    ServiceToOthers,
    /// Catalyst that moves toward service-to-self
    ServiceToSelf,
    /// Catalyst that is polarity-neutral
    Neutral,
}

impl CatalystEvent {
    /// Create a new catalyst event
    pub fn new(source: CatalystSource, intensity: Float, catalyst_type: CatalystType) -> Self {
        Self {
            source,
            intensity: intensity.clamp(0.0, 1.0),
            catalyst_type,
            polarity: CatalystPolarity::Neutral,
        }
    }

    /// Apply polarity bias to the catalyst
    pub fn with_polarity(mut self, polarity: CatalystPolarity) -> Self {
        self.polarity = polarity;
        self
    }
}

// ============================================================================
// Behavior Output
// ============================================================================

/// The behavior that emerges from archetype interference
#[derive(Debug, Clone)]
pub struct BehaviorOutput {
    /// Primary behavior category
    pub behavior_type: BehaviorType,
    /// Intensity of the behavior urge (0.0-1.0)
    pub intensity: Float,
    /// Confidence in this behavior (higher = less wavering)
    pub confidence: Float,
    /// Whether this represents growth or regression
    pub growth_direction: GrowthDirection,
    /// Detailed behavioral impulse description
    pub impulse: String,
}

/// Categories of behavior that can emerge
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BehaviorType {
    /// Survival-oriented action
    Survival,
    /// Creative expression
    Creative,
    /// Will exercises (assertion, discipline)
    Willful,
    /// Loving action (giving, accepting love)
    Loving,
    /// Communication (speaking, listening)
    Communicating,
    /// Intuitive insight
    Intuitive,
    /// Spiritual connection
    Spiritual,
    /// Seeking knowledge/truth
    Seeking,
    /// Social interaction
    Socializing,
    /// Rest/integration
    Resting,
    /// Flight from situation
    Avoiding,
    /// Confrontation
    Confronting,
    /// Reflection/meditation
    Reflecting,
    /// No strong impulse
    Neutral,
}

/// Whether this behavior represents growth or regression
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthDirection {
    /// Moving toward unity/love
    Positive,
    /// Moving toward separation/self
    Negative,
    /// Neutral/stabilizing
    Neutral,
}

// ============================================================================
// Main Archetype Processor
// ============================================================================

/// Processes archetypes for an entity each tick, transforming catalysts into behavior
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The 22 archetypes provide the computational architecture for consciousness"
#[derive(Debug, Clone)]
pub struct EntityArchetypeProcessor {
    /// Entity this processor belongs to
    pub entity_id: u64,
    /// The archetype system (22 archetypes)
    archetype_system: ArchetypeSystem,
    /// Current activation levels
    activations: ArchetypeActivationState,
    /// History of catalyst processing
    catalyst_history: Vec<CatalystRecord>,
    /// Current behavioral impulse
    current_behavior: Option<BehaviorOutput>,
    /// Lesser cycle state
    lesser_cycle: LesserCycleState,
    /// Greater cycle state  
    greater_cycle: GreaterCycleState,
    /// Free will kernel for choice
    free_will: Option<FreeWillKernel>,
    /// Density level affects processing
    density: Density,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CatalystRecord {
    catalyst: CatalystEvent,
    processed_at: Float,
    resulting_behavior: Option<BehaviorType>,
}

#[derive(Debug, Clone, Default)]
struct LesserCycleState {
    stage: u8, // 0-3 for A1->A2->A3->A4
    accumulated_input: Float,
    efficiency: Float,
}

#[derive(Debug, Clone, Default)]
struct GreaterCycleState {
    stage: u8, // 0-3 for A5->A22->A6->A7
    transformation_progress: Float,
    wisdom_accumulated: Float,
}

impl EntityArchetypeProcessor {
    /// Create a new archetype processor for an entity
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            archetype_system: ArchetypeSystem::new(),
            activations: ArchetypeActivationState::default(),
            catalyst_history: Vec::new(),
            current_behavior: None,
            lesser_cycle: LesserCycleState::default(),
            greater_cycle: GreaterCycleState::default(),
            free_will: None,
            density: Density::First(Density1SubLevel::Quantum),
        }
    }

    /// Set the entity's density level
    pub fn set_density(&mut self, density: Density) {
        self.density = density;
    }

    /// Initialize free will kernel
    pub fn init_free_will(&mut self) {
        // Create foundation archetype22
        let foundation_archetype22 = crate::foundation::indigo_realm::Archetype22::new();
        self.free_will = Some(FreeWillKernel::new(foundation_archetype22));
    }

    /// Process a catalyst event through the archetype system
    ///
    /// This is the main entry point - catalysts from body, environment,
    /// social, mind, or spirit are processed through the 22 archetypes
    /// to produce behavior.
    pub fn process_catalyst(&mut self, catalyst: &CatalystEvent) -> &BehaviorOutput {
        let mut rng = rand::thread_rng();

        // Step 1: Update archetype activations based on catalyst
        self.update_activations(catalyst);

        // Step 2: Run the Lesser Cycle (A1->A2->A3->A4)
        let lesser_result = self.run_lesser_cycle(catalyst, &mut rng);

        // Step 3: Run the Greater Cycle (A5->A22->A6->A7)
        let greater_result = self.run_greater_cycle(catalyst, &mut rng);

        // Step 4: Apply body complex archetypes (A8-A14)
        self.process_body_complex(catalyst);

        // Step 5: Apply spirit complex archetypes (A15-A21)
        self.process_spirit_complex(catalyst);

        // Step 6: Integrate through choice (A22)
        let choice_result = self.process_choice(lesser_result, greater_result, &mut rng);

        // Step 7: Generate behavior from archetype interference
        let behavior = self.generate_behavior(catalyst, choice_result);

        // Record in history
        self.catalyst_history.push(CatalystRecord {
            catalyst: catalyst.clone(),
            processed_at: 0.0, // Would use simulation time
            resulting_behavior: Some(behavior.behavior_type),
        });

        // Keep history bounded
        if self.catalyst_history.len() > 100 {
            self.catalyst_history.remove(0);
        }

        self.current_behavior = Some(behavior);
        self.current_behavior.as_ref().unwrap()
    }

    /// Update archetype activations based on incoming catalyst
    fn update_activations(&mut self, catalyst: &CatalystEvent) {
        // Different catalyst sources affect different complexes
        match catalyst.source {
            CatalystSource::Body => {
                // Body catalysts affect body complex (A8-A14)
                let base = catalyst.intensity;
                self.activations.body[0] = (self.activations.body[0] + base * 0.1).clamp(0.0, 1.0);
                self.activations.body[2] = (self.activations.body[2] + base * 0.15).clamp(0.0, 1.0);
            }
            CatalystSource::Environment => {
                // Environment affects mind complex
                let base = catalyst.intensity;
                self.activations.mind[2] = (self.activations.mind[2] + base * 0.1).clamp(0.0, 1.0);
                self.activations.mind[3] = (self.activations.mind[3] + base * 0.1).clamp(0.0, 1.0);
            }
            CatalystSource::Social => {
                // Social catalysts affect both mind and spirit
                let base = catalyst.intensity;
                self.activations.mind[4] = (self.activations.mind[4] + base * 0.1).clamp(0.0, 1.0);
                self.activations.spirit[2] =
                    (self.activations.spirit[2] + base * 0.1).clamp(0.0, 1.0);
            }
            CatalystSource::Mind => {
                // Mind catalysts affect mind complex
                let base = catalyst.intensity;
                self.activations.mind[0] = (self.activations.mind[0] + base * 0.1).clamp(0.0, 1.0);
                self.activations.mind[5] = (self.activations.mind[5] + base * 0.1).clamp(0.0, 1.0);
            }
            CatalystSource::Spirit => {
                // Spirit catalysts affect spirit complex
                let base = catalyst.intensity;
                self.activations.spirit[0] =
                    (self.activations.spirit[0] + base * 0.1).clamp(0.0, 1.0);
                self.activations.spirit[3] =
                    (self.activations.spirit[3] + base * 0.15).clamp(0.0, 1.0);
            }
        }

        // All catalysts slightly activate choice
        self.activations.choice =
            (self.activations.choice + catalyst.intensity * 0.05).clamp(0.0, 1.0);
    }

    /// Run the Lesser Cycle: Matrix(A1) -> Potentiator(A2) -> Catalyst(A3) -> Experience(A4)
    fn run_lesser_cycle(
        &mut self,
        catalyst: &CatalystEvent,
        rng: &mut impl Rng,
    ) -> LesserCycleOutput {
        // Stage progression
        self.lesser_cycle.stage = (self.lesser_cycle.stage + 1) % 4;
        self.lesser_cycle.accumulated_input += catalyst.intensity;

        

        match self.lesser_cycle.stage {
            0 => {
                // Matrix (A1) - capacity for mind
                let matrix_value = self.activations.mind[0];
                LesserCycleOutput {
                    processed: matrix_value * catalyst.intensity,
                    wisdom_generated: matrix_value * 0.1,
                    transformation_needed: false,
                }
            }
            1 => {
                // Potentiator (A2) - resource access
                let pot_value = self.activations.mind[1];
                LesserCycleOutput {
                    processed: pot_value * self.lesser_cycle.accumulated_input,
                    wisdom_generated: pot_value * 0.05,
                    transformation_needed: false,
                }
            }
            2 => {
                // Catalyst (A3) - the actual processing
                let cat_value = self.activations.mind[2];
                let randomness: Float = rng.gen();
                LesserCycleOutput {
                    processed: cat_value
                        * self.lesser_cycle.accumulated_input
                        * (0.8 + randomness * 0.4),
                    wisdom_generated: cat_value * 0.15,
                    transformation_needed: cat_value > 0.7,
                }
            }
            _ => {
                // Experience (A4) - storage/integration
                let exp_value = self.activations.mind[3];
                self.lesser_cycle.efficiency = exp_value;
                LesserCycleOutput {
                    processed: exp_value * catalyst.intensity,
                    wisdom_generated: exp_value * 0.2,
                    transformation_needed: false,
                }
            }
        }
    }

    /// Run the Greater Cycle: Significator(A5) -> Choice(A22) -> Transformation(A6) -> GreatWay(A7)
    fn run_greater_cycle(
        &mut self,
        catalyst: &CatalystEvent,
        rng: &mut impl Rng,
    ) -> GreaterCycleOutput {
        self.greater_cycle.stage = (self.greater_cycle.stage + 1) % 4;

        

        match self.greater_cycle.stage {
            0 => {
                // Significator (A5) - identifies what the catalyst means
                let sig_value = self.activations.mind[4];
                GreaterCycleOutput {
                    meaning_extracted: sig_value * catalyst.intensity,
                    wisdom_cost: 0.1,
                    transformation_triggered: false,
                }
            }
            1 => {
                // Choice (A22) - the moment of decision
                let choice_value = self.activations.choice;
                let randomness: Float = rng.gen();
                GreaterCycleOutput {
                    meaning_extracted: choice_value * catalyst.intensity,
                    wisdom_cost: 0.2,
                    transformation_triggered: choice_value > 0.6 && randomness > 0.5,
                }
            }
            2 => {
                // Transformation (A6) - actual change
                let trans_value = self.activations.mind[5];
                if trans_value > 0.5 {
                    self.greater_cycle.transformation_progress += catalyst.intensity * 0.1;
                }
                GreaterCycleOutput {
                    meaning_extracted: trans_value * catalyst.intensity,
                    wisdom_cost: 0.3,
                    transformation_triggered: self.greater_cycle.transformation_progress > 0.8,
                }
            }
            _ => {
                // Great Way (A7) - path forward
                let gw_value = self.activations.mind[6];
                self.greater_cycle.wisdom_accumulated += gw_value * catalyst.intensity * 0.1;
                GreaterCycleOutput {
                    meaning_extracted: gw_value * catalyst.intensity,
                    wisdom_cost: 0.0,
                    transformation_triggered: false,
                }
            }
        }
    }

    /// Process body complex archetypes (A8-A14)
    fn process_body_complex(&mut self, catalyst: &CatalystEvent) {
        // Body catalysts activate different body archetypes
        match catalyst.source {
            CatalystSource::Body => {
                // A10 (Body Catalyst) - physical sensations
                self.activations.body[2] =
                    (self.activations.body[2] + catalyst.intensity * 0.2).clamp(0.0, 1.0);
            }
            CatalystSource::Social => {
                // A12 (Body Significator) - how body relates to others
                self.activations.body[4] =
                    (self.activations.body[4] + catalyst.intensity * 0.15).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }

    /// Process spirit complex archetypes (A15-A21)
    fn process_spirit_complex(&mut self, catalyst: &CatalystEvent) {
        // Spirit catalysts
        match catalyst.source {
            CatalystSource::Spirit => {
                // A17 (Spirit Catalyst) - spiritual challenges
                self.activations.spirit[2] =
                    (self.activations.spirit[2] + catalyst.intensity * 0.2).clamp(0.0, 1.0);
            }
            CatalystSource::Mind => {
                // A19 (Spirit Significator) - spiritual meaning
                self.activations.spirit[4] =
                    (self.activations.spirit[4] + catalyst.intensity * 0.15).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }

    /// Process through choice archetype (A22) - integrate all complexes
    fn process_choice(
        &self,
        lesser: LesserCycleOutput,
        greater: GreaterCycleOutput,
        rng: &mut impl Rng,
    ) -> ChoiceOutput {
        // The choice archetype integrates mind, body, and spirit
        let integration =
            (lesser.processed + greater.meaning_extracted + self.activations.choice) / 3.0;

        // Apply free will if available
        let mut choice_power = integration;
        let has_free_will = self.free_will.is_some();
        if has_free_will {
            // Free will adds non-deterministic element
            let randomness: Float = rng.gen();
            choice_power *= 0.7 + randomness * 0.3;
        }

        ChoiceOutput {
            integration_level: choice_power,
            wisdom_required: greater.wisdom_cost + 0.1,
            transformation_offered: greater.transformation_triggered,
        }
    }

    /// Generate behavior from archetype interference pattern
    fn generate_behavior(&self, catalyst: &CatalystEvent, choice: ChoiceOutput) -> BehaviorOutput {
        // The behavior emerges from which archetypes are most activated
        // This is the "interference pattern" that creates behavior

        let mut rng = rand::thread_rng();

        // Find dominant archetype complex
        let mind_max = self
            .activations
            .mind
            .iter()
            .cloned()
            .fold(0.0f64, |a, b| a.max(b));
        let body_max = self
            .activations
            .body
            .iter()
            .cloned()
            .fold(0.0f64, |a, b| a.max(b));
        let spirit_max = self
            .activations
            .spirit
            .iter()
            .cloned()
            .fold(0.0f64, |a, b| a.max(b));

        let (dominant_complex, max_activation) = if mind_max >= body_max && mind_max >= spirit_max {
            ("mind", mind_max)
        } else if body_max >= mind_max && body_max >= spirit_max {
            ("body", body_max)
        } else {
            ("spirit", spirit_max)
        };

        // Determine behavior type from dominant complex
        let (behavior_type, impulse) = match dominant_complex {
            "mind" => {
                // Find most activated mind archetype
                let mut max_idx = 0;
                let mut max_val = 0.0;
                for (i, &v) in self.activations.mind.iter().enumerate() {
                    if v > max_val {
                        max_val = v;
                        max_idx = i;
                    }
                }
                match max_idx {
                    0 => (
                        BehaviorType::Seeking,
                        "Seeking to understand the nature of reality".to_string(),
                    ),
                    1 => (
                        BehaviorType::Creative,
                        "Exploring potential and possibility".to_string(),
                    ),
                    2 => (
                        BehaviorType::Confronting,
                        "Facing the challenge presented".to_string(),
                    ),
                    3 => (
                        BehaviorType::Reflecting,
                        "Processing and integrating experience".to_string(),
                    ),
                    4 => (
                        BehaviorType::Socializing,
                        "Understanding one's role in the whole".to_string(),
                    ),
                    5 => (
                        BehaviorType::Intuitive,
                        "Undergoing inner transformation".to_string(),
                    ),
                    _ => (
                        BehaviorType::Spiritual,
                        "Walking the path of service".to_string(),
                    ),
                }
            }
            "body" => {
                let mut max_idx = 0;
                let mut max_val = 0.0;
                for (i, &v) in self.activations.body.iter().enumerate() {
                    if v > max_val {
                        max_val = v;
                        max_idx = i;
                    }
                }
                match max_idx {
                    0 => (
                        BehaviorType::Survival,
                        "Ensuring physical continuation".to_string(),
                    ),
                    1 => (
                        BehaviorType::Creative,
                        "Expressing through the body".to_string(),
                    ),
                    2 => (
                        BehaviorType::Survival,
                        "Responding to physical catalyst".to_string(),
                    ),
                    3 => (
                        BehaviorType::Resting,
                        "Experiencing through the body".to_string(),
                    ),
                    4 => (
                        BehaviorType::Socializing,
                        "Relating through embodied presence".to_string(),
                    ),
                    5 => (
                        BehaviorType::Intuitive,
                        "Transforming body-based understanding".to_string(),
                    ),
                    _ => (BehaviorType::Loving, "Embodying love in form".to_string()),
                }
            }
            _ => {
                // spirit
                let mut max_idx = 0;
                let mut max_val = 0.0;
                for (i, &v) in self.activations.spirit.iter().enumerate() {
                    if v > max_val {
                        max_val = v;
                        max_idx = i;
                    }
                }
                match max_idx {
                    0 => (
                        BehaviorType::Spiritual,
                        "Connecting to spiritual foundation".to_string(),
                    ),
                    1 => (
                        BehaviorType::Spiritual,
                        "Spiritual potential awakening".to_string(),
                    ),
                    2 => (
                        BehaviorType::Confronting,
                        "Facing spiritual challenge".to_string(),
                    ),
                    3 => (
                        BehaviorType::Spiritual,
                        "Spiritual experience unfolding".to_string(),
                    ),
                    4 => (
                        BehaviorType::Intuitive,
                        "Understanding spiritual significance".to_string(),
                    ),
                    5 => (
                        BehaviorType::Intuitive,
                        "Spiritual transformation underway".to_string(),
                    ),
                    _ => (
                        BehaviorType::Spiritual,
                        "Walking the great way of spirit".to_string(),
                    ),
                }
            }
        };

        // Determine growth direction from catalyst polarity
        let growth_direction = match catalyst.polarity {
            CatalystPolarity::ServiceToOthers => GrowthDirection::Positive,
            CatalystPolarity::ServiceToSelf => GrowthDirection::Negative,
            CatalystPolarity::Neutral => {
                if choice.integration_level > 0.5 {
                    GrowthDirection::Positive
                } else {
                    GrowthDirection::Neutral
                }
            }
        };

        // Confidence influenced by integration level and catalyst intensity
        let confidence = (choice.integration_level * 0.7 + catalyst.intensity * 0.3)
            * (0.8 + rng.gen::<Float>() * 0.2);

        BehaviorOutput {
            behavior_type,
            intensity: max_activation * catalyst.intensity,
            confidence: confidence.clamp(0.0, 1.0),
            growth_direction,
            impulse,
        }
    }

    /// Get current activations for inspection
    pub fn get_activations(&self) -> &ArchetypeActivationState {
        &self.activations
    }

    /// Get current behavior if any
    pub fn get_current_behavior(&self) -> Option<&BehaviorOutput> {
        self.current_behavior.as_ref()
    }

    /// Get archetype system health
    pub fn get_system_health(&self) -> bool {
        self.archetype_system.assess_system_health()
    }
}

/// Output from lesser cycle processing
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LesserCycleOutput {
    processed: Float,
    wisdom_generated: Float,
    transformation_needed: bool,
}

/// Output from greater cycle processing
#[derive(Debug, Clone)]
struct GreaterCycleOutput {
    meaning_extracted: Float,
    wisdom_cost: Float,
    transformation_triggered: bool,
}

/// Output from choice processing
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChoiceOutput {
    integration_level: Float,
    wisdom_required: Float,
    transformation_offered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_processor_creation() {
        let processor = EntityArchetypeProcessor::new(1);
        assert_eq!(processor.entity_id, 1);
        assert!(processor.get_system_health());
    }

    #[test]
    fn test_catalyst_processing() {
        let mut processor = EntityArchetypeProcessor::new(1);
        let catalyst =
            CatalystEvent::new(CatalystSource::Environment, 0.7, CatalystType::Challenge);
        let behavior = processor.process_catalyst(&catalyst);

        assert!(behavior.intensity > 0.0);
        assert!(behavior.confidence >= 0.0 && behavior.confidence <= 1.0);
    }

    #[test]
    fn test_different_catalyst_sources() {
        let mut processor = EntityArchetypeProcessor::new(1);

        // Body catalyst
        let body_cat = CatalystEvent::new(CatalystSource::Body, 0.5, CatalystType::Health);
        processor.process_catalyst(&body_cat);

        // Environment catalyst
        let env_cat =
            CatalystEvent::new(CatalystSource::Environment, 0.5, CatalystType::Opportunity);
        processor.process_catalyst(&env_cat);

        // Social catalyst
        let social_cat =
            CatalystEvent::new(CatalystSource::Social, 0.5, CatalystType::Relationship);
        processor.process_catalyst(&social_cat);

        // Each should produce different behavior
        assert!(processor.catalyst_history.len() >= 3);
    }

    #[test]
    fn test_activation_levels() {
        let mut processor = EntityArchetypeProcessor::new(1);

        // Process several catalysts
        for _ in 0..10 {
            let catalyst =
                CatalystEvent::new(CatalystSource::Environment, 0.5, CatalystType::Challenge);
            processor.process_catalyst(&catalyst);
        }

        let activations = processor.get_activations();
        // At least some activations should have changed
        let total: Float = activations.to_array().iter().sum();
        assert!(total > 1.0); // Default was ~2.2 (22 * 0.1)
    }
}
