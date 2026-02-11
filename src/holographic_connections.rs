// Non-Local Holographic Connections
// Documentation: CONTINUOUS_INVOLUTION_EVOLUTION_PROPOSAL.md, Section 2.8
//
// MIGRATION NOTICE (2026-02-05): This module has been migrated to simulation_v3/holographic_field.rs
// The change propagation functionality is now in V3.0 architecture.
// This file now re-exports from the new location for backward compatibility.
//
// This module implements the holographic field that connects all entities,
// allowing changes in one entity to affect all connected entities through
// non-local influence. "What is learned by one is known to all."

// Re-export from V3.0 holographic field
pub use crate::simulation_v3::holographic_field::{ChangeType, HolographicChange, InfluenceResult};

// Legacy types - kept for backward compatibility
// These are now re-exported from simulation_v3/holographic_field.rs

use crate::types::Float;
use crate::types::HolonID;
use std::collections::HashMap;

/// Direction of influence between holographically connected entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfluenceDirection {
    /// Influence flows both ways between entities
    Bidirectional,
    /// Influence flows only to the target entity
    ToTarget,
    /// Influence flows only from the target entity
    FromTarget,
}

impl InfluenceDirection {
    /// Check if influence can flow from source to target
    pub fn can_flow_to_target(&self) -> bool {
        matches!(
            self,
            InfluenceDirection::Bidirectional | InfluenceDirection::ToTarget
        )
    }

    /// Check if influence can flow from target to source
    pub fn can_flow_from_target(&self) -> bool {
        matches!(
            self,
            InfluenceDirection::Bidirectional | InfluenceDirection::FromTarget
        )
    }
}

/// A holographic link between two entities
#[derive(Debug, Clone)]
pub struct HolographicLink {
    /// Target entity ID
    pub target_entity_id: HolonID,
    /// How strongly connected (0.0 to 1.0)
    pub resonance: Float,
    /// Direction of influence
    pub influence_direction: InfluenceDirection,
    /// When last synchronized (timestamp)
    pub last_sync: Float,
}

impl HolographicLink {
    /// Create a new holographic link
    pub fn new(
        target_entity_id: HolonID,
        resonance: Float,
        influence_direction: InfluenceDirection,
        last_sync: Float,
    ) -> Self {
        HolographicLink {
            target_entity_id,
            resonance: resonance.clamp(0.0, 1.0),
            influence_direction,
            last_sync,
        }
    }

    /// Update the sync timestamp
    pub fn update_sync(&mut self, new_timestamp: Float) {
        self.last_sync = new_timestamp;
    }

    /// Check if the link is active (resonance above threshold)
    pub fn is_active(&self) -> bool {
        self.resonance > 0.1
    }
}

// ============================================================================
// CHANGE PROPAGATION TYPES (Migrated to simulation_v3/holographic_field.rs)
// ============================================================================

// The following types have been migrated to simulation_v3/holographic_field.rs
// They are re-exported above for backward compatibility.

// Type of holographic change (now in simulation_v3/holographic_field.rs)
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// // pub enum ChangeType { ... }

// A holographic change that can propagate through the field (now in simulation_v3/holographic_field.rs)
// #[derive(Debug, Clone)]
// // pub struct HolographicChange { ... }

// Result of applying holographic influence (now in simulation_v3/holographic_field.rs)
// #[derive(Debug, Clone)]
// // pub struct InfluenceResult { ... }

/// Global holographic field that connects all entities
#[derive(Debug, Clone)]
pub struct HolographicField {
    /// Connections between entities (source, target) -> link
    pub connections: HashMap<(HolonID, HolonID), HolographicLink>,
    /// Resonance matrix: source -> (target -> resonance)
    pub resonance_matrix: HashMap<HolonID, HashMap<HolonID, Float>>,
    /// Threshold for propagation (only propagate if resonance > threshold)
    pub propagation_threshold: Float,
    /// Current timestamp
    pub current_timestamp: Float,
}

impl HolographicField {
    /// Create a new holographic field
    pub fn new() -> Self {
        HolographicField {
            connections: HashMap::new(),
            resonance_matrix: HashMap::new(),
            propagation_threshold: 0.1,
            current_timestamp: 0.0,
        }
    }

    /// Set the propagation threshold
    pub fn set_propagation_threshold(&mut self, threshold: Float) {
        self.propagation_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Advance the timestamp
    pub fn advance_timestamp(&mut self, delta: Float) {
        self.current_timestamp += delta.abs();
    }

    /// Set the timestamp
    pub fn set_timestamp(&mut self, timestamp: Float) {
        self.current_timestamp = timestamp.abs();
    }

    /// Get current timestamp
    pub fn get_timestamp(&self) -> Float {
        self.current_timestamp
    }

    /// Establish a bidirectional connection between two entities
    ///
    /// This creates a holographic link that allows changes to propagate
    /// between the two entities based on their resonance.
    pub fn establish_connection(&mut self, entity_a: HolonID, entity_b: HolonID, resonance: Float) {
        // Clamp resonance to valid range
        let resonance = resonance.clamp(0.0, 1.0);

        // Create bidirectional links
        let link_a_to_b = HolographicLink::new(
            entity_b,
            resonance,
            InfluenceDirection::Bidirectional,
            self.current_timestamp,
        );
        let link_b_to_a = HolographicLink::new(
            entity_a,
            resonance,
            InfluenceDirection::Bidirectional,
            self.current_timestamp,
        );

        // Insert connections
        self.connections.insert((entity_a, entity_b), link_a_to_b);
        self.connections.insert((entity_b, entity_a), link_b_to_a);

        // Update resonance matrix
        self.resonance_matrix
            .entry(entity_a)
            .or_insert_with(HashMap::new)
            .insert(entity_b, resonance);
        self.resonance_matrix
            .entry(entity_b)
            .or_insert_with(HashMap::new)
            .insert(entity_a, resonance);
    }

    /// Establish a unidirectional connection
    ///
    /// Creates a connection where influence flows only in one direction.
    pub fn establish_unidirectional_connection(
        &mut self,
        source: HolonID,
        target: HolonID,
        resonance: Float,
        direction: InfluenceDirection,
    ) {
        // Clamp resonance to valid range
        let resonance = resonance.clamp(0.0, 1.0);

        // Create unidirectional link
        let link = HolographicLink::new(target, resonance, direction, self.current_timestamp);

        // Insert connection
        self.connections.insert((source, target), link);

        // Update resonance matrix
        self.resonance_matrix
            .entry(source)
            .or_insert_with(HashMap::new)
            .insert(target, resonance);
    }

    /// Get the resonance between two entities
    pub fn get_resonance(&self, entity_a: HolonID, entity_b: HolonID) -> Float {
        self.resonance_matrix
            .get(&entity_a)
            .and_then(|connections| connections.get(&entity_b))
            .copied()
            .unwrap_or(0.0)
    }

    /// Check if two entities are connected
    pub fn are_connected(&self, entity_a: HolonID, entity_b: HolonID) -> bool {
        self.connections.contains_key(&(entity_a, entity_b))
            || self.connections.contains_key(&(entity_b, entity_a))
    }

    /// Get all connections for an entity
    pub fn get_connections(&self, entity_id: HolonID) -> Vec<HolonID> {
        self.resonance_matrix
            .get(&entity_id)
            .map(|connections| connections.keys().copied().collect())
            .unwrap_or_default()
    }

    /// Get the holographic link between two entities
    pub fn get_link(&self, source: HolonID, target: HolonID) -> Option<&HolographicLink> {
        self.connections.get(&(source, target))
    }

    /// Remove a connection between two entities
    pub fn remove_connection(&mut self, entity_a: HolonID, entity_b: HolonID) {
        // Remove connections
        self.connections.remove(&(entity_a, entity_b));
        self.connections.remove(&(entity_b, entity_a));

        // Remove from resonance matrix
        if let Some(connections) = self.resonance_matrix.get_mut(&entity_a) {
            connections.remove(&entity_b);
        }
        if let Some(connections) = self.resonance_matrix.get_mut(&entity_b) {
            connections.remove(&entity_a);
        }
    }

    /// Propagate a change from a source entity to all connected entities
    ///
    /// This implements non-local influence: changes in one entity
    /// propagate to all connected entities based on resonance.
    pub fn propagate_change(
        &mut self,
        source_entity_id: HolonID,
        change: &HolographicChange,
    ) -> Vec<InfluenceResult> {
        let mut results = Vec::new();

        // Get all connections for the source entity
        if let Some(connections) = self.resonance_matrix.get(&source_entity_id) {
            for (&target_id, &resonance) in connections {
                // Only propagate if resonance is above threshold
                if resonance > self.propagation_threshold {
                    // Calculate influence strength
                    let influence_strength = resonance * change.intensity;

                    // Apply influence to target
                    let result = self.apply_influence(target_id, change, influence_strength);

                    // Update sync timestamp
                    if let Some(link) = self.connections.get_mut(&(source_entity_id, target_id)) {
                        link.update_sync(self.current_timestamp);
                    }

                    results.push(result);
                }
            }
        }

        results
    }

    /// Apply holographic influence to a target entity
    ///
    /// The effect depends on veil transparency (thinner veil = more influence).
    /// This is a simplified implementation that returns the influence result.
    pub fn apply_influence(
        &self,
        target_id: HolonID,
        change: &HolographicChange,
        strength: Float,
    ) -> InfluenceResult {
        // Check if target is connected to the source
        // In a full implementation, this would modify the target's archetypical mind
        // For now, we return the influence result

        let effect = strength * change.intensity;

        // Convert HolonID to EntityId
        let entity_id = crate::entity_layer7::layer7::EntityId {
            uuid: target_id.to_string(),
            incarnation_number: 0,
        };

        InfluenceResult::new(entity_id, true, strength, effect)
    }

    /// Calculate total influence on an entity from all connected entities
    pub fn calculate_total_influence(&self, entity_id: HolonID) -> Float {
        self.resonance_matrix
            .get(&entity_id)
            .map(|connections| connections.values().sum())
            .unwrap_or(0.0)
    }

    /// Get the strongest connection for an entity
    pub fn get_strongest_connection(&self, entity_id: HolonID) -> Option<(HolonID, Float)> {
        self.resonance_matrix
            .get(&entity_id)
            .and_then(|connections| {
                connections
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(id, &resonance)| (*id, resonance))
            })
    }

    /// Get all entities in the holographic field
    pub fn get_all_entities(&self) -> Vec<HolonID> {
        let entities: Vec<HolonID> = self.resonance_matrix.keys().copied().collect();
        // Can't sort without Ord trait, order doesn't matter for Phase 1
        entities
    }

    /// Get the number of entities in the field
    pub fn entity_count(&self) -> usize {
        self.resonance_matrix.len()
    }

    /// Get the total number of connections
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Clear all connections
    pub fn clear(&mut self) {
        self.connections.clear();
        self.resonance_matrix.clear();
    }
}

impl Default for HolographicField {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // INFLUENCE DIRECTION TESTS
    // ============================================================================

    #[test]
    fn test_influence_direction_bidirectional() {
        let direction = InfluenceDirection::Bidirectional;
        assert!(direction.can_flow_to_target());
        assert!(direction.can_flow_from_target());
    }

    #[test]
    fn test_influence_direction_to_target() {
        let direction = InfluenceDirection::ToTarget;
        assert!(direction.can_flow_to_target());
        assert!(!direction.can_flow_from_target());
    }

    #[test]
    fn test_influence_direction_from_target() {
        let direction = InfluenceDirection::FromTarget;
        assert!(!direction.can_flow_to_target());
        assert!(direction.can_flow_from_target());
    }

    // ============================================================================
    // HOLOGRAPHIC LINK TESTS
    // ============================================================================

    #[test]
    fn test_holographic_link_creation() {
        let link = HolographicLink::new(1, 0.7, InfluenceDirection::Bidirectional, 0.0);
        assert_eq!(link.target_entity_id, 1);
        assert!((link.resonance - 0.7).abs() < 0.001);
        assert_eq!(link.influence_direction, InfluenceDirection::Bidirectional);
        assert_eq!(link.last_sync, 0.0);
    }

    #[test]
    fn test_holographic_link_resonance_clamping() {
        let link_high = HolographicLink::new(1, 1.5, InfluenceDirection::Bidirectional, 0.0);
        assert!((link_high.resonance - 1.0).abs() < 0.001);

        let link_low = HolographicLink::new(1, -0.5, InfluenceDirection::Bidirectional, 0.0);
        assert!((link_low.resonance - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_holographic_link_update_sync() {
        let mut link = HolographicLink::new(1, 0.7, InfluenceDirection::Bidirectional, 0.0);
        link.update_sync(5.0);
        assert_eq!(link.last_sync, 5.0);
    }

    #[test]
    fn test_holographic_link_is_active() {
        let link_active = HolographicLink::new(1, 0.5, InfluenceDirection::Bidirectional, 0.0);
        assert!(link_active.is_active());

        let link_inactive = HolographicLink::new(1, 0.05, InfluenceDirection::Bidirectional, 0.0);
        assert!(!link_inactive.is_active());

        let link_threshold = HolographicLink::new(1, 0.1, InfluenceDirection::Bidirectional, 0.0);
        assert!(!link_threshold.is_active()); // Must be > 0.1
    }

    // ============================================================================
    // HOLOGRAPHIC CHANGE TESTS
    // ============================================================================

    #[test]
    fn test_holographic_change_creation() {
        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        assert_eq!(change.archetype_id, 3);
        assert_eq!(change.change_type, ChangeType::ActivationChange);
        assert!((change.intensity - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_holographic_change_intensity_clamping() {
        let change_high = HolographicChange::new(3, ChangeType::ActivationChange, 1.5);
        assert!((change_high.intensity - 1.0).abs() < 0.001);

        let change_low = HolographicChange::new(3, ChangeType::ActivationChange, -0.5);
        assert!((change_low.intensity - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_holographic_change_getters() {
        let change = HolographicChange::new(3, ChangeType::BiasChange, 0.7);
        assert_eq!(change.get_change_type(), ChangeType::BiasChange);
        assert!((change.get_intensity() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_change_type_variants() {
        let activation = ChangeType::ActivationChange;
        let bias = ChangeType::BiasChange;
        let refinement = ChangeType::RefinementChange;

        assert_eq!(activation, ChangeType::ActivationChange);
        assert_eq!(bias, ChangeType::BiasChange);
        assert_eq!(refinement, ChangeType::RefinementChange);
    }

    // ============================================================================
    // INFLUENCE RESULT TESTS
    // ============================================================================

    #[test]
    fn test_influence_result_creation() {
        let entity_id = crate::entity_layer7::layer7::EntityId {
            uuid: "1".to_string(),
            incarnation_number: 0,
        };
        let result = InfluenceResult::new(entity_id, true, 0.7, 0.5);
        assert_eq!(result.target_id.uuid, "1");
        assert!(result.applied);
        assert!((result.strength - 0.7).abs() < 0.001);
        assert!((result.effect - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_influence_result_strength_clamping() {
        let entity_id = crate::entity_layer7::layer7::EntityId {
            uuid: "1".to_string(),
            incarnation_number: 0,
        };
        let result_high = InfluenceResult::new(entity_id.clone(), true, 1.5, 0.5);
        assert!((result_high.strength - 1.0).abs() < 0.001);

        let result_low = InfluenceResult::new(entity_id.clone(), true, -0.5, 0.5);
        assert!((result_low.strength - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_influence_result_was_applied() {
        let entity_id = crate::entity_layer7::layer7::EntityId {
            uuid: "1".to_string(),
            incarnation_number: 0,
        };
        let result_applied = InfluenceResult::new(entity_id.clone(), true, 0.7, 0.5);
        assert!(result_applied.was_applied());

        let result_not_applied = InfluenceResult::new(entity_id.clone(), false, 0.7, 0.5);
        assert!(!result_not_applied.was_applied());
    }

    // ============================================================================
    // HOLOGRAPHIC FIELD CREATION TESTS
    // ============================================================================

    #[test]
    fn test_holographic_field_creation() {
        let field = HolographicField::new();
        assert_eq!(field.connections.len(), 0);
        assert_eq!(field.resonance_matrix.len(), 0);
        assert!((field.propagation_threshold - 0.1).abs() < 0.001);
        assert_eq!(field.current_timestamp, 0.0);
    }

    #[test]
    fn test_holographic_field_default() {
        let field = HolographicField::default();
        assert_eq!(field.connections.len(), 0);
        assert_eq!(field.resonance_matrix.len(), 0);
    }

    #[test]
    fn test_set_propagation_threshold() {
        let mut field = HolographicField::new();
        field.set_propagation_threshold(0.3);
        assert!((field.propagation_threshold - 0.3).abs() < 0.001);

        field.set_propagation_threshold(1.5);
        assert!((field.propagation_threshold - 1.0).abs() < 0.001);

        field.set_propagation_threshold(-0.5);
        assert!((field.propagation_threshold - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_advance_timestamp() {
        let mut field = HolographicField::new();
        field.advance_timestamp(5.0);
        assert_eq!(field.current_timestamp, 5.0);

        field.advance_timestamp(3.0);
        assert_eq!(field.current_timestamp, 8.0);

        field.advance_timestamp(-2.0);
        assert_eq!(field.current_timestamp, 10.0); // Absolute value
    }

    #[test]
    fn test_set_timestamp() {
        let mut field = HolographicField::new();
        field.set_timestamp(10.0);
        assert_eq!(field.current_timestamp, 10.0);

        field.set_timestamp(-5.0);
        assert_eq!(field.current_timestamp, 5.0); // Absolute value
    }

    #[test]
    fn test_get_timestamp() {
        let mut field = HolographicField::new();
        field.set_timestamp(7.0);
        assert_eq!(field.get_timestamp(), 7.0);
    }

    // ============================================================================
    // CONNECTION ESTABLISHMENT TESTS
    // ============================================================================

    #[test]
    fn test_establish_connection_bidirectional() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        // Check connections exist in both directions
        assert!(field.connections.contains_key(&(1, 2)));
        assert!(field.connections.contains_key(&(2, 1)));

        // Check resonance matrix
        assert!((field.get_resonance(1, 2) - 0.7).abs() < 0.001);
        assert!((field.get_resonance(2, 1) - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_establish_connection_resonance_clamping() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 1.5);
        assert!((field.get_resonance(1, 2) - 1.0).abs() < 0.001);

        field.establish_connection(3, 4, -0.5);
        assert!((field.get_resonance(3, 4) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_establish_connection_bidirectional_links() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        // Check link properties
        let link_1_to_2 = field.get_link(1, 2).unwrap();
        assert_eq!(link_1_to_2.target_entity_id, 2);
        assert!((link_1_to_2.resonance - 0.7).abs() < 0.001);
        assert_eq!(
            link_1_to_2.influence_direction,
            InfluenceDirection::Bidirectional
        );

        let link_2_to_1 = field.get_link(2, 1).unwrap();
        assert_eq!(link_2_to_1.target_entity_id, 1);
        assert!((link_2_to_1.resonance - 0.7).abs() < 0.001);
        assert_eq!(
            link_2_to_1.influence_direction,
            InfluenceDirection::Bidirectional
        );
    }

    #[test]
    fn test_establish_unidirectional_connection_to_target() {
        let mut field = HolographicField::new();
        field.establish_unidirectional_connection(1, 2, 0.7, InfluenceDirection::ToTarget);

        // Check connection exists only in one direction
        assert!(field.connections.contains_key(&(1, 2)));
        assert!(!field.connections.contains_key(&(2, 1)));

        // Check resonance matrix (only one direction)
        assert!((field.get_resonance(1, 2) - 0.7).abs() < 0.001);
        assert!((field.get_resonance(2, 1) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_establish_unidirectional_connection_from_target() {
        let mut field = HolographicField::new();
        field.establish_unidirectional_connection(1, 2, 0.7, InfluenceDirection::FromTarget);

        // Check connection exists only in one direction
        assert!(field.connections.contains_key(&(1, 2)));
        assert!(!field.connections.contains_key(&(2, 1)));

        // Check resonance matrix (only one direction)
        assert!((field.get_resonance(1, 2) - 0.7).abs() < 0.001);
        assert!((field.get_resonance(2, 1) - 0.0).abs() < 0.001);
    }

    // ============================================================================
    // RESONANCE AND CONNECTION QUERIES TESTS
    // ============================================================================

    #[test]
    fn test_get_resonance() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        assert!((field.get_resonance(1, 2) - 0.7).abs() < 0.001);
        assert!((field.get_resonance(2, 1) - 0.7).abs() < 0.001);
        assert!((field.get_resonance(1, 3) - 0.0).abs() < 0.001); // No connection
    }

    #[test]
    fn test_are_connected() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        assert!(field.are_connected(1, 2));
        assert!(field.are_connected(2, 1));
        assert!(!field.are_connected(1, 3));
    }

    #[test]
    fn test_get_connections() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(1, 3, 0.5);

        let connections = field.get_connections(1);
        assert_eq!(connections.len(), 2);
        assert!(connections.contains(&2));
        assert!(connections.contains(&3));

        let connections_2 = field.get_connections(2);
        assert_eq!(connections_2.len(), 1);
        assert!(connections_2.contains(&1));
    }

    #[test]
    fn test_get_connections_empty() {
        let field = HolographicField::new();
        let connections = field.get_connections(1);
        assert_eq!(connections.len(), 0);
    }

    #[test]
    fn test_get_link() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        let link = field.get_link(1, 2);
        assert!(link.is_some());
        assert_eq!(link.unwrap().target_entity_id, 2);

        let no_link = field.get_link(1, 3);
        assert!(no_link.is_none());
    }

    // ============================================================================
    // CONNECTION REMOVAL TESTS
    // ============================================================================

    #[test]
    fn test_remove_connection() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);

        assert!(field.are_connected(1, 2));

        field.remove_connection(1, 2);

        assert!(!field.are_connected(1, 2));
        assert!(!field.are_connected(2, 1));
        assert!((field.get_resonance(1, 2) - 0.0).abs() < 0.001);
        assert!((field.get_resonance(2, 1) - 0.0).abs() < 0.001);
    }

    // ============================================================================
    // CHANGE PROPAGATION TESTS
    // ============================================================================

    #[test]
    fn test_propagate_change() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(1, 3, 0.5);

        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results = field.propagate_change(1, &change);

        assert_eq!(results.len(), 2);

        // Check results for entity 2
        let result_2 = results.iter().find(|r| r.target_id.uuid == "2").unwrap();
        assert!(result_2.was_applied());
        assert!((result_2.strength - (0.7 * 0.8)).abs() < 0.001);

        // Check results for entity 3
        let result_3 = results.iter().find(|r| r.target_id.uuid == "3").unwrap();
        assert!(result_3.was_applied());
        assert!((result_3.strength - (0.5 * 0.8)).abs() < 0.001);
    }

    #[test]
    fn test_propagate_change_below_threshold() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.05); // Below threshold

        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results = field.propagate_change(1, &change);

        assert_eq!(results.len(), 0); // No propagation below threshold
    }

    #[test]
    fn test_propagate_change_at_threshold() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.1); // At threshold

        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results = field.propagate_change(1, &change);

        assert_eq!(results.len(), 0); // No propagation at threshold (must be > threshold)
    }

    #[test]
    fn test_propagate_change_updates_sync() {
        let mut field = HolographicField::new();
        field.advance_timestamp(10.0);
        field.establish_connection(1, 2, 0.7);

        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        field.propagate_change(1, &change);

        let link = field.get_link(1, 2).unwrap();
        assert_eq!(link.last_sync, 10.0);
    }

    #[test]
    fn test_propagate_change_no_connections() {
        let mut field = HolographicField::new();
        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results = field.propagate_change(1, &change);

        assert_eq!(results.len(), 0);
    }

    // ============================================================================
    // INFLUENCE APPLICATION TESTS
    // ============================================================================

    #[test]
    fn test_apply_influence() {
        let field = HolographicField::new();
        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);

        let result = field.apply_influence(2, &change, 0.7);

        assert_eq!(result.target_id.uuid, "2");
        assert!(result.was_applied());
        assert!((result.strength - 0.7).abs() < 0.001);
        assert!((result.effect - (0.7 * 0.8)).abs() < 0.001);
    }

    // ============================================================================
    // TOTAL INFLUENCE TESTS
    // ============================================================================

    #[test]
    fn test_calculate_total_influence() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(1, 3, 0.5);
        field.establish_connection(1, 4, 0.3);

        let total = field.calculate_total_influence(1);
        assert!((total - 1.5).abs() < 0.001); // 0.7 + 0.5 + 0.3
    }

    #[test]
    fn test_calculate_total_influence_zero() {
        let field = HolographicField::new();
        let total = field.calculate_total_influence(1);
        assert!((total - 0.0).abs() < 0.001);
    }

    // ============================================================================
    // STRONGEST CONNECTION TESTS
    // ============================================================================

    #[test]
    fn test_get_strongest_connection() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(1, 3, 0.9);
        field.establish_connection(1, 4, 0.5);

        let strongest = field.get_strongest_connection(1);
        assert!(strongest.is_some());
        assert_eq!(strongest.unwrap().0, 3);
        assert!((strongest.unwrap().1 - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_get_strongest_connection_none() {
        let field = HolographicField::new();
        let strongest = field.get_strongest_connection(1);
        assert!(strongest.is_none());
    }

    // ============================================================================
    // FIELD QUERIES TESTS
    // ============================================================================

    #[test]
    fn test_get_all_entities() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(3, 4, 0.5);

        let entities = field.get_all_entities();
        assert_eq!(entities.len(), 4);
        assert!(entities.contains(&1));
        assert!(entities.contains(&2));
        assert!(entities.contains(&3));
        assert!(entities.contains(&4));
    }

    #[test]
    fn test_entity_count() {
        let mut field = HolographicField::new();
        assert_eq!(field.entity_count(), 0);

        field.establish_connection(1, 2, 0.7);
        assert_eq!(field.entity_count(), 2);

        field.establish_connection(3, 4, 0.5);
        assert_eq!(field.entity_count(), 4);
    }

    #[test]
    fn test_connection_count() {
        let mut field = HolographicField::new();
        assert_eq!(field.connection_count(), 0);

        field.establish_connection(1, 2, 0.7);
        assert_eq!(field.connection_count(), 2); // Bidirectional = 2 links

        field.establish_connection(3, 4, 0.5);
        assert_eq!(field.connection_count(), 4); // 2 bidirectional connections = 4 links
    }

    #[test]
    fn test_clear() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(3, 4, 0.5);

        assert_eq!(field.entity_count(), 4);
        assert_eq!(field.connection_count(), 4);

        field.clear();

        assert_eq!(field.entity_count(), 0);
        assert_eq!(field.connection_count(), 0);
    }

    // ============================================================================
    // COMPLEX SCENARIO TESTS
    // ============================================================================

    #[test]
    fn test_multiple_propagations() {
        let mut field = HolographicField::new();
        field.establish_connection(1, 2, 0.7);
        field.establish_connection(2, 3, 0.6);
        field.establish_connection(3, 4, 0.5);

        // First propagation from 1
        let change1 = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results1 = field.propagate_change(1, &change1);
        assert_eq!(results1.len(), 1); // Only entity 2

        // Second propagation from 2
        let change2 = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results2 = field.propagate_change(2, &change2);
        assert_eq!(results2.len(), 2); // Entities 1 and 3
    }

    #[test]
    fn test_network_topology() {
        let mut field = HolographicField::new();

        // Create a star topology: 1 connected to 2, 3, 4, 5
        for i in 2..=5 {
            field.establish_connection(1, i, 0.7);
        }

        // Verify connections
        assert_eq!(field.get_connections(1).len(), 4);
        assert_eq!(field.get_connections(2).len(), 1);
        assert_eq!(field.get_connections(3).len(), 1);
        assert_eq!(field.get_connections(4).len(), 1);
        assert_eq!(field.get_connections(5).len(), 1);

        // Verify total influence
        let total = field.calculate_total_influence(1);
        assert!((total - 2.8).abs() < 0.001); // 0.7 * 4
    }

    #[test]
    fn test_mixed_bidirectional_unidirectional() {
        let mut field = HolographicField::new();

        // Bidirectional connection
        field.establish_connection(1, 2, 0.7);

        // Unidirectional connection
        field.establish_unidirectional_connection(2, 3, 0.6, InfluenceDirection::ToTarget);

        // Verify connections (are_connected checks if there's a link in either direction)
        assert!(field.are_connected(1, 2));
        assert!(field.are_connected(2, 1));
        assert!(field.are_connected(2, 3));
        assert!(field.are_connected(3, 2)); // Still connected, just unidirectional influence

        // Verify link exists only in one direction
        assert!(field.get_link(2, 3).is_some());
        assert!(field.get_link(3, 2).is_none());

        // Verify propagation
        let change = HolographicChange::new(3, ChangeType::ActivationChange, 0.8);
        let results = field.propagate_change(1, &change);
        assert_eq!(results.len(), 1); // Only entity 2
    }

    #[test]
    fn test_timestamp_tracking() {
        let mut field = HolographicField::new();

        field.advance_timestamp(1.0);
        field.establish_connection(1, 2, 0.7);

        field.advance_timestamp(2.0);
        field.establish_connection(3, 4, 0.5);

        let link_1_to_2 = field.get_link(1, 2).unwrap();
        assert_eq!(link_1_to_2.last_sync, 1.0);

        let link_3_to_4 = field.get_link(3, 4).unwrap();
        assert_eq!(link_3_to_4.last_sync, 3.0); // 1.0 + 2.0

        field.advance_timestamp(3.0);
        field.propagate_change(
            1,
            &HolographicChange::new(3, ChangeType::ActivationChange, 0.8),
        );

        let link_1_to_2_updated = field.get_link(1, 2).unwrap();
        assert_eq!(link_1_to_2_updated.last_sync, 6.0); // 3.0 + 3.0
    }
}
