// Density Variation - How veil varies by density and polarization
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "The Veil separates these realms, creating the illusion that they are separate"

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;
use crate::types::Polarity;

/// Density-based transparency (higher density = more transparent)
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Veil thins as entities polarize and ascend"
///
/// Veil Thickness Curve (from COMPLETE_REFACTOR_ROADMAP_V4.md):
/// - D7: 0% veil (100% transparent) - Gateway density, no separation
/// - D6: 15% veil (85% transparent) - Unity density, minimal separation
/// - D5: 30% veil (70% transparent) - Wisdom density, light separation
/// - D4: 45% veil (55% transparent) - Love density, moderate separation
/// - D3: 60% veil (40% transparent) - Choice density, thick separation
/// - D2: 75% veil (25% transparent) - Growth density, thicker separation
/// - D1: 90% veil (10% transparent) - Awareness density, maximum separation
#[derive(Debug, Clone, PartialEq)]
pub struct DensityTransparency {
    /// Base thickness for each density (0.0 = no veil, 1.0 = thick veil)
    /// Index 0 = D1, Index 6 = D7
    base_thickness: [Float; 7],
}

impl DensityTransparency {
    /// Create new density transparency with standard values
    pub fn new() -> Self {
        DensityTransparency {
            // Veil thickness curve from roadmap:
            // D1: 90% veil (maximum separation for awareness)
            // D2: 75% veil (thick separation for growth)
            // D3: 60% veil (choice point with thick separation)
            // D4: 45% veil (love density with moderate separation)
            // D5: 30% veil (wisdom density with light separation)
            // D6: 15% veil (unity density with minimal separation)
            // D7: 0% veil (gateway density, no separation)
            base_thickness: [0.90, 0.75, 0.60, 0.45, 0.30, 0.15, 0.00],
        }
    }

    /// Create density transparency with custom base thickness values
    pub fn with_custom_thickness(thickness: [Float; 7]) -> Self {
        let clamped: [Float; 7] = thickness.map(|t| t.max(0.0).min(1.0));
        DensityTransparency {
            base_thickness: clamped,
        }
    }

    /// Get base thickness for a specific density
    pub fn base_thickness(&self, density: &Density) -> Float {
        let idx = Self::density_to_index(density);
        self.base_thickness[idx]
    }

    /// Convert Density to array index (0-6 for D1-D7)
    pub fn density_to_index(density: &Density) -> usize {
        match density {
            Density::First(_) => 0,
            Density::Second(_) => 1,
            Density::Third => 2,
            Density::Fourth => 3,
            Density::Fifth => 4,
            Density::Sixth => 5,
            Density::Seventh => 6,
            Density::Eighth => 6, // Eighth density uses same veil transparency as Seventh
        }
    }

    /// Get transparency (1.0 - thickness) for a specific density
    pub fn transparency(&self, density: &Density) -> Float {
        1.0 - self.base_thickness(density)
    }

    /// Set veil thickness for a specific density
    ///
    /// This method allows dynamic adjustment of veil thickness during involution.
    /// Called by VeilThickeningMechanism when entities transition between densities.
    ///
    /// # Arguments
    /// * `density` - The density to set thickness for
    /// * `thickness` - The new thickness (0.0 to 1.0)
    pub fn set_thickness(&mut self, density: Density, thickness: Float) {
        let idx = Self::density_to_index(&density);
        self.base_thickness[idx] = thickness.max(0.0).min(1.0);
    }

    /// Get the veil thickness curve as an array
    pub fn thickness_curve(&self) -> [Float; 7] {
        self.base_thickness
    }

    /// Calculate veil thickness between two densities (for transitions)
    ///
    /// This method calculates the veil thickness during a density transition,
    /// allowing for gradual thickening or thinning.
    ///
    /// # Arguments
    /// * `from_density` - Starting density
    /// * `to_density` - Target density
    /// * `progress` - Transition progress (0.0 to 1.0)
    ///
    /// # Returns
    /// Interpolated veil thickness
    pub fn transition_thickness(
        &self,
        from_density: Density,
        to_density: Density,
        progress: Float,
    ) -> Float {
        let from_thickness = self.base_thickness(&from_density);
        let to_thickness = self.base_thickness(&to_density);
        let clamped_progress = progress.max(0.0).min(1.0);

        from_thickness + (to_thickness - from_thickness) * clamped_progress
    }
}

impl Default for DensityTransparency {
    fn default() -> Self {
        Self::new()
    }
}

/// Polarity-based access to thinner veil
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Veil thins as entities polarize and ascend"
#[derive(Debug, Clone, PartialEq)]
pub struct PolarizationAccess {
    /// Thinning factor for STO polarization
    sto_thinning_factor: Float,

    /// Thinning factor for STS polarization
    sts_thinning_factor: Float,

    /// Thinning factor for neutral polarization
    neutral_thinning_factor: Float,
}

impl PolarizationAccess {
    /// Create new polarization access with standard values
    pub fn new() -> Self {
        PolarizationAccess {
            // Both polarizations thin the veil, but STO tends to thin more
            sto_thinning_factor: 0.3,
            sts_thinning_factor: 0.2,
            neutral_thinning_factor: 0.0,
        }
    }

    /// Create polarization access with custom thinning factors
    pub fn with_custom_factors(sto: Float, sts: Float, neutral: Float) -> Self {
        PolarizationAccess {
            sto_thinning_factor: sto.max(0.0).min(1.0),
            sts_thinning_factor: sts.max(0.0).min(1.0),
            neutral_thinning_factor: neutral.max(0.0).min(1.0),
        }
    }

    /// Calculate thinning based on polarization state
    ///
    /// # Arguments
    /// * `polarization` - The polarization state with polarization intensity (0.0 to 1.0)
    ///
    /// # Returns
    /// Thinning amount (0.0 to 1.0)
    pub fn calculate_thinning(&self, polarization: &PolarizationState) -> Float {
        let intensity = polarization.intensity;
        let thinning_factor = match polarization.polarization {
            crate::types::Polarity::STO | crate::types::Polarity::ServiceToOthers => {
                self.sto_thinning_factor
            }
            crate::types::Polarity::STS | crate::types::Polarity::ServiceToSelf => {
                self.sts_thinning_factor
            }
            crate::types::Polarity::SinkholeOfIndifference | crate::types::Polarity::Neutral => {
                self.neutral_thinning_factor
            }
        };

        intensity * thinning_factor
    }

    /// Set STO thinning factor
    pub fn set_sto_factor(&mut self, factor: Float) {
        self.sto_thinning_factor = factor.max(0.0).min(1.0);
    }

    /// Set STS thinning factor
    pub fn set_sts_factor(&mut self, factor: Float) {
        self.sts_thinning_factor = factor.max(0.0).min(1.0);
    }

    /// Set neutral thinning factor
    pub fn set_neutral_factor(&mut self, factor: Float) {
        self.neutral_thinning_factor = factor.max(0.0).min(1.0);
    }
}

impl Default for PolarizationAccess {
    fn default() -> Self {
        Self::new()
    }
}

/// Polarity state including polarization type and intensity
#[derive(Debug, Clone, PartialEq)]
pub struct PolarizationState {
    /// Polarity direction
    pub polarization: Polarity,

    /// Polarity intensity (0.0 = neutral, 1.0 = fully polarized)
    pub intensity: Float,
}

impl PolarizationState {
    /// Create new polarization state
    pub fn new(polarization: Polarity, intensity: Float) -> Self {
        PolarizationState {
            polarization,
            intensity: intensity.max(0.0).min(1.0),
        }
    }

    /// Create STO polarization state
    pub fn sto(intensity: Float) -> Self {
        Self::new(Polarity::STO, intensity)
    }

    /// Create STS polarization state
    pub fn sts(intensity: Float) -> Self {
        Self::new(Polarity::STS, intensity)
    }

    /// Create neutral polarization state
    pub fn neutral() -> Self {
        Self::new(Polarity::SinkholeOfIndifference, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};

    // ===== DensityTransparency Tests =====

    #[test]
    fn test_density_transparency_new() {
        let dt = DensityTransparency::new();
        // Veil thickness curve from roadmap
        assert_eq!(
            dt.base_thickness(&Density::First(Density1SubLevel::Atomic)),
            0.90
        );
        assert_eq!(dt.base_thickness(&Density::Third), 0.60);
        assert_eq!(dt.base_thickness(&Density::Seventh), 0.00);
    }

    #[test]
    fn test_density_transparency_thickness() {
        let dt = DensityTransparency::new();
        // Veil thickness curve from roadmap
        assert_eq!(
            dt.base_thickness(&Density::First(Density1SubLevel::Atomic)),
            0.90
        );
        assert_eq!(
            dt.base_thickness(&Density::Second(Density2SubLevel::Cellular)),
            0.75
        );
        assert_eq!(dt.base_thickness(&Density::Third), 0.60);
        assert_eq!(dt.base_thickness(&Density::Fourth), 0.45);
        assert_eq!(dt.base_thickness(&Density::Fifth), 0.30);
        assert_eq!(dt.base_thickness(&Density::Sixth), 0.15);
        assert_eq!(dt.base_thickness(&Density::Seventh), 0.00);
    }

    #[test]
    fn test_density_transparency_transparency() {
        let dt = DensityTransparency::new();
        // Transparency = 1.0 - thickness
        assert!((dt.transparency(&Density::First(Density1SubLevel::Atomic)) - 0.10).abs() < 1e-9);
        assert!(
            (dt.transparency(&Density::Second(Density2SubLevel::Cellular)) - 0.25).abs() < 1e-9
        );
        assert!((dt.transparency(&Density::Third) - 0.40).abs() < 1e-9);
        assert!((dt.transparency(&Density::Fourth) - 0.55).abs() < 1e-9);
        assert!((dt.transparency(&Density::Fifth) - 0.70).abs() < 1e-9);
        assert!((dt.transparency(&Density::Sixth) - 0.85).abs() < 1e-9);
        assert_eq!(dt.transparency(&Density::Seventh), 1.00);
    }

    #[test]
    fn test_density_transparency_custom() {
        let thickness = [0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0];
        let dt = DensityTransparency::with_custom_thickness(thickness);
        assert_eq!(
            dt.base_thickness(&Density::First(Density1SubLevel::Atomic)),
            0.5
        );
        assert_eq!(dt.base_thickness(&Density::Third), 0.3);
        assert_eq!(dt.base_thickness(&Density::Seventh), 0.0);
    }

    #[test]
    fn test_density_transparency_custom_clamped() {
        let thickness = [-0.5, 0.5, 1.5, 0.5, 0.5, 0.5, 0.5];
        let dt = DensityTransparency::with_custom_thickness(thickness);
        assert_eq!(
            dt.base_thickness(&Density::First(Density1SubLevel::Atomic)),
            0.0
        ); // Clamped to 0.0
        assert_eq!(dt.base_thickness(&Density::Third), 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_density_transparency_default() {
        let dt = DensityTransparency::default();
        assert_eq!(dt.base_thickness(&Density::Third), 0.60);
    }

    #[test]
    fn test_set_thickness() {
        let mut dt = DensityTransparency::new();
        dt.set_thickness(Density::Third, 0.7);
        assert_eq!(dt.base_thickness(&Density::Third), 0.7);
    }

    #[test]
    fn test_set_thickness_clamped() {
        let mut dt = DensityTransparency::new();
        dt.set_thickness(Density::Third, -0.5);
        assert_eq!(dt.base_thickness(&Density::Third), 0.0);

        dt.set_thickness(Density::Third, 1.5);
        assert_eq!(dt.base_thickness(&Density::Third), 1.0);
    }

    #[test]
    fn test_thickness_curve() {
        let dt = DensityTransparency::new();
        let curve = dt.thickness_curve();
        assert_eq!(curve, [0.90, 0.75, 0.60, 0.45, 0.30, 0.15, 0.00]);
    }

    #[test]
    fn test_transition_thickness_start() {
        let dt = DensityTransparency::new();
        let thickness = dt.transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            0.0,
        );
        assert_eq!(thickness, dt.base_thickness(&Density::Seventh)); // 0.0
    }

    #[test]
    fn test_transition_thickness_end() {
        let dt = DensityTransparency::new();
        let thickness = dt.transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            1.0,
        );
        assert_eq!(
            thickness,
            dt.base_thickness(&Density::First(Density1SubLevel::Quantum))
        ); // 0.90
    }

    #[test]
    fn test_transition_thickness_halfway() {
        let dt = DensityTransparency::new();
        let thickness = dt.transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            0.5,
        );
        // (0.0 + 0.90) / 2 = 0.45
        assert_eq!(thickness, 0.45);
    }

    #[test]
    fn test_transition_thickness_clamped() {
        let dt = DensityTransparency::new();
        let thickness_low = dt.transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            -0.5,
        );
        assert_eq!(thickness_low, dt.base_thickness(&Density::Seventh)); // 0.0

        let thickness_high = dt.transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            1.5,
        );
        assert_eq!(
            thickness_high,
            dt.base_thickness(&Density::First(Density1SubLevel::Quantum))
        ); // 0.90
    }

    #[test]
    fn test_transition_thickness_d7_to_d6() {
        let dt = DensityTransparency::new();
        let thickness = dt.transition_thickness(Density::Seventh, Density::Sixth, 0.5);
        // (0.0 + 0.15) / 2 = 0.075
        assert!((thickness - 0.075).abs() < 0.001);
    }

    #[test]
    fn test_transition_thickness_d4_to_d3() {
        let dt = DensityTransparency::new();
        let thickness = dt.transition_thickness(Density::Fourth, Density::Third, 1.0);
        assert_eq!(thickness, dt.base_thickness(&Density::Third)); // 0.60
    }

    // ===== PolarizationAccess Tests =====

    #[test]
    fn test_polarization_access_new() {
        let pa = PolarizationAccess::new();
        assert_eq!(pa.sto_thinning_factor, 0.3);
        assert_eq!(pa.sts_thinning_factor, 0.2);
        assert_eq!(pa.neutral_thinning_factor, 0.0);
    }

    #[test]
    fn test_polarization_access_custom() {
        let pa = PolarizationAccess::with_custom_factors(0.5, 0.4, 0.1);
        assert_eq!(pa.sto_thinning_factor, 0.5);
        assert_eq!(pa.sts_thinning_factor, 0.4);
        assert_eq!(pa.neutral_thinning_factor, 0.1);
    }

    #[test]
    fn test_polarization_access_custom_clamped() {
        let pa = PolarizationAccess::with_custom_factors(1.5, -0.5, 0.5);
        assert_eq!(pa.sto_thinning_factor, 1.0); // Clamped to 1.0
        assert_eq!(pa.sts_thinning_factor, 0.0); // Clamped to 0.0
        assert_eq!(pa.neutral_thinning_factor, 0.5);
    }

    #[test]
    fn test_polarization_access_calculate_thinning_sto() {
        let pa = PolarizationAccess::new();
        let polarization = PolarizationState::sto(0.5);
        let thinning = pa.calculate_thinning(&polarization);
        assert_eq!(thinning, 0.5 * 0.3); // 0.15
    }

    #[test]
    fn test_polarization_access_calculate_thinning_sts() {
        let pa = PolarizationAccess::new();
        let polarization = PolarizationState::sts(0.5);
        let thinning = pa.calculate_thinning(&polarization);
        assert_eq!(thinning, 0.5 * 0.2); // 0.1
    }

    #[test]
    fn test_polarization_access_calculate_thinning_neutral() {
        let pa = PolarizationAccess::new();
        let polarization = PolarizationState::neutral();
        let thinning = pa.calculate_thinning(&polarization);
        assert_eq!(thinning, 0.0); // No thinning for neutral
    }

    #[test]
    fn test_polarization_access_set_sto_factor() {
        let mut pa = PolarizationAccess::new();
        pa.set_sto_factor(0.5);
        assert_eq!(pa.sto_thinning_factor, 0.5);
    }

    #[test]
    fn test_polarization_access_set_sts_factor() {
        let mut pa = PolarizationAccess::new();
        pa.set_sts_factor(0.5);
        assert_eq!(pa.sts_thinning_factor, 0.5);
    }

    #[test]
    fn test_polarization_access_set_neutral_factor() {
        let mut pa = PolarizationAccess::new();
        pa.set_neutral_factor(0.1);
        assert_eq!(pa.neutral_thinning_factor, 0.1);
    }

    #[test]
    fn test_polarization_access_set_factor_clamped() {
        let mut pa = PolarizationAccess::new();
        pa.set_sto_factor(1.5);
        assert_eq!(pa.sto_thinning_factor, 1.0);
    }

    #[test]
    fn test_polarization_access_default() {
        let pa = PolarizationAccess::default();
        assert_eq!(pa.sto_thinning_factor, 0.3);
        assert_eq!(pa.sts_thinning_factor, 0.2);
        assert_eq!(pa.neutral_thinning_factor, 0.0);
    }

    // ===== PolarizationState Tests =====

    #[test]
    fn test_polarization_state_new() {
        let state = PolarizationState::new(Polarity::STO, 0.5);
        assert_eq!(state.polarization, Polarity::STO);
        assert_eq!(state.intensity, 0.5);
    }

    #[test]
    fn test_polarization_state_new_clamped() {
        let state = PolarizationState::new(Polarity::STO, 1.5);
        assert_eq!(state.intensity, 1.0); // Clamped to 1.0

        let state = PolarizationState::new(Polarity::STO, -0.5);
        assert_eq!(state.intensity, 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_polarization_state_sto() {
        let state = PolarizationState::sto(0.7);
        assert_eq!(state.polarization, Polarity::STO);
        assert_eq!(state.intensity, 0.7);
    }

    #[test]
    fn test_polarization_state_sts() {
        let state = PolarizationState::sts(0.7);
        assert_eq!(state.polarization, Polarity::STS);
        assert_eq!(state.intensity, 0.7);
    }

    #[test]
    fn test_polarization_state_neutral() {
        let state = PolarizationState::neutral();
        assert_eq!(state.polarization, Polarity::SinkholeOfIndifference);
        assert_eq!(state.intensity, 0.0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_combined_density_and_polarization_thinning() {
        let dt = DensityTransparency::new();
        let pa = PolarizationAccess::new();

        // D3 has base thickness 0.60
        // STO polarization at 0.5 intensity thins by 0.15
        // Final thickness: 0.60 - 0.15 = 0.45
        let base_thickness = dt.base_thickness(&Density::Third);
        let polarization = PolarizationState::sto(0.5);
        let thinning = pa.calculate_thinning(&polarization);
        let final_thickness = base_thickness - thinning;

        assert_eq!(base_thickness, 0.60);
        assert_eq!(thinning, 0.15);
        assert!((final_thickness - 0.45).abs() < 1e-9);
    }

    #[test]
    fn test_sto_thins_more_than_sts() {
        let pa = PolarizationAccess::new();
        let sto_state = PolarizationState::sto(0.5);
        let sts_state = PolarizationState::sts(0.5);

        let sto_thinning = pa.calculate_thinning(&sto_state);
        let sts_thinning = pa.calculate_thinning(&sts_state);

        assert!(sto_thinning > sts_thinning);
    }

    #[test]
    fn test_higher_intensity_more_thinning() {
        let pa = PolarizationAccess::new();
        let low_intensity = PolarizationState::sto(0.3);
        let high_intensity = PolarizationState::sto(0.8);

        let low_thinning = pa.calculate_thinning(&low_intensity);
        let high_thinning = pa.calculate_thinning(&high_intensity);

        assert!(high_thinning > low_thinning);
    }
}
