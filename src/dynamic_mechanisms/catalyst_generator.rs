//! Catalyst Generator (Dynamic Mechanism 1)
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
//! "Generate catalyst events with proper probability distribution"
//!
//! This module implements state-sensitive catalyst generation that:
//! - Generates catalysts based on entity developmental needs
//! - Correlates catalyst type with environment and entity state
//! - Varies catalyst intensity based on entity readiness
//! - Ensures catalysts drive meaningful evolution

use crate::entity_layer7::layer7::{EntityId, EntityState, EntityType};
use crate::evolution_density_octave::density_octave::Density;
use crate::simulation_v3::catalyst_system::{
    Catalyst, CatalystManager, CatalystType, CatalystVariety, ChallengeLevel,
};
use crate::types::Float;
use rand::Rng;
use std::collections::HashMap;

/// Catalyst Generator
///
/// Generates catalyst events with proper probability distribution based on
/// entity state, environment, and developmental needs.
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Success Criteria:
/// "Catalyst generation correlates with environment and entity state"
pub struct CatalystGenerator {
    /// Catalyst manager (handles catalyst storage and application)
    catalyst_manager: CatalystManager,

    /// Catalyst generation history (for learning)
    generation_history: Vec<CatalystGenerationRecord>,

    /// Simulation time
    simulation_time: u64,
}

/// Catalyst Generation Context
///
/// Context information for catalyst generation.
#[derive(Debug, Clone)]
pub struct CatalystGenerationContext {
    /// Current simulation time
    pub simulation_time: u64,

    /// Map of entity states
    pub entity_states: HashMap<EntityId, EntityState>,

    /// Map of entity types
    pub entity_types: HashMap<EntityId, EntityType>,

    /// Map of entity densities
    pub entity_densities: HashMap<EntityId, Density>,

    /// Global environmental factors
    pub environmental_factors: EnvironmentalFactors,
}

/// Catalyst Generation Record
///
/// Records catalyst generation events for learning and analysis.
#[derive(Debug, Clone)]
pub struct CatalystGenerationRecord {
    /// Timestamp
    pub timestamp: u64,

    /// Entity ID (if targeted)
    pub target_entity: Option<EntityId>,

    /// Catalyst type
    pub catalyst_type: CatalystType,

    /// Catalyst variety
    pub catalyst_variety: CatalystVariety,

    /// Catalyst intensity
    pub intensity: Float,

    /// Challenge level
    pub challenge_level: ChallengeLevel,

    /// Success metric (did catalyst drive evolution?)
    pub success_metric: Float,
}

/// Catalyst Generator State
///
/// Current state of the catalyst generator.
#[derive(Debug, Clone)]
pub struct CatalystGeneratorState {
    /// Total catalysts generated
    pub total_generated: usize,

    /// Catalysts by type
    pub catalysts_by_type: HashMap<String, usize>,

    /// Average catalyst intensity
    pub average_intensity: Float,

    /// Success rate (catalysts that drove evolution)
    pub success_rate: Float,
}

impl Default for CatalystGeneratorState {
    fn default() -> Self {
        Self {
            total_generated: 0,
            catalysts_by_type: HashMap::new(),
            average_intensity: 0.0,
            success_rate: 0.0,
        }
    }
}

/// Environmental Factors
///
/// Global environmental factors that influence catalyst generation.
#[derive(Debug, Clone)]
pub struct EnvironmentalFactors {
    /// Global stress level (0.0 to 1.0)
    pub global_stress: Float,

    /// Available energy (0.0 to 1.0)
    pub available_energy: Float,

    /// Entropy level (0.0 to 1.0)
    pub entropy_level: Float,

    /// Coherence level (0.0 to 1.0)
    pub coherence_level: Float,
}

impl Default for EnvironmentalFactors {
    fn default() -> Self {
        Self {
            global_stress: 0.3,
            available_energy: 0.7,
            entropy_level: 0.2,
            coherence_level: 0.8,
        }
    }
}

impl CatalystGenerator {
    /// Create a new catalyst generator
    pub fn new() -> Self {
        Self {
            catalyst_manager: CatalystManager::new(),
            generation_history: Vec::new(),
            simulation_time: 0,
        }
    }

    /// Generate catalysts for the current simulation step
    ///
    /// This is the main entry point for catalyst generation.
    /// It generates catalysts based on entity states and environmental factors.
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Catalyst generation correlates with environment and entity state"
    pub fn generate_catalysts(&mut self, context: &CatalystGenerationContext) -> Vec<Catalyst> {
        let mut rng = rand::thread_rng();
        let mut generated_catalysts = Vec::new();

        // Calculate number of catalysts to generate based on context
        let num_catalysts = self.calculate_num_catalysts(context);

        // Generate catalysts for entities
        for _i in 0..num_catalysts {
            // Select target entity based on developmental needs
            let target_entity = self.select_target_entity(context, &mut rng);

            if let Some(entity_id) = target_entity {
                // Generate catalyst for this entity
                if let Some(entity_state) = context.entity_states.get(&entity_id) {
                    let entity_density = context.entity_densities.get(&entity_id);
                    let entity_type = context.entity_types.get(&entity_id);

                    let catalyst = self.generate_catalyst_for_entity(
                        &entity_id,
                        entity_state,
                        entity_density,
                        entity_type,
                        context,
                        &mut rng,
                    );

                    if let Some(catalyst) = catalyst {
                        // Record generation
                        self.record_generation(&catalyst, Some(entity_id.clone()));

                        generated_catalysts.push(catalyst);
                    }
                }
            }
        }

        // Add generated catalysts to manager
        for catalyst in &generated_catalysts {
            self.catalyst_manager
                .active_catalysts
                .push(catalyst.clone());
        }

        self.simulation_time += 1;

        generated_catalysts
    }

    /// Generate a catalyst for a specific entity
    ///
    /// Generates a catalyst tailored to the entity's developmental needs.
    fn generate_catalyst_for_entity(
        &self,
        entity_id: &EntityId,
        entity_state: &EntityState,
        entity_density: Option<&Density>,
        _entity_type: Option<&EntityType>,
        context: &CatalystGenerationContext,
        rng: &mut impl Rng,
    ) -> Option<Catalyst> {
        // Determine catalyst variety based on entity's weakest developmental area
        let variety = self.select_catalyst_variety(entity_state, entity_density, rng);

        // Calculate catalyst intensity based on entity readiness
        let intensity = self.calculate_catalyst_intensity(
            entity_state,
            entity_density,
            &context.environmental_factors,
        );

        // Determine challenge level based on intensity
        let challenge_level = self.determine_challenge_level(intensity);

        // Select catalyst type based on variety and environmental factors
        let catalyst_type = self.select_catalyst_type(variety, &context.environmental_factors, rng);

        // Generate description
        let description = self.generate_catalyst_description(variety, intensity, challenge_level);

        // Generate duration
        let duration = rng.gen_range(1..10);

        // Generate polarity influence
        let polarity_influence = rng.gen_range(-0.5..0.5);

        // Calculate affected entities (may include nearby entities)
        let affected_entities =
            self.calculate_affected_entities(entity_id, catalyst_type, context, rng);

        Some(Catalyst {
            catalyst_id: format!("catalyst-{}-{}", self.simulation_time, rng.gen::<u64>()),
            catalyst_type,
            variety,
            intensity,
            description,
            challenge_level,
            affected_entities,
            duration,
            start_time: self.simulation_time,
            polarity_influence,
        })
    }

    /// Select catalyst variety based on entity's weakest developmental area
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
    /// "Catalyst variety is implemented (emotional, intellectual, physical, spiritual)"
    fn select_catalyst_variety(
        &self,
        entity_state: &EntityState,
        entity_density: Option<&Density>,
        rng: &mut impl Rng,
    ) -> CatalystVariety {
        // At 3rd density and above, all varieties are possible
        // Below 3rd density, focus on physical and basic catalysts
        let is_third_density_or_above = match entity_density {
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

        if !is_third_density_or_above {
            // Below 3rd density - focus on physical catalysts
            return CatalystVariety::Physical;
        }

        // At 3rd density and above, select based on developmental needs
        // Analyze entity's archetype activations (simplified for now)
        let consciousness_level = entity_state.consciousness_level;
        let polarization_strength = entity_state.polarity_state.polarization_strength;

        // Weight different varieties based on entity state
        let emotional_need = 1.0 - consciousness_level;
        let intellectual_need = 1.0 - entity_state.learning_progress;
        let physical_need = 1.0 - polarization_strength;
        let spiritual_need = consciousness_level * polarization_strength;

        let total_need = emotional_need + intellectual_need + physical_need + spiritual_need;
        let roll = rng.gen::<f64>() * total_need;

        if roll < emotional_need {
            CatalystVariety::Emotional
        } else if roll < emotional_need + intellectual_need {
            CatalystVariety::Intellectual
        } else if roll < emotional_need + intellectual_need + physical_need {
            CatalystVariety::Physical
        } else {
            CatalystVariety::Spiritual
        }
    }

    /// Calculate catalyst intensity based on entity readiness
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
    /// "Intensity is based on entity's readiness (more ready = higher intensity)"
    fn calculate_catalyst_intensity(
        &self,
        entity_state: &EntityState,
        entity_density: Option<&Density>,
        env_factors: &EnvironmentalFactors,
    ) -> Float {
        // Base intensity from polarization strength
        let polarization_factor = entity_state.polarity_state.polarization_strength;

        // Consciousness level factor
        let consciousness_factor = entity_state.consciousness_level;

        // Experience factor
        const MAX_EXPERIENCE: Float = 100.0;
        let experience_factor = (entity_state.experience_accumulation / MAX_EXPERIENCE).min(1.0);

        // Density factor (higher density = higher intensity)
        let density_factor = match entity_density {
            Some(Density::First(_)) => 0.3,
            Some(Density::Second(_)) => 0.5,
            Some(Density::Third) => 0.7,
            Some(Density::Fourth) => 0.8,
            Some(Density::Fifth) => 0.85,
            Some(Density::Sixth) => 0.9,
            Some(Density::Seventh) => 0.95,
            Some(Density::Eighth) => 1.0,
            None => 0.3,
        };

        // Environmental factor (higher stress = higher intensity)
        let environmental_factor = env_factors.global_stress;

        // Weighted average
        let intensity = polarization_factor * 0.3
            + consciousness_factor * 0.2
            + experience_factor * 0.2
            + density_factor * 0.2
            + environmental_factor * 0.1;

        // Clamp to range [0.1, 1.0]
        intensity.clamp(0.1, 1.0)
    }

    /// Determine challenge level based on intensity
    fn determine_challenge_level(&self, intensity: Float) -> ChallengeLevel {
        if intensity < 0.33 {
            ChallengeLevel::Low
        } else if intensity < 0.66 {
            ChallengeLevel::Medium
        } else {
            ChallengeLevel::High
        }
    }

    /// Select catalyst type based on variety and environmental factors
    fn select_catalyst_type(
        &self,
        variety: CatalystVariety,
        env_factors: &EnvironmentalFactors,
        rng: &mut impl Rng,
    ) -> CatalystType {
        match variety {
            CatalystVariety::Emotional => {
                // Emotional catalysts influenced by social factors
                if env_factors.global_stress > 0.5 && rng.gen::<f64>() < 0.5 {
                    CatalystType::Karmic
                } else {
                    CatalystType::Social
                }
            }
            CatalystVariety::Intellectual => CatalystType::Intellectual,
            CatalystVariety::Physical => {
                // Physical catalysts influenced by environmental factors
                if env_factors.available_energy < 0.5 && rng.gen::<f64>() < 0.5 {
                    CatalystType::Environmental
                } else {
                    CatalystType::Physical
                }
            }
            CatalystVariety::Spiritual => CatalystType::Spiritual,
        }
    }

    /// Generate catalyst description
    fn generate_catalyst_description(
        &self,
        variety: CatalystVariety,
        intensity: Float,
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

    /// Calculate affected entities for a catalyst
    ///
    /// Includes the target entity and nearby entities that would be affected.
    fn calculate_affected_entities(
        &self,
        target_entity: &EntityId,
        _catalyst_type: CatalystType,
        context: &CatalystGenerationContext,
        rng: &mut impl Rng,
    ) -> Vec<EntityId> {
        let mut affected_entities = vec![target_entity.clone()];

        // Optionally include nearby entities (for social and environmental catalysts)
        if context.entity_states.len() > 1 {
            let num_additional = rng.gen_range(0..=3);
            let other_entities: Vec<_> = context
                .entity_states
                .keys()
                .filter(|id| *id != target_entity)
                .collect();

            for _ in 0..num_additional {
                if !other_entities.is_empty() {
                    let idx = rng.gen_range(0..other_entities.len());
                    let entity_id = other_entities[idx];
                    if !affected_entities.contains(entity_id) {
                        affected_entities.push(entity_id.clone());
                    }
                }
            }
        }

        affected_entities
    }

    /// Select target entity based on developmental needs
    fn select_target_entity(
        &self,
        context: &CatalystGenerationContext,
        rng: &mut impl Rng,
    ) -> Option<EntityId> {
        if context.entity_states.is_empty() {
            return None;
        }

        // Weight entities by their need for catalyst
        // Entities with lower consciousness or polarization need more catalysts
        let mut weighted_entities: Vec<(EntityId, Float)> = Vec::new();

        for (entity_id, entity_state) in &context.entity_states {
            // Calculate catalyst need
            let need_score = (1.0 - entity_state.consciousness_level) * 0.5
                + (1.0 - entity_state.polarity_state.polarization_strength) * 0.3
                + (1.0 - entity_state.learning_progress) * 0.2;

            weighted_entities.push((entity_id.clone(), need_score));
        }

        // Select entity based on weighted probability
        let total_weight: Float = weighted_entities.iter().map(|(_, w)| w).sum();
        let roll = rng.gen::<f64>() * total_weight;
        let mut cumulative = 0.0;

        for (entity_id, weight) in weighted_entities {
            cumulative += weight;
            if roll < cumulative {
                return Some(entity_id);
            }
        }

        // Fallback to random entity
        let keys: Vec<_> = context.entity_states.keys().collect();
        if keys.is_empty() {
            None
        } else {
            Some(keys[rng.gen_range(0..keys.len())].clone())
        }
    }

    /// Calculate number of catalysts to generate
    fn calculate_num_catalysts(&self, context: &CatalystGenerationContext) -> usize {
        let base_num = (context.entity_states.len() as Float * 0.1) as usize;
        let stress_factor = (context.environmental_factors.global_stress * 5.0) as usize;
        let entropy_factor = (context.environmental_factors.entropy_level * 3.0) as usize;

        (base_num + stress_factor + entropy_factor).clamp(1, 10)
    }

    /// Record catalyst generation for learning
    fn record_generation(&mut self, catalyst: &Catalyst, target_entity: Option<EntityId>) {
        let record = CatalystGenerationRecord {
            timestamp: self.simulation_time,
            target_entity,
            catalyst_type: catalyst.catalyst_type,
            catalyst_variety: catalyst.variety,
            intensity: catalyst.intensity,
            challenge_level: catalyst.challenge_level,
            success_metric: 0.0, // Will be updated after catalyst is processed
        };

        self.generation_history.push(record);
    }

    /// Update catalyst success metrics after processing
    pub fn update_catalyst_success(&mut self, catalyst_id: &str, success_metric: Float) {
        if let Some(record) = self
            .generation_history
            .iter_mut()
            .find(|r| catalyst_id.starts_with(&format!("catalyst-{}-", r.timestamp)))
        {
            record.success_metric = success_metric;
        }
    }

    /// Get catalyst manager
    pub fn get_catalyst_manager(&self) -> &CatalystManager {
        &self.catalyst_manager
    }

    /// Get mutable catalyst manager
    pub fn get_catalyst_manager_mut(&mut self) -> &mut CatalystManager {
        &mut self.catalyst_manager
    }

    /// Get generator state
    pub fn get_state(&self) -> CatalystGeneratorState {
        let mut catalysts_by_type = HashMap::new();

        for record in &self.generation_history {
            let type_str = format!("{:?}", record.catalyst_type);
            *catalysts_by_type.entry(type_str).or_insert(0) += 1;
        }

        let average_intensity = if !self.generation_history.is_empty() {
            let total: Float = self.generation_history.iter().map(|r| r.intensity).sum();
            total / self.generation_history.len() as Float
        } else {
            0.0
        };

        let success_rate = if !self.generation_history.is_empty() {
            let total: Float = self
                .generation_history
                .iter()
                .map(|r| r.success_metric)
                .sum();
            total / self.generation_history.len() as Float
        } else {
            0.0
        };

        CatalystGeneratorState {
            total_generated: self.generation_history.len(),
            catalysts_by_type,
            average_intensity,
            success_rate,
        }
    }
}

impl Default for CatalystGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{PolarityState, VibrationalState};
    use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};

    fn create_test_context() -> CatalystGenerationContext {
        let mut entity_states = HashMap::new();
        let mut entity_types = HashMap::new();
        let mut entity_densities = HashMap::new();

        // Create test entity
        let entity_id = EntityId::new("test-entity".to_string());
        entity_states.insert(
            entity_id.clone(),
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
            },
        );
        entity_types.insert(entity_id.clone(), EntityType::Individual);
        entity_densities.insert(entity_id.clone(), Density::Third);

        CatalystGenerationContext {
            simulation_time: 0,
            entity_states,
            entity_types,
            entity_densities,
            environmental_factors: EnvironmentalFactors::default(),
        }
    }

    #[test]
    fn test_catalyst_generator_creation() {
        let generator = CatalystGenerator::new();
        assert_eq!(generator.simulation_time, 0);
        assert!(generator.generation_history.is_empty());
    }

    #[test]
    fn test_generate_catalysts() {
        let mut generator = CatalystGenerator::new();
        let context = create_test_context();

        let catalysts = generator.generate_catalysts(&context);

        assert!(!catalysts.is_empty());
        assert!(generator.simulation_time > 0);
    }

    #[test]
    fn test_catalyst_intensity_calculation() {
        let generator = CatalystGenerator::new();
        let context = create_test_context();

        let entity_id = EntityId::new("test-entity".to_string());
        let entity_state = context.entity_states.get(&entity_id).unwrap();
        let entity_density = context.entity_densities.get(&entity_id);

        let intensity = generator.calculate_catalyst_intensity(
            entity_state,
            entity_density,
            &context.environmental_factors,
        );

        assert!(intensity >= 0.1);
        assert!(intensity <= 1.0);
    }

    #[test]
    fn test_challenge_level_determination() {
        let generator = CatalystGenerator::new();

        assert_eq!(
            generator.determine_challenge_level(0.2),
            ChallengeLevel::Low
        );
        assert_eq!(
            generator.determine_challenge_level(0.5),
            ChallengeLevel::Medium
        );
        assert_eq!(
            generator.determine_challenge_level(0.8),
            ChallengeLevel::High
        );
    }

    #[test]
    fn test_get_state() {
        let mut generator = CatalystGenerator::new();
        let context = create_test_context();

        generator.generate_catalysts(&context);

        let state = generator.get_state();
        assert!(state.total_generated > 0);
    }
}
