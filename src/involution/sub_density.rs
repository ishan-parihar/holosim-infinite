//! Sub-Density System
//!
//! This module provides support for sub-densities within each density level.
//! Each density has 7 sub-densities (e.g., D4.1 to D4.7) for finer granularity
//! in tracking entity progression through the evolutionary journey.

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;

/// Sub-density level (1-7) within a density
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubDensity(u8);

impl SubDensity {
    /// Create a new sub-density (1-7)
    pub fn new(level: u8) -> Option<Self> {
        if level >= 1 && level <= 7 {
            Some(SubDensity(level))
        } else {
            None
        }
    }

    /// Get the sub-density level (1-7)
    pub fn level(&self) -> u8 {
        self.0
    }

    /// Get the next sub-density level
    pub fn next(&self) -> Option<Self> {
        if self.0 < 7 {
            Some(SubDensity(self.0 + 1))
        } else {
            None
        }
    }

    /// Get the previous sub-density level
    pub fn previous(&self) -> Option<Self> {
        if self.0 > 1 {
            Some(SubDensity(self.0 - 1))
        } else {
            None
        }
    }

    /// Check if this is the first sub-density
    pub fn is_first(&self) -> bool {
        self.0 == 1
    }

    /// Check if this is the last sub-density
    pub fn is_last(&self) -> bool {
        self.0 == 7
    }

    /// Get the stage of this sub-density
    pub fn stage(&self) -> SubDensityStage {
        match self.0 {
            1..=2 => SubDensityStage::Early,
            3..=4 => SubDensityStage::Middle,
            5..=6 => SubDensityStage::Late,
            7 => SubDensityStage::Completion,
            _ => unreachable!(),
        }
    }
}

impl Default for SubDensity {
    fn default() -> Self {
        SubDensity(1)
    }
}

impl std::fmt::Display for SubDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.0)
    }
}

/// Sub-density stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubDensityStage {
    /// Early stage (sub-density 1-2): Learning fundamentals
    Early,
    /// Middle stage (sub-density 3-4): Integration and practice
    Middle,
    /// Late stage (sub-density 5-6): Mastery and teaching
    Late,
    /// Completion (sub-density 7): Ready for next density
    Completion,
}

impl SubDensityStage {
    /// Get a description of this stage
    pub fn description(&self) -> &'static str {
        match self {
            SubDensityStage::Early => "Learning fundamentals",
            SubDensityStage::Middle => "Integration and practice",
            SubDensityStage::Late => "Mastery and teaching",
            SubDensityStage::Completion => "Ready for next density",
        }
    }

    /// Get the difficulty modifier for this stage
    pub fn difficulty_modifier(&self) -> Float {
        match self {
            SubDensityStage::Early => 1.0,      // Normal difficulty
            SubDensityStage::Middle => 1.2,     // Slightly harder
            SubDensityStage::Late => 1.5,       // More challenging
            SubDensityStage::Completion => 0.8, // Easier, preparing for transition
        }
    }

    /// Get the catalyst intensity multiplier for this stage
    pub fn catalyst_multiplier(&self) -> Float {
        match self {
            SubDensityStage::Early => 1.0,      // Normal catalyst
            SubDensityStage::Middle => 1.3,     // More catalyst
            SubDensityStage::Late => 1.6,       // Maximum catalyst
            SubDensityStage::Completion => 0.7, // Less catalyst, focusing on integration
        }
    }
}

/// Encoded sub-density value
///
/// Encoding: `encoded = density * 10 + sub_level`
/// Example: D4.3 = 4 * 10 + 3 = 43
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncodedSubDensity(u8);

impl EncodedSubDensity {
    /// Encode a density and sub-density level
    pub fn encode(density: Density, sub_level: u8) -> Option<Self> {
        if sub_level >= 1 && sub_level <= 7 {
            Some(EncodedSubDensity(density.as_u8() * 10 + sub_level))
        } else {
            None
        }
    }

    /// Decode an encoded sub-density value
    pub fn decode(encoded: u8) -> Option<(Density, SubDensity)> {
        let density_value = encoded / 10;
        let sub_level = encoded % 10;

        if sub_level >= 1 && sub_level <= 7 {
            if let Some(density) = Density::from_u8(density_value) {
                if let Some(sub_density) = SubDensity::new(sub_level) {
                    return Some((density, sub_density));
                }
            }
        }

        None
    }

    /// Get the encoded value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get the density from the encoded value
    pub fn density(&self) -> Option<Density> {
        Density::from_u8(self.0 / 10)
    }

    /// Get the sub-density level from the encoded value
    pub fn sub_level(&self) -> Option<SubDensity> {
        SubDensity::new(self.0 % 10)
    }
}

impl std::fmt::Display for EncodedSubDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(density) = self.density() {
            if let Some(sub_level) = self.sub_level() {
                return write!(f, "D{}.{}", density.as_u8(), sub_level.level());
            }
        }
        write!(f, "Invalid({})", self.0)
    }
}

/// Sub-density transition result
#[derive(Debug, Clone, PartialEq)]
pub enum SubDensityTransitionResult {
    /// Transition successful
    Success {
        /// Previous encoded sub-density
        from: EncodedSubDensity,
        /// New encoded sub-density
        to: EncodedSubDensity,
    },
    /// Transition failed - invalid sub-density
    InvalidSubDensity {
        /// The invalid sub-density value
        value: u8,
    },
    /// Transition failed - already at maximum sub-density
    AlreadyAtMaximum {
        /// Current encoded sub-density
        current: EncodedSubDensity,
    },
    /// Transition failed - cannot skip sub-densities
    CannotSkip {
        /// Current encoded sub-density
        current: EncodedSubDensity,
        /// Target encoded sub-density
        target: EncodedSubDensity,
    },
    /// Transition failed - cannot move backwards
    CannotMoveBackwards {
        /// Current encoded sub-density
        current: EncodedSubDensity,
        /// Target encoded sub-density
        target: EncodedSubDensity,
    },
}

/// Sub-density progression tracking for an entity
#[derive(Debug, Clone, PartialEq)]
pub struct SubDensityProgression {
    /// Current encoded sub-density
    pub current: EncodedSubDensity,
    /// Time spent in current sub-density (in seconds)
    pub time_in_current: u64,
    /// Total time spent in this density (in seconds)
    pub total_time_in_density: u64,
    /// Catalyst processed in current sub-density
    pub catalyst_processed: usize,
    /// Total catalyst processed in this density
    pub total_catalyst_in_density: usize,
    /// Stage completion status for each sub-density
    pub stages_completed: [bool; 7],
}

impl SubDensityProgression {
    /// Create a new sub-density progression starting at the first sub-density
    pub fn new(density: Density) -> Option<Self> {
        let current = EncodedSubDensity::encode(density, 1)?;
        Some(Self {
            current,
            time_in_current: 0,
            total_time_in_density: 0,
            catalyst_processed: 0,
            total_catalyst_in_density: 0,
            stages_completed: [false; 7],
        })
    }

    /// Create a new sub-density progression at a specific sub-density
    pub fn at(density: Density, sub_level: u8) -> Option<Self> {
        let current = EncodedSubDensity::encode(density, sub_level)?;
        Some(Self {
            current,
            time_in_current: 0,
            total_time_in_density: 0,
            catalyst_processed: 0,
            total_catalyst_in_density: 0,
            stages_completed: [false; 7],
        })
    }

    /// Advance to the next sub-density
    pub fn advance(&mut self) -> Option<EncodedSubDensity> {
        let current_sub_level = self.current.sub_level()?;
        if !current_sub_level.is_last() {
            // Mark current stage as completed
            let index = (current_sub_level.level() - 1) as usize;
            self.stages_completed[index] = true;

            // Reset current metrics
            self.time_in_current = 0;
            self.catalyst_processed = 0;

            // Advance to next sub-density
            let next_sub_level = current_sub_level.next()?;
            let density = self.current.density()?;
            self.current = EncodedSubDensity::encode(density, next_sub_level.level())?;
            Some(self.current)
        } else {
            // At the last sub-density, mark it as completed
            let index = (current_sub_level.level() - 1) as usize;
            self.stages_completed[index] = true;
            None
        }
    }

    /// Get the current sub-density stage
    pub fn current_stage(&self) -> Option<SubDensityStage> {
        Some(self.current.sub_level()?.stage())
    }

    /// Check if ready to advance to next density
    pub fn ready_for_next_density(&self) -> bool {
        if let Some(sub_level) = self.current.sub_level() {
            if sub_level.is_last() {
                // Check if all previous stages are completed
                let all_previous_completed =
                    (0..(sub_level.level() - 1) as usize).all(|i| self.stages_completed[i]);
                return all_previous_completed;
            }
        }
        false
    }

    /// Update time spent in current sub-density
    pub fn update_time(&mut self, delta_seconds: u64) {
        self.time_in_current += delta_seconds;
        self.total_time_in_density += delta_seconds;
    }

    /// Record catalyst processed
    pub fn record_catalyst(&mut self) {
        self.catalyst_processed += 1;
        self.total_catalyst_in_density += 1;
    }

    /// Get completion percentage for this density
    pub fn completion_percentage(&self) -> Float {
        let current_level = self.current.sub_level().map_or(1, |sd| sd.level()) as Float;
        (current_level / 7.0) * 100.0
    }
}

/// Sub-Density System
///
/// Manages sub-density transitions and progression for entities.
#[derive(Debug, Clone)]
pub struct SubDensitySystem {
    /// Maps entity IDs to their sub-density progression
    progressions: std::collections::HashMap<u64, SubDensityProgression>,
}

impl SubDensitySystem {
    /// Create a new sub-density system
    pub fn new() -> Self {
        Self {
            progressions: std::collections::HashMap::new(),
        }
    }

    /// Initialize sub-density progression for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `density` - The density to start at
    /// * `start_at_sub_level` - Starting sub-density level (1-7), defaults to 1
    ///
    /// # Returns
    /// The initial encoded sub-density, or None if invalid
    pub fn initialize(
        &mut self,
        entity_id: u64,
        density: Density,
        start_at_sub_level: Option<u8>,
    ) -> Option<EncodedSubDensity> {
        let sub_level = start_at_sub_level.unwrap_or(1);
        let progression = SubDensityProgression::at(density, sub_level)?;
        let current = progression.current;
        self.progressions.insert(entity_id, progression);
        Some(current)
    }

    /// Transition to a specific sub-density
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `target` - The target encoded sub-density
    ///
    /// # Returns
    /// Transition result
    pub fn transition_to_subdensity(
        &mut self,
        entity_id: u64,
        target: EncodedSubDensity,
    ) -> SubDensityTransitionResult {
        // Validate target
        if target.density().is_none() || target.sub_level().is_none() {
            return SubDensityTransitionResult::InvalidSubDensity {
                value: target.value(),
            };
        }

        // Get current progression
        let progression = self.progressions.get(&entity_id);

        if let Some(prog) = progression {
            let current = prog.current;

            // Check if trying to move backwards
            if target.value() < current.value() {
                return SubDensityTransitionResult::CannotMoveBackwards { current, target };
            }

            // Check if trying to skip sub-densities
            if target.value() > current.value() + 1 {
                return SubDensityTransitionResult::CannotSkip { current, target };
            }

            // Check if already at maximum
            if current.sub_level().map_or(false, |sd| sd.is_last()) {
                return SubDensityTransitionResult::AlreadyAtMaximum { current };
            }

            // Perform transition
            if let Some(prog) = self.progressions.get_mut(&entity_id) {
                if let Some(_) = prog.advance() {
                    return SubDensityTransitionResult::Success {
                        from: current,
                        to: prog.current,
                    };
                }
            }
        }

        SubDensityTransitionResult::InvalidSubDensity {
            value: target.value(),
        }
    }

    /// Advance to the next sub-density for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The new encoded sub-density, or None if already at maximum
    pub fn advance_subdensity(&mut self, entity_id: u64) -> Option<EncodedSubDensity> {
        self.progressions.get_mut(&entity_id)?.advance()
    }

    /// Get the current sub-density for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The current encoded sub-density, or None if entity not found
    pub fn get_current_subdensity(&self, entity_id: u64) -> Option<EncodedSubDensity> {
        self.progressions.get(&entity_id).map(|p| p.current)
    }

    /// Get the sub-density progression for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The sub-density progression, or None if entity not found
    pub fn get_progression(&self, entity_id: u64) -> Option<&SubDensityProgression> {
        self.progressions.get(&entity_id)
    }

    /// Check if an entity is ready for the next density
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// True if ready for next density, false otherwise
    pub fn is_ready_for_next_density(&self, entity_id: u64) -> bool {
        self.progressions
            .get(&entity_id)
            .map_or(false, |p| p.ready_for_next_density())
    }

    /// Update time spent in current sub-density
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `delta_seconds` - Time elapsed in seconds
    pub fn update_time(&mut self, entity_id: u64, delta_seconds: u64) {
        if let Some(prog) = self.progressions.get_mut(&entity_id) {
            prog.update_time(delta_seconds);
        }
    }

    /// Record catalyst processed for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    pub fn record_catalyst(&mut self, entity_id: u64) {
        if let Some(prog) = self.progressions.get_mut(&entity_id) {
            prog.record_catalyst();
        }
    }

    /// Remove an entity from the system
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.progressions.remove(&entity_id);
    }

    /// Clear all progressions
    pub fn clear_all(&mut self) {
        self.progressions.clear();
    }

    /// Get the number of entities being tracked
    pub fn entity_count(&self) -> usize {
        self.progressions.len()
    }
}

impl Default for SubDensitySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_density_creation() {
        let sd = SubDensity::new(3).unwrap();
        assert_eq!(sd.level(), 3);
    }

    #[test]
    fn test_sub_density_invalid() {
        assert!(SubDensity::new(0).is_none());
        assert!(SubDensity::new(8).is_none());
    }

    #[test]
    fn test_sub_density_next() {
        let sd = SubDensity::new(5).unwrap();
        assert_eq!(sd.next().unwrap().level(), 6);
        assert!(SubDensity::new(7).unwrap().next().is_none());
    }

    #[test]
    fn test_sub_density_previous() {
        let sd = SubDensity::new(5).unwrap();
        assert_eq!(sd.previous().unwrap().level(), 4);
        assert!(SubDensity::new(1).unwrap().previous().is_none());
    }

    #[test]
    fn test_sub_density_is_first() {
        assert!(SubDensity::new(1).unwrap().is_first());
        assert!(!SubDensity::new(2).unwrap().is_first());
    }

    #[test]
    fn test_sub_density_is_last() {
        assert!(SubDensity::new(7).unwrap().is_last());
        assert!(!SubDensity::new(6).unwrap().is_last());
    }

    #[test]
    fn test_sub_density_stage() {
        assert_eq!(SubDensity::new(1).unwrap().stage(), SubDensityStage::Early);
        assert_eq!(SubDensity::new(2).unwrap().stage(), SubDensityStage::Early);
        assert_eq!(SubDensity::new(3).unwrap().stage(), SubDensityStage::Middle);
        assert_eq!(SubDensity::new(4).unwrap().stage(), SubDensityStage::Middle);
        assert_eq!(SubDensity::new(5).unwrap().stage(), SubDensityStage::Late);
        assert_eq!(SubDensity::new(6).unwrap().stage(), SubDensityStage::Late);
        assert_eq!(
            SubDensity::new(7).unwrap().stage(),
            SubDensityStage::Completion
        );
    }

    #[test]
    fn test_sub_density_stage_description() {
        assert_eq!(
            SubDensityStage::Early.description(),
            "Learning fundamentals"
        );
        assert_eq!(
            SubDensityStage::Middle.description(),
            "Integration and practice"
        );
        assert_eq!(SubDensityStage::Late.description(), "Mastery and teaching");
        assert_eq!(
            SubDensityStage::Completion.description(),
            "Ready for next density"
        );
    }

    #[test]
    fn test_sub_density_stage_difficulty_modifier() {
        assert_eq!(SubDensityStage::Early.difficulty_modifier(), 1.0);
        assert_eq!(SubDensityStage::Middle.difficulty_modifier(), 1.2);
        assert_eq!(SubDensityStage::Late.difficulty_modifier(), 1.5);
        assert_eq!(SubDensityStage::Completion.difficulty_modifier(), 0.8);
    }

    #[test]
    fn test_sub_density_stage_catalyst_multiplier() {
        assert_eq!(SubDensityStage::Early.catalyst_multiplier(), 1.0);
        assert_eq!(SubDensityStage::Middle.catalyst_multiplier(), 1.3);
        assert_eq!(SubDensityStage::Late.catalyst_multiplier(), 1.6);
        assert_eq!(SubDensityStage::Completion.catalyst_multiplier(), 0.7);
    }

    #[test]
    fn test_encoded_sub_density_encode() {
        let encoded = EncodedSubDensity::encode(Density::Fourth, 3).unwrap();
        assert_eq!(encoded.value(), 43);
    }

    #[test]
    fn test_encoded_sub_density_encode_invalid() {
        assert!(EncodedSubDensity::encode(Density::Fourth, 0).is_none());
        assert!(EncodedSubDensity::encode(Density::Fourth, 8).is_none());
    }

    #[test]
    fn test_encoded_sub_density_decode() {
        let (density, sub_level) = EncodedSubDensity::decode(43).unwrap();
        assert_eq!(density, Density::Fourth);
        assert_eq!(sub_level.level(), 3);
    }

    #[test]
    fn test_encoded_sub_density_decode_invalid() {
        assert!(EncodedSubDensity::decode(40).is_none());
        assert!(EncodedSubDensity::decode(48).is_none());
    }

    #[test]
    fn test_encoded_sub_density_roundtrip() {
        let original = EncodedSubDensity::encode(Density::Fifth, 7).unwrap();
        let (density, sub_level) = EncodedSubDensity::decode(original.value()).unwrap();
        assert_eq!(density, Density::Fifth);
        assert_eq!(sub_level.level(), 7);
    }

    #[test]
    fn test_encoded_sub_density_density() {
        let encoded = EncodedSubDensity::encode(Density::Third, 5).unwrap();
        assert_eq!(encoded.density(), Some(Density::Third));
    }

    #[test]
    fn test_encoded_sub_density_sub_level() {
        let encoded =
            EncodedSubDensity::encode(Density::Second(Density2SubLevel::Cellular), 4).unwrap();
        assert_eq!(encoded.sub_level().map(|sd| sd.level()), Some(4));
    }

    #[test]
    fn test_encoded_sub_density_display() {
        let encoded = EncodedSubDensity::encode(Density::Fourth, 3).unwrap();
        assert_eq!(format!("{}", encoded), "D4.3");
    }

    #[test]
    fn test_sub_density_progression_new() {
        let prog = SubDensityProgression::new(Density::Fourth).unwrap();
        assert_eq!(prog.current.value(), 41);
        assert_eq!(prog.time_in_current, 0);
        assert!(!prog.ready_for_next_density());
    }

    #[test]
    fn test_sub_density_progression_advance() {
        let mut prog = SubDensityProgression::new(Density::Fourth).unwrap();
        let new_current = prog.advance().unwrap();
        assert_eq!(new_current.value(), 42);
        assert_eq!(prog.stages_completed[0], true);
    }

    #[test]
    fn test_sub_density_progression_advance_to_completion() {
        let mut prog = SubDensityProgression::new(Density::Fourth).unwrap();
        for _ in 1..7 {
            prog.advance();
        }
        assert!(prog.advance().is_none());
        assert!(prog.ready_for_next_density());
    }

    #[test]
    fn test_sub_density_progression_current_stage() {
        let prog = SubDensityProgression::new(Density::Fourth).unwrap();
        assert_eq!(prog.current_stage(), Some(SubDensityStage::Early));
    }

    #[test]
    fn test_sub_density_progression_completion_percentage() {
        let prog = SubDensityProgression::new(Density::Fourth).unwrap();
        let expected = (1.0 / 7.0) * 100.0;
        assert!((prog.completion_percentage() - expected).abs() < 1e-10);

        let mut prog = prog.clone();
        prog.advance();
        let expected = (2.0 / 7.0) * 100.0;
        assert!((prog.completion_percentage() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_sub_density_system_initialize() {
        let mut system = SubDensitySystem::new();
        let encoded = system.initialize(123, Density::Fourth, None).unwrap();
        assert_eq!(encoded.value(), 41);
    }

    #[test]
    fn test_sub_density_system_initialize_at_sub_level() {
        let mut system = SubDensitySystem::new();
        let encoded = system.initialize(123, Density::Fourth, Some(5)).unwrap();
        assert_eq!(encoded.value(), 45);
    }

    #[test]
    fn test_sub_density_system_transition_success() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        let target = EncodedSubDensity::encode(Density::Fourth, 2).unwrap();
        let result = system.transition_to_subdensity(123, target);

        match result {
            SubDensityTransitionResult::Success { from, to } => {
                assert_eq!(from.value(), 41);
                assert_eq!(to.value(), 42);
            }
            _ => panic!("Expected Success"),
        }
    }

    #[test]
    fn test_sub_density_system_transition_invalid() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        let target = EncodedSubDensity(48); // Invalid (D4.8)
        let result = system.transition_to_subdensity(123, target);

        match result {
            SubDensityTransitionResult::InvalidSubDensity { value } => {
                assert_eq!(value, 48);
            }
            _ => panic!("Expected InvalidSubDensity"),
        }
    }

    #[test]
    fn test_sub_density_system_transition_skip() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        let target = EncodedSubDensity::encode(Density::Fourth, 3).unwrap();
        let result = system.transition_to_subdensity(123, target);

        match result {
            SubDensityTransitionResult::CannotSkip { current, target: t } => {
                assert_eq!(current.value(), 41);
                assert_eq!(t.value(), 43);
            }
            _ => panic!("Expected CannotSkip"),
        }
    }

    #[test]
    fn test_sub_density_system_transition_backwards() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, Some(5));

        let target = EncodedSubDensity::encode(Density::Fourth, 3).unwrap();
        let result = system.transition_to_subdensity(123, target);

        match result {
            SubDensityTransitionResult::CannotMoveBackwards { current, target: t } => {
                assert_eq!(current.value(), 45);
                assert_eq!(t.value(), 43);
            }
            _ => panic!("Expected CannotMoveBackwards"),
        }
    }

    #[test]
    fn test_sub_density_system_advance_subdensity() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        let new_current = system.advance_subdensity(123).unwrap();
        assert_eq!(new_current.value(), 42);
    }

    #[test]
    fn test_sub_density_system_get_current_subdensity() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, Some(3));

        let current = system.get_current_subdensity(123).unwrap();
        assert_eq!(current.value(), 43);
    }

    #[test]
    fn test_sub_density_system_is_ready_for_next_density() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, Some(1));

        assert!(!system.is_ready_for_next_density(123));

        // Advance to completion
        for _ in 1..7 {
            system.advance_subdensity(123);
        }

        assert!(system.is_ready_for_next_density(123));
    }

    #[test]
    fn test_sub_density_system_update_time() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        system.update_time(123, 100);
        let prog = system.get_progression(123).unwrap();
        assert_eq!(prog.time_in_current, 100);
        assert_eq!(prog.total_time_in_density, 100);
    }

    #[test]
    fn test_sub_density_system_record_catalyst() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);

        system.record_catalyst(123);
        let prog = system.get_progression(123).unwrap();
        assert_eq!(prog.catalyst_processed, 1);
        assert_eq!(prog.total_catalyst_in_density, 1);
    }

    #[test]
    fn test_sub_density_system_remove_entity() {
        let mut system = SubDensitySystem::new();
        system.initialize(123, Density::Fourth, None);
        assert_eq!(system.entity_count(), 1);

        system.remove_entity(123);
        assert_eq!(system.entity_count(), 0);
        assert!(system.get_current_subdensity(123).is_none());
    }

    #[test]
    fn test_sub_density_system_clear_all() {
        let mut system = SubDensitySystem::new();
        system.initialize(111, Density::Fourth, None);
        system.initialize(222, Density::Third, None);
        assert_eq!(system.entity_count(), 2);

        system.clear_all();
        assert_eq!(system.entity_count(), 0);
    }

    #[test]
    fn test_default() {
        let system = SubDensitySystem::default();
        assert_eq!(system.entity_count(), 0);
    }
}
