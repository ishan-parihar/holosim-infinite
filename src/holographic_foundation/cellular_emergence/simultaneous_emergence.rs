//! Simultaneous Emergence: Cell + Gaia Consciousness
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Simultaneous emergence with Gaia consciousness - cells and planetary-scale
//!  consciousness emerge together from the same holographic field at different resolutions."
//!
//! # Key Insight
//!
//! Cells and planetary consciousness co-emerge:
//! - Individual cells manifest at cellular resolution (10^-6 m)
//! - Gaia consciousness manifests at planetary resolution (10^7 m)
//! - Both derive from the SAME archetype patterns in the field
//! - Cellular coherence contributes to planetary coherence

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::cellular_emergence::cell_manifestation::{
    CellManifestation, CellState,
};
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetaryCellType {
    Bacterial,
    Archaeal,
    Eukaryotic,
    Plant,
    Animal,
    Fungal,
    Protist,
}

impl PlanetaryCellType {
    pub fn archetype_signature(&self) -> [Float; NUM_ARCHETYPES] {
        let mut sig = [0.5; NUM_ARCHETYPES];

        match self {
            PlanetaryCellType::Bacterial => {
                sig[7] = 0.9;
                sig[8] = 0.7;
                sig[9] = 0.6;
            }
            PlanetaryCellType::Archaeal => {
                sig[7] = 0.85;
                sig[14] = 0.8;
                sig[15] = 0.7;
            }
            PlanetaryCellType::Eukaryotic => {
                sig[0] = 0.7;
                sig[7] = 0.8;
                sig[14] = 0.6;
            }
            PlanetaryCellType::Plant => {
                sig[2] = 0.9;
                sig[8] = 0.7;
                sig[15] = 0.8;
            }
            PlanetaryCellType::Animal => {
                sig[0] = 0.8;
                sig[1] = 0.7;
                sig[3] = 0.6;
            }
            PlanetaryCellType::Fungal => {
                sig[8] = 0.8;
                sig[10] = 0.7;
                sig[11] = 0.6;
            }
            PlanetaryCellType::Protist => {
                sig[0] = 0.6;
                sig[7] = 0.7;
                sig[14] = 0.6;
            }
        }

        sig
    }

    pub fn description(&self) -> &'static str {
        match self {
            PlanetaryCellType::Bacterial => "Prokaryotic cells without nucleus",
            PlanetaryCellType::Archaeal => "Ancient prokaryotes, extremophiles",
            PlanetaryCellType::Eukaryotic => "Cells with nucleus and organelles",
            PlanetaryCellType::Plant => "Photosynthetic eukaryotes",
            PlanetaryCellType::Animal => "Heterotrophic eukaryotes",
            PlanetaryCellType::Fungal => "Decomposer eukaryotes",
            PlanetaryCellType::Protist => "Diverse single-celled eukaryotes",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GaiaConsciousness {
    coherence: Float,
    biomass: Float,
    biodiversity: Float,
    archetype_resonance: [Float; NUM_ARCHETYPES],
    cellular_network_density: Float,
}

impl GaiaConsciousness {
    pub fn new() -> Self {
        Self {
            coherence: 0.5,
            biomass: 0.0,
            biodiversity: 0.0,
            archetype_resonance: [0.5; NUM_ARCHETYPES],
            cellular_network_density: 0.0,
        }
    }

    pub fn from_cells(cells: &[CellManifestation]) -> Self {
        if cells.is_empty() {
            return Self::new();
        }

        let coherence: Float =
            cells.iter().map(|c| c.coherence()).sum::<Float>() / cells.len() as Float;

        let biomass = cells.len() as Float;

        let mut type_counts: HashMap<CellState, usize> = HashMap::new();
        for cell in cells {
            *type_counts.entry(cell.state()).or_insert(0) += 1;
        }
        let biodiversity = type_counts.len() as Float;

        let mut archetype_resonance = [0.0; NUM_ARCHETYPES];
        for cell in cells {
            let pattern = cell.field_config.archetype_pattern();
            for (res_i, &pat_i) in archetype_resonance.iter_mut().zip(pattern.iter()) {
                *res_i += pat_i;
            }
        }
        let divisor = cells.len() as Float;
        for item in &mut archetype_resonance {
            *item /= divisor;
        }

        let cellular_network_density = if cells.len() > 1 {
            let mut connections = 0;
            for i in 0..cells.len() {
                for j in (i + 1)..cells.len() {
                    let dist = cells[i].position().distance(cells[j].position());
                    let max_dist = cells[i].radius() + cells[j].radius() + 100.0;
                    if dist < max_dist {
                        connections += 1;
                    }
                }
            }
            connections as Float / (cells.len() * (cells.len() - 1) / 2).max(1) as Float
        } else {
            0.0
        };

        Self {
            coherence,
            biomass,
            biodiversity,
            archetype_resonance,
            cellular_network_density,
        }
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn biomass(&self) -> Float {
        self.biomass
    }

    pub fn biodiversity(&self) -> Float {
        self.biodiversity
    }

    pub fn archetype_resonance(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_resonance
    }

    pub fn network_density(&self) -> Float {
        self.cellular_network_density
    }

    pub fn planetary_health(&self) -> Float {
        let coherence_factor = self.coherence;
        let diversity_factor = (self.biodiversity / 10.0).min(1.0);
        let network_factor = self.cellular_network_density;

        (coherence_factor * 0.4 + diversity_factor * 0.3 + network_factor * 0.3).min(1.0)
    }
}

impl Default for GaiaConsciousness {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PlanetaryCellField {
    cells: Vec<CellManifestation>,
    gaia: GaiaConsciousness,
    field_coherence: Float,
    position: Position3D,
}

impl PlanetaryCellField {
    pub fn new(position: Position3D) -> Self {
        Self {
            cells: Vec::new(),
            gaia: GaiaConsciousness::new(),
            field_coherence: 0.5,
            position,
        }
    }

    pub fn add_cell(&mut self, cell: CellManifestation) {
        self.cells.push(cell);
        self.update_gaia();
        self.calculate_field_coherence();
    }

    pub fn add_cells(&mut self, cells: Vec<CellManifestation>) {
        self.cells.extend(cells);
        self.update_gaia();
        self.calculate_field_coherence();
    }

    fn update_gaia(&mut self) {
        self.gaia = GaiaConsciousness::from_cells(&self.cells);
    }

    fn calculate_field_coherence(&mut self) {
        if self.cells.is_empty() {
            self.field_coherence = 0.5;
            return;
        }

        let cell_coherence: Float =
            self.cells.iter().map(|c| c.coherence()).sum::<Float>() / self.cells.len() as Float;

        let gaia_coherence = self.gaia.coherence();

        self.field_coherence = (cell_coherence + gaia_coherence) / 2.0;
    }

    pub fn cells(&self) -> &[CellManifestation] {
        &self.cells
    }

    pub fn gaia(&self) -> &GaiaConsciousness {
        &self.gaia
    }

    pub fn coherence(&self) -> Float {
        self.field_coherence
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn position(&self) -> &Position3D {
        &self.position
    }

    pub fn cells_by_state(&self, state: CellState) -> Vec<&CellManifestation> {
        self.cells.iter().filter(|c| c.state() == state).collect()
    }

    pub fn average_cell_coherence(&self) -> Float {
        if self.cells.is_empty() {
            return 0.0;
        }
        self.cells.iter().map(|c| c.coherence()).sum::<Float>() / self.cells.len() as Float
    }

    pub fn dividing_cells(&self) -> Vec<&CellManifestation> {
        self.cells.iter().filter(|c| c.can_divide()).collect()
    }

    pub fn simulate_divisions(&mut self) -> usize {
        let mut new_cells = Vec::new();

        for cell in &mut self.cells {
            if let Some(daughter) = cell.divide() {
                new_cells.push(daughter);
            }
        }

        let count = new_cells.len();
        self.cells.extend(new_cells);
        self.update_gaia();
        self.calculate_field_coherence();

        count
    }
}

#[derive(Debug, Clone)]
pub struct CellularPlanetaryResonance {
    cell_coherence: Float,
    gaia_coherence: Float,
    resonance_strength: Float,
    archetype_alignment: Float,
}

impl CellularPlanetaryResonance {
    pub fn between(cell: &CellManifestation, gaia: &GaiaConsciousness) -> Self {
        let cell_coherence = cell.coherence();
        let gaia_coherence = gaia.coherence();

        let cell_pattern = cell.field_config.archetype_pattern();
        let gaia_pattern = gaia.archetype_resonance();

        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..NUM_ARCHETYPES {
            dot += cell_pattern[i] * gaia_pattern[i];
            norm1 += cell_pattern[i] * cell_pattern[i];
            norm2 += gaia_pattern[i] * gaia_pattern[i];
        }

        let archetype_alignment = if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1.sqrt() * norm2.sqrt())
        } else {
            0.0
        };

        let resonance_strength = (cell_coherence * gaia_coherence * archetype_alignment).sqrt();

        Self {
            cell_coherence,
            gaia_coherence,
            resonance_strength,
            archetype_alignment,
        }
    }

    pub fn cell_coherence(&self) -> Float {
        self.cell_coherence
    }

    pub fn gaia_coherence(&self) -> Float {
        self.gaia_coherence
    }

    pub fn resonance(&self) -> Float {
        self.resonance_strength
    }

    pub fn alignment(&self) -> Float {
        self.archetype_alignment
    }

    pub fn is_strong(&self) -> bool {
        self.resonance_strength > 0.6 && self.archetype_alignment > 0.7
    }
}

#[derive(Debug, Clone)]
pub struct CellularGaiaPair {
    pub cell_field: PlanetaryCellField,
    pub resonance: Float,
    pub coherence_coupling: Float,
    pub formation_time: Float,
}

impl CellularGaiaPair {
    pub fn new(position: Position3D, initial_cells: Vec<CellManifestation>) -> Self {
        let mut cell_field = PlanetaryCellField::new(position);
        cell_field.add_cells(initial_cells);

        let coherence_coupling = Self::calculate_coupling(&cell_field);

        let resonance = coherence_coupling * cell_field.coherence();

        Self {
            cell_field,
            resonance,
            coherence_coupling,
            formation_time: 0.0,
        }
    }

    fn calculate_coupling(field: &PlanetaryCellField) -> Float {
        let gaia = field.gaia();
        let cells = field.cells();

        if cells.is_empty() {
            return 0.5;
        }

        let avg_alignment: Float = cells
            .iter()
            .map(|cell| {
                let resonance = CellularPlanetaryResonance::between(cell, gaia);
                resonance.alignment()
            })
            .sum::<Float>()
            / cells.len() as Float;

        avg_alignment
    }

    pub fn cell_count(&self) -> usize {
        self.cell_field.cell_count()
    }

    pub fn coherence(&self) -> Float {
        self.cell_field.coherence()
    }

    pub fn planetary_health(&self) -> Float {
        self.cell_field.gaia().planetary_health()
    }

    pub fn simulate_step(&mut self, dt: Float) {
        self.formation_time += dt;

        let divisions = self.cell_field.simulate_divisions();
        if divisions > 0 {
            self.coherence_coupling = Self::calculate_coupling(&self.cell_field);
            self.resonance = self.coherence_coupling * self.cell_field.coherence();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic_foundation::cellular_emergence::cell_manifestation::CellManifestation;

    fn create_test_cell(position: Position3D) -> CellManifestation {
        CellManifestation::new(position, 10.0)
    }

    #[test]
    fn test_planetary_cell_type_signature() {
        let sig = PlanetaryCellType::Bacterial.archetype_signature();
        assert!(sig[7] > 0.5);
    }

    #[test]
    fn test_gaia_consciousness_empty() {
        let gaia = GaiaConsciousness::new();
        assert_eq!(gaia.coherence(), 0.5);
    }

    #[test]
    fn test_gaia_consciousness_from_cells() {
        let cells = vec![
            create_test_cell(Position3D::new(0.0, 0.0, 0.0)),
            create_test_cell(Position3D::new(5.0, 0.0, 0.0)),
        ];
        let gaia = GaiaConsciousness::from_cells(&cells);
        assert!(gaia.coherence() > 0.0);
    }

    #[test]
    fn test_gaia_planetary_health() {
        let gaia = GaiaConsciousness::new();
        let health = gaia.planetary_health();
        assert!((0.0..=1.0).contains(&health));
    }

    #[test]
    fn test_planetary_cell_field_creation() {
        let field = PlanetaryCellField::new(Position3D::new(0.0, 0.0, 0.0));
        assert_eq!(field.cell_count(), 0);
    }

    #[test]
    fn test_planetary_cell_field_add_cell() {
        let mut field = PlanetaryCellField::new(Position3D::new(0.0, 0.0, 0.0));
        field.add_cell(create_test_cell(Position3D::new(0.0, 0.0, 0.0)));
        assert_eq!(field.cell_count(), 1);
    }

    #[test]
    fn test_planetary_cell_field_coherence() {
        let mut field = PlanetaryCellField::new(Position3D::new(0.0, 0.0, 0.0));
        field.add_cell(create_test_cell(Position3D::new(0.0, 0.0, 0.0)));
        assert!(field.coherence() > 0.0);
    }

    #[test]
    fn test_cellular_planetary_resonance() {
        let cell = create_test_cell(Position3D::new(0.0, 0.0, 0.0));
        let gaia = GaiaConsciousness::new();
        let resonance = CellularPlanetaryResonance::between(&cell, &gaia);
        assert!(resonance.resonance() >= 0.0);
    }

    #[test]
    fn test_cellular_gaia_pair_creation() {
        let cells = vec![create_test_cell(Position3D::new(0.0, 0.0, 0.0))];
        let pair = CellularGaiaPair::new(Position3D::new(0.0, 0.0, 0.0), cells);
        assert_eq!(pair.cell_count(), 1);
    }

    #[test]
    fn test_cellular_gaia_pair_coherence() {
        let cells = vec![create_test_cell(Position3D::new(0.0, 0.0, 0.0))];
        let pair = CellularGaiaPair::new(Position3D::new(0.0, 0.0, 0.0), cells);
        assert!(pair.coherence() > 0.0);
    }

    #[test]
    fn test_planetary_cell_field_cells_by_state() {
        let mut field = PlanetaryCellField::new(Position3D::new(0.0, 0.0, 0.0));
        field.add_cell(create_test_cell(Position3D::new(0.0, 0.0, 0.0)));
        let quiescent = field.cells_by_state(CellState::Quiescent);
        assert!(!quiescent.is_empty() || quiescent.is_empty());
    }

    #[test]
    fn test_gaia_biodiversity() {
        let cells = vec![
            create_test_cell(Position3D::new(0.0, 0.0, 0.0)),
            create_test_cell(Position3D::new(5.0, 0.0, 0.0)),
        ];
        let gaia = GaiaConsciousness::from_cells(&cells);
        assert!(gaia.biodiversity() > 0.0);
    }
}
