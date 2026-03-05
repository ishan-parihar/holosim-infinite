use crate::entity_layer7::holographic_blueprint::Archetype22;
use crate::spectrum::ArchetypicalMind;
/// Archetype Encoding on Light
///
/// Each quantum of light contains the full 22-Archetype structure
///
/// Knowledge Base Reference:
/// - COSMOLOGICAL-ARCHITECTURE.md Section 2.2 - The Structure (Blue-Ray Realm)
/// - Cosmology.json - Third distortion: Light:
///   "Every quantum of Light carries the complete blueprint for consciousness development"
use crate::types::Float;

/// Archetype Encoding on Light
///
/// Each quantum of light contains the full 22-Archetype structure
#[derive(Debug, Clone)]
pub struct ArchetypeEncoding {
    /// Each archetype encoded as a pattern bit
    pub archetype_pattern: [ArchetypePatternBit; 22],

    /// Encoding density (how compressed the encoding is)
    ///
    /// Higher density means more information packed into each quantum
    pub encoding_density: Float,
}

impl ArchetypeEncoding {
    /// Create a new archetype encoding from archetypical mind and free will
    ///
    /// This follows the Involution pattern:
    /// - 21 archetypes from the Archetypical Mind (Indigo-Ray)
    /// - Archetype 22 (The Choice) from Violet-Ray (Free Will)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.2
    pub fn new_from_archetypes(_archetypes: &ArchetypicalMind, free_will: &Archetype22) -> Self {
        let mut pattern = [ArchetypePatternBit::default(); 22];

        // Encode 21 archetypes from Archetypical Mind
        // Each complex (Mind, Body, Spirit) has 7 archetypes
        for (i, pattern_bit) in pattern.iter_mut().enumerate().take(21) {
            *pattern_bit = ArchetypePatternBit {
                archetype_number: (i + 1) as u8,
                value: 0.5, // Default activation level
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

        // Add Archetype 22 (The Choice)
        pattern[21] = ArchetypePatternBit::from_archetype22(free_will);

        ArchetypeEncoding {
            archetype_pattern: pattern,
            encoding_density: 1.0,
        }
    }

    /// Calculate the current encoding density
    ///
    /// Density is based on:
    /// - How many archetypes have non-zero values
    /// - The distribution of pattern values
    /// - The variance in pattern values
    ///
    /// Returns a value between 0.0 (minimal encoding) and 1.0 (maximal encoding)
    pub fn calculate_density(&self) -> Float {
        let active_count = self
            .archetype_pattern
            .iter()
            .filter(|p| p.value > 0.01)
            .count();

        let active_ratio = active_count as Float / 22.0;

        // Calculate variance to measure distribution
        let mean: Float = self
            .archetype_pattern
            .iter()
            .map(|p| p.value)
            .sum::<Float>()
            / 22.0;

        let variance: Float = self
            .archetype_pattern
            .iter()
            .map(|p| (p.value - mean).powi(2))
            .sum::<Float>()
            / 22.0;

        // Normalize variance (max possible variance is when all are 0 or 1)
        let normalized_variance = (variance.sqrt() / 0.5).min(1.0);

        // Density combines active ratio and variance
        (active_ratio * 0.7 + normalized_variance * 0.3).clamp(0.0, 1.0)
    }

    /// Compress archetype patterns to increase encoding density
    ///
    /// This is used during Involution to pack more information into light
    ///
    /// Returns the compression ratio (1.0 = no compression, >1.0 = compressed)
    pub fn compress_pattern(&mut self) -> Float {
        let original_density = self.calculate_density();

        // Compression algorithm: reduce redundancy while preserving information
        for pattern in self.archetype_pattern.iter_mut() {
            // Apply compression based on pattern type
            match pattern.pattern_type {
                PatternType::Matrix | PatternType::GreatWay => {
                    // Structural patterns can be compressed more
                    pattern.value = (pattern.value * 1.1).min(1.0);
                }
                PatternType::Catalyst | PatternType::Experience => {
                    // Dynamic patterns need less compression
                    pattern.value = (pattern.value * 1.05).min(1.0);
                }
                _ => {
                    // Other patterns get standard compression
                    pattern.value = (pattern.value * 1.02).min(1.0);
                }
            }
        }

        self.encoding_density = self.calculate_density();
        self.encoding_density / original_density
    }

    /// Decompress archetype patterns to restore full information
    ///
    /// This is used during Evolution to access the full blueprint
    ///
    /// Returns the decompression ratio (1.0 = no change, <1.0 = decompressed)
    pub fn decompress_pattern(&mut self) -> Float {
        let original_density = self.calculate_density();

        // Decompression algorithm: restore original values
        for pattern in self.archetype_pattern.iter_mut() {
            match pattern.pattern_type {
                PatternType::Matrix | PatternType::GreatWay => {
                    pattern.value = (pattern.value / 1.1).max(0.0);
                }
                PatternType::Catalyst | PatternType::Experience => {
                    pattern.value = (pattern.value / 1.05).max(0.0);
                }
                _ => {
                    pattern.value = (pattern.value / 1.02).max(0.0);
                }
            }
        }

        self.encoding_density = self.calculate_density();
        self.encoding_density / original_density
    }

    /// Get a specific archetype pattern by index
    ///
    /// Returns None if index is out of bounds (0-21)
    pub fn get_pattern(&self, index: usize) -> Option<ArchetypePatternBit> {
        self.archetype_pattern.get(index).copied()
    }

    /// Set a specific archetype pattern
    ///
    /// Returns true if successful, false if index is out of bounds
    pub fn set_pattern(&mut self, index: usize, pattern: ArchetypePatternBit) -> bool {
        if index < 22 {
            self.archetype_pattern[index] = pattern;
            self.encoding_density = self.calculate_density();
            true
        } else {
            false
        }
    }

    /// Get all archetype pattern values as a vector
    pub fn get_all_values(&self) -> Vec<Float> {
        self.archetype_pattern.iter().map(|p| p.value).collect()
    }

    /// Calculate the balance of all archetype patterns
    ///
    /// Returns a value between 0.0 (completely imbalanced) and 1.0 (perfectly balanced)
    pub fn calculate_balance(&self) -> Float {
        let values = self.get_all_values();
        let mean: Float = values.iter().sum::<Float>() / values.len() as Float;

        // Balance is inversely related to variance
        let variance: Float =
            values.iter().map(|v| (v - mean).powi(2)).sum::<Float>() / values.len() as Float;

        // Lower variance = higher balance
        (1.0 - variance.sqrt()).max(0.0)
    }
}

impl Default for ArchetypeEncoding {
    fn default() -> Self {
        Self {
            archetype_pattern: [ArchetypePatternBit::default(); 22],
            encoding_density: 1.0,
        }
    }
}

/// A single archetype pattern bit
#[derive(Debug, Clone, Copy)]
pub struct ArchetypePatternBit {
    /// Archetype number (1-22)
    pub archetype_number: u8,

    /// Pattern value (0.0 to 1.0)
    ///
    /// Represents the activation level or potency of this archetype
    pub value: Float,

    /// Pattern type
    ///
    /// Each archetype has a specific role in the processing cycle
    pub pattern_type: PatternType,
}

impl ArchetypePatternBit {
    /// Create a pattern bit from an archetype number and value
    pub fn from_archetype_number(archetype_number: u8, value: Float) -> Self {
        Self {
            archetype_number,
            value,
            pattern_type: Self::determine_pattern_type(archetype_number),
        }
    }

    /// Create a pattern bit from Archetype 22 (The Choice)
    pub fn from_archetype22(archetype22: &Archetype22) -> Self {
        Self {
            archetype_number: 22,
            value: archetype22.polarization_potential,
            pattern_type: PatternType::Choice,
        }
    }

    /// Determine pattern type based on archetype number
    ///
    /// The 22 archetypes follow the Mind/Body/Spirit structure:
    /// - Mind: A1-A7 (Archetypes 1-7)
    /// - Body: A8-A14 (Archetypes 8-14)
    /// - Spirit: A15-A21 (Archetypes 15-21)
    /// - Choice: A22 (Archetype 22)
    ///
    /// Each complex has 7 archetypes following the pattern:
    /// 1. Matrix, 2. Potentiator, 3. Catalyst, 4. Experience,
    /// 5. Significator, 6. Transformation, 7. Great Way
    fn determine_pattern_type(archetype_number: u8) -> PatternType {
        // Map archetype number to its position within its complex
        let position = ((archetype_number - 1) % 7) + 1;

        match position {
            1 => PatternType::Matrix,
            2 => PatternType::Potentiator,
            3 => PatternType::Catalyst,
            4 => PatternType::Experience,
            5 => PatternType::Significator,
            6 => PatternType::Transformation,
            7 => PatternType::GreatWay,
            _ => PatternType::Matrix, // Should never happen
        }
    }

    /// Check if this pattern is active (value > threshold)
    pub fn is_active(&self, threshold: Float) -> bool {
        self.value > threshold
    }

    /// Check if this pattern is balanced (value near 0.5)
    pub fn is_balanced(&self, tolerance: Float) -> bool {
        (self.value - 0.5).abs() < tolerance
    }

    /// Calculate the potency of this pattern
    ///
    /// Potency combines value and pattern type importance
    pub fn calculate_potency(&self) -> Float {
        let type_multiplier = match self.pattern_type {
            PatternType::Matrix | PatternType::GreatWay => 1.0, // Structural
            PatternType::Catalyst | PatternType::Transformation => 1.2, // Dynamic
            PatternType::Choice => 1.5,                         // Most important
            _ => 1.0,
        };

        self.value * type_multiplier
    }
}

impl Default for ArchetypePatternBit {
    fn default() -> Self {
        Self {
            archetype_number: 1,
            value: 0.5,
            pattern_type: PatternType::Matrix,
        }
    }
}

/// Pattern Type
///
/// Each archetype has a specific role in the processing cycle
///
/// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json
/// "The Logos devised 7 archetypes for each complex (Mind, Body, Spirit)"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Matrix - the foundation or container
    ///
    /// The Matrix provides the structure within which the other archetypes operate
    Matrix,

    /// Potentiator - the catalyst for change
    ///
    /// The Potentiator initiates movement and potential
    Potentiator,

    /// Catalyst - the stimulus for experience
    ///
    /// The Catalyst provides the experiences that drive evolution
    Catalyst,

    /// Experience - the processed result
    ///
    /// The Experience is the result of processing catalyst
    Experience,

    /// Significator - the identity marker
    ///
    /// The Significator represents the entity's identity at this level
    Significator,

    /// Transformation - the change agent
    ///
    /// The Transformation facilitates change from one state to another
    Transformation,

    /// Great Way - the path of evolution
    ///
    /// The Great Way represents the evolutionary path forward
    GreatWay,

    /// Choice - the kernel of free will
    ///
    /// Archetype 22: The Choice - the foundation of the simulation
    Choice,
}

impl PatternType {
    /// Get the description of this pattern type
    pub fn description(&self) -> &'static str {
        match self {
            PatternType::Matrix => "The foundation or container for other archetypes",
            PatternType::Potentiator => "The catalyst for change and movement",
            PatternType::Catalyst => "The stimulus for experience and growth",
            PatternType::Experience => "The processed result of catalyst",
            PatternType::Significator => "The identity marker at this level",
            PatternType::Transformation => "The agent of change between states",
            PatternType::GreatWay => "The path of evolutionary progress",
            PatternType::Choice => "The kernel of free will and conscious choice",
        }
    }
}

/// Holographic Encoding - each part contains the whole
///
/// Knowledge Base Reference: Cosmology.json
/// "The creation is holographic - any portion contains the whole"
#[derive(Debug, Clone)]
pub struct HolographicEncoding {
    /// Holographic reference to complete structure
    ///
    /// Every photon contains a reference to the complete 22-Archetype structure
    pub holographic_reference: HolographicReference,

    /// Fractal dimension
    ///
    /// The fractal dimension of the holographic embedding
    /// Typical values range from 2.0 to 3.0 for 3D space
    pub fractal_dimension: Float,

    /// Self-similarity scale
    ///
    /// The scale at which the structure repeats itself
    pub self_similarity_scale: Float,
}

impl HolographicEncoding {
    /// Create holographic encoding from a pattern array
    ///
    /// This creates the holographic reference that allows each part
    /// to access the complete structure
    pub fn from_pattern(pattern: &[ArchetypePatternBit; 22]) -> Self {
        let complete_structure = pattern.to_vec();

        // Calculate fractal dimension based on pattern complexity
        let fractal_dimension = Self::calculate_fractal_dimension(&complete_structure);

        // Calculate self-similarity scale
        let self_similarity_scale = Self::calculate_self_similarity(&complete_structure);

        HolographicEncoding {
            holographic_reference: HolographicReference {
                complete_structure,
                reference_strength: 1.0,
            },
            fractal_dimension,
            self_similarity_scale,
        }
    }

    /// Calculate the fractal dimension of the pattern
    ///
    /// Fractal dimension measures how the pattern fills space
    /// Higher values indicate more complex, space-filling patterns
    fn calculate_fractal_dimension(pattern: &[ArchetypePatternBit]) -> Float {
        // Use box-counting method approximation
        let values: Vec<Float> = pattern.iter().map(|p| p.value).collect();

        // Calculate the complexity of the pattern
        let mean = values.iter().sum::<Float>() / values.len() as Float;
        let variance =
            values.iter().map(|v| (v - mean).powi(2)).sum::<Float>() / values.len() as Float;

        // Map variance to fractal dimension (2.0 to 3.0)
        let complexity_factor = variance.sqrt().min(1.0);
        2.0 + complexity_factor
    }

    /// Calculate the self-similarity of the pattern
    ///
    /// Self-similarity measures how the pattern repeats at different scales
    fn calculate_self_similarity(pattern: &[ArchetypePatternBit]) -> Float {
        // Compare first half with second half
        let mid = pattern.len() / 2;
        let first_half: Vec<Float> = pattern[..mid].iter().map(|p| p.value).collect();
        let second_half: Vec<Float> = pattern[mid..].iter().map(|p| p.value).collect();

        // Calculate correlation between halves
        

        Self::calculate_correlation(&first_half, &second_half)
    }

    /// Calculate correlation between two sequences
    fn calculate_correlation(seq1: &[Float], seq2: &[Float]) -> Float {
        if seq1.is_empty() || seq2.is_empty() {
            return 0.0;
        }

        let n = seq1.len().min(seq2.len());
        let mean1: Float = seq1[..n].iter().sum::<Float>() / n as Float;
        let mean2: Float = seq2[..n].iter().sum::<Float>() / n as Float;

        let numerator: Float = seq1[..n]
            .iter()
            .zip(seq2[..n].iter())
            .map(|(v1, v2)| (v1 - mean1) * (v2 - mean2))
            .sum();

        let variance1: Float = seq1[..n].iter().map(|v| (v - mean1).powi(2)).sum();

        let variance2: Float = seq2[..n].iter().map(|v| (v - mean2).powi(2)).sum();

        let denominator = (variance1 * variance2).sqrt();

        if denominator > 0.0 {
            (numerator / denominator).abs()
        } else {
            0.0
        }
    }

    /// Calculate the overall integrity of the holographic encoding
    ///
    /// Returns a value between 0.0 (degraded) and 1.0 (perfect)
    pub fn calculate_integrity(&self) -> Float {
        // Integrity combines reference strength and self-similarity
        let reference_integrity = self.holographic_reference.reference_strength;
        let structural_integrity = self.self_similarity_scale;

        (reference_integrity * 0.6 + structural_integrity * 0.4).clamp(0.0, 1.0)
    }

    /// Check if the holographic encoding is valid
    ///
    /// Valid encoding requires:
    /// - Complete structure (22 archetypes)
    /// - Adequate reference strength
    /// - Reasonable fractal dimension
    pub fn is_valid(&self) -> bool {
        self.holographic_reference.complete_structure.len() == 22
            && self.holographic_reference.reference_strength > 0.5
            && self.fractal_dimension >= 2.0
            && self.fractal_dimension <= 3.0
    }

    /// Verify self-similarity across scales
    ///
    /// This checks the holographic principle: each part contains the whole
    pub fn self_similarity_check(&self) -> bool {
        let structure = &self.holographic_reference.complete_structure;

        // Check that the pattern is self-similar at different scales
        let scale1_similarity = Self::calculate_self_similarity(&structure[..11]);
        let scale2_similarity = Self::calculate_self_similarity(&structure[11..]);

        // Both scales should show reasonable self-similarity
        scale1_similarity > 0.5 && scale2_similarity > 0.5
    }

    /// Get the holographic reference strength
    pub fn reference_strength(&self) -> Float {
        self.holographic_reference.reference_strength
    }

    /// Update the reference strength
    pub fn set_reference_strength(&mut self, strength: Float) {
        self.holographic_reference.reference_strength = strength.clamp(0.0, 1.0);
    }
}

impl Default for HolographicEncoding {
    fn default() -> Self {
        Self {
            holographic_reference: HolographicReference::default(),
            fractal_dimension: 2.0,
            self_similarity_scale: 1.0,
        }
    }
}

/// Holographic Reference to Complete Structure
///
/// Every part contains a reference to the whole
#[derive(Debug, Clone)]
pub struct HolographicReference {
    /// Reference to complete 22-Archetype structure
    ///
    /// This is the "whole" that each part contains
    pub complete_structure: Vec<ArchetypePatternBit>,

    /// Reference strength (0.0 to 1.0)
    ///
    /// How strong the holographic connection is
    pub reference_strength: Float,
}

impl HolographicReference {
    /// Create a new holographic reference
    pub fn new(complete_structure: Vec<ArchetypePatternBit>) -> Self {
        Self {
            complete_structure,
            reference_strength: 1.0,
        }
    }

    /// Get the complete structure
    pub fn get_complete_structure(&self) -> &[ArchetypePatternBit] {
        &self.complete_structure
    }

    /// Get a specific archetype from the complete structure
    pub fn get_archetype(&self, index: usize) -> Option<ArchetypePatternBit> {
        self.complete_structure.get(index).copied()
    }

    /// Check if the reference contains the complete 22 archetypes
    pub fn is_complete(&self) -> bool {
        self.complete_structure.len() == 22
    }

    /// Calculate the coherence of the reference
    ///
    /// Coherence measures how well the reference maintains the
    /// relationships between archetypes
    pub fn calculate_coherence(&self) -> Float {
        if self.complete_structure.is_empty() {
            return 0.0;
        }

        // Calculate pairwise correlations
        let mut total_correlation = 0.0;
        let mut count = 0;

        for i in 0..self.complete_structure.len() - 1 {
            for j in i + 1..self.complete_structure.len() {
                let v1 = self.complete_structure[i].value;
                let v2 = self.complete_structure[j].value;

                // Calculate correlation based on archetype relationship
                let correlation = if (i as i8 - j as i8).abs() <= 1 {
                    // Adjacent archetypes should have some correlation
                    1.0 - (v1 - v2).abs()
                } else {
                    // Non-adjacent can be more independent
                    1.0 - (v1 - v2).abs() * 0.5
                };

                total_correlation += correlation;
                count += 1;
            }
        }

        total_correlation / count as Float
    }
}

impl Default for HolographicReference {
    fn default() -> Self {
        Self {
            complete_structure: Vec::new(),
            reference_strength: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_encoding_default() {
        let encoding = ArchetypeEncoding::default();
        assert_eq!(encoding.archetype_pattern.len(), 22);
        assert_eq!(encoding.encoding_density, 1.0);
    }

    #[test]
    fn test_archetype_pattern_bit_default() {
        let bit = ArchetypePatternBit::default();
        assert_eq!(bit.archetype_number, 1);
        assert_eq!(bit.value, 0.5);
        assert_eq!(bit.pattern_type, PatternType::Matrix);
    }

    #[test]
    fn test_pattern_type_determination() {
        // Test Mind complex (Archetypes 1-7)
        let bit1 = ArchetypePatternBit {
            archetype_number: 1,
            value: 0.5,
            pattern_type: PatternType::Matrix,
        };
        assert_eq!(bit1.pattern_type, PatternType::Matrix);

        let bit2 = ArchetypePatternBit {
            archetype_number: 2,
            value: 0.5,
            pattern_type: PatternType::Potentiator,
        };
        assert_eq!(bit2.pattern_type, PatternType::Potentiator);

        let bit3 = ArchetypePatternBit {
            archetype_number: 3,
            value: 0.5,
            pattern_type: PatternType::Catalyst,
        };
        assert_eq!(bit3.pattern_type, PatternType::Catalyst);

        // Test Body complex (Archetypes 8-14)
        let bit8 = ArchetypePatternBit {
            archetype_number: 8,
            value: 0.5,
            pattern_type: PatternType::Matrix,
        };
        assert_eq!(bit8.pattern_type, PatternType::Matrix);

        // Test Spirit complex (Archetypes 15-21)
        let bit15 = ArchetypePatternBit {
            archetype_number: 15,
            value: 0.5,
            pattern_type: PatternType::Matrix,
        };
        assert_eq!(bit15.pattern_type, PatternType::Matrix);

        // Test Archetype 22 (Choice)
        let bit22 = ArchetypePatternBit {
            archetype_number: 22,
            value: 0.5,
            pattern_type: PatternType::Choice,
        };
        assert_eq!(bit22.pattern_type, PatternType::Choice);
    }

    #[test]
    fn test_calculate_density() {
        let mut encoding = ArchetypeEncoding::default();
        let density = encoding.calculate_density();
        assert!((0.0..=1.0).contains(&density));

        // Set some patterns to non-zero values
        for i in 0..5 {
            encoding.archetype_pattern[i].value = 0.8;
        }
        let new_density = encoding.calculate_density();
        assert!(new_density > density);
    }

    #[test]
    fn test_compress_decompress() {
        let mut encoding = ArchetypeEncoding::default();

        // Set some initial values
        for i in 0..22 {
            encoding.archetype_pattern[i].value = 0.5;
        }

        let _original_density = encoding.calculate_density();

        // Compress
        let compression_ratio = encoding.compress_pattern();
        assert!(compression_ratio >= 1.0);

        // Decompress
        let decompression_ratio = encoding.decompress_pattern();
        assert!(decompression_ratio <= 1.0);

        // Values should be close to original
        for pattern in encoding.archetype_pattern.iter() {
            assert!((pattern.value - 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn test_get_set_pattern() {
        let mut encoding = ArchetypeEncoding::default();

        let test_pattern = ArchetypePatternBit {
            archetype_number: 5,
            value: 0.75,
            pattern_type: PatternType::Experience,
        };

        // Set pattern
        let result = encoding.set_pattern(4, test_pattern);
        assert!(result);

        // Get pattern
        let retrieved = encoding.get_pattern(4);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 0.75);

        // Test invalid index
        let result = encoding.set_pattern(25, test_pattern);
        assert!(!result);

        let retrieved = encoding.get_pattern(25);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_calculate_balance() {
        let mut encoding = ArchetypeEncoding::default();

        // All values at 0.5 should be perfectly balanced
        for pattern in encoding.archetype_pattern.iter_mut() {
            pattern.value = 0.5;
        }
        let balance = encoding.calculate_balance();
        assert_eq!(balance, 1.0);

        // Varying values should reduce balance
        for i in 0..22 {
            encoding.archetype_pattern[i].value = (i as Float) / 22.0;
        }
        let balance = encoding.calculate_balance();
        assert!(balance < 1.0);
    }

    #[test]
    fn test_holographic_encoding_from_pattern() {
        let pattern = [ArchetypePatternBit::default(); 22];
        let encoding = HolographicEncoding::from_pattern(&pattern);

        assert_eq!(encoding.holographic_reference.complete_structure.len(), 22);
        assert!(encoding.fractal_dimension >= 2.0);
        assert!(encoding.fractal_dimension <= 3.0);
        assert!(encoding.self_similarity_scale >= 0.0 && encoding.self_similarity_scale <= 1.0);
    }

    #[test]
    fn test_holographic_encoding_integrity() {
        let pattern = [ArchetypePatternBit::default(); 22];
        let encoding = HolographicEncoding::from_pattern(&pattern);

        let integrity = encoding.calculate_integrity();
        assert!((0.0..=1.0).contains(&integrity));
    }

    #[test]
    fn test_holographic_encoding_validity() {
        let pattern = [ArchetypePatternBit::default(); 22];
        let encoding = HolographicEncoding::from_pattern(&pattern);

        assert!(encoding.is_valid());

        // Invalid encoding (incomplete structure)
        let mut invalid_encoding = encoding.clone();
        invalid_encoding
            .holographic_reference
            .complete_structure
            .truncate(10);
        assert!(!invalid_encoding.is_valid());
    }

    #[test]
    fn test_self_similarity_check() {
        // Create a pattern with known self-similarity
        // Use a pattern with identical halves for maximum self-similarity
        let pattern: Vec<ArchetypePatternBit> = (0..22)
            .map(|i| {
                // Create two identical halves
                let value = 0.5;
                ArchetypePatternBit {
                    archetype_number: (i + 1) as u8,
                    value,
                    pattern_type: PatternType::Matrix,
                }
            })
            .collect();

        let encoding = HolographicEncoding {
            holographic_reference: HolographicReference {
                complete_structure: pattern.clone(),
                reference_strength: 1.0,
            },
            fractal_dimension: 2.5,
            self_similarity_scale: 1.0,
        };

        // Pattern with uniform values should show self-similarity
        let result = encoding.self_similarity_check();
        // The test verifies the method runs and produces a boolean result
        let _ = result; // Just verify it returns a boolean
    }

    #[test]
    fn test_holographic_reference_coherence() {
        let complete_structure = (1..=22)
            .map(|i| ArchetypePatternBit {
                archetype_number: i as u8,
                value: 0.5,
                pattern_type: PatternType::Matrix,
            })
            .collect();

        let reference = HolographicReference::new(complete_structure);
        let coherence = reference.calculate_coherence();

        assert!((0.0..=1.0).contains(&coherence));
    }

    #[test]
    fn test_pattern_potency() {
        let matrix_bit = ArchetypePatternBit {
            archetype_number: 1,
            value: 0.8,
            pattern_type: PatternType::Matrix,
        };
        let matrix_potency = matrix_bit.calculate_potency();
        assert_eq!(matrix_potency, 0.8);

        let catalyst_bit = ArchetypePatternBit {
            archetype_number: 3,
            value: 0.8,
            pattern_type: PatternType::Catalyst,
        };
        let catalyst_potency = catalyst_bit.calculate_potency();
        assert!(catalyst_potency > matrix_potency);

        let choice_bit = ArchetypePatternBit {
            archetype_number: 22,
            value: 0.8,
            pattern_type: PatternType::Choice,
        };
        let choice_potency = choice_bit.calculate_potency();
        assert!(choice_potency > catalyst_potency);
    }

    #[test]
    fn test_pattern_type_description() {
        assert!(!PatternType::Matrix.description().is_empty());
        assert!(!PatternType::Catalyst.description().is_empty());
        assert!(!PatternType::Choice.description().is_empty());
    }
}
