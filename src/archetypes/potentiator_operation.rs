// Potentiator Archetype as Computational Operation (Phase 2.1)
//
// This module implements the Potentiator archetype (A2) as a computational operation.
// Potentiator is the resource cache - the unconscious potential that provides
// resources to Matrix.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::archetypes::a2_mind_potentiator::PotentiatorMindArchetype;
use crate::archetypes::computational_operations::{
    ArchetypeOperation, ConsciousnessInput, ConsciousnessOutput, InputRequirements,
    OperationSignature, OutputCharacteristics, OutputType, ProcessingStyle,
};
use crate::types::Float;
use std::fmt;

/// Potentiator Computational Operation
///
/// Potentiator as a computational operation that provides resources to Matrix.
/// This is the second step in the Lesser Cycle processing pipeline.
///
/// Processing Flow:
/// 1. Receive request from Matrix
/// 2. Access unconscious resources
/// 3. Provide resources based on accessibility
/// 4. Output resources
#[derive(Debug, Clone)]
pub struct PotentiatorOperation {
    /// The underlying Potentiator archetype
    pub potentiator: PotentiatorMindArchetype,

    /// Operation name
    pub operation_name: String,
}

impl PotentiatorOperation {
    /// Create a new Potentiator operation
    pub fn new(potentiator: PotentiatorMindArchetype) -> Self {
        Self {
            potentiator,
            operation_name: "Potentiator Operation".to_string(),
        }
    }

    /// Create a new Potentiator operation with initial values
    pub fn initial() -> Self {
        Self::new(PotentiatorMindArchetype::new())
    }

    /// Calculate resource accessibility
    ///
    /// Resource accessibility depends on:
    /// - Resource accessibility (how accessible are resources?)
    /// - Resource quality (how good are the resources?)
    /// - Resource depth (how deep are the resources?)
    fn calculate_resource_accessibility(&self) -> Float {
        let accessibility = self.potentiator.resource_accessibility;
        let quality = self.potentiator.resource_quality;
        let depth = self.potentiator.resource_depth;

        // Resource accessibility = average of factors
        (accessibility + quality + depth) / 3.0
    }

    /// Provide resources to Matrix
    ///
    /// This is the core computational operation of Potentiator.
    /// It takes a request from Matrix and provides resources.
    fn provide_resources(&self, request: Float) -> Float {
        let resource_accessibility = self.calculate_resource_accessibility();

        // Resources = request × accessibility
        let resources = request * resource_accessibility;

        // Apply resource depth constraints
        let depth_factor = self.potentiator.resource_depth;
        let depth_constrained = resources * depth_factor;

        depth_constrained.clamp(0.0, 1.0)
    }

    /// Extract wisdom from resource provision
    ///
    /// Potentiator can extract some wisdom from resource provision.
    fn extract_wisdom(&self, resources: Float) -> Float {
        // Wisdom extraction rate depends on resource quality
        let wisdom_rate = self.potentiator.resource_quality * 0.2; // 20% max

        resources * wisdom_rate
    }

    /// Check if Potentiator can process the input
    ///
    /// Potentiator can process if:
    /// - Request is above minimum threshold
    /// - Resource accessibility is sufficient
    /// - Resource depth is sufficient
    fn can_process_input(&self, input: &ConsciousnessInput) -> bool {
        // Check if request is sufficient
        if input.raw_experience < 0.1 {
            return false;
        }

        // Check if resource accessibility is sufficient
        if self.potentiator.resource_accessibility < 0.2 {
            return false;
        }

        // Check if resource depth is sufficient
        if self.potentiator.resource_depth < 0.2 {
            return false;
        }

        true
    }
}

impl ArchetypeOperation for PotentiatorOperation {
    /// Process consciousness through Potentiator
    ///
    /// This is the core computational operation of Potentiator.
    /// It provides resources to Matrix.
    fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput {
        // Check if we can process this input
        if !self.can_process_input(&input) {
            let error = if self.potentiator.resource_accessibility < 0.2 {
                "Resource accessibility too low".to_string()
            } else if self.potentiator.resource_depth < 0.2 {
                "Resource depth too low".to_string()
            } else {
                "Insufficient input request".to_string()
            };

            return ConsciousnessOutput::failure(error);
        }

        // Provide resources based on request
        let resources = self.provide_resources(input.raw_experience);

        // Extract wisdom from resource provision
        let wisdom = self.extract_wisdom(resources);

        // Calculate processing efficiency
        let efficiency = self.calculate_resource_accessibility();

        // Create successful output
        let mut output = ConsciousnessOutput::success(resources, wisdom);
        output.efficiency = efficiency;
        output.metadata.debug_info.push(format!(
            "Provided {:.2} resources with {:.2} efficiency",
            resources, efficiency
        ));

        output
    }

    /// Get the computational signature of Potentiator operation
    fn signature(&self) -> OperationSignature {
        let mut signature = OperationSignature::new(
            2,
            "Potentiator of Mind".to_string(),
            ProcessingStyle::Regulatory,
            0.4, // Low resource requirements (provides resources)
            0.4, // Low complexity
        );

        // Input requirements
        signature.input_requirements = InputRequirements {
            min_experience: 0.1,
            min_catalyst: 0.0,
            catalyst_types: vec![],
            requires_previous_output: true, // Requires Matrix output
        };

        // Output characteristics
        signature.output_characteristics = OutputCharacteristics {
            primary_output: OutputType::Resources,
            wisdom_extraction: self.potentiator.resource_quality * 0.2,
            transformation_potential: 0.1,
            can_feedback: true,
        };

        signature
    }

    /// Get archetype ID
    fn archetype_id(&self) -> u8 {
        self.potentiator.archetype_id
    }

    /// Get archetype name
    fn archetype_name(&self) -> &str {
        &self.operation_name
    }

    /// Check if Potentiator can process the input
    fn can_process(&self, input: &ConsciousnessInput) -> bool {
        self.can_process_input(input)
    }

    /// Get processing efficiency
    fn processing_efficiency(&self) -> Float {
        self.calculate_resource_accessibility()
    }

    /// Get resource requirements
    fn resource_requirements(&self) -> Float {
        // Potentiator requires minimal resources (it provides resources)
        0.3
    }
}

impl fmt::Display for PotentiatorOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PotentiatorOperation (A2): accessibility={:.2}, quality={:.2}, depth={:.2}",
            self.potentiator.resource_accessibility,
            self.potentiator.resource_quality,
            self.potentiator.resource_depth
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
    fn test_potentiator_operation_creation() {
        let operation = PotentiatorOperation::initial();

        assert_eq!(operation.archetype_id(), 2);
        assert_eq!(operation.archetype_name(), "Potentiator Operation");
    }

    #[test]
    fn test_potentiator_operation_process() {
        let operation = PotentiatorOperation::initial();

        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7), // Previous output from Matrix
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Potentiator,
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
    fn test_potentiator_operation_signature() {
        let operation = PotentiatorOperation::initial();
        let signature = operation.signature();

        assert_eq!(signature.archetype_id, 2);
        assert_eq!(signature.archetype_name, "Potentiator of Mind");
        assert_eq!(signature.processing_style, ProcessingStyle::Regulatory);
        assert_eq!(
            signature.output_characteristics.primary_output,
            OutputType::Resources
        );
    }

    #[test]
    fn test_potentiator_operation_can_process() {
        let operation = PotentiatorOperation::initial();

        // Valid input
        let valid_input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: Some(0.7),
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Potentiator,
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
                cycle_step: CycleStep::Potentiator,
                iteration: 0,
            },
        };

        assert!(!operation.can_process(&invalid_input));
    }

    #[test]
    fn test_potentiator_operation_provide_resources() {
        let operation = PotentiatorOperation::initial();

        let resources = operation.provide_resources(0.8);

        assert!(resources > 0.0);
        assert!(resources <= 0.8); // Should be less than or equal to request
    }

    #[test]
    fn test_potentiator_operation_extract_wisdom() {
        let operation = PotentiatorOperation::initial();

        let wisdom = operation.extract_wisdom(0.8);

        assert!(wisdom > 0.0);
        assert!(wisdom <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_potentiator_operation_efficiency() {
        let operation = PotentiatorOperation::initial();

        let efficiency = operation.processing_efficiency();

        assert!(efficiency >= 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_potentiator_operation_display() {
        let operation = PotentiatorOperation::initial();

        let display = format!("{}", operation);

        assert!(display.contains("PotentiatorOperation"));
        assert!(display.contains("A2"));
    }
}
