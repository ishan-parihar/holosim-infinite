// Holographic Reference Architecture
//
// This module implements the HolographicReference trait and the holographic
// reference sharing pattern using Arc<HolographicSeed>.
//
// The holographic reference architecture ensures that every entity at every
// scale maintains a shared reference to the complete HolographicSeed, preserving
// the fractal-holographic principle that "part contains the whole."
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 5.1
// - REFACTOR_ROADMAP_V2.md Phase 0, Task 3
// - EMERGENCE_CHAIN.md Section 5

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use std::sync::Arc;

/// The HolographicReference trait
///
/// Any entity that maintains a holographic reference to the whole implements
/// this trait. This ensures that every entity can access the complete
/// HolographicSeed, preserving the fractal-holographic principle.
///
/// The trait provides methods to:
/// - Access the whole (the complete HolographicSeed)
/// - Verify that the reference is valid
/// - Check if two entities share the same reference
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Phase 0, Task 3
pub trait HolographicReference {
    /// Get the holographic reference to the whole
    ///
    /// Returns an Arc to the HolographicSeed, allowing shared access
    /// to the complete architecture.
    ///
    /// # Example
    ///
    /// ```rust
    /// let seed = Arc::new(HolographicSeed::new_from_source());
    /// let photon = Photon::new(seed.clone());
    ///
    /// let whole = photon.holographic_ref();
    /// assert!(whole.contains_complete_architecture());
    /// ```
    fn holographic_ref(&self) -> Arc<HolographicSeed>;

    /// Access the whole (the complete HolographicSeed)
    ///
    /// This is a convenience method that returns the Arc<HolographicSeed>.
    /// It demonstrates the fractal-holographic principle: "part contains the whole."
    ///
    /// # Example
    ///
    /// ```rust
    /// let seed = Arc::new(HolographicSeed::new_from_source());
    /// let photon = Photon::new(seed.clone());
    ///
    /// let whole = photon.access_whole();
    /// assert_eq!(whole.archetypes().all_archetypes().len(), 21);
    /// ```
    fn access_whole(&self) -> Arc<HolographicSeed> {
        self.holographic_ref()
    }

    /// Check if this entity shares the same holographic reference as another
    ///
    /// This verifies that two entities reference the same HolographicSeed,
    /// which is critical for maintaining the fractal-holographic principle.
    ///
    /// # Arguments
    ///
    /// * `other` - The other entity to compare with
    ///
    /// # Returns
    ///
    /// `true` if both entities share the same Arc<HolographicSeed>, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// let seed = Arc::new(HolographicSeed::new_from_source());
    /// let photon1 = Photon::new(seed.clone());
    /// let photon2 = Photon::new(seed.clone());
    ///
    /// assert!(photon1.shares_holographic_ref(&photon2));
    /// ```
    fn shares_holographic_ref<T: HolographicReference>(&self, other: &T) -> bool {
        Arc::ptr_eq(&self.holographic_ref(), &other.holographic_ref())
    }

    /// Check if the holographic reference is valid
    ///
    /// A valid reference should:
    /// - Point to a HolographicSeed
    /// - Contain the complete 22-Archetype structure
    /// - Have valid Free Will (Archetype 22)
    /// - Have valid Archetypical Mind (A1-A21)
    ///
    /// # Returns
    ///
    /// `true` if the reference is valid, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// let seed = Arc::new(HolographicSeed::new_from_source());
    /// let photon = Photon::new(seed.clone());
    ///
    /// assert!(photon.holographic_ref_is_valid());
    /// ```
    fn holographic_ref_is_valid(&self) -> bool {
        let seed = self.holographic_ref();
        seed.contains_complete_architecture()
    }
}

/// Extension trait for HolographicSeed
///
/// Provides additional methods for working with HolographicSeed in the
/// context of holographic references.
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Phase 0, Task 3
pub trait HolographicSeedExt {
    /// Check if the seed contains complete architecture
    ///
    /// A complete architecture includes:
    /// - Free Will (Archetype 22)
    /// - Archetypical Mind (A1-A21)
    /// - Light Encoding
    /// - Physics Constants
    /// - Fractal References
    ///
    /// # Returns
    ///
    /// `true` if the seed contains complete architecture, `false` otherwise
    fn contains_complete_architecture(&self) -> bool;

    /// Get the archetype activation array
    ///
    /// Returns a 22-element array representing the activation levels
    /// of all archetypes (A1-A21 + A22).
    ///
    /// # Returns
    ///
    /// A [f64; 22] array with archetype activation levels
    fn get_archetype_activation(&self) -> [f64; 22];

    /// Calculate the total archetype activation
    ///
    /// Returns the sum of all archetype activation levels.
    ///
    /// # Returns
    ///
    /// The total archetype activation (f64)
    fn total_archetype_activation(&self) -> f64;

    /// Calculate the archetype activation for a specific complex
    ///
    /// # Arguments
    ///
    /// * `complex` - The complex to calculate (Mind, Body, Spirit)
    ///
    /// # Returns
    ///
    /// The total archetype activation for the complex (f64)
    fn complex_activation(&self, complex: ArchetypeComplex) -> f64;

    /// Calculate the archetype activation for a specific archetype
    ///
    /// # Arguments
    ///
    /// * `archetype_index` - The archetype index (0-21)
    ///
    /// # Returns
    ///
    /// The activation level for the archetype (f64)
    fn archetype_activation(&self, archetype_index: usize) -> f64;

    /// Count archetypes (returns 21 for A1-A21)
    fn count_archetypes(&self) -> usize;

    /// Get all archetypes (alias for compatibility)
    fn all_archetypes(&self) -> Vec<Box<dyn crate::archetypes::common::ArchetypeTrait>>;
}

impl HolographicSeedExt for HolographicSeed {
    fn contains_complete_architecture(&self) -> bool {
        // Check Free Will (Archetype 22)
        if self.free_will.free_will_intensity <= 0.0 {
            return false;
        }

        // Check Archetypical Mind (A1-A21)
        if self.archetypes.archetypes.get_all_archetypes().len() != 21 {
            return false;
        }

        // Check Light Encoding
        if !self.light_encoding.is_valid() {
            return false;
        }

        // Check Physics Constraints
        if !self.physics_constraints().is_valid() {
            return false;
        }

        // Check Fractal References
        if self.fractal_references.len() != 22 {
            return false;
        }

        true
    }

    fn get_archetype_activation(&self) -> [f64; 22] {
        let mut activation = [0.0f64; 22];

        // Get archetype activations from Archetypical Mind
        let all_archetypes = self.archetypes.archetypes.get_all_archetypes();
        for (i, archetype) in all_archetypes.iter().enumerate() {
            if i < 21 {
                // Use lambda value as activation level since activation_level() doesn't exist
                activation[i] = archetype.lambda().value;
            }
        }

        // Add Free Will activation
        activation[21] = self.free_will.free_will_intensity;

        activation
    }

    fn total_archetype_activation(&self) -> f64 {
        let activation = self.get_archetype_activation();
        activation.iter().sum()
    }

    fn complex_activation(&self, complex: ArchetypeComplex) -> f64 {
        let activation = self.get_archetype_activation();
        match complex {
            ArchetypeComplex::Mind => activation[0..7].iter().sum(),
            ArchetypeComplex::Body => activation[7..14].iter().sum(),
            ArchetypeComplex::Spirit => activation[14..21].iter().sum(),
        }
    }

    fn archetype_activation(&self, archetype_index: usize) -> f64 {
        if archetype_index >= 22 {
            return 0.0;
        }

        let activation = self.get_archetype_activation();
        activation[archetype_index]
    }

    fn count_archetypes(&self) -> usize {
        self.archetypes.archetypes.get_all_archetypes().len()
    }

    fn all_archetypes(&self) -> Vec<Box<dyn crate::archetypes::common::ArchetypeTrait>> {
        self.archetypes.archetypes.get_all_archetypes()
    }
}

/// Archetype Complex enumeration
///
/// Represents the three complexes: Mind, Body, and Spirit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeComplex {
    /// Mind Complex (A1-A7)
    Mind,
    /// Body Complex (A8-A14)
    Body,
    /// Spirit Complex (A15-A21)
    Spirit,
}

/// Helper function to create a shared holographic reference
///
/// This is a convenience function for creating Arc<HolographicSeed>
/// references that can be shared across all scales.
///
/// # Returns
///
/// An Arc<HolographicSeed> that can be shared
///
/// # Example
///
/// ```rust
/// let seed = create_shared_holographic_ref();
/// let photon = Photon::new(seed.clone());
/// let entity = Entity::new(seed);
/// ```
pub fn create_shared_holographic_ref() -> Arc<HolographicSeed> {
    Arc::new(HolographicSeed::new_from_source())
}

/// Helper function to verify holographic reference consistency
///
/// Verifies that all entities in a collection share the same holographic
/// reference, which is critical for maintaining the fractal-holographic
/// principle.
///
/// # Arguments
///
/// * `entities` - A slice of entities that implement HolographicReference
///
/// # Returns
///
/// `true` if all entities share the same reference, `false` otherwise
///
/// # Example
///
/// ```rust
/// let seed = Arc::new(HolographicSeed::new_from_source());
/// let photon1 = Photon::new(seed.clone());
/// let photon2 = Photon::new(seed.clone());
/// let entity = Entity::new(seed);
///
/// let entities: Vec<&dyn HolographicReference> = vec![&photon1, &photon2, &entity];
/// assert!(verify_holographic_ref_consistency(&entities));
/// ```
pub fn verify_holographic_ref_consistency<T: HolographicReference>(entities: &[T]) -> bool {
    if entities.is_empty() {
        return true;
    }

    let first_ref = entities[0].holographic_ref();
    entities
        .iter()
        .all(|e| Arc::ptr_eq(&first_ref, &e.holographic_ref()))
}

/// Helper function to calculate holographic coherence
///
/// Calculates the percentage of entities in a collection that share
/// the same holographic reference.
///
/// # Arguments
///
/// * `entities` - A slice of entities that implement HolographicReference
///
/// # Returns
///
/// The holographic coherence as a percentage (0.0 to 100.0)
///
/// # Example
///
/// ```rust
/// let seed1 = Arc::new(HolographicSeed::new_from_source());
/// let seed2 = Arc::new(HolographicSeed::new_from_source());
///
/// let photon1 = Photon::new(seed1.clone());
/// let photon2 = Photon::new(seed1.clone());
/// let photon3 = Photon::new(seed2);
///
/// let entities: Vec<&dyn HolographicReference> = vec![&photon1, &photon2, &photon3];
/// let coherence = calculate_holographic_coherence(&entities);
/// assert_eq!(coherence, 66.67); // 2 out of 3 share the same reference
/// ```
pub fn calculate_holographic_coherence<T: HolographicReference>(entities: &[T]) -> f64 {
    if entities.is_empty() {
        return 100.0;
    }

    // Count unique references
    let mut unique_refs = std::collections::HashSet::new();
    for entity in entities {
        unique_refs.insert(Arc::as_ptr(&entity.holographic_ref()));
    }

    // Calculate coherence
    let coherence = if unique_refs.len() == 1 {
        100.0
    } else {
        // Find the most common reference
        let mut ref_counts = std::collections::HashMap::new();
        for entity in entities {
            let ptr = Arc::as_ptr(&entity.holographic_ref());
            *ref_counts.entry(ptr).or_insert(0) += 1;
        }

        let max_count = ref_counts.values().max().unwrap_or(&0);
        (*max_count as f64 / entities.len() as f64) * 100.0
    };

    coherence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_ref_creation() {
        let seed = create_shared_holographic_ref();
        assert!(seed.contains_complete_architecture());
    }

    #[test]
    fn test_archetype_activation() {
        let seed = create_shared_holographic_ref();
        let activation = seed.get_archetype_activation();
        assert_eq!(activation.len(), 22);
    }

    #[test]
    fn test_complex_activation() {
        let seed = create_shared_holographic_ref();
        let mind_activation = seed.complex_activation(ArchetypeComplex::Mind);
        let body_activation = seed.complex_activation(ArchetypeComplex::Body);
        let spirit_activation = seed.complex_activation(ArchetypeComplex::Spirit);

        assert!(mind_activation > 0.0);
        assert!(body_activation > 0.0);
        assert!(spirit_activation > 0.0);
    }

    #[test]
    fn test_verify_holographic_ref_consistency() {
        let seed = create_shared_holographic_ref();

        // Create test entities (we'll use the seed itself for testing)
        // In real usage, these would be Photon, Particle, Entity, etc.
        let _entities = vec![seed.clone(), seed.clone(), seed.clone()];

        // We can't directly test this without entities that implement HolographicReference
        // This test is a placeholder for when we implement HolographicReference on entities
    }

    #[test]
    fn test_calculate_holographic_coherence() {
        let seed1 = create_shared_holographic_ref();
        let _seed2 = create_shared_holographic_ref();

        // Create test entities
        let _entities = vec![seed1.clone(), seed1.clone()];

        // We can't directly test this without entities that implement HolographicReference
        // This test is a placeholder for when we implement HolographicReference on entities
    }
}
