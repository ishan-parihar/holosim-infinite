//! HPO (Hyperparameter Optimization) System Types
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 1:
//! "Build automated universe parameter optimization system that runs multiple parallel
//! simulations, detects failures, and keeps what works through survival of the fittest."

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// Floating-point type used throughout the simulation
pub type Float = f64;

/// Unique identifier for a simulation run
pub type SimulationId = u64;

/// Unique identifier for a parameter configuration
pub type ConfigId = u64;

/// Unique identifier for an entity
pub type EntityId = u64;

/// Simulation configuration for a single run
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Section 2.1:
/// "Parameter Space: Spectrum ratios, veil thickness, archetypical mind configuration,
/// density transition rates, catalyst generation rates"
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationConfig {
    /// Unique identifier for this configuration
    pub config_id: ConfigId,

    /// Number of entities in the simulation
    pub num_entities: usize,

    /// Number of steps to run the simulation
    pub num_steps: usize,

    /// Space/Time spectrum ratio (typically > 1.0)
    pub space_time_ratio: Float,

    /// Time/Space spectrum ratio (typically < 1.0, inverse of space_time_ratio)
    pub time_space_ratio: Float,

    /// Veil thickness (0.0 = no veil, 1.0 = complete separation)
    pub veil_thickness: Float,

    /// Number of archetypes in the mind system (10 or 22)
    pub num_archetypes: usize,

    /// Density transition rates for each density (1st-8th)
    pub density_transition_rates: Vec<Float>,

    /// Catalyst generation rate (probability per step)
    pub catalyst_generation_rate: Float,

    /// Random seed for reproducibility
    pub seed: u64,

    /// Optional metadata about this configuration
    pub metadata: HashMap<String, String>,
}

impl SimulationConfig {
    /// Create a new simulation configuration with default values
    pub fn new(config_id: ConfigId) -> Self {
        Self {
            config_id,
            num_entities: 128,
            num_steps: 500,
            space_time_ratio: 1.5,
            time_space_ratio: 0.67,
            veil_thickness: 0.5,
            num_archetypes: 22,
            density_transition_rates: vec![0.1; 8], // Default rates for 8 densities
            catalyst_generation_rate: 0.05,
            seed: config_id,
            metadata: HashMap::new(),
        }
    }

    /// Create a builder for constructing simulation configurations
    pub fn builder() -> SimulationConfigBuilder {
        SimulationConfigBuilder::new()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.num_entities == 0 {
            return Err(ConfigError::InvalidValue("num_entities", "must be > 0"));
        }
        if self.num_steps == 0 {
            return Err(ConfigError::InvalidValue("num_steps", "must be > 0"));
        }
        if self.space_time_ratio <= 0.0 {
            return Err(ConfigError::InvalidValue("space_time_ratio", "must be > 0"));
        }
        if self.time_space_ratio <= 0.0 {
            return Err(ConfigError::InvalidValue("time_space_ratio", "must be > 0"));
        }
        if !(0.0..=1.0).contains(&self.veil_thickness) {
            return Err(ConfigError::InvalidValue(
                "veil_thickness",
                "must be in [0.0, 1.0]",
            ));
        }
        if self.num_archetypes != 10 && self.num_archetypes != 22 {
            return Err(ConfigError::InvalidValue(
                "num_archetypes",
                "must be 10 or 22",
            ));
        }
        if self.density_transition_rates.len() != 8 {
            return Err(ConfigError::InvalidValue(
                "density_transition_rates",
                "must have 8 values",
            ));
        }
        if !(0.0..=1.0).contains(&self.catalyst_generation_rate) {
            return Err(ConfigError::InvalidValue(
                "catalyst_generation_rate",
                "must be in [0.0, 1.0]",
            ));
        }
        Ok(())
    }
}

/// Builder for constructing SimulationConfig
pub struct SimulationConfigBuilder {
    config: SimulationConfig,
}

impl SimulationConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: SimulationConfig::new(0),
        }
    }

    pub fn with_config_id(mut self, config_id: ConfigId) -> Self {
        self.config.config_id = config_id;
        self
    }

    pub fn with_num_entities(mut self, num_entities: usize) -> Self {
        self.config.num_entities = num_entities;
        self
    }

    pub fn with_num_steps(mut self, num_steps: usize) -> Self {
        self.config.num_steps = num_steps;
        self
    }

    pub fn with_space_time_ratio(mut self, ratio: Float) -> Self {
        self.config.space_time_ratio = ratio;
        self
    }

    pub fn with_time_space_ratio(mut self, ratio: Float) -> Self {
        self.config.time_space_ratio = ratio;
        self
    }

    pub fn with_veil_thickness(mut self, thickness: Float) -> Self {
        self.config.veil_thickness = thickness;
        self
    }

    pub fn with_num_archetypes(mut self, num_archetypes: usize) -> Self {
        self.config.num_archetypes = num_archetypes;
        self
    }

    pub fn with_density_transition_rates(mut self, rates: Vec<Float>) -> Self {
        self.config.density_transition_rates = rates;
        self
    }

    pub fn with_catalyst_generation_rate(mut self, rate: Float) -> Self {
        self.config.catalyst_generation_rate = rate;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = seed;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.config.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> Result<SimulationConfig, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for SimulationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of running a single simulation
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationResult {
    /// Simulation identifier
    pub simulation_id: SimulationId,

    /// Configuration used for this simulation
    pub config: SimulationConfig,

    /// Whether the simulation completed successfully
    pub completed: bool,

    /// Number of steps actually executed
    pub steps_executed: usize,

    /// Final coherence value (0.0 to 1.0)
    pub final_coherence: Float,

    /// Average coherence over the simulation
    pub average_coherence: Float,

    /// Coherence history over time
    pub coherence_history: Vec<Float>,

    /// Energy conservation error (should be close to 0.0)
    pub energy_conservation_error: Float,

    /// Number of entities that evolved
    pub entities_evolved: usize,

    /// Number of entities that reached higher densities
    pub entities_harvested: usize,

    /// Emergence metrics
    pub emergence_metrics: EmergenceMetrics,

    /// Stability metrics
    pub stability_metrics: StabilityMetrics,

    /// Complexity metrics
    pub complexity_metrics: ComplexityMetrics,

    /// Overall fitness score (0.0 to 1.0)
    pub fitness_score: Float,

    /// Time taken to run the simulation (in seconds)
    pub execution_time: Float,

    /// Any failure that occurred
    pub failure: Option<FailureType>,

    /// Optional metadata about the result
    pub metadata: HashMap<String, String>,
}

/// Metrics related to emergence (biological, noospheric, Gaia)
#[derive(Debug, Clone, PartialEq)]
pub struct EmergenceMetrics {
    /// Biological emergence score (0.0 to 1.0)
    pub biological_score: Float,

    /// Noospheric emergence score (0.0 to 1.0)
    pub noospheric_score: Float,

    /// Gaia emergence score (0.0 to 1.0)
    pub gaia_score: Float,

    /// Overall emergence score (weighted combination)
    pub overall_score: Float,
}

impl EmergenceMetrics {
    /// Calculate overall emergence score from components
    pub fn calculate_overall(&self) -> Float {
        let biological_weight = 0.4;
        let noospheric_weight = 0.3;
        let gaia_weight = 0.3;

        biological_weight * self.biological_score
            + noospheric_weight * self.noospheric_score
            + gaia_weight * self.gaia_score
    }
}

/// Metrics related to system stability
#[derive(Debug, Clone, PartialEq)]
pub struct StabilityMetrics {
    /// Coherence stability (0.0 to 1.0)
    pub coherence_stability: Float,

    /// Energy balance (0.0 to 1.0)
    pub energy_balance: Float,

    /// System resilience (0.0 to 1.0)
    pub resilience: Float,

    /// Overall stability score (weighted combination)
    pub overall_score: Float,
}

/// Metrics related to system complexity
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexityMetrics {
    /// Diversity score (entropy-based, 0.0 to 1.0)
    pub diversity_score: Float,

    /// Integration score (0.0 to 1.0)
    pub integration_score: Float,

    /// Depth score (0.0 to 1.0)
    pub depth_score: Float,

    /// Overall complexity score (weighted combination)
    pub overall_score: Float,
}

/// Types of simulation failures
#[derive(Debug, Clone, PartialEq)]
pub enum FailureType {
    /// Coherence dropped below threshold
    CoherenceViolation(Float),

    /// Energy was not conserved
    EnergyConservationViolation(Float),

    /// Simulation stagnated (no progress)
    Stagnation(usize),

    /// Simulation crashed (panic or error)
    Crash(String),

    /// Timeout (took too long)
    Timeout(Float),

    /// Invalid parameter configuration
    InvalidConfig(String),
}

impl Display for FailureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FailureType::CoherenceViolation(value) => {
                write!(f, "Coherence violation: coherence dropped to {}", value)
            }
            FailureType::EnergyConservationViolation(error) => {
                write!(f, "Energy conservation violation: error = {}", error)
            }
            FailureType::Stagnation(steps) => {
                write!(f, "Stagnation: no progress for {} steps", steps)
            }
            FailureType::Crash(message) => write!(f, "Crash: {}", message),
            FailureType::Timeout(time) => write!(f, "Timeout: exceeded {} seconds", time),
            FailureType::InvalidConfig(message) => write!(f, "Invalid configuration: {}", message),
        }
    }
}

/// Health status of a running simulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    /// Simulation is healthy and progressing normally
    Healthy,

    /// Simulation is unhealthy but recoverable
    Unhealthy,

    /// Simulation has failed critically
    Failed,
}

/// Fitness score for ranking simulation results
#[derive(Debug, Clone, Copy)]
pub struct FitnessScore(pub Float);

impl PartialEq for FitnessScore {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < Float::EPSILON
    }
}

impl PartialOrd for FitnessScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl FitnessScore {
    /// Create a new fitness score
    pub fn new(score: Float) -> Self {
        FitnessScore(score.clamp(0.0, 1.0))
    }

    /// Get the raw score value
    pub fn value(&self) -> Float {
        self.0
    }

    /// Check if the score is acceptable (> 0.5)
    pub fn is_acceptable(&self) -> bool {
        self.0 > 0.5
    }

    /// Check if the score is excellent (> 0.8)
    pub fn is_excellent(&self) -> bool {
        self.0 > 0.8
    }
}

impl Default for FitnessScore {
    fn default() -> Self {
        FitnessScore(0.0)
    }
}

/// Configuration errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Invalid value for a parameter
    InvalidValue(&'static str, &'static str),

    /// Missing required parameter
    MissingParameter(&'static str),

    /// Inconsistent parameters
    InconsistentParameters(&'static str),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidValue(param, reason) => {
                write!(f, "Invalid value for '{}': {}", param, reason)
            }
            ConfigError::MissingParameter(param) => {
                write!(f, "Missing required parameter: '{}'", param)
            }
            ConfigError::InconsistentParameters(reason) => {
                write!(f, "Inconsistent parameters: {}", reason)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Parameter space for configuration generation
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterSpace {
    /// Range for space/time ratio
    pub space_time_ratio_range: (Float, Float),

    /// Range for veil thickness
    pub veil_thickness_range: (Float, Float),

    /// Range for catalyst generation rate
    pub catalyst_generation_rate_range: (Float, Float),

    /// Range for density transition rates
    pub density_transition_rate_range: (Float, Float),

    /// Supported archetype configurations
    pub archetype_configs: Vec<usize>,
}

impl Default for ParameterSpace {
    fn default() -> Self {
        Self {
            space_time_ratio_range: (1.0, 3.0),
            veil_thickness_range: (0.0, 1.0),
            catalyst_generation_rate_range: (0.01, 0.2),
            density_transition_rate_range: (0.05, 0.3),
            archetype_configs: vec![10, 22],
        }
    }
}

impl ParameterSpace {
    /// Create a new parameter space with default ranges
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate that a configuration is within the parameter space
    pub fn is_valid(&self, config: &SimulationConfig) -> bool {
        config.space_time_ratio >= self.space_time_ratio_range.0
            && config.space_time_ratio <= self.space_time_ratio_range.1
            && config.veil_thickness >= self.veil_thickness_range.0
            && config.veil_thickness <= self.veil_thickness_range.1
            && config.catalyst_generation_rate >= self.catalyst_generation_rate_range.0
            && config.catalyst_generation_rate <= self.catalyst_generation_rate_range.1
            && config.density_transition_rates.iter().all(|&rate| {
                rate >= self.density_transition_rate_range.0
                    && rate <= self.density_transition_rate_range.1
            })
            && self.archetype_configs.contains(&config.num_archetypes)
    }
}

/// Tournament selection algorithm parameters
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionParameters {
    /// Size of tournament for selection
    pub tournament_size: usize,

    /// Elitism rate (fraction of top configs to keep)
    pub elitism_rate: Float,

    /// Mutation rate for parameter exploration
    pub mutation_rate: Float,

    /// Crossover rate for parameter mixing
    pub crossover_rate: Float,

    /// Number of generations without improvement before convergence
    pub convergence_threshold: usize,
}

impl Default for SelectionParameters {
    fn default() -> Self {
        Self {
            tournament_size: 5,
            elitism_rate: 0.1,
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            convergence_threshold: 10,
        }
    }
}

impl SelectionParameters {
    /// Create new selection parameters with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the parameters
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.tournament_size == 0 {
            return Err(ConfigError::InvalidValue("tournament_size", "must be > 0"));
        }
        if !(0.0..=1.0).contains(&self.elitism_rate) {
            return Err(ConfigError::InvalidValue(
                "elitism_rate",
                "must be in [0.0, 1.0]",
            ));
        }
        if !(0.0..=1.0).contains(&self.mutation_rate) {
            return Err(ConfigError::InvalidValue(
                "mutation_rate",
                "must be in [0.0, 1.0]",
            ));
        }
        if !(0.0..=1.0).contains(&self.crossover_rate) {
            return Err(ConfigError::InvalidValue(
                "crossover_rate",
                "must be in [0.0, 1.0]",
            ));
        }
        if self.convergence_threshold == 0 {
            return Err(ConfigError::InvalidValue(
                "convergence_threshold",
                "must be > 0",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_config_validation() {
        let config = SimulationConfig::new(1);
        assert!(config.validate().is_ok());

        // Test invalid num_entities
        let mut invalid_config = config.clone();
        invalid_config.num_entities = 0;
        assert!(invalid_config.validate().is_err());

        // Test invalid veil_thickness
        invalid_config = config.clone();
        invalid_config.veil_thickness = 1.5;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_simulation_config_builder() {
        let config = SimulationConfig::builder()
            .with_config_id(1)
            .with_num_entities(256)
            .with_veil_thickness(0.8)
            .with_catalyst_generation_rate(0.1)
            .build()
            .unwrap();

        assert_eq!(config.config_id, 1);
        assert_eq!(config.num_entities, 256);
        assert_eq!(config.veil_thickness, 0.8);
        assert_eq!(config.catalyst_generation_rate, 0.1);
    }

    #[test]
    fn test_fitness_score() {
        let score = FitnessScore::new(0.75);
        assert_eq!(score.value(), 0.75);
        assert!(score.is_acceptable());
        assert!(!score.is_excellent());

        let excellent = FitnessScore::new(0.9);
        assert!(excellent.is_excellent());

        // Test clamping
        let clamped = FitnessScore::new(1.5);
        assert_eq!(clamped.value(), 1.0);
    }

    #[test]
    fn test_parameter_space_validation() {
        let space = ParameterSpace::new();
        let config = SimulationConfig::new(1);
        assert!(space.is_valid(&config));

        // Test invalid config
        let mut invalid_config = config.clone();
        invalid_config.space_time_ratio = 5.0;
        assert!(!space.is_valid(&invalid_config));
    }

    #[test]
    fn test_selection_parameters_validation() {
        let params = SelectionParameters::new();
        assert!(params.validate().is_ok());

        // Test invalid tournament_size
        let mut invalid_params = params.clone();
        invalid_params.tournament_size = 0;
        assert!(invalid_params.validate().is_err());

        // Test invalid elitism_rate
        invalid_params = params.clone();
        invalid_params.elitism_rate = 1.5;
        assert!(invalid_params.validate().is_err());
    }
}
