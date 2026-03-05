//! Emergence Validator - Phase 6 Integration & Testing
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 6:
//! "Validate true emergence across biological, noospheric, and Gaia systems"
//!
//! This module provides validation mechanisms for:
//! - Biological emergence: Shannon diversity index >= 2.0
//! - Noospheric emergence: Cultural diversity index >= 1.5
//! - Gaia emergence: Ecosystem stability index >= 0.8

use crate::integrated_system::{
    BiologicalEmergence, GaiaEmergence, NoosphericEmergence, SimulationState,
};
use crate::types::Float;
use std::collections::HashMap;

/// Emergence Validator
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Clear criteria for what constitutes 'true emergence'"
#[derive(Debug, Clone, Default)]
pub struct EmergenceValidator {
    /// Biological criteria thresholds
    biological_criteria: BiologicalCriteria,

    /// Noospheric criteria thresholds
    noospheric_criteria: NoosphericCriteria,

    /// Gaia criteria thresholds
    gaia_criteria: GaiaCriteria,
}

/// Biological emergence criteria
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Biological Complexity: Shannon diversity index >= 2.0"
#[derive(Debug, Clone, Copy)]
pub struct BiologicalCriteria {
    /// Minimum cell count for valid emergence
    pub min_cell_count: usize,

    /// Minimum species count for valid emergence
    pub min_species_count: usize,

    /// Minimum ecosystem count for valid emergence
    pub min_ecosystem_count: usize,

    /// Minimum genetic diversity (Shannon index)
    pub min_genetic_diversity: Float,

    /// Maximum acceptable mutation rate
    pub max_mutation_rate: Float,

    /// Minimum protein diversity
    pub min_protein_diversity: Float,
}

impl Default for BiologicalCriteria {
    fn default() -> Self {
        Self {
            min_cell_count: 100,
            min_species_count: 10,
            min_ecosystem_count: 3,
            min_genetic_diversity: 2.0,
            max_mutation_rate: 0.01,
            min_protein_diversity: 0.5,
        }
    }
}

/// Noospheric emergence criteria
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Noospheric Complexity: Cultural diversity index >= 1.5"
#[derive(Debug, Clone, Copy)]
pub struct NoosphericCriteria {
    /// Minimum social memory complexes
    pub min_social_complexes: usize,

    /// Minimum collective intelligence score
    pub min_collective_intelligence: Float,

    /// Minimum cultural diversity index
    pub min_cultural_diversity: Float,

    /// Minimum ideological diversity index
    pub min_ideological_diversity: Float,

    /// Minimum societal complexity score
    pub min_societal_complexity: Float,

    /// Minimum telepathic link strength
    pub min_telepathic_strength: Float,
}

impl Default for NoosphericCriteria {
    fn default() -> Self {
        Self {
            min_social_complexes: 5,
            min_collective_intelligence: 0.8,
            min_cultural_diversity: 1.5,
            min_ideological_diversity: 1.0,
            min_societal_complexity: 0.7,
            min_telepathic_strength: 0.5,
        }
    }
}

/// Gaia emergence criteria
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Gaia Health: Ecosystem stability index >= 0.8"
#[derive(Debug, Clone, Copy)]
pub struct GaiaCriteria {
    /// Minimum planetary consciousness score
    pub min_consciousness_score: Float,

    /// Minimum ecosystem stability
    pub min_ecosystem_stability: Float,

    /// Minimum atmospheric health
    pub min_atmospheric_health: Float,

    /// Minimum climate stability
    pub min_climate_stability: Float,

    /// Minimum resource balance
    pub min_resource_balance: Float,

    /// Maximum pollution level (lower is better)
    pub max_pollution_level: Float,
}

impl Default for GaiaCriteria {
    fn default() -> Self {
        Self {
            min_consciousness_score: 0.6,
            min_ecosystem_stability: 0.8,
            min_atmospheric_health: 0.7,
            min_climate_stability: 0.75,
            min_resource_balance: 0.7,
            max_pollution_level: 0.3,
        }
    }
}

/// Validation result
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Overall validation passed
    pub passed: bool,

    /// Biological validation result
    pub biological: BiologicalValidation,

    /// Noospheric validation result
    pub noospheric: NoosphericValidation,

    /// Gaia validation result
    pub gaia: GaiaValidation,

    /// Overall confidence score (0.0-1.0)
    pub confidence: Float,

    /// Validation details
    pub details: HashMap<String, String>,
}

/// Biological validation result
#[derive(Debug, Clone, Default)]
pub struct BiologicalValidation {
    /// Validation passed
    pub passed: bool,

    /// Cell count valid
    pub cells_valid: bool,

    /// Species count valid
    pub species_valid: bool,

    /// Ecosystem count valid
    pub ecosystems_valid: bool,

    /// Genetic diversity valid
    pub genetic_diversity_valid: bool,

    /// Mutation rate valid
    pub mutation_rate_valid: bool,

    /// Overall biological score (0.0-1.0)
    pub overall_score: Float,

    /// Detailed metrics
    pub metrics: HashMap<String, Float>,
}

/// Noospheric validation result
#[derive(Debug, Clone, Default)]
pub struct NoosphericValidation {
    /// Validation passed
    pub passed: bool,

    /// Social complexes valid
    pub social_complexes_valid: bool,

    /// Collective intelligence valid
    pub collective_intelligence_valid: bool,

    /// Cultural diversity valid
    pub cultural_diversity_valid: bool,

    /// Ideological diversity valid
    pub ideological_diversity_valid: bool,

    /// Overall noospheric score (0.0-1.0)
    pub overall_score: Float,

    /// Detailed metrics
    pub metrics: HashMap<String, Float>,
}

/// Gaia validation result
#[derive(Debug, Clone, Default)]
pub struct GaiaValidation {
    /// Validation passed
    pub passed: bool,

    /// Consciousness score valid
    pub consciousness_valid: bool,

    /// Ecosystem stability valid
    pub ecosystem_stability_valid: bool,

    /// Atmospheric health valid
    pub atmospheric_health_valid: bool,

    /// Climate stability valid
    pub climate_stability_valid: bool,

    /// Resource balance valid
    pub resource_balance_valid: bool,

    /// Pollution level valid
    pub pollution_level_valid: bool,

    /// Overall Gaia score (0.0-1.0)
    pub overall_score: Float,

    /// Detailed metrics
    pub metrics: HashMap<String, Float>,
}

impl EmergenceValidator {
    /// Create a new emergence validator with default criteria
    pub fn new() -> Self {
        Self {
            biological_criteria: BiologicalCriteria::default(),
            noospheric_criteria: NoosphericCriteria::default(),
            gaia_criteria: GaiaCriteria::default(),
        }
    }

    /// Create a new emergence validator with custom criteria
    pub fn with_criteria(
        biological: BiologicalCriteria,
        noospheric: NoosphericCriteria,
        gaia: GaiaCriteria,
    ) -> Self {
        Self {
            biological_criteria: biological,
            noospheric_criteria: noospheric,
            gaia_criteria: gaia,
        }
    }

    /// Validate biological emergence
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Biological Complexity: Shannon diversity index >= 2.0"
    pub fn validate_biological(&self, emergence: &BiologicalEmergence) -> BiologicalValidation {
        let mut validation = BiologicalValidation::default();
        let criteria = self.biological_criteria;
        let mut metrics = HashMap::new();

        // Validate cell count
        validation.cells_valid = emergence.cell_count >= criteria.min_cell_count;
        metrics.insert("cell_count".to_string(), emergence.cell_count as Float);
        metrics.insert(
            "min_cell_count".to_string(),
            criteria.min_cell_count as Float,
        );

        // Validate species count
        validation.species_valid = emergence.species_count >= criteria.min_species_count;
        metrics.insert(
            "species_count".to_string(),
            emergence.species_count as Float,
        );
        metrics.insert(
            "min_species_count".to_string(),
            criteria.min_species_count as Float,
        );

        // Validate ecosystem count
        validation.ecosystems_valid = emergence.ecosystem_count >= criteria.min_ecosystem_count;
        metrics.insert(
            "ecosystem_count".to_string(),
            emergence.ecosystem_count as Float,
        );
        metrics.insert(
            "min_ecosystem_count".to_string(),
            criteria.min_ecosystem_count as Float,
        );

        // Validate genetic diversity
        validation.genetic_diversity_valid =
            emergence.genetic_diversity >= criteria.min_genetic_diversity;
        metrics.insert("genetic_diversity".to_string(), emergence.genetic_diversity);
        metrics.insert(
            "min_genetic_diversity".to_string(),
            criteria.min_genetic_diversity,
        );

        // Validate mutation rate
        validation.mutation_rate_valid = emergence.mutation_rate <= criteria.max_mutation_rate;
        metrics.insert("mutation_rate".to_string(), emergence.mutation_rate);
        metrics.insert("max_mutation_rate".to_string(), criteria.max_mutation_rate);

        // Calculate overall score
        let valid_count = [
            validation.cells_valid,
            validation.species_valid,
            validation.ecosystems_valid,
            validation.genetic_diversity_valid,
            validation.mutation_rate_valid,
        ]
        .iter()
        .filter(|&&v| v)
        .count() as Float;

        validation.overall_score = valid_count / 5.0;
        validation.passed = validation.overall_score >= 0.6;
        validation.metrics = metrics;

        validation
    }

    /// Validate noospheric emergence
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Noospheric Complexity: Cultural diversity index >= 1.5"
    pub fn validate_noospheric(&self, emergence: &NoosphericEmergence) -> NoosphericValidation {
        let mut validation = NoosphericValidation::default();
        let criteria = self.noospheric_criteria;
        let mut metrics = HashMap::new();

        // Validate social memory complexes
        validation.social_complexes_valid =
            emergence.social_complexes_count >= criteria.min_social_complexes;
        metrics.insert(
            "social_complexes".to_string(),
            emergence.social_complexes_count as Float,
        );
        metrics.insert(
            "min_social_complexes".to_string(),
            criteria.min_social_complexes as Float,
        );

        // Validate collective intelligence
        validation.collective_intelligence_valid =
            emergence.collective_intelligence >= criteria.min_collective_intelligence;
        metrics.insert(
            "collective_intelligence".to_string(),
            emergence.collective_intelligence,
        );
        metrics.insert(
            "min_collective_intelligence".to_string(),
            criteria.min_collective_intelligence,
        );

        // Validate cultural diversity
        validation.cultural_diversity_valid =
            emergence.cultural_diversity >= criteria.min_cultural_diversity;
        metrics.insert(
            "cultural_diversity".to_string(),
            emergence.cultural_diversity,
        );
        metrics.insert(
            "min_cultural_diversity".to_string(),
            criteria.min_cultural_diversity,
        );

        // Validate ideological diversity
        validation.ideological_diversity_valid =
            emergence.ideological_diversity >= criteria.min_ideological_diversity;
        metrics.insert(
            "ideological_diversity".to_string(),
            emergence.ideological_diversity,
        );
        metrics.insert(
            "min_ideological_diversity".to_string(),
            criteria.min_ideological_diversity,
        );

        // Calculate overall score
        let valid_count = [
            validation.social_complexes_valid,
            validation.collective_intelligence_valid,
            validation.cultural_diversity_valid,
            validation.ideological_diversity_valid,
        ]
        .iter()
        .filter(|&&v| v)
        .count() as Float;

        validation.overall_score = valid_count / 4.0;
        validation.passed = validation.overall_score >= 0.6;
        validation.metrics = metrics;

        validation
    }

    /// Validate Gaia emergence
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Gaia Health: Ecosystem stability index >= 0.8"
    pub fn validate_gaia(&self, emergence: &GaiaEmergence) -> GaiaValidation {
        let mut validation = GaiaValidation::default();
        let criteria = self.gaia_criteria;
        let mut metrics = HashMap::new();

        // Validate consciousness score
        validation.consciousness_valid =
            emergence.consciousness_score >= criteria.min_consciousness_score;
        metrics.insert(
            "consciousness_score".to_string(),
            emergence.consciousness_score,
        );
        metrics.insert(
            "min_consciousness_score".to_string(),
            criteria.min_consciousness_score,
        );

        // Validate ecosystem stability
        validation.ecosystem_stability_valid =
            emergence.ecosystem_stability >= criteria.min_ecosystem_stability;
        metrics.insert(
            "ecosystem_stability".to_string(),
            emergence.ecosystem_stability,
        );
        metrics.insert(
            "min_ecosystem_stability".to_string(),
            criteria.min_ecosystem_stability,
        );

        // Validate atmospheric health
        validation.atmospheric_health_valid =
            emergence.atmospheric_health >= criteria.min_atmospheric_health;
        metrics.insert(
            "atmospheric_health".to_string(),
            emergence.atmospheric_health,
        );
        metrics.insert(
            "min_atmospheric_health".to_string(),
            criteria.min_atmospheric_health,
        );

        // Validate climate stability
        validation.climate_stability_valid =
            emergence.climate_stability >= criteria.min_climate_stability;
        metrics.insert("climate_stability".to_string(), emergence.climate_stability);
        metrics.insert(
            "min_climate_stability".to_string(),
            criteria.min_climate_stability,
        );

        // Calculate overall score
        let valid_count = [
            validation.consciousness_valid,
            validation.ecosystem_stability_valid,
            validation.atmospheric_health_valid,
            validation.climate_stability_valid,
        ]
        .iter()
        .filter(|&&v| v)
        .count() as Float;

        validation.overall_score = valid_count / 4.0;
        validation.passed = validation.overall_score >= 0.6;
        validation.metrics = metrics;

        validation
    }

    /// Validate total emergence across all systems
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "True Emergence: Validated by expert panel"
    pub fn validate_total(&self, state: &SimulationState) -> ValidationResult {
        let mut result = ValidationResult::default();
        let mut details = HashMap::new();

        // Validate each system
        result.biological = self.validate_biological(&state.emergence.biological);
        result.noospheric = self.validate_noospheric(&state.emergence.noospheric);
        result.gaia = self.validate_gaia(&state.emergence.gaia);

        // Calculate overall confidence
        let scores = [result.biological.overall_score,
            result.noospheric.overall_score,
            result.gaia.overall_score];
        result.confidence = scores.iter().sum::<Float>() / scores.len() as Float;

        // Overall validation passes if all systems pass
        result.passed = result.biological.passed && result.noospheric.passed && result.gaia.passed;

        // Add details
        details.insert(
            "biological_passed".to_string(),
            result.biological.passed.to_string(),
        );
        details.insert(
            "biological_score".to_string(),
            format!("{:.2}", result.biological.overall_score),
        );
        details.insert(
            "noospheric_passed".to_string(),
            result.noospheric.passed.to_string(),
        );
        details.insert(
            "noospheric_score".to_string(),
            format!("{:.2}", result.noospheric.overall_score),
        );
        details.insert("gaia_passed".to_string(), result.gaia.passed.to_string());
        details.insert(
            "gaia_score".to_string(),
            format!("{:.2}", result.gaia.overall_score),
        );
        details.insert(
            "overall_confidence".to_string(),
            format!("{:.2}", result.confidence),
        );
        details.insert(
            "biological_score".to_string(),
            format!("{:.2}", result.biological.overall_score),
        );
        details.insert(
            "noospheric_passed".to_string(),
            result.noospheric.passed.to_string(),
        );
        details.insert(
            "noospheric_score".to_string(),
            format!("{:.2}", result.noospheric.overall_score),
        );
        details.insert(
            "noospheric_score".to_string(),
            format!("{:.2}", result.noospheric.overall_score),
        );
        details.insert("gaia_passed".to_string(), result.gaia.passed.to_string());
        details.insert(
            "gaia_score".to_string(),
            format!("{:.2}", result.gaia.overall_score),
        );
        details.insert(
            "overall_confidence".to_string(),
            format!("{:.2}", result.confidence),
        );
        details.insert(
            "biological_score".to_string(),
            format!("{:.2}", result.biological.overall_score),
        );
        details.insert(
            "noospheric_passed".to_string(),
            result.noospheric.passed.to_string(),
        );
        details.insert(
            "noospheric_score".to_string(),
            format!("{:.2}", result.noospheric.overall_score),
        );
        details.insert("gaia_passed".to_string(), result.gaia.passed.to_string());
        details.insert(
            "gaia_score".to_string(),
            format!("{:.2}", result.gaia.overall_score),
        );
        details.insert(
            "overall_confidence".to_string(),
            format!("{:.2}", result.confidence),
        );

        result.details = details;

        result
    }

    /// Check if emergence is true emergence
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "True Emergence: Validated by expert panel"
    pub fn is_true_emergence(&self, state: &SimulationState) -> bool {
        let validation = self.validate_total(state);
        validation.passed && validation.confidence >= 0.8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integrated_system::{
        BiologicalEmergence, EmergenceState, GaiaEmergence, NoosphericEmergence, SimulationState,
    };

    #[test]
    fn test_emergence_validator_creation() {
        let validator = EmergenceValidator::new();
        assert_eq!(validator.biological_criteria.min_genetic_diversity, 2.0);
        assert_eq!(validator.noospheric_criteria.min_cultural_diversity, 1.5);
        assert_eq!(validator.gaia_criteria.min_ecosystem_stability, 0.8);
    }

    #[test]
    fn test_validate_biological_valid() {
        let validator = EmergenceValidator::new();
        let emergence = BiologicalEmergence {
            cell_count: 1000,
            species_count: 50,
            ecosystem_count: 5,
            genetic_diversity: 2.5,
            mutation_rate: 0.001,
        };

        let result = validator.validate_biological(&emergence);
        assert!(result.passed);
        assert!(result.cells_valid);
        assert!(result.species_valid);
        assert!(result.genetic_diversity_valid);
    }

    #[test]
    fn test_validate_biological_invalid() {
        let validator = EmergenceValidator::new();
        let emergence = BiologicalEmergence {
            cell_count: 10,
            species_count: 1,
            ecosystem_count: 0,
            genetic_diversity: 1.0,
            mutation_rate: 0.1,
        };

        let result = validator.validate_biological(&emergence);
        assert!(!result.passed);
    }

    #[test]
    fn test_validate_noospheric_valid() {
        let validator = EmergenceValidator::new();
        let emergence = NoosphericEmergence {
            social_complexes_count: 10,
            collective_intelligence: 0.9,
            cultural_diversity: 2.0,
            ideological_diversity: 1.5,
        };

        let result = validator.validate_noospheric(&emergence);
        assert!(result.passed);
        assert!(result.social_complexes_valid);
        assert!(result.collective_intelligence_valid);
        assert!(result.cultural_diversity_valid);
    }

    #[test]
    fn test_validate_noospheric_invalid() {
        let validator = EmergenceValidator::new();
        let emergence = NoosphericEmergence {
            social_complexes_count: 1,
            collective_intelligence: 0.5,
            cultural_diversity: 0.5,
            ideological_diversity: 0.3,
        };

        let result = validator.validate_noospheric(&emergence);
        assert!(!result.passed);
    }

    #[test]
    fn test_validate_gaia_valid() {
        let validator = EmergenceValidator::new();
        let emergence = GaiaEmergence {
            consciousness_score: 0.8,
            ecosystem_stability: 0.9,
            atmospheric_health: 0.8,
            climate_stability: 0.85,
        };

        let result = validator.validate_gaia(&emergence);
        assert!(result.passed);
        assert!(result.consciousness_valid);
        assert!(result.ecosystem_stability_valid);
        assert!(result.atmospheric_health_valid);
    }

    #[test]
    fn test_validate_gaia_invalid() {
        let validator = EmergenceValidator::new();
        let emergence = GaiaEmergence {
            consciousness_score: 0.3,
            ecosystem_stability: 0.5,
            atmospheric_health: 0.4,
            climate_stability: 0.5,
        };

        let result = validator.validate_gaia(&emergence);
        assert!(!result.passed);
    }

    #[test]
    fn test_validate_total_valid() {
        let validator = EmergenceValidator::new();
        let state = SimulationState {
            step: 1000,
            entity_count: 10000,
            coherence: 0.95,
            energy_balance: 0.98,
            simulation_time: 1000.0,
            emergence: EmergenceState {
                biological: BiologicalEmergence {
                    cell_count: 1000,
                    species_count: 50,
                    ecosystem_count: 5,
                    genetic_diversity: 2.5,
                    mutation_rate: 0.001,
                },
                noospheric: NoosphericEmergence {
                    social_complexes_count: 10,
                    collective_intelligence: 0.9,
                    cultural_diversity: 2.0,
                    ideological_diversity: 1.5,
                },
                gaia: GaiaEmergence {
                    consciousness_score: 0.8,
                    ecosystem_stability: 0.9,
                    atmospheric_health: 0.8,
                    climate_stability: 0.85,
                },
            },
        };

        let result = validator.validate_total(&state);
        assert!(result.passed);
        assert!(result.confidence >= 0.8);
    }

    #[test]
    fn test_validate_total_invalid() {
        let validator = EmergenceValidator::new();
        let state = SimulationState {
            step: 10,
            entity_count: 10,
            coherence: 0.5,
            energy_balance: 0.5,
            simulation_time: 10.0,
            emergence: EmergenceState {
                biological: BiologicalEmergence {
                    cell_count: 10,
                    species_count: 1,
                    ecosystem_count: 0,
                    genetic_diversity: 1.0,
                    mutation_rate: 0.1,
                },
                noospheric: NoosphericEmergence {
                    social_complexes_count: 0,
                    collective_intelligence: 0.0,
                    cultural_diversity: 0.0,
                    ideological_diversity: 0.0,
                },
                gaia: GaiaEmergence {
                    consciousness_score: 0.0,
                    ecosystem_stability: 0.0,
                    atmospheric_health: 0.0,
                    climate_stability: 0.0,
                },
            },
        };

        let result = validator.validate_total(&state);
        assert!(!result.passed);
    }

    #[test]
    fn test_is_true_emergence() {
        let validator = EmergenceValidator::new();
        let state = SimulationState {
            step: 1000,
            entity_count: 10000,
            coherence: 0.95,
            energy_balance: 0.98,
            simulation_time: 1000.0,
            emergence: EmergenceState {
                biological: BiologicalEmergence {
                    cell_count: 1000,
                    species_count: 50,
                    ecosystem_count: 5,
                    genetic_diversity: 2.5,
                    mutation_rate: 0.001,
                },
                noospheric: NoosphericEmergence {
                    social_complexes_count: 10,
                    collective_intelligence: 0.9,
                    cultural_diversity: 2.0,
                    ideological_diversity: 1.5,
                },
                gaia: GaiaEmergence {
                    consciousness_score: 0.8,
                    ecosystem_stability: 0.9,
                    atmospheric_health: 0.8,
                    climate_stability: 0.85,
                },
            },
        };

        assert!(validator.is_true_emergence(&state));
    }

    #[test]
    fn test_is_true_emergence_false() {
        let validator = EmergenceValidator::new();
        let state = SimulationState {
            step: 10,
            entity_count: 10,
            coherence: 0.5,
            energy_balance: 0.5,
            simulation_time: 10.0,
            emergence: EmergenceState {
                biological: BiologicalEmergence {
                    cell_count: 10,
                    species_count: 1,
                    ecosystem_count: 0,
                    genetic_diversity: 1.0,
                    mutation_rate: 0.1,
                },
                noospheric: NoosphericEmergence {
                    social_complexes_count: 0,
                    collective_intelligence: 0.0,
                    cultural_diversity: 0.0,
                    ideological_diversity: 0.0,
                },
                gaia: GaiaEmergence {
                    consciousness_score: 0.0,
                    ecosystem_stability: 0.0,
                    atmospheric_health: 0.0,
                    climate_stability: 0.0,
                },
            },
        };

        assert!(!validator.is_true_emergence(&state));
    }

    #[test]
    fn test_custom_criteria() {
        let biological = BiologicalCriteria {
            min_cell_count: 50,
            min_species_count: 5,
            min_ecosystem_count: 1,
            min_genetic_diversity: 1.5,
            max_mutation_rate: 0.02,
            min_protein_diversity: 0.3,
        };

        let validator = EmergenceValidator::with_criteria(
            biological,
            NoosphericCriteria::default(),
            GaiaCriteria::default(),
        );
        assert_eq!(validator.biological_criteria.min_cell_count, 50);
        assert_eq!(validator.biological_criteria.min_genetic_diversity, 1.5);
    }
}
