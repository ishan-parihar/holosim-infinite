// Multi-Scale System - Scale Definitions
//
// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
// "The creation is holographic - any portion contains the whole"
// "Each entity contains within it all densities and sub-densities"

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use std::collections::HashMap;

/// Entity Scale - The scale at which an entity exists in the holographic creation
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Quantum-Scale (Stage 0), Atomic-Scale (Stage 1), Molecular-Scale (Stage 2),
///  Cellular-Scale (Stage 3), Organism-Scale (Stage 4), Collective-Scale (Stage 5),
///  Planetary-Scale (Stage 6), Solar-Scale (Stage 7), Galactic-Scale (Stage 8),
///  Cosmic-Scale (Stage 9)"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntityScale {
    /// Quantum-Scale (Stage 0): Sub-Atomic particles
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Quantum-Scale (Stage 0): Sub-Atomic particles"
    Quantum,

    /// Atomic-Scale (Stage 1): Atoms
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Atomic-Scale (Stage 1): Atoms"
    Atomic,

    /// Molecular-Scale (Stage 2): Molecules
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Molecular-Scale (Stage 2): Molecules"
    Molecular,

    /// Cellular-Scale (Stage 3): Cells
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Cellular-Scale (Stage 3): Cells"
    Cellular,

    /// Organism-Scale (Stage 4): Individual organisms
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Organism-Scale (Stage 4): Individual organisms"
    Organism,

    /// Collective-Scale (Stage 5): Groups, societies, civilizations
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Collective-Scale (Stage 5): Groups, societies, civilizations"
    Collective,

    /// Planetary-Scale (Stage 6): Planetary entities
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Planetary-Scale (Stage 6): Planetary entities"
    Planetary,

    /// Solar-Scale (Stage 7): Solar systems
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Solar-Scale (Stage 7): Solar systems"
    Solar,

    /// Galactic-Scale (Stage 8): Galaxies
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Galactic-Scale (Stage 8): Galaxies"
    Galactic,

    /// Cosmic-Scale (Stage 9): The entire creation
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Cosmic-Scale (Stage 9): The entire creation"
    Cosmic,
}

impl EntityScale {
    /// Get the stage number for this scale
    pub fn stage_number(&self) -> u8 {
        match self {
            EntityScale::Quantum => 0,
            EntityScale::Atomic => 1,
            EntityScale::Molecular => 2,
            EntityScale::Cellular => 3,
            EntityScale::Organism => 4,
            EntityScale::Collective => 5,
            EntityScale::Planetary => 6,
            EntityScale::Solar => 7,
            EntityScale::Galactic => 8,
            EntityScale::Cosmic => 9,
        }
    }

    /// Get the name of this scale
    pub fn name(&self) -> &str {
        match self {
            EntityScale::Quantum => "Quantum",
            EntityScale::Atomic => "Atomic",
            EntityScale::Molecular => "Molecular",
            EntityScale::Cellular => "Cellular",
            EntityScale::Organism => "Organism",
            EntityScale::Collective => "Collective",
            EntityScale::Planetary => "Planetary",
            EntityScale::Solar => "Solar",
            EntityScale::Galactic => "Galactic",
            EntityScale::Cosmic => "Cosmic",
        }
    }

    /// Check if this scale contains another scale
    pub fn contains(&self, other: &EntityScale) -> bool {
        self.stage_number() > other.stage_number()
    }

    /// Check if this scale is contained within another scale
    pub fn is_contained_in(&self, other: &EntityScale) -> bool {
        self.stage_number() < other.stage_number()
    }
}

/// Entity ID type alias for multi-scale system
pub type EntityId = u64;

/// MultiScaleEntity - Trait for all entities in the multi-scale holographic system
///
/// Every entity in the creation, from quantum particles to the entire cosmos,
/// implements this trait to participate in the holographic multi-scale system.
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Every portion contains the whole holographically"
/// "Each entity contains within it all densities and sub-densities"
pub trait MultiScaleEntity {
    /// Get the scale at which this entity exists
    fn get_scale(&self) -> EntityScale;

    /// Get the holographic seed (reference to complete architecture)
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Every portion contains the whole holographically"
    fn get_holographic_seed(&self) -> &HolographicSeed;

    /// Get entities contained at subscale levels
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Each entity contains within it all densities and sub-densities"
    fn contains_subscale_entities(&self) -> Vec<&dyn MultiScaleEntity>;

    /// Check if this entity contains the holographic whole
    ///
    /// From COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md:
    /// "Every portion contains the whole holographically"
    fn contains_holographic_whole(&self) -> bool;

    /// Get the entity's ID
    fn get_id(&self) -> EntityId;
}

/// MultiScaleHierarchy - Manages the fractal containment relationships across scales
///
/// Knowledge Base Reference: COMPREHENSIVE_HOLOGRAPHIC_MULTI_SCALE_ANALYSIS.md
/// "Fractal containment relationships: Atoms contain particles, molecules contain atoms, etc."
#[derive(Debug, Clone)]
pub struct MultiScaleHierarchy {
    /// Parent entities (entities that contain this entity)
    parent_entities: Vec<EntityId>,

    /// Subscale entities (entities contained within this entity)
    subscale_entities: HashMap<EntityScale, Vec<EntityId>>,

    /// The scale of this entity
    scale: EntityScale,
}

impl MultiScaleHierarchy {
    /// Create a new multi-scale hierarchy
    pub fn new(scale: EntityScale) -> Self {
        MultiScaleHierarchy {
            parent_entities: Vec::new(),
            subscale_entities: HashMap::new(),
            scale,
        }
    }

    /// Add a parent entity
    pub fn add_parent(&mut self, parent_id: EntityId) {
        if !self.parent_entities.contains(&parent_id) {
            self.parent_entities.push(parent_id);
        }
    }

    /// Add a subscale entity
    pub fn add_subscale_entity(&mut self, entity_id: EntityId, scale: EntityScale) {
        self.subscale_entities
            .entry(scale)
            .or_insert_with(Vec::new)
            .push(entity_id);
    }

    /// Get parent entities
    pub fn get_parent_entities(&self) -> &[EntityId] {
        &self.parent_entities
    }

    /// Get subscale entities at a specific scale
    pub fn get_subscale_entities(&self, scale: EntityScale) -> &[EntityId] {
        self.subscale_entities
            .get(&scale)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all subscale entities
    pub fn get_all_subscale_entities(&self) -> Vec<(EntityScale, EntityId)> {
        let mut result = Vec::new();
        for (scale, ids) in &self.subscale_entities {
            for &id in ids {
                result.push((*scale, id));
            }
        }
        result
    }

    /// Get the scale
    pub fn get_scale(&self) -> EntityScale {
        self.scale
    }

    /// Check if this entity has parent entities
    pub fn has_parent_entities(&self) -> bool {
        !self.parent_entities.is_empty()
    }

    /// Check if this entity has subscale entities
    pub fn has_subscale_entities(&self) -> bool {
        !self.subscale_entities.is_empty()
    }

    /// Count total subscale entities
    pub fn count_subscale_entities(&self) -> usize {
        self.subscale_entities.values().map(|v| v.len()).sum()
    }
}

impl Default for MultiScaleHierarchy {
    fn default() -> Self {
        Self::new(EntityScale::Organism)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== EntityScale Tests =====

    #[test]
    fn test_entity_scale_stage_number() {
        assert_eq!(EntityScale::Quantum.stage_number(), 0);
        assert_eq!(EntityScale::Atomic.stage_number(), 1);
        assert_eq!(EntityScale::Cosmic.stage_number(), 9);
    }

    #[test]
    fn test_entity_scale_name() {
        assert_eq!(EntityScale::Quantum.name(), "Quantum");
        assert_eq!(EntityScale::Atomic.name(), "Atomic");
        assert_eq!(EntityScale::Cosmic.name(), "Cosmic");
    }

    #[test]
    fn test_entity_scale_contains() {
        assert!(EntityScale::Atomic.contains(&EntityScale::Quantum));
        assert!(EntityScale::Molecular.contains(&EntityScale::Atomic));
        assert!(!EntityScale::Quantum.contains(&EntityScale::Atomic));
        assert!(!EntityScale::Atomic.contains(&EntityScale::Atomic));
    }

    #[test]
    fn test_entity_scale_is_contained_in() {
        assert!(EntityScale::Quantum.is_contained_in(&EntityScale::Atomic));
        assert!(EntityScale::Atomic.is_contained_in(&EntityScale::Molecular));
        assert!(!EntityScale::Atomic.is_contained_in(&EntityScale::Quantum));
        assert!(!EntityScale::Atomic.is_contained_in(&EntityScale::Atomic));
    }

    // ===== MultiScaleHierarchy Tests =====

    #[test]
    fn test_hierarchy_new() {
        let hierarchy = MultiScaleHierarchy::new(EntityScale::Atomic);
        assert_eq!(hierarchy.get_scale(), EntityScale::Atomic);
        assert!(!hierarchy.has_parent_entities());
        assert!(!hierarchy.has_subscale_entities());
    }

    #[test]
    fn test_hierarchy_add_parent() {
        let mut hierarchy = MultiScaleHierarchy::new(EntityScale::Atomic);
        hierarchy.add_parent(1);
        hierarchy.add_parent(2);
        hierarchy.add_parent(1); // Duplicate

        assert_eq!(hierarchy.get_parent_entities().len(), 2);
        assert!(hierarchy.has_parent_entities());
    }

    #[test]
    fn test_hierarchy_add_subscale_entity() {
        let mut hierarchy = MultiScaleHierarchy::new(EntityScale::Atomic);
        hierarchy.add_subscale_entity(1, EntityScale::Quantum);
        hierarchy.add_subscale_entity(2, EntityScale::Quantum);

        let quantum_entities = hierarchy.get_subscale_entities(EntityScale::Quantum);
        assert_eq!(quantum_entities.len(), 2);
        assert!(hierarchy.has_subscale_entities());
    }

    #[test]
    fn test_hierarchy_get_all_subscale_entities() {
        let mut hierarchy = MultiScaleHierarchy::new(EntityScale::Atomic);
        hierarchy.add_subscale_entity(1, EntityScale::Quantum);
        hierarchy.add_subscale_entity(2, EntityScale::Quantum);
        hierarchy.add_subscale_entity(3, EntityScale::Quantum);

        let all_entities = hierarchy.get_all_subscale_entities();
        assert_eq!(all_entities.len(), 3);
    }

    #[test]
    fn test_hierarchy_count_subscale_entities() {
        let mut hierarchy = MultiScaleHierarchy::new(EntityScale::Atomic);
        hierarchy.add_subscale_entity(1, EntityScale::Quantum);
        hierarchy.add_subscale_entity(2, EntityScale::Quantum);
        hierarchy.add_subscale_entity(3, EntityScale::Quantum);

        assert_eq!(hierarchy.count_subscale_entities(), 3);
    }

    #[test]
    fn test_hierarchy_default() {
        let hierarchy = MultiScaleHierarchy::default();
        assert_eq!(hierarchy.get_scale(), EntityScale::Organism);
    }

    // ===== Scale Ordering Tests =====

    #[test]
    fn test_scale_ordering() {
        assert!(EntityScale::Quantum < EntityScale::Atomic);
        assert!(EntityScale::Atomic < EntityScale::Molecular);
        assert!(EntityScale::Molecular < EntityScale::Cellular);
        assert!(EntityScale::Cellular < EntityScale::Organism);
        assert!(EntityScale::Organism < EntityScale::Collective);
        assert!(EntityScale::Collective < EntityScale::Planetary);
        assert!(EntityScale::Planetary < EntityScale::Solar);
        assert!(EntityScale::Solar < EntityScale::Galactic);
        assert!(EntityScale::Galactic < EntityScale::Cosmic);
    }

    #[test]
    fn test_scale_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(EntityScale::Quantum);
        set.insert(EntityScale::Atomic);
        set.insert(EntityScale::Molecular);
        assert_eq!(set.len(), 3);
    }
}
