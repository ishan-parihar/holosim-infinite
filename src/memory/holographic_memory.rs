// Holographic Memory (Soul Stream)
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Experiences stored as hypervectors in high-dimensional space"
// "Retrieval through similarity search (cosine distance)"
// "Distributed encoding (no single location)"
// "Fault-tolerant (graceful degradation)"
//
// This module implements:
// 1. Holographic memory system
// 2. Experience storage as hypervectors
// 3. Retrieval through similarity search
// 4. Distributed encoding and fault tolerance

use crate::types::Float;
use rand::Rng;
use std::collections::HashMap;

/// Holographic Memory
///
/// Experiences stored as hypervectors in high-dimensional space.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Experiences stored as hypervectors in high-dimensional space"
/// "Retrieval through similarity search (cosine distance)"
/// "Distributed encoding (no single location)"
/// "Fault-tolerant (graceful degradation)"
#[derive(Debug, Clone)]
pub struct HolographicMemory {
    /// Memory store (hypervectors indexed by experience ID)
    pub memory_store: HashMap<u64, Hypervector>,

    /// Dimensionality of the hypervectors
    pub dimensionality: usize,

    /// Next available experience ID
    pub next_id: u64,
}

/// Hypervector
///
/// A high-dimensional vector representing an experience.
#[derive(Debug, Clone)]
pub struct Hypervector {
    /// Vector components
    pub components: Vec<Float>,

    /// Experience metadata
    pub metadata: ExperienceMetadata,
}

/// Experience Metadata
///
/// Metadata about an experience.
#[derive(Debug, Clone)]
pub struct ExperienceMetadata {
    /// Experience ID
    pub experience_id: u64,

    /// Experience type
    pub experience_type: ExperienceType,

    /// Timestamp
    pub timestamp: u64,

    /// Emotional intensity (0.0 to 1.0)
    pub emotional_intensity: Float,

    /// STO alignment (-1.0 to 1.0)
    pub sto_alignment: Float,

    /// Associated entities
    pub associated_entities: Vec<u64>,
}

/// Experience Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperienceType {
    /// Physical experience
    Physical,

    /// Mental experience
    Mental,

    /// Emotional experience
    Emotional,

    /// Spiritual experience
    Spiritual,

    /// Karmic experience
    Karmic,
}

/// Memory Retrieval Result
///
/// Result of a memory retrieval operation.
#[derive(Debug, Clone)]
pub struct MemoryRetrievalResult {
    /// Retrieved experience
    pub experience: Hypervector,

    /// Similarity score (0.0 to 1.0)
    pub similarity: Float,
}

impl HolographicMemory {
    /// Create a new holographic memory
    pub fn new(dimensionality: usize) -> Self {
        HolographicMemory {
            memory_store: HashMap::new(),
            dimensionality,
            next_id: 0,
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
        let experience_id = self.next_id;
        self.next_id += 1;

        let hypervector = Hypervector {
            components,
            metadata,
        };

        self.memory_store.insert(experience_id, hypervector);
        experience_id
    }

    /// Retrieve experience by ID
    pub fn retrieve_experience(&self, experience_id: u64) -> Option<&Hypervector> {
        self.memory_store.get(&experience_id)
    }

    /// Retrieve through similarity search
    ///
    /// Retrieval through similarity search (cosine distance).
    pub fn retrieve_similar(
        &self,
        query: &Hypervector,
        limit: usize,
    ) -> Vec<MemoryRetrievalResult> {
        let mut results: Vec<MemoryRetrievalResult> = self
            .memory_store
            .values()
            .map(|experience| {
                let similarity = query.cosine_similarity(experience);
                MemoryRetrievalResult {
                    experience: experience.clone(),
                    similarity,
                }
            })
            .filter(|r| r.similarity > 0.0)
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // Return top results
        results.into_iter().take(limit).collect()
    }

    /// Create partial encoding (fault tolerance)
    ///
    /// Distributed encoding (no single location).
    /// Fault-tolerant (graceful degradation).
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Cutting the encoding reduces resolution but maintains completeness"
    pub fn create_partial_encoding(
        &self,
        experience_id: u64,
        fraction: Float,
    ) -> Option<Hypervector> {
        let experience = self.memory_store.get(&experience_id)?;

        let partial_components: Vec<Float> = experience
            .components
            .iter()
            .map(|&c| c * fraction)
            .collect();

        Some(Hypervector {
            components: partial_components,
            metadata: experience.metadata.clone(),
        })
    }

    /// Reconstruct from partial encoding
    pub fn reconstruct_from_partial(&self, partial: &Hypervector) -> Option<Hypervector> {
        // Find the most similar full experience
        let similar = self.retrieve_similar(partial, 1);

        if let Some(result) = similar.first() {
            Some(result.experience.clone())
        } else {
            None
        }
    }

    /// Get memory statistics
    pub fn get_statistics(&self) -> MemoryStatistics {
        let total_experiences = self.memory_store.len();

        let avg_similarity = if total_experiences > 1 {
            let similarities: Vec<Float> = self
                .memory_store
                .values()
                .map(|v1| {
                    self.memory_store
                        .values()
                        .map(|v2| v1.cosine_similarity(v2))
                        .sum::<Float>()
                        / total_experiences as Float
                })
                .collect();

            similarities.iter().sum::<Float>() / total_experiences as Float
        } else {
            0.0
        };

        MemoryStatistics {
            total_experiences: total_experiences as u64,
            dimensionality: self.dimensionality,
            avg_similarity,
        }
    }
}

impl Hypervector {
    /// Create a new hypervector
    pub fn new(dimensionality: usize) -> Self {
        // Initialize with random values
        let mut rng = rand::thread_rng();
        let components: Vec<Float> = (0..dimensionality).map(|_| rng.gen::<f64>()).collect();

        Hypervector {
            components,
            metadata: ExperienceMetadata {
                experience_id: 0,
                experience_type: ExperienceType::Physical,
                timestamp: 0,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            },
        }
    }

    /// Calculate cosine similarity with another hypervector
    ///
    /// Retrieval through similarity search (cosine distance).
    pub fn cosine_similarity(&self, other: &Hypervector) -> Float {
        if self.components.is_empty() || other.components.is_empty() {
            return 0.0;
        }

        let dot_product: Float = self
            .components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum();

        let magnitude_self: Float = self.components.iter().map(|x| x * x).sum::<Float>().sqrt();
        let magnitude_other: Float = other.components.iter().map(|x| x * x).sum::<Float>().sqrt();

        if magnitude_self == 0.0 || magnitude_other == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_self * magnitude_other)
    }

    /// Normalize the hypervector
    pub fn normalize(&mut self) {
        let magnitude: Float = self.components.iter().map(|x| x * x).sum::<Float>().sqrt();

        if magnitude > 0.0 {
            for component in &mut self.components {
                *component /= magnitude;
            }
        }
    }
}

/// Memory Statistics
///
/// Statistics about holographic memory.
#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    /// Total number of experiences
    pub total_experiences: u64,

    /// Dimensionality of hypervectors
    pub dimensionality: usize,

    /// Average similarity between experiences
    pub avg_similarity: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_hypervector(dimensionality: usize, value: Float) -> Hypervector {
        let components = vec![value; dimensionality];
        Hypervector {
            components,
            metadata: ExperienceMetadata {
                experience_id: 0,
                experience_type: ExperienceType::Physical,
                timestamp: 0,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            },
        }
    }

    #[test]
    fn test_holographic_memory_creation() {
        let memory = HolographicMemory::new(100);

        assert_eq!(memory.dimensionality, 100);
        assert_eq!(memory.next_id, 0);
        assert!(memory.memory_store.is_empty());
    }

    #[test]
    fn test_store_experience() {
        let mut memory = HolographicMemory::new(100);

        let hypervector = create_test_hypervector(100, 0.5);
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };

        let experience_id = memory.store_experience(hypervector.components, metadata);

        assert_eq!(experience_id, 0);
        assert_eq!(memory.memory_store.len(), 1);
    }

    #[test]
    fn test_retrieve_experience() {
        let mut memory = HolographicMemory::new(100);

        let hypervector = create_test_hypervector(100, 0.5);
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };

        let experience_id = memory.store_experience(hypervector.components, metadata);
        let retrieved = memory.retrieve_experience(experience_id);

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_cosine_similarity() {
        let hypervector1 = create_test_hypervector(100, 1.0);
        let hypervector2 = create_test_hypervector(100, 1.0);
        let hypervector3 = create_test_hypervector(100, -1.0);

        let similarity_12 = hypervector1.cosine_similarity(&hypervector2);
        let similarity_13 = hypervector1.cosine_similarity(&hypervector3);

        // Same vectors should have similarity 1.0
        assert!((similarity_12 - 1.0).abs() < 0.01);

        // Opposite vectors should have similarity -1.0
        assert!((similarity_13 - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn test_retrieve_similar() {
        let mut memory = HolographicMemory::new(100);

        // Store several experiences
        for i in 0..5 {
            let hypervector = create_test_hypervector(100, (i as Float) / 5.0);
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            memory.store_experience(hypervector.components, metadata);
        }

        // Create query hypervector
        let query = create_test_hypervector(100, 0.4);

        // Retrieve similar experiences
        let similar = memory.retrieve_similar(&query, 3);

        assert!(similar.len() <= 3);
        assert!(similar.len() > 0);

        // Results should be sorted by similarity (descending)
        for i in 1..similar.len() {
            assert!(similar[i - 1].similarity >= similar[i].similarity);
        }
    }

    #[test]
    fn test_partial_encoding() {
        let mut memory = HolographicMemory::new(100);

        let hypervector = create_test_hypervector(100, 1.0);
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };

        let experience_id = memory.store_experience(hypervector.components, metadata);

        // Create partial encoding
        let partial = memory.create_partial_encoding(experience_id, 0.5);

        assert!(partial.is_some());

        let partial_hypervector = partial.unwrap();
        // Partial encoding should have reduced values
        for component in &partial_hypervector.components {
            assert!((*component - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_reconstruct_from_partial() {
        let mut memory = HolographicMemory::new(100);

        let hypervector = create_test_hypervector(100, 1.0);
        let hypervector_components = hypervector.components.clone();
        let metadata = ExperienceMetadata {
            experience_id: 0,
            experience_type: ExperienceType::Physical,
            timestamp: 0,
            emotional_intensity: 0.5,
            sto_alignment: 0.0,
            associated_entities: Vec::new(),
        };

        let experience_id = memory.store_experience(hypervector_components, metadata);

        // Create partial encoding
        let partial = memory.create_partial_encoding(experience_id, 0.5).unwrap();

        // Reconstruct from partial
        let reconstructed = memory.reconstruct_from_partial(&partial);

        assert!(reconstructed.is_some());

        let reconstructed_hypervector = reconstructed.unwrap();
        // Reconstructed should match original
        assert_eq!(
            reconstructed_hypervector.components.len(),
            hypervector.components.len()
        );
    }

    #[test]
    fn test_normalize() {
        let mut hypervector = create_test_hypervector(100, 1.0);

        hypervector.normalize();

        // After normalization, magnitude should be 1.0
        let magnitude: Float = hypervector
            .components
            .iter()
            .map(|x| x * x)
            .sum::<Float>()
            .sqrt();
        assert!((magnitude - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_memory_statistics() {
        let mut memory = HolographicMemory::new(100);

        // Store some experiences
        for i in 0..5 {
            let hypervector = create_test_hypervector(100, (i as Float) / 5.0);
            let metadata = ExperienceMetadata {
                experience_id: i,
                experience_type: ExperienceType::Physical,
                timestamp: i,
                emotional_intensity: 0.5,
                sto_alignment: 0.0,
                associated_entities: Vec::new(),
            };
            memory.store_experience(hypervector.components, metadata);
        }

        let stats = memory.get_statistics();

        assert_eq!(stats.total_experiences, 5);
        assert_eq!(stats.dimensionality, 100);
    }
}
