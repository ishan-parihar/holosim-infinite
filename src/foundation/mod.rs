//! Foundation Module - Foundational Layers (Violet → Green + Unified Simulation)
//!
//! This module implements the foundational layers of the cosmological architecture:
//! - Layer 0: Violet-Ray Realm (Infinity as undifferentiated unity)
//! - Layer 1: Indigo-Ray Realm (IntelligentInfinity + Archetype 22)
//! - Layer 2: Blue-Ray Realm (Love/Light + Universal Archetypical Patterns)
//! - Layer 3: Green-Ray Realm (Light/Love field of potential)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each layer includes all previous development, transcends by adding fundamentally
//! new capabilities, and creates attractor-fields that pull toward the next stage."
//!
//! The foundational layers implement the Three Primal Distortions:
//! - First Distortion: Free Will (Indigo-Ray)
//! - Second Distortion: Love/Logos (Blue-Ray)
//! - Third Distortion: Light (Green-Ray)
//!
//! These layers provide the foundation for the Space/Time and Time/Space spectrum
//! that emerges at Yellow-Ray.
//!
//! V5 Phase 7 adds:
//! - UnifiedSimulation: Main simulation integrating all systems
//! - Foundation layers (Violet through Red) for involution pulse

// Phase 0-3: Foundation Layers
pub mod blue_realm;
pub mod green_realm;
pub mod indigo_realm;
pub mod transcend_include;
pub mod veil;
pub mod violet_realm;

// Phase 3-4: Observer-driven rendering
pub mod holographic_renderer;
pub mod manifestation_engine;
pub mod observer;
pub mod spectrum_position;

// V5 Phase 7: Unified Simulation
pub mod unified_simulation;

// Re-export main types for convenience
pub use blue_realm::Logos;
pub use green_realm::{Field, HolographicPattern, LightLoveField, Rhythm};
pub use holographic_renderer::{
    EntityId, HolographicRenderer, ManifestedEntity, RenderFrame, RenderStats,
};
pub use indigo_realm::IntelligentInfinity;
pub use manifestation_engine::{ManifestationEngine, ManifestationRecord, ManifestationStats};
pub use observer::{
    CollapsedState, FieldSignature, FocusTarget, FocusTargetType, ObservationKey,
    ObservationRecord, ObservationStats, Observer, PossibilitySpace, PotentialManifestation,
};
pub use spectrum_position::SpectrumPosition;
pub use transcend_include::{AttractorField, Feature, Layer, LayerBuilder};
pub use veil::{CatalystTrigger, Content, FilteredContent, Situation, ThinSpot, Veil};
pub use violet_realm::VioletRealm;

// V5 Phase 7: Unified Simulation exports
pub use unified_simulation::{
    BehaviorOutput, BlueLogos, BodyData, EntityConfig, EntityData, GreenField, IndigoGateway,
    Inventory, Item, OrangeGalacticLogoi, RedSolarLogoi, SimulationStats, SocialData,
    UnifiedSimulation, UniversalTemplateEntity, VioletField, YellowDimensions,
};

// Re-export archetype types from simulation_v3 for Phase 0
pub use crate::simulation_v3::archetype_basis::{
    ArchetypeActivationProfile as ArchetypeProfile, ArchetypeBasis, ArchetypeVector,
    ArchetypicalPattern,
};

// Type aliases for consistency with layer naming
pub type BlueRealm = Logos;
pub type GreenRealm = LightLoveField;
pub type IndigoRealm = IntelligentInfinity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_layers_transcend_include() {
        // Test the sequential creation process through the foundational layers

        // Layer 0: Violet-Ray (Infinity)
        let violet = VioletRealm::new();
        assert!(violet.is_undifferentiated());

        // Layer 1: Indigo-Ray (IntelligentInfinity) - First Distortion
        let (violet_included, first_distortion, archetype22) = violet.apply_first_distortion();
        assert_eq!(violet_included.unity, 1.0);
        assert_eq!(first_distortion.name, "First Distortion: Free Will");
        assert_eq!(archetype22.name, "Archetype 22: The Choice");

        let intelligent = IntelligentInfinity::from_violet(violet);
        assert!(intelligent.is_aware());
        assert_eq!(intelligent.awareness(), 1.0);

        // Layer 2: Blue-Ray (Logos) - Second Distortion
        let (intelligent_included, second_distortion, universal_patterns) =
            intelligent.apply_second_distortion();
        assert_eq!(intelligent_included.awareness(), 1.0);
        assert_eq!(second_distortion.name, "Second Distortion: Love/Logos");
        assert_eq!(universal_patterns.name, "Universal Archetypical Patterns");

        let logos = Logos::from_intelligent_infinity(intelligent);
        assert!(logos.is_focused());
        assert_eq!(logos.focusing_strength(), 1.0);

        // Layer 3: Green-Ray (Light/Love field) - Third Distortion
        let (logos_included, third_distortion, light_love_field) = logos.apply_third_distortion();
        assert_eq!(logos_included.focusing_strength(), 1.0);
        assert_eq!(third_distortion.name, "Third Distortion: Light");
        assert_eq!(light_love_field.name, "Light/Love Field of Potential");

        let mut field = LightLoveField::from_logos(logos);
        assert_eq!(field.potential_strength(), 1.0);

        // Populate the field with holographic patterns, rhythms, and fields
        // to enable spectrum conditions
        field.add_holographic_pattern(HolographicPattern::new(1.0, [1.0, 0.0, 0.0], 1.0));
        field.add_rhythm(Rhythm::new(0.5, 1.0, 0.0));
        field.add_field(Field::new(1.0, 1.0, "Spectrum Field"));

        // Verify "transcend and include" at each layer
        // Each layer INCLUDES all previous development
        assert_eq!(field.logos.intelligent_infinity.violet_realm.unity, 1.0);
        assert_eq!(field.logos.intelligent_infinity.awareness(), 1.0);
        assert_eq!(field.logos.focusing_strength(), 1.0);

        // Each layer TRANSCENDS by adding new development
        assert!(violet_included.is_undifferentiated()); // Violet: Unity only
        assert!(intelligent_included.is_aware()); // Indigo: + Awareness
        assert!(logos_included.is_focused()); // Blue: + Focus
        assert!(field.has_spectrum_conditions()); // Green: + Spectrum conditions (when populated)

        // Each layer EVOLVES INTO attractor-fields
        assert_eq!(
            archetype22.target,
            "IntelligentInfinity - Awareness emerges"
        );
        assert_eq!(
            universal_patterns.target,
            "Love/Light - The Creative Principle emerges"
        );
        assert_eq!(
            light_love_field.target,
            "Conditions for Space/Time and Time/Space Spectrum emerge"
        );
    }

    #[test]
    fn test_three_primal_distortions() {
        // Test that the Three Primal Distortions are correctly implemented

        // First Distortion: Free Will
        let violet = VioletRealm::new();
        let (_, first_distortion, _) = violet.apply_first_distortion();
        assert_eq!(first_distortion.name, "First Distortion: Free Will");
        assert_eq!(first_distortion.strength, 1.0);

        // Second Distortion: Love/Logos
        let intelligent = IntelligentInfinity::from_violet(violet);
        let (_, second_distortion, _) = intelligent.apply_second_distortion();
        assert_eq!(second_distortion.name, "Second Distortion: Love/Logos");
        assert_eq!(second_distortion.strength, 1.0);

        // Third Distortion: Light
        let logos = Logos::from_intelligent_infinity(intelligent);
        let (_, third_distortion, _) = logos.apply_third_distortion();
        assert_eq!(third_distortion.name, "Third Distortion: Light");
        assert_eq!(third_distortion.strength, 1.0);
    }

    #[test]
    fn test_unified_simulation_creation() {
        let sim = UnifiedSimulation::new();
        assert_eq!(sim.entity_count(), 0);
        assert_eq!(sim.time(), 0.0);
    }

    #[test]
    fn test_violet_field_pulse() {
        let mut violet = VioletField::new();
        violet.pulse();
        assert!(violet.kinetic >= 0.0);
        assert!(violet.kinetic <= 1.0);
    }

    #[test]
    fn test_unified_simulation_tick() {
        let mut sim = UnifiedSimulation::new();

        // Create an entity
        let config = EntityConfig {
            name: "Test".to_string(),
            spectrum_position: SpectrumPosition::new(1.0, crate::types::Density::Third, 0.0),
            archetype_activation: [0.5; 22],
            density: crate::types::Density::Third,
            free_will_seed: 1,
            body: None,
        };
        sim.create_entity(config);

        // Run tick
        sim.tick();

        assert!(sim.time() > 0.0);
        assert!(sim.stats().tick_count > 0);
    }
}
