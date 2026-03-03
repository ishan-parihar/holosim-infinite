//! EntityBehavior Trait - Common Interface for Entity Types
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "All entity types share common behaviors"
//!
//! This trait provides a unified interface that allows both SubSubLogos and
//! Entity (UniversalTemplate<EntityData>) to be used interchangeably during
//! the migration period and beyond.
//!
//! # Design Goals
//!
//! 1. **Backward Compatibility**: Allows SubSubLogos and Entity to coexist
//! 2. **Behavior Abstraction**: Common operations shared by all entity types
//! 3. **Field Integration**: Entity can reference shared HolographicField
//! 4. **Consciousness as Kernel**: Archetype activations from template and component_data

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::types::{Float, Polarity};

/// Trait defining all entity behaviors
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "All entity types share common behaviors"
///
/// This trait unifies the interface for both legacy SubSubLogos and the new
/// Entity (UniversalTemplate<EntityData>) type, enabling a smooth migration.
pub trait EntityBehavior {
    // =========================================================================
    // IDENTITY
    // =========================================================================

    /// Get the unique entity identifier
    fn entity_id(&self) -> EntityId;

    /// Get the entity type (Individual, Collective, Environmental, etc.)
    fn entity_type(&self) -> EntityType;

    // =========================================================================
    // COMPOSITION
    // =========================================================================

    /// Add a component entity to this entity's composition
    ///
    /// Phase 1: Material Composition Tracking
    /// Models hierarchical material composition:
    /// - An atom adds quantum particles to its composition
    /// - A molecule adds atoms to its composition
    /// - A cell adds molecules to its composition
    fn add_component(&mut self, component_id: EntityId);

    /// Remove a component entity from this entity's composition
    ///
    /// Returns true if the component was found and removed
    fn remove_component(&mut self, component_id: &EntityId) -> bool;

    /// Check if this entity is composed of a specific entity
    fn has_component(&self, component_id: &EntityId) -> bool;

    /// Get a reference to this entity's composition
    fn composition(&self) -> &[EntityId];

    /// Get the number of components in this entity's composition
    fn component_count(&self) -> usize;

    // =========================================================================
    // HIERARCHY
    // =========================================================================

    /// Add a child entity to this entity's children
    ///
    /// Models parent/child relationships for collective entities
    fn add_child(&mut self, child_id: EntityId);

    /// Remove a child entity from this entity's children
    ///
    /// Returns true if the child was found and removed
    fn remove_child(&mut self, child_id: &EntityId) -> bool;

    /// Get a reference to this entity's children
    fn children(&self) -> &[EntityId];

    /// Set the parent entity for this entity
    fn set_parent(&mut self, parent_id: Option<EntityId>);

    /// Get the parent entity ID
    fn parent(&self) -> Option<EntityId>;

    /// Check if this entity has a parent
    fn has_parent(&self) -> bool;

    /// Check if this entity is a root entity (no parent)
    fn is_root(&self) -> bool;

    /// Check if this entity is a leaf entity (no children)
    fn is_leaf(&self) -> bool;

    // =========================================================================
    // DENSITY & EVOLUTION
    // =========================================================================

    /// Get the current density level as a u8 (1-8)
    ///
    /// Returns the density level as a simple number for cross-compatibility
    fn density_level(&self) -> u8;

    /// Get the evolution clock value
    ///
    /// Phase 1: Each entity has its own evolution clock that determines
    /// when it is ready to make evolutionary choices.
    fn evolution_clock(&self) -> Float;

    /// Advance the evolution clock by a time delta
    ///
    /// The clock advances based on evolutionary rate and spectrum configuration
    fn advance_evolution_clock(&mut self, dt: Float);

    /// Check if entity is ready for density transition
    fn is_ready_for_density_transition(&self) -> bool;

    // =========================================================================
    // POLARIZATION
    // =========================================================================

    /// Get the current polarity orientation (if any)
    ///
    /// Returns None for unpolarized entities
    fn polarization(&self) -> Option<Polarity>;

    /// Get the polarization strength (0.0 to 1.0)
    ///
    /// - 0.0 = Unpolarized
    /// - 0.51+ = Harvestable for STO
    /// - 0.95+ = Harvestable for STS
    fn polarization_strength(&self) -> Float;

    // =========================================================================
    // ARCHETYPE
    // =========================================================================

    /// Get the archetype activations (22 coefficients)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Store 22 archetype coefficients instead of full patterns"
    fn archetype_activations(&self) -> &[Float; 22];

    /// Get mutable reference to archetype activations
    fn archetype_activations_mut(&mut self) -> &mut [Float; 22];

    // =========================================================================
    // SPECTRUM
    // =========================================================================

    /// Get the space/time ratio
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Space/Time and Time/Space spectrum is a continuous range of
    /// reciprocal ratios with a qualitative break at v = 1 (the Veil)"
    fn space_time_ratio(&self) -> Float;

    /// Get the veil transparency (0.0 = opaque, 1.0 = transparent)
    fn veil_transparency(&self) -> Float;
}

#[cfg(test)]
mod tests {
    // Tests will be added when implementations are complete
}
