//! Entity Lifecycle Transitions
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "Entity merge/split/birth/death transitions"
//!
//! This module implements smooth transitions for entity lifecycle events:
//! - Birth: Field manifestation of new entities
//! - Merge: Two entities combining into one (collective formation)
//! - Split: One entity dividing into two (polarization)
//! - Death: Entity dissolution back into the field
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Entity does not 'become' something it was not—it REMEMBERS what it always was."
//!
//! Transitions are field-driven, not procedural. They emerge from field dynamics.

use std::collections::HashMap;

use super::extraction_pipeline::ExtractedEntity;
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;

/// Configuration for lifecycle transitions
#[derive(Debug, Clone)]
pub struct LifecycleTransitionConfig {
    /// Minimum coherence for birth
    pub birth_coherence_threshold: Float,
    /// Minimum resonance for merge
    pub merge_resonance_threshold: Float,
    /// Maximum polarization before split
    pub split_polarization_threshold: Float,
    /// Maximum entropy before death
    pub death_entropy_threshold: Float,
    /// Transition animation duration (steps)
    pub transition_duration: u64,
    /// Enable smooth transitions
    pub smooth_transitions: bool,
}

impl Default for LifecycleTransitionConfig {
    fn default() -> Self {
        Self {
            birth_coherence_threshold: 0.3,
            merge_resonance_threshold: 0.7,
            split_polarization_threshold: 0.9,
            death_entropy_threshold: 0.95,
            transition_duration: 10,
            smooth_transitions: true,
        }
    }
}

/// Types of lifecycle transitions
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    /// New entity birth from field
    Birth,
    /// Two entities merging
    Merge,
    /// Entity splitting into two
    Split,
    /// Entity dissolution
    Death,
}

/// Entity birth transition data
#[derive(Debug, Clone)]
pub struct EntityBirth {
    /// Position of birth
    pub position: Position3D,
    /// Initial coherence
    pub initial_coherence: Float,
    /// Source potential ID
    pub source_potential_id: u64,
    /// Birth progress (0.0 to 1.0)
    pub progress: Float,
    /// Derived entity data
    pub entity_data: ExtractedEntity,
}

/// Entity merge transition data
#[derive(Debug, Clone)]
pub struct EntityMerge {
    /// First entity ID
    pub entity_a_id: u64,
    /// Second entity ID
    pub entity_b_id: u64,
    /// Resulting entity ID
    pub result_entity_id: u64,
    /// Resonance between entities
    pub resonance: Float,
    /// Merge position
    pub position: Position3D,
    /// Merge progress (0.0 to 1.0)
    pub progress: Float,
    /// Emergent properties
    pub emergent_properties: Vec<String>,
}

/// Entity split transition data
#[derive(Debug, Clone)]
pub struct EntitySplit {
    /// Original entity ID
    pub original_entity_id: u64,
    /// First resulting entity ID
    pub result_a_id: u64,
    /// Second resulting entity ID
    pub result_b_id: u64,
    /// Polarization that caused split
    pub polarization: Float,
    /// Split progress (0.0 to 1.0)
    pub progress: Float,
    /// Polarity of result A
    pub result_a_polarity: Float,
    /// Polarity of result B
    pub result_b_polarity: Float,
}

/// Entity death transition data
#[derive(Debug, Clone)]
pub struct EntityDeath {
    /// Entity ID
    pub entity_id: u64,
    /// Position at death
    pub position: Position3D,
    /// Entropy level at death
    pub entropy: Float,
    /// Death progress (0.0 to 1.0)
    pub progress: Float,
    /// Experiences returned to field
    pub experiences_returned: u64,
    /// Karmic patterns resolved
    pub karmic_resolutions: u64,
}

/// Active lifecycle transition
#[derive(Debug, Clone)]
pub struct EntityLifecycleTransition {
    /// Transition ID
    pub transition_id: u64,
    /// Transition type
    pub transition_type: TransitionType,
    /// Step when transition started
    pub start_step: u64,
    /// Step when transition completes
    pub end_step: u64,
    /// Current progress
    pub progress: Float,
    /// Birth data (if applicable)
    pub birth_data: Option<EntityBirth>,
    /// Merge data (if applicable)
    pub merge_data: Option<EntityMerge>,
    /// Split data (if applicable)
    pub split_data: Option<EntitySplit>,
    /// Death data (if applicable)
    pub death_data: Option<EntityDeath>,
}

/// Lifecycle Transition Manager
pub struct LifecycleTransitionManager {
    /// Configuration
    config: LifecycleTransitionConfig,
    /// Active transitions
    active_transitions: HashMap<u64, EntityLifecycleTransition>,
    /// Completed transitions count
    completed_count: u64,
    /// Next transition ID
    next_transition_id: u64,
    /// Statistics
    statistics: TransitionStatistics,
}

#[derive(Debug, Clone, Default)]
struct TransitionStatistics {
    births: u64,
    merges: u64,
    splits: u64,
    deaths: u64,
    total_entities_created: u64,
    total_entities_dissolved: u64,
}

impl LifecycleTransitionManager {
    /// Create a new transition manager
    pub fn new(config: LifecycleTransitionConfig) -> Self {
        Self {
            config,
            active_transitions: HashMap::new(),
            completed_count: 0,
            next_transition_id: 1,
            statistics: TransitionStatistics::default(),
        }
    }

    /// Initiate a birth transition
    pub fn initiate_birth(
        &mut self,
        position: Position3D,
        coherence: Float,
        entity_data: ExtractedEntity,
        current_step: u64,
    ) -> u64 {
        let transition_id = self.next_transition_id;
        self.next_transition_id += 1;

        let birth_data = EntityBirth {
            position,
            initial_coherence: coherence,
            source_potential_id: entity_data.entity_id,
            progress: 0.0,
            entity_data,
        };

        let transition = EntityLifecycleTransition {
            transition_id,
            transition_type: TransitionType::Birth,
            start_step: current_step,
            end_step: current_step + self.config.transition_duration,
            progress: 0.0,
            birth_data: Some(birth_data),
            merge_data: None,
            split_data: None,
            death_data: None,
        };

        self.active_transitions.insert(transition_id, transition);
        self.statistics.births += 1;

        transition_id
    }

    /// Initiate a merge transition
    pub fn initiate_merge(
        &mut self,
        entity_a_id: u64,
        entity_b_id: u64,
        position: Position3D,
        resonance: Float,
        current_step: u64,
    ) -> u64 {
        let transition_id = self.next_transition_id;
        self.next_transition_id += 1;

        let result_entity_id = self.next_transition_id + 1000000;

        let merge_data = EntityMerge {
            entity_a_id,
            entity_b_id,
            result_entity_id,
            resonance,
            position,
            progress: 0.0,
            emergent_properties: Vec::new(),
        };

        let transition = EntityLifecycleTransition {
            transition_id,
            transition_type: TransitionType::Merge,
            start_step: current_step,
            end_step: current_step + self.config.transition_duration * 2,
            progress: 0.0,
            birth_data: None,
            merge_data: Some(merge_data),
            split_data: None,
            death_data: None,
        };

        self.active_transitions.insert(transition_id, transition);
        self.statistics.merges += 1;

        transition_id
    }

    /// Initiate a split transition
    pub fn initiate_split(
        &mut self,
        original_entity_id: u64,
        polarization: Float,
        current_step: u64,
    ) -> u64 {
        let transition_id = self.next_transition_id;
        self.next_transition_id += 1;

        let result_a_id = self.next_transition_id + 2000000;
        let result_b_id = self.next_transition_id + 3000000;

        let split_data = EntitySplit {
            original_entity_id,
            result_a_id,
            result_b_id,
            polarization,
            progress: 0.0,
            result_a_polarity: 1.0,
            result_b_polarity: -1.0,
        };

        let transition = EntityLifecycleTransition {
            transition_id,
            transition_type: TransitionType::Split,
            start_step: current_step,
            end_step: current_step + self.config.transition_duration * 2,
            progress: 0.0,
            birth_data: None,
            merge_data: None,
            split_data: Some(split_data),
            death_data: None,
        };

        self.active_transitions.insert(transition_id, transition);
        self.statistics.splits += 1;

        transition_id
    }

    /// Initiate a death transition
    pub fn initiate_death(
        &mut self,
        entity_id: u64,
        position: Position3D,
        entropy: Float,
        experiences: u64,
        current_step: u64,
    ) -> u64 {
        let transition_id = self.next_transition_id;
        self.next_transition_id += 1;

        let death_data = EntityDeath {
            entity_id,
            position,
            entropy,
            progress: 0.0,
            experiences_returned: experiences,
            karmic_resolutions: 0,
        };

        let transition = EntityLifecycleTransition {
            transition_id,
            transition_type: TransitionType::Death,
            start_step: current_step,
            end_step: current_step + self.config.transition_duration,
            progress: 0.0,
            birth_data: None,
            merge_data: None,
            split_data: None,
            death_data: Some(death_data),
        };

        self.active_transitions.insert(transition_id, transition);
        self.statistics.deaths += 1;

        transition_id
    }

    /// Update all active transitions
    pub fn update(&mut self, current_step: u64) -> Vec<u64> {
        let mut completed = Vec::new();

        for (id, transition) in self.active_transitions.iter_mut() {
            let total_duration = transition.end_step - transition.start_step;
            let elapsed = current_step - transition.start_step;

            transition.progress = if total_duration > 0 {
                (elapsed as Float / total_duration as Float).min(1.0)
            } else {
                1.0
            };

            if self.config.smooth_transitions {
                let smooth_t =
                    transition.progress * transition.progress * (3.0 - 2.0 * transition.progress);
                transition.progress = smooth_t;
            }

            match transition.transition_type {
                TransitionType::Birth => {
                    if let Some(ref mut birth) = transition.birth_data {
                        birth.progress = transition.progress;
                    }
                }
                TransitionType::Merge => {
                    if let Some(ref mut merge) = transition.merge_data {
                        merge.progress = transition.progress;
                    }
                }
                TransitionType::Split => {
                    if let Some(ref mut split) = transition.split_data {
                        split.progress = transition.progress;
                    }
                }
                TransitionType::Death => {
                    if let Some(ref mut death) = transition.death_data {
                        death.progress = transition.progress;
                    }
                }
            }

            if transition.progress >= 1.0 {
                completed.push(*id);
                self.completed_count += 1;

                match transition.transition_type {
                    TransitionType::Birth => {
                        self.statistics.total_entities_created += 1;
                    }
                    TransitionType::Death => {
                        self.statistics.total_entities_dissolved += 1;
                    }
                    _ => {}
                }
            }
        }

        for id in &completed {
            self.active_transitions.remove(id);
        }

        completed
    }

    /// Get active transition by ID
    pub fn get_transition(&self, id: u64) -> Option<&EntityLifecycleTransition> {
        self.active_transitions.get(&id)
    }

    /// Get all active transitions
    pub fn active_transitions(&self) -> &HashMap<u64, EntityLifecycleTransition> {
        &self.active_transitions
    }

    /// Check if merge conditions are met
    pub fn check_merge_conditions(
        &self,
        entity_a: &ExtractedEntity,
        entity_b: &ExtractedEntity,
    ) -> bool {
        let resonance = self.calculate_resonance(entity_a, entity_b);
        resonance >= self.config.merge_resonance_threshold
    }

    /// Calculate resonance between two entities
    fn calculate_resonance(&self, entity_a: &ExtractedEntity, entity_b: &ExtractedEntity) -> Float {
        let mut resonance = 1.0;

        for i in 0..22 {
            let diff = (entity_a.archetype_vector[i] - entity_b.archetype_vector[i]).abs();
            resonance *= 1.0 - diff;
        }

        resonance *= entity_a.coherence * entity_b.coherence;

        let density_match = if std::mem::discriminant(&entity_a.density)
            == std::mem::discriminant(&entity_b.density)
        {
            1.2
        } else {
            0.8
        };

        resonance * density_match
    }

    /// Check if split conditions are met
    pub fn check_split_conditions(&self, polarization: Float) -> bool {
        polarization.abs() >= self.config.split_polarization_threshold
    }

    /// Check if death conditions are met
    pub fn check_death_conditions(&self, entropy: Float) -> bool {
        entropy >= self.config.death_entropy_threshold
    }

    /// Get transition statistics
    pub fn statistics(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("births".to_string(), self.statistics.births);
        stats.insert("merges".to_string(), self.statistics.merges);
        stats.insert("splits".to_string(), self.statistics.splits);
        stats.insert("deaths".to_string(), self.statistics.deaths);
        stats.insert(
            "total_entities_created".to_string(),
            self.statistics.total_entities_created,
        );
        stats.insert(
            "total_entities_dissolved".to_string(),
            self.statistics.total_entities_dissolved,
        );
        stats.insert(
            "active_transitions".to_string(),
            self.active_transitions.len() as u64,
        );
        stats.insert("completed_transitions".to_string(), self.completed_count);
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic_foundation::spectrum::SpectrumState;
    use crate::holographic_foundation::template::Density;

    #[test]
    fn test_transition_config_default() {
        let config = LifecycleTransitionConfig::default();
        assert_eq!(config.birth_coherence_threshold, 0.3);
        assert_eq!(config.merge_resonance_threshold, 0.7);
    }

    #[test]
    fn test_lifecycle_manager_creation() {
        let config = LifecycleTransitionConfig::default();
        let manager = LifecycleTransitionManager::new(config);
        let stats = manager.statistics();
        assert_eq!(stats.get("births"), Some(&0));
    }

    #[test]
    fn test_initiate_birth() {
        let config = LifecycleTransitionConfig::default();
        let mut manager = LifecycleTransitionManager::new(config);

        let entity_data = ExtractedEntity {
            entity_id: 1,
            position: Position3D::new(0.5, 0.5, 0.5),
            coherence: 0.8,
            amplitude: 1.0,
            density: Density::Third,
            spectrum_state: SpectrumState::default(),
            archetype_vector: [0.5; 22],
            dominant_archetype: 0,
            extraction_time: 0,
        };

        let transition_id =
            manager.initiate_birth(Position3D::new(0.5, 0.5, 0.5), 0.8, entity_data, 0);

        assert!(transition_id > 0);
        assert_eq!(manager.active_transitions().len(), 1);
    }

    #[test]
    fn test_initiate_merge() {
        let config = LifecycleTransitionConfig::default();
        let mut manager = LifecycleTransitionManager::new(config);

        let transition_id = manager.initiate_merge(1, 2, Position3D::new(0.5, 0.5, 0.5), 0.8, 0);

        assert!(transition_id > 0);

        let stats = manager.statistics();
        assert_eq!(stats.get("merges"), Some(&1));
    }

    #[test]
    fn test_initiate_split() {
        let config = LifecycleTransitionConfig::default();
        let mut manager = LifecycleTransitionManager::new(config);

        let transition_id = manager.initiate_split(1, 0.95, 0);

        assert!(transition_id > 0);

        let stats = manager.statistics();
        assert_eq!(stats.get("splits"), Some(&1));
    }

    #[test]
    fn test_initiate_death() {
        let config = LifecycleTransitionConfig::default();
        let mut manager = LifecycleTransitionManager::new(config);

        let transition_id = manager.initiate_death(1, Position3D::new(0.5, 0.5, 0.5), 0.98, 100, 0);

        assert!(transition_id > 0);

        let stats = manager.statistics();
        assert_eq!(stats.get("deaths"), Some(&1));
    }

    #[test]
    fn test_transition_update() {
        let config = LifecycleTransitionConfig {
            transition_duration: 5,
            ..Default::default()
        };
        let mut manager = LifecycleTransitionManager::new(config);

        let entity_data = ExtractedEntity {
            entity_id: 1,
            position: Position3D::new(0.5, 0.5, 0.5),
            coherence: 0.8,
            amplitude: 1.0,
            density: Density::Third,
            spectrum_state: SpectrumState::default(),
            archetype_vector: [0.5; 22],
            dominant_archetype: 0,
            extraction_time: 0,
        };

        manager.initiate_birth(Position3D::new(0.5, 0.5, 0.5), 0.8, entity_data, 0);

        let completed = manager.update(5);

        assert_eq!(completed.len(), 1);
        assert_eq!(manager.active_transitions().len(), 0);
    }

    #[test]
    fn test_check_merge_conditions() {
        let config = LifecycleTransitionConfig::default();
        let manager = LifecycleTransitionManager::new(config);

        let entity_a = ExtractedEntity {
            entity_id: 1,
            position: Position3D::new(0.5, 0.5, 0.5),
            coherence: 0.9,
            amplitude: 1.0,
            density: Density::Third,
            spectrum_state: SpectrumState::default(),
            archetype_vector: [0.5; 22],
            dominant_archetype: 0,
            extraction_time: 0,
        };

        let entity_b = ExtractedEntity {
            entity_id: 2,
            position: Position3D::new(0.5, 0.5, 0.5),
            coherence: 0.9,
            amplitude: 1.0,
            density: Density::Third,
            spectrum_state: SpectrumState::default(),
            archetype_vector: [0.5; 22],
            dominant_archetype: 0,
            extraction_time: 0,
        };

        assert!(manager.check_merge_conditions(&entity_a, &entity_b));
    }

    #[test]
    fn test_check_split_conditions() {
        let config = LifecycleTransitionConfig::default();
        let manager = LifecycleTransitionManager::new(config);

        assert!(manager.check_split_conditions(0.95));
        assert!(!manager.check_split_conditions(0.5));
    }

    #[test]
    fn test_check_death_conditions() {
        let config = LifecycleTransitionConfig::default();
        let manager = LifecycleTransitionManager::new(config);

        assert!(manager.check_death_conditions(0.98));
        assert!(!manager.check_death_conditions(0.5));
    }
}
