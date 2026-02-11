//! Holographic Memory System (Soul Stream)
//!
//! This module implements the holographic memory system that models the Soul Stream
//! as a high-dimensional hypervector storage system.
//!
//! # Core Concepts
//!
//! - **HolographicMemory**: Stores experiences as hypervectors in high-dimensional space
//! - **Hypervector**: 10,000-dimensional vector for encoding experiences
//! - **Experience**: A catalyst, emotional tone, and learning
//! - **AssociativeMemory**: Associative retrieval through similarity search
//!
//! # Holographic Memory Principles
//!
//! - **Distributed encoding**: Information is distributed across all dimensions
//! - **Similarity-based retrieval**: Retrieve memories through cosine similarity
//! - **Fault tolerance**: Graceful degradation (no single point of failure)
//! - **Binding and bundling**: Combine hypervectors for complex representations
//!
//! # Cosine Similarity
//!
//! Cosine similarity measures the similarity between two hypervectors:
//!
//! similarity = (A · B) / (||A|| * ||B||)
//!
//! Where:
//! - A · B is the dot product
//! - ||A|| is the norm of A
//!
//! Similarity ranges from -1.0 (opposite) to 1.0 (identical).

use std::collections::HashMap;

/// Float type for holographic calculations
pub type Float = f64;

/// A hypervector for encoding experiences.
///
/// Hypervectors are high-dimensional vectors (10,000 dimensions) that encode
/// experiences in a distributed, fault-tolerant manner.
#[derive(Clone, Debug)]
pub struct Hypervector {
    /// The vector components (10,000 dimensions)
    pub components: Vec<Float>,
}

impl Hypervector {
    /// Creates a new random hypervector with the specified dimensionality.
    ///
    /// # Arguments
    ///
    /// * `dimensionality` - The number of dimensions (default: 10,000)
    ///
    /// # Returns
    ///
    /// A new random hypervector with components in range [-1.0, 1.0]
    pub fn new(dimensionality: usize) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut components = Vec::with_capacity(dimensionality);

        // Generate pseudorandom components using a simple hash function
        let mut hasher = DefaultHasher::new();
        for i in 0..dimensionality {
            i.hash(&mut hasher);
            let hash = hasher.finish();
            components.push((hash as Float / u64::MAX as Float) * 2.0 - 1.0);
        }

        Hypervector { components }
    }

    /// Creates a new hypervector from the specified components.
    ///
    /// # Arguments
    ///
    /// * `components` - The vector components
    ///
    /// # Returns
    ///
    /// A new hypervector
    pub fn from_components(components: Vec<Float>) -> Self {
        Hypervector { components }
    }

    /// Calculates the cosine similarity between this hypervector and another.
    ///
    /// Cosine similarity measures the similarity between two vectors:
    ///
    /// similarity = (A · B) / (||A|| * ||B||)
    ///
    /// # Arguments
    ///
    /// * `other` - The other hypervector
    ///
    /// # Returns
    ///
    /// Cosine similarity (-1.0 to 1.0)
    pub fn cosine_similarity(&self, other: &Hypervector) -> Float {
        if self.components.is_empty() || other.components.is_empty() {
            return 0.0;
        }

        // Calculate dot product
        let dot_product: Float = self
            .components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum();

        // Calculate norms
        let norm_a: Float = self
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();
        let norm_b: Float = other
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }

    /// Binds this hypervector with another through circular convolution.
    ///
    /// Binding combines two hypervectors into a new one that represents
    /// their relationship.
    ///
    /// # Arguments
    ///
    /// * `other` - The other hypervector
    ///
    /// # Returns
    ///
    /// A new hypervector representing the binding
    pub fn bind(&self, other: &Hypervector) -> Hypervector {
        let n = self.components.len();
        let mut result = Hypervector::new(n);

        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                let k = (i + j) % n;
                sum += self.components[j] * other.components[k];
            }
            result.components[i] = sum / n as Float;
        }

        result
    }

    /// Bundles this hypervector with others through addition.
    ///
    /// Bundling combines multiple hypervectors into a single representation.
    ///
    /// # Arguments
    ///
    /// * `others` - The other hypervectors to bundle with
    ///
    /// # Returns
    ///
    /// A new hypervector representing the bundle
    pub fn bundle(&self, others: &[Hypervector]) -> Hypervector {
        let mut result = self.clone();

        for other in others {
            for i in 0..result.components.len().min(other.components.len()) {
                result.components[i] += other.components[i];
            }
        }

        // Normalize
        let norm: Float = result
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();
        if norm > 0.0 {
            for component in &mut result.components {
                *component /= norm;
            }
        }

        result
    }

    /// Normalizes this hypervector to unit length.
    pub fn normalize(&mut self) {
        let norm: Float = self
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();
        if norm > 0.0 {
            for component in &mut self.components {
                *component /= norm;
            }
        }
    }

    /// Returns the dimensionality of this hypervector.
    ///
    /// # Returns
    ///
    /// Number of dimensions
    pub fn dimensionality(&self) -> usize {
        self.components.len()
    }
}

/// An experience stored in holographic memory.
///
/// Experiences are encoded as hypervectors and stored in the Soul Stream.
#[derive(Clone, Debug)]
pub struct Experience {
    /// The catalyst that triggered the experience
    pub catalyst: String,

    /// Emotional tone (-1.0 negative to 1.0 positive)
    pub emotional_tone: Float,

    /// The learning from the experience
    pub learning: String,

    /// Timestamp of the experience
    pub timestamp: u64,
}

impl Experience {
    /// Creates a new experience.
    ///
    /// # Arguments
    ///
    /// * `catalyst` - The catalyst
    /// * `emotional_tone` - The emotional tone (-1.0 to 1.0)
    /// * `learning` - The learning
    /// * `timestamp` - The timestamp
    ///
    /// # Returns
    ///
    /// A new experience
    pub fn new(catalyst: String, emotional_tone: Float, learning: String, timestamp: u64) -> Self {
        Experience {
            catalyst,
            emotional_tone: emotional_tone.clamp(-1.0, 1.0),
            learning,
            timestamp,
        }
    }
}

/// Associative memory for holographic memory system.
///
/// Associative memory provides similarity-based retrieval of experiences.
#[derive(Clone, Debug)]
pub struct AssociativeMemory {
    /// Index of hypervectors by catalyst hash
    index: HashMap<u64, usize>,
}

impl AssociativeMemory {
    /// Creates a new associative memory.
    ///
    /// # Returns
    ///
    /// A new associative memory
    pub fn new() -> Self {
        AssociativeMemory {
            index: HashMap::new(),
        }
    }

    /// Adds an experience to the index.
    ///
    /// # Arguments
    ///
    /// * `experience` - The experience to add
    /// * `index` - The index in the memory store
    pub fn add(&mut self, experience: &Experience, index: usize) {
        let hash = self.hash_string(&experience.catalyst);
        self.index.insert(hash, index);
    }

    /// Finds experiences similar to a catalyst.
    ///
    /// # Arguments
    ///
    /// * `catalyst` - The catalyst to search for
    ///
    /// # Returns
    ///
    /// Vector of indices of similar experiences
    pub fn find_similar(&self, catalyst: &str) -> Vec<usize> {
        let hash = self.hash_string(catalyst);

        // In a full implementation, this would use similarity search
        // For now, return exact matches
        self.index
            .get(&hash)
            .map(|&index| vec![index])
            .unwrap_or_default()
    }

    /// Hashes a string to a u64.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to hash
    ///
    /// # Returns
    ///
    /// Hash value
    fn hash_string(&self, s: &str) -> u64 {
        let mut hash: u64 = 5381;
        for c in s.chars() {
            hash = hash.wrapping_mul(33).wrapping_add(c as u64);
        }
        hash
    }
}

impl Default for AssociativeMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Holographic memory system (Soul Stream).
///
/// The holographic memory system stores experiences as hypervectors in
/// high-dimensional space, enabling similarity-based retrieval and
/// fault-tolerant storage.
///
/// # Fields
///
/// - `memory_store`: The stored experience hypervectors
/// - `dimensionality`: The dimensionality of hypervectors (default: 10,000)
/// - `associative_memory`: Associative index for similarity search
#[derive(Clone, Debug)]
pub struct HolographicMemory {
    /// The stored experience hypervectors
    pub memory_store: Vec<Hypervector>,

    /// The dimensionality of hypervectors
    pub dimensionality: usize,

    /// The associative memory index
    pub associative_memory: AssociativeMemory,

    /// The stored experiences (for retrieval)
    pub experiences: Vec<Experience>,
}

impl HolographicMemory {
    /// Creates a new holographic memory system.
    ///
    /// # Returns
    ///
    /// A new holographic memory system with 10,000 dimensions
    pub fn new() -> Self {
        HolographicMemory {
            memory_store: Vec::new(),
            dimensionality: 10000,
            associative_memory: AssociativeMemory::new(),
            experiences: Vec::new(),
        }
    }

    /// Creates a new holographic memory system with specified dimensionality.
    ///
    /// # Arguments
    ///
    /// * `dimensionality` - The dimensionality of hypervectors
    ///
    /// # Returns
    ///
    /// A new holographic memory system
    pub fn with_dimensionality(dimensionality: usize) -> Self {
        HolographicMemory {
            memory_store: Vec::new(),
            dimensionality,
            associative_memory: AssociativeMemory::new(),
            experiences: Vec::new(),
        }
    }

    /// Adds an experience to the holographic memory.
    ///
    /// # Arguments
    ///
    /// * `experience` - The experience to add
    pub fn add_experience(&mut self, experience: Experience) {
        // Convert experience to hypervector
        let hypervector = self.experience_to_hypervector(&experience);

        // Add to memory store
        let index = self.memory_store.len();
        self.memory_store.push(hypervector);
        self.experiences.push(experience.clone());

        // Add to associative memory
        self.associative_memory.add(&experience, index);
    }

    /// Retrieves memories similar to a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query experience
    ///
    /// # Returns
    ///
    /// Vector of similar experiences
    pub fn retrieve_memory(&self, query: Experience) -> Vec<Experience> {
        if self.memory_store.is_empty() {
            return Vec::new();
        }

        // Convert query to hypervector
        let query_hypervector = self.experience_to_hypervector(&query);

        // Similarity-based search
        let similar_memories: Vec<(usize, Float)> = self
            .memory_store
            .iter()
            .enumerate()
            .map(|(i, mem)| (i, query_hypervector.cosine_similarity(mem)))
            .filter(|(_, sim)| *sim > 0.8) // Threshold for similarity
            .collect();

        // Sort by similarity (descending)
        let mut sorted = similar_memories;
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return top matches
        sorted
            .iter()
            .take(5) // Top 5 matches
            .filter_map(|(i, _)| self.experiences.get(*i).cloned())
            .collect()
    }

    /// Returns the number of experiences stored.
    ///
    /// # Returns
    ///
    /// Number of experiences
    pub fn len(&self) -> usize {
        self.memory_store.len()
    }

    /// Returns whether the memory is empty.
    ///
    /// # Returns
    ///
    /// True if empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.memory_store.is_empty()
    }

    /// Converts an experience to a hypervector.
    ///
    /// # Arguments
    ///
    /// * `experience` - The experience to convert
    ///
    /// # Returns
    ///
    /// A hypervector encoding the experience
    fn experience_to_hypervector(&self, experience: &Experience) -> Hypervector {
        let mut hypervector = Hypervector::new(self.dimensionality);

        // Encode emotional tone in first dimension
        hypervector.components[0] = experience.emotional_tone;

        // Encode catalyst in next dimensions (hash-based)
        let catalyst_hash = self.hash_string(&experience.catalyst);
        for i in 1..self.dimensionality.min(101) {
            hypervector.components[i] = ((catalyst_hash >> (i % 64)) & 1) as Float * 2.0 - 1.0;
        }

        // Encode timestamp in remaining dimensions
        let timestamp_hash = self.hash_u64(experience.timestamp);
        for i in 101..self.dimensionality.min(201) {
            hypervector.components[i] =
                ((timestamp_hash >> ((i - 101) % 64)) & 1) as Float * 2.0 - 1.0;
        }

        hypervector
    }

    /// Hashes a string to a u64.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to hash
    ///
    /// # Returns
    ///
    /// Hash value
    fn hash_string(&self, s: &str) -> u64 {
        let mut hash: u64 = 5381;
        for c in s.chars() {
            hash = hash.wrapping_mul(33).wrapping_add(c as u64);
        }
        hash
    }

    /// Hashes a u64 to a u64.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to hash
    ///
    /// # Returns
    ///
    /// Hash value
    fn hash_u64(&self, n: u64) -> u64 {
        let mut hash: u64 = 5381;
        hash = hash.wrapping_mul(33).wrapping_add(n);
        hash
    }
}

impl Default for HolographicMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypervector_creation() {
        let hypervector = Hypervector::new(1000);

        assert_eq!(hypervector.dimensionality(), 1000);
        assert_eq!(hypervector.components.len(), 1000);
    }

    #[test]
    fn test_hypervector_from_components() {
        let components = vec![0.5, -0.3, 0.7];
        let hypervector = Hypervector::from_components(components.clone());

        assert_eq!(hypervector.components, components);
    }

    #[test]
    fn test_hypervector_cosine_similarity() {
        let hv1 = Hypervector::from_components(vec![1.0, 0.0, 0.0]);
        let hv2 = Hypervector::from_components(vec![1.0, 0.0, 0.0]);
        let hv3 = Hypervector::from_components(vec![0.0, 1.0, 0.0]);

        assert!((hv1.cosine_similarity(&hv2) - 1.0).abs() < 1e-10); // Identical
        assert!((hv1.cosine_similarity(&hv3) - 0.0).abs() < 1e-10); // Orthogonal
    }

    #[test]
    fn test_hypervector_bind() {
        let hv1 = Hypervector::from_components(vec![1.0, 0.0]);
        let hv2 = Hypervector::from_components(vec![0.0, 1.0]);

        let bound = hv1.bind(&hv2);
        assert_eq!(bound.dimensionality(), 2);
    }

    #[test]
    fn test_hypervector_bundle() {
        let hv1 = Hypervector::from_components(vec![1.0, 0.0]);
        let hv2 = Hypervector::from_components(vec![0.0, 1.0]);

        let bundled = hv1.bundle(&[hv2]);
        assert_eq!(bundled.dimensionality(), 2);
    }

    #[test]
    fn test_hypervector_normalize() {
        let mut hv = Hypervector::from_components(vec![3.0, 4.0]);
        hv.normalize();

        let norm: Float = hv
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_experience_creation() {
        let experience = Experience::new(
            "Test catalyst".to_string(),
            0.5,
            "Test learning".to_string(),
            12345,
        );

        assert_eq!(experience.catalyst, "Test catalyst");
        assert_eq!(experience.emotional_tone, 0.5);
        assert_eq!(experience.learning, "Test learning");
        assert_eq!(experience.timestamp, 12345);
    }

    #[test]
    fn test_experience_emotional_tone_clamping() {
        let experience = Experience::new("Test".to_string(), 1.5, "Test".to_string(), 0);
        assert_eq!(experience.emotional_tone, 1.0);

        let experience = Experience::new("Test".to_string(), -1.5, "Test".to_string(), 0);
        assert_eq!(experience.emotional_tone, -1.0);
    }

    #[test]
    fn test_associative_memory_creation() {
        let memory = AssociativeMemory::new();
        assert_eq!(memory.index.len(), 0);
    }

    #[test]
    fn test_associative_memory_add() {
        let mut memory = AssociativeMemory::new();
        let experience = Experience::new(
            "Test catalyst".to_string(),
            0.5,
            "Test learning".to_string(),
            0,
        );

        memory.add(&experience, 0);
        assert_eq!(memory.index.len(), 1);
    }

    #[test]
    fn test_associative_memory_find_similar() {
        let mut memory = AssociativeMemory::new();
        let experience = Experience::new(
            "Test catalyst".to_string(),
            0.5,
            "Test learning".to_string(),
            0,
        );

        memory.add(&experience, 0);
        let similar = memory.find_similar("Test catalyst");
        assert_eq!(similar.len(), 1);
        assert_eq!(similar[0], 0);
    }

    #[test]
    fn test_holographic_memory_creation() {
        let memory = HolographicMemory::new();

        assert_eq!(memory.dimensionality, 10000);
        assert!(memory.is_empty());
    }

    #[test]
    fn test_holographic_memory_with_dimensionality() {
        let memory = HolographicMemory::with_dimensionality(5000);

        assert_eq!(memory.dimensionality, 5000);
    }

    #[test]
    fn test_holographic_memory_add_experience() {
        let mut memory = HolographicMemory::new();
        let experience = Experience::new(
            "Test catalyst".to_string(),
            0.5,
            "Test learning".to_string(),
            12345,
        );

        memory.add_experience(experience);

        assert_eq!(memory.len(), 1);
        assert!(!memory.is_empty());
    }

    #[test]
    fn test_holographic_memory_retrieve_memory() {
        let mut memory = HolographicMemory::new();
        let experience = Experience::new(
            "Test catalyst".to_string(),
            0.5,
            "Test learning".to_string(),
            12345,
        );

        memory.add_experience(experience.clone());

        let retrieved = memory.retrieve_memory(experience);
        assert!(!retrieved.is_empty());
    }

    #[test]
    fn test_holographic_memory_retrieve_empty() {
        let memory = HolographicMemory::new();
        let experience = Experience::new("Test".to_string(), 0.5, "Test".to_string(), 0);

        let retrieved = memory.retrieve_memory(experience);
        assert!(retrieved.is_empty());
    }

    #[test]
    fn test_holographic_memory_len() {
        let mut memory = HolographicMemory::new();

        assert_eq!(memory.len(), 0);

        let experience = Experience::new("Test".to_string(), 0.5, "Test".to_string(), 0);
        memory.add_experience(experience);

        assert_eq!(memory.len(), 1);
    }

    #[test]
    fn test_holographic_memory_default() {
        let memory = HolographicMemory::default();

        assert_eq!(memory.dimensionality, 10000);
        assert!(memory.is_empty());
    }

    #[test]
    fn test_hypervector_dimensionality() {
        let hypervector = Hypervector::new(1000);
        assert_eq!(hypervector.dimensionality(), 1000);
    }

    #[test]
    fn test_associative_memory_default() {
        let memory = AssociativeMemory::default();
        assert_eq!(memory.index.len(), 0);
    }
}
