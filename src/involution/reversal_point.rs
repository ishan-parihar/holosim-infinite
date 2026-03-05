//! Reversal Point System
//!
//! This module detects when involution stops and evolution begins (typically at D1 or D2).
//! It monitors entity progression and determines when they have met the criteria to begin
//! the evolutionary phase of their journey.

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;

/// Reversal criteria thresholds
#[derive(Debug, Clone, PartialEq)]
pub struct ReversalCriteria {
    /// Minimum density that must be reached (D1 or D2)
    pub min_density: Density,
    /// Minimum catalyst that must be processed
    pub min_catalyst: usize,
    /// Minimum polarization intensity that must be established
    pub min_polarization: Float,
    /// Minimum time spent at minimum density (in seconds)
    pub min_time_at_min_density: u64,
    /// Whether free will choice is required
    pub require_free_will_choice: bool,
}

impl ReversalCriteria {
    /// Create initial reversal criteria
    pub fn initial() -> Self {
        Self {
            min_density: Density::Second(Density2SubLevel::Cellular),
            min_catalyst: 10,
            min_polarization: 0.5,
            min_time_at_min_density: 3600, // 1 hour
            require_free_will_choice: true,
        }
    }

    /// Create reversal criteria for D1 reversal (more challenging)
    pub fn for_d1() -> Self {
        Self {
            min_density: Density::First(Density1SubLevel::Quantum),
            min_catalyst: 20,
            min_polarization: 0.7,
            min_time_at_min_density: 7200, // 2 hours
            require_free_will_choice: true,
        }
    }

    /// Create reversal criteria for D2 reversal (standard)
    pub fn for_d2() -> Self {
        Self {
            min_density: Density::Second(Density2SubLevel::Cellular),
            min_catalyst: 10,
            min_polarization: 0.5,
            min_time_at_min_density: 3600, // 1 hour
            require_free_will_choice: true,
        }
    }
}

impl Default for ReversalCriteria {
    fn default() -> Self {
        Self::initial()
    }
}

/// Reversal readiness status
#[derive(Debug, Clone, PartialEq)]
pub enum ReversalReadiness {
    /// Not ready for reversal
    NotReady {
        /// Reason why not ready
        reason: String,
        /// Current progress percentage (0-100)
        progress: Float,
    },
    /// Ready for reversal, waiting for free will choice
    ReadyForChoice {
        /// How long entity has been ready (in seconds)
        ready_since: u64,
    },
    /// Reversal confirmed and evolution phase begun
    ReversalConfirmed {
        /// Timestamp when reversal was confirmed
        confirmed_at: u64,
    },
}

impl ReversalReadiness {
    /// Check if entity is ready for reversal
    pub fn is_ready(&self) -> bool {
        matches!(
            self,
            ReversalReadiness::ReadyForChoice { .. } | ReversalReadiness::ReversalConfirmed { .. }
        )
    }

    /// Check if reversal has been confirmed
    pub fn is_confirmed(&self) -> bool {
        matches!(self, ReversalReadiness::ReversalConfirmed { .. })
    }

    /// Get progress percentage (0-100)
    pub fn progress(&self) -> Float {
        match self {
            ReversalReadiness::NotReady { progress, .. } => *progress,
            ReversalReadiness::ReadyForChoice { .. } => 100.0,
            ReversalReadiness::ReversalConfirmed { .. } => 100.0,
        }
    }
}

/// Reversal event
#[derive(Debug, Clone, PartialEq)]
pub struct ReversalEvent {
    /// Entity ID
    pub entity_id: u64,
    /// Event type
    pub event_type: ReversalEventType,
    /// Timestamp
    pub timestamp: u64,
    /// Density at reversal
    pub density: Density,
    /// Polarity intensity at reversal
    pub polarization_intensity: Float,
    /// Total catalyst processed at reversal
    pub catalyst_processed: usize,
}

/// Reversal event type
#[derive(Debug, Clone, PartialEq)]
pub enum ReversalEventType {
    /// Entity reached minimum density
    ReachedMinDensity,
    /// Entity met catalyst threshold
    MetCatalystThreshold,
    /// Entity met polarization threshold
    MetPolarizationThreshold,
    /// Entity met time threshold
    MetTimeThreshold,
    /// Entity became ready for reversal
    BecameReady,
    /// Entity confirmed reversal (free will choice)
    ConfirmedReversal,
    /// Evolution phase begun
    EvolutionBegun,
}

/// Reversal state for an entity
#[derive(Debug, Clone, PartialEq)]
pub struct ReversalState {
    /// Current readiness status
    pub readiness: ReversalReadiness,
    /// Timestamp when entity reached minimum density
    pub reached_min_density_at: Option<u64>,
    /// Timestamp when entity became ready for reversal
    pub ready_since: Option<u64>,
    /// Timestamp when reversal was confirmed
    pub confirmed_at: Option<u64>,
    /// Whether free will choice has been made
    pub free_will_choice: Option<bool>, // true = choose evolution, false = decline
    /// Events logged for this entity
    pub events: Vec<ReversalEvent>,
}

impl ReversalState {
    /// Create a new reversal state
    pub fn new() -> Self {
        Self {
            readiness: ReversalReadiness::NotReady {
                reason: "Not yet reached minimum density".to_string(),
                progress: 0.0,
            },
            reached_min_density_at: None,
            ready_since: None,
            confirmed_at: None,
            free_will_choice: None,
            events: Vec::new(),
        }
    }

    /// Add an event to the state
    pub fn add_event(&mut self, event: ReversalEvent) {
        self.events.push(event);
    }

    /// Get the latest event
    pub fn latest_event(&self) -> Option<&ReversalEvent> {
        self.events.last()
    }
}

impl Default for ReversalState {
    fn default() -> Self {
        Self::new()
    }
}

/// Reversal Point System
///
/// Detects when involution stops and evolution begins.
/// Monitors entity progression and determines reversal readiness.
#[derive(Debug, Clone)]
pub struct ReversalPointSystem {
    /// Reversal criteria
    pub criteria: ReversalCriteria,
    /// Maps entity IDs to their reversal states
    states: std::collections::HashMap<u64, ReversalState>,
    /// All reversal events logged
    all_events: Vec<ReversalEvent>,
}

impl ReversalPointSystem {
    /// Create a new reversal point system with default criteria
    pub fn new() -> Self {
        Self {
            criteria: ReversalCriteria::initial(),
            states: std::collections::HashMap::new(),
            all_events: Vec::new(),
        }
    }

    /// Create a new reversal point system with custom criteria
    pub fn with_criteria(criteria: ReversalCriteria) -> Self {
        Self {
            criteria,
            states: std::collections::HashMap::new(),
            all_events: Vec::new(),
        }
    }

    /// Detect reversal readiness for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `current_density` - Current density of the entity
    /// * `catalyst_processed` - Total catalyst processed
    /// * `polarization_intensity` - Current polarization intensity
    /// * `time_at_min_density` - Time spent at minimum density (in seconds)
    ///
    /// # Returns
    /// Current reversal readiness status
    pub fn detect_readiness(
        &mut self,
        entity_id: u64,
        current_density: Density,
        catalyst_processed: usize,
        polarization_intensity: Float,
        time_at_min_density: u64,
    ) -> &ReversalReadiness {
        let state = self
            .states
            .entry(entity_id)
            .or_default();
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check if already confirmed
        if state.readiness.is_confirmed() {
            return &state.readiness;
        }

        // Calculate progress
        let mut progress = 0.0;
        let mut reasons = Vec::new();

        // Check minimum density
        if current_density.as_u8() <= self.criteria.min_density.as_u8() {
            progress += 25.0;
            if state.reached_min_density_at.is_none() {
                state.reached_min_density_at = Some(current_time);
                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::ReachedMinDensity,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::ReachedMinDensity,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
            }
        } else {
            reasons.push("Not yet reached minimum density".to_string());
        }

        // Check catalyst threshold
        if catalyst_processed >= self.criteria.min_catalyst {
            progress += 25.0;
            if !state
                .events
                .iter()
                .any(|e| e.event_type == ReversalEventType::MetCatalystThreshold)
            {
                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetCatalystThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetCatalystThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
            }
        } else {
            progress += (catalyst_processed as Float / self.criteria.min_catalyst as Float) * 25.0;
            reasons.push(format!(
                "Catalyst: {}/{}",
                catalyst_processed, self.criteria.min_catalyst
            ));
        }

        // Check polarization threshold
        if polarization_intensity >= self.criteria.min_polarization {
            progress += 25.0;
            if !state
                .events
                .iter()
                .any(|e| e.event_type == ReversalEventType::MetPolarizationThreshold)
            {
                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetPolarizationThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetPolarizationThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
            }
        } else {
            progress += (polarization_intensity / self.criteria.min_polarization) * 25.0;
            reasons.push(format!(
                "Polarity: {:.2}/{:.2}",
                polarization_intensity, self.criteria.min_polarization
            ));
        }

        // Check time threshold
        if time_at_min_density >= self.criteria.min_time_at_min_density {
            progress += 25.0;
            if !state
                .events
                .iter()
                .any(|e| e.event_type == ReversalEventType::MetTimeThreshold)
            {
                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetTimeThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::MetTimeThreshold,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
            }
        } else {
            progress += (time_at_min_density as Float
                / self.criteria.min_time_at_min_density as Float)
                * 25.0;
            reasons.push(format!(
                "Time: {}s/{}s",
                time_at_min_density, self.criteria.min_time_at_min_density
            ));
        }

        // Check if all criteria met
        if progress >= 99.0 {
            // All criteria met
            if !state.readiness.is_ready() {
                state.readiness = ReversalReadiness::ReadyForChoice {
                    ready_since: current_time,
                };
                state.ready_since = Some(current_time);
                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::BecameReady,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::BecameReady,
                    timestamp: current_time,
                    density: current_density,
                    polarization_intensity,
                    catalyst_processed,
                });
            }
        } else {
            state.readiness = ReversalReadiness::NotReady {
                reason: reasons.join("; "),
                progress,
            };
        }

        &state.readiness
    }

    /// Mark the reversal point for an entity (free will choice)
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `choose_evolution` - Whether entity chooses to evolve (true) or decline (false)
    ///
    /// # Returns
    /// True if reversal was confirmed, false otherwise
    pub fn mark_reversal(&mut self, entity_id: u64, choose_evolution: bool) -> bool {
        let state = self.states.get_mut(&entity_id);

        if let Some(state) = state {
            if !state.readiness.is_ready() {
                return false;
            }

            if !self.criteria.require_free_will_choice || choose_evolution {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                state.free_will_choice = Some(choose_evolution);
                state.confirmed_at = Some(current_time);
                state.readiness = ReversalReadiness::ReversalConfirmed {
                    confirmed_at: current_time,
                };

                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::ConfirmedReversal,
                    timestamp: current_time,
                    density: state
                        .latest_event()
                        .map(|e| e.density)
                        .unwrap_or(Density::Second(Density2SubLevel::Cellular)),
                    polarization_intensity: state
                        .latest_event()
                        .map(|e| e.polarization_intensity)
                        .unwrap_or(0.0),
                    catalyst_processed: state
                        .latest_event()
                        .map(|e| e.catalyst_processed)
                        .unwrap_or(0),
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::ConfirmedReversal,
                    timestamp: current_time,
                    density: state
                        .latest_event()
                        .map(|e| e.density)
                        .unwrap_or(Density::Second(Density2SubLevel::Cellular)),
                    polarization_intensity: state
                        .latest_event()
                        .map(|e| e.polarization_intensity)
                        .unwrap_or(0.0),
                    catalyst_processed: state
                        .latest_event()
                        .map(|e| e.catalyst_processed)
                        .unwrap_or(0),
                });

                return true;
            }
        }

        false
    }

    /// Begin the evolution phase for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// True if evolution phase begun, false otherwise
    pub fn begin_evolution(&mut self, entity_id: u64) -> bool {
        let state = self.states.get_mut(&entity_id);

        if let Some(state) = state {
            if !state.readiness.is_confirmed() {
                return false;
            }

            if state.free_will_choice == Some(true) {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                state.add_event(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::EvolutionBegun,
                    timestamp: current_time,
                    density: state
                        .latest_event()
                        .map(|e| e.density)
                        .unwrap_or(Density::Second(Density2SubLevel::Cellular)),
                    polarization_intensity: state
                        .latest_event()
                        .map(|e| e.polarization_intensity)
                        .unwrap_or(0.0),
                    catalyst_processed: state
                        .latest_event()
                        .map(|e| e.catalyst_processed)
                        .unwrap_or(0),
                });
                self.all_events.push(ReversalEvent {
                    entity_id,
                    event_type: ReversalEventType::EvolutionBegun,
                    timestamp: current_time,
                    density: state
                        .latest_event()
                        .map(|e| e.density)
                        .unwrap_or(Density::Second(Density2SubLevel::Cellular)),
                    polarization_intensity: state
                        .latest_event()
                        .map(|e| e.polarization_intensity)
                        .unwrap_or(0.0),
                    catalyst_processed: state
                        .latest_event()
                        .map(|e| e.catalyst_processed)
                        .unwrap_or(0),
                });

                return true;
            }
        }

        false
    }

    /// Get the reversal state for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The reversal state, or None if entity not found
    pub fn get_state(&self, entity_id: u64) -> Option<&ReversalState> {
        self.states.get(&entity_id)
    }

    /// Get all events for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// Vector of events, or None if entity not found
    pub fn get_events(&self, entity_id: u64) -> Option<&[ReversalEvent]> {
        self.states
            .get(&entity_id)
            .map(|state| state.events.as_slice())
    }

    /// Get all reversal events
    pub fn get_all_events(&self) -> &[ReversalEvent] {
        &self.all_events
    }

    /// Remove an entity from the system
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.states.remove(&entity_id);
    }

    /// Clear all states and events
    pub fn clear_all(&mut self) {
        self.states.clear();
        self.all_events.clear();
    }

    /// Get the number of entities being tracked
    pub fn entity_count(&self) -> usize {
        self.states.len()
    }

    /// Get the number of entities ready for reversal
    pub fn ready_count(&self) -> usize {
        self.states
            .values()
            .filter(|s| s.readiness.is_ready())
            .count()
    }

    /// Get the number of entities that have confirmed reversal
    pub fn confirmed_count(&self) -> usize {
        self.states
            .values()
            .filter(|s| s.readiness.is_confirmed())
            .count()
    }
}

impl Default for ReversalPointSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reversal_criteria_default() {
        let criteria = ReversalCriteria::initial();
        assert_eq!(
            criteria.min_density,
            Density::Second(Density2SubLevel::Cellular)
        );
        assert_eq!(criteria.min_catalyst, 10);
        assert_eq!(criteria.min_polarization, 0.5);
        assert_eq!(criteria.min_time_at_min_density, 3600);
        assert!(criteria.require_free_will_choice);
    }

    #[test]
    fn test_reversal_criteria_for_d1() {
        let criteria = ReversalCriteria::for_d1();
        assert_eq!(
            criteria.min_density,
            Density::First(Density1SubLevel::Quantum)
        );
        assert_eq!(criteria.min_catalyst, 20);
        assert_eq!(criteria.min_polarization, 0.7);
        assert_eq!(criteria.min_time_at_min_density, 7200);
    }

    #[test]
    fn test_reversal_readiness_not_ready() {
        let readiness = ReversalReadiness::NotReady {
            reason: "Test".to_string(),
            progress: 50.0,
        };
        assert!(!readiness.is_ready());
        assert!(!readiness.is_confirmed());
        assert_eq!(readiness.progress(), 50.0);
    }

    #[test]
    fn test_reversal_readiness_ready_for_choice() {
        let readiness = ReversalReadiness::ReadyForChoice { ready_since: 12345 };
        assert!(readiness.is_ready());
        assert!(!readiness.is_confirmed());
        assert_eq!(readiness.progress(), 100.0);
    }

    #[test]
    fn test_reversal_readiness_reversal_confirmed() {
        let readiness = ReversalReadiness::ReversalConfirmed {
            confirmed_at: 12345,
        };
        assert!(readiness.is_ready());
        assert!(readiness.is_confirmed());
        assert_eq!(readiness.progress(), 100.0);
    }

    #[test]
    fn test_reversal_state_new() {
        let state = ReversalState::new();
        assert!(!state.readiness.is_ready());
        assert_eq!(state.events.len(), 0);
    }

    #[test]
    fn test_reversal_state_add_event() {
        let mut state = ReversalState::new();
        let event = ReversalEvent {
            entity_id: 123,
            event_type: ReversalEventType::ReachedMinDensity,
            timestamp: 1000,
            density: Density::Second(Density2SubLevel::Cellular),
            polarization_intensity: 0.5,
            catalyst_processed: 10,
        };
        state.add_event(event);
        assert_eq!(state.events.len(), 1);
    }

    #[test]
    fn test_reversal_point_system_new() {
        let system = ReversalPointSystem::new();
        assert_eq!(system.entity_count(), 0);
        assert_eq!(system.ready_count(), 0);
        assert_eq!(system.confirmed_count(), 0);
    }

    #[test]
    fn test_reversal_point_system_with_criteria() {
        let criteria = ReversalCriteria::for_d1();
        let system = ReversalPointSystem::with_criteria(criteria.clone());
        assert_eq!(
            system.criteria.min_density,
            Density::First(Density1SubLevel::Quantum)
        );
    }

    #[test]
    fn test_detect_readiness_not_ready() {
        let mut system = ReversalPointSystem::new();
        let readiness = system.detect_readiness(123, Density::Fifth, 0, 0.0, 0);
        assert!(!readiness.is_ready());
        assert!(readiness.progress() < 100.0);
    }

    #[test]
    fn test_detect_readiness_partially_ready() {
        let mut system = ReversalPointSystem::new();
        let readiness = system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            5,
            0.3,
            1800,
        );
        assert!(!readiness.is_ready());
        assert!(readiness.progress() > 0.0);
        assert!(readiness.progress() < 100.0);
    }

    #[test]
    fn test_detect_readiness_fully_ready() {
        let mut system = ReversalPointSystem::new();
        let readiness = system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        assert!(readiness.is_ready());
        assert_eq!(readiness.progress(), 100.0);
    }

    #[test]
    fn test_mark_reversal_success() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let result = system.mark_reversal(123, true);
        assert!(result);
        assert!(system.get_state(123).unwrap().readiness.is_confirmed());
    }

    #[test]
    fn test_mark_reversal_decline() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let result = system.mark_reversal(123, false);
        assert!(!result);
    }

    #[test]
    fn test_mark_reversal_not_ready() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(123, Density::Fifth, 0, 0.0, 0);
        let result = system.mark_reversal(123, true);
        assert!(!result);
    }

    #[test]
    fn test_begin_evolution_success() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        system.mark_reversal(123, true);
        let result = system.begin_evolution(123);
        assert!(result);
    }

    #[test]
    fn test_begin_evolution_not_confirmed() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let result = system.begin_evolution(123);
        assert!(!result);
    }

    #[test]
    fn test_get_state() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let state = system.get_state(123);
        assert!(state.is_some());
        assert!(state.unwrap().readiness.is_ready());
    }

    #[test]
    fn test_get_events() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let events = system.get_events(123);
        assert!(events.is_some());
        assert!(!events.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_events() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        system.detect_readiness(
            456,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        let events = system.get_all_events();
        assert!(!events.is_empty());
    }

    #[test]
    fn test_remove_entity() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        assert_eq!(system.entity_count(), 1);
        system.remove_entity(123);
        assert_eq!(system.entity_count(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        system.detect_readiness(
            456,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        assert_eq!(system.entity_count(), 2);
        system.clear_all();
        assert_eq!(system.entity_count(), 0);
        assert_eq!(system.get_all_events().len(), 0);
    }

    #[test]
    fn test_ready_count() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        system.detect_readiness(456, Density::Fifth, 0, 0.0, 0);
        assert_eq!(system.ready_count(), 1);
    }

    #[test]
    fn test_confirmed_count() {
        let mut system = ReversalPointSystem::new();
        system.detect_readiness(
            123,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        system.mark_reversal(123, true);
        system.detect_readiness(
            456,
            Density::Second(Density2SubLevel::Cellular),
            10,
            0.5,
            3600,
        );
        assert_eq!(system.confirmed_count(), 1);
    }

    #[test]
    fn test_default() {
        let system = ReversalPointSystem::default();
        assert_eq!(system.entity_count(), 0);
    }
}
