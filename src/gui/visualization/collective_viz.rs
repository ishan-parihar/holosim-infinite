//! Collective Dynamics Visualization
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Collective consciousness emerges through resonance
//! between entities sharing similar frequencies, archetypes, or purposes. Groups form not by
//! proximity but by vibrational alignment."
//!
//! This module provides visualization for:
//! - Resonance fields (heatmap visualization of collective consciousness)
//! - Group consciousness patterns
//! - Collective behavior dynamics
//! - Social complex formation

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::types::{Density, Polarity};

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
use nalgebra_glm::{Vec2, Vec3};
use std::collections::HashMap;

/// Level of collective consciousness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectiveLevel {
    /// Local group (2-10 entities)
    Local,
    /// Community (10-100 entities)
    Community,
    /// Regional collective (100-1000 entities)
    Regional,
    /// Planetary collective (1000+ entities)
    Planetary,
    /// Social complex (archetype-based grouping)
    SocialComplex,
}

impl CollectiveLevel {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            CollectiveLevel::Local => "Local",
            CollectiveLevel::Community => "Community",
            CollectiveLevel::Regional => "Regional",
            CollectiveLevel::Planetary => "Planetary",
            CollectiveLevel::SocialComplex => "Social Complex",
        }
    }

    /// Get color for visualization
    pub fn color(&self) -> [f32; 4] {
        match self {
            CollectiveLevel::Local => [0.4, 0.8, 0.4, 0.3], // Green
            CollectiveLevel::Community => [0.4, 0.6, 0.9, 0.4], // Blue
            CollectiveLevel::Regional => [0.6, 0.4, 0.9, 0.5], // Purple
            CollectiveLevel::Planetary => [0.9, 0.6, 0.4, 0.6], // Orange
            CollectiveLevel::SocialComplex => [0.9, 0.4, 0.8, 0.7], // Magenta
        }
    }

    /// Get max entity count for this level
    pub fn max_entities(&self) -> usize {
        match self {
            CollectiveLevel::Local => 10,
            CollectiveLevel::Community => 100,
            CollectiveLevel::Regional => 1000,
            CollectiveLevel::Planetary => 10000,
            CollectiveLevel::SocialComplex => 1000,
        }
    }
}

/// Type of resonance between entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceType {
    /// Density resonance (same density level)
    Density,
    /// Polarity resonance (same polarity)
    Polarity,
    /// Archetype resonance (shared archetype activation)
    Archetype,
    /// Purpose resonance (shared goals/intent)
    Purpose,
    /// Geographic resonance (physical proximity)
    Geographic,
    /// Temporal resonance (synchronized timing)
    Temporal,
}

impl ResonanceType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ResonanceType::Density => "Density",
            ResonanceType::Polarity => "Polarity",
            ResonanceType::Archetype => "Archetype",
            ResonanceType::Purpose => "Purpose",
            ResonanceType::Geographic => "Geographic",
            ResonanceType::Temporal => "Temporal",
        }
    }

    /// Get color for visualization
    pub fn color(&self) -> [f32; 4] {
        match self {
            ResonanceType::Density => [1.0, 0.5, 0.3, 0.5], // Orange
            ResonanceType::Polarity => [0.3, 0.5, 1.0, 0.5], // Blue
            ResonanceType::Archetype => [0.9, 0.3, 0.9, 0.5], // Purple
            ResonanceType::Purpose => [0.3, 1.0, 0.5, 0.5], // Green
            ResonanceType::Geographic => [1.0, 1.0, 0.3, 0.5], // Yellow
            ResonanceType::Temporal => [1.0, 0.3, 0.3, 0.5], // Red
        }
    }
}

/// Collective group of entities
#[derive(Debug, Clone)]
pub struct CollectiveGroup {
    /// Unique group identifier
    pub group_id: usize,
    /// Level of collective consciousness
    pub level: CollectiveLevel,
    /// Type of resonance
    pub resonance_type: ResonanceType,
    /// Member entities
    pub members: Vec<EntityId>,
    /// Center of collective
    pub center: Vec3,
    /// Resonance strength (0.0 to 1.0)
    pub resonance_strength: f64,
    /// Coherence (0.0 to 1.0)
    pub coherence: f64,
    /// Primary archetype (if archetype-based)
    pub primary_archetype: Option<usize>,
    /// Average density
    pub average_density: Density,
    /// Dominant polarity
    pub dominant_polarity: Polarity,
    /// Creation timestamp
    pub creation_time: f64,
    /// Group health (0.0 to 1.0)
    pub health: f64,
}

impl CollectiveGroup {
    /// Calculate size (number of members)
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Calculate radius (spread of members)
    pub fn radius(&self) -> f64 {
        // Simplified: scale with member count
        (self.size() as f64).sqrt() * 10.0
    }

    /// Check if entity is member
    pub fn is_member(&self, entity_id: &EntityId) -> bool {
        self.members.contains(entity_id)
    }
}

/// Resonance field point for heatmap visualization
#[derive(Debug, Clone, Copy)]
pub struct ResonanceFieldPoint {
    /// Position in world space
    pub position: Vec3,
    /// Resonance intensity (0.0 to 1.0)
    pub intensity: f64,
    /// Resonance type
    pub resonance_type: ResonanceType,
}

/// Collective dynamics metrics
#[derive(Debug, Clone)]
pub struct CollectiveMetrics {
    /// Total number of collectives
    pub total_collectives: usize,
    /// Total entities in collectives
    pub entities_in_collectives: usize,
    /// Average collective size
    pub average_collective_size: f64,
    /// Average resonance strength
    pub average_resonance: f64,
    /// Average coherence
    pub average_coherence: f64,
    /// Collective health index (0.0 to 1.0)
    pub health_index: f64,
    /// Distribution by level
    pub level_distribution: HashMap<CollectiveLevel, usize>,
    /// Distribution by resonance type
    pub resonance_distribution: HashMap<ResonanceType, usize>,
    /// Largest collective size
    pub largest_collective_size: usize,
    /// Most common resonance type
    pub most_common_resonance: Option<ResonanceType>,
}

impl Default for CollectiveMetrics {
    fn default() -> Self {
        CollectiveMetrics {
            total_collectives: 0,
            entities_in_collectives: 0,
            average_collective_size: 0.0,
            average_resonance: 0.0,
            average_coherence: 0.0,
            health_index: 0.0,
            level_distribution: HashMap::new(),
            resonance_distribution: HashMap::new(),
            largest_collective_size: 0,
            most_common_resonance: None,
        }
    }
}

/// Collective behavior pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorPattern {
    /// Coordinated movement
    Coordinated,
    /// Synchronized action
    Synchronized,
    /// Emergent intelligence
    Emergent,
    /// Swarm behavior
    Swarm,
    /// Hierarchical organization
    Hierarchical,
    /// Distributed network
    Distributed,
}

impl BehaviorPattern {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            BehaviorPattern::Coordinated => "Coordinated",
            BehaviorPattern::Synchronized => "Synchronized",
            BehaviorPattern::Emergent => "Emergent",
            BehaviorPattern::Swarm => "Swarm",
            BehaviorPattern::Hierarchical => "Hierarchical",
            BehaviorPattern::Distributed => "Distributed",
        }
    }

    /// Get color for visualization
    pub fn color(&self) -> [f32; 4] {
        match self {
            BehaviorPattern::Coordinated => [0.3, 0.9, 0.5, 0.6], // Green
            BehaviorPattern::Synchronized => [0.3, 0.5, 0.9, 0.6], // Blue
            BehaviorPattern::Emergent => [0.9, 0.3, 0.9, 0.6],    // Purple
            BehaviorPattern::Swarm => [0.9, 0.9, 0.3, 0.6],       // Yellow
            BehaviorPattern::Hierarchical => [0.9, 0.5, 0.3, 0.6], // Orange
            BehaviorPattern::Distributed => [0.5, 0.5, 0.5, 0.6], // Gray
        }
    }
}

/// Collective dynamics visualizer
pub struct CollectiveVisualizer {
    /// Collective groups
    pub groups: Vec<CollectiveGroup>,
    /// Resonance field points for heatmap
    pub resonance_field: Vec<ResonanceFieldPoint>,
    /// Current metrics
    pub metrics: CollectiveMetrics,
    /// History of metrics for graphing
    pub metrics_history: Vec<(f64, CollectiveMetrics)>,
    /// Maximum history length
    pub max_history: usize,
    /// Next group ID
    pub next_group_id: usize,
    /// Show visualization
    pub show_visualization: bool,
    /// Show resonance field
    pub show_resonance_field: bool,
    /// Show group boundaries
    pub show_group_boundaries: bool,
    /// Show connections
    pub show_connections: bool,
    /// Heatmap resolution
    pub heatmap_resolution: usize,
    /// Minimum resonance for visualization
    pub min_resonance_threshold: f64,
}

impl CollectiveVisualizer {
    /// Create new collective visualizer
    pub fn new() -> Self {
        CollectiveVisualizer {
            groups: Vec::new(),
            resonance_field: Vec::new(),
            metrics: CollectiveMetrics::default(),
            metrics_history: Vec::new(),
            max_history: 1000,
            next_group_id: 0,
            show_visualization: true,
            show_resonance_field: true,
            show_group_boundaries: true,
            show_connections: true,
            heatmap_resolution: 64,
            min_resonance_threshold: 0.3,
        }
    }

    /// Update collective visualization from entities
    pub fn update(&mut self, entities: &[Entity], time: f64) {
        // Clear previous state
        self.groups.clear();
        self.resonance_field.clear();

        // Identify collectives based on resonance
        self.identify_collectives(entities, time);

        // Generate resonance field
        self.generate_resonance_field();

        // Calculate metrics
        self.calculate_metrics();

        // Store history
        self.metrics_history.push((time, self.metrics.clone()));
        if self.metrics_history.len() > self.max_history {
            self.metrics_history.remove(0);
        }
    }

    /// Identify collectives from entities
    fn identify_collectives(&mut self, entities: &[Entity], time: f64) {
        // Group entities by density
        let mut density_groups: HashMap<Density, Vec<&Entity>> = HashMap::new();
        for entity in entities {
            density_groups
                .entry(entity.density)
                .or_insert_with(Vec::new)
                .push(entity);
        }

        // Create collectives for each density group
        for (density, group_entities) in density_groups {
            if group_entities.len() >= 2 {
                let level = if group_entities.len() <= 10 {
                    CollectiveLevel::Local
                } else if group_entities.len() <= 100 {
                    CollectiveLevel::Community
                } else if group_entities.len() <= 1000 {
                    CollectiveLevel::Regional
                } else {
                    CollectiveLevel::Planetary
                };

                let center = self.calculate_group_center(&group_entities);
                let resonance_strength = self.calculate_resonance_strength(&group_entities);
                let coherence = self.calculate_coherence(&group_entities);

                let group = CollectiveGroup {
                    group_id: self.next_group_id,
                    level,
                    resonance_type: ResonanceType::Density,
                    members: group_entities.iter().map(|e| e.entity_id.clone()).collect(),
                    center,
                    resonance_strength,
                    coherence,
                    primary_archetype: None,
                    average_density: density,
                    dominant_polarity: self.calculate_dominant_polarity(&group_entities),
                    creation_time: time,
                    health: coherence * resonance_strength,
                };

                self.groups.push(group);
                self.next_group_id += 1;
            }
        }

        // Group entities by polarity
        let mut polarity_groups: HashMap<Polarity, Vec<&Entity>> = HashMap::new();
        for entity in entities {
            polarity_groups
                .entry(entity.polarity)
                .or_insert_with(Vec::new)
                .push(entity);
        }

        // Create collectives for each polarity group (if large enough)
        for (polarity, group_entities) in polarity_groups {
            if group_entities.len() >= 5 && group_entities.len() <= 20 {
                let center = self.calculate_group_center(&group_entities);
                let resonance_strength = self.calculate_resonance_strength(&group_entities);
                let coherence = self.calculate_coherence(&group_entities);

                let group = CollectiveGroup {
                    group_id: self.next_group_id,
                    level: CollectiveLevel::Local,
                    resonance_type: ResonanceType::Polarity,
                    members: group_entities.iter().map(|e| e.entity_id.clone()).collect(),
                    center,
                    resonance_strength,
                    coherence,
                    primary_archetype: None,
                    average_density: self.calculate_average_density(&group_entities),
                    dominant_polarity: polarity,
                    creation_time: time,
                    health: coherence * resonance_strength,
                };

                self.groups.push(group);
                self.next_group_id += 1;
            }
        }
    }

    /// Calculate group center
    fn calculate_group_center(&self, entities: &[&Entity]) -> Vec3 {
        if entities.is_empty() {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let mut sum = Vec3::new(0.0_f32, 0.0_f32, 0.0_f32);
        for entity in entities {
            sum += entity.position.clone();
        }

        let count = entities.len() as f32;
        Vec3::new(sum.x / count, sum.y / count, sum.z / count)
    }

    /// Calculate resonance strength
    fn calculate_resonance_strength(&self, entities: &[&Entity]) -> f64 {
        if entities.len() < 2 {
            return 0.0;
        }

        // Calculate average pairwise similarity
        let mut total_similarity = 0.0;
        let mut comparisons = 0;

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let similarity = self.calculate_similarity(entities[i], entities[j]);
                total_similarity += similarity;
                comparisons += 1;
            }
        }

        if comparisons == 0 {
            0.0
        } else {
            total_similarity / comparisons as f64
        }
    }

    /// Calculate similarity between two entities
    fn calculate_similarity(&self, a: &Entity, b: &Entity) -> f64 {
        let mut similarity = 0.0;

        // Density similarity
        if a.density == b.density {
            similarity += 0.3;
        }

        // Polarity similarity
        if a.polarity == b.polarity {
            similarity += 0.2;
        }

        // Consciousness similarity
        let consciousness_diff = (a.consciousness - b.consciousness).abs();
        similarity += (1.0 - consciousness_diff) * 0.3;

        // Health similarity
        let health_diff = (a.health - b.health).abs();
        similarity += (1.0 - health_diff) * 0.2;

        similarity.min(1.0)
    }

    /// Calculate coherence
    fn calculate_coherence(&self, entities: &[&Entity]) -> f64 {
        if entities.len() < 2 {
            return 0.0;
        }

        // Calculate variance in consciousness
        let avg_consciousness: f64 =
            entities.iter().map(|e| e.consciousness).sum::<f64>() / entities.len() as f64;
        let variance: f64 = entities
            .iter()
            .map(|e| (e.consciousness - avg_consciousness).powi(2))
            .sum::<f64>()
            / entities.len() as f64;

        // Lower variance = higher coherence
        (1.0 - variance.min(1.0)).max(0.0)
    }

    /// Calculate dominant polarity
    fn calculate_dominant_polarity(&self, entities: &[&Entity]) -> Polarity {
        let mut counts: HashMap<Polarity, usize> = HashMap::new();

        for entity in entities {
            *counts.entry(entity.polarity).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(polarity, _)| polarity)
            .unwrap_or(Polarity::Neutral)
    }

    /// Calculate average density
    fn calculate_average_density(&self, entities: &[&Entity]) -> Density {
        let sum: usize = entities.iter().map(|e| e.density as usize).sum();
        let avg = sum / entities.len();

        match avg {
            1 => Density::First,
            2 => Density::Second,
            3 => Density::Third,
            4 => Density::Fourth,
            5 => Density::Fifth,
            6 => Density::Sixth,
            7 => Density::Seventh,
            _ => Density::Eighth,
        }
    }

    /// Generate resonance field for heatmap
    fn generate_resonance_field(&mut self) {
        let resolution = self.heatmap_resolution;
        let range = 1000.0; // World space range [-500, 500]
        let step = range / resolution as f64;

        for x in 0..resolution {
            for y in 0..resolution {
                let world_x = (x as f64 * step) - (range / 2.0);
                let world_y = (y as f64 * step) - (range / 2.0);
                let position = Vec3::new(world_x as f32, world_y as f32, 0.0_f32);

                let intensity = self.calculate_resonance_at_point(&position);

                if intensity >= self.min_resonance_threshold {
                    // Determine resonance type based on nearest group
                    let resonance_type = self.find_nearest_resonance_type(&position);

                    self.resonance_field.push(ResonanceFieldPoint {
                        position,
                        intensity,
                        resonance_type,
                    });
                }
            }
        }
    }

    /// Calculate resonance at a point
    fn calculate_resonance_at_point(&self, point: &Vec3) -> f64 {
        let mut total_resonance = 0.0;

        for group in &self.groups {
            let distance = (point - group.center).magnitude() as f64;
            let influence = group.resonance_strength * group.coherence;
            let falloff = 1.0 / (1.0 + distance / group.radius());
            total_resonance += influence * falloff;
        }

        total_resonance.min(1.0)
    }

    /// Find nearest resonance type
    fn find_nearest_resonance_type(&self, point: &Vec3) -> ResonanceType {
        self.groups
            .iter()
            .min_by_key(|group| (point - group.center).magnitude() as i64)
            .map(|group| group.resonance_type)
            .unwrap_or(ResonanceType::Density)
    }

    /// Calculate current metrics
    fn calculate_metrics(&mut self) {
        if self.groups.is_empty() {
            self.metrics = CollectiveMetrics::default();
            return;
        }

        let total_entities: usize = self.groups.iter().map(|g| g.size()).sum();
        let avg_size = total_entities as f64 / self.groups.len() as f64;
        let avg_resonance = self
            .groups
            .iter()
            .map(|g| g.resonance_strength)
            .sum::<f64>()
            / self.groups.len() as f64;
        let avg_coherence =
            self.groups.iter().map(|g| g.coherence).sum::<f64>() / self.groups.len() as f64;
        let health_index =
            self.groups.iter().map(|g| g.health).sum::<f64>() / self.groups.len() as f64;

        let mut level_distribution = HashMap::new();
        let mut resonance_distribution = HashMap::new();
        let mut largest_size = 0;

        for group in &self.groups {
            *level_distribution.entry(group.level).or_insert(0) += 1;
            *resonance_distribution
                .entry(group.resonance_type)
                .or_insert(0) += 1;
            largest_size = largest_size.max(group.size());
        }

        let most_common_resonance = resonance_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(resonance_type, _)| *resonance_type);

        self.metrics = CollectiveMetrics {
            total_collectives: self.groups.len(),
            entities_in_collectives: total_entities,
            average_collective_size: avg_size,
            average_resonance: avg_resonance,
            average_coherence: avg_coherence,
            health_index,
            level_distribution,
            resonance_distribution,
            largest_collective_size: largest_size,
            most_common_resonance,
        };
    }

    /// Get group by entity ID
    pub fn get_group_for_entity(&self, entity_id: &EntityId) -> Option<&CollectiveGroup> {
        self.groups.iter().find(|g| g.is_member(entity_id))
    }

    /// Get resonance field data for rendering
    pub fn get_resonance_field_data(&self) -> &Vec<ResonanceFieldPoint> {
        &self.resonance_field
    }

    /// Get behavior pattern for a group
    pub fn detect_behavior_pattern(&self, group: &CollectiveGroup) -> BehaviorPattern {
        if group.size() <= 5 {
            BehaviorPattern::Coordinated
        } else if group.coherence > 0.8 {
            BehaviorPattern::Synchronized
        } else if group.resonance_strength > 0.7 {
            BehaviorPattern::Emergent
        } else if group.size() > 50 {
            BehaviorPattern::Swarm
        } else {
            BehaviorPattern::Distributed
        }
    }
}

impl Default for CollectiveVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_level_colors() {
        assert_eq!(CollectiveLevel::Local.color(), [0.4, 0.8, 0.4, 0.3]);
        assert_eq!(CollectiveLevel::Community.color(), [0.4, 0.6, 0.9, 0.4]);
    }

    #[test]
    fn test_resonance_type_colors() {
        assert_eq!(ResonanceType::Density.color(), [1.0, 0.5, 0.3, 0.5]);
        assert_eq!(ResonanceType::Polarity.color(), [0.3, 0.5, 1.0, 0.5]);
    }

    #[test]
    fn test_collective_group_size() {
        let group = CollectiveGroup {
            group_id: 0,
            level: CollectiveLevel::Local,
            resonance_type: ResonanceType::Density,
            members: vec![
                EntityId::new("1".to_string()),
                EntityId::new("2".to_string()),
                EntityId::new("3".to_string()),
            ],
            center: Vec3::new(0.0, 0.0, 0.0),
            resonance_strength: 0.8,
            coherence: 0.9,
            primary_archetype: None,
            average_density: Density::Third,
            dominant_polarity: Polarity::ServiceToOthers,
            creation_time: 0.0,
            health: 0.72,
        };

        assert_eq!(group.size(), 3);
    }

    #[test]
    fn test_collective_group_is_member() {
        let entity_id1 = EntityId::new("1".to_string());
        let entity_id2 = EntityId::new("4".to_string());

        let group = CollectiveGroup {
            group_id: 0,
            level: CollectiveLevel::Local,
            resonance_type: ResonanceType::Density,
            members: vec![
                EntityId::new("1".to_string()),
                EntityId::new("2".to_string()),
                EntityId::new("3".to_string()),
            ],
            center: Vec3::new(0.0, 0.0, 0.0),
            resonance_strength: 0.8,
            coherence: 0.9,
            primary_archetype: None,
            average_density: Density::Third,
            dominant_polarity: Polarity::ServiceToOthers,
            creation_time: 0.0,
            health: 0.72,
        };

        assert!(group.is_member(&entity_id1));
        assert!(!group.is_member(&entity_id2));
    }

    #[test]
    fn test_collective_metrics_default() {
        let metrics = CollectiveMetrics::default();
        assert_eq!(metrics.total_collectives, 0);
        assert_eq!(metrics.entities_in_collectives, 0);
        assert_eq!(metrics.average_collective_size, 0.0);
    }

    #[test]
    fn test_collective_visualizer_new() {
        let visualizer = CollectiveVisualizer::new();
        assert!(visualizer.groups.is_empty());
        assert!(visualizer.resonance_field.is_empty());
        assert_eq!(visualizer.next_group_id, 0);
        assert!(visualizer.show_visualization);
    }

    #[test]
    fn test_behavior_pattern_colors() {
        assert_eq!(BehaviorPattern::Coordinated.color(), [0.3, 0.9, 0.5, 0.6]);
        assert_eq!(BehaviorPattern::Synchronized.color(), [0.3, 0.5, 0.9, 0.6]);
    }

    #[test]
    fn test_collective_level_max_entities() {
        assert_eq!(CollectiveLevel::Local.max_entities(), 10);
        assert_eq!(CollectiveLevel::Community.max_entities(), 100);
        assert_eq!(CollectiveLevel::Regional.max_entities(), 1000);
        assert_eq!(CollectiveLevel::Planetary.max_entities(), 10000);
    }

    #[test]
    fn test_detect_behavior_pattern() {
        let visualizer = CollectiveVisualizer::new();

        let small_group = CollectiveGroup {
            group_id: 0,
            level: CollectiveLevel::Local,
            resonance_type: ResonanceType::Density,
            members: vec![
                EntityId::new("1".to_string()),
                EntityId::new("2".to_string()),
            ],
            center: Vec3::new(0.0, 0.0, 0.0),
            resonance_strength: 0.8,
            coherence: 0.9,
            primary_archetype: None,
            average_density: Density::Third,
            dominant_polarity: Polarity::ServiceToOthers,
            creation_time: 0.0,
            health: 0.72,
        };

        let pattern = visualizer.detect_behavior_pattern(&small_group);
        assert_eq!(pattern, BehaviorPattern::Coordinated);
    }

    #[test]
    fn test_get_group_for_entity() {
        let mut visualizer = CollectiveVisualizer::new();

        let entity_id = EntityId::new("1".to_string());
        let group = CollectiveGroup {
            group_id: 0,
            level: CollectiveLevel::Local,
            resonance_type: ResonanceType::Density,
            members: vec![entity_id.clone()],
            center: Vec3::new(0.0, 0.0, 0.0),
            resonance_strength: 0.8,
            coherence: 0.9,
            primary_archetype: None,
            average_density: Density::Third,
            dominant_polarity: Polarity::ServiceToOthers,
            creation_time: 0.0,
            health: 0.72,
        };

        visualizer.groups.push(group);

        let found = visualizer.get_group_for_entity(&entity_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().group_id, 0);
    }
}
