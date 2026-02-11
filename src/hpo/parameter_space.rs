//! Parameter Space Manager - Generate and mutate simulation configurations
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Spectrum ratios (space/time, time/space), veil thickness (0.0-1.0),
//! archetypical mind configuration (10 or 22 archetypes), density transition rates,
//! catalyst generation rates"

use crate::hpo::types::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use rand::SeedableRng;

/// Parameter Space Manager for generating and mutating configurations
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Spectrum ratios, veil thickness, archetypical mind configuration,
/// density transition rates, catalyst generation rates"
#[derive(Debug, Clone)]
pub struct ParameterSpaceManager {
    /// Parameter space definition
    parameter_space: ParameterSpace,

    /// Random number generator
    rng_seed: u64,
}

impl ParameterSpaceManager {
    /// Create a new parameter space manager with default ranges
    pub fn new() -> Self {
        Self {
            parameter_space: ParameterSpace::new(),
            rng_seed: 42,
        }
    }

    /// Create a new parameter space manager with custom parameter space
    pub fn with_parameter_space(parameter_space: ParameterSpace) -> Self {
        Self {
            parameter_space,
            rng_seed: 42,
        }
    }

    /// Set the random seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng_seed = seed;
        self
    }

    /// Generate a random configuration within the parameter space
    ///
    /// # Arguments
    /// * `config_id` - Unique identifier for the configuration
    ///
    /// # Returns
    /// Random simulation configuration
    pub fn generate_random_config(&self, config_id: ConfigId) -> SimulationConfig {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.rng_seed + config_id);

        // Generate random space/time ratio
        let space_time_ratio = rng.gen_range(
            self.parameter_space.space_time_ratio_range.0
                ..=self.parameter_space.space_time_ratio_range.1,
        );

        // Generate random veil thickness
        let veil_thickness = rng.gen_range(
            self.parameter_space.veil_thickness_range.0
                ..=self.parameter_space.veil_thickness_range.1,
        );

        // Generate random catalyst generation rate
        let catalyst_generation_rate = rng.gen_range(
            self.parameter_space.catalyst_generation_rate_range.0
                ..=self.parameter_space.catalyst_generation_rate_range.1,
        );

        // Generate random density transition rates
        let density_transition_rates: Vec<Float> = (0..8)
            .map(|_| {
                rng.gen_range(
                    self.parameter_space.density_transition_rate_range.0
                        ..=self.parameter_space.density_transition_rate_range.1,
                )
            })
            .collect();

        // Select random archetype configuration
        let num_archetypes = *self
            .parameter_space
            .archetype_configs
            .choose(&mut rng)
            .unwrap_or(&22);

        // Calculate time/space ratio (inverse of space/time)
        let time_space_ratio = 1.0 / space_time_ratio;

        SimulationConfig {
            config_id,
            num_entities: 128, // Default, can be customized
            num_steps: 500,    // Default, can be customized
            space_time_ratio,
            time_space_ratio,
            veil_thickness,
            num_archetypes,
            density_transition_rates,
            catalyst_generation_rate,
            seed: config_id as u64,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Generate a batch of random configurations
    ///
    /// # Arguments
    /// * `count` - Number of configurations to generate
    ///
    /// # Returns
    /// Vector of random simulation configurations
    pub fn generate_random_configs(&self, count: usize) -> Vec<SimulationConfig> {
        (0..count)
            .map(|i| self.generate_random_config(i as ConfigId))
            .collect()
    }

    /// Mutate a configuration for exploration
    ///
    /// # Arguments
    /// * `config` - Configuration to mutate
    /// * `mutation_rate` - Probability of mutating each parameter (0.0 to 1.0)
    ///
    /// # Returns
    /// Mutated configuration
    pub fn mutate(&self, config: &SimulationConfig, mutation_rate: Float) -> SimulationConfig {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.rng_seed + config.config_id);
        let mut mutated = config.clone();

        // Mutate space/time ratio
        if rng.gen::<Float>() < mutation_rate {
            mutated.space_time_ratio = rng.gen_range(
                self.parameter_space.space_time_ratio_range.0
                    ..=self.parameter_space.space_time_ratio_range.1,
            );
            mutated.time_space_ratio = 1.0 / mutated.space_time_ratio;
        }

        // Mutate veil thickness
        if rng.gen::<Float>() < mutation_rate {
            mutated.veil_thickness = rng.gen_range(
                self.parameter_space.veil_thickness_range.0
                    ..=self.parameter_space.veil_thickness_range.1,
            );
        }

        // Mutate catalyst generation rate
        if rng.gen::<Float>() < mutation_rate {
            mutated.catalyst_generation_rate = rng.gen_range(
                self.parameter_space.catalyst_generation_rate_range.0
                    ..=self.parameter_space.catalyst_generation_rate_range.1,
            );
        }

        // Mutate density transition rates
        if rng.gen::<Float>() < mutation_rate {
            mutated.density_transition_rates = (0..8)
                .map(|_| {
                    rng.gen_range(
                        self.parameter_space.density_transition_rate_range.0
                            ..=self.parameter_space.density_transition_rate_range.1,
                    )
                })
                .collect();
        }

        // Mutate archetype configuration (with lower probability)
        if rng.gen::<Float>() < mutation_rate * 0.3 {
            mutated.num_archetypes = *self
                .parameter_space
                .archetype_configs
                .choose(&mut rng)
                .unwrap_or(&22);
        }

        mutated.config_id += 1000; // Ensure unique ID
        mutated.seed = mutated.config_id as u64;

        mutated
    }

    /// Crossover two configurations
    ///
    /// # Arguments
    /// * `parent_a` - First parent configuration
    /// * `parent_b` - Second parent configuration
    ///
    /// # Returns
    /// Child configuration (combination of parents)
    pub fn crossover(
        &self,
        parent_a: &SimulationConfig,
        parent_b: &SimulationConfig,
    ) -> SimulationConfig {
        let mut rng = rand::rngs::StdRng::seed_from_u64(
            self.rng_seed + parent_a.config_id + parent_b.config_id,
        );

        // Crossover space/time ratio
        let space_time_ratio = if rng.gen::<bool>() {
            parent_a.space_time_ratio
        } else {
            parent_b.space_time_ratio
        };

        // Crossover veil thickness
        let veil_thickness = if rng.gen::<bool>() {
            parent_a.veil_thickness
        } else {
            parent_b.veil_thickness
        };

        // Crossover catalyst generation rate
        let catalyst_generation_rate = if rng.gen::<bool>() {
            parent_a.catalyst_generation_rate
        } else {
            parent_b.catalyst_generation_rate
        };

        // Crossover density transition rates (mix parents)
        let density_transition_rates: Vec<Float> = (0..8)
            .map(|i| {
                if rng.gen::<bool>() {
                    parent_a.density_transition_rates[i]
                } else {
                    parent_b.density_transition_rates[i]
                }
            })
            .collect();

        // Crossover archetype configuration
        let num_archetypes = if rng.gen::<bool>() {
            parent_a.num_archetypes
        } else {
            parent_b.num_archetypes
        };

        // Calculate time/space ratio
        let time_space_ratio = 1.0 / space_time_ratio;

        SimulationConfig {
            config_id: parent_a.config_id + parent_b.config_id,
            num_entities: if rng.gen::<bool>() {
                parent_a.num_entities
            } else {
                parent_b.num_entities
            },
            num_steps: if rng.gen::<bool>() {
                parent_a.num_steps
            } else {
                parent_b.num_steps
            },
            space_time_ratio,
            time_space_ratio,
            veil_thickness,
            num_archetypes,
            density_transition_rates,
            catalyst_generation_rate,
            seed: (parent_a.config_id + parent_b.config_id) as u64,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Generate a new generation of configurations from a population
    ///
    /// # Arguments
    /// * `population` - Current population of simulation results
    /// * `population_size` - Desired size of new generation
    /// * `selection_params` - Selection algorithm parameters
    ///
    /// # Returns
    /// Vector of configurations for the next generation
    pub fn generate_next_generation(
        &self,
        population: &[SimulationResult],
        population_size: usize,
        selection_params: &SelectionParameters,
    ) -> Vec<SimulationConfig> {
        if population.is_empty() {
            return self.generate_random_configs(population_size);
        }

        let mut next_generation = Vec::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.rng_seed);

        // Keep top elite (elitism)
        let elite_count = (population_size as Float * selection_params.elitism_rate) as usize;
        let mut sorted_population = population.to_vec();
        sorted_population.sort_by(|a, b| {
            b.fitness_score
                .partial_cmp(&a.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for i in 0..elite_count.min(sorted_population.len()) {
            next_generation.push(sorted_population[i].config.clone());
        }

        // Generate rest through crossover and mutation
        while next_generation.len() < population_size {
            let parent_a = sorted_population
                .choose(&mut rng)
                .expect("Population should not be empty");

            let parent_b = sorted_population
                .choose(&mut rng)
                .expect("Population should not be empty");

            let mut child = if rng.gen::<Float>() < selection_params.crossover_rate {
                self.crossover(&parent_a.config, &parent_b.config)
            } else {
                parent_a.config.clone()
            };

            // Apply mutation
            if rng.gen::<Float>() < selection_params.mutation_rate {
                child = self.mutate(&child, selection_params.mutation_rate);
            }

            next_generation.push(child);
        }

        next_generation
    }

    /// Validate that a configuration is within the parameter space
    ///
    /// # Arguments
    /// * `config` - Configuration to validate
    ///
    /// # Returns
    /// True if configuration is valid
    pub fn is_valid(&self, config: &SimulationConfig) -> bool {
        self.parameter_space.is_valid(config)
    }

    /// Get the parameter space
    pub fn parameter_space(&self) -> &ParameterSpace {
        &self.parameter_space
    }

    /// Set a custom parameter space
    pub fn set_parameter_space(&mut self, parameter_space: ParameterSpace) {
        self.parameter_space = parameter_space;
    }
}

impl Default for ParameterSpaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_space_manager_creation() {
        let manager = ParameterSpaceManager::new();
        assert_eq!(manager.rng_seed, 42);
    }

    #[test]
    fn test_generate_random_config() {
        let manager = ParameterSpaceManager::new();
        let config = manager.generate_random_config(1);

        assert_eq!(config.config_id, 1);
        assert!(manager.parameter_space().is_valid(&config));
        assert_eq!(config.time_space_ratio, 1.0 / config.space_time_ratio);
    }

    #[test]
    fn test_generate_random_configs() {
        let manager = ParameterSpaceManager::new();
        let configs = manager.generate_random_configs(10);

        assert_eq!(configs.len(), 10);
        for (i, config) in configs.iter().enumerate() {
            assert_eq!(config.config_id, i as ConfigId);
            assert!(manager.is_valid(config));
        }
    }

    #[test]
    fn test_mutate() {
        let manager = ParameterSpaceManager::new();
        let config = SimulationConfig::new(1);

        let mutated = manager.mutate(&config, 0.5);

        assert_ne!(mutated.config_id, config.config_id);
        assert!(manager.is_valid(&mutated));
    }

    #[test]
    fn test_crossover() {
        let manager = ParameterSpaceManager::new();

        let parent_a = SimulationConfig::builder()
            .with_config_id(1)
            .with_space_time_ratio(2.0)
            .with_veil_thickness(0.3)
            .build()
            .unwrap();

        let parent_b = SimulationConfig::builder()
            .with_config_id(2)
            .with_space_time_ratio(1.5)
            .with_veil_thickness(0.7)
            .build()
            .unwrap();

        let child = manager.crossover(&parent_a, &parent_b);

        assert_eq!(child.config_id, 3);
        assert!(child.space_time_ratio == 2.0 || child.space_time_ratio == 1.5);
        assert!(child.veil_thickness == 0.3 || child.veil_thickness == 0.7);
        assert!(manager.is_valid(&child));
    }

    #[test]
    fn test_generate_next_generation() {
        let manager = ParameterSpaceManager::new();
        let selection_params = SelectionParameters::new();

        // Create a mock population
        let mut population = Vec::new();
        for i in 0..10 {
            let result = SimulationResult {
                simulation_id: i as SimulationId,
                config: SimulationConfig::new(i as ConfigId),
                completed: true,
                steps_executed: 100,
                final_coherence: 0.5 + i as Float * 0.05,
                average_coherence: 0.5 + i as Float * 0.05,
                coherence_history: vec![0.5 + i as Float * 0.05; 100],
                energy_conservation_error: 0.05,
                entities_evolved: 10,
                entities_harvested: 5,
                emergence_metrics: EmergenceMetrics {
                    biological_score: 0.5,
                    noospheric_score: 0.5,
                    gaia_score: 0.5,
                    overall_score: 0.5,
                },
                stability_metrics: StabilityMetrics {
                    coherence_stability: 0.8,
                    energy_balance: 0.9,
                    resilience: 1.0,
                    overall_score: 0.9,
                },
                complexity_metrics: ComplexityMetrics {
                    diversity_score: 0.6,
                    integration_score: 0.7,
                    depth_score: 0.5,
                    overall_score: 0.6,
                },
                fitness_score: 0.5 + i as Float * 0.05,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            };
            population.push(result);
        }

        let next_gen = manager.generate_next_generation(&population, 20, &selection_params);

        assert_eq!(next_gen.len(), 20);
        for config in &next_gen {
            assert!(manager.is_valid(config));
        }
    }

    #[test]
    fn test_is_valid() {
        let manager = ParameterSpaceManager::new();
        let config = SimulationConfig::new(1);

        assert!(manager.is_valid(&config));

        // Test invalid config
        let mut invalid_config = config.clone();
        invalid_config.space_time_ratio = 10.0;
        assert!(!manager.is_valid(&invalid_config));
    }
}
