//! Cell Manifestation as Field Boundary
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Cell membrane as field boundary - the cell is a coherent field configuration
//!  with a membrane that separates internal field dynamics from external environment."
//!
//! # Key Insight
//!
//! Cells are field configurations with:
//! - Membrane as field boundary (amphipathic molecules at field interface)
//! - Organelles as specialized field regions
//! - Cytoplasm as the cellular field medium
//! - Nucleus as the genetic field center

use crate::holographic_foundation::archetype_profile::{
    ArchetypeActivationProfile, NUM_ARCHETYPES,
};
use crate::holographic_foundation::cellular_emergence::archetype_genes::GeneExpressionProfile;
use crate::holographic_foundation::cellular_emergence::gene_expression::GeneExpressionEngine;
use crate::holographic_foundation::cellular_emergence::nucleotide_interference::DNAHelix;
use crate::holographic_foundation::cellular_emergence::protein_field::{
    AminoAcid, ProteinFoldingField, ProteinManifestation,
};
use crate::holographic_foundation::field_state::Position3D;
use crate::holographic_foundation::molecular_emergence::MolecularManifestation;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellId(pub u64);

impl CellId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    Quiescent,
    Growing,
    Dividing,
    Differentiating,
    Apoptotic,
    Senescent,
}

impl CellState {
    pub fn from_field_coherence(coherence: Float) -> Self {
        if coherence > 0.8 {
            CellState::Growing
        } else if coherence > 0.6 {
            CellState::Quiescent
        } else if coherence > 0.4 {
            CellState::Differentiating
        } else if coherence > 0.2 {
            CellState::Senescent
        } else {
            CellState::Apoptotic
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellOrganelle {
    Nucleus,
    Mitochondria,
    EndoplasmicReticulum,
    GolgiApparatus,
    Lysosome,
    Ribosome,
    Cytoskeleton,
    Centrosome,
    Vacuole,
    Chloroplast,
}

impl CellOrganelle {
    pub fn archetype_affinity(&self) -> [Float; NUM_ARCHETYPES] {
        let mut affinity = [0.5; NUM_ARCHETYPES];

        match self {
            CellOrganelle::Nucleus => {
                affinity[0] = 0.9;
                affinity[1] = 0.8;
                affinity[14] = 0.7;
            }
            CellOrganelle::Mitochondria => {
                affinity[2] = 0.9;
                affinity[3] = 0.8;
                affinity[9] = 0.7;
            }
            CellOrganelle::EndoplasmicReticulum => {
                affinity[3] = 0.8;
                affinity[4] = 0.7;
                affinity[10] = 0.8;
            }
            CellOrganelle::GolgiApparatus => {
                affinity[4] = 0.8;
                affinity[10] = 0.9;
            }
            CellOrganelle::Lysosome => {
                affinity[5] = 0.8;
                affinity[6] = 0.7;
                affinity[11] = 0.8;
            }
            CellOrganelle::Ribosome => {
                affinity[0] = 0.7;
                affinity[7] = 0.9;
                affinity[8] = 0.8;
            }
            CellOrganelle::Cytoskeleton => {
                affinity[8] = 0.9;
                affinity[9] = 0.8;
            }
            CellOrganelle::Centrosome => {
                affinity[6] = 0.9;
                affinity[12] = 0.8;
            }
            CellOrganelle::Vacuole => {
                affinity[11] = 0.8;
                affinity[12] = 0.7;
            }
            CellOrganelle::Chloroplast => {
                affinity[2] = 0.8;
                affinity[15] = 0.9;
            }
        }

        affinity
    }

    pub fn dominant_archetypes(&self) -> &'static [usize] {
        match self {
            CellOrganelle::Nucleus => &[0, 1, 14],
            CellOrganelle::Mitochondria => &[2, 3, 9],
            CellOrganelle::EndoplasmicReticulum => &[3, 4, 10],
            CellOrganelle::GolgiApparatus => &[4, 10],
            CellOrganelle::Lysosome => &[5, 6, 11],
            CellOrganelle::Ribosome => &[0, 7, 8],
            CellOrganelle::Cytoskeleton => &[8, 9],
            CellOrganelle::Centrosome => &[6, 12],
            CellOrganelle::Vacuole => &[11, 12],
            CellOrganelle::Chloroplast => &[2, 15],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellMembrane {
    thickness: Float,
    permeability: Float,
    potential: Float,
    receptor_density: Float,
    archetype_pattern: [Float; NUM_ARCHETYPES],
}

impl CellMembrane {
    pub fn new() -> Self {
        Self {
            thickness: 7.5,
            permeability: 0.5,
            potential: -70.0,
            receptor_density: 0.5,
            archetype_pattern: [0.5; NUM_ARCHETYPES],
        }
    }

    pub fn from_archetype_profile(profile: &ArchetypeActivationProfile) -> Self {
        let coeffs = profile.coefficients();
        Self {
            thickness: 7.5,
            permeability: 0.3 + coeffs[7] * 0.4,
            potential: -70.0 + coeffs[2] * 20.0 - coeffs[15] * 20.0,
            receptor_density: 0.3 + coeffs[0] * 0.4 + coeffs[14] * 0.3,
            archetype_pattern: *coeffs,
        }
    }

    pub fn thickness(&self) -> Float {
        self.thickness
    }

    pub fn permeability(&self) -> Float {
        self.permeability
    }

    pub fn potential(&self) -> Float {
        self.potential
    }

    pub fn receptor_density(&self) -> Float {
        self.receptor_density
    }

    pub fn transport_efficiency(&self) -> Float {
        self.permeability * self.receptor_density
    }
}

impl Default for CellMembrane {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CellBoundary {
    center: Position3D,
    radius: Float,
    membrane: CellMembrane,
    surface_area: Float,
    volume: Float,
}

impl CellBoundary {
    pub fn new(center: Position3D, radius: Float) -> Self {
        let membrane = CellMembrane::new();
        let surface_area = 4.0 * std::f64::consts::PI * radius * radius;
        let volume = (4.0 / 3.0) * std::f64::consts::PI * radius * radius * radius;

        Self {
            center,
            radius,
            membrane,
            surface_area,
            volume,
        }
    }

    pub fn center(&self) -> &Position3D {
        &self.center
    }

    pub fn radius(&self) -> Float {
        self.radius
    }

    pub fn membrane(&self) -> &CellMembrane {
        &self.membrane
    }

    pub fn surface_area(&self) -> Float {
        self.surface_area
    }

    pub fn volume(&self) -> Float {
        self.volume
    }

    pub fn surface_to_volume_ratio(&self) -> Float {
        if self.volume > 0.0 {
            self.surface_area / self.volume
        } else {
            0.0
        }
    }

    pub fn contains_point(&self, point: &Position3D) -> bool {
        self.center.distance(point) < self.radius
    }
}

#[derive(Debug, Clone)]
pub struct CellularFieldConfiguration {
    organelles: HashMap<CellOrganelle, Position3D>,
    cytoplasm_coherence: Float,
    internal_field_strength: Float,
    archetype_pattern: [Float; NUM_ARCHETYPES],
}

impl CellularFieldConfiguration {
    pub fn new() -> Self {
        let mut organelles = HashMap::new();
        organelles.insert(CellOrganelle::Nucleus, Position3D::new(0.0, 0.0, 0.0));

        Self {
            organelles,
            cytoplasm_coherence: 0.7,
            internal_field_strength: 0.5,
            archetype_pattern: [0.5; NUM_ARCHETYPES],
        }
    }

    pub fn from_profile(profile: &ArchetypeActivationProfile) -> Self {
        let coeffs = *profile.coefficients();
        let mut config = Self::new();

        config.archetype_pattern = coeffs;
        config.cytoplasm_coherence =
            0.5 + coeffs.iter().sum::<Float>() / (NUM_ARCHETYPES as Float * 2.0);
        config.internal_field_strength = 0.3 + coeffs[0] * 0.4 + coeffs[7] * 0.3;

        config
    }

    pub fn add_organelle(&mut self, organelle: CellOrganelle, position: Position3D) {
        self.organelles.insert(organelle, position);
    }

    pub fn organelles(&self) -> &HashMap<CellOrganelle, Position3D> {
        &self.organelles
    }

    pub fn coherence(&self) -> Float {
        self.cytoplasm_coherence
    }

    pub fn field_strength(&self) -> Float {
        self.internal_field_strength
    }

    pub fn archetype_pattern(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_pattern
    }

    pub fn organelle_count(&self) -> usize {
        self.organelles.len()
    }
}

impl Default for CellularFieldConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CellManifestation {
    pub id: CellId,
    pub boundary: CellBoundary,
    pub field_config: CellularFieldConfiguration,
    pub state: CellState,
    pub dna: Option<DNAHelix>,
    pub proteins: ProteinFoldingField,
    pub expression_profile: GeneExpressionProfile,
    pub generation: u32,
    pub age: Float,
}

impl CellManifestation {
    pub fn new(position: Position3D, radius: Float) -> Self {
        let boundary = CellBoundary::new(position, radius);
        let field_config = CellularFieldConfiguration::new();
        let state = CellState::Quiescent;
        let expression_profile = GeneExpressionProfile::default();

        Self {
            id: CellId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            boundary,
            field_config,
            state,
            dna: None,
            proteins: ProteinFoldingField::new(),
            expression_profile,
            generation: 0,
            age: 0.0,
        }
    }

    pub fn with_dna(mut self, dna: DNAHelix) -> Self {
        self.dna = Some(dna);
        self
    }

    pub fn with_profile(mut self, profile: &ArchetypeActivationProfile) -> Self {
        self.field_config = CellularFieldConfiguration::from_profile(profile);
        self.boundary.membrane = CellMembrane::from_archetype_profile(profile);
        self.expression_profile = GeneExpressionProfile::from_profile(profile);
        self.state = CellState::from_field_coherence(self.field_config.coherence());
        self
    }

    pub fn add_protein(&mut self, protein: ProteinManifestation) {
        self.proteins.add_protein(protein);
    }

    pub fn update_state(&mut self) {
        self.state = CellState::from_field_coherence(self.field_config.coherence());
    }

    pub fn id(&self) -> CellId {
        self.id
    }

    pub fn position(&self) -> &Position3D {
        self.boundary.center()
    }

    pub fn radius(&self) -> Float {
        self.boundary.radius()
    }

    pub fn boundary(&self) -> &CellBoundary {
        &self.boundary
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn coherence(&self) -> Float {
        self.field_config.coherence()
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn age(&self) -> Float {
        self.age
    }

    pub fn protein_count(&self) -> usize {
        self.proteins.protein_count()
    }

    pub fn has_dna(&self) -> bool {
        self.dna.is_some()
    }

    pub fn can_divide(&self) -> bool {
        matches!(self.state, CellState::Growing) && self.coherence() > 0.7 && self.has_dna()
    }

    pub fn divide(&mut self) -> Option<Self> {
        if !self.can_divide() {
            return None;
        }

        self.generation += 1;
        self.age = 0.0;

        let daughter_position = Position3D::new(
            self.boundary.center().x + self.boundary.radius(),
            self.boundary.center().y,
            self.boundary.center().z,
        );

        let mut daughter = Self::new(daughter_position, self.boundary.radius() * 0.8);
        daughter.generation = self.generation;
        daughter.dna = self.dna.clone();
        daughter.expression_profile = self.expression_profile.clone();
        daughter.field_config = self.field_config.clone();

        Some(daughter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_id_creation() {
        let id = CellId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_cell_state_from_coherence() {
        assert_eq!(CellState::from_field_coherence(0.9), CellState::Growing);
        assert_eq!(CellState::from_field_coherence(0.7), CellState::Quiescent);
        assert_eq!(
            CellState::from_field_coherence(0.5),
            CellState::Differentiating
        );
        assert_eq!(CellState::from_field_coherence(0.1), CellState::Apoptotic);
    }

    #[test]
    fn test_cell_organelle_affinity() {
        let affinity = CellOrganelle::Nucleus.archetype_affinity();
        assert!(affinity[0] > 0.5);
    }

    #[test]
    fn test_cell_membrane_creation() {
        let membrane = CellMembrane::new();
        assert!(membrane.thickness() > 0.0);
        assert!(membrane.permeability() > 0.0);
    }

    #[test]
    fn test_cell_membrane_transport() {
        let membrane = CellMembrane::new();
        let transport = membrane.transport_efficiency();
        assert!(transport >= 0.0 && transport <= 1.0);
    }

    #[test]
    fn test_cell_boundary_creation() {
        let boundary = CellBoundary::new(Position3D::new(0.0, 0.0, 0.0), 10.0);
        assert_eq!(boundary.radius(), 10.0);
        assert!(boundary.surface_area() > 0.0);
        assert!(boundary.volume() > 0.0);
    }

    #[test]
    fn test_cell_boundary_contains_point() {
        let boundary = CellBoundary::new(Position3D::new(0.0, 0.0, 0.0), 10.0);
        assert!(boundary.contains_point(&Position3D::new(0.0, 0.0, 0.0)));
        assert!(!boundary.contains_point(&Position3D::new(20.0, 0.0, 0.0)));
    }

    #[test]
    fn test_cellular_field_config_creation() {
        let config = CellularFieldConfiguration::new();
        assert!(config.organelle_count() >= 1);
        assert!(config.coherence() > 0.0);
    }

    #[test]
    fn test_cell_manifestation_creation() {
        let cell = CellManifestation::new(Position3D::new(0.0, 0.0, 0.0), 10.0);
        assert!(cell.radius() > 0.0);
        assert_eq!(cell.generation(), 0);
    }

    #[test]
    fn test_cell_manifestation_with_profile() {
        let profile = ArchetypeActivationProfile::default();
        let cell =
            CellManifestation::new(Position3D::new(0.0, 0.0, 0.0), 10.0).with_profile(&profile);
        assert!(cell.coherence() > 0.0);
    }

    #[test]
    fn test_cell_cannot_divide_without_dna() {
        let mut cell = CellManifestation::new(Position3D::new(0.0, 0.0, 0.0), 10.0);
        assert!(!cell.can_divide());
    }

    #[test]
    fn test_surface_to_volume_ratio() {
        let boundary = CellBoundary::new(Position3D::new(0.0, 0.0, 0.0), 10.0);
        let ratio = boundary.surface_to_volume_ratio();
        assert!(ratio > 0.0);
    }
}
