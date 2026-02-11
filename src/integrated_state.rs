/// Integrated State - the sum of all energy center states
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "Sum of mind/body/spirit complex"
///
/// From Ra Material:
/// - The violet ray represents the total integration of all energy centers
/// - It is the sum of mind/body/spirit complex
/// - All centers contribute to the violet ray expression
use crate::energy_ray_centers::RayCenter;

/// The integrated state of all energy centers
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "Sum of mind/body/spirit complex"
#[derive(Debug, Clone, PartialEq)]
pub struct IntegratedState {
    /// Total activation across all centers (0.0 to 1.0)
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "Sum of mind/body/spirit complex"
    pub total_activation: f64,

    /// Overall balance across all centers (0.0 to 1.0)
    ///
    /// How well all centers are balanced with each other
    pub overall_balance: f64,

    /// Per-center activation levels
    ///
    /// Activation level for each of the 7 energy centers
    pub center_activations: [f64; 7],

    /// Per-center balance levels
    ///
    /// Balance level for each of the 7 energy centers
    pub center_balances: [f64; 7],

    /// Integration coherence (0.0 to 1.0)
    ///
    /// How well all centers work together
    pub integration_coherence: f64,
}

impl Default for IntegratedState {
    fn default() -> Self {
        IntegratedState {
            total_activation: 0.0,
            overall_balance: 0.0,
            center_activations: [0.0; 7],
            center_balances: [0.0; 7],
            integration_coherence: 0.0,
        }
    }
}

impl IntegratedState {
    /// Create a new integrated state from center data
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn new(center_activations: [f64; 7], center_balances: [f64; 7]) -> Self {
        let total_activation = Self::calculate_total_activation(&center_activations);
        let overall_balance = Self::calculate_overall_balance(&center_balances);
        let integration_coherence =
            Self::calculate_integration_coherence(&center_activations, &center_balances);

        IntegratedState {
            total_activation,
            overall_balance,
            center_activations,
            center_balances,
            integration_coherence,
        }
    }

    /// Calculate total activation across all centers
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    fn calculate_total_activation(activations: &[f64; 7]) -> f64 {
        let sum: f64 = activations.iter().sum();
        sum / 7.0
    }

    /// Calculate overall balance across all centers
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    fn calculate_overall_balance(balances: &[f64; 7]) -> f64 {
        let sum: f64 = balances.iter().sum();
        sum / 7.0
    }

    /// Calculate integration coherence
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    fn calculate_integration_coherence(activations: &[f64; 7], balances: &[f64; 7]) -> f64 {
        // Coherence is high when activations are balanced with balances
        let mut coherence = 0.0;
        for i in 0..7 {
            // Ideally, activation and balance should be aligned
            let alignment = 1.0 - (activations[i] - balances[i]).abs();
            coherence += alignment;
        }
        coherence / 7.0
    }

    /// Get activation for a specific center
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn get_center_activation(&self, center: RayCenter) -> f64 {
        // RayCenter enum starts at 1, but array indexing starts at 0
        let index = center as usize - 1;
        self.center_activations[index]
    }

    /// Get balance for a specific center
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn get_center_balance(&self, center: RayCenter) -> f64 {
        // RayCenter enum starts at 1, but array indexing starts at 0
        let index = center as usize - 1;
        self.center_balances[index]
    }

    /// Check if the integrated state is well-balanced
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_well_balanced(&self) -> bool {
        self.overall_balance >= 0.6 && self.integration_coherence >= 0.6
    }

    /// Check if the integrated state is highly activated
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_highly_activated(&self) -> bool {
        self.total_activation >= 0.7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrated_state_creation() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        assert_eq!(state.center_activations, activations);
        assert_eq!(state.center_balances, balances);
    }

    #[test]
    fn test_total_activation_calculation() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        // Average should be (0.5 + 0.6 + 0.7 + 0.8 + 0.7 + 0.6 + 0.5) / 7
        let expected = 4.4 / 7.0;
        assert!((state.total_activation - expected).abs() < 0.001);
    }

    #[test]
    fn test_overall_balance_calculation() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        // Average should be (0.5 + 0.6 + 0.7 + 0.8 + 0.7 + 0.6 + 0.5) / 7
        let expected = 4.4 / 7.0;
        assert!((state.overall_balance - expected).abs() < 0.001);
    }

    #[test]
    fn test_integration_coherence_calculation() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        // When activations and balances are perfectly aligned, coherence should be 1.0
        assert!((state.integration_coherence - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_get_center_activation() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        assert_eq!(state.get_center_activation(RayCenter::Red), 0.5);
        assert_eq!(state.get_center_activation(RayCenter::Green), 0.8);
        assert_eq!(state.get_center_activation(RayCenter::Violet), 0.5);
    }

    #[test]
    fn test_get_center_balance() {
        let activations = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];
        let balances = [0.5, 0.6, 0.7, 0.8, 0.7, 0.6, 0.5];

        let state = IntegratedState::new(activations, balances);

        assert_eq!(state.get_center_balance(RayCenter::Red), 0.5);
        assert_eq!(state.get_center_balance(RayCenter::Green), 0.8);
        assert_eq!(state.get_center_balance(RayCenter::Violet), 0.5);
    }

    #[test]
    fn test_is_well_balanced() {
        let well_balanced = IntegratedState::new(
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
        );

        let not_well_balanced = IntegratedState::new(
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
        );

        assert!(well_balanced.is_well_balanced());
        assert!(!not_well_balanced.is_well_balanced());
    }

    #[test]
    fn test_is_highly_activated() {
        let highly_activated = IntegratedState::new(
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
            [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
        );

        let not_highly_activated = IntegratedState::new(
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
        );

        assert!(highly_activated.is_highly_activated());
        assert!(!not_highly_activated.is_highly_activated());
    }

    #[test]
    fn test_integrated_state_default() {
        let state = IntegratedState::default();

        assert_eq!(state.total_activation, 0.0);
        assert_eq!(state.overall_balance, 0.0);
        assert_eq!(state.center_activations, [0.0; 7]);
        assert_eq!(state.center_balances, [0.0; 7]);
        assert_eq!(state.integration_coherence, 0.0);
    }
}
