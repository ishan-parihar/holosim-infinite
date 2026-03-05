// Archetype Patterns Module
//
// This module implements Logos' archetype choice patterns that determine
// how the 22 archetypes manifest to produce known particles.
//
// Knowledge Base Reference:
// - REFACTOR_ROADMAP_V2.md Section 2.2 - The Bridge: Archetype Activation Patterns
// - ARCHITYPE_MAPPINGS.md - All derivation formulas
//
// Phase 1: Implementation of archetype activation patterns for known particles

use crate::types::Float;

/// Archetype activation patterns chosen by Logos
///
/// This structure contains the archetype activation patterns that Logos chose
/// to create the known particles in the Standard Model. Each pattern is a
/// 22-value array representing the activation level (0.0 to 1.0) of each archetype.
///
/// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.2
/// "Archetype Activation Patterns - Logos chose specific patterns that produce known particles"
#[derive(Debug, Clone)]
pub struct ArchetypePatterns {
    /// Electron pattern
    ///
    /// The electron is a light, negatively charged particle with spin 1/2.
    /// It has a mass of 9.10938356e-31 kg.
    pub electron: [Float; 22],

    /// Proton pattern
    ///
    /// The proton is a heavy, positively charged particle with spin 1/2.
    /// It has a mass of 1.6726219e-27 kg.
    pub proton: [Float; 22],

    /// Neutron pattern
    ///
    /// The neutron is a heavy, neutral particle with spin 1/2.
    /// It has a mass of 1.67492749804e-27 kg.
    pub neutron: [Float; 22],

    /// Photon pattern (pure light)
    ///
    /// The photon is massless and chargeless, with spin 1.
    pub photon: [Float; 22],

    /// Up quark pattern
    ///
    /// The up quark has charge +2/3 and spin 1/2.
    pub up_quark: [Float; 22],

    /// Down quark pattern
    ///
    /// The down quark has charge -1/3 and spin 1/2.
    pub down_quark: [Float; 22],
}

impl ArchetypePatterns {
    /// Create archetype patterns for an Earth-like solar system
    ///
    /// This represents the archetype choices made by the Solar Logos
    /// for a solar system similar to Earth's.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V2.md Section 2.2
    ///
    /// # Returns
    /// Archetype patterns for known particles
    pub fn earth_like() -> Self {
        // Electron pattern
        // Catalyst low (0.0) produces negative charge
        // Potentiator, Catalyst, Experience moderate produce electron mass
        let electron = [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix - full potential
            1.0, // A2: Potentiator - full capacity
            0.0, // A3: Catalyst - low (negative charge)
            1.0, // A4: Experience - full experience
            1.0, // A5: Significator - strong identity
            0.5, // A6: Transformation - moderate
            1.0, // A7: Great Way - full field
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            0.0, // A10: Catalyst
            1.0, // A11: Experience
            1.0, // A12: Significator
            0.5, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            0.0, // A17: Catalyst
            1.0, // A18: Experience
            1.0, // A19: Significator
            0.5, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice - full free will
        ];

        // Proton pattern
        // Catalyst high (1.0) produces positive charge
        // Transformation high (1.0) produces higher mass
        let proton = [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix
            1.0, // A2: Potentiator
            1.0, // A3: Catalyst - high (positive charge)
            1.0, // A4: Experience
            1.0, // A5: Significator
            1.0, // A6: Transformation - high (increases mass)
            1.0, // A7: Great Way
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            1.0, // A10: Catalyst
            1.0, // A11: Experience
            1.0, // A12: Significator
            1.0, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            1.0, // A17: Catalyst
            1.0, // A18: Experience
            1.0, // A19: Significator
            1.0, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ];

        // Neutron pattern
        // Catalyst moderate (0.5) produces neutral charge
        // Experience moderate to allow decay
        let neutron = [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix
            1.0, // A2: Potentiator
            0.5, // A3: Catalyst - moderate (neutral charge)
            0.5, // A4: Experience - moderate (allows decay)
            1.0, // A5: Significator
            1.0, // A6: Transformation
            1.0, // A7: Great Way
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            0.5, // A10: Catalyst
            0.5, // A11: Experience
            1.0, // A12: Significator
            1.0, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            0.5, // A17: Catalyst
            0.5, // A18: Experience
            1.0, // A19: Significator
            1.0, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ];

        // Photon pattern (pure light)
        // All archetypes balanced to produce massless, chargeless particle
        let photon = [
            // Mind Complex (A1-A7)
            0.5, // A1: Matrix
            0.5, // A2: Potentiator
            0.5, // A3: Catalyst - balanced (no charge)
            0.0, // A4: Experience - zero (massless)
            0.5, // A5: Significator
            0.5, // A6: Transformation
            1.0, // A7: Great Way - full (spin 1)
            // Body Complex (A8-A14)
            0.5, // A8: Matrix
            0.5, // A9: Potentiator
            0.5, // A10: Catalyst
            0.0, // A11: Experience
            0.5, // A12: Significator
            0.5, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            0.5, // A15: Matrix
            0.5, // A16: Potentiator
            0.5, // A17: Catalyst
            0.0, // A18: Experience
            0.5, // A19: Significator
            0.5, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ];

        // Up quark pattern
        // Catalyst high (1.0) produces positive charge (+2/3)
        let up_quark = [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix
            1.0, // A2: Potentiator
            1.0, // A3: Catalyst - high (positive charge)
            1.0, // A4: Experience
            1.0, // A5: Significator
            1.0, // A6: Transformation
            1.0, // A7: Great Way
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            1.0, // A10: Catalyst
            1.0, // A11: Experience
            1.0, // A12: Significator
            1.0, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            1.0, // A17: Catalyst
            1.0, // A18: Experience
            1.0, // A19: Significator
            1.0, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ];

        // Down quark pattern
        // Catalyst low (0.0) produces negative charge (-1/3)
        let down_quark = [
            // Mind Complex (A1-A7)
            1.0, // A1: Matrix
            1.0, // A2: Potentiator
            0.0, // A3: Catalyst - low (negative charge)
            1.0, // A4: Experience
            1.0, // A5: Significator
            0.5, // A6: Transformation
            1.0, // A7: Great Way
            // Body Complex (A8-A14)
            1.0, // A8: Matrix
            1.0, // A9: Potentiator
            0.0, // A10: Catalyst
            1.0, // A11: Experience
            1.0, // A12: Significator
            0.5, // A13: Transformation
            1.0, // A14: Great Way
            // Spirit Complex (A15-A21)
            1.0, // A15: Matrix
            1.0, // A16: Potentiator
            0.0, // A17: Catalyst
            1.0, // A18: Experience
            1.0, // A19: Significator
            0.5, // A20: Transformation
            1.0, // A21: Great Way
            // Choice (A22)
            1.0, // A22: Choice
        ];

        ArchetypePatterns {
            electron,
            proton,
            neutron,
            photon,
            up_quark,
            down_quark,
        }
    }

    /// Create archetype patterns for a Mars-like solar system
    ///
    /// This represents the archetype choices made by a Solar Logos
    /// for a solar system similar to Mars' (different physical constants).
    ///
    /// # Returns
    /// Archetype patterns for known particles in a Mars-like system
    pub fn mars_like() -> Self {
        // For a Mars-like system, we use slightly different patterns
        // This demonstrates that different solar systems can have different
        // physical constants based on Logos' archetype choices

        let mut patterns = Self::earth_like();

        // Modify patterns to reflect different physical constants
        // For example, reduce Great Way activation to affect gravity
        for pattern in &mut [
            &mut patterns.electron,
            &mut patterns.proton,
            &mut patterns.neutron,
        ] {
            pattern[6] *= 0.9; // A7: Great Way
            pattern[13] *= 0.9; // A14: Great Way
            pattern[20] *= 0.9; // A21: Great Way
        }

        patterns
    }

    /// Get a pattern by particle name
    ///
    /// # Arguments
    /// * `particle_name` - Name of the particle ("electron", "proton", "neutron", "photon", "up_quark", "down_quark")
    ///
    /// # Returns
    /// The archetype activation pattern for the particle, or None if not found
    pub fn get_pattern(&self, particle_name: &str) -> Option<[Float; 22]> {
        match particle_name {
            "electron" => Some(self.electron),
            "proton" => Some(self.proton),
            "neutron" => Some(self.neutron),
            "photon" => Some(self.photon),
            "up_quark" => Some(self.up_quark),
            "down_quark" => Some(self.down_quark),
            _ => None,
        }
    }

    /// Validate that all patterns have the correct length (22)
    ///
    /// # Returns
    /// True if all patterns are valid
    pub fn is_valid(&self) -> bool {
        self.electron.len() == 22
            && self.proton.len() == 22
            && self.neutron.len() == 22
            && self.photon.len() == 22
            && self.up_quark.len() == 22
            && self.down_quark.len() == 22
    }

    /// Validate that all activation values are in the valid range [0.0, 1.0]
    ///
    /// # Returns
    /// True if all activation values are valid
    pub fn activation_values_valid(&self) -> bool {
        fn is_valid_array(arr: &[Float]) -> bool {
            arr.iter().all(|&v| (0.0..=1.0).contains(&v))
        }

        is_valid_array(&self.electron)
            && is_valid_array(&self.proton)
            && is_valid_array(&self.neutron)
            && is_valid_array(&self.photon)
            && is_valid_array(&self.up_quark)
            && is_valid_array(&self.down_quark)
    }
}

impl Default for ArchetypePatterns {
    fn default() -> Self {
        Self::earth_like()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_patterns_creation() {
        let patterns = ArchetypePatterns::earth_like();

        assert!(patterns.is_valid());
        assert!(patterns.activation_values_valid());
    }

    #[test]
    fn test_get_pattern() {
        let patterns = ArchetypePatterns::earth_like();

        let electron = patterns.get_pattern("electron");
        assert!(electron.is_some());
        assert_eq!(electron.unwrap().len(), 22);

        let unknown = patterns.get_pattern("unknown");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_electron_pattern() {
        let patterns = ArchetypePatterns::earth_like();

        // Electron should have low Catalyst (negative charge)
        assert_eq!(patterns.electron[2], 0.0); // A3: Catalyst
        assert_eq!(patterns.electron[9], 0.0); // A10: Catalyst
        assert_eq!(patterns.electron[16], 0.0); // A17: Catalyst
    }

    #[test]
    fn test_proton_pattern() {
        let patterns = ArchetypePatterns::earth_like();

        // Proton should have high Catalyst (positive charge)
        assert_eq!(patterns.proton[2], 1.0); // A3: Catalyst
        assert_eq!(patterns.proton[9], 1.0); // A10: Catalyst
        assert_eq!(patterns.proton[16], 1.0); // A17: Catalyst
    }

    #[test]
    fn test_neutron_pattern() {
        let patterns = ArchetypePatterns::earth_like();

        // Neutron should have moderate Catalyst (neutral charge)
        assert_eq!(patterns.neutron[2], 0.5); // A3: Catalyst
        assert_eq!(patterns.neutron[9], 0.5); // A10: Catalyst
        assert_eq!(patterns.neutron[16], 0.5); // A17: Catalyst
    }

    #[test]
    fn test_photon_pattern() {
        let patterns = ArchetypePatterns::earth_like();

        // Photon should have zero Experience (massless)
        assert_eq!(patterns.photon[3], 0.0); // A4: Experience
        assert_eq!(patterns.photon[10], 0.0); // A11: Experience
        assert_eq!(patterns.photon[17], 0.0); // A18: Experience

        // Photon should have high Great Way (spin 1)
        assert_eq!(patterns.photon[6], 1.0); // A7: Great Way
        assert_eq!(patterns.photon[13], 1.0); // A14: Great Way
        assert_eq!(patterns.photon[20], 1.0); // A21: Great Way
    }

    #[test]
    fn test_mars_like_patterns() {
        let patterns = ArchetypePatterns::mars_like();

        assert!(patterns.is_valid());
        assert!(patterns.activation_values_valid());

        // Mars-like should have slightly different Great Way values
        let earth_patterns = ArchetypePatterns::earth_like();
        assert!(patterns.electron[6] < earth_patterns.electron[6]);
    }

    #[test]
    fn test_default_patterns() {
        let patterns = ArchetypePatterns::default();

        assert!(patterns.is_valid());
        assert!(patterns.activation_values_valid());
    }
}
