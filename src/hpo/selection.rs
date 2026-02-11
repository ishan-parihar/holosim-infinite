//! Selection Algorithm - Tournament selection with elitism and convergence detection
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Tournament selection, elitism (keep top 10%), mutation/crossover for parameter
//! exploration, convergence detection"

use crate::hpo::types::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;

/// Selection Algorithm for evolutionary optimization
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Tournament selection, elitism (keep top 10%), mutation/crossover for parameter
/// exploration, convergence detection"
#[derive(Debug, Clone)]
pub struct SelectionAlgorithm {
    /// Selection parameters
    params: SelectionParameters,

    /// Random seed
    rng_seed: u64,

    /// Best fitness from previous generation
    previous_best_fitness: Option<Float>,

    /// Number of generations without improvement
    generations_without_improvement: usize,
}

impl SelectionAlgorithm {
    /// Create a new selection algorithm with default parameters
    pub fn new() -> Self {
        Self {
            params: SelectionParameters::new(),
            rng_seed: 42,
            previous_best_fitness: None,
            generations_without_improvement: 0,
        }
    }

    /// Create a new selection algorithm with custom parameters
    pub fn with_params(params: SelectionParameters) -> Result<Self, ConfigError> {
        params.validate()?;

        Ok(Self {
            params,
            rng_seed: 42,
            previous_best_fitness: None,
            generations_without_improvement: 0,
        })
    }

    /// Set the random seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng_seed = seed;
        self
    }

    /// Perform tournament selection on a population
    ///
    /// # Arguments
    /// * `population` - Population of simulation results
    /// * `num_to_select` - Number of individuals to select
    ///
    /// # Returns
    /// Selected simulation results
    pub fn select(
        &mut self,
        population: &[SimulationResult],
        num_to_select: usize,
    ) -> Vec<SimulationResult> {
        if population.is_empty() {
            return vec![];
        }

        if num_to_select == 0 {
            return vec![];
        }

        let mut selected = Vec::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.rng_seed);

        // Select individuals using tournament selection
        for _ in 0..num_to_select {
            let winner = self.tournament_select(&mut rng, population);
            selected.push(winner);
        }

        selected
    }

    /// Perform tournament selection to select one individual
    ///
    /// # Arguments
    /// * `rng` - Random number generator
    /// * `population` - Population of simulation results
    ///
    /// # Returns
    /// Selected simulation result
    fn tournament_select<R: Rng>(
        &self,
        rng: &mut R,
        population: &[SimulationResult],
    ) -> SimulationResult {
        let mut best = population
            .iter()
            .choose(rng)
            .expect("Population should not be empty")
            .clone();

        // Run tournament
        for _ in 1..self.params.tournament_size {
            let contender = population
                .iter()
                .choose(rng)
                .expect("Population should not be empty");

            if contender.fitness_score > best.fitness_score {
                best = contender.clone();
            }
        }

        best
    }

    /// Select elite individuals (top N by fitness)
    ///
    /// # Arguments
    /// * `population` - Population of simulation results
    /// * `num_elite` - Number of elite individuals to select
    ///
    /// # Returns
    /// Elite simulation results
    pub fn select_elite(
        &self,
        population: &[SimulationResult],
        num_elite: usize,
    ) -> Vec<SimulationResult> {
        if population.is_empty() {
            return vec![];
        }

        let mut sorted = population.to_vec();
        sorted.sort_by(|a, b| {
            b.fitness_score
                .partial_cmp(&a.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        sorted.into_iter().take(num_elite).collect()
    }

    /// Calculate number of elite individuals based on elitism rate
    ///
    /// # Arguments
    /// * `population_size` - Size of the population
    ///
    /// # Returns
    /// Number of elite individuals
    pub fn calculate_elite_count(&self, population_size: usize) -> usize {
        ((population_size as Float) * self.params.elitism_rate).ceil() as usize
    }

    /// Check if the population has converged
    ///
    /// # Arguments
    /// * `population` - Current population
    /// * `improvement_threshold` - Minimum improvement to consider significant
    ///
    /// # Returns
    /// True if population has converged
    pub fn has_converged(
        &mut self,
        population: &[SimulationResult],
        improvement_threshold: Float,
    ) -> bool {
        if population.is_empty() {
            return false;
        }

        // Find best fitness in current population
        let current_best_fitness = population
            .iter()
            .map(|r| r.fitness_score)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        // Check if we've improved
        let improved = match self.previous_best_fitness {
            Some(previous_best_fitness) => {
                current_best_fitness - previous_best_fitness >= improvement_threshold
            }
            None => {
                // First generation, always "improved"
                true
            }
        };

        if improved {
            self.previous_best_fitness = Some(current_best_fitness);
            self.generations_without_improvement = 0;
            false
        } else {
            self.generations_without_improvement += 1;
            self.generations_without_improvement >= self.params.convergence_threshold
        }
    }

    /// Reset convergence tracking
    pub fn reset_convergence_tracking(&mut self) {
        self.previous_best_fitness = None;
        self.generations_without_improvement = 0;
    }

    /// Get the current best fitness
    pub fn best_fitness(&self) -> Option<Float> {
        self.previous_best_fitness
    }

    /// Get the number of generations without improvement
    pub fn generations_without_improvement(&self) -> usize {
        self.generations_without_improvement
    }

    /// Get the selection parameters
    pub fn params(&self) -> &SelectionParameters {
        &self.params
    }

    /// Set the selection parameters
    pub fn set_params(&mut self, params: SelectionParameters) -> Result<(), ConfigError> {
        params.validate()?;
        self.params = params;
        Ok(())
    }

    /// Calculate selection statistics for a population
    ///
    /// # Arguments
    /// * `population` - Population of simulation results
    ///
    /// # Returns
    /// Selection statistics
    pub fn calculate_statistics(&self, population: &[SimulationResult]) -> SelectionStatistics {
        if population.is_empty() {
            return SelectionStatistics {
                population_size: 0,
                best_fitness: 0.0,
                worst_fitness: 0.0,
                average_fitness: 0.0,
                fitness_std_dev: 0.0,
                elite_count: 0,
                convergence_status: ConvergenceStatus::NotConverged,
                generations_without_improvement: self.generations_without_improvement,
            };
        }

        let fitness_values: Vec<Float> = population.iter().map(|r| r.fitness_score).collect();

        let population_size = fitness_values.len();
        let best_fitness = fitness_values
            .iter()
            .cloned()
            .fold(Float::NEG_INFINITY, f64::max);
        let worst_fitness = fitness_values
            .iter()
            .cloned()
            .fold(Float::INFINITY, f64::min);
        let average_fitness = fitness_values.iter().sum::<Float>() / population_size as Float;

        // Calculate standard deviation
        let variance: Float = fitness_values
            .iter()
            .map(|x| (x - average_fitness).powi(2))
            .sum::<Float>()
            / population_size as Float;
        let fitness_std_dev = variance.sqrt();

        let elite_count = self.calculate_elite_count(population_size);

        let convergence_status =
            if self.generations_without_improvement >= self.params.convergence_threshold {
                ConvergenceStatus::Converged
            } else if self.generations_without_improvement > self.params.convergence_threshold / 2 {
                ConvergenceStatus::Converging
            } else {
                ConvergenceStatus::NotConverged
            };

        SelectionStatistics {
            population_size,
            best_fitness,
            worst_fitness,
            average_fitness,
            fitness_std_dev,
            elite_count,
            convergence_status,
            generations_without_improvement: self.generations_without_improvement,
        }
    }
}

impl Default for SelectionAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

/// Convergence status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceStatus {
    /// Population is not converged
    NotConverged,

    /// Population is converging
    Converging,

    /// Population has converged
    Converged,
}

/// Selection statistics
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionStatistics {
    /// Size of the population
    pub population_size: usize,

    /// Best fitness in the population
    pub best_fitness: Float,

    /// Worst fitness in the population
    pub worst_fitness: Float,

    /// Average fitness of the population
    pub average_fitness: Float,

    /// Standard deviation of fitness
    pub fitness_std_dev: Float,

    /// Number of elite individuals
    pub elite_count: usize,

    /// Convergence status
    pub convergence_status: ConvergenceStatus,

    /// Number of generations without improvement
    pub generations_without_improvement: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_population(count: usize) -> Vec<SimulationResult> {
        (0..count)
            .map(|i| SimulationResult {
                simulation_id: i as SimulationId,
                config: SimulationConfig::new(i as ConfigId),
                completed: true,
                steps_executed: 100,
                final_coherence: 0.5 + i as Float * 0.01,
                average_coherence: 0.5 + i as Float * 0.01,
                coherence_history: vec![0.5 + i as Float * 0.01; 100],
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
                fitness_score: 0.5 + i as Float * 0.01,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            })
            .collect()
    }

    #[test]
    fn test_selection_algorithm_creation() {
        let algorithm = SelectionAlgorithm::new();
        assert_eq!(algorithm.params.tournament_size, 5);
    }

    #[test]
    fn test_selection_algorithm_with_params() {
        let params = SelectionParameters::new();
        let algorithm = SelectionAlgorithm::with_params(params.clone()).unwrap();
        assert_eq!(algorithm.params.tournament_size, params.tournament_size);
    }

    #[test]
    fn test_select() {
        let mut algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        let selected = algorithm.select(&population, 5);
        assert_eq!(selected.len(), 5);
    }

    #[test]
    fn test_select_elite() {
        let algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        let elite = algorithm.select_elite(&population, 3);
        assert_eq!(elite.len(), 3);

        // Elite should have higher fitness
        assert!(elite[0].fitness_score > elite[1].fitness_score);
        assert!(elite[1].fitness_score > elite[2].fitness_score);
    }

    #[test]
    fn test_calculate_elite_count() {
        let algorithm = SelectionAlgorithm::new();

        let elite_count = algorithm.calculate_elite_count(100);
        assert_eq!(elite_count, 10); // 10% of 100
    }

    #[test]
    fn test_has_converged_first_generation() {
        let mut algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        assert!(!algorithm.has_converged(&population, 0.001));
    }

    #[test]
    fn test_has_converged_with_improvement() {
        let mut algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        // First generation
        assert!(!algorithm.has_converged(&population, 0.001));

        // Create population with better fitness
        let better_population: Vec<SimulationResult> = (0..20)
            .map(|i| SimulationResult {
                simulation_id: i as SimulationId,
                config: SimulationConfig::new(i as ConfigId),
                completed: true,
                steps_executed: 100,
                final_coherence: 0.6 + i as Float * 0.01,
                average_coherence: 0.6 + i as Float * 0.01,
                coherence_history: vec![0.6 + i as Float * 0.01; 100],
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
                fitness_score: 0.6 + i as Float * 0.01,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            })
            .collect();

        assert!(!algorithm.has_converged(&better_population, 0.001));
        assert_eq!(algorithm.generations_without_improvement(), 0);
    }

    #[test]
    fn test_has_converged_no_improvement() {
        let mut algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        // First generation
        assert!(!algorithm.has_converged(&population, 0.001));

        // Create population with same fitness (no improvement)
        let same_population = population.clone();

        // Run 9 more generations without improvement (total 10 including first)
        for _ in 0..9 {
            assert!(!algorithm.has_converged(&same_population, 0.001));
        }

        // Should converge on the 10th generation without improvement
        assert!(algorithm.has_converged(&same_population, 0.001));
    }

    #[test]
    fn test_reset_convergence_tracking() {
        let mut algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        // Run some generations
        for _ in 0..5 {
            algorithm.has_converged(&population, 0.001);
        }

        assert!(algorithm.generations_without_improvement() > 0);

        // Reset
        algorithm.reset_convergence_tracking();

        assert_eq!(algorithm.generations_without_improvement(), 0);
        assert!(algorithm.best_fitness().is_none());
    }

    #[test]
    fn test_calculate_statistics() {
        let algorithm = SelectionAlgorithm::new();
        let population = create_mock_population(20);

        let stats = algorithm.calculate_statistics(&population);

        assert_eq!(stats.population_size, 20);
        assert!(stats.best_fitness > stats.worst_fitness);
        assert!(stats.average_fitness > 0.0);
    }

    #[test]
    fn test_selection_algorithm_invalid_params() {
        let mut params = SelectionParameters::new();
        params.tournament_size = 0;

        let result = SelectionAlgorithm::with_params(params);
        assert!(result.is_err());
    }
}
