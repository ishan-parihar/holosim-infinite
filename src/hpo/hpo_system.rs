//! HPO System - Main orchestrator for hyperparameter optimization
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Build automated universe parameter optimization system that runs multiple parallel
//! simulations, detects failures, and keeps what works through survival of the fittest."

use crate::hpo::types::*;
use crate::hpo::{
    FitnessEvaluator, HealthMonitor, ParameterSpaceManager, SelectionAlgorithm, SimulationRunner,
};
use std::sync::{Arc, Mutex};

/// HPO System - Main orchestrator for hyperparameter optimization
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Build automated universe parameter optimization system that runs multiple parallel
/// simulations, detects failures, and keeps what works through survival of the fittest."
#[derive(Clone)]
pub struct HpoSystem {
    /// Simulation runner for parallel execution
    runner: SimulationRunner,

    /// Health monitor for failure detection
    health_monitor: HealthMonitor,

    /// Fitness evaluator for ranking results
    fitness_evaluator: FitnessEvaluator,

    /// Parameter space manager for generating configs
    parameter_space: ParameterSpaceManager,

    /// Selection algorithm for evolutionary optimization
    selection: SelectionAlgorithm,

    /// Current generation number
    current_generation: usize,

    /// History of best fitness per generation
    fitness_history: Vec<Float>,

    /// All results from all generations
    all_results: Vec<SimulationResult>,
}

impl HpoSystem {
    /// Create a new HPO system with default settings
    pub fn new() -> Self {
        Self {
            runner: SimulationRunner::new(),
            health_monitor: HealthMonitor::new(),
            fitness_evaluator: FitnessEvaluator::new(),
            parameter_space: ParameterSpaceManager::new(),
            selection: SelectionAlgorithm::new(),
            current_generation: 0,
            fitness_history: Vec::new(),
            all_results: Vec::new(),
        }
    }

    /// Create a new HPO system with custom components
    pub fn with_components(
        runner: SimulationRunner,
        health_monitor: HealthMonitor,
        fitness_evaluator: FitnessEvaluator,
        parameter_space: ParameterSpaceManager,
        selection: SelectionAlgorithm,
    ) -> Self {
        Self {
            runner,
            health_monitor,
            fitness_evaluator,
            parameter_space,
            selection,
            current_generation: 0,
            fitness_history: Vec::new(),
            all_results: Vec::new(),
        }
    }

    /// Run the full optimization process
    ///
    /// # Arguments
    /// * `num_simulations` - Number of simulations per generation
    /// * `max_generations` - Maximum number of generations to run
    ///
    /// # Returns
    /// Final optimization result
    pub fn run_optimization(
        &mut self,
        num_simulations: usize,
        max_generations: usize,
    ) -> Result<OptimizationResult, String> {
        println!("Starting HPO optimization...");
        println!("  Simulations per generation: {}", num_simulations);
        println!("  Maximum generations: {}", max_generations);

        let mut best_overall_fitness = FitnessScore::new(0.0);
        let mut best_overall_config: Option<SimulationConfig> = None;

        // Run generations
        for generation in 0..max_generations {
            self.current_generation = generation;

            println!("\n=== Generation {} ===", generation);

            // Generate configurations for this generation
            let configs = if generation == 0 {
                // First generation: random configs
                self.parameter_space
                    .generate_random_configs(num_simulations)
            } else {
                // Subsequent generations: evolve from previous results
                let population: Vec<SimulationResult> = self.all_results.clone();
                self.parameter_space.generate_next_generation(
                    &population,
                    num_simulations,
                    self.selection.params(),
                )
            };

            println!("  Generated {} configurations", configs.len());

            // Run simulations
            let progress_callback = Arc::new(Mutex::new(|completed: usize, total: usize| {
                if completed.is_multiple_of(10) || completed == total {
                    println!("  Progress: {}/{} simulations completed", completed, total);
                }
            }));

            let runner_with_progress =
                self.runner
                    .clone()
                    .with_progress_callback(move |completed, total| {
                        let callback = progress_callback.lock().unwrap();
                        callback(completed, total);
                    });

            let mut results = runner_with_progress.run_batch(configs);
            println!("  Completed {} simulations", results.len());

            // Evaluate fitness
            for result in &mut results {
                result.fitness_score = self.fitness_evaluator.evaluate(result).0;
            }

            // Filter out failed simulations
            let (successful, failed) = self.health_monitor.filter_failures(results);
            println!("  Successful: {}", successful.len());
            println!("  Failed: {}", failed.len());

            // Store all results
            self.all_results.extend(successful.clone());
            self.all_results.extend(failed.clone());

            // Track best fitness
            if let Some(best) = successful.iter().max_by(|a, b| {
                a.fitness_score
                    .partial_cmp(&b.fitness_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let fitness = FitnessScore::new(best.fitness_score);
                if fitness.0 > best_overall_fitness.0 {
                    best_overall_fitness = fitness;
                    best_overall_config = Some(best.config.clone());
                }
            }

            // Record fitness history
            let best_gen_fitness = successful
                .iter()
                .map(|r| r.fitness_score)
                .fold(Float::NEG_INFINITY, f64::max);
            self.fitness_history.push(best_gen_fitness);

            // Print statistics
            let stats = self.selection.calculate_statistics(&successful);
            println!("  Best fitness: {:.4}", stats.best_fitness);
            println!("  Average fitness: {:.4}", stats.average_fitness);
            println!("  Fitness std dev: {:.4}", stats.fitness_std_dev);

            // Check for convergence
            if self.selection.has_converged(&successful, 0.001) {
                println!("\n  Converged after {} generations!", generation + 1);
                break;
            }
        }

        println!("\n=== Optimization Complete ===");
        println!("  Total generations: {}", self.current_generation + 1);
        println!("  Total simulations run: {}", self.all_results.len());
        println!("  Best overall fitness: {:.4}", best_overall_fitness.0);

        Ok(OptimizationResult {
            best_config: best_overall_config,
            best_fitness: best_overall_fitness,
            generations_run: self.current_generation + 1,
            total_simulations: self.all_results.len(),
            fitness_history: self.fitness_history.clone(),
            all_results: self.all_results.clone(),
        })
    }

    /// Run a single generation
    ///
    /// # Arguments
    /// * `num_simulations` - Number of simulations to run
    ///
    /// # Returns
    /// Vector of simulation results
    pub fn run_generation(&mut self, num_simulations: usize) -> Vec<SimulationResult> {
        self.current_generation += 1;

        println!("\n=== Generation {} ===", self.current_generation);

        // Generate configurations
        let configs = if self.current_generation == 1 {
            self.parameter_space
                .generate_random_configs(num_simulations)
        } else {
            let population: Vec<SimulationResult> = self.all_results.clone();
            self.parameter_space.generate_next_generation(
                &population,
                num_simulations,
                self.selection.params(),
            )
        };

        println!("  Generated {} configurations", configs.len());

        // Run simulations
        let mut results = self.runner.run_batch(configs);
        println!("  Completed {} simulations", results.len());

        // Evaluate fitness
        for result in &mut results {
            result.fitness_score = self.fitness_evaluator.evaluate(result).0;
        }

        // Store results
        self.all_results.extend(results.clone());

        // Update convergence tracking
        let (successful, _) = self.health_monitor.filter_failures(results.clone());

        // Always record best fitness from this generation (even if all failed)
        let best_gen_fitness = if !successful.is_empty() {
            successful
                .iter()
                .map(|r| r.fitness_score)
                .fold(Float::NEG_INFINITY, f64::max)
        } else {
            // If all failed, use the best (even if failed)
            results
                .iter()
                .map(|r| r.fitness_score)
                .fold(Float::NEG_INFINITY, f64::max)
        };
        self.fitness_history.push(best_gen_fitness);

        if !successful.is_empty() {
            self.selection.has_converged(&successful, 0.001);
        }

        results
    }

    /// Get the current generation number
    pub fn current_generation(&self) -> usize {
        self.current_generation
    }

    /// Get the fitness history
    pub fn fitness_history(&self) -> &[Float] {
        &self.fitness_history
    }

    /// Get all results
    pub fn all_results(&self) -> &[SimulationResult] {
        &self.all_results
    }

    /// Get the best result so far
    pub fn best_result(&self) -> Option<&SimulationResult> {
        self.all_results.iter().max_by(|a, b| {
            a.fitness_score
                .partial_cmp(&b.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Check if the optimization has converged
    pub fn has_converged(&self) -> bool {
        self.selection.generations_without_improvement()
            >= self.selection.params().convergence_threshold
    }

    /// Reset the HPO system
    pub fn reset(&mut self) {
        self.current_generation = 0;
        self.fitness_history.clear();
        self.all_results.clear();
        self.selection.reset_convergence_tracking();
    }

    /// Get the simulation runner
    pub fn runner(&self) -> &SimulationRunner {
        &self.runner
    }

    /// Get the health monitor
    pub fn health_monitor(&self) -> &HealthMonitor {
        &self.health_monitor
    }

    /// Get the fitness evaluator
    pub fn fitness_evaluator(&self) -> &FitnessEvaluator {
        &self.fitness_evaluator
    }

    /// Get the parameter space manager
    pub fn parameter_space(&self) -> &ParameterSpaceManager {
        &self.parameter_space
    }

    /// Get the selection algorithm
    pub fn selection(&self) -> &SelectionAlgorithm {
        &self.selection
    }
}

impl Default for HpoSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of HPO optimization
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationResult {
    /// Best configuration found
    pub best_config: Option<SimulationConfig>,

    /// Best fitness achieved
    pub best_fitness: FitnessScore,

    /// Number of generations run
    pub generations_run: usize,

    /// Total number of simulations run
    pub total_simulations: usize,

    /// Fitness history (best fitness per generation)
    pub fitness_history: Vec<Float>,

    /// All results from all generations
    pub all_results: Vec<SimulationResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpo_system_creation() {
        let hpo = HpoSystem::new();
        assert_eq!(hpo.current_generation(), 0);
        assert!(hpo.fitness_history().is_empty());
    }

    #[test]
    fn test_run_generation() {
        let mut hpo = HpoSystem::new();

        let results = hpo.run_generation(5);

        assert_eq!(hpo.current_generation(), 1);
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_best_result() {
        let mut hpo = HpoSystem::new();

        hpo.run_generation(10);

        let best = hpo.best_result();
        assert!(best.is_some());
    }

    #[test]
    fn test_reset() {
        let mut hpo = HpoSystem::new();

        hpo.run_generation(5);
        assert_eq!(hpo.current_generation(), 1);
        assert!(!hpo.fitness_history().is_empty());

        hpo.reset();
        assert_eq!(hpo.current_generation(), 0);
        assert!(hpo.fitness_history().is_empty());
    }

    #[test]
    fn test_has_converged() {
        let hpo = HpoSystem::new();
        // Should not be converged initially
        assert!(!hpo.has_converged());
    }
}
