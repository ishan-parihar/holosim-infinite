//! Hierarchy Connection - Data structures for entity relationship visualization
//!
//! Phase 4: Hierarchy Visualization
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md:
//! "Visualize composition hierarchy, parent-child relationships, and environment relationships."
//!
//! This module provides data structures for rendering connection lines between related entities.

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::renderer::entity_instance::EntityInstance;
use crate::hpo::RenderableEntity;

/// Type of hierarchical relationship between entities
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ConnectionType {
    /// Parent-child relationship (collective composition)
    ParentChild = 0,
    /// Material composition (atoms composed of particles, molecules composed of atoms, etc.)
    Composition = 1,
    /// Entity-environment relationship
    Environment = 2,
    /// Field-derived phase coherence relationship
    PhaseCoherence = 3,
    /// Strong phase-lock / entanglement relationship
    Entanglement = 4,
}

/// Connection data for rendering a relationship between two entities
///
/// Memory layout (GPU-friendly, aligned):
/// - Total size: 44 bytes
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct HierarchyConnection {
    /// Position of the connection source (parent, container, environment)
    pub from_position: [f32; 3],

    /// Position of the connection target (child, component, entity in environment)
    pub to_position: [f32; 3],

    /// Type of relationship
    pub connection_type: u32,

    /// Connection intensity (0.0 to 1.0)
    /// Used for alpha blending and line thickness
    pub intensity: f32,

    /// Phase difference proxy in [0, 1].
    /// 0 means phase-aligned, 1 means maximally offset.
    /// For phase edges without explicit phase delta, this is derived as `1 - coherence`.
    pub phase_delta: f32,

    /// Padding for alignment
    pub _padding: [f32; 2],
}

impl HierarchyConnection {
    /// Create a parent-child connection
    pub fn parent_child(from_pos: [f32; 3], to_pos: [f32; 3], intensity: f32) -> Self {
        Self {
            from_position: from_pos,
            to_position: to_pos,
            connection_type: ConnectionType::ParentChild as u32,
            intensity: intensity.clamp(0.0, 1.0),
            phase_delta: 0.0,
            _padding: [0.0; 2],
        }
    }

    /// Create a composition connection
    pub fn composition(from_pos: [f32; 3], to_pos: [f32; 3], intensity: f32) -> Self {
        Self {
            from_position: from_pos,
            to_position: to_pos,
            connection_type: ConnectionType::Composition as u32,
            intensity: intensity.clamp(0.0, 1.0),
            phase_delta: 0.0,
            _padding: [0.0; 2],
        }
    }

    /// Create an environment connection
    pub fn environment(from_pos: [f32; 3], to_pos: [f32; 3], intensity: f32) -> Self {
        Self {
            from_position: from_pos,
            to_position: to_pos,
            connection_type: ConnectionType::Environment as u32,
            intensity: intensity.clamp(0.0, 1.0),
            phase_delta: 0.0,
            _padding: [0.0; 2],
        }
    }

    /// Create a phase-coherence connection
    pub fn phase_coherence(from_pos: [f32; 3], to_pos: [f32; 3], intensity: f32) -> Self {
        let coherence = intensity.clamp(0.0, 1.0);
        Self::phase_coherence_with_delta(from_pos, to_pos, coherence, 1.0 - coherence)
    }

    /// Create a phase-coherence connection with explicit phase delta/proxy
    pub fn phase_coherence_with_delta(
        from_pos: [f32; 3],
        to_pos: [f32; 3],
        intensity: f32,
        phase_delta: f32,
    ) -> Self {
        Self {
            from_position: from_pos,
            to_position: to_pos,
            connection_type: ConnectionType::PhaseCoherence as u32,
            intensity: intensity.clamp(0.0, 1.0),
            phase_delta: phase_delta.clamp(0.0, 1.0),
            _padding: [0.0; 2],
        }
    }

    /// Create an entanglement connection with explicit phase delta/proxy
    pub fn entanglement(
        from_pos: [f32; 3],
        to_pos: [f32; 3],
        intensity: f32,
        phase_delta: f32,
    ) -> Self {
        Self {
            from_position: from_pos,
            to_position: to_pos,
            connection_type: ConnectionType::Entanglement as u32,
            intensity: intensity.clamp(0.0, 1.0),
            phase_delta: phase_delta.clamp(0.0, 1.0),
            _padding: [0.0; 2],
        }
    }
}

/// Generate hierarchy connections from a collection of entities
///
/// This function analyzes the parent-child, composition, and environment relationships
/// in the entity collection and generates connection data for visualization.
///
/// # Arguments
/// * `entities` - Slice of SubSubLogos entities
/// * `entity_instances` - Slice of EntityInstance with position data
///
/// # Returns
/// Vector of HierarchyConnection structures
pub fn generate_connections(
    entities: &[SubSubLogos],
    entity_instances: &[EntityInstance],
) -> Vec<HierarchyConnection> {
    let mut connections = Vec::new();
    let mut entity_map = std::collections::HashMap::new();

    // Build entity ID to index map for position lookup
    for (index, entity) in entities.iter().enumerate() {
        entity_map.insert(entity.entity_id.as_u64(), index);
    }

    for (entity_idx, entity) in entities.iter().enumerate() {
        let entity_pos = entity_instances[entity_idx].position;

        // Generate parent-child connections
        if let Some(parent_id) = entity.parent_id.as_ref() {
            if let Some(&parent_idx) = entity_map.get(&parent_id.as_u64()) {
                let parent_pos = entity_instances[parent_idx].position;
                // Connection intensity based on child's consciousness level
                let intensity = entity.consciousness_level as f32;
                connections.push(HierarchyConnection::parent_child(
                    parent_pos,
                    entity_pos,
                    intensity * 0.5 + 0.3, // Base 0.3 + consciousness influence
                ));
            }
        }

        // Generate composition connections
        for component_id in &entity.composition {
            if let Some(&component_idx) = entity_map.get(&component_id.as_u64()) {
                let component_pos = entity_instances[component_idx].position;
                // Connection intensity based on component's integration level
                let intensity = 0.5; // Fixed intensity for composition
                connections.push(HierarchyConnection::composition(
                    entity_pos,
                    component_pos,
                    intensity,
                ));
            }
        }

        // Generate environment connections
        if let Some(environment_id) = entity.environment_id.as_ref() {
            if let Some(&env_idx) = entity_map.get(&environment_id.as_u64()) {
                let env_pos = entity_instances[env_idx].position;
                // Connection intensity based on entity's integration with environment
                let intensity = 0.4; // Fixed intensity for environment
                connections.push(HierarchyConnection::environment(
                    env_pos, entity_pos, intensity,
                ));
            }
        }
    }

    connections
}

/// Generate structural connections directly from field-derived renderable entities.
///
/// Heuristic graph constraints:
/// - Up to 3 nearest neighbors per entity
/// - Distance threshold: `0.45 + 0.10 * clamp(avg_veil_distance, 0, 1)`
/// - Density band difference <= 2
/// - Duplicate undirected edges removed
/// - Hard cap at 3000 total edges
pub fn generate_holo_connections(entities: &[RenderableEntity]) -> Vec<HierarchyConnection> {
    if entities.len() < 2 {
        return Vec::new();
    }

    const MAX_NEIGHBORS: usize = 3;
    const MAX_EDGES: usize = 3000;

    let avg_veil_distance = (entities
        .iter()
        .map(|entity| entity.veil_distance)
        .sum::<f64>()
        / entities.len() as f64)
        .clamp(0.0, 1.0) as f32;
    let threshold = 0.45 + 0.10 * avg_veil_distance;

    let mut connections = Vec::with_capacity(entities.len().saturating_mul(MAX_NEIGHBORS));
    let mut seen_edges = std::collections::HashSet::new();

    for (i, entity) in entities.iter().enumerate() {
        let mut candidates: Vec<(usize, f32)> = Vec::new();

        for (j, other) in entities.iter().enumerate() {
            if i == j {
                continue;
            }

            let density_diff = entity
                .density_band
                .index()
                .abs_diff(other.density_band.index());
            if density_diff > 2 {
                continue;
            }

            let dx = entity.position[0] as f32 - other.position[0] as f32;
            let dy = entity.position[1] as f32 - other.position[1] as f32;
            let dz = entity.position[2] as f32 - other.position[2] as f32;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance < threshold {
                candidates.push((j, distance));
            }
        }

        candidates.sort_by(|(a_idx, a_dist), (b_idx, b_dist)| {
            a_dist
                .partial_cmp(b_dist)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a_idx.cmp(b_idx))
        });

        for (j, distance) in candidates.into_iter().take(MAX_NEIGHBORS) {
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            if !seen_edges.insert((a, b)) {
                continue;
            }

            let from = [
                entities[a].position[0] as f32,
                entities[a].position[1] as f32,
                entities[a].position[2] as f32,
            ];
            let to = [
                entities[b].position[0] as f32,
                entities[b].position[1] as f32,
                entities[b].position[2] as f32,
            ];

            let proximity = (1.0 - (distance / threshold)).clamp(0.0, 1.0);
            let consciousness_avg =
                (((entities[a].consciousness + entities[b].consciousness) * 0.5) as f32)
                    .clamp(0.0, 1.0);
            let intensity = (0.65 * proximity + 0.35 * consciousness_avg).clamp(0.0, 1.0);

            let connection = if entities[a].in_collective && entities[b].in_collective {
                HierarchyConnection::parent_child(from, to, intensity)
            } else if entities[a].density_band == entities[b].density_band {
                HierarchyConnection::composition(from, to, intensity)
            } else {
                HierarchyConnection::environment(from, to, intensity)
            };

            connections.push(connection);
            if connections.len() >= MAX_EDGES {
                return connections;
            }
        }
    }

    connections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_connection_size() {
        assert_eq!(std::mem::size_of::<HierarchyConnection>(), 44);
        assert_eq!(std::mem::align_of::<HierarchyConnection>(), 4);
    }

    #[test]
    fn test_parent_child_connection() {
        let conn = HierarchyConnection::parent_child([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.8);
        assert_eq!(conn.from_position, [0.0, 0.0, 0.0]);
        assert_eq!(conn.to_position, [1.0, 1.0, 0.0]);
        assert_eq!(conn.connection_type, ConnectionType::ParentChild as u32);
        assert_eq!(conn.intensity, 0.8);
    }

    #[test]
    fn test_composition_connection() {
        let conn = HierarchyConnection::composition([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.6);
        assert_eq!(conn.from_position, [0.0, 0.0, 0.0]);
        assert_eq!(conn.to_position, [1.0, 1.0, 0.0]);
        assert_eq!(conn.connection_type, ConnectionType::Composition as u32);
        assert_eq!(conn.intensity, 0.6);
    }

    #[test]
    fn test_environment_connection() {
        let conn = HierarchyConnection::environment([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.4);
        assert_eq!(conn.from_position, [0.0, 0.0, 0.0]);
        assert_eq!(conn.to_position, [1.0, 1.0, 0.0]);
        assert_eq!(conn.connection_type, ConnectionType::Environment as u32);
        assert_eq!(conn.intensity, 0.4);
        assert_eq!(conn.phase_delta, 0.0);
    }

    #[test]
    fn test_phase_coherence_derives_phase_delta_from_coherence() {
        let conn = HierarchyConnection::phase_coherence([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.75);
        assert_eq!(conn.connection_type, ConnectionType::PhaseCoherence as u32);
        assert!((conn.phase_delta - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_entanglement_connection_type() {
        let conn = HierarchyConnection::entanglement([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.9, 0.1);
        assert_eq!(conn.connection_type, ConnectionType::Entanglement as u32);
        assert_eq!(conn.intensity, 0.9);
        assert_eq!(conn.phase_delta, 0.1);
    }

    #[test]
    fn test_intensity_clamping() {
        let conn1 = HierarchyConnection::parent_child([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], -0.5);
        assert_eq!(conn1.intensity, 0.0);

        let conn2 = HierarchyConnection::parent_child([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], 1.5);
        assert_eq!(conn2.intensity, 1.0);
    }
}
