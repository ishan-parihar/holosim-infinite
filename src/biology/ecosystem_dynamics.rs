// Ecosystem Dynamics - Species Interactions and Energy Flow
//
// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 3:
// "Ecosystem Dynamics - Simulate species interactions (predation, competition, cooperation),
// calculate energy flow (trophic levels), track population dynamics, simulate co-evolution"
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Entities as catalysts for each other"

use crate::biology::cellular_emergence::{Cell, CellType, Eukaryote, Prokaryote};
use crate::types::Float;
use rand::Rng;
use std::collections::HashMap;

/// Ecosystem Dynamics - Simulates species interactions and energy flow
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Simulate species interactions, calculate energy flow, track population dynamics,
/// simulate co-evolution"
#[derive(Debug, Clone)]
pub struct EcosystemDynamics {
    /// Species interactor
    species_interactor: SpeciesInteractor,

    /// Energy flow calculator
    energy_flow_calculator: EnergyFlowCalculator,

    /// Population tracker
    population_tracker: PopulationTracker,

    /// Co-evolution simulator
    co_evolution_simulator: CoEvolutionSimulator,

    /// System configuration
    config: EcosystemDynamicsConfig,
}

impl Default for EcosystemDynamics {
    fn default() -> Self {
        Self::new()
    }
}

impl EcosystemDynamics {
    /// Create a new ecosystem dynamics system
    pub fn new() -> Self {
        Self {
            species_interactor: SpeciesInteractor::new(),
            energy_flow_calculator: EnergyFlowCalculator::new(),
            population_tracker: PopulationTracker::new(),
            co_evolution_simulator: CoEvolutionSimulator::new(),
            config: EcosystemDynamicsConfig::default(),
        }
    }

    /// Create an ecosystem dynamics system with custom configuration
    pub fn with_config(config: EcosystemDynamicsConfig) -> Self {
        Self {
            species_interactor: SpeciesInteractor::new(),
            energy_flow_calculator: EnergyFlowCalculator::new(),
            population_tracker: PopulationTracker::new(),
            co_evolution_simulator: CoEvolutionSimulator::new(),
            config,
        }
    }

    /// Simulate interaction between two species
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Simulate species interactions"
    pub fn simulate_interaction(
        &self,
        species_a: &Species,
        species_b: &Species,
    ) -> InteractionResult {
        self.species_interactor
            .interact(species_a, species_b, &self.config)
    }

    /// Calculate energy flow in ecosystem
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Calculate energy flow (trophic levels)"
    pub fn calculate_energy_flow(&self, ecosystem: &Ecosystem) -> EnergyFlow {
        self.energy_flow_calculator
            .calculate(ecosystem, &self.config)
    }

    /// Track population dynamics
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Track population dynamics"
    pub fn track_population(&self, ecosystem: &Ecosystem) -> PopulationDynamics {
        self.population_tracker.track(ecosystem)
    }

    /// Simulate co-evolution
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Simulate co-evolution"
    pub fn simulate_co_evolution(&self, ecosystem: &Ecosystem) -> CoEvolutionResult {
        self.co_evolution_simulator
            .simulate(ecosystem, &self.config)
    }

    /// Get configuration
    pub fn config(&self) -> &EcosystemDynamicsConfig {
        &self.config
    }
}

/// Ecosystem Dynamics Configuration
#[derive(Debug, Clone)]
pub struct EcosystemDynamicsConfig {
    /// Interaction strength (0.0 to 1.0)
    pub interaction_strength: Float,

    /// Energy flow efficiency (0.0 to 1.0)
    pub energy_flow_efficiency: Float,

    /// Co-evolution rate (0.0 to 1.0)
    pub co_evolution_rate: Float,

    /// Competition intensity (0.0 to 1.0)
    pub competition_intensity: Float,

    /// Cooperation rate (0.0 to 1.0)
    pub cooperation_rate: Float,
}

impl Default for EcosystemDynamicsConfig {
    fn default() -> Self {
        Self {
            interaction_strength: 0.7,
            energy_flow_efficiency: 0.8,
            co_evolution_rate: 0.01,
            competition_intensity: 0.6,
            cooperation_rate: 0.3,
        }
    }
}

/// Species - Group of similar organisms
#[derive(Debug, Clone)]
pub struct Species {
    /// Species identifier
    pub species_id: String,

    /// Species name
    pub name: String,

    /// Cells in this species
    pub cells: Vec<Cell>,

    /// Trophic level
    pub trophic_level: TrophicLevel,

    /// Population size
    pub population_size: usize,

    /// Species traits
    pub traits: Vec<SpeciesTrait>,

    /// Interactions with other species
    pub interactions: HashMap<String, InteractionType>,
}

impl Species {
    /// Create a new species
    pub fn new(species_id: String, name: String, trophic_level: TrophicLevel) -> Self {
        Species {
            species_id,
            name,
            cells: Vec::new(),
            trophic_level,
            population_size: 0,
            traits: Vec::new(),
            interactions: HashMap::new(),
        }
    }

    /// Add cell to species
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
        self.population_size = self.cells.len();
    }

    /// Remove cell from species
    pub fn remove_cell(&mut self, cell_id: &str) -> Option<Cell> {
        if let Some(pos) = self.cells.iter().position(|c| c.cell_id == cell_id) {
            let cell = self.cells.remove(pos);
            self.population_size = self.cells.len();
            Some(cell)
        } else {
            None
        }
    }

    /// Get average health
    pub fn average_health(&self) -> Float {
        if self.cells.is_empty() {
            0.0
        } else {
            let total: Float = self.cells.iter().map(|c| c.health).sum();
            total / self.cells.len() as Float
        }
    }
}

/// Trophic Level - Position in food chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrophicLevel {
    Producer,          // Level 1: Plants, photosynthetic organisms
    PrimaryConsumer,   // Level 2: Herbivores
    SecondaryConsumer, // Level 3: Carnivores
    TertiaryConsumer,  // Level 4: Top predators
    Decomposer,        // Decomposers and detritivores
}

/// Species Trait
#[derive(Debug, Clone)]
pub struct SpeciesTrait {
    /// Trait identifier
    pub trait_id: String,

    /// Trait name
    pub name: String,

    /// Trait value (0.0 to 1.0)
    pub value: Float,

    /// Effect on fitness
    pub fitness_effect: Float,
}

/// Interaction Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    Predation,    // One species eats another
    Competition,  // Both species compete for resources
    Cooperation,  // Both species benefit
    Mutualism,    // Both species benefit strongly
    Parasitism,   // One benefits, one harmed
    Commensalism, // One benefits, one unaffected
}

/// Interaction Result
#[derive(Debug, Clone)]
pub struct InteractionResult {
    /// Species A involved
    pub species_a: String,

    /// Species B involved
    pub species_b: String,

    /// Interaction type
    pub interaction_type: InteractionType,

    /// Effect on species A (-1.0 to 1.0)
    pub effect_on_a: Float,

    /// Effect on species B (-1.0 to 1.0)
    pub effect_on_b: Float,

    /// Energy transferred
    pub energy_transferred: Float,

    /// Interaction success
    pub success: bool,
}

/// Ecosystem - Collection of species and their environment
#[derive(Debug, Clone)]
pub struct Ecosystem {
    /// Ecosystem identifier
    pub ecosystem_id: String,

    /// Species in ecosystem
    pub species: Vec<Species>,

    /// Total energy available
    pub total_energy: Float,

    /// Environmental conditions
    pub environment: EnvironmentalConditions,

    /// Ecosystem age (in simulation steps)
    pub age: usize,
}

impl Ecosystem {
    /// Create a new ecosystem
    pub fn new(ecosystem_id: String, total_energy: Float) -> Self {
        Ecosystem {
            ecosystem_id,
            species: Vec::new(),
            total_energy,
            environment: EnvironmentalConditions::default(),
            age: 0,
        }
    }

    /// Add species to ecosystem
    pub fn add_species(&mut self, species: Species) {
        self.species.push(species);
    }

    /// Get species by ID
    pub fn get_species(&self, species_id: &str) -> Option<&Species> {
        self.species.iter().find(|s| s.species_id == species_id)
    }

    /// Update ecosystem
    pub fn update(&mut self) {
        self.age += 1;
        // Energy naturally decreases
        self.total_energy = (self.total_energy * 0.99).max(0.0);
    }
}

/// Environmental Conditions
#[derive(Debug, Clone)]
pub struct EnvironmentalConditions {
    /// Temperature (0.0 to 1.0)
    pub temperature: Float,

    /// Humidity (0.0 to 1.0)
    pub humidity: Float,

    /// Light availability (0.0 to 1.0)
    pub light: Float,

    /// Resource availability (0.0 to 1.0)
    pub resources: Float,
}

impl Default for EnvironmentalConditions {
    fn default() -> Self {
        EnvironmentalConditions {
            temperature: 0.5,
            humidity: 0.5,
            light: 0.5,
            resources: 0.5,
        }
    }
}

/// Energy Flow - Energy transfer through ecosystem
#[derive(Debug, Clone)]
pub struct EnergyFlow {
    /// Energy at each trophic level
    pub energy_by_trophic_level: HashMap<TrophicLevel, Float>,

    /// Total energy flow
    pub total_flow: Float,

    /// Energy transfer efficiency
    pub transfer_efficiency: Float,

    /// Energy loss (heat, waste)
    pub energy_loss: Float,
}

/// Population Dynamics - Changes in population over time
#[derive(Debug, Clone)]
pub struct PopulationDynamics {
    /// Population sizes by species
    pub population_sizes: HashMap<String, usize>,

    /// Growth rates by species
    pub growth_rates: HashMap<String, Float>,

    /// Carrying capacities by species
    pub carrying_capacities: HashMap<String, usize>,

    /// Population stability (0.0 to 1.0)
    pub stability: Float,
}

/// Co-Evolution Result
#[derive(Debug, Clone)]
pub struct CoEvolutionResult {
    /// Species that co-evolved
    pub co_evolved_pairs: Vec<(String, String)>,

    /// Trait changes
    pub trait_changes: HashMap<String, Vec<SpeciesTrait>>,

    /// Evolutionary pressure
    pub evolutionary_pressure: Float,

    /// Co-evolution success
    pub success: bool,
}

// ============================================================================
// SUBSYSTEMS
// ============================================================================

/// Species Interactor
#[derive(Debug, Clone)]
struct SpeciesInteractor {
    interaction_matrix: HashMap<String, HashMap<String, InteractionType>>,
}

impl SpeciesInteractor {
    fn new() -> Self {
        Self {
            interaction_matrix: HashMap::new(),
        }
    }

    fn interact(
        &self,
        species_a: &Species,
        species_b: &Species,
        config: &EcosystemDynamicsConfig,
    ) -> InteractionResult {
        let mut rng = rand::thread_rng();

        // Determine interaction type based on trophic levels
        let interaction_type = self.determine_interaction_type(species_a, species_b);

        // Calculate effects based on interaction type
        let (effect_on_a, effect_on_b, energy_transferred) = match interaction_type {
            InteractionType::Predation => {
                // Determine predator and prey
                let (_predator, prey) =
                    if species_a.trophic_level as i32 > species_b.trophic_level as i32 {
                        (species_a, species_b)
                    } else {
                        (species_b, species_a)
                    };

                let energy = prey.average_health() * config.interaction_strength * 0.5;
                (0.2, -0.3, energy)
            }
            InteractionType::Competition => {
                let competition_intensity = config.competition_intensity;
                (
                    -competition_intensity * 0.2,
                    -competition_intensity * 0.2,
                    0.0,
                )
            }
            InteractionType::Cooperation => {
                let cooperation_benefit = config.cooperation_rate * 0.3;
                (cooperation_benefit, cooperation_benefit, 0.0)
            }
            InteractionType::Mutualism => (0.4, 0.4, 0.0),
            InteractionType::Parasitism => {
                let harm = config.interaction_strength * 0.3;
                (0.2, -harm, harm * 0.5)
            }
            InteractionType::Commensalism => (0.2, 0.0, 0.0),
        };

        // Interaction succeeds with probability based on interaction strength
        let success = rng.gen::<Float>() < config.interaction_strength;

        InteractionResult {
            species_a: species_a.species_id.clone(),
            species_b: species_b.species_id.clone(),
            interaction_type,
            effect_on_a,
            effect_on_b,
            energy_transferred,
            success,
        }
    }

    fn determine_interaction_type(
        &self,
        species_a: &Species,
        species_b: &Species,
    ) -> InteractionType {
        // Determine interaction based on trophic levels and traits
        let level_diff = (species_a.trophic_level as i32 - species_b.trophic_level as i32).abs();

        if level_diff >= 2 {
            InteractionType::Predation
        } else if level_diff == 1 {
            InteractionType::Predation
        } else if species_a.trophic_level == species_b.trophic_level {
            InteractionType::Competition
        } else {
            InteractionType::Cooperation
        }
    }
}

/// Energy Flow Calculator
#[derive(Debug, Clone)]
struct EnergyFlowCalculator {
    trophic_efficiency: Float,
}

impl EnergyFlowCalculator {
    fn new() -> Self {
        Self {
            trophic_efficiency: 0.1, // 10% efficiency (ecological rule of thumb)
        }
    }

    fn calculate(&self, ecosystem: &Ecosystem, config: &EcosystemDynamicsConfig) -> EnergyFlow {
        let mut energy_by_trophic_level = HashMap::new();
        let mut total_flow = 0.0;

        // Calculate energy at each trophic level
        for species in &ecosystem.species {
            let level_energy = species.average_health() * species.population_size as Float;
            *energy_by_trophic_level
                .entry(species.trophic_level)
                .or_insert(0.0) += level_energy;
            total_flow += level_energy;
        }

        // Calculate energy loss
        let energy_loss = total_flow * (1.0 - config.energy_flow_efficiency);

        EnergyFlow {
            energy_by_trophic_level,
            total_flow,
            transfer_efficiency: config.energy_flow_efficiency,
            energy_loss,
        }
    }
}

/// Population Tracker
#[derive(Debug, Clone)]
struct PopulationTracker {
    tracking_window: usize,
}

impl PopulationTracker {
    fn new() -> Self {
        Self {
            tracking_window: 100,
        }
    }

    fn track(&self, ecosystem: &Ecosystem) -> PopulationDynamics {
        let mut population_sizes = HashMap::new();
        let mut growth_rates = HashMap::new();
        let mut carrying_capacities = HashMap::new();

        for species in &ecosystem.species {
            population_sizes.insert(species.species_id.clone(), species.population_size);

            // Calculate growth rate based on health and environment
            let growth_rate = species.average_health() * ecosystem.environment.resources - 0.5;
            growth_rates.insert(species.species_id.clone(), growth_rate);

            // Estimate carrying capacity based on total energy
            let carrying_capacity = (ecosystem.total_energy / 100.0) as usize;
            carrying_capacities.insert(species.species_id.clone(), carrying_capacity.max(1));
        }

        // Calculate stability
        let stability = if ecosystem.species.is_empty() {
            0.0
        } else {
            let avg_health: Float = ecosystem
                .species
                .iter()
                .map(|s| s.average_health())
                .sum::<Float>()
                / ecosystem.species.len() as Float;
            avg_health
        };

        PopulationDynamics {
            population_sizes,
            growth_rates,
            carrying_capacities,
            stability,
        }
    }
}

/// Co-Evolution Simulator
#[derive(Debug, Clone)]
struct CoEvolutionSimulator {
    simulation_steps: usize,
}

impl CoEvolutionSimulator {
    fn new() -> Self {
        Self {
            simulation_steps: 1000,
        }
    }

    fn simulate(
        &self,
        ecosystem: &Ecosystem,
        config: &EcosystemDynamicsConfig,
    ) -> CoEvolutionResult {
        let mut co_evolved_pairs = Vec::new();
        let mut trait_changes = HashMap::new();
        let mut rng = rand::thread_rng();

        // Check for co-evolution between interacting species
        for species_a in &ecosystem.species {
            for species_b in &ecosystem.species {
                if species_a.species_id != species_b.species_id {
                    // Co-evolution occurs with probability based on co-evolution rate
                    if rng.gen::<Float>() < config.co_evolution_rate {
                        co_evolved_pairs
                            .push((species_a.species_id.clone(), species_b.species_id.clone()));

                        // Generate trait changes
                        let new_trait = SpeciesTrait {
                            trait_id: format!("trait-{}", uuid::Uuid::new_v4()),
                            name: "adaptation".to_string(),
                            value: rng.gen::<Float>(),
                            fitness_effect: rng.gen::<Float>() * 0.2,
                        };

                        trait_changes
                            .entry(species_a.species_id.clone())
                            .or_insert_with(Vec::new)
                            .push(new_trait);
                    }
                }
            }
        }

        let evolutionary_pressure = if co_evolved_pairs.is_empty() {
            0.0
        } else {
            co_evolved_pairs.len() as Float / ecosystem.species.len() as Float
        };

        let success = !co_evolved_pairs.is_empty();

        CoEvolutionResult {
            co_evolved_pairs,
            trait_changes,
            evolutionary_pressure,
            success,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::dna_system::DNA;
    use crate::entity_layer7::dna_encoding::DNAPattern;
    use crate::entity_layer7::{EvolutionaryStage, IndividualSpectrumConfiguration};
    use crate::spectrum::{ArchetypicalMind, ArchetypicalSystemType, SpectrumRatio, SpectrumSide};

    fn create_test_cell() -> Cell {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::cellular_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );
        let dna = DNA::from_blueprint(dna_pattern);
        Cell::new(CellType::Prokaryotic, dna)
    }

    #[test]
    fn test_ecosystem_dynamics_creation() {
        let dynamics = EcosystemDynamics::new();
        assert_eq!(dynamics.config().interaction_strength, 0.7);
        assert_eq!(dynamics.config().energy_flow_efficiency, 0.8);
    }

    #[test]
    fn test_species_creation() {
        let species = Species::new(
            "species-1".to_string(),
            "Test Species".to_string(),
            TrophicLevel::Producer,
        );
        assert_eq!(species.species_id, "species-1");
        assert_eq!(species.population_size, 0);
    }

    #[test]
    fn test_add_cell_to_species() {
        let mut species = Species::new(
            "species-1".to_string(),
            "Test Species".to_string(),
            TrophicLevel::Producer,
        );
        let cell = create_test_cell();
        species.add_cell(cell);

        assert_eq!(species.population_size, 1);
    }

    #[test]
    fn test_simulate_interaction() {
        let dynamics = EcosystemDynamics::new();

        let mut species_a = Species::new(
            "species-a".to_string(),
            "Species A".to_string(),
            TrophicLevel::SecondaryConsumer,
        );
        let mut species_b = Species::new(
            "species-b".to_string(),
            "Species B".to_string(),
            TrophicLevel::PrimaryConsumer,
        );

        species_a.add_cell(create_test_cell());
        species_b.add_cell(create_test_cell());

        let result = dynamics.simulate_interaction(&species_a, &species_b);
        assert!(matches!(
            result.interaction_type,
            InteractionType::Predation
        ));
    }

    #[test]
    fn test_ecosystem_creation() {
        let ecosystem = Ecosystem::new("eco-1".to_string(), 1000.0);
        assert_eq!(ecosystem.ecosystem_id, "eco-1");
        assert_eq!(ecosystem.total_energy, 1000.0);
        assert_eq!(ecosystem.age, 0);
    }

    #[test]
    fn test_calculate_energy_flow() {
        let dynamics = EcosystemDynamics::new();
        let mut ecosystem = Ecosystem::new("eco-1".to_string(), 1000.0);

        let mut producer = Species::new(
            "producer".to_string(),
            "Producer".to_string(),
            TrophicLevel::Producer,
        );
        for _ in 0..10 {
            producer.add_cell(create_test_cell());
        }
        ecosystem.add_species(producer);

        let energy_flow = dynamics.calculate_energy_flow(&ecosystem);
        assert!(energy_flow.total_flow > 0.0);
        assert!(energy_flow.transfer_efficiency > 0.0);
    }

    #[test]
    fn test_track_population() {
        let dynamics = EcosystemDynamics::new();
        let mut ecosystem = Ecosystem::new("eco-1".to_string(), 1000.0);

        let mut species = Species::new(
            "species-1".to_string(),
            "Species 1".to_string(),
            TrophicLevel::Producer,
        );
        for _ in 0..20 {
            species.add_cell(create_test_cell());
        }
        ecosystem.add_species(species);

        let dynamics_result = dynamics.track_population(&ecosystem);
        assert_eq!(dynamics_result.population_sizes.get("species-1"), Some(&20));
    }

    #[test]
    fn test_simulate_co_evolution() {
        let dynamics = EcosystemDynamics::new();
        let mut ecosystem = Ecosystem::new("eco-1".to_string(), 1000.0);

        let mut species_a = Species::new(
            "species-a".to_string(),
            "Species A".to_string(),
            TrophicLevel::PrimaryConsumer,
        );
        let mut species_b = Species::new(
            "species-b".to_string(),
            "Species B".to_string(),
            TrophicLevel::SecondaryConsumer,
        );

        species_a.add_cell(create_test_cell());
        species_b.add_cell(create_test_cell());

        ecosystem.add_species(species_a);
        ecosystem.add_species(species_b);

        let result = dynamics.simulate_co_evolution(&ecosystem);
        // Co-evolution is probabilistic, so just check result structure
        assert!(result.evolutionary_pressure >= 0.0);
    }

    #[test]
    fn test_ecosystem_update() {
        let mut ecosystem = Ecosystem::new("eco-1".to_string(), 1000.0);
        let initial_age = ecosystem.age;
        let initial_energy = ecosystem.total_energy;

        ecosystem.update();

        assert_eq!(ecosystem.age, initial_age + 1);
        assert!(ecosystem.total_energy < initial_energy);
    }

    #[test]
    fn test_species_average_health() {
        let mut species = Species::new(
            "species-1".to_string(),
            "Species 1".to_string(),
            TrophicLevel::Producer,
        );

        let mut cell1 = create_test_cell();
        cell1.health = 0.8;
        let mut cell2 = create_test_cell();
        cell2.health = 0.6;

        species.add_cell(cell1);
        species.add_cell(cell2);

        let avg_health = species.average_health();
        assert!((avg_health - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_remove_cell_from_species() {
        let mut species = Species::new(
            "species-1".to_string(),
            "Species 1".to_string(),
            TrophicLevel::Producer,
        );
        let cell = create_test_cell();
        let cell_id = cell.cell_id.clone();
        species.add_cell(cell);

        assert_eq!(species.population_size, 1);

        let removed = species.remove_cell(&cell_id);
        assert!(removed.is_some());
        assert_eq!(species.population_size, 0);
    }
}
