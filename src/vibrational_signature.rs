/// Vibratory Signature - the mark, register, and identity of an entity
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "The mark, register, identity, and true vibration of an entity"
///
/// From Ra Material:
/// - The violet ray is the total expression of entity's vibratory complex
/// - It is the sum of mind/body/spirit complex
/// - It represents the entity's true identity beyond veils
use crate::types::Polarity;

/// The complete vibratory signature of an entity
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "The mark, register, identity, and true vibration of an entity"
#[derive(Debug, Clone, PartialEq)]
pub struct VibratorySignature {
    /// Overall activation level (0.0 to 1.0)
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "Total expression of entity's vibratory complex"
    pub activation: f64,

    /// Overall balance level (0.0 to 1.0)
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "Sum of mind/body/spirit complex"
    pub balance: f64,

    /// Resonance quality (0.0 to 1.0)
    ///
    /// How well the entity's vibration aligns with its true nature
    pub resonance: f64,

    /// Dominant polarity (STO or STS)
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "The true vibration of an entity"
    pub polarity: Polarity,

    /// Overall quality of the violet ray (0.0 to 1.0)
    ///
    /// A measure of how developed the violet ray is
    pub quality: f64,
}

impl Default for VibratorySignature {
    fn default() -> Self {
        VibratorySignature {
            activation: 0.0,
            balance: 0.0,
            resonance: 0.0,
            polarity: Polarity::Neutral,
            quality: 0.0,
        }
    }
}

impl VibratorySignature {
    /// Create a new vibratory signature
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn new(activation: f64, balance: f64, resonance: f64, polarity: Polarity) -> Self {
        let quality = Self::calculate_quality(activation, balance, resonance);

        VibratorySignature {
            activation: activation.clamp(0.0, 1.0),
            balance: balance.clamp(0.0, 1.0),
            resonance: resonance.clamp(0.0, 1.0),
            polarity,
            quality,
        }
    }

    /// Calculate overall quality from components
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    fn calculate_quality(activation: f64, balance: f64, resonance: f64) -> f64 {
        // Quality is the average of activation, balance, and resonance
        (activation + balance + resonance) / 3.0
    }

    /// Check if the vibratory signature is well-developed
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_well_developed(&self) -> bool {
        self.quality >= 0.7 && self.balance >= 0.6
    }

    /// Check if the vibratory signature is polarized
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_polarized(&self) -> bool {
        matches!(self.polarity, Polarity::Positive | Polarity::Negative)
    }

    /// Check if the vibratory signature has variation
    ///
    /// Used for unity verification to ensure entities differ in vibrational state
    pub fn has_variation(&self) -> bool {
        // Check if any component is not at default value
        self.activation > 0.0 || self.balance > 0.0 || self.resonance > 0.0
    }

    /// Calculate overall level of the vibratory signature
    ///
    /// Used for unity verification to track vibrational state
    pub fn overall_level(&self) -> f64 {
        self.quality
    }

    /// Get dominant polarity
    ///
    /// Returns the polarity as FreeWillIntegrationPolarity
    pub fn dominant_polarity(&self) -> crate::free_will_integration::FreeWillIntegrationPolarity {
        match self.polarity {
            Polarity::Positive => {
                crate::free_will_integration::FreeWillIntegrationPolarity::Positive
            }
            Polarity::Negative => {
                crate::free_will_integration::FreeWillIntegrationPolarity::Negative
            }
            Polarity::Neutral => crate::free_will_integration::FreeWillIntegrationPolarity::Neutral,
        }
    }

    /// Get dominant polarity as archetypes::common::Polarity
    ///
    /// Returns the polarity for unity verification
    pub fn dominant_polarity_archetypes(&self) -> crate::types::Polarity {
        match self.polarity {
        
            Polarity::Positive => crate::types::Polarity::STO,
            Polarity::Negative => crate::types::Polarity::STS,
            Polarity::Neutral => crate::types::Polarity::SinkholeOfIndifference,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => -1.0,
    }
    }

    /// Calculate difference from another vibratory signature
    ///
    /// Used for unity verification to compare vibrational states
    pub fn difference_from(&self, other: &VibratorySignature) -> f64 {
        let activation_diff = (self.activation - other.activation).abs();
        let balance_diff = (self.balance - other.balance).abs();
        let resonance_diff = (self.resonance - other.resonance).abs();

        (activation_diff + balance_diff + resonance_diff) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibratory_signature_creation() {
        let signature = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        assert_eq!(signature.activation, 0.8);
        assert_eq!(signature.balance, 0.7);
        assert_eq!(signature.resonance, 0.75);
        assert_eq!(signature.polarity, Polarity::Positive);
    }

    #[test]
    fn test_vibratory_signature_quality_calculation() {
        let signature = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        // Quality should be average of activation, balance, and resonance
        let expected_quality = (0.8 + 0.7 + 0.75) / 3.0;
        assert!((signature.quality - expected_quality).abs() < 0.001);
    }

    #[test]
    fn test_vibratory_signature_clamping() {
        // Values above 1.0 should be clamped
        let signature = VibratorySignature::new(1.5, 2.0, 0.75, Polarity::Positive);

        assert_eq!(signature.activation, 1.0);
        assert_eq!(signature.balance, 1.0);
    }

    #[test]
    fn test_vibratory_signature_is_well_developed() {
        let well_developed = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let not_well_developed = VibratorySignature::new(0.6, 0.5, 0.5, Polarity::Neutral);

        assert!(well_developed.is_well_developed());
        assert!(!not_well_developed.is_well_developed());
    }

    #[test]
    fn test_vibratory_signature_is_polarized() {
        let sto_polarized = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Positive);

        let sts_polarized = VibratorySignature::new(0.8, 0.7, 0.75, Polarity::Negative);

        let undetermined = VibratorySignature::new(0.5, 0.5, 0.5, Polarity::Neutral);

        assert!(sto_polarized.is_polarized());
        assert!(sts_polarized.is_polarized());
        assert!(!undetermined.is_polarized());
    }

    #[test]
    fn test_vibratory_signature_default() {
        let signature = VibratorySignature::default();

        assert_eq!(signature.activation, 0.0);
        assert_eq!(signature.balance, 0.0);
        assert_eq!(signature.resonance, 0.0);
        assert_eq!(signature.polarity, Polarity::Neutral);
        assert_eq!(signature.quality, 0.0);
    }
}
