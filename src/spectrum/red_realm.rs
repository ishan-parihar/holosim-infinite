//! # Red-Ray Realm (Solar-scale Spectrum Configuration + Archetypical Mind System)
//!
//! This module implements Layer 6 of the holographic architecture: the Red-Ray Realm.
//!
//! ## Theoretical Foundation
//!
//! The Red-Ray Realm is where **Solar-scale Logoi/Sub-Logoi** create solar systems and develop archetypical mind systems.
//!
//! ## Characteristics
//!
//! - "Individualized portions of the galactic-scale Logoi that create solar systems and planetary systems"
//! - "Develop specific archetypical mind systems (10 or 22 archetypes)"
//! - "The archetypical mind is a training aid for entities in veiled experience"
//! - "Different Solar-Logoi make different design choices based on their priorities"
//!
//! ## The Three-Tier Archetypical Mind Refinement
//!
//! 1. **Cosmic Mind**: Universal field of potential, the same for all sub-Logoi
//! 2. **Primary Logos Refinement**: Universal archetypical patterns created at Blue-Ray (structure unknown)
//! 3. **Sub-Logos Refinement**: Solar-scale Logoi further refine these patterns into specific archetypical mind systems
//!
//! ## Different Solar-Logoi, Different Choices
//!
//! - Some Solar-Logoi use **10-archetype systems** (two systems of five)
//! - Other Solar-Logoi use **22-archetype systems** (7+7+7+1)
//! - Our Solar-Logos chose the **22-archetype system** (maximum articulation) to support intense veiling and polarization acceleration
//!
//! ## Includes, Transcends, Evolves Into
//!
//! - **INCLUDES**: Violet-Ray + Indigo-Ray + Blue-Ray + Green-Ray + Yellow-Ray + Orange-Ray (all previous layers)
//! - **TRANSCENDS**: Adds Solar-scale spectrum configuration + archetypical mind system
//! - **EVOLVES INTO**: Attractor-field for Layer 7 (Individual entity inheritance)

use crate::foundation::transcend_include::AttractorField;
use crate::spectrum::archetypical_mind::{
    ArchetypicalMind, ArchetypicalSystemType, PolarityGuidance, TrainingAid,
};
use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
use crate::spectrum::orange_realm::{LocalizedSpectrumConfiguration, OrangeRealm, SpectrumScale};

/// Represents a Solar-scale Logos
#[derive(Debug, Clone)]
pub struct SolarLogos {
    /// The unique identifier
    pub id: String,
    /// The name of this solar Logos
    pub name: String,
    /// The spectrum configuration for this solar system
    pub spectrum_configuration: LocalizedSpectrumConfiguration,
    /// The archetypical mind system
    pub archetypical_mind: ArchetypicalMind,
    /// The planets within this solar system
    pub planets: Vec<LocalizedSpectrumConfiguration>,
    /// Whether this solar system is ready for entities
    pub ready_for_entities: bool,
}

impl SolarLogos {
    /// Creates a new Solar-scale Logos
    pub fn new(
        id: String,
        name: String,
        spectrum_configuration: LocalizedSpectrumConfiguration,
        archetypical_system_type: ArchetypicalSystemType,
    ) -> Self {
        let archetypical_mind = ArchetypicalMind::new(archetypical_system_type, id.clone());

        SolarLogos {
            id,
            name,
            spectrum_configuration,
            archetypical_mind,
            planets: Vec::new(),
            ready_for_entities: false,
        }
    }

    /// Creates our Solar-Logos with the 22-archetype system
    pub fn our_solar_logos(spectrum_configuration: LocalizedSpectrumConfiguration) -> Self {
        SolarLogos::new(
            "our_solar_logos".to_string(),
            "Our Sun".to_string(),
            spectrum_configuration,
            ArchetypicalSystemType::TwentyTwoArchetype, // Maximum articulation
        )
    }

    /// Adds a planet to this solar system
    pub fn add_planet(&mut self, planet_config: LocalizedSpectrumConfiguration) {
        self.planets.push(planet_config);
    }

    /// Creates a planet configuration
    pub fn create_planet(&mut self, id: String, spectrum_ratio: SpectrumRatio) {
        let planet_config =
            self.spectrum_configuration
                .child(id, SpectrumScale::Planetary, spectrum_ratio, 0.6);
        self.add_planet(planet_config);
    }

    /// Gets the training aid for this solar system
    pub fn get_training_aid(&self) -> TrainingAid {
        TrainingAid::new(self.archetypical_mind.clone())
    }

    /// Checks if this solar system is ready for entities
    pub fn check_readiness(&mut self) {
        self.ready_for_entities = self.spectrum_configuration.is_stable()
            && !self.planets.is_empty()
            && self.archetypical_mind.archetype_count() > 0;
    }

    /// Activates the archetypical mind for entities
    pub fn activate_archetypical_mind(&mut self) {
        // Activate key archetypes for veiled experience
        let _ = self.archetypical_mind.activate_archetype(1, 0.7); // Mind Matrix
        let _ = self.archetypical_mind.activate_archetype(8, 0.7); // Body Matrix
        let _ = self.archetypical_mind.activate_archetype(15, 0.7); // Spirit Matrix
        let _ = self.archetypical_mind.activate_archetype(22, 0.5); // The Choice (initially low activation)
    }
}

/// Represents the Red-Ray Realm
#[derive(Debug, Clone)]
pub struct RedRealm {
    /// The Orange-Ray Realm (INCLUDED)
    pub orange_realm: OrangeRealm,
    /// The Solar-scale Logoi
    pub solar_logoi: Vec<SolarLogos>,
    /// The attractor-field for Layer 7
    pub attractor_field: AttractorField,
}

impl RedRealm {
    /// Creates a new Red-Ray Realm from the Orange-Ray Realm
    pub fn new(orange_realm: OrangeRealm) -> Self {
        // The attractor-field pulls toward Layer 7 (Individual entity inheritance)
        let attractor_field = AttractorField::new(
            "Individual Entity Inheritance with Holographic Blueprint".to_string(),
            0.9,
            "Individual entity inheritance with holographic blueprint encoding".to_string(),
        );

        RedRealm {
            orange_realm,
            solar_logoi: Vec::new(),
            attractor_field,
        }
    }

    /// Applies solar-scale spectrum configuration and archetypical mind development
    pub fn apply_solar_configuration(&mut self) -> Result<(), String> {
        // Check if Orange-Ray is ready
        if !self.orange_realm.ready_for_red_transition() {
            return Err("Orange-Ray not ready for Red-Ray transition".to_string());
        }

        // Get the solar system configurations from Orange-Ray
        for galaxy in &self.orange_realm.galactic_logoi {
            for solar_config in &galaxy.solar_systems {
                // Determine archetypical system type based on Solar-Logos choice
                // Our Solar-Logos chose the 22-archetype system
                let system_type = if solar_config.id.contains("our_solar") {
                    ArchetypicalSystemType::TwentyTwoArchetype
                } else {
                    // Other Solar-Logoi might choose differently
                    // For this implementation, we'll use 22-archetype for all
                    ArchetypicalSystemType::TwentyTwoArchetype
                };

                let solar_logos = SolarLogos::new(
                    solar_config.id.clone(),
                    format!("Solar System {}", solar_config.id),
                    solar_config.clone(),
                    system_type,
                );

                self.solar_logoi.push(solar_logos);
            }
        }

        Ok(())
    }

    /// Creates planets within solar systems
    pub fn create_planets(&mut self, planets_per_solar_system: usize) {
        for solar_logos in &mut self.solar_logoi {
            for i in 0..planets_per_solar_system {
                let planet_id = format!("{}_planet_{}", solar_logos.id, i);

                // Create a unique spectrum ratio for each planet
                let base_ratio = &solar_logos.spectrum_configuration.spectrum_ratio;
                let planet_ratio = if base_ratio.side == SpectrumSide::SpaceTime {
                    SpectrumRatio::space_time(
                        base_ratio.space_component * (1.0 + (i as f64 * 0.03)),
                        base_ratio.time_component,
                    )
                } else {
                    SpectrumRatio::time_space(
                        base_ratio.space_component,
                        base_ratio.time_component * (1.0 + (i as f64 * 0.03)),
                    )
                };

                solar_logos.create_planet(planet_id, planet_ratio);
            }

            // Check readiness
            solar_logos.check_readiness();
        }
    }

    /// Activates archetypical minds for entities
    pub fn activate_archetypical_minds(&mut self) {
        for solar_logos in &mut self.solar_logoi {
            solar_logos.activate_archetypical_mind();
        }
    }

    /// Checks if solar configuration is complete
    pub fn solar_configuration_complete(&self) -> bool {
        !self.solar_logoi.is_empty() && self.solar_logoi.iter().all(|s| s.ready_for_entities)
    }

    /// Checks if ready for Layer 7 transition
    pub fn ready_for_layer7_transition(&self) -> bool {
        self.solar_configuration_complete()
            && self
                .solar_logoi
                .iter()
                .any(|s| s.archetypical_mind.get_choice().is_some())
    }

    /// Transitions to Layer 7 (Individual entity inheritance)
    pub fn transition_to_layer7(&self) -> Result<AttractorField, String> {
        if !self.ready_for_layer7_transition() {
            return Err("Red-Ray not ready for Layer 7 transition".to_string());
        }

        Ok(self.attractor_field.clone())
    }

    /// Gets the total number of solar systems
    pub fn solar_system_count(&self) -> usize {
        self.solar_logoi.len()
    }

    /// Gets the total number of planets
    pub fn planet_count(&self) -> usize {
        self.solar_logoi.iter().map(|s| s.planets.len()).sum()
    }

    /// Gets our Solar-Logos
    pub fn our_solar_logos(&self) -> Option<&SolarLogos> {
        self.solar_logoi.iter().find(|s| s.id.contains("our_solar"))
    }

    /// Gets the attractor-field strength
    pub fn attractor_field_strength(&self) -> f64 {
        self.attractor_field.strength
    }

    /// Gets polarity guidance from our Solar-Logos
    pub fn get_polarity_guidance(&self) -> Option<PolarityGuidance> {
        if let Some(our_solar) = self.our_solar_logos() {
            let training_aid = our_solar.get_training_aid();
            Some(training_aid.assist_polarity_choice())
        } else {
            None
        }
    }

    /// Gets the archetypical mind (for use by Layer 7 entities)
    pub fn get_archetypical_mind(&self) -> &ArchetypicalMind {
        if let Some(our_solar) = self.our_solar_logos() {
            &our_solar.archetypical_mind
        } else if let Some(first_solar) = self.solar_logoi.first() {
            &first_solar.archetypical_mind
        } else {
            // Return a default archetypical mind if no solar systems exist
            static DEFAULT_MIND: std::sync::OnceLock<ArchetypicalMind> = std::sync::OnceLock::new();
            DEFAULT_MIND.get_or_init(|| {
                ArchetypicalMind::new(
                    ArchetypicalSystemType::TwentyTwoArchetype,
                    "default".to_string(),
                )
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::larson_framework::SpectrumSide;
    use crate::spectrum::YellowRealm;

    /// Create a test Light/Love Field for testing
    fn create_test_light_love_field() -> crate::foundation::green_realm::LightLoveField {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = crate::foundation::blue_realm::Logos::from_intelligent_infinity(intelligent);
        crate::foundation::green_realm::LightLoveField::from_logos(logos)
    }

    /// Create a test Yellow Realm for testing
    fn create_test_yellow_realm() -> YellowRealm {
        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();
        yellow
    }

    fn create_test_orange_realm() -> OrangeRealm {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);
        orange
    }

    #[test]
    fn test_solar_logos_new() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        assert_eq!(logos.id, "solar1");
        assert_eq!(logos.name, "Solar System 1");
        assert!(logos.archetypical_mind.is_twenty_two_system());
        assert!(!logos.ready_for_entities);
    }

    #[test]
    fn test_solar_logos_our_solar_logos() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "our_solar_logos".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let logos = SolarLogos::our_solar_logos(config);

        assert_eq!(logos.id, "our_solar_logos");
        assert_eq!(logos.name, "Our Sun");
        assert!(logos.archetypical_mind.is_twenty_two_system());
    }

    #[test]
    fn test_solar_logos_add_planet() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let mut logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        let planet_ratio = SpectrumRatio::space_time(2.0, 1.0);
        let planet_config = LocalizedSpectrumConfiguration::new(
            "planet1".to_string(),
            SpectrumScale::Planetary,
            planet_ratio,
            0.6,
        );

        logos.add_planet(planet_config);

        assert_eq!(logos.planets.len(), 1);
    }

    #[test]
    fn test_solar_logos_create_planet() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let mut logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        logos.create_planet("planet1".to_string(), SpectrumRatio::space_time(2.0, 1.0));

        assert_eq!(logos.planets.len(), 1);
        assert_eq!(logos.planets[0].parent_id, Some("solar1".to_string()));
    }

    #[test]
    fn test_solar_logos_get_training_aid() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        let training_aid = logos.get_training_aid();

        assert_eq!(training_aid.effectiveness, 0.8);
    }

    #[test]
    fn test_solar_logos_check_readiness() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let mut logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        logos.check_readiness();
        assert!(!logos.ready_for_entities);

        logos.create_planet("planet1".to_string(), SpectrumRatio::space_time(2.0, 1.0));
        logos.check_readiness();
        assert!(logos.ready_for_entities);
    }

    #[test]
    fn test_solar_logos_activate_archetypical_mind() {
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let mut logos = SolarLogos::new(
            "solar1".to_string(),
            "Solar System 1".to_string(),
            config,
            ArchetypicalSystemType::TwentyTwoArchetype,
        );

        logos.activate_archetypical_mind();

        assert_eq!(
            logos.archetypical_mind.get_archetype(1).unwrap().activation,
            0.7
        );
        assert_eq!(
            logos
                .archetypical_mind
                .get_archetype(22)
                .unwrap()
                .activation,
            0.5
        );
    }

    #[test]
    fn test_red_realm_new() {
        let orange = create_test_orange_realm();
        let red = RedRealm::new(orange);

        assert_eq!(
            red.attractor_field.name,
            "Individual Entity Inheritance with Holographic Blueprint"
        );
        assert!(red.solar_logoi.is_empty());
    }

    #[test]
    fn test_red_realm_apply_solar_configuration() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        let result = red.apply_solar_configuration();

        assert!(result.is_ok());
        assert!(!red.solar_logoi.is_empty());
    }

    #[test]
    fn test_red_realm_apply_solar_configuration_not_ready() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);
        orange.apply_galactic_configuration().unwrap();
        // Don't create solar systems

        let mut red = RedRealm::new(orange);

        let result = red.apply_solar_configuration();

        assert!(result.is_err());
    }

    #[test]
    fn test_red_realm_create_planets() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);

        let planet_count = red.planet_count();
        assert!(planet_count > 0);
    }

    #[test]
    fn test_red_realm_activate_archetypical_minds() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Check that archetypical minds are activated
        for solar_logos in &red.solar_logoi {
            assert_eq!(
                solar_logos
                    .archetypical_mind
                    .get_archetype(1)
                    .unwrap()
                    .activation,
                0.7
            );
        }
    }

    #[test]
    fn test_red_realm_solar_configuration_complete() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);

        assert!(red.solar_configuration_complete());
    }

    #[test]
    fn test_red_realm_ready_for_layer7_transition() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        assert!(red.ready_for_layer7_transition());
    }

    #[test]
    fn test_red_realm_transition_to_layer7() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        let result = red.transition_to_layer7();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().name,
            "Individual Entity Inheritance with Holographic Blueprint"
        );
    }

    #[test]
    fn test_red_realm_transition_to_layer7_not_ready() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        // Don't activate archetypical minds

        let result = red.transition_to_layer7();

        assert!(result.is_err());
    }

    #[test]
    fn test_red_realm_solar_system_count() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();

        assert_eq!(red.solar_system_count(), red.solar_logoi.len());
    }

    #[test]
    fn test_red_realm_planet_count() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(5);

        assert!(red.planet_count() > 0);
    }

    #[test]
    fn test_red_realm_our_solar_logos() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        // Add our solar logos
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "our_solar_logos".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        red.solar_logoi.push(SolarLogos::our_solar_logos(config));

        let our_solar = red.our_solar_logos();

        assert!(our_solar.is_some());
        assert_eq!(our_solar.unwrap().id, "our_solar_logos");
    }

    #[test]
    fn test_red_realm_attractor_field_strength() {
        let orange = create_test_orange_realm();
        let red = RedRealm::new(orange);

        assert_eq!(red.attractor_field_strength(), 0.9);
    }

    #[test]
    fn test_red_realm_get_polarity_guidance() {
        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        // Add our solar logos
        let ratio = SpectrumRatio::space_time(2.5, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "our_solar_logos".to_string(),
            SpectrumScale::Solar,
            ratio,
            0.7,
        );

        let mut solar = SolarLogos::our_solar_logos(config);
        solar.activate_archetypical_mind();

        red.solar_logoi.push(solar);

        // Apply solar configuration to properly initialize archetypical mind
        let _ = red.apply_solar_configuration();

        let guidance = red.get_polarity_guidance();

        assert!(guidance.is_some());
    }

    #[test]
    fn test_red_realm_includes_previous_layers() {
        let orange = create_test_orange_realm();
        let red = RedRealm::new(orange);

        // Verify that Red-Ray includes all previous layers
        assert!(red.orange_realm.galactic_configuration_complete());
        assert!(
            red.orange_realm
                .yellow_realm
                .dimensional_architecture
                .emerged
        );
        assert!(red.orange_realm.yellow_realm.quantum_realm.formed);
    }

    #[test]
    fn test_three_tier_archetypical_mind_refinement() {
        // Solar-Logoi refine universal patterns into specific archetypical mind systems

        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();

        // Check that each Solar-Logos has an archetypical mind system
        for solar_logos in &red.solar_logoi {
            assert!(solar_logos.archetypical_mind.archetype_count() > 0);
        }
    }

    #[test]
    fn test_archetypical_mind_as_training_aid() {
        // Archetypical minds are training aids for entities in veiled experience

        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);
        red.activate_archetypical_minds();

        // Get training aid from one of the solar systems
        if let Some(solar_logos) = red.solar_logoi.first() {
            let training_aid = solar_logos.get_training_aid();

            // Training aid provides guidance
            let guidance = training_aid.provide_guidance(1);
            assert!(guidance.is_some());

            // Training aid assists with polarity choice
            let polarity_guidance = training_aid.assist_polarity_choice();
            match polarity_guidance {
                PolarityGuidance::ChoiceAvailable { .. } => {
                    // Choice is available
                }
                _ => {
                    // Choice not ready yet (expected if Archetype 22 activation is low)
                }
            }
        }
    }

    #[test]
    fn test_consciousness_first_cosmology() {
        // Spectrum patterns exist BEFORE physical matter

        let orange = create_test_orange_realm();
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration().unwrap();
        red.create_planets(3);

        // Spectrum configurations exist (no physical entities yet)
        assert!(!red.solar_logoi.is_empty());
        assert!(red.planet_count() > 0);

        // Archetypical mind systems exist (no physical entities yet)
        for solar_logos in &red.solar_logoi {
            assert!(solar_logos.archetypical_mind.archetype_count() > 0);
        }
    }
}
