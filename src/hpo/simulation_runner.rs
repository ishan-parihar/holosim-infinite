//! Simulation Runner - Parallel execution of multiple simulations
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Thread pool for parallel execution, configuration generator, result collector,
//! progress tracking"

use crate::complete_simulation::CompleteSimulation;
use crate::hpo::types::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Thread pool size for parallel simulation execution
const DEFAULT_THREAD_POOL_SIZE: usize = 8;

/// Progress callback type
pub type ProgressCallback = Arc<Mutex<dyn Fn(usize, usize) + Send + Sync>>;

/// Simulation Runner for parallel execution
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Thread pool for parallel execution, configuration generator, result collector,
/// progress tracking"
#[derive(Clone)]
pub struct SimulationRunner {
    /// Thread pool for parallel execution
    thread_pool_size: usize,

    /// Next simulation ID to assign
    #[allow(dead_code)]
    next_simulation_id: Arc<Mutex<SimulationId>>,

    /// Optional progress callback
    progress_callback: Option<ProgressCallback>,

    /// Timeout for individual simulations (in seconds)
    simulation_timeout: Float,
}

impl SimulationRunner {
    /// Create a new simulation runner with default settings
    pub fn new() -> Self {
        Self {
            thread_pool_size: DEFAULT_THREAD_POOL_SIZE,
            next_simulation_id: Arc::new(Mutex::new(1)),
            progress_callback: None,
            simulation_timeout: 300.0, // 5 minutes default
        }
    }

    /// Create a new simulation runner with custom thread pool size
    pub fn with_thread_pool_size(thread_pool_size: usize) -> Self {
        Self {
            thread_pool_size,
            next_simulation_id: Arc::new(Mutex::new(1)),
            progress_callback: None,
            simulation_timeout: 300.0,
        }
    }

    /// Set the progress callback
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(Mutex::new(callback)));
        self
    }

    /// Set the simulation timeout
    pub fn with_timeout(mut self, timeout_seconds: Float) -> Self {
        self.simulation_timeout = timeout_seconds;
        self
    }

    /// Run a batch of simulations in parallel
    ///
    /// # Arguments
    /// * `configs` - Vector of simulation configurations to run
    ///
    /// # Returns
    /// Vector of simulation results in the same order as configs
    pub fn run_batch(&self, configs: Vec<SimulationConfig>) -> Vec<SimulationResult> {
        if configs.is_empty() {
            return vec![];
        }

        // Update thread pool size if needed
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_pool_size)
            .build_global()
            .ok();

        let total = configs.len();
        let completed = Arc::new(Mutex::new(0usize));

        // Run simulations in parallel
        let results: Vec<SimulationResult> = configs
            .into_par_iter()
            .enumerate()
            .map(|(index, config)| {
                let result = self.run_single_simulation(config, index as SimulationId);

                // Update progress
                {
                    let mut count = completed.lock().unwrap();
                    *count += 1;
                    if let Some(ref callback) = self.progress_callback {
                        let callback = callback.lock().unwrap();
                        callback(*count, total);
                    }
                }

                result
            })
            .collect();

        results
    }

    /// Run a specified number of parallel simulations with randomly generated configs
    ///
    /// # Arguments
    /// * `num_simulations` - Number of simulations to run
    /// * `parameter_space` - Parameter space for generating random configs
    ///
    /// # Returns
    /// Vector of simulation results
    pub fn run_parallel(
        &self,
        num_simulations: usize,
        parameter_space: &crate::hpo::ParameterSpaceManager,
    ) -> Vec<SimulationResult> {
        let configs: Vec<SimulationConfig> = (0..num_simulations)
            .map(|i| {
                let mut config = parameter_space.generate_random_config(i as ConfigId);
                config.seed = (i as u64).wrapping_add(1);
                config
            })
            .collect();

        self.run_batch(configs)
    }

    /// Run a single simulation
    ///
    /// # Arguments
    /// * `config` - Simulation configuration
    /// * `simulation_id` - Unique identifier for this simulation
    ///
    /// # Returns
    /// Simulation result
    fn run_single_simulation(
        &self,
        config: SimulationConfig,
        simulation_id: SimulationId,
    ) -> SimulationResult {
        let start_time = Instant::now();

        // Validate configuration
        if let Err(e) = config.validate() {
            return SimulationResult {
                simulation_id,
                config,
                completed: false,
                steps_executed: 0,
                final_coherence: 0.0,
                average_coherence: 0.0,
                coherence_history: vec![],
                energy_conservation_error: 0.0,
                entities_evolved: 0,
                entities_harvested: 0,
                emergence_metrics: EmergenceMetrics {
                    biological_score: 0.0,
                    noospheric_score: 0.0,
                    gaia_score: 0.0,
                    overall_score: 0.0,
                },
                stability_metrics: StabilityMetrics {
                    coherence_stability: 0.0,
                    energy_balance: 0.0,
                    resilience: 0.0,
                    overall_score: 0.0,
                },
                complexity_metrics: ComplexityMetrics {
                    diversity_score: 0.0,
                    integration_score: 0.0,
                    depth_score: 0.0,
                    overall_score: 0.0,
                },
                fitness_score: 0.0,
                execution_time: start_time.elapsed().as_secs_f64(),
                failure: Some(FailureType::InvalidConfig(e.to_string())),
                metadata: HashMap::new(),
            };
        }

        // Create and run simulation
        let mut simulation = CompleteSimulation::new();

        // Run simulation steps
        let mut coherence_history = Vec::with_capacity(config.num_steps);
        let mut completed_steps = 0;

        for _step in 0..config.num_steps {
            // Check timeout
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > self.simulation_timeout {
                return SimulationResult {
                    simulation_id,
                    config,
                    completed: false,
                    steps_executed: completed_steps,
                    final_coherence: coherence_history.last().copied().unwrap_or(0.0),
                    average_coherence: if coherence_history.is_empty() {
                        0.0
                    } else {
                        coherence_history.iter().sum::<Float>() / coherence_history.len() as Float
                    },
                    coherence_history,
                    energy_conservation_error: 0.0,
                    entities_evolved: 0,
                    entities_harvested: 0,
                    emergence_metrics: EmergenceMetrics {
                        biological_score: 0.0,
                        noospheric_score: 0.0,
                        gaia_score: 0.0,
                        overall_score: 0.0,
                    },
                    stability_metrics: StabilityMetrics {
                        coherence_stability: 0.0,
                        energy_balance: 0.0,
                        resilience: 0.0,
                        overall_score: 0.0,
                    },
                    complexity_metrics: ComplexityMetrics {
                        diversity_score: 0.0,
                        integration_score: 0.0,
                        depth_score: 0.0,
                        overall_score: 0.0,
                    },
                    fitness_score: 0.0,
                    execution_time: elapsed,
                    failure: Some(FailureType::Timeout(elapsed)),
                    metadata: HashMap::new(),
                };
            }

            // Process step
            if let Ok(result) = simulation.process_step() {
                coherence_history.push(result.net_flow.abs());
                completed_steps += 1;
            } else {
                break;
            }
        }

        // Calculate final metrics
        let final_coherence = coherence_history.last().copied().unwrap_or(0.0);
        let average_coherence = if coherence_history.is_empty() {
            0.0
        } else {
            coherence_history.iter().sum::<Float>() / coherence_history.len() as Float
        };

        let execution_time = start_time.elapsed().as_secs_f64();

        // Calculate emergence, stability, and complexity metrics
        // Note: These are placeholder values for Phase 1
        // They will be fully implemented in Phases 2-4
        let emergence_metrics = EmergenceMetrics {
            biological_score: 0.0, // Will be implemented in Phase 3
            noospheric_score: 0.0, // Will be implemented in Phase 4
            gaia_score: 0.0,       // Will be implemented in Phase 4
            overall_score: 0.0,
        };

        let stability_metrics = StabilityMetrics {
            coherence_stability: average_coherence,
            energy_balance: 1.0 - (0.0), // Placeholder energy conservation error
            resilience: if completed_steps >= config.num_steps {
                1.0
            } else {
                completed_steps as Float / config.num_steps as Float
            },
            overall_score: 0.0,
        };

        let complexity_metrics = ComplexityMetrics {
            diversity_score: 0.0,   // Will be calculated based on entity diversity
            integration_score: 0.0, // Will be calculated based on entity interactions
            depth_score: 0.0,       // Will be calculated based on density progression
            overall_score: 0.0,
        };

        let num_steps = config.num_steps;
        SimulationResult {
            simulation_id,
            config,
            completed: completed_steps >= num_steps,
            steps_executed: completed_steps,
            final_coherence,
            average_coherence,
            coherence_history,
            energy_conservation_error: 0.0, // Placeholder
            entities_evolved: 0,            // Placeholder
            entities_harvested: 0,          // Placeholder
            emergence_metrics,
            stability_metrics,
            complexity_metrics,
            fitness_score: 0.0, // Will be calculated by FitnessEvaluator
            execution_time,
            failure: None,
            metadata: HashMap::new(),
        }
    }
}

impl Default for SimulationRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_runner_creation() {
        let runner = SimulationRunner::new();
        assert_eq!(runner.thread_pool_size, DEFAULT_THREAD_POOL_SIZE);
    }

    #[test]
    fn test_simulation_runner_with_thread_pool_size() {
        let runner = SimulationRunner::with_thread_pool_size(16);
        assert_eq!(runner.thread_pool_size, 16);
    }

    #[test]
    fn test_simulation_runner_with_timeout() {
        let runner = SimulationRunner::new().with_timeout(600.0);
        assert_eq!(runner.simulation_timeout, 600.0);
    }

    #[test]
    fn test_run_single_simulation() {
        let runner = SimulationRunner::new();
        let config = SimulationConfig::builder()
            .with_config_id(1)
            .with_num_entities(32)
            .with_num_steps(50)
            .build()
            .unwrap();

        let result = runner.run_single_simulation(config, 1);
        assert!(result.steps_executed > 0);
        assert_eq!(result.simulation_id, 1);
    }

    #[test]
    fn test_run_batch_small() {
        let runner = SimulationRunner::new();
        let configs = vec![
            SimulationConfig::builder()
                .with_config_id(1)
                .with_num_entities(16)
                .with_num_steps(20)
                .build()
                .unwrap(),
            SimulationConfig::builder()
                .with_config_id(2)
                .with_num_entities(16)
                .with_num_steps(20)
                .build()
                .unwrap(),
        ];

        let results = runner.run_batch(configs);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_invalid_config_handling() {
        let runner = SimulationRunner::new();
        let mut config = SimulationConfig::new(1);
        config.num_entities = 0; // Invalid

        let result = runner.run_single_simulation(config, 1);
        assert!(!result.completed);
        assert!(result.failure.is_some());
        match result.failure {
            Some(FailureType::InvalidConfig(_)) => {}
            _ => panic!("Expected InvalidConfig failure"),
        }
    }
}
