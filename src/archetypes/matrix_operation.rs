// Matrix Archetype as Computational Operation (Phase 2.1)
//
// This module implements the Matrix archetype (A1) as a computational operation.
// Matrix is the input container - the conscious structure that receives and
// organizes experience.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::archetypes::a1_mind_matrix::MatrixMindArchetype;
use crate::archetypes::archetype_traits::MatrixArchetypeTrait;
use crate::archetypes::computational_operations::{
    ArchetypeOperation, ConsciousnessInput, ConsciousnessOutput, InputRequirements,
    OperationSignature, OutputCharacteristics, OutputType, ProcessingStyle,
};
use crate::types::Float;
use std::fmt;

/// Matrix Computational Operation
///
/// Matrix as a computational operation that receives and organizes experience.
/// This is the first step in the Lesser Cycle processing pipeline.
///
/// Processing Flow:
/// 1. Receive raw experience input
/// 2. Organize experience based on structural permeability
/// 3. Apply conscious coherence
/// 4. Output organized experience
#[derive(Debug, Clone)]
pub struct MatrixOperation {
    /// The underlying Matrix archetype
    pub matrix: MatrixMindArchetype,

    /// Operation name
    pub operation_name: String,
}

impl MatrixOperation {
    /// Create a new Matrix operation
    pub fn new(matrix: MatrixMindArchetype) -> Self {
        Self {
            matrix,
            operation_name: "Matrix Operation".to_string(),
        }
    }

    /// Create a new Matrix operation with default values
    pub fn default() -> Self {
        Self::new(MatrixMindArchetype::new())
    }

    /// Calculate organization efficiency
    ///
    /// Organization efficiency depends on:
    /// - Structural permeability (how flexible is the structure?)
    /// - Conscious coherence (how unified is awareness?)
    /// - Resource access (how well can we access resources?)
    fn calculate_organization_efficiency(&self) -> Float {
        let permeability = self.matrix.structural_permeability;
        let coherence = self.matrix.conscious_coherence;
        let resource_access = self.matrix.resource_access;

        // Organization = average of factors
        (permeability + coherence + resource_access) / 3.0
    }

    /// Organize raw experience
    ///
    /// This is the core computational operation of Matrix.
    /// It takes raw experience and organizes it into a structured form.
    fn organize_experience(&self, raw_experience: Float) -> Float {
        let organization_efficiency = self.calculate_organization_efficiency();

        // Organized experience = raw experience × organization efficiency
        let organized = raw_experience * organization_efficiency;

        // Apply structural capacity constraints
        let capacity_factor = 1.0 - self.matrix.current_load / self.matrix.structural_capacity;
        let capacity_constrained = organized * capacity_factor.max(0.0);

        capacity_constrained.clamp(0.0, 1.0)
    }

    /// Extract wisdom from organized experience
    ///
    /// Matrix can extract some wisdom from the organization process itself.
    fn extract_wisdom(&self, organized_experience: Float) -> Float {
        // Wisdom extraction rate depends on integration capacity
        let wisdom_rate = self.matrix.integration_capacity * 0.3; // 30% max

        organized_experience * wisdom_rate
    }

    /// Check if Matrix can process the input
    ///
    /// Matrix can process if:
    /// - Raw experience is above minimum threshold
    /// - Matrix is not overloaded
    /// - Structural permeability is sufficient
    fn can_process_input(&self, input: &ConsciousnessInput) -> bool {
        // Check if raw experience is sufficient
        if input.raw_experience < 0.1 {
            return false;
        }

        // Check if Matrix is overloaded
        if self.matrix.is_overloaded() {
            return false;
        }

        // Check if structural permeability is sufficient
        if self.matrix.structural_permeability < 0.2 {
            return false;
        }

        true
    }
}

impl ArchetypeOperation for MatrixOperation {
    /// Process consciousness through Matrix
    ///
    /// This is the core computational operation of Matrix.
    /// It receives raw experience and organizes it.
    fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput {
        // Check if we can process this input
        if !self.can_process_input(&input) {
            let error = if self.matrix.is_overloaded() {
                "Matrix is overloaded".to_string()
            } else if self.matrix.structural_permeability < 0.2 {
                "Matrix permeability too low".to_string()
            } else {
                "Insufficient input experience".to_string()
            };

            return ConsciousnessOutput::failure(error);
        }

        // Organize the raw experience
        let organized_experience = self.organize_experience(input.raw_experience);

        // Extract wisdom from the organization process
        let wisdom = self.extract_wisdom(organized_experience);

        // Calculate processing efficiency
        let efficiency = self.calculate_organization_efficiency();

        // Create successful output
        let mut output = ConsciousnessOutput::success(organized_experience, wisdom);
        output.efficiency = efficiency;

        output
    }

    /// Get the computational signature of Matrix operation
    fn signature(&self) -> OperationSignature {
        let mut signature = OperationSignature::new(
            1,
            "Matrix of Mind".to_string(),
            ProcessingStyle::Receptive,
            self.matrix.structural_capacity * 0.5,
            0.5, // Medium complexity
        );

        // Input requirements
        signature.input_requirements = InputRequirements {
            min_experience: 0.1,
            min_catalyst: 0.0,
            catalyst_types: vec![],
            requires_previous_output: false,
        };

        // Output characteristics
        signature.output_characteristics = OutputCharacteristics {
            primary_output: OutputType::OrganizedExperience,
            wisdom_extraction: self.matrix.integration_capacity * 0.3,
            transformation_potential: 0.2,
            can_feedback: true,
        };

        signature
    }

    /// Get archetype ID
    fn archetype_id(&self) -> u8 {
        self.matrix.archetype_id
    }

    /// Get archetype name
    fn archetype_name(&self) -> &str {
        &self.operation_name
    }

    /// Check if Matrix can process the input
    fn can_process(&self, input: &ConsciousnessInput) -> bool {
        self.can_process_input(input)
    }

    /// Get processing efficiency
    fn processing_efficiency(&self) -> Float {
        self.calculate_organization_efficiency()
    }

    /// Get resource requirements
    fn resource_requirements(&self) -> Float {
        // Matrix requires resources based on structural capacity
        self.matrix.structural_capacity * 0.5
    }
}

impl fmt::Display for MatrixOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MatrixOperation (A1): permeability={:.2}, coherence={:.2}, capacity={:.2}",
            self.matrix.structural_permeability,
            self.matrix.conscious_coherence,
            self.matrix.structural_capacity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::computational_operations::{
        Catalyst, CatalystSource, CatalystType, ComplexType, CycleStep, ProcessingContext,
    };

    #[test]
    fn test_matrix_operation_creation() {
        let operation = MatrixOperation::default();

        assert_eq!(operation.archetype_id(), 1);
        assert_eq!(operation.archetype_name(), "Matrix Operation");
    }

    #[test]
    fn test_matrix_operation_process() {
        let operation = MatrixOperation::default();

        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
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
    fn test_matrix_operation_signature() {
        let operation = MatrixOperation::default();
        let signature = operation.signature();

        assert_eq!(signature.archetype_id, 1);
        assert_eq!(signature.archetype_name, "Matrix of Mind");
        assert_eq!(signature.processing_style, ProcessingStyle::Receptive);
        assert_eq!(
            signature.output_characteristics.primary_output,
            OutputType::OrganizedExperience
        );
    }

    #[test]
    fn test_matrix_operation_can_process() {
        let operation = MatrixOperation::default();

        // Valid input
        let valid_input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
                iteration: 0,
            },
        };

        assert!(operation.can_process(&valid_input));

        // Invalid input (too low experience)
        let invalid_input = ConsciousnessInput {
            raw_experience: 0.05,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
                iteration: 0,
            },
        };

        assert!(!operation.can_process(&invalid_input));
    }

    #[test]
    fn test_matrix_operation_overloaded() {
        let mut matrix = MatrixMindArchetype::new();
        // Overload the matrix
        matrix.current_load = matrix.structural_capacity * 0.95;

        let operation = MatrixOperation::new(matrix);

        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst: Catalyst::mind(0.7),
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
                iteration: 0,
            },
        };

        let output = operation.process(input);

        assert!(!output.success);
        assert!(output.metadata.errors.len() > 0);
    }

    #[test]
    fn test_matrix_operation_organize_experience() {
        let operation = MatrixOperation::default();

        let organized = operation.organize_experience(0.8);

        assert!(organized > 0.0);
        assert!(organized <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_matrix_operation_extract_wisdom() {
        let operation = MatrixOperation::default();

        let wisdom = operation.extract_wisdom(0.8);

        assert!(wisdom > 0.0);
        assert!(wisdom <= 0.8); // Should be less than or equal to input
    }

    #[test]
    fn test_matrix_operation_efficiency() {
        let operation = MatrixOperation::default();

        let efficiency = operation.processing_efficiency();

        assert!(efficiency >= 0.0);
        assert!(efficiency <= 1.0);
    }

    #[test]
    fn test_matrix_operation_display() {
        let operation = MatrixOperation::default();

        let display = format!("{}", operation);

        assert!(display.contains("MatrixOperation"));
        assert!(display.contains("A1"));
    }
}
