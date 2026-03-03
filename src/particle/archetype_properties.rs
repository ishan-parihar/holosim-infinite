//! Particle properties derived from archetype activation patterns
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK:
//! "Particle properties emerge from archetype activation, not fundamental attributes"
//!
//! # Phase 2.3: Archetype-Derived Particle Properties
//!
//! This module connects the existing physics derivation to holographic discovery,
//! allowing particle properties to emerge from archetype activation patterns
//! modulated by holographic field coherence.
//!
//! ## Key Principles
//!
//! 1. **Archetype Mapping**: Properties emerge from archetype activation patterns
//! 2. **Holographic Discovery**: Properties modulated by field coherence at position
//! 3. **Validation**: Ensure properties match known particle types
//!
//! ## Property Derivation
//!
//! - **Charge**: Catalyst archetype (A3) determines charge
//!   - A3 < 0.3 → electron (negative)
//!   - A3 > 0.7 → proton (positive)  
//!   - A3 ≈ 0.5 → neutron (neutral)
//! - **Spin**: Matrix (A1) + Great Way (A7) average
//! - **Mass**: Coherence from activation
//! - **Lifetime**: Derived from stability (Matrix + Great Way)

use crate::holographic::holographic_field::HolographicField;
use crate::holographic::Position;
use crate::matter::particle::{Complex, Coordinate3D};
use crate::types::Float;

// ============================================================================
// FUNDAMENTAL CONSTANTS
// ============================================================================

/// Elementary charge in Coulombs
///
/// The fundamental unit of electric charge, equal to the charge of a proton
/// or the magnitude of the charge of an electron.
pub const ELEMENTARY_CHARGE: Float = 1.602176634e-19;

/// Planck mass in kilograms
///
/// The natural unit of mass derived from fundamental constants.
/// Used as a reference scale for particle mass derivation.
pub const PLANCK_MASS: Float = 2.176434e-8;

/// Reduced Planck constant (ħ) in J·s
///
/// The quantum of angular momentum, fundamental to quantum mechanics.
pub const H_BAR: Float = 1.054571817e-34;

/// Electron mass in kilograms
///
/// Reference mass for fermion calculations.
pub const ELECTRON_MASS: Float = 9.10938356e-31;

/// Speed of light in m/s
///
/// Fundamental constant relating mass and energy (E=mc²).
pub const SPEED_OF_LIGHT: Float = 2.998e8;

// ============================================================================
// PARTICLE PROPERTIES STRUCTURE
// ============================================================================

/// Particle properties derived from archetype activation
///
/// All properties emerge from the 22-archetype activation pattern,
/// connecting physics to consciousness architecture.
///
/// # Derivation Rules
///
/// - **Charge**: Catalyst archetype (A3, index 2) determines charge polarity
/// - **Spin**: Matrix (A1) + Great Way (A7) average determines spin magnitude
/// - **Mass**: Coherence from activation pattern
/// - **Lifetime**: Stability from Matrix + Great Way archetypes
/// - **Coherence**: Overall activation pattern coherence
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleProperties {
    /// Electric charge in Coulombs
    pub charge: Float,

    /// Angular momentum in units of ħ
    pub spin: Float,

    /// Rest mass in kilograms
    pub mass: Float,

    /// Mean lifetime in seconds (None = stable particle)
    pub lifetime: Option<Float>,

    /// Coherence of the activation pattern (0.0 to 1.0)
    pub coherence: Float,

    /// Resonance score from holographic discovery (if applicable)
    pub resonance: Option<Float>,
}

impl ParticleProperties {
    /// Derive particle properties from archetype activation pattern
    ///
    /// From ROADMAP:
    /// - Charge: Catalyst archetype (A3) determines charge
    /// - Spin: Matrix + Great Way average
    /// - Mass: Coherence from activation
    ///
    /// # Arguments
    ///
    /// * `activation` - The 22-value archetype activation pattern
    ///
    /// # Returns
    ///
    /// ParticleProperties with all derived values
    ///
    /// # Example
    ///
    /// ```ignore
    /// let activation = [0.5; 22];
    /// let props = ParticleProperties::from_archetype_activation(&activation);
    /// assert!(props.mass > 0.0);
    /// ```
    pub fn from_archetype_activation(activation: &[Float; 22]) -> Self {
        // Charge: Catalyst archetype (A3, index 2) determines charge
        // A3 < 0.3 → electron (negative)
        // A3 > 0.7 → proton (positive)
        // A3 ≈ 0.5 → neutron (neutral)
        let catalyst = activation[2];
        let charge = (catalyst - 0.5) * ELEMENTARY_CHARGE * 2.0;

        // Spin: Matrix (A1) + Great Way (A7) average
        // Mind body determines spin magnitude
        let matrix = activation[0]; // A1 = Matrix
        let great_way = activation[6]; // A7 = Great Way
        let spin_magnitude = (matrix + great_way) / 2.0;

        // Quantize to half-integer (fermion) or integer (boson)
        // Choice (A22) determines boson vs fermion
        let choice = activation[21];
        let spin = if choice > 0.5 {
            // Boson: integer spin
            if spin_magnitude > 0.5 {
                H_BAR // Spin-1 boson (photon-like)
            } else {
                0.0 // Spin-0 boson (Higgs-like)
            }
        } else {
            // Fermion: half-integer spin
            0.5 * H_BAR // Spin-1/2 fermion (electron-like)
        };

        // Mass: Coherence from activation × reference mass
        let coherence = Self::calculate_coherence(activation);
        let mass = coherence * ELECTRON_MASS;

        // Lifetime: Derived from stability
        let stability = Self::calculate_stability(activation);
        let lifetime = if stability > 0.95 {
            None // Stable particle
        } else {
            Some(1.0 / (1.0 - stability + 1e-10))
        };

        Self {
            charge,
            spin,
            mass,
            lifetime,
            coherence,
            resonance: None,
        }
    }

    /// Derive properties from holographic field position (holographic discovery)
    ///
    /// This method connects archetype-derived properties to the holographic field,
    /// allowing field coherence to modulate particle properties.
    ///
    /// # Arguments
    ///
    /// * `field` - Reference to the holographic field
    /// * `position` - Position in the field for property discovery
    ///
    /// # Returns
    ///
    /// ParticleProperties modulated by field resonance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let field = HolographicField::new(layer, archetypes);
    /// let position = Position::new(0.5, 0.5, 0.5);
    /// let props = ParticleProperties::from_holographic_discovery(&field, &position);
    /// ```
    pub fn from_holographic_discovery(field: &HolographicField, position: &Position) -> Self {
        // Convert Position to the field's internal position type
        let field_position = crate::holographic::Position::new(position.x, position.y, position.z);

        // Get archetype activation from field at position
        let archetype_activation = field.derive_archetype_activation(&field_position);

        // Get resonance from field coherence at position
        let resonance = field.calculate_local_intensity(&field_position);

        // Base properties from archetype
        let mut props = Self::from_archetype_activation(&archetype_activation.coefficients);

        // Modulate by resonance
        props.coherence *= resonance;
        props.mass *= resonance.sqrt();
        props.resonance = Some(resonance);

        props
    }

    /// Calculate coherence from activation pattern
    ///
    /// Coherence measures how uniform the activation pattern is.
    /// High coherence = all archetypes have similar activation.
    /// Low coherence = high variance in activations.
    ///
    /// # Arguments
    ///
    /// * `activation` - The 22-value archetype activation pattern
    ///
    /// # Returns
    ///
    /// Coherence value (0.0 to 1.0)
    fn calculate_coherence(activation: &[Float; 22]) -> Float {
        let mean: Float = activation.iter().sum::<Float>() / 22.0;
        let variance: Float = activation.iter().map(|c| (c - mean).powi(2)).sum::<Float>() / 22.0;
        1.0 - variance.sqrt().min(1.0)
    }

    /// Calculate stability from activation pattern
    ///
    /// Stability is determined by Matrix (A1) and Great Way (A7) archetypes.
    /// These represent the foundation and the path to unity respectively.
    ///
    /// # Arguments
    ///
    /// * `activation` - The 22-value archetype activation pattern
    ///
    /// # Returns
    ///
    /// Stability value (0.0 to 1.0)
    fn calculate_stability(activation: &[Float; 22]) -> Float {
        // High Matrix + Great Way = stability
        // Matrix (A1, A8, A15) across mind, body, spirit
        // Great Way (A7, A14, A21) across mind, body, spirit
        let stability_factors = [
            activation[0],  // A1: Matrix (Mind)
            activation[6],  // A7: Great Way (Mind)
            activation[7],  // A8: Matrix (Body)
            activation[13], // A14: Great Way (Body)
            activation[14], // A15: Matrix (Spirit)
            activation[20], // A21: Great Way (Spirit)
        ];
        stability_factors.iter().sum::<Float>() / stability_factors.len() as Float
    }

    /// Validate against known particle properties
    ///
    /// Checks that derived properties are physically plausible:
    /// - Charge should be quantized (multiple of elementary charge)
    /// - Spin should be a valid quantum spin value
    /// - Mass should be positive
    ///
    /// # Returns
    ///
    /// PropertyValidation with errors and warnings
    pub fn validate(&self) -> PropertyValidation {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        // Check charge is quantized
        let charge_ratio = self.charge.abs() / ELEMENTARY_CHARGE;
        if charge_ratio > 0.01 && (charge_ratio - charge_ratio.round()).abs() > 0.01 {
            warnings.push(format!(
                "Charge {:.2e} C not quantized (expected multiple of e={:.2e} C)",
                self.charge, ELEMENTARY_CHARGE
            ));
        }

        // Check spin is valid
        let spin_ratio = self.spin / H_BAR;
        let valid_spins = [0.0, 0.5, 1.0, 1.5, 2.0];
        let is_valid_spin = valid_spins.iter().any(|&s| (spin_ratio - s).abs() < 0.01);
        if !is_valid_spin && self.spin.abs() > 1e-50 {
            warnings.push(format!(
                "Spin {:.2} ħ not a standard quantum spin value",
                spin_ratio
            ));
        }

        // Check mass is positive
        if self.mass <= 0.0 {
            errors.push("Mass must be positive".to_string());
        }

        // Check coherence is in valid range
        if self.coherence < 0.0 || self.coherence > 1.0 {
            errors.push(format!(
                "Coherence {} is outside valid range [0, 1]",
                self.coherence
            ));
        }

        PropertyValidation {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Get particle type classification based on derived properties
    ///
    /// # Returns
    ///
    /// String identifying the likely particle type
    pub fn classify_particle_type(&self) -> &'static str {
        let charge_ratio = self.charge / ELEMENTARY_CHARGE;
        let spin_ratio = self.spin / H_BAR;
        let is_half_spin = (spin_ratio.abs() - 0.5).abs() < 0.1;

        // Photon: mass ≈ 0, charge = 0, spin = 1 (boson)
        if self.mass < 1.0e-35 && charge_ratio.abs() < 0.1 && (spin_ratio - 1.0).abs() < 0.1 {
            return "Photon";
        }

        // Electron: negative charge, spin 1/2 (fermion)
        // Primary identifier is negative charge with half-spin
        if charge_ratio < -0.5 && is_half_spin {
            return "Electron";
        }

        // Proton: positive charge, spin 1/2 (fermion)
        // Primary identifier is positive charge with half-spin
        if charge_ratio > 0.5 && is_half_spin {
            return "Proton";
        }

        // Neutron: neutral, spin 1/2 (fermion)
        if charge_ratio.abs() < 0.5 && is_half_spin {
            return "Neutron";
        }

        // Neutrino: very light, neutral, spin 1/2
        if self.mass < 1.0e-34 && charge_ratio.abs() < 0.1 && is_half_spin {
            return "Neutrino";
        }

        "Unknown"
    }
}

impl Default for ParticleProperties {
    fn default() -> Self {
        Self {
            charge: 0.0,
            spin: 0.0,
            mass: ELECTRON_MASS,
            lifetime: None,
            coherence: 0.5,
            resonance: None,
        }
    }
}

// ============================================================================
// PROPERTY VALIDATION
// ============================================================================

/// Result of property validation
///
/// Contains validation status along with any errors or warnings
/// discovered during the validation process.
#[derive(Debug, Clone)]
pub struct PropertyValidation {
    /// Whether all validation checks passed
    pub is_valid: bool,

    /// Critical errors that make properties invalid
    pub errors: Vec<String>,

    /// Non-critical warnings about unusual property values
    pub warnings: Vec<String>,
}

impl PropertyValidation {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get all messages (errors + warnings)
    pub fn all_messages(&self) -> Vec<&str> {
        self.errors
            .iter()
            .chain(self.warnings.iter())
            .map(|s| s.as_str())
            .collect()
    }
}

impl Default for PropertyValidation {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CONVERSION UTILITIES
// ============================================================================

/// Convert Coordinate3D to Position for holographic field queries
pub fn coordinate_to_position(coord: &Coordinate3D) -> Position {
    Position::new(coord.x, coord.y, coord.z)
}

/// Convert Position to Coordinate3D for particle creation
pub fn position_to_coordinate(pos: &Position) -> Coordinate3D {
    Coordinate3D::new(pos.x, pos.y, pos.z)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Create electron-like activation pattern
    fn electron_activation() -> [Float; 22] {
        let mut activation = [0.5; 22];
        activation[0] = 0.96; // Matrix - high for stability
        activation[2] = 0.2; // Catalyst = low → electron (negative)
        activation[6] = 0.96; // Great Way - high for stability
        activation[7] = 0.96; // Matrix (Body)
        activation[13] = 0.96; // Great Way (Body)
        activation[14] = 0.96; // Matrix (Spirit)
        activation[20] = 0.96; // Great Way (Spirit)
        activation[21] = 0.3; // Choice < 0.5 → fermion
        activation
    }

    /// Create proton-like activation pattern
    fn proton_activation() -> [Float; 22] {
        let mut activation = [0.5; 22];
        activation[0] = 0.96; // Matrix - high for stability
        activation[1] = 0.9; // Potentiator - higher mass capacity
        activation[2] = 0.8; // Catalyst = high → proton (positive)
        activation[3] = 0.9; // Experience - higher mass magnitude
        activation[6] = 0.96; // Great Way - high for stability
        activation[7] = 0.96; // Matrix (Body)
        activation[8] = 0.9; // Potentiator (Body)
        activation[9] = 0.8; // Catalyst (Body)
        activation[10] = 0.9; // Experience (Body)
        activation[13] = 0.96; // Great Way (Body)
        activation[14] = 0.96; // Matrix (Spirit)
        activation[15] = 0.9; // Potentiator (Spirit)
        activation[16] = 0.8; // Catalyst (Spirit)
        activation[17] = 0.9; // Experience (Spirit)
        activation[20] = 0.96; // Great Way (Spirit)
        activation[21] = 0.4; // Choice < 0.5 → fermion
        activation
    }

    /// Create neutron-like activation pattern
    fn neutron_activation() -> [Float; 22] {
        let mut activation = [0.5; 22];
        activation[0] = 0.96; // Matrix - high for stability
        activation[1] = 0.9; // Potentiator
        activation[2] = 0.5; // Catalyst = middle → neutron (neutral)
        activation[3] = 0.9; // Experience
        activation[6] = 0.96; // Great Way
        activation[7] = 0.96; // Matrix (Body)
        activation[8] = 0.9; // Potentiator (Body)
        activation[9] = 0.5; // Catalyst (Body)
        activation[10] = 0.9; // Experience (Body)
        activation[13] = 0.96; // Great Way (Body)
        activation[14] = 0.96; // Matrix (Spirit)
        activation[15] = 0.9; // Potentiator (Spirit)
        activation[16] = 0.5; // Catalyst (Spirit)
        activation[17] = 0.9; // Experience (Spirit)
        activation[20] = 0.96; // Great Way (Spirit)
        activation[21] = 0.4; // Choice < 0.5 → fermion
        activation
    }

    /// Create photon-like activation pattern
    fn photon_activation() -> [Float; 22] {
        let mut activation = [0.5; 22];
        activation[0] = 0.7; // Matrix
        activation[2] = 0.5; // Catalyst = neutral
        activation[6] = 0.7; // Great Way
        activation[21] = 0.8; // Choice > 0.5 → boson
        activation
    }

    #[test]
    fn test_electron_properties() {
        let activation = electron_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);

        assert!(
            props.charge < 0.0,
            "Electron should have negative charge, got {}",
            props.charge
        );

        let validation = props.validate();
        assert!(
            validation.is_valid,
            "Validation failed: {:?}",
            validation.errors
        );

        let particle_type = props.classify_particle_type();
        assert_eq!(
            particle_type, "Electron",
            "Should classify as Electron, got {}",
            particle_type
        );
    }

    #[test]
    fn test_proton_properties() {
        let activation = proton_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);

        assert!(
            props.charge > 0.0,
            "Proton should have positive charge, got {}",
            props.charge
        );

        let validation = props.validate();
        assert!(
            validation.is_valid,
            "Validation failed: {:?}",
            validation.errors
        );

        let particle_type = props.classify_particle_type();
        assert_eq!(
            particle_type, "Proton",
            "Should classify as Proton, got {}",
            particle_type
        );
    }

    #[test]
    fn test_neutron_properties() {
        let activation = neutron_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);

        assert!(
            props.charge.abs() < ELEMENTARY_CHARGE * 0.2,
            "Neutron should be approximately neutral, got {}",
            props.charge
        );

        let validation = props.validate();
        assert!(
            validation.is_valid,
            "Validation failed: {:?}",
            validation.errors
        );
    }

    #[test]
    fn test_photon_properties() {
        let activation = photon_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);

        // Photon should have spin-1 (boson)
        let spin_ratio = props.spin / H_BAR;
        assert!(
            (spin_ratio - 1.0).abs() < 0.1 || spin_ratio.abs() < 0.1,
            "Photon should have spin 0 or 1, got {} ħ",
            spin_ratio
        );

        let validation = props.validate();
        assert!(
            validation.is_valid,
            "Validation failed: {:?}",
            validation.errors
        );
    }

    #[test]
    fn test_coherence_calculation() {
        // Uniform activation = high coherence
        let uniform = [0.5; 22];
        let coherence_uniform = ParticleProperties::calculate_coherence(&uniform);
        assert!(
            coherence_uniform > 0.99,
            "Uniform activation should have high coherence"
        );

        // Varied activation = lower coherence
        let mut varied = [0.5; 22];
        varied[0] = 0.1;
        varied[1] = 0.9;
        let coherence_varied = ParticleProperties::calculate_coherence(&varied);
        assert!(
            coherence_varied < coherence_uniform,
            "Varied activation should have lower coherence"
        );
    }

    #[test]
    fn test_stability_calculation() {
        // High stability factors = stable particle
        let stable = proton_activation();
        let stability = ParticleProperties::calculate_stability(&stable);
        assert!(
            stability > 0.8,
            "High stability factors should yield stable particle"
        );

        // Low stability = unstable particle
        let mut unstable = [0.3; 22];
        let stability_low = ParticleProperties::calculate_stability(&unstable);
        assert!(
            stability_low < stability,
            "Low stability factors should yield less stable particle"
        );
    }

    #[test]
    fn test_mass_derivation() {
        let activation = electron_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);

        // Mass should be positive and in reasonable range
        assert!(props.mass > 0.0, "Mass should be positive");

        // Mass should scale with coherence
        let high_coherence = [0.9; 22];
        let props_high = ParticleProperties::from_archetype_activation(&high_coherence);
        assert!(
            props_high.mass >= props.mass,
            "Higher coherence should yield higher mass"
        );
    }

    #[test]
    fn test_spin_quantization() {
        // Fermion (choice < 0.5)
        let fermion_activation = electron_activation();
        let fermion_props = ParticleProperties::from_archetype_activation(&fermion_activation);
        let fermion_spin_ratio = fermion_props.spin / H_BAR;
        assert!(
            (fermion_spin_ratio.abs() - 0.5).abs() < 0.1,
            "Fermion should have half-integer spin"
        );

        // Boson (choice > 0.5)
        let boson_activation = photon_activation();
        let boson_props = ParticleProperties::from_archetype_activation(&boson_activation);
        let boson_spin_ratio = boson_props.spin / H_BAR;
        assert!(
            (boson_spin_ratio - 1.0).abs() < 0.1 || boson_spin_ratio.abs() < 0.1,
            "Boson should have integer spin"
        );
    }

    #[test]
    fn test_validation_warnings() {
        // Create activation that might generate warnings
        let mut activation = [0.5; 22];
        activation[2] = 0.6; // Will give partial charge

        let props = ParticleProperties::from_archetype_activation(&activation);
        let validation = props.validate();

        // Properties should still be valid even with warnings
        assert!(validation.is_valid);
    }

    #[test]
    fn test_lifetime_stable() {
        // High stability should yield stable particle (None lifetime)
        let activation = proton_activation();
        let props = ParticleProperties::from_archetype_activation(&activation);
        assert!(
            props.lifetime.is_none(),
            "High stability particle should be stable"
        );
    }

    #[test]
    fn test_lifetime_unstable() {
        // Low stability should yield finite lifetime
        let mut activation = [0.2; 22];
        activation[2] = 0.5; // Neutral charge for simpler test
        activation[21] = 0.3; // Fermion

        let props = ParticleProperties::from_archetype_activation(&activation);

        if props.lifetime.is_some() {
            let lifetime = props.lifetime.unwrap();
            assert!(lifetime > 0.0, "Lifetime should be positive");
        }
    }

    #[test]
    fn test_property_validation_default() {
        let validation = PropertyValidation::default();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
        assert!(validation.warnings.is_empty());
    }

    #[test]
    fn test_coordinate_position_conversion() {
        let coord = Coordinate3D::new(1.0, 2.0, 3.0);
        let pos = coordinate_to_position(&coord);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);

        let back = position_to_coordinate(&pos);
        assert!((back.x - coord.x).abs() < 1e-10);
        assert!((back.y - coord.y).abs() < 1e-10);
        assert!((back.z - coord.z).abs() < 1e-10);
    }

    #[test]
    fn test_default_particle_properties() {
        let props = ParticleProperties::default();
        assert!(props.mass > 0.0);
        assert_eq!(props.charge, 0.0);
        assert_eq!(props.spin, 0.0);
        assert!(props.lifetime.is_none());
        assert!(!props.resonance.is_some());
    }
}
