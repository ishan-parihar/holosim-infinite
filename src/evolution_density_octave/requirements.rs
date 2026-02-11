// Density Transition Requirements
//
// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
// "Define density requirements"
//
// This module defines the requirements for transitioning between densities
// in the collective system.

use crate::entity_layer7::layer7::EntityState;
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};

/// Requirements for density transition
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
/// "Requirements for density transition"
///
/// IMPORTANT: These requirements are for COLLECTIVE system emergence,
/// not for individual entity transitions. Individual entities DO NOT
/// change density - they progress within their assigned density.
#[derive(Debug, Clone, PartialEq)]
pub struct DensityTransitionRequirements {
    /// Minimum consciousness level required
    pub consciousness_level: f64,

    /// Minimum polarization required
    pub polarization: f64,

    /// Minimum coherence required
    pub coherence: f64,

    /// Catalyst threshold (experience accumulation)
    pub catalyst_threshold: f64,

    /// Veil transparency required
    pub veil_transparency: f64,

    /// Minimum experience accumulation required
    pub experience_threshold: f64,
}

impl DensityTransitionRequirements {
    /// Get requirements for transition between densities
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
    /// "Get requirements for transition from X to Y density"
    pub fn get_requirements(from: &Density, to: &Density) -> Self {
        match (from, to) {
            // 1st Density sub-level transitions
            (
                Density::First(Density1SubLevel::Quantum),
                Density::First(Density1SubLevel::Atomic),
            ) => Self::quantum_to_atomic(),
            (
                Density::First(Density1SubLevel::Atomic),
                Density::First(Density1SubLevel::Molecular),
            ) => Self::atomic_to_molecular(),
            (
                Density::First(Density1SubLevel::Molecular),
                Density::First(Density1SubLevel::Planetary),
            ) => Self::molecular_to_planetary(),

            // 1st → 2nd Density transition
            (Density::First(Density1SubLevel::Planetary), Density::Second(_)) => {
                Self::first_to_second()
            }

            // 2nd Density sub-level transitions
            (
                Density::Second(Density2SubLevel::Cellular),
                Density::Second(Density2SubLevel::SimpleLife),
            ) => Self::cellular_to_simple_life(),
            (
                Density::Second(Density2SubLevel::SimpleLife),
                Density::Second(Density2SubLevel::ComplexLife),
            ) => Self::simple_life_to_complex_life(),

            // 2nd → 3rd Density transition
            (Density::Second(Density2SubLevel::ComplexLife), Density::Third) => {
                Self::second_to_third()
            }

            // 3rd → 4th Density transition
            (Density::Third, Density::Fourth) => Self::third_to_fourth(),

            // 4th → 5th Density transition
            (Density::Fourth, Density::Fifth) => Self::fourth_to_fifth(),

            // 5th → 6th Density transition
            (Density::Fifth, Density::Sixth) => Self::fifth_to_sixth(),

            // 6th → 7th Density transition
            (Density::Sixth, Density::Seventh) => Self::sixth_to_seventh(),

            // 7th → 8th Density transition
            (Density::Seventh, Density::Eighth) => Self::seventh_to_eighth(),

            // Default requirements (for any transition)
            _ => Self::default(),
        }
    }

    /// 1st Density: Quantum → Atomic
    fn quantum_to_atomic() -> Self {
        Self {
            consciousness_level: 0.06,
            polarization: 0.1,
            coherence: 0.1,
            catalyst_threshold: 0.15,
            veil_transparency: 0.05,
            experience_threshold: 15.0,
        }
    }

    /// 1st Density: Atomic → Molecular
    fn atomic_to_molecular() -> Self {
        Self {
            consciousness_level: 0.12,
            polarization: 0.15,
            coherence: 0.15,
            catalyst_threshold: 0.30,
            veil_transparency: 0.05,
            experience_threshold: 30.0,
        }
    }

    /// 1st Density: Molecular → Planetary
    fn molecular_to_planetary() -> Self {
        Self {
            consciousness_level: 0.18,
            polarization: 0.2,
            coherence: 0.2,
            catalyst_threshold: 0.45,
            veil_transparency: 0.05,
            experience_threshold: 45.0,
        }
    }

    /// 1st → 2nd Density transition
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
    /// "Get requirements for transition from 1st to 2nd Density"
    fn first_to_second() -> Self {
        Self {
            consciousness_level: 0.25,
            polarization: 0.3,
            coherence: 0.25,
            catalyst_threshold: 0.7,
            veil_transparency: 0.1,
            experience_threshold: 62.5, // 250.0 * 0.25
        }
    }

    /// 2nd Density: Cellular → Simple Life
    fn cellular_to_simple_life() -> Self {
        Self {
            consciousness_level: 0.31,
            polarization: 0.35,
            coherence: 0.3,
            catalyst_threshold: 0.75,
            veil_transparency: 0.1,
            experience_threshold: 77.5, // 250.0 * 0.31
        }
    }

    /// 2nd Density: Simple Life → Complex Life
    fn simple_life_to_complex_life() -> Self {
        Self {
            consciousness_level: 0.40,
            polarization: 0.4,
            coherence: 0.35,
            catalyst_threshold: 0.8,
            veil_transparency: 0.1,
            experience_threshold: 100.0, // 250.0 * 0.40
        }
    }

    /// 2nd → 3rd Density transition
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
    /// "Get requirements for transition from 2nd to 3rd Density"
    fn second_to_third() -> Self {
        Self {
            consciousness_level: 0.50,
            polarization: 0.5,
            coherence: 0.45,
            catalyst_threshold: 0.85,
            veil_transparency: 0.15,
            experience_threshold: 125.0, // 250.0 * 0.50
        }
    }

    /// 3rd → 4th Density transition
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
    /// "Get requirements for transition from 3rd to 4th Density"
    fn third_to_fourth() -> Self {
        Self {
            consciousness_level: 0.75,
            polarization: 0.6,
            coherence: 0.6,
            catalyst_threshold: 0.9,
            veil_transparency: 0.4,
            experience_threshold: 187.5, // 250.0 * 0.75
        }
    }

    /// 4th → 5th Density transition
    fn fourth_to_fifth() -> Self {
        Self {
            consciousness_level: 0.85,
            polarization: 0.7,
            coherence: 0.7,
            catalyst_threshold: 0.92,
            veil_transparency: 0.6,
            experience_threshold: 212.5, // 250.0 * 0.85
        }
    }

    /// 5th → 6th Density transition
    fn fifth_to_sixth() -> Self {
        Self {
            consciousness_level: 0.95,
            polarization: 0.8,
            coherence: 0.8,
            catalyst_threshold: 0.95,
            veil_transparency: 0.8,
            experience_threshold: 237.5, // 250.0 * 0.95
        }
    }

    /// 6th → 7th Density transition
    fn sixth_to_seventh() -> Self {
        Self {
            consciousness_level: 0.97,
            polarization: 0.9,
            coherence: 0.9,
            catalyst_threshold: 0.98,
            veil_transparency: 0.95,
            experience_threshold: 242.5, // 250.0 * 0.97
        }
    }

    /// 7th → 8th Density transition
    fn seventh_to_eighth() -> Self {
        Self {
            consciousness_level: 0.99,
            polarization: 0.95,
            coherence: 0.95,
            catalyst_threshold: 0.99,
            veil_transparency: 1.0,
            experience_threshold: 247.5, // 250.0 * 0.99
        }
    }

    /// Check if entity state meets requirements
    pub fn entity_meets_requirements(&self, entity_state: &EntityState) -> bool {
        // Check consciousness level
        if entity_state.consciousness_level < self.consciousness_level {
            return false;
        }

        // Check polarization
        if entity_state.polarity_state.polarization_strength < self.polarization {
            return false;
        }

        // Check coherence
        if entity_state.vibrational_state.coherence < self.coherence {
            return false;
        }

        // Check experience accumulation
        if entity_state.experience_accumulation < self.experience_threshold {
            return false;
        }

        true
    }
}

impl Default for DensityTransitionRequirements {
    fn default() -> Self {
        Self {
            consciousness_level: 0.5,
            polarization: 0.5,
            coherence: 0.5,
            catalyst_threshold: 0.7,
            veil_transparency: 0.5,
            experience_threshold: 125.0,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{EntityState, PolarityState, VibrationalState};

    fn create_test_entity_state(
        consciousness: f64,
        polarization: f64,
        coherence: f64,
        experience: f64,
    ) -> EntityState {
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence,
                density: crate::evolution_density_octave::density_octave::Density::First(
                    Density1SubLevel::Quantum,
                ),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: polarization,
            },
            consciousness_level: consciousness,
            experience_accumulation: experience,
            learning_progress: 5.0,
        }
    }

    #[test]
    fn test_first_to_second_requirements() {
        let requirements = DensityTransitionRequirements::first_to_second();

        assert_eq!(requirements.consciousness_level, 0.25);
        assert_eq!(requirements.polarization, 0.3);
        assert_eq!(requirements.coherence, 0.25);
        assert_eq!(requirements.catalyst_threshold, 0.7);
        assert_eq!(requirements.veil_transparency, 0.1);
        assert_eq!(requirements.experience_threshold, 62.5);
    }

    #[test]
    fn test_second_to_third_requirements() {
        let requirements = DensityTransitionRequirements::second_to_third();

        assert_eq!(requirements.consciousness_level, 0.50);
        assert_eq!(requirements.polarization, 0.5);
        assert_eq!(requirements.coherence, 0.45);
        assert_eq!(requirements.catalyst_threshold, 0.85);
        assert_eq!(requirements.veil_transparency, 0.15);
        assert_eq!(requirements.experience_threshold, 125.0);
    }

    #[test]
    fn test_third_to_fourth_requirements() {
        let requirements = DensityTransitionRequirements::third_to_fourth();

        assert_eq!(requirements.consciousness_level, 0.75);
        assert_eq!(requirements.polarization, 0.6);
        assert_eq!(requirements.coherence, 0.6);
        assert_eq!(requirements.catalyst_threshold, 0.9);
        assert_eq!(requirements.veil_transparency, 0.4);
        assert_eq!(requirements.experience_threshold, 187.5);
    }

    #[test]
    fn test_entity_meets_requirements() {
        let requirements = DensityTransitionRequirements::second_to_third();

        // Entity that meets requirements
        let entity_meets = create_test_entity_state(0.6, 0.6, 0.5, 150.0);
        assert!(requirements.entity_meets_requirements(&entity_meets));

        // Entity that doesn't meet requirements (consciousness too low)
        let entity_not_meets_consciousness = create_test_entity_state(0.4, 0.6, 0.5, 150.0);
        assert!(!requirements.entity_meets_requirements(&entity_not_meets_consciousness));

        // Entity that doesn't meet requirements (polarization too low)
        let entity_not_meets_polarization = create_test_entity_state(0.6, 0.4, 0.5, 150.0);
        assert!(!requirements.entity_meets_requirements(&entity_not_meets_polarization));

        // Entity that doesn't meet requirements (experience too low)
        let entity_not_meets_experience = create_test_entity_state(0.6, 0.6, 0.5, 100.0);
        assert!(!requirements.entity_meets_requirements(&entity_not_meets_experience));
    }

    #[test]
    fn test_get_requirements() {
        let from = Density::First(Density1SubLevel::Planetary);
        let to = Density::Second(Density2SubLevel::Cellular);

        let requirements = DensityTransitionRequirements::get_requirements(&from, &to);

        assert!(requirements.consciousness_level > 0.0);
        assert!(requirements.polarization > 0.0);
        assert!(requirements.coherence > 0.0);
        assert!(requirements.catalyst_threshold > 0.0);
    }

    #[test]
    fn test_default_requirements() {
        let requirements = DensityTransitionRequirements::default();

        assert_eq!(requirements.consciousness_level, 0.5);
        assert_eq!(requirements.polarization, 0.5);
        assert_eq!(requirements.coherence, 0.5);
        assert_eq!(requirements.catalyst_threshold, 0.7);
        assert_eq!(requirements.veil_transparency, 0.5);
        assert_eq!(requirements.experience_threshold, 125.0);
    }
}
