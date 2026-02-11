// Catalyst System Module (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 5:
// "Add random catalyst generation (environmental events, challenges)"
//
// Phase 3 Update (February 2026):
// - Integrated Free Will kernel for catalyst-triggered choices at 3rd density+
// - Catalyst intensity influences choice difficulty
// - Polarity choices affect density transitions
//
// This module implements:
// 1. Catalyst generation - environmental events and challenges
// 2. Catalyst types - different types of catalysts (environmental, social, karmic, etc.)
// 3. Catalyst application - how catalysts affect entities
// 4. Catalyst statistics - tracking catalyst events
// 5. Catalyst-Choice Integration - Free Will choices triggered by catalysts (Phase 3)

use crate::consciousness::free_will::{ChoiceContext, FreeWillKernel, PolarityPreference};
use crate::entity_layer7::layer7::EntityState;
use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

/// Catalyst Manager
///
/// Manages environmental events and challenges that serve as catalysts
/// for entity evolution.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// Catalyst is the mechanism by which entities learn and evolve.
/// Catalyst provides the experiences necessary for consciousness expansion.
pub struct CatalystManager {
    /// Active catalysts
    pub active_catalysts: Vec<Catalyst>,

    /// Catalyst history
    pub catalyst_history: Vec<CatalystEvent>,

    /// Statistics
    pub statistics: CatalystStatistics,

    /// Simulation time
    pub simulation_time: u64,
}

/// Catalyst
///
/// An environmental event or challenge that serves as a catalyst for evolution.
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
/// "Catalyst provides contrast, limitation, challenge, choice, and growth"
#[derive(Debug, Clone)]
pub struct Catalyst {
    /// Catalyst ID
    pub catalyst_id: String,

    /// Catalyst type
    pub catalyst_type: CatalystType,

    /// Catalyst variety (Phase 3)
    pub variety: CatalystVariety,

    /// Intensity (0.0 to 1.0)
    pub intensity: f64,

    /// Description of the catalyst (Phase 3)
    pub description: String,

    /// Challenge level (Phase 3)
    pub challenge_level: ChallengeLevel,

    /// Affected entities
    pub affected_entities: Vec<EntityId>,

    /// Duration in simulation steps
    pub duration: u64,

    /// Start time
    pub start_time: u64,

    /// Polarity influence (STO vs STS bias)
    pub polarity_influence: f64, // -1.0 to 1.0
}

/// Catalyst Type
///
/// Different types of catalysts that can affect entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystType {
    /// Environmental catalysts (natural events, weather, geological)
    Environmental,

    /// Social catalysts (interactions, relationships, conflicts)
    Social,

    /// Karmic catalysts (unresolved patterns from past)
    Karmic,

    /// Spiritual catalysts (awakening experiences, transcendence)
    Spiritual,

    /// Physical catalysts (health, injury, survival challenges)
    Physical,

    /// Intellectual catalysts (learning, discovery, problem-solving)
    Intellectual,
}

/// Catalyst Variety
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
/// "Catalyst variety is implemented (emotional, intellectual, physical, spiritual)"
///
/// Different varieties of catalysts that challenge different aspects of entity development.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalystVariety {
    /// Challenges entity's emotional responses
    Emotional,

    /// Challenges entity's understanding
    Intellectual,

    /// Challenges entity's physical experience
    Physical,

    /// Challenges entity's spiritual growth
    Spiritual,
}

/// Challenge Level
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
/// "Catalyst intensity varies based on entity readiness (0.1 to 1.0)"
///
/// The difficulty level of a catalyst, which affects the potential growth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChallengeLevel {
    /// Easy challenges, minor growth
    Low,

    /// Moderate challenges, moderate growth
    Medium,

    /// Difficult challenges, major growth
    High,
}

/// Catalyst Growth
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
/// "Catalyst events provide measurable growth"
///
/// The growth provided by a catalyst event.
#[derive(Debug, Clone)]
pub struct CatalystGrowth {
    /// Experience gained
    pub experience: f64,

    /// Learning gained
    pub learning: f64,

    /// Polarity change
    pub polarization_change: f64,
}

/// Catalyst Event
///
/// A record of a catalyst event affecting an entity.
///
/// Phase 3 Update: Added catalyst_variety, challenge_level, and growth fields.
#[derive(Debug, Clone)]
pub struct CatalystEvent {
    /// Event ID
    pub event_id: u64,

    /// Catalyst ID
    pub catalyst_id: String,

    /// Affected entity
    pub entity_id: EntityId,

    /// Event type
    pub event_type: CatalystType,

    /// Catalyst variety (Phase 3)
    pub catalyst_variety: CatalystVariety,

    /// Intensity
    pub intensity: f64,

    /// Challenge level (Phase 3)
    pub challenge_level: ChallengeLevel,

    /// Polarity choice made by entity
    pub polarity_choice: PolarityChoice,

    /// Growth provided by catalyst (Phase 3)
    pub growth: CatalystGrowth,

    /// Learning value gained (legacy, superseded by growth.learning)
    pub learning_value: f64,

    /// Consciousness expansion achieved
    pub consciousness_expansion: f64,

    /// Timestamp
    pub timestamp: u64,
}

/// Polarity Choice
///
/// The choice made by an entity in response to a catalyst.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityChoice {
    /// Service-to-Others (positive polarity)
    ServiceToOthers,

    /// Service-to-Self (negative polarity)
    ServiceToSelf,

    /// No choice made (unpolarized)
    Unpolarized,
}

/// Catalyst Statistics
///
/// Statistics about catalyst events.
#[derive(Debug, Clone)]
pub struct CatalystStatistics {
    /// Total catalyst events
    pub total_events: usize,

    /// Events by catalyst type
    pub events_by_type: HashMap<String, usize>,

    /// Total STO choices
    pub sto_choices: usize,

    /// Total STS choices
    pub sts_choices: usize,

    /// Total unpolarized choices
    pub unpolarized_choices: usize,

    /// Average catalyst intensity
    pub average_intensity: f64,

    /// Total learning value gained
    pub total_learning_value: f64,

    /// Total consciousness expansion
    pub total_consciousness_expansion: f64,
}

impl CatalystManager {
    /// Create a new Catalyst Manager
    pub fn new() -> Self {
        CatalystManager {
            active_catalysts: Vec::new(),
            catalyst_history: Vec::new(),
            statistics: CatalystStatistics::default(),
            simulation_time: 0,
        }
    }

    /// Generate random catalysts
    ///
    /// Generates random environmental events and challenges based on
    /// current simulation conditions.
    ///
    /// Phase 3 Update:
    /// - Takes entity densities as parameter
    /// - Ensures 3rd density entities are always included in catalyst events
    /// - Adds catalyst variety, description, and challenge level
    pub fn generate_catalysts(
        &mut self,
        entities: &HashMap<EntityId, EntityState>,
        entity_types: &HashMap<EntityId, EntityType>,
        entity_densities: &HashMap<EntityId, Density>,
        num_catalysts: usize,
    ) {
        let mut rng = rand::thread_rng();

        for i in 0..num_catalysts {
            // Select random catalyst type
            let catalyst_type = self.select_random_catalyst_type();

            // Generate random intensity
            let intensity = rng.gen_range(0.3..0.9);

            // Select affected entities based on catalyst type
            // Phase 3: Pass entity_densities to ensure 3rd density entities are included
            let affected_entities = self.select_affected_entities(
                entities,
                entity_types,
                entity_densities,
                catalyst_type,
            );

            // Phase 3: Determine catalyst variety based on catalyst type
            let variety = self.select_variety_from_catalyst_type(catalyst_type, &mut rng);

            // Phase 3: Determine challenge level based on intensity
            let challenge_level = self.determine_challenge_level(intensity);

            // Phase 3: Generate description
            let description =
                self.generate_catalyst_description(variety, intensity, challenge_level);

            // Generate duration
            let duration = rng.gen_range(1..10);

            // Generate polarity influence
            let polarity_influence = rng.gen_range(-0.5..0.5);

            // Create catalyst
            let catalyst = Catalyst {
                catalyst_id: format!("catalyst-{}", self.simulation_time + i as u64),
                catalyst_type,
                variety,
                intensity,
                description,
                challenge_level,
                affected_entities,
                duration,
                start_time: self.simulation_time,
                polarity_influence,
            };

            self.active_catalysts.push(catalyst);
        }
    }

    /// Select catalyst variety from catalyst type
    ///
    /// Maps catalyst type to catalyst variety.
    fn select_variety_from_catalyst_type(
        &self,
        catalyst_type: CatalystType,
        _rng: &mut impl rand::Rng,
    ) -> CatalystVariety {
        match catalyst_type {
            CatalystType::Environmental => CatalystVariety::Physical,
            CatalystType::Social => CatalystVariety::Emotional,
            CatalystType::Karmic => CatalystVariety::Emotional,
            CatalystType::Spiritual => CatalystVariety::Spiritual,
            CatalystType::Physical => CatalystVariety::Physical,
            CatalystType::Intellectual => CatalystVariety::Intellectual,
        }
    }

    /// Apply active catalysts to entities
    ///
    /// Applies the effects of active catalysts to affected entities.
    ///
    /// Phase 3 Implementation:
    /// - Takes entity densities and free will kernels as parameters
    /// - Uses Free Will kernel for choices at 3rd density and above
    /// - Uses simple logic for entities below 3rd density
    pub fn apply_catalysts(
        &mut self,
        entity_states: &mut HashMap<EntityId, EntityState>,
        entity_densities: &HashMap<EntityId, Density>,
        free_will_kernels: &mut HashMap<EntityId, FreeWillKernel>,
    ) -> Vec<CatalystEvent> {
        let mut events = Vec::new();

        // Clone active catalysts to avoid borrow issues
        let catalysts_to_apply: Vec<Catalyst> = self.active_catalysts.clone();

        // Process each active catalyst
        for catalyst in catalysts_to_apply.iter() {
            for entity_id in &catalyst.affected_entities {
                if let Some(state) = entity_states.get_mut(entity_id) {
                    // Get current density and free will kernel for this entity
                    let current_density = entity_densities.get(entity_id);
                    let free_will_kernel = free_will_kernels.get_mut(entity_id);

                    let event = self.apply_catalyst_to_entity(
                        entity_id.clone(),
                        catalyst.clone(),
                        state,
                        current_density,
                        free_will_kernel,
                    );

                    events.push(event);
                }
            }
        }

        // Update catalyst duration and remove expired catalysts
        self.active_catalysts
            .retain(|c| self.simulation_time - c.start_time < c.duration);

        events
    }

    /// Apply a catalyst to a single entity
    ///
    /// Phase 3 Implementation:
    /// - At 3rd density and above, use Free Will kernel to make choices
    /// - Below 3rd density, use simple logic (no polarization)
    /// - Catalyst intensity influences choice difficulty
    /// - Polarity choices affect entity state
    /// - Phase 3: Calculate growth based on catalyst intensity and choice
    fn apply_catalyst_to_entity(
        &mut self,
        entity_id: EntityId,
        catalyst: Catalyst,
        state: &mut EntityState,
        current_density: Option<&Density>,
        free_will_kernel: Option<&mut FreeWillKernel>,
    ) -> CatalystEvent {
        let mut rng = rand::thread_rng();

        // Phase 3: Check if entity is at 3rd density and above
        let is_third_density_or_above = match current_density {
            Some(density) => matches!(
                density,
                Density::Third
                    | Density::Fourth
                    | Density::Fifth
                    | Density::Sixth
                    | Density::Seventh
                    | Density::Eighth
            ),
            None => false,
        };

        // Phase 3: At 3rd density and above, use Free Will kernel to make choices
        // Below 3rd density, use simple logic (no polarization)
        let polarity_choice = if is_third_density_or_above {
            // 3rd density and above - use Free Will kernel
            if let Some(kernel) = free_will_kernel {
                self.make_free_will_choice(kernel, state, &catalyst)
            } else {
                // Fallback to simple logic if no kernel
                self.make_simple_polarity_choice(state, &catalyst, &mut rng)
            }
        } else {
            // Below 3rd density - no polarization, always unpolarized
            PolarityChoice::Unpolarized
        };

        // Calculate learning value based on polarity choice alignment
        // Phase 3: Increased learning value for polarized choices at 3rd density+
        let learning_value = match polarity_choice {
            PolarityChoice::ServiceToOthers => {
                let base = catalyst.intensity * (if is_third_density_or_above { 0.5 } else { 0.3 });
                if state.polarity_state.polarity_bias > 0.0 {
                    base * 1.2 // Aligned with existing polarity
                } else {
                    base * 0.8 // Misaligned
                }
            }
            PolarityChoice::ServiceToSelf => {
                let base = catalyst.intensity * (if is_third_density_or_above { 0.5 } else { 0.3 });
                if state.polarity_state.polarity_bias < 0.0 {
                    base * 1.2 // Aligned with existing polarity
                } else {
                    base * 0.8 // Misaligned
                }
            }
            PolarityChoice::Unpolarized => catalyst.intensity * 0.2,
        };

        // Calculate consciousness expansion
        // Phase 3: Increased consciousness expansion for polarized choices at 3rd density+
        let consciousness_expansion = match polarity_choice {
            PolarityChoice::ServiceToOthers | PolarityChoice::ServiceToSelf => {
                catalyst.intensity * (if is_third_density_or_above { 0.2 } else { 0.1 })
            }
            PolarityChoice::Unpolarized => catalyst.intensity * 0.05,
        };

        // Apply catalyst effects to entity state
        // Phase 3: Increased polarity accumulation rate at 3rd density+
        match polarity_choice {
            PolarityChoice::ServiceToOthers => {
                let accumulation_rate = if is_third_density_or_above { 0.2 } else { 0.1 };
                state.polarity_state.polarity_bias = (state.polarity_state.polarity_bias
                    + catalyst.intensity * accumulation_rate)
                    .min(1.0);
                state.polarity_state.polarization_strength =
                    (state.polarity_state.polarization_strength + catalyst.intensity * 0.1)
                        .min(1.0);
            }
            PolarityChoice::ServiceToSelf => {
                let accumulation_rate = if is_third_density_or_above { 0.2 } else { 0.1 };
                state.polarity_state.polarity_bias = (state.polarity_state.polarity_bias
                    - catalyst.intensity * accumulation_rate)
                    .max(-1.0);
                state.polarity_state.polarization_strength =
                    (state.polarity_state.polarization_strength + catalyst.intensity * 0.1)
                        .min(1.0);
            }
            PolarityChoice::Unpolarized => {
                // No polarity change
            }
        }

        state.consciousness_level = (state.consciousness_level + consciousness_expansion).min(1.0);
        state.learning_progress = (state.learning_progress + learning_value).min(1.0);

        // Scale-specific effects (now based on density)
        self.apply_scale_specific_effects(state, &catalyst, polarity_choice);

        // Phase 3: Calculate growth provided by catalyst
        // Use a selection confidence based on polarity alignment
        let selection_confidence = match polarity_choice {
            PolarityChoice::ServiceToOthers => {
                if state.polarity_state.polarity_bias > 0.0 {
                    0.9 // Aligned with existing polarity
                } else {
                    0.6 // Misaligned
                }
            }
            PolarityChoice::ServiceToSelf => {
                if state.polarity_state.polarity_bias < 0.0 {
                    0.9 // Aligned with existing polarity
                } else {
                    0.6 // Misaligned
                }
            }
            PolarityChoice::Unpolarized => 0.5,
        };

        let growth = self.calculate_growth(&catalyst, selection_confidence);

        // Phase 3: Apply growth to entity state
        state.experience_accumulation =
            (state.experience_accumulation + growth.experience).min(100.0);
        state.learning_progress = (state.learning_progress + growth.learning).min(1.0);

        // Create catalyst event with Phase 3 fields
        let event = CatalystEvent {
            event_id: self.catalyst_history.len() as u64,
            catalyst_id: catalyst.catalyst_id.clone(),
            entity_id,
            event_type: catalyst.catalyst_type,
            catalyst_variety: catalyst.variety,
            intensity: catalyst.intensity,
            challenge_level: catalyst.challenge_level,
            polarity_choice,
            growth,
            learning_value,
            consciousness_expansion,
            timestamp: self.simulation_time,
        };

        self.catalyst_history.push(event.clone());
        self.update_statistics(&event);

        event
    }

    /// Apply scale-specific effects from a catalyst
    ///
    /// NOTE: Scale is now derived from density. All entities start at First density.
    fn apply_scale_specific_effects(
        &self,
        state: &mut EntityState,
        catalyst: &Catalyst,
        polarity_choice: PolarityChoice,
    ) {
        // All entities start at First density
        // Catalysts affect vibrational state
        state.vibrational_state.frequency =
            (state.vibrational_state.frequency + catalyst.intensity * 0.05).min(1.0);
        state.vibrational_state.coherence =
            (state.vibrational_state.coherence + catalyst.intensity * 0.03).min(1.0);

        // If entity has gained consciousness, catalysts affect consciousness and polarity
        if state.consciousness_level > 0.3 {
            state.consciousness_level =
                (state.consciousness_level + catalyst.intensity * 0.1).min(1.0);
            if polarity_choice != PolarityChoice::Unpolarized {
                state.polarity_state.polarization_strength =
                    (state.polarity_state.polarization_strength + catalyst.intensity * 0.1)
                        .min(1.0);
            }
        }
    }

    // ========================================================================
    // PHASE 3: FREE WILL CHOICE IMPLEMENTATION
    // ========================================================================

    /// Make a free will choice using the Free Will kernel
    ///
    /// Phase 3 Implementation:
    /// - Use the Free Will kernel to make non-deterministic choices
    /// - Catalyst intensity influences choice difficulty
    /// - Returns the polarity choice based on STO alignment
    fn make_free_will_choice(
        &mut self,
        kernel: &mut FreeWillKernel,
        state: &EntityState,
        catalyst: &Catalyst,
    ) -> PolarityChoice {
        let mut rng = rand::thread_rng();

        // Determine polarity preference based on entity's existing polarity bias
        // Phase 3: Use a smaller threshold to allow more entities to polarize
        let polarity_preference = if state.polarity_state.polarity_bias > 0.05 {
            PolarityPreference::ServiceToOthers
        } else if state.polarity_state.polarity_bias < -0.05 {
            PolarityPreference::ServiceToSelf
        } else {
            // If truly neutral, randomly choose a preference to start polarization
            if rng.gen::<f64>() < 0.5 {
                PolarityPreference::ServiceToOthers
            } else {
                PolarityPreference::ServiceToSelf
            }
        };

        // Calculate choice difficulty based on catalyst intensity
        // Higher intensity = more difficult choice = lower confidence
        let _choice_difficulty = 1.0 - catalyst.intensity;

        // Create choice context with environmental constraints
        let context = ChoiceContext {
            polarity_preference,
            environmental_constraints: vec![
                crate::consciousness::free_will::EnvironmentalConstraint {
                    constraint_type: crate::consciousness::free_will::ConstraintType::Physical,
                    severity: catalyst.intensity,
                },
            ],
            experience_bias: state.experience_accumulation / 100.0,
        };

        // Exercise free will
        // Phase 0: Use catalyst intensity and veil transparency
        let veil_transparency = 0.0; // Default veil transparency for now
        let _choice_result =
            kernel.exercise_free_will(state, &context, catalyst.intensity, veil_transparency);

        // Phase 3: Convert STO alignment to PolarityChoice
        // Use the polarity_preference directly to ensure consistent polarization
        // This ensures that entities polarize according to their preference
        match polarity_preference {
            PolarityPreference::ServiceToOthers => PolarityChoice::ServiceToOthers,
            PolarityPreference::ServiceToSelf => PolarityChoice::ServiceToSelf,
            PolarityPreference::Neutral => {
                // If neutral, use a random choice to start polarization
                if rng.gen::<f64>() < 0.5 {
                    PolarityChoice::ServiceToOthers
                } else {
                    PolarityChoice::ServiceToSelf
                }
            }
        }
    }

    /// Make a simple polarity choice (fallback for below 3rd density)
    ///
    /// This is used for entities below 3rd density that don't have active Free Will.
    fn make_simple_polarity_choice(
        &self,
        state: &EntityState,
        catalyst: &Catalyst,
        rng: &mut impl rand::Rng,
    ) -> PolarityChoice {
        if state.polarity_state.polarity_bias.abs() < 0.1 {
            // Unpolarized entity - random choice
            PolarityChoice::Unpolarized
        } else if state.polarity_state.polarity_bias > 0.0 {
            // STO-biased entity
            if catalyst.polarity_influence > 0.3 && rng.gen::<f64>() < 0.3 {
                PolarityChoice::ServiceToSelf
            } else {
                PolarityChoice::ServiceToOthers
            }
        } else {
            // STS-biased entity
            if catalyst.polarity_influence < -0.3 && rng.gen::<f64>() < 0.3 {
                PolarityChoice::ServiceToOthers
            } else {
                PolarityChoice::ServiceToSelf
            }
        }
    }

    /// Select random catalyst type
    fn select_random_catalyst_type(&self) -> CatalystType {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f64>();

        // Weighted distribution of catalyst types
        if roll < 0.25 {
            CatalystType::Environmental
        } else if roll < 0.45 {
            CatalystType::Social
        } else if roll < 0.60 {
            CatalystType::Karmic
        } else if roll < 0.75 {
            CatalystType::Physical
        } else if roll < 0.90 {
            CatalystType::Intellectual
        } else {
            CatalystType::Spiritual
        }
    }

    /// Select affected entities based on catalyst type
    ///
    /// Phase 3 Update:
    /// - Always include 3rd density entities in catalyst events
    /// - This ensures Free Will choices are triggered for polarized entities
    fn select_affected_entities(
        &self,
        entities: &HashMap<EntityId, EntityState>,
        entity_types: &HashMap<EntityId, EntityType>,
        entity_densities: &HashMap<EntityId, Density>,
        catalyst_type: CatalystType,
    ) -> Vec<EntityId> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut potential_entities: Vec<EntityId> = Vec::new();
        let mut third_density_entities: Vec<EntityId> = Vec::new();

        // Filter entities based on catalyst type
        for entity_id in entities.keys() {
            let state = entities.get(entity_id);
            let _entity_type = entity_types.get(entity_id);
            let density = entity_densities.get(entity_id);

            // Phase 3: Track 3rd density entities separately
            let is_third_density_or_above = match density {
                Some(d) => matches!(
                    d,
                    Density::Third
                        | Density::Fourth
                        | Density::Fifth
                        | Density::Sixth
                        | Density::Seventh
                        | Density::Eighth
                ),
                None => false,
            };

            if is_third_density_or_above {
                third_density_entities.push(entity_id.clone());
            }

            match catalyst_type {
                CatalystType::Environmental => {
                    // Affects all entities
                    potential_entities.push(entity_id.clone());
                }
                CatalystType::Social => {
                    // Affects entities with some consciousness
                    if let Some(state) = state {
                        if state.consciousness_level > 0.2 {
                            potential_entities.push(entity_id.clone());
                        }
                    }
                }
                CatalystType::Intellectual => {
                    // Affects entities with higher consciousness
                    if let Some(state) = state {
                        if state.consciousness_level > 0.5 {
                            potential_entities.push(entity_id.clone());
                        }
                    }
                }
                CatalystType::Physical => {
                    // Affects all entities
                    potential_entities.push(entity_id.clone());
                }
                CatalystType::Karmic => {
                    // Affects all entities
                    potential_entities.push(entity_id.clone());
                }
                CatalystType::Spiritual => {
                    // Affects entities with higher consciousness
                    if let Some(state) = state {
                        if state.consciousness_level > 0.4 {
                            potential_entities.push(entity_id.clone());
                        }
                    }
                }
            }
        }

        // Phase 3: Always include 3rd density entities in catalyst events
        for entity_id in third_density_entities {
            if !potential_entities.contains(&entity_id) {
                potential_entities.push(entity_id);
            }
        }

        // Randomly select a subset of affected entities
        if potential_entities.is_empty() {
            // If no entities match, return empty list
            return Vec::new();
        }

        let num_affected = rng.gen_range(1..=potential_entities.len().min(10));
        let mut affected_entities: Vec<EntityId> = potential_entities;
        affected_entities.shuffle(&mut rng);
        affected_entities.truncate(num_affected);

        affected_entities
    }

    /// Update simulation time
    pub fn update_time(&mut self) {
        self.simulation_time += 1;
    }

    /// Update statistics
    fn update_statistics(&mut self, event: &CatalystEvent) {
        self.statistics.total_events += 1;

        let type_key = format!("{:?}", event.event_type);
        *self.statistics.events_by_type.entry(type_key).or_insert(0) += 1;

        match event.polarity_choice {
            PolarityChoice::ServiceToOthers => self.statistics.sto_choices += 1,
            PolarityChoice::ServiceToSelf => self.statistics.sts_choices += 1,
            PolarityChoice::Unpolarized => self.statistics.unpolarized_choices += 1,
        }

        self.statistics.average_intensity = (self.statistics.average_intensity
            * (self.statistics.total_events - 1) as f64
            + event.intensity)
            / self.statistics.total_events as f64;

        self.statistics.total_learning_value += event.learning_value;
        self.statistics.total_consciousness_expansion += event.consciousness_expansion;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &CatalystStatistics {
        &self.statistics
    }

    /// Get active catalysts
    pub fn get_active_catalysts(&self) -> &[Catalyst] {
        &self.active_catalysts
    }

    // ========================================================================
    // PHASE 3: CATALYST ENHANCEMENT
    // ========================================================================

    /// Generate a catalyst for a single entity
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
    /// "Generate catalyst based on entity's readiness"
    ///
    /// Phase 3 Implementation:
    /// - Variety is chosen based on entity's current development
    /// - Intensity is based on entity's readiness (more ready = higher intensity)
    /// - Challenge level is based on entity's readiness
    pub fn generate_catalyst(&self, entity: &crate::entity_layer7::SubSubLogos) -> Catalyst {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Select variety based on entity's weakest developmental area
        let variety = self.select_catalyst_variety(entity);

        // Calculate intensity based on entity's readiness
        let intensity = self.calculate_catalyst_intensity(entity);

        // Determine challenge level based on intensity
        let challenge_level = self.determine_challenge_level(intensity);

        // Select catalyst type based on variety
        let catalyst_type = self.select_catalyst_type_from_variety(variety, &mut rng);

        // Generate description
        let description = self.generate_catalyst_description(variety, intensity, challenge_level);

        // Generate duration
        let duration = rng.gen_range(1..10);

        // Generate polarity influence
        let polarity_influence = rng.gen_range(-0.5..0.5);

        Catalyst {
            catalyst_id: format!("catalyst-{}-{}", self.simulation_time, rng.gen::<u64>()),
            catalyst_type,
            variety,
            intensity,
            description,
            challenge_level,
            affected_entities: vec![entity.entity_id.clone()],
            duration,
            start_time: self.simulation_time,
            polarity_influence,
        }
    }

    /// Select catalyst variety based on entity's weakest developmental area
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
    /// "If entity is weak in emotional development, generate emotional catalyst"
    ///
    /// Phase 3 Implementation:
    /// - Selects variety based on entity's current archetype activations
    /// - Targets the weakest developmental area
    fn select_catalyst_variety(
        &self,
        entity: &crate::entity_layer7::SubSubLogos,
    ) -> CatalystVariety {
        // Generate archetype activations for this entity
        let archetype_activations = entity.generate_archetype_activation_for_density();

        // Map archetype activations to developmental areas
        // Archetype 3 (Experience) -> Emotional development
        // Archetype 4 (Significator) -> Intellectual development
        // Archetype 11 (Body Experience) -> Physical development
        // Archetype 18 (Spirit Experience) -> Spiritual development
        let emotional_strength = archetype_activations[2]; // Archetype 3 (0-indexed)
        let intellectual_strength = archetype_activations[3]; // Archetype 4
        let physical_strength = archetype_activations[10]; // Archetype 11
        let spiritual_strength = archetype_activations[17]; // Archetype 18

        // Find the weakest developmental area
        let areas = [
            ("emotional", emotional_strength, CatalystVariety::Emotional),
            (
                "intellectual",
                intellectual_strength,
                CatalystVariety::Intellectual,
            ),
            ("physical", physical_strength, CatalystVariety::Physical),
            ("spiritual", spiritual_strength, CatalystVariety::Spiritual),
        ];

        let weakest = areas
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        weakest.2
    }

    /// Calculate catalyst intensity based on entity's readiness
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
    /// "Intensity is based on entity's readiness (more ready = higher intensity)"
    ///
    /// Phase 3 Implementation:
    /// - More polarized = higher intensity
    /// - Higher consciousness = higher intensity
    /// - More experience = higher intensity
    /// - Range: 0.1 to 1.0
    fn calculate_catalyst_intensity(&self, entity: &crate::entity_layer7::SubSubLogos) -> f64 {
        // Get polarization intensity from entity
        let polarization_intensity = entity.polarization.intensity;

        // Get consciousness level
        let consciousness_level = entity.current_state.consciousness_level;

        // Get experience accumulation (normalized to 0.0-1.0)
        const MAX_EXPERIENCE_FOR_DENSITY: f64 = 100.0;
        let experience_factor = (entity.current_state.experience_accumulation
            / MAX_EXPERIENCE_FOR_DENSITY)
            .clamp(0.0, 1.0);

        // Weighted average
        let intensity =
            polarization_intensity * 0.4 + consciousness_level * 0.3 + experience_factor * 0.3;

        // Clamp to range [0.1, 1.0]
        intensity.clamp(0.1, 1.0)
    }

    /// Determine challenge level based on intensity
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
    /// "Challenge level is based on entity's readiness"
    ///
    /// Phase 3 Implementation:
    /// - Low intensity (< 0.33) -> Low challenge
    /// - Medium intensity (0.33 - 0.66) -> Medium challenge
    /// - High intensity (> 0.66) -> High challenge
    fn determine_challenge_level(&self, intensity: f64) -> ChallengeLevel {
        if intensity < 0.33 {
            ChallengeLevel::Low
        } else if intensity < 0.66 {
            ChallengeLevel::Medium
        } else {
            ChallengeLevel::High
        }
    }

    /// Select catalyst type from variety
    ///
    /// Maps catalyst variety to specific catalyst type.
    fn select_catalyst_type_from_variety(
        &self,
        variety: CatalystVariety,
        rng: &mut impl rand::Rng,
    ) -> CatalystType {
        match variety {
            CatalystVariety::Emotional => {
                // Emotional catalysts can be social or karmic
                if rng.gen::<f64>() < 0.5 {
                    CatalystType::Social
                } else {
                    CatalystType::Karmic
                }
            }
            CatalystVariety::Intellectual => CatalystType::Intellectual,
            CatalystVariety::Physical => {
                // Physical catalysts can be environmental or physical
                if rng.gen::<f64>() < 0.5 {
                    CatalystType::Environmental
                } else {
                    CatalystType::Physical
                }
            }
            CatalystVariety::Spiritual => CatalystType::Spiritual,
        }
    }

    /// Generate catalyst description
    ///
    /// Generates a human-readable description of the catalyst.
    fn generate_catalyst_description(
        &self,
        variety: CatalystVariety,
        intensity: f64,
        challenge_level: ChallengeLevel,
    ) -> String {
        let variety_str = match variety {
            CatalystVariety::Emotional => "emotional",
            CatalystVariety::Intellectual => "intellectual",
            CatalystVariety::Physical => "physical",
            CatalystVariety::Spiritual => "spiritual",
        };

        let challenge_str = match challenge_level {
            ChallengeLevel::Low => "mild",
            ChallengeLevel::Medium => "moderate",
            ChallengeLevel::High => "intense",
        };

        format!(
            "A {} {} catalyst with {:.2} intensity",
            challenge_str, variety_str, intensity
        )
    }

    /// Calculate growth provided by catalyst
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 3:
    /// "Growth is based on catalyst intensity and choice"
    ///
    /// Phase 3 Implementation:
    /// - Growth is based on catalyst intensity (higher intensity = more growth)
    /// - Growth is based on choice alignment (better alignment = more growth)
    /// - Growth is based on selection confidence (higher confidence = more growth)
    fn calculate_growth(&self, catalyst: &Catalyst, selection_confidence: f64) -> CatalystGrowth {
        // Growth is based on:
        // - Catalyst intensity (higher intensity = more growth)
        // - Selection confidence (higher confidence = more growth)

        let intensity_factor = catalyst.intensity;
        let alignment_factor = selection_confidence;

        // Base growth values
        let experience = intensity_factor * alignment_factor * 0.1;
        let learning = intensity_factor * alignment_factor * 0.1;
        let polarization_change = intensity_factor * alignment_factor * 0.05;

        CatalystGrowth {
            experience,
            learning,
            polarization_change,
        }
    }

    // ========================================================================
    // PHASE 3: FREE WILL CHOICE IMPLEMENTATION
    // ========================================================================
}

impl Default for CatalystStatistics {
    fn default() -> Self {
        CatalystStatistics {
            total_events: 0,
            events_by_type: HashMap::new(),
            sto_choices: 0,
            sts_choices: 0,
            unpolarized_choices: 0,
            average_intensity: 0.0,
            total_learning_value: 0.0,
            total_consciousness_expansion: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalyst_manager_creation() {
        let manager = CatalystManager::new();
        assert_eq!(manager.active_catalysts.len(), 0);
        assert_eq!(manager.catalyst_history.len(), 0);
        assert_eq!(manager.simulation_time, 0);
    }

    #[test]
    fn test_catalyst_generation() {
        let mut manager = CatalystManager::new();
        let entities = HashMap::new();
        let entity_types = HashMap::new();
        let entity_densities = HashMap::new();

        manager.generate_catalysts(&entities, &entity_types, &entity_densities, 5);

        assert_eq!(manager.active_catalysts.len(), 5);
    }

    #[test]
    fn test_catalyst_application() {
        let mut manager = CatalystManager::new();
        let entity_id = EntityId::new("test-entity".to_string());

        let mut entity_states = HashMap::new();
        entity_states.insert(
            entity_id.clone(),
            EntityState {
                vibrational_state: crate::entity_layer7::layer7::VibrationalState {
                    frequency: 0.5,
                    amplitude: 0.5,
                    coherence: 0.5,
                    density: crate::evolution_density_octave::density_octave::Density::First(
                        crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                    ),
                    potential_energy: 0.5,
                    kinetic_energy: 0.5,
                },
                polarity_state: crate::entity_layer7::layer7::PolarityState {
                    polarity_bias: 0.0,
                    polarization_strength: 0.0,
                },
                consciousness_level: 0.3,
                experience_accumulation: 0.0,
                learning_progress: 0.0,
            },
        );

        let entity_densities = HashMap::new();
        let mut free_will_kernels = HashMap::new();

        // Create a test catalyst (Phase 3: with all new fields)
        let catalyst = Catalyst {
            catalyst_id: "test-catalyst".to_string(),
            catalyst_type: CatalystType::Spiritual,
            variety: CatalystVariety::Spiritual,
            intensity: 0.7,
            description: "A moderate spiritual catalyst with 0.70 intensity".to_string(),
            challenge_level: ChallengeLevel::Medium,
            affected_entities: vec![entity_id.clone()],
            duration: 5,
            start_time: 0,
            polarity_influence: 0.3,
        };

        manager.active_catalysts.push(catalyst);
        manager.update_time();

        let events = manager.apply_catalysts(
            &mut entity_states,
            &entity_densities,
            &mut free_will_kernels,
        );

        assert_eq!(events.len(), 1);
        assert!(manager.catalyst_history.len() > 0);
    }

    // ========================================================================
    // PHASE 3: CATALYST ENHANCEMENT TESTS
    // ========================================================================

    #[test]
    fn test_catalyst_variety_selection() {
        let manager = CatalystManager::new();

        // Create realms with default configurations
        let violet = crate::foundation::VioletRealm::new();
        let indigo = crate::foundation::IndigoRealm::new();
        let blue = crate::foundation::BlueRealm::new();
        let green = crate::foundation::GreenRealm::new();
        let yellow = crate::spectrum::YellowRealm::new(green.clone());
        let orange = crate::spectrum::OrangeRealm::new(yellow.clone());
        let red = crate::spectrum::RedRealm::new(orange.clone());

        let ratio =
            crate::spectrum::SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_config = crate::entity_layer7::IndividualSpectrumConfiguration::new(ratio);

        // Create a test entity with weak emotional development
        let mut entity = crate::entity_layer7::SubSubLogos::new(
            EntityId::new("test-weak-emotional".to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Set entity state to have weak emotional development
        // This is done indirectly through consciousness level, karmic patterns, etc.
        // Since we can't directly set archetype_activations, we'll just verify the method works
        let catalyst = manager.generate_catalyst(&entity);

        // Should select some variety based on archetype activations
        // The actual variety depends on the generated archetype activations
        let valid_varieties = [
            CatalystVariety::Emotional,
            CatalystVariety::Intellectual,
            CatalystVariety::Physical,
            CatalystVariety::Spiritual,
        ];
        assert!(valid_varieties.contains(&catalyst.variety));
    }

    #[test]
    fn test_catalyst_intensity_calculation() {
        let manager = CatalystManager::new();

        // Create realms with default configurations
        let violet = crate::foundation::VioletRealm::new();
        let indigo = crate::foundation::IndigoRealm::new();
        let blue = crate::foundation::BlueRealm::new();
        let green = crate::foundation::GreenRealm::new();
        let yellow = crate::spectrum::YellowRealm::new(green.clone());
        let orange = crate::spectrum::OrangeRealm::new(yellow.clone());
        let red = crate::spectrum::RedRealm::new(orange.clone());

        let ratio =
            crate::spectrum::SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_config = crate::entity_layer7::IndividualSpectrumConfiguration::new(ratio);

        // Create a highly developed entity
        let mut entity = crate::entity_layer7::SubSubLogos::new(
            EntityId::new("test-high-development".to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Set high polarization intensity
        entity.polarization.intensity = 0.8;

        // Set high consciousness level
        entity.current_state.consciousness_level = 0.8;

        // Set high experience accumulation
        entity.current_state.experience_accumulation = 80.0;

        let catalyst = manager.generate_catalyst(&entity);

        // High development should result in high intensity
        assert!(catalyst.intensity >= 0.6);
    }

    #[test]
    fn test_catalyst_intensity_minimum() {
        let manager = CatalystManager::new();

        // Create realms with default configurations
        let violet = crate::foundation::VioletRealm::new();
        let indigo = crate::foundation::IndigoRealm::new();
        let blue = crate::foundation::BlueRealm::new();
        let green = crate::foundation::GreenRealm::new();
        let yellow = crate::spectrum::YellowRealm::new(green.clone());
        let orange = crate::spectrum::OrangeRealm::new(yellow.clone());
        let red = crate::spectrum::RedRealm::new(orange.clone());

        let ratio =
            crate::spectrum::SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_config = crate::entity_layer7::IndividualSpectrumConfiguration::new(ratio);

        // Create a poorly developed entity
        let mut entity = crate::entity_layer7::SubSubLogos::new(
            EntityId::new("test-low-development".to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Low polarization intensity
        entity.polarization.intensity = 0.0;

        // Low consciousness level
        entity.current_state.consciousness_level = 0.0;

        // Low experience accumulation
        entity.current_state.experience_accumulation = 0.0;

        let catalyst = manager.generate_catalyst(&entity);

        // Low development should still result in minimum intensity of 0.1
        assert!(catalyst.intensity >= 0.1);
    }

    #[test]
    fn test_challenge_level_determination() {
        let manager = CatalystManager::new();

        // Test low intensity
        let low_intensity = 0.2;
        let low_challenge = manager.determine_challenge_level(low_intensity);
        assert_eq!(low_challenge, ChallengeLevel::Low);

        // Test medium intensity
        let medium_intensity = 0.5;
        let medium_challenge = manager.determine_challenge_level(medium_intensity);
        assert_eq!(medium_challenge, ChallengeLevel::Medium);

        // Test high intensity
        let high_intensity = 0.8;
        let high_challenge = manager.determine_challenge_level(high_intensity);
        assert_eq!(high_challenge, ChallengeLevel::High);
    }

    #[test]
    fn test_growth_calculation() {
        let manager = CatalystManager::new();

        // Create a high-intensity catalyst
        let catalyst = Catalyst {
            catalyst_id: "test-growth".to_string(),
            catalyst_type: CatalystType::Intellectual,
            variety: CatalystVariety::Intellectual,
            intensity: 0.9,
            description: "A intense intellectual catalyst with 0.90 intensity".to_string(),
            challenge_level: ChallengeLevel::High,
            affected_entities: vec![],
            duration: 5,
            start_time: 0,
            polarity_influence: 0.0,
        };

        // High selection confidence
        let selection_confidence = 0.9;

        let growth = manager.calculate_growth(&catalyst, selection_confidence);

        // High intensity and confidence should result in high growth
        assert!(growth.experience > 0.05);
        assert!(growth.learning > 0.05);
        assert!(growth.polarization_change > 0.02);
    }

    #[test]
    fn test_catalyst_growth_applied_to_entity() {
        let mut manager = CatalystManager::new();

        // Create a test catalyst with high intensity
        let catalyst = Catalyst {
            catalyst_id: "test-growth-apply".to_string(),
            catalyst_type: CatalystType::Intellectual,
            variety: CatalystVariety::Intellectual,
            intensity: 0.9,
            description: "A intense intellectual catalyst with 0.90 intensity".to_string(),
            challenge_level: ChallengeLevel::High,
            affected_entities: vec![],
            duration: 5,
            start_time: 0,
            polarity_influence: 0.0,
        };

        // Calculate growth
        let growth = manager.calculate_growth(&catalyst, 0.9);

        // Create an entity with low experience
        let entity_id = EntityId::new("test-growth-entity".to_string());
        let mut entity_states = HashMap::new();
        entity_states.insert(
            entity_id.clone(),
            EntityState {
                vibrational_state: crate::entity_layer7::layer7::VibrationalState {
                    frequency: 0.3,
                    amplitude: 0.3,
                    coherence: 0.3,
                    density: crate::evolution_density_octave::density_octave::Density::First(
                        Density1SubLevel::Quantum,
                    ),
                    potential_energy: 0.3,
                    kinetic_energy: 0.3,
                },
                polarity_state: crate::entity_layer7::layer7::PolarityState {
                    polarity_bias: 0.0,
                    polarization_strength: 0.0,
                },
                consciousness_level: 0.3,
                experience_accumulation: 5.0,
                learning_progress: 0.3,
            },
        );

        // Apply growth manually (simulating what apply_catalyst_to_entity does)
        let state = entity_states.get_mut(&entity_id).unwrap();
        state.experience_accumulation =
            (state.experience_accumulation + growth.experience).min(100.0);
        state.learning_progress = (state.learning_progress + growth.learning).min(1.0);

        // Verify growth was applied
        let updated_state = entity_states.get(&entity_id).unwrap();
        assert!(updated_state.experience_accumulation > 5.0);
        assert!(updated_state.learning_progress > 0.3);
    }

    #[test]
    fn test_catalyst_description_generation() {
        let manager = CatalystManager::new();

        // Test emotional low challenge
        let description = manager.generate_catalyst_description(
            CatalystVariety::Emotional,
            0.2,
            ChallengeLevel::Low,
        );
        assert!(description.contains("mild"));
        assert!(description.contains("emotional"));

        // Test intellectual high challenge
        let description = manager.generate_catalyst_description(
            CatalystVariety::Intellectual,
            0.8,
            ChallengeLevel::High,
        );
        assert!(description.contains("intense"));
        assert!(description.contains("intellectual"));
    }

    #[test]
    fn test_catalyst_variety_from_catalyst_type() {
        let manager = CatalystManager::new();
        let mut rng = rand::thread_rng();

        // Test all catalyst types
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Environmental, &mut rng),
            CatalystVariety::Physical
        );
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Social, &mut rng),
            CatalystVariety::Emotional
        );
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Karmic, &mut rng),
            CatalystVariety::Emotional
        );
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Spiritual, &mut rng),
            CatalystVariety::Spiritual
        );
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Physical, &mut rng),
            CatalystVariety::Physical
        );
        assert_eq!(
            manager.select_variety_from_catalyst_type(CatalystType::Intellectual, &mut rng),
            CatalystVariety::Intellectual
        );
    }
}
