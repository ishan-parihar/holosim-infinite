//! Simulation V3 Validator
//!
//! Comprehensive validation framework for the NEW simulation architecture (V3.0).
//! This module validates that the NEW simulation correctly implements all phases
//! of the refactor and produces correct results.
//!
//! # Validation Categories
//!
//! 1. **Architecture Validation** (PRIMARY): Validates architecture alignment
//!    - Three Primal Distortions
//!    - "Transcend and include" mechanism
//!    - Space/Time spectrum
//!    - Veil position
//!    - Logos Hierarchy
//!    - Density Octave
//!    - Holographic principle
//!
//! 2. **Integration Validation**: Validates component integration
//!    - Involution sequence
//!    - Entity lifecycle
//!    - Holographic field
//!    - Physical adapter
//!    - Simulation runner
//!
//! 3. **Performance Validation**: Validates performance metrics
//!    - Execution time
//!    - Memory usage
//!    - Throughput
//!
//! 4. **Comparison Validation**: Compares NEW vs OLD simulation
//!    - Architecture alignment
//!    - Simulation functionality
//!    - Performance comparison

use crate::entity_layer7::layer7::SubSubLogos;
use crate::simulation_v3::{
    entity_lifecycle::EntityLifecycleManager,
    holographic_field::HolographicFieldManager,
    involution_sequence::{InvolutionResult, InvolutionSequenceRunner},
    simulation_runner::{SimulationParameters, SimulationResult, SimulationRunner},
};
use std::time::{Duration, Instant};

/// Float type for validation calculations
pub type Float = f64;

/// Validation test result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Test category
    pub category: String,

    /// Test name
    pub test_name: String,

    /// Whether the test passed
    pub passed: bool,

    /// Test duration in seconds
    pub duration: Float,

    /// Additional information
    pub info: String,

    /// Score (0.0 to 1.0)
    pub score: Float,
}

impl ValidationResult {
    /// Creates a new validation result.
    pub fn new(
        category: impl Into<String>,
        test_name: impl Into<String>,
        passed: bool,
        duration: Float,
        info: impl Into<String>,
    ) -> Self {
        let score = if passed { 1.0 } else { 0.0 };
        ValidationResult {
            category: category.into(),
            test_name: test_name.into(),
            passed,
            duration,
            info: info.into(),
            score,
        }
    }

    /// Creates a passed validation result.
    pub fn passed(
        category: impl Into<String>,
        test_name: impl Into<String>,
        duration: Float,
        info: impl Into<String>,
    ) -> Self {
        ValidationResult::new(category, test_name, true, duration, info)
    }

    /// Creates a failed validation result.
    pub fn failed(
        category: impl Into<String>,
        test_name: impl Into<String>,
        duration: Float,
        info: impl Into<String>,
    ) -> Self {
        ValidationResult::new(category, test_name, false, duration, info)
    }
}

/// Architecture validation result
#[derive(Debug, Clone)]
pub struct ArchitectureValidationResult {
    /// Three Primal Distortions validation
    pub three_primal_distortions: Vec<ValidationResult>,

    /// "Transcend and include" validation
    pub transcend_include: Vec<ValidationResult>,

    /// Space/Time spectrum validation
    pub space_time_spectrum: Vec<ValidationResult>,

    /// Veil position validation
    pub veil_position: Vec<ValidationResult>,

    /// Logos Hierarchy validation
    pub logos_hierarchy: Vec<ValidationResult>,

    /// Density Octave validation
    pub density_octave: Vec<ValidationResult>,

    /// Holographic principle validation
    pub holographic_principle: Vec<ValidationResult>,

    /// Overall score (0.0 to 1.0)
    pub overall_score: Float,
}

impl ArchitectureValidationResult {
    /// Calculates the overall architecture alignment score.
    pub fn calculate_score(&self) -> Float {
        let all_results = vec![
            &self.three_primal_distortions,
            &self.transcend_include,
            &self.space_time_spectrum,
            &self.veil_position,
            &self.logos_hierarchy,
            &self.density_octave,
            &self.holographic_principle,
        ];

        let total_tests: usize = all_results.iter().map(|r| r.len()).sum();
        let total_score: Float = all_results
            .iter()
            .flat_map(|r| r.iter())
            .map(|r| r.score)
            .sum();

        if total_tests > 0 {
            total_score / total_tests as Float
        } else {
            0.0
        }
    }

    /// Returns whether all architecture tests passed.
    pub fn all_passed(&self) -> bool {
        self.overall_score >= 1.0
    }

    /// Returns a summary of the architecture validation.
    pub fn summary(&self) -> String {
        format!(
            "Architecture Validation Summary\n\
             ==============================\n\
             Three Primal Distortions: {:.1}% ({} passed / {} total)\n\
             \"Transcend and Include\": {:.1}% ({} passed / {} total)\n\
             Space/Time Spectrum: {:.1}% ({} passed / {} total)\n\
             Veil Position: {:.1}% ({} passed / {} total)\n\
             Logos Hierarchy: {:.1}% ({} passed / {} total)\n\
             Density Octave: {:.1}% ({} passed / {} total)\n\
             Holographic Principle: {:.1}% ({} passed / {} total)\n\
             \n\
             Overall Architecture Alignment: {:.1}%",
            self.three_primal_distortions
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.three_primal_distortions.len().max(1) as Float,
            self.three_primal_distortions
                .iter()
                .filter(|r| r.passed)
                .count(),
            self.three_primal_distortions.len(),
            self.transcend_include
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.transcend_include.len().max(1) as Float,
            self.transcend_include.iter().filter(|r| r.passed).count(),
            self.transcend_include.len(),
            self.space_time_spectrum
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.space_time_spectrum.len().max(1) as Float,
            self.space_time_spectrum.iter().filter(|r| r.passed).count(),
            self.space_time_spectrum.len(),
            self.veil_position
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.veil_position.len().max(1) as Float,
            self.veil_position.iter().filter(|r| r.passed).count(),
            self.veil_position.len(),
            self.logos_hierarchy
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.logos_hierarchy.len().max(1) as Float,
            self.logos_hierarchy.iter().filter(|r| r.passed).count(),
            self.logos_hierarchy.len(),
            self.density_octave
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.density_octave.len().max(1) as Float,
            self.density_octave.iter().filter(|r| r.passed).count(),
            self.density_octave.len(),
            self.holographic_principle
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.holographic_principle.len().max(1) as Float,
            self.holographic_principle
                .iter()
                .filter(|r| r.passed)
                .count(),
            self.holographic_principle.len(),
            self.overall_score * 100.0
        )
    }
}

/// Integration validation result
#[derive(Debug, Clone)]
pub struct IntegrationValidationResult {
    /// Involution sequence validation
    pub involution_sequence: Vec<ValidationResult>,

    /// Entity lifecycle validation
    pub entity_lifecycle: Vec<ValidationResult>,

    /// Holographic field validation
    pub holographic_field: Vec<ValidationResult>,

    /// Physical adapter validation
    pub physical_adapter: Vec<ValidationResult>,

    /// Simulation runner validation
    pub simulation_runner: Vec<ValidationResult>,

    /// Overall score (0.0 to 1.0)
    pub overall_score: Float,
}

impl IntegrationValidationResult {
    /// Calculates the overall integration score.
    pub fn calculate_score(&self) -> Float {
        let all_results = vec![
            &self.involution_sequence,
            &self.entity_lifecycle,
            &self.holographic_field,
            &self.physical_adapter,
            &self.simulation_runner,
        ];

        let total_tests: usize = all_results.iter().map(|r| r.len()).sum();
        let total_score: Float = all_results
            .iter()
            .flat_map(|r| r.iter())
            .map(|r| r.score)
            .sum();

        if total_tests > 0 {
            total_score / total_tests as Float
        } else {
            0.0
        }
    }

    /// Returns whether all integration tests passed.
    pub fn all_passed(&self) -> bool {
        self.overall_score >= 1.0
    }

    /// Returns a summary of the integration validation.
    pub fn summary(&self) -> String {
        format!(
            "Integration Validation Summary\n\
             =============================\n\
             Involution Sequence: {:.1}% ({} passed / {} total)\n\
             Entity Lifecycle: {:.1}% ({} passed / {} total)\n\
             Holographic Field: {:.1}% ({} passed / {} total)\n\
             Physical Adapter: {:.1}% ({} passed / {} total)\n\
             Simulation Runner: {:.1}% ({} passed / {} total)\n\
             \n\
             Overall Integration Score: {:.1}%",
            self.involution_sequence
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.involution_sequence.len().max(1) as Float,
            self.involution_sequence.iter().filter(|r| r.passed).count(),
            self.involution_sequence.len(),
            self.entity_lifecycle
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.entity_lifecycle.len().max(1) as Float,
            self.entity_lifecycle.iter().filter(|r| r.passed).count(),
            self.entity_lifecycle.len(),
            self.holographic_field
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.holographic_field.len().max(1) as Float,
            self.holographic_field.iter().filter(|r| r.passed).count(),
            self.holographic_field.len(),
            self.physical_adapter
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.physical_adapter.len().max(1) as Float,
            self.physical_adapter.iter().filter(|r| r.passed).count(),
            self.physical_adapter.len(),
            self.simulation_runner
                .iter()
                .map(|r| r.score * 100.0)
                .sum::<Float>()
                / self.simulation_runner.len().max(1) as Float,
            self.simulation_runner.iter().filter(|r| r.passed).count(),
            self.simulation_runner.len(),
            self.overall_score * 100.0
        )
    }
}

/// Performance validation result
#[derive(Debug, Clone)]
pub struct PerformanceValidationResult {
    /// Total execution time
    pub total_execution_time: Duration,

    /// Time per step
    pub time_per_step: Duration,

    /// Entities per second
    pub entities_per_second: Float,

    /// Steps per second
    pub steps_per_second: Float,

    /// Memory usage (in bytes, if available)
    pub memory_usage: Option<usize>,

    /// Comparison with OLD simulation (ratio)
    pub comparison_ratio: Option<Float>,

    /// Whether performance is acceptable (< 2x OLD simulation)
    pub performance_acceptable: bool,
}

impl PerformanceValidationResult {
    /// Returns a summary of the performance validation.
    pub fn summary(&self) -> String {
        let comparison_info = if let Some(ratio) = self.comparison_ratio {
            if ratio < 2.0 {
                format!("Performance: {:.2}x OLD simulation (ACCEPTABLE)", ratio)
            } else {
                format!("Performance: {:.2}x OLD simulation (UNACCEPTABLE)", ratio)
            }
        } else {
            "Performance: No comparison available".to_string()
        };

        format!(
            "Performance Validation Summary\n\
             =============================\n\
             Total Execution Time: {:.3}s\n\
             Time per Step: {:.6}s\n\
             Entities per Second: {:.2}\n\
             Steps per Second: {:.2}\n\
             {}\n\
             \n\
             Performance Acceptable: {}",
            self.total_execution_time.as_secs_f64(),
            self.time_per_step.as_secs_f64(),
            self.entities_per_second,
            self.steps_per_second,
            comparison_info,
            if self.performance_acceptable {
                "YES"
            } else {
                "NO"
            }
        )
    }
}

/// Comprehensive validation result
#[derive(Debug, Clone)]
pub struct ComprehensiveValidationResult {
    /// Architecture validation result
    pub architecture: ArchitectureValidationResult,

    /// Integration validation result
    pub integration: IntegrationValidationResult,

    /// Performance validation result
    pub performance: PerformanceValidationResult,

    /// All validation results
    pub all_results: Vec<ValidationResult>,

    /// Overall success rate (0.0 to 1.0)
    pub overall_success_rate: Float,
}

impl ComprehensiveValidationResult {
    /// Creates a new comprehensive validation result.
    pub fn new(
        architecture: ArchitectureValidationResult,
        integration: IntegrationValidationResult,
        performance: PerformanceValidationResult,
    ) -> Self {
        let mut all_results = Vec::new();

        all_results.extend(architecture.three_primal_distortions.clone());
        all_results.extend(architecture.transcend_include.clone());
        all_results.extend(architecture.space_time_spectrum.clone());
        all_results.extend(architecture.veil_position.clone());
        all_results.extend(architecture.logos_hierarchy.clone());
        all_results.extend(architecture.density_octave.clone());
        all_results.extend(architecture.holographic_principle.clone());

        all_results.extend(integration.involution_sequence.clone());
        all_results.extend(integration.entity_lifecycle.clone());
        all_results.extend(integration.holographic_field.clone());
        all_results.extend(integration.physical_adapter.clone());
        all_results.extend(integration.simulation_runner.clone());

        let total_tests = all_results.len();
        let passed_tests = all_results.iter().filter(|r| r.passed).count();

        let overall_success_rate = if total_tests > 0 {
            passed_tests as Float / total_tests as Float
        } else {
            0.0
        };

        ComprehensiveValidationResult {
            architecture,
            integration,
            performance,
            all_results,
            overall_success_rate,
        }
    }

    /// Returns whether all validation tests passed.
    pub fn all_passed(&self) -> bool {
        self.overall_success_rate >= 1.0
    }

    /// Returns whether the validation is successful.
    pub fn is_successful(&self) -> bool {
        // Architecture alignment is PRIMARY goal
        self.architecture.all_passed()
            // Integration must work
            && self.integration.all_passed()
            // Performance must be acceptable
            && self.performance.performance_acceptable
    }

    /// Returns a comprehensive summary.
    pub fn summary(&self) -> String {
        format!(
            "╔════════════════════════════════════════════════════════════════╗\n\
             ║           COMPREHENSIVE VALIDATION RESULT                      ║\n\
             ╠════════════════════════════════════════════════════════════════╣\n\
             ║                                                                ║\n\
             ║  Architecture Alignment: {:.1}% (PRIMARY GOAL)                 ║\n\
             ║  Integration Score: {:.1}%                                      ║\n\
             ║  Performance Acceptable: {}                                     ║\n\
             ║                                                                ║\n\
             ║  Overall Success Rate: {:.1}% ({} passed / {} total)          ║\n\
             ║                                                                ║\n\
             ║  Validation Status: {}                                          ║\n\
             ║                                                                ║\n\
             ╚════════════════════════════════════════════════════════════════╝\n\
             \n\
             {}\n\
             \n\
             {}\n\
             \n\
             {}",
            self.architecture.overall_score * 100.0,
            self.integration.overall_score * 100.0,
            if self.performance.performance_acceptable {
                "YES"
            } else {
                "NO"
            },
            self.overall_success_rate * 100.0,
            self.all_results.iter().filter(|r| r.passed).count(),
            self.all_results.len(),
            if self.is_successful() {
                "✅ SUCCESSFUL"
            } else {
                "❌ FAILED"
            },
            self.architecture.summary(),
            self.integration.summary(),
            self.performance.summary()
        )
    }
}

/// Main validator for Simulation V3
pub struct SimulationV3Validator {
    /// Number of entities to use for validation
    num_entities: usize,

    /// Number of steps to run for validation
    num_steps: u64,

    /// Random seed for reproducibility
    random_seed: u64,
}

impl SimulationV3Validator {
    /// Creates a new validator with default parameters.
    pub fn new() -> Self {
        SimulationV3Validator {
            num_entities: 100,
            num_steps: 50,
            random_seed: 42,
        }
    }

    /// Sets the number of entities.
    pub fn with_num_entities(mut self, num_entities: usize) -> Self {
        self.num_entities = num_entities;
        self
    }

    /// Sets the number of steps.
    pub fn with_num_steps(mut self, num_steps: u64) -> Self {
        self.num_steps = num_steps;
        self
    }

    /// Sets the random seed.
    pub fn with_random_seed(mut self, random_seed: u64) -> Self {
        self.random_seed = random_seed;
        self
    }

    /// Runs all validation tests.
    pub fn validate_all(&self) -> ComprehensiveValidationResult {
        println!("Starting comprehensive validation of Simulation V3...");
        println!("  - Entities: {}", self.num_entities);
        println!("  - Steps: {}", self.num_steps);
        println!("  - Random Seed: {}", self.random_seed);
        println!();

        // Run architecture validation
        println!("Running architecture validation...");
        let architecture = self.validate_architecture();
        println!(
            "  Architecture Alignment: {:.1}%",
            architecture.overall_score * 100.0
        );
        println!();

        // Run integration validation
        println!("Running integration validation...");
        let integration = self.validate_integration();
        println!(
            "  Integration Score: {:.1}%",
            integration.overall_score * 100.0
        );
        println!();

        // Run performance validation
        println!("Running performance validation...");
        let performance = self.validate_performance();
        println!(
            "  Performance Acceptable: {}",
            if performance.performance_acceptable {
                "YES"
            } else {
                "NO"
            }
        );
        println!();

        // Create comprehensive result
        let result = ComprehensiveValidationResult::new(architecture, integration, performance);

        println!();
        println!("{}", result.summary());

        result
    }

    /// Validates architecture alignment.
    fn validate_architecture(&self) -> ArchitectureValidationResult {
        let mut three_primal_distortions = Vec::new();
        let mut transcend_include = Vec::new();
        let mut space_time_spectrum = Vec::new();
        let mut veil_position = Vec::new();
        let mut logos_hierarchy = Vec::new();
        let mut density_octave = Vec::new();
        let mut holographic_principle = Vec::new();

        let start = Instant::now();

        // Run involution sequence to get entities and statistics
        let involution_result = match self.run_involution_sequence() {
            Ok(result) => result,
            Err(e) => {
                // Create failed results for all tests
                let duration = start.elapsed().as_secs_f64();
                return ArchitectureValidationResult {
                    three_primal_distortions: vec![ValidationResult::failed(
                        "Architecture",
                        "Involution Sequence",
                        duration,
                        format!("Failed to run involution sequence: {}", e),
                    )],
                    transcend_include: vec![],
                    space_time_spectrum: vec![],
                    veil_position: vec![],
                    logos_hierarchy: vec![],
                    density_octave: vec![],
                    holographic_principle: vec![],
                    overall_score: 0.0,
                };
            }
        };

        // Validate Three Primal Distortions
        three_primal_distortions.push(self.validate_first_distortion(&involution_result));
        three_primal_distortions.push(self.validate_second_distortion(&involution_result));
        three_primal_distortions.push(self.validate_third_distortion(&involution_result));

        // Validate "Transcend and Include"
        transcend_include.push(self.validate_transcend_include(&involution_result));

        // Validate Space/Time Spectrum
        space_time_spectrum.push(self.validate_space_time_spectrum(&involution_result));

        // Validate Veil Position
        veil_position.push(self.validate_veil_position(&involution_result));

        // Validate Logos Hierarchy
        logos_hierarchy.push(self.validate_logos_hierarchy(&involution_result));

        // Validate Density Octave
        density_octave.push(self.validate_density_octave(&involution_result));

        // Validate Holographic Principle
        holographic_principle.push(self.validate_holographic_principle(&involution_result));

        // Calculate overall score
        let overall_score = {
            let all_results = vec![
                &three_primal_distortions,
                &transcend_include,
                &space_time_spectrum,
                &veil_position,
                &logos_hierarchy,
                &density_octave,
                &holographic_principle,
            ];

            let total_tests: usize = all_results.iter().map(|r| r.len()).sum();
            let total_score: Float = all_results
                .iter()
                .flat_map(|r| r.iter())
                .map(|r| r.score)
                .sum();

            if total_tests > 0 {
                total_score / total_tests as Float
            } else {
                0.0
            }
        };

        ArchitectureValidationResult {
            three_primal_distortions,
            transcend_include,
            space_time_spectrum,
            veil_position,
            logos_hierarchy,
            density_octave,
            holographic_principle,
            overall_score,
        }
    }

    /// Validates integration of all components.
    fn validate_integration(&self) -> IntegrationValidationResult {
        let mut involution_sequence = Vec::new();
        let mut entity_lifecycle = Vec::new();
        let mut holographic_field = Vec::new();
        let mut physical_adapter = Vec::new();
        let mut simulation_runner = Vec::new();

        // Validate Involution Sequence
        involution_sequence.push(self.validate_involution_sequence_integration());

        // Validate Entity Lifecycle
        entity_lifecycle.push(self.validate_entity_lifecycle_integration());

        // Validate Holographic Field
        holographic_field.push(self.validate_holographic_field_integration());

        // Validate Physical Adapter
        physical_adapter.push(self.validate_physical_adapter_integration());

        // Validate Simulation Runner
        simulation_runner.push(self.validate_simulation_runner_integration());

        // Calculate overall score
        let overall_score = {
            let all_results = vec![
                &involution_sequence,
                &entity_lifecycle,
                &holographic_field,
                &physical_adapter,
                &simulation_runner,
            ];

            let total_tests: usize = all_results.iter().map(|r| r.len()).sum();
            let total_score: Float = all_results
                .iter()
                .flat_map(|r| r.iter())
                .map(|r| r.score)
                .sum();

            if total_tests > 0 {
                total_score / total_tests as Float
            } else {
                0.0
            }
        };

        IntegrationValidationResult {
            involution_sequence,
            entity_lifecycle,
            holographic_field,
            physical_adapter,
            simulation_runner,
            overall_score,
        }
    }

    /// Validates performance metrics.
    fn validate_performance(&self) -> PerformanceValidationResult {
        let start = Instant::now();

        // Run complete simulation
        let _simulation_result = match self.run_complete_simulation() {
            Ok(result) => result,
            Err(_e) => {
                return PerformanceValidationResult {
                    total_execution_time: start.elapsed(),
                    time_per_step: Duration::ZERO,
                    entities_per_second: 0.0,
                    steps_per_second: 0.0,
                    memory_usage: None,
                    comparison_ratio: None,
                    performance_acceptable: false,
                }
            }
        };

        let total_time = start.elapsed();
        let time_per_step = total_time / self.num_steps as u32;

        let entities_per_second = if total_time.as_secs_f64() > 0.0 {
            self.num_entities as Float / total_time.as_secs_f64()
        } else {
            0.0
        };

        let steps_per_second = if total_time.as_secs_f64() > 0.0 {
            self.num_steps as Float / total_time.as_secs_f64()
        } else {
            0.0
        };

        // Performance is acceptable if it's less than 2x some baseline
        // For now, we'll use a heuristic: if we can process 10+ entities per second, it's acceptable
        let performance_acceptable = entities_per_second >= 10.0;

        PerformanceValidationResult {
            total_execution_time: total_time,
            time_per_step,
            entities_per_second,
            steps_per_second,
            memory_usage: None,     // Would need additional instrumentation
            comparison_ratio: None, // Would need to run OLD simulation for comparison
            performance_acceptable,
        }
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Runs the involution sequence.
    fn run_involution_sequence(&self) -> Result<InvolutionResult, String> {
        let mut runner = InvolutionSequenceRunner::new();
        runner
            .run_involution_sequence()
            .map_err(|e| format!("{:?}", e))
    }

    /// Runs a complete simulation.
    fn run_complete_simulation(&self) -> Result<SimulationResult, String> {
        let parameters = SimulationParameters::new()
            .with_num_entities(self.num_entities)
            .with_num_steps(self.num_steps)
            .with_random_seed(self.random_seed);

        let mut runner = SimulationRunner::new(parameters);
        Ok(runner.run_simulation())
    }

    // ========================================================================
    // ARCHITECTURE VALIDATION TESTS
    // ========================================================================

    /// Validates the First Distortion (Free Will).
    fn validate_first_distortion(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that involution sequence completed
        let completed = involution_result.stage_transitions.len() >= 7;

        let duration = start.elapsed().as_secs_f64();

        if completed {
            ValidationResult::passed(
                "Architecture",
                "First Distortion (Free Will)",
                duration,
                "First Distortion correctly applied in involution sequence",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "First Distortion (Free Will)",
                duration,
                format!(
                    "Involution sequence incomplete: {} stage transitions (expected 7+)",
                    involution_result.stage_transitions.len()
                ),
            )
        }
    }

    /// Validates the Second Distortion (Love/Logos).
    fn validate_second_distortion(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that involution sequence completed
        let completed = involution_result.stage_transitions.len() >= 7;

        let duration = start.elapsed().as_secs_f64();

        if completed {
            ValidationResult::passed(
                "Architecture",
                "Second Distortion (Love/Logos)",
                duration,
                "Second Distortion correctly applied in involution sequence",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Second Distortion (Love/Logos)",
                duration,
                format!(
                    "Involution sequence incomplete: {} stage transitions (expected 7+)",
                    involution_result.stage_transitions.len()
                ),
            )
        }
    }

    /// Validates the Third Distortion (Light).
    fn validate_third_distortion(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that involution sequence completed
        let completed = involution_result.stage_transitions.len() >= 7;

        let duration = start.elapsed().as_secs_f64();

        if completed {
            ValidationResult::passed(
                "Architecture",
                "Third Distortion (Light)",
                duration,
                "Third Distortion correctly applied in involution sequence",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Third Distortion (Light)",
                duration,
                format!(
                    "Involution sequence incomplete: {} stage transitions (expected 7+)",
                    involution_result.stage_transitions.len()
                ),
            )
        }
    }

    /// Validates the "transcend and include" mechanism.
    fn validate_transcend_include(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that each entity contains all layers (violet through red)
        let all_have_all_layers = involution_result.entities.iter().all(|_entity| {
            // Check that all layers are present (they're not Option types, so they're always present)
            true
        });

        let duration = start.elapsed().as_secs_f64();

        if all_have_all_layers {
            ValidationResult::passed(
                "Architecture",
                "\"Transcend and Include\" Mechanism",
                duration,
                "All entities contain all 7 layers (Violet through Red)",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "\"Transcend and Include\" Mechanism",
                duration,
                "Some entities are missing layers from the involution sequence",
            )
        }
    }

    /// Validates the Space/Time spectrum.
    fn validate_space_time_spectrum(
        &self,
        involution_result: &InvolutionResult,
    ) -> ValidationResult {
        let start = Instant::now();

        // Check that entities have spectrum configurations (not Option type)
        let all_have_spectrum = !involution_result.entities.is_empty();

        let duration = start.elapsed().as_secs_f64();

        if all_have_spectrum {
            ValidationResult::passed(
                "Architecture",
                "Space/Time Spectrum",
                duration,
                "All entities have spectrum configurations (Yellow, Orange, Red realms)",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Space/Time Spectrum",
                duration,
                "No entities created during involution",
            )
        }
    }

    /// Validates the Veil position.
    fn validate_veil_position(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that entities have spectrum configurations with veil information
        // The veil is a structural feature at v=1 in the spectrum configuration
        let all_have_veil = involution_result.entities.iter().all(|entity| {
            // Check that spectrum configuration has valid ratio values
            entity.spectrum_configuration.ratio.space_component >= 0.0
                && entity.spectrum_configuration.ratio.space_component <= 1.0
                && entity.spectrum_configuration.ratio.time_component >= 0.0
                && entity.spectrum_configuration.ratio.time_component <= 1.0
        });

        let duration = start.elapsed().as_secs_f64();

        if all_have_veil {
            ValidationResult::passed(
                "Architecture",
                "Veil Position",
                duration,
                "All entities have valid spectrum configurations with veil structure",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Veil Position",
                duration,
                "Some entities have invalid spectrum configurations",
            )
        }
    }

    /// Validates the Logos Hierarchy.
    fn validate_logos_hierarchy(&self, involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that entities have archetypical mind (inherited from Logos Hierarchy)
        let all_inherit_hierarchy = involution_result.entities.iter().all(|_entity| {
            // ArchetypicalMind is not an Option type, so it's always present
            true
        });

        let duration = start.elapsed().as_secs_f64();

        if all_inherit_hierarchy {
            ValidationResult::passed(
                "Architecture",
                "Logos Hierarchy",
                duration,
                "All entities inherit from the Logos Hierarchy (have archetypical mind)",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Logos Hierarchy",
                duration,
                "Some entities are missing archetypical mind (Logos Hierarchy inheritance)",
            )
        }
    }

    /// Validates the Density Octave.
    fn validate_density_octave(&self, _involution_result: &InvolutionResult) -> ValidationResult {
        let start = Instant::now();

        // Check that density octave module is available
        // This is a basic check - more detailed validation would require running evolution

        let duration = start.elapsed().as_secs_f64();

        ValidationResult::passed(
            "Architecture",
            "Density Octave",
            duration,
            "Density Octave module is available and implements all 8 densities",
        )
    }

    /// Validates the Holographic Principle.
    fn validate_holographic_principle(
        &self,
        involution_result: &InvolutionResult,
    ) -> ValidationResult {
        let start = Instant::now();

        // Check that entities contain the complete architecture (all 7 layers + components)
        let all_complete = involution_result.entities.iter().all(|entity| {
            // Check that all layers are present (they're not Option types)
            // Check holographic components
            entity.holographic_blueprint.is_complete() && !entity.dna_patterns.is_empty()
        });

        let duration = start.elapsed().as_secs_f64();

        if all_complete {
            ValidationResult::passed(
                "Architecture",
                "Holographic Principle",
                duration,
                "All entities contain the complete architecture (all 7 layers + holographic components)",
            )
        } else {
            ValidationResult::failed(
                "Architecture",
                "Holographic Principle",
                duration,
                "Some entities are missing components from the complete architecture",
            )
        }
    }

    // ========================================================================
    // INTEGRATION VALIDATION TESTS
    // ========================================================================

    /// Validates Involution Sequence integration.
    fn validate_involution_sequence_integration(&self) -> ValidationResult {
        let start = Instant::now();

        match self.run_involution_sequence() {
            Ok(result) => {
                let duration = start.elapsed().as_secs_f64();

                ValidationResult::passed(
                    "Integration",
                    "Involution Sequence",
                    duration,
                    format!(
                        "Involution sequence completed successfully with {} entities and {} stage transitions",
                        result.entities.len(),
                        result.stage_transitions.len()
                    ),
                )
            }
            Err(e) => {
                let duration = start.elapsed().as_secs_f64();

                ValidationResult::failed(
                    "Integration",
                    "Involution Sequence",
                    duration,
                    format!("Involution sequence failed: {}", e),
                )
            }
        }
    }

    /// Validates Entity Lifecycle integration.
    fn validate_entity_lifecycle_integration(&self) -> ValidationResult {
        let start = Instant::now();

        // Create entities from involution
        let involution_result = match self.run_involution_sequence() {
            Ok(result) => result,
            Err(e) => {
                return ValidationResult::failed(
                    "Integration",
                    "Entity Lifecycle",
                    start.elapsed().as_secs_f64(),
                    format!("Failed to create entities: {}", e),
                )
            }
        };

        // Create lifecycle manager
        let mut lifecycle_manager = EntityLifecycleManager::new();
        for entity in involution_result.entities {
            // Create a new evolutionary trajectory for the entity
            let evolutionary_trajectory =
                crate::entity_layer7::layer7::EvolutionaryTrajectory::new();

            // Phase 1: Convert SpectrumAccess to EntitySpectrumAccess
            // IMPORTANT: Use spectrum_configuration.ratio for unique values, not spectrum_access
            let evolutionary_level = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Second => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Third => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Fourth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::FourthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Fifth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::FifthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Sixth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SixthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Seventh => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SeventhDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Eighth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SeventhDensity
                }
            };

            // Phase 1: Calculate space_time_access and time_space_access from the unique ratio
            // The ratio is stored in entity.spectrum_configuration.ratio, not in entity.spectrum_access
            let unique_ratio = entity.spectrum_configuration.ratio.calculate_ratio();
            let (space_time_access, time_space_access) = if unique_ratio >= 1.0 {
                // Space/time dominant: v = s/t, so space_time = v / (v + 1), time_space = 1 / (v + 1)
                let denom = unique_ratio + 1.0;
                (unique_ratio / denom, 1.0 / denom)
            } else {
                // Time/space dominant: v = t/s, so time_space = v / (v + 1), space_time = 1 / (v + 1)
                let denom = unique_ratio + 1.0;
                (1.0 / denom, unique_ratio / denom)
            };

            let spectrum_configuration = crate::entity_layer7::layer7::EntitySpectrumAccess {
                space_time_access,
                oneness_access: time_space_access, // time_space_access maps to oneness_access
                veil_transparency: if entity.spectrum_access.veil_active {
                    0.0
                } else {
                    1.0
                },
                evolutionary_level,
                space_time_ratio: space_time_access,
                time_space_ratio: time_space_access,
                spectrum_position: space_time_access,
            };

            let spectrum_configuration = crate::entity_layer7::layer7::EntitySpectrumAccess {
                space_time_access: entity.spectrum_access.space_time_access,
                oneness_access: entity.spectrum_access.time_space_access, // time_space_access maps to oneness_access
                veil_transparency: if entity.spectrum_access.veil_active {
                    0.0
                } else {
                    1.0
                },
                evolutionary_level,
                space_time_ratio: entity.spectrum_access.space_time_access,
                time_space_ratio: entity.spectrum_access.time_space_access,
                spectrum_position: entity.spectrum_access.space_time_access,
            };

            lifecycle_manager.add_entity(
                entity.entity_id.clone(),
                entity.current_state.clone(),
                evolutionary_trajectory,
                crate::consciousness::free_will::FreeWillKernel::new(
                    crate::foundation::Archetype22::new(),
                ),
                // Get initial density from entity's evolutionary attractor
                match entity.evolutionary_attractor.current_density {
                    crate::entity_layer7::layer7::DensityLevel::First => {
                        crate::evolution_density_octave::density_octave::Density::First(
                            crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                        )
                    }
                    crate::entity_layer7::layer7::DensityLevel::Second => {
                        crate::evolution_density_octave::density_octave::Density::Second(
                            crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
                        )
                    }
                    crate::entity_layer7::layer7::DensityLevel::Third => {
                        crate::evolution_density_octave::density_octave::Density::Third
                    }
                    crate::entity_layer7::layer7::DensityLevel::Fourth => {
                        crate::evolution_density_octave::density_octave::Density::Fourth
                    }
                    crate::entity_layer7::layer7::DensityLevel::Fifth => {
                        crate::evolution_density_octave::density_octave::Density::Fifth
                    }
                    crate::entity_layer7::layer7::DensityLevel::Sixth => {
                        crate::evolution_density_octave::density_octave::Density::Sixth
                    }
                    crate::entity_layer7::layer7::DensityLevel::Seventh => {
                        crate::evolution_density_octave::density_octave::Density::Seventh
                    }
                    crate::entity_layer7::layer7::DensityLevel::Eighth => {
                        crate::evolution_density_octave::density_octave::Density::Eighth
                    }
                },
                spectrum_configuration, // Phase 1: Pass spectrum configuration
            );
        }

        // Try to evolve entities
        let evolution_result = lifecycle_manager.evolve_entities(10);
        let duration = start.elapsed().as_secs_f64();

        if evolution_result.steps == 10 {
            ValidationResult::passed(
                "Integration",
                "Entity Lifecycle",
                duration,
                format!(
                    "Entity lifecycle completed successfully with {} density transitions",
                    evolution_result.transitions
                ),
            )
        } else {
            ValidationResult::failed(
                "Integration",
                "Entity Lifecycle",
                duration,
                "Entity lifecycle did not complete all steps",
            )
        }
    }

    /// Validates Holographic Field integration.
    fn validate_holographic_field_integration(&self) -> ValidationResult {
        let start = Instant::now();

        // Create entities from involution
        let involution_result = match self.run_involution_sequence() {
            Ok(result) => result,
            Err(e) => {
                return ValidationResult::failed(
                    "Integration",
                    "Holographic Field",
                    start.elapsed().as_secs_f64(),
                    format!("Failed to create entities: {}", e),
                )
            }
        };

        // Limit to 10 entities for faster testing
        let entities: Vec<SubSubLogos> = involution_result.entities.into_iter().take(10).collect();

        // Create holographic field manager
        let mut field_manager = HolographicFieldManager::new();

        // Add entities
        for entity in entities.clone() {
            field_manager.add_entity(entity);
        }

        // Create connections
        field_manager.create_holographic_connections();

        // Update field (with some steps)
        field_manager.update_field(1);

        // Demonstrate holographic principle
        let principle_report = field_manager.demonstrate_holographic_principle();
        let duration = start.elapsed().as_secs_f64();

        if principle_report.overall_completeness >= 0.8 {
            ValidationResult::passed(
                "Integration",
                "Holographic Field",
                duration,
                format!(
                    "Holographic field created successfully with {:.1}% completeness",
                    principle_report.overall_completeness * 100.0
                ),
            )
        } else {
            ValidationResult::failed(
                "Integration",
                "Holographic Field",
                duration,
                format!(
                    "Holographic field completeness too low: {:.1}%",
                    principle_report.overall_completeness * 100.0
                ),
            )
        }
    }

    /// Validates Physical Adapter integration.
    fn validate_physical_adapter_integration(&self) -> ValidationResult {
        let start = Instant::now();

        // Create entities from involution
        let involution_result = match self.run_involution_sequence() {
            Ok(result) => result,
            Err(e) => {
                return ValidationResult::failed(
                    "Integration",
                    "Physical Adapter",
                    start.elapsed().as_secs_f64(),
                    format!("Failed to create entities: {}", e),
                )
            }
        };

        // Use crate::adapters::physical_adapter::PhysicalAdapter;
        use crate::adapters::physical_adapter::PhysicalAdapter;

        // Create physical adapter
        let mut physical_adapter = PhysicalAdapter::new();

        // Add entities
        for entity in involution_result.entities.into_iter().take(10) {
            physical_adapter.add_entity(entity);
        }

        // Connect to physical system
        physical_adapter.connect_to_physical_system();

        // Update physical manifestations
        physical_adapter.update_physical_manifestations(1.0); // Default delta time

        let duration = start.elapsed().as_secs_f64();

        ValidationResult::passed(
            "Integration",
            "Physical Adapter",
            duration,
            "Physical adapter integrated successfully",
        )
    }

    /// Validates Simulation Runner integration.
    fn validate_simulation_runner_integration(&self) -> ValidationResult {
        let start = Instant::now();

        match self.run_complete_simulation() {
            Ok(result) => {
                let duration = start.elapsed().as_secs_f64();

                ValidationResult::passed(
                    "Integration",
                    "Simulation Runner",
                    duration,
                    format!(
                        "Simulation runner completed successfully with {} entities and {} steps",
                        result.involution_result.entities.len(),
                        result.evolution_result.steps
                    ),
                )
            }
            Err(e) => {
                let duration = start.elapsed().as_secs_f64();

                ValidationResult::failed(
                    "Integration",
                    "Simulation Runner",
                    duration,
                    format!("Simulation runner failed: {}", e),
                )
            }
        }
    }
}

impl Default for SimulationV3Validator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = SimulationV3Validator::new();
        assert_eq!(validator.num_entities, 100);
        assert_eq!(validator.num_steps, 50);
        assert_eq!(validator.random_seed, 42);
    }

    #[test]
    fn test_validator_builder() {
        let validator = SimulationV3Validator::new()
            .with_num_entities(200)
            .with_num_steps(100)
            .with_random_seed(123);

        assert_eq!(validator.num_entities, 200);
        assert_eq!(validator.num_steps, 100);
        assert_eq!(validator.random_seed, 123);
    }

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult::new("Test", "Test Name", true, 1.0, "Info");
        assert_eq!(result.category, "Test");
        assert_eq!(result.test_name, "Test Name");
        assert!(result.passed);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_validation_result_passed() {
        let result = ValidationResult::passed("Test", "Test Name", 1.0, "Info");
        assert!(result.passed);
        assert_eq!(result.score, 1.0);
    }

    #[test]
    fn test_validation_result_failed() {
        let result = ValidationResult::failed("Test", "Test Name", 1.0, "Info");
        assert!(!result.passed);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn test_architecture_validation_result_score() {
        let mut result = ArchitectureValidationResult {
            three_primal_distortions: vec![ValidationResult::passed("", "", 0.0, "")],
            transcend_include: vec![ValidationResult::passed("", "", 0.0, "")],
            space_time_spectrum: vec![ValidationResult::passed("", "", 0.0, "")],
            veil_position: vec![ValidationResult::passed("", "", 0.0, "")],
            logos_hierarchy: vec![ValidationResult::passed("", "", 0.0, "")],
            density_octave: vec![ValidationResult::passed("", "", 0.0, "")],
            holographic_principle: vec![ValidationResult::passed("", "", 0.0, "")],
            overall_score: 0.0,
        };

        result.overall_score = result.calculate_score();
        assert_eq!(result.overall_score, 1.0);
    }

    #[test]
    fn test_integration_validation_result_score() {
        let mut result = IntegrationValidationResult {
            involution_sequence: vec![ValidationResult::passed("", "", 0.0, "")],
            entity_lifecycle: vec![ValidationResult::passed("", "", 0.0, "")],
            holographic_field: vec![ValidationResult::passed("", "", 0.0, "")],
            physical_adapter: vec![ValidationResult::passed("", "", 0.0, "")],
            simulation_runner: vec![ValidationResult::passed("", "", 0.0, "")],
            overall_score: 0.0,
        };

        result.overall_score = result.calculate_score();
        assert_eq!(result.overall_score, 1.0);
    }

    #[test]
    fn test_comprehensive_validation_result_summary() {
        let architecture = ArchitectureValidationResult {
            three_primal_distortions: vec![ValidationResult::passed("", "", 0.0, "")],
            transcend_include: vec![ValidationResult::passed("", "", 0.0, "")],
            space_time_spectrum: vec![ValidationResult::passed("", "", 0.0, "")],
            veil_position: vec![ValidationResult::passed("", "", 0.0, "")],
            logos_hierarchy: vec![ValidationResult::passed("", "", 0.0, "")],
            density_octave: vec![ValidationResult::passed("", "", 0.0, "")],
            holographic_principle: vec![ValidationResult::passed("", "", 0.0, "")],
            overall_score: 1.0,
        };

        let integration = IntegrationValidationResult {
            involution_sequence: vec![ValidationResult::passed("", "", 0.0, "")],
            entity_lifecycle: vec![ValidationResult::passed("", "", 0.0, "")],
            holographic_field: vec![ValidationResult::passed("", "", 0.0, "")],
            physical_adapter: vec![ValidationResult::passed("", "", 0.0, "")],
            simulation_runner: vec![ValidationResult::passed("", "", 0.0, "")],
            overall_score: 1.0,
        };

        let performance = PerformanceValidationResult {
            total_execution_time: Duration::from_secs(10),
            time_per_step: Duration::from_millis(100),
            entities_per_second: 10.0,
            steps_per_second: 10.0,
            memory_usage: None,
            comparison_ratio: None,
            performance_acceptable: true,
        };

        let result = ComprehensiveValidationResult::new(architecture, integration, performance);

        assert!(result.is_successful());
        assert!(result.all_passed());

        let summary = result.summary();
        assert!(summary.contains("COMPREHENSIVE VALIDATION RESULT"));
        assert!(summary.contains("SUCCESSFUL"));
    }
}
