// Lesser Cycle as Computational Pipeline (Phase 2.1)
//
// This module demonstrates the Lesser Cycle (A1→A2→A3→A4) as a computational
// pipeline of archetype operations. This is the core demonstration of Phase 2.1:
// Archetypes as Computational Operations.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::archetypes::catalyst_operation::CatalystOperation;
use crate::archetypes::computational_operations::{
    Catalyst, ComplexType, ComputationalPipeline, ConsciousnessInput, ProcessingContext,
};
use crate::archetypes::experience_operation::ExperienceOperation;
use crate::archetypes::matrix_operation::MatrixOperation;
use crate::archetypes::potentiator_operation::PotentiatorOperation;
use crate::types::Float;
use std::fmt;

/// Lesser Cycle as Computational Pipeline
///
/// The Lesser Cycle is the fundamental processing cycle of consciousness:
/// - A1 (Matrix) reaches into A2 (Potentiator) for resources
/// - A3 (Catalyst) acts upon conscious mind
/// - A4 (Experience) stores processed catalyst in unconscious
///
/// This implementation demonstrates the cycle as a computational pipeline
/// of archetype operations.
#[derive(Debug)]
pub struct LesserCyclePipeline {
    /// The computational pipeline
    pub pipeline: ComputationalPipeline,

    /// Pipeline name
    pub name: String,

    /// Processing statistics
    pub statistics: PipelineStatistics,
}

/// Pipeline Statistics
///
/// Statistics about the pipeline's performance.
#[derive(Debug, Clone, Default)]
pub struct PipelineStatistics {
    /// Total inputs processed
    pub total_inputs: usize,

    /// Successful processing
    pub successful_processing: usize,

    /// Failed processing
    pub failed_processing: usize,

    /// Total wisdom extracted
    pub total_wisdom: Float,

    /// Total transformation achieved
    pub total_transformation: Float,

    /// Average processing efficiency
    pub average_efficiency: Float,
}

impl LesserCyclePipeline {
    /// Create a new Lesser Cycle pipeline
    ///
    /// This creates a computational pipeline with the 4 archetype operations:
    /// 1. Matrix (A1) - Input container
    /// 2. Potentiator (A2) - Resource cache
    /// 3. Catalyst (A3) - Input friction
    /// 4. Experience (A4) - Processed output
    pub fn new() -> Self {
        let mut pipeline = ComputationalPipeline::new("Lesser Cycle Pipeline".to_string());

        // Add the 4 archetype operations in sequence
        pipeline.add_operation(Box::new(MatrixOperation::default()));
        pipeline.add_operation(Box::new(PotentiatorOperation::default()));
        pipeline.add_operation(Box::new(CatalystOperation::default()));
        pipeline.add_operation(Box::new(ExperienceOperation::default()));

        Self {
            pipeline,
            name: "Lesser Cycle Pipeline".to_string(),
            statistics: PipelineStatistics::default(),
        }
    }

    /// Process catalyst through the Lesser Cycle
    ///
    /// This method processes catalyst through the complete 4-step cycle.
    ///
    /// # Arguments
    /// * `catalyst` - The catalyst to process
    /// * `raw_experience` - The raw experience value
    ///
    /// # Returns
    /// The final consciousness output after all operations
    pub fn process_catalyst(
        &mut self,
        catalyst: Catalyst,
        raw_experience: Float,
    ) -> ConsciousnessInput {
        // Create initial input
        let initial_input = ConsciousnessInput {
            raw_experience,
            catalyst: catalyst.clone(),
            vibrational_level: 0.6, // Default vibrational level
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: crate::archetypes::computational_operations::CycleStep::Matrix,
                iteration: 0,
            },
        };

        // Process through the pipeline
        let output = self.pipeline.process(initial_input);

        // Update statistics
        self.statistics.total_inputs += 1;
        if output.success {
            self.statistics.successful_processing += 1;
            self.statistics.total_wisdom += output.wisdom;
            self.statistics.total_transformation += output.transformation;
            self.statistics.average_efficiency = (self.statistics.average_efficiency
                * (self.statistics.successful_processing - 1) as Float
                + output.efficiency)
                / self.statistics.successful_processing as Float;
        } else {
            self.statistics.failed_processing += 1;
        }

        // Return the final output as input for next cycle
        ConsciousnessInput {
            raw_experience: output.processed_experience,
            catalyst: catalyst.clone(),
            vibrational_level: 0.6,
            previous_output: Some(output.processed_experience),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: crate::archetypes::computational_operations::CycleStep::Experience,
                iteration: self.statistics.total_inputs,
            },
        }
    }

    /// Get pipeline statistics
    pub fn get_statistics(&self) -> &PipelineStatistics {
        &self.statistics
    }

    /// Reset pipeline statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = PipelineStatistics::default();
    }

    /// Calculate processing efficiency
    ///
    /// Returns the overall processing efficiency (0.0 to 1.0).
    pub fn calculate_efficiency(&self) -> Float {
        if self.statistics.total_inputs == 0 {
            return 0.0;
        }

        self.statistics.successful_processing as Float / self.statistics.total_inputs as Float
    }

    /// Calculate wisdom extraction rate
    ///
    /// Returns the average wisdom extracted per input (0.0 to 1.0).
    pub fn calculate_wisdom_extraction_rate(&self) -> Float {
        if self.statistics.total_inputs == 0 {
            return 0.0;
        }

        self.statistics.total_wisdom / self.statistics.total_inputs as Float
    }

    /// Calculate transformation rate
    ///
    /// Returns the average transformation achieved per input (0.0 to 1.0).
    pub fn calculate_transformation_rate(&self) -> Float {
        if self.statistics.total_inputs == 0 {
            return 0.0;
        }

        self.statistics.total_transformation / self.statistics.total_inputs as Float
    }

    /// Get pipeline report
    ///
    /// Returns a detailed report of the pipeline's performance.
    pub fn get_report(&self) -> String {
        format!(
            "Lesser Cycle Pipeline Report:\n\
             Name: {}\n\
             Operations: {}\n\
             \n\
             Statistics:\n\
             - Total Inputs: {}\n\
             - Successful Processing: {}\n\
             - Failed Processing: {}\n\
             - Total Wisdom Extracted: {:.3}\n\
             - Total Transformation: {:.3}\n\
             - Average Efficiency: {:.3}\n\
             \n\
             Performance:\n\
             - Processing Efficiency: {:.3}\n\
             - Wisdom Extraction Rate: {:.3}\n\
             - Transformation Rate: {:.3}",
            self.name,
            self.pipeline.operation_count(),
            self.statistics.total_inputs,
            self.statistics.successful_processing,
            self.statistics.failed_processing,
            self.statistics.total_wisdom,
            self.statistics.total_transformation,
            self.statistics.average_efficiency,
            self.calculate_efficiency(),
            self.calculate_wisdom_extraction_rate(),
            self.calculate_transformation_rate()
        )
    }
}

impl fmt::Display for LesserCyclePipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LesserCyclePipeline: {} operations, {:.2}% efficiency",
            self.pipeline.operation_count(),
            self.calculate_efficiency() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::computational_operations::ConsciousnessOutput;

    #[test]
    fn test_lesser_cycle_pipeline_creation() {
        let pipeline = LesserCyclePipeline::new();

        assert_eq!(pipeline.pipeline.operation_count(), 4);
        assert_eq!(pipeline.name, "Lesser Cycle Pipeline");
    }

    #[test]
    fn test_lesser_cycle_pipeline_process_single() {
        let mut pipeline = LesserCyclePipeline::new();

        let catalyst = Catalyst::mind(0.7);
        let input = pipeline.process_catalyst(catalyst, 0.8);

        assert_eq!(pipeline.statistics.total_inputs, 1);
        assert_eq!(pipeline.statistics.successful_processing, 1);
        assert!(pipeline.statistics.total_wisdom > 0.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_process_multiple() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process multiple catalysts
        for i in 0..10 {
            let catalyst = Catalyst::mind(0.5 + (i as Float * 0.05));
            let input = pipeline.process_catalyst(catalyst, 0.6 + (i as Float * 0.03));
        }

        assert_eq!(pipeline.statistics.total_inputs, 10);
        assert!(pipeline.statistics.successful_processing > 0);
        assert!(pipeline.statistics.total_wisdom > 0.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_efficiency() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        let efficiency = pipeline.calculate_efficiency();

        assert!(efficiency >= 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_wisdom_extraction() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        let wisdom_rate = pipeline.calculate_wisdom_extraction_rate();

        assert!(wisdom_rate >= 0.0);
        assert!(wisdom_rate <= 1.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_transformation() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        let transformation_rate = pipeline.calculate_transformation_rate();

        assert!(transformation_rate >= 0.0);
        assert!(transformation_rate <= 1.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_statistics() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        let stats = pipeline.get_statistics();

        assert_eq!(stats.total_inputs, 5);
        assert!(stats.successful_processing > 0);
        assert!(stats.total_wisdom > 0.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_reset_statistics() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        // Reset statistics
        pipeline.reset_statistics();

        assert_eq!(pipeline.statistics.total_inputs, 0);
        assert_eq!(pipeline.statistics.successful_processing, 0);
        assert_eq!(pipeline.statistics.failed_processing, 0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_report() {
        let mut pipeline = LesserCyclePipeline::new();

        // Process some catalysts
        for _ in 0..5 {
            let catalyst = Catalyst::mind(0.7);
            let input = pipeline.process_catalyst(catalyst, 0.8);
        }

        let report = pipeline.get_report();

        assert!(report.contains("Lesser Cycle Pipeline"));
        assert!(report.contains("Statistics"));
        assert!(report.contains("Performance"));
    }

    #[test]
    fn test_lesser_cycle_pipeline_display() {
        let pipeline = LesserCyclePipeline::new();

        let display = format!("{}", pipeline);

        assert!(display.contains("LesserCyclePipeline"));
        assert!(display.contains("4 operations"));
    }

    #[test]
    fn test_lesser_cycle_pipeline_computational_operations() {
        let mut pipeline = LesserCyclePipeline::new();

        let catalyst = Catalyst::mind(0.7);

        // Create initial input
        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: catalyst.clone(),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: crate::archetypes::computational_operations::CycleStep::Matrix,
                iteration: 0,
            },
        };

        // Process through the pipeline
        let output = pipeline.pipeline.process(input);

        // Verify that processing occurred
        assert!(output.success);
        assert!(output.processed_experience > 0.0);
        assert!(output.wisdom > 0.0);

        // Verify that wisdom was accumulated
        assert!(pipeline.pipeline.state.accumulated_wisdom > 0.0);
    }

    #[test]
    fn test_lesser_cycle_pipeline_logic_gates() {
        let mut pipeline = LesserCyclePipeline::new();

        let catalyst = Catalyst::mind(0.7);

        // Process through the pipeline
        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: catalyst.clone(),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: crate::archetypes::computational_operations::CycleStep::Matrix,
                iteration: 0,
            },
        };

        let output = pipeline.pipeline.process(input);

        // Verify that each operation contributed to the output
        // This demonstrates the "logic gates" aspect of archetypes
        assert!(output.success);
        assert!(output.efficiency > 0.0);
        assert!(output.wisdom > 0.0);

        // Verify that processing time was accumulated
        assert!(pipeline.pipeline.state.total_processing_time > 0.0);
    }
}
