// Catalyst Archetype as Computational Operation (Phase 2.1)
//
// This module implements the Catalyst archetype (A3) as a computational operation.
// Catalyst is the input friction - raw experience that acts upon conscious mind
// to change it.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::archetypes::a3_mind_catalyst::CatalystMindArchetype;
use crate::archetypes::archetype_traits::CatalystArchetypeTrait;
use crate::archetypes::computational_operations::{
    ArchetypeOperation, Catalyst as CatalystInput, CatalystType, ConsciousnessInput,
    ConsciousnessOutput, InputRequirements, OperationSignature, OutputCharacteristics, OutputType,
    ProcessingStyle,
};
use crate::types::Float;
use std::fmt;

/// Catalyst Computational Operation
///
/// Catalyst as a computational operation that acts upon conscious mind.
/// This is the third step in the Lesser Cycle processing pipeline.
///
/// Processing Flow:
/// 1. Receive raw catalyst
/// 2. Apply processing to conscious mind
/// 3. Transform catalyst based on processing capacity
/// 4. Output transformed catalyst
#[derive(Debug, Clone)]
pub struct CatalystOperation {
    /// The underlying Catalyst archetype
    pub catalyst: CatalystMindArchetype,

    /// Operation name
    pub operation_name: String,
}

impl CatalystOperation {
    /// Create a new Catalyst operation
    pub fn new(catalyst: CatalystMindArchetype) -> Self {
        Self {
            catalyst,
            operation_name: "Catalyst Operation".to_string(),
        }
    }

    /// Create a new Catalyst operation with initial values
    pub fn initial() -> Self {
        Self::new(CatalystMindArchetype::new())
    }

    /// Calculate processing effectiveness
    ///
    /// Processing effectiveness depends on:
    /// - Processing rate (how fast do we process?)
    /// - Processing capacity (how much can we process?)
    /// - Processing efficiency (how efficient is processing?)
    fn calculate_processing_effectiveness(&self) -> Float {
        let rate = self.catalyst.processing_rate;
        let capacity = self.catalyst.processing_capacity;
        let efficiency = self.catalyst.processing_efficiency;

        // Processing effectiveness = average of factors
        (rate + capacity + efficiency) / 3.0
    }

    /// Process catalyst
    ///
    /// This is the core computational operation of Catalyst.
    /// It takes raw catalyst and transforms it.
    fn process_catalyst(&self, raw_catalyst: Float, catalyst_input: &CatalystInput) -> Float {
        let processing_effectiveness = self.calculate_processing_effectiveness();

        // Apply catalyst intensity
        let intensity = catalyst_input.effective_intensity();

        // Processed catalyst = raw catalyst × effectiveness × intensity
        let processed = raw_catalyst * processing_effectiveness * intensity;

        // Apply processing capacity constraints
        let capacity_factor = 1.0_f64 - self.catalyst.get_accumulation_rate();
        let capacity_constrained = processed * capacity_factor.max(0.0_f64);

        capacity_constrained.clamp(0.0_f64, 1.0_f64)
    }

    /// Extract wisdom from catalyst processing
    ///
    /// Catalyst can extract wisdom from the processing process.
    fn extract_wisdom(&self, processed_catalyst: Float) -> Float {
        // Wisdom extraction rate depends on processing efficiency
        let wisdom_rate = self.catalyst.processing_efficiency * 0.25; // 25% max

        processed_catalyst * wisdom_rate
    }

    /// Check if Catalyst can process the input
    ///
    /// Catalyst can process if:
    /// - Raw catalyst is above minimum threshold
    /// - Processing capacity is sufficient
    /// - Processing rate is sufficient
    fn can_process_input(&self, input: &ConsciousnessInput) -> bool {
        // Check if raw catalyst is sufficient
        if input.raw_experience < 0.1 {
            return false;
        }

        // Check if processing capacity is sufficient
        if self.catalyst.processing_capacity < 0.2 {
            return false;
        }

        // Check if processing rate is sufficient
        if self.catalyst.processing_rate < 0.2 {
            return false;
        }

        true
    }
}

impl ArchetypeOperation for CatalystOperation {
    /// Process consciousness through Catalyst
    ///
    /// This is the core computational operation of Catalyst.
    /// It acts upon conscious mind and transforms catalyst.
    fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput {
        // Check if we can process this input
        if !self.can_process_input(&input) {
            let error = if self.catalyst.processing_capacity < 0.2 {
                "Processing capacity too low".to_string()
            } else if self.catalyst.processing_rate < 0.2 {
                "Processing rate too low".to_string()
            } else {
                "Insufficient input catalyst".to_string()
            };

            return ConsciousnessOutput::failure(error);
        }

        // Process catalyst
        let processed = self.process_catalyst(input.raw_experience, &input.catalyst);

        // Extract wisdom from catalyst processing
        let wisdom = self.extract_wisdom(processed);

        // Calculate processing efficiency
        let efficiency = self.calculate_processing_effectiveness();

        // Create successful output
        let mut output = ConsciousnessOutput::success(processed, wisdom);
        output.efficiency = efficiency;
        output.transformation = processed * 0.3; // 30% transformation potential

        output
    }

    /// Get the computational signature of Catalyst operation
    fn signature(&self) -> OperationSignature {
        let mut signature = OperationSignature::new(
            3,
            "Catalyst of Mind".to_string(),
            ProcessingStyle::Active,
            self.catalyst.processing_capacity,
            0.6, // Medium-high complexity
        );

        // Input requirements
        signature.input_requirements = InputRequirements {
            min_experience: 0.1,
            min_catalyst: 0.2,
            catalyst_types: vec![
                CatalystType::Mind,
                CatalystType::Body,
                CatalystType::Spirit,
                CatalystType::Integrated,
            ],
            requires_previous_output: true, // Requires Potentiator output
        };

        // Output characteristics
        signature.output_characteristics = OutputCharacteristics {
            primary_output: OutputType::OrganizedExperience,
            wisdom_extraction: self.catalyst.processing_efficiency * 0.25,
            transformation_potential: 0.3,
            can_feedback: true,
        };

        signature
    }

    /// Get archetype ID
    fn archetype_id(&self) -> u8 {
        self.catalyst.archetype_id
    }

    /// Get archetype name
    fn archetype_name(&self) -> &str {
        &self.operation_name
    }

    /// Check if Catalyst can process the input
    fn can_process(&self, input: &ConsciousnessInput) -> bool {
        self.can_process_input(input)
    }

    /// Get processing efficiency
    fn processing_efficiency(&self) -> Float {
        self.calculate_processing_effectiveness()
    }

    /// Get resource requirements
    fn resource_requirements(&self) -> Float {
        // Catalyst requires resources based on processing capacity
        self.catalyst.processing_capacity * 0.7
    }
}

impl fmt::Display for CatalystOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CatalystOperation (A3): rate={:.2}, capacity={:.2}, efficiency={:.2}",
            self.catalyst.processing_rate,
            self.catalyst.processing_capacity,
            self.catalyst.processing_efficiency
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::computational_operations::{ComplexType, CycleStep, ProcessingContext};

    #[test]
    fn test_catalyst_operation_creation() {
        let operation = CatalystOperation::initial();

        assert_eq!(operation.archetype_id(), 3);
        assert_eq!(operation.archetype_name(), "Catalyst Operation");
    }

    #[test]
    fn test_catalyst_operation_process() {
        let operation = CatalystOperation::initial();

        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: CatalystInput::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7), // Previous output from Potentiator
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Catalyst,
                iteration: 0,
            },
        };

        let output = operation.process(input);

        assert!(output.success);
        assert!(output.processed_experience > 0.0);
        assert!(output.wisdom > 0.0);
        assert!(output.efficiency > 0.0);
        assert!(output.transformation > 0.0);
    }

    #[test]
    fn test_catalyst_operation_signature() {
        let operation = CatalystOperation::initial();
        let signature = operation.signature();

        assert_eq!(signature.archetype_id, 3);
        assert_eq!(signature.archetype_name, "Catalyst of Mind");
        assert_eq!(signature.processing_style, ProcessingStyle::Active);
        assert_eq!(
            signature.output_characteristics.primary_output,
            OutputType::OrganizedExperience
        );
    }

    #[test]
    fn test_catalyst_operation_can_process() {
        let operation = CatalystOperation::initial();

        // Valid input
        let valid_input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: CatalystInput::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Catalyst,
                iteration: 0,
            },
        };

        assert!(operation.can_process(&valid_input));

        // Invalid input (too low experience)
        let invalid_input = ConsciousnessInput {
            raw_experience: 0.05,
            catalyst: CatalystInput::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Catalyst,
                iteration: 0,
            },
        };

        assert!(!operation.can_process(&invalid_input));
    }

    #[test]
    fn test_catalyst_operation_process_catalyst() {
        let operation = CatalystOperation::initial();

        let catalyst_input = CatalystInput::mind(0.7);
        let processed = operation.process_catalyst(0.8, &catalyst_input);

        assert!(processed > 0.0);
        assert!(processed <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_catalyst_operation_extract_wisdom() {
        let operation = CatalystOperation::initial();

        let wisdom = operation.extract_wisdom(0.8);

        assert!(wisdom > 0.0);
        assert!(wisdom <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_catalyst_operation_efficiency() {
        let operation = CatalystOperation::initial();

        let efficiency = operation.processing_efficiency();

        assert!(efficiency >= 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_catalyst_operation_display() {
        let operation = CatalystOperation::initial();

        let display = format!("{}", operation);

        assert!(display.contains("CatalystOperation"));
        assert!(display.contains("A3"));
    }
}
