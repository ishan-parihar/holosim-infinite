//! Health Monitor - Failure detection for running simulations
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Coherence threshold detection (< 0.8 = unhealthy), energy conservation validation,
//! emergence stagnation detection (no change in metrics), crash recovery"

use crate::hpo::types::*;

/// Coherence threshold below which a simulation is considered unhealthy
const DEFAULT_COHERENCE_THRESHOLD: Float = 0.8;

/// Energy conservation error threshold
const DEFAULT_ENERGY_CONSERVATION_THRESHOLD: Float = 0.1;

/// Number of steps without progress before considering stagnation
const DEFAULT_STAGNATION_THRESHOLD: usize = 50;

/// Health Monitor for detecting simulation failures
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Coherence threshold detection (< 0.8 = unhealthy), energy conservation validation,
/// emergence stagnation detection (no change in metrics), crash recovery"
#[derive(Debug, Clone)]
pub struct HealthMonitor {
    /// Coherence threshold (simulations below this are unhealthy)
    coherence_threshold: Float,

    /// Energy conservation error threshold
    energy_conservation_threshold: Float,

    /// Number of steps without progress before stagnation
    stagnation_threshold: usize,

    /// Minimum required improvement to avoid stagnation
    minimum_improvement: Float,
}

impl HealthMonitor {
    /// Create a new health monitor with default thresholds
    pub fn new() -> Self {
        Self {
            coherence_threshold: DEFAULT_COHERENCE_THRESHOLD,
            energy_conservation_threshold: DEFAULT_ENERGY_CONSERVATION_THRESHOLD,
            stagnation_threshold: DEFAULT_STAGNATION_THRESHOLD,
            minimum_improvement: 0.001,
        }
    }

    /// Create a new health monitor with custom thresholds
    pub fn with_thresholds(
        coherence_threshold: Float,
        energy_conservation_threshold: Float,
        stagnation_threshold: usize,
    ) -> Self {
        Self {
            coherence_threshold,
            energy_conservation_threshold,
            stagnation_threshold,
            minimum_improvement: 0.001,
        }
    }

    /// Set the minimum improvement threshold
    pub fn with_minimum_improvement(mut self, minimum: Float) -> Self {
        self.minimum_improvement = minimum;
        self
    }

    /// Check the health status of a simulation result
    ///
    /// # Arguments
    /// * `result` - Simulation result to check
    ///
    /// # Returns
    /// Health status (Healthy, Unhealthy, or Failed)
    pub fn check_health(&self, result: &SimulationResult) -> HealthStatus {
        // Check for explicit failure
        if result.failure.is_some() {
            return HealthStatus::Failed;
        }

        // Check if simulation completed
        if !result.completed {
            return HealthStatus::Failed;
        }

        // Check coherence threshold
        if result.final_coherence < self.coherence_threshold {
            return HealthStatus::Unhealthy;
        }

        // Check energy conservation
        if result.energy_conservation_error > self.energy_conservation_threshold {
            return HealthStatus::Unhealthy;
        }

        // Check for stagnation
        if self.detect_stagnation(&result.coherence_history) {
            return HealthStatus::Unhealthy;
        }

        HealthStatus::Healthy
    }

    /// Detect if a failure occurred in a simulation result
    ///
    /// # Arguments
    /// * `result` - Simulation result to check
    ///
    /// # Returns
    /// Optional failure type (None if no failure)
    pub fn detect_failure(&self, result: &SimulationResult) -> Option<FailureType> {
        // Check explicit failure
        if let Some(ref failure) = result.failure {
            return Some(failure.clone());
        }

        // Check if simulation completed
        if !result.completed {
            return Some(FailureType::Crash(
                "Simulation did not complete".to_string(),
            ));
        }

        // Check coherence violation
        if result.final_coherence < self.coherence_threshold {
            return Some(FailureType::CoherenceViolation(result.final_coherence));
        }

        // Check energy conservation violation
        if result.energy_conservation_error > self.energy_conservation_threshold {
            return Some(FailureType::EnergyConservationViolation(
                result.energy_conservation_error,
            ));
        }

        // Check stagnation
        if let Some(stagnation_steps) = self.detect_stagnation_with_steps(&result.coherence_history)
        {
            return Some(FailureType::Stagnation(stagnation_steps));
        }

        None
    }

    /// Detect stagnation in coherence history
    ///
    /// # Arguments
    /// * `coherence_history` - History of coherence values
    ///
    /// # Returns
    /// True if stagnation detected
    pub fn detect_stagnation(&self, coherence_history: &[Float]) -> bool {
        if coherence_history.len() < self.stagnation_threshold {
            return false;
        }

        // Check if there's been significant improvement in the last N steps
        let recent_window: &[Float] =
            &coherence_history[coherence_history.len() - self.stagnation_threshold..];
        let older_window: &[Float] = &coherence_history[coherence_history
            .len()
            .saturating_sub(self.stagnation_threshold * 2)
            ..coherence_history
                .len()
                .saturating_sub(self.stagnation_threshold)];

        if older_window.is_empty() || recent_window.is_empty() {
            return false;
        }

        let recent_avg: Float = recent_window.iter().sum::<Float>() / recent_window.len() as Float;
        let older_avg: Float = older_window.iter().sum::<Float>() / older_window.len() as Float;

        // If improvement is less than minimum, consider it stagnation
        (recent_avg - older_avg).abs() < self.minimum_improvement
    }

    /// Detect stagnation and return the number of stagnant steps
    ///
    /// # Arguments
    /// * `coherence_history` - History of coherence values
    ///
    /// # Returns
    /// Number of stagnant steps, or None if no stagnation
    pub fn detect_stagnation_with_steps(&self, coherence_history: &[Float]) -> Option<usize> {
        if coherence_history.len() < self.stagnation_threshold {
            return None;
        }

        // Count consecutive steps without improvement from the end
        let mut stagnant_steps = 0;
        for i in (1..coherence_history.len()).rev() {
            let improvement = (coherence_history[i] - coherence_history[i - 1]).abs();
            if improvement < self.minimum_improvement {
                stagnant_steps += 1;
            } else {
                break;
            }
        }

        if stagnant_steps >= self.stagnation_threshold {
            Some(stagnant_steps)
        } else {
            None
        }
    }

    /// Validate energy conservation
    ///
    /// # Arguments
    /// * `energy_error` - Energy conservation error
    ///
    /// # Returns
    /// True if energy is sufficiently conserved
    pub fn validate_energy_conservation(&self, energy_error: Float) -> bool {
        energy_error <= self.energy_conservation_threshold
    }

    /// Validate coherence
    ///
    /// # Arguments
    /// * `coherence` - Coherence value
    ///
    /// # Returns
    /// True if coherence is above threshold
    pub fn validate_coherence(&self, coherence: Float) -> bool {
        coherence >= self.coherence_threshold
    }

    /// Calculate health score (0.0 to 1.0)
    ///
    /// # Arguments
    /// * `result` - Simulation result
    ///
    /// # Returns
    /// Health score (0.0 = failed, 1.0 = perfectly healthy)
    pub fn calculate_health_score(&self, result: &SimulationResult) -> Float {
        if result.failure.is_some() {
            return 0.0;
        }

        if !result.completed {
            return 0.0;
        }

        let coherence_score = (result.final_coherence / self.coherence_threshold).min(1.0);
        let energy_score = if result.energy_conservation_error <= self.energy_conservation_threshold
        {
            1.0
        } else {
            self.energy_conservation_threshold / result.energy_conservation_error
        };

        let stagnation_penalty = if self.detect_stagnation(&result.coherence_history) {
            0.2
        } else {
            0.0
        };

        // Weighted average
        (coherence_score * 0.5 + energy_score * 0.3 + (1.0 - stagnation_penalty) * 0.2)
            .clamp(0.0, 1.0)
    }

    /// Filter out failed simulations
    ///
    /// # Arguments
    /// * `results` - Vector of simulation results
    ///
    /// # Returns
    /// Tuple of (successful_results, failed_results)
    pub fn filter_failures(
        &self,
        results: Vec<SimulationResult>,
    ) -> (Vec<SimulationResult>, Vec<SimulationResult>) {
        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for result in results {
            if self.detect_failure(&result).is_none() {
                successful.push(result);
            } else {
                failed.push(result);
            }
        }

        (successful, failed)
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new();
        assert_eq!(monitor.coherence_threshold, DEFAULT_COHERENCE_THRESHOLD);
        assert_eq!(monitor.stagnation_threshold, DEFAULT_STAGNATION_THRESHOLD);
    }

    #[test]
    fn test_health_monitor_with_thresholds() {
        let monitor = HealthMonitor::with_thresholds(0.7, 0.2, 100);
        assert_eq!(monitor.coherence_threshold, 0.7);
        assert_eq!(monitor.energy_conservation_threshold, 0.2);
        assert_eq!(monitor.stagnation_threshold, 100);
    }

    #[test]
    fn test_validate_coherence() {
        let monitor = HealthMonitor::new();

        assert!(monitor.validate_coherence(0.9));
        assert!(monitor.validate_coherence(0.8));
        assert!(!monitor.validate_coherence(0.7));
    }

    #[test]
    fn test_validate_energy_conservation() {
        let monitor = HealthMonitor::new();

        assert!(monitor.validate_energy_conservation(0.05));
        assert!(monitor.validate_energy_conservation(0.1));
        assert!(!monitor.validate_energy_conservation(0.15));
    }

    #[test]
    fn test_detect_stagnation() {
        let monitor = HealthMonitor::new();

        // Non-stagnant history
        let growing_history: Vec<Float> = (0..100).map(|i| 0.5 + i as Float * 0.001).collect();
        assert!(!monitor.detect_stagnation(&growing_history));

        // Stagnant history
        let stagnant_history: Vec<Float> = vec![0.5; 100];
        assert!(monitor.detect_stagnation(&stagnant_history));
    }

    #[test]
    fn test_detect_stagnation_with_steps() {
        let monitor = HealthMonitor::new();

        // Create history that stagnates after 70 steps
        let mut history: Vec<Float> = (0..70).map(|i| 0.5 + i as Float * 0.001).collect();
        history.extend(vec![0.57; 50]);

        let steps = monitor.detect_stagnation_with_steps(&history);
        // Should detect 49 stagnant steps (the 50 values of 0.57, minus 1 for the transition from 0.569 to 0.57)
        // However, 49 < 50 (stagnation_threshold), so it returns None
        assert!(
            steps.is_none(),
            "Expected None (stagnation < threshold), got {:?}",
            steps
        );

        // Create history with 51 stagnant steps (above threshold)
        let mut history2: Vec<Float> = (0..70).map(|i| 0.5 + i as Float * 0.001).collect();
        history2.extend(vec![0.57; 51]);

        let steps2 = monitor.detect_stagnation_with_steps(&history2);
        // Should detect 50 stagnant steps (above threshold of 50)
        assert!(
            steps2.is_some(),
            "Expected Some(stagnant_steps) but got None"
        );
        assert!(
            steps2.unwrap() >= 50,
            "Expected at least 50 stagnant steps, got {:?}",
            steps2
        );
    }

    #[test]
    fn test_check_health_healthy() {
        let monitor = HealthMonitor::new();

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
                biological_score: 0.5,
                noospheric_score: 0.5,
                gaia_score: 0.5,
                overall_score: 0.5,
            },
            stability_metrics: StabilityMetrics {
                coherence_stability: 0.8,
                energy_balance: 0.9,
                resilience: 0.7,
                overall_score: 0.8,
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

        assert_eq!(monitor.check_health(&result), HealthStatus::Healthy);
    }

    #[test]
    fn test_check_health_unhealthy_coherence() {
        let monitor = HealthMonitor::new();

        let mut result = SimulationResult {
            simulation_id: 1,
            config: SimulationConfig::new(1),
            completed: true,
            steps_executed: 100,
            final_coherence: 0.7, // Below threshold
            average_coherence: 0.75,
            coherence_history: vec![0.7; 100],
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
                resilience: 0.7,
                overall_score: 0.8,
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

        assert_eq!(monitor.check_health(&result), HealthStatus::Unhealthy);
    }

    #[test]
    fn test_detect_failure_coherence_violation() {
        let monitor = HealthMonitor::new();

        let result = SimulationResult {
            simulation_id: 1,
            config: SimulationConfig::new(1),
            completed: true,
            steps_executed: 100,
            final_coherence: 0.6,
            average_coherence: 0.65,
            coherence_history: vec![0.6; 100],
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
                resilience: 0.7,
                overall_score: 0.8,
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

        let failure = monitor.detect_failure(&result);
        assert!(failure.is_some());
        match failure {
            Some(FailureType::CoherenceViolation(value)) => assert_eq!(value, 0.6),
            _ => panic!("Expected CoherenceViolation"),
        }
    }

    #[test]
    fn test_filter_failures() {
        let monitor = HealthMonitor::new();

        let mut results = Vec::new();

        // Add healthy result
        results.push(SimulationResult {
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
                biological_score: 0.5,
                noospheric_score: 0.5,
                gaia_score: 0.5,
                overall_score: 0.5,
            },
            stability_metrics: StabilityMetrics {
                coherence_stability: 0.8,
                energy_balance: 0.9,
                resilience: 0.7,
                overall_score: 0.8,
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

        // Add failed result
        results.push(SimulationResult {
            simulation_id: 2,
            config: SimulationConfig::new(2),
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
        });

        let (successful, failed) = monitor.filter_failures(results);
        assert_eq!(successful.len(), 1);
        assert_eq!(failed.len(), 1);
    }
}
