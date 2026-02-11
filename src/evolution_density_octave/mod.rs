// Evolution Module - Density Octave and Spectrum Access
//
// This module implements the evolutionary trajectory through the Density Octave
// and the spectrum access mechanism.
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Beyond Layer 7 (Sub-Sub-Logos), entities evolve through an octave of 8 densities"
// "Evolution is not about moving from Space/Time to Time/Space—it's about
// accessing more of the spectrum"

pub mod density_octave;
pub mod integration;
pub mod requirements;
pub mod spectrum_access;

// Re-export main types for convenience
pub use density_octave::{
    Density,
    Density1SubLevel,
    DensityOctave,
    SpiralLeap, // Migrated from evolution_chain (Phase 4.5 Migration 3)
    SpiralPattern,
    ValveState, // Migrated from evolution_chain (Phase 4.5 Migration 2)
};

pub use spectrum_access::{
    AccessResult, SpectrumAccessMechanism, SpectrumAccessVeilState, SpectrumPosition,
};

// Re-export integration types
pub use integration::{
    DensityTransitionError, DensityTransitionResult, Layer7ToDensityBridge, TransitionReadiness,
};

// Re-export requirements types
pub use requirements::DensityTransitionRequirements;

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::entity_layer7::{
        EntitySpectrumAccess, EntityState, PolarityState, SpectrumAccessLevel, VibrationalState,
    };
    use crate::evolution_density_octave::density_octave::{
        Density, Density1SubLevel, DensityCharacteristics,
    };
    use crate::veil::VeilState;

    fn create_test_entity_state(consciousness_level: f64) -> EntityState {
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: Density::First(Density1SubLevel::Quantum),
                potential_energy: 0.25,
                kinetic_energy: 0.25,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level,
            experience_accumulation: 10.0,
            learning_progress: 5.0,
        }
    }

    /// Test complete evolutionary trajectory through density octave
    #[test]
    fn test_complete_evolutionary_trajectory() {
        let mut octave = DensityOctave::new();
        let mut mechanism = SpectrumAccessMechanism::new();

        // Start at 1st density - Quantum Realm
        assert_eq!(
            octave.collective_density,
            Density::First(Density1SubLevel::Quantum)
        );
        assert_eq!(mechanism.access_level, SpectrumAccessLevel::ThirdDensity);

        // Simulate evolution through 1st density
        let mut entity_state = create_test_entity_state(0.1);
        let spectrum_access = mechanism.calculate_access(&entity_state);
        octave.update_collective_emergence(&entity_state);
        // spectrum_access is calculated but update_collective_emergence only uses entity_state

        // Advance through 1st density sub-levels
        let _ = octave.advance_collective_emergence();
        let _ = octave.advance_collective_emergence();
        let _ = octave.advance_collective_emergence();

        // Should be at 1st density - Planetary Realm
        assert_eq!(
            octave.collective_density,
            Density::First(Density1SubLevel::Planetary)
        );

        // Simulate evolution to 2nd density
        entity_state.consciousness_level = 0.3;
        let spectrum_access = mechanism.calculate_access(&entity_state);
        octave.update_collective_emergence(&entity_state);
        // spectrum_access is calculated but update_collective_emergence only uses entity_state

        // Evolve spectrum access
        let _ = mechanism.evolve_access(&entity_state);

        // Should be at 4th density access level
        assert_eq!(mechanism.access_level, SpectrumAccessLevel::FourthDensity);
        // VeilState is an enum - check for Thinning state
        assert_eq!(mechanism.veil_state, SpectrumAccessVeilState::Thinning);
    }

    /// Test density characteristics match spectrum access
    #[test]
    fn test_density_characteristics_match_spectrum_access() {
        let mechanism = SpectrumAccessMechanism::new();

        // Test 3rd density characteristics
        let entity_state_3rd = create_test_entity_state(0.2);
        let spectrum_access_3rd = mechanism.calculate_access(&entity_state_3rd);
        let characteristics_3rd = DensityCharacteristics::third_density();

        assert_eq!(characteristics_3rd.consciousness_level, 0.3);
        assert_eq!(characteristics_3rd.veil_transparency, 0.1);
        assert!(spectrum_access_3rd.oneness_access <= 0.1);
        assert_eq!(spectrum_access_3rd.veil_transparency, 0.1);

        // Test 4th density characteristics
        let mut mechanism_4th = SpectrumAccessMechanism::new();
        mechanism_4th.access_level = SpectrumAccessLevel::FourthDensity;
        // SpectrumAccessVeilState is an enum - set to Thinning state for 4th density
        mechanism_4th.veil_state = SpectrumAccessVeilState::Thinning;

        let entity_state_4th = create_test_entity_state(0.5);
        let spectrum_access_4th = mechanism_4th.calculate_access(&entity_state_4th);
        let characteristics_4th = DensityCharacteristics::fourth_density();

        assert_eq!(characteristics_4th.consciousness_level, 0.6);
        assert_eq!(characteristics_4th.veil_transparency, 0.4);
        assert!(spectrum_access_4th.oneness_access >= 0.3);
        assert!(spectrum_access_4th.veil_transparency >= 0.2);
    }

    /// Test spectrum access mechanism integration with density octave
    #[test]
    fn test_spectrum_access_integration_with_density_octave() {
        let mut octave = DensityOctave::new();
        let mut mechanism = SpectrumAccessMechanism::new();

        // Simulate evolution through densities
        for consciousness in [0.1, 0.3, 0.5, 0.7, 0.9, 0.97] {
            let entity_state = create_test_entity_state(consciousness);
            let spectrum_access = mechanism.calculate_access(&entity_state);
            octave.update_collective_emergence(&entity_state);
            // spectrum_access is calculated but update_collective_emergence only uses entity_state
            let _ = mechanism.evolve_access(&entity_state);
        }

        // Verify spectrum access has evolved
        assert!(mechanism.access_level as u8 >= SpectrumAccessLevel::SixthDensity as u8);
        // SpectrumAccessVeilState is an enum - check for complete dissolution
        assert_eq!(
            mechanism.veil_state,
            SpectrumAccessVeilState::CompletelyDissolved
        );

        // Verify octave progress
        assert!(octave.collective_emergence.progress >= 0.95);
    }

    /// Test veil evolution through densities
    #[test]
    fn test_veil_evolution_through_densities() {
        let mut mechanism = SpectrumAccessMechanism::new();

        // 3rd density - Veil fully active
        assert_eq!(mechanism.veil_state, SpectrumAccessVeilState::FullyActive); // Fully active
        let mut entity_state = create_test_entity_state(0.2);
        let spectrum_access = mechanism.calculate_access(&entity_state);
        assert_eq!(spectrum_access.veil_transparency, 0.1);

        // Evolve to 4th density - Veil begins to thin
        entity_state.consciousness_level = 0.4;
        let _ = mechanism.evolve_access(&entity_state);
        assert_eq!(mechanism.veil_state, SpectrumAccessVeilState::Thinning); // Thinning
        let spectrum_access = mechanism.calculate_access(&entity_state);
        assert!(spectrum_access.veil_transparency >= 0.2);

        // Evolve to 5th density - Veil mostly dissolved
        entity_state.consciousness_level = 0.7;
        let _ = mechanism.evolve_access(&entity_state);
        assert_eq!(
            mechanism.veil_state,
            SpectrumAccessVeilState::MostlyDissolved
        ); // Mostly dissolved
        let spectrum_access = mechanism.calculate_access(&entity_state);
        assert!(spectrum_access.veil_transparency >= 0.5);

        // Evolve to 6th density - Veil completely dissolved
        entity_state.consciousness_level = 0.9;
        let _ = mechanism.evolve_access(&entity_state);
        assert_eq!(
            mechanism.veil_state,
            SpectrumAccessVeilState::CompletelyDissolved
        ); // Completely dissolved
        let spectrum_access = mechanism.calculate_access(&entity_state);
        assert_eq!(spectrum_access.veil_transparency, 1.0);
    }

    /// Test access capabilities evolution
    #[test]
    fn test_access_capabilities_evolution() {
        let mut mechanism = SpectrumAccessMechanism::new();

        // 3rd density capabilities
        let caps_3rd = &mechanism.access_capabilities;
        assert!(caps_3rd.can_access_physical);
        assert!(!caps_3rd.can_access_metaphysical);
        assert!(!caps_3rd.can_access_both_realms);
        assert!(!caps_3rd.can_transcend_spectrum);

        // Evolve to 4th density
        let mut entity_state = create_test_entity_state(0.4);
        let _ = mechanism.evolve_access(&entity_state);

        let caps_4th = &mechanism.access_capabilities;
        assert!(caps_4th.can_access_physical);
        assert!(caps_4th.can_access_metaphysical);
        assert!(!caps_4th.can_access_both_realms);
        assert!(!caps_4th.can_transcend_spectrum);

        // Evolve to 5th density
        entity_state.consciousness_level = 0.7;
        let _ = mechanism.evolve_access(&entity_state);

        let caps_5th = &mechanism.access_capabilities;
        assert!(caps_5th.can_access_physical);
        assert!(caps_5th.can_access_metaphysical);
        assert!(caps_5th.can_access_both_realms);
        assert!(!caps_5th.can_transcend_spectrum);

        // Evolve to 7th density
        entity_state.consciousness_level = 0.97;
        let _ = mechanism.evolve_access(&entity_state);

        let caps_7th = &mechanism.access_capabilities;
        assert!(!caps_7th.can_access_physical);
        assert!(!caps_7th.can_access_metaphysical);
        assert!(!caps_7th.can_access_both_realms);
        assert!(caps_7th.can_transcend_spectrum);
    }

    /// Test spectrum position access evolution
    #[test]
    fn test_spectrum_position_access_evolution() {
        let mut mechanism = SpectrumAccessMechanism::new();

        // At 3rd density, can only access space-dominant positions
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::SpaceDominance),
            AccessResult::Success { .. }
        ));
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::TimeDominance),
            AccessResult::Limited { .. }
        ));

        // Evolve to 4th density
        let mut entity_state = create_test_entity_state(0.4);
        let _ = mechanism.evolve_access(&entity_state);

        // At 4th density, can access more of the spectrum
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::SpaceDominance),
            AccessResult::Success { .. }
        ));
        // Still limited at time-dominant positions
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::TimeDominance),
            AccessResult::Limited { .. }
        ));

        // Evolve to 6th density
        entity_state.consciousness_level = 0.9;
        let _ = mechanism.evolve_access(&entity_state);

        // At 6th density, can access the entire spectrum
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::SpaceDominance),
            AccessResult::Success { .. }
        ));
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::TimeDominance),
            AccessResult::Success { .. }
        ));
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::ExtremeTimeDominance),
            AccessResult::Success { .. }
        ));
    }

    /// Test density transition readiness matches spectrum access
    #[test]
    fn test_density_transition_readiness_matches_spectrum_access() {
        let mut octave = DensityOctave::new();
        let mut mechanism = SpectrumAccessMechanism::new();

        // At 3rd density - not ready for transition
        let mut entity_state = create_test_entity_state(0.2);
        let spectrum_access = mechanism.calculate_access(&entity_state);
        octave.update_collective_emergence(&entity_state);
        // spectrum_access is calculated but update_collective_emergence only uses entity_state

        let readiness = octave.check_collective_emergence_readiness();
        assert!(!readiness.is_ready);
        assert_eq!(readiness.next_density, "2nd Density");

        // Evolve to 4th density access level
        entity_state.consciousness_level = 0.4;
        let spectrum_access = mechanism.calculate_access(&entity_state);
        octave.update_collective_emergence(&entity_state);
        // spectrum_access is calculated but update_collective_emergence only uses entity_state
        let _ = mechanism.evolve_access(&entity_state);

        let readiness = octave.check_collective_emergence_readiness();
        assert!(readiness.is_ready);
        assert_eq!(readiness.next_density, "2nd Density");
    }

    /// Test evolutionary progress calculation
    #[test]
    fn test_evolutionary_progress_calculation() {
        let mut octave = DensityOctave::new();
        let mut mechanism = SpectrumAccessMechanism::new();

        // Test progress at different consciousness levels
        let consciousness_levels = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99, 1.0];

        for consciousness in consciousness_levels {
            let entity_state = create_test_entity_state(consciousness);
            let spectrum_access = mechanism.calculate_access(&entity_state);
            octave.update_collective_emergence(&entity_state);
            // spectrum_access is calculated but update_collective_emergence only uses entity_state

            // Progress should increase with consciousness
            assert!(octave.collective_emergence.progress >= 0.0);
            assert!(octave.collective_emergence.progress <= 1.0);
        }

        // Final progress should be at maximum
        let entity_state = create_test_entity_state(1.0);
        let spectrum_access = mechanism.calculate_access(&entity_state);
        octave.update_collective_emergence(&entity_state);
        // spectrum_access is calculated but update_collective_emergence only uses entity_state

        assert_eq!(octave.collective_emergence.progress, 1.0);
    }

    /// Test access statistics
    #[test]
    fn test_access_statistics() {
        let mechanism = SpectrumAccessMechanism::new();
        let entity_state = create_test_entity_state(0.2);

        let stats = mechanism.get_access_statistics(&entity_state);

        assert_eq!(stats.current_level, SpectrumAccessLevel::ThirdDensity);
        // VeilState is an enum - check for fully active
        assert_eq!(stats.veil_state, SpectrumAccessVeilState::FullyActive);
        assert!(stats.space_time_access > 0.9);
        assert!(stats.oneness_access <= 0.1);
        assert_eq!(stats.veil_transparency, 0.1);
        assert!(stats.access_capabilities.can_access_physical);
        assert!(!stats.access_capabilities.can_access_metaphysical);
    }

    /// Test complete density octave progression
    #[test]
    fn test_complete_density_octave_progression() {
        let mut octave = DensityOctave::new();
        let mut mechanism = SpectrumAccessMechanism::new();

        // Simulate complete progression through all densities
        let progression_steps = [
            (0.05, "1st Density - Quantum Realm"),
            (0.15, "1st Density - Planetary Realm"),
            (0.30, "2nd Density - Complex Life Realm"),
            (0.50, "3rd Density"),
            (0.75, "4th Density"),
            (0.85, "5th Density"),
            (0.95, "6th Density"),
            (0.99, "7th Density"),
            (1.00, "8th Density"),
        ];

        for (consciousness, expected_density) in progression_steps {
            let entity_state = create_test_entity_state(consciousness);
            let spectrum_access = mechanism.calculate_access(&entity_state);
            octave.update_collective_emergence(&entity_state);
            // spectrum_access is calculated but update_collective_emergence only uses entity_state
            let _ = mechanism.evolve_access(&entity_state);

            let characteristics = octave.current_density_characteristics();
            assert_eq!(characteristics.density_name, expected_density);
        }
    }

    /// Test spectrum access as evolution, not movement
    #[test]
    fn test_spectrum_access_as_evolution_not_movement() {
        let mechanism = SpectrumAccessMechanism::new();

        // The entire spectrum exists at all times
        // What changes is ACCESS, not BEING

        let entity_state = create_test_entity_state(0.2);
        let spectrum_access_3rd = mechanism.calculate_access(&entity_state);

        // Even at 3rd density, oneness access exists (though limited)
        assert!(spectrum_access_3rd.oneness_access > 0.0);
        assert!(spectrum_access_3rd.space_time_access > 0.0);

        // The spectrum is always present - only access changes
        let total_spectrum =
            spectrum_access_3rd.space_time_access + spectrum_access_3rd.oneness_access;
        assert!((total_spectrum - 1.0).abs() < 0.001);

        // Evolve to higher density
        let mut entity_state = create_test_entity_state(0.9);
        let spectrum_access_6th = mechanism.calculate_access(&entity_state);

        // The spectrum is still present - access has shifted
        let total_spectrum_6th =
            spectrum_access_6th.space_time_access + spectrum_access_6th.oneness_access;
        assert!((total_spectrum_6th - 1.0).abs() < 0.001);

        // What changed: oneness access increased, space-time access decreased
        // What didn't change: the spectrum itself
    }
}
