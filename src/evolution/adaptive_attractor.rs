//! Adaptive Attractor Fields
//!
//! Implements dynamic attractor fields that adjust based on entity feedback,
//! replacing static mechanical attractors with intelligent learning attractors.
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
//! "Feedback loop: entities → Intelligent-Infinity"
//! "Adaptive attractor fields (not static)"
//! "Learning system: attractors adjust based on entity choices"
//!
//! ## Key Insights
//!
//! **Mechanical (Static) Attractors**:
//! - Fixed strength values (e.g., attractor_strength = 0.3)
//! - No learning or adaptation
//! - One-size-fits-all approach
//! - Evolution is random walk around fixed point
//!
//! **Intelligent (Adaptive) Attractors**:
//! - Dynamic strength that adjusts based on feedback
//! - Learning system that responds to entity behavior
//! - Personalized guidance for each entity
//! - Evolution is guided toward optimal paths
//!
//! ## Feedback Mechanism
//!
//! 1. Entity experiences attractor pull (how strongly attracted)
//! 2. Entity makes choice and evolves
//! 3. Entity provides feedback on effectiveness
//! 4. Attractor adjusts strength based on effectiveness
//! 5. Next entity gets improved guidance
//!
//! ## Effectiveness Calculation
//!
//! Effectiveness = evolution_progress / attractor_pull
//!
//! - High effectiveness (> 0.7): Attractor working well → strengthen
//! - Low effectiveness (< 0.7): Attractor not working → weaken and adjust
//!
//! This creates a learning loop where the attractor field evolves alongside
//! the entities it guides.

use crate::entity_layer7::layer7::EntityId;
use crate::types::Density;
use std::time::{SystemTime, UNIX_EPOCH};

/// Entity Feedback - response from entity about attractor effectiveness
///
/// Each entity provides feedback on how well the attractor field guided it.
/// This feedback is used to adjust the attractor's strength for future entities.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Entities provide feedback on attractor effectiveness"
#[derive(Debug, Clone)]
pub struct EntityFeedback {
    /// Entity providing feedback
    pub entity_id: EntityId,

    /// How strongly the attractor pulled this entity (0.0 to 1.0)
    ///
    /// This represents the intensity of the "spiritual gravity" the entity experienced.
    pub attractor_pull: f64,

    /// Actual evolution progress made (0.0 to 1.0)
    ///
    /// This is the measurable advancement in the entity's evolution.
    pub evolution_progress: f64,

    /// How well aligned the entity's choice was with attractor direction (0.0 to 1.0)
    ///
    /// Higher values indicate the entity followed the attractor's guidance.
    pub alignment_with_attractor: f64,

    /// When this feedback was provided (UNIX timestamp)
    pub timestamp: u64,
}

impl EntityFeedback {
    /// Create new entity feedback
    ///
    /// # Arguments
    /// * `entity_id` - Entity providing feedback
    /// * `attractor_pull` - Strength of attraction experienced (0.0 to 1.0)
    /// * `evolution_progress` - Actual progress made (0.0 to 1.0)
    /// * `alignment_with_attractor` - Alignment with attractor guidance (0.0 to 1.0)
    pub fn new(
        entity_id: EntityId,
        attractor_pull: f64,
        evolution_progress: f64,
        alignment_with_attractor: f64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        EntityFeedback {
            entity_id,
            attractor_pull: attractor_pull.clamp(0.0, 1.0),
            evolution_progress: evolution_progress.clamp(0.0, 1.0),
            alignment_with_attractor: alignment_with_attractor.clamp(0.0, 1.0),
            timestamp,
        }
    }

    /// Calculate effectiveness of attractor for this feedback
    ///
    /// Effectiveness = evolution_progress / attractor_pull
    ///
    /// Higher values indicate the attractor was effective at guiding evolution.
    pub fn calculate_effectiveness(&self) -> f64 {
        if self.attractor_pull <= 0.0 {
            return 0.0;
        }
        self.evolution_progress / self.attractor_pull
    }
}

/// Feedback Effectiveness - summary of how well attractor is working
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Adjust strength based on effectiveness > 0.7 (strengthen) or < 0.7 (weaken)"
#[derive(Debug, Clone, Copy)]
pub struct FeedbackEffectiveness {
    /// Average effectiveness across all feedback (0.0 to 1.0)
    pub average_effectiveness: f64,

    /// Number of feedback samples
    pub sample_count: usize,

    /// Is attractor working well (effectiveness > 0.7)?
    pub is_effective: bool,

    /// Confidence in effectiveness assessment (0.0 to 1.0)
    /// Higher with more samples and consistent feedback
    pub confidence: f64,
}

impl FeedbackEffectiveness {
    /// Create new effectiveness assessment
    pub fn new(average_effectiveness: f64, sample_count: usize) -> Self {
        let is_effective = average_effectiveness > 0.7;
        let confidence = (sample_count as f64 / (sample_count as f64 + 10.0)).min(1.0);

        FeedbackEffectiveness {
            average_effectiveness,
            sample_count,
            is_effective,
            confidence,
        }
    }
}

/// Adaptive Attractor Field
///
/// A dynamic attractor field that learns and adjusts based on entity feedback.
/// Unlike static attractors with fixed strength, adaptive attractors evolve
/// alongside the entities they guide.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 9-10:
/// "Adaptive attractor fields (not static)"
/// "Learning system: attractors adjust based on entity choices"
///
/// ## Before (Mechanical, Static):
/// ```rust
/// pub struct DensityAttractorField {
///     pub attractor_strength: f64 = 0.3,  // Fixed!
///     pub attractor_range: f64 = 0.3,
/// }
/// ```
///
/// ## After (Intelligent, Adaptive):
/// ```rust
/// pub struct AdaptiveAttractorField {
///     pub base_strength: f64,
///     pub current_strength: f64,        // Adjusts based on feedback
///     pub learning_rate: f64,
///     pub entity_feedback_history: Vec<Feedback>,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AdaptiveAttractorField {
    /// Target density this attractor guides toward
    pub target_density: Density,

    /// Base strength (initial strength before learning)
    pub base_strength: f64,

    /// Current strength (adjusted based on feedback)
    ///
    /// This value changes over time as the attractor learns from entity feedback.
    pub current_strength: f64,

    /// Learning rate (how quickly to adjust strength)
    ///
    /// Higher values = faster learning but potentially unstable
    /// Lower values = slower learning but more stable
    pub learning_rate: f64,

    /// History of feedback from entities
    pub entity_feedback_history: Vec<EntityFeedback>,

    /// Maximum feedback history to keep
    max_history_size: usize,

    /// Minimum strength clamp
    min_strength: f64,

    /// Maximum strength clamp
    max_strength: f64,
}

impl AdaptiveAttractorField {
    /// Create a new adaptive attractor field
    ///
    /// # Arguments
    /// * `target_density` - Density this attractor guides toward
    /// * `base_strength` - Initial strength (0.0 to 1.0)
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::AdaptiveAttractorField;
    /// use holonic_realms::types::Density;
    ///
    /// let attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
    /// assert_eq!(attractor.target_density, Density::Third);
    /// assert_eq!(attractor.current_strength, 0.3);
    /// ```
    pub fn new(target_density: Density, base_strength: f64) -> Self {
        let base_strength = base_strength.clamp(0.0, 1.0);

        AdaptiveAttractorField {
            target_density,
            base_strength,
            current_strength: base_strength,
            learning_rate: 0.1, // Default learning rate
            entity_feedback_history: Vec::new(),
            max_history_size: 1000, // Keep last 1000 feedback entries
            min_strength: 0.05,
            max_strength: 1.0,
        }
    }

    /// Create with custom learning rate
    ///
    /// # Arguments
    /// * `target_density` - Density this attractor guides toward
    /// * `base_strength` - Initial strength (0.0 to 1.0)
    /// * `learning_rate` - How quickly to adjust (0.0 to 1.0)
    pub fn with_learning_rate(
        target_density: Density,
        base_strength: f64,
        learning_rate: f64,
    ) -> Self {
        let base_strength = base_strength.clamp(0.0, 1.0);
        let learning_rate = learning_rate.clamp(0.01, 1.0);

        let mut attractor = Self::new(target_density, base_strength);
        attractor.learning_rate = learning_rate;
        attractor
    }

    /// Receive feedback from an entity
    ///
    /// This method stores the feedback for later analysis and strength adjustment.
    /// Call `adjust_strength()` to actually update the attractor strength.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "Entities provide feedback on attractor effectiveness"
    ///
    /// # Arguments
    /// * `feedback` - Feedback from entity about attractor effectiveness
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::{AdaptiveAttractorField, EntityFeedback};
    /// use holonic_realms::types::Density;
    /// use holonic_realms::entity_layer7::layer7::EntityId;
    ///
    /// let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
    /// let feedback = EntityFeedback::new(
    ///     EntityId::new("entity-1".to_string()),
    ///     0.7,
    ///     0.5,
    ///     0.8,
    /// );
    /// attractor.receive_feedback(feedback);
    /// assert_eq!(attractor.entity_feedback_history.len(), 1);
    /// ```
    pub fn receive_feedback(&mut self, feedback: EntityFeedback) {
        self.entity_feedback_history.push(feedback);

        // Limit history size
        if self.entity_feedback_history.len() > self.max_history_size {
            self.entity_feedback_history.remove(0);
        }
    }

    /// Calculate current effectiveness of the attractor
    ///
    /// Returns a summary of how well the attractor is working based on
    /// all feedback received so far.
    ///
    /// # Returns
    /// FeedbackEffectiveness summary
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::{AdaptiveAttractorField, EntityFeedback};
    /// use holonic_realms::types::Density;
    /// use holonic_realms::entity_layer7::layer7::EntityId;
    ///
    /// let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
    ///
    /// // Add some feedback
    /// for i in 0..10 {
    ///     let feedback = EntityFeedback::new(
    ///         EntityId::new(format!("entity-{}", i)),
    ///         0.7,
    ///         0.5 + (i as f64 * 0.05),
    ///         0.8,
    ///     );
    ///     attractor.receive_feedback(feedback);
    /// }
    ///
    /// let effectiveness = attractor.calculate_effectiveness();
    /// assert!(effectiveness.sample_count > 0);
    /// ```
    pub fn calculate_effectiveness(&self) -> FeedbackEffectiveness {
        if self.entity_feedback_history.is_empty() {
            return FeedbackEffectiveness::new(0.5, 0); // Neutral with no data
        }

        let total_effectiveness: f64 = self
            .entity_feedback_history
            .iter()
            .map(|f| f.calculate_effectiveness())
            .sum();

        let average_effectiveness = total_effectiveness / self.entity_feedback_history.len() as f64;

        FeedbackEffectiveness::new(average_effectiveness, self.entity_feedback_history.len())
    }

    /// Adjust attractor strength based on feedback
    ///
    /// This is the core learning mechanism. The attractor:
    /// - Strengthens if effectiveness > 0.7 (working well)
    /// - Weakens if effectiveness < 0.7 (not working well)
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "Adjust strength based on effectiveness > 0.7 (strengthen) or < 0.7 (weaken)"
    ///
    /// # Examples
    /// ```
    /// use holonic_realms::evolution::{AdaptiveAttractorField, EntityFeedback};
    /// use holonic_realms::types::Density;
    /// use holonic_realms::entity_layer7::layer7::EntityId;
    ///
    /// let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
    /// let initial_strength = attractor.current_strength;
    ///
    /// // Add positive feedback (attractor working well)
    /// for i in 0..5 {
    ///     let feedback = EntityFeedback::new(
    ///         EntityId::new(format!("entity-{}", i)),
    ///         0.5,
    ///         0.9, // High progress = high effectiveness
    ///         0.9,
    ///     );
    ///     attractor.receive_feedback(feedback);
    /// }
    ///
    /// attractor.adjust_strength();
    ///
    /// // Attractor should have strengthened
    /// assert!(attractor.current_strength > initial_strength);
    /// ```
    pub fn adjust_strength(&mut self) {
        let effectiveness = self.calculate_effectiveness();

        if effectiveness.sample_count == 0 {
            return; // No feedback to learn from
        }

        // Adjust strength based on effectiveness
        if effectiveness.is_effective {
            // Attractor working well - strengthen it
            let adjustment = self.learning_rate * effectiveness.average_effectiveness;
            self.current_strength += adjustment;
        } else {
            // Attractor not working - weaken it
            let adjustment = self.learning_rate * (1.0 - effectiveness.average_effectiveness);
            self.current_strength -= adjustment;
        }

        // Clamp to valid range
        self.current_strength = self
            .current_strength
            .clamp(self.min_strength, self.max_strength);
    }

    /// Get current pull strength for an entity
    ///
    /// Returns the strength of attraction that an entity would experience
    /// from this attractor field.
    ///
    /// # Returns
    /// Current pull strength (0.0 to 1.0)
    pub fn get_pull_strength(&self) -> f64 {
        self.current_strength
    }

    /// Reset to base strength
    ///
    /// Clears all learning and returns the attractor to its initial state.
    #[cfg(test)]
    pub fn reset(&mut self) {
        self.current_strength = self.base_strength;
        self.entity_feedback_history.clear();
    }

    /// Get feedback count
    pub fn feedback_count(&self) -> usize {
        self.entity_feedback_history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_attractor_creation() {
        let attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

        assert_eq!(attractor.target_density, Density::Third);
        assert_eq!(attractor.base_strength, 0.3);
        assert_eq!(attractor.current_strength, 0.3);
        assert_eq!(attractor.feedback_count(), 0);
    }

    #[test]
    fn test_adaptive_attractor_clamping() {
        let attractor = AdaptiveAttractorField::new(Density::Third, 1.5);
        assert_eq!(attractor.base_strength, 1.0); // Should clamp to 1.0

        let attractor = AdaptiveAttractorField::new(Density::Third, -0.5);
        assert_eq!(attractor.base_strength, 0.0); // Should clamp to 0.0
    }

    #[test]
    fn test_adaptive_attractor_with_learning_rate() {
        let attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.3, 0.2);

        assert_eq!(attractor.target_density, Density::Third);
        assert_eq!(attractor.current_strength, 0.3);
        assert_eq!(attractor.learning_rate, 0.2);
    }

    #[test]
    fn test_entity_feedback_creation() {
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id.clone(), 0.7, 0.5, 0.8);

        assert_eq!(feedback.entity_id, entity_id);
        assert_eq!(feedback.attractor_pull, 0.7);
        assert_eq!(feedback.evolution_progress, 0.5);
        assert_eq!(feedback.alignment_with_attractor, 0.8);
        assert!(feedback.timestamp > 0);
    }

    #[test]
    fn test_entity_feedback_clamping() {
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id.clone(), 1.5, -0.5, 2.0);

        assert_eq!(feedback.attractor_pull, 1.0); // Clamped
        assert_eq!(feedback.evolution_progress, 0.0); // Clamped
        assert_eq!(feedback.alignment_with_attractor, 1.0); // Clamped
    }

    #[test]
    fn test_receive_feedback() {
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
        let entity_id = EntityId::new("test-entity".to_string());

        let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
        attractor.receive_feedback(feedback);

        assert_eq!(attractor.feedback_count(), 1);
    }

    #[test]
    fn test_feedback_history_limit() {
        let mut attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.3, 0.1);
        attractor.max_history_size = 10; // Set small limit for testing

        // Add more feedback than limit
        for i in 0..20 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
            attractor.receive_feedback(feedback);
        }

        assert_eq!(attractor.feedback_count(), 10); // Should be limited
    }

    #[test]
    fn test_calculate_effectiveness_no_feedback() {
        let attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
        let effectiveness = attractor.calculate_effectiveness();

        assert_eq!(effectiveness.average_effectiveness, 0.5);
        assert_eq!(effectiveness.sample_count, 0);
        assert!(!effectiveness.is_effective);
    }

    #[test]
    fn test_calculate_effectiveness_with_feedback() {
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

        // Add feedback with known effectiveness
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id, 0.5, 0.5, 0.8); // Effectiveness = 1.0
        attractor.receive_feedback(feedback);

        let effectiveness = attractor.calculate_effectiveness();

        assert_eq!(effectiveness.sample_count, 1);
        assert_eq!(effectiveness.average_effectiveness, 1.0);
        assert!(effectiveness.is_effective);
    }

    #[test]
    fn test_effectiveness_threshold() {
        // Test the 0.7 threshold
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

        // Add feedback with effectiveness just above threshold
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8); // Effectiveness ≈ 0.714
        attractor.receive_feedback(feedback);

        let effectiveness = attractor.calculate_effectiveness();
        assert!(effectiveness.is_effective); // Should be > 0.7
    }

    #[test]
    fn test_adjust_strength_strengthens() {
        let mut attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.3, 0.2);
        let initial_strength = attractor.current_strength;

        // Add feedback showing attractor is working well
        for i in 0..5 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9); // High effectiveness
            attractor.receive_feedback(feedback);
        }

        attractor.adjust_strength();

        // Should have strengthened
        assert!(attractor.current_strength > initial_strength);
    }

    #[test]
    fn test_adjust_strength_weakens() {
        let mut attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.5, 0.2);
        let initial_strength = attractor.current_strength;

        // Add feedback showing attractor is not working well
        for i in 0..5 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.8, 0.3, 0.5); // Low effectiveness
            attractor.receive_feedback(feedback);
        }

        attractor.adjust_strength();

        // Should have weakened
        assert!(attractor.current_strength < initial_strength);
    }

    #[test]
    fn test_adjust_strength_clamping() {
        let mut attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.5, 0.5);

        // Add very strong positive feedback
        for i in 0..20 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.1, 1.0, 1.0); // Very high effectiveness
            attractor.receive_feedback(feedback);
        }

        attractor.adjust_strength();

        // Should not exceed max_strength
        assert!(attractor.current_strength <= attractor.max_strength);
    }

    #[test]
    fn test_adjust_strength_no_feedback() {
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
        let initial_strength = attractor.current_strength;

        attractor.adjust_strength();

        // Should not change without feedback
        assert_eq!(attractor.current_strength, initial_strength);
    }

    #[test]
    fn test_get_pull_strength() {
        let attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
        assert_eq!(attractor.get_pull_strength(), 0.3);
    }

    #[test]
    fn test_reset() {
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

        // Add feedback and adjust
        let entity_id = EntityId::new("test-entity".to_string());
        let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9);
        attractor.receive_feedback(feedback);
        attractor.adjust_strength();

        let _adjusted_strength = attractor.current_strength;
        assert!(attractor.feedback_count() > 0);

        // Reset
        attractor.reset();

        assert_eq!(attractor.current_strength, attractor.base_strength);
        assert_eq!(attractor.feedback_count(), 0);
    }

    #[test]
    fn test_confidence_increases_with_samples() {
        let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

        // Few samples
        for i in 0..3 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9);
            attractor.receive_feedback(feedback);
        }
        let effectiveness_low = attractor.calculate_effectiveness();

        // Many samples
        for i in 3..30 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9);
            attractor.receive_feedback(feedback);
        }
        let effectiveness_high = attractor.calculate_effectiveness();

        // Confidence should increase with more samples
        assert!(effectiveness_high.confidence > effectiveness_low.confidence);
    }
}
