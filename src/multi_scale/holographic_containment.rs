// Multi-Scale System - Holographic Containment
//
// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
// "The creation is holographic - any portion contains the whole"
// "Each entity contains within it all densities and sub-densities"

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use crate::multi_scale::scales::{EntityId, EntityScale};
use crate::types::Float;

/// HolographicReference - Reference to complete 22-Architecture contained within each entity
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Every portion contains the whole holographically"
#[derive(Debug, Clone)]
pub struct HolographicReference {
    /// Reference to the complete holographic seed
    pub seed: HolographicSeed,

    /// Reference strength (0.0 to 1.0) - how strong the holographic connection is
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "The holographic reference can vary in strength based on entity complexity"
    pub reference_strength: Float,

    /// Fractal dimension - the holographic fractal scaling factor
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Fractal structure with self-similarity across scales"
    pub fractal_dimension: Float,

    /// Self-similarity scale - the scale at which the holographic pattern repeats
    pub self_similarity_scale: Float,
}

impl HolographicReference {
    /// Create a new holographic reference
    pub fn new(seed: HolographicSeed) -> Self {
        HolographicReference {
            seed,
            reference_strength: 1.0,
            fractal_dimension: 1.0,
            self_similarity_scale: 1.0,
        }
    }

    /// Create a holographic reference with custom parameters
    pub fn with_parameters(
        seed: HolographicSeed,
        reference_strength: Float,
        fractal_dimension: Float,
        self_similarity_scale: Float,
    ) -> Self {
        HolographicReference {
            seed,
            reference_strength: reference_strength.clamp(0.0, 1.0),
            fractal_dimension: fractal_dimension.max(0.0),
            self_similarity_scale: self_similarity_scale.max(0.0),
        }
    }

    /// Create a standard holographic reference
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Standard holographic reference with full strength"
    pub fn standard() -> Self {
        Self::new(HolographicSeed::new_from_source())
    }

    /// Check if the holographic reference is valid
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "A valid holographic reference has sufficient strength"
    pub fn is_valid(&self) -> bool {
        self.reference_strength > 0.0
    }

    /// Get the holographic seed
    pub fn get_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    /// Get the reference strength
    pub fn get_reference_strength(&self) -> Float {
        self.reference_strength
    }

    /// Get the fractal dimension
    pub fn get_fractal_dimension(&self) -> Float {
        self.fractal_dimension
    }

    /// Get the self-similarity scale
    pub fn get_self_similarity_scale(&self) -> Float {
        self.self_similarity_scale
    }

    /// Calculate holographic similarity between two references
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Holographic similarity measures how closely two entities share holographic patterns"
    pub fn similarity(&self, other: &HolographicReference) -> Float {
        // Simplified similarity calculation based on reference strength
        // In a full implementation, this would compare archetypal patterns
        (self.reference_strength * other.reference_strength).min(1.0)
    }

    /// Attenuate reference strength (reduce over distance/scale difference)
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Holographic connection attenuates with scale difference"
    pub fn attenuate(&mut self, factor: Float) {
        self.reference_strength = (self.reference_strength * factor.clamp(0.0, 1.0)).max(0.0);
    }
}

impl Default for HolographicReference {
    fn default() -> Self {
        Self::standard()
    }
}

/// HolographicSeed type alias for backward compatibility
///
/// This is a type alias to the HolographicSeed from holographic_seed module
/// maintained for consistency with the knowledge base terminology.
pub type HolographicSeedAlias = HolographicSeed;

/// HolographicContainment - Manages holographic containment relationships
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Each entity contains within it all densities and sub-densities"
#[derive(Debug, Clone)]
pub struct HolographicContainment {
    /// The entity's holographic reference
    pub holographic_reference: HolographicReference,

    /// Contained entities (subscale entities that this entity contains)
    pub contained_entities: Vec<EntityId>,

    /// Parent entities (entities that contain this entity)
    pub parent_entities: Vec<EntityId>,

    /// The scale of this entity
    pub scale: EntityScale,
}

impl HolographicContainment {
    /// Create a new holographic containment
    pub fn new(scale: EntityScale, holographic_reference: HolographicReference) -> Self {
        HolographicContainment {
            holographic_reference,
            contained_entities: Vec::new(),
            parent_entities: Vec::new(),
            scale,
        }
    }

    /// Create a standard holographic containment
    pub fn standard(scale: EntityScale) -> Self {
        Self::new(scale, HolographicReference::standard())
    }

    /// Add a contained entity
    pub fn add_contained_entity(&mut self, entity_id: EntityId) {
        if !self.contained_entities.contains(&entity_id) {
            self.contained_entities.push(entity_id);
        }
    }

    /// Add a parent entity
    pub fn add_parent_entity(&mut self, entity_id: EntityId) {
        if !self.parent_entities.contains(&entity_id) {
            self.parent_entities.push(entity_id);
        }
    }

    /// Remove a contained entity
    pub fn remove_contained_entity(&mut self, entity_id: EntityId) {
        self.contained_entities.retain(|&id| id != entity_id);
    }

    /// Remove a parent entity
    pub fn remove_parent_entity(&mut self, entity_id: EntityId) {
        self.parent_entities.retain(|&id| id != entity_id);
    }

    /// Get contained entities
    pub fn get_contained_entities(&self) -> &[EntityId] {
        &self.contained_entities
    }

    /// Get parent entities
    pub fn get_parent_entities(&self) -> &[EntityId] {
        &self.parent_entities
    }

    /// Get the scale
    pub fn get_scale(&self) -> EntityScale {
        self.scale
    }

    /// Get the holographic reference
    pub fn get_holographic_reference(&self) -> &HolographicReference {
        &self.holographic_reference
    }

    /// Check if this entity has contained entities
    pub fn has_contained_entities(&self) -> bool {
        !self.contained_entities.is_empty()
    }

    /// Check if this entity has parent entities
    pub fn has_parent_entities(&self) -> bool {
        !self.parent_entities.is_empty()
    }

    /// Count contained entities
    pub fn count_contained_entities(&self) -> usize {
        self.contained_entities.len()
    }

    /// Count parent entities
    pub fn count_parent_entities(&self) -> usize {
        self.parent_entities.len()
    }

    /// Check if this entity contains a specific entity
    pub fn contains_entity(&self, entity_id: EntityId) -> bool {
        self.contained_entities.contains(&entity_id)
    }

    /// Check if this entity is contained by a specific entity
    pub fn is_contained_by(&self, entity_id: EntityId) -> bool {
        self.parent_entities.contains(&entity_id)
    }

    /// Check if the holographic reference is valid
    pub fn is_holographically_valid(&self) -> bool {
        self.holographic_reference.is_valid()
    }

    /// Get holographic similarity with another containment
    pub fn holographic_similarity(&self, other: &HolographicContainment) -> Float {
        self.holographic_reference
            .similarity(&other.holographic_reference)
    }
}

impl Default for HolographicContainment {
    fn default() -> Self {
        Self::standard(EntityScale::Organism)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== HolographicReference Tests =====

    #[test]
    fn test_holographic_reference_new() {
        let seed = HolographicSeed::new_from_source();
        let reference = HolographicReference::new(seed);
        assert_eq!(reference.reference_strength, 1.0);
        assert_eq!(reference.fractal_dimension, 1.0);
        assert_eq!(reference.self_similarity_scale, 1.0);
    }

    #[test]
    fn test_holographic_reference_with_parameters() {
        let seed = HolographicSeed::new_from_source();
        let reference = HolographicReference::with_parameters(seed, 0.5, 2.0, 0.8);
        assert_eq!(reference.reference_strength, 0.5);
        assert_eq!(reference.fractal_dimension, 2.0);
        assert_eq!(reference.self_similarity_scale, 0.8);
    }

    #[test]
    fn test_holographic_reference_standard() {
        let reference = HolographicReference::standard();
        assert!(reference.is_valid());
        assert_eq!(reference.reference_strength, 1.0);
    }

    #[test]
    fn test_holographic_reference_is_valid() {
        let seed = HolographicSeed::new_from_source();
        let mut reference = HolographicReference::new(seed);
        assert!(reference.is_valid());

        reference.reference_strength = 0.0;
        assert!(!reference.is_valid());
    }

    #[test]
    fn test_holographic_reference_attenuate() {
        let seed = HolographicSeed::new_from_source();
        let mut reference = HolographicReference::new(seed);
        reference.attenuate(0.5);
        assert_eq!(reference.reference_strength, 0.5);

        reference.attenuate(0.3);
        assert!((reference.reference_strength - 0.15).abs() < 0.001);
    }

    #[test]
    fn test_holographic_reference_similarity() {
        let seed = HolographicSeed::new_from_source();
        let reference1 = HolographicReference::new(seed.clone());
        let reference2 = HolographicReference::new(seed);

        assert_eq!(reference1.similarity(&reference2), 1.0);
    }

    #[test]
    fn test_holographic_reference_default() {
        let reference = HolographicReference::default();
        assert!(reference.is_valid());
    }

    // ===== HolographicContainment Tests =====

    #[test]
    fn test_holographic_containment_new() {
        let reference = HolographicReference::standard();
        let containment = HolographicContainment::new(EntityScale::Atomic, reference);
        assert_eq!(containment.get_scale(), EntityScale::Atomic);
        assert!(!containment.has_contained_entities());
        assert!(!containment.has_parent_entities());
    }

    #[test]
    fn test_holographic_containment_standard() {
        let containment = HolographicContainment::standard(EntityScale::Molecular);
        assert_eq!(containment.get_scale(), EntityScale::Molecular);
        assert!(containment.is_holographically_valid());
    }

    #[test]
    fn test_holographic_containment_add_contained_entity() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_contained_entity(1);
        containment.add_contained_entity(2);
        containment.add_contained_entity(1); // Duplicate

        assert_eq!(containment.count_contained_entities(), 2);
        assert!(containment.has_contained_entities());
    }

    #[test]
    fn test_holographic_containment_add_parent_entity() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_parent_entity(1);
        containment.add_parent_entity(2);
        containment.add_parent_entity(1); // Duplicate

        assert_eq!(containment.count_parent_entities(), 2);
        assert!(containment.has_parent_entities());
    }

    #[test]
    fn test_holographic_containment_remove_contained_entity() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_contained_entity(1);
        containment.add_contained_entity(2);
        containment.remove_contained_entity(1);

        assert_eq!(containment.count_contained_entities(), 1);
        assert!(!containment.contains_entity(1));
        assert!(containment.contains_entity(2));
    }

    #[test]
    fn test_holographic_containment_remove_parent_entity() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_parent_entity(1);
        containment.add_parent_entity(2);
        containment.remove_parent_entity(1);

        assert_eq!(containment.count_parent_entities(), 1);
        assert!(!containment.is_contained_by(1));
        assert!(containment.is_contained_by(2));
    }

    #[test]
    fn test_holographic_containment_contains_entity() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_contained_entity(1);
        containment.add_contained_entity(2);

        assert!(containment.contains_entity(1));
        assert!(containment.contains_entity(2));
        assert!(!containment.contains_entity(3));
    }

    #[test]
    fn test_holographic_containment_is_contained_by() {
        let mut containment = HolographicContainment::standard(EntityScale::Atomic);
        containment.add_parent_entity(1);
        containment.add_parent_entity(2);

        assert!(containment.is_contained_by(1));
        assert!(containment.is_contained_by(2));
        assert!(!containment.is_contained_by(3));
    }

    #[test]
    fn test_holographic_containment_holographic_similarity() {
        let containment1 = HolographicContainment::standard(EntityScale::Atomic);
        let containment2 = HolographicContainment::standard(EntityScale::Molecular);

        assert_eq!(containment1.holographic_similarity(&containment2), 1.0);
    }

    #[test]
    fn test_holographic_containment_default() {
        let containment = HolographicContainment::default();
        assert_eq!(containment.get_scale(), EntityScale::Organism);
        assert!(containment.is_holographically_valid());
    }

    // ===== Integration Tests =====

    #[test]
    fn test_holographic_containment_hierarchy() {
        // Create a hierarchy: Molecular -> Atomic -> Quantum
        let mut molecular = HolographicContainment::standard(EntityScale::Molecular);
        let mut atomic = HolographicContainment::standard(EntityScale::Atomic);
        let mut quantum = HolographicContainment::standard(EntityScale::Quantum);

        // Build hierarchy
        molecular.add_contained_entity(2);
        atomic.add_contained_entity(3);
        atomic.add_parent_entity(1);
        quantum.add_parent_entity(2);

        // Verify hierarchy
        assert!(molecular.contains_entity(2));
        assert!(atomic.contains_entity(3));
        assert!(atomic.is_contained_by(1));
        assert!(quantum.is_contained_by(2));
    }

    #[test]
    fn test_holographic_reference_attenuation_chain() {
        let seed = HolographicSeed::new_from_source();
        let mut reference = HolographicReference::new(seed);

        // Simulate attenuation through scale levels
        reference.attenuate(0.9); // Quantum -> Atomic
        reference.attenuate(0.8); // Atomic -> Molecular
        reference.attenuate(0.7); // Molecular -> Cellular

        assert!((reference.reference_strength - 0.504).abs() < 0.001);
    }
}
