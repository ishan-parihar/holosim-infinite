//! Spatial Field Foundation (Phase F1)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Connect the holographic field to actual 3D spatial coordinates, not abstract mathematical distributions."
//!
//! This module implements the spatial foundation for the holographic universe simulation:
//! - SpatialFieldNode: Field nodes with ACTUAL x, y, z coordinates
//! - SpatialOctree: Adaptive spatial subdivision based on field activity
//! - Entity-Spatial Bridge: Entity positions derived from field node positions
//!
//! KEY PRINCIPLE: The field EXISTS at spatial positions. Each SpatialFieldNode
//! represents a region of ACTUAL space, not an abstract mathematical construct.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::field_state::{Complex, FieldBounds, FieldNodeData, Float, HolographicFieldConfig};

/// Spatial position in 3D space
#[derive(Debug, Clone, Copy, Default)]
pub struct Position3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Position3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Position3D { x, y, z }
    }

    pub fn origin() -> Self {
        Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_array(arr: [Float; 3]) -> Self {
        Position3D {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    pub fn to_array(&self) -> [Float; 3] {
        [self.x, self.y, self.z]
    }

    pub fn distance_to(&self, other: &Position3D) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn lerp(&self, other: &Position3D, t: Float) -> Position3D {
        Position3D {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    pub fn scale(&self, factor: Float) -> Position3D {
        Position3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn add(&self, other: &Position3D) -> Position3D {
        Position3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Calculate magnitude/length of position vector
    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize to unit vector (R&D-7 helper)
    pub fn normalize(&self) -> Position3D {
        let mag = self.magnitude();
        if mag < 0.001 {
            Position3D::new(0.0, 0.0, 0.0)
        } else {
            Position3D::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    /// Normalize to unit vector
    pub fn normalized(&self) -> Position3D {
        let mag = self.magnitude();
        if mag > 0.0001 {
            self.scale(1.0 / mag)
        } else {
            *self
        }
    }
}

/// Transformed position with dimension information
/// Used for positions above the veil where time becomes navigable
#[derive(Debug, Clone, Copy)]
pub struct TransformedPosition {
    pub position: Position3D,
    pub dimensions: usize,
    pub time_extent: Float,
}

impl TransformedPosition {
    pub fn new(position: Position3D, dimensions: usize) -> Self {
        TransformedPosition {
            position,
            dimensions,
            time_extent: 0.0,
        }
    }

    pub fn with_time_extent(position: Position3D, dimensions: usize, time_extent: Float) -> Self {
        TransformedPosition {
            position,
            dimensions,
            time_extent,
        }
    }
}

/// Spatial bounds with center and extent
#[derive(Debug, Clone, Copy)]
pub struct SpatialBounds {
    pub center: Position3D,
    pub extent: Position3D, // Half-size in each dimension
}

impl SpatialBounds {
    pub fn from_min_max(min: [Float; 3], max: [Float; 3]) -> Self {
        let center = Position3D::new(
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        );
        let extent = Position3D::new(
            (max[0] - min[0]) / 2.0,
            (max[1] - min[1]) / 2.0,
            (max[2] - min[2]) / 2.0,
        );
        SpatialBounds { center, extent }
    }

    pub fn from_field_bounds(bounds: &FieldBounds) -> Self {
        Self::from_min_max(bounds.min, bounds.max)
    }

    pub fn contains(&self, pos: &Position3D) -> bool {
        (pos.x - self.center.x).abs() <= self.extent.x
            && (pos.y - self.center.y).abs() <= self.extent.y
            && (pos.z - self.center.z).abs() <= self.extent.z
    }

    pub fn min(&self) -> Position3D {
        Position3D::new(
            self.center.x - self.extent.x,
            self.center.y - self.extent.y,
            self.center.z - self.extent.z,
        )
    }

    pub fn max(&self) -> Position3D {
        Position3D::new(
            self.center.x + self.extent.x,
            self.center.y + self.extent.y,
            self.center.z + self.extent.z,
        )
    }

    pub fn volume(&self) -> Float {
        8.0 * self.extent.x * self.extent.y * self.extent.z
    }

    pub fn child_bounds(&self, child_index: usize) -> SpatialBounds {
        let quarter_x = self.extent.x / 2.0;
        let quarter_y = self.extent.y / 2.0;
        let quarter_z = self.extent.z / 2.0;

        let (new_center_x, new_extent_x) = if child_index & 1 != 0 {
            (self.center.x + quarter_x, quarter_x)
        } else {
            (self.center.x - quarter_x, quarter_x)
        };

        let (new_center_y, new_extent_y) = if child_index & 2 != 0 {
            (self.center.y + quarter_y, quarter_y)
        } else {
            (self.center.y - quarter_y, quarter_y)
        };

        let (new_center_z, new_extent_z) = if child_index & 4 != 0 {
            (self.center.z + quarter_z, quarter_z)
        } else {
            (self.center.z - quarter_z, quarter_z)
        };

        SpatialBounds {
            center: Position3D::new(new_center_x, new_center_y, new_center_z),
            extent: Position3D::new(new_extent_x, new_extent_y, new_extent_z),
        }
    }
}

/// Spatial resolution level (LOD - Level of Detail)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionLevel {
    Cosmic,    // Level 0: Galaxy-scale (extent: 10000+)
    Galactic,  // Level 1: Star-cluster scale (extent: 1000-10000)
    Stellar,   // Level 2: Solar system scale (extent: 100-1000)
    Planetary, // Level 3: Planet scale (extent: 10-100)
    Regional,  // Level 4: Continent scale (extent: 1-10)
    Local,     // Level 5: Human scale (extent: 0.1-1)
    Molecular, // Level 6: Molecular scale (extent: 0.01-0.1)
    Atomic,    // Level 7: Atomic scale (extent: 0.001-0.01)
}

impl ResolutionLevel {
    pub fn from_depth(depth: usize) -> Self {
        match depth {
            0 => ResolutionLevel::Cosmic,
            1 => ResolutionLevel::Galactic,
            2 => ResolutionLevel::Stellar,
            3 => ResolutionLevel::Planetary,
            4 => ResolutionLevel::Regional,
            5 => ResolutionLevel::Local,
            6 => ResolutionLevel::Molecular,
            _ => ResolutionLevel::Atomic,
        }
    }

    pub fn extent(&self) -> Float {
        match self {
            ResolutionLevel::Cosmic => 20000.0,
            ResolutionLevel::Galactic => 5000.0,
            ResolutionLevel::Stellar => 500.0,
            ResolutionLevel::Planetary => 50.0,
            ResolutionLevel::Regional => 5.0,
            ResolutionLevel::Local => 0.5,
            ResolutionLevel::Molecular => 0.05,
            ResolutionLevel::Atomic => 0.005,
        }
    }
}

/// Field activity level for adaptive resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldActivity {
    Empty,    // No field energy
    Low,      // Minimal activity
    Medium,   // Normal activity
    High,     // High activity - subdivide
    Critical, // Maximum activity - maximum detail
}

impl FieldActivity {
    pub fn from_energy(energy: Float, threshold_high: Float, threshold_critical: Float) -> Self {
        if energy <= 0.0 {
            FieldActivity::Empty
        } else if energy < threshold_high * 0.3 {
            FieldActivity::Low
        } else if energy < threshold_high {
            FieldActivity::Medium
        } else if energy < threshold_critical {
            FieldActivity::High
        } else {
            FieldActivity::Critical
        }
    }

    pub fn target_depth(&self, base_depth: usize, max_depth: usize) -> usize {
        match self {
            FieldActivity::Empty => base_depth.saturating_sub(2),
            FieldActivity::Low => base_depth.saturating_sub(1),
            FieldActivity::Medium => base_depth,
            FieldActivity::High => (base_depth + 1).min(max_depth),
            FieldActivity::Critical => max_depth,
        }
    }
}

/// Spatial field node with ACTUAL coordinates
///
/// This is the key innovation: instead of abstract mathematical positions,
/// each node has real x, y, z coordinates that represent actual space.
#[derive(Debug, Clone)]
pub struct SpatialFieldNode {
    /// ACTUAL 3D position in space
    pub position: Position3D,

    /// Spatial bounds of this node
    pub bounds: SpatialBounds,

    /// Depth in the octree
    pub depth: usize,

    /// Field data at this node
    pub field_data: FieldNodeData,

    /// Child nodes (8 children for 3D)
    pub children: Option<Box<[SpatialFieldNode; 8]>>,

    /// Whether this node is subdivided
    pub subdivided: bool,

    /// Total field energy in this subtree
    pub subtree_energy: Float,

    /// Maximum amplitude in this subtree
    pub max_amplitude: Float,

    /// Entity IDs associated with this node
    pub entities: Vec<usize>,

    /// Resolution level
    pub resolution: ResolutionLevel,
}

impl SpatialFieldNode {
    pub fn new(position: Position3D, bounds: SpatialBounds, depth: usize) -> Self {
        SpatialFieldNode {
            position,
            bounds,
            depth,
            field_data: FieldNodeData::new(),
            children: None,
            subdivided: false,
            subtree_energy: 0.0,
            max_amplitude: 0.0,
            entities: Vec::new(),
            resolution: ResolutionLevel::from_depth(depth),
        }
    }

    /// Create root node from simulation bounds
    pub fn from_simulation_bounds(min: [Float; 3], max: [Float; 3], depth: usize) -> Self {
        let bounds = SpatialBounds::from_min_max(min, max);
        let position = bounds.center;
        Self::new(position, bounds, depth)
    }

    /// Check if this node should be subdivided
    pub fn should_subdivide(
        &self,
        max_depth: usize,
        energy_threshold: Float,
        activity_threshold: Float,
    ) -> bool {
        if self.subdivided || self.depth >= max_depth {
            return false;
        }

        let activity = FieldActivity::from_energy(
            self.field_data.energy,
            energy_threshold,
            activity_threshold,
        );

        activity.target_depth(self.depth, max_depth) > self.depth
    }

    /// Subdivide this node into 8 children
    pub fn subdivide(&mut self) {
        if self.subdivided {
            return;
        }

        let mut children: Box<[SpatialFieldNode; 8]> = Box::new([
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
            SpatialFieldNode::new(
                Position3D::origin(),
                SpatialBounds::from_min_max([0.0; 3], [0.0; 3]),
                self.depth + 1,
            ),
        ]);

        for i in 0..8 {
            let child_bounds = self.bounds.child_bounds(i);
            children[i] = SpatialFieldNode::new(child_bounds.center, child_bounds, self.depth + 1);
            // Distribute field energy to children
            children[i].field_data.energy = self.field_data.energy / 8.0;
        }

        // Move entities to appropriate children (collect indices first to avoid borrow issue)
        let entity_ids: Vec<usize> = self.entities.drain(..).collect();
        for entity_id in entity_ids {
            let index = self.child_index_for_entity(entity_id);
            children[index].entities.push(entity_id);
        }

        self.children = Some(children);
        self.subdivided = true;
    }

    /// Get child index for an entity based on its position
    fn child_index_for_entity(&self, entity_id: usize) -> usize {
        // Use entity ID as a seed to derive a deterministic position
        let seed = entity_id as Float;
        let x = (seed * 17.0).sin() * self.bounds.extent.x * 0.9;
        let y = (seed * 23.0).cos() * self.bounds.extent.y * 0.9;
        let z = (seed * 31.0).sin() * self.bounds.extent.z * 0.9;

        let local_pos = Position3D::new(
            self.position.x + x,
            self.position.y + y,
            self.position.z + z,
        );

        self.child_index(&local_pos)
    }

    /// Get child index for a position
    pub fn child_index(&self, pos: &Position3D) -> usize {
        let mut index = 0;
        if pos.x >= self.position.x {
            index |= 1;
        }
        if pos.y >= self.position.y {
            index |= 2;
        }
        if pos.z >= self.position.z {
            index |= 4;
        }
        index
    }

    /// Check if this node is a leaf
    pub fn is_leaf(&self) -> bool {
        !self.subdivided
    }

    /// Update subtree statistics
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

    /// Get all leaf nodes (spatial resolution)
    pub fn get_leaf_nodes(&self) -> Vec<&SpatialFieldNode> {
        if self.is_leaf() {
            vec![self]
        } else if let Some(ref children) = self.children {
            children.iter().flat_map(|c| c.get_leaf_nodes()).collect()
        } else {
            vec![self]
        }
    }

    /// Find the leaf node containing a position
    pub fn find_leaf(&self, pos: &Position3D) -> Option<&SpatialFieldNode> {
        if !self.bounds.contains(pos) {
            return None;
        }

        if self.is_leaf() {
            return Some(self);
        }

        if let Some(ref children) = self.children {
            let index = self.child_index(pos);
            children[index].find_leaf(pos)
        } else {
            Some(self)
        }
    }

    /// Find the leaf node containing a position (mutable)
    pub fn find_leaf_mut(&mut self, pos: &Position3D) -> Option<&mut SpatialFieldNode> {
        if !self.bounds.contains(pos) {
            return None;
        }

        if self.is_leaf() {
            return Some(self);
        }

        // Compute index before borrowing children mutably
        let index = {
            let mut index_val = 0;
            if pos.x >= self.position.x {
                index_val |= 1;
            }
            if pos.y >= self.position.y {
                index_val |= 2;
            }
            if pos.z >= self.position.z {
                index_val |= 4;
            }
            index_val
        };

        if let Some(ref mut children) = self.children {
            children[index].find_leaf_mut(pos)
        } else {
            Some(self)
        }
    }
}

/// Configuration for spatial field
#[derive(Debug, Clone)]
pub struct SpatialFieldConfig {
    /// Simulation space bounds
    pub bounds: SpatialBounds,

    /// Maximum octree depth
    pub max_depth: usize,

    /// Energy threshold for subdivision
    pub subdivision_threshold: Float,

    /// Critical energy threshold (maximum detail)
    pub critical_threshold: Float,

    /// Minimum energy to keep a node
    pub merge_threshold: Float,

    /// Initial field coherence
    pub initial_coherence: Float,

    /// Spectrum-driven position calculation enabled
    pub use_spectrum_position: bool,

    /// Time scale for spectrum position conversion
    pub time_scale: Float,

    /// Veil position (spectrum = 1.0)
    pub veil_position: Float,

    /// Veil transition width
    pub veil_width: Float,

    /// Coherence gradient influence on position
    pub coherence_gradient_strength: Float,

    /// Golden ratio fallback (for initial distribution)
    pub use_golden_ratio_fallback: bool,
}

impl Default for SpatialFieldConfig {
    fn default() -> Self {
        SpatialFieldConfig {
            bounds: SpatialBounds::from_min_max(
                [-10000.0, -10000.0, -10000.0],
                [10000.0, 10000.0, 10000.0],
            ),
            max_depth: 8,
            subdivision_threshold: 0.1,
            critical_threshold: 1.0,
            merge_threshold: 0.001,
            initial_coherence: 0.5,
            use_spectrum_position: true,
            time_scale: 100.0,
            veil_position: 1.0,
            veil_width: 0.1,
            coherence_gradient_strength: 1.0,
            use_golden_ratio_fallback: false,
        }
    }
}

impl From<&HolographicFieldConfig> for SpatialFieldConfig {
    fn from(config: &HolographicFieldConfig) -> Self {
        SpatialFieldConfig {
            bounds: SpatialBounds::from_field_bounds(&config.bounds),
            max_depth: config.max_depth,
            subdivision_threshold: config.subdivision_threshold,
            critical_threshold: 1.0,
            merge_threshold: config.subdivision_threshold * 0.1,
            initial_coherence: config.initial_coherence,
            use_spectrum_position: true,
            time_scale: 100.0,
            veil_position: 1.0,
            veil_width: 0.1,
            coherence_gradient_strength: 1.0,
            use_golden_ratio_fallback: false,
        }
    }
}

/// Spatial Field - The foundation of actual space
///
/// This is the core of Phase F1: replacing abstract positions with actual
/// spatial coordinates. The field EXISTS at spatial positions.
#[derive(Debug, Clone)]
pub struct SpatialField {
    pub config: SpatialFieldConfig,
    pub root: SpatialFieldNode,
    pub time: Float,
    pub total_energy: Float,
    pub active_node_count: usize,
    pub leaf_node_count: usize,
    /// Map from entity ID to position
    pub entity_positions: HashMap<usize, Position3D>,
}

impl SpatialField {
    pub fn new(config: SpatialFieldConfig) -> Self {
        let root = SpatialFieldNode::from_simulation_bounds(
            config.bounds.min().to_array(),
            config.bounds.max().to_array(),
            0,
        );

        SpatialField {
            config,
            root,
            time: 0.0,
            total_energy: 0.0,
            active_node_count: 1,
            leaf_node_count: 1,
            entity_positions: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(SpatialFieldConfig::default())
    }

    /// Create from existing HolographicFieldConfig
    pub fn from_holographic_config(config: &HolographicFieldConfig) -> Self {
        Self::new(SpatialFieldConfig::from(config))
    }

    /// Add an entity at a position
    pub fn add_entity(&mut self, entity_id: usize, position: Position3D) {
        // Find the leaf node for this position
        if let Some(node) = self.root.find_leaf_mut(&position) {
            node.entities.push(entity_id);
            node.field_data.energy += 0.1; // Entity contributes to field
        }

        // Store the position
        self.entity_positions.insert(entity_id, position);
    }

    /// Get entity position
    pub fn get_entity_position(&self, entity_id: usize) -> Option<Position3D> {
        self.entity_positions.get(&entity_id).copied()
    }

    /// Add energy at a position
    pub fn add_energy_at(&mut self, position: Position3D, density: usize, energy: Float) {
        if let Some(node) = self.root.find_leaf_mut(&position) {
            node.field_data.energy += energy;

            if density < 8 {
                let amplitude =
                    Complex::from_polar(energy.sqrt(), position.x * 0.01 + position.y * 0.01);
                node.field_data.density_amplitudes[density] = amplitude;
            }

            // Check if we should subdivide
            if node.should_subdivide(
                self.config.max_depth,
                self.config.subdivision_threshold,
                self.config.critical_threshold,
            ) {
                node.subdivide();
                self.active_node_count += 8;
                self.leaf_node_count += 7;
            }
        }
    }

    /// Update field and manage resolution
    pub fn update(&mut self) {
        self.root.update_subtree_stats();
        self.total_energy = self.root.subtree_energy;

        // Count leaf nodes
        self.leaf_node_count = self.root.get_leaf_nodes().len();
    }

    /// Get all spatial positions with field activity above threshold
    pub fn get_active_positions(&self, min_energy: Float) -> Vec<(Position3D, Float)> {
        let mut positions = Vec::new();
        self.collect_active_positions(&self.root, min_energy, &mut positions);
        positions
    }

    fn collect_active_positions(
        &self,
        node: &SpatialFieldNode,
        min_energy: Float,
        positions: &mut Vec<(Position3D, Float)>,
    ) {
        if node.field_data.energy >= min_energy {
            positions.push((node.position, node.field_data.energy));
        }

        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.collect_active_positions(child, min_energy, positions);
            }
        }
    }

    /// Get field value at a position (interpolated)
    pub fn sample_field(&self, position: &Position3D) -> FieldNodeData {
        if let Some(node) = self.root.find_leaf(position) {
            node.field_data.clone()
        } else {
            FieldNodeData::new()
        }
    }

    /// Advance simulation
    pub fn step(&mut self, dt: Float) {
        self.time += dt;
        self.update();
    }

    /// Convert entity ID to spatial position using SPECTRUM-DRIVEN coordinates
    ///
    /// From R&D-2 Roadmap: This replaces golden ratio positions with spectrum-derived
    /// spatial coordinates based on field coherence gradients.
    ///
    /// Key principle:
    /// - v = s/t (spectrum < 1.0): 3D space dominant, 1D time
    /// - v = t/s (spectrum > 1.0): 1D space fixed, 3D time accessible
    /// - The Veil at v=1 is a coordinate transformation zone
    pub fn derive_entity_position(&self, entity_id: usize, total_entities: usize) -> Position3D {
        // If we already have a position, return it
        if let Some(pos) = self.entity_positions.get(&entity_id) {
            return *pos;
        }

        // Use spectrum-driven position if enabled
        if self.config.use_spectrum_position {
            // Derive spectrum position from entity ID
            let spectrum_pos = self.derive_spectrum_from_entity(entity_id, total_entities);

            // Get field data at this position for coherence gradients
            let field_data = self.get_field_data_at_entity(entity_id, total_entities);

            // Calculate position based on spectrum location
            return self.derive_position_from_spectrum(spectrum_pos, &field_data);
        }

        // Fallback to golden ratio if configured
        if self.config.use_golden_ratio_fallback {
            let normalized_id = entity_id as Float / total_entities as Float;
            let golden_ratio = 1.618033988749895;
            let theta = normalized_id * 2.0 * std::f64::consts::PI * golden_ratio;
            let phi = (1.0 - 2.0 * normalized_id).acos();
            let radius = self.config.bounds.extent.x * 0.8;

            return Position3D::new(
                radius * phi.sin() * theta.cos(),
                radius * phi.sin() * theta.sin(),
                radius * phi.cos(),
            );
        }

        // Default: small random offset from origin
        let seed = entity_id as Float * 12.345678;
        Position3D::new(
            seed.sin() * 10.0,
            (seed * 2.0).cos() * 10.0,
            (seed * 3.0).sin() * 10.0,
        )
    }

    /// Derive spectrum position from entity ID
    /// Maps entity to a position in the creation spectrum
    fn derive_spectrum_from_entity(&self, entity_id: usize, total_entities: usize) -> Float {
        // Entities are distributed across the spectrum based on their ID
        // This creates a meaningful progression through the density octave
        let normalized = entity_id as Float / total_entities as Float;

        // Map to spectrum range (0.0 - 2.0 to cover below and above veil)
        // Most entities start below the veil (spectrum < 1.0)
        normalized * 2.0
    }

    /// Get field data for an entity based on its ID
    /// This provides the coherence information needed for position derivation
    fn get_field_data_at_entity(&self, entity_id: usize, total_entities: usize) -> FieldNodeData {
        // Create field data based on entity's spectrum position
        let spectrum = self.derive_spectrum_from_entity(entity_id, total_entities);

        // Higher spectrum position = higher coherence
        let coherence = (spectrum / 2.0).min(1.0);

        // Energy based on position in spectrum
        let energy = 0.5 + coherence * 0.5;

        let mut data = FieldNodeData::new();
        data.spectrum_position = spectrum;
        data.coherence = coherence;
        data.energy = energy;

        // Set density amplitudes based on spectrum position
        let density_idx = (spectrum * 4.0) as usize; // Map to 0-7
        for i in 0..8 {
            let dist = (i as Float - density_idx as Float).abs();
            let amplitude = if dist < 1.0 { 1.0 - dist } else { 0.0 };
            data.density_amplitudes[i] = Complex::new(amplitude * 0.5, 0.0);
        }

        data
    }

    /// Derive position from spectrum using coherence gradients
    ///
    /// From R&D-2 Roadmap:
    /// - Below veil (spectrum < 1.0): Position emerges from field coherence gradients
    /// - Above veil (spectrum > 1.0): Position fixed in space, time becomes navigable
    fn derive_position_from_spectrum(
        &self,
        spectrum: Float,
        field_data: &FieldNodeData,
    ) -> Position3D {
        if spectrum < self.config.veil_position {
            // Below veil: 3D space dominant
            // Position emerges from field coherence gradients
            let coherence_gradient = self.compute_coherence_gradient_for_spectrum(spectrum);
            let base_position = self.derive_from_gradient(coherence_gradient, spectrum);

            // Add time dimension (linear, below veil)
            let time_offset = field_data.spectrum_position * self.config.time_scale;

            Position3D::new(
                base_position.x,
                base_position.y,
                base_position.z + time_offset,
            )
        } else {
            // Above veil: Time dominant
            // Position is fixed in space, but has extent in time dimensions
            let coherence_gradient = self.compute_coherence_gradient_for_spectrum(spectrum);
            let spatial = self.derive_from_gradient(coherence_gradient, spectrum);

            // Time becomes navigable dimensions
            let time_extent = field_data.total_magnitude() * self.config.time_scale;
            let time_direction = coherence_gradient.magnitude() * self.config.time_scale * 0.5;

            Position3D::new(
                spatial.x,      // Fixed spatial position
                time_extent,    // Time as magnitude
                time_direction, // Time direction
            )
        }
    }

    /// Compute coherence gradient for a given spectrum position
    fn compute_coherence_gradient_for_spectrum(&self, spectrum: Float) -> CoherenceGradient {
        // Gradient points toward higher coherence regions
        // The gradient direction changes based on spectrum position

        let base_angle = spectrum * std::f64::consts::PI * 2.0;

        // Coherence increases with spectrum position (generally)
        let coherence = (spectrum / 2.0).min(1.0);

        // Gradient direction spirals through space as spectrum increases
        let dx = base_angle.sin() * coherence;
        let dy = (base_angle * 1.3).cos() * coherence;
        let dz = (base_angle * 0.7).sin() * coherence;

        CoherenceGradient {
            dx,
            dy,
            dz,
            magnitude: coherence,
        }
    }

    /// Derive position from coherence gradient
    fn derive_from_gradient(&self, gradient: CoherenceGradient, spectrum: Float) -> Position3D {
        let radius = self.config.bounds.extent.x * 0.5;

        // Position is influenced by gradient direction
        // Higher coherence = closer to origin (more "ground" state)
        let coherence_factor = gradient.magnitude;

        // Map gradient to position using spherical coordinates
        let theta = gradient.dx.atan2(gradient.dy) + spectrum * std::f64::consts::PI;
        let phi = gradient.dz.acos();

        // Radius decreases with higher coherence (entities settle into coherence wells)
        let r = radius * (1.0 - coherence_factor * 0.5);

        Position3D::new(
            r * phi.sin() * theta.cos(),
            r * phi.sin() * theta.sin(),
            r * phi.cos(),
        )
    }

    /// Apply Veil coordinate transformation
    ///
    /// From R&D-2 Roadmap: The Veil as coordinate transformation zone
    /// At spectrum = 1.0, transform from space/time to time/space basis
    #[allow(dead_code)]
    fn apply_veil_transformation(
        &self,
        position: Position3D,
        spectrum: Float,
    ) -> TransformedPosition {
        let veil = self.config.veil_position;
        let width = self.config.veil_width;

        if (spectrum - veil).abs() < width {
            // At veil - coordinate transformation
            // Transform from space/time to time/space basis

            // Smooth transition factor (0 to 1)
            let t = ((spectrum - veil).abs() / width).min(1.0);
            let transform_factor = t * t * (3.0 - 2.0 * t); // Smoothstep

            // Transform: swap space and time dimensions
            let new_x = position.x * (1.0 - transform_factor) + position.y * transform_factor;
            let new_y = position.y * (1.0 - transform_factor) + position.x * transform_factor;
            let new_z = position.z;

            let transformed = Position3D::new(new_x, new_y, new_z);

            TransformedPosition::with_time_extent(
                transformed,
                3,                                         // Still 3 dimensions
                transform_factor * self.config.time_scale, // Time extent emerges
            )
        } else {
            TransformedPosition::new(position, 3)
        }
    }

    /// Get coherence peaks from the field (for spatial organization)
    pub fn get_coherence_peaks(&self) -> Vec<Position3D> {
        // In a full implementation, this would query the unified field
        // For now, derive peaks from the spatial structure
        let mut peaks = Vec::new();

        // Generate coherence peaks based on golden ratio spiral
        // These represent regions where matter/structure tends to form
        let golden_ratio = 1.618033988749895;
        let num_peaks = 20;

        for i in 0..num_peaks {
            let t = i as Float / num_peaks as Float;
            let theta = t * 2.0 * std::f64::consts::PI * golden_ratio;
            let phi = (1.0 - 2.0 * t).acos();

            let radius = self.config.bounds.extent.x * 0.6;

            peaks.push(Position3D::new(
                radius * phi.sin() * theta.cos(),
                radius * phi.sin() * theta.sin(),
                radius * phi.cos(),
            ));
        }

        peaks
    }
}

/// Coherence gradient for position derivation
#[derive(Debug, Clone, Copy)]
struct CoherenceGradient {
    dx: Float,
    dy: Float,
    dz: Float,
    magnitude: Float,
}

impl CoherenceGradient {
    fn magnitude(&self) -> Float {
        (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt()
    }
}

/// Bridge between entity IDs and spatial positions
pub struct EntitySpatialBridge {
    field: Arc<RwLock<SpatialField>>,
    pub total_entities: usize,
}

impl EntitySpatialBridge {
    pub fn new(field: Arc<RwLock<SpatialField>>, total_entities: usize) -> Self {
        EntitySpatialBridge {
            field,
            total_entities,
        }
    }

    /// Get position for an entity
    pub fn get_position(&self, entity_id: usize) -> Position3D {
        let field = self.field.read().unwrap();

        // First check if entity has explicit position
        if let Some(pos) = field.get_entity_position(entity_id) {
            return pos;
        }

        // Otherwise derive from field structure
        field.derive_entity_position(entity_id, self.total_entities)
    }

    /// Get all entity positions
    pub fn get_all_positions(&self) -> Vec<(usize, Position3D)> {
        let _field = self.field.read().unwrap();
        let mut positions = Vec::with_capacity(self.total_entities);

        for entity_id in 0..self.total_entities {
            positions.push((entity_id, self.get_position(entity_id)));
        }

        positions
    }
}

/// Statistics for spatial field
#[derive(Debug, Clone, Default)]
pub struct SpatialFieldStatistics {
    pub total_nodes: usize,
    pub leaf_nodes: usize,
    pub active_nodes: usize,
    pub total_energy: Float,
    pub average_energy: Float,
    pub max_depth_reached: usize,
    pub subdivision_count: usize,
}

impl SpatialField {
    pub fn get_statistics(&self) -> SpatialFieldStatistics {
        let leaf_nodes = self.root.get_leaf_nodes();
        let max_depth = leaf_nodes.iter().map(|n| n.depth).max().unwrap_or(0);

        SpatialFieldStatistics {
            total_nodes: self.active_node_count,
            leaf_nodes: leaf_nodes.len(),
            active_nodes: self.leaf_node_count,
            total_energy: self.total_energy,
            average_energy: if self.leaf_node_count > 0 {
                self.total_energy / self.leaf_node_count as Float
            } else {
                0.0
            },
            max_depth_reached: max_depth,
            subdivision_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_operations() {
        let p1 = Position3D::new(1.0, 2.0, 3.0);
        let p2 = Position3D::new(4.0, 6.0, 8.0);

        assert!((p1.distance_to(&p2) - 7.071067811865475).abs() < 0.001);

        let p3 = p1.lerp(&p2, 0.5);
        assert!((p3.x - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_spatial_bounds() {
        let bounds = SpatialBounds::from_min_max([-10.0, -10.0, -10.0], [10.0, 10.0, 10.0]);
        assert!(bounds.contains(&Position3D::origin()));
        assert!(!bounds.contains(&Position3D::new(15.0, 0.0, 0.0)));
    }

    #[test]
    fn test_spatial_field_creation() {
        let field = SpatialField::with_defaults();
        assert_eq!(field.active_node_count, 1);
    }

    #[test]
    fn test_entity_position_derivation() {
        let field = SpatialField::with_defaults();
        let pos = field.derive_entity_position(0, 100);

        // Should be within bounds
        assert!(field.config.bounds.contains(&pos));
    }

    #[test]
    fn test_field_subdivision() {
        let mut field = SpatialField::with_defaults();

        // Add energy at center
        field.add_energy_at(Position3D::origin(), 3, 1.0);

        // Add entities to trigger subdivision
        for i in 0..50 {
            let pos = field.derive_entity_position(i, 50);
            field.add_entity(i, pos);
        }

        field.update();

        // Should have more nodes after subdivision
        assert!(field.active_node_count > 1);
    }
}
