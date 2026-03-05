// Individual Entity Progression Module (Phase 3)
//
// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 3:
// "Implement individual entity progression within their density"
//
// This module implements:
// 1. Individual entity progression tracking (entities progress WITHIN their density)
// 2. Density-specific progression logic for all 8 densities
// 3. Polarity system for 3rd Density entities
// 4. Progression metrics and scoring
//
// KEY PRINCIPLE: Individual entities DO NOT change density - they progress WITHIN their density.
// Collective emergence (when densities emerge in the system) is tracked separately in CollectiveSystemManager.

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::evolution_density_octave::density_octave::Density;
use crate::simulation_v3::catalyst_system::{Catalyst, CatalystEvent, PolarityChoice};
use std::collections::HashMap;

/// Individual Progression Manager
///
/// Tracks individual entity progression within their density.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Individual entities at each scale progress in their own way"
/// - 1st Density entities: Progress through physical interaction
/// - 2nd Density entities: Progress through growth and survival
/// - 3rd Density entities: Progress through polarization and choice
/// - 4th+ Density entities: Progress through wisdom and service
pub struct IndividualProgressionManager {
    /// Progression state for each entity
    entity_progression: HashMap<EntityId, EntityProgressionState>,

    /// Simulation time
    simulation_time: u64,

    /// Progression statistics
    statistics: ProgressionStatistics,
}

/// Entity Progression State
///
/// Tracks the progression state of a single entity within its density.
#[derive(Debug, Clone)]
pub struct EntityProgressionState {
    /// Entity ID
    pub entity_id: EntityId,

    /// Entity's density level (fixed - entities do NOT change density)
    pub current_density: Density,

    /// Overall progression score within density (0.0 to 1.0)
    pub progression_score: f64,

    /// Density-specific progression metrics
    pub density_specific_metrics: DensitySpecificProgression,

    /// Polarity state (3rd Density only)
    pub polarization: Option<EntityPolarizationState>,

    /// Creation time
    pub creation_time: u64,

    /// Last update time
    pub last_update_time: u64,
}

/// Density-Specific Progression Metrics
///
/// Different progression metrics for each density level.
#[derive(Debug, Clone)]
pub enum DensitySpecificProgression {
    /// 1st Density: Physical interaction progression
    FirstDensity {
        /// Physical interaction score (quantum bonding, atomic bonding)
        physical_interaction_score: f64,

        /// Bonding formation count
        bonding_formation_count: u64,

        /// Stability score
        stability_score: f64,

        /// Energy integration score
        energy_integration_score: f64,
    },

    /// 2nd Density: Growth and survival progression
    SecondDensity {
        /// Growth score (cellular growth, organism growth)
        growth_score: f64,

        /// Survival score (survival challenges overcome)
        survival_score: f64,

        /// Reproduction count
        reproduction_count: u64,

        /// Adaptation score (environmental adaptation)
        adaptation_score: f64,
    },

    /// 3rd Density: Polarity and choice progression
    ThirdDensity {
        /// Polarity percentage (-100% STS to +100% STO)
        polarization_percentage: f64,

        /// Catalyst events processed
        catalyst_events_processed: u64,

        /// Wisdom score (accumulated wisdom from choices)
        wisdom_score: f64,

        /// Free will exercises (number of free will choices made)
        free_will_exercises: u64,
    },

    /// 4th Density: Service and social memory complex progression
    FourthDensity {
        /// Service score (service to others)
        service_score: f64,

        /// Social memory complex integration score
        social_memory_complex_score: f64,

        /// Wisdom score
        wisdom_score: f64,

        /// Love understanding score
        love_understanding_score: f64,
    },

    /// 5th Density: Unity and wisdom progression
    FifthDensity {
        /// Unity score (unity with others)
        unity_score: f64,

        /// Light score (light/energy work)
        light_score: f64,

        /// Wisdom score
        wisdom_score: f64,

        /// Teaching ability score
        teaching_ability_score: f64,
    },

    /// 6th Density: Balance and equality progression
    SixthDensity {
        /// Balance score (balance of all polarities)
        balance_score: f64,

        /// Equality score (seeing all as equal)
        equality_score: f64,

        /// Harmony score
        harmony_score: f64,

        /// Integration score (integration of all lessons)
        integration_score: f64,
    },

    /// 7th Density: Completion and review progression
    SeventhDensity {
        /// Completion score (completion of all density lessons)
        completion_score: f64,

        /// Review score (review of entire octave journey)
        review_score: f64,

        /// Preparation score (preparation for 8th density)
        preparation_score: f64,

        /// Mastery score (mastery of all densities)
        mastery_score: f64,
    },

    /// 8th Density: Return to Creator progression
    EighthDensity {
        /// Return score (return to Creator)
        return_score: f64,

        /// Unity with Creator score
        unity_with_creator_score: f64,

        /// Transcendence score
        transcendence_score: f64,

        /// Completion score (completion of octave)
        completion_score: f64,
    },
}

/// Entity Polarity State
///
/// Tracks polarization for 3rd Density entities.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Polarity is the alignment of service: service-to-others (STO) or service-to-self (STS)"
#[derive(Debug, Clone)]
pub struct EntityPolarizationState {
    /// Service-to-Others alignment (0.0 to 1.0)
    pub sto_alignment: f64,

    /// Service-to-Self alignment (0.0 to 1.0)
    pub sts_alignment: f64,

    /// Net polarization percentage (-100% STS to +100% STO)
    pub polarization_percentage: f64,

    /// Catalyst events experienced
    pub catalyst_events: Vec<CatalystEvent>,

    /// Polarity choices made
    pub polarity_choices: Vec<PolarityChoice>,

    /// Harvest readiness (51%+ polarization required for harvest)
    pub harvest_ready: bool,

    /// Current polarity (STO or STS)
    pub current_polarity: Option<EntityPolarity>,
}

/// Entity Polarity
///
/// The polarity of a 3rd Density entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityPolarity {
    /// Service-to-Others (positive polarity)
    STO,

    /// Service-to-Self (negative polarity)
    STS,

    /// Unpolarized (not yet chosen)
    Unpolarized,
}

/// Progression Statistics
///
/// Tracks statistics about entity progression.
#[derive(Debug, Clone)]
pub struct ProgressionStatistics {
    /// Total entities tracked
    pub total_entities: u64,

    /// Entities at each density
    pub entities_by_density: HashMap<String, u64>,

    /// Average progression score
    pub average_progression_score: f64,

    /// Total polarity choices made
    pub total_polarity_choices: u64,

    /// STO vs STS distribution
    pub sto_count: u64,
    pub sts_count: u64,
    pub unpolarized_count: u64,
}

impl Default for IndividualProgressionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl IndividualProgressionManager {
    /// Create a new individual progression manager
    pub fn new() -> Self {
        IndividualProgressionManager {
            entity_progression: HashMap::new(),
            simulation_time: 0,
            statistics: ProgressionStatistics::new(),
        }
    }

    /// Register an entity for progression tracking
    pub fn register_entity(&mut self, entity: &SubSubLogos, density: Density) {
        let progression_state =
            EntityProgressionState::new(entity.entity_id.clone(), density, self.simulation_time);
        self.entity_progression
            .insert(entity.entity_id.clone(), progression_state);
        self.update_statistics();
    }

    /// Update progression for all entities
    pub fn update_all_progression<F>(&mut self, get_entity_fn: F)
    where
        F: Fn(EntityId) -> Option<SubSubLogos>,
    {
        let entity_ids: Vec<_> = self.entity_progression.keys().cloned().collect();

        for entity_id in entity_ids {
            if let Some(entity) = get_entity_fn(entity_id) {
                self.update_entity_progression(&entity);
            }
        }

        self.simulation_time += 1;
    }

    /// Update progression for a single entity
    pub fn update_entity_progression(&mut self, entity: &SubSubLogos) {
        if let Some(progression_state) = self.entity_progression.get_mut(&entity.entity_id) {
            let consciousness_level = entity.current_state.consciousness_level;
            let stability = entity.current_state.vibrational_state.coherence; // Use coherence as stability
            let energy_level = entity.current_state.vibrational_state.amplitude; // Use amplitude as energy
            let harmonic_resonance = entity.current_state.vibrational_state.frequency; // Use frequency as resonance
            let wisdom_level = entity.current_state.learning_progress; // Use learning_progress as wisdom proxy

            match &mut progression_state.density_specific_metrics {
                DensitySpecificProgression::FirstDensity {
                    physical_interaction_score,
                    bonding_formation_count,
                    stability_score,
                    energy_integration_score,
                } => {
                    // 1st Density: Progress through physical interaction
                    *physical_interaction_score =
                        (*physical_interaction_score * 0.95 + consciousness_level * 0.05).min(1.0);
                    *stability_score = (*stability_score * 0.98 + stability * 0.02).min(1.0);
                    *energy_integration_score =
                        (*energy_integration_score * 0.97 + energy_level * 0.03).min(1.0);

                    // Random bonding formation
                    if rand::random::<f64>() < 0.01 {
                        *bonding_formation_count += 1;
                    }

                    progression_state.progression_score = (*physical_interaction_score * 0.4
                        + *stability_score * 0.3
                        + *energy_integration_score * 0.3)
                        .min(1.0);
                }

                DensitySpecificProgression::SecondDensity {
                    growth_score,
                    survival_score,
                    reproduction_count,
                    adaptation_score,
                } => {
                    // 2nd Density: Progress through growth and survival
                    *growth_score = (*growth_score * 0.96 + consciousness_level * 0.04).min(1.0);
                    *survival_score = (*survival_score * 0.97 + stability * 0.03).min(1.0);
                    *adaptation_score =
                        (*adaptation_score * 0.98 + harmonic_resonance * 0.02).min(1.0);

                    // Random reproduction
                    if rand::random::<f64>() < 0.005 {
                        *reproduction_count += 1;
                    }

                    progression_state.progression_score =
                        (*growth_score * 0.4 + *survival_score * 0.3 + *adaptation_score * 0.3)
                            .min(1.0);
                }

                DensitySpecificProgression::ThirdDensity {
                    polarization_percentage,
                    catalyst_events_processed: _,
                    wisdom_score,
                    free_will_exercises: _,
                } => {
                    // 3rd Density: Progress through polarization and wisdom
                    *wisdom_score = (*wisdom_score * 0.97 + wisdom_level * 0.03).min(1.0);

                    progression_state.progression_score =
                        (*wisdom_score * 0.5 + polarization_percentage.abs() * 0.5).min(1.0);
                }

                DensitySpecificProgression::FourthDensity {
                    service_score,
                    social_memory_complex_score,
                    wisdom_score,
                    love_understanding_score,
                } => {
                    // 4th Density: Progress through service and love
                    *service_score = (*service_score * 0.96 + harmonic_resonance * 0.04).min(1.0);
                    *wisdom_score = (*wisdom_score * 0.97 + wisdom_level * 0.03).min(1.0);
                    *love_understanding_score =
                        (*love_understanding_score * 0.98 + harmonic_resonance * 0.02).min(1.0);

                    progression_state.progression_score = (*service_score * 0.3
                        + *social_memory_complex_score * 0.3
                        + *wisdom_score * 0.2
                        + *love_understanding_score * 0.2)
                        .min(1.0);
                }

                DensitySpecificProgression::FifthDensity {
                    unity_score,
                    light_score,
                    wisdom_score,
                    teaching_ability_score,
                } => {
                    // 5th Density: Progress through unity and light
                    *unity_score = (*unity_score * 0.97 + harmonic_resonance * 0.03).min(1.0);
                    *wisdom_score = (*wisdom_score * 0.98 + wisdom_level * 0.02).min(1.0);

                    progression_state.progression_score = (*unity_score * 0.3
                        + *light_score * 0.3
                        + *wisdom_score * 0.2
                        + *teaching_ability_score * 0.2)
                        .min(1.0);
                }

                DensitySpecificProgression::SixthDensity {
                    balance_score,
                    equality_score,
                    harmony_score,
                    integration_score,
                } => {
                    // 6th Density: Progress through balance and equality
                    *balance_score = (*balance_score * 0.97 + harmonic_resonance * 0.03).min(1.0);
                    *harmony_score = (*harmony_score * 0.98 + harmonic_resonance * 0.02).min(1.0);

                    progression_state.progression_score = (*balance_score * 0.3
                        + *equality_score * 0.3
                        + *harmony_score * 0.2
                        + *integration_score * 0.2)
                        .min(1.0);
                }

                DensitySpecificProgression::SeventhDensity {
                    completion_score,
                    review_score,
                    preparation_score,
                    mastery_score,
                } => {
                    // 7th Density: Progress through completion and review
                    *completion_score = (*completion_score * 0.98 + wisdom_level * 0.02).min(1.0);
                    *mastery_score = (*mastery_score * 0.97 + wisdom_level * 0.03).min(1.0);

                    progression_state.progression_score = (*completion_score * 0.3
                        + *review_score * 0.3
                        + *preparation_score * 0.2
                        + *mastery_score * 0.2)
                        .min(1.0);
                }

                DensitySpecificProgression::EighthDensity {
                    return_score,
                    unity_with_creator_score,
                    transcendence_score,
                    completion_score,
                } => {
                    // 8th Density: Progress through return to Creator
                    *return_score = (*return_score * 0.99 + harmonic_resonance * 0.01).min(1.0);
                    *unity_with_creator_score =
                        (*unity_with_creator_score * 0.99 + harmonic_resonance * 0.01).min(1.0);

                    progression_state.progression_score = (*return_score * 0.4
                        + *unity_with_creator_score * 0.3
                        + *transcendence_score * 0.2
                        + *completion_score * 0.1)
                        .min(1.0);
                }
            }

            progression_state.last_update_time = self.simulation_time;
        }

        self.update_statistics();
    }

    /// Get progression state for an entity
    pub fn get_entity_progression(&self, entity_id: EntityId) -> Option<&EntityProgressionState> {
        self.entity_progression.get(&entity_id)
    }

    /// Get mutable progression state for an entity
    pub fn get_entity_progression_mut(
        &mut self,
        entity_id: EntityId,
    ) -> Option<&mut EntityProgressionState> {
        self.entity_progression.get_mut(&entity_id)
    }

    /// Process a catalyst event for an entity (3rd Density only)
    pub fn process_catalyst_event(&mut self, entity_id: EntityId, catalyst: &Catalyst) {
        if let Some(progression_state) = self.entity_progression.get_mut(&entity_id) {
            if let Some(polarization) = &mut progression_state.polarization {
                // Record catalyst event (simplified - just store catalyst info)
                let catalyst_event = CatalystEvent {
                    event_id: polarization.catalyst_events.len() as u64,
                    catalyst_id: catalyst.catalyst_id.clone(),
                    entity_id,
                    event_type: catalyst.catalyst_type,
                    catalyst_variety: catalyst.variety,
                    intensity: catalyst.intensity,
                    challenge_level: catalyst.challenge_level,
                    polarity_choice: PolarityChoice::Unpolarized,
                    learning_value: 0.0,
                    consciousness_expansion: 0.0,
                    growth: crate::simulation_v3::catalyst_system::CatalystGrowth {
                        experience: 0.0,
                        learning: 0.0,
                        polarization_change: 0.0,
                    },
                    timestamp: self.simulation_time,
                };

                polarization.catalyst_events.push(catalyst_event);

                // Update catalyst events processed
                if let DensitySpecificProgression::ThirdDensity {
                    catalyst_events_processed,
                    ..
                } = &mut progression_state.density_specific_metrics
                {
                    *catalyst_events_processed += 1;
                }

                self.update_statistics();
            }
        }
    }

    /// Make a polarity choice for an entity (3rd Density only)
    pub fn make_polarity_choice(&mut self, entity_id: EntityId, choice: PolarityChoice) {
        if let Some(progression_state) = self.entity_progression.get_mut(&entity_id) {
            if let Some(polarization) = &mut progression_state.polarization {
                // Record polarity choice
                polarization.polarity_choices.push(choice);

                // Update alignment based on choice
                match choice {
                    PolarityChoice::ServiceToOthers => {
                        polarization.sto_alignment = (polarization.sto_alignment + 0.05).min(1.0);
                    }
                    PolarityChoice::ServiceToSelf => {
                        polarization.sts_alignment = (polarization.sts_alignment + 0.05).min(1.0);
                    }
                    PolarityChoice::Unpolarized => {
                        // No change
                    }
                }

                // Recalculate polarization percentage
                polarization.polarization_percentage =
                    (polarization.sto_alignment - polarization.sts_alignment) * 100.0;

                // Determine current polarity
                polarization.current_polarity = if polarization.polarization_percentage.abs() < 10.0
                {
                    Some(EntityPolarity::Unpolarized)
                } else if polarization.polarization_percentage > 0.0 {
                    Some(EntityPolarity::STO)
                } else {
                    Some(EntityPolarity::STS)
                };

                // Check harvest readiness (51%+ polarization required)
                polarization.harvest_ready = polarization.polarization_percentage.abs() >= 51.0;

                // Update free will exercises
                if let DensitySpecificProgression::ThirdDensity {
                    free_will_exercises,
                    polarization_percentage,
                    ..
                } = &mut progression_state.density_specific_metrics
                {
                    *free_will_exercises += 1;
                    *polarization_percentage = polarization.polarization_percentage;
                }

                self.update_statistics();
            }
        }
    }

    /// Calculate polarization percentage for an entity
    pub fn calculate_polarization_percentage(&self, entity_id: EntityId) -> Option<f64> {
        self.entity_progression
            .get(&entity_id)
            .and_then(|state| state.polarization.as_ref())
            .map(|polarization| polarization.polarization_percentage)
    }

    /// Get polarization state for an entity
    pub fn get_polarization_state(&self, entity_id: EntityId) -> Option<&EntityPolarizationState> {
        self.entity_progression
            .get(&entity_id)
            .and_then(|state| state.polarization.as_ref())
    }

    /// Get progression statistics
    pub fn get_statistics(&self) -> &ProgressionStatistics {
        &self.statistics
    }

    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_entities = self.entity_progression.len() as u64;

        // Count entities by density
        self.statistics.entities_by_density.clear();
        for progression_state in self.entity_progression.values() {
            let density_name = match &progression_state.current_density {
                Density::First(_) => "1st Density",
                Density::Second(_) => "2nd Density",
                Density::Third => "3rd Density",
                Density::Fourth => "4th Density",
                Density::Fifth => "5th Density",
                Density::Sixth => "6th Density",
                Density::Seventh => "7th Density",
                Density::Eighth => "8th Density",
            };

            *self
                .statistics
                .entities_by_density
                .entry(density_name.to_string())
                .or_insert(0) += 1;
        }

        // Calculate average progression score
        if !self.entity_progression.is_empty() {
            let total_score: f64 = self
                .entity_progression
                .values()
                .map(|state| state.progression_score)
                .sum();

            self.statistics.average_progression_score =
                total_score / self.entity_progression.len() as f64;
        }

        // Count polarity distribution
        self.statistics.sto_count = 0;
        self.statistics.sts_count = 0;
        self.statistics.unpolarized_count = 0;
        self.statistics.total_polarity_choices = 0;

        for progression_state in self.entity_progression.values() {
            if let Some(polarization) = &progression_state.polarization {
                self.statistics.total_polarity_choices +=
                    polarization.polarity_choices.len() as u64;

                match polarization.current_polarity {
                    Some(EntityPolarity::STO) => self.statistics.sto_count += 1,
                    Some(EntityPolarity::STS) => self.statistics.sts_count += 1,
                    Some(EntityPolarity::Unpolarized) => self.statistics.unpolarized_count += 1,
                    None => self.statistics.unpolarized_count += 1,
                }
            }
        }
    }
}

impl EntityProgressionState {
    /// Create a new entity progression state
    pub fn new(entity_id: EntityId, density: Density, creation_time: u64) -> Self {
        let density_specific_metrics = match &density {
            Density::First(_) => DensitySpecificProgression::FirstDensity {
                physical_interaction_score: 0.1,
                bonding_formation_count: 0,
                stability_score: 0.5,
                energy_integration_score: 0.1,
            },
            Density::Second(_) => DensitySpecificProgression::SecondDensity {
                growth_score: 0.1,
                survival_score: 0.5,
                reproduction_count: 0,
                adaptation_score: 0.1,
            },
            Density::Third => DensitySpecificProgression::ThirdDensity {
                polarization_percentage: 0.0,
                catalyst_events_processed: 0,
                wisdom_score: 0.1,
                free_will_exercises: 0,
            },
            Density::Fourth => DensitySpecificProgression::FourthDensity {
                service_score: 0.1,
                social_memory_complex_score: 0.1,
                wisdom_score: 0.3,
                love_understanding_score: 0.1,
            },
            Density::Fifth => DensitySpecificProgression::FifthDensity {
                unity_score: 0.1,
                light_score: 0.1,
                wisdom_score: 0.4,
                teaching_ability_score: 0.1,
            },
            Density::Sixth => DensitySpecificProgression::SixthDensity {
                balance_score: 0.1,
                equality_score: 0.1,
                harmony_score: 0.1,
                integration_score: 0.1,
            },
            Density::Seventh => DensitySpecificProgression::SeventhDensity {
                completion_score: 0.1,
                review_score: 0.1,
                preparation_score: 0.1,
                mastery_score: 0.1,
            },
            Density::Eighth => DensitySpecificProgression::EighthDensity {
                return_score: 0.1,
                unity_with_creator_score: 0.1,
                transcendence_score: 0.1,
                completion_score: 0.1,
            },
        };

        // Initialize polarization for 3rd Density entities
        let polarization = if matches!(density, Density::Third) {
            Some(EntityPolarizationState::new())
        } else {
            None
        };

        EntityProgressionState {
            entity_id,
            current_density: density,
            progression_score: 0.1,
            density_specific_metrics,
            polarization,
            creation_time,
            last_update_time: creation_time,
        }
    }
}

impl Default for EntityPolarizationState {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityPolarizationState {
    /// Create a new polarization state
    pub fn new() -> Self {
        EntityPolarizationState {
            sto_alignment: 0.0,
            sts_alignment: 0.0,
            polarization_percentage: 0.0,
            catalyst_events: Vec::new(),
            polarity_choices: Vec::new(),
            harvest_ready: false,
            current_polarity: Some(EntityPolarity::Unpolarized),
        }
    }
}

impl Default for ProgressionStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressionStatistics {
    /// Create new progression statistics
    pub fn new() -> Self {
        ProgressionStatistics {
            total_entities: 0,
            entities_by_density: HashMap::new(),
            average_progression_score: 0.0,
            total_polarity_choices: 0,
            sto_count: 0,
            sts_count: 0,
            unpolarized_count: 0,
        }
    }
}
