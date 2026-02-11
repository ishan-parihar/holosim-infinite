// Light Architecture Module
//
// This module implements the LightArchitecture structure that contains the
// complete 22-Archetype structure impressed upon Light.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 2.2 (The Structure - Blue-Ray Realm)
// - Energy-Ray-Centers/5. Blue Ray.json

use super::encoding::{ArchetypeEncoding, ArchetypePatternBit, HolographicEncoding, PatternType};
use crate::entity_layer7::holographic_blueprint::Archetype22;
use crate::spectrum::ArchetypicalMind;
use crate::types::Float;

/// Light Architecture - 22-Archetype structure impressed on Light
///
/// Every quantum of Light carries the complete blueprint for consciousness development.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.2
/// "The Logos took the 21 Archetypes (from Indigo-Ray Realm)
/// Combined them with Archetype 22 (The Choice from Violet-Ray Realm)
/// Impressed this 22-Archetype structure upon the Light"
#[derive(Debug, Clone)]
pub struct LightArchitecture {
    /// The 22-Archetype encoding on light
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The 22-Archetype structure is impressed upon Light"
    pub archetype_encoding: ArchetypeEncoding,

    /// Fractal-holographic embedding
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each photon is encoded with the full 22-Archetype structure"
    pub holographic_encoding: HolographicEncoding,

    /// The Law of Light principles
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Logos discovered the Law of Light"
    pub light_laws: LightLaws,
}

impl Default for LightArchitecture {
    fn default() -> Self {
        Self {
            archetype_encoding: ArchetypeEncoding::default(),
            holographic_encoding: HolographicEncoding::default(),
            light_laws: LightLaws::default(),
        }
    }
}

impl LightArchitecture {
    /// Create light architecture by impressing archetypes
    ///
    /// This follows the Involution process at the Blue-Ray layer:
    /// 1. Take the 21 Archetypes from the Archetypical Mind (Indigo-Ray)
    /// 2. Combine with Archetype 22 (The Choice from Violet-Ray)
    /// 3. Impress this 22-Archetype structure upon the Light
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.2
    pub fn impress_archetypes(_archetypes: &ArchetypicalMind, free_will: &Archetype22) -> Self {
        // Encode 21 archetypes (A1-A21)
        let mut pattern = [ArchetypePatternBit::default(); 22];

        // Map the 21 archetypes to pattern bits
        // A1-A7: Mind Complex
        // A8-A14: Body Complex
        // A15-A21: Spirit Complex
        for i in 0..21 {
            pattern[i] = ArchetypePatternBit {
                archetype_number: (i + 1) as u8,
                value: 0.5, // Default balanced value
                pattern_type: match i % 7 {
                    0 => PatternType::Matrix,
                    1 => PatternType::Potentiator,
                    2 => PatternType::Catalyst,
                    3 => PatternType::Experience,
                    4 => PatternType::Significator,
                    5 => PatternType::Transformation,
                    6 => PatternType::GreatWay,
                    _ => PatternType::Matrix,
                },
            };
        }

        // Add Archetype 22 (The Choice) - the kernel from Violet-Ray
        pattern[21] = ArchetypePatternBit {
            archetype_number: 22,
            value: free_will.polarization_potential,
            pattern_type: PatternType::Choice,
        };

        // Create holographic encoding
        let holographic_encoding = HolographicEncoding::from_pattern(&pattern);

        // Apply Law of Light principles
        let light_laws = LightLaws::from_archetypal_structure(&pattern);

        LightArchitecture {
            archetype_encoding: ArchetypeEncoding {
                archetype_pattern: pattern,
                encoding_density: 1.0,
            },
            holographic_encoding,
            light_laws,
        }
    }

    /// Get the archetype encoding
    pub fn archetype_encoding(&self) -> &ArchetypeEncoding {
        &self.archetype_encoding
    }

    /// Get the holographic encoding
    pub fn holographic_encoding(&self) -> &HolographicEncoding {
        &self.holographic_encoding
    }

    /// Get the light laws
    pub fn light_laws(&self) -> &LightLaws {
        &self.light_laws
    }

    /// Decode archetype information at a specific index
    ///
    /// Returns the archetype pattern bit for the specified archetype number (1-22)
    pub fn decode_archetype(&self, index: usize) -> Option<ArchetypePatternBit> {
        self.archetype_encoding
            .archetype_pattern
            .get(index)
            .copied()
    }

    /// Check if light architecture is balanced
    ///
    /// A balanced light architecture has all archetype values within a reasonable range
    pub fn is_balanced(&self) -> bool {
        const MIN_VALUE: Float = 0.1;
        const MAX_VALUE: Float = 0.9;

        self.archetype_encoding
            .archetype_pattern
            .iter()
            .all(|bit| bit.value >= MIN_VALUE && bit.value <= MAX_VALUE)
    }

    /// Check if light architecture is valid
    ///
    /// A valid light architecture has:
    /// - All 22 archetype pattern bits
    /// - Valid holographic encoding
    /// - Valid light laws
    pub fn is_valid(&self) -> bool {
        // Check archetype encoding has 22 pattern bits
        if self.archetype_encoding.archetype_pattern.len() != 22 {
            return false;
        }

        // Check holographic encoding is valid
        if !self.holographic_encoding.is_valid() {
            return false;
        }

        // Check light laws are valid
        if !self.light_laws.is_valid() {
            return false;
        }

        true
    }

    /// Calculate overall encoding integrity
    ///
    /// Returns a value from 0.0 to 1.0 indicating how well-encoded the architecture is
    pub fn calculate_integrity(&self) -> Float {
        let encoding_factor = self.archetype_encoding.encoding_density;
        let holographic_factor = self.holographic_encoding.calculate_integrity();
        let laws_factor = self.light_laws.calculate_integrity();

        (encoding_factor + holographic_factor + laws_factor) / 3.0
    }
}

/// The Law of Light Principles
///
/// These are the fundamental laws that govern how Light operates in the creation.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.2
/// "The Logos discovered the Law of Light"
#[derive(Debug, Clone)]
pub struct LightLaws {
    /// Law of One (all is one)
    ///
    /// From Cosmology.json:
    /// "The first distortion is Free Will, the second is Love, the third is Light"
    pub law_of_one: Float,

    /// Law of Confusion (free will requires veiling)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Veil creates the conditions for the Entity to experience separation"
    pub law_of_confusion: Float,

    /// Law of Love (energy flows through love)
    ///
    /// From Cosmology.json:
    /// "The second distortion is Love"
    pub law_of_love: Float,

    /// Law of Light (light carries intelligence)
    ///
    /// From Cosmology.json:
    /// "The third distortion is Light"
    pub law_of_light: Float,
}

impl Default for LightLaws {
    fn default() -> Self {
        Self {
            law_of_one: 1.0,
            law_of_confusion: 1.0,
            law_of_love: 1.0,
            law_of_light: 1.0,
        }
    }
}

impl LightLaws {
    /// Create light laws from archetypal structure
    ///
    /// The laws are derived from the archetypal pattern
    pub fn from_archetypal_structure(pattern: &[ArchetypePatternBit]) -> Self {
        // Calculate law strengths based on pattern values
        let law_of_one = Self::calculate_law_of_one_strength(pattern);
        let law_of_confusion = Self::calculate_law_of_confusion_strength(pattern);
        let law_of_love = Self::calculate_law_of_love_strength(pattern);
        let law_of_light = Self::calculate_law_of_light_strength(pattern);

        Self {
            law_of_one,
            law_of_confusion,
            law_of_love,
            law_of_light,
        }
    }

    /// Calculate Law of One strength from pattern
    ///
    /// The Law of One is strongest when all archetypes are balanced
    fn calculate_law_of_one_strength(pattern: &[ArchetypePatternBit]) -> Float {
        if pattern.is_empty() {
            return 1.0;
        }

        let sum: Float = pattern.iter().map(|bit| bit.value).sum();
        let average = sum / pattern.len() as Float;

        // Balance is when values are close to 0.5
        let balance_score = 1.0 - (average - 0.5).abs() * 2.0;
        balance_score.max(0.0).min(1.0)
    }

    /// Calculate Law of Confusion strength from pattern
    ///
    /// The Law of Confusion is necessary for free will to operate
    fn calculate_law_of_confusion_strength(pattern: &[ArchetypePatternBit]) -> Float {
        // Confusion is proportional to the diversity of pattern values
        if pattern.is_empty() {
            return 1.0;
        }

        let variance = Self::calculate_variance(pattern);
        // More variance = more confusion potential
        variance.min(1.0)
    }

    /// Calculate Law of Love strength from pattern
    ///
    /// The Law of Love is strongest when archetypes are harmonious
    fn calculate_law_of_love_strength(pattern: &[ArchetypePatternBit]) -> Float {
        if pattern.is_empty() {
            return 1.0;
        }

        // Love is related to the average value (higher = more love)
        let sum: Float = pattern.iter().map(|bit| bit.value).sum();
        let average = sum / pattern.len() as Float;

        average
    }

    /// Calculate Law of Light strength from pattern
    ///
    /// The Law of Light is the foundation of all encoding
    fn calculate_law_of_light_strength(pattern: &[ArchetypePatternBit]) -> Float {
        if pattern.is_empty() {
            return 1.0;
        }

        // Light strength is related to the completeness of the pattern
        // All 22 archetypes should be present
        let completeness = pattern.len() as Float / 22.0;

        completeness.min(1.0)
    }

    /// Calculate variance of pattern values
    fn calculate_variance(pattern: &[ArchetypePatternBit]) -> Float {
        if pattern.is_empty() {
            return 0.0;
        }

        let sum: Float = pattern.iter().map(|bit| bit.value).sum();
        let mean = sum / pattern.len() as Float;

        let variance_sum: Float = pattern.iter().map(|bit| (bit.value - mean).powi(2)).sum();

        (variance_sum / pattern.len() as Float).sqrt()
    }

    /// Calculate overall integrity of light laws
    ///
    /// Returns a value from 0.0 to 1.0
    pub fn calculate_integrity(&self) -> Float {
        (self.law_of_one + self.law_of_confusion + self.law_of_love + self.law_of_light) / 4.0
    }

    /// Check if all laws are active
    pub fn all_laws_active(&self) -> bool {
        self.law_of_one > 0.0
            && self.law_of_confusion > 0.0
            && self.law_of_love > 0.0
            && self.law_of_light > 0.0
    }

    /// Check if light laws are valid
    ///
    /// Valid light laws have all values within reasonable bounds
    pub fn is_valid(&self) -> bool {
        self.law_of_one >= 0.0
            && self.law_of_one <= 1.0
            && self.law_of_confusion >= 0.0
            && self.law_of_confusion <= 1.0
            && self.law_of_love >= 0.0
            && self.law_of_love <= 1.0
            && self.law_of_light >= 0.0
            && self.law_of_light <= 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_architecture_creation() {
        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        assert_eq!(light.archetype_encoding.archetype_pattern.len(), 22);
    }

    #[test]
    fn test_light_architecture_decode() {
        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let archetype_1 = light.decode_archetype(0);
        assert!(archetype_1.is_some());
        assert_eq!(archetype_1.unwrap().archetype_number, 1);

        let archetype_22 = light.decode_archetype(21);
        assert!(archetype_22.is_some());
        assert_eq!(archetype_22.unwrap().archetype_number, 22);
    }

    #[test]
    fn test_light_architecture_integrity() {
        let archetypes = ArchetypicalMind::new_from_logos();
        let free_will = Archetype22::new_from_infinity();
        let light = LightArchitecture::impress_archetypes(&archetypes, &free_will);

        let integrity = light.calculate_integrity();
        assert!(integrity > 0.0);
        assert!(integrity <= 1.0);
    }

    #[test]
    fn test_light_laws_default() {
        let laws = LightLaws::default();
        assert_eq!(laws.law_of_one, 1.0);
        assert_eq!(laws.law_of_confusion, 1.0);
        assert_eq!(laws.law_of_love, 1.0);
        assert_eq!(laws.law_of_light, 1.0);
    }

    #[test]
    fn test_light_laws_integrity() {
        let laws = LightLaws::default();
        let integrity = laws.calculate_integrity();
        assert_eq!(integrity, 1.0);
    }

    #[test]
    fn test_light_laws_all_active() {
        let laws = LightLaws::default();
        assert!(laws.all_laws_active());
    }
}
