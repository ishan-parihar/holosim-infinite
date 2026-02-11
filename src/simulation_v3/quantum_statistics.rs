// Quantum Statistics - Phase 7: Consciousness-First Cosmology
//
// This module implements statistics tracking for quantum information structures,
// consciousness-to-matter transitions, and attractor fields.
//
// Key Statistics:
// - Quantum energy pool statistics
// - Decoherence statistics
// - Attractor field statistics
// - Consciousness-to-matter transition statistics
// - Quantum information metrics

use crate::physical_manifestation::consciousness_to_matter::{
    AttractorField, ConsciousnessToMatterManager, QuantumEnergyPool,
};
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// QUANTUM POOL STATISTICS
// ============================================================================

/// Statistics for quantum energy pools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumPoolStatistics {
    /// Total number of pools
    pub total_pools: usize,

    /// Number of coherent pools
    pub coherent_pools: usize,

    /// Number of decohered pools
    pub decohered_pools: usize,

    /// Total information content (bits)
    pub total_information_content: Float,

    /// Average information content (bits)
    pub average_information_content: Float,

    /// Total energy (Joules)
    pub total_energy: Float,

    /// Average coherence level
    pub average_coherence: Float,

    /// Coherence level distribution
    pub coherence_distribution: HashMap<String, usize>,

    /// Information content distribution
    pub information_distribution: HashMap<String, usize>,

    /// Total entanglement links
    pub total_entanglement_links: usize,

    /// Average entanglement per pool
    pub average_entanglement_per_pool: Float,
}

impl QuantumPoolStatistics {
    /// Create quantum pool statistics from manager
    pub fn from_manager(manager: &ConsciousnessToMatterManager) -> Self {
        let pools: Vec<&QuantumEnergyPool> = manager.quantum_pools.values().collect();

        let total_pools = pools.len();
        let coherent_pools = pools.iter().filter(|p| !p.is_decohered()).count();
        let decohered_pools = pools.iter().filter(|p| p.is_decohered()).count();

        let total_information_content: Float = pools.iter().map(|p| p.information_content).sum();
        let average_information_content = if total_pools > 0 {
            total_information_content / total_pools as Float
        } else {
            0.0
        };

        let total_energy: Float = pools.iter().map(|p| p.energy).sum();

        let average_coherence = if total_pools > 0 {
            pools.iter().map(|p| p.coherence_level).sum::<Float>() / total_pools as Float
        } else {
            0.0
        };

        // Coherence distribution
        let mut coherence_distribution = HashMap::new();
        for pool in &pools {
            let range = Self::coherence_range(pool.coherence_level);
            *coherence_distribution.entry(range).or_insert(0) += 1;
        }

        // Information distribution
        let mut information_distribution = HashMap::new();
        for pool in &pools {
            let range = Self::information_range(pool.information_content);
            *information_distribution.entry(range).or_insert(0) += 1;
        }

        // Entanglement statistics
        let total_entanglement_links: usize =
            pools.iter().map(|p| p.entanglement_links.len()).sum();
        let average_entanglement_per_pool = if total_pools > 0 {
            total_entanglement_links as Float / total_pools as Float
        } else {
            0.0
        };

        Self {
            total_pools,
            coherent_pools,
            decohered_pools,
            total_information_content,
            average_information_content,
            total_energy,
            average_coherence,
            coherence_distribution,
            information_distribution,
            total_entanglement_links,
            average_entanglement_per_pool,
        }
    }

    /// Get coherence range label
    fn coherence_range(coherence: Float) -> String {
        if coherence >= 0.9 {
            "Very High (0.9-1.0)".to_string()
        } else if coherence >= 0.7 {
            "High (0.7-0.9)".to_string()
        } else if coherence >= 0.5 {
            "Medium (0.5-0.7)".to_string()
        } else if coherence >= 0.3 {
            "Low (0.3-0.5)".to_string()
        } else {
            "Very Low (0.0-0.3)".to_string()
        }
    }

    /// Get information range label
    fn information_range(information: Float) -> String {
        if information >= 1000.0 {
            "Very High (>=1000 bits)".to_string()
        } else if information >= 500.0 {
            "High (500-1000 bits)".to_string()
        } else if information >= 100.0 {
            "Medium (100-500 bits)".to_string()
        } else if information >= 10.0 {
            "Low (10-100 bits)".to_string()
        } else {
            "Very Low (0-10 bits)".to_string()
        }
    }
}

// ============================================================================
// DECOHERENCE STATISTICS
// ============================================================================

/// Statistics for quantum decoherence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoherenceStatistics {
    /// Total collapse attempts
    pub total_collapse_attempts: usize,

    /// Successful collapses
    pub successful_collapses: usize,

    /// Failed collapses
    pub failed_collapses: usize,

    /// Collapse success rate
    pub collapse_success_rate: Float,

    /// Average collapse probability
    pub average_collapse_probability: Float,

    /// Average decoherence time (seconds)
    pub average_decoherence_time: Float,

    /// Decoherence time distribution
    pub decoherence_time_distribution: HashMap<String, usize>,

    /// Coherence loss rate (per second)
    pub coherence_loss_rate: Float,
}

impl DecoherenceStatistics {
    /// Create decoherence statistics from manager
    pub fn from_manager(manager: &ConsciousnessToMatterManager) -> Self {
        let pools: Vec<&QuantumEnergyPool> = manager.quantum_pools.values().collect();

        let total_collapse_attempts = pools.len();
        let successful_collapses = pools.iter().filter(|p| p.is_decohered()).count();
        let failed_collapses = total_collapse_attempts - successful_collapses;

        let collapse_success_rate = if total_collapse_attempts > 0 {
            successful_collapses as Float / total_collapse_attempts as Float
        } else {
            0.0
        };

        let average_collapse_probability = if !pools.is_empty() {
            pools.iter().map(|p| p.collapse_probability).sum::<Float>() / pools.len() as Float
        } else {
            0.0
        };

        // Simplified decoherence time calculation
        let average_decoherence_time = 10.0; // Placeholder

        let mut decoherence_time_distribution = HashMap::new();
        decoherence_time_distribution.insert("0-10s".to_string(), successful_collapses / 3);
        decoherence_time_distribution.insert("10-100s".to_string(), successful_collapses / 3);
        decoherence_time_distribution.insert(">100s".to_string(), successful_collapses / 3);

        // Coherence loss rate
        let coherence_loss_rate = manager.decoherence_rate;

        Self {
            total_collapse_attempts,
            successful_collapses,
            failed_collapses,
            collapse_success_rate,
            average_collapse_probability,
            average_decoherence_time,
            decoherence_time_distribution,
            coherence_loss_rate,
        }
    }
}

// ============================================================================
// ATTRACTOR FIELD STATISTICS
// ============================================================================

/// Statistics for attractor fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractorFieldStatistics {
    /// Total number of attractor fields
    pub total_attractor_fields: usize,

    /// Number of strong attractor fields (strength >= 0.8)
    pub strong_attractor_fields: usize,

    /// Number of medium attractor fields (strength 0.5-0.8)
    pub medium_attractor_fields: usize,

    /// Number of weak attractor fields (strength < 0.5)
    pub weak_attractor_fields: usize,

    /// Average attractor strength
    pub average_attractor_strength: Float,

    /// Average stability
    pub average_stability: Float,

    /// Attractor strength distribution
    pub strength_distribution: HashMap<String, usize>,

    /// Element type distribution (by period)
    pub element_period_distribution: HashMap<String, usize>,

    /// Total energy well depth (Joules)
    pub total_energy_well_depth: Float,

    /// Average energy well depth (Joules)
    pub average_energy_well_depth: Float,
}

impl AttractorFieldStatistics {
    /// Create attractor field statistics from manager
    pub fn from_manager(manager: &ConsciousnessToMatterManager) -> Self {
        let fields: Vec<&AttractorField> = manager.attractor_fields.all_fields().values().collect();

        let total_attractor_fields = fields.len();
        let strong_attractor_fields = fields
            .iter()
            .filter(|f| f.attractor_strength >= 0.8)
            .count();
        let medium_attractor_fields = fields
            .iter()
            .filter(|f| f.attractor_strength >= 0.5 && f.attractor_strength < 0.8)
            .count();
        let weak_attractor_fields = fields.iter().filter(|f| f.attractor_strength < 0.5).count();

        let average_attractor_strength = if !fields.is_empty() {
            fields.iter().map(|f| f.attractor_strength).sum::<Float>() / fields.len() as Float
        } else {
            0.0
        };

        let average_stability = if !fields.is_empty() {
            fields.iter().map(|f| f.stability).sum::<Float>() / fields.len() as Float
        } else {
            0.0
        };

        // Strength distribution
        let mut strength_distribution = HashMap::new();
        for field in &fields {
            let range = Self::strength_range(field.attractor_strength);
            *strength_distribution.entry(range).or_insert(0) += 1;
        }

        // Element period distribution
        let mut element_period_distribution = HashMap::new();
        for field in &fields {
            let period = Self::element_period(field.element_type);
            *element_period_distribution.entry(period).or_insert(0) += 1;
        }

        let total_energy_well_depth: Float = fields.iter().map(|f| f.energy_well_depth).sum();
        let average_energy_well_depth = if !fields.is_empty() {
            total_energy_well_depth / fields.len() as Float
        } else {
            0.0
        };

        Self {
            total_attractor_fields,
            strong_attractor_fields,
            medium_attractor_fields,
            weak_attractor_fields,
            average_attractor_strength,
            average_stability,
            strength_distribution,
            element_period_distribution,
            total_energy_well_depth,
            average_energy_well_depth,
        }
    }

    /// Get strength range label
    fn strength_range(strength: Float) -> String {
        if strength >= 0.8 {
            "Strong (0.8-1.0)".to_string()
        } else if strength >= 0.5 {
            "Medium (0.5-0.8)".to_string()
        } else {
            "Weak (0.0-0.5)".to_string()
        }
    }

    /// Get element period
    fn element_period(atomic_number: u32) -> String {
        match atomic_number {
            1..=2 => "Period 1".to_string(),
            3..=10 => "Period 2".to_string(),
            11..=18 => "Period 3".to_string(),
            19..=36 => "Period 4".to_string(),
            37..=54 => "Period 5".to_string(),
            55..=86 => "Period 6".to_string(),
            87..=118 => "Period 7".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

// ============================================================================
// TRANSITION STATISTICS
// ============================================================================

/// Statistics for consciousness-to-matter transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStatistics {
    /// Total transitions
    pub total_transitions: usize,

    /// Active transitions
    pub active_transitions: usize,

    /// Completed transitions
    pub completed_transitions: usize,

    /// Failed transitions
    pub failed_transitions: usize,

    /// Transition success rate
    pub transition_success_rate: Float,

    /// Average transition time (seconds)
    pub average_transition_time: Float,

    /// Transition time distribution
    pub transition_time_distribution: HashMap<String, usize>,

    /// Transition state distribution
    pub state_distribution: HashMap<String, usize>,

    /// Average decoherence level at completion
    pub average_decoherence_level: Float,

    /// Attractor field match rate
    pub attractor_match_rate: Float,
}

impl TransitionStatistics {
    /// Create transition statistics from manager
    pub fn from_manager(manager: &ConsciousnessToMatterManager) -> Self {
        let total_transitions = manager.active_transitions.len()
            + manager.completed_transitions.len()
            + manager.failed_transitions.len();

        let active_transitions = manager.active_transitions.len();
        let completed_transitions = manager.completed_transitions.len();
        let failed_transitions = manager.failed_transitions.len();

        let transition_success_rate = if total_transitions > 0 {
            completed_transitions as Float / total_transitions as Float
        } else {
            0.0
        };

        // Simplified transition time calculation
        let average_transition_time = 5.0; // Placeholder

        let mut transition_time_distribution = HashMap::new();
        transition_time_distribution.insert("0-5s".to_string(), completed_transitions / 3);
        transition_time_distribution.insert("5-10s".to_string(), completed_transitions / 3);
        transition_time_distribution.insert(">10s".to_string(), completed_transitions / 3);

        // State distribution
        let mut state_distribution = HashMap::new();
        for transition in &manager.active_transitions {
            let state = format!("{:?}", transition.transition_state);
            *state_distribution.entry(state).or_insert(0) += 1;
        }

        // Average decoherence level
        let all_transitions = manager
            .active_transitions
            .iter()
            .chain(manager.completed_transitions.iter())
            .chain(manager.failed_transitions.iter());
        let average_decoherence_level = if total_transitions > 0 {
            all_transitions.map(|t| t.decoherence_level).sum::<Float>() / total_transitions as Float
        } else {
            0.0
        };

        // Attractor match rate
        let attractor_match_rate = if completed_transitions > 0 {
            manager
                .completed_transitions
                .iter()
                .filter(|t| t.attractor_field.is_some())
                .count() as Float
                / completed_transitions as Float
        } else {
            0.0
        };

        Self {
            total_transitions,
            active_transitions,
            completed_transitions,
            failed_transitions,
            transition_success_rate,
            average_transition_time,
            transition_time_distribution,
            state_distribution,
            average_decoherence_level,
            attractor_match_rate,
        }
    }
}

// ============================================================================
// CONSCIOUSNESS-TO-MATTER STATISTICS TRACKER
// ============================================================================

/// Comprehensive statistics tracker for consciousness-to-matter systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessToMatterStatisticsTracker {
    /// Quantum pool statistics
    pub quantum_pool_stats: QuantumPoolStatistics,

    /// Decoherence statistics
    pub decoherence_stats: DecoherenceStatistics,

    /// Attractor field statistics
    pub attractor_field_stats: AttractorFieldStatistics,

    /// Transition statistics
    pub transition_stats: TransitionStatistics,

    /// Historical data
    pub historical_data: Vec<HistoricalQuantumData>,

    /// Maximum historical data points
    pub max_history: usize,
}

/// Historical quantum data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalQuantumData {
    /// Timestamp
    pub timestamp: u64,

    /// Coherent pools
    pub coherent_pools: usize,

    /// Decohered pools
    pub decohered_pools: usize,

    /// Active transitions
    pub active_transitions: usize,

    /// Completed transitions
    pub completed_transitions: usize,

    /// Average coherence
    pub average_coherence: Float,

    /// Total information content
    pub total_information_content: Float,
}

impl ConsciousnessToMatterStatisticsTracker {
    /// Create a new statistics tracker from manager
    pub fn from_manager(manager: &ConsciousnessToMatterManager) -> Self {
        Self {
            quantum_pool_stats: QuantumPoolStatistics::from_manager(manager),
            decoherence_stats: DecoherenceStatistics::from_manager(manager),
            attractor_field_stats: AttractorFieldStatistics::from_manager(manager),
            transition_stats: TransitionStatistics::from_manager(manager),
            historical_data: Vec::new(),
            max_history: 1000,
        }
    }

    /// Update statistics from manager
    pub fn update(&mut self, manager: &ConsciousnessToMatterManager, timestamp: u64) {
        self.quantum_pool_stats = QuantumPoolStatistics::from_manager(manager);
        self.decoherence_stats = DecoherenceStatistics::from_manager(manager);
        self.attractor_field_stats = AttractorFieldStatistics::from_manager(manager);
        self.transition_stats = TransitionStatistics::from_manager(manager);

        // Add historical data point
        let historical_point = HistoricalQuantumData {
            timestamp,
            coherent_pools: self.quantum_pool_stats.coherent_pools,
            decohered_pools: self.quantum_pool_stats.decohered_pools,
            active_transitions: self.transition_stats.active_transitions,
            completed_transitions: self.transition_stats.completed_transitions,
            average_coherence: self.quantum_pool_stats.average_coherence,
            total_information_content: self.quantum_pool_stats.total_information_content,
        };

        self.historical_data.push(historical_point);

        // Limit history size
        if self.historical_data.len() > self.max_history {
            self.historical_data.remove(0);
        }
    }

    /// Get historical data
    pub fn get_historical_data(&self) -> &[HistoricalQuantumData] {
        &self.historical_data
    }

    /// Get summary statistics
    ///
    /// # Returns
    /// Summary statistics as a formatted string
    pub fn get_summary(&self) -> String {
        format!(
            "=== Consciousness-to-Matter Statistics ===\n\
             Quantum Pools: {} ({} coherent, {} decohered)\n\
             Total Information: {:.2} bits\n\
             Average Coherence: {:.3}\n\
             \n\
             Decoherence: {:.1}% success rate\n\
             \n\
             Attractor Fields: {} ({} strong, {} medium, {} weak)\n\
             \n\
             Transitions: {} ({} active, {} completed, {} failed)\n\
             Success Rate: {:.1}%",
            self.quantum_pool_stats.total_pools,
            self.quantum_pool_stats.coherent_pools,
            self.quantum_pool_stats.decohered_pools,
            self.quantum_pool_stats.total_information_content,
            self.quantum_pool_stats.average_coherence,
            self.decoherence_stats.collapse_success_rate * 100.0,
            self.attractor_field_stats.total_attractor_fields,
            self.attractor_field_stats.strong_attractor_fields,
            self.attractor_field_stats.medium_attractor_fields,
            self.attractor_field_stats.weak_attractor_fields,
            self.transition_stats.total_transitions,
            self.transition_stats.active_transitions,
            self.transition_stats.completed_transitions,
            self.transition_stats.failed_transitions,
            self.transition_stats.transition_success_rate * 100.0
        )
    }
}

impl Default for ConsciousnessToMatterStatisticsTracker {
    fn default() -> Self {
        Self {
            quantum_pool_stats: QuantumPoolStatistics {
                total_pools: 0,
                coherent_pools: 0,
                decohered_pools: 0,
                total_information_content: 0.0,
                average_information_content: 0.0,
                total_energy: 0.0,
                average_coherence: 0.0,
                coherence_distribution: HashMap::new(),
                information_distribution: HashMap::new(),
                total_entanglement_links: 0,
                average_entanglement_per_pool: 0.0,
            },
            decoherence_stats: DecoherenceStatistics {
                total_collapse_attempts: 0,
                successful_collapses: 0,
                failed_collapses: 0,
                collapse_success_rate: 0.0,
                average_collapse_probability: 0.0,
                average_decoherence_time: 0.0,
                decoherence_time_distribution: HashMap::new(),
                coherence_loss_rate: 0.0,
            },
            attractor_field_stats: AttractorFieldStatistics {
                total_attractor_fields: 0,
                strong_attractor_fields: 0,
                medium_attractor_fields: 0,
                weak_attractor_fields: 0,
                average_attractor_strength: 0.0,
                average_stability: 0.0,
                strength_distribution: HashMap::new(),
                element_period_distribution: HashMap::new(),
                total_energy_well_depth: 0.0,
                average_energy_well_depth: 0.0,
            },
            transition_stats: TransitionStatistics {
                total_transitions: 0,
                active_transitions: 0,
                completed_transitions: 0,
                failed_transitions: 0,
                transition_success_rate: 0.0,
                average_transition_time: 0.0,
                transition_time_distribution: HashMap::new(),
                state_distribution: HashMap::new(),
                average_decoherence_level: 0.0,
                attractor_match_rate: 0.0,
            },
            historical_data: Vec::new(),
            max_history: 1000,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_pool_statistics() {
        let manager = ConsciousnessToMatterManager::new();
        let stats = QuantumPoolStatistics::from_manager(&manager);

        assert_eq!(stats.total_pools, 0);
        assert_eq!(stats.coherent_pools, 0);
        assert_eq!(stats.decohered_pools, 0);
    }

    #[test]
    fn test_decoherence_statistics() {
        let manager = ConsciousnessToMatterManager::new();
        let stats = DecoherenceStatistics::from_manager(&manager);

        assert_eq!(stats.total_collapse_attempts, 0);
        assert_eq!(stats.successful_collapses, 0);
        assert_eq!(stats.failed_collapses, 0);
    }

    #[test]
    fn test_attractor_field_statistics() {
        let manager = ConsciousnessToMatterManager::new();
        let stats = AttractorFieldStatistics::from_manager(&manager);

        assert_eq!(stats.total_attractor_fields, 118);
    }

    #[test]
    fn test_transition_statistics() {
        let manager = ConsciousnessToMatterManager::new();
        let stats = TransitionStatistics::from_manager(&manager);

        assert_eq!(stats.total_transitions, 0);
    }

    #[test]
    fn test_consciousness_to_matter_statistics_tracker() {
        let manager = ConsciousnessToMatterManager::new();
        let mut tracker = ConsciousnessToMatterStatisticsTracker::from_manager(&manager);

        tracker.update(&manager, 100);

        assert_eq!(tracker.historical_data.len(), 1);
    }
}
