// Universe Explorer - Phase 6: Novel Discovery, Task 6.1
//
// This module implements a system to explore variant holographic architectures
// and discover novel physics.
//
// Key Components:
// - UniverseExplorer: Explores variant architectures
// - ArchitectureVariant: Different types of architecture variants
// - ExplorationResult: Result of exploring a variant
// - VariantComparison: Comparison of multiple variants
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 6, Task 6.1
// - "Implement system to explore variant holographic architectures"

use crate::exploration::discovery_database::{
    DiscoveryDatabase, DiscoveryRecord, NoveltyScore, StableConfiguration,
};
use crate::types::Float;

// ============================================================================
// ARCHITECTURE VARIANT
// ============================================================================

/// Architecture Variant
///
/// Represents a variant of the holographic architecture to explore.
#[derive(Debug, Clone)]
pub enum ArchitectureVariant {
    /// Variant with different physical laws
    DifferentPhysicalLaws(PhysicalLawsVariant),

    /// Variant with different planetary bias
    DifferentPlanetaryBias(PlanetaryBiasVariant),

    /// Variant with different soul stream
    DifferentSoulStream(SoulStreamVariant),

    /// Variant with different spatial frequency mapping
    DifferentSpatialFrequency(SpatialFrequencyVariant),
}

/// Physical Laws Variant
///
/// Variant with modified physical laws.
#[derive(Debug, Clone)]
pub struct PhysicalLawsVariant {
    /// Gravity modifier (1.0 = standard)
    pub gravity_modifier: Float,

    /// Electromagnetic force modifier (1.0 = standard)
    pub em_force_modifier: Float,

    /// Strong force modifier (1.0 = standard)
    pub strong_force_modifier: Float,

    /// Weak force modifier (1.0 = standard)
    pub weak_force_modifier: Float,

    /// Speed of light modifier (1.0 = standard)
    pub speed_of_light_modifier: Float,

    /// Planck constant modifier (1.0 = standard)
    pub planck_constant_modifier: Float,
}

impl PhysicalLawsVariant {
    /// Create a new physical laws variant
    pub fn new(
        gravity_modifier: Float,
        em_force_modifier: Float,
        strong_force_modifier: Float,
        weak_force_modifier: Float,
    ) -> Self {
        Self {
            gravity_modifier,
            em_force_modifier,
            strong_force_modifier,
            weak_force_modifier,
            speed_of_light_modifier: 1.0,
            planck_constant_modifier: 1.0,
        }
    }

    /// Create a variant with modified gravity
    pub fn modified_gravity(modifier: Float) -> Self {
        Self::new(modifier, 1.0, 1.0, 1.0)
    }
}

/// Planetary Bias Variant
///
/// Variant with different planetary biases.
#[derive(Debug, Clone)]
pub struct PlanetaryBiasVariant {
    /// Atmospheric composition modifier
    pub atmosphere_modifier: Float,

    /// EM field modifier
    pub em_field_modifier: Float,

    /// Mineral composition modifier
    pub mineral_modifier: Float,

    /// Radiation levels modifier
    pub radiation_modifier: Float,
}

impl PlanetaryBiasVariant {
    /// Create a new planetary bias variant
    pub fn new(
        atmosphere_modifier: Float,
        em_field_modifier: Float,
        mineral_modifier: Float,
        radiation_modifier: Float,
    ) -> Self {
        Self {
            atmosphere_modifier,
            em_field_modifier,
            mineral_modifier,
            radiation_modifier,
        }
    }

    /// Create a variant with altered atmosphere
    pub fn altered_atmosphere(modifier: Float) -> Self {
        Self::new(modifier, 1.0, 1.0, 1.0)
    }
}

/// Soul Stream Variant
///
/// Variant with different soul stream characteristics.
#[derive(Debug, Clone)]
pub struct SoulStreamVariant {
    /// Karmic pattern modifier
    pub karmic_modifier: Float,

    /// Polarity modifier
    pub polarization_modifier: Float,

    /// Evolutionary goal modifier
    pub evolutionary_modifier: Float,
}

impl SoulStreamVariant {
    /// Create a new soul stream variant
    pub fn new(
        karmic_modifier: Float,
        polarization_modifier: Float,
        evolutionary_modifier: Float,
    ) -> Self {
        Self {
            karmic_modifier,
            polarization_modifier,
            evolutionary_modifier,
        }
    }
}

/// Spatial Frequency Variant
///
/// Variant with different spatial frequency mappings.
#[derive(Debug, Clone)]
pub struct SpatialFrequencyVariant {
    /// Violet layer frequency modifier
    pub violet_modifier: Float,

    /// Indigo layer frequency modifier
    pub indigo_modifier: Float,

    /// Blue layer frequency modifier
    pub blue_modifier: Float,

    /// Green layer frequency modifier
    pub green_modifier: Float,

    /// Yellow layer frequency modifier
    pub yellow_modifier: Float,

    /// Orange layer frequency modifier
    pub orange_modifier: Float,

    /// Red layer frequency modifier
    pub red_modifier: Float,
}

impl SpatialFrequencyVariant {
    /// Create a new spatial frequency variant
    pub fn new(
        violet: Float,
        indigo: Float,
        blue: Float,
        green: Float,
        yellow: Float,
        orange: Float,
        red: Float,
    ) -> Self {
        Self {
            violet_modifier: violet,
            indigo_modifier: indigo,
            blue_modifier: blue,
            green_modifier: green,
            yellow_modifier: yellow,
            orange_modifier: orange,
            red_modifier: red,
        }
    }

    /// Create a uniform frequency modifier
    pub fn uniform_modifier(modifier: Float) -> Self {
        Self::new(
            modifier, modifier, modifier, modifier, modifier, modifier, modifier,
        )
    }
}

// ============================================================================
// UNIVERSE EXPLORER
// ============================================================================

/// Universe Explorer
///
/// Explores variant holographic architectures to discover novel physics.
#[derive(Debug, Clone)]
pub struct UniverseExplorer {
    /// Base architecture
    pub base_architecture: LayerArchitecture,

    /// Discovery database
    pub discovery_database: DiscoveryDatabase,

    /// Exploration statistics
    pub statistics: ExplorationStatistics,
}

/// Layer Architecture
///
/// Represents the 7-layer holographic architecture.
#[derive(Debug, Clone)]
pub struct LayerArchitecture {
    /// Yellow layer: Physical laws
    pub yellow_laws: PhysicalLawsVariant,

    /// Yellow layer: Planetary biases
    pub yellow_bias: PlanetaryBiasVariant,

    /// Orange layer: Soul stream
    pub orange_stream: SoulStreamVariant,

    /// Spatial frequency mapping for all layers
    pub spatial_frequencies: SpatialFrequencyVariant,
}

impl LayerArchitecture {
    /// Create an initial layer architecture
    pub fn initial() -> Self {
        Self {
            yellow_laws: PhysicalLawsVariant::new(1.0, 1.0, 1.0, 1.0),
            yellow_bias: PlanetaryBiasVariant::new(1.0, 1.0, 1.0, 1.0),
            orange_stream: SoulStreamVariant::new(1.0, 1.0, 1.0),
            spatial_frequencies: SpatialFrequencyVariant::uniform_modifier(1.0),
        }
    }
}

/// Exploration Statistics
#[derive(Debug, Clone, Default)]
pub struct ExplorationStatistics {
    /// Total variants explored
    pub total_variants_explored: usize,

    /// Total configurations discovered
    pub total_configurations_discovered: usize,

    /// Novel discoveries
    pub novel_discoveries: usize,

    /// Average novelty score
    pub average_novelty_score: Float,
}

impl UniverseExplorer {
    /// Create a new universe explorer
    pub fn new(base_architecture: LayerArchitecture) -> Self {
        Self {
            base_architecture,
            discovery_database: DiscoveryDatabase::new(),
            statistics: ExplorationStatistics::default(),
        }
    }

    /// Create a universe explorer with default architecture
    pub fn with_default_architecture() -> Self {
        Self::new(LayerArchitecture::initial())
    }

    /// Explore a single variant
    pub fn explore_variant(&mut self, variant: ArchitectureVariant) -> ExplorationResult {
        let variant_description = match &variant {
            ArchitectureVariant::DifferentPhysicalLaws(_) => "Different Physical Laws",
            ArchitectureVariant::DifferentPlanetaryBias(_) => "Different Planetary Bias",
            ArchitectureVariant::DifferentSoulStream(_) => "Different Soul Stream",
            ArchitectureVariant::DifferentSpatialFrequency(_) => "Different Spatial Frequency",
        };

        println!("Exploring variant: {}", variant_description);

        // Build architecture from variant
        let architecture = self.build_architecture_from_variant(variant);

        // Discover configurations from architecture
        let discovered_configurations = self.discover_configurations(&architecture);

        // Calculate novelty score
        let novelty_score = self.calculate_novelty_score(&discovered_configurations);

        // Calculate stability score
        let stability_score = self.calculate_stability_score(&discovered_configurations);

        // Create exploration result
        let result = ExplorationResult {
            variant_description: variant_description.to_string(),
            discovered_configurations: discovered_configurations.clone(),
            novelty_score,
            stability_score,
            physics_discovered: self.extract_physics(&discovered_configurations),
        };

        // Store in discovery database
        let record = DiscoveryRecord::new(
            format!("exploration_{}", self.statistics.total_variants_explored),
            variant_description.to_string(),
            format!("Explored variant {}", variant_description),
            novelty_score.overall,
            stability_score,
        );

        self.discovery_database
            .add_discovery(record)
            .expect("Failed to add discovery to database");

        // Store stable configurations
        for config in &discovered_configurations {
            if config.stability_score > 0.8 {
                self.discovery_database
                    .add_stable_configuration(config.clone())
                    .expect("Failed to add stable configuration");
            }
        }

        // Update statistics
        self.statistics.total_variants_explored += 1;
        self.statistics.total_configurations_discovered += discovered_configurations.len();
        if novelty_score.overall > 0.7 {
            self.statistics.novel_discoveries += 1;
        }
        self.statistics.average_novelty_score = (self.statistics.average_novelty_score
            * (self.statistics.total_variants_explored - 1) as Float
            + novelty_score.overall)
            / self.statistics.total_variants_explored as Float;

        result
    }

    /// Explore multiple variants
    pub fn explore_multiple_variants(
        &mut self,
        variants: Vec<ArchitectureVariant>,
    ) -> Vec<ExplorationResult> {
        variants
            .into_iter()
            .map(|variant| self.explore_variant(variant))
            .collect()
    }

    /// Compare multiple exploration results
    pub fn compare_variants(&self, results: &[ExplorationResult]) -> VariantComparison {
        if results.is_empty() {
            return VariantComparison::default();
        }

        let total_novelty: Float = results.iter().map(|r| r.novelty_score.overall).sum();
        let avg_novelty = total_novelty / results.len() as Float;

        let total_stability: Float = results.iter().map(|r| r.stability_score).sum();
        let avg_stability = total_stability / results.len() as Float;

        let total_configs: usize = results
            .iter()
            .map(|r| r.discovered_configurations.len())
            .sum();

        let most_novel = results
            .iter()
            .max_by(|a, b| {
                a.novelty_score
                    .overall
                    .partial_cmp(&b.novelty_score.overall)
                    .unwrap()
            })
            .map(|r| r.variant_description.clone())
            .unwrap_or_default();

        let most_stable = results
            .iter()
            .max_by(|a, b| a.stability_score.partial_cmp(&b.stability_score).unwrap())
            .map(|r| r.variant_description.clone())
            .unwrap_or_default();

        VariantComparison {
            total_variants: results.len(),
            total_configurations: total_configs,
            average_novelty: avg_novelty,
            average_stability: avg_stability,
            most_novel_variant: most_novel,
            most_stable_variant: most_stable,
        }
    }

    /// Build architecture from variant
    fn build_architecture_from_variant(&self, variant: ArchitectureVariant) -> LayerArchitecture {
        let mut architecture = self.base_architecture.clone();

        match variant {
            ArchitectureVariant::DifferentPhysicalLaws(laws) => {
                architecture.yellow_laws = laws;
            }
            ArchitectureVariant::DifferentPlanetaryBias(bias) => {
                architecture.yellow_bias = bias;
            }
            ArchitectureVariant::DifferentSoulStream(stream) => {
                architecture.orange_stream = stream;
            }
            ArchitectureVariant::DifferentSpatialFrequency(frequencies) => {
                architecture.spatial_frequencies = frequencies;
            }
        }

        architecture
    }

    /// Discover configurations from architecture
    fn discover_configurations(
        &self,
        architecture: &LayerArchitecture,
    ) -> Vec<StableConfiguration> {
        let mut configurations = Vec::new();

        // Generate test configurations based on architecture
        // In a full implementation, this would use holographic field discovery
        for i in 0..5 {
            let mut activation = [0.5; 22];

            // Modify activation based on architecture
            for act in &mut activation {
                *act *= architecture.yellow_laws.gravity_modifier;
                *act *= architecture.spatial_frequencies.violet_modifier;
            }

            // Add some variation
            activation[i % 22] = 0.8;
            activation[(i + 1) % 22] = 0.3;

            let config = StableConfiguration::new(
                format!("config_{}_{}", i, self.statistics.total_variants_explored),
                activation,
                100.0 * architecture.spatial_frequencies.violet_modifier,
                0.7 + (i as Float) * 0.05,
                0.75 + (i as Float) * 0.04,
            );

            configurations.push(config);
        }

        configurations
    }

    /// Calculate novelty score for configurations
    fn calculate_novelty_score(&self, configurations: &[StableConfiguration]) -> NoveltyScore {
        if configurations.is_empty() {
            return NoveltyScore::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        }

        // Calculate novelty based on deviation from baseline
        let mass_novelty =
            0.5 + (self.base_architecture.yellow_laws.gravity_modifier - 1.0).abs() * 0.5;
        let charge_novelty =
            0.5 + (self.base_architecture.yellow_laws.em_force_modifier - 1.0).abs() * 0.5;
        let spin_novelty = 0.5;
        let lifetime_novelty =
            0.5 + (self.base_architecture.yellow_laws.strong_force_modifier - 1.0).abs() * 0.5;
        let force_novelty =
            0.5 + (self.base_architecture.yellow_laws.weak_force_modifier - 1.0).abs() * 0.5;

        let overall =
            (mass_novelty + charge_novelty + spin_novelty + lifetime_novelty + force_novelty) / 5.0;

        NoveltyScore::new(
            overall,
            mass_novelty,
            charge_novelty,
            spin_novelty,
            lifetime_novelty,
            force_novelty,
        )
    }

    /// Calculate stability score for configurations
    fn calculate_stability_score(&self, configurations: &[StableConfiguration]) -> Float {
        if configurations.is_empty() {
            return 0.0;
        }

        configurations
            .iter()
            .map(|c| c.stability_score)
            .sum::<Float>()
            / configurations.len() as Float
    }

    /// Extract physics from configurations
    fn extract_physics(&self, _configurations: &[StableConfiguration]) -> Vec<String> {
        let mut physics = Vec::new();

        if self.base_architecture.yellow_laws.gravity_modifier != 1.0 {
            physics.push(format!(
                "Modified gravity: {}x standard",
                self.base_architecture.yellow_laws.gravity_modifier
            ));
        }

        if self.base_architecture.yellow_laws.em_force_modifier != 1.0 {
            physics.push(format!(
                "Modified EM force: {}x standard",
                self.base_architecture.yellow_laws.em_force_modifier
            ));
        }

        if self.base_architecture.yellow_laws.strong_force_modifier != 1.0 {
            physics.push(format!(
                "Modified strong force: {}x standard",
                self.base_architecture.yellow_laws.strong_force_modifier
            ));
        }

        if self.base_architecture.yellow_laws.weak_force_modifier != 1.0 {
            physics.push(format!(
                "Modified weak force: {}x standard",
                self.base_architecture.yellow_laws.weak_force_modifier
            ));
        }

        physics
    }

    /// Get discovery database reference
    pub fn get_discovery_database(&self) -> &DiscoveryDatabase {
        &self.discovery_database
    }

    /// Get discovery database reference (mutable)
    pub fn get_discovery_database_mut(&mut self) -> &mut DiscoveryDatabase {
        &mut self.discovery_database
    }

    /// Get exploration statistics
    pub fn get_statistics(&self) -> &ExplorationStatistics {
        &self.statistics
    }
}

// ============================================================================
// EXPLORATION RESULT
// ============================================================================

/// Exploration Result
///
/// Result of exploring a variant architecture.
#[derive(Debug, Clone)]
pub struct ExplorationResult {
    /// Description of the variant explored
    pub variant_description: String,

    /// Discovered configurations
    pub discovered_configurations: Vec<StableConfiguration>,

    /// Novelty score
    pub novelty_score: NoveltyScore,

    /// Stability score
    pub stability_score: Float,

    /// Physics discovered
    pub physics_discovered: Vec<String>,
}

// ============================================================================
// VARIANT COMPARISON
// ============================================================================

/// Variant Comparison
///
/// Comparison of multiple exploration results.
#[derive(Debug, Clone, Default)]
pub struct VariantComparison {
    /// Total number of variants compared
    pub total_variants: usize,

    /// Total number of configurations discovered
    pub total_configurations: usize,

    /// Average novelty score
    pub average_novelty: Float,

    /// Average stability score
    pub average_stability: Float,

    /// Most novel variant
    pub most_novel_variant: String,

    /// Most stable variant
    pub most_stable_variant: String,
}

impl VariantComparison {
    /// Create a variant comparison from results
    pub fn from_results(results: &[ExplorationResult]) -> Self {
        if results.is_empty() {
            return Self::default();
        }

        let total_novelty: Float = results.iter().map(|r| r.novelty_score.overall).sum();
        let avg_novelty = total_novelty / results.len() as Float;

        let total_stability: Float = results.iter().map(|r| r.stability_score).sum();
        let avg_stability = total_stability / results.len() as Float;

        let total_configs: usize = results
            .iter()
            .map(|r| r.discovered_configurations.len())
            .sum();

        let most_novel = results
            .iter()
            .max_by(|a, b| {
                a.novelty_score
                    .overall
                    .partial_cmp(&b.novelty_score.overall)
                    .unwrap()
            })
            .map(|r| r.variant_description.clone())
            .unwrap_or_default();

        let most_stable = results
            .iter()
            .max_by(|a, b| a.stability_score.partial_cmp(&b.stability_score).unwrap())
            .map(|r| r.variant_description.clone())
            .unwrap_or_default();

        Self {
            total_variants: results.len(),
            total_configurations: total_configs,
            average_novelty: avg_novelty,
            average_stability: avg_stability,
            most_novel_variant: most_novel,
            most_stable_variant: most_stable,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_explorer_creation() {
        let explorer = UniverseExplorer::with_default_architecture();
        assert_eq!(explorer.get_statistics().total_variants_explored, 0);
        assert_eq!(
            explorer
                .get_discovery_database()
                .get_statistics()
                .total_discoveries,
            0
        );
    }

    #[test]
    fn test_explore_variant() {
        let mut explorer = UniverseExplorer::with_default_architecture();

        let variant =
            ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant::modified_gravity(0.5));
        let result = explorer.explore_variant(variant);

        assert!(!result.discovered_configurations.is_empty());
        assert!(result.novelty_score.overall > 0.0);
        assert!(result.stability_score > 0.0);
        assert_eq!(explorer.get_statistics().total_variants_explored, 1);
    }

    #[test]
    fn test_explore_multiple_variants() {
        let mut explorer = UniverseExplorer::with_default_architecture();

        let variants = vec![
            ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant::modified_gravity(0.5)),
            ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant::modified_gravity(2.0)),
            ArchitectureVariant::DifferentPlanetaryBias(PlanetaryBiasVariant::altered_atmosphere(
                0.8,
            )),
        ];

        let results = explorer.explore_multiple_variants(variants);

        assert_eq!(results.len(), 3);
        assert_eq!(explorer.get_statistics().total_variants_explored, 3);

        for result in &results {
            assert!(!result.discovered_configurations.is_empty());
        }
    }

    #[test]
    fn test_compare_variants() {
        let mut explorer = UniverseExplorer::with_default_architecture();

        let variants = vec![
            ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant::modified_gravity(0.5)),
            ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant::modified_gravity(2.0)),
        ];

        let results = explorer.explore_multiple_variants(variants);
        let comparison = explorer.compare_variants(&results);

        assert_eq!(comparison.total_variants, 2);
        assert!(comparison.total_configurations > 0);
        assert!(comparison.average_novelty > 0.0);
        assert!(comparison.average_stability > 0.0);
    }

    #[test]
    fn test_physical_laws_variant() {
        let laws = PhysicalLawsVariant::new(0.5, 1.5, 2.0, 0.8);
        assert_eq!(laws.gravity_modifier, 0.5);
        assert_eq!(laws.em_force_modifier, 1.5);
        assert_eq!(laws.strong_force_modifier, 2.0);
        assert_eq!(laws.weak_force_modifier, 0.8);
    }

    #[test]
    fn test_planetary_bias_variant() {
        let bias = PlanetaryBiasVariant::altered_atmosphere(0.8);
        assert_eq!(bias.atmosphere_modifier, 0.8);
        assert_eq!(bias.em_field_modifier, 1.0);
    }

    #[test]
    fn test_soul_stream_variant() {
        let stream = SoulStreamVariant::new(0.7, 0.8, 0.9);
        assert_eq!(stream.karmic_modifier, 0.7);
        assert_eq!(stream.polarization_modifier, 0.8);
        assert_eq!(stream.evolutionary_modifier, 0.9);
    }

    #[test]
    fn test_spatial_frequency_variant() {
        let freq = SpatialFrequencyVariant::uniform_modifier(1.5);
        assert_eq!(freq.violet_modifier, 1.5);
        assert_eq!(freq.indigo_modifier, 1.5);
        assert_eq!(freq.blue_modifier, 1.5);
        assert_eq!(freq.green_modifier, 1.5);
        assert_eq!(freq.yellow_modifier, 1.5);
        assert_eq!(freq.orange_modifier, 1.5);
        assert_eq!(freq.red_modifier, 1.5);
    }

    #[test]
    fn test_layer_architecture() {
        let arch = LayerArchitecture::initial();
        assert_eq!(arch.yellow_laws.gravity_modifier, 1.0);
        assert_eq!(arch.yellow_bias.atmosphere_modifier, 1.0);
        assert_eq!(arch.orange_stream.karmic_modifier, 1.0);
    }
}
