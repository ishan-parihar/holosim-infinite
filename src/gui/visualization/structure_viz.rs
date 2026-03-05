//! Physical Structure Visualization
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Physical matter emerges simultaneously at all scales
//! through the holographic principle - each entity contains the whole. Structure is not
//! built from bottom-up but emerges from the interplay of consciousness fields."
//!
//! This module provides visualization for:
//! - Hierarchical composition (atoms → molecules → cells → organisms)
//! - Simultaneous emergence across scales
//! - Physical matter visualization
//! - Structure tree representation

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::types::{Density, Polarity};
use nalgebra_glm::Vec3;
use std::collections::HashMap;

/// Simple Entity structure for visualization (placeholder)
#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_id: EntityId,
    pub entity_type: EntityType,
    pub position: Vec3,
    pub scale: f64,
    pub density: Density,
    pub polarity: Polarity,
    pub consciousness: f64,
    pub health: f64,
    pub archetype_activations: Vec<(usize, f64)>,
    pub evolution_clock: f64,
    pub spectrum_position: f64,
}

/// Level of physical structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StructureLevel {
    /// Quantum level (10^-35 to 10^-15 m)
    Quantum,
    /// Atomic level (10^-15 to 10^-10 m)
    Atomic,
    /// Molecular level (10^-10 to 10^-6 m)
    Molecular,
    /// Cellular level (10^-6 to 10^-4 m)
    Cellular,
    /// Tissue level (10^-4 to 10^-2 m)
    Tissue,
    /// Organism level (10^-2 to 10^0 m)
    Organism,
    /// Planetary level (10^0 to 10^8 m)
    Planetary,
    /// Stellar level (10^8 to 10^13 m)
    Stellar,
    /// Galactic level (10^13 to 10^26 m)
    Galactic,
}

impl StructureLevel {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            StructureLevel::Quantum => "Quantum",
            StructureLevel::Atomic => "Atomic",
            StructureLevel::Molecular => "Molecular",
            StructureLevel::Cellular => "Cellular",
            StructureLevel::Tissue => "Tissue",
            StructureLevel::Organism => "Organism",
            StructureLevel::Planetary => "Planetary",
            StructureLevel::Stellar => "Stellar",
            StructureLevel::Galactic => "Galactic",
        }
    }

    /// Get scale range in meters (log10)
    pub fn scale_range(&self) -> (f64, f64) {
        match self {
            StructureLevel::Quantum => (-35.0, -15.0),
            StructureLevel::Atomic => (-15.0, -10.0),
            StructureLevel::Molecular => (-10.0, -6.0),
            StructureLevel::Cellular => (-6.0, -4.0),
            StructureLevel::Tissue => (-4.0, -2.0),
            StructureLevel::Organism => (-2.0, 0.0),
            StructureLevel::Planetary => (0.0, 8.0),
            StructureLevel::Stellar => (8.0, 13.0),
            StructureLevel::Galactic => (13.0, 26.0),
        }
    }

    /// Get color for visualization
    pub fn color(&self) -> [f32; 4] {
        match self {
            StructureLevel::Quantum => [0.8, 0.2, 1.0, 0.8], // Purple
            StructureLevel::Atomic => [0.2, 0.6, 1.0, 0.8],  // Blue
            StructureLevel::Molecular => [0.2, 1.0, 0.6, 0.8], // Cyan
            StructureLevel::Cellular => [0.6, 1.0, 0.2, 0.8], // Green
            StructureLevel::Tissue => [1.0, 1.0, 0.2, 0.8],  // Yellow
            StructureLevel::Organism => [1.0, 0.6, 0.2, 0.8], // Orange
            StructureLevel::Planetary => [1.0, 0.2, 0.2, 0.8], // Red
            StructureLevel::Stellar => [1.0, 0.8, 0.2, 0.8], // Gold
            StructureLevel::Galactic => [0.8, 0.2, 0.6, 0.8], // Magenta
        }
    }

    /// Get typical entity count at this level
    pub fn typical_entity_count(&self) -> usize {
        match self {
            StructureLevel::Quantum => 1000000,
            StructureLevel::Atomic => 100000,
            StructureLevel::Molecular => 10000,
            StructureLevel::Cellular => 1000,
            StructureLevel::Tissue => 100,
            StructureLevel::Organism => 10,
            StructureLevel::Planetary => 5,
            StructureLevel::Stellar => 2,
            StructureLevel::Galactic => 1,
        }
    }
}

/// Type of physical structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructureType {
    /// Subatomic particle
    SubatomicParticle,
    /// Atom
    Atom,
    /// Molecule
    Molecule,
    /// Cell
    Cell,
    /// Organ
    Organ,
    /// Organism
    Organism,
    /// Planet
    Planet,
    /// Star
    Star,
    /// Galaxy
    Galaxy,
    /// Social complex (meta-structure)
    SocialComplex,
}

impl StructureType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            StructureType::SubatomicParticle => "Subatomic Particle",
            StructureType::Atom => "Atom",
            StructureType::Molecule => "Molecule",
            StructureType::Cell => "Cell",
            StructureType::Organ => "Organ",
            StructureType::Organism => "Organism",
            StructureType::Planet => "Planet",
            StructureType::Star => "Star",
            StructureType::Galaxy => "Galaxy",
            StructureType::SocialComplex => "Social Complex",
        }
    }

    /// Get icon for visualization
    pub fn icon(&self) -> &'static str {
        match self {
            StructureType::SubatomicParticle => "●",
            StructureType::Atom => "◉",
            StructureType::Molecule => "⬡",
            StructureType::Cell => "⬢",
            StructureType::Organ => "⬣",
            StructureType::Organism => "⬤",
            StructureType::Planet => "🌍",
            StructureType::Star => "★",
            StructureType::Galaxy => "🌀",
            StructureType::SocialComplex => "🔮",
        }
    }
}

/// Structure node representing a physical entity
#[derive(Debug, Clone)]
pub struct StructureNode {
    /// Node identifier
    pub node_id: usize,
    /// Entity ID
    pub entity_id: EntityId,
    /// Structure level
    pub level: StructureLevel,
    /// Structure type
    pub structure_type: StructureType,
    /// Position in world space
    pub position: Vec3,
    /// Scale (log10 meters)
    pub scale: f64,
    /// Parent node ID (None for root)
    pub parent_id: Option<usize>,
    /// Child node IDs
    pub children: Vec<usize>,
    /// Health (0.0 to 1.0)
    pub health: f64,
    /// Density
    pub density: Density,
    /// Emergence timestamp
    pub emergence_time: f64,
    /// Has simultaneous emergence
    pub simultaneous_emergence: bool,
    /// Number of subcomponents
    pub component_count: usize,
}

impl StructureNode {
    /// Calculate depth in hierarchy
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut current_id = self.parent_id;
        // Simplified: actual implementation would need tree traversal
        while current_id.is_some() {
            depth += 1;
            current_id = None; // Placeholder
        }
        depth
    }

    /// Check if is leaf node
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Check if is root node
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

/// Structure metrics for a level
#[derive(Debug, Clone)]
pub struct StructureMetrics {
    /// Total nodes at this level
    pub total_nodes: usize,
    /// Average health
    pub average_health: f64,
    /// Total components
    pub total_components: usize,
    /// Average components per node
    pub average_components: f64,
    /// Nodes with simultaneous emergence
    pub simultaneous_emergence_count: usize,
    /// Emergence percentage
    pub emergence_percentage: f64,
    /// Density distribution
    pub density_distribution: HashMap<Density, usize>,
}

impl Default for StructureMetrics {
    fn default() -> Self {
        StructureMetrics {
            total_nodes: 0,
            average_health: 0.0,
            total_components: 0,
            average_components: 0.0,
            simultaneous_emergence_count: 0,
            emergence_percentage: 0.0,
            density_distribution: HashMap::new(),
        }
    }
}

/// Physical structure visualizer
pub struct StructureVisualizer {
    /// All structure nodes
    pub nodes: Vec<StructureNode>,
    /// Root nodes (top-level structures)
    pub roots: Vec<usize>,
    /// Metrics per level
    pub metrics: HashMap<StructureLevel, StructureMetrics>,
    /// History of metrics
    pub metrics_history: Vec<(f64, HashMap<StructureLevel, StructureMetrics>)>,
    /// Maximum history length
    pub max_history: usize,
    /// Next node ID
    pub next_node_id: usize,
    /// Show visualization
    pub show_visualization: bool,
    /// Show hierarchy lines
    pub show_hierarchy_lines: bool,
    /// Show component counts
    pub show_component_counts: bool,
    /// Selected node
    pub selected_node: Option<usize>,
    /// Expanded nodes
    pub expanded_nodes: Vec<usize>,
}

impl StructureVisualizer {
    /// Create new structure visualizer
    pub fn new() -> Self {
        StructureVisualizer {
            nodes: Vec::new(),
            roots: Vec::new(),
            metrics: HashMap::new(),
            metrics_history: Vec::new(),
            max_history: 1000,
            next_node_id: 0,
            show_visualization: true,
            show_hierarchy_lines: true,
            show_component_counts: true,
            selected_node: None,
            expanded_nodes: Vec::new(),
        }
    }

    /// Update structure visualization from entities
    pub fn update(&mut self, entities: &[Entity], time: f64) {
        // Clear previous state
        self.nodes.clear();
        self.roots.clear();
        self.next_node_id = 0;

        // Build structure hierarchy
        self.build_structure_hierarchy(entities, time);

        // Calculate metrics
        self.calculate_metrics();

        // Store history
        self.metrics_history.push((time, self.metrics.clone()));
        if self.metrics_history.len() > self.max_history {
            self.metrics_history.remove(0);
        }
    }

    /// Build structure hierarchy from entities
    fn build_structure_hierarchy(&mut self, entities: &[Entity], time: f64) {
        // Group entities by structure level
        let mut level_groups: HashMap<StructureLevel, Vec<&Entity>> = HashMap::new();

        for entity in entities {
            let level = self.determine_structure_level(entity);
            level_groups
                .entry(level)
                .or_default()
                .push(entity);
        }

        // Create hierarchy from bottom (quantum) to top (galactic)
        let levels = [
            StructureLevel::Quantum,
            StructureLevel::Atomic,
            StructureLevel::Molecular,
            StructureLevel::Cellular,
            StructureLevel::Tissue,
            StructureLevel::Organism,
            StructureLevel::Planetary,
            StructureLevel::Stellar,
            StructureLevel::Galactic,
        ];

        for level in levels {
            if let Some(entities_at_level) = level_groups.get(&level) {
                self.create_nodes_for_level(entities_at_level, level, time);
            }
        }
    }

    /// Determine structure level for an entity
    fn determine_structure_level(&self, entity: &Entity) -> StructureLevel {
        let log10_scale = entity.scale.log10();

        if log10_scale < -15.0 {
            StructureLevel::Quantum
        } else if log10_scale < -10.0 {
            StructureLevel::Atomic
        } else if log10_scale < -6.0 {
            StructureLevel::Molecular
        } else if log10_scale < -4.0 {
            StructureLevel::Cellular
        } else if log10_scale < -2.0 {
            StructureLevel::Tissue
        } else if log10_scale < 0.0 {
            StructureLevel::Organism
        } else if log10_scale < 8.0 {
            StructureLevel::Planetary
        } else if log10_scale < 13.0 {
            StructureLevel::Stellar
        } else {
            StructureLevel::Galactic
        }
    }

    /// Create nodes for a structure level
    fn create_nodes_for_level(&mut self, entities: &[&Entity], level: StructureLevel, time: f64) {
        for entity in entities {
            let structure_type = self.determine_structure_type(entity, level);

            let node = StructureNode {
                node_id: self.next_node_id,
                entity_id: entity.entity_id.clone(),
                level,
                structure_type,
                position: entity.position,
                scale: entity.scale,
                parent_id: self.find_parent(entity, level),
                children: Vec::new(),
                health: entity.health,
                density: entity.density,
                emergence_time: time,
                simultaneous_emergence: entity.consciousness > 0.5,
                component_count: self.estimate_component_count(entity, level),
            };

            self.nodes.push(node);

            // Update parent's children list
            if let Some(parent_id) = nodes_last(&self.nodes).parent_id {
                if let Some(parent) = self.nodes.get_mut(parent_id) {
                    parent.children.push(self.next_node_id);
                }
            } else {
                // Root node
                self.roots.push(self.next_node_id);
            }

            self.next_node_id += 1;
        }
    }

    /// Determine structure type
    fn determine_structure_type(&self, _entity: &Entity, level: StructureLevel) -> StructureType {
        match level {
            StructureLevel::Quantum => StructureType::SubatomicParticle,
            StructureLevel::Atomic => StructureType::Atom,
            StructureLevel::Molecular => StructureType::Molecule,
            StructureLevel::Cellular => StructureType::Cell,
            StructureLevel::Tissue => StructureType::Organ,
            StructureLevel::Organism => StructureType::Organism,
            StructureLevel::Planetary => StructureType::Planet,
            StructureLevel::Stellar => StructureType::Star,
            StructureLevel::Galactic => StructureType::Galaxy,
        }
    }

    /// Find parent entity for hierarchical relationship
    fn find_parent(&self, entity: &Entity, level: StructureLevel) -> Option<usize> {
        // Simplified: find nearest larger-scale entity
        let scale_factor = 10.0; // Parent is ~10x larger
        let _target_scale = entity.scale * scale_factor;

        for (i, node) in self.nodes.iter().enumerate() {
            if node.level < level {
                let ratio = node.scale / entity.scale;
                if ratio > scale_factor * 0.5 && ratio < scale_factor * 2.0 {
                    let distance = (entity.position - node.position).magnitude() as f64;
                    if distance < entity.scale * 5.0 {
                        return Some(i);
                    }
                }
            }
        }

        None
    }

    /// Estimate component count
    fn estimate_component_count(&self, entity: &Entity, level: StructureLevel) -> usize {
        // Higher consciousness = more organized = more components
        let base_count = match level {
            StructureLevel::Quantum => 1,
            StructureLevel::Atomic => 10,
            StructureLevel::Molecular => 100,
            StructureLevel::Cellular => 1000,
            StructureLevel::Tissue => 100,
            StructureLevel::Organism => 10,
            StructureLevel::Planetary => 5,
            StructureLevel::Stellar => 2,
            StructureLevel::Galactic => 1,
        };

        let consciousness_factor = (entity.consciousness * 2.0) as usize;
        base_count * (1 + consciousness_factor)
    }

    /// Calculate metrics for each level
    fn calculate_metrics(&mut self) {
        self.metrics.clear();

        for level in [
            StructureLevel::Quantum,
            StructureLevel::Atomic,
            StructureLevel::Molecular,
            StructureLevel::Cellular,
            StructureLevel::Tissue,
            StructureLevel::Organism,
            StructureLevel::Planetary,
            StructureLevel::Stellar,
            StructureLevel::Galactic,
        ] {
            let nodes_at_level: Vec<&StructureNode> =
                self.nodes.iter().filter(|n| n.level == level).collect();

            let total_nodes = nodes_at_level.len();
            let average_health = if total_nodes > 0 {
                nodes_at_level.iter().map(|n| n.health).sum::<f64>() / total_nodes as f64
            } else {
                0.0
            };
            let total_components = nodes_at_level
                .iter()
                .map(|n| n.component_count)
                .sum::<usize>();
            let average_components = if total_nodes > 0 {
                total_components as f64 / total_nodes as f64
            } else {
                0.0
            };
            let simultaneous_emergence_count = nodes_at_level
                .iter()
                .filter(|n| n.simultaneous_emergence)
                .count();
            let emergence_percentage = if total_nodes > 0 {
                simultaneous_emergence_count as f64 / total_nodes as f64
            } else {
                0.0
            };

            let mut density_distribution = HashMap::new();
            for node in &nodes_at_level {
                *density_distribution.entry(node.density).or_insert(0) += 1;
            }

            self.metrics.insert(
                level,
                StructureMetrics {
                    total_nodes,
                    average_health,
                    total_components,
                    average_components,
                    simultaneous_emergence_count,
                    emergence_percentage,
                    density_distribution,
                },
            );
        }
    }

    /// Get node by ID
    pub fn get_node(&self, node_id: usize) -> Option<&StructureNode> {
        self.nodes.get(node_id)
    }

    /// Get children of a node
    pub fn get_children(&self, node_id: usize) -> Vec<&StructureNode> {
        if let Some(node) = self.get_node(node_id) {
            node.children
                .iter()
                .filter_map(|&child_id| self.get_node(child_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all nodes at a level
    pub fn get_nodes_at_level(&self, level: StructureLevel) -> Vec<&StructureNode> {
        self.nodes.iter().filter(|n| n.level == level).collect()
    }

    /// Get metrics for a level
    pub fn get_metrics(&self, level: StructureLevel) -> Option<&StructureMetrics> {
        self.metrics.get(&level)
    }

    /// Toggle expansion of a node
    pub fn toggle_expansion(&mut self, node_id: usize) {
        if let Some(pos) = self.expanded_nodes.iter().position(|&id| id == node_id) {
            self.expanded_nodes.remove(pos);
        } else {
            self.expanded_nodes.push(node_id);
        }
    }

    /// Check if node is expanded
    pub fn is_expanded(&self, node_id: usize) -> bool {
        self.expanded_nodes.contains(&node_id)
    }

    /// Select a node
    pub fn select_node(&mut self, node_id: Option<usize>) {
        self.selected_node = node_id;
    }

    /// Get selected node
    pub fn get_selected_node(&self) -> Option<&StructureNode> {
        self.selected_node.and_then(|id| self.get_node(id))
    }

    /// Get hierarchy path for a node
    pub fn get_hierarchy_path(&self, node_id: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current_id = Some(node_id);

        while let Some(id) = current_id {
            path.push(id);
            current_id = self.get_node(id).and_then(|n| n.parent_id);
        }

        path.reverse();
        path
    }
}

impl Default for StructureVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to get last node reference
fn nodes_last(nodes: &[StructureNode]) -> &StructureNode {
    nodes.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_level_names() {
        assert_eq!(StructureLevel::Quantum.name(), "Quantum");
        assert_eq!(StructureLevel::Atomic.name(), "Atomic");
        assert_eq!(StructureLevel::Galactic.name(), "Galactic");
    }

    #[test]
    fn test_structure_level_scale_ranges() {
        assert_eq!(StructureLevel::Quantum.scale_range(), (-35.0, -15.0));
        assert_eq!(StructureLevel::Atomic.scale_range(), (-15.0, -10.0));
        assert_eq!(StructureLevel::Galactic.scale_range(), (13.0, 26.0));
    }

    #[test]
    fn test_structure_level_colors() {
        assert_eq!(StructureLevel::Quantum.color(), [0.8, 0.2, 1.0, 0.8]);
        assert_eq!(StructureLevel::Atomic.color(), [0.2, 0.6, 1.0, 0.8]);
    }

    #[test]
    fn test_structure_type_names() {
        assert_eq!(
            StructureType::SubatomicParticle.name(),
            "Subatomic Particle"
        );
        assert_eq!(StructureType::Atom.name(), "Atom");
        assert_eq!(StructureType::Galaxy.name(), "Galaxy");
    }

    #[test]
    fn test_structure_type_icons() {
        assert_eq!(StructureType::SubatomicParticle.icon(), "●");
        assert_eq!(StructureType::Atom.icon(), "◉");
        assert_eq!(StructureType::Galaxy.icon(), "🌀");
    }

    #[test]
    fn test_structure_node_is_leaf() {
        let node = StructureNode {
            node_id: 0,
            entity_id: EntityId::new("1".to_string()),
            level: StructureLevel::Atomic,
            structure_type: StructureType::Atom,
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0e-10,
            parent_id: None,
            children: Vec::new(),
            health: 0.9,
            density: Density::Third,
            emergence_time: 0.0,
            simultaneous_emergence: false,
            component_count: 10,
        };

        assert!(node.is_leaf());
        assert!(node.is_root());
    }

    #[test]
    fn test_structure_metrics_default() {
        let metrics = StructureMetrics::default();
        assert_eq!(metrics.total_nodes, 0);
        assert_eq!(metrics.average_health, 0.0);
        assert_eq!(metrics.total_components, 0);
    }

    #[test]
    fn test_structure_visualizer_new() {
        let visualizer = StructureVisualizer::new();
        assert!(visualizer.nodes.is_empty());
        assert!(visualizer.roots.is_empty());
        assert_eq!(visualizer.next_node_id, 0);
        assert!(visualizer.show_visualization);
    }

    #[test]
    fn test_determine_structure_level() {
        let visualizer = StructureVisualizer::new();

        let mut entity = Entity {
            entity_id: EntityId::new("1".to_string()),
            entity_type: EntityType::Individual,
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0e-12, // log10 = -12, which is in Atomic range (-15, -10)
            density: Density::Third,
            polarity: Polarity::ServiceToOthers,
            consciousness: 0.5,
            health: 0.9,
            archetype_activations: Vec::new(),
            evolution_clock: 0.0,
            spectrum_position: 0.5,
        };

        let level = visualizer.determine_structure_level(&entity);
        assert_eq!(level, StructureLevel::Atomic);

        entity.scale = 1.0e-8; // log10 = -8, which is in Molecular range (-10, -6)
        let level = visualizer.determine_structure_level(&entity);
        assert_eq!(level, StructureLevel::Molecular);
    }

    #[test]
    fn test_select_node() {
        let mut visualizer = StructureVisualizer::new();
        visualizer.select_node(Some(5));
        assert_eq!(visualizer.selected_node, Some(5));
        visualizer.select_node(None);
        assert_eq!(visualizer.selected_node, None);
    }

    #[test]
    fn test_toggle_expansion() {
        let mut visualizer = StructureVisualizer::new();
        assert!(!visualizer.is_expanded(5));
        visualizer.toggle_expansion(5);
        assert!(visualizer.is_expanded(5));
        visualizer.toggle_expansion(5);
        assert!(!visualizer.is_expanded(5));
    }
}
