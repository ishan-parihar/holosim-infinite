// Experience Archetype as Computational Operation (Phase 2.1)
//
// This module implements the Experience archetype (A4) as a computational operation.
// Experience is the processed output - integrated wisdom stored in the unconscious
// which creates its continuing bias.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::archetypes::a4_mind_experience::ExperienceMindArchetype;
use crate::archetypes::computational_operations::{
    ArchetypeOperation, ConsciousnessInput, ConsciousnessOutput, InputRequirements,
    OperationSignature, OutputCharacteristics, OutputType, ProcessingStyle,
};
use crate::types::Float;
use std::fmt;

/// Experience Computational Operation
///
/// Experience as a computational operation that stores processed catalyst
// in the unconscious and creates continuing bias.
/// This is the fourth step in the Lesser Cycle processing pipeline.
///
/// Processing Flow:
/// 1. Receive processed catalyst
/// 2. Integrate into unconscious
/// 3. Extract wisdom
/// 4. Create continuing bias
/// 5. Output integrated experience
#[derive(Debug, Clone)]
pub struct ExperienceOperation {
    /// The underlying Experience archetype
    pub experience: ExperienceMindArchetype,

    /// Operation name
    pub operation_name: String,
}

impl ExperienceOperation {
    /// Create a new Experience operation
    pub fn new(experience: ExperienceMindArchetype) -> Self {
        Self {
            experience,
            operation_name: "Experience Operation".to_string(),
        }
    }

    /// Create a new Experience operation with default values
    pub fn default() -> Self {
        Self::new(ExperienceMindArchetype::new())
    }

    /// Calculate integration effectiveness
    ///
    /// Integration effectiveness depends on:
    /// - Experience depth (how deep can we integrate?)
    /// - Integration quality (how good is the integration?)
    /// - Continuing bias strength (how strong is the bias?)
    fn calculate_integration_effectiveness(&self) -> Float {
        let depth = self.experience.experience_depth;
        let quality = self.experience.integration_quality;
        let bias_strength = self.experience.continuing_bias_strength;

        // Integration effectiveness = average of factors
        (depth + quality + bias_strength) / 3.0
    }

    /// Integrate experience
    ///
    /// This is the core computational operation of Experience.
    /// It takes processed catalyst and integrates it into the unconscious.
    fn integrate_experience(&self, processed_catalyst: Float) -> Float {
        let integration_effectiveness = self.calculate_integration_effectiveness();

        // Integrated experience = processed catalyst × effectiveness
        let integrated = processed_catalyst * integration_effectiveness;

        // Apply catalyst integration constraints
        let depth_factor = self.experience.catalyst_integration;
        let depth_constrained = integrated * depth_factor;

        depth_constrained.clamp(0.0, 1.0)
    }

    /// Extract wisdom from integrated experience
    ///
    /// Experience can extract wisdom from the integration process.
    fn extract_wisdom(&self, integrated_experience: Float) -> Float {
        // Wisdom extraction rate depends on integration quality
        let wisdom_rate = self.experience.integration_quality * 0.4; // 40% max

        integrated_experience * wisdom_rate
    }

    /// Create continuing bias
    ///
    /// Experience creates a continuing bias that affects future processing.
    fn create_continuing_bias(&self, integrated_experience: Float) -> Float {
        // Bias strength depends on continuing bias strength
        let bias_strength = self.experience.continuing_bias_strength;

        integrated_experience * bias_strength
    }

    /// Check if Experience can process the input
    ///
    /// Experience can process if:
    /// - Processed catalyst is above minimum threshold
    /// - Experience depth is sufficient
    /// - Integration quality is sufficient
    fn can_process_input(&self, input: &ConsciousnessInput) -> bool {
        // Check if processed catalyst is sufficient
        if input.raw_experience < 0.1 {
            return false;
        }

        // Check if experience depth is sufficient
        if self.experience.experience_depth < 0.2 {
            return false;
        }

        // Check if integration quality is sufficient
        if self.experience.integration_quality < 0.2 {
            return false;
        }

        true
    }
}

impl ArchetypeOperation for ExperienceOperation {
    /// Process consciousness through Experience
    ///
    /// This is the core computational operation of Experience.
    /// It integrates processed catalyst into the unconscious and extracts wisdom.
    fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput {
        // Check if we can process this input
        if !self.can_process_input(&input) {
            let error = if self.experience.experience_depth < 0.2 {
                "Experience depth too low".to_string()
            } else if self.experience.integration_quality < 0.2 {
                "Integration quality too low".to_string()
            } else {
                "Insufficient input experience".to_string()
            };

            return ConsciousnessOutput::failure(error);
        }

        // Integrate experience
        let integrated = self.integrate_experience(input.raw_experience);

        // Extract wisdom from integration
        let wisdom = self.extract_wisdom(integrated);

        // Create continuing bias
        let bias = self.create_continuing_bias(integrated);

        // Calculate processing efficiency
        let efficiency = self.calculate_integration_effectiveness();

        // Create successful output
        let mut output = ConsciousnessOutput::success(integrated, wisdom);
        output.efficiency = efficiency;
        output.transformation = bias * 0.2; // 20% transformation potential from bias

        // Add debug info
        output.metadata.debug_info.push(format!(
            "Integrated {:.2} experience with {:.2} efficiency, created {:.2} bias",
            integrated, efficiency, bias
        ));

        output
    }

    /// Get the computational signature of Experience operation
    fn signature(&self) -> OperationSignature {
        let mut signature = OperationSignature::new(
            4,
            "Experience of Mind".to_string(),
            ProcessingStyle::Integrative,
            self.experience.experience_depth,
            0.7, // High complexity
        );

        // Input requirements
        signature.input_requirements = InputRequirements {
            min_experience: 0.1,
            min_catalyst: 0.0,
            catalyst_types: vec![],
            requires_previous_output: true, // Requires Catalyst output
        };

        // Output characteristics
        signature.output_characteristics = OutputCharacteristics {
            primary_output: OutputType::Wisdom,
            wisdom_extraction: self.experience.integration_quality * 0.4,
            transformation_potential: 0.2,
            can_feedback: true,
        };

        signature
    }

    /// Get archetype ID
    fn archetype_id(&self) -> u8 {
        self.experience.archetype_id
    }

    /// Get archetype name
    fn archetype_name(&self) -> &str {
        &self.operation_name
    }

    /// Check if Experience can process the input
    fn can_process(&self, input: &ConsciousnessInput) -> bool {
        self.can_process_input(input)
    }

    /// Get processing efficiency
    fn processing_efficiency(&self) -> Float {
        self.calculate_integration_effectiveness()
    }

    /// Get resource requirements
    fn resource_requirements(&self) -> Float {
        // Experience requires resources based on experience depth
        self.experience.experience_depth * 0.6
    }
}

impl fmt::Display for ExperienceOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExperienceOperation (A4): depth={:.2}, quality={:.2}, bias={:.2}",
            self.experience.experience_depth,
            self.experience.integration_quality,
            self.experience.continuing_bias_strength
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::computational_operations::{
        Catalyst, ComplexType, CycleStep, ProcessingContext,
    };

    #[test]
    fn test_experience_operation_creation() {
        let operation = ExperienceOperation::default();

        assert_eq!(operation.archetype_id(), 4);
        assert_eq!(operation.archetype_name(), "Experience Operation");
    }

    #[test]
    fn test_experience_operation_process() {
        let operation = ExperienceOperation::default();

        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7), // Previous output from Catalyst
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Experience,
                iteration: 0,
            },
        };

        let output = operation.process(input);

        assert!(output.success);
        assert!(output.processed_experience > 0.0);
        assert!(output.wisdom > 0.0);
        assert!(output.efficiency > 0.0);
    }

    #[test]
    fn test_experience_operation_signature() {
        let operation = ExperienceOperation::default();
        let signature = operation.signature();

        assert_eq!(signature.archetype_id, 4);
        assert_eq!(signature.archetype_name, "Experience of Mind");
        assert_eq!(signature.processing_style, ProcessingStyle::Integrative);
        assert_eq!(
            signature.output_characteristics.primary_output,
            OutputType::Wisdom
        );
    }

    #[test]
    fn test_experience_operation_can_process() {
        let operation = ExperienceOperation::default();

        // Valid input
        let valid_input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Experience,
                iteration: 0,
            },
        };

        assert!(operation.can_process(&valid_input));

        // Invalid input (too low experience)
        let invalid_input = ConsciousnessInput {
            raw_experience: 0.05,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Experience,
                iteration: 0,
            },
        };

        assert!(!operation.can_process(&invalid_input));
    }

    #[test]
    fn test_experience_operation_integrate_experience() {
        let operation = ExperienceOperation::default();

        let integrated = operation.integrate_experience(0.8);

        assert!(integrated > 0.0);
        assert!(integrated <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_experience_operation_extract_wisdom() {
        let operation = ExperienceOperation::default();

        let wisdom = operation.extract_wisdom(0.8);

        assert!(wisdom > 0.0);
        assert!(wisdom <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_experience_operation_create_continuing_bias() {
        let operation = ExperienceOperation::default();

        let bias = operation.create_continuing_bias(0.8);

        assert!(bias >= 0.0);
        assert!(bias <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_experience_operation_efficiency() {
        let operation = ExperienceOperation::default();

        let efficiency = operation.processing_efficiency();

        assert!(efficiency >= 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_experience_operation_display() {
        let operation = ExperienceOperation::default();

        let display = format!("{}", operation);

        assert!(display.contains("ExperienceOperation"));
        assert!(display.contains("A4"));
    }
}
