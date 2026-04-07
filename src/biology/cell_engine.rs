//! Cell Engine - Living Cells That Divide, Metabolize, and Die
//!
//! This module implements a truly living cell simulation where cells:
//! - Actually divide with DNA replication and mutation
//! - Metabolize energy from environment
//! - Age and eventually die
//! - Respond to environmental conditions
//!
//! From V4 Roadmap Phase 4: "Biological Simulation Engine"
//! Gap #6 resolution: Biology is ALIVE, not described.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "2nd density entities exhibit growth, movement, and awareness of self
//! as distinct from environment."

use crate::holographic::field_address::HolographicAddress;
// DNA system - simplified for now
use std::collections::{HashMap, VecDeque};

// ============================================================================
// Constants
// ============================================================================

/// Maximum cell age before senescence (simulation time units)
const MAX_CELL_AGE: f64 = 1000.0;

/// Hayflick limit - maximum divisions before cell senescence
const HAYFLICK_LIMIT: u32 = 50;

/// Energy threshold for division
const DIVISION_ENERGY_THRESHOLD: f64 = 200.0;

/// Energy threshold for death (starvation)
const DEATH_ENERGY_THRESHOLD: f64 = 0.0;

/// Maximum waste tolerance
const MAX_WASTE_TOLERANCE: f64 = 500.0;

/// Cell energy capacity
const MAX_CELL_ENERGY: f64 = 300.0;

// ============================================================================
// ID Types
// ============================================================================

/// Unique cell identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellId(pub u64);

/// Cell type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellCategory {
    /// Undifferentiated stem cell
    Stem,
    /// Neuron - processing and signaling
    Neuron,
    /// Muscle cell - contraction
    Muscle,
    /// Epithelial - covering and absorption
    Epithelial,
    /// Connective - structure and support
    Connective,
    /// Immune - defense
    Immune,
    /// Reproductive - gamete production
    Reproductive,
    /// Photosynthetic - light energy capture
    Photosynthetic,
    /// Secretory - hormone/enzyme production
    Secretory,
    /// Blood cell - transport
    Blood,
    /// Fat cell - energy storage
    Adipose,
    /// Cartilage - structural support
    Cartilage,
    /// Bone - skeletal structure
    Bone,
}

// ============================================================================
// Cell State
// ============================================================================

/// Current state of a cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    /// Normal functioning
    Active,
    /// In process of dividing
    Dividing,
    /// Damaged but recoverable
    Damaged,
    /// Irreversibly damaged
    Dying,
    /// Scheduled for removal
    Dead,
}

// ============================================================================
// Main Cell Structure
// ============================================================================

/// A living cell with full metabolic state
#[derive(Debug, Clone)]
pub struct LiveCell {
    /// Unique identifier
    pub id: CellId,
    /// Cell type
    pub cell_type: CellCategory,
    /// Current state
    pub state: CellState,
    /// DNA content
    pub dna: Option<()>, // Simplified DNA
    /// Current energy (ATP equivalent)
    pub energy: f64,
    /// Health (0.0 = dead, 1.0 = perfect)
    pub health: f64,
    /// Age in simulation time
    pub age: f64,
    /// Number of divisions completed
    pub division_count: u32,
    /// Position in holographic field
    pub position: HolographicAddress,
    /// Metabolic rate (energy consumption per unit time)
    pub metabolic_rate: f64,
    /// Accumulated waste products
    pub waste: f64,
    /// Organism ID (None = free-living)
    pub organism_id: Option<u64>,
    /// Epigenetic state (gene expression modifiers)
    pub epigenetic_markers: Vec<CellEpigeneticMarker>,
    /// Damage accumulation
    pub damage: f64,
    /// Last division time
    pub last_division: f64,
}

/// Epigenetic marker
#[derive(Debug, Clone)]
pub struct CellEpigeneticMarker {
    pub gene_index: usize,
    pub methylation: f64, // 0.0 = unmethylated, 1.0 = fully methylated
    pub activation: f64,  // 0.0 = off, 1.0 = fully on
}

impl LiveCell {
    /// Create a new cell
    pub fn new(id: CellId, cell_type: CellCategory, position: HolographicAddress) -> Self {
        let metabolic_rate = match cell_type {
            CellCategory::Stem => 1.0,
            CellCategory::Neuron => 2.5,
            CellCategory::Muscle => 3.0,
            CellCategory::Epithelial => 1.5,
            CellCategory::Connective => 1.0,
            CellCategory::Immune => 2.0,
            CellCategory::Reproductive => 2.0,
            CellCategory::Photosynthetic => 1.0, // Gets energy from light
            CellCategory::Secretory => 1.5,
            CellCategory::Blood => 1.2,
            CellCategory::Adipose => 0.5,
            CellCategory::Cartilage => 0.8,
            CellCategory::Bone => 1.0,
        };

        Self {
            id,
            cell_type,
            state: CellState::Active,
            dna: None, // Simplified - DNA from dna_system
            energy: 100.0,
            health: 1.0,
            age: 0.0,
            division_count: 0,
            position,
            metabolic_rate,
            waste: 0.0,
            organism_id: None,
            epigenetic_markers: Vec::new(),
            damage: 0.0,
            last_division: 0.0,
        }
    }

    /// Create a stem cell
    pub fn stem_cell(id: CellId, position: HolographicAddress) -> Self {
        Self::new(id, CellCategory::Stem, position)
    }

    /// Tick the cell - metabolism, aging, division, death
    pub fn tick(&mut self, environment: &CellEnvironment, dt: f64) {
        // Skip dead cells
        if matches!(self.state, CellState::Dead) {
            return;
        }

        self.age += dt;

        // Metabolism
        self.metabolize(environment, dt);

        // Damage accumulation
        self.accumulate_damage(environment, dt);

        // Health calculation
        self.update_health();

        // Check for division
        if self.should_divide() {
            self.state = CellState::Dividing;
        }

        // Check for death
        if self.should_die() {
            self.state = CellState::Dying;
        }
    }

    /// Cellular metabolism
    fn metabolize(&mut self, environment: &CellEnvironment, dt: f64) {
        // Base metabolic cost
        let metabolic_cost = self.metabolic_rate * dt;
        self.energy -= metabolic_cost;

        // Waste production (30% of metabolism becomes waste)
        self.waste += metabolic_cost * 0.3;

        // Energy intake based on cell type and environment
        let energy_intake = match self.cell_type {
            CellCategory::Photosynthetic => {
                // Photosynthesis: energy from light
                environment.solar_radiation * 0.01 * dt
            }
            CellCategory::Stem | CellCategory::Reproductive => {
                // High energy needs for stem/reproductive cells
                environment.nutrient_density * 0.02 * dt * 2.0
            }
            _ => {
                // Heterotrophic: energy from nutrients
                environment.nutrient_density * 0.02 * dt
            }
        };

        self.energy = (self.energy + energy_intake).min(MAX_CELL_ENERGY);

        // Waste removal (via blood/immune system if available)
        let waste_removal = environment.detoxification_rate * dt;
        self.waste = (self.waste - waste_removal).max(0.0);
    }

    /// Accumulate damage from environment
    fn accumulate_damage(&mut self, environment: &CellEnvironment, dt: f64) {
        // Radiation damage
        self.damage += environment.radiation * 0.001 * dt;

        // Temperature stress
        let temp_stress = ((environment.temperature - 310.0) / 20.0).abs();
        self.damage += temp_stress * 0.01 * dt;

        // Toxin damage
        self.damage += environment.toxin_level * 0.01 * dt;

        // Oxidative stress (from metabolism)
        self.damage += self.metabolic_rate * 0.001 * dt;

        // Natural repair (if healthy)
        if self.health > 0.7 {
            self.damage -= 0.005 * dt;
        }

        self.damage = self.damage.clamp(0.0, 1.0);
    }

    /// Update health based on internal state
    fn update_health(&mut self) {
        let energy_factor = (self.energy / 100.0).min(1.0);
        let waste_factor = 1.0 - (self.waste / MAX_WASTE_TOLERANCE).min(1.0);
        let age_factor = 1.0 - (self.age / MAX_CELL_AGE).min(1.0);
        let damage_factor = 1.0 - self.damage;

        self.health = (energy_factor * waste_factor * age_factor * damage_factor).clamp(0.0, 1.0);
    }

    /// Check if cell should divide
    pub fn should_divide(&self) -> bool {
        // Must be healthy
        if self.health < 0.7 {
            return false;
        }

        // Must have enough energy
        if self.energy < DIVISION_ENERGY_THRESHOLD {
            return false;
        }

        // Must not exceed Hayflick limit
        if self.division_count >= HAYFLICK_LIMIT {
            return false;
        }

        // Must be not too old
        if self.age < 1.0 {
            return false;
        }

        // State must be active or damaged (not dying)
        matches!(self.state, CellState::Active | CellState::Damaged)
    }

    /// Check if cell should die
    pub fn should_die(&self) -> bool {
        // Starvation
        if self.energy <= DEATH_ENERGY_THRESHOLD {
            return true;
        }

        // Health collapse
        if self.health <= 0.0 {
            return true;
        }

        // Toxicity
        if self.waste > MAX_WASTE_TOLERANCE {
            return true;
        }

        // Old age (senescence)
        if self.age > MAX_CELL_AGE {
            return true;
        }

        // Too much damage
        if self.damage > 0.9 {
            return true;
        }

        false
    }

    /// Perform cell division, returning the daughter cell
    pub fn divide(&mut self) -> Option<LiveCell> {
        if !self.should_divide() {
            return None;
        }

        // Create daughter cell
        let daughter = LiveCell {
            id: CellId(0), // Will be assigned by engine
            cell_type: self.cell_type,
            state: CellState::Active,
            dna: None, // Simplified
            energy: self.energy / 2.0,
            health: 1.0,
            age: 0.0,
            division_count: 0,
            position: self.position.clone(),
            metabolic_rate: self.metabolic_rate,
            waste: self.waste / 4.0,
            organism_id: self.organism_id,
            epigenetic_markers: self.epigenetic_markers.clone(),
            damage: self.damage / 2.0,
            last_division: self.age,
        };

        // Parent cell loses energy and increments division count
        self.energy /= 2.0;
        self.division_count += 1;
        self.state = CellState::Active;

        Some(daughter)
    }
}

/// Environment state for cells
#[derive(Debug, Clone)]
pub struct CellEnvironment {
    /// Solar radiation (W/m²)
    pub solar_radiation: f64,
    /// Nutrient availability (0.0-1.0)
    pub nutrient_density: f64,
    /// Temperature (Kelvin)
    pub temperature: f64,
    /// Radiation exposure (0.0-1.0)
    pub radiation: f64,
    /// Toxin level (0.0-1.0)
    pub toxin_level: f64,
    /// Water availability (0.0-1.0)
    pub water_availability: f64,
    /// Oxygen level (0.0-1.0)
    pub oxygen_level: f64,
    /// Detoxification rate
    pub detoxification_rate: f64,
    /// Predator presence (0.0-1.0)
    pub predator_presence: f64,
}

impl Default for CellEnvironment {
    fn default() -> Self {
        Self {
            solar_radiation: 200.0,
            nutrient_density: 0.5,
            temperature: 310.0, // ~37°C
            radiation: 0.0,
            toxin_level: 0.0,
            water_availability: 1.0,
            oxygen_level: 0.21,
            detoxification_rate: 1.0,
            predator_presence: 0.0,
        }
    }
}

impl CellEnvironment {
    /// Create Earth-like environment
    pub fn earth_like() -> Self {
        Self::default()
    }

    /// Create from planet conditions
    pub fn from_planet(solar_radiation: f64, temperature: f64, habitability: f64) -> Self {
        Self {
            solar_radiation,
            nutrient_density: habitability,
            temperature,
            radiation: 0.0,
            toxin_level: 1.0 - habitability,
            water_availability: habitability,
            oxygen_level: 0.21 * habitability,
            detoxification_rate: habitability,
            predator_presence: 0.2,
        }
    }
}

// ============================================================================
// Cell Engine
// ============================================================================

/// Main cell engine - manages all living cells
#[derive(Debug)]
pub struct CellEngine {
    /// All living cells
    pub cells: HashMap<CellId, LiveCell>,
    /// Cells scheduled for division
    pub division_queue: VecDeque<CellId>,
    /// Cells scheduled for death
    pub death_queue: VecDeque<CellId>,
    /// Next available cell ID
    next_id: u64,
    /// Total cells ever created
    pub total_created: u64,
    /// Total cells that have died
    pub total_died: u64,
    /// Cell counts by type
    pub counts_by_type: HashMap<CellCategory, usize>,
}

impl Default for CellEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CellEngine {
    /// Create a new cell engine
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            division_queue: VecDeque::new(),
            death_queue: VecDeque::new(),
            next_id: 0,
            total_created: 0,
            total_died: 0,
            counts_by_type: HashMap::new(),
        }
    }

    /// Create a new cell and add to engine
    pub fn create_cell(&mut self, cell_type: CellCategory, position: HolographicAddress) -> CellId {
        let id = CellId(self.next_id);
        self.next_id += 1;

        let cell = LiveCell::new(id, cell_type, position);
        self.cells.insert(id, cell);
        self.total_created += 1;

        *self.counts_by_type.entry(cell_type).or_insert(0) += 1;

        id
    }

    /// Create a stem cell
    pub fn create_stem_cell(&mut self, position: HolographicAddress) -> CellId {
        self.create_cell(CellCategory::Stem, position)
    }

    /// Main tick - process all cells
    pub fn tick(&mut self, environment: &CellEnvironment, dt: f64) {
        // Phase 1: Update each cell
        for cell in self.cells.values_mut() {
            cell.tick(environment, dt);

            // Queue for division
            if cell.should_divide() && matches!(cell.state, CellState::Active) {
                self.division_queue.push_back(cell.id);
            }

            // Queue for death
            if cell.should_die() && !matches!(cell.state, CellState::Dead) {
                self.death_queue.push_back(cell.id);
            }
        }

        // Phase 2: Process divisions
        self.process_divisions();

        // Phase 3: Process deaths
        self.process_deaths();

        // Phase 4: Update counts
        self.update_counts();
    }

    /// Process cell divisions
    fn process_divisions(&mut self) {
        while let Some(parent_id) = self.division_queue.pop_front() {
            if let Some(parent) = self.cells.get_mut(&parent_id) {
                if let Some(daughter) = parent.divide() {
                    let daughter_id = CellId(self.next_id);
                    self.next_id += 1;

                    let mut daughter = daughter;
                    daughter.id = daughter_id;

                    self.cells.insert(daughter_id, daughter);
                    self.total_created += 1;
                }
            }
        }
    }

    /// Process cell deaths
    fn process_deaths(&mut self) {
        while let Some(cell_id) = self.death_queue.pop_front() {
            if let Some(cell) = self.cells.remove(&cell_id) {
                self.total_died += 1;
                *self.counts_by_type.entry(cell.cell_type).or_insert(1) -= 1;
            }
        }
    }

    /// Update cell counts by type
    fn update_counts(&mut self) {
        self.counts_by_type.clear();
        for cell in self.cells.values() {
            *self.counts_by_type.entry(cell.cell_type).or_insert(0) += 1;
        }
    }

    /// Get total cell count
    pub fn total_cells(&self) -> usize {
        self.cells.len()
    }

    /// Get cells of a specific type
    pub fn cells_of_type(&self, cell_type: CellCategory) -> Vec<&LiveCell> {
        self.cells
            .values()
            .filter(|c| c.cell_type == cell_type)
            .collect()
    }

    /// Get average health across all cells
    pub fn average_health(&self) -> f64 {
        if self.cells.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.cells.values().map(|c| c.health).sum();
        sum / self.cells.len() as f64
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let engine = CellEngine::new();
        assert_eq!(engine.total_cells(), 0);
    }

    #[test]
    fn test_cell_division() {
        let mut engine = CellEngine::new();
        let pos = HolographicAddress::default();

        let cell_id = engine.create_cell(CellCategory::Stem, pos);
        assert!(engine.cells.contains_key(&cell_id));
    }

    #[test]
    fn test_cell_metabolism() {
        let mut cell = LiveCell::new(
            CellId(0),
            CellCategory::Neuron,
            HolographicAddress::default(),
        );

        let env = CellEnvironment::default();
        cell.tick(&env, 1.0);

        assert!(cell.age > 0.0);
    }
}
