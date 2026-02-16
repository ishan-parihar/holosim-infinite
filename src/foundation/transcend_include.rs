/// The Universal Constant: Transcend and Include
///
/// This is the fundamental mechanism by which the holographic principle operates.
/// Each step in involution/evolution is a "transcend and include" operation:
/// - INCLUDE: Retains all development from previous stages
/// - TRANSCEND: Adds new development that transcends the previous
/// - EVOLVES INTO: Creates attractor-fields that pull toward the next stage
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each layer includes all previous development, transcends by adding fundamentally new capabilities,
/// and creates attractor-fields that pull toward the next stage."
use std::fmt::Debug;

/// Attractor-field - "spiritual gravity" that pulls toward the next stage
///
/// Each stage creates attractor-fields that pull the previous stage toward the next
/// level of vibration/frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorField {
    /// Name of the attractor-field
    pub name: String,
    /// Strength of the attraction (0.0 to 1.0)
    pub strength: f64,
    /// Description of what this attractor-field pulls toward
    pub target: String,
}

impl AttractorField {
    /// Create a new attractor-field
    pub fn new(name: impl Into<String>, strength: f64, target: impl Into<String>) -> Self {
        assert!(
            strength >= 0.0 && strength <= 1.0,
            "Strength must be between 0.0 and 1.0"
        );
        AttractorField {
            name: name.into(),
            strength,
            target: target.into(),
        }
    }

    /// Pull the previous stage toward the next level
    pub fn pull(&self) -> f64 {
        self.strength
    }
}

/// Stage transition - represents the "transcend and include" operation
///
/// This is the universal mechanism that operates at every stage of creation.
/// Each transition includes all previous development, transcends by adding new development,
/// and creates an attractor-field that pulls toward the next stage.
#[derive(Debug, Clone)]
pub struct StageTransition<L1> {
    /// The previous stage (what is INCLUDED)
    pub includes: L1,
    /// The new development (what TRANSCENDS)
    pub transcends: Feature,
    /// The attractor-field (what EVOLVES INTO)
    pub evolves_into: AttractorField,
}

/// Represents a new feature that transcends the previous stage
#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    /// Name of the feature
    pub name: String,
    /// Description of the feature
    pub description: String,
    /// Strength of the feature (0.0 to 1.0)
    pub strength: f64,
}

impl Feature {
    /// Create a new feature
    pub fn new(name: impl Into<String>, description: impl Into<String>, strength: f64) -> Self {
        assert!(
            strength >= 0.0 && strength <= 1.0,
            "Strength must be between 0.0 and 1.0"
        );
        Feature {
            name: name.into(),
            description: description.into(),
            strength,
        }
    }

    /// Apply this feature to the previous stage
    pub fn apply(&self) -> f64 {
        self.strength
    }
}

impl<L1> StageTransition<L1>
where
    L1: Debug + Clone,
{
    /// Apply the stage transition
    ///
    /// This implements the "transcend and include" mechanism:
    /// 1. INCLUDE: Retain all previous development
    /// 2. TRANSCEND: Add new development
    /// 3. EVOLVES INTO: Create attractor-field
    pub fn apply(&self, previous: L1) -> (L1, Feature, AttractorField) {
        // INCLUDE: Retain all previous development
        let included = previous.clone();

        // TRANSCEND: Add new development
        let transcended = self.transcends.clone();

        // EVOLVES INTO: Create attractor-field
        let attractor_field = self.evolves_into.clone();

        (included, transcended, attractor_field)
    }

    /// Get the attractor-field strength
    pub fn attractor_strength(&self) -> f64 {
        self.evolves_into.strength
    }
}

/// The universal "transcend and include" operation
///
/// This function represents the universal constant that operates at every stage.
pub fn transcend_include<L1>(
    previous: L1,
    new_feature: Feature,
    attractor_field: AttractorField,
) -> (L1, Feature, AttractorField)
where
    L1: Debug + Clone,
{
    (
        previous.clone(), // INCLUDE: Retain all previous development
        new_feature,      // TRANSCEND: Add new development
        attractor_field,  // EVOLVES INTO: Create attractor-field
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attractor_field_creation() {
        let attractor = AttractorField::new("Test Attractor", 0.8, "Next Stage");
        assert_eq!(attractor.name, "Test Attractor");
        assert_eq!(attractor.strength, 0.8);
        assert_eq!(attractor.target, "Next Stage");
    }

    #[test]
    fn test_attractor_field_strength_validation() {
        // Valid strength
        let attractor = AttractorField::new("Test", 0.5, "Target");
        assert_eq!(attractor.strength, 0.5);
    }

    #[test]
    #[should_panic(expected = "Strength must be between 0.0 and 1.0")]
    fn test_attractor_field_invalid_strength() {
        // Invalid strength (should panic)
        let _invalid = AttractorField::new("Test", 1.5, "Target");
    }

    #[test]
    fn test_feature_creation() {
        let feature = Feature::new("Test Feature", "A test feature", 0.7);
        assert_eq!(feature.name, "Test Feature");
        assert_eq!(feature.description, "A test feature");
        assert_eq!(feature.strength, 0.7);
    }

    #[test]
    fn test_stage_transition_creation() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        assert_eq!(transition.includes, "Previous Stage");
        assert_eq!(transition.transcends.name, "New Feature");
        assert_eq!(transition.evolves_into.name, "Attractor");
    }

    #[test]
    fn test_stage_transition_apply() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        let (included, transcended, attractor) = transition.apply(previous.to_string());

        assert_eq!(included, "Previous Stage");
        assert_eq!(transcended.name, "New Feature");
        assert_eq!(attractor.name, "Attractor");
    }

    #[test]
    fn test_transcend_include_function() {
        let previous = "Previous Stage";
        let new_feature = Feature::new("New Feature", "A new feature", 0.8);
        let attractor_field = AttractorField::new("Attractor", 0.9, "Next Stage");

        let (included, transcended, attractor) =
            transcend_include(previous, new_feature, attractor_field);

        assert_eq!(included, "Previous Stage");
        assert_eq!(transcended.name, "New Feature");
        assert_eq!(attractor.name, "Attractor");
    }

    #[test]
    fn test_attractor_field_pull() {
        let attractor = AttractorField::new("Test Attractor", 0.8, "Next Stage");
        assert_eq!(attractor.pull(), 0.8);
    }

    #[test]
    fn test_stage_transition_attractor_strength() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        assert_eq!(transition.attractor_strength(), 0.9);
    }
}
