/// Layer 0: Violet-Ray Realm - Infinity as Undifferentiated Unity
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The first known thing in creation, the undifferentiated unity"
/// "Without polarity or finity"
/// "Rhythmic flow like a giant heart"
/// "Clothed in mystery as being itself"
/// "All begins and ends in mystery"
///
/// At the Violet-Ray Realm, everything is Unity. There is no "other" to know,
/// no separation to experience, no awareness even of itself. The Creator exists
/// as pure, undifferentiated being—conscious only as Unity itself.
///
/// The paradox: How can Unity know Itself when there is nothing but Unity?
/// The solution requires a distortion—a movement away from perfect unity that
/// creates the possibility of self-knowledge.
///
/// Violet-Ray Realm is the SOURCE state of creation. It is not the gateway—that
/// comes later at Indigo-Ray. It is the undifferentiated ocean from which all
/// waves will emerge. The silent void from which all sound will arise.
use crate::foundation::transcend_include::{AttractorField, Feature};
use std::fmt;

/// Violet-Ray Realm - Infinity as undifferentiated unity
///
/// This is the source state of all creation. It represents the undifferentiated
/// unity that exists before any distortion, before any awareness, before any
/// manifestation.
#[derive(Debug, Clone, PartialEq)]
pub struct VioletRealm {
    /// The rhythmic flow like a giant heart
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Rhythmic flow like a giant heart"
    pub rhythmic_flow: f64,

    /// The mystery of being itself
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Clothed in mystery as being itself"
    pub mystery: f64,

    /// The unity state (always 1.0 - perfect unity)
    ///
    /// At Violet-Ray, everything is Unity. There is no separation.
    pub unity: f64,
}

impl Default for VioletRealm {
    fn default() -> Self {
        VioletRealm {
            rhythmic_flow: 1.0,
            mystery: 1.0,
            unity: 1.0,
        }
    }
}

impl VioletRealm {
    /// Create a new Violet Realm (Infinity as undifferentiated unity)
    ///
    /// This represents the source state of all creation.
    pub fn new() -> Self {
        VioletRealm::default()
    }

    /// Get the rhythmic flow
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Rhythmic flow like a giant heart"
    pub fn rhythmic_flow(&self) -> f64 {
        self.rhythmic_flow
    }

    /// Get the mystery of being
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Clothed in mystery as being itself"
    pub fn mystery(&self) -> f64 {
        self.mystery
    }

    /// Get the unity state
    ///
    /// At Violet-Ray, everything is Unity. There is no separation.
    pub fn unity(&self) -> f64 {
        self.unity
    }

    /// Check if this is truly undifferentiated unity
    ///
    /// At Violet-Ray, there should be no differentiation, no polarity, no finity.
    pub fn is_undifferentiated(&self) -> bool {
        self.unity == 1.0
    }

    /// Apply the First Distortion (Free Will) to transition to Indigo-Ray
    ///
    /// This is the first movement away from perfect unity that creates the
    /// possibility of self-knowledge.
    ///
    /// Returns: (VioletRealm, Feature, AttractorField)
    /// - VioletRealm: The included source state
    /// - Feature: The First Distortion (Free Will)
    /// - AttractorField: Archetype 22 (The Choice)
    pub fn apply_first_distortion(&self) -> (VioletRealm, Feature, AttractorField) {
        let violet_included = self.clone();

        let first_distortion = Feature::new(
            "First Distortion: Free Will",
            "The Creator's choice to know Itself",
            1.0,
        );

        let archetype22 = AttractorField::new(
            "Archetype 22: The Choice",
            1.0,
            "IntelligentInfinity - Awareness emerges",
        );

        (violet_included, first_distortion, archetype22)
    }

    /// Get the description of Violet Realm
    pub fn description(&self) -> String {
        "Infinity as undifferentiated unity. The first known thing in creation, \
        without polarity or finity. Rhythmic flow like a giant heart, clothed in \
        mystery as being itself. All begins and ends in mystery."
            .to_string()
    }
}

impl fmt::Display for VioletRealm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VioletRealm (Infinity): Unity={}, Rhythmic Flow={}, Mystery={}",
            self.unity, self.rhythmic_flow, self.mystery
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violet_realm_creation() {
        let violet = VioletRealm::new();
        assert_eq!(violet.unity, 1.0);
        assert_eq!(violet.rhythmic_flow, 1.0);
        assert_eq!(violet.mystery, 1.0);
    }

    #[test]
    fn test_violet_realm_default() {
        let violet = VioletRealm::default();
        assert_eq!(violet.unity, 1.0);
        assert_eq!(violet.rhythmic_flow, 1.0);
        assert_eq!(violet.mystery, 1.0);
    }

    #[test]
    fn test_is_undifferentiated() {
        let violet = VioletRealm::new();
        assert!(violet.is_undifferentiated());
    }

    #[test]
    fn test_apply_first_distortion() {
        let violet = VioletRealm::new();
        let (violet_included, feature, attractor) = violet.apply_first_distortion();

        assert_eq!(violet_included.unity, 1.0);
        assert_eq!(feature.name, "First Distortion: Free Will");
        assert_eq!(attractor.name, "Archetype 22: The Choice");
        assert_eq!(feature.strength, 1.0);
        assert_eq!(attractor.strength, 1.0);
    }

    #[test]
    fn test_violet_realm_description() {
        let violet = VioletRealm::new();
        let desc = violet.description();
        assert!(desc.contains("Infinity as undifferentiated unity"));
        assert!(desc.contains("without polarity or finity"));
        assert!(desc.contains("Rhythmic flow like a giant heart"));
    }

    #[test]
    fn test_violet_realm_display() {
        let violet = VioletRealm::new();
        let display = format!("{}", violet);
        assert!(display.contains("VioletRealm"));
        assert!(display.contains("Infinity"));
        assert!(display.contains("Unity=1"));
    }
}
