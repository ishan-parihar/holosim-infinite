//! Standalone test for evolution module
//!
//! This file tests the evolution module independently of the rest of the codebase.

use holonic_realms::entity_layer7::layer7::EntityId;
use holonic_realms::evolution::{
    evaluate_purpose, AdaptiveAttractorField, EntityFeedback, FeedbackAnalysis,
    IntelligentInfinity, TeleologicalProgress,
};
use holonic_realms::types::Density;

#[test]
fn test_adaptive_attractor_basic() {
    let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);

    // Test initial state
    assert_eq!(attractor.target_density, Density::Third);
    assert_eq!(attractor.current_strength, 0.3);
    assert_eq!(attractor.feedback_count(), 0);

    // Test receiving feedback
    let entity_id = EntityId::new("test-entity".to_string());
    let feedback = EntityFeedback::new(entity_id, 0.7, 0.5, 0.8);
    attractor.receive_feedback(feedback);

    assert_eq!(attractor.feedback_count(), 1);

    // Test effectiveness calculation
    let effectiveness = attractor.calculate_effectiveness();
    assert!(effectiveness.average_effectiveness > 0.0);

    println!("✓ AdaptiveAttractorField basic functionality works");
}

#[test]
fn test_entity_feedback_creation() {
    let entity_id = EntityId::new("test-entity".to_string());
    let feedback = EntityFeedback::new(entity_id.clone(), 0.7, 0.5, 0.8);

    assert_eq!(feedback.entity_id, entity_id);
    assert_eq!(feedback.attractor_pull, 0.7);
    assert_eq!(feedback.evolution_progress, 0.5);
    assert_eq!(feedback.alignment_with_attractor, 0.8);

    // Test effectiveness calculation
    let effectiveness = feedback.calculate_effectiveness();
    assert!((effectiveness - 0.714).abs() < 0.01); // 0.5 / 0.7 ≈ 0.714

    println!("✓ EntityFeedback creation and calculation works");
}

#[test]
fn test_adaptive_attractor_learning() {
    let mut attractor = AdaptiveAttractorField::with_learning_rate(Density::Third, 0.3, 0.2);
    let initial_strength = attractor.current_strength;

    // Add positive feedback (attractor working well)
    for i in 0..5 {
        let entity_id = EntityId::new(format!("entity-{}", i));
        let feedback = EntityFeedback::new(entity_id, 0.5, 0.9, 0.9);
        attractor.receive_feedback(feedback);
    }

    attractor.adjust_strength();

    // Should have strengthened
    assert!(attractor.current_strength > initial_strength);

    println!("✓ AdaptiveAttractorField learning works");
}

#[test]
fn test_teleological_progress_creation() {
    let progress = TeleologicalProgress::new(0.7, 0.8, 0.5, 10.0);

    assert_eq!(progress.purpose_alignment, 0.7);
    assert_eq!(progress.coherence_with_source, 0.8);
    assert_eq!(progress.service_orientation, 0.5);
    assert_eq!(progress.wisdom_accumulated, 10.0);
    assert!(progress.overall_progress > 0.0);
    assert!(progress.overall_progress <= 1.0);

    // Test polarity detection
    assert_eq!(progress.primary_polarity(), "STO");

    let sts = TeleologicalProgress::new(0.7, 0.8, -0.5, 10.0);
    assert_eq!(sts.primary_polarity(), "STS");

    println!("✓ TeleologicalProgress creation works");
}

#[test]
fn test_intelligent_infinity_basic() {
    let mut ii = IntelligentInfinity::new(0.1);

    // Test initial state
    assert_eq!(ii.potential, 1.0);
    assert_eq!(ii.kinetic, 0.5);
    assert_eq!(ii.teleological_emission, 0.5);
    assert_eq!(ii.feedback_count(), 0);

    // Test pulsing
    let initial_kinetic = ii.kinetic;
    ii.pulse();
    assert_ne!(ii.kinetic, initial_kinetic);
    assert!(ii.kinetic >= 0.0 && ii.kinetic <= 1.0);

    println!("✓ IntelligentInfinity basic functionality works");
}

#[test]
fn test_intelligent_infinity_feedback() {
    let mut ii = IntelligentInfinity::with_default_frequency();

    // Add feedback
    for i in 0..10 {
        let entity_id = EntityId::new(format!("entity-{}", i));
        let feedback = EntityFeedback::new(entity_id, 0.7, 0.5 + (i as f64 * 0.05), 0.8);
        ii.receive_entity_feedback(feedback);
    }

    assert_eq!(ii.feedback_count(), 10);

    // Analyze feedback
    let analysis = ii.analyze_feedback_patterns();
    assert_eq!(analysis.entity_count, 10);
    assert!(analysis.average_effectiveness > 0.0);

    // Test teleological emission
    let pull = ii.emit_teleological_pull();
    assert!(pull >= 0.0 && pull <= 1.0);

    println!("✓ IntelligentInfinity feedback collection works");
}

#[test]
fn test_feedback_analysis_creation() {
    let analysis = FeedbackAnalysis::new(10, 0.75, 0.8, 0.7, 0.3, 0.75);

    assert_eq!(analysis.entity_count, 10);
    assert_eq!(analysis.average_effectiveness, 0.75);
    assert_eq!(analysis.average_alignment, 0.8);
    assert_eq!(analysis.average_progress, 0.7);
    assert_eq!(analysis.service_orientation_bias, 0.3);
    assert_eq!(analysis.collective_progress, 0.75);
    assert!(analysis.is_collective_progress_meaningful);

    println!("✓ FeedbackAnalysis creation works");
}

fn main() {
    println!("Running evolution module standalone tests...\n");

    test_adaptive_attractor_basic();
    test_entity_feedback_creation();
    test_adaptive_attractor_learning();
    test_teleological_progress_creation();
    test_intelligent_infinity_basic();
    test_intelligent_infinity_feedback();
    test_feedback_analysis_creation();

    println!("\n✅ All evolution module tests passed!");
}
