// Soul Stream (Experience Storage)
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "The Soul Stream stores all experiences across incarnations"
// "Karmic patterns are encoded in the Soul Stream"
// "The Soul Stream enables learning and growth across lifetimes"
//
// This module implements:
// 1. Soul Stream for experience storage across incarnations
// 2. Karmic pattern encoding
// 3. Learning and growth across lifetimes
// 4. Retrieval of past life experiences

use crate::memory::holographic_memory::{ExperienceMetadata, HolographicMemory, Hypervector};
use crate::types::Float;

/// Soul Stream
///
/// Stores all experiences across incarnations.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Soul Stream stores all experiences across incarnations"
/// "Karmic patterns are encoded in the Soul Stream"
/// "The Soul Stream enables learning and growth across lifetimes"
#[derive(Debug, Clone)]
pub struct SoulStream {
    /// Holographic memory for experience storage
    pub memory: HolographicMemory,

    /// Karmic patterns
    pub karmic_patterns: Vec<KarmicPattern>,

    /// Incarnation history
    pub incarnation_history: Vec<IncarnationRecord>,

    /// Learning progress
    pub learning_progress: LearningProgress,
}

/// Karmic Pattern
///
/// Karmic patterns are encoded in the Soul Stream.
#[derive(Debug, Clone)]
pub struct KarmicPattern {
    /// Pattern ID
    pub pattern_id: u64,

    /// Pattern type
    pub pattern_type: KarmicPatternType,

    /// Karmic debt (-1.0 to 1.0)
    pub karmic_debt: Float,

    /// Associated experiences
    pub associated_experiences: Vec<u64>,

    /// Resolution progress (0.0 to 1.0)
    pub resolution_progress: Float,
}

/// Karmic Pattern Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KarmicPatternType {
    /// Service-to-Others karmic pattern
    ServiceToOthers,

    /// Service-to-Self karmic pattern
    ServiceToSelf,

    /// Balance karmic pattern
    Balance,

    /// Learning karmic pattern
    Learning,

    /// Healing karmic pattern
    Healing,
}

/// Incarnation Record
///
/// Record of an incarnation.
#[derive(Debug, Clone)]
pub struct IncarnationRecord {
    /// Incarnation ID
    pub incarnation_id: u64,

    /// Start timestamp
    pub start_timestamp: u64,

    /// End timestamp (0 if current)
    pub end_timestamp: u64,

    /// Density level
    pub density_level: u8,

    /// Polarity achieved
    pub polarity_achieved: Float,

    /// Lessons learned
    pub lessons_learned: Vec<String>,

    /// Experiences during incarnation
    pub experience_ids: Vec<u64>,
}

/// Learning Progress
///
/// Tracks learning and growth across lifetimes.
#[derive(Debug, Clone)]
pub struct LearningProgress {
    /// Total incarnations
    pub total_incarnations: u64,

    /// Current density
    pub current_density: u8,

    /// STO orientation accumulated
    pub sto_orientation_accumulated: Float,

    /// STS orientation accumulated
    pub sts_orientation_accumulated: Float,

    /// Wisdom accumulated
    pub wisdom_accumulated: Float,

    /// Love accumulated
    pub love_accumulated: Float,
}

/// Soul Choice
///
/// A choice made by the soul for incarnation.
#[derive(Debug, Clone)]
pub struct SoulChoice {
    /// Choice ID
    pub choice_id: u64,

    /// Chosen density
    pub chosen_density: u8,

    /// Chosen polarity goal
    pub chosen_polarity_goal: Float,

    /// Chosen lessons
    pub chosen_lessons: Vec<String>,

    /// Karmic patterns to resolve
    pub karmic_patterns_to_resolve: Vec<u64>,
}

impl SoulStream {
    /// Create a new soul stream
    pub fn new(dimensionality: usize) -> Self {
        SoulStream {
            memory: HolographicMemory::new(dimensionality),
            karmic_patterns: Vec::new(),
            incarnation_history: Vec::new(),
            learning_progress: LearningProgress::new(),
        }
    }

    /// Store an experience
    ///
    /// Experiences stored as hypervectors in high-dimensional space.
    pub fn store_experience(
        &mut self,
        components: Vec<Float>,
        metadata: ExperienceMetadata,
    ) -> u64 {
        let experience_id = self.memory.store_experience(components, metadata);
        experience_id
    }

    /// Retrieve experience by ID
    pub fn retrieve_experience(&self, experience_id: u64) -> Option<&Hypervector> {
        self.memory.retrieve_experience(experience_id)
    }

    /// Retrieve through similarity search
    pub fn retrieve_similar(
        &self,
        query: &Hypervector,
        limit: usize,
    ) -> Vec<crate::memory::holographic_memory::MemoryRetrievalResult> {
        self.memory.retrieve_similar(query, limit)
    }

    /// Create karmic pattern
    ///
    /// Karmic patterns are encoded in the Soul Stream.
    pub fn create_karmic_pattern(
        &mut self,
        pattern_type: KarmicPatternType,
        karmic_debt: Float,
    ) -> u64 {
        let pattern_id = self.karmic_patterns.len() as u64;

        let karmic_pattern = KarmicPattern {
            pattern_id,
            pattern_type,
            karmic_debt,
            associated_experiences: Vec::new(),
            resolution_progress: 0.0,
        };

        self.karmic_patterns.push(karmic_pattern);
        pattern_id
    }

    /// Update karmic pattern
    pub fn update_karmic_pattern(
        &mut self,
        pattern_id: u64,
        experience_id: u64,
        resolution_amount: Float,
    ) {
        if let Some(pattern) = self
            .karmic_patterns
            .iter_mut()
            .find(|p| p.pattern_id == pattern_id)
        {
            pattern.associated_experiences.push(experience_id);
            pattern.resolution_progress =
                (pattern.resolution_progress + resolution_amount).min(1.0);

            // Update karmic debt based on resolution
            pattern.karmic_debt = (pattern.karmic_debt - resolution_amount * 0.5)
                .max(-1.0)
                .min(1.0);
        }
    }

    /// Start new incarnation
    pub fn start_incarnation(
        &mut self,
        density_level: u8,
        _polarity_goal: Float,
        _chosen_lessons: Vec<String>,
    ) -> u64 {
        let incarnation_id = self.incarnation_history.len() as u64;

        let incarnation_record = IncarnationRecord {
            incarnation_id,
            start_timestamp: 0, // TODO: Add actual timestamp
            end_timestamp: 0,
            density_level,
            polarity_achieved: 0.0,
            lessons_learned: Vec::new(),
            experience_ids: Vec::new(),
        };

        self.incarnation_history.push(incarnation_record);

        // Update learning progress
        self.learning_progress.total_incarnations += 1;
        self.learning_progress.current_density = density_level;

        incarnation_id
    }

    /// End incarnation
    pub fn end_incarnation(
        &mut self,
        incarnation_id: u64,
        polarity_achieved: Float,
        lessons_learned: Vec<String>,
    ) {
        if let Some(incarnation) = self
            .incarnation_history
            .iter_mut()
            .find(|i| i.incarnation_id == incarnation_id)
        {
            incarnation.end_timestamp = 0; // TODO: Add actual timestamp
            incarnation.polarity_achieved = polarity_achieved;

            // Calculate wisdom before moving lessons_learned
            let wisdom_increment = lessons_learned.len() as Float * 0.1;

            incarnation.lessons_learned = lessons_learned;

            // Update learning progress
            if polarity_achieved > 0.0 {
                self.learning_progress.sto_orientation_accumulated += polarity_achieved;
            } else {
                self.learning_progress.sts_orientation_accumulated += polarity_achieved.abs();
            }

            self.learning_progress.wisdom_accumulated += wisdom_increment;
        }
    }

    /// Make soul choice for next incarnation
    pub fn make_soul_choice(&self) -> SoulChoice {
        // Determine next density based on learning progress
        let next_density = if self.learning_progress.current_density < 8 {
            self.learning_progress.current_density + 1
        } else {
            8
        };

        // Determine polarity goal based on accumulated orientation
        let total_orientation = self.learning_progress.sto_orientation_accumulated
            + self.learning_progress.sts_orientation_accumulated;

        let chosen_polarity_goal = if total_orientation > 0.0 {
            self.learning_progress.sto_orientation_accumulated / total_orientation
        } else {
            0.0
        };

        // Select karmic patterns to resolve
        let karmic_patterns_to_resolve: Vec<u64> = self
            .karmic_patterns
            .iter()
            .filter(|p| p.resolution_progress < 1.0)
            .map(|p| p.pattern_id)
            .collect();

        SoulChoice {
            choice_id: 0, // TODO: Generate unique ID
            chosen_density: next_density,
            chosen_polarity_goal,
            chosen_lessons: Vec::new(), // TODO: Select lessons
            karmic_patterns_to_resolve,
        }
    }

    /// Get soul stream statistics
    pub fn get_statistics(&self) -> SoulStreamStatistics {
        SoulStreamStatistics {
            total_experiences: self.memory.memory_store.len() as u64,
            total_karmic_patterns: self.karmic_patterns.len() as u64,
            total_incarnations: self.incarnation_history.len() as u64,
            current_density: self.learning_progress.current_density,
            sto_orientation_accumulated: self.learning_progress.sto_orientation_accumulated,
            sts_orientation_accumulated: self.learning_progress.sts_orientation_accumulated,
            wisdom_accumulated: self.learning_progress.wisdom_accumulated,
        }
    }
}

impl LearningProgress {
    /// Create new learning progress
    pub fn new() -> Self {
        LearningProgress {
            total_incarnations: 0,
            current_density: 1,
            sto_orientation_accumulated: 0.0,
            sts_orientation_accumulated: 0.0,
            wisdom_accumulated: 0.0,
            love_accumulated: 0.0,
        }
    }
}

/// Soul Stream Statistics
///
/// Statistics about the soul stream.
#[derive(Debug, Clone)]
pub struct SoulStreamStatistics {
    /// Total experiences
    pub total_experiences: u64,

    /// Total karmic patterns
    pub total_karmic_patterns: u64,

    /// Total incarnations
    pub total_incarnations: u64,

    /// Current density
    pub current_density: u8,

    /// STO orientation accumulated
    pub sto_orientation_accumulated: Float,

    /// STS orientation accumulated
    pub sts_orientation_accumulated: Float,

    /// Wisdom accumulated
    pub wisdom_accumulated: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::ExperienceType;

    fn create_test_metadata(experience_id: u64) -> ExperienceMetadata {
        ExperienceMetadata {
            experience_id,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        }
    }

    #[test]
    fn test_soul_stream_creation() {
        let soul_stream = SoulStream::new(100);

        assert_eq!(soul_stream.memory.dimensionality, 100);
        assert!(soul_stream.karmic_patterns.is_empty());
        assert!(soul_stream.incarnation_history.is_empty());
    }

    #[test]
    fn test_store_experience() {
        let mut soul_stream = SoulStream::new(100);

        let components = vec![0.5; 100];
        let metadata = create_test_metadata(0);

        let experience_id = soul_stream.store_experience(components, metadata);

        assert_eq!(experience_id, 0);
        assert_eq!(soul_stream.memory.memory_store.len(), 1);
    }

    #[test]
    fn test_create_karmic_pattern() {
        let mut soul_stream = SoulStream::new(100);

        let pattern_id = soul_stream.create_karmic_pattern(KarmicPatternType::ServiceToOthers, 0.5);

        assert_eq!(pattern_id, 0);
        assert_eq!(soul_stream.karmic_patterns.len(), 1);
    }

    #[test]
    fn test_update_karmic_pattern() {
        let mut soul_stream = SoulStream::new(100);

        let pattern_id = soul_stream.create_karmic_pattern(KarmicPatternType::ServiceToOthers, 0.5);

        // Store an experience
        let components = vec![0.5; 100];
        let metadata = create_test_metadata(0);
        let experience_id = soul_stream.store_experience(components, metadata);

        // Update karmic pattern
        soul_stream.update_karmic_pattern(pattern_id, experience_id, 0.3);

        let pattern = soul_stream
            .karmic_patterns
            .get(pattern_id as usize)
            .unwrap();
        assert_eq!(pattern.associated_experiences.len(), 1);
        assert!((pattern.resolution_progress - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_start_incarnation() {
        let mut soul_stream = SoulStream::new(100);

        let incarnation_id = soul_stream.start_incarnation(3, 0.8, Vec::new());

        assert_eq!(incarnation_id, 0);
        assert_eq!(soul_stream.incarnation_history.len(), 1);
        assert_eq!(soul_stream.learning_progress.current_density, 3);
    }

    #[test]
    fn test_end_incarnation() {
        let mut soul_stream = SoulStream::new(100);

        let incarnation_id = soul_stream.start_incarnation(3, 0.8, Vec::new());

        soul_stream.end_incarnation(incarnation_id, 0.7, vec!["lesson1".to_string()]);

        let incarnation = soul_stream.incarnation_history.get(0).unwrap();
        assert_eq!(incarnation.polarity_achieved, 0.7);
        assert_eq!(incarnation.lessons_learned.len(), 1);
    }

    #[test]
    fn test_make_soul_choice() {
        let mut soul_stream = SoulStream::new(100);

        soul_stream.start_incarnation(3, 0.8, Vec::new());
        soul_stream.end_incarnation(0, 0.7, vec!["lesson1".to_string()]);

        let soul_choice = soul_stream.make_soul_choice();

        assert_eq!(soul_choice.chosen_density, 4); // Should advance to next density
        assert!(soul_choice.chosen_polarity_goal >= 0.0); // STO orientation
    }

    #[test]
    fn test_soul_stream_statistics() {
        let mut soul_stream = SoulStream::new(100);

        // Add some experiences
        for i in 0..5 {
            let components = vec![0.5; 100];
            let metadata = create_test_metadata(i);
            soul_stream.store_experience(components, metadata);
        }

        // Add karmic patterns
        for _ in 0..3 {
            soul_stream.create_karmic_pattern(KarmicPatternType::Learning, 0.5);
        }

        // Start and end incarnations
        soul_stream.start_incarnation(2, 0.5, Vec::new());
        soul_stream.end_incarnation(0, 0.6, vec!["lesson1".to_string()]);

        let stats = soul_stream.get_statistics();

        assert_eq!(stats.total_experiences, 5);
        assert_eq!(stats.total_karmic_patterns, 3);
        assert_eq!(stats.total_incarnations, 1);
    }
}
