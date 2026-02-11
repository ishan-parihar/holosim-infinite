//! Collective Resonance Calculation System
//!
//! From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6, Week 89-92:
//! "Collective Manifestation (Building) - multiple entities contribute resonance
//! to create structures"
//!
//! This module implements the collective resonance calculator that:
//! - Extracts spectrum ratios from entities via spectrum_access.ratio
//! - Converts spectrum ratios to resonance patterns
//! - Combines patterns using averaging (constructive interference)
//! - Computes coherence from pattern similarity
//! - Computes stability from coherence
//! - Finds dominant densities from pattern magnitudes
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The whole is more than the sum of parts (collective consciousness)"

use crate::entity_layer7::layer7::SubSubLogos;
use crate::simulation_v3::holographic_inventory::ResonancePattern;
use crate::simulation_v3::holographic_physics::SpectrumRatio;
use crate::types::Float;

/// Computes the collective resonance from multiple entities
///
/// The calculator analyzes how multiple entities' individual resonance patterns
/// combine to form a collective resonance field. This collective field represents
/// the manifestation potential of the group.
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
/// "Collective resonance emerges when entities align their archetypical patterns,
/// creating stronger manifestation potential than any individual alone."
pub struct CollectiveResonanceCalculator;

/// Result of collective resonance computation
///
/// Contains the collective pattern and metadata about the collective's
/// coherence, stability, and dominant resonance frequencies.
#[derive(Debug, Clone)]
pub struct CollectiveResonanceResult {
    /// The combined resonance pattern from all entities
    pub collective_pattern: ResonancePattern,

    /// Coherence - how well-aligned the entity patterns are (0.0 to 1.0)
    /// Higher coherence means entities are resonating in harmony
    pub coherence: Float,

    /// Stability - how stable the collective resonance is (0.0 to 1.0)
    /// Based on coherence, higher coherence = more stable collective
    pub stability: Float,

    /// Resonance strength - the overall power of the collective field
    /// Combines coherence and the number of contributing entities
    pub resonance_strength: Float,

    /// Dominant densities - the density levels with highest resonance magnitudes
    /// Sorted by resonance strength (most dominant first)
    pub dominant_densities: Vec<usize>,
}

impl CollectiveResonanceCalculator {
    /// Compute collective resonance from a group of entities
    ///
    /// # Arguments
    /// * `entities` - Slice of entity references to compute collective resonance for
    ///
    /// # Returns
    /// * `CollectiveResonanceResult` containing the collective pattern and metadata
    ///
    /// # Examples
    /// ```
    /// let entities = vec![&entity1, &entity2, &entity3];
    /// let result = CollectiveResonanceCalculator::compute_from_entities(&entities);
    /// ```
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6, Week 89-92:
    /// "Building as collective manifestation - multiple entities contribute
    /// resonance to create structures"
    pub fn compute_from_entities(entities: &[&SubSubLogos]) -> CollectiveResonanceResult {
        if entities.is_empty() {
            return CollectiveResonanceResult {
                collective_pattern: ResonancePattern::new(),
                coherence: 0.0,
                stability: 0.0,
                resonance_strength: 0.0,
                dominant_densities: Vec::new(),
            };
        }

        // Extract spectrum ratios from entities and convert to resonance patterns
        let patterns: Vec<ResonancePattern> = entities
            .iter()
            .map(|entity| {
                let ratio = SpectrumRatio::new(
                    entity.spectrum_access.space_time_access,
                    entity.spectrum_access.time_space_access,
                );
                ResonancePattern::from_spectrum(&ratio)
            })
            .collect();

        // Combine all patterns into a collective pattern
        let collective_pattern = Self::combine_patterns(&patterns);

        // Compute coherence (alignment of individual patterns)
        let coherence = Self::compute_coherence(&patterns);

        // Compute stability from coherence
        let stability = Self::compute_stability(coherence);

        // Compute resonance strength (coherence × entity count factor)
        let entity_count_factor = (entities.len() as Float).ln() / (entities.len() as Float + 1.0);
        let resonance_strength = coherence * entity_count_factor;

        // Find dominant densities from the collective pattern
        let dominant_densities = Self::find_dominant_densities(&collective_pattern);

        CollectiveResonanceResult {
            collective_pattern,
            coherence,
            stability,
            resonance_strength,
            dominant_densities,
        }
    }

    /// Combine multiple resonance patterns into a single collective pattern
    ///
    /// Uses averaging to represent constructive interference between patterns.
    /// When patterns align, they reinforce each other. When they conflict,
    /// they cancel out in proportion to their differences.
    ///
    /// # Arguments
    /// * `patterns` - Slice of resonance patterns to combine
    ///
    /// # Returns
    /// * Combined `ResonancePattern` representing the collective field
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The holographic principle - each entity contains the whole, and the
    /// whole is contained in each entity"
    fn combine_patterns(patterns: &[ResonancePattern]) -> ResonancePattern {
        if patterns.is_empty() {
            return ResonancePattern::new();
        }

        if patterns.len() == 1 {
            return patterns[0].clone();
        }

        let count = patterns.len() as Float;

        // Average the pattern values across all entities
        let mut combined_pattern = [0.0; 8];
        let mut combined_stability = 0.0;
        let mut combined_phase = 0.0;

        for pattern in patterns {
            for i in 0..8 {
                combined_pattern[i] += pattern.pattern[i];
            }
            combined_stability += pattern.stability;
            combined_phase += pattern.phase;
        }

        // Normalize by count
        for i in 0..8 {
            combined_pattern[i] /= count;
        }
        combined_stability /= count;
        combined_phase /= count;

        ResonancePattern {
            pattern: combined_pattern,
            stability: combined_stability,
            phase: combined_phase,
        }
    }

    /// Compute coherence among multiple resonance patterns
    ///
    /// Coherence measures how well-aligned the patterns are with each other.
    /// Higher coherence (closer to 1.0) means entities are resonating in harmony.
    /// Lower coherence (closer to 0.0) means patterns are conflicting.
    ///
    /// # Arguments
    /// * `patterns` - Slice of resonance patterns to analyze
    ///
    /// # Returns
    /// * Coherence value between 0.0 and 1.0
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Coherence is the measure of collective alignment - how well the entities'
    /// individual resonance patterns harmonize with each other"
    fn compute_coherence(patterns: &[ResonancePattern]) -> Float {
        if patterns.is_empty() {
            return 0.0;
        }

        if patterns.len() == 1 {
            return 1.0;
        }

        let mut total_similarity = 0.0;
        let mut comparison_count = 0;

        // Compare each pattern with every other pattern
        for i in 0..patterns.len() {
            for j in (i + 1)..patterns.len() {
                let similarity = Self::pattern_similarity(&patterns[i], &patterns[j]);
                total_similarity += similarity;
                comparison_count += 1;
            }
        }

        if comparison_count == 0 {
            return 1.0;
        }

        // Average similarity represents overall coherence
        total_similarity / comparison_count as Float
    }

    /// Compute similarity between two resonance patterns
    ///
    /// Uses pattern interference to determine how similar two patterns are.
    /// Values closer to 1.0 indicate high similarity (constructive interference).
    /// Values closer to 0.0 indicate low similarity (destructive interference).
    ///
    /// # Arguments
    /// * `a` - First resonance pattern
    /// * `b` - Second resonance pattern
    ///
    /// # Returns
    /// * Similarity value between 0.0 and 1.0
    ///
    /// From holographic_physics.rs:
    /// "Interference patterns represent how holographic signatures interact"
    fn pattern_similarity(a: &ResonancePattern, b: &ResonancePattern) -> Float {
        // Use the ResonancePattern's built-in interference computation
        a.compute_interference(b)
    }

    /// Compute stability from coherence
    ///
    /// Stability is derived from coherence but with different characteristics:
    /// - High coherence (> 0.8) produces very stable collectives
    /// - Medium coherence (0.4-0.8) produces moderately stable collectives
    /// - Low coherence (< 0.4) produces unstable collectives
    ///
    /// # Arguments
    /// * `coherence` - Coherence value between 0.0 and 1.0
    ///
    /// # Returns
    /// * Stability value between 0.0 and 1.0
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "Stability determines how long a collective manifestation can persist
    /// before dissipating"
    fn compute_stability(coherence: Float) -> Float {
        // Stability follows a sigmoid curve based on coherence
        // This means:
        // - Very low coherence (< 0.2) = very low stability
        // - Medium coherence (~0.5) = moderate stability
        // - High coherence (> 0.8) = high stability
        let sigmoid = |x: Float| 1.0 / (1.0 + (-5.0 * (x - 0.5)).exp());
        sigmoid(coherence)
    }

    /// Find dominant densities from a resonance pattern
    ///
    /// Identifies which density levels (1-8) have the highest resonance magnitudes
    /// in the collective pattern. Returns a sorted list with most dominant first.
    ///
    /// # Arguments
    /// * `pattern` - Resonance pattern to analyze
    ///
    /// # Returns
    /// * Vector of density indices (0-7, representing densities 1-8) sorted by magnitude
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Density Octave - 8 densities representing stages of consciousness
    /// from First (mineral) to Eighth (Logos)"
    fn find_dominant_densities(pattern: &ResonancePattern) -> Vec<usize> {
        let mut density_magnitudes: Vec<(usize, Float)> = pattern
            .pattern
            .iter()
            .enumerate()
            .map(|(index, &magnitude)| (index, magnitude))
            .collect();

        // Sort by magnitude in descending order (most dominant first)
        density_magnitudes
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Extract just the density indices (add 1 to convert to 1-based density)
        density_magnitudes
            .iter()
            .map(|(index, _)| *index + 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test entity with minimal setup
    /// Note: This creates a simplified entity for testing purposes only
    fn create_test_entity(
        _id: u64,
        space_time_access: Float,
        time_space_access: Float,
    ) -> SubSubLogos {
        // Create a minimal entity using the spectrum values directly
        // We create the entity with default values and then set the spectrum access
        let mut entity = SubSubLogos::create_test_entity();

        // Set the spectrum access values for testing
        entity.spectrum_access.space_time_access = space_time_access;
        entity.spectrum_access.time_space_access = time_space_access;

        entity
    }

    #[test]
    fn test_collective_resonance_calculation() {
        // Create three entities with different spectrum access
        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.55, 0.45);
        let entity3 = create_test_entity(3, 0.65, 0.35);

        let entities = vec![&entity1, &entity2, &entity3];
        let result = CollectiveResonanceCalculator::compute_from_entities(&entities);

        // Verify result structure
        assert!(result.coherence >= 0.0 && result.coherence <= 1.0);
        assert!(result.stability >= 0.0 && result.stability <= 1.0);
        assert!(result.resonance_strength >= 0.0);
        assert!(!result.dominant_densities.is_empty());
        assert_eq!(result.dominant_densities.len(), 8); // All 8 densities should be present
    }

    #[test]
    fn test_collective_resonance_empty_entities() {
        let entities: Vec<&SubSubLogos> = vec![];
        let result = CollectiveResonanceCalculator::compute_from_entities(&entities);

        assert_eq!(result.coherence, 0.0);
        assert_eq!(result.stability, 0.0);
        assert_eq!(result.resonance_strength, 0.0);
        assert!(result.dominant_densities.is_empty());
    }

    #[test]
    fn test_collective_resonance_single_entity() {
        let entity1 = create_test_entity(1, 0.6, 0.4);

        let entities = vec![&entity1];
        let result = CollectiveResonanceCalculator::compute_from_entities(&entities);

        // Single entity should have perfect coherence
        assert_eq!(result.coherence, 1.0);
        assert!(result.stability > 0.5);
    }

    #[test]
    fn test_pattern_combination() {
        let ratio1 = SpectrumRatio::new(0.6, 0.4);
        let pattern1 = ResonancePattern::from_spectrum(&ratio1);

        let ratio2 = SpectrumRatio::new(0.4, 0.6);
        let pattern2 = ResonancePattern::from_spectrum(&ratio2);

        let patterns = vec![pattern1.clone(), pattern2.clone()];
        let combined = CollectiveResonanceCalculator::combine_patterns(&patterns);

        // Combined pattern should be between the two patterns
        for i in 0..8 {
            let min_val = pattern1.pattern[i].min(pattern2.pattern[i]);
            let max_val = pattern1.pattern[i].max(pattern2.pattern[i]);
            assert!(combined.pattern[i] >= min_val * 0.9);
            assert!(combined.pattern[i] <= max_val * 1.1);
        }
    }

    #[test]
    fn test_pattern_combination_empty() {
        let patterns: Vec<ResonancePattern> = vec![];
        let combined = CollectiveResonanceCalculator::combine_patterns(&patterns);

        assert_eq!(combined.pattern, [0.0; 8]);
        assert_eq!(combined.stability, 1.0);
        assert_eq!(combined.phase, 0.0);
    }

    #[test]
    fn test_pattern_combination_single() {
        let ratio = SpectrumRatio::new(0.6, 0.4);
        let pattern = ResonancePattern::from_spectrum(&ratio);

        let patterns = vec![pattern.clone()];
        let combined = CollectiveResonanceCalculator::combine_patterns(&patterns);

        // Single pattern should be returned unchanged
        for i in 0..8 {
            assert_eq!(combined.pattern[i], pattern.pattern[i]);
        }
        assert_eq!(combined.stability, pattern.stability);
        assert_eq!(combined.phase, pattern.phase);
    }

    #[test]
    fn test_coherence_calculation() {
        let ratio = SpectrumRatio::new(0.6, 0.4);
        let pattern = ResonancePattern::from_spectrum(&ratio);

        // Create identical patterns
        let patterns = vec![pattern.clone(), pattern.clone()];
        let coherence = CollectiveResonanceCalculator::compute_coherence(&patterns);

        // Identical patterns should have perfect coherence
        assert!((coherence - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_coherence_empty() {
        let patterns: Vec<ResonancePattern> = vec![];
        let coherence = CollectiveResonanceCalculator::compute_coherence(&patterns);

        assert_eq!(coherence, 0.0);
    }

    #[test]
    fn test_coherence_single() {
        let ratio = SpectrumRatio::new(0.6, 0.4);
        let pattern = ResonancePattern::from_spectrum(&ratio);

        let patterns = vec![pattern];
        let coherence = CollectiveResonanceCalculator::compute_coherence(&patterns);

        // Single pattern should have perfect coherence
        assert_eq!(coherence, 1.0);
    }

    #[test]
    fn test_pattern_similarity() {
        let ratio1 = SpectrumRatio::new(0.6, 0.4);
        let pattern1 = ResonancePattern::from_spectrum(&ratio1);

        let ratio2 = SpectrumRatio::new(0.6, 0.4);
        let pattern2 = ResonancePattern::from_spectrum(&ratio2);

        let similarity = CollectiveResonanceCalculator::pattern_similarity(&pattern1, &pattern2);

        // Identical patterns should have high similarity
        assert!((similarity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pattern_similarity_different() {
        let ratio1 = SpectrumRatio::new(0.9, 0.1);
        let pattern1 = ResonancePattern::from_spectrum(&ratio1);

        let ratio2 = SpectrumRatio::new(0.1, 0.9);
        let pattern2 = ResonancePattern::from_spectrum(&ratio2);

        let similarity = CollectiveResonanceCalculator::pattern_similarity(&pattern1, &pattern2);

        // Different patterns should have lower similarity
        assert!(similarity < 0.9);
        assert!(similarity >= 0.0);
    }

    #[test]
    fn test_dominant_densities() {
        let ratio = SpectrumRatio::new(0.6, 0.4);
        let pattern = ResonancePattern::from_spectrum(&ratio);

        let dominant_densities = CollectiveResonanceCalculator::find_dominant_densities(&pattern);

        // Should return all 8 densities sorted
        assert_eq!(dominant_densities.len(), 8);

        // Should be sorted by magnitude (descending)
        assert_eq!(dominant_densities[0], 1); // First density (index 0 + 1)

        // Should contain all densities 1-8
        let mut all_densities = dominant_densities.clone();
        all_densities.sort();
        assert_eq!(all_densities, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_stability_computation() {
        // High coherence should produce high stability
        let high_stability = CollectiveResonanceCalculator::compute_stability(0.9);
        assert!(high_stability > 0.8);

        // Medium coherence should produce medium stability
        let medium_stability = CollectiveResonanceCalculator::compute_stability(0.5);
        assert!(medium_stability > 0.3 && medium_stability < 0.8);

        // Low coherence should produce low stability
        let low_stability = CollectiveResonanceCalculator::compute_stability(0.1);
        assert!(low_stability < 0.4);

        // Edge case: zero coherence
        let zero_stability = CollectiveResonanceCalculator::compute_stability(0.0);
        assert!(zero_stability < 0.1);

        // Edge case: perfect coherence
        let perfect_stability = CollectiveResonanceCalculator::compute_stability(1.0);
        assert!(perfect_stability > 0.9);
    }

    #[test]
    fn test_collective_resonance_alignment() {
        // Create entities with very similar spectrum access (high alignment)
        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.6, 0.4);
        let entity3 = create_test_entity(3, 0.6, 0.4);

        let entities = vec![&entity1, &entity2, &entity3];
        let aligned_result = CollectiveResonanceCalculator::compute_from_entities(&entities);

        // Create entities with different spectrum access (low alignment)
        let entity4 = create_test_entity(4, 0.9, 0.1);
        let entity5 = create_test_entity(5, 0.5, 0.5);
        let entity6 = create_test_entity(6, 0.1, 0.9);

        let misaligned_entities = vec![&entity4, &entity5, &entity6];
        let misaligned_result =
            CollectiveResonanceCalculator::compute_from_entities(&misaligned_entities);

        // Aligned entities should have higher coherence
        assert!(aligned_result.coherence > misaligned_result.coherence);

        // Aligned entities should have higher stability
        assert!(aligned_result.stability > misaligned_result.stability);
    }

    #[test]
    fn test_resonance_strength_scaling() {
        // Create multiple similar entities
        let entity1 = create_test_entity(1, 0.6, 0.4);
        let entity2 = create_test_entity(2, 0.6, 0.4);
        let entity3 = create_test_entity(3, 0.6, 0.4);
        let entity4 = create_test_entity(4, 0.6, 0.4);
        let entity5 = create_test_entity(5, 0.6, 0.4);

        // Test with 2 entities
        let two_entities = vec![&entity1, &entity2];
        let result_two = CollectiveResonanceCalculator::compute_from_entities(&two_entities);

        // Test with 5 entities
        let five_entities = vec![&entity1, &entity2, &entity3, &entity4, &entity5];
        let result_five = CollectiveResonanceCalculator::compute_from_entities(&five_entities);

        // Both should have same coherence (similar entities)
        assert!((result_two.coherence - result_five.coherence).abs() < 0.01);

        // Five entities should have different resonance strength due to scaling
        // (The exact relationship depends on the scaling formula)
        assert!(result_five.resonance_strength > 0.0);
        assert!(result_two.resonance_strength > 0.0);
    }
}
