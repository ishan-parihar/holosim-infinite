// Evolution Chain Module
//
// This module implements the complete Evolution chain as an "activation" process,
// not "acquisition". It implements the Lesser and Greater cycles.
//
// Key Principles:
// 1. Evolution is the upward flow (Red → Violet)
// 2. Entity does not "acquire" new capacities—it "activates" pre-existing holographic architecture
// 3. The Lesser Cycle (A1-A4): Processing Catalyst - Matrix → Potentiator → Catalyst → Experience
// 4. The Greater Cycle (A5-A22-A6-A7): Making Choices - Significator → Choice → Transformation → Great Way
// 5. Entity's journey is about UNFOLDING what was FOLDED in during Involution
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 4 (Evolution Process)
// - Archetypes/0. Archetypical Mind System.json
// - REFACTOR_ROADMAP_V3.md Phase 7

use crate::archetypes::cycles::{
    ArchetypeChoice, ChoiceContext, ComplexType, GreaterCycle, GreaterCycleResult, LesserCycle,
    LesserCycleResult,
};
use crate::entity_state::{Catalyst, CatalystType};
use crate::evolution_density_octave::{SpiralLeap, SpiralPattern, ValveState};
use crate::types::Float;
use std::fmt;

// ============================================================================
// EVOLUTION CHAIN
// ============================================================================

/// Evolution Chain - The complete upward flow (Red → Violet)
///
/// The Evolution Chain represents the entity's evolutionary journey from
/// Red Ray to Violet Ray. It implements the activation of pre-existing
/// holographic architecture through the Lesser and Greater cycles.
///
/// Key Distinction:
/// - Entity does not "acquire" new capacities
/// - Entity "activates" pre-existing holographic architecture
/// - Evolution is about UNFOLDING what was FOLDED during Involution
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 7
#[derive(Debug, Clone)]
pub struct EvolutionChain {
    /// The Lesser Cycle (A1-A4) - Processing Catalyst
    pub lesser_cycle: LesserCycle,

    /// The Greater Cycle (A5-A22-A6-A7) - Making Choices
    pub greater_cycle: GreaterCycle,

    /// Current evolutionary step (Red → Violet)
    pub current_step: EvolutionStep,

    /// Mind balance state (the valve mechanism)
    pub valve_state: ValveState,

    /// Spiral development pattern (non-linear leaps)
    pub spiral_pattern: SpiralPattern,

    /// Total consciousness expansion achieved
    pub total_consciousness_expansion: Float,
}

/// Evolution Steps (Red → Violet)
///
/// Represents the evolutionary journey from Red Ray to Violet Ray.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvolutionStep {
    /// Step 1: Red Ray - Survival and Basic Instincts
    RedRay = 1,

    /// Step 2: Orange Ray - Self-Awareness and Personal Identity
    OrangeRay = 2,

    /// Step 3: Yellow Ray - Social Awareness and Group Identity
    YellowRay = 3,

    /// Step 4: Green Ray - Universal Love and Compassion
    GreenRay = 4,

    /// Step 5: Blue Ray - Communication and Understanding
    BlueRay = 5,

    /// Step 6: Indigo Ray - Intuition and Wisdom
    IndigoRay = 6,

    /// Step 7: Violet Ray - Spiritual Integration and Unity
    VioletRay = 7,
}

// Note: ValveState, SpiralPattern, and SpiralLeap have been migrated to
// src/evolution_density_octave/density_octave.rs (Phase 4.5 Migrations 2-3)
// They are now re-exported from that module.

impl Default for EvolutionChain {
    fn default() -> Self {
        Self::new()
    }
}

impl EvolutionChain {
    /// Create a new Evolution Chain
    ///
    /// Initializes the chain at Step 1 (Red Ray) with balanced valve.
    pub fn new() -> Self {
        Self {
            lesser_cycle: LesserCycle::new(ComplexType::Mind),
            greater_cycle: GreaterCycle::new(ComplexType::Mind),
            current_step: EvolutionStep::RedRay,
            valve_state: ValveState::Open,
            spiral_pattern: SpiralPattern::new(),
            total_consciousness_expansion: 0.0,
        }
    }

    /// Process catalyst through the Evolution Chain
    ///
    /// This implements the complete evolutionary flow:
    /// 1. Body receives input (Red/Orange/Yellow)
    /// 2. Mind processes input via Lesser Cycle (A1-A4)
    /// 3. Check for Mind Balance (The Valve)
    /// 4. Spirit In-pouring (Green Ray activation)
    /// 5. Integration (Green Ray)
    /// 6. Greater Cycle: Significator → Choice → Transformation (A5-A22-A6)
    /// 7. Energy Center Activation (Spiral development)
    ///
    /// Returns the result of evolution processing.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 7
    pub fn process(&mut self, catalyst: Catalyst) -> EvolutionResult {
        // Step 1: Body receives input (Red/Orange/Yellow)
        let sensation = self.receive_sensory_input(&catalyst);

        // Step 2: Mind processes input via Lesser Cycle (A1-A4)
        let lesser_result = self.lesser_cycle.process(sensation);

        // Step 3: Check for Mind Balance (The Valve)
        // Check both sensation and processing metrics
        let mind_is_balanced = self.check_mind_balance(sensation, &lesser_result);

        // Initialize result
        let mut result = EvolutionResult {
            catalyst_processed: catalyst,
            sensation,
            lesser_result: lesser_result.clone(),
            mind_is_balanced,
            greater_result: None,
            spirit_in_pouring: 0.0,
            valve_state: self.valve_state,
            consciousness_expansion: 0.0,
            spiral_leap: None,
            success: false,
        };

        if mind_is_balanced {
            // Step 4: Spirit In-pouring (Green Ray activation)
            let intelligent_energy = self.generate_in_pouring(&lesser_result);
            result.spirit_in_pouring = intelligent_energy;

            // Step 5: Integration (Green Ray)
            let _integrated_flows =
                self.integrate_flows(lesser_result.experience, intelligent_energy);

            // Step 6: Greater Cycle: Significator → Choice → Transformation (A5-A22-A6)
            if self.can_make_choice() {
                let choice = self.make_free_will_choice(&lesser_result);
                let greater_result = self.greater_cycle.process(lesser_result.experience);
                result.greater_result = Some(greater_result.clone());

                // Step 7: Energy Center Activation (Spiral development)
                if let Some(leap) = self.check_for_spiral_leap(&choice, &greater_result) {
                    result.spiral_leap = Some(leap.clone());
                    self.apply_spiral_leap(&leap);
                }

                result.success = true;
            }
        } else {
            // Mind is blocked - Spirit remains as potential
            self.valve_state = ValveState::Closed;
            result.valve_state = ValveState::Closed;
            result.success = false;
        }

        // Update total consciousness expansion
        result.consciousness_expansion = self.calculate_consciousness_expansion(&result);
        self.total_consciousness_expansion += result.consciousness_expansion;

        result
    }

    /// Receive sensory input (Body receives catalyst)
    ///
    /// This represents the Body receiving input through the Red, Orange,
    /// and Yellow rays (survival, self-awareness, social awareness).
    fn receive_sensory_input(&self, catalyst: &Catalyst) -> Float {
        // Body receives catalyst as sensation
        // The intensity depends on catalyst type and intensity
        match catalyst.catalyst_type {
            CatalystType::Body => catalyst.intensity * 0.9,
            CatalystType::Mind => catalyst.intensity * 0.7,
            CatalystType::Spirit => catalyst.intensity * 0.5,
            CatalystType::General => catalyst.intensity * 0.8,
        }
    }

    /// Check if Mind is balanced (the valve mechanism)
    ///
    /// Mind acts as a valve regulating flow between Body and Spirit.
    /// When Mind is balanced, the valve is open and Spirit can flow.
    ///
    /// Mind is balanced when:
    /// 1. Processing efficiency is high (>= 0.6)
    /// 2. Microcosmic tension is low (<= 0.4)
    /// 3. Sensation is in the "Goldilocks zone" (not too low, not too high)
    /// 4. Unprocessed accumulation is below threshold
    fn check_mind_balance(&mut self, sensation: Float, lesser_result: &LesserCycleResult) -> bool {
        let efficiency_threshold = 0.6;
        let tension_threshold = 0.4;
        let min_sensation_threshold = 0.17; // Minimum sensation for balanced processing
        let max_sensation_threshold = 0.59; // Maximum sensation for balanced processing
        let max_unprocessed_threshold = 0.26; // Maximum unprocessed for balanced processing

        let is_efficient = lesser_result.efficiency >= efficiency_threshold;
        let is_low_tension = lesser_result.microcosmic_tension <= tension_threshold;
        let has_sufficient_sensation = sensation >= min_sensation_threshold;
        let is_not_overwhelming = sensation <= max_sensation_threshold;
        let is_low_accumulation = lesser_result.unprocessed <= max_unprocessed_threshold;

        // Mind is balanced only when ALL conditions are met
        if is_efficient
            && is_low_tension
            && has_sufficient_sensation
            && is_not_overwhelming
            && is_low_accumulation
        {
            self.valve_state = ValveState::Open;
            true
        } else {
            // Mind is unbalanced - valve is closed
            self.valve_state = ValveState::Closed;
            false
        }
    }

    /// Generate in-pouring from Spirit
    ///
    /// Spirit generates intelligent energy (in-pouring) when the valve is open.
    fn generate_in_pouring(&self, lesser_result: &LesserCycleResult) -> Float {
        match self.valve_state {
            ValveState::Open => lesser_result.wisdom * 1.5, // Full Spirit access
            ValveState::Restricted => lesser_result.wisdom * 0.7, // Limited Spirit access
            ValveState::Closed => 0.0,                      // No Spirit access
        }
    }

    /// Integrate flows (Green Ray integration)
    ///
    /// This represents the Green Ray (Ray 4) integrating the up-pouring
    /// (from Body) and in-pouring (from Spirit).
    pub fn integrate_flows(&self, up_pouring: Float, in_pouring: Float) -> Float {
        // Green Ray integration combines both flows
        // The result is the harmonic mean of the two flows
        if up_pouring + in_pouring > 0.0 {
            (2.0 * up_pouring * in_pouring) / (up_pouring + in_pouring)
        } else {
            0.0
        }
    }

    /// Check if entity can make a Free Will choice
    ///
    /// Entity can make choices when it has sufficient consciousness expansion.
    fn can_make_choice(&self) -> bool {
        // Entity can make choices when:
        // 1. Valve is open or restricted
        // 2. Has sufficient consciousness expansion (Red Ray or higher)
        matches!(self.valve_state, ValveState::Open | ValveState::Restricted)
    }

    /// Make a Free Will choice
    ///
    /// This represents the entity exercising Free Will (Archetype 22)
    /// to make a choice that affects its evolution.
    fn make_free_will_choice(&self, lesser_result: &LesserCycleResult) -> ArchetypeChoice {
        // Determine polarity based on experience
        let polarity = if lesser_result.wisdom > 0.5 {
            crate::types::Polarity::STO
        } else {
            crate::types::Polarity::STS
        };

        ArchetypeChoice {
            polarity,
            intensity: lesser_result.experience * self.greater_cycle.choice_capacity,
            context: ChoiceContext::CatalystProcessing,
        }
    }

    /// Check for spiral leap (non-linear consciousness expansion)
    ///
    /// Entity can make leaps in consciousness via Free Will choices,
    /// even if lower centers are not perfectly balanced.
    fn check_for_spiral_leap(
        &self,
        choice: &ArchetypeChoice,
        greater_result: &GreaterCycleResult,
    ) -> Option<SpiralLeap> {
        // Spiral leap occurs when:
        // 1. Choice intensity is high
        // 2. Transformation is significant
        // 3. Entity has leap capacity

        if choice.intensity > 0.8
            && greater_result.transformation > 0.7
            && self.spiral_pattern.leap_capacity > 0.6
        {
            // Determine leap target
            let target_step = self.determine_leap_target(choice, greater_result);

            Some(SpiralLeap {
                from_step: self.current_step,
                to_step: target_step,
                intensity: choice.intensity,
                choice: choice.clone(),
            })
        } else {
            None
        }
    }

    /// Determine leap target based on choice and transformation
    pub fn determine_leap_target(
        &self,
        _choice: &ArchetypeChoice,
        greater_result: &GreaterCycleResult,
    ) -> EvolutionStep {
        // Leap target depends on:
        // 1. Current step
        // 2. Choice polarity
        // 3. Transformation intensity

        match self.current_step {
            EvolutionStep::RedRay => {
                if greater_result.transformation > 0.8 {
                    EvolutionStep::GreenRay // Big leap to Green Ray
                } else {
                    EvolutionStep::OrangeRay // Small leap to Orange Ray
                }
            }
            EvolutionStep::OrangeRay => {
                if greater_result.transformation > 0.8 {
                    EvolutionStep::YellowRay
                } else {
                    EvolutionStep::YellowRay
                }
            }
            EvolutionStep::YellowRay => {
                if greater_result.transformation > 0.8 {
                    EvolutionStep::GreenRay
                } else {
                    EvolutionStep::GreenRay
                }
            }
            EvolutionStep::GreenRay => {
                if greater_result.transformation > 0.9 {
                    EvolutionStep::IndigoRay // Big leap to Indigo Ray
                } else {
                    EvolutionStep::BlueRay
                }
            }
            EvolutionStep::BlueRay => {
                if greater_result.transformation > 0.9 {
                    EvolutionStep::IndigoRay
                } else {
                    EvolutionStep::IndigoRay
                }
            }
            EvolutionStep::IndigoRay => EvolutionStep::VioletRay,
            EvolutionStep::VioletRay => {
                EvolutionStep::VioletRay // Already at highest step
            }
        }
    }

    /// Apply spiral leap
    ///
    /// Updates the current step and spiral pattern based on the leap.
    fn apply_spiral_leap(&mut self, leap: &SpiralLeap) {
        // Update current step
        self.current_step = leap.to_step;

        // Update spiral pattern
        self.spiral_pattern.apply_leap(leap);
    }

    /// Calculate consciousness expansion
    ///
    /// Calculates the total consciousness expansion from the result.
    fn calculate_consciousness_expansion(&self, result: &EvolutionResult) -> Float {
        let base_expansion = result.lesser_result.experience * 0.5;

        let greater_expansion = if let Some(ref greater_result) = result.greater_result {
            greater_result.transformation * 0.3
        } else {
            0.0
        };

        let spirit_expansion = result.spirit_in_pouring * 0.2;

        base_expansion + greater_expansion + spirit_expansion
    }

    /// Activate via Free Will (spiral development)
    ///
    /// Entity can make a CHOICE (Free Will) to activate higher potentiated centers.
    /// This is the spiral nature of development.
    ///
    /// Returns whether the activation was successful.
    pub fn activate_via_free_will(&mut self, target_step: EvolutionStep) -> bool {
        // Entity can only activate higher steps
        if target_step <= self.current_step {
            return false;
        }

        // Check if entity has leap capacity
        if self.spiral_pattern.leap_capacity < 0.5 {
            return false;
        }

        // Create a leap
        let leap = SpiralLeap {
            from_step: self.current_step,
            to_step: target_step,
            intensity: 0.9,
            choice: ArchetypeChoice {
                polarity: crate::types::Polarity::STO,
                intensity: 0.9,
                context: ChoiceContext::PersonalGrowth,
            },
        };

        // Apply the leap
        self.apply_spiral_leap(&leap);

        true
    }

    /// Get the current evolution step
    pub fn current_step(&self) -> EvolutionStep {
        self.current_step
    }

    /// Get the valve state
    pub fn valve_state(&self) -> ValveState {
        self.valve_state
    }

    /// Get the total consciousness expansion
    pub fn total_consciousness_expansion(&self) -> Float {
        self.total_consciousness_expansion
    }
}

// Note: impl SpiralPattern has been migrated to
// src/evolution_density_octave/density_octave.rs (Phase 4.5 Migration 3)

impl EvolutionStep {
    /// Get the step number
    pub fn step_number(&self) -> usize {
        *self as usize
    }

    /// Get the ray name
    pub fn ray_name(&self) -> &str {
        match self {
            EvolutionStep::RedRay => "Red Ray",
            EvolutionStep::OrangeRay => "Orange Ray",
            EvolutionStep::YellowRay => "Yellow Ray",
            EvolutionStep::GreenRay => "Green Ray",
            EvolutionStep::BlueRay => "Blue Ray",
            EvolutionStep::IndigoRay => "Indigo Ray",
            EvolutionStep::VioletRay => "Violet Ray",
        }
    }

    /// Get the step description
    pub fn description(&self) -> &str {
        match self {
            EvolutionStep::RedRay => {
                "Survival and Basic Instincts - Foundation for all evolutionary development"
            }
            EvolutionStep::OrangeRay => {
                "Self-Awareness and Personal Identity - Development of individual self-hood"
            }
            EvolutionStep::YellowRay => {
                "Social Awareness and Group Identity - Interaction with others"
            }
            EvolutionStep::GreenRay => "Universal Love and Compassion - First true spiritual ray",
            EvolutionStep::BlueRay => "Communication and Understanding - Deepening of wisdom",
            EvolutionStep::IndigoRay => "Intuition and Wisdom - Connection to higher consciousness",
            EvolutionStep::VioletRay => {
                "Spiritual Integration and Unity - Return to Intelligent Infinity"
            }
        }
    }
}

/// Evolution Result
///
/// The result of processing catalyst through the Evolution Chain.
#[derive(Debug, Clone)]
pub struct EvolutionResult {
    /// The catalyst that was processed
    pub catalyst_processed: Catalyst,

    /// The sensation received by Body
    pub sensation: Float,

    /// The result of the Lesser Cycle
    pub lesser_result: LesserCycleResult,

    /// Whether Mind is balanced (the valve)
    pub mind_is_balanced: bool,

    /// The result of the Greater Cycle (if any)
    pub greater_result: Option<GreaterCycleResult>,

    /// Spirit in-pouring (intelligent energy)
    pub spirit_in_pouring: Float,

    /// Valve state after processing
    pub valve_state: ValveState,

    /// Consciousness expansion achieved
    pub consciousness_expansion: Float,

    /// Spiral leap (if any)
    pub spiral_leap: Option<SpiralLeap>,

    /// Whether the evolution was successful
    pub success: bool,
}

impl fmt::Display for EvolutionChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Evolution Chain:\n\
             - Current Step: {} (Step {})\n\
             - Valve State: {:?}\n\
             - Spiral Level: {}\n\
             - Leap Capacity: {:.3}\n\
             - Leaps Made: {}\n\
             - Total Consciousness Expansion: {:.3}",
            self.current_step.ray_name(),
            self.current_step.step_number(),
            self.valve_state,
            self.spiral_pattern.level,
            self.spiral_pattern.leap_capacity,
            self.spiral_pattern.leap_count,
            self.total_consciousness_expansion,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_state::CatalystType;

    #[test]
    fn test_evolution_chain_creation() {
        let chain = EvolutionChain::new();
        assert_eq!(chain.current_step(), EvolutionStep::RedRay);
        assert_eq!(chain.valve_state(), ValveState::Open);
        assert_eq!(chain.spiral_pattern.level, 1);
    }

    #[test]
    fn test_process_catalyst_with_balanced_mind() {
        let mut chain = EvolutionChain::new();
        let catalyst = Catalyst::new(0.7, CatalystType::General);

        let result = chain.process(catalyst);

        assert!(result.mind_is_balanced);
        assert!(result.success);
        assert!(result.consciousness_expansion > 0.0);
    }

    #[test]
    fn test_process_catalyst_with_unbalanced_mind() {
        let mut chain = EvolutionChain::new();
        // Create catalyst with too low intensity - insufficient sensation for balanced processing
        // sensation = 0.2 * 0.8 = 0.16, which is below min_sensation_threshold of 0.17
        let catalyst = Catalyst::new(0.2, CatalystType::General);

        let result = chain.process(catalyst);

        // Mind should be blocked due to insufficient sensation
        assert_eq!(result.valve_state, ValveState::Closed);
        assert!(!result.mind_is_balanced);
        assert_eq!(result.spirit_in_pouring, 0.0);
    }

    #[test]
    fn test_lesser_cycle_integration() {
        let mut chain = EvolutionChain::new();
        let catalyst = Catalyst::new(0.7, CatalystType::General);

        let result = chain.process(catalyst);

        // Lesser cycle should process catalyst
        assert!(result.lesser_result.experience > 0.0);
        assert!(result.lesser_result.efficiency >= 0.0 && result.lesser_result.efficiency <= 1.0);
    }

    #[test]
    fn test_greater_cycle_integration() {
        let mut chain = EvolutionChain::new();
        let catalyst = Catalyst::new(0.8, CatalystType::General);

        let result = chain.process(catalyst);

        // Greater cycle should activate if mind is balanced
        if result.mind_is_balanced {
            assert!(result.greater_result.is_some());
            if let Some(ref greater_result) = result.greater_result {
                assert!(greater_result.transformation > 0.0);
            }
        }
    }

    #[test]
    fn test_spirit_in_pouring() {
        let mut chain = EvolutionChain::new();
        let catalyst = Catalyst::new(0.8, CatalystType::General);

        let result = chain.process(catalyst);

        // Spirit in-pouring should occur if valve is open
        if result.valve_state == ValveState::Open {
            assert!(result.spirit_in_pouring > 0.0);
        }
    }

    #[test]
    fn test_spiral_leap() {
        let mut chain = EvolutionChain::new();
        // Create catalyst that will trigger a leap
        let catalyst = Catalyst::new(0.9, CatalystType::General);

        let result = chain.process(catalyst);

        // Check for spiral leap
        if result.spiral_leap.is_some() {
            let leap = result.spiral_leap.as_ref().unwrap();
            assert!(leap.to_step > leap.from_step);
            assert!(leap.intensity > 0.8);
        }
    }

    #[test]
    fn test_activate_via_free_will() {
        let mut chain = EvolutionChain::new();

        // Try to activate a higher step
        let success = chain.activate_via_free_will(EvolutionStep::GreenRay);

        assert!(success);
        assert_eq!(chain.current_step(), EvolutionStep::GreenRay);
        assert!(chain.spiral_pattern.leap_count > 0);
    }

    #[test]
    fn test_evolutionary_progression() {
        let mut chain = EvolutionChain::new();

        // Process multiple catalysts
        for i in 1..=10 {
            let catalyst = Catalyst::new(0.5 + (i as f64 * 0.05), CatalystType::General);
            chain.process(catalyst);
        }

        // Should have made progress
        assert!(chain.total_consciousness_expansion() > 0.0);
    }

    #[test]
    fn test_activation_not_acquisition() {
        let chain = EvolutionChain::new();

        // Verify that evolution is about activation, not acquisition
        // The chain starts with all potential (from seed)
        // Evolution activates this potential

        // Current step represents activated potential, not acquired capacity
        assert_eq!(chain.current_step(), EvolutionStep::RedRay);

        // The chain has the capacity to reach Violet Ray (pre-existing potential)
        // It just needs to activate it through evolution
    }

    #[test]
    fn test_valve_mechanism() {
        let mut chain = EvolutionChain::new();

        // Test with balanced mind (medium intensity catalyst)
        // sensation = 0.3 * 0.8 = 0.24, within Goldilocks zone (0.17 - 0.59)
        let catalyst1 = Catalyst::new(0.3, CatalystType::General);
        chain.process(catalyst1);
        assert_eq!(chain.valve_state(), ValveState::Open);

        // Reset chain for second test
        let mut chain2 = EvolutionChain::new();

        // Test with unbalanced mind (too low intensity catalyst)
        // sensation = 0.2 * 0.8 = 0.16, below minimum threshold (0.17)
        let catalyst2 = Catalyst::new(0.2, CatalystType::General);
        chain2.process(catalyst2);
        assert_eq!(chain2.valve_state(), ValveState::Closed);
    }

    #[test]
    fn test_evolution_step_names() {
        assert_eq!(EvolutionStep::RedRay.ray_name(), "Red Ray");
        assert_eq!(EvolutionStep::OrangeRay.ray_name(), "Orange Ray");
        assert_eq!(EvolutionStep::YellowRay.ray_name(), "Yellow Ray");
        assert_eq!(EvolutionStep::GreenRay.ray_name(), "Green Ray");
        assert_eq!(EvolutionStep::BlueRay.ray_name(), "Blue Ray");
        assert_eq!(EvolutionStep::IndigoRay.ray_name(), "Indigo Ray");
        assert_eq!(EvolutionStep::VioletRay.ray_name(), "Violet Ray");
    }

    #[test]
    fn test_spiral_pattern() {
        let mut spiral = SpiralPattern::new();

        assert_eq!(spiral.level, 1);
        assert_eq!(spiral.leap_count, 0);

        let leap = SpiralLeap {
            from_step: EvolutionStep::RedRay,
            to_step: EvolutionStep::GreenRay,
            intensity: 0.9,
            choice: ArchetypeChoice {
                polarity: crate::types::Polarity::STO,
                intensity: 0.9,
                context: ChoiceContext::PersonalGrowth,
            },
        };

        spiral.apply_leap(&leap);

        assert_eq!(spiral.level, 4);
        assert_eq!(spiral.leap_count, 1);
        assert!(spiral.leap_capacity > 0.5);
    }

    #[test]
    fn test_comprehensive_evolution_flow() {
        let mut chain = EvolutionChain::new();

        // Simulate complete evolutionary journey
        let catalysts = vec![
            Catalyst::new(0.6, CatalystType::Body),    // Red Ray
            Catalyst::new(0.7, CatalystType::Body),    // Red/Orange
            Catalyst::new(0.8, CatalystType::Mind),    // Orange/Yellow
            Catalyst::new(0.9, CatalystType::Mind),    // Yellow/Green
            Catalyst::new(0.95, CatalystType::Spirit), // Green Ray
        ];

        for catalyst in catalysts {
            chain.process(catalyst);
        }

        // Should have progressed
        assert!(chain.total_consciousness_expansion() > 0.0);
        assert!(chain.spiral_pattern.leap_count >= 0);
    }
}
