// Archetype Cycles Module
//
// This module implements the two fundamental cycles of the Archetypical Mind:
// - Lesser Cycle (A1→A2→A3→A4): Processing Engine
// - Greater Cycle (A5→A22→A6→A7): Evolutionary Engine
//
// The cycles represent how consciousness processes catalyst and evolves through
// the exercise of Free Will (Archetype 22 - The Choice).
//
// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json

use crate::archetypes::{
    common::ArchetypeTrait,
    complexes::{BodyComplex, MindComplex, SpiritComplex},
};
use crate::types::Float;
use crate::types::Polarity;
use std::fmt;

// ============================================================================
// LESSER CYCLE (A1→A2→A3→A4)
// ============================================================================

/// Lesser Cycle - The Processing Engine
///
/// The Lesser Cycle is the fundamental processing cycle of consciousness:
/// - A1 (Matrix) reaches into A2 (Potentiator) for resources
/// - A3 (Catalyst) acts upon conscious mind
/// - A4 (Experience) stores processed catalyst in unconscious
///
/// This cycle processes raw experience (catalyst) into integrated wisdom.
///
/// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json
#[derive(Debug, Clone)]
pub struct LesserCycle {
    /// The complex being processed (Mind, Body, or Spirit)
    pub complex_type: ComplexType,

    /// Processing efficiency (0.0 to 1.0)
    pub processing_efficiency: Float,

    /// Microcosmic tension - tension between Matrix and Potentiator
    pub microcosmic_tension: Float,

    /// Catalyst inflow rate
    pub catalyst_inflow: Float,

    /// Experience output rate
    pub experience_output: Float,

    /// Unprocessed catalyst accumulation
    pub accumulation: Float,
}

/// The type of complex being processed by the Lesser Cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    /// Mind Complex (σA)
    Mind,
    /// Body Complex (σB)
    Body,
    /// Spirit Complex (σC)
    Spirit,
}

/// Lesser Cycle Result
///
/// The result of processing catalyst through the Lesser Cycle.
#[derive(Debug, Clone)]
pub struct LesserCycleResult {
    /// Processed experience
    pub experience: Float,

    /// Processing efficiency
    pub efficiency: Float,

    /// Microcosmic tension
    pub microcosmic_tension: Float,

    /// Unprocessed catalyst remaining
    pub unprocessed: Float,

    /// Wisdom gained
    pub wisdom: Float,

    /// Whether the cycle was successful
    pub success: bool,
}

impl LesserCycle {
    /// Create a new Lesser Cycle for a specific complex
    pub fn new(complex_type: ComplexType) -> Self {
        Self {
            complex_type,
            processing_efficiency: 0.7,
            microcosmic_tension: 0.2,
            catalyst_inflow: 0.5,
            experience_output: 0.5,
            accumulation: 0.0,
        }
    }

    /// Process catalyst through the Lesser Cycle
    ///
    /// The processing follows the 4-step cycle:
    /// 1. Matrix (A1) reaches into Potentiator (A2) for resources
    /// 2. Catalyst (A3) acts upon conscious mind
    /// 3. Experience (A4) stores processed catalyst in unconscious
    ///
    /// Returns the processed experience and cycle metrics.
    pub fn process(&mut self, catalyst: Float) -> LesserCycleResult {
        // Step 1: Matrix reaches into Potentiator for resources
        // Microcosmic tension = |A1 - A2|
        let matrix_access = self.calculate_matrix_access();

        // Step 2: Catalyst acts upon conscious mind
        let catalyst_intensity = catalyst * matrix_access;

        // Step 3: Experience stores processed catalyst
        // Processing efficiency = A4 / A3
        let experience = catalyst_intensity * self.processing_efficiency;

        // Calculate unprocessed catalyst
        let unprocessed = catalyst - experience;

        // Update accumulation
        self.accumulation += unprocessed;

        // Calculate wisdom gained (processed catalyst integrated)
        let wisdom = experience * 0.5;

        // Calculate microcosmic tension
        self.microcosmic_tension = (1.0 - matrix_access).abs();

        // Update rates
        self.catalyst_inflow = catalyst;
        self.experience_output = experience;

        LesserCycleResult {
            experience,
            efficiency: self.processing_efficiency,
            microcosmic_tension: self.microcosmic_tension,
            unprocessed,
            wisdom,
            success: experience > 0.0,
        }
    }

    /// Calculate Matrix access to Potentiator resources
    ///
    /// Matrix access is determined by the microcosmic tension.
    /// Lower tension = better access.
    fn calculate_matrix_access(&self) -> Float {
        // Access is inversely proportional to tension
        (1.0 - self.microcosmic_tension).max(0.0)
    }

    /// Process through Mind Complex
    pub fn process_mind(&mut self, mind: &mut MindComplex, catalyst: Float) -> LesserCycleResult {
        // Update mind archetypes
        mind.catalyst
            .process(catalyst, mind.catalyst.developmental_position);
        mind.experience
            .process(catalyst, mind.experience.developmental_position);

        // Process catalyst
        self.process(catalyst)
    }

    /// Process through Body Complex
    pub fn process_body(&mut self, body: &mut BodyComplex, catalyst: Float) -> LesserCycleResult {
        // Update body archetypes
        body.catalyst
            .process(catalyst, body.catalyst.developmental_position);
        body.experience
            .process(catalyst, body.experience.developmental_position);

        // Process catalyst
        self.process(catalyst)
    }

    /// Process through Spirit Complex
    pub fn process_spirit(
        &mut self,
        spirit: &mut SpiritComplex,
        catalyst: Float,
    ) -> LesserCycleResult {
        // Update spirit archetypes
        spirit
            .catalyst
            .process(catalyst, spirit.catalyst.developmental_position);
        spirit
            .experience
            .process(catalyst, spirit.experience.developmental_position);

        // Process catalyst
        self.process(catalyst)
    }

    /// Calculate cycle efficiency
    ///
    /// Returns (Microcosmic Tension, Processing Efficiency)
    pub fn calculate_efficiency(&self) -> (Float, Float) {
        (self.microcosmic_tension, self.processing_efficiency)
    }

    /// Clear accumulated unprocessed catalyst
    pub fn clear_accumulation(&mut self) {
        self.accumulation = 0.0;
    }

    /// Get complex type as string
    pub fn complex_type_str(&self) -> &'static str {
        match self.complex_type {
            ComplexType::Mind => "Mind",
            ComplexType::Body => "Body",
            ComplexType::Spirit => "Spirit",
        }
    }
}

impl Default for LesserCycle {
    fn default() -> Self {
        Self::new(ComplexType::Mind)
    }
}

impl fmt::Display for LesserCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lesser Cycle ({} Complex):\n\
             - Processing Efficiency: {:.3}\n\
             - Microcosmic Tension: {:.3}\n\
             - Catalyst Inflow: {:.3}\n\
             - Experience Output: {:.3}\n\
             - Accumulation: {:.3}",
            self.complex_type_str(),
            self.processing_efficiency,
            self.microcosmic_tension,
            self.catalyst_inflow,
            self.experience_output,
            self.accumulation,
        )
    }
}

// ============================================================================
// GREATER CYCLE (A5→A22→A6→A7)
// ============================================================================

/// Greater Cycle - The Evolutionary Engine
///
/// The Greater Cycle is the transformation cycle of consciousness:
/// - A5 (Significator) chooses
/// - A22 (The Choice) manifests Free Will
/// - A6 (Transformation) occurs
/// - A7 (Great Way) is modified
///
/// This cycle enables evolutionary change through the exercise of Free Will.
///
/// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json
#[derive(Debug, Clone)]
pub struct GreaterCycle {
    /// The complex being transformed (Mind, Body, or Spirit)
    pub complex_type: ComplexType,

    /// Transformation intensity (0.0 to 1.0)
    pub transformation_intensity: Float,

    /// Choice capacity (0.0 to 1.0)
    pub choice_capacity: Float,

    /// Polarity potential (0.0 to 1.0)
    pub polarity_potential: Float,

    /// Current polarization
    pub current_polarization: Option<Polarity>,

    /// Free Will expression (0.0 to 1.0)
    pub free_will_expression: Float,
}

/// Greater Cycle Result
///
/// The result of processing experience through the Greater Cycle.
#[derive(Debug, Clone)]
pub struct GreaterCycleResult {
    /// The choice made
    pub choice: ArchetypeChoice,

    /// The transformation that occurred
    pub transformation: Float,

    /// Modified Great Way field
    pub great_way_modification: Float,

    /// Polarity shift
    pub polarity_shift: Float,

    /// Whether the cycle was successful
    pub success: bool,
}

/// A Choice made through Free Will
#[derive(Debug, Clone)]
pub struct ArchetypeChoice {
    /// The polarity of the choice (STO or STS)
    pub polarity: Polarity,

    /// The intensity of the choice (0.0 to 1.0)
    pub intensity: Float,

    /// The context of the choice
    pub context: ChoiceContext,
}

/// The context in which a choice is made
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceContext {
    /// Processing catalyst
    CatalystProcessing,
    /// Serving others
    ServingOthers,
    /// Serving self
    ServingSelf,
    /// Seeking wisdom
    SeekingWisdom,
    /// Personal growth
    PersonalGrowth,
}

impl GreaterCycle {
    /// Create a new Greater Cycle for a specific complex
    pub fn new(complex_type: ComplexType) -> Self {
        Self {
            complex_type,
            transformation_intensity: 0.5,
            choice_capacity: 1.0,
            polarity_potential: 0.5,
            current_polarization: None,
            free_will_expression: 0.7,
        }
    }

    /// Process experience through the Greater Cycle
    ///
    /// The processing follows the 4-step cycle:
    /// 1. Significator (A5) chooses
    /// 2. The Choice (A22) manifests Free Will
    /// 3. Transformation (A6) occurs
    /// 4. Great Way (A7) is modified
    ///
    /// Returns the transformation result and choice metrics.
    pub fn process(&mut self, experience: Float) -> GreaterCycleResult {
        // Step 1: Significator (A5) chooses
        let choice = self.make_choice(experience);

        // Step 2: The Choice (A22) manifests Free Will
        let free_will_expression = self.calculate_free_will_expression(&choice);

        // Step 3: Transformation (A6) occurs
        let transformation = self.calculate_transformation(experience, free_will_expression);

        // Step 4: Great Way (A7) is modified
        let great_way_modification = transformation * 0.8;

        // Calculate polarity shift before moving choice
        let polarity_shift = choice.intensity * 0.1;

        // Update polarization
        self.update_polarization(&choice);

        // Update free will expression
        self.free_will_expression = free_will_expression;

        GreaterCycleResult {
            choice,
            transformation,
            great_way_modification,
            polarity_shift,
            success: transformation > 0.0,
        }
    }

    /// Make a choice through Free Will
    ///
    /// This is the zero-point polarity moment where the entity chooses
    /// between Service-to-Others (STO) and Service-to-Self (STS).
    fn make_choice(&self, experience: Float) -> ArchetypeChoice {
        // Determine polarity based on experience and current state
        let polarity = if experience > self.polarity_potential {
            Polarity::STO
        } else {
            Polarity::STS
        };

        ArchetypeChoice {
            polarity,
            intensity: experience * self.choice_capacity,
            context: ChoiceContext::CatalystProcessing,
        }
    }

    /// Calculate Free Will expression
    fn calculate_free_will_expression(&self, choice: &ArchetypeChoice) -> Float {
        self.free_will_expression * choice.intensity
    }

    /// Calculate transformation
    fn calculate_transformation(&self, experience: Float, free_will: Float) -> Float {
        experience * free_will * self.transformation_intensity
    }

    /// Update polarization based on choice
    fn update_polarization(&mut self, choice: &ArchetypeChoice) {
        if choice.intensity > 0.7 {
            self.current_polarization = Some(choice.polarity);
        }
    }

    /// Process through Mind Complex
    pub fn process_mind(
        &mut self,
        mind: &mut MindComplex,
        experience: Float,
    ) -> GreaterCycleResult {
        // Update mind archetypes
        mind.significator
            .process(experience, mind.significator.developmental_position);
        mind.transformation
            .process(experience, mind.transformation.developmental_position);
        mind.great_way
            .process(experience, mind.great_way.developmental_position);

        // Process experience
        self.process(experience)
    }

    /// Process through Body Complex
    pub fn process_body(
        &mut self,
        body: &mut BodyComplex,
        experience: Float,
    ) -> GreaterCycleResult {
        // Update body archetypes
        body.significator
            .process(experience, body.significator.developmental_position);
        body.transformation
            .process(experience, body.transformation.developmental_position);
        body.great_way
            .process(experience, body.great_way.developmental_position);

        // Process experience
        self.process(experience)
    }

    /// Process through Spirit Complex
    pub fn process_spirit(
        &mut self,
        spirit: &mut SpiritComplex,
        experience: Float,
    ) -> GreaterCycleResult {
        // Update spirit archetypes
        spirit
            .significator
            .process(experience, spirit.significator.developmental_position);
        spirit
            .transformation
            .process(experience, spirit.transformation.developmental_position);
        spirit
            .great_way
            .process(experience, spirit.great_way.developmental_position);

        // Process experience
        self.process(experience)
    }

    /// Get complex type as string
    pub fn complex_type_str(&self) -> &'static str {
        match self.complex_type {
            ComplexType::Mind => "Mind",
            ComplexType::Body => "Body",
            ComplexType::Spirit => "Spirit",
        }
    }
}

impl Default for GreaterCycle {
    fn default() -> Self {
        Self::new(ComplexType::Mind)
    }
}

impl fmt::Display for GreaterCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Greater Cycle ({} Complex):\n\
             - Transformation Intensity: {:.3}\n\
             - Choice Capacity: {:.3}\n\
             - Polarity Potential: {:.3}\n\
             - Free Will Expression: {:.3}",
            self.complex_type_str(),
            self.transformation_intensity,
            self.choice_capacity,
            self.polarity_potential,
            self.free_will_expression,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesser_cycle_creation() {
        let cycle = LesserCycle::new(ComplexType::Mind);
        assert_eq!(cycle.complex_type, ComplexType::Mind);
        assert!(cycle.processing_efficiency > 0.0);
        assert!(cycle.processing_efficiency <= 1.0);
    }

    #[test]
    fn test_lesser_cycle_processing() {
        let mut cycle = LesserCycle::new(ComplexType::Mind);
        let catalyst = 0.7;

        let result = cycle.process(catalyst);

        assert!(result.experience > 0.0);
        assert!(result.experience <= catalyst);
        assert!(result.efficiency >= 0.0 && result.efficiency <= 1.0);
        assert!(result.success);
    }

    #[test]
    fn test_lesser_cycle_efficiency() {
        let cycle = LesserCycle::new(ComplexType::Mind);
        let (tension, efficiency) = cycle.calculate_efficiency();

        assert!(tension >= 0.0 && tension <= 1.0);
        assert!(efficiency >= 0.0 && efficiency <= 1.0);
    }

    #[test]
    fn test_greater_cycle_creation() {
        let cycle = GreaterCycle::new(ComplexType::Mind);
        assert_eq!(cycle.complex_type, ComplexType::Mind);
        assert!(cycle.choice_capacity > 0.0);
        assert!(cycle.free_will_expression > 0.0);
    }

    #[test]
    fn test_greater_cycle_processing() {
        let mut cycle = GreaterCycle::new(ComplexType::Mind);
        let experience = 0.7;

        let result = cycle.process(experience);

        assert!(result.transformation > 0.0);
        assert!(result.transformation <= experience);
        assert!(result.choice.intensity > 0.0);
        assert!(result.success);
    }

    #[test]
    fn test_choice_polarity() {
        let mut cycle = GreaterCycle::new(ComplexType::Mind);

        // High experience should lead to STO choice
        let result_sto = cycle.process(0.8);
        assert_eq!(result_sto.choice.polarity, Polarity::STO);

        // Low experience should lead to STS choice
        let result_sts = cycle.process(0.3);
        assert_eq!(result_sts.choice.polarity, Polarity::STS);
    }

    #[test]
    fn test_complex_type_str() {
        assert_eq!(
            LesserCycle::new(ComplexType::Mind).complex_type_str(),
            "Mind"
        );
        assert_eq!(
            LesserCycle::new(ComplexType::Body).complex_type_str(),
            "Body"
        );
        assert_eq!(
            LesserCycle::new(ComplexType::Spirit).complex_type_str(),
            "Spirit"
        );
    }
}
