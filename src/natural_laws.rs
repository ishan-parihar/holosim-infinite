// Natural Laws System - Phase 10 Implementation
// Provides parameterized physics for the physical dimension
// Each Logos can generate unique natural laws, creating different universes

use serde::{Deserialize, Serialize};
use std::fmt;

/// Float type for physical calculations
pub type Float = f64;

/// Force law types that can vary between universes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ForceLawType {
    /// Standard inverse-square law (1/r²)
    InverseSquare,
    /// Linear law (1/r)
    Linear,
    /// Exponential decay (e^(-r))
    Exponential,
    /// Custom power law (1/r^n)
    PowerLaw(Float),
}

/// Entropy law types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntropyLawType {
    /// Standard entropy increase (dS >= 0)
    AlwaysIncrease,
    /// Entropy can decrease locally
    LocalDecrease,
    /// Entropy oscillates
    Oscillating,
    /// Custom entropy behavior
    Custom,
}

/// Natural Laws - Parameterized physics for a universe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalLaws {
    // Universal constants
    /// Speed of light in vacuum (m/s)
    pub speed_of_light: Float,
    /// Gravitational constant (m³/kg·s²)
    pub gravitational_constant: Float,
    /// Planck's constant (J·s)
    pub planck_constant: Float,
    /// Fine-structure constant (dimensionless)
    pub fine_structure_constant: Float,
    /// Boltzmann constant (J/K)
    pub boltzmann_constant: Float,

    // Force laws
    pub gravitational_force_law: ForceLawType,
    pub electromagnetic_force_law: ForceLawType,
    pub strong_nuclear_force_law: ForceLawType,
    pub weak_nuclear_force_law: ForceLawType,

    // Conservation laws
    pub energy_conservation: bool,
    pub momentum_conservation: bool,
    pub charge_conservation: bool,
    pub angular_momentum_conservation: bool,

    // Thermodynamic laws
    pub entropy_law: EntropyLawType,
    pub absolute_zero: bool, // Whether absolute zero is achievable

    // Quantum mechanics
    pub quantum_mechanics_enabled: bool,
    pub uncertainty_principle: bool,
    pub superposition: bool,
    pub entanglement: bool,
    pub wave_function_collapse: bool,

    // Space-time properties
    pub spatial_dimensions: u8,
    pub temporal_dimensions: u8,
    pub spacetime_curvature: bool,
    pub causality_preserved: bool,
}

impl Default for NaturalLaws {
    fn default() -> Self {
        Self::earth_like()
    }
}

impl NaturalLaws {
    /// Create natural laws similar to Earth's universe
    pub fn earth_like() -> Self {
        NaturalLaws {
            // Universal constants (SI units)
            speed_of_light: 299_792_458.0,
            gravitational_constant: 6.67430e-11,
            planck_constant: 6.62607015e-34,
            fine_structure_constant: 1.0 / 137.035999084,
            boltzmann_constant: 1.380649e-23,

            // Force laws (standard inverse-square)
            gravitational_force_law: ForceLawType::InverseSquare,
            electromagnetic_force_law: ForceLawType::InverseSquare,
            strong_nuclear_force_law: ForceLawType::Exponential,
            weak_nuclear_force_law: ForceLawType::Exponential,

            // Conservation laws
            energy_conservation: true,
            momentum_conservation: true,
            charge_conservation: true,
            angular_momentum_conservation: true,

            // Thermodynamics
            entropy_law: EntropyLawType::AlwaysIncrease,
            absolute_zero: false,

            // Quantum mechanics
            quantum_mechanics_enabled: true,
            uncertainty_principle: true,
            superposition: true,
            entanglement: true,
            wave_function_collapse: true,

            // Space-time
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            spacetime_curvature: true,
            causality_preserved: true,
        }
    }

    /// Create natural laws with higher speed of light (less dense universe)
    pub fn high_light_universe(multiplier: Float) -> Self {
        let mut laws = Self::earth_like();
        laws.speed_of_light *= multiplier;
        laws
    }

    /// Create natural laws with stronger gravity
    pub fn high_gravity_universe(multiplier: Float) -> Self {
        let mut laws = Self::earth_like();
        laws.gravitational_constant *= multiplier;
        laws
    }

    /// Create natural laws with different spatial dimensions
    pub fn different_dimensions(spatial: u8, temporal: u8) -> Self {
        let mut laws = Self::earth_like();
        laws.spatial_dimensions = spatial;
        laws.temporal_dimensions = temporal;
        laws
    }

    /// Calculate gravitational force between two masses
    pub fn gravitational_force(&self, mass1: Float, mass2: Float, distance: Float) -> Float {
        if distance <= 0.0 {
            return 0.0;
        }

        match self.gravitational_force_law {
            ForceLawType::InverseSquare => {
                self.gravitational_constant * mass1 * mass2 / (distance * distance)
            }
            ForceLawType::Linear => self.gravitational_constant * mass1 * mass2 / distance,
            ForceLawType::Exponential => {
                self.gravitational_constant * mass1 * mass2 * (-distance).exp()
            }
            ForceLawType::PowerLaw(n) => {
                self.gravitational_constant * mass1 * mass2 / distance.powf(n)
            }
        }
    }

    /// Calculate electromagnetic force between two charges
    pub fn electromagnetic_force(&self, charge1: Float, charge2: Float, distance: Float) -> Float {
        if distance <= 0.0 {
            return 0.0;
        }

        // Coulomb's constant k = 1/(4πε₀)
        let coulomb_constant = 8.9875517923e9; // N·m²/C²

        match self.electromagnetic_force_law {
            ForceLawType::InverseSquare => {
                coulomb_constant * charge1 * charge2 / (distance * distance)
            }
            ForceLawType::Linear => coulomb_constant * charge1 * charge2 / distance,
            ForceLawType::Exponential => coulomb_constant * charge1 * charge2 * (-distance).exp(),
            ForceLawType::PowerLaw(n) => coulomb_constant * charge1 * charge2 / distance.powf(n),
        }
    }

    /// Calculate nuclear force strength at given distance
    pub fn nuclear_force(&self, force_type: NuclearForceType, distance: Float) -> Float {
        if distance <= 0.0 {
            return 0.0;
        }

        let law = match force_type {
            NuclearForceType::Strong => self.strong_nuclear_force_law,
            NuclearForceType::Weak => self.weak_nuclear_force_law,
        };

        match law {
            ForceLawType::InverseSquare => 1.0 / (distance * distance),
            ForceLawType::Linear => 1.0 / distance,
            ForceLawType::Exponential => (-distance).exp(),
            ForceLawType::PowerLaw(n) => 1.0 / distance.powf(n),
        }
    }

    /// Calculate Planck length (fundamental length scale)
    #[allow(non_snake_case)]
    pub fn planck_length(&self) -> Float {
        let G = self.gravitational_constant;
        let h_bar = self.planck_constant / (2.0 * std::f64::consts::PI);
        let c = self.speed_of_light;

        (G * h_bar / c.powi(3)).sqrt()
    }

    /// Calculate Planck time (fundamental time scale)
    #[allow(non_snake_case)]
    pub fn planck_time(&self) -> Float {
        let G = self.gravitational_constant;
        let h_bar = self.planck_constant / (2.0 * std::f64::consts::PI);
        let c = self.speed_of_light;

        (G * h_bar / c.powi(5)).sqrt()
    }

    /// Calculate Planck mass (fundamental mass scale)
    #[allow(non_snake_case)]
    pub fn planck_mass(&self) -> Float {
        let G = self.gravitational_constant;
        let h_bar = self.planck_constant / (2.0 * std::f64::consts::PI);
        let c = self.speed_of_light;

        (h_bar * c / G).sqrt()
    }

    /// Calculate Planck energy (fundamental energy scale)
    pub fn planck_energy(&self) -> Float {
        self.planck_mass() * self.speed_of_light.powi(2)
    }

    /// Check if conservation laws are respected
    pub fn check_conservation(&self, quantity: ConservationQuantity) -> bool {
        match quantity {
            ConservationQuantity::Energy => self.energy_conservation,
            ConservationQuantity::Momentum => self.momentum_conservation,
            ConservationQuantity::Charge => self.charge_conservation,
            ConservationQuantity::AngularMomentum => self.angular_momentum_conservation,
        }
    }

    /// Get total force types available
    pub fn force_type_count(&self) -> usize {
        4 // Gravitational, Electromagnetic, Strong Nuclear, Weak Nuclear
    }

    /// Validate natural laws for consistency
    pub fn validate(&self) -> Result<(), String> {
        // Check that speed of light is positive
        if self.speed_of_light <= 0.0 {
            return Err("Speed of light must be positive".to_string());
        }

        // Check that gravitational constant is positive
        if self.gravitational_constant <= 0.0 {
            return Err("Gravitational constant must be positive".to_string());
        }

        // Check that Planck's constant is positive
        if self.planck_constant <= 0.0 {
            return Err("Planck's constant must be positive".to_string());
        }

        // Check that spatial dimensions are at least 1
        if self.spatial_dimensions < 1 {
            return Err("Spatial dimensions must be at least 1".to_string());
        }

        // Check that temporal dimensions are at least 1
        if self.temporal_dimensions < 1 {
            return Err("Temporal dimensions must be at least 1".to_string());
        }

        Ok(())
    }

    /// Get a summary of natural laws
    pub fn summary(&self) -> String {
        format!(
            "Natural Laws:\n\
             - Speed of Light: {:.2e} m/s\n\
             - Gravitational Constant: {:.2e} m³/kg·s²\n\
             - Planck's Constant: {:.2e} J·s\n\
             - Fine-Structure Constant: {:.6}\n\
             - Spatial Dimensions: {}\n\
             - Temporal Dimensions: {}\n\
             - Quantum Mechanics: {}\n\
             - Conservation Laws: E={}, M={}, C={}",
            self.speed_of_light,
            self.gravitational_constant,
            self.planck_constant,
            self.fine_structure_constant,
            self.spatial_dimensions,
            self.temporal_dimensions,
            self.quantum_mechanics_enabled,
            self.energy_conservation,
            self.momentum_conservation,
            self.charge_conservation
        )
    }
}

impl fmt::Display for NaturalLaws {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Types of nuclear forces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuclearForceType {
    Strong,
    Weak,
}

/// Types of conserved quantities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConservationQuantity {
    Energy,
    Momentum,
    Charge,
    AngularMomentum,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_natural_laws() {
        let laws = NaturalLaws::default();
        assert_eq!(laws.speed_of_light, 299_792_458.0);
        assert_eq!(laws.gravitational_constant, 6.67430e-11);
        assert_eq!(laws.spatial_dimensions, 3);
        assert_eq!(laws.temporal_dimensions, 1);
    }

    #[test]
    fn test_earth_like_laws() {
        let laws = NaturalLaws::earth_like();
        assert!(laws.energy_conservation);
        assert!(laws.momentum_conservation);
        assert!(laws.charge_conservation);
        assert!(laws.quantum_mechanics_enabled);
    }

    #[test]
    fn test_high_light_universe() {
        let laws = NaturalLaws::high_light_universe(2.0);
        assert_eq!(laws.speed_of_light, 299_792_458.0 * 2.0);
    }

    #[test]
    fn test_high_gravity_universe() {
        let laws = NaturalLaws::high_gravity_universe(10.0);
        assert_eq!(laws.gravitational_constant, 6.67430e-11 * 10.0);
    }

    #[test]
    fn test_different_dimensions() {
        let laws = NaturalLaws::different_dimensions(4, 2);
        assert_eq!(laws.spatial_dimensions, 4);
        assert_eq!(laws.temporal_dimensions, 2);
    }

    #[test]
    fn test_gravitational_force_inverse_square() {
        let laws = NaturalLaws::earth_like();
        let force = laws.gravitational_force(1.0, 1.0, 1.0);
        assert_eq!(force, 6.67430e-11);
    }

    #[test]
    fn test_gravitational_force_zero_distance() {
        let laws = NaturalLaws::earth_like();
        let force = laws.gravitational_force(1.0, 1.0, 0.0);
        assert_eq!(force, 0.0);
    }

    #[test]
    fn test_electromagnetic_force() {
        let laws = NaturalLaws::earth_like();
        let force = laws.electromagnetic_force(1.0e-6, 1.0e-6, 1.0);
        assert!(force > 0.0);
    }

    #[test]
    fn test_nuclear_force_strong() {
        let laws = NaturalLaws::earth_like();
        let force = laws.nuclear_force(NuclearForceType::Strong, 1.0e-15);
        assert!(force > 0.0);
    }

    #[test]
    fn test_planck_length() {
        let laws = NaturalLaws::earth_like();
        let planck_length = laws.planck_length();
        assert!(planck_length > 0.0);
        assert!(planck_length < 1.0e-34); // Should be very small
    }

    #[test]
    fn test_planck_time() {
        let laws = NaturalLaws::earth_like();
        let planck_time = laws.planck_time();
        assert!(planck_time > 0.0);
        assert!(planck_time < 1.0e-43); // Should be very small
    }

    #[test]
    fn test_planck_mass() {
        let laws = NaturalLaws::earth_like();
        let planck_mass = laws.planck_mass();
        assert!(planck_mass > 0.0);
        assert!(planck_mass > 1.0e-8); // Should be macroscopic
    }

    #[test]
    fn test_planck_energy() {
        let laws = NaturalLaws::earth_like();
        let planck_energy = laws.planck_energy();
        assert!(planck_energy > 0.0);
    }

    #[test]
    fn test_check_conservation() {
        let laws = NaturalLaws::earth_like();
        assert!(laws.check_conservation(ConservationQuantity::Energy));
        assert!(laws.check_conservation(ConservationQuantity::Momentum));
        assert!(laws.check_conservation(ConservationQuantity::Charge));
        assert!(laws.check_conservation(ConservationQuantity::AngularMomentum));
    }

    #[test]
    fn test_force_type_count() {
        let laws = NaturalLaws::earth_like();
        assert_eq!(laws.force_type_count(), 4);
    }

    #[test]
    fn test_validate_valid_laws() {
        let laws = NaturalLaws::earth_like();
        assert!(laws.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_speed_of_light() {
        let mut laws = NaturalLaws::earth_like();
        laws.speed_of_light = -1.0;
        assert!(laws.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_gravitational_constant() {
        let mut laws = NaturalLaws::earth_like();
        laws.gravitational_constant = -1.0;
        assert!(laws.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_dimensions() {
        let mut laws = NaturalLaws::earth_like();
        laws.spatial_dimensions = 0;
        assert!(laws.validate().is_err());
    }

    #[test]
    fn test_gravitational_force_power_law() {
        let mut laws = NaturalLaws::earth_like();
        laws.gravitational_force_law = ForceLawType::PowerLaw(3.0);
        let force = laws.gravitational_force(1.0, 1.0, 2.0);
        assert!(force > 0.0);
    }

    #[test]
    fn test_electromagnetic_force_linear() {
        let mut laws = NaturalLaws::earth_like();
        laws.electromagnetic_force_law = ForceLawType::Linear;
        let force = laws.electromagnetic_force(1.0e-6, 1.0e-6, 1.0);
        assert!(force > 0.0);
    }

    #[test]
    fn test_nuclear_force_weak() {
        let laws = NaturalLaws::earth_like();
        let force = laws.nuclear_force(NuclearForceType::Weak, 1.0e-18);
        assert!(force > 0.0);
    }

    #[test]
    fn test_high_light_universe_multiplier() {
        let laws = NaturalLaws::high_light_universe(5.0);
        assert_eq!(laws.speed_of_light, 299_792_458.0 * 5.0);
        // Other constants should remain the same
        assert_eq!(laws.gravitational_constant, 6.67430e-11);
    }

    #[test]
    fn test_summary_output() {
        let laws = NaturalLaws::earth_like();
        let summary = laws.summary();
        assert!(summary.contains("Speed of Light"));
        assert!(summary.contains("Gravitational Constant"));
        assert!(summary.contains("Spatial Dimensions"));
    }

    #[test]
    fn test_display_trait() {
        let laws = NaturalLaws::earth_like();
        let display = format!("{}", laws);
        assert!(display.contains("Natural Laws"));
    }
}
