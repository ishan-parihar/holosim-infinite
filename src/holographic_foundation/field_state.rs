//! Holographic Field State with Recursive Subdivision
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The holographic field is stored at multiple scale levels.
//!  Each level is a compressed version of the previous (MERA-style)."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The holographic principle states that each entity contains the whole.
//!  Each part contains the whole image - the completeness is maintained."
//!
//! The field uses octree subdivision with adaptive refinement:
//! - Only regions with significant activity are subdivided to full depth
//! - Each node contains field amplitude for all 8 density components
//! - MERA compression enables efficient storage and O(log n) access

use std::fmt;

use super::archetype_profile::NUM_ARCHETYPES;
use super::scale_level::{all_scale_levels, ScaleLevel, NUM_SCALE_LEVELS};
use crate::types::Float;

pub const MAX_SUBDIVISION_DEPTH: u8 = 12;
pub const COHERENCE_THRESHOLD: Float = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldAmplitude {
    pub real: Float,
    pub imaginary: Float,
}

impl FieldAmplitude {
    pub fn new(real: Float, imaginary: Float) -> Self {
        Self { real, imaginary }
    }

    pub fn zero() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            real: 1.0,
            imaginary: 0.0,
        }
    }

    pub fn magnitude(&self) -> Float {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    pub fn phase(&self) -> Float {
        self.imaginary.atan2(self.real)
    }

    pub fn from_polar(magnitude: Float, phase: Float) -> Self {
        Self {
            real: magnitude * phase.cos(),
            imaginary: magnitude * phase.sin(),
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn add(&self, other: &FieldAmplitude) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    pub fn multiply(&self, other: &FieldAmplitude) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl Default for FieldAmplitude {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone)]
pub struct FieldNode {
    pub position: Position3D,
    pub bounds: BoundingBox,
    pub depth: u8,
    pub amplitudes: [FieldAmplitude; NUM_SCALE_LEVELS],
    pub archetype_vector: [Float; NUM_ARCHETYPES],
    pub coherence: Float,
    pub children: Option<Box<[FieldNode; 8]>>,
}

impl FieldNode {
    pub fn new(position: Position3D, bounds: BoundingBox, depth: u8) -> Self {
        Self {
            position,
            bounds,
            depth,
            amplitudes: [FieldAmplitude::zero(); NUM_SCALE_LEVELS],
            archetype_vector: [0.5; NUM_ARCHETYPES],
            coherence: 0.5,
            children: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn should_subdivide(&self) -> bool {
        self.coherence > COHERENCE_THRESHOLD && self.depth < MAX_SUBDIVISION_DEPTH
    }

    pub fn subdivide(&mut self) {
        if self.children.is_some() {
            return;
        }

        let half_size = self.bounds.size / 2.0;
        let mut children = Vec::with_capacity(8);

        for octant in 0..8 {
            let child_min = Position3D {
                x: self.bounds.min.x + if octant & 1 != 0 { half_size } else { 0.0 },
                y: self.bounds.min.y + if octant & 2 != 0 { half_size } else { 0.0 },
                z: self.bounds.min.z + if octant & 4 != 0 { half_size } else { 0.0 },
            };

            let child_bounds = BoundingBox {
                min: child_min,
                size: half_size,
            };

            let child_position = child_bounds.center();

            let mut child = FieldNode::new(child_position, child_bounds, self.depth + 1);

            for i in 0..NUM_SCALE_LEVELS {
                child.amplitudes[i] = self.amplitudes[i].clone();
            }
            child.archetype_vector = self.archetype_vector;
            child.coherence = self.coherence * 0.9;

            children.push(child);
        }

        let children_array: [FieldNode; 8] = children
            .try_into()
            .unwrap_or_else(|v: Vec<FieldNode>| panic!("Expected 8 children, got {}", v.len()));

        self.children = Some(Box::new(children_array));
    }

    pub fn get_amplitude_at_scale(&self, scale: ScaleLevel) -> &FieldAmplitude {
        &self.amplitudes[scale as usize]
    }

    pub fn set_amplitude_at_scale(&mut self, scale: ScaleLevel, amplitude: FieldAmplitude) {
        self.amplitudes[scale as usize] = amplitude;
        self.recalculate_coherence();
    }

    pub fn recalculate_coherence(&mut self) {
        let magnitudes: Vec<Float> = self.amplitudes.iter().map(|a| a.magnitude()).collect();
        let mean: Float = magnitudes.iter().sum::<Float>() / magnitudes.len() as Float;
        let variance: Float = magnitudes.iter().map(|m| (m - mean).powi(2)).sum::<Float>()
            / magnitudes.len() as Float;
        self.coherence = 1.0 - variance.sqrt().min(1.0);
    }

    pub fn total_amplitude(&self) -> Float {
        self.amplitudes.iter().map(|a| a.magnitude()).sum()
    }

    pub fn dominant_scale(&self) -> ScaleLevel {
        let mut max_mag = 0.0;
        let mut dominant = ScaleLevel::Quantum;

        for (i, amp) in self.amplitudes.iter().enumerate() {
            let mag = amp.magnitude();
            if mag > max_mag {
                max_mag = mag;
                dominant = ScaleLevel::from_u8(i as u8).unwrap_or(ScaleLevel::Quantum);
            }
        }

        dominant
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Position3D {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn origin() -> Self {
        Self::zero()
    }

    pub fn distance(&self, other: &Position3D) -> Float {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn add(&self, other: &Position3D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Default for Position3D {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Position3D,
    pub size: Float,
}

impl BoundingBox {
    pub fn new(min: Position3D, size: Float) -> Self {
        Self { min, size }
    }

    pub fn unit() -> Self {
        Self {
            min: Position3D::zero(),
            size: 1.0,
        }
    }

    pub fn contains(&self, position: &Position3D) -> bool {
        position.x >= self.min.x
            && position.y >= self.min.y
            && position.z >= self.min.z
            && position.x <= self.min.x + self.size
            && position.y <= self.min.y + self.size
            && position.z <= self.min.z + self.size
    }

    pub fn center(&self) -> Position3D {
        Position3D::new(
            self.min.x + self.size / 2.0,
            self.min.y + self.size / 2.0,
            self.min.z + self.size / 2.0,
        )
    }

    pub fn octant(&self, position: &Position3D) -> Option<u8> {
        if !self.contains(position) {
            return None;
        }

        let center = self.center();
        let mut octant = 0u8;

        if position.x >= center.x {
            octant |= 1;
        }
        if position.y >= center.y {
            octant |= 2;
        }
        if position.z >= center.z {
            octant |= 4;
        }

        Some(octant)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::unit()
    }
}

pub struct HolographicFieldState {
    root: FieldNode,
    scale_levels: Vec<ScaleLevelData>,
    total_nodes: usize,
    max_depth: u8,
    global_coherence: Float,
}

impl HolographicFieldState {
    pub fn new(size: Float) -> Self {
        let bounds = BoundingBox::new(Position3D::zero(), size);
        let root = FieldNode::new(
            Position3D::new(size / 2.0, size / 2.0, size / 2.0),
            bounds,
            0,
        );

        let scale_levels = all_scale_levels()
            .iter()
            .map(|&scale| ScaleLevelData::new(scale))
            .collect();

        Self {
            root,
            scale_levels,
            total_nodes: 1,
            max_depth: 0,
            global_coherence: 0.5,
        }
    }

    pub fn unity() -> Self {
        let mut field = Self::new(1.0);
        for i in 0..NUM_SCALE_LEVELS {
            field.root.amplitudes[i] = FieldAmplitude::one();
        }
        field.global_coherence = 1.0;
        field
    }

    pub fn total_nodes(&self) -> usize {
        self.total_nodes
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }

    pub fn global_coherence(&self) -> Float {
        self.global_coherence
    }

    pub fn get_node_at(&self, position: &Position3D) -> Option<&FieldNode> {
        self.get_node_recursive(&self.root, position)
    }

    fn get_node_recursive<'a>(
        &'a self,
        node: &'a FieldNode,
        position: &Position3D,
    ) -> Option<&'a FieldNode> {
        if !node.bounds.contains(position) {
            return None;
        }

        if node.is_leaf() {
            return Some(node);
        }

        if let Some(ref children) = node.children {
            if let Some(octant) = node.bounds.octant(position) {
                return self.get_node_recursive(&children[octant as usize], position);
            }
        }

        Some(node)
    }

    pub fn get_or_create_node_at(&mut self, position: &Position3D) -> Option<&FieldNode> {
        if !self.root.bounds.contains(position) {
            return None;
        }

        let (new_nodes, new_depth) =
            Self::ensure_path_exists_impl(&mut self.root, position, self.max_depth);
        self.total_nodes += new_nodes;
        self.max_depth = new_depth;
        self.get_node_at(position)
    }

    fn ensure_path_exists_impl(
        node: &mut FieldNode,
        position: &Position3D,
        current_max_depth: u8,
    ) -> (usize, u8) {
        if !node.bounds.contains(position) {
            return (0, current_max_depth);
        }

        let mut new_nodes = 0;
        let mut max_depth = current_max_depth;

        if node.should_subdivide() {
            node.subdivide();
            new_nodes += 8;
            max_depth = max_depth.max(node.depth + 1);
        }

        if node.is_leaf() {
            return (new_nodes, max_depth);
        }

        if let Some(ref mut children) = node.children {
            if let Some(octant) = node.bounds.octant(position) {
                let (child_nodes, child_depth) = Self::ensure_path_exists_impl(
                    &mut children[octant as usize],
                    position,
                    max_depth,
                );
                new_nodes += child_nodes;
                max_depth = child_depth;
            }
        }

        (new_nodes, max_depth)
    }

    pub fn get_or_create_node_mut(&mut self, position: &Position3D) -> Option<&mut FieldNode> {
        if !self.root.bounds.contains(position) {
            return None;
        }

        let (new_nodes, new_depth) =
            Self::ensure_path_exists_impl(&mut self.root, position, self.max_depth);
        self.total_nodes += new_nodes;
        self.max_depth = new_depth;
        Self::get_node_mut_impl(&mut self.root, position)
    }

    fn get_node_mut_impl<'a>(
        node: &'a mut FieldNode,
        position: &Position3D,
    ) -> Option<&'a mut FieldNode> {
        if !node.bounds.contains(position) {
            return None;
        }

        if node.is_leaf() {
            return Some(node);
        }

        let octant = match node.bounds.octant(position) {
            Some(o) => o,
            None => return Some(node),
        };

        if let Some(ref mut children) = node.children {
            return Self::get_node_mut_impl(&mut children[octant as usize], position);
        }

        Some(node)
    }

    pub fn set_amplitude(
        &mut self,
        position: &Position3D,
        scale: ScaleLevel,
        amplitude: FieldAmplitude,
    ) {
        if !self.root.bounds.contains(position) {
            return;
        }

        let (new_nodes, new_depth) =
            Self::ensure_path_exists_impl(&mut self.root, position, self.max_depth);
        self.total_nodes += new_nodes;
        self.max_depth = new_depth;

        if let Some(node) = Self::get_node_mut_impl(&mut self.root, position) {
            node.set_amplitude_at_scale(scale, amplitude);
        }
        self.recalculate_global_coherence();
    }

    pub fn get_amplitude(
        &self,
        position: &Position3D,
        scale: ScaleLevel,
    ) -> Option<FieldAmplitude> {
        self.get_node_at(position)
            .map(|n| n.get_amplitude_at_scale(scale).clone())
    }

    fn recalculate_global_coherence(&mut self) {
        let mut total_coherence = 0.0;
        let mut node_count = 0;

        self.traverse_nodes(&self.root, &mut |node: &FieldNode| {
            total_coherence += node.coherence;
            node_count += 1;
        });

        self.global_coherence = if node_count > 0 {
            total_coherence / node_count as Float
        } else {
            0.0
        };
    }

    fn traverse_nodes<F>(&self, node: &FieldNode, visitor: &mut F)
    where
        F: FnMut(&FieldNode),
    {
        visitor(node);

        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.traverse_nodes(child, visitor);
            }
        }
    }

    pub fn for_each_node<F>(&self, mut visitor: F)
    where
        F: FnMut(&FieldNode),
    {
        self.traverse_nodes(&self.root, &mut visitor);
    }

    pub fn root(&self) -> &FieldNode {
        &self.root
    }

    pub fn extract_coherence_peaks(&self, threshold: Float) -> Vec<CoherencePeak> {
        let mut peaks = Vec::new();
        self.extract_peaks_recursive(&self.root, threshold, &mut peaks);
        peaks
    }

    fn extract_peaks_recursive(
        &self,
        node: &FieldNode,
        threshold: Float,
        peaks: &mut Vec<CoherencePeak>,
    ) {
        if node.coherence >= threshold {
            let total_amp = node.total_amplitude();
            if total_amp > threshold {
                peaks.push(CoherencePeak {
                    position: node.position,
                    coherence: node.coherence,
                    amplitude: total_amp,
                    dominant_scale: node.dominant_scale(),
                    archetype_vector: node.archetype_vector,
                    depth: node.depth,
                });
            }
        }

        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.extract_peaks_recursive(child, threshold, peaks);
            }
        }
    }

    pub fn evolve(&mut self, dt: Float) {
        Self::evolve_node_impl(&mut self.root, dt);
        self.recalculate_global_coherence();
    }

    fn evolve_node_impl(node: &mut FieldNode, dt: Float) {
        for i in 0..NUM_SCALE_LEVELS {
            let current = &node.amplitudes[i];
            let phase_evolution = 0.1 * dt * (i + 1) as Float;
            let new_amplitude = FieldAmplitude::from_polar(
                current.magnitude() * (1.0 - 0.01 * dt),
                current.phase() + phase_evolution,
            );
            node.amplitudes[i] = new_amplitude;
        }

        node.recalculate_coherence();

        if let Some(ref mut children) = node.children {
            for child in children.iter_mut() {
                Self::evolve_node_impl(child, dt);
            }
        }
    }

    pub fn get_scale_data(&self, scale: ScaleLevel) -> Option<&ScaleLevelData> {
        self.scale_levels.get(scale as usize)
    }

    pub fn field_statistics(&self) -> FieldStatistics {
        let mut stats = FieldStatistics::default();

        self.traverse_nodes(&self.root, &mut |node: &FieldNode| {
            stats.node_count += 1;
            stats.total_coherence += node.coherence;
            stats.total_amplitude += node.total_amplitude();

            for i in 0..NUM_SCALE_LEVELS {
                stats.scale_amplitudes[i] += node.amplitudes[i].magnitude();
            }
        });

        if stats.node_count > 0 {
            stats.average_coherence = stats.total_coherence / stats.node_count as Float;
            stats.average_amplitude = stats.total_amplitude / stats.node_count as Float;
        }

        stats
    }
}

impl fmt::Debug for HolographicFieldState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HolographicFieldState")
            .field("total_nodes", &self.total_nodes)
            .field("max_depth", &self.max_depth)
            .field("global_coherence", &self.global_coherence)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ScaleLevelData {
    pub scale: ScaleLevel,
    pub total_amplitude: Float,
    pub average_coherence: Float,
    pub node_count: usize,
}

impl ScaleLevelData {
    pub fn new(scale: ScaleLevel) -> Self {
        Self {
            scale,
            total_amplitude: 0.0,
            average_coherence: 0.0,
            node_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoherencePeak {
    pub position: Position3D,
    pub coherence: Float,
    pub amplitude: Float,
    pub dominant_scale: ScaleLevel,
    pub archetype_vector: [Float; NUM_ARCHETYPES],
    pub depth: u8,
}

impl CoherencePeak {
    pub fn to_entity_potential(&self) -> EntityPotential {
        EntityPotential {
            position: self.position,
            coherence: self.coherence,
            suggested_density: self.dominant_scale.to_density(),
            archetype_activation: self.archetype_vector,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityPotential {
    pub position: Position3D,
    pub coherence: Float,
    pub suggested_density: u8,
    pub archetype_activation: [Float; NUM_ARCHETYPES],
}

#[derive(Debug, Clone)]
pub struct FieldStatistics {
    pub node_count: usize,
    pub total_coherence: Float,
    pub total_amplitude: Float,
    pub average_coherence: Float,
    pub average_amplitude: Float,
    pub scale_amplitudes: [Float; NUM_SCALE_LEVELS],
}

impl Default for FieldStatistics {
    fn default() -> Self {
        Self {
            node_count: 0,
            total_coherence: 0.0,
            total_amplitude: 0.0,
            average_coherence: 0.0,
            average_amplitude: 0.0,
            scale_amplitudes: [0.0; NUM_SCALE_LEVELS],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_amplitude_operations() {
        let a = FieldAmplitude::new(1.0, 0.0);
        let b = FieldAmplitude::new(0.0, 1.0);

        assert!((a.magnitude() - 1.0).abs() < 1e-10);
        assert!((b.phase() - std::f64::consts::PI / 2.0).abs() < 1e-10);

        let sum = a.add(&b);
        assert!((sum.magnitude() - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_field_creation() {
        let field = HolographicFieldState::new(1.0);
        assert_eq!(field.total_nodes(), 1);
        assert_eq!(field.max_depth(), 0);
    }

    #[test]
    fn test_field_subdivision() {
        let mut field = HolographicFieldState::new(1.0);
        field.root.coherence = 0.9;

        let pos = Position3D::new(0.75, 0.75, 0.75);
        let _ = field.get_or_create_node_at(&pos);

        assert!(field.total_nodes() > 1);
        assert!(field.max_depth() > 0);
    }

    #[test]
    fn test_amplitude_setting() {
        let mut field = HolographicFieldState::new(1.0);
        let pos = Position3D::new(0.5, 0.5, 0.5);

        field.set_amplitude(&pos, ScaleLevel::Biological, FieldAmplitude::one());

        let amp = field.get_amplitude(&pos, ScaleLevel::Biological);
        assert!(amp.is_some());
        assert!((amp.unwrap().magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_coherence_peak_extraction() {
        let mut field = HolographicFieldState::new(1.0);
        field.root.coherence = 0.8;
        for amp in field.root.amplitudes.iter_mut() {
            *amp = FieldAmplitude::one();
        }

        let peaks = field.extract_coherence_peaks(0.5);
        assert!(!peaks.is_empty());
    }

    #[test]
    fn test_field_evolution() {
        let mut field = HolographicFieldState::new(1.0);
        field.root.amplitudes[0] = FieldAmplitude::one();

        let initial_amp = field.root.amplitudes[0].magnitude();
        field.evolve(1.0);
        let final_amp = field.root.amplitudes[0].magnitude();

        assert!(final_amp < initial_amp);
    }

    #[test]
    fn test_unity_field() {
        let field = HolographicFieldState::unity();
        assert!((field.global_coherence() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box_octant() {
        let bounds = BoundingBox::unit();

        assert_eq!(bounds.octant(&Position3D::new(0.25, 0.25, 0.25)), Some(0));
        assert_eq!(bounds.octant(&Position3D::new(0.75, 0.25, 0.25)), Some(1));
        assert_eq!(bounds.octant(&Position3D::new(0.25, 0.75, 0.25)), Some(2));
        assert_eq!(bounds.octant(&Position3D::new(0.75, 0.75, 0.75)), Some(7));
        assert_eq!(bounds.octant(&Position3D::new(1.5, 0.5, 0.5)), None);
    }

    #[test]
    fn test_dominant_scale() {
        let mut node = FieldNode::new(Position3D::zero(), BoundingBox::unit(), 0);

        node.amplitudes[ScaleLevel::Stellar as usize] = FieldAmplitude::new(2.0, 0.0);

        assert_eq!(node.dominant_scale(), ScaleLevel::Stellar);
    }
}
