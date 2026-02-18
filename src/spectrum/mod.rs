//! # Spectrum Module
//!
//! This module implements the spectrum and dimensional emergence (Yellow → Red).
//!
//! ## Components
//!
//! - **Larson Framework**: The reciprocal space/time framework
//! - **Veil**: The structural feature at v = 1
//! - **Yellow-Ray**: The Great Mystery - Space/Time and Time/Space Spectrum emergence
//! - **Orange-Ray**: Galactic-scale spectrum configuration
//! - **Red-Ray**: Solar-scale spectrum configuration + archetypical mind system
//! - **Archetypical Mind**: The training aid for entities in veiled experience

pub mod archetypical_mind;
pub mod larson_framework;
pub mod orange_realm;
pub mod red_realm;
pub mod veil;
pub mod yellow_realm;
pub mod dynamical_spectrum;
pub mod density_transitions;

// Re-export commonly used types
pub use archetypical_mind::{ArchetypeRole, ArchetypicalMind, ArchetypicalSystemType, ComplexType};
pub use larson_framework::{SpectrumQuality, SpectrumRatio, SpectrumSide};
pub use orange_realm::OrangeRealm;
pub use red_realm::RedRealm;
pub use yellow_realm::YellowRealm;
pub use dynamical_spectrum::{DynamicalSpectrum, DynamicalSpectrumConfig, DynamicalSpectrumStatistics, EntityChoice, PolarityOrientation, SpectrumTrajectory};
pub use density_transitions::{DensityTransitionResult, DensityTransitionStatistics, DensityTransitionSystem, DensityTransitionConfig, EntityTransitionState, TranscendAndInclude, TransitionReadiness, TransitionRequirement};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::foundation::green_realm::LightLoveField;
    use crate::spectrum::archetypical_mind::PolarityGuidance;
    use crate::spectrum::larson_framework::{LarsonFramework, ScalarMotionUnit};
    use crate::spectrum::veil::{SpectrumAccess, Veil};

    /// Create a test Light/Love Field for testing
    fn create_test_light_love_field() -> LightLoveField {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = crate::foundation::blue_realm::Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);

        // Add holographic patterns, rhythms, and fields for spectrum conditions
        field.add_holographic_pattern(crate::foundation::green_realm::HolographicPattern::new(
            0.8,
            [1.0, 0.0, 0.0],
            0.5,
        ));
        field.add_holographic_pattern(crate::foundation::green_realm::HolographicPattern::new(
            0.7,
            [0.0, 1.0, 0.0],
            0.6,
        ));

        field.add_rhythm(crate::foundation::green_realm::Rhythm::new(0.5, 0.8, 0.3));
        field.add_rhythm(crate::foundation::green_realm::Rhythm::new(0.6, 0.7, 0.4));

        field.add_field(crate::foundation::green_realm::Field::new(
            0.9,
            0.8,
            "coherence_field",
        ));

        // Add universal archetypical patterns to the Logos
        // From COSMOLOGICAL-ARCHITECTURE.md: "Blue-Ray has Universal Archetypical Patterns"
        field
            .logos
            .universal_patterns
            .add_pattern("cosmic_creation_pattern");
        field
            .logos
            .universal_patterns
            .add_pattern("evolutionary_template");
        field
            .logos
            .universal_patterns
            .add_pattern("consciousness_structure");

        field
    }

    #[test]
    fn test_spectrum_layers_transcend_include() {
        // Test that spectrum layers correctly implement "transcend and include"

        // Create Yellow-Ray (includes Violet + Indigo + Blue + Green)
        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        // Yellow-Ray includes all previous layers
        assert!(
            yellow
                .light_love_field
                .logos
                .intelligent_infinity
                .violet_realm
                .unity
                == 1.0
        );
        assert!(yellow.light_love_field.logos.intelligent_infinity.awareness > 0.0);
        assert!(yellow.light_love_field.logos.focusing_strength > 0.0);
        assert!(yellow.light_love_field.potential_strength > 0.0);

        // Yellow-Ray transcends by adding dimensional architecture
        assert!(yellow.dimensional_architecture.emerged);
        assert!(yellow.dimensional_architecture.has_dimensions());

        // Yellow-Ray creates attractor-field for Orange-Ray
        assert_eq!(
            yellow.attractor_field.name,
            "Galactic-scale Spectrum Configuration"
        );

        // Create Orange-Ray (includes Yellow + all previous layers)
        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(1); // Ensure solar systems exist for Red transition

        // Orange-Ray includes Yellow-Ray
        assert!(orange.yellow_realm.dimensional_architecture.emerged);

        // Orange-Ray transcends by adding galactic-scale configuration
        assert!(!orange.galactic_logoi.is_empty());

        // Orange-Ray creates attractor-field for Red-Ray
        assert_eq!(
            orange.attractor_field.name,
            "Solar-scale Spectrum Configuration + Archetypical Mind"
        );

        // Create Red-Ray (includes Orange + all previous layers)
        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Red-Ray includes Orange-Ray
        assert!(red.orange_realm.galactic_configuration_complete());

        // Red-Ray transcends by adding solar-scale configuration and archetypical mind
        assert!(!red.solar_logoi.is_empty());
        assert!(red
            .solar_logoi
            .iter()
            .all(|s| s.archetypical_mind.archetype_count() > 0));

        // Red-Ray creates attractor-field for Layer 7
        assert_eq!(
            red.attractor_field.name,
            "Individual Entity Inheritance with Holographic Blueprint"
        );
    }

    #[test]
    fn test_larson_framework_spectrum_continuum() {
        // Test that the Larson framework correctly implements the spectrum continuum

        let mut framework = LarsonFramework::new();

        // Add motion units on both sides of the spectrum
        let ratio1 = SpectrumRatio::space_time(3.0, 1.0);
        let ratio2 = SpectrumRatio::time_space(1.0, 3.0);

        framework.add_motion_unit(ScalarMotionUnit::new(ratio1, 2.0));
        framework.add_motion_unit(ScalarMotionUnit::new(ratio2, 2.0));

        let continuum = framework.spectrum_continuum();

        assert_eq!(continuum.len(), 2);
        assert_eq!(continuum[0].quality(), SpectrumQuality::ManyNess);
        assert_eq!(continuum[1].quality(), SpectrumQuality::Oneness);
    }

    #[test]
    fn test_veil_as_structural_feature() {
        // Test that the Veil is a structural feature, not a separator

        let veil = Veil::new(0.5);

        // The Veil is at v = 1
        let ratio = SpectrumRatio::space_time(1.0, 1.0);
        assert!(veil.is_at_transition(&ratio));

        // The Veil limits access, not existence
        let mut access = SpectrumAccess::full();
        veil.limit_access(&mut access);

        assert!(access.oneness_access < 1.0);
        assert_eq!(access.many_ness_access, 1.0);

        // Both sides of the spectrum still exist
        let ratio_oneness = SpectrumRatio::time_space(1.0, 3.0);
        let ratio_mannyness = SpectrumRatio::space_time(3.0, 1.0);

        assert_eq!(ratio_oneness.quality(), SpectrumQuality::Oneness);
        assert_eq!(ratio_mannyness.quality(), SpectrumQuality::ManyNess);
    }

    #[test]
    fn test_dimensional_emergence_algorithm() {
        // Test the dimensional emergence algorithm

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);

        // Step 1: Holographic Field contains complete information
        assert!(!yellow.light_love_field.holographic_patterns.is_empty());

        // Step 2: Scalar Motion Units organize by reciprocal ratios
        yellow.apply_mysterious_emergence().unwrap();
        assert!(!yellow
            .dimensional_architecture
            .larson_framework
            .motion_units
            .is_empty());

        // Step 3: Resonant Standing Waves emerge
        assert!(!yellow
            .dimensional_architecture
            .larson_framework
            .standing_waves
            .is_empty());

        // Step 4: Dimensional Structures manifest
        assert!(!yellow.dimensional_architecture.dimensions.is_empty());

        // Step 5: The Veil forms at v = 1
        assert!(yellow
            .dimensional_architecture
            .veil
            .is_at_transition(&SpectrumRatio::space_time(1.0, 1.0)));
    }

    #[test]
    fn test_galactic_scale_spectrum_configuration() {
        // Test galactic-scale spectrum configuration

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();

        // Each galaxy has a unique spectrum configuration
        assert!(!orange.galactic_logoi.is_empty());

        for galaxy in &orange.galactic_logoi {
            assert!(galaxy.spectrum_configuration.is_stable());
            assert!(galaxy.galaxy_spiraling_energy().is_some());
        }

        // Spectrum patterns exist BEFORE physical matter
        assert!(orange.galactic_configuration_complete());
    }

    #[test]
    fn test_solar_scale_spectrum_configuration() {
        // Test solar-scale spectrum configuration

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);

        // Each solar system has a unique spectrum configuration
        assert!(!red.solar_logoi.is_empty());

        for solar in &red.solar_logoi {
            assert!(solar.spectrum_configuration.is_stable());
            assert!(!solar.planets.is_empty());
        }

        // Spectrum patterns exist BEFORE physical matter
        assert!(red.solar_configuration_complete());
    }

    #[test]
    fn test_archetypical_mind_at_red_ray() {
        // Test that archetypical mind is at Red-Ray, not Blue-Ray

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        // Yellow-Ray does NOT have archetypical mind
        // (Archetypical mind is a Solar-Logos feature at Red-Ray)

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        // Orange-Ray does NOT have archetypical mind
        // (Archetypical mind is a Solar-Logos feature at Red-Ray)

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();

        // Red-Ray DOES have archetypical mind
        for solar in &red.solar_logoi {
            assert!(solar.archetypical_mind.archetype_count() > 0);
        }
    }

    #[test]
    fn test_twenty_two_archetype_system() {
        // Test the 22-archetype system

        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        assert_eq!(mind.archetype_count(), 22);
        assert!(mind.is_twenty_two_system());

        // Check the three complexes
        let mind_archetypes = mind.get_complex_archetypes(ComplexType::Mind);
        assert_eq!(mind_archetypes.len(), 7);

        let body_archetypes = mind.get_complex_archetypes(ComplexType::Body);
        assert_eq!(body_archetypes.len(), 7);

        let spirit_archetypes = mind.get_complex_archetypes(ComplexType::Spirit);
        assert_eq!(spirit_archetypes.len(), 7);

        // Check Archetype 22 (The Choice)
        let choice = mind.get_choice();
        assert!(choice.is_some());
        assert_eq!(choice.unwrap().role, ArchetypeRole::Choice);
    }

    #[test]
    fn test_archetypical_mind_as_training_aid() {
        // Test that archetypical mind is a training aid for veiled experience

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Get training aid from one of the solar systems
        if let Some(solar) = red.solar_logoi.first() {
            let training_aid = solar.get_training_aid();

            // Training aid provides guidance
            let guidance = training_aid.provide_guidance(1);
            assert!(guidance.is_some());

            // Training aid assists with polarity choice
            let polarity_guidance = training_aid.assist_polarity_choice();

            // Should be ChoiceAvailable or ChoiceNotReady (not NoChoiceArchetype)
            assert!(!matches!(
                polarity_guidance,
                PolarityGuidance::NoChoiceArchetype
            ));
        }
    }

    #[test]
    fn test_three_tier_archetypical_mind_refinement() {
        // Test the three-tier archetypical mind refinement

        // Tier 1: Cosmic Mind (at Blue-Ray)
        // Tier 2: Primary Logos Refinement (at Blue-Ray)
        // Tier 3: Sub-Logos Refinement (at Red-Ray)

        let field = create_test_light_love_field();

        // Blue-Ray has Universal Archetypical Patterns (structure UNKNOWN)
        assert!(!field.logos.universal_patterns.patterns.is_empty());

        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();

        // Red-Ray has Solar-Logoi refinement into specific archetypical mind systems
        for solar in &red.solar_logoi {
            // Solar-Logoi have refined the universal patterns into specific systems
            assert!(solar.archetypical_mind.archetype_count() > 0);
            assert!(solar.archetypical_mind.solar_logos_id == solar.id);
        }
    }

    #[test]
    fn test_consciousness_first_cosmology() {
        // Test consciousness-first cosmology

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Spectrum patterns exist BEFORE physical matter
        assert!(red.orange_realm.galactic_configuration_complete());
        assert!(red.solar_configuration_complete());

        // Archetypical mind systems exist BEFORE physical entities
        assert!(red
            .solar_logoi
            .iter()
            .all(|s| s.archetypical_mind.archetype_count() > 0));

        // No physical entities exist yet (they will incarnate later)
        // This is the consciousness-first cosmology
    }

    #[test]
    fn test_spectrum_access_evolution() {
        // Test evolution as spectrum access

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        // Initially, the Veil is fully active (3rd Density)
        assert_eq!(yellow.veil_strength(), 1.0);

        // As evolution progresses, the Veil thins
        yellow.dimensional_architecture.thin_veil(0.3);

        // Access to Oneness side increases
        assert_eq!(yellow.veil_strength(), 0.7);

        // Further evolution
        yellow.dimensional_architecture.thin_veil(0.3);

        assert!((yellow.veil_strength() - 0.4).abs() < 0.01);

        // Eventually, the Veil dissolves (6th Density)
        yellow.dimensional_architecture.thin_veil(0.4);

        assert!((yellow.veil_strength() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_logos_hierarchy() {
        // Test the Logos Hierarchy

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        // Primary Logos: Configures the universal spectrum framework (Yellow-Ray)
        assert!(yellow.dimensional_architecture.has_dimensions());

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(1); // Ensure solar systems exist

        // Galactic-scale Logoi: Configure galactic-scale spectrum patterns (Orange-Ray)
        assert!(!orange.galactic_logoi.is_empty());

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();

        // Solar-scale Logoi: Configure solar-system scale spectrum patterns and develop archetypical mind systems (Red-Ray)
        assert!(!red.solar_logoi.is_empty());
        assert!(red
            .solar_logoi
            .iter()
            .all(|s| s.archetypical_mind.archetype_count() > 0));
    }

    #[test]
    fn test_spectrum_layer_transitions() {
        // Test transitions between spectrum layers

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);

        // Yellow-Ray emergence
        yellow.apply_mysterious_emergence().unwrap();
        assert!(yellow.emergence_complete());

        // Yellow-Ray to Orange-Ray transition
        assert!(yellow.ready_for_orange_transition());
        let orange_attractor = yellow.transition_to_orange().unwrap();
        assert_eq!(
            orange_attractor.name,
            "Galactic-scale Spectrum Configuration"
        );

        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();

        // Orange-Ray to Red-Ray transition
        orange.create_solar_systems(2);
        assert!(orange.ready_for_red_transition());
        let red_attractor = orange.transition_to_red().unwrap();
        assert_eq!(
            red_attractor.name,
            "Solar-scale Spectrum Configuration + Archetypical Mind"
        );

        let mut red = RedRealm::new(orange);
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Red-Ray to Layer 7 transition
        assert!(red.ready_for_layer7_transition());
        let layer7_attractor = red.transition_to_layer7().unwrap();
        assert_eq!(
            layer7_attractor.name,
            "Individual Entity Inheritance with Holographic Blueprint"
        );
    }

    #[test]
    fn test_spectrum_implementation_complete() {
        // Test that spectrum implementation is complete

        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();

        let mut orange = OrangeRealm::new(yellow.clone());
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        let mut red = RedRealm::new(orange.clone());
        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // All spectrum layers are implemented
        assert!(yellow.emergence_complete());
        assert!(orange.galactic_configuration_complete());
        assert!(red.solar_configuration_complete());

        // All transitions are ready
        assert!(yellow.ready_for_orange_transition());
        assert!(orange.ready_for_red_transition());
        assert!(red.ready_for_layer7_transition());
    }
}
