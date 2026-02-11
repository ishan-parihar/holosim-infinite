//! # Orange-Ray Realm (Galactic-scale Spectrum Configuration)
//!
//! This module implements Layer 5 of the holographic architecture: the Orange-Ray Realm.
//!
//! ## Theoretical Foundation
//!
//! The Orange-Ray Realm is where **Galactic-scale Logoi** configure the spectrum at galactic scales.
//!
//! ## Characteristics
//!
//! - "Individualized portions of the Primary Logos that create galaxies"
//! - Creates the PATTERNS that galaxies will follow, not the galaxies themselves
//! - "Each galaxy has unique local coordinate system of illusory natural laws"
//! - "Creates energy patterns that focus in multitudinous conscious awareness"
//! - "Progression from galaxy spiraling energy to solar spiraling energy to planetary spiraling energy"
//!
//! ## The Refactor Process
//!
//! 1. The unified spectrum (all scales mixed) from Yellow-Ray
//! 2. Galactic-scale Logoi separate the spectrum into distinct galactic-scale configurations
//! 3. Each galaxy gets a unique spectrum ratio
//! 4. Each solar system within a galaxy gets a unique local spectrum ratio
//! 5. Each planet within a solar system gets a unique local spectrum ratio
//! 6. Energy patterns manifest as "galaxy spiraling energy," "solar spiraling energy," "planetary spiraling energy"
//!
//! ## Consciousness-First Cosmology
//!
//! - Spectrum patterns (information) exist BEFORE physical matter
//! - The spectrum is configured at galactic scales before atoms exist
//! - Physical galaxies form later, following these pre-existing spectrum patterns
//! - The physical universe is the manifestation of pre-existing spectrum configuration
//!
//! ## Includes, Transcends, Evolves Into
//!
//! - **INCLUDES**: Violet-Ray + Indigo-Ray + Blue-Ray + Green-Ray + Yellow-Ray (all previous layers)
//! - **TRANSCENDS**: Adds Galactic-scale spectrum configuration
//! - **EVOLVES INTO**: Attractor-field for Red-Ray (Solar-scale configuration + archetypical mind)

use crate::foundation::transcend_include::AttractorField;
use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
use crate::spectrum::yellow_realm::YellowRealm;

/// Represents the scale of spectrum configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumScale {
    /// Galactic scale
    Galactic,
    /// Solar scale
    Solar,
    /// Planetary scale
    Planetary,
}

/// Represents a localized spectrum configuration
#[derive(Debug, Clone)]
pub struct LocalizedSpectrumConfiguration {
    /// The unique identifier for this configuration
    pub id: String,
    /// The scale of this configuration
    pub scale: SpectrumScale,
    /// The spectrum ratio for this configuration
    pub spectrum_ratio: SpectrumRatio,
    /// The energy pattern strength
    pub energy_strength: f64,
    /// The parent configuration (if any)
    pub parent_id: Option<String>,
}

impl LocalizedSpectrumConfiguration {
    /// Creates a new localized spectrum configuration
    pub fn new(
        id: String,
        scale: SpectrumScale,
        spectrum_ratio: SpectrumRatio,
        energy_strength: f64,
    ) -> Self {
        LocalizedSpectrumConfiguration {
            id,
            scale,
            spectrum_ratio,
            energy_strength: energy_strength.clamp(0.0, 1.0),
            parent_id: None,
        }
    }

    /// Creates a child configuration
    pub fn child(
        &self,
        id: String,
        scale: SpectrumScale,
        spectrum_ratio: SpectrumRatio,
        energy_strength: f64,
    ) -> Self {
        LocalizedSpectrumConfiguration {
            id,
            scale,
            spectrum_ratio,
            energy_strength: energy_strength.clamp(0.0, 1.0),
            parent_id: Some(self.id.clone()),
        }
    }

    /// Gets the spectrum quality
    pub fn quality(&self) -> crate::spectrum::larson_framework::SpectrumQuality {
        self.spectrum_ratio.quality()
    }

    /// Checks if this configuration is stable
    pub fn is_stable(&self) -> bool {
        self.energy_strength > 0.5
    }
}

/// Represents energy patterns at different scales
#[derive(Debug, Clone)]
pub struct EnergyPattern {
    /// The scale of this energy pattern
    pub scale: SpectrumScale,
    /// The spiraling energy strength
    pub spiraling_strength: f64,
    /// The conscious awareness focus
    pub conscious_focus: f64,
    /// The pattern geometry
    pub geometry: String,
}

impl EnergyPattern {
    /// Creates a new energy pattern
    pub fn new(scale: SpectrumScale, spiraling_strength: f64, conscious_focus: f64) -> Self {
        let geometry = match scale {
            SpectrumScale::Galactic => "spiral",
            SpectrumScale::Solar => "helical",
            SpectrumScale::Planetary => "orbital",
        }
        .to_string();

        EnergyPattern {
            scale,
            spiraling_strength: spiraling_strength.clamp(0.0, 1.0),
            conscious_focus: conscious_focus.clamp(0.0, 1.0),
            geometry,
        }
    }

    /// Checks if this energy pattern is active
    pub fn is_active(&self) -> bool {
        self.spiraling_strength > 0.3 && self.conscious_focus > 0.3
    }

    /// Gets the pattern description
    pub fn description(&self) -> String {
        match self.scale {
            SpectrumScale::Galactic => "galaxy spiraling energy".to_string(),
            SpectrumScale::Solar => "solar spiraling energy".to_string(),
            SpectrumScale::Planetary => "planetary spiraling energy".to_string(),
        }
    }
}

/// Represents a Galactic-scale Logos
#[derive(Debug, Clone)]
pub struct GalacticLogos {
    /// The unique identifier
    pub id: String,
    /// The name of this galactic Logos
    pub name: String,
    /// The spectrum configuration for this galaxy
    pub spectrum_configuration: LocalizedSpectrumConfiguration,
    /// The energy patterns
    pub energy_patterns: Vec<EnergyPattern>,
    /// The solar systems within this galaxy
    pub solar_systems: Vec<LocalizedSpectrumConfiguration>,
}

impl GalacticLogos {
    /// Creates a new Galactic-scale Logos
    pub fn new(id: String, name: String, spectrum_ratio: SpectrumRatio) -> Self {
        let spectrum_configuration = LocalizedSpectrumConfiguration::new(
            id.clone(),
            SpectrumScale::Galactic,
            spectrum_ratio,
            0.8,
        );

        // Create galaxy spiraling energy pattern
        let energy_pattern = EnergyPattern::new(SpectrumScale::Galactic, 0.8, 0.7);

        GalacticLogos {
            id,
            name,
            spectrum_configuration,
            energy_patterns: vec![energy_pattern],
            solar_systems: Vec::new(),
        }
    }

    /// Adds a solar system to this galaxy
    pub fn add_solar_system(&mut self, solar_config: LocalizedSpectrumConfiguration) {
        self.solar_systems.push(solar_config);
    }

    /// Creates a solar system configuration
    pub fn create_solar_system(&mut self, id: String, spectrum_ratio: SpectrumRatio) {
        let solar_config =
            self.spectrum_configuration
                .child(id, SpectrumScale::Solar, spectrum_ratio, 0.7);
        self.add_solar_system(solar_config);
    }

    /// Gets the galaxy spiraling energy
    pub fn galaxy_spiraling_energy(&self) -> Option<&EnergyPattern> {
        self.energy_patterns
            .iter()
            .find(|p| p.scale == SpectrumScale::Galactic)
    }

    /// Checks if this galaxy is ready for solar system creation
    pub fn ready_for_solar_systems(&self) -> bool {
        self.spectrum_configuration.is_stable()
            && self
                .galaxy_spiraling_energy()
                .map_or(false, |e| e.is_active())
    }
}

/// Represents the Orange-Ray Realm
#[derive(Debug, Clone)]
pub struct OrangeRealm {
    /// The Yellow-Ray Realm (INCLUDED)
    pub yellow_realm: YellowRealm,
    /// The Galactic-scale Logoi
    pub galactic_logoi: Vec<GalacticLogos>,
    /// The attractor-field for Red-Ray
    pub attractor_field: AttractorField,
}

impl OrangeRealm {
    /// Creates a new Orange-Ray Realm from the Yellow-Ray Realm
    pub fn new(yellow_realm: YellowRealm) -> Self {
        // The attractor-field pulls toward Red-Ray (Solar-scale configuration + archetypical mind)
        let attractor_field = AttractorField::new(
            "Solar-scale Spectrum Configuration + Archetypical Mind".to_string(),
            0.85,
            "Solar-scale spectrum configuration and archetypical mind development".to_string(),
        );

        OrangeRealm {
            yellow_realm,
            galactic_logoi: Vec::new(),
            attractor_field,
        }
    }

    /// Applies galactic-scale spectrum configuration
    pub fn apply_galactic_configuration(&mut self) -> Result<(), String> {
        // Check if Yellow-Ray is ready
        if !self.yellow_realm.ready_for_orange_transition() {
            return Err("Yellow-Ray not ready for Orange-Ray transition".to_string());
        }

        // Get the spectrum continuum from Yellow-Ray
        let continuum = self.yellow_realm.spectrum_continuum();

        if continuum.is_empty() {
            return Err("No spectrum continuum available".to_string());
        }

        // Create Galactic-scale Logoi based on the spectrum continuum
        // Each galaxy gets a unique spectrum ratio
        for (i, ratio) in continuum.iter().enumerate() {
            let galaxy_id = format!("galaxy_{}", i);
            let galaxy_name = format!("Galaxy {}", i + 1);

            // Create a slightly modified ratio for uniqueness
            let modified_ratio = if ratio.side == SpectrumSide::SpaceTime {
                SpectrumRatio::space_time(
                    ratio.space_component * (1.0 + (i as f64 * 0.01)),
                    ratio.time_component,
                )
            } else {
                SpectrumRatio::time_space(
                    ratio.space_component,
                    ratio.time_component * (1.0 + (i as f64 * 0.01)),
                )
            };

            let galactic_logos = GalacticLogos::new(galaxy_id, galaxy_name, modified_ratio);
            self.galactic_logoi.push(galactic_logos);
        }

        Ok(())
    }

    /// Creates solar systems within galaxies
    pub fn create_solar_systems(&mut self, solar_systems_per_galaxy: usize) {
        for galaxy in &mut self.galactic_logoi {
            if !galaxy.ready_for_solar_systems() {
                continue;
            }

            for i in 0..solar_systems_per_galaxy {
                let solar_id = format!("{}_solar_{}", galaxy.id, i);

                // Create a unique spectrum ratio for each solar system
                let base_ratio = &galaxy.spectrum_configuration.spectrum_ratio;
                let solar_ratio = if base_ratio.side == SpectrumSide::SpaceTime {
                    SpectrumRatio::space_time(
                        base_ratio.space_component * (1.0 + (i as f64 * 0.02)),
                        base_ratio.time_component,
                    )
                } else {
                    SpectrumRatio::time_space(
                        base_ratio.space_component,
                        base_ratio.time_component * (1.0 + (i as f64 * 0.02)),
                    )
                };

                galaxy.create_solar_system(solar_id, solar_ratio);
            }
        }
    }

    /// Checks if galactic configuration is complete
    pub fn galactic_configuration_complete(&self) -> bool {
        !self.galactic_logoi.is_empty()
            && self
                .galactic_logoi
                .iter()
                .all(|g| g.ready_for_solar_systems())
    }

    /// Checks if ready for Red-Ray transition
    pub fn ready_for_red_transition(&self) -> bool {
        self.galactic_configuration_complete()
            && self
                .galactic_logoi
                .iter()
                .any(|g| !g.solar_systems.is_empty())
    }

    /// Transitions to Red-Ray (Solar-scale configuration + archetypical mind)
    pub fn transition_to_red(&self) -> Result<AttractorField, String> {
        if !self.ready_for_red_transition() {
            return Err("Orange-Ray not ready for Red-Ray transition".to_string());
        }

        Ok(self.attractor_field.clone())
    }

    /// Gets the total number of galaxies
    pub fn galaxy_count(&self) -> usize {
        self.galactic_logoi.len()
    }

    /// Gets the total number of solar systems
    pub fn solar_system_count(&self) -> usize {
        self.galactic_logoi
            .iter()
            .map(|g| g.solar_systems.len())
            .sum()
    }

    /// Gets the attractor-field strength
    pub fn attractor_field_strength(&self) -> f64 {
        self.attractor_field.strength
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a test Light/Love Field for testing
    fn create_test_light_love_field() -> crate::foundation::green_realm::LightLoveField {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = crate::foundation::blue_realm::Logos::from_intelligent_infinity(intelligent);
        crate::foundation::green_realm::LightLoveField::from_logos(logos)
    }

    fn create_test_yellow_realm() -> YellowRealm {
        let field = create_test_light_love_field();
        let mut yellow = YellowRealm::new(field);
        yellow.apply_mysterious_emergence().unwrap();
        yellow
    }

    #[test]
    fn test_localized_spectrum_configuration_new() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let config = LocalizedSpectrumConfiguration::new(
            "test".to_string(),
            SpectrumScale::Galactic,
            ratio,
            0.8,
        );

        assert_eq!(config.id, "test");
        assert_eq!(config.scale, SpectrumScale::Galactic);
        assert_eq!(config.energy_strength, 0.8);
        assert!(config.is_stable());
    }

    #[test]
    fn test_localized_spectrum_configuration_child() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let parent = LocalizedSpectrumConfiguration::new(
            "parent".to_string(),
            SpectrumScale::Galactic,
            ratio,
            0.8,
        );

        let child_ratio = SpectrumRatio::space_time(2.5, 1.0);
        let child = parent.child("child".to_string(), SpectrumScale::Solar, child_ratio, 0.7);

        assert_eq!(child.parent_id, Some("parent".to_string()));
        assert_eq!(child.scale, SpectrumScale::Solar);
    }

    #[test]
    fn test_energy_pattern_new() {
        let pattern = EnergyPattern::new(SpectrumScale::Galactic, 0.8, 0.7);

        assert_eq!(pattern.scale, SpectrumScale::Galactic);
        assert_eq!(pattern.geometry, "spiral");
        assert!(pattern.is_active());
        assert_eq!(pattern.description(), "galaxy spiraling energy");
    }

    #[test]
    fn test_energy_pattern_solar() {
        let pattern = EnergyPattern::new(SpectrumScale::Solar, 0.7, 0.6);

        assert_eq!(pattern.geometry, "helical");
        assert_eq!(pattern.description(), "solar spiraling energy");
    }

    #[test]
    fn test_energy_pattern_planetary() {
        let pattern = EnergyPattern::new(SpectrumScale::Planetary, 0.6, 0.5);

        assert_eq!(pattern.geometry, "orbital");
        assert_eq!(pattern.description(), "planetary spiraling energy");
    }

    #[test]
    fn test_energy_pattern_inactive() {
        let pattern = EnergyPattern::new(SpectrumScale::Galactic, 0.2, 0.2);

        assert!(!pattern.is_active());
    }

    #[test]
    fn test_galactic_logos_new() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let logos = GalacticLogos::new("galaxy1".to_string(), "Milky Way".to_string(), ratio);

        assert_eq!(logos.id, "galaxy1");
        assert_eq!(logos.name, "Milky Way");
        assert_eq!(logos.spectrum_configuration.scale, SpectrumScale::Galactic);
        assert_eq!(logos.energy_patterns.len(), 1);
        assert!(logos.galaxy_spiraling_energy().is_some());
    }

    #[test]
    fn test_galactic_logos_add_solar_system() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let mut logos = GalacticLogos::new("galaxy1".to_string(), "Milky Way".to_string(), ratio);

        let solar_ratio = SpectrumRatio::space_time(2.5, 1.0);
        let solar_config = LocalizedSpectrumConfiguration::new(
            "solar1".to_string(),
            SpectrumScale::Solar,
            solar_ratio,
            0.7,
        );

        logos.add_solar_system(solar_config);

        assert_eq!(logos.solar_systems.len(), 1);
    }

    #[test]
    fn test_galactic_logos_create_solar_system() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let mut logos = GalacticLogos::new("galaxy1".to_string(), "Milky Way".to_string(), ratio);

        logos.create_solar_system("solar1".to_string(), SpectrumRatio::space_time(2.5, 1.0));

        assert_eq!(logos.solar_systems.len(), 1);
        assert_eq!(logos.solar_systems[0].id, "solar1");
        assert_eq!(
            logos.solar_systems[0].parent_id,
            Some("galaxy1".to_string())
        );
    }

    #[test]
    fn test_galactic_logos_ready_for_solar_systems() {
        let ratio = SpectrumRatio::space_time(3.0, 1.0);
        let logos = GalacticLogos::new("galaxy1".to_string(), "Milky Way".to_string(), ratio);

        assert!(logos.ready_for_solar_systems());
    }

    #[test]
    fn test_orange_realm_new() {
        let yellow = create_test_yellow_realm();
        let orange = OrangeRealm::new(yellow);

        assert_eq!(
            orange.attractor_field.name,
            "Solar-scale Spectrum Configuration + Archetypical Mind"
        );
        assert!(orange.galactic_logoi.is_empty());
    }

    #[test]
    fn test_orange_realm_apply_galactic_configuration() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        let result = orange.apply_galactic_configuration();

        assert!(result.is_ok());
        assert!(!orange.galactic_logoi.is_empty());
    }

    #[test]
    fn test_orange_realm_apply_galactic_configuration_not_ready() {
        let field = create_test_light_love_field();
        let yellow = YellowRealm::new(field); // Don't apply emergence
        let mut orange = OrangeRealm::new(yellow);

        let result = orange.apply_galactic_configuration();

        assert!(result.is_err());
    }

    #[test]
    fn test_orange_realm_create_solar_systems() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(3);

        let solar_count = orange.solar_system_count();
        assert!(solar_count > 0);
    }

    #[test]
    fn test_orange_realm_galactic_configuration_complete() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();

        assert!(orange.galactic_configuration_complete());
    }

    #[test]
    fn test_orange_realm_ready_for_red_transition() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(3);

        assert!(orange.ready_for_red_transition());
    }

    #[test]
    fn test_orange_realm_transition_to_red() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(3);

        let result = orange.transition_to_red();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().name,
            "Solar-scale Spectrum Configuration + Archetypical Mind"
        );
    }

    #[test]
    fn test_orange_realm_transition_to_red_not_ready() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        // Don't create solar systems

        let result = orange.transition_to_red();

        assert!(result.is_err());
    }

    #[test]
    fn test_orange_realm_galaxy_count() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();

        assert_eq!(orange.galaxy_count(), orange.galactic_logoi.len());
    }

    #[test]
    fn test_orange_realm_solar_system_count() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(5);

        assert!(orange.solar_system_count() > 0);
    }

    #[test]
    fn test_orange_realm_attractor_field_strength() {
        let yellow = create_test_yellow_realm();
        let orange = OrangeRealm::new(yellow);

        assert_eq!(orange.attractor_field_strength(), 0.85);
    }

    #[test]
    fn test_orange_realm_includes_previous_layers() {
        let yellow = create_test_yellow_realm();
        let orange = OrangeRealm::new(yellow);

        // Verify that Orange-Ray includes all previous layers
        assert!(orange.yellow_realm.dimensional_architecture.emerged);
        assert!(orange.yellow_realm.quantum_realm.formed);
    }

    #[test]
    fn test_consciousness_first_cosmology() {
        // Spectrum patterns exist BEFORE physical matter
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();

        // Spectrum configurations exist (no physical matter yet)
        assert!(!orange.galactic_logoi.is_empty());

        // Each galaxy has a unique spectrum configuration
        for galaxy in &orange.galactic_logoi {
            assert!(galaxy.spectrum_configuration.is_stable());
        }
    }

    #[test]
    fn test_energy_patterns_at_different_scales() {
        let yellow = create_test_yellow_realm();
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().unwrap();
        orange.create_solar_systems(2);

        // Check galaxy spiraling energy
        for galaxy in &orange.galactic_logoi {
            assert!(galaxy.galaxy_spiraling_energy().is_some());
            assert_eq!(
                galaxy.galaxy_spiraling_energy().unwrap().description(),
                "galaxy spiraling energy"
            );
        }

        // Solar systems would have solar spiraling energy (not explicitly stored but implied)
        assert!(orange.solar_system_count() > 0);
    }
}
