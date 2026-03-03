//! Spatial Ecosystem Structure - Patches, Corridors, Spatial Structure
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Spatial ecosystem structure (patches, corridors)"
//!
//! # Key Insight
//!
//! Ecosystems have spatial structure determined by field configuration:
//! - Patches = areas of coherent field pattern
//! - Corridors = field resonance connections between patches
//! - Fragmentation = field coherence disruption

use crate::holographic_foundation::ecosystem_dynamics::population_dynamics::PopulationId;
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatchId(pub u64);

impl PatchId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchType {
    Core,
    Edge,
    Buffer,
    Corridor,
    Matrix,
    Sink,
    Source,
}

impl PatchType {
    pub fn quality_multiplier(&self) -> Float {
        match self {
            PatchType::Core => 1.0,
            PatchType::Source => 0.95,
            PatchType::Buffer => 0.8,
            PatchType::Edge => 0.6,
            PatchType::Corridor => 0.5,
            PatchType::Sink => 0.3,
            PatchType::Matrix => 0.1,
        }
    }

    pub fn disturbance_susceptibility(&self) -> Float {
        match self {
            PatchType::Core => 0.1,
            PatchType::Source => 0.15,
            PatchType::Buffer => 0.3,
            PatchType::Edge => 0.5,
            PatchType::Corridor => 0.4,
            PatchType::Sink => 0.7,
            PatchType::Matrix => 0.9,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PatchType::Core => "Interior habitat with minimal edge effects",
            PatchType::Edge => "Transition zone between habitat types",
            PatchType::Buffer => "Protective zone around core habitat",
            PatchType::Corridor => "Linear connection between patches",
            PatchType::Matrix => "Non-habitat surrounding patches",
            PatchType::Sink => "Population mortality exceeds reproduction",
            PatchType::Source => "Population reproduction exceeds mortality",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EcologicalPatch {
    pub id: PatchId,
    pub patch_type: PatchType,
    pub center: Position3D,
    pub area: Float,
    pub perimeter: Float,
    pub populations: Vec<PopulationId>,
    pub resource_density: Float,
    pub field_coherence: Float,
    pub carrying_capacity_modifier: Float,
    pub edge_ratio: Float,
    pub shape_index: Float,
}

impl EcologicalPatch {
    pub fn new(patch_type: PatchType, center: Position3D, area: Float) -> Self {
        let perimeter = 4.0 * area.sqrt();
        let edge_ratio = perimeter / (2.0 * (std::f64::consts::PI * area).sqrt());
        let shape_index = perimeter / (4.0 * area.sqrt());

        Self {
            id: PatchId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            patch_type,
            center,
            area,
            perimeter,
            populations: Vec::new(),
            resource_density: match patch_type {
                PatchType::Core => 0.9,
                PatchType::Source => 0.85,
                PatchType::Buffer => 0.6,
                PatchType::Edge => 0.5,
                PatchType::Corridor => 0.4,
                PatchType::Sink => 0.3,
                PatchType::Matrix => 0.1,
            },
            field_coherence: match patch_type {
                PatchType::Core => 0.9,
                PatchType::Source => 0.85,
                _ => 0.5,
            },
            carrying_capacity_modifier: patch_type.quality_multiplier(),
            edge_ratio,
            shape_index,
        }
    }

    pub fn add_population(&mut self, pop_id: PopulationId) {
        if !self.populations.contains(&pop_id) {
            self.populations.push(pop_id);
        }
    }

    pub fn remove_population(&mut self, pop_id: &PopulationId) {
        self.populations.retain(|id| id != pop_id);
    }

    pub fn effective_capacity(&self, base_capacity: Float) -> Float {
        base_capacity * self.area * self.carrying_capacity_modifier * self.field_coherence
    }

    pub fn core_area(&self) -> Float {
        let edge_width = 10.0;
        let effective_edge = self.perimeter * edge_width;
        (self.area - effective_edge).max(0.0)
    }

    pub fn interior_to_edge_ratio(&self) -> Float {
        let core = self.core_area();
        if self.area > 0.0 {
            core / self.area
        } else {
            0.0
        }
    }

    pub fn update(&mut self, dt: Float) {
        let stability_factor =
            self.field_coherence * (1.0 - self.patch_type.disturbance_susceptibility());
        self.resource_density = (self.resource_density * (1.0 - 0.01 * dt)
            + stability_factor * 0.01 * dt)
            .clamp(0.1, 1.0);
    }

    pub fn distance_to(&self, other: &EcologicalPatch) -> Float {
        self.center.distance(&other.center)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchConnectivity {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl PatchConnectivity {
    pub fn from_strength(strength: Float) -> Self {
        if strength < 0.1 {
            PatchConnectivity::None
        } else if strength < 0.3 {
            PatchConnectivity::Low
        } else if strength < 0.6 {
            PatchConnectivity::Medium
        } else if strength < 0.8 {
            PatchConnectivity::High
        } else {
            PatchConnectivity::VeryHigh
        }
    }

    pub fn movement_probability(&self) -> Float {
        match self {
            PatchConnectivity::None => 0.0,
            PatchConnectivity::Low => 0.1,
            PatchConnectivity::Medium => 0.3,
            PatchConnectivity::High => 0.6,
            PatchConnectivity::VeryHigh => 0.9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Corridor {
    pub patch_a: PatchId,
    pub patch_b: PatchId,
    pub length: Float,
    pub width: Float,
    pub quality: Float,
    pub permeability: Float,
    pub field_resonance: Float,
}

impl Corridor {
    pub fn new(patch_a: PatchId, patch_b: PatchId, length: Float, width: Float) -> Self {
        let quality = (width / length).min(1.0);
        let permeability = quality * 0.8;

        Self {
            patch_a,
            patch_b,
            length,
            width,
            quality,
            permeability,
            field_resonance: 0.5,
        }
    }

    pub fn effective_conductance(&self) -> Float {
        self.permeability * self.field_resonance * (self.width / self.length).min(1.0)
    }

    pub fn travel_cost(&self) -> Float {
        self.length / (self.width * self.permeability).max(0.1)
    }

    pub fn supports_species(&self, species_dispersal_ability: Float) -> bool {
        self.effective_conductance() > 0.1 && self.length < species_dispersal_ability
    }
}

#[derive(Debug, Clone)]
pub struct SpatialEcosystem {
    pub patches: HashMap<PatchId, EcologicalPatch>,
    pub corridors: Vec<Corridor>,
    pub connectivity_matrix: HashMap<(PatchId, PatchId), PatchConnectivity>,
    pub total_habitat_area: Float,
    pub fragmentation_index: Float,
    pub landscape_coherence: Float,
}

impl SpatialEcosystem {
    pub fn new() -> Self {
        Self {
            patches: HashMap::new(),
            corridors: Vec::new(),
            connectivity_matrix: HashMap::new(),
            total_habitat_area: 0.0,
            fragmentation_index: 0.0,
            landscape_coherence: 0.8,
        }
    }

    pub fn add_patch(&mut self, patch: EcologicalPatch) -> PatchId {
        let id = patch.id;
        self.total_habitat_area += patch.area;
        self.patches.insert(id, patch);
        self.update_fragmentation();
        id
    }

    pub fn remove_patch(&mut self, patch_id: &PatchId) {
        if let Some(patch) = self.patches.remove(patch_id) {
            self.total_habitat_area -= patch.area;
            self.corridors
                .retain(|c| c.patch_a != *patch_id && c.patch_b != *patch_id);
            self.connectivity_matrix
                .retain(|(a, b), _| *a != *patch_id && *b != *patch_id);
            self.update_fragmentation();
        }
    }

    pub fn add_corridor(&mut self, corridor: Corridor) {
        let connectivity = PatchConnectivity::from_strength(corridor.effective_conductance());
        self.connectivity_matrix
            .insert((corridor.patch_a, corridor.patch_b), connectivity);
        self.connectivity_matrix
            .insert((corridor.patch_b, corridor.patch_a), connectivity);
        self.corridors.push(corridor);
    }

    pub fn connect_patches(&mut self, patch_a: PatchId, patch_b: PatchId) {
        if let (Some(a), Some(b)) = (self.patches.get(&patch_a), self.patches.get(&patch_b)) {
            let length = a.distance_to(b);
            let width = (a.area + b.area).sqrt() * 0.5;
            let mut corridor = Corridor::new(patch_a, patch_b, length, width);
            corridor.field_resonance = 0.9;
            self.add_corridor(corridor);
        }
    }

    fn update_fragmentation(&mut self) {
        if self.patches.is_empty() {
            self.fragmentation_index = 0.0;
            return;
        }

        let n = self.patches.len() as Float;
        let sum_area: Float = self.patches.values().map(|p| p.area).sum();

        let sum_squared: Float = self.patches.values().map(|p| p.area.powi(2)).sum();

        if sum_area > 0.0 {
            self.fragmentation_index = (n - 1.0) * sum_squared / sum_area.powi(2);
        } else {
            self.fragmentation_index = 0.0;
        }

        self.fragmentation_index = (self.fragmentation_index / n).min(1.0);
    }

    pub fn connectivity(&self, patch_a: &PatchId, patch_b: &PatchId) -> PatchConnectivity {
        *self
            .connectivity_matrix
            .get(&(*patch_a, *patch_b))
            .unwrap_or(&PatchConnectivity::None)
    }

    pub fn find_connected_patches(&self, patch_id: &PatchId) -> Vec<PatchId> {
        self.connectivity_matrix
            .iter()
            .filter(|((a, b), c)| {
                (a == patch_id || b == patch_id) && !matches!(c, PatchConnectivity::None)
            })
            .map(|((a, b), _)| if a == patch_id { *b } else { *a })
            .collect()
    }

    pub fn shortest_path(&self, from: &PatchId, to: &PatchId) -> Option<Vec<PatchId>> {
        if from == to {
            return Some(vec![*from]);
        }

        let mut visited: HashSet<PatchId> = HashSet::new();
        let mut queue: Vec<(PatchId, Vec<PatchId>)> = vec![(*from, vec![*from])];
        visited.insert(*from);

        while let Some((current, path)) = queue.pop() {
            for neighbor in self.find_connected_patches(&current) {
                if neighbor == *to {
                    let mut result = path.clone();
                    result.push(neighbor);
                    return Some(result);
                }

                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.insert(0, (neighbor, new_path));
                }
            }
        }

        None
    }

    pub fn dispersal_probability(
        &self,
        from: &PatchId,
        to: &PatchId,
        dispersal_distance: Float,
    ) -> Float {
        if let (Some(patch_from), Some(patch_to)) = (self.patches.get(from), self.patches.get(to)) {
            let distance = patch_from.distance_to(patch_to);

            if distance > dispersal_distance {
                return 0.0;
            }

            let connectivity = self.connectivity(from, to);
            let connectivity_prob = connectivity.movement_probability();

            let distance_prob = 1.0 - (distance / dispersal_distance);

            connectivity_prob * distance_prob * self.landscape_coherence
        } else {
            0.0
        }
    }

    pub fn update(&mut self, dt: Float) {
        for patch in self.patches.values_mut() {
            patch.update(dt);
        }

        for corridor in self.corridors.iter_mut() {
            let decay = 0.001 * dt;
            corridor.quality = (corridor.quality - decay).max(0.1);
            corridor.permeability = corridor.quality * 0.8;
        }

        self.landscape_coherence = self
            .patches
            .values()
            .map(|p| p.field_coherence)
            .sum::<Float>()
            / self.patches.len().max(1) as Float;
    }

    pub fn patch_richness(&self, patch_id: &PatchId) -> usize {
        self.patches
            .get(patch_id)
            .map(|p| p.populations.len())
            .unwrap_or(0)
    }

    pub fn total_richness(&self) -> usize {
        let mut all_species: HashSet<PopulationId> = HashSet::new();
        for patch in self.patches.values() {
            all_species.extend(&patch.populations);
        }
        all_species.len()
    }
}

impl Default for SpatialEcosystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_id_creation() {
        let id = PatchId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_patch_type_quality() {
        assert!(PatchType::Core.quality_multiplier() > PatchType::Edge.quality_multiplier());
    }

    #[test]
    fn test_patch_creation() {
        let patch = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        assert_eq!(patch.patch_type, PatchType::Core);
        assert!(patch.area > 0.0);
    }

    #[test]
    fn test_patch_effective_capacity() {
        let patch = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        let capacity = patch.effective_capacity(10.0);
        assert!(capacity > 0.0);
    }

    #[test]
    fn test_patch_add_population() {
        let mut patch =
            EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        patch.add_population(PopulationId::new(1));
        assert_eq!(patch.populations.len(), 1);
    }

    #[test]
    fn test_patch_distance() {
        let patch1 = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        let patch2 = EcologicalPatch::new(PatchType::Core, Position3D::new(3.0, 4.0, 0.0), 100.0);
        let distance = patch1.distance_to(&patch2);
        assert!((distance - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_patch_connectivity_from_strength() {
        assert_eq!(
            PatchConnectivity::from_strength(0.0),
            PatchConnectivity::None
        );
        assert_eq!(
            PatchConnectivity::from_strength(0.2),
            PatchConnectivity::Low
        );
        assert_eq!(
            PatchConnectivity::from_strength(0.5),
            PatchConnectivity::Medium
        );
        assert_eq!(
            PatchConnectivity::from_strength(0.7),
            PatchConnectivity::High
        );
        assert_eq!(
            PatchConnectivity::from_strength(0.9),
            PatchConnectivity::VeryHigh
        );
    }

    #[test]
    fn test_corridor_creation() {
        let corridor = Corridor::new(PatchId::new(1), PatchId::new(2), 100.0, 10.0);
        assert!(corridor.effective_conductance() > 0.0);
    }

    #[test]
    fn test_corridor_travel_cost() {
        let corridor = Corridor::new(PatchId::new(1), PatchId::new(2), 100.0, 10.0);
        assert!(corridor.travel_cost() > 0.0);
    }

    #[test]
    fn test_spatial_ecosystem_creation() {
        let ecosystem = SpatialEcosystem::new();
        assert!(ecosystem.patches.is_empty());
        assert!(ecosystem.corridors.is_empty());
    }

    #[test]
    fn test_spatial_ecosystem_add_patch() {
        let mut ecosystem = SpatialEcosystem::new();
        let patch = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        ecosystem.add_patch(patch);

        assert_eq!(ecosystem.patches.len(), 1);
        assert!(ecosystem.total_habitat_area > 0.0);
    }

    #[test]
    fn test_spatial_ecosystem_connect_patches() {
        let mut ecosystem = SpatialEcosystem::new();

        let patch1 = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        let id1 = ecosystem.add_patch(patch1);

        let patch2 = EcologicalPatch::new(PatchType::Core, Position3D::new(10.0, 0.0, 0.0), 100.0);
        let id2 = ecosystem.add_patch(patch2);

        ecosystem.connect_patches(id1, id2);

        assert_eq!(ecosystem.corridors.len(), 1);
    }

    #[test]
    fn test_spatial_ecosystem_find_connected() {
        let mut ecosystem = SpatialEcosystem::new();

        let patch1 = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        let id1 = ecosystem.add_patch(patch1);

        let patch2 = EcologicalPatch::new(PatchType::Core, Position3D::new(10.0, 0.0, 0.0), 100.0);
        let id2 = ecosystem.add_patch(patch2);

        ecosystem.connect_patches(id1, id2);

        let connected = ecosystem.find_connected_patches(&id1);
        assert!(connected.contains(&id2));
    }

    #[test]
    fn test_spatial_ecosystem_dispersal_probability() {
        let mut ecosystem = SpatialEcosystem::new();

        let patch1 = EcologicalPatch::new(PatchType::Core, Position3D::new(0.0, 0.0, 0.0), 100.0);
        let id1 = ecosystem.add_patch(patch1);

        let patch2 = EcologicalPatch::new(PatchType::Core, Position3D::new(5.0, 0.0, 0.0), 100.0);
        let id2 = ecosystem.add_patch(patch2);

        ecosystem.connect_patches(id1, id2);

        let prob = ecosystem.dispersal_probability(&id1, &id2, 20.0);
        assert!(prob > 0.0 && prob <= 1.0);
    }
}
