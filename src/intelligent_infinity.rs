// Intelligent Infinity Continuous Field
// Phase 1: Continuous Involution-Evolution Architecture
//
// This module implements the continuous pulsing of Intelligent Infinity as the source of all energy.
//
// CRITICAL UNDERSTANDING:
// - Intelligent Infinity is a continuous background field, not a one-time pulse
// - The field pulses rhythmically (like a giant heart) from potential to kinetic
// - All entities can tap into this field through their free will capacity
// - The field never stops pulsing (never-ending involution)
// - Tap strength depends on entity's free will capacity
//
// From Law of One Cosmology:
// "Intelligent Infinity is, in many ways, less illogical than the concept of the beginning
// of a creation. The creation is continuous, without beginning or end, and is rhythmic in
// nature. This rhythmic pulsing is the heartbeat of the universe."

use crate::types::Float;
use crate::types::HolonID;
use std::collections::HashMap;

/// Intelligent Infinity Field
///
/// Represents the continuous pulsing field of Intelligent Infinity that is the source
/// of all energy in the universe. The field pulses rhythmically from potential (unmanifest)
/// to kinetic (manifest) and back again.
#[derive(Debug, Clone)]
pub struct IntelligentInfinity {
    /// Unmanifest source (always 1.0 - infinite potential)
    pub potential: Float,

    /// Manifest energy (pulses 0.0 ↔ 1.0)
    pub kinetic: Float,

    /// Current phase in pulse cycle (0.0 to 2π)
    pub pulse_phase: Float,

    /// Rhythmic flow rate (determines pulse frequency)
    pub pulse_frequency: Float,

    /// Who's tapping and how much (entity_id → tap_strength)
    pub free_will_taps: HashMap<HolonID, Float>,

    /// Total energy tapped in current cycle
    pub total_tapped_energy: Float,

    /// Pulse count (for tracking cycles)
    pub pulse_count: u64,
}

impl IntelligentInfinity {
    /// Create a new Intelligent Infinity field with continuous pulsing
    ///
    /// # Arguments
    /// * `pulse_frequency` - The frequency of the rhythmic pulse (default: 0.1)
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::intelligent_infinity::IntelligentInfinity;
    ///
    /// let ii_field = IntelligentInfinity::new(0.1);
    /// assert_eq!(ii_field.potential, 1.0);
    /// assert_eq!(ii_field.kinetic, 0.5); // Starts at midpoint
    /// ```
    pub fn new(pulse_frequency: Float) -> Self {
        let pulse_phase = 0.0_f64;
        let kinetic = (pulse_phase.sin() + 1.0) / 2.0; // 0.0 to 1.0

        IntelligentInfinity {
            potential: 1.0, // Infinite potential
            kinetic,        // Starts at midpoint
            pulse_phase,
            pulse_frequency,
            free_will_taps: HashMap::new(),
            total_tapped_energy: 0.0,
            pulse_count: 0,
        }
    }

    /// Create with default pulse frequency
    pub fn with_default_frequency() -> Self {
        Self::new(0.1)
    }

    /// Continuous pulsing (called every simulation tick)
    ///
    /// This method advances the pulse phase and updates the kinetic energy level.
    /// The pulse follows a sine wave pattern: potential ↔ kinetic ↔ potential.
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::intelligent_infinity::IntelligentInfinity;
    ///
    /// let mut ii_field = IntelligentInfinity::with_default_frequency();
    /// ii_field.pulse();
    ///
    /// // Kinetic energy should have changed
    /// assert!(ii_field.kinetic >= 0.0 && ii_field.kinetic <= 1.0);
    /// ```
    pub fn pulse(&mut self) {
        // Advance phase
        self.pulse_phase += self.pulse_frequency;

        // Track pulse cycles
        if self.pulse_phase >= (std::f64::consts::PI * 2.0) {
            self.pulse_phase -= std::f64::consts::PI * 2.0;
            self.pulse_count += 1;
        }

        // Calculate kinetic energy (pulses 0.0 ↔ 1.0)
        self.kinetic = (self.pulse_phase.sin() + 1.0) / 2.0;

        // Reset tapped energy for new cycle
        if self.pulse_count > 0 && self.pulse_phase < self.pulse_frequency {
            self.total_tapped_energy = 0.0;
            self.free_will_taps.clear();
        }
    }

    /// Tap into Intelligent Infinity through free will
    ///
    /// Entities can tap into the Intelligent Infinity field based on their free will capacity.
    /// The amount tapped depends on:
    /// 1. Entity's free will capacity (awareness, understanding, etc.)
    /// 2. Current kinetic energy level of the field
    /// 3. Entity's current state (developmental level, polarization, etc.)
    ///
    /// # Arguments
    /// * `entity_id` - The ID of the entity tapping
    /// * `free_will_capacity` - The entity's free will capacity (0.0 to 1.0)
    ///
    /// # Returns
    /// The amount of energy tapped (0.0 to 1.0)
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::intelligent_infinity::IntelligentInfinity;
    /// use holonic_realms::types::HolonID;
    ///
    /// let mut ii_field = IntelligentInfinity::with_default_frequency();
    /// let energy = ii_field.tap(1, 0.5);
    ///
    /// assert!(energy >= 0.0 && energy <= 1.0);
    /// ```
    pub fn tap(&mut self, entity_id: HolonID, free_will_capacity: Float) -> Float {
        // Amount tapped = free will capacity × kinetic energy
        let tap_strength = free_will_capacity * self.kinetic;

        // Record tap
        self.free_will_taps.insert(entity_id, tap_strength);
        self.total_tapped_energy += tap_strength;

        tap_strength
    }

    /// Tap with additional modifiers
    ///
    /// Advanced tap method that considers additional factors:
    /// - Developmental level
    /// - Polarity intensity
    /// - Veil transparency
    ///
    /// # Arguments
    /// * `entity_id` - The ID of the entity tapping
    /// * `free_will_capacity` - The entity's free will capacity (0.0 to 1.0)
    /// * `developmental_level` - The entity's developmental level (0.0 to 1.0)
    /// * `polarization_intensity` - The entity's polarization intensity (0.0 to 1.0)
    /// * `veil_transparency` - The entity's veil transparency (0.0 to 1.0)
    ///
    /// # Returns
    /// The amount of energy tapped (0.0 to 1.0)
    pub fn tap_advanced(
        &mut self,
        entity_id: HolonID,
        free_will_capacity: Float,
        developmental_level: Float,
        polarization_intensity: Float,
        veil_transparency: Float,
    ) -> Float {
        // Base tap strength
        let base_tap = free_will_capacity * self.kinetic;

        // Developmental modifier (more developed = can tap more)
        let developmental_modifier = 1.0 + developmental_level * 0.5;

        // Polarity modifier (strong polarization = can tap more)
        let polarization_modifier = 1.0 + polarization_intensity * 0.3;

        // Veil modifier (thinner veil = can tap more)
        let veil_modifier = 1.0 + veil_transparency * 0.4;

        // Combined tap strength
        let tap_strength =
            base_tap * developmental_modifier * polarization_modifier * veil_modifier;

        // Clamp to valid range
        let tap_strength = tap_strength.min(1.0);

        // Record tap
        self.free_will_taps.insert(entity_id, tap_strength);
        self.total_tapped_energy += tap_strength;

        tap_strength
    }

    /// Get current pulse state
    ///
    /// Returns information about the current pulse cycle.
    pub fn get_pulse_state(&self) -> PulseState {
        PulseState {
            phase: self.pulse_phase,
            kinetic: self.kinetic,
            potential: self.potential,
            direction: if self.pulse_phase.cos() > 0.0 {
                PulseDirection::PotentialToKinetic
            } else {
                PulseDirection::KineticToPotential
            },
            cycle_count: self.pulse_count,
        }
    }

    /// Get tap strength for a specific entity
    pub fn get_tap_strength(&self, entity_id: HolonID) -> Option<Float> {
        self.free_will_taps.get(&entity_id).copied()
    }

    /// Get total tapped energy in current cycle
    pub fn get_total_tapped_energy(&self) -> Float {
        self.total_tapped_energy
    }

    /// Get number of entities currently tapping
    pub fn get_tapping_entity_count(&self) -> usize {
        self.free_will_taps.len()
    }

    /// Get average tap strength across all entities
    pub fn get_average_tap_strength(&self) -> Float {
        if self.free_will_taps.is_empty() {
            0.0
        } else {
            self.total_tapped_energy / self.free_will_taps.len() as Float
        }
    }

    /// Reset the field (for testing purposes)
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.pulse_phase = 0.0;
        self.kinetic = 0.5;
        self.free_will_taps.clear();
        self.total_tapped_energy = 0.0;
        self.pulse_count = 0;
    }
}

/// Pulse State
///
/// Represents the current state of the pulse cycle.
#[derive(Debug, Clone, Copy)]
pub struct PulseState {
    /// Current phase (0.0 to 2π)
    pub phase: Float,

    /// Current kinetic energy (0.0 to 1.0)
    pub kinetic: Float,

    /// Current potential energy (always 1.0)
    pub potential: Float,

    /// Pulse direction
    pub direction: PulseDirection,

    /// Pulse cycle count
    pub cycle_count: u64,
}

/// Pulse Direction
///
/// Indicates whether the pulse is moving from potential to kinetic or vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseDirection {
    /// Moving from potential to kinetic (manifestation)
    PotentialToKinetic,

    /// Moving from kinetic to potential (involution)
    KineticToPotential,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intelligent_infinity_creation() {
        let ii_field = IntelligentInfinity::new(0.1);

        assert_eq!(ii_field.potential, 1.0);
        assert_eq!(ii_field.kinetic, 0.5);
        assert_eq!(ii_field.pulse_phase, 0.0);
        assert_eq!(ii_field.pulse_frequency, 0.1);
        assert_eq!(ii_field.pulse_count, 0);
        assert_eq!(ii_field.total_tapped_energy, 0.0);
        assert_eq!(ii_field.get_tapping_entity_count(), 0);
    }

    #[test]
    fn test_intelligent_infinity_with_default_frequency() {
        let ii_field = IntelligentInfinity::with_default_frequency();

        assert_eq!(ii_field.pulse_frequency, 0.1);
    }

    #[test]
    fn test_pulse() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        let initial_kinetic = ii_field.kinetic;
        ii_field.pulse();

        // Kinetic should have changed
        assert_ne!(ii_field.kinetic, initial_kinetic);

        // Kinetic should be in valid range
        assert!(ii_field.kinetic >= 0.0 && ii_field.kinetic <= 1.0);

        // Phase should have advanced
        assert_eq!(ii_field.pulse_phase, 0.1);
    }

    #[test]
    fn test_multiple_pulses() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        for _ in 0..10 {
            ii_field.pulse();
        }

        // Phase should have advanced 10 times (approximately due to floating point)
        assert!(
            (ii_field.pulse_phase - 1.0).abs() < 0.0001,
            "Expected ≈1.0, got {}",
            ii_field.pulse_phase
        );

        // Kinetic should still be in valid range
        assert!(ii_field.kinetic >= 0.0 && ii_field.kinetic <= 1.0);
    }

    #[test]
    fn test_pulse_cycle_completion() {
        let mut ii_field = IntelligentInfinity::new(0.5); // Faster frequency

        // Pulse enough to complete one cycle
        for _ in 0..13 {
            ii_field.pulse();
        }

        // Phase should wrap around
        assert!(ii_field.pulse_phase < std::f64::consts::PI * 2.0);

        // Pulse count should be at least 1
        assert!(ii_field.pulse_count >= 1);
    }

    #[test]
    fn test_tap() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // Tap with 50% capacity
        let energy = ii_field.tap(1, 0.5);

        // Energy should be positive
        assert!(energy > 0.0);

        // Energy should be <= 0.5 (capacity × kinetic)
        assert!(energy <= 0.5);

        // Should have recorded tap
        assert_eq!(ii_field.get_tapping_entity_count(), 1);
        assert_eq!(ii_field.get_tap_strength(1), Some(energy));
    }

    #[test]
    fn test_tap_with_zero_capacity() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        let energy = ii_field.tap(1, 0.0);

        assert_eq!(energy, 0.0);
    }

    #[test]
    fn test_tap_with_full_capacity() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        let energy = ii_field.tap(1, 1.0);

        // Energy should be close to kinetic (0.5 at start)
        assert!((energy - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_multiple_taps() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        ii_field.tap(1, 0.5);
        ii_field.tap(2, 0.7);
        ii_field.tap(3, 0.3);

        assert_eq!(ii_field.get_tapping_entity_count(), 3);
        assert!(ii_field.get_total_tapped_energy() > 0.0);
    }

    #[test]
    fn test_tap_advanced() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        let energy = ii_field.tap_advanced(
            1, 0.5, // free_will_capacity
            0.8, // developmental_level
            0.9, // polarization_intensity
            0.7, // veil_transparency
        );

        // Energy should be positive
        assert!(energy > 0.0);

        // Energy should be clamped to 1.0
        assert!(energy <= 1.0);
    }

    #[test]
    fn test_tap_advanced_modifiers() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // Base tap
        let base_energy = ii_field.tap(1, 0.5);

        // Advanced tap with modifiers
        let advanced_energy = ii_field.tap_advanced(
            2, 0.5, // same free_will_capacity
            0.8, // high developmental_level
            0.9, // high polarization_intensity
            0.7, // high veil_transparency
        );

        // Advanced tap should be higher due to modifiers
        assert!(advanced_energy > base_energy);
    }

    #[test]
    fn test_get_pulse_state() {
        let ii_field = IntelligentInfinity::with_default_frequency();

        let state = ii_field.get_pulse_state();

        assert_eq!(state.phase, 0.0);
        assert_eq!(state.kinetic, 0.5);
        assert_eq!(state.potential, 1.0);
        assert_eq!(state.direction, PulseDirection::PotentialToKinetic);
        assert_eq!(state.cycle_count, 0);
    }

    #[test]
    fn test_pulse_state_changes() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        ii_field.pulse();

        let state = ii_field.get_pulse_state();

        assert_eq!(state.phase, 0.1);
        assert!(state.kinetic > 0.5); // Should be increasing
        assert_eq!(state.direction, PulseDirection::PotentialToKinetic);
    }

    #[test]
    fn test_pulse_direction_changes() {
        let mut ii_field = IntelligentInfinity::new(0.5);

        // Pulse to quarter cycle (PI/2) - need 3-4 pulses with frequency 0.5
        for _ in 0..3 {
            ii_field.pulse();
        }

        let state = ii_field.get_pulse_state();
        assert_eq!(state.direction, PulseDirection::PotentialToKinetic);

        // Pulse to three-quarter cycle (3*PI/2) - add 6 more pulses
        for _ in 0..6 {
            ii_field.pulse();
        }

        let state = ii_field.get_pulse_state();
        assert_eq!(state.direction, PulseDirection::KineticToPotential);
    }

    #[test]
    fn test_get_average_tap_strength() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // No taps yet
        assert_eq!(ii_field.get_average_tap_strength(), 0.0);

        // Add some taps
        ii_field.tap(1, 0.5);
        ii_field.tap(2, 0.7);

        let avg = ii_field.get_average_tap_strength();
        assert!(avg > 0.0);
        assert!(avg < 0.7); // Average should be between 0.5 and 0.7
    }

    #[test]
    fn test_reset() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // Do some operations
        ii_field.pulse();
        ii_field.tap(1, 0.5);

        // Reset
        ii_field.reset();

        assert_eq!(ii_field.pulse_phase, 0.0);
        assert_eq!(ii_field.kinetic, 0.5);
        assert_eq!(ii_field.get_tapping_entity_count(), 0);
        assert_eq!(ii_field.total_tapped_energy, 0.0);
        assert_eq!(ii_field.pulse_count, 0);
    }

    #[test]
    fn test_kinetic_never_exceeds_bounds() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // Pulse many times
        for _ in 0..1000 {
            ii_field.pulse();
            assert!(ii_field.kinetic >= 0.0);
            assert!(ii_field.kinetic <= 1.0);
        }
    }

    #[test]
    fn test_potential_always_one() {
        let mut ii_field = IntelligentInfinity::with_default_frequency();

        // Potential should never change
        assert_eq!(ii_field.potential, 1.0);

        ii_field.pulse();
        assert_eq!(ii_field.potential, 1.0);

        ii_field.tap(1, 0.5);
        assert_eq!(ii_field.potential, 1.0);
    }
}
