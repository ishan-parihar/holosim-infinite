//! Evolution Engine - Species, Natural Selection, Speciation
//!
//! This module implements species-level evolution:
//! - Population dynamics with carrying capacity
//! - Natural selection based on fitness
//! - Mutation and genetic variation
//! - Speciation events
//! - Extinction events
//!
//! From V4 Roadmap Phase 4: "Biological Simulation Engine"
//! Species evolve through selection, not just description.

use crate::biology::organism_lifecycle::{
    BodyPlan, Organism, OrganismEnvironment, OrganismManager,
};
use crate::holographic::field_address::HolographicAddress;
use rand::Rng;
use std::collections::HashMap;

// ============================================================================
// Species ID
// ============================================================================

/// Unique species identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u64);

// ============================================================================
// Trophic Level
// ============================================================================

/// Trophic level in ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrophicLevel {
    /// Producer (plants, algae)
    Producer,
    /// Primary consumer (herbivore)
    PrimaryConsumer,
    /// Secondary consumer (carnivore)
    SecondaryConsumer,
    /// Tertiary consumer (apex predator)
    TertiaryConsumer,
    /// Decomposer
    Decomposer,
}

// ============================================================================
// Species
// ============================================================================

/// A species - group of organisms that can interbreed
#[derive(Debug, Clone)]
pub struct Species {
    /// Unique identifier
    pub id: SpeciesId,
    /// Species name
    pub name: String,
    /// Body plan template
    pub body_plan: BodyPlan,
    /// Trophic level
    pub trophic_level: TrophicLevel,
    /// Habitat preference (0.0 = any, 1.0 = specialized)
    pub habitat_specialization: f64,
    /// Climate tolerance range
    pub temperature_range: (f64, f64), // (min, max) in Kelvin
    /// Genetic diversity (0.0-1.0)
    pub genetic_diversity: f64,
    /// Origin time
    pub origin_time: f64,
    /// Parent species (for evolved species)
    pub parent: Option<SpeciesId>,
}

impl Species {
    /// Create a new species
    pub fn new(
        id: SpeciesId,
        name: &str,
        body_plan: BodyPlan,
        trophic_level: TrophicLevel,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            body_plan,
            trophic_level,
            habitat_specialization: 0.5,
            temperature_range: (250.0, 320.0), // Wide tolerance
            genetic_diversity: 0.5,
            origin_time: 0.0,
            parent: None,
        }
    }

    /// Create a default plant species
    pub fn plant_species(id: SpeciesId, name: &str) -> Self {
        Self::new(id, name, BodyPlan::plant(1.0), TrophicLevel::Producer)
    }

    /// Create a default animal species
    pub fn animal_species(
        id: SpeciesId,
        name: &str,
        trophic_level: TrophicLevel,
        mass: f64,
    ) -> Self {
        Self::new(id, name, BodyPlan::simple_animal(mass), trophic_level)
    }
}

// ============================================================================
// Population
// ============================================================================

/// Population of a species in a region
#[derive(Debug, Clone)]
pub struct Population {
    /// Species ID
    pub species_id: SpeciesId,
    /// Current count
    pub count: usize,
    /// Carrying capacity
    pub carrying_capacity: usize,
    /// Growth rate (r)
    pub growth_rate: f64,
    /// Average fitness
    pub average_fitness: f64,
    /// Geographic range (addresses)
    pub range: Vec<HolographicAddress>,
    /// Age distribution
    pub age_distribution: (f64, f64, f64), // (young, adult, old)
}

impl Population {
    /// Create a new population
    pub fn new(species_id: SpeciesId, initial_count: usize, capacity: usize) -> Self {
        Self {
            species_id,
            count: initial_count,
            carrying_capacity: capacity,
            growth_rate: 0.1,
            average_fitness: 0.5,
            range: Vec::new(),
            age_distribution: (0.3, 0.5, 0.2),
        }
    }

    /// Logistic growth
    pub fn grow(&mut self, dt: f64, environment: &PopulationEnvironment) {
        let n = self.count as f64;
        let k = self.carrying_capacity as f64;
        let r = self.growth_rate;

        // Logistic growth: dN/dt = rN(1 - N/K)
        let environmental_factor = environment.habitat_quality * environment.climate_suitability;
        let effective_r = r * environmental_factor;

        let growth = effective_r * n * (1.0 - n / k);
        self.count = (n + growth * dt).max(0.0) as usize;

        // Clamp to carrying capacity
        self.count = self.count.min(self.carrying_capacity);
    }
}

/// Environment for populations
#[derive(Debug, Clone)]
pub struct PopulationEnvironment {
    /// Habitat quality (0.0-1.0)
    pub habitat_quality: f64,
    /// Climate suitability (0.0-1.0)
    pub climate_suitability: f64,
    /// Food availability (0.0-1.0)
    pub food_availability: f64,
    /// Predator pressure (0.0-1.0)
    pub predator_pressure: f64,
    /// Disease prevalence (0.0-1.0)
    pub disease_prevalence: f64,
}

impl Default for PopulationEnvironment {
    fn default() -> Self {
        Self {
            habitat_quality: 0.8,
            climate_suitability: 0.8,
            food_availability: 0.7,
            predator_pressure: 0.2,
            disease_prevalence: 0.1,
        }
    }
}

// ============================================================================
// Evolution Events
// ============================================================================

/// Extinction event
#[derive(Debug, Clone)]
pub struct ExtinctionEvent {
    pub species_id: SpeciesId,
    pub cause: ExtinctionCause,
    pub time: f64,
    pub population_at_extinction: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtinctionCause {
    ClimateChange,
    HabitatLoss,
    Predation,
    Disease,
    Competition,
    Catastrophe,
    HumanActivity,
}

/// Speciation event
#[derive(Debug, Clone)]
pub struct SpeciationEvent {
    pub parent_species: SpeciesId,
    pub new_species: SpeciesId,
    pub cause: SpeciationCause,
    pub time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciationCause {
    GeographicIsolation,
    EcologicalDivergence,
    SexualSelection,
    MutationAccumulation,
}

// ============================================================================
// Evolution Engine
// ============================================================================

/// Evolution engine - species-level dynamics
#[derive(Debug)]
pub struct EvolutionEngine {
    /// All known species
    pub species: HashMap<SpeciesId, Species>,
    /// Populations by species
    pub populations: HashMap<SpeciesId, Population>,
    /// Extinction log
    pub extinction_log: Vec<ExtinctionEvent>,
    /// Speciation log
    pub speciation_log: Vec<SpeciationEvent>,
    /// Next species ID
    next_species_id: u64,
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EvolutionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            species: HashMap::new(),
            populations: HashMap::new(),
            extinction_log: Vec::new(),
            speciation_log: Vec::new(),
            next_species_id: 0,
        };

        // Initialize with some basic species
        engine.initialize_ecosystem();

        engine
    }

    /// Initialize a basic ecosystem
    fn initialize_ecosystem(&mut self) {
        // Plants
        let grass = Species::plant_species(SpeciesId(0), "Grass");
        let tree = Species::plant_species(SpeciesId(1), "Oak Tree");

        self.species.insert(grass.id, grass);
        self.species.insert(tree.id, tree);

        self.populations
            .insert(SpeciesId(0), Population::new(SpeciesId(0), 10000, 50000));
        self.populations
            .insert(SpeciesId(1), Population::new(SpeciesId(1), 1000, 5000));

        // Herbivores
        let rabbit =
            Species::animal_species(SpeciesId(2), "Rabbit", TrophicLevel::PrimaryConsumer, 2.0);
        let deer =
            Species::animal_species(SpeciesId(3), "Deer", TrophicLevel::PrimaryConsumer, 50.0);

        self.species.insert(rabbit.id, rabbit.clone());
        self.species.insert(deer.id, deer.clone());

        self.populations
            .insert(rabbit.id, Population::new(rabbit.id, 500, 2000));
        self.populations
            .insert(deer.id, Population::new(deer.id, 100, 500));

        // Carnivores
        let wolf =
            Species::animal_species(SpeciesId(4), "Wolf", TrophicLevel::SecondaryConsumer, 40.0);

        self.species.insert(wolf.id, wolf);
        self.populations
            .insert(SpeciesId(4), Population::new(SpeciesId(4), 20, 100));

        self.next_species_id = 5;
    }

    /// Main tick - population dynamics, selection, speciation
    pub fn tick(&mut self, environment: &PopulationEnvironment, dt: f64) {
        // Phase 1: Population growth/decline
        for (species_id, population) in self.populations.iter_mut() {
            population.grow(dt, environment);
        }

        // Phase 2: Natural selection (fitness-based mortality)
        self.apply_natural_selection(environment);

        // Phase 3: Extinction check
        self.check_extinctions();

        // Phase 4: Speciation check
        self.check_speciation();
    }

    /// Apply natural selection
    fn apply_natural_selection(&mut self, environment: &PopulationEnvironment) {
        for (species_id, population) in self.populations.iter_mut() {
            // Fitness = survival and reproduction capability
            let mut fitness = environment.habitat_quality * environment.climate_suitability;

            // Food availability affects herbivores
            if let Some(species) = self.species.get(species_id) {
                match species.trophic_level {
                    TrophicLevel::Producer => {
                        fitness *= environment.habitat_quality;
                    }
                    TrophicLevel::PrimaryConsumer => {
                        fitness *= environment.food_availability;
                    }
                    TrophicLevel::SecondaryConsumer | TrophicLevel::TertiaryConsumer => {
                        fitness *= 1.0 - environment.predator_pressure;
                    }
                    TrophicLevel::Decomposer => {
                        fitness *= 1.0 - environment.disease_prevalence;
                    }
                }
            }

            population.average_fitness = fitness;

            // Selection pressure kills less fit individuals
            if fitness < 0.3 && population.count > 10 {
                let mortality = (0.3 - fitness) * 0.1;
                let deaths = (population.count as f64 * mortality) as usize;
                population.count = population.count.saturating_sub(deaths);
            }
        }
    }

    /// Check for extinctions
    fn check_extinctions(&mut self) {
        let mut extinct = Vec::new();

        for (species_id, population) in self.populations.iter() {
            if population.count == 0 {
                let event = ExtinctionEvent {
                    species_id: *species_id,
                    cause: ExtinctionCause::Competition,
                    time: 0.0,
                    population_at_extinction: 0,
                };
                self.extinction_log.push(event);
                extinct.push(*species_id);
            }
        }

        // Remove extinct populations
        for species_id in extinct {
            self.populations.remove(&species_id);
        }
    }

    /// Check for speciation
    fn check_speciation(&mut self) {
        let species_ids: Vec<SpeciesId> = self.populations.keys().cloned().collect();

        for species_id in species_ids {
            if let Some(population) = self.populations.get(&species_id) {
                // Speciation requires:
                // 1. Large population (genetic variation)
                // 2. Geographic isolation (simulated by random chance)
                // 3. Environmental pressure

                if population.count > 1000 && rand::thread_rng().gen::<f64>() < 0.0001 {
                    self.attempt_speciation(species_id);
                }
            }
        }
    }

    /// Attempt to create a new species
    fn attempt_speciation(&mut self, parent_id: SpeciesId) {
        if let Some(parent) = self.species.get(&parent_id).cloned() {
            let new_id = SpeciesId(self.next_species_id);
            self.next_species_id += 1;

            // Create variant with slight modifications
            let mut variant = parent.clone();
            variant.id = new_id;
            variant.name = format!("{}_variant_{}", parent.name, new_id.0);
            variant.parent = Some(parent_id);
            variant.genetic_diversity = 0.3; // New species starts with lower diversity

            // Slight body plan variation
            variant.body_plan.mass *= 0.8 + rand::thread_rng().gen::<f64>() * 0.4;

            self.species.insert(new_id, variant);

            // Create population for new species (small founding population)
            let parent_pop = self.populations.get(&parent_id).unwrap();
            let founding_size = parent_pop.count / 10;

            self.populations.insert(
                new_id,
                Population::new(new_id, founding_size, parent_pop.carrying_capacity / 2),
            );

            // Log speciation
            self.speciation_log.push(SpeciationEvent {
                parent_species: parent_id,
                new_species: new_id,
                cause: SpeciationCause::GeographicIsolation,
                time: 0.0,
            });
        }
    }

    /// Get species count
    pub fn species_count(&self) -> usize {
        self.species.len()
    }

    /// Get total population
    pub fn total_population(&self) -> usize {
        self.populations.values().map(|p| p.count).sum()
    }

    /// Get extinction rate
    pub fn extinction_rate(&self) -> f64 {
        if self.extinction_log.is_empty() {
            return 0.0;
        }

        self.extinction_log.len() as f64 / self.species.len() as f64
    }
}

// ============================================================================
// Integration with Organism Manager
// ============================================================================

impl EvolutionEngine {
    /// Sync organisms from population to organism manager
    pub fn sync_to_organisms(&self, organism_manager: &mut OrganismManager) {
        for (species_id, population) in &self.populations {
            // Sample organisms to create based on population
            let sample_size = (population.count as f64 * 0.01) as usize; // 1% sample
            let sample_size = sample_size.max(1).min(100);

            if let Some(species) = self.species.get(species_id) {
                let current_count = organism_manager
                    .organisms
                    .values()
                    .filter(|o| o.species_id == species_id.0)
                    .count();

                // Create organisms if below sample size
                if current_count < sample_size {
                    for _ in 0..(sample_size - current_count) {
                        let pos = HolographicAddress::default();
                        organism_manager.create_organism(
                            species_id.0,
                            species.body_plan.clone(),
                            pos,
                        );
                    }
                }
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_engine_creation() {
        let engine = EvolutionEngine::new();
        assert!(engine.species_count() > 0);
    }

    #[test]
    fn test_population_growth() {
        let mut engine = EvolutionEngine::new();
        let env = PopulationEnvironment::default();

        let initial = engine.populations.get(&SpeciesId(0)).unwrap().count;
        engine.tick(&env, 100.0);

        let after = engine.populations.get(&SpeciesId(0)).unwrap().count;
        assert!(after >= initial);
    }
}
