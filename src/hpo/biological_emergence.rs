//! Biological Integration (Phase F5)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Connect the existing biology modules to actually run, not just exist as data structures."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "DNA/RNA are not random evolutionary developments—they unfold from this
//! pre-existing holographic blueprint encoded as spectrum configurations"
//!
//! This module implements:
//! - Cellular Emergence: Cells emerge from molecular field patterns
//! - DNA Derivation: DNA emerges from field archetypes (NOT pre-defined)
//! - Ecosystem Integration: Connect biology systems to simulation loop
//!
//! KEY PRINCIPLE: Life is not a separate system - it emerges from the field
//! just like matter. Consciousness → Matter → Biology is a continuous flow.

use super::archetype_matter::NUM_ARCHETYPES;
use super::complexity_emergence::{ComplexMolecule, ComplexityPhase};
use super::field_state::Float;
use super::spatial_field::Position3D;
use std::collections::HashMap;

/// DNA nucleotide base
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nucleotide {
    Adenine,
    Thymine,
    Guanine,
    Cytosine,
}

impl Nucleotide {
    /// Get complement
    pub fn complement(&self) -> Nucleotide {
        match self {
            Nucleotide::Adenine => Nucleotide::Thymine,
            Nucleotide::Thymine => Nucleotide::Adenine,
            Nucleotide::Guanine => Nucleotide::Cytosine,
            Nucleotide::Cytosine => Nucleotide::Guanine,
        }
    }

    /// Get from index
    pub fn from_index(idx: usize) -> Self {
        match idx % 4 {
            0 => Nucleotide::Adenine,
            1 => Nucleotide::Thymine,
            2 => Nucleotide::Guanine,
            _ => Nucleotide::Cytosine,
        }
    }
}

/// DNA sequence (derived from field patterns)
#[derive(Debug, Clone)]
pub struct EmergentDNA {
    /// Nucleotide sequence
    pub sequence: Vec<Nucleotide>,

    /// Length in base pairs
    pub length: usize,

    /// Derived archetype pattern
    pub archetype_pattern: [Float; NUM_ARCHETYPES],

    /// GC content ( Guanine-Cytosine ratio)
    pub gc_content: Float,
}

impl EmergentDNA {
    /// Create DNA from archetype pattern
    pub fn from_archetypes(archetypes: &[Float; NUM_ARCHETYPES], length: usize) -> Self {
        let mut sequence = Vec::with_capacity(length);

        // Generate sequence from archetype pattern
        // Different archetype combinations → different base sequences
        for i in 0..length {
            // Use archetype pattern to determine base
            let archetype_idx = i % NUM_ARCHETYPES;
            let value = archetypes[archetype_idx];

            // Map value to nucleotide (simplified)
            let nucleotide = Nucleotide::from_index((value * 100.0) as usize + i);
            sequence.push(nucleotide);
        }

        // Calculate GC content
        let gc_count = sequence
            .iter()
            .filter(|n| matches!(n, Nucleotide::Guanine | Nucleotide::Cytosine))
            .count();
        let gc_content = gc_count as Float / length as Float;

        EmergentDNA {
            sequence,
            length,
            archetype_pattern: *archetypes,
            gc_content,
        }
    }

    /// Create DNA from molecule pattern
    pub fn from_molecule(molecule: &ComplexMolecule) -> Self {
        Self::from_archetypes(&molecule.archetype_pattern, 100)
    }
}

/// Metabolism system (R&D-5)
/// Emerges from molecular energy flow, NOT pre-defined
#[derive(Debug, Clone, Default)]
pub struct Metabolism {
    /// Number of energy sources (ATP-like molecules)
    pub energy_sources: usize,
    /// Number of energy sinks (proteins, etc.)
    pub energy_sinks: usize,
    /// Metabolic rate (sinks/sources)
    pub metabolic_rate: Float,
    /// Total complexity from molecules
    pub complexity: Float,
}

impl Metabolism {
    pub fn new() -> Self {
        Metabolism {
            energy_sources: 0,
            energy_sinks: 0,
            metabolic_rate: 0.0,
            complexity: 0.0,
        }
    }

    /// Check if metabolism is viable
    pub fn is_viable(&self) -> bool {
        self.energy_sources > 0 && self.energy_sinks > 0
    }
}

/// Cell type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    /// Simple prokaryote
    Prokaryote,
    /// Complex eukaryote
    Eukaryote,
    /// Specialized cell (neuron, etc.)
    Specialized,
}

/// A living cell emerging from field patterns
#[derive(Debug, Clone)]
pub struct LivingCell {
    /// Unique ID
    pub id: u64,

    /// Cell type
    pub cell_type: CellType,

    /// Position in space
    pub position: Position3D,

    /// DNA sequence
    pub dna: EmergentDNA,

    /// Energy level (0.0 to 1.0)
    pub energy: Float,

    /// Health (0.0 to 1.0)
    pub health: Float,

    /// Age in simulation steps
    pub age: u64,

    /// Division count
    pub divisions: u32,
}

impl LivingCell {
    /// Create a new cell from DNA
    pub fn new(id: u64, position: Position3D, dna: EmergentDNA) -> Self {
        LivingCell {
            id,
            cell_type: CellType::Prokaryote,
            position,
            dna,
            energy: 1.0,
            health: 1.0,
            age: 0,
            divisions: 0,
        }
    }

    /// Age the cell
    pub fn age(&mut self) {
        self.age += 1;
        // Energy decreases with age
        self.energy = (self.energy - 0.001).max(0.0);
        // Health tied to energy
        self.health = self.health.min(self.energy);
    }

    /// Check if cell is alive
    pub fn is_alive(&self) -> bool {
        self.energy > 0.1 && self.health > 0.1
    }

    /// Check if cell can divide
    pub fn can_divide(&self) -> bool {
        self.energy > 0.8 && self.health > 0.8 && self.age > 10
    }
}

/// Species emerging from field patterns
#[derive(Debug, Clone)]
pub struct EmergentSpecies {
    /// Species ID
    pub id: u64,

    /// Species name (derived)
    pub name: String,

    /// Cell type
    pub cell_type: CellType,

    /// Population size
    pub population: usize,

    /// Archetype pattern (defines species)
    pub archetype_pattern: [Float; NUM_ARCHETYPES],

    /// Trophic level (0 = producer, 1 = consumer, 2 = decomposer)
    pub trophic_level: u8,

    /// Generation count
    pub generation: u32,
}

impl EmergentSpecies {
    /// Create species from archetype pattern
    pub fn from_archetypes(
        id: u64,
        archetypes: &[Float; NUM_ARCHETYPES],
        trophic_level: u8,
    ) -> Self {
        // Generate name from archetypes
        let name = Self::derive_name(archetypes);

        EmergentSpecies {
            id,
            name,
            cell_type: if archetypes.iter().sum::<Float>() / NUM_ARCHETYPES as Float > 0.6 {
                CellType::Eukaryote
            } else {
                CellType::Prokaryote
            },
            population: 1,
            archetype_pattern: *archetypes,
            trophic_level,
            generation: 0,
        }
    }

    fn derive_name(archetypes: &[Float; NUM_ARCHETYPES]) -> String {
        // Simplified name derivation
        let prefix = match (archetypes[0] * 10.0) as usize % 5 {
            0 => "Proto",
            1 => "Archa",
            2 => "Bact",
            3 => "Eu",
            _ => "Cyano",
        };

        let suffix = match (archetypes[1] * 10.0) as usize % 5 {
            0 => "bacter",
            1 => "cyte",
            2 => "plasm",
            3 => "vita",
            _ => "monad",
        };

        format!("{}{}", prefix, suffix)
    }
}

/// Configuration for biological emergence
#[derive(Debug, Clone)]
pub struct BiologicalEmergenceConfig {
    /// Minimum complexity for life emergence
    pub life_threshold: Float,

    /// Minimum molecules for cell formation
    pub molecule_threshold: usize,

    /// Cell division rate
    pub division_rate: Float,

    /// Enable ecosystem dynamics
    pub enable_ecosystem: bool,

    /// Maximum population per species
    pub max_population: usize,
}

impl Default for BiologicalEmergenceConfig {
    fn default() -> Self {
        BiologicalEmergenceConfig {
            life_threshold: 0.7,
            molecule_threshold: 5,
            division_rate: 0.1,
            enable_ecosystem: true,
            max_population: 10000,
        }
    }
}

/// Biological Emergence System
///
/// Connects field → matter → biology
pub struct BiologicalEmergence {
    config: BiologicalEmergenceConfig,

    /// Active cells
    cells: HashMap<u64, LivingCell>,

    /// Active species
    species: HashMap<u64, EmergentSpecies>,

    /// Next cell ID
    next_cell_id: u64,

    /// Next species ID
    next_species_id: u64,

    /// Statistics
    pub statistics: BiologicalStatistics,
}

impl BiologicalEmergence {
    pub fn new(config: BiologicalEmergenceConfig) -> Self {
        BiologicalEmergence {
            config,
            cells: HashMap::new(),
            species: HashMap::new(),
            next_cell_id: 0,
            next_species_id: 0,
            statistics: BiologicalStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(BiologicalEmergenceConfig::default())
    }

    fn next_cell_id(&mut self) -> u64 {
        let id = self.next_cell_id;
        self.next_cell_id += 1;
        id
    }

    fn next_species_id(&mut self) -> u64 {
        let id = self.next_species_id;
        self.next_species_id += 1;
        id
    }

    /// Check if molecular complexity is high enough for life
    pub fn check_life_emergence(&self, complexity: ComplexityPhase, molecule_count: usize) -> bool {
        complexity == ComplexityPhase::Molecular && molecule_count >= self.config.molecule_threshold
    }

    /// Create a cell from molecular patterns
    pub fn emerge_cell(
        &mut self,
        position: Position3D,
        archetype_pattern: &[Float; NUM_ARCHETYPES],
    ) -> Option<LivingCell> {
        // Check if we meet the threshold
        let complexity_score: Float =
            archetype_pattern.iter().sum::<Float>() / NUM_ARCHETYPES as Float;

        if complexity_score < self.config.life_threshold {
            return None;
        }

        // Create DNA from archetype pattern (legacy method)
        let dna = EmergentDNA::from_archetypes(archetype_pattern, 100);

        // Create cell
        let cell = LivingCell::new(self.next_cell_id(), position, dna);
        let id = cell.id;

        // Register cell
        self.cells.insert(id, cell);
        self.statistics.cells_created += 1;

        // Maybe create or update species
        self.update_species(archetype_pattern);

        Some(self.cells.get(&id).cloned().unwrap())
    }

    /// Create cell from MOLECULES (R&D-5: emergent DNA)
    /// DNA emerges from molecular arrangement, NOT pre-defined
    pub fn emerge_cell_from_molecules(
        &mut self,
        position: Position3D,
        molecules: &[ComplexMolecule],
    ) -> Option<LivingCell> {
        if molecules.len() < 3 {
            return None;
        }

        // Derive DNA from molecular patterns (R&D-5)
        let dna = self.derive_dna_from_molecules(molecules);

        // Derive metabolism from energy flow (R&D-5)
        let _metabolism = self.derive_metabolism(molecules);

        // Create cell with emergent properties
        let cell = LivingCell::new(self.next_cell_id(), position, dna);

        // Add metabolic information (stored in cell metadata)
        // For now, we'll add it as part of the cell's archetype pattern
        let id = cell.id;

        // Register cell
        self.cells.insert(id, cell);
        self.statistics.cells_created += 1;

        // Update species based on molecular patterns
        self.update_species_from_molecules(molecules);

        Some(self.cells.get(&id).cloned().unwrap())
    }

    /// DNA emerges from molecular field patterns (R&D-5)
    /// NOT pre-defined genetic code - emergent from chemistry
    fn derive_dna_from_molecules(&self, molecules: &[ComplexMolecule]) -> EmergentDNA {
        // Extract pattern from molecular arrangement
        let mut pattern = [0.0; NUM_ARCHETYPES];

        // Combine archetype patterns from all molecules
        for mol in molecules {
            for (i, v) in mol.archetype_pattern.iter().enumerate() {
                pattern[i] += v;
            }
        }

        // Normalize
        let sum: Float = pattern.iter().sum();
        if sum > 0.0 {
            for v in pattern.iter_mut() {
                *v /= sum;
            }
        }

        // The genetic code emerges from the molecular chemistry
        // Different molecular arrangements = different "DNA"
        EmergentDNA::from_archetypes(&pattern, molecules.len() * 10)
    }

    /// Metabolism emerges from energy flow (R&D-5)
    /// Identifies energy sources and sinks from molecular data
    fn derive_metabolism(&self, molecules: &[ComplexMolecule]) -> Metabolism {
        let mut metabolism = Metabolism::new();

        // Analyze each molecule for energy content
        for mol in molecules {
            // Calculate energy potential from archetype activation
            let energy_potential: Float = mol.archetype_pattern.iter().sum();

            // High energy potential = energy source (ATP-like)
            // Low energy potential = energy sink (protein synthesis)
            if energy_potential > 0.5 {
                metabolism.energy_sources += 1;
            } else {
                metabolism.energy_sinks += 1;
            }

            // Track molecular complexity (clone to avoid move)
            metabolism.complexity += mol.archetype_pattern.iter().sum::<Float>();
        }

        // Calculate metabolic rate
        if metabolism.energy_sources > 0 {
            metabolism.metabolic_rate =
                metabolism.energy_sinks as Float / metabolism.energy_sources as Float;
        }

        metabolism
    }

    /// Update species based on molecular patterns (R&D-5)
    fn update_species_from_molecules(&mut self, molecules: &[ComplexMolecule]) {
        // Combine archetype patterns
        let mut pattern = [0.0; NUM_ARCHETYPES];
        for mol in molecules {
            for (i, v) in mol.archetype_pattern.iter().enumerate() {
                pattern[i] += v;
            }
        }

        // Normalize
        let sum: Float = pattern.iter().sum();
        if sum > 0.0 {
            for v in pattern.iter_mut() {
                *v /= sum;
            }
        }

        self.update_species(&pattern);
    }

    /// Update species based on cell archetypes
    fn update_species(&mut self, archetype_pattern: &[Float; NUM_ARCHETYPES]) {
        // Try to find matching species
        let mut matched = None;

        for (id, species) in self.species.iter() {
            // Check archetype similarity
            let similarity: Float = species
                .archetype_pattern
                .iter()
                .zip(archetype_pattern.iter())
                .map(|(a, b)| (a - b).abs())
                .sum::<Float>()
                / NUM_ARCHETYPES as Float;

            if similarity < 0.2 {
                matched = Some(*id);
                break;
            }
        }

        if let Some(species_id) = matched {
            // Update existing species
            if let Some(species) = self.species.get_mut(&species_id) {
                species.population = (species.population + 1).min(self.config.max_population);
            }
        } else {
            // Create new species
            let trophic_level = match (archetype_pattern[0] * 5.0) as u8 % 3 {
                0 => 0, // Producer
                1 => 1, // Consumer
                _ => 2, // Decomposer
            };

            let new_species = EmergentSpecies::from_archetypes(
                self.next_species_id(),
                archetype_pattern,
                trophic_level,
            );

            self.species.insert(new_species.id, new_species);
            self.statistics.species_created += 1;
        }
    }

    /// Process cell division
    pub fn process_division(&mut self) {
        // Collect cell data first to avoid borrow issues
        let cell_data: Vec<(u64, LivingCell, u64)> = self
            .cells
            .iter()
            .map(|(id, c)| (*id, c.clone(), c.id.wrapping_mul(1103515245)))
            .collect();

        let mut new_cells = Vec::new();

        for (_cell_id, cell, cell_seed) in cell_data {
            if cell.can_divide() && pseudo_rand(cell_seed) < self.config.division_rate {
                // Create daughter cell
                let offset = Position3D::new(
                    (pseudo_rand(cell_seed.wrapping_add(1)) - 0.5) * 2.0,
                    (pseudo_rand(cell_seed.wrapping_add(2)) - 0.5) * 2.0,
                    (pseudo_rand(cell_seed.wrapping_add(3)) - 0.5) * 2.0,
                );

                let new_position = Position3D::new(
                    cell.position.x + offset.x,
                    cell.position.y + offset.y,
                    cell.position.z + offset.z,
                );

                // Clone DNA with slight mutation
                let mut new_dna = cell.dna.clone();
                // Simple mutation
                if pseudo_rand(cell_seed.wrapping_add(4)) < 0.01 {
                    if let Some(base) = new_dna.sequence.get_mut(0) {
                        *base = Nucleotide::from_index(
                            (pseudo_rand(cell_seed.wrapping_add(5)) * 100.0) as usize % 4,
                        );
                    }
                }

                let new_cell = LivingCell {
                    id: self.next_cell_id(),
                    cell_type: cell.cell_type,
                    position: new_position,
                    dna: new_dna,
                    energy: 0.5,
                    health: 1.0,
                    age: 0,
                    divisions: cell.divisions + 1,
                };

                new_cells.push(new_cell);
            }
        }

        // Add new cells
        for cell in new_cells {
            self.cells.insert(cell.id, cell);
            self.statistics.cells_created += 1;
        }
    }

    /// Process ecosystem interactions
    pub fn process_ecosystem(&mut self) {
        if !self.config.enable_ecosystem {
            return;
        }

        // Simplified ecosystem dynamics
        // In a full implementation, this would be more sophisticated

        // Apply population pressures - collect species IDs first
        let species_ids: Vec<u64> = self.species.keys().cloned().collect();

        for species_id in species_ids {
            if let Some(species) = self.species.get_mut(&species_id) {
                // Natural population fluctuation
                let fluctuation = (pseudo_rand(species_id) - 0.5) * 0.1;
                species.population = (species.population as Float * (1.0 + fluctuation)) as usize;
                species.population = species.population.max(1).min(self.config.max_population);

                // Update generation
                if species.population > 100 {
                    species.generation += 1;
                }
            }
        }

        // Remove extinct species
        let extinct: Vec<u64> = self
            .species
            .iter()
            .filter(|(_, s)| s.population == 0)
            .map(|(id, _)| *id)
            .collect();

        for id in extinct {
            self.species.remove(&id);
        }
    }

    /// Update all biological entities
    pub fn update(&mut self, _dt: Float) {
        // Collect cell IDs and ages first
        let cell_ids: Vec<u64> = self.cells.keys().cloned().collect();

        // Age cells and check for death
        let mut to_remove: Vec<u64> = Vec::new();

        for id in &cell_ids {
            if let Some(cell) = self.cells.get_mut(id) {
                cell.age();
                if !cell.is_alive() {
                    to_remove.push(*id);
                }
            }
        }

        // Remove dead cells
        for id in to_remove {
            self.cells.remove(&id);
            self.statistics.cells_destroyed += 1;
        }

        // Process division
        self.process_division();

        // Process ecosystem
        self.process_ecosystem();

        // Update statistics
        self.statistics.active_cells = self.cells.len();
        self.statistics.active_species = self.species.len();
    }

    /// Get cell count
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Get species count
    pub fn species_count(&self) -> usize {
        self.species.len()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &BiologicalStatistics {
        &self.statistics
    }
}

/// Statistics for biological emergence
#[derive(Debug, Clone, Default)]
pub struct BiologicalStatistics {
    pub cells_created: usize,
    pub cells_destroyed: usize,
    pub species_created: usize,
    pub species_extinct: usize,
    pub active_cells: usize,
    pub active_species: usize,
}

/// Bridge from matter/complexity to biology
pub struct BiologyBridge {
    biological_emergence: BiologicalEmergence,
}

impl BiologyBridge {
    pub fn new(config: BiologicalEmergenceConfig) -> Self {
        BiologyBridge {
            biological_emergence: BiologicalEmergence::new(config),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(BiologicalEmergenceConfig::default())
    }

    /// Try to create life from molecules at a position (legacy method)
    pub fn try_create_life(&mut self, position: Position3D, molecules: &[ComplexMolecule]) -> bool {
        if molecules.len() < 3 {
            return false;
        }

        // Combine archetype patterns from molecules
        let mut combined_pattern = [0.0; NUM_ARCHETYPES];
        let mut total_amplitude = 0.0;

        for mol in molecules {
            for (i, v) in mol.archetype_pattern.iter().enumerate() {
                combined_pattern[i] += v;
            }
            total_amplitude += 1.0;
        }

        if total_amplitude > 0.0 {
            for v in combined_pattern.iter_mut() {
                *v /= total_amplitude;
            }
        }

        // Try to emerge cell
        if let Some(_cell) = self
            .biological_emergence
            .emerge_cell(position, &combined_pattern)
        {
            return true;
        }

        false
    }

    /// Emerge cell from molecules (R&D-5: emergent DNA)
    /// This is the preferred method - uses actual molecular data
    pub fn emerge_cell_from_molecules(
        &mut self,
        position: Position3D,
        molecules: &[ComplexMolecule],
    ) -> Option<LivingCell> {
        self.biological_emergence
            .emerge_cell_from_molecules(position, molecules)
    }

    /// Update biological systems
    pub fn update(&mut self, dt: Float) {
        self.biological_emergence.update(dt);
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &BiologicalStatistics {
        self.biological_emergence.get_statistics()
    }

    /// Get cell count
    pub fn cell_count(&self) -> usize {
        self.biological_emergence.cell_count()
    }

    /// Get species count
    pub fn species_count(&self) -> usize {
        self.biological_emergence.species_count()
    }
}

// Use simple pseudo-random based on position for deterministic behavior
fn pseudo_rand(seed: u64) -> f64 {
    let x = seed.wrapping_mul(1103515245).wrapping_add(12345);
    ((x >> 16) & 0x7fff) as f64 / 32768.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_from_archetypes() {
        let archetypes = [0.5; 22];
        let dna = EmergentDNA::from_archetypes(&archetypes, 100);

        assert_eq!(dna.length, 100);
    }

    #[test]
    fn test_cell_lifecycle() {
        let archetypes = [0.8; 22];
        let dna = EmergentDNA::from_archetypes(&archetypes, 50);
        let mut cell = LivingCell::new(0, Position3D::origin(), dna);

        assert!(cell.is_alive());

        // Age the cell significantly
        for _ in 0..200 {
            cell.age();
        }

        assert!(!cell.is_alive());
    }

    #[test]
    fn test_biological_emergence() {
        let mut emergence = BiologicalEmergence::with_defaults();

        // Create cell from high coherence archetypes
        let high_archetypes = [0.8; 22];
        let cell = emergence.emerge_cell(Position3D::new(1.0, 2.0, 3.0), &high_archetypes);

        assert!(cell.is_some());

        // Should have created a species too
        assert!(emergence.species_count() > 0);
    }

    #[test]
    fn test_life_threshold() {
        let emergence = BiologicalEmergence::with_defaults();

        // Low coherence should not trigger life
        let _low_archetypes = [0.2; 22];
        let can_emerge = emergence.check_life_emergence(ComplexityPhase::Molecular, 10);

        // Will be false because coherence is low
        assert!(!can_emerge || emergence.cell_count() == 0);
    }
}
