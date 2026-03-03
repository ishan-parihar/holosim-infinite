//! HPO Core: Field-First Architecture (Phase 0)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "Replace entity-centric state with field-centric state"
//!
//! This module implements the foundational data structures for the field-first
//! simulation architecture. Instead of tracking entities as primary objects,
//! we track field configurations and derive entity positions through inverse
//! holographic projection.
//!
//! Key Concepts:
//! - HolographicFieldState: Primary field representation with complex amplitudes
//! - OctreeNode: Recursive spatial subdivision for efficient field storage
//! - HolographicEncoder: Maps entity properties to field amplitudes/phases
//! - EntityExtractor: Derives entities from field configurations

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Float type for field calculations
pub type Float = f64;

/// Complex number for field representation (amplitude + phase)
#[derive(Debug, Clone, Copy, Default)]
pub struct Complex {
    pub re: Float,
    pub im: Float,
}

impl Complex {
    pub fn new(re: Float, im: Float) -> Self {
        Complex { re, im }
    }

    pub fn magnitude(&self) -> Float {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> Float {
        self.im.atan2(self.re)
    }

    pub fn from_polar(magnitude: Float, phase: Float) -> Self {
        Complex {
            re: magnitude * phase.cos(),
            im: magnitude * phase.sin(),
        }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn scale(&self, factor: Float) -> Complex {
        Complex {
            re: self.re * factor,
            im: self.im * factor,
        }
    }
}

/// Density band representation (8 densities)
/// From COSMOLOGICAL-ARCHITECTURE.md: "Density Octave: 8 densities representing consciousness stages"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensityBand {
    Violet = 0,  // 1st density: Mineral
    Indigo = 1,  // 2nd density: Plant
    Blue = 2,    // 3rd density: Animal/Human
    Green = 3,   // 4th density: Love/Light
    Yellow = 4,  // 5th density: Wisdom
    Orange = 5,  // 6th density: Unity
    Red = 6,     // 7th density: Avatar
    Unknown = 7, // 8th density: Bridge
}

impl DensityBand {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => DensityBand::Violet,
            1 => DensityBand::Indigo,
            2 => DensityBand::Blue,
            3 => DensityBand::Green,
            4 => DensityBand::Yellow,
            5 => DensityBand::Orange,
            6 => DensityBand::Red,
            _ => DensityBand::Unknown,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn count() -> usize {
        8
    }
}

/// Field configuration at a single spatial node
/// Each node stores complex amplitudes for all 8 density bands
#[derive(Debug, Clone, Default)]
pub struct FieldNodeData {
    /// Complex amplitudes for each density band [0-7]
    pub density_amplitudes: [Complex; 8],

    /// Field coherence at this node (0.0 - 1.0)
    pub coherence: Float,

    /// Spectrum position (0.0 - 1.0, continuous)
    pub spectrum_position: Float,

    /// Veil transparency at this node (0.0 = opaque, 1.0 = transparent)
    pub veil_transparency: Float,

    /// Energy level at this node
    pub energy: Float,
}

impl FieldNodeData {
    pub fn new() -> Self {
        FieldNodeData {
            density_amplitudes: [Complex::default(); 8],
            coherence: 0.0,
            spectrum_position: 0.0,
            veil_transparency: 0.0,
            energy: 0.0,
        }
    }

    /// Get total field magnitude across all densities
    pub fn total_magnitude(&self) -> Float {
        self.density_amplitudes.iter().map(|c| c.magnitude()).sum()
    }
}

/// Spatial bounds for a field node
#[derive(Debug, Clone, Copy, Default)]
pub struct FieldBounds {
    pub min: [Float; 3],
    pub max: [Float; 3],
}

impl FieldBounds {
    pub fn new(min: [Float; 3], max: [Float; 3]) -> Self {
        FieldBounds { min, max }
    }

    pub fn center(&self) -> [Float; 3] {
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
            (self.min[2] + self.max[2]) / 2.0,
        ]
    }

    pub fn size(&self) -> [Float; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    pub fn contains(&self, point: &[Float; 3]) -> bool {
        point[0] >= self.min[0]
            && point[0] <= self.max[0]
            && point[1] >= self.min[1]
            && point[1] <= self.max[1]
            && point[2] >= self.min[2]
            && point[2] <= self.max[2]
    }

    pub fn volume(&self) -> Float {
        let s = self.size();
        s[0] * s[1] * s[2]
    }
}

/// Octree node for recursive spatial subdivision
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md: "Recursive subdivision with holographic encoding"
#[derive(Debug, Clone)]
pub struct OctreeNode {
    /// Spatial bounds of this node
    pub bounds: FieldBounds,

    /// Depth in the octree (0 = root)
    pub depth: usize,

    /// Field data at this node
    pub field_data: FieldNodeData,

    /// Child nodes (8 children for 3D)
    pub children: Option<Box<[OctreeNode; 8]>>,

    /// Whether this node is subdivided
    pub subdivided: bool,

    /// Total field energy in this subtree
    pub subtree_energy: Float,

    /// Maximum amplitude in this subtree
    pub max_amplitude: Float,
}

impl OctreeNode {
    pub fn new(bounds: FieldBounds, depth: usize) -> Self {
        OctreeNode {
            bounds,
            depth,
            field_data: FieldNodeData::new(),
            children: None,
            subdivided: false,
            subtree_energy: 0.0,
            max_amplitude: 0.0,
        }
    }

    /// Check if this node should be subdivided based on field activity
    pub fn should_subdivide(&self, max_depth: usize, energy_threshold: Float) -> bool {
        !self.subdivided && self.depth < max_depth && self.field_data.energy > energy_threshold
    }

    /// Subdivide this node into 8 children
    pub fn subdivide(&mut self) {
        if self.subdivided {
            return;
        }

        let center = self.bounds.center();
        let size = self.bounds.size();
        let half_size = [size[0] / 2.0, size[1] / 2.0, size[2] / 2.0];

        let mut children = Box::new([
            // Bottom layer (y = min)
            OctreeNode::new(
                FieldBounds::new(
                    [self.bounds.min[0], self.bounds.min[1], self.bounds.min[2]],
                    [center[0], center[1], center[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [center[0], self.bounds.min[1], self.bounds.min[2]],
                    [self.bounds.max[0], center[1], center[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [self.bounds.min[0], self.bounds.min[1], center[2]],
                    [center[0], center[1], self.bounds.max[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [center[0], self.bounds.min[1], center[2]],
                    [self.bounds.max[0], center[1], self.bounds.max[2]],
                ),
                self.depth + 1,
            ),
            // Top layer (y = max)
            OctreeNode::new(
                FieldBounds::new(
                    [self.bounds.min[0], center[1], self.bounds.min[2]],
                    [center[0], self.bounds.max[1], center[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [center[0], center[1], self.bounds.min[2]],
                    [self.bounds.max[0], self.bounds.max[1], center[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [self.bounds.min[0], center[1], center[2]],
                    [center[0], self.bounds.max[1], self.bounds.max[2]],
                ),
                self.depth + 1,
            ),
            OctreeNode::new(
                FieldBounds::new(
                    [center[0], center[1], center[2]],
                    [self.bounds.max[0], self.bounds.max[1], self.bounds.max[2]],
                ),
                self.depth + 1,
            ),
        ]);

        // Distribute field energy to children (simplified)
        let child_energy = self.field_data.energy / 8.0;
        for child in children.iter_mut() {
            child.field_data.energy = child_energy;
        }

        self.children = Some(children);
        self.subdivided = true;
    }

    /// Check if this node is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        !self.subdivided
    }

    /// Get the child index for a point
    pub fn child_index(&self, point: &[Float; 3]) -> usize {
        let center = self.bounds.center();
        let mut index = 0;
        if point[0] >= center[0] {
            index |= 1;
        }
        if point[1] >= center[1] {
            index |= 2;
        }
        if point[2] >= center[2] {
            index |= 4;
        }
        index
    }

    /// Update subtree energy statistics
    pub fn update_subtree_stats(&mut self) {
        if let Some(ref children) = self.children {
            self.subtree_energy = children.iter().map(|c| c.subtree_energy).sum();
            self.max_amplitude = children
                .iter()
                .map(|c| c.max_amplitude)
                .fold(0.0, Float::max);
        } else {
            self.subtree_energy = self.field_data.energy;
            self.max_amplitude = self.field_data.total_magnitude();
        }
    }
}

/// Configuration for the holographic field simulation
#[derive(Debug, Clone)]
pub struct HolographicFieldConfig {
    /// Maximum octree depth
    pub max_depth: usize,

    /// Energy threshold for subdivision
    pub subdivision_threshold: Float,

    /// Number of density bands (typically 8)
    pub density_band_count: usize,

    /// Initial field coherence
    pub initial_coherence: Float,

    /// Veil position (v = 1.0 is the crossing point)
    pub veil_position: Float,

    /// Field evolution time step
    pub time_step: Float,

    /// Wave propagation speed (speed of "light")
    pub propagation_speed: Float,

    /// Spatial bounds of the simulation
    pub bounds: FieldBounds,
}

impl Default for HolographicFieldConfig {
    fn default() -> Self {
        HolographicFieldConfig {
            max_depth: 8,
            subdivision_threshold: 0.01,
            density_band_count: 8,
            initial_coherence: 0.5,
            veil_position: 1.0,
            time_step: 0.1,
            propagation_speed: 1.0,
            bounds: FieldBounds::new([-1000.0, -1000.0, -1000.0], [1000.0, 1000.0, 1000.0]),
        }
    }
}

/// Holographic Field State - Primary simulation state
///
/// From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
/// "FieldState stores complex amplitudes, not entity positions"
///
/// This is the core data structure for the field-first simulation.
/// Instead of storing entities, we store field configurations that can
/// be used to derive entity positions through inverse projection.
#[derive(Debug, Clone)]
pub struct HolographicFieldState {
    /// Configuration
    pub config: HolographicFieldConfig,

    /// Root of the octree
    pub root: OctreeNode,

    /// Current simulation time
    pub time: Float,

    /// Total field energy
    pub total_energy: Float,

    /// Average coherence across the field
    pub average_coherence: Float,

    /// Number of active nodes
    pub active_node_count: usize,

    /// Number of leaf nodes
    pub leaf_node_count: usize,

    /// Statistics
    pub statistics: FieldStatistics,
}

/// Statistics for the field state
#[derive(Debug, Clone, Default)]
pub struct FieldStatistics {
    /// Peak field magnitude
    pub peak_magnitude: Float,

    /// Minimum field magnitude
    pub min_magnitude: Float,

    /// Average field magnitude
    pub avg_magnitude: Float,

    /// Total nodes visited in last update
    pub nodes_visited: usize,

    /// Subdivisions performed
    pub subdivisions: usize,

    /// Merge operations performed
    pub merges: usize,
}

impl HolographicFieldState {
    /// Create a new holographic field state
    pub fn new(config: HolographicFieldConfig) -> Self {
        HolographicFieldState {
            config: config.clone(),
            root: OctreeNode::new(config.bounds, 0),
            time: 0.0,
            total_energy: 0.0,
            average_coherence: config.initial_coherence,
            active_node_count: 1,
            leaf_node_count: 1,
            statistics: FieldStatistics::default(),
        }
    }

    /// Initialize with default configuration
    pub fn with_defaults() -> Self {
        Self::new(HolographicFieldConfig::default())
    }

    /// Get or create node at a point
    pub fn get_or_create_node(&mut self, point: [Float; 3]) -> &mut OctreeNode {
        if !self.root.bounds.contains(&point) {
            // Expand bounds if necessary (simplified)
            return &mut self.root;
        }

        let mut current = &mut self.root;

        while current.subdivided {
            let index = current.child_index(&point);
            if let Some(ref mut children) = current.children {
                current = &mut children[index];
            } else {
                break;
            }
        }

        current
    }

    /// Add energy at a point (creates entity signature)
    pub fn add_energy_at(&mut self, point: [Float; 3], density: usize, energy: Float) {
        // Extract config values first to avoid borrow issues
        let max_depth = self.config.max_depth;
        let threshold = self.config.subdivision_threshold;

        let node = self.get_or_create_node(point);
        node.field_data.energy += energy;

        if density < 8 {
            let amplitude = Complex::from_polar(energy.sqrt(), point[0] * 0.01 + point[1] * 0.01);
            node.field_data.density_amplitudes[density] = amplitude;
        }

        // Check if we should subdivide
        if node.should_subdivide(max_depth, threshold) {
            node.subdivide();
            self.active_node_count += 8;
            self.leaf_node_count += 7;
            self.statistics.subdivisions += 1;
        }
    }

    /// Update field statistics
    pub fn update_statistics(&mut self) {
        self.root.update_subtree_stats();
        self.total_energy = self.root.subtree_energy;
        self.statistics.peak_magnitude = self.root.max_amplitude;
    }

    /// Advance simulation by one time step
    pub fn step(&mut self) {
        self.time += self.config.time_step;
        // Field evolution will be added in Phase 1-2
        self.update_statistics();
    }
}

/// Extracted entity from field configuration
#[derive(Debug, Clone)]
pub struct ExtractedEntity {
    /// Position in space
    pub position: [Float; 3],

    /// Consciousness level (derived from coherence)
    pub consciousness: Float,

    /// Density band
    pub density: DensityBand,

    /// Spectrum position
    pub spectrum_position: Float,

    /// Energy/importance
    pub energy: Float,

    /// Phase (for resonance calculations)
    pub phase: Float,
}

impl ExtractedEntity {
    pub fn new(position: [Float; 3]) -> Self {
        ExtractedEntity {
            position,
            consciousness: 0.0,
            density: DensityBand::Unknown,
            spectrum_position: 0.0,
            energy: 0.0,
            phase: 0.0,
        }
    }
}

/// Entity extraction result
#[derive(Debug, Clone)]
pub struct EntityExtractionResult {
    /// Extracted entities
    pub entities: Vec<ExtractedEntity>,

    /// Extraction time
    pub extraction_time_ms: Float,

    /// Entities found
    pub entity_count: usize,
}

impl Default for EntityExtractionResult {
    fn default() -> Self {
        EntityExtractionResult {
            entities: Vec::new(),
            extraction_time_ms: 0.0,
            entity_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_operations() {
        let a = Complex::new(3.0, 4.0);
        assert!((a.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_octree_creation() {
        let bounds = FieldBounds::new([-10.0, -10.0, -10.0], [10.0, 10.0, 10.0]);
        let node = OctreeNode::new(bounds, 0);
        assert!(node.is_leaf());
    }

    #[test]
    fn test_field_state_creation() {
        let state = HolographicFieldState::with_defaults();
        assert_eq!(state.active_node_count, 1);
    }

    #[test]
    fn test_energy_addition() {
        let mut state = HolographicFieldState::with_defaults();
        state.add_energy_at([0.0, 0.0, 0.0], 3, 1.0);
        assert!(state.root.field_data.energy > 0.0);
    }
}
