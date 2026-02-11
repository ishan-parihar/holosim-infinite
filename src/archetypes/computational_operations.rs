// Archetype Operation System (Phase 2.1)
//
// This module implements the computational operation model for archetypes.
// Archetypes are not static structures - they are active computational operations
// that process consciousness through the 7-step processing cycle.
//
// Knowledge Base Reference:
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5 (High Impact Gap #5)
// - COSMOLOGICAL-ARCHITECTURE.md Section 3.2
//
// From the Audit Report:
// "The 7 archetypes represent the complete processing cycle of consciousness.
// They are not metaphorical symbols but actual computational operations.
// They form the logic gates through which consciousness processes experience."

use crate::types::Float;
use std::fmt;

// ============================================================================
// CONSCIOUSNESS INPUT/OUTPUT
// ============================================================================

/// Consciousness Input - The raw experience entering archetype processing
///
/// This represents the consciousness input to an archetype operation.
/// It includes raw experience, catalyst, and contextual information.
#[derive(Debug, Clone)]
pub struct ConsciousnessInput {
    /// Raw experience value (0.0 to 1.0)
    pub raw_experience: Float,

    /// Catalyst that triggered this processing
    pub catalyst: Catalyst,

    /// Current vibrational state of consciousness
    pub vibrational_level: Float,

    /// Previous archetype output (if any)
    pub previous_output: Option<Float>,

    /// Processing context (which complex, which step in cycle)
    pub context: ProcessingContext,
}

/// Catalyst - The trigger for archetype processing
///
/// Catalyst is the input friction that acts upon consciousness to change it.
#[derive(Debug, Clone)]
pub struct Catalyst {
    /// Catalyst type (Mind, Body, Spirit)
    pub catalyst_type: CatalystType,

    /// Catalyst intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Catalyst quality (0.0 to 1.0)
    pub quality: Float,

    /// Catalyst source (external or internal)
    pub source: CatalystSource,
}

/// Catalyst Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystType {
    /// Mind catalyst - mental challenge or opportunity
    Mind,
    /// Body catalyst - physical challenge or opportunity
    Body,
    /// Spirit catalyst - spiritual challenge or opportunity
    Spirit,
    /// Integrated catalyst - affects all complexes
    Integrated,
}

/// Catalyst Source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystSource {
    /// External catalyst (from environment)
    External,
    /// Internal catalyst (from within)
    Internal,
}

impl Catalyst {
    /// Create a new catalyst
    pub fn new(catalyst_type: CatalystType, intensity: Float, source: CatalystSource) -> Self {
        Self {
            catalyst_type,
            intensity: intensity.clamp(0.0, 1.0),
            quality: 0.7, // Default quality
            source,
        }
    }

    /// Create a mind catalyst
    pub fn mind(intensity: Float) -> Self {
        Self::new(CatalystType::Mind, intensity, CatalystSource::External)
    }

    /// Create a body catalyst
    pub fn body(intensity: Float) -> Self {
        Self::new(CatalystType::Body, intensity, CatalystSource::External)
    }

    /// Create a spirit catalyst
    pub fn spirit(intensity: Float) -> Self {
        Self::new(CatalystType::Spirit, intensity, CatalystSource::External)
    }

    /// Get effective intensity (adjusted by quality)
    pub fn effective_intensity(&self) -> Float {
        self.intensity * self.quality
    }
}

/// Processing Context
///
/// Context information for archetype processing.
#[derive(Debug, Clone)]
pub struct ProcessingContext {
    /// Which complex is being processed
    pub complex_type: ComplexType,

    /// Which step in the processing cycle
    pub cycle_step: CycleStep,

    /// Iteration number (for recursive processing)
    pub iteration: usize,
}

/// Complex Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    /// Mind Complex (σA)
    Mind,
    /// Body Complex (σB)
    Body,
    /// Spirit Complex (σC)
    Spirit,
}

/// Cycle Step
///
/// The 7 steps of the archetype processing cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleStep {
    /// Step 1: Matrix - Input container
    Matrix,
    /// Step 2: Potentiator - Resource cache
    Potentiator,
    /// Step 3: Catalyst - Input friction
    Catalyst,
    /// Step 4: Experience - Processed output
    Experience,
    /// Step 5: Significator - Identity agent
    Significator,
    /// Step 6: Transformation - Choice meets action
    Transformation,
    /// Step 7: Great Way - Framework definition
    GreatWay,
}

// ============================================================================
// CONSCIOUSNESS OUTPUT
// ============================================================================

/// Consciousness Output - The result of archetype processing
///
/// This represents the consciousness output from an archetype operation.
/// It includes processed experience, wisdom, and transformation.
#[derive(Debug, Clone)]
pub struct ConsciousnessOutput {
    /// Processed experience (0.0 to 1.0)
    pub processed_experience: Float,

    /// Wisdom extracted from experience (0.0 to 1.0)
    pub wisdom: Float,

    /// Transformation applied (0.0 to 1.0)
    pub transformation: Float,

    /// Processing efficiency (0.0 to 1.0)
    pub efficiency: Float,

    /// Whether processing was successful
    pub success: bool,

    /// Processing metadata
    pub metadata: ProcessingMetadata,
}

/// Processing Metadata
///
/// Additional information about the processing operation.
#[derive(Debug, Clone)]
pub struct ProcessingMetadata {
    /// Processing time (simulated)
    pub processing_time: Float,

    /// Resources consumed
    pub resources_consumed: Float,

    /// Errors or warnings
    pub errors: Vec<String>,

    /// Debug information
    pub debug_info: Vec<String>,
}

impl ConsciousnessOutput {
    /// Create a successful output
    pub fn success(processed_experience: Float, wisdom: Float) -> Self {
        Self {
            processed_experience: processed_experience.clamp(0.0, 1.0),
            wisdom: wisdom.clamp(0.0, 1.0),
            transformation: 0.0,
            efficiency: 1.0,
            success: true,
            metadata: ProcessingMetadata {
                processing_time: 1.0,
                resources_consumed: 0.5,
                errors: Vec::new(),
                debug_info: Vec::new(),
            },
        }
    }

    /// Create a failed output
    pub fn failure(error: String) -> Self {
        Self {
            processed_experience: 0.0,
            wisdom: 0.0,
            transformation: 0.0,
            efficiency: 0.0,
            success: false,
            metadata: ProcessingMetadata {
                processing_time: 0.0,
                resources_consumed: 0.0,
                errors: vec![error],
                debug_info: Vec::new(),
            },
        }
    }

    /// Get total output value
    pub fn total_output(&self) -> Float {
        self.processed_experience + self.wisdom + self.transformation
    }
}

// ============================================================================
// ARCHETYPE OPERATION TRAIT
// ============================================================================

/// Archetype Operation Trait
///
/// All archetypes implement this trait to define their computational operation.
/// This is the core of Phase 2.1 - archetypes as computational operations.
///
/// Knowledge Base Reference:
/// - ARCHITECTURE_AUDIT_REPORT.md Section 2.5
/// "The 7 archetypes represent the complete processing cycle of consciousness.
/// They are not metaphorical symbols but actual computational operations.
/// They form the logic gates through which consciousness processes experience."
pub trait ArchetypeOperation: fmt::Debug + Send + Sync {
    /// Process consciousness through this archetype
    ///
    /// This is the core computational operation of the archetype.
    /// It takes ConsciousnessInput and returns ConsciousnessOutput.
    ///
    /// # Arguments
    /// * `input` - The consciousness input to process
    ///
    /// # Returns
    /// The processed consciousness output
    fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput;

    /// Get the computational signature of this operation
    ///
    /// The signature describes the operation's characteristics:
    /// - Input requirements
    /// - Output characteristics
    /// - Processing style
    /// - Resource requirements
    fn signature(&self) -> OperationSignature;

    /// Get the archetype ID (1-22)
    fn archetype_id(&self) -> u8;

    /// Get the archetype name
    fn archetype_name(&self) -> &str;

    /// Check if this archetype can process the given input
    fn can_process(&self, _input: &ConsciousnessInput) -> bool {
        // Default: can process any input
        true
    }

    /// Get the processing efficiency for this archetype
    ///
    /// Returns the efficiency (0.0 to 1.0) based on current state.
    fn processing_efficiency(&self) -> Float {
        0.7 // Default efficiency
    }

    /// Get the resource requirements for this archetype
    ///
    /// Returns the resource requirements (0.0 to 1.0).
    fn resource_requirements(&self) -> Float {
        0.5 // Default resource requirements
    }
}

/// Operation Signature
///
/// Describes the computational characteristics of an archetype operation.
#[derive(Debug, Clone)]
pub struct OperationSignature {
    /// Archetype ID (1-22)
    pub archetype_id: u8,

    /// Archetype name
    pub archetype_name: String,

    /// Processing style
    pub processing_style: ProcessingStyle,

    /// Input requirements
    pub input_requirements: InputRequirements,

    /// Output characteristics
    pub output_characteristics: OutputCharacteristics,

    /// Resource requirements (0.0 to 1.0)
    pub resource_requirements: Float,

    /// Processing complexity (0.0 = simple, 1.0 = complex)
    pub complexity: Float,
}

/// Processing Style
///
/// How the archetype processes consciousness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingStyle {
    /// Receptive - receives and organizes input
    Receptive,
    /// Active - transforms and modifies input
    Active,
    /// Integrative - combines multiple inputs
    Integrative,
    /// Transformative - fundamentally changes input
    Transformative,
    /// Regulatory - controls and regulates flow
    Regulatory,
}

/// Input Requirements
///
/// What the archetype needs as input.
#[derive(Debug, Clone)]
pub struct InputRequirements {
    /// Minimum raw experience (0.0 to 1.0)
    pub min_experience: Float,

    /// Minimum catalyst intensity (0.0 to 1.0)
    pub min_catalyst: Float,

    /// Required catalyst types
    pub catalyst_types: Vec<CatalystType>,

    /// Whether previous output is required
    pub requires_previous_output: bool,
}

/// Output Characteristics
///
/// What the archetype produces as output.
#[derive(Debug, Clone)]
pub struct OutputCharacteristics {
    /// Primary output type
    pub primary_output: OutputType,

    /// Wisdom extraction rate (0.0 to 1.0)
    pub wisdom_extraction: Float,

    /// Transformation potential (0.0 to 1.0)
    pub transformation_potential: Float,

    /// Whether output can be fed back as input
    pub can_feedback: bool,
}

/// Output Type
///
/// The type of output produced by the archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    /// Organized experience
    OrganizedExperience,
    /// Wisdom/knowledge
    Wisdom,
    /// Transformation
    Transformation,
    /// Resources
    Resources,
    /// Identity
    Identity,
    /// Framework
    Framework,
}

impl OperationSignature {
    /// Create a new operation signature
    pub fn new(
        archetype_id: u8,
        archetype_name: String,
        processing_style: ProcessingStyle,
        resource_requirements: Float,
        complexity: Float,
    ) -> Self {
        Self {
            archetype_id,
            archetype_name,
            processing_style,
            input_requirements: InputRequirements {
                min_experience: 0.0,
                min_catalyst: 0.0,
                catalyst_types: Vec::new(),
                requires_previous_output: false,
            },
            output_characteristics: OutputCharacteristics {
                primary_output: OutputType::OrganizedExperience,
                wisdom_extraction: 0.5,
                transformation_potential: 0.5,
                can_feedback: false,
            },
            resource_requirements: resource_requirements.clamp(0.0, 1.0),
            complexity: complexity.clamp(0.0, 1.0),
        }
    }
}

// ============================================================================
// COMPUTATIONAL PIPELINE
// ============================================================================

/// Computational Pipeline
///
/// A pipeline of archetype operations that process consciousness in sequence.
/// This implements the Lesser Cycle and Greater Cycle as computational pipelines.
#[derive(Debug)]
pub struct ComputationalPipeline {
    /// The operations in the pipeline (in order)
    pub operations: Vec<Box<dyn ArchetypeOperation>>,

    /// Pipeline name
    pub name: String,

    /// Pipeline efficiency (0.0 to 1.0)
    pub efficiency: Float,

    /// Pipeline state
    pub state: PipelineState,
}

/// Pipeline State
///
/// The current state of the pipeline.
#[derive(Debug, Clone)]
pub struct PipelineState {
    /// Current operation index
    pub current_operation: usize,

    /// Accumulated wisdom
    pub accumulated_wisdom: Float,

    /// Accumulated transformation
    pub accumulated_transformation: Float,

    /// Total processing time
    pub total_processing_time: Float,

    /// Errors encountered
    pub errors: Vec<String>,
}

impl ComputationalPipeline {
    /// Create a new computational pipeline
    pub fn new(name: String) -> Self {
        Self {
            operations: Vec::new(),
            name,
            efficiency: 0.7,
            state: PipelineState {
                current_operation: 0,
                accumulated_wisdom: 0.0,
                accumulated_transformation: 0.0,
                total_processing_time: 0.0,
                errors: Vec::new(),
            },
        }
    }

    /// Add an operation to the pipeline
    pub fn add_operation(&mut self, operation: Box<dyn ArchetypeOperation>) {
        self.operations.push(operation);
    }

    /// Process consciousness through the entire pipeline
    ///
    /// This processes consciousness through all operations in sequence.
    ///
    /// # Arguments
    /// * `input` - The initial consciousness input
    ///
    /// # Returns
    /// The final consciousness output after all operations
    pub fn process(&mut self, input: ConsciousnessInput) -> ConsciousnessOutput {
        let mut current_input = input;
        let mut final_output = ConsciousnessOutput::success(0.0, 0.0);

        // Process through each operation in sequence
        for operation in &self.operations {
            // Check if operation can process the input
            if !operation.can_process(&current_input) {
                let error = format!(
                    "Operation {} cannot process input",
                    operation.archetype_name()
                );
                self.state.errors.push(error);
                continue;
            }

            // Process through the operation
            let output = operation.process(current_input.clone());

            // Check if processing was successful
            if !output.success {
                self.state.errors.extend(output.metadata.errors);
                continue;
            }

            // Accumulate wisdom and transformation
            self.state.accumulated_wisdom += output.wisdom;
            self.state.accumulated_transformation += output.transformation;

            // Update total processing time
            self.state.total_processing_time += output.metadata.processing_time;

            // Prepare input for next operation
            current_input = ConsciousnessInput {
                raw_experience: output.processed_experience,
                catalyst: current_input.catalyst,
                vibrational_level: current_input.vibrational_level,
                previous_output: Some(output.processed_experience),
                context: current_input.context,
            };

            // Keep track of the last successful output
            final_output = output;
        }

        // Update accumulated values in final output
        final_output.wisdom = self.state.accumulated_wisdom;
        final_output.transformation = self.state.accumulated_transformation;
        final_output.efficiency = self.efficiency;

        final_output
    }

    /// Get the number of operations in the pipeline
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Check if pipeline has any operations
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype operation for testing
    #[derive(Debug, Clone)]
    struct MockArchetypeOperation {
        id: u8,
        name: String,
    }

    impl ArchetypeOperation for MockArchetypeOperation {
        fn process(&self, input: ConsciousnessInput) -> ConsciousnessOutput {
            ConsciousnessOutput::success(input.raw_experience * 0.9, input.raw_experience * 0.5)
        }

        fn signature(&self) -> OperationSignature {
            OperationSignature::new(
                self.id,
                self.name.clone(),
                ProcessingStyle::Receptive,
                0.5,
                0.5,
            )
        }

        fn archetype_id(&self) -> u8 {
            self.id
        }

        fn archetype_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_catalyst_creation() {
        let catalyst = Catalyst::mind(0.7);

        assert_eq!(catalyst.catalyst_type, CatalystType::Mind);
        assert_eq!(catalyst.intensity, 0.7);
        assert!((catalyst.effective_intensity() - 0.49).abs() < 0.01); // 0.7 * 0.7 (approximate)
    }

    #[test]
    fn test_consciousness_input() {
        let catalyst = Catalyst::mind(0.7);
        let input = ConsciousnessInput {
            raw_experience: 0.8,
            catalyst,
            vibrational_level: 0.6,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
                iteration: 0,
            },
        };

        assert_eq!(input.raw_experience, 0.8);
        assert_eq!(input.vibrational_level, 0.6);
    }

    #[test]
    fn test_consciousness_output_success() {
        let output = ConsciousnessOutput::success(0.8, 0.5);

        assert!(output.success);
        assert_eq!(output.processed_experience, 0.8);
        assert_eq!(output.wisdom, 0.5);
        assert_eq!(output.efficiency, 1.0);
    }

    #[test]
    fn test_consciousness_output_failure() {
        let output = ConsciousnessOutput::failure("Test error".to_string());

        assert!(!output.success);
        assert_eq!(output.processed_experience, 0.0);
        assert_eq!(output.wisdom, 0.0);
        assert_eq!(output.metadata.errors.len(), 1);
    }

    #[test]
    fn test_mock_archetype_operation() {
        let operation = MockArchetypeOperation {
            id: 1,
            name: "Test Archetype".to_string(),
        };

        assert_eq!(operation.archetype_id(), 1);
        assert_eq!(operation.archetype_name(), "Test Archetype");
        assert!(operation.can_process(&ConsciousnessInput {
            raw_experience: 0.5,
            catalyst: Catalyst::mind(0.5),
            vibrational_level: 0.5,
            previous_output: None,
            context: ProcessingContext {
                complex_type: ComplexType::Mind,
                cycle_step: CycleStep::Matrix,
                iteration: 0,
            },
        }));
    }

    #[test]
    fn test_computational_pipeline() {
        let mut pipeline = ComputationalPipeline::new("Test Pipeline".to_string());

        // Add mock operations
        pipeline.add_operation(Box::new(MockArchetypeOperation {
            id: 1,
            name: "Operation 1".to_string(),
        }));
        pipeline.add_operation(Box::new(MockArchetypeOperation {
            id: 2,
            name: "Operation 2".to_string(),
        }));

        assert_eq!(pipeline.operation_count(), 2);
        assert!(!pipeline.is_empty());
    }

    #[test]
    fn test_pipeline_processing() {
        let mut pipeline = ComputationalPipeline::new("Test Pipeline".to_string());

        // Add mock operations
        pipeline.add_operation(Box::new(MockArchetypeOperation {
            id: 1,
            name: "Operation 1".to_string(),
        }));

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

        let output = pipeline.process(input);

        assert!(output.success);
        assert!(output.processed_experience > 0.0);
        assert!(output.wisdom > 0.0);
    }

    #[test]
    fn test_operation_signature() {
        let signature = OperationSignature::new(
            1,
            "Test Archetype".to_string(),
            ProcessingStyle::Receptive,
            0.5,
            0.5,
        );

        assert_eq!(signature.archetype_id, 1);
        assert_eq!(signature.archetype_name, "Test Archetype");
        assert_eq!(signature.processing_style, ProcessingStyle::Receptive);
    }
}
