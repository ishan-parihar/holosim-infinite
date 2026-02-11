//! Archetype Processor (Dynamic Mechanism 2) - Simplified Version
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
//! "Process 22 archetypes through the mind matrix"
//!
//! This module implements archetype processing that:
//! - Activates and processes archetypes through the mind complex
//! - Calculates lambda values for archetype evolution
//! - Detects archetype pathologies
//! - Tracks archetype rung progression

use crate::archetypes::common::{
    DevelopmentalPosition, HealthStatus, LambdaMeasurement, LambdaMeasurementType,
};
use crate::types::{Float, Polarity, Rung};
use std::collections::HashMap;

/// Archetype Processor
///
/// Processes 22 archetypes through the mind matrix with dynamic activation,
/// lambda evolution, and pathology detection.
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Success Criteria:
/// "Archetype processing changes lambda values"
pub struct ArchetypeProcessor {
    /// Processing history
    processing_history: Vec<ArchetypeProcessingRecord>,

    /// Lambda evolution history
    lambda_history: HashMap<usize, Vec<LambdaMeasurement>>,
}

/// Archetype Processing Result
///
/// Result of processing archetypes for an entity.
#[derive(Debug, Clone)]
pub struct ArchetypeProcessingResult {
    /// Entity ID
    pub entity_id: usize,

    /// Processing timestamp
    pub timestamp: u64,

    /// Archetype activations (0.0 to 1.0 for each archetype)
    pub archetype_activations: Vec<Float>,

    /// Lambda values for each archetype
    pub lambda_values: Vec<LambdaMeasurement>,

    /// Pathologies detected
    pub pathologies: Vec<PathologicalIndicator>,

    /// Overall health status
    pub overall_health: HealthStatus,

    /// Evolution progress (0.0 to 1.0)
    pub evolution_progress: Float,
}

/// Archetype Processing Record
///
/// Records archetype processing events.
#[derive(Debug, Clone)]
pub struct ArchetypeProcessingRecord {
    /// Timestamp
    pub timestamp: u64,

    /// Entity ID
    pub entity_id: usize,

    /// Number of archetypes processed
    pub archetypes_processed: usize,

    /// Total lambda change
    pub total_lambda_change: Float,

    /// Pathologies detected
    pub pathologies_detected: usize,
}

/// Pathological Indicator
///
/// Indicates a pathological pattern in archetype processing.
#[derive(Debug, Clone)]
pub struct PathologicalIndicator {
    /// Archetype ID (1-22)
    pub archetype_id: u8,

    /// Pathology type
    pub pathology_type: PathologyType,

    /// Severity (0.0 to 1.0)
    pub severity: Float,

    /// Description
    pub description: String,
}

/// Pathology Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathologyType {
    /// Blockage (energy flow blocked)
    Blockage,

    /// Imbalance (too much or too little energy)
    Imbalance,

    /// Fragmentation (disconnected from whole)
    Fragmentation,

    /// Inversion (polarity reversed)
    Inversion,

    /// Stagnation (no evolution)
    Stagnation,
}

/// Archetype Processor State
///
/// Current state of the archetype processor.
#[derive(Debug, Clone)]
pub struct ArchetypeProcessorState {
    /// Total processing cycles
    pub total_cycles: usize,

    /// Average lambda change per cycle
    pub avg_lambda_change: Float,

    /// Total pathologies detected
    pub total_pathologies: usize,

    /// Archetypes processed
    pub archetypes_processed: HashMap<usize, usize>,
}

impl Default for ArchetypeProcessorState {
    fn default() -> Self {
        Self {
            total_cycles: 0,
            avg_lambda_change: 0.0,
            total_pathologies: 0,
            archetypes_processed: HashMap::new(),
        }
    }
}

impl ArchetypeProcessor {
    /// Create a new archetype processor
    pub fn new() -> Self {
        Self {
            processing_history: Vec::new(),
            lambda_history: HashMap::new(),
        }
    }

    /// Process archetypes for an entity
    ///
    /// This is the main entry point for archetype processing.
    /// It processes all archetypes and returns the result.
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Archetype processing changes lambda values"
    pub fn process_archetypes(
        &mut self,
        entity_id: usize,
        catalyst_intensity: Float,
        polarity_choice: Option<Polarity>,
    ) -> ArchetypeProcessingResult {
        let timestamp = 0; // TODO: Use actual timestamp

        // Process each archetype
        let mut archetype_activations = Vec::new();
        let mut lambda_values = Vec::new();
        let mut pathologies = Vec::new();

        // Process all 21 archetypes (A1-A21)
        for archetype_id in 1..=21 {
            let activation = self.calculate_archetype_activation(archetype_id, catalyst_intensity);
            archetype_activations.push(activation);

            let lambda = self.calculate_lambda(archetype_id, activation);
            lambda_values.push(lambda);

            // Detect pathologies
            self.detect_archetype_pathology(
                archetype_id,
                activation,
                &mut pathologies,
                polarity_choice,
            );
        }

        // Calculate overall health
        let overall_health = self.calculate_overall_health(&pathologies);

        // Calculate evolution progress
        let evolution_progress = self.calculate_evolution_progress(&lambda_values);

        // Record processing
        self.record_processing(entity_id, &lambda_values, &pathologies);

        ArchetypeProcessingResult {
            entity_id,
            timestamp,
            archetype_activations,
            lambda_values,
            pathologies,
            overall_health,
            evolution_progress,
        }
    }

    /// Calculate archetype activation
    fn calculate_archetype_activation(
        &self,
        archetype_id: usize,
        catalyst_intensity: Float,
    ) -> Float {
        // Base activation from catalyst intensity
        let base_activation = catalyst_intensity * 0.5;

        // Add archetype-specific factors
        let archetype_factor = match archetype_id {
            1 => 0.8, // Matrix: strong activation
            2 => 0.6, // Potentiator: moderate activation
            3 => 0.9, // Catalyst: strong activation with catalyst
            4 => 0.7, // Experience: moderate-high activation
            5 => 0.8, // Significator: strong activation
            6 => 0.6, // Transformation: moderate activation
            7 => 0.5, // Great Way: moderate activation
            _ => 0.5, // Other archetypes: moderate activation
        };

        (base_activation + archetype_factor * 0.5).min(1.0)
    }

    /// Calculate lambda for an archetype
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Archetype processing changes lambda values"
    fn calculate_lambda(&self, archetype_id: usize, activation: Float) -> LambdaMeasurement {
        // Lambda is related to archetype activation and development
        // Higher activation = higher lambda (more evolved)

        let base_lambda = 0.5;
        let activation_factor = activation * 0.5;

        // Archetype-specific lambda adjustments
        let archetype_factor = match archetype_id {
            1 => 0.1,  // Matrix: slow evolution
            2 => 0.15, // Potentiator: moderate evolution
            3 => 0.2,  // Catalyst: active evolution
            4 => 0.15, // Experience: moderate evolution
            5 => 0.2,  // Significator: active evolution
            6 => 0.25, // Transformation: fast evolution
            7 => 0.1,  // Great Way: slow evolution
            _ => 0.15,
        };

        let lambda_value = base_lambda + activation_factor + archetype_factor;

        LambdaMeasurement::new(lambda_value.min(1.0), LambdaMeasurementType::Activation)
    }

    /// Detect pathology in a single archetype
    fn detect_archetype_pathology(
        &self,
        archetype_id: usize,
        activation: Float,
        pathologies: &mut Vec<PathologicalIndicator>,
        polarity_choice: Option<Polarity>,
    ) {
        // Check for blockage (very low activation)
        if activation < 0.2 {
            pathologies.push(PathologicalIndicator {
                archetype_id: archetype_id as u8,
                pathology_type: PathologyType::Blockage,
                severity: 1.0 - activation,
                description: format!(
                    "Archetype {} has low activation ({:.2})",
                    archetype_id, activation
                ),
            });
        }

        // Check for imbalance (very high activation)
        if activation > 0.9 {
            pathologies.push(PathologicalIndicator {
                archetype_id: archetype_id as u8,
                pathology_type: PathologyType::Imbalance,
                severity: activation - 0.8,
                description: format!(
                    "Archetype {} has excessive activation ({:.2})",
                    archetype_id, activation
                ),
            });
        }

        // Check for polarity inversion (if polarity choice is available)
        if let Some(polarity) = polarity_choice {
            if archetype_id == 6 && matches!(polarity, Polarity::ServiceToSelf) {
                // Transformation archetype with STS polarity
                pathologies.push(PathologicalIndicator {
                    archetype_id: archetype_id as u8,
                    pathology_type: PathologyType::Inversion,
                    severity: 0.7,
                    description: format!("Archetype {} shows polarity inversion", archetype_id),
                });
            }
        }
    }

    /// Calculate overall health status
    fn calculate_overall_health(&self, pathologies: &[PathologicalIndicator]) -> HealthStatus {
        if pathologies.is_empty() {
            return HealthStatus::Healthy;
        }

        let total_severity: Float = pathologies.iter().map(|p| p.severity).sum();
        let avg_severity = total_severity / pathologies.len() as Float;

        if avg_severity < 0.3 {
            HealthStatus::Healthy
        } else if avg_severity < 0.6 {
            HealthStatus::Balanced
        } else {
            HealthStatus::Pathological
        }
    }

    /// Calculate evolution progress
    fn calculate_evolution_progress(&self, lambda_values: &[LambdaMeasurement]) -> Float {
        if lambda_values.is_empty() {
            return 0.0;
        }

        let total_lambda: Float = lambda_values.iter().map(|l| l.value).sum();
        let avg_lambda = total_lambda / lambda_values.len() as Float;

        avg_lambda
    }

    /// Record archetype processing
    fn record_processing(
        &mut self,
        entity_id: usize,
        lambda_values: &[LambdaMeasurement],
        pathologies: &[PathologicalIndicator],
    ) {
        let total_lambda_change: Float = lambda_values.iter().map(|l| l.value).sum();

        let record = ArchetypeProcessingRecord {
            timestamp: 0, // TODO: Use actual timestamp
            entity_id,
            archetypes_processed: lambda_values.len(),
            total_lambda_change,
            pathologies_detected: pathologies.len(),
        };

        self.processing_history.push(record);

        // Update lambda history
        for (idx, lambda) in lambda_values.iter().enumerate() {
            let archetype_id = idx + 1;
            self.lambda_history
                .entry(archetype_id)
                .or_insert_with(Vec::new)
                .push(lambda.clone());
        }
    }

    /// Get processor state
    pub fn get_state(&self) -> ArchetypeProcessorState {
        let total_cycles = self.processing_history.len();

        let avg_lambda_change = if !self.processing_history.is_empty() {
            let total: Float = self
                .processing_history
                .iter()
                .map(|r| r.total_lambda_change)
                .sum();
            total / total_cycles as Float
        } else {
            0.0
        };

        let total_pathologies: usize = self
            .processing_history
            .iter()
            .map(|r| r.pathologies_detected)
            .sum();

        let mut archetypes_processed = HashMap::new();
        for record in &self.processing_history {
            *archetypes_processed.entry(record.entity_id).or_insert(0) +=
                record.archetypes_processed;
        }

        ArchetypeProcessorState {
            total_cycles,
            avg_lambda_change,
            total_pathologies,
            archetypes_processed,
        }
    }
}

impl Default for ArchetypeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_processor_creation() {
        let processor = ArchetypeProcessor::new();
        assert_eq!(processor.processing_history.len(), 0);
    }

    #[test]
    fn test_process_archetypes() {
        let mut processor = ArchetypeProcessor::new();

        let result = processor.process_archetypes(1, 0.5, Some(Polarity::ServiceToOthers));

        assert_eq!(result.entity_id, 1);
        assert_eq!(result.archetype_activations.len(), 21);
        assert_eq!(result.lambda_values.len(), 21);
    }

    #[test]
    fn test_lambda_calculation() {
        let processor = ArchetypeProcessor::new();

        let lambda = processor.calculate_lambda(1, 0.5);

        assert!(lambda.value > 0.0);
        assert!(lambda.value <= 1.0);
    }

    #[test]
    fn test_get_state() {
        let mut processor = ArchetypeProcessor::new();

        processor.process_archetypes(1, 0.5, Some(Polarity::ServiceToOthers));

        let state = processor.get_state();
        assert_eq!(state.total_cycles, 1);
    }
}
