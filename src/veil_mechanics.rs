// Dynamic Veil Mechanics - Phase 6
// Models how veil thickness varies by density, entity, and spiritual development
//
// CONSTRAINT: No artificial randomness - all veil mechanics are deterministic
// based on entity capacities and states.

use crate::types::{Density, Float};

/// Veil structure that models the forgetting mechanism
/// between higher-dimensional awareness and physical consciousness
#[derive(Debug, Clone, PartialEq)]
pub struct Veil {
    // Base thickness (by density)
    base_thickness: Float, // 0.0 (no veil) to 1.0 (thick veil)

    // Entity-specific modifications
    spiritual_development: Float, // How spiritually developed (0.0 to 1.0)
    magical_personality: bool,    // Has magical personality (can thin veil)
    polarization_strength: Float, // Polarity intensity (0.0 to 1.0)

    // Current state
    current_thickness: Float,
    transparency: Float, // 1.0 - thickness
}

impl Veil {
    /// Create a new veil with default values
    pub fn new() -> Self {
        let base_thickness = 0.0;
        let spiritual_development = 0.0;
        let magical_personality = false;
        let polarization_strength = 0.0;

        let current_thickness = Self::calculate_thickness(
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
        );

        let transparency = 1.0 - current_thickness;

        Veil {
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
            current_thickness,
            transparency,
        }
    }

    /// Create a veil with specific base thickness (for a given density)
    pub fn with_base_thickness(base_thickness: Float) -> Self {
        let base_thickness = base_thickness.clamp(0.0, 1.0);
        let spiritual_development = 0.0;
        let magical_personality = false;
        let polarization_strength = 0.0;

        let current_thickness = Self::calculate_thickness(
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
        );

        let transparency = 1.0 - current_thickness;

        Veil {
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
            current_thickness,
            transparency,
        }
    }

    /// Create a veil with all parameters
    pub fn with_parameters(
        base_thickness: Float,
        spiritual_development: Float,
        magical_personality: bool,
        polarization_strength: Float,
    ) -> Self {
        let base_thickness = base_thickness.clamp(0.0, 1.0);
        let spiritual_development = spiritual_development.clamp(0.0, 1.0);
        let polarization_strength = polarization_strength.clamp(0.0, 1.0);

        let current_thickness = Self::calculate_thickness(
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
        );

        let transparency = 1.0 - current_thickness;

        Veil {
            base_thickness,
            spiritual_development,
            magical_personality,
            polarization_strength,
            current_thickness,
            transparency,
        }
    }

    /// Calculate current thickness based on all factors
    /// This is a deterministic calculation - no randomness
    fn calculate_thickness(
        base_thickness: Float,
        spiritual_development: Float,
        magical_personality: bool,
        polarization_strength: Float,
    ) -> Float {
        let mut thickness = base_thickness;

        // Modify by spiritual development (more developed = thinner veil)
        // Maximum reduction: 0.3 (when spiritual_development = 1.0)
        thickness -= spiritual_development * 0.3;

        // Modify by magical personality (can thin veil)
        // Fixed reduction: 0.2
        if magical_personality {
            thickness -= 0.2;
        }

        // Modify by polarization strength (strong polarization = thinner veil)
        // Maximum reduction: 0.1 (when polarization_strength = 1.0)
        thickness -= polarization_strength * 0.1;

        // Clamp to valid range [0.0, 1.0]
        thickness.max(0.0).min(1.0)
    }

    /// Recalculate current thickness based on current state
    pub fn calculate_current_thickness(&self) -> Float {
        Self::calculate_thickness(
            self.base_thickness,
            self.spiritual_development,
            self.magical_personality,
            self.polarization_strength,
        )
    }

    /// Thin the veil through spiritual development
    /// Increases spiritual_development by the given amount
    pub fn thin(&mut self, amount: Float) {
        self.spiritual_development += amount;
        self.spiritual_development = self.spiritual_development.clamp(0.0, 1.0);
        self.current_thickness = self.calculate_current_thickness();
        self.transparency = 1.0 - self.current_thickness;
    }

    /// Thicken the veil (during involution)
    /// Increases base_thickness by the given amount
    pub fn thicken(&mut self, amount: Float) {
        self.base_thickness += amount;
        self.base_thickness = self.base_thickness.clamp(0.0, 1.0);
        self.current_thickness = self.calculate_current_thickness();
        self.transparency = 1.0 - self.current_thickness;
    }

    /// Set magical personality (can thin veil)
    pub fn set_magical_personality(&mut self, magical: bool) {
        self.magical_personality = magical;
        self.current_thickness = self.calculate_current_thickness();
        self.transparency = 1.0 - self.current_thickness;
    }

    /// Set polarization strength
    pub fn set_polarization_strength(&mut self, strength: Float) {
        self.polarization_strength = strength.clamp(0.0, 1.0);
        self.current_thickness = self.calculate_current_thickness();
        self.transparency = 1.0 - self.current_thickness;
    }

    /// Set spiritual development directly
    pub fn set_spiritual_development(&mut self, development: Float) {
        self.spiritual_development = development.clamp(0.0, 1.0);
        self.current_thickness = self.calculate_current_thickness();
        self.transparency = 1.0 - self.current_thickness;
    }

    /// Get current thickness
    pub fn get_thickness(&self) -> Float {
        self.current_thickness
    }

    /// Get transparency (1.0 - thickness)
    pub fn get_transparency(&self) -> Float {
        self.transparency
    }

    /// Get base thickness
    pub fn get_base_thickness(&self) -> Float {
        self.base_thickness
    }

    /// Get spiritual development
    pub fn get_spiritual_development(&self) -> Float {
        self.spiritual_development
    }

    /// Get magical personality
    pub fn get_magical_personality(&self) -> bool {
        self.magical_personality
    }

    /// Get polarization strength
    pub fn get_polarization_strength(&self) -> Float {
        self.polarization_strength
    }

    /// Check if veil is completely transparent (no veil)
    pub fn is_transparent(&self) -> bool {
        self.current_thickness <= 0.001
    }

    /// Check if veil is completely opaque (thick veil)
    pub fn is_opaque(&self) -> bool {
        self.current_thickness >= 0.999
    }

    /// Get veil thickness percentage (0-100%)
    pub fn thickness_percentage(&self) -> Float {
        self.current_thickness * 100.0
    }

    /// Get transparency percentage (0-100%)
    pub fn transparency_percentage(&self) -> Float {
        self.transparency * 100.0
    }
}

impl Default for Veil {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for Density to provide base veil thickness
pub trait DensityVeilThickness {
    /// Get base veil thickness for this density
    fn base_veil_thickness(&self) -> Float;
}

impl DensityVeilThickness for Density {
    fn base_veil_thickness(&self) -> Float {
        match self {
            Density::First(Density1SubLevel::Quantum) => 0.0, // No veil (elemental consciousness)
            Density::Second(Density2SubLevel::Cellular) => 0.2, // Thin veil (plant/animal)
            Density::Third => 0.8, // Thick veil (self-conscious, choice point)
            Density::Fourth => 0.6, // Medium-thick (understanding love)
            Density::Fifth => 0.4, // Medium (wisdom density)
            Density::Sixth => 0.2, // Thin (unity density)
            Density::Seventh => 0.0, // No veil (completion, gateway)
        }
    }
}

/// Create a veil for a specific density
pub fn veil_for_density(density: Density) -> Veil {
    Veil::with_base_thickness(density.base_veil_thickness())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Basic Veil Creation Tests =====

    #[test]
    fn test_veil_new() {
        let veil = Veil::new();
        assert_eq!(veil.get_base_thickness(), 0.0);
        assert_eq!(veil.get_spiritual_development(), 0.0);
        assert_eq!(veil.get_magical_personality(), false);
        assert_eq!(veil.get_polarization_strength(), 0.0);
        assert_eq!(veil.get_thickness(), 0.0);
        assert_eq!(veil.get_transparency(), 1.0);
    }

    #[test]
    fn test_veil_with_base_thickness() {
        let veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.get_base_thickness(), 0.8);
        assert_eq!(veil.get_spiritual_development(), 0.0);
        assert_eq!(veil.get_magical_personality(), false);
        assert_eq!(veil.get_polarization_strength(), 0.0);
        assert_eq!(veil.get_thickness(), 0.8);
        assert!((veil.get_transparency() - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_veil_with_base_thickness_clamped() {
        let veil_low = Veil::with_base_thickness(-0.5);
        assert_eq!(veil_low.get_base_thickness(), 0.0);

        let veil_high = Veil::with_base_thickness(1.5);
        assert_eq!(veil_high.get_base_thickness(), 1.0);
    }

    #[test]
    fn test_veil_with_parameters() {
        let veil = Veil::with_parameters(0.8, 0.5, true, 0.7);
        assert_eq!(veil.get_base_thickness(), 0.8);
        assert_eq!(veil.get_spiritual_development(), 0.5);
        assert_eq!(veil.get_magical_personality(), true);
        assert_eq!(veil.get_polarization_strength(), 0.7);
    }

    #[test]
    fn test_veil_with_parameters_clamped() {
        let veil = Veil::with_parameters(1.5, -0.5, true, 1.5);
        assert_eq!(veil.get_base_thickness(), 1.0);
        assert_eq!(veil.get_spiritual_development(), 0.0);
        assert_eq!(veil.get_magical_personality(), true);
        assert_eq!(veil.get_polarization_strength(), 1.0);
    }

    #[test]
    fn test_veil_default() {
        let veil = Veil::default();
        assert_eq!(veil.get_base_thickness(), 0.0);
        assert_eq!(veil.get_spiritual_development(), 0.0);
        assert_eq!(veil.get_magical_personality(), false);
        assert_eq!(veil.get_polarization_strength(), 0.0);
    }

    // ===== Thickness Calculation Tests =====

    #[test]
    fn test_calculate_thickness_base_only() {
        let veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.get_thickness(), 0.8);
    }

    #[test]
    fn test_calculate_thickness_with_spiritual_development() {
        // Base thickness: 0.8
        // Spiritual development: 0.5
        // Reduction: 0.5 * 0.3 = 0.15
        // Expected thickness: 0.8 - 0.15 = 0.65
        let veil = Veil::with_parameters(0.8, 0.5, false, 0.0);
        assert!((veil.get_thickness() - 0.65).abs() < 0.001);
    }

    #[test]
    fn test_calculate_thickness_with_magical_personality() {
        // Base thickness: 0.8
        // Magical personality: true
        // Reduction: 0.2
        // Expected thickness: 0.8 - 0.2 = 0.6
        let veil = Veil::with_parameters(0.8, 0.0, true, 0.0);
        assert!((veil.get_thickness() - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_calculate_thickness_with_polarization_strength() {
        // Base thickness: 0.8
        // Polarity strength: 0.5
        // Reduction: 0.5 * 0.1 = 0.05
        // Expected thickness: 0.8 - 0.05 = 0.75
        let veil = Veil::with_parameters(0.8, 0.0, false, 0.5);
        assert!((veil.get_thickness() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_calculate_thickness_all_factors() {
        // Base thickness: 0.8
        // Spiritual development: 0.5 (reduction: 0.15)
        // Magical personality: true (reduction: 0.2)
        // Polarity strength: 0.7 (reduction: 0.07)
        // Total reduction: 0.15 + 0.2 + 0.07 = 0.42
        // Expected thickness: 0.8 - 0.42 = 0.38
        let veil = Veil::with_parameters(0.8, 0.5, true, 0.7);
        assert!((veil.get_thickness() - 0.38).abs() < 0.001);
    }

    #[test]
    fn test_calculate_thickness_clamped_to_zero() {
        // Base thickness: 0.2
        // Spiritual development: 1.0 (reduction: 0.3)
        // Magical personality: true (reduction: 0.2)
        // Polarity strength: 1.0 (reduction: 0.1)
        // Total reduction: 0.3 + 0.2 + 0.1 = 0.6
        // Expected thickness: max(0.2 - 0.6, 0.0) = 0.0
        let veil = Veil::with_parameters(0.2, 1.0, true, 1.0);
        assert_eq!(veil.get_thickness(), 0.0);
    }

    #[test]
    fn test_calculate_thickness_clamped_to_one() {
        // Base thickness: 1.0
        // No reductions
        // Expected thickness: 1.0
        let veil = Veil::with_parameters(1.0, 0.0, false, 0.0);
        assert_eq!(veil.get_thickness(), 1.0);
    }

    // ===== Transparency Tests =====

    #[test]
    fn test_transparency_calculated_correctly() {
        let veil = Veil::with_base_thickness(0.8);
        assert!((veil.get_transparency() - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_transparency_updates_with_thickness() {
        let mut veil = Veil::with_base_thickness(0.8);
        assert!((veil.get_transparency() - 0.2).abs() < 0.001);

        veil.thin(0.3);
        // Base: 0.8, Spiritual: 0.3, Reduction: 0.3 * 0.3 = 0.09
        // Thickness: 0.8 - 0.09 = 0.71
        // Transparency: 1.0 - 0.71 = 0.29
        assert!((veil.get_transparency() - 0.29).abs() < 0.001);
    }

    #[test]
    fn test_is_transparent() {
        let veil = Veil::with_base_thickness(0.0);
        assert!(veil.is_transparent());

        let veil = Veil::with_base_thickness(0.001);
        assert!(veil.is_transparent());

        let veil = Veil::with_base_thickness(0.1);
        assert!(!veil.is_transparent());
    }

    #[test]
    fn test_is_opaque() {
        let veil = Veil::with_base_thickness(1.0);
        assert!(veil.is_opaque());

        let veil = Veil::with_base_thickness(0.999);
        assert!(veil.is_opaque());

        let veil = Veil::with_base_thickness(0.9);
        assert!(!veil.is_opaque());
    }

    // ===== Thin Method Tests =====

    #[test]
    fn test_thin_increases_spiritual_development() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.thin(0.3);
        assert_eq!(veil.get_spiritual_development(), 0.3);
    }

    #[test]
    fn test_thin_reduces_thickness() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.thin(0.3);
        // Base: 0.8, Spiritual: 0.3, Reduction: 0.3 * 0.3 = 0.09
        // Expected: 0.8 - 0.09 = 0.71
        assert!((veil.get_thickness() - 0.71).abs() < 0.001);
    }

    #[test]
    fn test_thin_clamped_to_one() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.thin(1.5);
        assert_eq!(veil.get_spiritual_development(), 1.0);
    }

    #[test]
    fn test_thin_multiple_times() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.thin(0.2);
        veil.thin(0.3);
        veil.thin(0.1);
        assert_eq!(veil.get_spiritual_development(), 0.6);
    }

    // ===== Thicken Method Tests =====

    #[test]
    fn test_thicken_increases_base_thickness() {
        let mut veil = Veil::with_base_thickness(0.5);
        veil.thicken(0.3);
        assert_eq!(veil.get_base_thickness(), 0.8);
    }

    #[test]
    fn test_thicken_increases_thickness() {
        let mut veil = Veil::with_base_thickness(0.5);
        veil.thicken(0.3);
        assert_eq!(veil.get_thickness(), 0.8);
    }

    #[test]
    fn test_thicken_clamped_to_one() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.thicken(0.5);
        assert_eq!(veil.get_base_thickness(), 1.0);
    }

    #[test]
    fn test_thicken_with_reductions_still_affects() {
        let mut veil = Veil::with_parameters(0.5, 0.5, true, 0.5);
        veil.thicken(0.3);
        // Base: 0.8, Spiritual: 0.5, Magical: true, Polarity: 0.5
        // Reductions: 0.5 * 0.3 = 0.15, 0.2, 0.5 * 0.1 = 0.05
        // Total reduction: 0.4
        // Expected: 0.8 - 0.4 = 0.4
        assert!((veil.get_thickness() - 0.4).abs() < 0.001);
    }

    // ===== Setter Method Tests =====

    #[test]
    fn test_set_magical_personality() {
        let mut veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.get_thickness(), 0.8);

        veil.set_magical_personality(true);
        assert_eq!(veil.get_magical_personality(), true);
        assert!((veil.get_thickness() - 0.6).abs() < 0.001);

        veil.set_magical_personality(false);
        assert_eq!(veil.get_magical_personality(), false);
        assert_eq!(veil.get_thickness(), 0.8);
    }

    #[test]
    fn test_set_polarization_strength() {
        let mut veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.get_thickness(), 0.8);

        veil.set_polarization_strength(0.5);
        assert_eq!(veil.get_polarization_strength(), 0.5);
        assert!((veil.get_thickness() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_set_polarization_strength_clamped() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.set_polarization_strength(1.5);
        assert_eq!(veil.get_polarization_strength(), 1.0);
    }

    #[test]
    fn test_set_spiritual_development() {
        let mut veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.get_thickness(), 0.8);

        veil.set_spiritual_development(0.5);
        assert_eq!(veil.get_spiritual_development(), 0.5);
        assert!((veil.get_thickness() - 0.65).abs() < 0.001);
    }

    #[test]
    fn test_set_spiritual_development_clamped() {
        let mut veil = Veil::with_base_thickness(0.8);
        veil.set_spiritual_development(1.5);
        assert_eq!(veil.get_spiritual_development(), 1.0);
    }

    // ===== Density Veil Thickness Tests =====

    #[test]
    fn test_density_d1_veil_thickness() {
        let thickness = Density::First(Density1SubLevel::Quantum).base_veil_thickness();
        assert_eq!(thickness, 0.0);
    }

    #[test]
    fn test_density_d2_veil_thickness() {
        let thickness = Density::Second(Density2SubLevel::Cellular).base_veil_thickness();
        assert_eq!(thickness, 0.2);
    }

    #[test]
    fn test_density_d3_veil_thickness() {
        let thickness = Density::Third.base_veil_thickness();
        assert_eq!(thickness, 0.8);
    }

    #[test]
    fn test_density_d4_veil_thickness() {
        let thickness = Density::Fourth.base_veil_thickness();
        assert_eq!(thickness, 0.6);
    }

    #[test]
    fn test_density_d5_veil_thickness() {
        let thickness = Density::Fifth.base_veil_thickness();
        assert_eq!(thickness, 0.4);
    }

    #[test]
    fn test_density_d6_veil_thickness() {
        let thickness = Density::Sixth.base_veil_thickness();
        assert_eq!(thickness, 0.2);
    }

    #[test]
    fn test_density_d7_veil_thickness() {
        let thickness = Density::Seventh.base_veil_thickness();
        assert_eq!(thickness, 0.0);
    }

    #[test]
    fn test_veil_for_density() {
        let veil = veil_for_density(Density::Third);
        assert_eq!(veil.get_base_thickness(), 0.8);
        assert_eq!(veil.get_thickness(), 0.8);
    }

    #[test]
    fn test_veil_for_density_d7() {
        let veil = veil_for_density(Density::Seventh);
        assert_eq!(veil.get_base_thickness(), 0.0);
        assert_eq!(veil.get_thickness(), 0.0);
        assert!(veil.is_transparent());
    }

    // ===== Percentage Tests =====

    #[test]
    fn test_thickness_percentage() {
        let veil = Veil::with_base_thickness(0.8);
        assert_eq!(veil.thickness_percentage(), 80.0);
    }

    #[test]
    fn test_transparency_percentage() {
        let veil = Veil::with_base_thickness(0.8);
        assert!((veil.transparency_percentage() - 20.0).abs() < 0.001);
    }

    // ===== Deterministic Behavior Tests =====

    #[test]
    fn test_deterministic_thickness_calculation() {
        // Same inputs should always produce the same output
        let veil1 = Veil::with_parameters(0.8, 0.5, true, 0.7);
        let veil2 = Veil::with_parameters(0.8, 0.5, true, 0.7);

        assert_eq!(veil1.get_thickness(), veil2.get_thickness());
        assert_eq!(veil1.get_transparency(), veil2.get_transparency());
    }

    #[test]
    fn test_deterministic_thin() {
        // Same thin operations should produce the same result
        let mut veil1 = Veil::with_base_thickness(0.8);
        let mut veil2 = Veil::with_base_thickness(0.8);

        veil1.thin(0.3);
        veil2.thin(0.3);

        assert_eq!(veil1.get_thickness(), veil2.get_thickness());
        assert_eq!(
            veil1.get_spiritual_development(),
            veil2.get_spiritual_development()
        );
    }

    #[test]
    fn test_deterministic_thicken() {
        // Same thicken operations should produce the same result
        let mut veil1 = Veil::with_base_thickness(0.5);
        let mut veil2 = Veil::with_base_thickness(0.5);

        veil1.thicken(0.3);
        veil2.thicken(0.3);

        assert_eq!(veil1.get_thickness(), veil2.get_thickness());
        assert_eq!(veil1.get_base_thickness(), veil2.get_base_thickness());
    }

    // ===== Complex Scenario Tests =====

    #[test]
    fn test_complex_veil_evolution() {
        // Simulate a veil evolving through spiritual development
        let mut veil = veil_for_density(Density::Third); // Base: 0.8

        // Initial state
        assert_eq!(veil.get_thickness(), 0.8);

        // Develop spiritually
        veil.thin(0.3);
        assert!((veil.get_thickness() - 0.71).abs() < 0.001);

        // Gain magical personality
        veil.set_magical_personality(true);
        assert!((veil.get_thickness() - 0.51).abs() < 0.001);

        // Polarize strongly
        veil.set_polarization_strength(0.8);
        assert!((veil.get_thickness() - 0.43).abs() < 0.001);

        // More spiritual development
        veil.thin(0.4);
        assert!((veil.get_thickness() - 0.31).abs() < 0.001);

        // Final state: veil has thinned from 0.8 to 0.31
    }

    #[test]
    fn test_involution_thickening_veil() {
        // Simulate involution thickening the veil
        let mut veil = veil_for_density(Density::Third); // Base: 0.8

        // Thin the veil through spiritual development
        veil.thin(0.5);
        assert!((veil.get_thickness() - 0.65).abs() < 0.001);

        // Involution thickens the veil
        veil.thicken(0.3);
        // Base becomes 1.1, clamped to 1.0
        // Spiritual = 0.5, reduction = 0.5 * 0.3 = 0.15
        // Thickness = 1.0 - 0.15 = 0.85
        assert!((veil.get_thickness() - 0.85).abs() < 0.001);

        // Not clamped to 1.0 yet
        assert!((veil.get_thickness() - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_veil_clone() {
        let veil1 = Veil::with_parameters(0.8, 0.5, true, 0.7);
        let veil2 = veil1.clone();

        assert_eq!(veil1, veil2);
    }

    #[test]
    fn test_veil_partial_eq() {
        let veil1 = Veil::with_parameters(0.8, 0.5, true, 0.7);
        let veil2 = Veil::with_parameters(0.8, 0.5, true, 0.7);
        let veil3 = Veil::with_parameters(0.8, 0.5, true, 0.6);

        assert_eq!(veil1, veil2);
        assert_ne!(veil1, veil3);
    }

    #[test]
    fn test_veil_debug() {
        let veil = Veil::with_parameters(0.8, 0.5, true, 0.7);
        let debug_str = format!("{:?}", veil);
        assert!(debug_str.contains("Veil"));
    }
}
