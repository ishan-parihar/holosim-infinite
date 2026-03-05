// A4: The Experience of Mind
// Processed output - integrated wisdom
// Documentation: 01_Metaphysics/archetypes/A4_Experience.md

use crate::archetypes::archetype_traits::{
    ExperienceArchetypeTrait, MatrixArchetypeTrait, PotentiatorArchetypeTrait,
};
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Polarity, Rung};
use std::collections::HashMap;

// Forward declaration of CatalystVariety (defined in a3_mind_catalyst.rs)
#[derive(Debug, Clone, Copy)]
pub enum CatalystVariety {
    Emotional,
    Intellectual,
    Physical,
    Spiritual,
}

/// Bias Structure - Different types of biases created by experience
#[derive(Debug, Clone)]
pub struct BiasStructure {
    /// How perception is biased (-1.0 to 1.0)
    pub perceptual_bias: Float,
    /// How thinking is biased (-1.0 to 1.0)
    pub cognitive_bias: Float,
    /// How emotions are biased (-1.0 to 1.0)
    pub emotional_bias: Float,
    /// How behavior is biased (-1.0 to 1.0)
    pub behavioral_bias: Float,
    /// How spirituality is biased (-1.0 to 1.0)
    pub spiritual_bias: Float,
}

impl BiasStructure {
    /// Calculate total bias (average of all bias types)
    pub fn total_bias(&self) -> Float {
        (self.perceptual_bias
            + self.cognitive_bias
            + self.emotional_bias
            + self.behavioral_bias
            + self.spiritual_bias)
            / 5.0
    }

    /// Get bias direction (positive = integration, negative = distortion, 0 = neutral)
    pub fn bias_direction(&self) -> Float {
        let total = self.total_bias();
        if total > 0.5 {
            1.0 // Positive/integration
        } else if total < -0.5 {
            -1.0 // Negative/distortion
        } else if total.abs() < 0.1 {
            0.0 // Neutral (close to zero)
        } else {
            // Return the actual bias value for intermediate states
            total
        }
    }
}

/// A4: The Experience of Mind - Processed Output (Integrated Wisdom)
///
/// The Experience of the Mind is processed catalyst — the lessons learned,
/// the memories formed, the wisdom gained. It represents the material stored
/// in the unconscious which creates its continuing bias. Experience is that
/// which has been processed by the mind/body/spirit complex, ennobling the
/// conscious mind through the use of vast resources of the unconscious mind.
///
/// Core Concept: The Experience of the Mind is that material stored in the
/// unconscious which creates its continuing bias — the result of catalyst
/// being processed by the mind/body/spirit complex.
///
/// **Lambda Measurement**: Experience depth and integration
/// - Healthy range: 0.5 - 0.8 (deep processing without over-identification)
/// - Pathological: < 0.5 (shallow, unprocessed) or > 0.8 (over-identified, stuck)
#[derive(Debug, Clone)]
pub struct ExperienceMindArchetype {
    /// Archetype ID (A4)
    pub archetype_id: u8,

    /// Lambda measurement - Experience depth and integration
    pub lambda: LambdaMeasurement,

    /// Tarot correlation
    pub tarot_correlation: TarotCorrelation,

    /// Experience depth - how deeply catalyst has been processed (0.0 - 1.0)
    pub experience_depth: Float,

    /// Integration quality - how well experience is integrated into wisdom (0.0 - 1.0)
    pub integration_quality: Float,

    /// Bias flexibility - how flexible is the continuing bias (0.0 - 1.0)
    pub bias_flexibility: Float,

    /// Authority balance - balance between firm authority and openness (0.0 - 1.0)
    pub authority_balance: Float,

    /// Processing velocity - speed of converting catalyst to experience (0.0 - 1.0)
    pub processing_velocity: Float,

    /// Imperfect memory factor - how memory distortion affects experience (0.0 - 1.0)
    pub imperfect_memory_factor: Float,

    /// Catalyst integration - how well catalyst is integrated into experience (0.0 - 1.0)
    pub catalyst_integration: Float,

    /// Polarity bias - how experience biases interpretation
    pub polarization_bias: Polarity,

    /// Protection level - protection from negative catalyst (STO only) (0.0 - 1.0)
    pub protection_level: Float,

    /// Veil permeability modifier - how experience changes veil permeability (0.0 - 1.0)
    pub veil_permeability_modifier: Float,

    /// Continuing bias strength - strength of bias created by experience (0.0 - 1.0)
    pub continuing_bias_strength: Float,

    /// NEW: Bias structure - structured bias created by experience
    pub bias_structure: BiasStructure,
    /// NEW: Bias formation rate - how fast biases form (0.0 - 1.0)
    pub bias_formation_rate: Float,
    /// NEW: Bias strength evolution - how bias strength has evolved
    pub bias_strength_evolution: Vec<Float>,

    /// Developmental position in Sigma Network
    pub developmental_position: DevelopmentalPosition,

    /// Activated rungs (R1-R7)
    pub activated_rungs: Vec<Rung>,

    /// Activation levels for each rung (0.0 to 1.0)
    pub activation_levels: HashMap<Rung, Float>,

    /// Description
    pub description: String,

    /// Holonic level
    pub holonic_level: HolonicLevel,

    /// Integration capacity (0.0 to 1.0)
    pub integration_capacity: Float,

    /// Processing efficiency - ratio of experience depth to catalyst inflow
    pub processing_efficiency: Float,

    /// Stored experiences - collection of processed experiences
    pub stored_experiences: Vec<StoredExperience>,

    /// Bias weights - weights for different bias types
    pub bias_weights: BiasWeights,
}

/// Stored experience - represents a processed experience
#[derive(Debug, Clone)]
pub struct StoredExperience {
    pub experience_id: u64,
    pub intensity: Float,
    pub variety: CatalystVariety,
    pub timestamp: u64,
    pub polarization_impact: Float,
}

/// Bias weights - weights for different bias types
#[derive(Debug, Clone)]
pub struct BiasWeights {
    pub emotional: Float,
    pub intellectual: Float,
    pub physical: Float,
    pub spiritual: Float,
}

impl BiasWeights {
    /// Get bias weights as iterator of (rung, bias) pairs
    pub fn iter(&self) -> impl Iterator<Item = (Rung, Float)> {
        [
            (Rung::R1, self.emotional),
            (Rung::R2, self.intellectual),
            (Rung::R3, self.physical),
            (Rung::R4, self.spiritual),
        ]
        .into_iter()
    }

    /// Get bias value for a specific rung
    pub fn get(&self, rung: &Rung) -> Float {
        match *rung {
            Rung::R1 => self.emotional,
            Rung::R2 => self.intellectual,
            Rung::R3 => self.physical,
            Rung::R4 => self.spiritual,
            Rung(0_u8) => 0.0,
            Rung(8_u8..=u8::MAX) => 0.0,
            _ => 0.0,
        }
    }
}

impl Default for ExperienceMindArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperienceMindArchetype {
    /// Create a new Experience archetype with default healthy state
    pub fn new() -> Self {
        // Initialize with optimal lambda value (middle of healthy range)
        // Tests expect: value=0.5, healthy_min=0.5, healthy_max=0.8
        let mut lambda = LambdaMeasurement::new(0.5, LambdaMeasurementType::ExperienceDepth);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        let tarot_correlation = TarotCorrelation::new("The Emperor (IV): Processed output, integrated wisdom, firm authority".to_string());

        let activation_levels = HashMap::new();

        // Initialize bias structure with neutral values
        let bias_structure = BiasStructure {
            perceptual_bias: 0.0,
            cognitive_bias: 0.0,
            emotional_bias: 0.0,
            behavioral_bias: 0.0,
            spiritual_bias: 0.0,
        };

        // Initialize bias weights with neutral values
        let bias_weights = BiasWeights {
            emotional: 0.0,
            intellectual: 0.0,
            physical: 0.0,
            spiritual: 0.0,
        };

        // Initialize stored experiences as empty
        let stored_experiences: Vec<StoredExperience> = Vec::new();

        ExperienceMindArchetype {
            archetype_id: 4,
            lambda,
            tarot_correlation,
            experience_depth: 0.65,
            integration_quality: 0.70,
            bias_flexibility: 0.60,
            authority_balance: 0.65,
            processing_velocity: 0.60,
            imperfect_memory_factor: 0.30,
            catalyst_integration: 0.70,
            polarization_bias: Polarity::SinkholeOfIndifference,
            protection_level: 0.0,
            veil_permeability_modifier: 0.50,
            continuing_bias_strength: 0.50,
            bias_structure,
            bias_formation_rate: 0.3,
            bias_strength_evolution: Vec::new(),
            developmental_position: DevelopmentalPosition::Experience,
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3],
            activation_levels,
            description: "The Experience of Mind is processed catalyst — the lessons learned, the memories formed, the wisdom gained. It represents the material stored in the unconscious which creates its continuing bias. Experience is that which has been processed by the mind/body/spirit complex, ennobling the conscious mind through the use of vast resources of the unconscious mind.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.7,
            processing_efficiency: 0.75,
            stored_experiences,
            bias_weights,
        }
    }

    /// Calculate experience depth
    ///
    /// Experience depth increases with:
    /// - Higher integration quality
    /// - Higher processing velocity
    /// - Lower imperfect memory factor
    ///
    /// Returns: Experience depth (0.0 to 1.0)
    pub fn calculate_experience_depth(&self) -> Float {
        let depth = self.integration_quality * 0.4
            + self.processing_velocity * 0.3
            + (1.0 - self.imperfect_memory_factor) * 0.3;
        depth.clamp(0.0, 1.0)
    }

    /// Calculate integration quality
    ///
    /// Integration quality increases with:
    /// - Higher experience depth
    /// - Higher bias flexibility
    /// - Higher authority balance
    ///
    /// Returns: Integration quality (0.0 to 1.0)
    pub fn calculate_integration_quality(&self) -> Float {
        let quality = self.experience_depth * 0.4
            + self.bias_flexibility * 0.3
            + self.authority_balance * 0.3;
        quality.clamp(0.0, 1.0)
    }

    /// Calculate bias flexibility
    ///
    /// Bias flexibility increases with:
    /// - Higher integration quality
    /// - Lower continuing bias strength
    ///
    /// Returns: Bias flexibility (0.0 to 1.0)
    pub fn calculate_bias_flexibility(&self) -> Float {
        let flexibility =
            self.integration_quality * 0.6 + (1.0 - self.continuing_bias_strength) * 0.4;
        flexibility.clamp(0.0, 1.0)
    }

    /// Calculate authority balance
    ///
    /// Authority balance is the balance between:
    /// - Firm authority (experience-based wisdom)
    /// - Openness (willingness to update based on new catalyst)
    ///
    /// Returns: Authority balance (0.0 to 1.0)
    pub fn calculate_authority_balance(&self) -> Float {
        // Balance increases with both integration quality and bias flexibility
        (self.integration_quality * 0.5 + self.bias_flexibility * 0.5)
            .clamp(0.0, 1.0)
    }

    /// Calculate continuing bias strength
    ///
    /// Continuing bias strength increases with:
    /// - Higher experience depth
    /// - Lower bias flexibility
    ///
    /// Returns: Continuing bias strength (0.0 to 1.0)
    pub fn calculate_continuing_bias_strength(&self) -> Float {
        let strength = self.experience_depth * 0.6 + (1.0 - self.bias_flexibility) * 0.4;
        strength.clamp(0.0, 1.0)
    }

    /// Calculate protection level (STO only)
    ///
    /// Protection level increases with:
    /// - Higher integration quality
    /// - STO polarization
    ///
    /// Returns: Protection level (0.0 to 1.0)
    pub fn calculate_protection_level(&self) -> Float {
        if matches!(self.polarization_bias, Polarity::STO) {
            self.integration_quality * 0.8
        } else {
            0.0
        }
    }

    /// Calculate processing efficiency
    ///
    /// Efficiency = experience_depth / catalyst_integration
    ///
    /// Returns: Processing efficiency (0.0 to 1.0)
    pub fn calculate_processing_efficiency(&self) -> Float {
        if self.catalyst_integration <= 0.0 {
            return 1.0; // No catalyst means 100% efficiency
        }

        (self.experience_depth / self.catalyst_integration)
            .clamp(0.0, 1.0)
    }

    /// Process experience
    ///
    /// Updates all experience properties based on current state
    pub fn process_experience(&mut self) {
        self.experience_depth = self.calculate_experience_depth();
        self.integration_quality = self.calculate_integration_quality();
        self.bias_flexibility = self.calculate_bias_flexibility();
        self.authority_balance = self.calculate_authority_balance();
        self.continuing_bias_strength = self.calculate_continuing_bias_strength();
        self.protection_level = self.calculate_protection_level();
        self.processing_efficiency = self.calculate_processing_efficiency();

        // Update lambda based on experience depth
        self.lambda.value = self.experience_depth;
    }

    /// Set polarization bias
    ///
    /// Sets the polarization bias (STO, STS, or Sinkhole of Indifference)
    pub fn set_polarization_bias(&mut self, polarization: Polarity) {
        self.polarization_bias = polarization;
    }

    /// Get polarization bias
    ///
    /// Returns current polarization bias
    pub fn get_polarization_bias(&self) -> Polarity {
        self.polarization_bias
    }

    /// Check if experience is shallow
    ///
    /// Returns true if experience depth is too low
    pub fn is_shallow(&self) -> bool {
        self.experience_depth < 0.5
    }

    /// Check if experience is over-identified
    ///
    /// Returns true if continuing bias strength is too high
    pub fn is_over_identified(&self) -> bool {
        self.continuing_bias_strength > 0.8
    }

    /// Get experience status
    ///
    /// Returns description of current experience state
    pub fn get_experience_status(&self) -> String {
        match self.lambda.health_status() {
            HealthStatus::Healthy => {
                if self.processing_efficiency > 0.8 {
                    "Optimal: Deep processing, excellent integration".to_string()
                } else if self.processing_efficiency > 0.6 {
                    "Healthy: Good processing, balanced integration".to_string()
                } else {
                    "Acceptable: Processing creating wisdom, moderate integration".to_string()
                }
            }
            HealthStatus::Balanced => "Balanced: Stable processing, good integration".to_string(),
            HealthStatus::Imbalanced => "Imbalanced: Processing issues present".to_string(),
            HealthStatus::Warning => {
                "Warning: Processing efficiency declining, attention needed".to_string()
            }
            HealthStatus::Degraded => {
                "Degraded: Multiple processing issues, significant integration loss".to_string()
            }
            HealthStatus::Pathological => {
                "Pathological: Severe processing dysfunction, wisdom formation impaired".to_string()
            }
            HealthStatus::PathologicalLow => {
                if self.is_shallow() {
                    "Pathological: Shallow processing, unprocessed catalyst".to_string()
                } else {
                    "Pathological: Low integration, weak wisdom formation".to_string()
                }
            }
            HealthStatus::PathologicalHigh => {
                if self.is_over_identified() {
                    "Pathological: Over-identified, rigid continuing bias".to_string()
                } else {
                    "Pathological: Excessive authority, stuck in past experience".to_string()
                }
            }
        }
    }

    // ============================================================================
    // PHASE 6: BIAS STRUCTURE METHODS
    // ============================================================================

    /// Form continuing bias from experience
    ///
    /// Bias formation = experience depth × bias formation rate
    /// Updates all bias dimensions based on experience
    pub fn form_continuing_bias(&mut self, _experience: Float) {
        // Bias formation = experience depth × bias formation rate
        let bias_formation = self.experience_depth * self.bias_formation_rate;

        // Update bias structure based on experience
        self.bias_structure.perceptual_bias =
            (self.bias_structure.perceptual_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.cognitive_bias =
            (self.bias_structure.cognitive_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.emotional_bias =
            (self.bias_structure.emotional_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.behavioral_bias =
            (self.bias_structure.behavioral_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.spiritual_bias =
            (self.bias_structure.spiritual_bias + bias_formation * 0.2).clamp(-1.0, 1.0);

        // Update continuing bias strength
        let total_bias = self.bias_structure.total_bias();
        self.continuing_bias_strength = total_bias.abs();

        // Record bias strength evolution
        self.bias_strength_evolution.push(total_bias);
        if self.bias_strength_evolution.len() > 100 {
            self.bias_strength_evolution.remove(0);
        }
    }

    /// Get bias direction (positive = integration, negative = distortion, 0 = neutral)
    pub fn get_bias_direction(&self) -> Float {
        self.bias_structure.bias_direction()
    }

    /// Get total bias (average of all bias types)
    pub fn get_total_bias(&self) -> Float {
        self.bias_structure.total_bias()
    }

    /// Get bias strength evolution history
    pub fn get_bias_strength_evolution(&self) -> &Vec<Float> {
        &self.bias_strength_evolution
    }

    /// Check if bias is positive (integration-oriented)
    pub fn is_bias_positive(&self) -> bool {
        self.get_bias_direction() > 0.0
    }

    /// Check if bias is negative (distortion-oriented)
    pub fn is_bias_negative(&self) -> bool {
        self.get_bias_direction() < 0.0
    }

    /// Check if bias is neutral
    pub fn is_bias_neutral(&self) -> bool {
        self.get_bias_direction() == 0.0
    }

    /// Get bias strength for a specific dimension
    pub fn get_bias_dimension(&self, dimension: &str) -> Float {
        match dimension {
            "perceptual" => self.bias_structure.perceptual_bias,
            "cognitive" => self.bias_structure.cognitive_bias,
            "emotional" => self.bias_structure.emotional_bias,
            "behavioral" => self.bias_structure.behavioral_bias,
            "spiritual" => self.bias_structure.spiritual_bias,
            _ => 0.0,
        }
    }

    /// Reset bias structure to neutral
    pub fn reset_bias(&mut self) {
        self.bias_structure.perceptual_bias = 0.0;
        self.bias_structure.cognitive_bias = 0.0;
        self.bias_structure.emotional_bias = 0.0;
        self.bias_structure.behavioral_bias = 0.0;
        self.bias_structure.spiritual_bias = 0.0;
        self.continuing_bias_strength = 0.0;
        self.bias_strength_evolution.clear();
    }

    // Additional methods for archetype processing

    /// Store experiences (no-op for this implementation)
    pub fn store_experiences(&mut self, _experiences: Vec<crate::complex::Experience>) {
        // Experiences are managed internally through experience_depth
        // This is a no-op for this implementation
    }

    /// Get bias for a specific rung
    pub fn get_bias(&self, rung: crate::types::Rung) -> Float {
        // Return bias based on bias_structure and rung level
        let rung_level = rung.value() as Float / 7.0;
        self.continuing_bias_strength * rung_level
    }
}

// Implement ArchetypeTrait
impl ArchetypeTrait for ExperienceMindArchetype {
    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Experience of Mind"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Mind
    }

    fn activate(&mut self, intensity: Float) {
        self.lambda.adjust(intensity);
    }

    fn deactivate(&mut self) {
        self.lambda.adjust(-0.1);
    }

    fn is_active(&self) -> bool {
        self.lambda.value > 0.0
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::Experience
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
        self.experience_depth = self.lambda.value;
    }

    fn health_status(&self) -> HealthStatus {
        self.lambda.health_status()
    }

    fn is_healthy(&self) -> bool {
        self.lambda.is_healthy()
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaA
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::ProcessPair
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        self.process_experience();
    }

    fn description(&self) -> &str {
        &self.description
    }
}

// Implement LambdaMeasurable
impl LambdaMeasurable for ExperienceMindArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        self.lambda.value
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.is_pathological_low() {
            indicators.push("Shallow experience - unprocessed catalyst".to_string());
            indicators.push("Low integration quality - weak wisdom formation".to_string());
            indicators.push("Inadequate bias - no continuing structure".to_string());
        }

        if self.lambda.is_pathological_high() {
            indicators.push("Over-identified experience - rigid continuing bias".to_string());
            indicators.push("Excessive authority - stuck in past patterns".to_string());
            indicators.push("Low flexibility - unable to update from new catalyst".to_string());
        }

        indicators
    }
}

// Implement Developmental
impl Developmental for ExperienceMindArchetype {
    fn develop(&mut self, catalyst: Float) {
        self.lambda.value += catalyst * 0.1;
        if self.lambda.value > 1.0 {
            self.lambda.value = 1.0;
        }
    }

    fn regress(&mut self, resistance: Float) {
        self.lambda.value -= resistance * 0.1;
        if self.lambda.value < 0.0 {
            self.lambda.value = 0.0;
        }
    }

    fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        self.activation_levels.get(&rung).copied().unwrap_or(0.0)
    }
}

// Implement Paired
impl Paired for ExperienceMindArchetype {
    fn get_pair(&self) -> Option<u8> {
        Some(3) // A3: The Catalyst of Mind
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        Some(3) // A3: The Catalyst of Mind
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Tension between Catalyst and Experience
        // Healthy tension: 0.3 - 0.6
        // This is a placeholder - actual calculation requires Catalyst reference
        0.5
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Balance = 1.0 - |tension - 0.5|
        let tension = self.calculate_pair_tension(paired_archetype);
        1.0 - (tension - 0.5).abs()
    }
}

// Implement Holonic
impl Holonic for ExperienceMindArchetype {
    fn get_holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn set_holonic_level(&mut self, level: HolonicLevel) {
        self.holonic_level = level;
    }

    fn holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn integration_capacity(&self) -> Float {
        self.integration_capacity
    }

    fn transcend(&mut self) -> bool {
        // Attempt to transcend to next holonic level
        match self.holonic_level {
            HolonicLevel::Micro => {
                if self.integration_capacity > 0.7 {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meso => {
                if self.integration_capacity > 0.8 {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Macro => {
                if self.integration_capacity > 0.9 {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meta => false, // Already at highest level
        }
    }

    fn include(&mut self, _lower_level: &dyn Holonic) -> bool {
        // Include lower level holonic properties
        self.integration_capacity = (self.integration_capacity + 0.1).min(1.0);
        true
    }
}

// ============================================================================
// EXPERIENCE ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl ExperienceArchetypeTrait for ExperienceMindArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_experience_depth(&self) -> Float {
        self.experience_depth
    }

    fn get_integration_quality(&self) -> Float {
        self.integration_quality
    }

    fn get_continuing_bias_strength(&self) -> Float {
        self.continuing_bias_strength
    }

    // Core setters
    fn set_experience_depth(&mut self, value: Float) {
        self.experience_depth = value.clamp(0.0, 1.0);
        self.lambda.value = (value.clamp(0.5, 0.8) + value.clamp(0.5, 0.8)) / 2.0;
    }

    fn set_integration_quality(&mut self, value: Float) {
        self.integration_quality = value.clamp(0.0, 1.0);
    }

    fn set_continuing_bias_strength(&mut self, value: Float) {
        self.continuing_bias_strength = value.clamp(0.0, 1.0);
    }

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    fn get_activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn get_activation_level(&self, rung: Rung) -> Float {
        self.activation_levels.get(&rung).copied().unwrap_or(0.0)
    }

    fn set_activation_level(&mut self, rung: Rung, level: Float) {
        self.activation_levels.insert(rung, level.clamp(0.0, 1.0));
    }

    // Bias structure methods (already exist)
    fn form_continuing_bias(&mut self, experience: Float) {
        ExperienceMindArchetype::form_continuing_bias(self, experience);
    }

    fn get_bias_direction(&self) -> Float {
        ExperienceMindArchetype::get_bias_direction(self)
    }

    fn apply_bias_to_matrix(&self, matrix: &mut Box<dyn MatrixArchetypeTrait>) {
        // Apply perceptual bias to structural permeability
        let permeability_modifier = 1.0 + self.bias_structure.perceptual_bias * 0.1;
        matrix.set_structural_permeability(
            matrix.get_structural_permeability() * permeability_modifier,
        );

        // Apply cognitive bias to conscious coherence
        let coherence_modifier = 1.0 + self.bias_structure.cognitive_bias * 0.1;
        matrix.set_conscious_coherence(matrix.get_conscious_coherence() * coherence_modifier);

        // Apply emotional bias to willful integration
        let integration_modifier = 1.0 + self.bias_structure.emotional_bias * 0.1;
        matrix.set_willful_integration(matrix.get_willful_integration() * integration_modifier);

        // Apply behavioral bias to resource access
        let access_modifier = 1.0 + self.bias_structure.behavioral_bias * 0.1;
        matrix.set_resource_access(matrix.get_resource_access() * access_modifier);

        // Apply spiritual bias to integration capacity
        let capacity_modifier = 1.0 + self.bias_structure.spiritual_bias * 0.1;
        matrix.set_integration_capacity(matrix.get_integration_capacity() * capacity_modifier);
    }

    fn apply_bias_to_potentiator(&self, potentiator: &mut Box<dyn PotentiatorArchetypeTrait>) {
        // Positive bias opens new Potentiator access
        if self.get_bias_direction() > 0.0 {
            let access_increase = self.continuing_bias_strength * 0.1;
            potentiator.increase_resource_depth(access_increase);
        }
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Map common HealthStatus to complex HealthStatus
        let common_health = ExperienceMindArchetype::health_status(self);
        match common_health {
            HealthStatus::Healthy => HealthStatus::Healthy,
            HealthStatus::PathologicalLow => HealthStatus::Degraded,
            HealthStatus::PathologicalHigh => HealthStatus::Pathological,
            _ => HealthStatus::Warning,
        }
    }

    // Experience management methods (used by complex.rs)
    fn store_experiences(&mut self, _experiences: Vec<crate::complex::Experience>) {
        // Experiences are managed internally through experience_depth
        // This is a no-op for this implementation
    }

    fn get_bias(&self, rung: Rung) -> Float {
        // Return bias based on bias_structure and rung level
        let rung_level = rung.value() as Float / 7.0;
        self.continuing_bias_strength * rung_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_creation() {
        let experience = ExperienceMindArchetype::new();
        assert_eq!(experience.archetype_id, 4);
        assert_eq!(experience.name(), "The Experience of Mind");
        assert_eq!(experience.complex(), ArchetypeComplex::Mind);
        assert_eq!(experience.role(), ArchetypeRole::Experience);
    }

    #[test]
    fn test_experience_lambda_initialization() {
        let experience = ExperienceMindArchetype::new();
        assert_eq!(experience.lambda.value, 0.5);
        assert_eq!(experience.lambda.healthy_min, 0.5);
        assert_eq!(experience.lambda.healthy_max, 0.8);
        assert!(experience.is_healthy());
    }

    #[test]
    fn test_experience_health_status() {
        let mut experience = ExperienceMindArchetype::new();

        // Test healthy state
        experience.update_lambda(0.65);
        assert_eq!(experience.health_status(), HealthStatus::Healthy);
        assert!(experience.is_healthy());

        // Test pathological low
        experience.update_lambda(0.4);
        assert_eq!(experience.health_status(), HealthStatus::PathologicalLow);
        assert!(!experience.is_healthy());

        // Test pathological high
        experience.update_lambda(0.85);
        assert_eq!(experience.health_status(), HealthStatus::PathologicalHigh);
        assert!(!experience.is_healthy());
    }

    #[test]
    fn test_experience_processing() {
        let mut experience = ExperienceMindArchetype::new();
        experience.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        assert!(experience.experience_depth > 0.0);
        assert!(experience.experience_depth <= 1.0);
        assert!(experience.integration_quality > 0.0);
        assert!(experience.integration_quality <= 1.0);
        assert!(experience.bias_flexibility > 0.0);
        assert!(experience.bias_flexibility <= 1.0);
    }

    #[test]
    fn test_experience_polarization_bias() {
        let mut experience = ExperienceMindArchetype::new();

        // Initially sinkhole of indifference
        assert_eq!(
            experience.get_polarization_bias(),
            Polarity::SinkholeOfIndifference
        );

        // Set STO polarization
        experience.set_polarization_bias(Polarity::STO);
        assert_eq!(experience.get_polarization_bias(), Polarity::STO);

        // Set STS polarization
        experience.set_polarization_bias(Polarity::STS);
        assert_eq!(experience.get_polarization_bias(), Polarity::STS);
    }

    #[test]
    fn test_experience_shallow() {
        let mut experience = ExperienceMindArchetype::new();

        // Normal state
        assert!(!experience.is_shallow());

        // Shallow state
        experience.experience_depth = 0.4;
        assert!(experience.is_shallow());
    }

    #[test]
    fn test_experience_over_identified() {
        let mut experience = ExperienceMindArchetype::new();

        // Normal state
        assert!(!experience.is_over_identified());

        // Over-identified state
        experience.continuing_bias_strength = 0.85;
        assert!(experience.is_over_identified());
    }

    #[test]
    fn test_experience_protection_level() {
        let mut experience = ExperienceMindArchetype::new();

        // No polarization - no protection
        assert_eq!(experience.calculate_protection_level(), 0.0);

        // STO polarization - protection available
        experience.set_polarization_bias(Polarity::STO);
        experience.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);
        assert!(experience.protection_level > 0.0);

        // STS polarization - no protection
        experience.set_polarization_bias(Polarity::STS);
        experience.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);
        assert_eq!(experience.protection_level, 0.0);
    }

    #[test]
    fn test_experience_status() {
        let mut experience = ExperienceMindArchetype::new();

        // Healthy status
        experience.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);
        let status = experience.get_experience_status();
        assert!(
            status.contains("Healthy")
                || status.contains("Optimal")
                || status.contains("Acceptable")
        );

        // Pathological low status
        experience.update_lambda(0.4);
        let status = experience.get_experience_status();
        assert!(status.contains("Pathological"));

        // Pathological high status
        experience.update_lambda(0.85);
        let status = experience.get_experience_status();
        assert!(status.contains("Pathological"));
    }

    #[test]
    fn test_experience_pathological_indicators() {
        let mut experience = ExperienceMindArchetype::new();

        // Test pathological low
        experience.update_lambda(0.4);
        let indicators_low = experience.pathological_indicators();
        assert!(!indicators_low.is_empty());
        assert!(indicators_low
            .iter()
            .any(|i| i.contains("shallow") || i.contains("unprocessed")));

        // Test pathological high
        experience.update_lambda(0.85);
        let indicators_high = experience.pathological_indicators();
        assert!(!indicators_high.is_empty());
        assert!(indicators_high
            .iter()
            .any(|i| i.contains("over-identified") || i.contains("rigid")));
    }

    #[test]
    fn test_experience_functional_pair() {
        let experience = ExperienceMindArchetype::new();
        assert_eq!(experience.functional_pair(), FunctionalPair::ProcessPair);
        assert_eq!(experience.paired_archetype_id(), Some(3));
    }

    #[test]
    fn test_experience_holonic_transcendence() {
        let mut experience = ExperienceMindArchetype::new();

        // Start at Meso level
        assert_eq!(experience.holonic_level(), HolonicLevel::Meso);

        // Try to transcend without sufficient integration capacity
        let result = experience.transcend();
        assert!(!result);
        assert_eq!(experience.holonic_level(), HolonicLevel::Meso);

        // Increase integration capacity and transcend
        experience.integration_capacity = 0.85;
        let result = experience.transcend();
        assert!(result);
        assert_eq!(experience.holonic_level(), HolonicLevel::Macro);
    }

    #[test]
    fn test_experience_developmental_position() {
        let experience = ExperienceMindArchetype::new();

        assert_eq!(
            experience.developmental_position(),
            DevelopmentalPosition::Experience
        );

        let activated_rungs = experience.activated_rungs();
        assert_eq!(activated_rungs.len(), 3);
        assert!(activated_rungs.contains(&Rung::R1));
        assert!(activated_rungs.contains(&Rung::R2));
        assert!(activated_rungs.contains(&Rung::R3));
    }

    // ============================================================================
    // PHASE 6: BIAS STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn test_bias_structure_initialization() {
        let experience = ExperienceMindArchetype::new();

        // Bias should be initialized to neutral
        assert!((experience.bias_structure.perceptual_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.cognitive_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.emotional_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.behavioral_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.spiritual_bias - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_bias_structure_total_bias() {
        let bias = BiasStructure {
            perceptual_bias: 0.5,
            cognitive_bias: 0.5,
            emotional_bias: 0.5,
            behavioral_bias: 0.5,
            spiritual_bias: 0.5,
        };

        let total = bias.total_bias();
        assert!((total - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_bias_structure_direction_positive() {
        let bias = BiasStructure {
            perceptual_bias: 0.8,
            cognitive_bias: 0.8,
            emotional_bias: 0.8,
            behavioral_bias: 0.8,
            spiritual_bias: 0.8,
        };

        let direction = bias.bias_direction();
        assert!((direction - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bias_structure_direction_negative() {
        let bias = BiasStructure {
            perceptual_bias: -0.8,
            cognitive_bias: -0.8,
            emotional_bias: -0.8,
            behavioral_bias: -0.8,
            spiritual_bias: -0.8,
        };

        let direction = bias.bias_direction();
        // Total bias = -0.8, which is < 0.3, so direction = -1.0
        assert!((direction - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_bias_structure_direction_neutral() {
        let bias = BiasStructure {
            perceptual_bias: 0.05,
            cognitive_bias: 0.05,
            emotional_bias: 0.05,
            behavioral_bias: 0.05,
            spiritual_bias: 0.05,
        };

        let direction = bias.bias_direction();
        // Total bias = 0.05, which is < 0.1, so direction = 0.0
        assert!((direction - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_form_continuing_bias() {
        let mut experience = ExperienceMindArchetype::new();
        let initial_perceptual = experience.bias_structure.perceptual_bias;

        // Form bias from experience
        experience.form_continuing_bias(0.8);

        // Bias should have increased
        assert!(experience.bias_structure.perceptual_bias > initial_perceptual);
        assert!(experience.continuing_bias_strength > 0.0);

        // Bias strength evolution should have one entry
        assert_eq!(experience.bias_strength_evolution.len(), 1);
    }

    #[test]
    fn test_form_continuing_bias_clamping() {
        let mut experience = ExperienceMindArchetype::new();

        // Test positive clamping - set bias to near maximum
        experience.bias_structure.perceptual_bias = 0.97;
        experience.bias_structure.cognitive_bias = 0.97;
        experience.bias_structure.emotional_bias = 0.97;
        experience.bias_structure.behavioral_bias = 0.97;
        experience.bias_structure.spiritual_bias = 0.97;

        // Form bias from experience (should clamp to 1.0)
        experience.form_continuing_bias(1.0);

        // Bias should be clamped to 1.0
        assert!((experience.bias_structure.perceptual_bias - 1.0).abs() < 0.001);

        // Test that bias stays clamped at 1.0
        experience.form_continuing_bias(1.0);
        experience.form_continuing_bias(1.0);

        // Bias should still be clamped to 1.0
        assert!((experience.bias_structure.perceptual_bias - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_get_bias_direction() {
        let mut experience = ExperienceMindArchetype::new();

        // Initially neutral (total bias = 0)
        assert_eq!(experience.get_bias_direction(), 0.0);

        // Set positive bias directly
        experience.bias_structure.perceptual_bias = 0.8;
        experience.bias_structure.cognitive_bias = 0.8;
        experience.bias_structure.emotional_bias = 0.8;
        experience.bias_structure.behavioral_bias = 0.8;
        experience.bias_structure.spiritual_bias = 0.8;

        assert_eq!(experience.get_bias_direction(), 1.0);

        // Reset and set negative bias directly
        experience.reset_bias();
        experience.bias_structure.perceptual_bias = -0.8;
        experience.bias_structure.cognitive_bias = -0.8;
        experience.bias_structure.emotional_bias = -0.8;
        experience.bias_structure.behavioral_bias = -0.8;
        experience.bias_structure.spiritual_bias = -0.8;

        assert_eq!(experience.get_bias_direction(), -1.0);

        // Reset and set neutral bias (0.05 average)
        experience.reset_bias();
        experience.bias_structure.perceptual_bias = 0.05;
        experience.bias_structure.cognitive_bias = 0.05;
        experience.bias_structure.emotional_bias = 0.05;
        experience.bias_structure.behavioral_bias = 0.05;
        experience.bias_structure.spiritual_bias = 0.05;

        assert_eq!(experience.get_bias_direction(), 0.0);
    }

    #[test]
    fn test_get_total_bias() {
        let mut experience = ExperienceMindArchetype::new();

        // Set specific bias values
        experience.bias_structure.perceptual_bias = 0.5;
        experience.bias_structure.cognitive_bias = 0.5;
        experience.bias_structure.emotional_bias = 0.5;
        experience.bias_structure.behavioral_bias = 0.5;
        experience.bias_structure.spiritual_bias = 0.5;

        let total = experience.get_total_bias();
        assert!((total - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_bias_strength_evolution() {
        let mut experience = ExperienceMindArchetype::new();

        // Form bias multiple times
        for _ in 0..5 {
            experience.form_continuing_bias(0.7);
        }

        // Should have 5 entries in evolution history
        assert_eq!(experience.bias_strength_evolution.len(), 5);
    }

    #[test]
    fn test_bias_strength_evolution_limit() {
        let mut experience = ExperienceMindArchetype::new();

        // Form bias more than 100 times
        for _ in 0..150 {
            experience.form_continuing_bias(0.7);
        }

        // Should be limited to 100 entries
        assert_eq!(experience.bias_strength_evolution.len(), 100);
    }

    #[test]
    fn test_is_bias_positive() {
        let mut experience = ExperienceMindArchetype::new();

        // Set positive bias
        experience.bias_structure.perceptual_bias = 0.8;
        experience.bias_structure.cognitive_bias = 0.8;
        experience.bias_structure.emotional_bias = 0.8;
        experience.bias_structure.behavioral_bias = 0.8;
        experience.bias_structure.spiritual_bias = 0.8;

        assert!(experience.is_bias_positive());
        assert!(!experience.is_bias_negative());
    }

    #[test]
    fn test_is_bias_negative() {
        let mut experience = ExperienceMindArchetype::new();

        // Set negative bias
        experience.bias_structure.perceptual_bias = -0.8;
        experience.bias_structure.cognitive_bias = -0.8;
        experience.bias_structure.emotional_bias = -0.8;
        experience.bias_structure.behavioral_bias = -0.8;
        experience.bias_structure.spiritual_bias = -0.8;

        assert!(experience.is_bias_negative());
        assert!(!experience.is_bias_positive());
    }

    #[test]
    fn test_is_bias_neutral() {
        let experience = ExperienceMindArchetype::new();

        assert!(experience.is_bias_neutral());
        assert!(!experience.is_bias_positive());
        assert!(!experience.is_bias_negative());
    }

    #[test]
    fn test_get_bias_dimension() {
        let mut experience = ExperienceMindArchetype::new();

        experience.bias_structure.perceptual_bias = 0.5;
        experience.bias_structure.cognitive_bias = 0.6;
        experience.bias_structure.emotional_bias = 0.7;
        experience.bias_structure.behavioral_bias = 0.8;
        experience.bias_structure.spiritual_bias = 0.9;

        assert!((experience.get_bias_dimension("perceptual") - 0.5).abs() < 0.001);
        assert!((experience.get_bias_dimension("cognitive") - 0.6).abs() < 0.001);
        assert!((experience.get_bias_dimension("emotional") - 0.7).abs() < 0.001);
        assert!((experience.get_bias_dimension("behavioral") - 0.8).abs() < 0.001);
        assert!((experience.get_bias_dimension("spiritual") - 0.9).abs() < 0.001);
        assert!((experience.get_bias_dimension("invalid") - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_reset_bias() {
        let mut experience = ExperienceMindArchetype::new();

        // Set bias values
        experience.bias_structure.perceptual_bias = 0.8;
        experience.bias_structure.cognitive_bias = 0.8;
        experience.bias_structure.emotional_bias = 0.8;
        experience.bias_structure.behavioral_bias = 0.8;
        experience.bias_structure.spiritual_bias = 0.8;
        experience.continuing_bias_strength = 0.8;
        experience.form_continuing_bias(0.7);

        // Reset bias
        experience.reset_bias();

        // All bias values should be zero
        assert!((experience.bias_structure.perceptual_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.cognitive_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.emotional_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.behavioral_bias - 0.0).abs() < 0.001);
        assert!((experience.bias_structure.spiritual_bias - 0.0).abs() < 0.001);
        assert!((experience.continuing_bias_strength - 0.0).abs() < 0.001);
        assert!(experience.bias_strength_evolution.is_empty());
    }

    #[test]
    fn test_bias_formation_rate() {
        let experience = ExperienceMindArchetype::new();

        // Bias formation rate should be initialized
        assert!(experience.bias_formation_rate > 0.0);
        assert!(experience.bias_formation_rate <= 1.0);
    }

    #[test]
    fn test_continuing_bias_strength_update() {
        let mut experience = ExperienceMindArchetype::new();

        // Form bias
        experience.form_continuing_bias(0.8);

        // Continuing bias strength should be updated based on total bias
        let total_bias = experience.bias_structure.total_bias();
        assert!((experience.continuing_bias_strength - total_bias.abs()).abs() < 0.001);
    }
}
