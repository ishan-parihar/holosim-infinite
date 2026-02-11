// Individual Variation Statistics (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 5:
// "Add individual variation statistics tracking"
//
// This module implements statistics tracking for individual variation:
// 1. Evolutionary rate distribution
// 2. Karmic pattern statistics
// 3. Catalyst event statistics
// 4. Holographic interaction statistics

use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Individual Variation Statistics
///
/// Tracks statistics about individual variation across entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualVariationStatistics {
    /// Evolutionary rate distribution (0.5x to 1.5x)
    pub evolutionary_rate_distribution: HashMap<String, usize>,

    /// Average evolutionary rate
    pub average_evolutionary_rate: Float,

    /// Minimum evolutionary rate
    pub min_evolutionary_rate: Float,

    /// Maximum evolutionary rate
    pub max_evolutionary_rate: Float,

    /// Karmic pattern statistics
    pub karmic_patterns: KarmicPatternStatistics,

    /// Catalyst event statistics
    pub catalyst_events: CatalystEventStatistics,

    /// Holographic interaction statistics
    pub holographic_interactions: HolographicInteractionStatistics,

    /// Unique entity trajectories count
    pub unique_trajectories: usize,

    /// Polarity diversity (how diverse the polarization choices are)
    pub polarization_diversity: Float,
}

/// Karmic Pattern Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmicPatternStatistics {
    /// Total karmic patterns
    pub total_patterns: usize,

    /// Patterns by resolution status
    pub by_resolution_status: HashMap<String, usize>,

    /// Average karmic pattern intensity
    pub average_intensity: Float,

    /// Average number of patterns per entity
    pub average_patterns_per_entity: Float,
}

/// Catalyst Event Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalystEventStatistics {
    /// Total catalyst events
    pub total_events: usize,

    /// Events by catalyst type
    pub by_catalyst_type: HashMap<String, usize>,

    /// STO vs STS choice distribution
    pub polarity_choices: PolarityChoiceDistribution,

    /// Average catalyst intensity
    pub average_intensity: Float,

    /// Total learning value gained
    pub total_learning_value: Float,

    /// Total consciousness expansion
    pub total_consciousness_expansion: Float,

    /// Catalyst effectiveness (learning per intensity)
    pub catalyst_effectiveness: Float,
}

/// Polarity Choice Distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolarityChoiceDistribution {
    /// Number of STO (Service-to-Others) choices
    pub sto_choices: usize,

    /// Number of STS (Service-to-Self) choices
    pub sts_choices: usize,

    /// Number of unpolarized choices
    pub unpolarized_choices: usize,

    /// Percentage of STO choices
    pub sto_percentage: Float,

    /// Percentage of STS choices
    pub sts_percentage: Float,

    /// Percentage of unpolarized choices
    pub unpolarized_percentage: Float,
}

/// Holographic Interaction Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolographicInteractionStatistics {
    /// Total holographic connections
    pub total_connections: usize,

    /// Connections by type
    pub by_connection_type: HashMap<String, usize>,

    /// Average connection strength
    pub average_connection_strength: Float,

    /// Number of resonant connections
    pub resonant_connections: usize,

    /// Number of entangled connections
    pub entangled_connections: usize,

    /// Number of harmonic connections
    pub harmonic_connections: usize,

    /// Global phase coherence
    pub global_phase_coherence: Float,

    /// Number of resonance clusters
    pub resonance_cluster_count: usize,

    /// Average cluster size
    pub average_cluster_size: Float,
}

impl Default for IndividualVariationStatistics {
    fn default() -> Self {
        IndividualVariationStatistics {
            evolutionary_rate_distribution: HashMap::new(),
            average_evolutionary_rate: 1.0,
            min_evolutionary_rate: 1.0,
            max_evolutionary_rate: 1.0,
            karmic_patterns: KarmicPatternStatistics::default(),
            catalyst_events: CatalystEventStatistics::default(),
            holographic_interactions: HolographicInteractionStatistics::default(),
            unique_trajectories: 0,
            polarization_diversity: 0.0,
        }
    }
}

impl Default for KarmicPatternStatistics {
    fn default() -> Self {
        KarmicPatternStatistics {
            total_patterns: 0,
            by_resolution_status: HashMap::new(),
            average_intensity: 0.0,
            average_patterns_per_entity: 0.0,
        }
    }
}

impl Default for CatalystEventStatistics {
    fn default() -> Self {
        CatalystEventStatistics {
            total_events: 0,
            by_catalyst_type: HashMap::new(),
            polarity_choices: PolarityChoiceDistribution::default(),
            average_intensity: 0.0,
            total_learning_value: 0.0,
            total_consciousness_expansion: 0.0,
            catalyst_effectiveness: 0.0,
        }
    }
}

impl Default for PolarityChoiceDistribution {
    fn default() -> Self {
        PolarityChoiceDistribution {
            sto_choices: 0,
            sts_choices: 0,
            unpolarized_choices: 0,
            sto_percentage: 0.0,
            sts_percentage: 0.0,
            unpolarized_percentage: 0.0,
        }
    }
}

impl Default for HolographicInteractionStatistics {
    fn default() -> Self {
        HolographicInteractionStatistics {
            total_connections: 0,
            by_connection_type: HashMap::new(),
            average_connection_strength: 0.0,
            resonant_connections: 0,
            entangled_connections: 0,
            harmonic_connections: 0,
            global_phase_coherence: 0.0,
            resonance_cluster_count: 0,
            average_cluster_size: 0.0,
        }
    }
}

impl PolarityChoiceDistribution {
    /// Update percentages based on counts
    pub fn update_percentages(&mut self) {
        let total = self.sto_choices + self.sts_choices + self.unpolarized_choices;

        if total > 0 {
            self.sto_percentage = self.sto_choices as Float / total as Float * 100.0;
            self.sts_percentage = self.sts_choices as Float / total as Float * 100.0;
            self.unpolarized_percentage =
                self.unpolarized_choices as Float / total as Float * 100.0;
        }
    }

    /// Calculate polarization diversity
    ///
    /// Returns a value from 0.0 (no diversity, all unpolarized or all same polarity)
    /// to 1.0 (maximum diversity, equal distribution across all three)
    pub fn calculate_diversity(&self) -> Float {
        let total = self.sto_choices + self.sts_choices + self.unpolarized_choices;

        if total == 0 {
            return 0.0;
        }

        let sto_ratio = self.sto_choices as Float / total as Float;
        let sts_ratio = self.sts_choices as Float / total as Float;
        let unpolarized_ratio = self.unpolarized_choices as Float / total as Float;

        // Shannon entropy
        let mut entropy = 0.0;
        for ratio in [sto_ratio, sts_ratio, unpolarized_ratio] {
            if ratio > 0.0 {
                entropy -= ratio * ratio.log2();
            }
        }

        // Normalize to 0-1 range (max entropy is log2(3) ≈ 1.585)
        entropy / 3.0_f64.log2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polarity_choice_distribution() {
        let mut distribution = PolarityChoiceDistribution {
            sto_choices: 30,
            sts_choices: 20,
            unpolarized_choices: 50,
            ..Default::default()
        };

        distribution.update_percentages();

        assert_eq!(distribution.sto_percentage, 30.0);
        assert_eq!(distribution.sts_percentage, 20.0);
        assert_eq!(distribution.unpolarized_percentage, 50.0);
    }

    #[test]
    fn test_polarization_diversity() {
        let distribution = PolarityChoiceDistribution {
            sto_choices: 33,
            sts_choices: 33,
            unpolarized_choices: 34,
            ..Default::default()
        };

        let diversity = distribution.calculate_diversity();

        // High diversity (near equal distribution)
        assert!(diversity > 0.9);
    }

    #[test]
    fn test_polarization_diversity_no_diversity() {
        let distribution = PolarityChoiceDistribution {
            sto_choices: 100,
            sts_choices: 0,
            unpolarized_choices: 0,
            ..Default::default()
        };

        let diversity = distribution.calculate_diversity();

        // No diversity (all same choice)
        assert_eq!(diversity, 0.0);
    }
}
