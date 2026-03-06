// Memory Module
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Experiences stored as hypervectors in high-dimensional space"
// "Retrieval through similarity search (cosine distance)"
// "Distributed encoding (no single location)"
// "Fault-tolerant (graceful degradation)"
//
// This module implements:
// 1. Holographic memory system
// 2. Soul Stream (experience storage across incarnations)
// 3. Karmic pattern encoding
// 4. Learning and growth across lifetimes

pub mod holographic_memory;
pub mod soul_stream;

// Re-export main types for convenience
pub use holographic_memory::{ExperienceMetadata, ExperienceType, Hypervector};
pub use soul_stream::{
    IncarnationRecord as Incarnation, KarmicPattern, KarmicPatternType, SoulChoice, SoulStream,
    SoulStreamStatistics as SoulStatistics,
};

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_holographic_memory_soul_stream_integration() {
        // Create soul stream with holographic memory
        let mut soul_stream = SoulStream::new(100);

        // Store experiences
        for i in 0..5 {
            let components = vec![(i as f64) / 5.0; 100];
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            soul_stream.store_experience(components, metadata);
        }

        // Retrieve experiences
        let experience = soul_stream.retrieve_experience(0);
        assert!(experience.is_some());

        // Retrieve similar experiences
        let query = Hypervector::new(100);
        let similar = soul_stream.retrieve_similar(&query, 3);
        assert!(similar.len() <= 3);
    }

    #[test]
    fn test_karmic_pattern_experience_integration() {
        let mut soul_stream = SoulStream::new(100);

        // Create karmic pattern
        let pattern_id = soul_stream.create_karmic_pattern(KarmicPatternType::Learning, 0.5);

        // Store experience
        let components = vec![0.5; 100];
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };
        let experience_id = soul_stream.store_experience(components, metadata);

        // Update karmic pattern with experience
        soul_stream.update_karmic_pattern(pattern_id, experience_id, 0.3);

        // Verify integration
        let pattern = soul_stream
            .karmic_patterns
            .get(pattern_id as usize)
            .unwrap();
        assert_eq!(pattern.associated_experiences.len(), 1);
        assert_eq!(pattern.associated_experiences[0], experience_id);
    }

    #[test]
    fn test_incarnation_experience_integration() {
        let mut soul_stream = SoulStream::new(100);

        // Start incarnation
        let incarnation_id = soul_stream.start_incarnation(3, 0.8, Vec::new());

        // Store experiences during incarnation
        for i in 0..5 {
            let components = vec![0.5; 100];
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            soul_stream.store_experience(components, metadata);
        }

        // End incarnation
        soul_stream.end_incarnation(incarnation_id, 0.7, vec!["lesson1".to_string()]);

        // Verify integration
        let incarnation = soul_stream.incarnation_history.first().unwrap();
        assert_eq!(incarnation.incarnation_id, incarnation_id);
        assert_eq!(incarnation.polarity_achieved, 0.7);
    }

    #[test]
    fn test_complete_soul_stream_flow() {
        let mut soul_stream = SoulStream::new(100);

        // 1. First incarnation
        let incarnation1_id = soul_stream.start_incarnation(2, 0.5, Vec::new());

        // Store experiences
        for i in 0..3 {
            let components = vec![0.5; 100];
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            soul_stream.store_experience(components, metadata);
        }

        // Create karmic pattern
        let pattern_id = soul_stream.create_karmic_pattern(KarmicPatternType::ServiceToOthers, 0.5);

        // Update karmic pattern
        soul_stream.update_karmic_pattern(pattern_id, 0, 0.3);

        // End incarnation
        soul_stream.end_incarnation(incarnation1_id, 0.6, vec!["lesson1".to_string()]);

        // 2. Make soul choice for next incarnation
        let soul_choice = soul_stream.make_soul_choice();

        // Verify soul choice reflects learning
        assert_eq!(soul_choice.chosen_density, 3); // Advanced to next density
        assert!(soul_choice.karmic_patterns_to_resolve.contains(&pattern_id));

        // 3. Second incarnation
        let _incarnation2_id = soul_stream.start_incarnation(
            soul_choice.chosen_density,
            soul_choice.chosen_polarity_goal,
            soul_choice.chosen_lessons,
        );

        // Verify learning progress
        let stats = soul_stream.get_statistics();
        assert_eq!(stats.total_incarnations, 2);
        assert_eq!(stats.current_density, 3);
        assert!(stats.sto_orientation_accumulated > 0.0);
    }

    #[test]
    fn test_fault_tolerance_with_partial_encoding() {
        let mut soul_stream = SoulStream::new(100);

        // Store experience
        let components = vec![1.0; 100];
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };
        let experience_id = soul_stream.store_experience(components, metadata);

        // Create partial encoding
        let partial = soul_stream
            .memory
            .create_partial_encoding(experience_id, 0.5);

        assert!(partial.is_some());

        // Reconstruct from partial
        let reconstructed = soul_stream
            .memory
            .reconstruct_from_partial(&partial.unwrap());

        assert!(reconstructed.is_some());

        // Verify reconstruction maintains completeness
        let reconstructed_hypervector = reconstructed.unwrap();
        assert_eq!(reconstructed_hypervector.components.len(), 100);
    }

    #[test]
    fn test_similarity_search_across_incarnations() {
        let mut soul_stream = SoulStream::new(100);

        // First incarnation experiences
        let incarnation1_id = soul_stream.start_incarnation(2, 0.5, Vec::new());

        for i in 0..3 {
            let components = vec![0.3; 100];
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            soul_stream.store_experience(components, metadata);
        }

        soul_stream.end_incarnation(incarnation1_id, 0.6, Vec::new());

        // Second incarnation experiences
        let incarnation2_id = soul_stream.start_incarnation(3, 0.7, Vec::new());

        for i in 3..6 {
            let components = vec![0.7; 100];
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.6,
                sto_alignment: 0.2,
                associated_entities: Vec::new(),
            };
            soul_stream.store_experience(components, metadata);
        }

        soul_stream.end_incarnation(incarnation2_id, 0.8, Vec::new());

        // Create query hypervector
        let query = Hypervector::new(100);

        // Retrieve similar experiences (should find from both incarnations)
        let similar = soul_stream.retrieve_similar(&query, 5);

        assert!(!similar.is_empty());
    }
}
