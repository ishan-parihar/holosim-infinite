// Hypothesis Generator - Phase 6: Novel Discovery, Task 6.2
//
// This module implements a system to generate novel hypotheses from discoveries.
//
// Key Components:
// - HypothesisGenerator: Generates hypotheses from discoveries
// - PatternRecognizer: Recognizes patterns in discoveries
// - Hypothesis: A novel hypothesis about physics
// - TestablePrediction: A testable prediction from a hypothesis
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 6, Task 6.2
// - "Implement system to generate novel hypotheses from discoveries"

use crate::exploration::discovery_database::StableConfiguration;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// PATTERN
// ============================================================================

/// Pattern
///
/// A pattern recognized in discoveries.
#[derive(Debug, Clone)]
pub struct Pattern {
    /// Unique identifier
    pub id: String,

    /// Pattern type
    pub pattern_type: PatternType,

    /// Description of the pattern
    pub description: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Float,

    /// Pattern data
    pub data: HashMap<String, Float>,

    /// Related configurations
    pub related_configurations: Vec<String>,
}

/// Pattern Type
///
/// Type of pattern recognized.
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Mass pattern
    MassPattern,

    /// Charge pattern
    ChargePattern,

    /// Spin pattern
    SpinPattern,

    /// Lifetime pattern
    LifetimePattern,

    /// Force pattern
    ForcePattern,

    /// Resonance pattern
    ResonancePattern,

    /// Stability pattern
    StabilityPattern,

    /// Custom pattern
    Custom(String),
}

// ============================================================================
// PATTERN RECOGNIZER
// ============================================================================

/// Pattern Recognizer
///
/// Recognizes patterns in discoveries.
#[derive(Debug, Clone)]
pub struct PatternRecognizer {
    /// Recognized patterns
    patterns: Vec<Pattern>,

    /// Recognition statistics
    statistics: RecognitionStatistics,
}

/// Recognition Statistics
#[derive(Debug, Clone, Default)]
pub struct RecognitionStatistics {
    /// Total patterns recognized
    pub total_patterns: usize,

    /// High-confidence patterns
    pub high_confidence_patterns: usize,

    /// Average confidence
    pub average_confidence: Float,
}

impl PatternRecognizer {
    /// Create a new pattern recognizer
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            statistics: RecognitionStatistics::default(),
        }
    }

    /// Recognize patterns in discoveries
    pub fn recognize_patterns(&mut self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut recognized_patterns = Vec::new();

        // Recognize mass patterns
        recognized_patterns.extend(self.recognize_mass_patterns(discoveries));

        // Recognize charge patterns
        recognized_patterns.extend(self.recognize_charge_patterns(discoveries));

        // Recognize spin patterns
        recognized_patterns.extend(self.recognize_spin_patterns(discoveries));

        // Recognize resonance patterns
        recognized_patterns.extend(self.recognize_resonance_patterns(discoveries));

        // Recognize stability patterns
        recognized_patterns.extend(self.recognize_stability_patterns(discoveries));

        // Store patterns
        self.patterns.extend(recognized_patterns.clone());

        // Update statistics
        self.statistics.total_patterns += recognized_patterns.len();
        self.statistics.high_confidence_patterns += recognized_patterns
            .iter()
            .filter(|p| p.confidence > 0.7)
            .count();

        let total_confidence: Float = recognized_patterns.iter().map(|p| p.confidence).sum();
        self.statistics.average_confidence = if !recognized_patterns.is_empty() {
            total_confidence / recognized_patterns.len() as Float
        } else {
            0.0
        };

        recognized_patterns
    }

    /// Recognize mass patterns
    fn recognize_mass_patterns(&self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        if discoveries.is_empty() {
            return patterns;
        }

        // Calculate mass statistics
        let masses: Vec<Float> = discoveries
            .iter()
            .filter_map(|c| c.physical_properties.get("mass").copied())
            .collect();

        if masses.is_empty() {
            return patterns;
        }

        let avg_mass: Float = masses.iter().sum::<Float>() / masses.len() as Float;
        let min_mass = masses.iter().cloned().fold(Float::INFINITY, Float::min);
        let max_mass = masses.iter().cloned().fold(Float::NEG_INFINITY, Float::max);

        // Check for pattern: masses cluster around values
        let clustering_score = self.calculate_clustering_score(&masses);

        if clustering_score > 0.5 {
            let mut data = HashMap::new();
            data.insert("average_mass".to_string(), avg_mass);
            data.insert("min_mass".to_string(), min_mass);
            data.insert("max_mass".to_string(), max_mass);
            data.insert("clustering_score".to_string(), clustering_score);

            patterns.push(Pattern {
                id: format!("mass_pattern_{}", self.patterns.len()),
                pattern_type: PatternType::MassPattern,
                description: format!(
                    "Masses cluster around {:.2e} kg with clustering score {:.2}",
                    avg_mass, clustering_score
                ),
                confidence: clustering_score,
                data,
                related_configurations: discoveries.iter().map(|c| c.id.clone()).collect(),
            });
        }

        patterns
    }

    /// Recognize charge patterns
    fn recognize_charge_patterns(&self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        if discoveries.is_empty() {
            return patterns;
        }

        // Calculate charge statistics
        let charges: Vec<Float> = discoveries
            .iter()
            .filter_map(|c| c.physical_properties.get("charge").copied())
            .collect();

        if charges.is_empty() {
            return patterns;
        }

        // Check for pattern: charges are quantized
        let quantization_score = self.calculate_quantization_score(&charges);

        if quantization_score > 0.5 {
            let mut data = HashMap::new();
            data.insert("quantization_score".to_string(), quantization_score);

            patterns.push(Pattern {
                id: format!("charge_pattern_{}", self.patterns.len()),
                pattern_type: PatternType::ChargePattern,
                description: format!(
                    "Charges show quantization with score {:.2}",
                    quantization_score
                ),
                confidence: quantization_score,
                data,
                related_configurations: discoveries.iter().map(|c| c.id.clone()).collect(),
            });
        }

        patterns
    }

    /// Recognize spin patterns
    fn recognize_spin_patterns(&self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        if discoveries.is_empty() {
            return patterns;
        }

        // Calculate spin statistics
        let spins: Vec<Float> = discoveries
            .iter()
            .filter_map(|c| c.physical_properties.get("spin").copied())
            .collect();

        if spins.is_empty() {
            return patterns;
        }

        // Check for pattern: spins are half-integer or integer
        let half_integer_score = self.calculate_half_integer_score(&spins);

        if half_integer_score > 0.5 {
            let mut data = HashMap::new();
            data.insert("half_integer_score".to_string(), half_integer_score);

            patterns.push(Pattern {
                id: format!("spin_pattern_{}", self.patterns.len()),
                pattern_type: PatternType::SpinPattern,
                description: format!(
                    "Spins show half-integer quantization with score {:.2}",
                    half_integer_score
                ),
                confidence: half_integer_score,
                data,
                related_configurations: discoveries.iter().map(|c| c.id.clone()).collect(),
            });
        }

        patterns
    }

    /// Recognize resonance patterns
    fn recognize_resonance_patterns(&self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        if discoveries.is_empty() {
            return patterns;
        }

        // Calculate resonance statistics
        let resonances: Vec<Float> = discoveries.iter().map(|c| c.resonance_score).collect();

        let avg_resonance: Float = resonances.iter().sum::<Float>() / resonances.len() as Float;

        // Check for pattern: resonance correlates with stability
        let correlation_score = self.calculate_resonance_stability_correlation(discoveries);

        if correlation_score.abs() > 0.5 {
            let mut data = HashMap::new();
            data.insert("average_resonance".to_string(), avg_resonance);
            data.insert("correlation_score".to_string(), correlation_score);

            patterns.push(Pattern {
                id: format!("resonance_pattern_{}", self.patterns.len()),
                pattern_type: PatternType::ResonancePattern,
                description: format!(
                    "Resonance correlates with stability (correlation: {:.2})",
                    correlation_score
                ),
                confidence: correlation_score.abs(),
                data,
                related_configurations: discoveries.iter().map(|c| c.id.clone()).collect(),
            });
        }

        patterns
    }

    /// Recognize stability patterns
    fn recognize_stability_patterns(&self, discoveries: &[StableConfiguration]) -> Vec<Pattern> {
        let mut patterns = Vec::new();

        if discoveries.is_empty() {
            return patterns;
        }

        // Calculate stability statistics
        let stabilities: Vec<Float> = discoveries.iter().map(|c| c.stability_score).collect();

        let avg_stability: Float = stabilities.iter().sum::<Float>() / stabilities.len() as Float;

        // Check for pattern: stability correlates with archetype activation
        let correlation_score = self.calculate_stability_activation_correlation(discoveries);

        if correlation_score.abs() > 0.5 {
            let mut data = HashMap::new();
            data.insert("average_stability".to_string(), avg_stability);
            data.insert("correlation_score".to_string(), correlation_score);

            patterns.push(Pattern {
                id: format!("stability_pattern_{}", self.patterns.len()),
                pattern_type: PatternType::StabilityPattern,
                description: format!(
                    "Stability correlates with archetype activation (correlation: {:.2})",
                    correlation_score
                ),
                confidence: correlation_score.abs(),
                data,
                related_configurations: discoveries.iter().map(|c| c.id.clone()).collect(),
            });
        }

        patterns
    }

    /// Calculate clustering score
    fn calculate_clustering_score(&self, values: &[Float]) -> Float {
        if values.len() < 2 {
            return 0.0;
        }

        let mean: Float = values.iter().sum::<Float>() / values.len() as Float;
        let variance: Float =
            values.iter().map(|v| (v - mean).powi(2)).sum::<Float>() / values.len() as Float;

        // Lower variance = higher clustering score
        1.0 / (1.0 + variance.sqrt())
    }

    /// Calculate quantization score
    fn calculate_quantization_score(&self, values: &[Float]) -> Float {
        if values.is_empty() {
            return 0.0;
        }

        // Check if values are close to integer multiples of a base value
        let base = values
            .iter()
            .cloned()
            .fold(Float::INFINITY, |a, b| a.min(b.abs()));

        if base == 0.0 || base == Float::INFINITY {
            return 0.0;
        }

        let quantization_errors: Vec<Float> = values
            .iter()
            .map(|v| (v / base).round() * base - v)
            .map(|e| e.abs())
            .collect();

        let mean_error: Float =
            quantization_errors.iter().sum::<Float>() / quantization_errors.len() as Float;

        // Lower error = higher quantization score
        1.0 / (1.0 + mean_error)
    }

    /// Calculate half-integer score
    fn calculate_half_integer_score(&self, values: &[Float]) -> Float {
        if values.is_empty() {
            return 0.0;
        }

        let half_integer_count = values
            .iter()
            .filter(|v| (2.0 * **v).round().abs() % 2.0 == 1.0)
            .count();

        half_integer_count as Float / values.len() as Float
    }

    /// Calculate resonance-stability correlation
    fn calculate_resonance_stability_correlation(
        &self,
        discoveries: &[StableConfiguration],
    ) -> Float {
        if discoveries.len() < 2 {
            return 0.0;
        }

        let resonances: Vec<Float> = discoveries.iter().map(|c| c.resonance_score).collect();
        let stabilities: Vec<Float> = discoveries.iter().map(|c| c.stability_score).collect();

        self.calculate_correlation(&resonances, &stabilities)
    }

    /// Calculate stability-activation correlation
    fn calculate_stability_activation_correlation(
        &self,
        discoveries: &[StableConfiguration],
    ) -> Float {
        if discoveries.len() < 2 {
            return 0.0;
        }

        let stabilities: Vec<Float> = discoveries.iter().map(|c| c.stability_score).collect();
        let activations: Vec<Float> = discoveries
            .iter()
            .map(|c| c.archetype_activation.iter().sum::<Float>() / 22.0)
            .collect();

        self.calculate_correlation(&stabilities, &activations)
    }

    /// Calculate correlation between two vectors
    fn calculate_correlation(&self, x: &[Float], y: &[Float]) -> Float {
        if x.len() != y.len() || x.len() < 2 {
            return 0.0;
        }

        let n = x.len() as Float;

        let mean_x: Float = x.iter().sum::<Float>() / n;
        let mean_y: Float = y.iter().sum::<Float>() / n;

        let covariance: Float = x
            .iter()
            .zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<Float>()
            / n;

        let std_x: Float = (x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<Float>() / n).sqrt();
        let std_y: Float = (y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<Float>() / n).sqrt();

        if std_x == 0.0 || std_y == 0.0 {
            return 0.0;
        }

        covariance / (std_x * std_y)
    }

    /// Get all recognized patterns
    pub fn get_patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Get recognition statistics
    pub fn get_statistics(&self) -> &RecognitionStatistics {
        &self.statistics
    }
}

// ============================================================================
// HYPOTHESIS
// ============================================================================

/// Hypothesis
///
/// A novel hypothesis about physics generated from patterns.
#[derive(Debug, Clone)]
pub struct Hypothesis {
    /// Unique identifier
    pub id: String,

    /// Pattern that generated this hypothesis
    pub pattern: Pattern,

    /// Description of the hypothesis
    pub description: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Float,

    /// Testable predictions
    pub testable_predictions: Vec<TestablePrediction>,

    /// Related configurations
    pub related_configurations: Vec<String>,
}

// ============================================================================
// TESTABLE PREDICTION
// ============================================================================

/// Testable Prediction
///
/// A testable prediction from a hypothesis.
#[derive(Debug, Clone)]
pub struct TestablePrediction {
    /// Unique identifier
    pub id: String,

    /// Description of the prediction
    pub description: String,

    /// Test method
    pub test_method: Option<String>,

    /// Expected outcome
    pub expected_outcome: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Float,
}

// ============================================================================
// HYPOTHESIS GENERATOR
// ============================================================================

/// Hypothesis Generator
///
/// Generates novel hypotheses from discoveries.
#[derive(Debug, Clone)]
pub struct HypothesisGenerator {
    /// Pattern recognizer
    pub pattern_recognizer: PatternRecognizer,

    /// Hypothesis database
    pub hypothesis_database: HypothesisDatabase,

    /// Generation statistics
    pub statistics: GenerationStatistics,
}

/// Generation Statistics
#[derive(Debug, Clone, Default)]
pub struct GenerationStatistics {
    /// Total hypotheses generated
    pub total_hypotheses: usize,

    /// High-confidence hypotheses
    pub high_confidence_hypotheses: usize,

    /// Total predictions generated
    pub total_predictions: usize,

    /// Average confidence
    pub average_confidence: Float,
}

impl HypothesisGenerator {
    /// Create a new hypothesis generator
    pub fn new() -> Self {
        Self {
            pattern_recognizer: PatternRecognizer::new(),
            hypothesis_database: HypothesisDatabase::new(),
            statistics: GenerationStatistics::default(),
        }
    }

    /// Generate hypotheses from discoveries
    pub fn generate_hypotheses(&mut self, discoveries: &[StableConfiguration]) -> Vec<Hypothesis> {
        let mut hypotheses = Vec::new();

        // Recognize patterns in discoveries
        let patterns = self.pattern_recognizer.recognize_patterns(discoveries);

        // Generate hypotheses from patterns
        for pattern in patterns {
            let hypothesis = self.formulate_hypothesis(pattern.clone());
            hypotheses.push(hypothesis);
        }

        // Store hypotheses in database
        for hypothesis in &hypotheses {
            self.hypothesis_database
                .add_hypothesis(hypothesis.clone())
                .expect("Failed to add hypothesis to database");
        }

        // Update statistics
        self.statistics.total_hypotheses += hypotheses.len();
        self.statistics.high_confidence_hypotheses +=
            hypotheses.iter().filter(|h| h.confidence > 0.7).count();
        self.statistics.total_predictions += hypotheses
            .iter()
            .map(|h| h.testable_predictions.len())
            .sum::<usize>();

        let total_confidence: Float = hypotheses.iter().map(|h| h.confidence).sum();
        self.statistics.average_confidence = if !hypotheses.is_empty() {
            total_confidence / hypotheses.len() as Float
        } else {
            0.0
        };

        hypotheses
    }

    /// Generate testable predictions from hypotheses
    pub fn generate_testable_predictions(
        &self,
        hypotheses: &[Hypothesis],
    ) -> Vec<TestablePrediction> {
        hypotheses
            .iter()
            .flat_map(|h| h.testable_predictions.clone())
            .collect()
    }

    /// Formulate a hypothesis from a pattern
    fn formulate_hypothesis(&self, pattern: Pattern) -> Hypothesis {
        let description = self.describe_pattern(&pattern);
        let testable_predictions = self.generate_predictions_from_pattern(&pattern);

        Hypothesis {
            id: format!("hypothesis_{}", self.statistics.total_hypotheses),
            pattern: pattern.clone(),
            description,
            confidence: pattern.confidence,
            testable_predictions,
            related_configurations: pattern.related_configurations,
        }
    }

    /// Describe a pattern
    fn describe_pattern(&self, pattern: &Pattern) -> String {
        match &pattern.pattern_type {
            PatternType::MassPattern => {
                format!(
                    "Hypothesis: Mass values in holographic configurations follow the pattern: {}",
                    pattern.description
                )
            }
            PatternType::ChargePattern => {
                format!(
                    "Hypothesis: Charge values in holographic configurations follow the pattern: {}",
                    pattern.description
                )
            }
            PatternType::SpinPattern => {
                format!(
                    "Hypothesis: Spin values in holographic configurations follow the pattern: {}",
                    pattern.description
                )
            }
            PatternType::ResonancePattern => {
                format!(
                    "Hypothesis: Resonance in holographic configurations follows the pattern: {}",
                    pattern.description
                )
            }
            PatternType::StabilityPattern => {
                format!(
                    "Hypothesis: Stability in holographic configurations follows the pattern: {}",
                    pattern.description
                )
            }
            _ => {
                format!("Hypothesis: {}", pattern.description)
            }
        }
    }

    /// Generate predictions from a pattern
    fn generate_predictions_from_pattern(&self, pattern: &Pattern) -> Vec<TestablePrediction> {
        let mut predictions = Vec::new();

        // Generate prediction based on pattern type
        match &pattern.pattern_type {
            PatternType::MassPattern => {
                predictions.push(TestablePrediction {
                    id: format!("prediction_{}_0", pattern.id),
                    description: "Mass values will cluster around specific values".to_string(),
                    test_method: Some("Measure masses of holographic configurations".to_string()),
                    expected_outcome: "Observed masses will show clustering".to_string(),
                    confidence: pattern.confidence,
                });
            }
            PatternType::ChargePattern => {
                predictions.push(TestablePrediction {
                    id: format!("prediction_{}_0", pattern.id),
                    description: "Charge values will be quantized".to_string(),
                    test_method: Some("Measure charges of holographic configurations".to_string()),
                    expected_outcome:
                        "Observed charges will be integer multiples of elementary charge"
                            .to_string(),
                    confidence: pattern.confidence,
                });
            }
            PatternType::SpinPattern => {
                predictions.push(TestablePrediction {
                    id: format!("prediction_{}_0", pattern.id),
                    description: "Spin values will be half-integer or integer".to_string(),
                    test_method: Some("Measure spins of holographic configurations".to_string()),
                    expected_outcome: "Observed spins will be multiples of 0.5".to_string(),
                    confidence: pattern.confidence,
                });
            }
            PatternType::ResonancePattern => {
                predictions.push(TestablePrediction {
                    id: format!("prediction_{}_0", pattern.id),
                    description: "Resonance will correlate with stability".to_string(),
                    test_method: Some(
                        "Measure resonance and stability of holographic configurations".to_string(),
                    ),
                    expected_outcome: "Higher resonance will correlate with higher stability"
                        .to_string(),
                    confidence: pattern.confidence,
                });
            }
            PatternType::StabilityPattern => {
                predictions.push(TestablePrediction {
                    id: format!("prediction_{}_0", pattern.id),
                    description: "Stability will correlate with archetype activation".to_string(),
                    test_method: Some(
                        "Measure stability and archetype activation of holographic configurations"
                            .to_string(),
                    ),
                    expected_outcome:
                        "Higher stability will correlate with higher archetype activation"
                            .to_string(),
                    confidence: pattern.confidence,
                });
            }
            _ => {}
        }

        predictions
    }

    /// Get hypothesis database reference
    pub fn get_hypothesis_database(&self) -> &HypothesisDatabase {
        &self.hypothesis_database
    }

    /// Get hypothesis database reference (mutable)
    pub fn get_hypothesis_database_mut(&mut self) -> &mut HypothesisDatabase {
        &mut self.hypothesis_database
    }

    /// Get generation statistics
    pub fn get_statistics(&self) -> &GenerationStatistics {
        &self.statistics
    }
}

// ============================================================================
// HYPOTHESIS DATABASE
// ============================================================================

/// Hypothesis Database
///
/// Stores generated hypotheses.
#[derive(Debug, Clone)]
pub struct HypothesisDatabase {
    /// Hypotheses
    hypotheses: Vec<Hypothesis>,

    /// Database statistics
    statistics: HypothesisDatabaseStatistics,
}

/// Hypothesis Database Statistics
#[derive(Debug, Clone, Default)]
pub struct HypothesisDatabaseStatistics {
    /// Total hypotheses
    pub total_hypotheses: usize,

    /// High-confidence hypotheses
    pub high_confidence_hypotheses: usize,
}

impl HypothesisDatabase {
    /// Create a new hypothesis database
    pub fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
            statistics: HypothesisDatabaseStatistics::default(),
        }
    }

    /// Add a hypothesis
    pub fn add_hypothesis(&mut self, hypothesis: Hypothesis) -> Result<(), String> {
        self.hypotheses.push(hypothesis.clone());
        self.statistics.total_hypotheses += 1;

        if hypothesis.confidence > 0.7 {
            self.statistics.high_confidence_hypotheses += 1;
        }

        Ok(())
    }

    /// Get all hypotheses
    pub fn get_hypotheses(&self) -> &[Hypothesis] {
        &self.hypotheses
    }

    /// Get hypotheses by minimum confidence
    pub fn get_hypotheses_by_confidence(&self, min_confidence: Float) -> Vec<Hypothesis> {
        self.hypotheses
            .iter()
            .filter(|h| h.confidence >= min_confidence)
            .cloned()
            .collect()
    }

    /// Get database statistics
    pub fn get_statistics(&self) -> &HypothesisDatabaseStatistics {
        &self.statistics
    }

    /// Clear the database
    pub fn clear(&mut self) {
        self.hypotheses.clear();
        self.statistics = HypothesisDatabaseStatistics::default();
    }
}

impl Default for HypothesisDatabase {
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

    fn create_test_configurations() -> Vec<StableConfiguration> {
        let mut configs = Vec::new();

        for i in 0..5 {
            let mut config = StableConfiguration::new(
                format!("test_config_{}", i),
                [0.5; 22],
                100.0,
                0.7 + (i as Float) * 0.05,
                0.75 + (i as Float) * 0.04,
            );

            config.add_property("mass".to_string(), 1.0e-30 * (i as Float + 1.0));
            config.add_property("charge".to_string(), (i % 3) as Float - 1.0);
            config.add_property("spin".to_string(), 0.5 * (i % 2) as Float);

            configs.push(config);
        }

        configs
    }

    #[test]
    fn test_pattern_recognizer_creation() {
        let recognizer = PatternRecognizer::new();
        assert_eq!(recognizer.get_statistics().total_patterns, 0);
    }

    #[test]
    fn test_recognize_patterns() {
        let mut recognizer = PatternRecognizer::new();
        let configs = create_test_configurations();

        let patterns = recognizer.recognize_patterns(&configs);

        assert!(!patterns.is_empty());
        assert!(recognizer.get_statistics().total_patterns > 0);
    }

    #[test]
    fn test_hypothesis_generator_creation() {
        let generator = HypothesisGenerator::new();
        assert_eq!(generator.get_statistics().total_hypotheses, 0);
    }

    #[test]
    fn test_generate_hypotheses() {
        let mut generator = HypothesisGenerator::new();
        let configs = create_test_configurations();

        let hypotheses = generator.generate_hypotheses(&configs);

        assert!(!hypotheses.is_empty());
        assert!(generator.get_statistics().total_hypotheses > 0);

        for hypothesis in &hypotheses {
            assert!(!hypothesis.id.is_empty());
            assert!(!hypothesis.description.is_empty());
            assert!(hypothesis.confidence >= 0.0 && hypothesis.confidence <= 1.0);
        }
    }

    #[test]
    fn test_generate_testable_predictions() {
        let mut generator = HypothesisGenerator::new();
        let configs = create_test_configurations();

        let hypotheses = generator.generate_hypotheses(&configs);
        let predictions = generator.generate_testable_predictions(&hypotheses);

        assert!(!predictions.is_empty());

        for prediction in &predictions {
            assert!(!prediction.id.is_empty());
            assert!(!prediction.description.is_empty());
            assert!(prediction.confidence >= 0.0 && prediction.confidence <= 1.0);
        }
    }

    #[test]
    fn test_hypothesis_database() {
        let mut db = HypothesisDatabase::new();

        let pattern = Pattern {
            id: "test_pattern".to_string(),
            pattern_type: PatternType::MassPattern,
            description: "Test pattern".to_string(),
            confidence: 0.8,
            data: HashMap::new(),
            related_configurations: Vec::new(),
        };

        let hypothesis = Hypothesis {
            id: "test_hypothesis".to_string(),
            pattern: pattern.clone(),
            description: "Test hypothesis".to_string(),
            confidence: 0.8,
            testable_predictions: Vec::new(),
            related_configurations: Vec::new(),
        };

        db.add_hypothesis(hypothesis).unwrap();
        assert_eq!(db.get_statistics().total_hypotheses, 1);
        assert_eq!(db.get_statistics().high_confidence_hypotheses, 1);
    }

    #[test]
    fn test_get_hypotheses_by_confidence() {
        let mut db = HypothesisDatabase::new();

        let pattern = Pattern {
            id: "test_pattern".to_string(),
            pattern_type: PatternType::MassPattern,
            description: "Test pattern".to_string(),
            confidence: 0.8,
            data: HashMap::new(),
            related_configurations: Vec::new(),
        };

        let hypothesis1 = Hypothesis {
            id: "test_hypothesis_1".to_string(),
            pattern: pattern.clone(),
            description: "Test hypothesis 1".to_string(),
            confidence: 0.9,
            testable_predictions: Vec::new(),
            related_configurations: Vec::new(),
        };

        let hypothesis2 = Hypothesis {
            id: "test_hypothesis_2".to_string(),
            pattern: pattern.clone(),
            description: "Test hypothesis 2".to_string(),
            confidence: 0.5,
            testable_predictions: Vec::new(),
            related_configurations: Vec::new(),
        };

        db.add_hypothesis(hypothesis1).unwrap();
        db.add_hypothesis(hypothesis2).unwrap();

        let high_confidence = db.get_hypotheses_by_confidence(0.7);
        assert_eq!(high_confidence.len(), 1);
    }
}
