//! Biology Pipeline — Unified orchestration of all biology subsystems.
//!
//! `BiologyPipeline` wraps every biology engine and exposes a single
//! `tick()` call that the `SimulationRunner` can invoke each step.
//!
//! Tick order (data flows bottom-up):
//! 1. Molecular field — atoms → molecules
//! 2. Cell engine     — metabolism, division, death
//! 3. Organism manager — multicellular lifecycle
//! 4. Evolution engine — population dynamics, speciation
//! 5. Neural field    — sensory processing, attractors
//! 6. Consciousness   — archetype processing, polarity
//!
//! From V5 Phase 5: "Unified biology API for simulation integration"

use crate::biology::cell_engine::{CellEngine, CellEnvironment};
use crate::biology::consciousness_tick::{ConsciousnessTickEngine, ConsciousnessTickInput};
use crate::biology::evolution_engine::{EvolutionEngine, PopulationEnvironment};
use crate::biology::molecular_field::MolecularField;
use crate::biology::neural_field::{NeuralField, SensoryInput};
use crate::biology::organism_lifecycle::{OrganismEnvironment, OrganismManager};
use crate::evolution_density_octave::density_octave::Density;
use crate::types::Float;

use super::BiologicalConfig;

// ============================================================================
// Result & Stats types
// ============================================================================

/// Aggregated result of one biology tick.
#[derive(Debug, Default)]
pub struct BiologyTickResult {
    pub tick_count: u64,
    pub cell_count: usize,
    pub organism_count: usize,
    pub species_count: usize,
    pub molecule_count: usize,
    pub neuron_count: usize,
    pub cell_divisions: usize,
    pub organism_deaths: usize,
    pub average_cell_health: Float,
    pub average_organism_health: Float,
}

/// Snapshot of current biology statistics.
#[derive(Debug, Clone, Default)]
pub struct BiologyStats {
    pub cell_count: usize,
    pub organism_count: usize,
    pub species_count: usize,
    pub molecule_count: usize,
    pub neuron_count: usize,
    pub average_cell_health: Float,
    pub average_organism_health: Float,
    pub molecular_bonds: usize,
    pub synapses: usize,
}

// ============================================================================
// BiologyPipeline
// ============================================================================

/// Unified biology pipeline wrapping all subsystems.
///
/// ```ignore
/// let mut pipeline = BiologyPipeline::new();
/// pipeline.register_entity(42, 3, 1.5);
/// let result = pipeline.tick(1.0, &Density::Third, &[0.5; 22]);
/// ```
pub struct BiologyPipeline {
    pub cell_engine: CellEngine,
    pub organism_manager: OrganismManager,
    pub evolution_engine: EvolutionEngine,
    pub molecular_field: MolecularField,
    pub neural_field: NeuralField,
    pub consciousness_engine: ConsciousnessTickEngine,
    tick_count: u64,
    entity_count: usize,
    config: BiologicalConfig,
}

impl Default for BiologyPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl BiologyPipeline {
    /// Create a new pipeline with default configuration.
    pub fn new() -> Self {
        Self::with_config(BiologicalConfig::default())
    }

    /// Create a new pipeline with custom configuration.
    pub fn with_config(config: BiologicalConfig) -> Self {
        Self {
            cell_engine: CellEngine::new(),
            organism_manager: OrganismManager::new(),
            evolution_engine: EvolutionEngine::new(),
            molecular_field: MolecularField::new(),
            neural_field: NeuralField::new(),
            consciousness_engine: ConsciousnessTickEngine::new(),
            tick_count: 0,
            entity_count: 0,
            config,
        }
    }

    /// Register an entity for consciousness tracking.
    pub fn register_entity(&mut self, entity_id: u64, density: u8, st_ratio: Float) {
        self.consciousness_engine
            .register_entity(entity_id, density, st_ratio);
        self.entity_count += 1;
    }

    /// Execute one full biology tick across all subsystems.
    ///
    /// # Arguments
    /// * `dt` — Time step (simulation units).
    /// * `global_density` — Current density band for archetype mapping.
    /// * `archetype_profile` — 22-element archetype activation vector.
    pub fn tick(
        &mut self,
        dt: Float,
        _global_density: &Density,
        archetype_profile: &[Float; 22],
    ) -> BiologyTickResult {
        // Derive environment parameters from archetype profile
        let temperature = archetype_profile[8] * 350.0 + 250.0; // body → 250-600K
        let energy = archetype_profile[14] * 100.0; // spirit → energy
        let _social_density = archetype_profile[15] * 10.0; // connection → pop density
        let danger = (1.0 - archetype_profile[6]) * 0.5; // wisdom⁻¹ → danger

        // ---- Step 1: Molecular field ----
        self.molecular_field.tick(dt);
        let _new_molecules = self.molecular_field.form_molecules();

        // ---- Step 2: Cell engine ----
        let cell_env = CellEnvironment {
            solar_radiation: archetype_profile[8] * 500.0,
            nutrient_density: energy / 100.0,
            temperature,
            radiation: danger * 0.3,
            toxin_level: danger * 0.2,
            water_availability: archetype_profile[9].max(0.1),
            oxygen_level: 0.21,
            detoxification_rate: archetype_profile[7].max(0.1),
            predator_presence: danger,
        };
        self.cell_engine.tick(&cell_env, dt);

        // ---- Step 3: Organism manager ----
        let organism_env = OrganismEnvironment {
            solar_radiation: archetype_profile[8] * 500.0,
            plant_density: energy / 100.0,
            prey_density: archetype_profile[10] * 0.5,
            predator_density: danger,
            carrion_density: 0.05,
            water_availability: archetype_profile[9].max(0.1),
            temperature,
            season: archetype_profile[11].max(0.0).min(1.0),
        };
        self.organism_manager.tick(&organism_env, dt);

        // ---- Step 4: Evolution engine ----
        let pop_env = PopulationEnvironment {
            habitat_quality: energy / 100.0,
            climate_suitability: (1.0 - danger).clamp(0.0, 1.0),
            food_availability: energy / 100.0,
            predator_pressure: danger,
            disease_prevalence: danger * 0.2,
        };
        self.evolution_engine.tick(&pop_env, dt);

        // ---- Step 5: Neural field ----
        let sensory_input = SensoryInput::new();
        self.neural_field.tick(dt, &sensory_input);

        // ---- Step 6: Consciousness tick ----
        let consciousness_inputs: std::collections::HashMap<u64, ConsciousnessTickInput> =
            std::collections::HashMap::new();
        let _consciousness_outputs = self.consciousness_engine.tick(&consciousness_inputs);

        // ---- Increment tick counter ----
        self.tick_count += 1;

        // ---- Build result ----
        BiologyTickResult {
            tick_count: self.tick_count,
            cell_count: self.cell_engine.total_cells(),
            organism_count: self.organism_manager.total(),
            species_count: self.evolution_engine.species_count(),
            molecule_count: self.molecular_field.molecules().len(),
            neuron_count: self.neural_field.neuron_count(),
            cell_divisions: self.cell_engine.division_queue.len(),
            organism_deaths: self.organism_manager.total_died as usize,
            average_cell_health: self.cell_engine.average_health(),
            average_organism_health: self.organism_manager.average_health(),
        }
    }

    /// Return a snapshot of current biology statistics.
    pub fn stats(&self) -> BiologyStats {
        BiologyStats {
            cell_count: self.cell_engine.total_cells(),
            organism_count: self.organism_manager.total(),
            species_count: self.evolution_engine.species_count(),
            molecule_count: self.molecular_field.molecules().len(),
            neuron_count: self.neural_field.neuron_count(),
            average_cell_health: self.cell_engine.average_health(),
            average_organism_health: self.organism_manager.average_health(),
            molecular_bonds: self.molecular_field.stats().bonds_created as usize,
            synapses: self.neural_field.synapse_count(),
        }
    }

    /// Number of ticks executed so far.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Number of registered entities.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Access the biological configuration.
    pub fn config(&self) -> &BiologicalConfig {
        &self.config
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = BiologyPipeline::new();
        assert_eq!(pipeline.tick_count, 0);
    }

    #[test]
    fn test_pipeline_tick() {
        let mut pipeline = BiologyPipeline::new();
        let profile = [0.5; 22];
        let result = pipeline.tick(1.0, &Density::Third, &profile);
        assert_eq!(result.tick_count, 1);
    }
}
