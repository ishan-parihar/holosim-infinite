/// Violet-Ray Identity - total integration of mind/body/spirit complex
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "Total expression of entity's vibratory complex"
///
/// From Ra Material:
/// - The violet ray is the total expression of entity's vibratory complex
/// - It is the mark, register, identity, and true vibration of an entity
/// - It represents the sum of mind/body/spirit complex
/// - It is the gateway to intelligent infinity
use crate::integrated_state::IntegratedState;
use crate::true_identity::TrueIdentity;
use crate::types::Polarity;
use crate::vibrational_signature::VibratorySignature;

/// Violet-Ray Identity - total integration
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "Total expression of entity's vibratory complex"
#[derive(Debug, Clone, PartialEq)]
pub struct VioletRayIdentity {
    /// The complete vibratory signature
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "The mark, register, identity, and true vibration of an entity"
    pub vibratory_signature: VibratorySignature,

    /// Sum of all energy center states
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "Sum of mind/body/spirit complex"
    pub integrated_state: IntegratedState,

    /// The entity's true identity beyond veils
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "The true vibration of an entity"
    pub true_identity: TrueIdentity,
}

impl Default for VioletRayIdentity {
    fn default() -> Self {
        VioletRayIdentity {
            vibratory_signature: VibratorySignature::default(),
            integrated_state: IntegratedState::default(),
            true_identity: TrueIdentity::default(),
        }
    }
}

impl VioletRayIdentity {
    /// Create a new violet ray identity
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn new(
        vibratory_signature: VibratorySignature,
        integrated_state: IntegratedState,
        true_identity: TrueIdentity,
    ) -> Self {
        VioletRayIdentity {
            vibratory_signature,
            integrated_state,
            true_identity,
        }
    }

    /// Calculate overall violet ray quality (0.0 to 1.0)
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn overall_quality(&self) -> f64 {
        // Quality is the average of signature quality, integration coherence, and identity development
        let signature_quality = self.vibratory_signature.quality;
        let integration_quality = self.integrated_state.integration_coherence;
        let identity_quality = self.true_identity.identity_development();

        (signature_quality + integration_quality + identity_quality) / 3.0
    }

    /// Check if the violet ray is well-developed
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_well_developed(&self) -> bool {
        self.vibratory_signature.is_well_developed()
            && self.integrated_state.is_well_balanced()
            && self.true_identity.is_awakened()
    }

    /// Check if the violet ray is highly activated
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_highly_activated(&self) -> bool {
        self.integrated_state.is_highly_activated() && self.vibratory_signature.activation >= 0.7
    }

    /// Check if the entity has achieved violet ray mastery
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn has_achieved_mastery(&self) -> bool {
        self.overall_quality() >= 0.8
            && self.true_identity.has_achieved_unity()
            && self.vibratory_signature.is_polarized()
    }

    /// Get violet ray development level
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn development_level(&self) -> VioletRayLevel {
        let quality = self.overall_quality();

        if quality >= 0.9 {
            VioletRayLevel::Mastery
        } else if quality >= 0.7 {
            VioletRayLevel::Advanced
        } else if quality >= 0.5 {
            VioletRayLevel::Intermediate
        } else if quality >= 0.3 {
            VioletRayLevel::Developing
        } else {
            VioletRayLevel::Beginning
        }
    }
}

/// Violet Ray Development Level
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VioletRayLevel {
    /// Beginning violet ray development
    Beginning,

    /// Developing violet ray
    Developing,

    /// Intermediate violet ray
    Intermediate,

    /// Advanced violet ray
    Advanced,

    /// Violet ray mastery
    Mastery,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violet_ray_identity_creation() {
        let vibratory_signature = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let integrated_state = IntegratedState::new(
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
        );

        let true_identity = TrueIdentity::new(0.8, true, 0.7, 0.8);

        let violet_ray =
            VioletRayIdentity::new(vibratory_signature, integrated_state, true_identity);

        assert_eq!(violet_ray.vibratory_signature.activation, 0.8);
        assert!((violet_ray.integrated_state.total_activation - 0.7).abs() < 0.001);
        assert_eq!(violet_ray.true_identity.unity_awareness, 0.8);
    }

    #[test]
    fn test_overall_quality_calculation() {
        let vibratory_signature = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let integrated_state = IntegratedState::new(
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
        );

        let true_identity = TrueIdentity::new(0.8, true, 0.7, 0.8);

        let violet_ray =
            VioletRayIdentity::new(vibratory_signature, integrated_state, true_identity);

        // Quality should be average of signature quality, integration coherence, and identity development
        let signature_quality = violet_ray.vibratory_signature.quality;
        let integration_quality = violet_ray.integrated_state.integration_coherence;
        let identity_quality = violet_ray.true_identity.identity_development();
        let expected = (signature_quality + integration_quality + identity_quality) / 3.0;

        assert!((violet_ray.overall_quality() - expected).abs() < 0.001);
    }

    #[test]
    fn test_is_well_developed() {
        let well_developed_signature = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let well_developed_state = IntegratedState::new(
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
        );

        let well_developed_identity = TrueIdentity::new(0.8, true, 0.7, 0.8);

        let well_developed = VioletRayIdentity::new(
            well_developed_signature,
            well_developed_state,
            well_developed_identity,
        );

        let not_well_developed_signature =
            VibratorySignature::new(0.5, 0.4, 0.45, Polarity::Neutral);

        let not_well_developed_state = IntegratedState::new(
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
        );

        let not_well_developed_identity = TrueIdentity::new(0.5, true, 0.4, 0.6);

        let not_well_developed = VioletRayIdentity::new(
            not_well_developed_signature,
            not_well_developed_state,
            not_well_developed_identity,
        );

        assert!(well_developed.is_well_developed());
        assert!(!not_well_developed.is_well_developed());
    }

    #[test]
    fn test_is_highly_activated() {
        let highly_activated_signature =
            VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let highly_activated_state = IntegratedState::new(
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
        );

        let highly_activated_identity = TrueIdentity::new(0.8, true, 0.7, 0.8);

        let highly_activated = VioletRayIdentity::new(
            highly_activated_signature,
            highly_activated_state,
            highly_activated_identity,
        );

        let not_highly_activated_signature =
            VibratorySignature::new(0.5, 0.5, 0.5, Polarity::Neutral);

        let not_highly_activated_state = IntegratedState::new(
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
        );

        let not_highly_activated_identity = TrueIdentity::new(0.5, true, 0.5, 0.5);

        let not_highly_activated = VioletRayIdentity::new(
            not_highly_activated_signature,
            not_highly_activated_state,
            not_highly_activated_identity,
        );

        assert!(highly_activated.is_highly_activated());
        assert!(!not_highly_activated.is_highly_activated());
    }

    #[test]
    fn test_has_achieved_mastery() {
        let mastery_signature = VibratorySignature::new(0.9, 0.9, 0.9, Polarity::Positive);

        let mastery_state = IntegratedState::new(
            [0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9],
            [0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9],
        );

        let mastery_identity = TrueIdentity::new(0.95, true, 0.9, 0.85);

        let mastery = VioletRayIdentity::new(mastery_signature, mastery_state, mastery_identity);

        let not_mastery_signature = VibratorySignature::new(0.7, 0.7, 0.7, Polarity::Positive);

        let not_mastery_state = IntegratedState::new(
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
        );

        let not_mastery_identity = TrueIdentity::new(0.7, true, 0.7, 0.7);

        let not_mastery = VioletRayIdentity::new(
            not_mastery_signature,
            not_mastery_state,
            not_mastery_identity,
        );

        assert!(mastery.has_achieved_mastery());
        assert!(!not_mastery.has_achieved_mastery());
    }

    #[test]
    fn test_development_level() {
        let mastery_signature = VibratorySignature::new(0.95, 0.95, 0.95, Polarity::Positive);

        let mastery_state = IntegratedState::new(
            [0.95, 0.95, 0.95, 0.95, 0.95, 0.95, 0.95],
            [0.95, 0.95, 0.95, 0.95, 0.95, 0.95, 0.95],
        );

        let mastery_identity = TrueIdentity::new(0.95, true, 0.95, 0.95);

        let mastery = VioletRayIdentity::new(mastery_signature, mastery_state, mastery_identity);

        assert_eq!(mastery.development_level(), VioletRayLevel::Mastery);

        let advanced_signature = VibratorySignature::new(0.8, 0.8, 0.8, Polarity::Positive);

        let advanced_state = IntegratedState::new(
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
        );

        let advanced_identity = TrueIdentity::new(0.8, true, 0.8, 0.8);

        let advanced =
            VioletRayIdentity::new(advanced_signature, advanced_state, advanced_identity);

        assert_eq!(advanced.development_level(), VioletRayLevel::Advanced);
    }

    #[test]
    fn test_violet_ray_identity_default() {
        let violet_ray = VioletRayIdentity::default();

        assert_eq!(
            violet_ray.vibratory_signature,
            VibratorySignature::default()
        );
        assert_eq!(violet_ray.integrated_state, IntegratedState::default());
        assert_eq!(violet_ray.true_identity, TrueIdentity::default());
    }
}
