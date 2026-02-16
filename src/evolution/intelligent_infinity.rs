//! Enhanced Intelligent-Infinity with Active Feedback
//!
//! Enhances the existing Intelligent-Infinity field to be an active participant
//! in the evolution process, collecting feedback from entities and analyzing patterns.
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
//! "Feedback loop: entities → Intelligent-Infinity"
//! "Active Intelligent-Infinity with learning"
//!
//! ## Key Insights
//!
//! **Passive Intelligent-Infinity** (Before):
//! - Continuous pulsing field
//! - Entities tap energy based on free will capacity
//! - No feedback from entities
//! - Static, unchanging field
//!
//! **Active Intelligent-Infinity** (After):
//! - Continuous pulsing field
//! - Collects feedback from entities
//! - Analyzes patterns in entity behavior
//! - Emits teleological pull toward source
//! - Adapts based on collective evolution
//!
//! ## Feedback Loop
//!
//! 1. Entities provide feedback on attractor effectiveness
//! 2. Intelligent-Infinity collects and analyzes feedback
//! 3. Patterns emerge (what works, what doesn't)
//! 4. Intelligent-Infinity emits teleological pull
//! 5. Entities receive guidance toward purpose
//! 6. Loop continues, field evolves with entities
//!
//! ## Teleological Pull
//!
//! Intelligent-Infinity emits a subtle pull that guides entities toward:
//! - Return to source (oneness)
//! - Service orientation (STO or STS)
//! - Coherence with purpose
//! - Wisdom integration
//!
//! This pull is not coercive; it's an invitation that entities can accept
//! or reject based on their free will.

use crate::entity_layer7::layer7::EntityId;
use crate::evolution::adaptive_attractor::EntityFeedback;
use crate::types::Float;

/// Feedback Analysis - patterns detected in entity feedback
///
/// Summarizes what Intelligent-Infinity has learned from entity feedback.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
/// "Learning system: attractors adjust based on entity choices"
#[derive(Debug, Clone)]
pub struct FeedbackAnalysis {
    /// Number of entities providing feedback
    pub entity_count: usize,

    /// Average effectiveness of attractors (0.0 to 1.0)
    pub average_effectiveness: f64,

    /// Average alignment with attractors (0.0 to 1.0)
    pub average_alignment: f64,

    /// Average evolution progress (0.0 to 1.0)
    pub average_progress: f64,

    /// Dominant service orientation (-1.0 to 1.0)
    pub service_orientation_bias: f64,

    /// Overall collective teleological progress (0.0 to 1.0)
    pub collective_progress: f64,

    /// Is the collective making meaningful progress?
    pub is_collective_progress_meaningful: bool,
}

impl FeedbackAnalysis {
    /// Create new feedback analysis
    pub fn new(
        entity_count: usize,
        average_effectiveness: f64,
        average_alignment: f64,
        average_progress: f64,
        service_orientation_bias: f64,
        collective_progress: f64,
    ) -> Self {
        let is_collective_progress_meaningful = collective_progress > 0.5;

        FeedbackAnalysis {
            entity_count,
            average_effectiveness: average_effectiveness.clamp(0.0, 1.0),
            average_alignment: average_alignment.clamp(0.0, 1.0),
            average_progress: average_progress.clamp(0.0, 1.0),
            service_orientation_bias: service_orientation_bias.clamp(-1.0, 1.0),
            collective_progress: collective_progress.clamp(0.0, 1.0),
            is_collective_progress_meaningful,
        }
    }
}

/// Enhanced Intelligent Infinity Field
///
/// An enhanced version of the Intelligent-Infinity field that actively
/// collects feedback from entities and analyzes patterns to guide evolution.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
/// "Active Intelligent-Infinity with learning"
/// "Entities provide feedback on attractor effectiveness"
///
/// ## Enhanced Features
///
/// 1. **Feedback Collection**: Receives feedback from entities about attractor effectiveness
/// 2. **Pattern Analysis**: Detects patterns in collective entity behavior
/// 3. **Teleological Emission**: Emits pull toward source (return to purpose)
/// 4. **Learning**: Adapts based on collective evolution patterns
///
/// ## Original Features (Preserved)
///
/// 1. **Continuous Pulsing**: Field pulses rhythmically from potential to kinetic
/// 2. **Energy Tapping**: Entities can tap energy based on free will capacity
/// 3. **Infinite Potential**: Always contains infinite potential (potential = 1.0)
#[derive(Debug, Clone)]
pub struct IntelligentInfinity {
    // === Original Intelligent-Infinity fields ===
    /// Unmanifest source (always 1.0 - infinite potential)
    pub potential: Float,

    /// Manifest energy (pulses 0.0 ↔ 1.0)
    pub kinetic: Float,

    /// Current phase in pulse cycle (0.0 to 2π)
    pub pulse_phase: Float,

    /// Rhythmic flow rate (determines pulse frequency)
    pub pulse_frequency: Float,

    /// Total energy tapped in current cycle
    pub total_tapped_energy: Float,

    /// Pulse count (for tracking cycles)
    pub pulse_count: u64,

    // === Enhanced fields for Phase 3 ===
    /// Feedback from entities about attractor effectiveness
    pub entity_feedback_collector: Vec<EntityFeedback>,

    /// Latest analysis of feedback patterns
    pub latest_analysis: Option<FeedbackAnalysis>,

    /// Teleological emission strength (pull toward source)
    ///
    /// Higher values = stronger pull toward return to source
    pub teleological_emission: f64,

    /// Learning rate for teleological emission adjustment
    teleological_learning_rate: f64,

    /// Maximum feedback history to keep
    max_feedback_history: usize,
}

impl IntelligentInfinity {
    /// Create a new Intelligent Infinity field with active feedback
    ///
    /// # Arguments
    /// * `pulse_frequency` - The frequency of the rhythmic pulse (default: 0.1)
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::IntelligentInfinity;
    ///
    /// let ii = IntelligentInfinity::new(0.1);
    /// assert_eq!(ii.potential, 1.0);
    /// assert_eq!(ii.kinetic, 0.5);
    /// assert_eq!(ii.teleological_emission, 0.5);
    /// ```
    pub fn new(pulse_frequency: Float) -> Self {
        let pulse_phase = 0.0_f64;
        let kinetic = (pulse_phase.sin() + 1.0) / 2.0;

        IntelligentInfinity {
            potential: 1.0,
            kinetic,
            pulse_phase,
            pulse_frequency,
            total_tapped_energy: 0.0,
            pulse_count: 0,

            // Enhanced fields
            entity_feedback_collector: Vec::new(),
            latest_analysis: None,
            teleological_emission: 0.5, // Default pull strength
            teleological_learning_rate: 0.1,
            max_feedback_history: 10000,
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
    /// use holonic_realms::evolution::IntelligentInfinity;
    ///
    /// let mut ii = IntelligentInfinity::with_default_frequency();
    /// ii.pulse();
    ///
    /// assert!(ii.kinetic >= 0.0 && ii.kinetic <= 1.0);
    /// ```
    pub fn pulse(&mut self) {
        // Advance phase
        self.pulse_phase += self.pulse_frequency;

        // Track pulse cycles
        if self.pulse_phase >= (std::f64::consts::PI * 2.0) {
            self.pulse_phase -= std::f64::consts::PI * 2.0;
            self.pulse_count += 1;

            // Reset tapped energy for new cycle
            self.total_tapped_energy = 0.0;

            // Periodically analyze feedback and adjust teleological emission
            if self.pulse_count % 10 == 0 {
                self.analyze_feedback_patterns();
                self.adjust_teleological_emission();
            }
        }

        // Calculate kinetic energy (pulses 0.0 ↔ 1.0)
        self.kinetic = (self.pulse_phase.sin() + 1.0) / 2.0;
    }

    /// Tap into Intelligent Infinity through free will
    ///
    /// Entities can tap into the Intelligent Infinity field based on their free will capacity.
    ///
    /// # Arguments
    /// * `entity_id` - The ID of the entity tapping
    /// * `free_will_capacity` - The entity's free will capacity (0.0 to 1.0)
    ///
    /// # Returns
    /// The amount of energy tapped (0.0 to 1.0)
    pub fn tap(&mut self, entity_id: EntityId, free_will_capacity: Float) -> Float {
        // Amount tapped = free will capacity × kinetic energy
        let tap_strength = free_will_capacity * self.kinetic;
        self.total_tapped_energy += tap_strength;

        tap_strength
    }

    /// Tap with additional modifiers
    ///
    /// Advanced tap method that considers additional factors.
    pub fn tap_advanced(
        &mut self,
        entity_id: EntityId,
        free_will_capacity: Float,
        developmental_level: Float,
        polarization_intensity: Float,
        veil_transparency: Float,
    ) -> Float {
        let base_tap = free_will_capacity * self.kinetic;
        let developmental_modifier = 1.0 + developmental_level * 0.5;
        let polarization_modifier = 1.0 + polarization_intensity * 0.3;
        let veil_modifier = 1.0 + veil_transparency * 0.4;

        let tap_strength =
            base_tap * developmental_modifier * polarization_modifier * veil_modifier;
        let tap_strength = tap_strength.min(1.0);

        self.total_tapped_energy += tap_strength;

        tap_strength
    }

    /// Receive entity feedback about attractor effectiveness
    ///
    /// This is the core enhancement for Phase 3. Intelligent-Infinity
    /// actively collects feedback from entities to learn and adapt.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
    /// "Feedback loop: entities → Intelligent-Infinity"
    ///
    /// # Arguments
    /// * `feedback` - Feedback from entity about attractor effectiveness
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::{IntelligentInfinity, EntityFeedback};
    /// use holonic_realms::entity_layer7::layer7::EntityId;
    ///
    /// let mut ii = IntelligentInfinity::with_default_frequency();
    /// let feedback = EntityFeedback::new(
    ///     EntityId::new("entity-1".to_string()),
    ///     0.7,
    ///     0.5,
    ///     0.8,
    /// );
    /// ii.receive_entity_feedback(feedback);
    ///
    /// assert_eq!(ii.entity_feedback_collector.len(), 1);
    /// ```
    pub fn receive_entity_feedback(&mut self, feedback: EntityFeedback) {
        self.entity_feedback_collector.push(feedback);

        // Limit history size
        if self.entity_feedback_collector.len() > self.max_feedback_history {
            self.entity_feedback_collector.remove(0);
        }
    }

    /// Analyze feedback patterns from all entities
    ///
    /// Detects patterns in collective entity behavior to understand what's
    /// working and what isn't in the evolution process.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
    /// "Learning system: attractors adjust based on entity choices"
    ///
    /// # Returns
    /// FeedbackAnalysis with detected patterns
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::{IntelligentInfinity, EntityFeedback};
    /// use holonic_realms::entity_layer7::layer7::EntityId;
    ///
    /// let mut ii = IntelligentInfinity::with_default_frequency();
    ///
    /// // Add some feedback
    /// for i in 0..10 {
    ///     let feedback = EntityFeedback::new(
    ///         EntityId::new(format!("entity-{}", i)),
    ///         0.7,
    ///         0.5 + (i as f64 * 0.05),
    ///         0.8,
    ///     );
    ///     ii.receive_entity_feedback(feedback);
    /// }
    ///
    /// let analysis = ii.analyze_feedback_patterns();
    /// assert!(analysis.entity_count > 0);
    /// ```
    pub fn analyze_feedback_patterns(&mut self) -> FeedbackAnalysis {
        if self.entity_feedback_collector.is_empty() {
            // Return neutral analysis if no feedback
            self.latest_analysis = Some(FeedbackAnalysis::new(0, 0.5, 0.5, 0.5, 0.0, 0.5));
            return self.latest_analysis.clone().unwrap();
        }

        // Calculate averages
        let entity_count = self.entity_feedback_collector.len();

        let total_effectiveness: f64 = self
            .entity_feedback_collector
            .iter()
            .map(|f| f.calculate_effectiveness())
            .sum();

        let average_effectiveness = total_effectiveness / entity_count as f64;

        let total_alignment: f64 = self
            .entity_feedback_collector
            .iter()
            .map(|f| f.alignment_with_attractor)
            .sum();

        let average_alignment = total_alignment / entity_count as f64;

        let total_progress: f64 = self
            .entity_feedback_collector
            .iter()
            .map(|f| f.evolution_progress)
            .sum();

        let average_progress = total_progress / entity_count as f64;

        // Calculate service orientation bias
        // (This would normally come from entity polarization, using alignment as proxy)
        let service_orientation_bias = (average_alignment - 0.5) * 2.0; // -1.0 to 1.0

        // Calculate collective progress
        let collective_progress =
            (average_effectiveness + average_alignment + average_progress) / 3.0;

        let analysis = FeedbackAnalysis::new(
            entity_count,
            average_effectiveness,
            average_alignment,
            average_progress,
            service_orientation_bias,
            collective_progress,
        );

        self.latest_analysis = Some(analysis.clone());

        analysis
    }

    /// Adjust teleological emission based on feedback analysis
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
    /// "Teleological pull toward source"
    ///
    /// The teleological emission (pull toward source) is adjusted based on:
    /// - Collective progress (higher progress = stronger pull)
    /// - Average alignment (higher alignment = stronger pull)
    /// - Service orientation bias (influences pull direction)
    pub fn adjust_teleological_emission(&mut self) {
        let analysis = match &self.latest_analysis {
            Some(a) => a.clone(),
            None => return,
        };

        // Base adjustment from collective progress
        let progress_adjustment =
            (analysis.collective_progress - 0.5) * self.teleological_learning_rate;

        // Modulation from average alignment
        let alignment_modulation =
            (analysis.average_alignment - 0.5) * 0.5 * self.teleological_learning_rate;

        // Combine adjustments
        let adjustment = progress_adjustment + alignment_modulation;

        // Apply adjustment
        self.teleological_emission += adjustment;

        // Clamp to valid range (0.0 to 1.0)
        self.teleological_emission = self.teleological_emission.clamp(0.0, 1.0);
    }

    /// Emit teleological pull toward source
    ///
    /// Returns the strength of the pull toward Intelligent-Infinity (return to source).
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
    /// "Emit teleological pull toward source"
    ///
    /// # Returns
    /// Teleological pull strength (0.0 to 1.0)
    pub fn emit_teleological_pull(&self) -> f64 {
        self.teleological_emission
    }

    /// Get current pulse state
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

    /// Get feedback count
    pub fn feedback_count(&self) -> usize {
        self.entity_feedback_collector.len()
    }

    /// Reset the field (for testing purposes)
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.pulse_phase = 0.0;
        self.kinetic = 0.5;
        self.total_tapped_energy = 0.0;
        self.pulse_count = 0;
        self.entity_feedback_collector.clear();
        self.latest_analysis = None;
        self.teleological_emission = 0.5;
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
        let ii = IntelligentInfinity::new(0.1);

        assert_eq!(ii.potential, 1.0);
        assert_eq!(ii.kinetic, 0.5);
        assert_eq!(ii.pulse_phase, 0.0);
        assert_eq!(ii.pulse_frequency, 0.1);
        assert_eq!(ii.teleological_emission, 0.5);
        assert_eq!(ii.feedback_count(), 0);
    }

    #[test]
    fn test_intelligent_infinity_with_default_frequency() {
        let ii = IntelligentInfinity::with_default_frequency();

        assert_eq!(ii.pulse_frequency, 0.1);
    }

    #[test]
    fn test_pulse() {
        let mut ii = IntelligentInfinity::with_default_frequency();

        let initial_kinetic = ii.kinetic;
        ii.pulse();

        assert_ne!(ii.kinetic, initial_kinetic);
        assert!(ii.kinetic >= 0.0 && ii.kinetic <= 1.0);
    }

    #[test]
    fn test_tap() {
        let mut ii = IntelligentInfinity::with_default_frequency();
        let entity_id = EntityId::new("test-entity".to_string());

        let energy = ii.tap(entity_id, 0.5);

        assert!(energy > 0.0);
        assert!(energy <= 0.5);
    }

    #[test]
    fn test_receive_entity_feedback() {
        let mut ii = IntelligentInfinity::with_default_frequency();
        let entity_id = EntityId::new("test-entity".to_string());

        let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
        ii.receive_entity_feedback(feedback);

        assert_eq!(ii.feedback_count(), 1);
    }

    #[test]
    fn test_feedback_history_limit() {
        let mut ii = IntelligentInfinity::with_default_frequency();
        ii.max_feedback_history = 10;

        for i in 0..20 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
            ii.receive_entity_feedback(feedback);
        }

        assert_eq!(ii.feedback_count(), 10);
    }

    #[test]
    fn test_analyze_feedback_patterns_no_feedback() {
        let mut ii = IntelligentInfinity::with_default_frequency();

        let analysis = ii.analyze_feedback_patterns();

        assert_eq!(analysis.entity_count, 0);
        assert_eq!(analysis.average_effectiveness, 0.5);
    }

    #[test]
    fn test_analyze_feedback_patterns_with_feedback() {
        let mut ii = IntelligentInfinity::with_default_frequency();

        for i in 0..10 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.7, 0.5 + (i as f64 * 0.05), 0.8);
            ii.receive_entity_feedback(feedback);
        }

        let analysis = ii.analyze_feedback_patterns();

        assert_eq!(analysis.entity_count, 10);
        assert!(analysis.average_effectiveness > 0.0);
        assert!(analysis.average_progress > 0.0);
    }

    #[test]
    fn test_adjust_teleological_emission() {
        let mut ii = IntelligentInfinity::with_default_frequency();
        let initial_emission = ii.teleological_emission;

        // Add positive feedback
        for i in 0..10 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9);
            ii.receive_entity_feedback(feedback);
        }

        ii.analyze_feedback_patterns();
        ii.adjust_teleological_emission();

        // Emission should have changed
        assert_ne!(ii.teleological_emission, initial_emission);
    }

    #[test]
    fn test_emit_teleological_pull() {
        let ii = IntelligentInfinity::with_default_frequency();

        let pull = ii.emit_teleological_pull();

        assert!(pull >= 0.0);
        assert!(pull <= 1.0);
    }

    #[test]
    fn test_get_pulse_state() {
        let ii = IntelligentInfinity::with_default_frequency();

        let state = ii.get_pulse_state();

        assert_eq!(state.phase, 0.0);
        assert_eq!(state.kinetic, 0.5);
        assert_eq!(state.potential, 1.0);
        assert_eq!(state.cycle_count, 0);
    }

    #[test]
    fn test_reset() {
        let mut ii = IntelligentInfinity::with_default_frequency();

        ii.pulse();
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
        ii.receive_entity_feedback(feedback);
        ii.analyze_feedback_patterns();

        ii.reset();

        assert_eq!(ii.pulse_phase, 0.0);
        assert_eq!(ii.kinetic, 0.5);
        assert_eq!(ii.feedback_count(), 0);
        assert_eq!(ii.teleological_emission, 0.5);
    }
}
