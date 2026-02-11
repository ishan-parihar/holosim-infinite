//! Fitness Evaluator - Evaluate simulation results for selection
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Emergence metrics (biological, noospheric, Gaia), stability metrics (coherence,
//! energy balance), complexity metrics (diversity, integration), overall fitness score
//! (weighted combination)"

use crate::hpo::types::*;
use std::cmp::Ordering;

/// Default weight for emergence metrics (0.0 to 1.0)
const DEFAULT_EMERGENCE_WEIGHT: Float = 0.4;

/// Default weight for stability metrics (0.0 to 1.0)
const DEFAULT_STABILITY_WEIGHT: Float = 0.3;

/// Default weight for complexity metrics (0.0 to 1.0)
const DEFAULT_COMPLEXITY_WEIGHT: Float = 0.3;

/// Fitness Evaluator for ranking simulation results
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Emergence metrics (biological, noospheric, Gaia), stability metrics (coherence,
/// energy balance), complexity metrics (diversity, integration), overall fitness score
/// (weighted combination)"
#[derive(Debug, Clone)]
pub struct FitnessEvaluator {
    /// Weight for emergence metrics in overall fitness (default: 0.4)
    emergence_weight: Float,

    /// Weight for stability metrics in overall fitness (default: 0.3)
    stability_weight: Float,

    /// Weight for complexity metrics in overall fitness (default: 0.3)
    complexity_weight: Float,

    /// Penalty for failed simulations
    failure_penalty: Float,
}

impl FitnessEvaluator {
    /// Create a new fitness evaluator with default weights
    pub fn new() -> Self {
        Self {
            emergence_weight: DEFAULT_EMERGENCE_WEIGHT,
            stability_weight: DEFAULT_STABILITY_WEIGHT,
            complexity_weight: DEFAULT_COMPLEXITY_WEIGHT,
            failure_penalty: 0.0,
        }
    }

    /// Create a new fitness evaluator with custom weights
    pub fn with_weights(
        emergence_weight: Float,
        stability_weight: Float,
        complexity_weight: Float,
    ) -> Result<Self, String> {
        let total = emergence_weight + stability_weight + complexity_weight;
        if (total - 1.0).abs() > 0.01 {
            return Err(format!("Weights must sum to 1.0, got {}", total));
        }

        Ok(Self {
            emergence_weight,
            stability_weight,
            complexity_weight,
            failure_penalty: 0.0,
        })
    }

    /// Set the failure penalty
    pub fn with_failure_penalty(mut self, penalty: Float) -> Self {
        self.failure_penalty = penalty;
        self
    }

    /// Evaluate the fitness of a simulation result
    ///
    /// # Arguments
    /// * `result` - Simulation result to evaluate
    ///
    /// # Returns
    /// Fitness score (0.0 to 1.0)
    pub fn evaluate(&self, result: &SimulationResult) -> FitnessScore {
        // Check for failure
        if result.failure.is_some() {
            return FitnessScore::new(self.failure_penalty);
        }

        // Calculate emergence score
        let emergence_score = self.calculate_emergence_score(result);

        // Calculate stability score
        let stability_score = self.calculate_stability_score(result);

        // Calculate complexity score
        let complexity_score = self.calculate_complexity_score(result);

        // Calculate overall fitness score (weighted combination)
        let overall_fitness = (self.emergence_weight * emergence_score
            + self.stability_weight * stability_score
            + self.complexity_weight * complexity_score)
            .clamp(0.0, 1.0);

        FitnessScore(overall_fitness)
    }

    /// Calculate emergence score from emergence metrics
    ///
    /// # Arguments
    /// * `result` - Simulation result
    ///
    /// # Returns
    /// Emergence score (0.0 to 1.0)
    pub fn calculate_emergence_score(&self, result: &SimulationResult) -> Float {
        let metrics = &result.emergence_metrics;

        // Weighted average of emergence metrics
        // Note: In Phase 1, these are placeholder values
        // They will be fully implemented in Phases 2-4
        let biological_weight = 0.4;
        let noospheric_weight = 0.3;
        let gaia_weight = 0.3;

        let score = biological_weight * metrics.biological_score
            + noospheric_weight * metrics.noospheric_score
            + gaia_weight * metrics.gaia_score;

        score.clamp(0.0, 1.0)
    }

    /// Calculate stability score from stability metrics
    ///
    /// # Arguments
    /// * `result` - Simulation result
    ///
    /// # Returns
    /// Stability score (0.0 to 1.0)
    pub fn calculate_stability_score(&self, result: &SimulationResult) -> Float {
        let metrics = &result.stability_metrics;

        // Weighted average of stability metrics
        let coherence_weight = 0.4;
        let energy_weight = 0.3;
        let resilience_weight = 0.3;

        let score = coherence_weight * metrics.coherence_stability
            + energy_weight * metrics.energy_balance
            + resilience_weight * metrics.resilience;

        score.clamp(0.0, 1.0)
    }

    /// Calculate complexity score from complexity metrics
    ///
    /// # Arguments
    /// * `result` - Simulation result
    ///
    /// # Returns
    /// Complexity score (0.0 to 1.0)
    pub fn calculate_complexity_score(&self, result: &SimulationResult) -> Float {
        let metrics = &result.complexity_metrics;

        // Weighted average of complexity metrics
        let diversity_weight = 0.4;
        let integration_weight = 0.3;
        let depth_weight = 0.3;

        let score = diversity_weight * metrics.diversity_score
            + integration_weight * metrics.integration_score
            + depth_weight * metrics.depth_score;

        score.clamp(0.0, 1.0)
    }

    /// Compare two fitness scores
    ///
    /// # Arguments
    /// * `a` - First fitness score
    /// * `b` - Second fitness score
    ///
    /// # Returns
    /// Ordering (Less, Greater, or Equal)
    pub fn compare(&self, a: &FitnessScore, b: &FitnessScore) -> Ordering {
        a.partial_cmp(b).unwrap_or(Ordering::Equal)
    }

    /// Evaluate a batch of simulation results
    ///
    /// # Arguments
    /// * `results` - Vector of simulation results
    ///
    /// # Returns
    /// Vector of fitness scores in the same order as results
    pub fn evaluate_batch(&self, results: &[SimulationResult]) -> Vec<FitnessScore> {
        results.iter().map(|r| self.evaluate(r)).collect()
    }

    /// Rank simulation results by fitness
    ///
    /// # Arguments
    /// * `results` - Vector of simulation results
    ///
    /// # Returns
    /// Vector of results sorted by fitness (highest first)
    pub fn rank_results(&self, mut results: Vec<SimulationResult>) -> Vec<SimulationResult> {
        results.sort_by(|a, b| {
            let fitness_a = self.evaluate(a);
            let fitness_b = self.evaluate(b);
            fitness_b.partial_cmp(&fitness_a).unwrap_or(Ordering::Equal)
        });
        results
    }

    /// Get the top N results by fitness
    ///
    /// # Arguments
    /// * `results` - Vector of simulation results
    /// * `n` - Number of top results to return
    ///
    /// # Returns
    /// Vector of top N results
    pub fn top_n(&self, results: Vec<SimulationResult>, n: usize) -> Vec<SimulationResult> {
        let ranked = self.rank_results(results);
        ranked.into_iter().take(n).collect()
    }

    /// Calculate fitness statistics for a batch of results
    ///
    /// # Arguments
    /// * `results` - Vector of simulation results
    ///
    /// # Returns
    /// Fitness statistics (min, max, mean, median, std_dev)
    pub fn calculate_statistics(&self, results: &[SimulationResult]) -> FitnessStatistics {
        if results.is_empty() {
            return FitnessStatistics {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                count: 0,
            };
        }

        let fitness_scores: Vec<Float> = results.iter().map(|r| self.evaluate(r).value()).collect();

        let count = fitness_scores.len();
        let min = fitness_scores
            .iter()
            .cloned()
            .fold(Float::INFINITY, f64::min);
        let max = fitness_scores
            .iter()
            .cloned()
            .fold(Float::NEG_INFINITY, f64::max);
        let sum: Float = fitness_scores.iter().sum();
        let mean = sum / count as Float;

        // Calculate median
        let mut sorted = fitness_scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let median = if count % 2 == 0 {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
        } else {
            sorted[count / 2]
        };

        // Calculate standard deviation
        let variance: Float = fitness_scores
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<Float>()
            / count as Float;
        let std_dev = variance.sqrt();

        FitnessStatistics {
            min,
            max,
            mean,
            median,
            std_dev,
            count,
        }
    }

    /// Check if fitness is improving across generations
    ///
    /// # Arguments
    /// * `previous_best` - Best fitness from previous generation
    /// * `current_best` - Best fitness from current generation
    /// * `improvement_threshold` - Minimum improvement to consider significant
    ///
    /// # Returns
    /// True if fitness has improved significantly
    pub fn is_improving(
        &self,
        previous_best: FitnessScore,
        current_best: FitnessScore,
        improvement_threshold: Float,
    ) -> bool {
        (current_best.value() - previous_best.value()) >= improvement_threshold
    }
}

impl Default for FitnessEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Fitness statistics for a batch of results
#[derive(Debug, Clone, PartialEq)]
pub struct FitnessStatistics {
    /// Minimum fitness score
    pub min: Float,

    /// Maximum fitness score
    pub max: Float,

    /// Mean fitness score
    pub mean: Float,

    /// Median fitness score
    pub median: Float,

    /// Standard deviation of fitness scores
    pub std_dev: Float,

    /// Number of results
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitness_evaluator_creation() {
        let evaluator = FitnessEvaluator::new();
        assert_eq!(evaluator.emergence_weight, DEFAULT_EMERGENCE_WEIGHT);
        assert_eq!(evaluator.stability_weight, DEFAULT_STABILITY_WEIGHT);
        assert_eq!(evaluator.complexity_weight, DEFAULT_COMPLEXITY_WEIGHT);
    }

    #[test]
    fn test_fitness_evaluator_with_weights() {
        let evaluator = FitnessEvaluator::with_weights(0.5, 0.3, 0.2).unwrap();
        assert_eq!(evaluator.emergence_weight, 0.5);
        assert_eq!(evaluator.stability_weight, 0.3);
        assert_eq!(evaluator.complexity_weight, 0.2);
    }

    #[test]
    fn test_fitness_evaluator_invalid_weights() {
        let result = FitnessEvaluator::with_weights(0.5, 0.5, 0.2);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_healthy_result() {
        let evaluator = FitnessEvaluator::new();

        let result = SimulationResult {
            simulation_id: 1,
            config: SimulationConfig::new(1),
            completed: true,
            steps_executed: 100,
            final_coherence: 0.9,
            average_coherence: 0.85,
            coherence_history: (0..100).map(|i| 0.5 + i as Float * 0.001).collect(),
            energy_conservation_error: 0.05,
            entities_evolved: 10,
            entities_harvested: 5,
            emergence_metrics: EmergenceMetrics {
                biological_score: 0.7,
                noospheric_score: 0.6,
                gaia_score: 0.5,
                overall_score: 0.6,
            },
            stability_metrics: StabilityMetrics {
                coherence_stability: 0.9,
                energy_balance: 0.95,
                resilience: 1.0,
                overall_score: 0.95,
            },
            complexity_metrics: ComplexityMetrics {
                diversity_score: 0.6,
                integration_score: 0.7,
                depth_score: 0.5,
                overall_score: 0.6,
            },
            fitness_score: 0.7,
            execution_time: 10.0,
            failure: None,
            metadata: std::collections::HashMap::new(),
        };

        let fitness = evaluator.evaluate(&result);
        assert!(fitness.value() > 0.0);
    }

    #[test]
    fn test_evaluate_failed_result() {
        let evaluator = FitnessEvaluator::new();

        let result = SimulationResult {
            simulation_id: 1,
            config: SimulationConfig::new(1),
            completed: false,
            steps_executed: 50,
            final_coherence: 0.6,
            average_coherence: 0.65,
            coherence_history: vec![0.6; 50],
            energy_conservation_error: 0.15,
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
            execution_time: 5.0,
            failure: Some(FailureType::CoherenceViolation(0.6)),
            metadata: std::collections::HashMap::new(),
        };

        let fitness = evaluator.evaluate(&result);
        assert_eq!(fitness.value(), 0.0);
    }

    #[test]
    fn test_compare_fitness_scores() {
        let evaluator = FitnessEvaluator::new();

        let a = FitnessScore::new(0.7);
        let b = FitnessScore::new(0.8);

        assert_eq!(evaluator.compare(&a, &b), Ordering::Less);
        assert_eq!(evaluator.compare(&b, &a), Ordering::Greater);
    }

    #[test]
    fn test_rank_results() {
        let evaluator = FitnessEvaluator::new();

        let mut results = Vec::new();
        for i in 0..5 {
            results.push(SimulationResult {
                simulation_id: i as SimulationId,
                config: SimulationConfig::new(i as ConfigId),
                completed: true,
                steps_executed: 100,
                final_coherence: 0.5 + i as Float * 0.1,
                average_coherence: 0.5 + i as Float * 0.1,
                coherence_history: vec![0.5 + i as Float * 0.1; 100],
                energy_conservation_error: 0.05,
                entities_evolved: 10,
                entities_harvested: 5,
                emergence_metrics: EmergenceMetrics {
                    biological_score: 0.5 + i as Float * 0.1,
                    noospheric_score: 0.5 + i as Float * 0.1,
                    gaia_score: 0.5 + i as Float * 0.1,
                    overall_score: 0.5 + i as Float * 0.1,
                },
                stability_metrics: StabilityMetrics {
                    coherence_stability: 0.8 + i as Float * 0.02,
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
                fitness_score: 0.7,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            });
        }

        let ranked = evaluator.rank_results(results);
        assert_eq!(ranked.len(), 5);
        assert!(ranked[0].final_coherence > ranked[1].final_coherence);
    }

    #[test]
    fn test_top_n() {
        let evaluator = FitnessEvaluator::new();

        let mut results = Vec::new();
        for i in 0..10 {
            results.push(SimulationResult {
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
                    coherence_stability: 0.8 + i as Float * 0.02,
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
                fitness_score: 0.7,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            });
        }

        let top_3 = evaluator.top_n(results, 3);
        assert_eq!(top_3.len(), 3);
    }

    #[test]
    fn test_calculate_statistics() {
        let evaluator = FitnessEvaluator::new();

        let mut results = Vec::new();
        for i in 0..10 {
            results.push(SimulationResult {
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
                    coherence_stability: 0.8 + i as Float * 0.02,
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
                fitness_score: 0.7,
                execution_time: 10.0,
                failure: None,
                metadata: std::collections::HashMap::new(),
            });
        }

        let stats = evaluator.calculate_statistics(&results);
        assert_eq!(stats.count, 10);
        assert!(stats.max > stats.min);
        assert!(stats.mean > stats.min);
        assert!(stats.mean < stats.max);
    }

    #[test]
    fn test_is_improving() {
        let evaluator = FitnessEvaluator::new();

        let previous = FitnessScore::new(0.7);
        let current = FitnessScore::new(0.8);

        assert!(evaluator.is_improving(previous, current, 0.05));
        assert!(!evaluator.is_improving(current, previous, 0.05));
        assert!(!evaluator.is_improving(previous, current, 0.15));
    }
}
