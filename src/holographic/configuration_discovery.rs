// Configuration Discovery Engine
//
// This module implements the discovery of stable configurations from holographic encoding.
// Stable configurations emerge at constructive interference nodes (bright fringes).
// Resonance with the holographic encoding determines stability.
//
// Key Principle:
// "Reality is not constructed; it is Unfolded from a Pre-Existing Whole"
// - We DISCOVER configurations through resonance, not derive them through formulas
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Section "Configuration Discovery Engine"
// - "Stable configurations are constructive interference nodes"

use crate::holographic::{
    ComplexVector, Configuration, ConfigurationClassification, HolographicField, InvolutionLayer,
    Position, StableConfiguration,
};
use crate::types::Float;

// ============================================================================
// CONFIGURATION DISCOVERY ENGINE
// ============================================================================

/// Engine for discovering stable configurations from holographic encoding.
///
/// The ConfigurationDiscoveryEngine discovers stable configurations by:
/// 1. Finding constructive interference nodes (bright fringes)
/// 2. Converting nodes to configurations
/// 3. Calculating resonance with holographic encoding
/// 4. Selecting configurations above resonance threshold
///
/// # Key Concept
///
/// **Resonance Instead of Energy Minimization**
/// - Traditional physics: Find minimum energy configurations
/// - Holographic physics: Find configurations that resonate with encoding
/// - Higher resonance = more stable configuration
///
/// # Example
///
/// ```rust
/// use holographic::configuration_discovery::ConfigurationDiscoveryEngine;
/// use holographic::holographic_field::{HolographicField, InvolutionLayer};
///
/// let archetypes = generate_test_archetypes();
/// let field = HolographicField::new(InvolutionLayer::Red, archetypes);
/// let engine = ConfigurationDiscoveryEngine::new();
///
/// let configurations = engine.discover_configurations(&field);
/// ```
#[derive(Clone, Debug)]
pub struct ConfigurationDiscoveryEngine {
    /// Resonance threshold for stable configurations (0.0 to 1.0)
    pub resonance_threshold: Float,
}

impl ConfigurationDiscoveryEngine {
    /// Creates a new configuration discovery engine with default threshold.
    ///
    /// # Returns
    ///
    /// A new engine with resonance threshold of 0.8
    pub fn new() -> Self {
        ConfigurationDiscoveryEngine {
            resonance_threshold: 0.8,
        }
    }

    /// Sets the resonance threshold for stable configurations.
    ///
    /// # Arguments
    ///
    /// * `threshold` - The resonance threshold (will be clamped to 0.0 to 1.0)
    pub fn set_resonance_threshold(&mut self, threshold: Float) {
        self.resonance_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Discovers stable configurations from a holographic field.
    ///
    /// This is the core method that implements holographic discovery:
    /// 1. Find accessible constructive nodes (bright fringes)
    /// 2. Convert each node to a configuration
    /// 3. Calculate resonance with holographic encoding
    /// 4. Filter by resonance threshold
    ///
    /// # Arguments
    ///
    /// * `field` - The holographic field to search for configurations
    ///
    /// # Returns
    ///
    /// Vector of stable configurations sorted by resonance score (descending)
    pub fn discover_configurations(&self, field: &HolographicField) -> Vec<StableConfiguration> {
        // Get accessible constructive nodes (stable configurations)
        let accessible_nodes = field.accessible_constructive_nodes();

        let mut configurations = Vec::new();

        for node in accessible_nodes {
            // Convert node position to configuration
            let config = self.node_to_configuration(node);

            // Calculate resonance with holographic encoding
            let resonance = self.calculate_resonance(&config, field);

            // Calculate interference alignment
            let interference_alignment = self.calculate_interference_alignment(&config, field);

            // Only keep configurations above resonance threshold
            if resonance >= self.resonance_threshold {
                configurations.push(StableConfiguration {
                    config,
                    resonance_score: resonance,
                    interference_alignment,
                    spatial_frequency: field.spatial_frequency,
                    layer: field.layer,
                });
            }
        }

        // Sort by resonance score (descending)
        configurations.sort_by(|a, b| b.resonance_score.partial_cmp(&a.resonance_score).unwrap());

        configurations
    }

    /// Calculates resonance between a configuration and holographic encoding.
    ///
    /// Resonance is calculated through cross-correlation with the holographic encoding.
    /// This is like holographic reconstruction - we're checking how well the
    /// configuration "reconstructs" the holographic encoding.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to test
    /// * `field` - The holographic field with encoding
    ///
    /// # Returns
    ///
    /// Resonance score (0.0 to 1.0, where 1.0 is perfect resonance)
    fn calculate_resonance(&self, config: &Configuration, field: &HolographicField) -> Float {
        // Convert configuration to complex vector representation
        let config_vector = self.config_to_complex_vector(config);

        // Calculate cross-correlation with holographic encoding
        let cross_correlation =
            self.calculate_cross_correlation(&config_vector, &field.archetype_complex_vectors);

        // Normalize to 0.0-1.0 range
        let amplitude = cross_correlation.amplitude();
        let max_possible_amplitude = self.max_possible_amplitude(&field.archetype_complex_vectors);

        if max_possible_amplitude > 0.0 {
            (amplitude / max_possible_amplitude).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Calculates cross-correlation between configuration and holographic encoding.
    ///
    /// Cross-correlation: Σ(config * encoding_i^*)
    /// This measures how well the configuration matches the holographic encoding.
    ///
    /// # Arguments
    ///
    /// * `config_vector` - The configuration as a complex vector
    /// * `encoding` - The holographic encoding (22 archetype vectors)
    ///
    /// # Returns
    ///
    /// Cross-correlation complex vector
    fn calculate_cross_correlation(
        &self,
        config_vector: &ComplexVector,
        encoding: &[ComplexVector; 22],
    ) -> ComplexVector {
        let mut result = ComplexVector {
            real: 0.0,
            imag: 0.0,
        };

        for archetype_vector in encoding {
            // Multiply by conjugate (holographic reconstruction)
            let conjugate = ComplexVector {
                real: archetype_vector.real,
                imag: -archetype_vector.imag,
            };

            // Accumulate cross-correlation
            result.real +=
                config_vector.real * conjugate.real - config_vector.imag * conjugate.imag;
            result.imag +=
                config_vector.real * conjugate.imag + config_vector.imag * conjugate.real;
        }

        result
    }

    /// Calculates the maximum possible amplitude for cross-correlation.
    ///
    /// This is used to normalize resonance scores to 0.0-1.0 range.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The holographic encoding
    ///
    /// # Returns
    ///
    /// Maximum possible amplitude
    fn max_possible_amplitude(&self, encoding: &[ComplexVector; 22]) -> Float {
        let mut total_amplitude = 0.0;

        for archetype_vector in encoding {
            total_amplitude += archetype_vector.amplitude();
        }

        total_amplitude
    }

    /// Calculates interference alignment for a configuration.
    ///
    /// Interference alignment measures how well the configuration aligns
    /// with the interference pattern structure.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to test
    /// * `field` - The holographic field
    ///
    /// # Returns
    ///
    /// Interference alignment score (0.0 to 1.0)
    fn calculate_interference_alignment(
        &self,
        config: &Configuration,
        field: &HolographicField,
    ) -> Float {
        let _config_vector = self.config_to_complex_vector(config);

        // Check if configuration is at a constructive node
        for node in &field.interference_pattern.constructive_nodes {
            if config.position == *node {
                return 1.0; // Perfect alignment
            }
        }

        // If not at exact node, calculate proximity to nearest constructive node
        if field.interference_pattern.constructive_nodes.is_empty() {
            return 0.0;
        }

        let nearest_node = field
            .interference_pattern
            .constructive_nodes
            .iter()
            .min_by(|a, b| {
                config
                    .position
                    .distance_to(a)
                    .partial_cmp(&config.position.distance_to(b))
                    .unwrap()
            })
            .unwrap();

        let distance = config.position.distance_to(nearest_node);
        let max_distance = Position::new(1.0, 1.0, 1.0).distance_to(&Position::origin());

        // Inverse distance: closer = higher alignment
        if max_distance > 0.0 {
            (1.0 - distance / max_distance).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Converts a node position to a configuration.
    ///
    /// # Arguments
    ///
    /// * `node` - The node position
    ///
    /// # Returns
    ///
    /// Configuration with position and internal structure
    fn node_to_configuration(&self, node: Position) -> Configuration {
        // Internal structure is derived from position (simplified)
        let internal_structure = vec![
            node.x,
            node.y,
            node.z,
            node.norm(),
            node.spatial_frequency(),
        ];

        Configuration {
            position: node,
            internal_structure,
        }
    }

    /// Converts a configuration to a complex vector.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to convert
    ///
    /// # Returns
    ///
    /// Complex vector representation
    fn config_to_complex_vector(&self, config: &Configuration) -> ComplexVector {
        // Real part = position norm (spatial magnitude)
        // Imag part = internal structure sum (internal complexity)
        let real = config.position.norm();
        let imag: Float = config.internal_structure.iter().sum();

        ComplexVector { real, imag }
    }

    /// Classifies configurations by involution layer.
    ///
    /// Configurations are classified based on where they were discovered:
    /// - Violet/Indigo/Blue: Particles (subatomic)
    /// - Green/Yellow: Atoms (atomic)
    /// - Orange/Red: Molecules (molecular)
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations to classify
    ///
    /// # Returns
    ///
    /// Classification of configurations
    pub fn classify_configurations(
        &self,
        configurations: &[StableConfiguration],
    ) -> ConfigurationClassification {
        let mut particles = Vec::new();
        let mut atoms = Vec::new();
        let mut molecules = Vec::new();

        for config in configurations {
            match config.layer {
                InvolutionLayer::Violet | InvolutionLayer::Indigo | InvolutionLayer::Blue => {
                    particles.push(config.clone());
                }
                InvolutionLayer::Green | InvolutionLayer::Yellow => {
                    atoms.push(config.clone());
                }
                InvolutionLayer::Orange | InvolutionLayer::Red => {
                    molecules.push(config.clone());
                }
            }
        }

        ConfigurationClassification {
            particles,
            atoms,
            molecules,
        }
    }

    /// Finds the most stable configuration (highest resonance).
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations to search
    ///
    /// # Returns
    ///
    /// The most stable configuration, or None if empty
    pub fn find_most_stable(
        &self,
        configurations: &[StableConfiguration],
    ) -> Option<StableConfiguration> {
        configurations
            .iter()
            .max_by(|a, b| a.resonance_score.partial_cmp(&b.resonance_score).unwrap())
            .cloned()
    }

    /// Filters configurations by minimum resonance score.
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations to filter
    /// * `min_resonance` - Minimum resonance score
    ///
    /// # Returns
    ///
    /// Filtered configurations
    pub fn filter_by_resonance(
        &self,
        configurations: &[StableConfiguration],
        min_resonance: Float,
    ) -> Vec<StableConfiguration> {
        configurations
            .iter()
            .filter(|c| c.resonance_score >= min_resonance)
            .cloned()
            .collect()
    }

    /// Calculates average resonance score.
    ///
    /// # Arguments
    ///
    /// * `configurations` - The configurations to analyze
    ///
    /// # Returns
    ///
    /// Average resonance score, or 0.0 if empty
    pub fn average_resonance(&self, configurations: &[StableConfiguration]) -> Float {
        if configurations.is_empty() {
            return 0.0;
        }

        let sum: Float = configurations.iter().map(|c| c.resonance_score).sum();
        sum / configurations.len() as Float
    }
}

impl Default for ConfigurationDiscoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic::complex_vectors::ComplexArchetype;
    use std::f64::consts::PI;

    /// Generates test archetypes with reasonable values
    fn generate_test_archetypes() -> [ComplexArchetype; 22] {
        let mut archetypes = [ComplexArchetype {
            amplitude: 0.0,
            phase: 0.0,
        }; 22];
        for i in 0..22 {
            archetypes[i] = ComplexArchetype {
                amplitude: (i as Float + 1.0) / 22.0, // 0.045 to 1.0
                phase: (i as Float) * PI / 11.0,      // 0 to 2π
            };
        }
        archetypes
    }

    #[test]
    fn test_configuration_discovery_engine_creation() {
        let engine = ConfigurationDiscoveryEngine::new();
        assert_eq!(engine.resonance_threshold, 0.8);
    }

    #[test]
    fn test_configuration_discovery_engine_default() {
        let engine = ConfigurationDiscoveryEngine::default();
        assert_eq!(engine.resonance_threshold, 0.8);
    }

    #[test]
    fn test_set_resonance_threshold() {
        let mut engine = ConfigurationDiscoveryEngine::new();
        engine.set_resonance_threshold(0.9);
        assert_eq!(engine.resonance_threshold, 0.9);

        engine.set_resonance_threshold(1.5); // Should be clamped to 1.0
        assert_eq!(engine.resonance_threshold, 1.0);

        engine.set_resonance_threshold(-0.5); // Should be clamped to 0.0
        assert_eq!(engine.resonance_threshold, 0.0);
    }

    #[test]
    fn test_discover_configurations() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let configurations = engine.discover_configurations(&field);

        // Should discover some configurations
        assert!(!configurations.is_empty());

        // All configurations should have resonance above threshold
        for config in &configurations {
            assert!(config.resonance_score >= 0.8);
        }

        // Configurations should be sorted by resonance (descending)
        for i in 1..configurations.len() {
            assert!(configurations[i - 1].resonance_score >= configurations[i].resonance_score);
        }
    }

    #[test]
    fn test_discover_configurations_with_low_threshold() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let mut engine = ConfigurationDiscoveryEngine::new();
        engine.set_resonance_threshold(0.1);

        let configurations = engine.discover_configurations(&field);

        // Should discover more configurations with lower threshold
        assert!(!configurations.is_empty());

        // All configurations should have resonance above threshold
        for config in &configurations {
            assert!(config.resonance_score >= 0.1);
        }
    }

    #[test]
    fn test_discover_configurations_at_different_layers() {
        let archetypes = generate_test_archetypes();
        let engine = ConfigurationDiscoveryEngine::new();

        for layer in InvolutionLayer::all_layers() {
            let field = HolographicField::new(layer, archetypes);
            let configurations = engine.discover_configurations(&field);

            // Should discover configurations at each layer
            assert!(!configurations.is_empty());

            // All configurations should be at the correct layer
            for config in &configurations {
                assert_eq!(config.layer, layer);
            }
        }
    }

    #[test]
    fn test_classify_configurations() {
        let archetypes = generate_test_archetypes();
        let engine = ConfigurationDiscoveryEngine::new();

        let mut all_configs = Vec::new();

        for layer in [
            InvolutionLayer::Violet,
            InvolutionLayer::Green,
            InvolutionLayer::Red,
        ] {
            let field = HolographicField::new(layer, archetypes.clone());
            let configs = engine.discover_configurations(&field);
            all_configs.extend(configs);
        }

        let classification = engine.classify_configurations(&all_configs);

        // Should classify configurations correctly
        assert!(!classification.particles.is_empty());
        assert!(!classification.atoms.is_empty());
        assert!(!classification.molecules.is_empty());
    }

    #[test]
    fn test_find_most_stable() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let configurations = engine.discover_configurations(&field);

        let most_stable = engine.find_most_stable(&configurations);

        assert!(most_stable.is_some());

        if let Some(stable) = most_stable {
            // Should have the highest resonance score
            assert_eq!(stable.resonance_score, configurations[0].resonance_score);
        }
    }

    #[test]
    fn test_filter_by_resonance() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let configurations = engine.discover_configurations(&field);

        // Filter by high resonance
        let high_resonance = engine.filter_by_resonance(&configurations, 0.9);
        assert!(high_resonance.len() <= configurations.len());

        for config in &high_resonance {
            assert!(config.resonance_score >= 0.9);
        }
    }

    #[test]
    fn test_average_resonance() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let configurations = engine.discover_configurations(&field);

        let avg = engine.average_resonance(&configurations);

        // Average should be between min and max
        if !configurations.is_empty() {
            let min = configurations
                .iter()
                .map(|c| c.resonance_score)
                .fold(Float::INFINITY, |a, b| a.min(b));
            let max = configurations
                .iter()
                .map(|c| c.resonance_score)
                .fold(Float::NEG_INFINITY, |a, b| a.max(b));

            assert!(avg >= min);
            assert!(avg <= max);
        }
    }

    #[test]
    fn test_resonance_calculation() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let config = Configuration {
            position: Position::new(0.5, 0.5, 0.5),
            internal_structure: vec![0.5, 0.5, 0.5, 0.866, 866.0],
        };

        let resonance = engine.calculate_resonance(&config, &field);

        // Resonance should be between 0.0 and 1.0
        assert!(resonance >= 0.0);
        assert!(resonance <= 1.0);
    }

    #[test]
    fn test_interference_alignment() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let config = Configuration {
            position: Position::new(0.5, 0.5, 0.5),
            internal_structure: vec![0.5, 0.5, 0.5, 0.866, 866.0],
        };

        let alignment = engine.calculate_interference_alignment(&config, &field);

        // Alignment should be between 0.0 and 1.0
        assert!(alignment >= 0.0);
        assert!(alignment <= 1.0);
    }

    #[test]
    fn test_node_to_configuration() {
        let engine = ConfigurationDiscoveryEngine::new();
        let node = Position::new(0.5, 0.5, 0.5);

        let config = engine.node_to_configuration(node);

        assert_eq!(config.position, node);
        assert_eq!(config.internal_structure.len(), 5);
    }

    #[test]
    fn test_config_to_complex_vector() {
        let engine = ConfigurationDiscoveryEngine::new();
        let config = Configuration {
            position: Position::new(3.0, 4.0, 0.0),
            internal_structure: vec![1.0, 2.0, 3.0],
        };

        let cv = engine.config_to_complex_vector(&config);

        // Real part should be position norm (5.0)
        assert!((cv.real - 5.0).abs() < 1e-10);

        // Imag part should be sum of internal structure (6.0)
        assert!((cv.imag - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_cross_correlation() {
        let engine = ConfigurationDiscoveryEngine::new();
        let config_vector = ComplexVector::from_amplitude_phase(1.0, PI / 4.0);

        let mut encoding = [ComplexVector::default(); 22];
        for i in 0..22 {
            encoding[i] = ComplexVector::from_amplitude_phase(0.5, (i as Float) * PI / 11.0);
        }

        let cross_correlation = engine.calculate_cross_correlation(&config_vector, &encoding);

        // Cross-correlation should have some magnitude
        assert!(cross_correlation.amplitude() > 0.0);
    }

    #[test]
    fn test_configuration_stability() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Red, archetypes);
        let engine = ConfigurationDiscoveryEngine::new();

        let configurations = engine.discover_configurations(&field);

        // Check that all configurations have valid properties
        for config in &configurations {
            assert!(config.resonance_score >= 0.0);
            assert!(config.resonance_score <= 1.0);
            assert!(config.interference_alignment >= 0.0);
            assert!(config.interference_alignment <= 1.0);
            assert!(config.spatial_frequency > 0.0);
        }
    }
}
