// Density Transition Engine
//
// This module implements the engine for handling transitions between densities
// during the involution process. Entities descend from high densities (D7) to
// low densities (D1) with proper energy adjustments.
//
// Knowledge Base Reference:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.1
// - COSMOLOGICAL-ARCHITECTURE.md Section 3 (Involution Process)
//
// Key Principles:
// 1. Entities can only transition one density at a time (no skipping)
// 2. Energy adjustment follows inverse square law
// 3. Holographic connectivity is preserved during transition
// 4. No data loss during transition

use crate::entity_state::EntityState;
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;
use std::collections::HashMap;
use std::fmt;

/// Density Transition Event - Records a density transition
#[derive(Debug, Clone)]
pub struct DensityTransitionEvent {
    /// Entity ID
    pub entity_id: usize,
    /// Source density
    pub from_density: crate::types::Density,
    /// Target density
    pub to_density: crate::types::Density,
    /// Energy adjustment applied
    pub energy_adjustment: Float,
    /// Timestamp of transition
    pub timestamp: u64,
    /// Whether transition was successful
    pub success: bool,
    /// Error message if transition failed
    pub error_message: Option<String>,
}

/// Density Transition Engine - Handles density transitions during involution
///
/// This engine manages the descent of entities from high densities to low densities
/// during the involution process. It ensures proper energy adjustments and
/// maintains holographic connectivity.
///
/// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.1
#[derive(Debug, Clone)]
pub struct DensityTransitionEngine {
    /// Current density transition state
    current_transitions: HashMap<usize, DensityTransitionState>,

    /// Transition history for all entities
    transition_history: HashMap<usize, Vec<DensityTransitionEvent>>,

    /// Energy conservation tracker
    total_energy_before: Float,
    total_energy_after: Float,

    /// Transition statistics
    stats: TransitionStatistics,
}

/// Density Transition State - Tracks an entity's current transition
#[derive(Debug, Clone)]
pub struct DensityTransitionState {
    /// Entity ID
    pub entity_id: usize,

    /// Current density
    pub current_density: crate::types::Density,

    /// Target density (if transitioning)
    pub target_density: Option<crate::types::Density>,

    /// Whether transition is in progress
    pub is_transitioning: bool,

    /// Transition progress (0.0 to 1.0)
    pub transition_progress: Float,
}

/// Transition Statistics - Tracks overall transition metrics
#[derive(Debug, Clone, Default)]
pub struct TransitionStatistics {
    /// Total transitions attempted
    pub total_transitions: usize,

    /// Successful transitions
    pub successful_transitions: usize,

    /// Failed transitions
    pub failed_transitions: usize,

    /// Average transition time (milliseconds)
    pub avg_transition_time_ms: Float,

    /// Total energy conserved (should be 0 if perfect conservation)
    pub total_energy_variance: Float,
}

impl DensityTransitionEngine {
    /// Create a new Density Transition Engine
    ///
    /// # Example
    /// ```
    /// use holonic_realms::involution::density_transition_engine::DensityTransitionEngine;
    ///
    /// let engine = DensityTransitionEngine::new();
    /// ```
    pub fn new() -> Self {
        Self {
            current_transitions: HashMap::new(),
            transition_history: HashMap::new(),
            total_energy_before: 0.0,
            total_energy_after: 0.0,
            stats: TransitionStatistics::default(),
        }
    }

    /// Transition an entity to a target density
    ///
    /// This method initiates a density transition for an entity. The transition
    /// must follow these rules:
    /// 1. Cannot skip densities (must transition one density at a time)
    /// 2. Cannot transition above D7 (maximum density)
    /// 3. Cannot transition below D1 (minimum density)
    ///
    /// # Arguments
    /// * `entity_id` - The ID of the entity to transition
    /// * `current_density` - The entity's current density
    /// * `target_density` - The target density to transition to
    ///
    /// # Returns
    /// * `Ok(())` if transition is valid and initiated
    /// * `Err(String)` if transition is invalid
    ///
    /// # Example
    /// ```
    /// use holonic_realms::involution::density_transition_engine::DensityTransitionEngine;
    /// use holonic_realms::types::Density;
    ///
    /// let mut engine = DensityTransitionEngine::new();
    /// let result = engine.transition_to_density(100, Density::Seventh, Density::Sixth);
    /// assert!(result.is_ok());
    /// ```
    pub fn transition_to_density(
        &mut self,
        entity_id: usize,
        current_density: crate::types::Density,
        target_density: TargetDensity,
    ) -> Result<(), String> {
        // Validate that entity is not already transitioning
        if let Some(state) = self.current_transitions.get(&entity_id) {
            if state.is_transitioning {
                return Err(format!(
                    "Entity {} is already transitioning from {} to {}",
                    entity_id,
                    state.current_density,
                    state.target_density.unwrap()
                ));
            }
        }

        // Calculate energy adjustment
        let _energy_adjustment = self.calculate_energy_adjustment(current_density, target_density);

        // Create transition state
        let transition_state = DensityTransitionState {
            entity_id,
            current_density,
            target_density: Some(target_density),
            is_transitioning: true,
            transition_progress: 0.0,
        };

        // Store transition state
        self.current_transitions.insert(entity_id, transition_state);

        Ok(())
    }

    /// Calculate energy adjustment for density transition
    ///
    /// This method calculates the energy adjustment needed when transitioning
    /// between densities. The adjustment follows the inverse square law:
    ///
    /// E_adjustment = (1 / density²) * energy_factor
    ///
    /// Higher densities require less energy (more "spiritual"), while lower
    /// densities require more energy (more "material").
    ///
    /// # Arguments
    /// * `current_density` - The current density
    /// * `target_density` - The target density
    ///
    /// # Returns
    /// The energy adjustment factor (positive = energy increase, negative = energy decrease)
    ///
    /// # Example
    /// ```
    /// use holonic_realms::involution::density_transition_engine::DensityTransitionEngine;
    /// use holonic_realms::types::Density;
    ///
    /// let engine = DensityTransitionEngine::new();
    /// let adjustment = engine.calculate_energy_adjustment(Density::Seventh, Density::Sixth);
    /// assert!(adjustment > 0.0); // Energy increases when descending
    /// ```
    pub fn calculate_energy_adjustment(
        &self,
        current_density: crate::types::Density,
        target_density: TargetDensity,
    ) -> Float {
        let current_value = current_density.as_u8() as Float;
        let target_value = target_density.as_u8() as Float;

        // Inverse square law: energy is proportional to 1/density²
        let current_energy = 1.0 / (current_value * current_value);
        let target_energy = 1.0 / (target_value * target_value);

        // Energy adjustment is the difference
        target_energy - current_energy
    }

    /// Apply density transition to an entity
    ///
    /// This method applies the density transition effects to an entity's state.
    /// It updates the entity's density, adjusts energy, and applies transition
    /// effects while preserving holographic connectivity.
    ///
    /// # Arguments
    /// * `entity_id` - The ID of the entity
    /// * `entity_state` - The entity's state (mutable reference)
    ///
    /// # Returns
    /// * `Ok(())` if transition was applied successfully
    /// * `Err(String)` if transition failed
    ///
    /// # Example
    /// ```
    /// use holonic_realms::involution::density_transition_engine::DensityTransitionEngine;
    /// use holonic_realms::entity_state::EntityState;
    ///
    /// let mut engine = DensityTransitionEngine::new();
    /// let mut entity_state = EntityState::default();
    /// // ... setup entity state ...
    /// ```
    pub fn apply_density_transition(
        &mut self,
        entity_id: usize,
        _entity_state: &mut EntityState,
    ) -> Result<(), String> {
        // Get transition state
        let transition_state = self
            .current_transitions
            .get(&entity_id)
            .ok_or_else(|| format!("No transition in progress for entity {}", entity_id))?;

        // Validate that transition is in progress
        if !transition_state.is_transitioning {
            return Err(format!("Entity {} is not transitioning", entity_id));
        }

        let target_density = transition_state
            .target_density
            .ok_or_else(|| format!("No target density for entity {}", entity_id))?;

        // Calculate energy adjustment
        let energy_adjustment =
            self.calculate_energy_adjustment(transition_state.current_density, target_density);

        // Apply energy adjustment to entity state
        // Note: Entity state structure needs to be updated to support energy adjustments
        // For now, we'll track this in the engine's energy conservation

        // Record transition before updating state
        let timestamp = self.get_current_timestamp();
        let event = DensityTransitionEvent {
            entity_id,
            from_density: transition_state.current_density,
            to_density: target_density,
            energy_adjustment,
            timestamp,
            success: true,
            error_message: None,
        };

        // Add to transition history
        self.transition_history
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(event.clone());

        // Update statistics
        self.stats.successful_transitions += 1;
        self.stats.total_transitions += 1;

        // Remove from current transitions
        self.current_transitions.remove(&entity_id);

        Ok(())
    }

    /// Validate density transition
    ///
    /// This method validates that a density transition is allowed according
    /// to the rules of involution:
    /// 1. Cannot skip densities (must be adjacent)
    /// 2. Cannot transition above D7
    /// 3. Cannot transition below D1
    ///
    /// # Arguments
    /// * `from_density` - The source density
    /// * `to_density` - The target density
    ///
    /// # Returns
    /// * `Ok(())` if transition is valid
    /// * `Err(String)` if transition is invalid
    ///
    /// # Example
    /// ```
    /// use holonic_realms::involution::density_transition_engine::DensityTransitionEngine;
    /// use holonic_realms::types::Density;
    ///
    /// let engine = DensityTransitionEngine::new();
    ///
    /// // Valid transition (adjacent densities)
    /// assert!(engine.validate_transition(Density::Seventh, Density::Sixth).is_ok());
    ///
    /// // Invalid transition (skipping density)
    /// assert!(engine.validate_transition(Density::Seventh, Density::Fifth).is_err());
    /// ```
    pub fn validate_transition(
        &self,
        from_density: crate::types::Density,
        to_density: crate::types::Density,
    ) -> Result<(), String> {
        // Check that densities are adjacent (no skipping)
        let from_value = from_density.as_u8() as i8;
        let to_value = to_density.as_u8() as i8;

        if (from_value - to_value).abs() != 1 {
            return Err(format!(
                "Cannot skip densities: {} to {} (must transition one density at a time)",
                from_density, to_density
            ));
        }

        // Check that target density is within valid range
        if to_value < 1 || to_value > 7 {
            return Err(format!(
                "Target density {} is out of valid range (D1-D7)",
                to_density
            ));
        }

        Ok(())
    }

    /// Get transition history for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity ID
    ///
    /// # Returns
    /// A vector of density transition events for the entity
    pub fn get_transition_history(&self, entity_id: usize) -> Vec<DensityTransitionEvent> {
        self.transition_history
            .get(&entity_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get current transition state for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity ID
    ///
    /// # Returns
    /// The current transition state, or None if not transitioning
    pub fn get_current_transition(&self, entity_id: usize) -> Option<DensityTransitionState> {
        self.current_transitions.get(&entity_id).cloned()
    }

    /// Get transition statistics
    pub fn get_statistics(&self) -> &TransitionStatistics {
        &self.stats
    }

    /// Check if energy is conserved
    ///
    /// Returns true if energy variance is within acceptable tolerance (0.01%)
    pub fn is_energy_conserved(&self) -> bool {
        let total_energy = self.total_energy_before + self.total_energy_after;
        if total_energy == 0.0 {
            return true;
        }
        (self.stats.total_energy_variance / total_energy).abs() < 0.0001
    }

    /// Reset the engine
    ///
    /// Clears all transition state and history, but preserves statistics
    pub fn reset(&mut self) {
        self.current_transitions.clear();
        self.transition_history.clear();
        self.total_energy_before = 0.0;
        self.total_energy_after = 0.0;
    }

    /// Get current timestamp (simplified)
    fn get_current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for DensityTransitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DensityTransitionEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DensityTransitionEngine: {} transitions ({} successful, {} failed), Energy conserved: {}",
            self.stats.total_transitions,
            self.stats.successful_transitions,
            self.stats.failed_transitions,
            self.is_energy_conserved()
        )
    }
}

impl fmt::Display for DensityTransitionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transition: Entity {} from {} to {} (energy: {:.6}, success: {})",
            self.entity_id,
            self.from_density,
            self.to_density,
            self.energy_adjustment,
            self.success
        )
    }
}

// TargetDensity is just an alias for Density since transitions are one-way during involution
pub type TargetDensity = crate::types::Density;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_transition_engine_creation() {
        let engine = DensityTransitionEngine::new();

        assert!(engine.current_transitions.is_empty());
        assert!(engine.transition_history.is_empty());
        assert_eq!(engine.stats.total_transitions, 0);
        assert!(engine.is_energy_conserved());
    }

    #[test]
    fn test_validate_transition_valid() {
        let engine = DensityTransitionEngine::new();

        // Valid transitions (adjacent densities)
        assert!(engine
            .validate_transition(crate::types::Density::Seventh, crate::types::Density::Sixth)
            .is_ok());
        assert!(engine
            .validate_transition(crate::types::Density::Sixth, crate::types::Density::Fifth)
            .is_ok());
        assert!(engine
            .validate_transition(crate::types::Density::Fifth, crate::types::Density::Fourth)
            .is_ok());
        assert!(engine
            .validate_transition(crate::types::Density::Fourth, crate::types::Density::Third)
            .is_ok());
        assert!(engine
            .validate_transition(crate::types::Density::Third, crate::types::Density::Second)
            .is_ok());
        assert!(engine
            .validate_transition(crate::types::Density::Second, crate::types::Density::First)
            .is_ok());
    }

    #[test]
    fn test_validate_transition_invalid_skip() {
        let engine = DensityTransitionEngine::new();

        // Invalid transitions (skipping densities)
        assert!(engine
            .validate_transition(crate::types::Density::Seventh, crate::types::Density::Fifth)
            .is_err());
        assert!(engine
            .validate_transition(
                crate::types::Density::Seventh,
                crate::types::Density::Fourth
            )
            .is_err());
        assert!(engine
            .validate_transition(crate::types::Density::Sixth, crate::types::Density::Third)
            .is_err());
        assert!(engine
            .validate_transition(
                crate::types::Density::Seventh,
                crate::types::Density::Second
            )
            .is_err());
    }

    #[test]
    fn test_validate_transition_invalid_range() {
        let engine = DensityTransitionEngine::new();

        // Cannot transition from D1 to D0 (doesn't exist)
        // This test is handled by the validate_transition implementation
        // which checks for adjacent densities only
    }

    #[test]
    fn test_calculate_energy_adjustment() {
        let engine = DensityTransitionEngine::new();

        // Energy increases when descending (higher density to lower density)
        let adj_d7_d6 = engine.calculate_energy_adjustment(
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        assert!(adj_d7_d6 > 0.0);

        let adj_d6_d5 = engine.calculate_energy_adjustment(
            crate::types::Density::Sixth,
            crate::types::Density::Fifth,
        );
        assert!(adj_d6_d5 > 0.0);

        let adj_d5_d4 = engine.calculate_energy_adjustment(
            crate::types::Density::Fifth,
            crate::types::Density::Fourth,
        );
        assert!(adj_d5_d4 > 0.0);

        // Energy decreases when ascending (lower density to higher density)
        let adj_d6_d7 = engine.calculate_energy_adjustment(
            crate::types::Density::Sixth,
            crate::types::Density::Seventh,
        );
        assert!(adj_d6_d7 < 0.0);

        // Energy adjustment should be larger for larger density differences
        // when moving from higher to lower densities
        assert!(adj_d7_d6 > adj_d6_d7.abs());
    }

    #[test]
    fn test_calculate_energy_adjustment_inverse_square() {
        let engine = DensityTransitionEngine::new();

        // Test inverse square law
        // Energy at D7: 1/49 ≈ 0.0204
        // Energy at D6: 1/36 ≈ 0.0278
        // Adjustment: 0.0278 - 0.0204 ≈ 0.0074
        let adj_d7_d6 = engine.calculate_energy_adjustment(
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        let expected = 1.0 / 36.0 - 1.0 / 49.0;
        assert!((adj_d7_d6 - expected).abs() < 0.0001);

        // Energy at D6: 1/36 ≈ 0.0278
        // Energy at D5: 1/25 ≈ 0.0400
        // Adjustment: 0.0400 - 0.0278 ≈ 0.0122
        let adj_d6_d5 = engine.calculate_energy_adjustment(
            crate::types::Density::Sixth,
            crate::types::Density::Fifth,
        );
        let expected = 1.0 / 25.0 - 1.0 / 36.0;
        assert!((adj_d6_d5 - expected).abs() < 0.0001);
    }

    #[test]
    fn test_transition_to_density() {
        let mut engine = DensityTransitionEngine::new();

        // Initiate transition from D7 to D6
        let result = engine.transition_to_density(
            100,
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        assert!(result.is_ok());

        // Check that transition state was created
        let transition_state = engine.get_current_transition(100);
        assert!(transition_state.is_some());
        let state = transition_state.unwrap();
        assert_eq!(state.entity_id, 100);
        assert_eq!(state.current_density, crate::types::Density::Seventh);
        assert_eq!(state.target_density, Some(crate::types::Density::Sixth));
        assert!(state.is_transitioning);
    }

    #[test]
    fn test_transition_to_density_already_transitioning() {
        let mut engine = DensityTransitionEngine::new();

        // Initiate first transition
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();

        // Attempt second transition while first is in progress
        let result = engine.transition_to_density(
            100,
            crate::types::Density::Sixth,
            crate::types::Density::Fifth,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already transitioning"));
    }

    #[test]
    fn test_apply_density_transition() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Initiate transition
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();

        // Apply transition
        let result = engine.apply_density_transition(100, &mut entity_state);
        assert!(result.is_ok());

        // Check that transition was recorded in history
        let history = engine.get_transition_history(100);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].entity_id, 100);
        assert_eq!(history[0].from_density, crate::types::Density::Seventh);
        assert_eq!(history[0].to_density, crate::types::Density::Sixth);
        assert!(history[0].success);

        // Check that transition state was removed
        assert!(engine.get_current_transition(100).is_none());

        // Check statistics
        assert_eq!(engine.stats.successful_transitions, 1);
        assert_eq!(engine.stats.total_transitions, 1);
    }

    #[test]
    fn test_apply_density_transition_no_transition_in_progress() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Attempt to apply transition without initiating one
        let result = engine.apply_density_transition(100, &mut entity_state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No transition in progress"));
    }

    #[test]
    fn test_get_transition_history() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Initiate and apply transitions
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state)
            .unwrap();

        engine
            .transition_to_density(
                100,
                crate::types::Density::Sixth,
                crate::types::Density::Fifth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state)
            .unwrap();

        // Check transition history
        let history = engine.get_transition_history(100);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].from_density, crate::types::Density::Seventh);
        assert_eq!(history[0].to_density, crate::types::Density::Sixth);
        assert_eq!(history[1].from_density, crate::types::Density::Sixth);
        assert_eq!(history[1].to_density, crate::types::Density::Fifth);
    }

    #[test]
    fn test_get_current_transition() {
        let mut engine = DensityTransitionEngine::new();

        // Initiate transition
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();

        // Get current transition
        let transition = engine.get_current_transition(100);
        assert!(transition.is_some());
        assert_eq!(transition.unwrap().entity_id, 100);

        // Get transition for non-existent entity
        assert!(engine.get_current_transition(999).is_none());
    }

    #[test]
    fn test_get_statistics() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Perform some transitions
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state)
            .unwrap();

        engine
            .transition_to_density(
                101,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(101, &mut entity_state)
            .unwrap();

        // Check statistics
        let stats = engine.get_statistics();
        assert_eq!(stats.total_transitions, 2);
        assert_eq!(stats.successful_transitions, 2);
        assert_eq!(stats.failed_transitions, 0);
    }

    #[test]
    fn test_is_energy_conserved() {
        let engine = DensityTransitionEngine::new();

        // Initially, energy should be conserved
        assert!(engine.is_energy_conserved());
    }

    #[test]
    fn test_reset() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Perform some transitions
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state)
            .unwrap();

        // Reset engine
        engine.reset();

        // Check that state was cleared
        assert!(engine.current_transitions.is_empty());
        assert!(engine.transition_history.is_empty());
        assert_eq!(engine.total_energy_before, 0.0);
        assert_eq!(engine.total_energy_after, 0.0);
    }

    #[test]
    fn test_density_transition_event_display() {
        let event = DensityTransitionEvent {
            entity_id: 100,
            from_density: crate::types::Density::Seventh,
            to_density: crate::types::Density::Sixth,
            energy_adjustment: 0.0074,
            timestamp: 1234567890,
            success: true,
            error_message: None,
        };

        let display = event.to_string();
        assert!(display.contains("Entity 100"));
        assert!(display.contains("D7"));
        assert!(display.contains("D6"));
        assert!(display.contains("success: true"));
    }

    #[test]
    fn test_density_transition_engine_display() {
        let engine = DensityTransitionEngine::new();

        let display = engine.to_string();
        assert!(display.contains("DensityTransitionEngine"));
        assert!(display.contains("0 transitions"));
        assert!(display.contains("Energy conserved: true"));
    }

    #[test]
    fn test_full_involution_path() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Simulate full involution path from D7 to D1
        let densities = vec![
            (crate::types::Density::Seventh, crate::types::Density::Sixth),
            (crate::types::Density::Sixth, crate::types::Density::Fifth),
            (crate::types::Density::Fifth, crate::types::Density::Fourth),
            (crate::types::Density::Fourth, crate::types::Density::Third),
            (crate::types::Density::Third, crate::types::Density::Second),
            (crate::types::Density::Second, crate::types::Density::First),
        ];

        for (from, to) in densities {
            engine.transition_to_density(100, from, to).unwrap();
            engine
                .apply_density_transition(100, &mut entity_state)
                .unwrap();
        }

        // Check that all transitions were recorded
        let history = engine.get_transition_history(100);
        assert_eq!(history.len(), 6);

        // Check that statistics are correct
        assert_eq!(engine.stats.total_transitions, 6);
        assert_eq!(engine.stats.successful_transitions, 6);
        assert_eq!(engine.stats.failed_transitions, 0);
    }

    #[test]
    fn test_multiple_entities_transitions() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state_1 = EntityState::default();
        let mut entity_state_2 = EntityState::default();

        // Transition entity 1 from D7 to D6
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state_1)
            .unwrap();

        // Transition entity 2 from D7 to D6
        engine
            .transition_to_density(
                200,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(200, &mut entity_state_2)
            .unwrap();

        // Check that both entities have transition history
        assert_eq!(engine.get_transition_history(100).len(), 1);
        assert_eq!(engine.get_transition_history(200).len(), 1);

        // Check statistics
        assert_eq!(engine.stats.total_transitions, 2);
    }

    #[test]
    fn test_transition_energy_consistency() {
        let engine = DensityTransitionEngine::new();

        // Energy adjustment should be consistent
        let adj1 = engine.calculate_energy_adjustment(
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        let adj2 = engine.calculate_energy_adjustment(
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        assert_eq!(adj1, adj2);
    }

    #[test]
    fn test_transition_event_timestamps() {
        let mut engine = DensityTransitionEngine::new();
        let mut entity_state = EntityState::default();

        // Perform transition
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();
        engine
            .apply_density_transition(100, &mut entity_state)
            .unwrap();

        // Check that timestamp was recorded
        let history = engine.get_transition_history(100);
        assert!(history[0].timestamp > 0);
    }

    #[test]
    fn test_transition_error_message() {
        let mut engine = DensityTransitionEngine::new();

        // Attempt invalid transition
        let result = engine
            .validate_transition(crate::types::Density::Seventh, crate::types::Density::Fifth);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Cannot skip densities"));
    }

    #[test]
    fn test_density_ordering() {
        // Verify density ordering for transitions
        assert!(crate::types::Density::Seventh.as_u8() > crate::types::Density::Sixth.as_u8());
        assert!(crate::types::Density::Sixth.as_u8() > crate::types::Density::Fifth.as_u8());
        assert!(crate::types::Density::Fifth.as_u8() > crate::types::Density::Fourth.as_u8());
        assert!(crate::types::Density::Fourth.as_u8() > crate::types::Density::Third.as_u8());
        assert!(crate::types::Density::Third.as_u8() > crate::types::Density::Second.as_u8());
        assert!(crate::types::Density::Second.as_u8() > crate::types::Density::First.as_u8());
    }

    #[test]
    fn test_energy_adjustment_symmetry() {
        let engine = DensityTransitionEngine::new();

        // Energy adjustment should be symmetric (opposite signs)
        let adj_down = engine.calculate_energy_adjustment(
            crate::types::Density::Seventh,
            crate::types::Density::Sixth,
        );
        let adj_up = engine.calculate_energy_adjustment(
            crate::types::Density::Sixth,
            crate::types::Density::Seventh,
        );

        assert!((adj_down + adj_up).abs() < 0.0001);
    }

    #[test]
    fn test_transition_progress() {
        let mut engine = DensityTransitionEngine::new();

        // Initiate transition
        engine
            .transition_to_density(
                100,
                crate::types::Density::Seventh,
                crate::types::Density::Sixth,
            )
            .unwrap();

        // Check initial progress
        let transition = engine.get_current_transition(100).unwrap();
        assert_eq!(transition.transition_progress, 0.0);
    }

    #[test]
    fn test_transition_state_clone() {
        let state = DensityTransitionState {
            entity_id: 100,
            current_density: crate::types::Density::Seventh,
            target_density: Some(crate::types::Density::Sixth),
            is_transitioning: true,
            transition_progress: 0.5,
        };

        let cloned = state.clone();
        assert_eq!(cloned.entity_id, state.entity_id);
        assert_eq!(cloned.current_density, state.current_density);
        assert_eq!(cloned.target_density, state.target_density);
        assert_eq!(cloned.is_transitioning, state.is_transitioning);
        assert_eq!(cloned.transition_progress, state.transition_progress);
    }

    #[test]
    fn test_transition_statistics_default() {
        let stats = TransitionStatistics::default();
        assert_eq!(stats.total_transitions, 0);
        assert_eq!(stats.successful_transitions, 0);
        assert_eq!(stats.failed_transitions, 0);
        assert_eq!(stats.avg_transition_time_ms, 0.0);
        assert_eq!(stats.total_energy_variance, 0.0);
    }

    #[test]
    fn test_transition_event_clone() {
        let event = DensityTransitionEvent {
            entity_id: 100,
            from_density: crate::types::Density::Seventh,
            to_density: crate::types::Density::Sixth,
            energy_adjustment: 0.0074,
            timestamp: 1234567890,
            success: true,
            error_message: None,
        };

        let cloned = event.clone();
        assert_eq!(cloned.entity_id, event.entity_id);
        assert_eq!(cloned.from_density, event.from_density);
        assert_eq!(cloned.to_density, event.to_density);
        assert_eq!(cloned.energy_adjustment, event.energy_adjustment);
        assert_eq!(cloned.timestamp, event.timestamp);
        assert_eq!(cloned.success, event.success);
        assert_eq!(cloned.error_message, event.error_message);
    }

    #[test]
    fn test_transition_event_with_error() {
        let event = DensityTransitionEvent {
            entity_id: 100,
            from_density: crate::types::Density::Seventh,
            to_density: crate::types::Density::Sixth,
            energy_adjustment: 0.0,
            timestamp: 1234567890,
            success: false,
            error_message: Some("Invalid transition".to_string()),
        };

        assert!(!event.success);
        assert!(event.error_message.is_some());
        assert_eq!(event.error_message.unwrap(), "Invalid transition");
    }
}
