//! Density Transition System - Reincarnation mechanism with holographic continuity
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 6: Density-Based Gaming:
//! "Density progression is evolutionary, not level-based. Reincarnation maintains
//! holographic continuity."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.6: The Universal Constant: Transcend and Include:
//! "Each step INCLUDES all previous development and TRANSCENDS by adding new development."
//! "Transcend and Include is the mechanism by which the holographic principle operates."
//!
//! This module provides:
//! - DensityTransitionSystem: Main reincarnation mechanism
//! - HolographicContinuity: Preservation of all previous development
//! - Transition animations with interpolation
//! - Veil thickness variations by density
//! - Spectrum access variations by density
//! - Catalyst accumulation and reset (preserve polarity)
//! - Memory of previous density experiences

use crate::simulation_v3::archetype_basis::ArchetypeActivationProfile;
use crate::simulation_v3::density_mechanics::{
    ActionSpace, Catalyst, Density, DensityMechanics, InteractionRules, PerceptionFilter, Polarity,
};
use crate::types::Float;
use std::collections::HashMap;

/// Default transition duration (in seconds)
pub const DEFAULT_TRANSITION_DURATION: Float = 5.0;

/// Minimum transition duration
pub const MIN_TRANSITION_DURATION: Float = 1.0;

/// Maximum transition duration
pub const MAX_TRANSITION_DURATION: Float = 30.0;

/// Maximum number of density memories to store
pub const MAX_DENSITY_MEMORIES: usize = 8;

/// Density transition state
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionState {
    /// Entity is stable in current density
    Stable,
    /// Entity is transitioning between densities
    Transitioning {
        from_density: Density,
        to_density: Density,
        progress: Float,
        interpolation_mode: TransitionInterpolationMode,
        start_time: Float,
        duration: Float,
    },
}

impl DensityTransitionState {
    /// Check if entity is currently transitioning
    pub fn is_transitioning(&self) -> bool {
        matches!(self, DensityTransitionState::Transitioning { .. })
    }

    /// Get current density (if transitioning, returns from_density)
    pub fn current_density(&self) -> Option<Density> {
        match self {
            DensityTransitionState::Stable => None,
            DensityTransitionState::Transitioning { from_density, .. } => Some(*from_density),
        }
    }

    /// Get target density (if transitioning)
    pub fn target_density(&self) -> Option<Density> {
        match self {
            DensityTransitionState::Stable => None,
            DensityTransitionState::Transitioning { to_density, .. } => Some(*to_density),
        }
    }
}

/// Transition interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionInterpolationMode {
    /// Linear interpolation
    Linear,
    /// Ease-in-out interpolation
    EaseInOut,
    /// Holographic interpolation (non-linear based on spectrum ratios)
    Holographic,
}

impl TransitionInterpolationMode {
    /// Apply interpolation function to a value
    pub fn apply(&self, t: Float) -> Float {
        let t = t.clamp(0.0, 1.0);
        match self {
            TransitionInterpolationMode::Linear => t,
            TransitionInterpolationMode::EaseInOut => {
                // Smoothstep: 3t^2 - 2t^3
                t * t * (3.0 - 2.0 * t)
            }
            TransitionInterpolationMode::Holographic => {
                // Holographic: uses sine function for smooth, non-linear transition
                // Represents the wave-like nature of holographic interference
                (1.0 - (t * std::f64::consts::PI).cos()) / 2.0
            }
        }
    }
}

/// Density memory entry - stores experience from a density
#[derive(Debug, Clone, PartialEq)]
pub struct DensityMemoryEntry {
    /// Density level
    pub density: Density,
    /// Time spent in this density (in simulation steps)
    pub time_spent: u64,
    /// Catalyst accumulated during this density
    pub catalyst_accumulated: Float,
    /// Polarity achieved during this density
    pub polarity: Option<Polarity>,
    /// Polarization strength achieved (0.0 to 1.0)
    pub polarization_strength: Float,
    /// Key lessons learned (archetypical patterns integrated)
    pub lessons_learned: Vec<String>,
    /// Timestamp when density was entered
    pub entry_timestamp: Float,
    /// Timestamp when density was exited
    pub exit_timestamp: Option<Float>,
}

impl DensityMemoryEntry {
    /// Create new density memory entry
    pub fn new(density: Density, timestamp: Float) -> Self {
        DensityMemoryEntry {
            density,
            time_spent: 0,
            catalyst_accumulated: 0.0,
            polarity: None,
            polarization_strength: 0.0,
            lessons_learned: Vec::new(),
            entry_timestamp: timestamp,
            exit_timestamp: None,
        }
    }

    /// Add time spent in this density
    pub fn add_time(&mut self, steps: u64) {
        self.time_spent += steps;
    }

    /// Add catalyst accumulated
    pub fn add_catalyst(&mut self, catalyst: Float) {
        self.catalyst_accumulated = (self.catalyst_accumulated + catalyst).clamp(0.0, 1.0);
    }

    /// Set polarity
    pub fn set_polarity(&mut self, polarity: Polarity, strength: Float) {
        self.polarity = Some(polarity);
        self.polarization_strength = strength.clamp(0.0, 1.0);
    }

    /// Add lesson learned
    pub fn add_lesson(&mut self, lesson: String) {
        self.lessons_learned.push(lesson);
    }

    /// Mark density as exited
    pub fn exit(&mut self, timestamp: Float) {
        self.exit_timestamp = Some(timestamp);
    }

    /// Check if memory entry is complete (density exited)
    pub fn is_complete(&self) -> bool {
        self.exit_timestamp.is_some()
    }
}

/// Holographic continuity - preservation of all previous development
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Transcend and Include is the mechanism by which the holographic principle operates.
/// Each stage INCLUDES all previous development and TRANSCENDS by adding new development."
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicContinuity {
    /// Archetype activation profile (persists across densities)
    pub archetype_activation: ArchetypeActivationProfile,
    /// Polarity choice (persists once made)
    pub polarity: Option<Polarity>,
    /// Polarization strength (accumulates across densities)
    pub polarization_strength: Float,
    /// Catalyst accumulated in current density (resets on transition)
    pub current_catalyst: Float,
    /// Total catalyst across all densities (accumulates)
    pub total_catalyst: Float,
    /// Density memories (experiences from previous densities)
    pub density_memories: Vec<DensityMemoryEntry>,
    /// Current density level
    pub current_density: Density,
    /// Number of density transitions completed
    pub transitions_completed: u32,
    /// First density entered
    pub first_density: Density,
}

impl HolographicContinuity {
    /// Create new holographic continuity
    pub fn new(density: Density, archetype_activation: ArchetypeActivationProfile) -> Self {
        let mut density_memories = Vec::new();
        density_memories.push(DensityMemoryEntry::new(density, 0.0));

        HolographicContinuity {
            archetype_activation,
            polarity: None,
            polarization_strength: 0.0,
            current_catalyst: 0.0,
            total_catalyst: 0.0,
            density_memories,
            current_density: density,
            transitions_completed: 0,
            first_density: density,
        }
    }

    /// Update archetype activation
    pub fn update_archetype_activation(&mut self, profile: ArchetypeActivationProfile) {
        self.archetype_activation = profile;
    }

    /// Set polarity (can only be set once)
    pub fn set_polarity(&mut self, polarity: Polarity) -> Result<(), DensityTransitionError> {
        if self.polarity.is_some() && self.polarity != Some(polarity) {
            return Err(DensityTransitionError::PolarityAlreadySet);
        }
        self.polarity = Some(polarity);
        Ok(())
    }

    /// Add catalyst (current and total)
    pub fn add_catalyst(&mut self, catalyst: Float) {
        let catalyst = catalyst.clamp(0.0, 1.0);
        self.current_catalyst = (self.current_catalyst + catalyst).clamp(0.0, 1.0);
        self.total_catalyst = (self.total_catalyst + catalyst).clamp(0.0, 1.0);

        // Also add to current density memory
        if let Some(current_memory) = self.density_memories.last_mut() {
            current_memory.add_catalyst(catalyst);
        }
    }

    /// Update polarization strength
    pub fn update_polarization_strength(&mut self, strength: Float) {
        self.polarization_strength = strength.clamp(0.0, 1.0);

        if let Some(current_memory) = self.density_memories.last_mut() {
            if let Some(polarity) = self.polarity {
                current_memory.set_polarity(polarity, strength);
            }
        }
    }

    /// Add lesson learned
    pub fn add_lesson(&mut self, lesson: String) {
        if let Some(current_memory) = self.density_memories.last_mut() {
            current_memory.add_lesson(lesson);
        }
    }

    /// Get memory for a specific density
    pub fn get_density_memory(&self, density: Density) -> Option<&DensityMemoryEntry> {
        self.density_memories.iter().find(|m| m.density == density)
    }

    /// Check if entity has experienced a density
    pub fn has_experienced_density(&self, density: Density) -> bool {
        self.density_memories.iter().any(|m| m.density == density)
    }

    /// Prepare for density transition
    pub fn prepare_for_transition(&mut self, current_time: Float) {
        if let Some(current_memory) = self.density_memories.last_mut() {
            current_memory.exit(current_time);
        }
    }

    /// Complete density transition
    pub fn complete_transition(&mut self, new_density: Density, current_time: Float) {
        self.current_density = new_density;
        self.current_catalyst = 0.0;
        self.transitions_completed += 1;

        // Create new memory entry for new density
        self.density_memories
            .push(DensityMemoryEntry::new(new_density, current_time));

        // Limit memory entries
        if self.density_memories.len() > MAX_DENSITY_MEMORIES {
            self.density_memories.remove(0);
        }
    }
}

/// Density transition update result
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionUpdateResult {
    /// Transition is in progress
    InProgress {
        progress: Float,
        interpolated_veil: Float,
        interpolated_spectrum_access: Float,
    },
    /// Transition is complete
    Complete { new_density: Density },
    /// No transition in progress
    Stable,
}

/// Density transition system
///
/// From GAMING_ENGINE_ROADMAP_v2.md:
/// "Density transition mechanics (reincarnation). Reincarnation maintains holographic continuity."
#[derive(Debug, Clone, PartialEq)]
pub struct DensityTransitionSystem {
    /// Current density
    pub current_density: Density,
    /// Transition state
    pub transition_state: DensityTransitionState,
    /// Holographic continuity (preserves all previous development)
    pub holographic_continuity: HolographicContinuity,
    /// Current density mechanics
    pub current_mechanics: DensityMechanics,
    /// Target density mechanics (during transition)
    pub target_mechanics: Option<DensityMechanics>,
    /// Current simulation time
    pub current_time: Float,
}

impl DensityTransitionSystem {
    /// Create new density transition system
    pub fn new(density: Density, archetype_activation: ArchetypeActivationProfile) -> Self {
        let holographic_continuity = HolographicContinuity::new(density, archetype_activation);
        let current_mechanics = DensityMechanics::from_density(density);

        DensityTransitionSystem {
            current_density: density,
            transition_state: DensityTransitionState::Stable,
            holographic_continuity,
            current_mechanics,
            target_mechanics: None,
            current_time: 0.0,
        }
    }

    /// Get current perception filter (interpolated during transition)
    pub fn get_perception_filter(&self) -> PerceptionFilter {
        match &self.transition_state {
            DensityTransitionState::Stable => self.current_mechanics.perception_filter(),
            DensityTransitionState::Transitioning {
                to_density,
                progress,
                interpolation_mode,
                ..
            } => {
                let t = interpolation_mode.apply(*progress);
                let from_filter = self.current_mechanics.perception_filter();
                let to_filter = if let Some(target) = &self.target_mechanics {
                    target.perception_filter()
                } else {
                    DensityMechanics::from_density(*to_density).perception_filter()
                };

                // Interpolate perception filter
                PerceptionFilter {
                    veil_transparency: Self::interpolate_float(
                        from_filter.veil_transparency,
                        to_filter.veil_transparency,
                        t,
                    ),
                    spectrum_access: Self::interpolate_float(
                        from_filter.spectrum_access,
                        to_filter.spectrum_access,
                        t,
                    ),
                    physical_dominant: t < 0.5,  // Switch at midpoint
                    telepathy_enabled: t >= 0.5, // Enable at midpoint
                    collective_awareness: Self::interpolate_float(
                        from_filter.collective_awareness,
                        to_filter.collective_awareness,
                        t,
                    ),
                    time_space_access: Self::interpolate_float(
                        from_filter.time_space_access,
                        to_filter.time_space_access,
                        t,
                    ),
                }
            }
        }
    }

    /// Get current action space (interpolated during transition)
    pub fn get_action_space(&self) -> ActionSpace {
        match &self.transition_state {
            DensityTransitionState::Stable => self.current_mechanics.action_space(),
            DensityTransitionState::Transitioning {
                progress,
                interpolation_mode,
                ..
            } => {
                let t = interpolation_mode.apply(*progress);
                let from_space = self.current_mechanics.action_space();
                let to_space = if let Some(target) = &self.target_mechanics {
                    target.action_space()
                } else {
                    self.current_mechanics.action_space()
                };

                // During transition, blend action spaces based on interpolation
                // At t < 0.5, use more from_space actions; at t >= 0.5, use more to_space actions
                let physical_actions = if t < 0.5 {
                    from_space.physical_actions.clone()
                } else {
                    to_space.physical_actions.clone()
                };

                let mental_actions = if t < 0.5 {
                    from_space.mental_actions.clone()
                } else {
                    to_space.mental_actions.clone()
                };

                let spiritual_actions = if t < 0.5 {
                    from_space.spiritual_actions.clone()
                } else {
                    to_space.spiritual_actions.clone()
                };

                let collective_actions = if t < 0.5 {
                    from_space.collective_actions.clone()
                } else {
                    to_space.collective_actions.clone()
                };

                ActionSpace {
                    physical_actions,
                    mental_actions,
                    spiritual_actions,
                    collective_actions,
                }
            }
        }
    }

    /// Get current interaction rules (interpolated during transition)
    pub fn get_interaction_rules(&self) -> InteractionRules {
        match &self.transition_state {
            DensityTransitionState::Stable => self.current_mechanics.interaction_rules(),
            DensityTransitionState::Transitioning {
                progress,
                interpolation_mode,
                ..
            } => {
                let t = interpolation_mode.apply(*progress);
                let from_rules = self.current_mechanics.interaction_rules();
                let to_rules = if let Some(target) = &self.target_mechanics {
                    target.interaction_rules()
                } else {
                    self.current_mechanics.interaction_rules()
                };

                InteractionRules {
                    can_harm: if t < 0.5 {
                        from_rules.can_harm
                    } else {
                        to_rules.can_harm
                    },
                    can_heal: from_rules.can_heal || to_rules.can_heal,
                    can_communicate: from_rules.can_communicate || to_rules.can_communicate,
                    telepathic_communication: if t >= 0.5 {
                        to_rules.telepathic_communication
                    } else {
                        from_rules.telepathic_communication
                    },
                    collective_merge: if t >= 0.5 {
                        to_rules.collective_merge
                    } else {
                        from_rules.collective_merge
                    },
                    polarity_conflict: if t < 0.5 {
                        from_rules.polarity_conflict
                    } else {
                        to_rules.polarity_conflict
                    },
                }
            }
        }
    }

    /// Initiate density transition (reincarnation)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Transcend and Include: Each step INCLUDES all previous development
    /// and TRANSCENDS by adding new development."
    pub fn initiate_transition(
        &mut self,
        target_density: Density,
        interpolation_mode: TransitionInterpolationMode,
        duration: Float,
    ) -> Result<(), DensityTransitionError> {
        let duration = duration.clamp(MIN_TRANSITION_DURATION, MAX_TRANSITION_DURATION);

        if self.transition_state.is_transitioning() {
            return Err(DensityTransitionError::AlreadyTransitioning);
        }

        if target_density == self.current_density {
            return Err(DensityTransitionError::SameDensity);
        }

        // Prepare holographic continuity for transition
        self.holographic_continuity
            .prepare_for_transition(self.current_time);

        // Initialize target mechanics
        self.target_mechanics = Some(DensityMechanics::from_density(target_density));

        // Set transition state
        self.transition_state = DensityTransitionState::Transitioning {
            from_density: self.current_density,
            to_density: target_density,
            progress: 0.0,
            interpolation_mode,
            start_time: self.current_time,
            duration,
        };

        Ok(())
    }

    /// Update transition progress
    pub fn update_transition(&mut self, delta_time: Float) -> TransitionUpdateResult {
        if !self.transition_state.is_transitioning() {
            return TransitionUpdateResult::Stable;
        }

        // Extract values from transition state before mutation
        let (from_density, to_density, interpolation_mode, start_time, duration) =
            match &self.transition_state {
                DensityTransitionState::Transitioning {
                    from_density,
                    to_density,
                    interpolation_mode,
                    start_time,
                    duration,
                    ..
                } => (
                    *from_density,
                    *to_density,
                    *interpolation_mode,
                    *start_time,
                    *duration,
                ),
                _ => return TransitionUpdateResult::Stable,
            };

        self.current_time += delta_time;
        let elapsed = self.current_time - start_time;
        let new_progress = (elapsed / duration).clamp(0.0, 1.0);

        if new_progress >= 1.0 {
            // Transition complete
            self.complete_transition();
            return TransitionUpdateResult::Complete {
                new_density: to_density,
            };
        }

        // Update progress
        self.transition_state = DensityTransitionState::Transitioning {
            from_density,
            to_density,
            progress: new_progress,
            interpolation_mode,
            start_time,
            duration,
        };

        // Calculate interpolated values
        let t = interpolation_mode.apply(new_progress);
        let from_veil = from_density.veil_transparency();
        let to_veil = to_density.veil_transparency();
        let from_spectrum = from_density.spectrum_access();
        let to_spectrum = to_density.spectrum_access();

        TransitionUpdateResult::InProgress {
            progress: new_progress,
            interpolated_veil: Self::interpolate_float(from_veil, to_veil, t),
            interpolated_spectrum_access: Self::interpolate_float(from_spectrum, to_spectrum, t),
        }
    }

    /// Complete transition
    fn complete_transition(&mut self) {
        let new_density = if let DensityTransitionState::Transitioning { to_density, .. } =
            &self.transition_state
        {
            *to_density
        } else {
            return;
        };

        // Update current density and mechanics
        self.current_density = new_density;
        self.current_mechanics = DensityMechanics::from_density(new_density);

        // Complete holographic continuity transition
        self.holographic_continuity
            .complete_transition(new_density, self.current_time);

        // Reset transition state
        self.transition_state = DensityTransitionState::Stable;
        self.target_mechanics = None;
    }

    /// Interpolate between two float values
    fn interpolate_float(from: Float, to: Float, t: Float) -> Float {
        from + (to - from) * t
    }

    /// Check if entity can transition to target density
    pub fn can_transition_to(&self, target_density: Density) -> bool {
        // Entities can generally transition up (evolution) or down (regression)
        // But some restrictions apply:
        // - Cannot skip densities during normal evolution
        // - Cannot regress to densities already transcended (unless special circumstances)
        // - 3rd density requires polarity choice before transitioning to 4th

        match self.current_density {
            Density::Third => {
                if target_density == Density::Fourth {
                    // Must have made polarity choice
                    self.holographic_continuity.polarity.is_some()
                } else {
                    true
                }
            }
            _ => true,
        }
    }

    /// Get current catalyst
    pub fn get_catalyst(&self) -> Catalyst {
        Catalyst {
            intensity: self.holographic_continuity.current_catalyst,
            polarity_affinity: self.holographic_continuity.polarity,
            source: format!("{} Density", self.current_density.name()),
        }
    }

    /// Process catalyst (add to accumulation)
    pub fn process_catalyst(&mut self, catalyst: Float) {
        self.holographic_continuity.add_catalyst(catalyst);
    }

    /// Choose polarity (only available in 3rd density)
    pub fn choose_polarity(&mut self, polarity: Polarity) -> Result<(), DensityTransitionError> {
        if self.current_density != Density::Third {
            return Err(DensityTransitionError::InvalidDensityForPolarityChoice);
        }
        self.holographic_continuity.set_polarity(polarity)
    }

    /// Get polarization strength
    pub fn get_polarization_strength(&self) -> Float {
        self.holographic_continuity.polarization_strength
    }

    /// Add lesson learned
    pub fn add_lesson(&mut self, lesson: String) {
        self.holographic_continuity.add_lesson(lesson);
    }
}

/// Density transition error
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionError {
    /// Entity is already transitioning
    AlreadyTransitioning,
    /// Cannot transition to same density
    SameDensity,
    /// Invalid density transition
    InvalidTransition,
    /// Polarity already set
    PolarityAlreadySet,
    /// Invalid density for polarity choice
    InvalidDensityForPolarityChoice,
    /// Transition not in progress
    NotTransitioning,
    /// Catalyst intensity exceeded
    CatalystExceeded,
}

impl std::fmt::Display for DensityTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DensityTransitionError::AlreadyTransitioning => {
                write!(f, "Entity is already transitioning")
            }
            DensityTransitionError::SameDensity => {
                write!(f, "Cannot transition to the same density")
            }
            DensityTransitionError::InvalidTransition => {
                write!(f, "Invalid density transition")
            }
            DensityTransitionError::PolarityAlreadySet => {
                write!(f, "Polarity has already been set")
            }
            DensityTransitionError::InvalidDensityForPolarityChoice => {
                write!(f, "Polarity choice only available in 3rd density")
            }
            DensityTransitionError::NotTransitioning => {
                write!(f, "No transition in progress")
            }
            DensityTransitionError::CatalystExceeded => {
                write!(f, "Catalyst intensity exceeded maximum")
            }
        }
    }
}

impl std::error::Error for DensityTransitionError {}

/// Density transition statistics
#[derive(Debug, Clone, Default)]
pub struct DensityTransitionStatistics {
    /// Total transitions initiated
    pub total_transitions_initiated: u32,
    /// Total transitions completed
    pub total_transitions_completed: u32,
    /// Total time spent in transitions
    pub total_transition_time: Float,
    /// Transitions by density (from -> to -> count)
    pub transitions_by_density: HashMap<(Density, Density), u32>,
    /// Average transition duration
    pub average_transition_duration: Float,
}

impl DensityTransitionStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        DensityTransitionStatistics::default()
    }

    /// Record transition initiated
    pub fn record_initiated(&mut self, from: Density, to: Density) {
        self.total_transitions_initiated += 1;
        *self.transitions_by_density.entry((from, to)).or_insert(0) += 1;
    }

    /// Record transition completed
    pub fn record_completed(&mut self, duration: Float) {
        self.total_transitions_completed += 1;
        self.total_transition_time += duration;
        self.average_transition_duration =
            self.total_transition_time / self.total_transitions_completed as Float;
    }

    /// Get transition count between two densities
    pub fn get_transition_count(&self, from: Density, to: Density) -> u32 {
        self.transitions_by_density
            .get(&(from, to))
            .copied()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::archetype_basis::{ArchetypeBasis, NUM_ARCHETYPES};

    fn create_test_profile() -> ArchetypeActivationProfile {
        let basis = ArchetypeBasis::new(NUM_ARCHETYPES);
        let mut coefficients = [0.0; NUM_ARCHETYPES];
        coefficients[0] = 0.8;
        coefficients[1] = 0.6;
        coefficients[7] = 0.7;
        ArchetypeActivationProfile::new(coefficients)
    }

    #[test]
    fn test_density_transition_system_creation() {
        let system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        assert_eq!(system.current_density, Density::Third);
        assert_eq!(
            system.holographic_continuity.current_density,
            Density::Third
        );
        assert!(!system.transition_state.is_transitioning());
    }

    #[test]
    fn test_initiate_transition() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let result = system.initiate_transition(
            Density::Fourth,
            TransitionInterpolationMode::Holographic,
            5.0,
        );
        assert!(result.is_ok());
        assert!(system.transition_state.is_transitioning());
    }

    #[test]
    fn test_cannot_transition_to_same_density() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let result =
            system.initiate_transition(Density::Third, TransitionInterpolationMode::Linear, 5.0);
        assert!(matches!(result, Err(DensityTransitionError::SameDensity)));
    }

    #[test]
    fn test_cannot_double_transition() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system
            .initiate_transition(Density::Fourth, TransitionInterpolationMode::Linear, 5.0)
            .unwrap();
        let result =
            system.initiate_transition(Density::Fifth, TransitionInterpolationMode::Linear, 5.0);
        assert!(matches!(
            result,
            Err(DensityTransitionError::AlreadyTransitioning)
        ));
    }

    #[test]
    fn test_transition_progress() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system
            .initiate_transition(Density::Fourth, TransitionInterpolationMode::Linear, 5.0)
            .unwrap();

        let update = system.update_transition(2.5);
        assert!(
            matches!(update, TransitionUpdateResult::InProgress { progress, .. } if progress >= 0.49 && progress <= 0.51)
        );
    }

    #[test]
    fn test_transition_completion() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system
            .initiate_transition(Density::Fourth, TransitionInterpolationMode::Linear, 5.0)
            .unwrap();

        // Fast forward to completion
        let update = system.update_transition(6.0);
        assert!(matches!(update, TransitionUpdateResult::Complete { .. }));
        assert_eq!(system.current_density, Density::Fourth);
        assert!(!system.transition_state.is_transitioning());
    }

    #[test]
    fn test_holographic_continuity() {
        let system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let continuity = &system.holographic_continuity;
        assert_eq!(continuity.current_density, Density::Third);
        assert_eq!(continuity.density_memories.len(), 1);
        assert_eq!(continuity.density_memories[0].density, Density::Third);
    }

    #[test]
    fn test_catalyst_accumulation() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system.process_catalyst(0.5);
        assert_eq!(system.holographic_continuity.current_catalyst, 0.5);
        assert_eq!(system.holographic_continuity.total_catalyst, 0.5);

        system.process_catalyst(0.3);
        assert_eq!(system.holographic_continuity.current_catalyst, 0.8);
        assert_eq!(system.holographic_continuity.total_catalyst, 0.8);
    }

    #[test]
    fn test_catalyst_clamping() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system.process_catalyst(0.5);
        system.process_catalyst(0.6);
        assert_eq!(system.holographic_continuity.current_catalyst, 1.0);
        assert_eq!(system.holographic_continuity.total_catalyst, 1.0);
    }

    #[test]
    fn test_polarity_choice() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let result = system.choose_polarity(Polarity::ServiceToOthers);
        assert!(result.is_ok());
        assert_eq!(
            system.holographic_continuity.polarity,
            Some(Polarity::ServiceToOthers)
        );
    }

    #[test]
    fn test_polarity_cannot_change() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system.choose_polarity(Polarity::ServiceToOthers).unwrap();
        let result = system.choose_polarity(Polarity::ServiceToSelf);
        assert!(matches!(
            result,
            Err(DensityTransitionError::PolarityAlreadySet)
        ));
    }

    #[test]
    fn test_polarity_only_in_third_density() {
        let mut system = DensityTransitionSystem::new(Density::Fourth, create_test_profile());
        let result = system.choose_polarity(Polarity::ServiceToOthers);
        assert!(matches!(
            result,
            Err(DensityTransitionError::InvalidDensityForPolarityChoice)
        ));
    }

    #[test]
    fn test_4th_density_requires_polarity() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system
            .initiate_transition(Density::Fourth, TransitionInterpolationMode::Linear, 5.0)
            .unwrap();

        let result = system.can_transition_to(Density::Fourth);
        assert!(!result);
    }

    #[test]
    fn test_4th_density_allowed_with_polarity() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system.choose_polarity(Polarity::ServiceToOthers).unwrap();
        assert!(system.can_transition_to(Density::Fourth));
    }

    #[test]
    fn test_transition_preserves_continuity() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system.process_catalyst(0.5);
        system.choose_polarity(Polarity::ServiceToOthers).unwrap();
        system.add_lesson("Catalyst is essential for growth".to_string());

        system
            .initiate_transition(
                Density::Fourth,
                TransitionInterpolationMode::Holographic,
                5.0,
            )
            .unwrap();
        system.update_transition(6.0);

        // Continuity preserved
        assert_eq!(
            system.holographic_continuity.polarity,
            Some(Polarity::ServiceToOthers)
        );
        assert_eq!(system.holographic_continuity.transitions_completed, 1);
        assert_eq!(
            system.holographic_continuity.current_density,
            Density::Fourth
        );
        assert!(system.holographic_continuity.density_memories.len() >= 2);

        // Catalyst reset but total preserved
        assert_eq!(system.holographic_continuity.current_catalyst, 0.0);
        assert_eq!(system.holographic_continuity.total_catalyst, 0.5);
    }

    #[test]
    fn test_interpolation_modes() {
        let t = 0.5;

        let linear = TransitionInterpolationMode::Linear.apply(t);
        assert_eq!(linear, 0.5);

        let ease_in_out = TransitionInterpolationMode::EaseInOut.apply(t);
        assert!((ease_in_out - 0.5).abs() < 0.01);

        let holographic = TransitionInterpolationMode::Holographic.apply(t);
        assert!((holographic - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_perception_filter_during_transition() {
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        system
            .initiate_transition(Density::Sixth, TransitionInterpolationMode::Linear, 10.0)
            .unwrap();
        system.update_transition(5.0);

        let filter = system.get_perception_filter();
        assert!(filter.veil_transparency > 0.1);
        assert!(filter.spectrum_access > 0.2);
    }

    #[test]
    fn test_density_memory_entry() {
        let mut entry = DensityMemoryEntry::new(Density::Third, 100.0);
        entry.add_time(1000);
        entry.add_catalyst(0.7);
        entry.set_polarity(Polarity::ServiceToOthers, 0.8);
        entry.add_lesson("Lesson 1".to_string());
        entry.add_lesson("Lesson 2".to_string());
        entry.exit(200.0);

        assert_eq!(entry.time_spent, 1000);
        assert_eq!(entry.catalyst_accumulated, 0.7);
        assert_eq!(entry.polarity, Some(Polarity::ServiceToOthers));
        assert_eq!(entry.polarization_strength, 0.8);
        assert_eq!(entry.lessons_learned.len(), 2);
        assert_eq!(entry.exit_timestamp, Some(200.0));
        assert!(entry.is_complete());
    }

    #[test]
    fn test_transition_statistics() {
        let mut stats = DensityTransitionStatistics::new();
        stats.record_initiated(Density::Third, Density::Fourth);
        stats.record_completed(5.0);
        stats.record_initiated(Density::Third, Density::Fourth);
        stats.record_completed(7.0);

        assert_eq!(stats.total_transitions_initiated, 2);
        assert_eq!(stats.total_transitions_completed, 2);
        assert_eq!(
            stats.get_transition_count(Density::Third, Density::Fourth),
            2
        );
        assert!((stats.average_transition_duration - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_duration_clamping() {
        // Test minimum duration (should be clamped to MIN_TRANSITION_DURATION)
        let mut system = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let result =
            system.initiate_transition(Density::Fourth, TransitionInterpolationMode::Linear, 0.5);
        assert!(result.is_ok());

        // Test maximum duration (should be clamped to MAX_TRANSITION_DURATION)
        let mut system2 = DensityTransitionSystem::new(Density::Third, create_test_profile());
        let result2 = system2.initiate_transition(
            Density::Fourth,
            TransitionInterpolationMode::Linear,
            100.0,
        );
        assert!(result2.is_ok());
    }

    #[test]
    fn test_multiple_density_memories() {
        let mut system = DensityTransitionSystem::new(Density::First, create_test_profile());

        // Transition through multiple densities
        for (from, to) in [
            (Density::First, Density::Second),
            (Density::Second, Density::Third),
            (Density::Third, Density::Fourth),
        ] {
            system.current_density = from;
            system.holographic_continuity.current_density = from;
            system
                .initiate_transition(to, TransitionInterpolationMode::Linear, 5.0)
                .unwrap();
            system.update_transition(6.0);
        }

        assert!(system.holographic_continuity.density_memories.len() >= 4);
    }

    #[test]
    fn test_veil_transparency_values() {
        assert_eq!(Density::Third.veil_transparency(), 0.1);
        assert_eq!(Density::Sixth.veil_transparency(), 1.0);
        assert_eq!(Density::Eighth.veil_transparency(), 1.0);
    }

    #[test]
    fn test_spectrum_access_values() {
        assert_eq!(Density::Third.spectrum_access(), 0.2);
        assert_eq!(Density::Sixth.spectrum_access(), 0.9);
        assert_eq!(Density::Eighth.spectrum_access(), 1.0);
    }
}
