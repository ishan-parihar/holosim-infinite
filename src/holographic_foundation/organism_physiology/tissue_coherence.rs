//! Tissue as Coherent Cell Field
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Tissue as coherent cell field - tissues are NOT just collections of cells,
//!  but coherent field configurations where cells share archetype patterns."
//!
//! # Key Insight
//!
//! Tissues are field configurations:
//! - Cells in a tissue share coherent archetype patterns
//! - Tissue coherence = degree of archetype alignment among cells
//! - Tissue function = field dynamics at tissue resolution

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::cellular_emergence::{CellId, CellManifestation};
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TissueId(pub u64);

impl TissueId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TissueType {
    Epithelial,
    Connective,
    Muscle,
    Nervous,
    Blood,
    Adipose,
    Cartilage,
    Bone,
    Lymphoid,
}

impl TissueType {
    pub fn dominant_archetype(&self) -> usize {
        match self {
            TissueType::Epithelial => 0,
            TissueType::Connective => 8,
            TissueType::Muscle => 9,
            TissueType::Nervous => 1,
            TissueType::Blood => 2,
            TissueType::Adipose => 10,
            TissueType::Cartilage => 8,
            TissueType::Bone => 10,
            TissueType::Lymphoid => 5,
        }
    }

    pub fn archetype_pattern(&self) -> [Float; NUM_ARCHETYPES] {
        let mut pattern = [0.5; NUM_ARCHETYPES];
        let dominant = self.dominant_archetype();

        pattern[dominant] = 0.9;

        match self {
            TissueType::Epithelial => {
                pattern[0] = 0.85;
                pattern[7] = 0.75;
            }
            TissueType::Connective => {
                pattern[8] = 0.85;
                pattern[9] = 0.7;
            }
            TissueType::Muscle => {
                pattern[9] = 0.9;
                pattern[2] = 0.7;
            }
            TissueType::Nervous => {
                pattern[0] = 0.9;
                pattern[1] = 0.85;
                pattern[2] = 0.75;
            }
            TissueType::Blood => {
                pattern[2] = 0.85;
                pattern[8] = 0.7;
            }
            TissueType::Adipose => {
                pattern[10] = 0.8;
                pattern[11] = 0.65;
            }
            TissueType::Cartilage => {
                pattern[8] = 0.8;
                pattern[10] = 0.75;
            }
            TissueType::Bone => {
                pattern[10] = 0.9;
                pattern[8] = 0.7;
            }
            TissueType::Lymphoid => {
                pattern[5] = 0.85;
                pattern[11] = 0.7;
            }
        }

        for i in 14..21 {
            pattern[i] = pattern[i - 14] * 0.6;
        }
        pattern[21] = 0.5;

        pattern
    }

    pub fn description(&self) -> &'static str {
        match self {
            TissueType::Epithelial => "Protective covering and lining tissue",
            TissueType::Connective => "Support and binding tissue",
            TissueType::Muscle => "Contractile tissue for movement",
            TissueType::Nervous => "Signal transmission tissue",
            TissueType::Blood => "Circulatory fluid tissue",
            TissueType::Adipose => "Fat storage tissue",
            TissueType::Cartilage => "Flexible support tissue",
            TissueType::Bone => "Rigid structural tissue",
            TissueType::Lymphoid => "Immune system tissue",
        }
    }

    pub fn regenerative_capacity(&self) -> Float {
        match self {
            TissueType::Epithelial => 0.9,
            TissueType::Connective => 0.7,
            TissueType::Muscle => 0.3,
            TissueType::Nervous => 0.1,
            TissueType::Blood => 1.0,
            TissueType::Adipose => 0.8,
            TissueType::Cartilage => 0.2,
            TissueType::Bone => 0.5,
            TissueType::Lymphoid => 0.7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TissueVitality {
    pub coherence: Float,
    pub cell_density: Float,
    pub vascularization: Float,
    pub innervation: Float,
    pub extracellular_matrix_health: Float,
}

impl Default for TissueVitality {
    fn default() -> Self {
        Self {
            coherence: 0.85,
            cell_density: 0.8,
            vascularization: 0.75,
            innervation: 0.7,
            extracellular_matrix_health: 0.8,
        }
    }
}

impl TissueVitality {
    pub fn overall_health(&self) -> Float {
        (self.coherence
            + self.cell_density
            + self.vascularization
            + self.innervation
            + self.extracellular_matrix_health)
            / 5.0
    }

    pub fn is_healthy(&self) -> bool {
        self.overall_health() > 0.7
    }
}

#[derive(Debug, Clone)]
pub struct TissueCoherence {
    archetype_pattern: [Float; NUM_ARCHETYPES],
    coherence_value: Float,
    cell_alignment: Float,
    field_resonance: Float,
}

impl TissueCoherence {
    pub fn new() -> Self {
        Self {
            archetype_pattern: [0.5; NUM_ARCHETYPES],
            coherence_value: 0.85,
            cell_alignment: 0.8,
            field_resonance: 0.75,
        }
    }

    pub fn from_cells(cells: &[CellManifestation], tissue_type: TissueType) -> Self {
        if cells.is_empty() {
            return Self::new();
        }

        let archetype_pattern = tissue_type.archetype_pattern();

        let mut total_alignment = 0.0;
        for cell in cells {
            let cell_pattern = cell.field_config.archetype_pattern();
            let mut alignment = 0.0;
            for i in 0..NUM_ARCHETYPES {
                alignment += cell_pattern[i] * archetype_pattern[i];
            }
            total_alignment += alignment / NUM_ARCHETYPES as Float;
        }
        let cell_alignment = total_alignment / cells.len() as Float;

        let coherence_value = cell_alignment * 0.6 + 0.4;

        let field_resonance = coherence_value * cell_alignment;

        Self {
            archetype_pattern,
            coherence_value,
            cell_alignment,
            field_resonance,
        }
    }

    pub fn archetype_pattern(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_pattern
    }

    pub fn coherence(&self) -> Float {
        self.coherence_value
    }

    pub fn alignment(&self) -> Float {
        self.cell_alignment
    }

    pub fn resonance(&self) -> Float {
        self.field_resonance
    }
}

impl Default for TissueCoherence {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Tissue {
    pub id: TissueId,
    pub tissue_type: TissueType,
    pub position: Position3D,
    pub cells: Vec<CellId>,
    pub coherence: TissueCoherence,
    pub vitality: TissueVitality,
    pub volume: Float,
    pub parent_organ:
        Option<crate::holographic_foundation::organism_physiology::organ_field::OrganId>,
}

impl Tissue {
    pub fn new(tissue_type: TissueType, position: Position3D, volume: Float) -> Self {
        let coherence = TissueCoherence::new();
        let vitality = TissueVitality::default();

        Self {
            id: TissueId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            tissue_type,
            position,
            cells: Vec::new(),
            coherence,
            vitality,
            volume,
            parent_organ: None,
        }
    }

    pub fn with_cells(mut self, cells: Vec<CellManifestation>) -> Self {
        self.coherence = TissueCoherence::from_cells(&cells, self.tissue_type);
        self.cells = cells.iter().map(|c| c.id()).collect();
        self.vitality.cell_density = (cells.len() as Float / self.volume.max(1.0)).min(1.0);
        self
    }

    pub fn add_cell(&mut self, cell: &CellManifestation) {
        self.cells.push(cell.id());
        self.recalculate_coherence();
    }

    fn recalculate_coherence(&mut self) {
        self.vitality.cell_density = (self.cells.len() as Float / self.volume.max(1.0)).min(1.0);
    }

    pub fn id(&self) -> TissueId {
        self.id
    }

    pub fn tissue_type(&self) -> TissueType {
        self.tissue_type
    }

    pub fn coherence(&self) -> Float {
        self.coherence.coherence()
    }

    pub fn health(&self) -> Float {
        self.vitality.overall_health()
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn volume(&self) -> Float {
        self.volume
    }

    pub fn regenerate(&mut self, dt: Float) {
        let regen_rate = self.tissue_type.regenerative_capacity();
        self.vitality.coherence = (self.vitality.coherence + regen_rate * dt * 0.01).min(1.0);
        self.vitality.extracellular_matrix_health =
            (self.vitality.extracellular_matrix_health + regen_rate * dt * 0.005).min(1.0);
    }

    pub fn update(&mut self, dt: Float) {
        self.regenerate(dt);
    }

    pub fn apply_damage(&mut self, amount: Float) {
        self.vitality.coherence = (self.vitality.coherence - amount * 0.1).max(0.0);
        self.vitality.extracellular_matrix_health =
            (self.vitality.extracellular_matrix_health - amount * 0.05).max(0.0);
        self.coherence.coherence_value = self.vitality.coherence;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tissue_id_creation() {
        let id = TissueId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_tissue_type_dominant_archetype() {
        assert_eq!(TissueType::Nervous.dominant_archetype(), 1);
        assert_eq!(TissueType::Muscle.dominant_archetype(), 9);
        assert_eq!(TissueType::Bone.dominant_archetype(), 10);
    }

    #[test]
    fn test_tissue_type_archetype_pattern() {
        let pattern = TissueType::Nervous.archetype_pattern();
        assert!(pattern[0] > 0.8);
        assert!(pattern[1] > 0.8);
    }

    #[test]
    fn test_tissue_type_regenerative_capacity() {
        assert!(
            TissueType::Blood.regenerative_capacity() > TissueType::Nervous.regenerative_capacity()
        );
    }

    #[test]
    fn test_tissue_vitality_default() {
        let vit = TissueVitality::default();
        assert!(vit.coherence > 0.8);
    }

    #[test]
    fn test_tissue_vitality_overall_health() {
        let vit = TissueVitality::default();
        assert!(vit.overall_health() > 0.7);
    }

    #[test]
    fn test_tissue_coherence_new() {
        let coherence = TissueCoherence::new();
        assert!(coherence.coherence() > 0.8);
    }

    #[test]
    fn test_tissue_creation() {
        let tissue = Tissue::new(TissueType::Muscle, Position3D::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(tissue.tissue_type(), TissueType::Muscle);
    }

    #[test]
    fn test_tissue_volume() {
        let tissue = Tissue::new(TissueType::Bone, Position3D::new(0.0, 0.0, 0.0), 5.0);
        assert_eq!(tissue.volume(), 5.0);
    }

    #[test]
    fn test_tissue_regenerate() {
        let mut tissue = Tissue::new(TissueType::Epithelial, Position3D::new(0.0, 0.0, 0.0), 1.0);
        let initial_coherence = tissue.coherence();
        tissue.regenerate(1.0);
        assert!(tissue.coherence() >= initial_coherence);
    }

    #[test]
    fn test_tissue_apply_damage() {
        let mut tissue = Tissue::new(TissueType::Muscle, Position3D::new(0.0, 0.0, 0.0), 1.0);
        let initial_coherence = tissue.coherence();
        tissue.apply_damage(0.5);
        assert!(tissue.coherence() < initial_coherence);
    }

    #[test]
    fn test_tissue_health() {
        let tissue = Tissue::new(TissueType::Nervous, Position3D::new(0.0, 0.0, 0.0), 1.0);
        assert!(tissue.health() > 0.5);
    }
}
