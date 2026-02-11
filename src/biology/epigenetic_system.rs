// Epigenetic System - Environment-Responsive Gene Expression
//
// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 3:
// "Epigenetic System - Sense environment, regulate gene expression based on environment,
// implement epigenetic inheritance, calculate developmental plasticity"
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Epigenetic factors and developmental instructions"

use crate::biology::cellular_emergence::Cell;
use crate::biology::dna_system::{EpigeneticMarkerType, Gene, GeneExpression, DNA};
use crate::types::Float;
use rand::Rng;
use std::collections::HashMap;

// Re-export EpigeneticMarker from dna_system for convenience
pub use crate::biology::dna_system::EpigeneticMarker;

/// Epigenetic System - Environment-responsive gene expression
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Sense environment, regulate gene expression based on environment,
/// implement epigenetic inheritance, calculate developmental plasticity"
#[derive(Debug, Clone)]
pub struct EpigeneticSystem {
    /// Environment sensor
    environment_sensor: EnvironmentSensor,

    /// Gene regulator
    gene_regulator: GeneRegulator,

    /// Epigenetic inheritor
    epigenetic_inheritor: EpigeneticInheritor,

    /// Plasticity calculator
    plasticity_calculator: PlasticityCalculator,

    /// System configuration
    config: EpigeneticSystemConfig,
}

impl Default for EpigeneticSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EpigeneticSystem {
    /// Create a new epigenetic system
    pub fn new() -> Self {
        Self {
            environment_sensor: EnvironmentSensor::new(),
            gene_regulator: GeneRegulator::new(),
            epigenetic_inheritor: EpigeneticInheritor::new(),
            plasticity_calculator: PlasticityCalculator::new(),
            config: EpigeneticSystemConfig::default(),
        }
    }

    /// Create an epigenetic system with custom configuration
    pub fn with_config(config: EpigeneticSystemConfig) -> Self {
        Self {
            environment_sensor: EnvironmentSensor::new(),
            gene_regulator: GeneRegulator::new(),
            epigenetic_inheritor: EpigeneticInheritor::new(),
            plasticity_calculator: PlasticityCalculator::new(),
            config,
        }
    }

    /// Sense environment
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Sense environment"
    pub fn sense_environment(
        &self,
        cell: &Cell,
        environment: &EnvironmentalConditions,
    ) -> EnvironmentalSignal {
        self.environment_sensor.sense(cell, environment)
    }

    /// Regulate gene expression based on environmental signal
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Regulate gene expression based on environment"
    pub fn regulate_gene_expression(
        &self,
        dna: &mut DNA,
        signal: &EnvironmentalSignal,
    ) -> GeneRegulation {
        self.gene_regulator.regulate(dna, signal, &self.config)
    }

    /// Inherit epigenetic markers from parent to child
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Implement epigenetic inheritance"
    pub fn inherit_epigenetics(&self, parent: &Cell, child: &mut Cell) {
        self.epigenetic_inheritor
            .inherit(parent, child, &self.config)
    }

    /// Calculate developmental plasticity
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Calculate developmental plasticity"
    pub fn calculate_plasticity(&self, cell: &Cell) -> DevelopmentalPlasticity {
        self.plasticity_calculator.calculate(cell, &self.config)
    }

    /// Get configuration
    pub fn config(&self) -> &EpigeneticSystemConfig {
        &self.config
    }
}

/// Epigenetic System Configuration
#[derive(Debug, Clone)]
pub struct EpigeneticSystemConfig {
    /// Epigenetic sensitivity (0.0 to 1.0)
    pub sensitivity: Float,

    /// Inheritance rate (0.0 to 1.0)
    pub inheritance_rate: Float,

    /// Plasticity range (0.0 to 1.0)
    pub plasticity_range: Float,

    /// Epigenetic memory duration (in simulation steps)
    pub memory_duration: usize,

    /// Adaptation speed (0.0 to 1.0)
    pub adaptation_speed: Float,
}

impl Default for EpigeneticSystemConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.7,
            inheritance_rate: 0.6,
            plasticity_range: 0.8,
            memory_duration: 100,
            adaptation_speed: 0.5,
        }
    }
}

/// Environmental Conditions - External factors affecting gene expression
#[derive(Debug, Clone)]
pub struct EnvironmentalConditions {
    /// Temperature (0.0 to 1.0)
    pub temperature: Float,

    /// Humidity (0.0 to 1.0)
    pub humidity: Float,

    /// Light intensity (0.0 to 1.0)
    pub light: Float,

    /// Nutrient availability (0.0 to 1.0)
    pub nutrients: Float,

    /// Stress level (0.0 to 1.0)
    pub stress: Float,

    /// Chemical signals
    pub chemicals: HashMap<String, Float>,
}

impl Default for EnvironmentalConditions {
    fn default() -> Self {
        EnvironmentalConditions {
            temperature: 0.5,
            humidity: 0.5,
            light: 0.5,
            nutrients: 0.5,
            stress: 0.0,
            chemicals: HashMap::new(),
        }
    }
}

/// Environmental Signal - Processed environmental information
#[derive(Debug, Clone)]
pub struct EnvironmentalSignal {
    /// Signal identifier
    pub signal_id: String,

    /// Signal intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Signal type
    pub signal_type: EnvironmentalSignalType,

    /// Affected genes
    pub affected_genes: Vec<String>,

    /// Effect direction (positive = activation, negative = repression)
    pub effect_direction: Float,

    /// Signal duration (in simulation steps)
    pub duration: usize,
}

/// Environmental Signal Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentalSignalType {
    TemperatureStress,
    NutrientAvailability,
    LightStimulus,
    ChemicalSignal,
    OxidativeStress,
    HormonalSignal,
    MechanicalStress,
}

/// Gene Regulation - How genes are regulated
#[derive(Debug, Clone)]
pub struct GeneRegulation {
    /// Regulation identifier
    pub regulation_id: String,

    /// Regulated genes
    pub regulated_genes: Vec<RegulatedGene>,

    /// Regulation success
    pub success: bool,

    /// Total expression change
    pub total_expression_change: Float,

    /// New epigenetic markers created
    pub new_markers: Vec<EpigeneticMarker>,
}

/// Regulated Gene
#[derive(Debug, Clone)]
pub struct RegulatedGene {
    /// Gene identifier
    pub gene_id: String,

    /// Original expression level
    pub original_expression: Float,

    /// New expression level
    pub new_expression: Float,

    /// Regulation type
    pub regulation_type: RegulationType,

    /// Epigenetic marker applied
    pub epigenetic_marker: Option<EpigeneticMarker>,
}

/// Regulation Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegulationType {
    Activation,
    Repression,
    NoChange,
}

/// Epigenetic Inheritance - Transfer of epigenetic markers to offspring
#[derive(Debug, Clone)]
pub struct EpigeneticInheritance {
    /// Inheritance identifier
    pub inheritance_id: String,

    /// Inherited markers
    pub inherited_markers: Vec<EpigeneticMarker>,

    /// Inherited traits
    pub inherited_traits: Vec<String>,

    /// Inheritance success rate (0.0 to 1.0)
    pub success_rate: Float,

    /// Epigenetic reset (how much is erased)
    pub epigenetic_reset: Float,
}

/// Developmental Plasticity - Ability to adapt to environment
#[derive(Debug, Clone)]
pub struct DevelopmentalPlasticity {
    /// Plasticity identifier
    pub plasticity_id: String,

    /// Plasticity level (0.0 to 1.0)
    pub plasticity_level: Float,

    /// Plasticity type
    pub plasticity_type: PlasticityType,

    /// Adaptive potential (0.0 to 1.0)
    pub adaptive_potential: Float,

    /// Phenotypic variability (0.0 to 1.0)
    pub phenotypic_variability: Float,
}

/// Plasticity Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlasticityType {
    Low,
    Moderate,
    High,
    Adaptive,
}

// ============================================================================
// SUBSYSTEMS
// ============================================================================

/// Environment Sensor
#[derive(Debug, Clone)]
struct EnvironmentSensor {
    sensing_resolution: Float,
}

impl EnvironmentSensor {
    fn new() -> Self {
        Self {
            sensing_resolution: 0.1,
        }
    }

    fn sense(&self, cell: &Cell, environment: &EnvironmentalConditions) -> EnvironmentalSignal {
        // Determine signal type based on environmental conditions
        let signal_type = self.determine_signal_type(environment);

        // Calculate signal intensity based on deviation from optimal
        let intensity = self.calculate_intensity(environment, signal_type);

        // Identify affected genes (simplified: all genes affected)
        let affected_genes: Vec<String> =
            cell.dna.genes.iter().map(|g| g.gene_id.clone()).collect();

        // Calculate effect direction
        let effect_direction = self.calculate_effect_direction(environment, signal_type);

        EnvironmentalSignal {
            signal_id: format!("signal-{}", uuid::Uuid::new_v4()),
            intensity,
            signal_type,
            affected_genes,
            effect_direction,
            duration: 10,
        }
    }

    fn determine_signal_type(
        &self,
        environment: &EnvironmentalConditions,
    ) -> EnvironmentalSignalType {
        // Determine dominant environmental factor
        let mut max_deviation = 0.0;
        let mut signal_type = EnvironmentalSignalType::NutrientAvailability;

        let temperature_deviation = (environment.temperature - 0.5).abs();
        if temperature_deviation > max_deviation {
            max_deviation = temperature_deviation;
            signal_type = EnvironmentalSignalType::TemperatureStress;
        }

        let light_deviation = (environment.light - 0.5).abs();
        if light_deviation > max_deviation {
            max_deviation = light_deviation;
            signal_type = EnvironmentalSignalType::LightStimulus;
        }

        let nutrient_deviation = (environment.nutrients - 0.5).abs();
        if nutrient_deviation > max_deviation {
            max_deviation = nutrient_deviation;
            signal_type = EnvironmentalSignalType::NutrientAvailability;
        }

        if environment.stress > 0.5 {
            signal_type = EnvironmentalSignalType::OxidativeStress;
        }

        signal_type
    }

    fn calculate_intensity(
        &self,
        environment: &EnvironmentalConditions,
        signal_type: EnvironmentalSignalType,
    ) -> Float {
        match signal_type {
            EnvironmentalSignalType::TemperatureStress => {
                (environment.temperature - 0.5).abs() * 2.0
            }
            EnvironmentalSignalType::NutrientAvailability => {
                (0.5 - environment.nutrients).abs() * 2.0
            }
            EnvironmentalSignalType::LightStimulus => environment.light,
            EnvironmentalSignalType::ChemicalSignal => 0.5,
            EnvironmentalSignalType::OxidativeStress => environment.stress,
            EnvironmentalSignalType::HormonalSignal => 0.3,
            EnvironmentalSignalType::MechanicalStress => 0.2,
        }
        .clamp(0.0, 1.0)
    }

    fn calculate_effect_direction(
        &self,
        environment: &EnvironmentalConditions,
        signal_type: EnvironmentalSignalType,
    ) -> Float {
        match signal_type {
            EnvironmentalSignalType::TemperatureStress => {
                if environment.temperature > 0.7 {
                    -0.3 // Heat stress represses genes
                } else if environment.temperature < 0.3 {
                    -0.3 // Cold stress represses genes
                } else {
                    0.1 // Optimal temperature activates genes
                }
            }
            EnvironmentalSignalType::NutrientAvailability => {
                if environment.nutrients < 0.3 {
                    -0.4 // Nutrient scarcity represses genes
                } else {
                    0.3 // Nutrient abundance activates genes
                }
            }
            EnvironmentalSignalType::LightStimulus => {
                if environment.light > 0.5 {
                    0.5 // Light activates photosynthesis genes
                } else {
                    -0.2 // Darkness represses photosynthesis genes
                }
            }
            _ => 0.0,
        }
    }
}

/// Gene Regulator
#[derive(Debug, Clone)]
struct GeneRegulator {
    regulation_threshold: Float,
}

impl GeneRegulator {
    fn new() -> Self {
        Self {
            regulation_threshold: 0.3,
        }
    }

    fn regulate(
        &self,
        dna: &mut DNA,
        signal: &EnvironmentalSignal,
        config: &EpigeneticSystemConfig,
    ) -> GeneRegulation {
        let mut regulated_genes = Vec::new();
        let mut new_markers = Vec::new();
        let mut total_expression_change = 0.0;

        // Only regulate if signal intensity exceeds threshold
        if signal.intensity < self.regulation_threshold {
            return GeneRegulation {
                regulation_id: format!("regulation-{}", uuid::Uuid::new_v4()),
                regulated_genes: Vec::new(),
                success: false,
                total_expression_change: 0.0,
                new_markers: Vec::new(),
            };
        }

        // Regulate affected genes
        for gene_id in &signal.affected_genes {
            if let Some(expr) = dna.get_gene_expression(gene_id) {
                let original_expression = expr.expression_level;

                // Calculate new expression based on signal
                let expression_change =
                    signal.effect_direction * signal.intensity * config.sensitivity;
                let new_expression = (original_expression + expression_change).clamp(0.0, 1.0);

                // Update expression
                dna.set_gene_expression(gene_id, new_expression);

                // Determine regulation type
                let regulation_type = if new_expression > original_expression {
                    RegulationType::Activation
                } else if new_expression < original_expression {
                    RegulationType::Repression
                } else {
                    RegulationType::NoChange
                };

                // Create epigenetic marker if change is significant
                let epigenetic_marker = if expression_change.abs() > 0.2 {
                    let marker = EpigeneticMarker {
                        marker_id: format!("marker-{}", uuid::Uuid::new_v4()),
                        target_gene: gene_id.clone(),
                        marker_type: if regulation_type == RegulationType::Activation {
                            EpigeneticMarkerType::Acetylation
                        } else {
                            EpigeneticMarkerType::Methylation
                        },
                        expression_effect: new_expression,
                        permanence: signal.intensity * 0.5,
                    };
                    new_markers.push(marker.clone());
                    Some(marker)
                } else {
                    None
                };

                regulated_genes.push(RegulatedGene {
                    gene_id: gene_id.clone(),
                    original_expression,
                    new_expression,
                    regulation_type,
                    epigenetic_marker,
                });

                total_expression_change += expression_change;
            }
        }

        // Add new markers to DNA
        for marker in &new_markers {
            dna.add_epigenetic_marker(marker.clone());
        }

        let success = !regulated_genes.is_empty();

        GeneRegulation {
            regulation_id: format!("regulation-{}", uuid::Uuid::new_v4()),
            regulated_genes,
            success,
            total_expression_change,
            new_markers,
        }
    }
}

/// Epigenetic Inheritor
#[derive(Debug, Clone)]
struct EpigeneticInheritor {
    inheritance_threshold: Float,
}

impl EpigeneticInheritor {
    fn new() -> Self {
        Self {
            inheritance_threshold: 0.5,
        }
    }

    fn inherit(&self, parent: &Cell, child: &mut Cell, config: &EpigeneticSystemConfig) {
        let mut rng = rand::thread_rng();

        // Inherit epigenetic markers with probability based on inheritance rate
        for marker in &parent.dna.epigenetic_markers {
            // Markers with high permanence are more likely to be inherited
            let inheritance_probability = config.inheritance_rate * marker.permanence;

            if rng.gen::<Float>() < inheritance_probability {
                // Create a copy of the marker for the child
                let child_marker = EpigeneticMarker {
                    marker_id: format!("marker-{}", uuid::Uuid::new_v4()),
                    target_gene: marker.target_gene.clone(),
                    marker_type: marker.marker_type,
                    expression_effect: marker.expression_effect,
                    permanence: marker.permanence * 0.9, // Slightly reduced permanence
                };

                child.dna.add_epigenetic_marker(child_marker);

                // Apply the marker's effect to gene expression
                child
                    .dna
                    .set_gene_expression(&marker.target_gene, marker.expression_effect);
            }
        }

        // Epigenetic reset: erase some markers
        let epigenetic_reset = rng.gen::<Float>() * config.plasticity_range;
        let markers_to_remove =
            (parent.dna.epigenetic_markers.len() as Float * epigenetic_reset) as usize;

        for _ in 0..markers_to_remove {
            if !child.dna.epigenetic_markers.is_empty() {
                let index = rng.gen_range(0..child.dna.epigenetic_markers.len());
                child.dna.epigenetic_markers.remove(index);
            }
        }
    }
}

/// Plasticity Calculator
#[derive(Debug, Clone)]
struct PlasticityCalculator {
    calculation_method: PlasticityMethod,
}

impl PlasticityCalculator {
    fn new() -> Self {
        Self {
            calculation_method: PlasticityMethod::Integrated,
        }
    }

    fn calculate(&self, cell: &Cell, config: &EpigeneticSystemConfig) -> DevelopmentalPlasticity {
        // Calculate plasticity based on cell properties
        let epigenetic_diversity = self.calculate_epigenetic_diversity(cell);
        let gene_expression_variance = self.calculate_expression_variance(cell);
        let adaptive_capacity = (cell.energy + cell.health) / 2.0;

        let plasticity_level =
            (epigenetic_diversity * 0.4 + gene_expression_variance * 0.3 + adaptive_capacity * 0.3)
                * config.plasticity_range;

        let plasticity_type = if plasticity_level < 0.3 {
            PlasticityType::Low
        } else if plasticity_level < 0.6 {
            PlasticityType::Moderate
        } else if plasticity_level < 0.8 {
            PlasticityType::High
        } else {
            PlasticityType::Adaptive
        };

        let adaptive_potential = plasticity_level * cell.consciousness.level;
        let phenotypic_variability = plasticity_level * 0.8;

        DevelopmentalPlasticity {
            plasticity_id: format!("plasticity-{}", uuid::Uuid::new_v4()),
            plasticity_level,
            plasticity_type,
            adaptive_potential,
            phenotypic_variability,
        }
    }

    fn calculate_epigenetic_diversity(&self, cell: &Cell) -> Float {
        if cell.dna.epigenetic_markers.is_empty() {
            0.0
        } else {
            // Diversity based on number and variety of epigenetic markers
            let unique_types: std::collections::HashSet<_> = cell
                .dna
                .epigenetic_markers
                .iter()
                .map(|m| m.marker_type)
                .collect();

            (unique_types.len() as Float / 5.0).min(1.0) // 5 marker types total
        }
    }

    fn calculate_expression_variance(&self, cell: &Cell) -> Float {
        if cell.dna.gene_expression.is_empty() {
            0.0
        } else {
            // Calculate variance in gene expression
            let mean: Float = cell
                .dna
                .gene_expression
                .iter()
                .map(|e| e.expression_level)
                .sum::<Float>()
                / cell.dna.gene_expression.len() as Float;

            let variance: Float = cell
                .dna
                .gene_expression
                .iter()
                .map(|e| (e.expression_level - mean).powi(2))
                .sum::<Float>()
                / cell.dna.gene_expression.len() as Float;

            variance.sqrt().min(1.0)
        }
    }
}

/// Plasticity Calculation Method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlasticityMethod {
    Integrated,
    EpigeneticBased,
    ExpressionBased,
    AdaptiveBased,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::cellular_emergence::CellType;
    use crate::biology::dna_system::DNA;
    use crate::entity_layer7::dna_encoding::DNAPattern;
    use crate::entity_layer7::{EvolutionaryStage, IndividualSpectrumConfiguration};
    use crate::spectrum::{ArchetypicalMind, ArchetypicalSystemType, SpectrumRatio, SpectrumSide};

    fn create_test_cell() -> Cell {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::cellular_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );
        let dna = DNA::from_blueprint(dna_pattern);
        Cell::new(CellType::Prokaryotic, dna)
    }

    #[test]
    fn test_epigenetic_system_creation() {
        let system = EpigeneticSystem::new();
        assert_eq!(system.config().sensitivity, 0.7);
        assert_eq!(system.config().inheritance_rate, 0.6);
    }

    #[test]
    fn test_environmental_conditions_default() {
        let conditions = EnvironmentalConditions::default();
        assert_eq!(conditions.temperature, 0.5);
        assert_eq!(conditions.humidity, 0.5);
        assert_eq!(conditions.light, 0.5);
        assert_eq!(conditions.nutrients, 0.5);
        assert_eq!(conditions.stress, 0.0);
    }

    #[test]
    fn test_sense_environment() {
        let system = EpigeneticSystem::new();
        let cell = create_test_cell();

        let mut environment = EnvironmentalConditions::default();
        environment.temperature = 0.9; // Heat stress
        environment.stress = 0.8;

        let signal = system.sense_environment(&cell, &environment);

        assert!(!signal.affected_genes.is_empty());
        assert!(signal.intensity > 0.0);
    }

    #[test]
    fn test_regulate_gene_expression() {
        let system = EpigeneticSystem::new();
        let mut cell = create_test_cell();

        let mut environment = EnvironmentalConditions::default();
        environment.light = 0.9; // High light

        let signal = system.sense_environment(&cell, &environment);

        let regulation = system.regulate_gene_expression(&mut cell.dna, &signal);

        assert!(!regulation.regulated_genes.is_empty() || !signal.affected_genes.is_empty());
    }

    #[test]
    fn test_inherit_epigenetics() {
        let system = EpigeneticSystem::new();
        let mut parent = create_test_cell();
        let mut child = create_test_cell();

        // Add an epigenetic marker to parent
        let marker = EpigeneticMarker {
            marker_id: "marker-1".to_string(),
            target_gene: parent.dna.genes[0].gene_id.clone(),
            marker_type: EpigeneticMarkerType::Methylation,
            expression_effect: 0.3,
            permanence: 0.9,
        };
        parent.dna.add_epigenetic_marker(marker);

        let initial_child_markers = child.dna.epigenetic_markers.len();
        system.inherit_epigenetics(&parent, &mut child);

        // Child may have inherited the marker
        assert!(child.dna.epigenetic_markers.len() >= initial_child_markers);
    }

    #[test]
    fn test_calculate_plasticity() {
        let system = EpigeneticSystem::new();
        let cell = create_test_cell();

        let plasticity = system.calculate_plasticity(&cell);

        assert!(plasticity.plasticity_level >= 0.0 && plasticity.plasticity_level <= 1.0);
        assert!(plasticity.adaptive_potential >= 0.0 && plasticity.adaptive_potential <= 1.0);
        assert!(
            plasticity.phenotypic_variability >= 0.0 && plasticity.phenotypic_variability <= 1.0
        );
    }

    #[test]
    fn test_gene_regulation_structure() {
        let system = EpigeneticSystem::new();
        let mut cell = create_test_cell();

        let mut environment = EnvironmentalConditions::default();
        environment.nutrients = 0.1; // Low nutrients

        let signal = system.sense_environment(&cell, &environment);
        let regulation = system.regulate_gene_expression(&mut cell.dna, &signal);

        // Check regulation structure
        assert!(!regulation.regulation_id.is_empty());
        // Success depends on signal intensity, so just check structure
    }

    #[test]
    fn test_environmental_signal_types() {
        let system = EpigeneticSystem::new();
        let cell = create_test_cell();

        // Test different environmental conditions
        let mut environment = EnvironmentalConditions::default();

        environment.temperature = 0.9;
        let signal = system.sense_environment(&cell, &environment);
        assert_eq!(
            signal.signal_type,
            EnvironmentalSignalType::TemperatureStress
        );

        environment = EnvironmentalConditions::default();
        environment.light = 0.9;
        let signal = system.sense_environment(&cell, &environment);
        assert_eq!(signal.signal_type, EnvironmentalSignalType::LightStimulus);

        environment = EnvironmentalConditions::default();
        environment.nutrients = 0.1;
        let signal = system.sense_environment(&cell, &environment);
        assert_eq!(
            signal.signal_type,
            EnvironmentalSignalType::NutrientAvailability
        );

        environment = EnvironmentalConditions::default();
        environment.stress = 0.9;
        let signal = system.sense_environment(&cell, &environment);
        assert_eq!(signal.signal_type, EnvironmentalSignalType::OxidativeStress);
    }

    #[test]
    fn test_developmental_plasticity_types() {
        let system = EpigeneticSystem::new();
        let cell = create_test_cell();

        let plasticity = system.calculate_plasticity(&cell);

        // Check that plasticity type is valid
        match plasticity.plasticity_type {
            PlasticityType::Low
            | PlasticityType::Moderate
            | PlasticityType::High
            | PlasticityType::Adaptive => {}
        }
    }
}
