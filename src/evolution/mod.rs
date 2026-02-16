//! Evolution Module - Intelligent Evolution System
//!
//! This module implements intelligent evolution for HoloSim_Infinite, replacing mechanical
//! probabilistic evolution with directed, purposeful evolution.
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3 (Weeks 9-12):
//! "The critical phase that addresses the audit's main finding: evolution is mechanical, not intelligent."
//!
//! ## Architecture
//!
//! The evolution module consists of three components:
//!
//! 1. **Adaptive Attractor Fields** (`adaptive_attractor.rs`):
//!    - Dynamic attractor fields that adjust based on entity feedback
//!    - Learning system that strengthens/weakens attractors based on effectiveness
//!    - Feedback loop: entities → attractors → entities
//!
//! 2. **Teleological Progress** (`teleological.rs`):
//!    - Purpose tracking toward return to Intelligent-Infinity
//!    - Progress measurement: alignment, coherence, service orientation, wisdom
//!    - Meaningful choices that align with purpose
//!
//! 3. **Enhanced Intelligent-Infinity** (`intelligent_infinity.rs`):
//!    - Active feedback collector from entities
//!    - Pattern analysis of entity behavior
//!    - Teleological pull toward source
//!
//! ## Key Principles
//!
//! - **Feedback-Based Learning**: Attractors adapt based on entity response
//! - **Teleological Direction**: Evolution has purpose (return to source, having served)
//! - **Meaningful Choices**: Not random, but aligned with purpose
//! - **Coherence Matters**: Alignment with purpose affects evolution rate
//!
//! ## Usage
//!
//! ```rust
//! use holonic_realms::evolution::{
//!     AdaptiveAttractorField, EntityFeedback,
//!     TeleologicalProgress, evaluate_purpose,
//!     IntelligentInfinity
//! };
//!
//! // Create adaptive attractor field
//! let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
//!
//! // Entity provides feedback
//! let feedback = EntityFeedback {
//!     entity_id: EntityId::new("entity-1".to_string()),
//!     attractor_pull: 0.7,
//!     evolution_progress: 0.5,
//!     alignment_with_attractor: 0.8,
//!     timestamp: 12345,
//! };
//! attractor.receive_feedback(feedback);
//! attractor.adjust_strength();
//!
//! // Evaluate entity's purpose alignment
//! let entity = /* ... */;
//! let teleological = evaluate_purpose(&entity);
//!
//! // Enhanced Intelligent-Infinity collects feedback
//! let mut ii = IntelligentInfinity::new();
//! ii.receive_entity_feedback(feedback);
//! let analysis = ii.analyze_feedback_patterns();
//! ```

pub mod adaptive_attractor;
pub mod intelligent_infinity;
pub mod teleological;

pub use adaptive_attractor::{
    AdaptiveAttractorField, EntityFeedback, FeedbackEffectiveness,
};
pub use intelligent_infinity::{FeedbackAnalysis, IntelligentInfinity};
pub use teleological::{evaluate_purpose, TeleologicalProgress};
// ============================================================================
// PHASE 3 INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod phase3_integration_tests {
    use super::*;
    use crate::entity_layer7::layer7::{EntityId, EntityType, EntitySpectrumAccess, EntityState, SubSubLogos};
    use crate::foundation::{IntelligentInfinity as IndigoRealm, LightLoveField as GreenRealm, Logos as BlueRealm, VioletRealm};
    use crate::spectrum::{OrangeRealm, RedRealm, SpectrumRatio, YellowRealm};
    use crate::gui::visualization::entity_viz::{EntityVisualizationData, update_teleological_metrics, format_teleological_progress, get_teleological_color, get_service_orientation_color};

    #[test]
    fn test_feedback_loop_end_to_end() {
        let mut attractor = AdaptiveAttractorField::with_learning_rate(
            crate::types::Density::Third,
            0.3,
            0.2,
        );

        let initial_strength = attractor.current_strength;

        for i in 0..20 {
            let entity_id = EntityId::new(format!("entity-{}", i));
            let (pull, progress, alignment) = if i < 10 {
                (0.6, 0.8, 0.9)
            } else {
                (0.8, 0.3, 0.4)
            };

            let feedback = EntityFeedback::new(entity_id, pull, progress, alignment);
            attractor.receive_feedback(feedback);
        }

        attractor.adjust_strength();

        assert_eq!(attractor.feedback_count(), 20);
        assert!((attractor.current_strength - initial_strength).abs() > 0.01);

        let effectiveness = attractor.calculate_effectiveness();
        assert_eq!(effectiveness.sample_count, 20);
        assert!(effectiveness.average_effectiveness > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_pattern_analysis() {
        let mut ii = IntelligentInfinity::with_default_frequency();

        for i in 0..50 {
            let entity_id = EntityId::new(format!("entity-{}", i));

            let (pull, progress, alignment) = match i % 5 {
                0 => (0.7, 0.9, 0.9),
                1 => (0.6, 0.7, 0.8),
                2 => (0.5, 0.5, 0.6),
                3 => (0.7, 0.3, 0.4),
                _ => (0.4, 0.2, 0.3),
            };

            let feedback = EntityFeedback::new(entity_id, pull, progress, alignment);
            ii.receive_entity_feedback(feedback);
        }

        let analysis = ii.analyze_feedback_patterns();

        assert_eq!(analysis.entity_count, 50);
        assert!(analysis.average_effectiveness > 0.0);
        assert!(analysis.collective_progress > 0.0);
        assert!(analysis.collective_progress < 1.0);
    }

    #[test]
    fn test_teleological_progress_tracking() {
        let entity = create_test_entity();
        let teleological = evaluate_purpose(&entity);

        assert!(teleological.purpose_alignment >= 0.0);
        assert!(teleological.purpose_alignment <= 1.0);
        assert!(teleological.coherence_with_source >= 0.0);
        assert!(teleological.coherence_with_source <= 1.0);
        assert!(teleological.service_orientation >= -1.0);
        assert!(teleological.service_orientation <= 1.0);
        assert!(teleological.overall_progress >= 0.0);
        assert!(teleological.overall_progress <= 1.0);
    }

    #[test]
    fn test_visualization_teleological_metrics() {
        let mut viz_data = EntityVisualizationData {
            position: [0.0, 0.0, 0.0],
            scale: 1.0,
            color: [0.0, 0.0, 0.0, 1.0],
            polarity_color: [0.0, 0.0, 0.0, 0.0],
            archetype_glow: [0.0, 0.0, 0.0, 0.0],
            density: 1.0,
            polarity: 0.0,
            consciousness: 0.0,
            archetype_activations: [0.0; 22],
            style: 0,
            focused: 0,
            geometry: 0,
            purpose_alignment: 0.0,
            coherence_with_source: 0.0,
            service_orientation: 0.0,
            wisdom_accumulated: 0.0,
            teleological_progress: 0.0,
        };

        update_teleological_metrics(&mut viz_data, 0.85, 0.90, 0.7, 25.5, 0.88);

        assert_eq!(viz_data.purpose_alignment, 0.85);
        assert_eq!(viz_data.coherence_with_source, 0.90);
        assert_eq!(viz_data.service_orientation, 0.7);
        assert_eq!(viz_data.wisdom_accumulated, 25.5);
        assert_eq!(viz_data.teleological_progress, 0.88);

        let formatted = format_teleological_progress(&viz_data);
        assert!(formatted.contains("85%"));
        assert!(formatted.contains("90%"));
        assert!(formatted.contains("STO"));
    }

    #[test]
    fn test_teleological_color_indicators() {
        let red = get_teleological_color(0.0);
        let yellow = get_teleological_color(0.5);
        let green = get_teleological_color(1.0);

        assert!(red[0] > 0.9);
        assert!(yellow[0] > 0.8 && yellow[1] > 0.8);
        assert!(green[1] > 0.9);

        let sts = get_service_orientation_color(-1.0);
        let neutral = get_service_orientation_color(0.0);
        let sto = get_service_orientation_color(1.0);

        assert!(sts[0] > 0.9 && sts[1] < 0.5);
        assert!((neutral[0] - neutral[1]).abs() < 0.2);
        assert!(sto[1] > 0.9 && sto[0] < 0.5);
    }

    /// Create a test entity for teleological evaluation
    fn create_test_entity() -> SubSubLogos {
        let entity_id = EntityId::new("test-entity".to_string());
        let entity_type = EntityType::Individual;

        let violet_realm = VioletRealm::new();
        let indigo_realm = IndigoRealm::from_violet(violet_realm.clone());
        let blue_realm = BlueRealm::from_intelligent_infinity(indigo_realm.clone());
        let green_realm = GreenRealm::from_logos(blue_realm.clone());

        let yellow_realm = YellowRealm::new(green_realm.clone());
        let orange_realm = OrangeRealm::new(yellow_realm.clone());
        let red_realm = RedRealm::new(orange_realm.clone());

        let spectrum_ratio = SpectrumRatio::new(1.0, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_configuration =
            crate::entity_layer7::layer7::IndividualSpectrumConfiguration::new(spectrum_ratio);

        SubSubLogos::new(
            entity_id,
            entity_type,
            None,
            Vec::new(),
            None,
            violet_realm,
            indigo_realm,
            blue_realm,
            green_realm,
            yellow_realm,
            orange_realm,
            red_realm,
            spectrum_configuration,
        )
    }
}
