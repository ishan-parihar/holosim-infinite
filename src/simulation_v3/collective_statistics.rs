// Collective Statistics Module (Phase 6)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 6:
// "Track collective metrics"
//
// This module implements detailed statistics tracking for collective entities.

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::collective_dynamics::{
    CollectiveBehavior, CollectiveEvolution, CollectiveFreeWill, GroupConsciousness,
    PolarityDistribution,
};
use crate::simulation_v3::holographic_field::HolographicFieldManager;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// DETAILED COLLECTIVE STATISTICS
// ============================================================================

/// Detailed collective statistics
#[derive(Debug, Clone, Default)]
pub struct DetailedCollectiveStatistics {
    /// Basic statistics
    pub basic: CollectiveDynamicsStatistics,

    /// Collective behavior statistics
    pub behavior: CollectiveBehaviorStatistics,

    /// Group consciousness statistics
    pub consciousness: GroupConsciousnessStatistics,

    /// Collective evolution statistics
    pub evolution: CollectiveEvolutionStatistics,

    /// Collective free will statistics
    pub free_will: CollectiveFreeWillStatistics,

    /// Collective resonance statistics
    pub resonance: CollectiveResonanceStatistics,
}

/// Collective dynamics statistics (basic)
#[derive(Debug, Clone, Default)]
pub struct CollectiveDynamicsStatistics {
    /// Total number of collectives
    pub total_collectives: usize,

    /// Average collective resonance
    pub average_collective_resonance: Float,

    /// Average group consciousness level
    pub average_group_consciousness: Float,

    /// Collective entities by behavior type
    pub collectives_by_behavior: HashMap<String, usize>,

    /// Collective density distribution
    pub collective_density_distribution: HashMap<String, usize>,

    /// Total collective decisions
    pub total_collective_decisions: usize,

    /// Total shared experiences
    pub total_shared_experiences: usize,

    /// Collective polarity distribution
    pub collective_polarity_distribution: PolarityDistribution,
}

/// Collective behavior statistics
#[derive(Debug, Clone)]
pub struct CollectiveBehaviorStatistics {
    /// Behavior type distribution
    pub behavior_type_distribution: HashMap<String, usize>,

    /// Average rotation speed
    pub average_rotation_speed: Float,

    /// Average stability
    pub average_stability: Float,

    /// Average complexity
    pub average_complexity: Float,

    /// Average adaptation rate
    pub average_adaptation_rate: Float,

    /// Average integration level
    pub average_integration_level: Float,
}

/// Group consciousness statistics
#[derive(Debug, Clone)]
pub struct GroupConsciousnessStatistics {
    /// Average consciousness coherence
    pub average_consciousness_coherence: Float,

    /// Average information integration
    pub average_information_integration: Float,

    /// Shared experience types distribution
    pub shared_experience_types: HashMap<String, usize>,

    /// Average shared experience intensity
    pub average_shared_experience_intensity: Float,

    /// Total collective memories
    pub total_collective_memories: usize,

    /// Average memory strength
    pub average_memory_strength: Float,

    /// Emergent properties statistics
    pub emergent_properties: HashMap<String, Float>,
}

/// Collective evolution statistics
#[derive(Debug, Clone)]
pub struct CollectiveEvolutionStatistics {
    /// Density distribution
    pub density_distribution: HashMap<String, usize>,

    /// Total density transitions
    pub total_density_transitions: usize,

    /// Average evolutionary progress
    pub average_evolutionary_progress: Float,

    /// Average collective learning rate
    pub average_collective_learning_rate: Float,

    /// Transitions by density
    pub transitions_by_density: HashMap<String, usize>,
}

/// Collective free will statistics
#[derive(Debug, Clone)]
pub struct CollectiveFreeWillStatistics {
    /// Total decisions made
    pub total_decisions: usize,

    /// Consensus mechanism distribution
    pub consensus_mechanism_distribution: HashMap<String, usize>,

    /// Average voting power
    pub average_voting_power: Float,

    /// Average decision confidence
    pub average_decision_confidence: Float,

    /// Average decision alignment
    pub average_decision_alignment: Float,

    /// Decision frequency (decisions per step)
    pub decision_frequency: Float,
}

/// Collective resonance statistics
#[derive(Debug, Clone)]
pub struct CollectiveResonanceStatistics {
    /// Average collective resonance
    pub average_collective_resonance: Float,

    /// Resonance distribution by scale
    pub resonance_by_scale: HashMap<String, Float>,

    /// Resonance clustering metrics
    pub resonance_clusters: usize,

    /// Average resonance strength
    pub average_resonance_strength: Float,

    /// Resonance coherence
    pub resonance_coherence: Float,
}

/// Collective Statistics Tracker
///
/// Tracks detailed statistics for collective entities.
pub struct CollectiveStatisticsTracker {
    /// Detailed statistics
    pub statistics: DetailedCollectiveStatistics,

    /// Historical data (for tracking trends)
    pub historical_data: Vec<HistoricalCollectiveData>,

    /// Simulation time
    pub simulation_time: u64,
}

/// Historical collective data
#[derive(Debug, Clone)]
pub struct HistoricalCollectiveData {
    /// Timestamp
    pub timestamp: u64,

    /// Total collectives
    pub total_collectives: usize,

    /// Average collective resonance
    pub average_collective_resonance: Float,

    /// Average group consciousness
    pub average_group_consciousness: Float,

    /// Total density transitions
    pub total_density_transitions: usize,

    /// Total collective decisions
    pub total_collective_decisions: usize,
}

// ============================================================================
// IMPLEMENTATIONS
// ============================================================================

impl Default for CollectiveBehaviorStatistics {
    fn default() -> Self {
        CollectiveBehaviorStatistics {
            behavior_type_distribution: HashMap::new(),
            average_rotation_speed: 0.0,
            average_stability: 0.0,
            average_complexity: 0.0,
            average_adaptation_rate: 0.0,
            average_integration_level: 0.0,
        }
    }
}

impl Default for GroupConsciousnessStatistics {
    fn default() -> Self {
        GroupConsciousnessStatistics {
            average_consciousness_coherence: 0.0,
            average_information_integration: 0.0,
            shared_experience_types: HashMap::new(),
            average_shared_experience_intensity: 0.0,
            total_collective_memories: 0,
            average_memory_strength: 0.0,
            emergent_properties: HashMap::new(),
        }
    }
}

impl Default for CollectiveEvolutionStatistics {
    fn default() -> Self {
        CollectiveEvolutionStatistics {
            density_distribution: HashMap::new(),
            total_density_transitions: 0,
            average_evolutionary_progress: 0.0,
            average_collective_learning_rate: 0.0,
            transitions_by_density: HashMap::new(),
        }
    }
}

impl Default for CollectiveFreeWillStatistics {
    fn default() -> Self {
        CollectiveFreeWillStatistics {
            total_decisions: 0,
            consensus_mechanism_distribution: HashMap::new(),
            average_voting_power: 0.0,
            average_decision_confidence: 0.0,
            average_decision_alignment: 0.0,
            decision_frequency: 0.0,
        }
    }
}

impl Default for CollectiveResonanceStatistics {
    fn default() -> Self {
        CollectiveResonanceStatistics {
            average_collective_resonance: 0.0,
            resonance_by_scale: HashMap::new(),
            resonance_clusters: 0,
            average_resonance_strength: 0.0,
            resonance_coherence: 0.0,
        }
    }
}

impl CollectiveStatisticsTracker {
    /// Create a new collective statistics tracker
    pub fn new() -> Self {
        CollectiveStatisticsTracker {
            statistics: DetailedCollectiveStatistics::default(),
            historical_data: Vec::new(),
            simulation_time: 0,
        }
    }

    /// Record statistics from collective dynamics manager
    pub fn record_statistics(
        &mut self,
        collective_behaviors: &HashMap<EntityId, CollectiveBehavior>,
        group_consciousness: &HashMap<EntityId, GroupConsciousness>,
        collective_evolution: &HashMap<EntityId, CollectiveEvolution>,
        collective_free_will: &HashMap<EntityId, CollectiveFreeWill>,
        holographic_field: &HolographicFieldManager,
    ) {
        self.simulation_time += 1;

        // Record basic statistics
        self.record_basic_statistics(
            collective_behaviors,
            collective_evolution,
            collective_free_will,
        );

        // Record behavior statistics
        self.record_behavior_statistics(collective_behaviors);

        // Record consciousness statistics
        self.record_consciousness_statistics(group_consciousness);

        // Record evolution statistics
        self.record_evolution_statistics(collective_evolution);

        // Record free will statistics
        self.record_free_will_statistics(collective_free_will);

        // Record resonance statistics (no longer uses entity_scales)
        self.record_resonance_statistics(collective_behaviors, holographic_field);

        // Record historical data periodically
        if self.simulation_time.is_multiple_of(50) {
            self.record_historical_data();
        }
    }

    /// Record basic statistics
    fn record_basic_statistics(
        &mut self,
        collective_behaviors: &HashMap<EntityId, CollectiveBehavior>,
        collective_evolution: &HashMap<EntityId, CollectiveEvolution>,
        collective_free_will: &HashMap<EntityId, CollectiveFreeWill>,
    ) {
        self.statistics.basic.total_collectives = collective_behaviors.len();

        if collective_behaviors.is_empty() {
            return;
        }

        // Average collective resonance
        let total_resonance: Float = collective_behaviors
            .values()
            .map(|b| b.collective_resonance)
            .sum();
        self.statistics.basic.average_collective_resonance =
            total_resonance / collective_behaviors.len() as Float;

        // Average group consciousness
        let total_consciousness: Float = collective_evolution
            .values()
            .map(|e| e.evolutionary_progress)
            .sum();
        self.statistics.basic.average_group_consciousness =
            total_consciousness / collective_evolution.len().max(1) as Float;

        // Collectives by behavior type
        self.statistics.basic.collectives_by_behavior.clear();
        for behavior in collective_behaviors.values() {
            let type_name = format!("{:?}", behavior.behavior_type);
            *self
                .statistics
                .basic
                .collectives_by_behavior
                .entry(type_name)
                .or_insert(0) += 1;
        }

        // Collective density distribution
        self.statistics
            .basic
            .collective_density_distribution
            .clear();
        for evolution in collective_evolution.values() {
            let density_name = format!("{:?}", evolution.collective_density);
            *self
                .statistics
                .basic
                .collective_density_distribution
                .entry(density_name)
                .or_insert(0) += 1;
        }

        // Total collective decisions
        self.statistics.basic.total_collective_decisions = collective_free_will
            .values()
            .map(|fw| fw.decision_history.len())
            .sum();

        // Total shared experiences (simplified - would need group consciousness data)
        self.statistics.basic.total_shared_experiences = 0;

        // Collective polarity distribution
        let mut sto_count = 0;
        let mut sts_count = 0;
        let mut unpolarized_count = 0;
        let mut total_sto_score = 0.0;
        let mut total_sts_score = 0.0;

        for evolution in collective_evolution.values() {
            match evolution.group_polarity {
                Some(crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToOthers) => {
                    sto_count += 1;
                    if let Some(fw) = collective_free_will.get(&evolution.collective_id) {
                        total_sto_score += fw.collective_polarization_score.abs();
                    }
                }
                Some(crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToSelf) => {
                    sts_count += 1;
                    if let Some(fw) = collective_free_will.get(&evolution.collective_id) {
                        total_sts_score += fw.collective_polarization_score.abs();
                    }
                }
                Some(crate::simulation_v3::catalyst_system::PolarityChoice::Unpolarized) | None => {
                    unpolarized_count += 1
                }
            }
        }

        self.statistics.basic.collective_polarity_distribution = PolarityDistribution {
            sto_count,
            sts_count,
            unpolarized_count,
            avg_sto_score: if sto_count > 0 {
                total_sto_score / sto_count as Float
            } else {
                0.0
            },
            avg_sts_score: if sts_count > 0 {
                total_sts_score / sts_count as Float
            } else {
                0.0
            },
        };
    }

    /// Record behavior statistics
    fn record_behavior_statistics(
        &mut self,
        collective_behaviors: &HashMap<EntityId, CollectiveBehavior>,
    ) {
        if collective_behaviors.is_empty() {
            return;
        }

        // Behavior type distribution
        self.statistics.behavior.behavior_type_distribution.clear();
        for behavior in collective_behaviors.values() {
            let type_name = format!("{:?}", behavior.behavior_type);
            *self
                .statistics
                .behavior
                .behavior_type_distribution
                .entry(type_name)
                .or_insert(0) += 1;
        }

        // Average behavior parameters
        let total_rotation_speed: Float = collective_behaviors
            .values()
            .map(|b| b.behavior_parameters.rotation_speed)
            .sum();
        self.statistics.behavior.average_rotation_speed =
            total_rotation_speed / collective_behaviors.len() as Float;

        let total_stability: Float = collective_behaviors
            .values()
            .map(|b| b.behavior_parameters.stability)
            .sum();
        self.statistics.behavior.average_stability =
            total_stability / collective_behaviors.len() as Float;

        let total_complexity: Float = collective_behaviors
            .values()
            .map(|b| b.behavior_parameters.complexity)
            .sum();
        self.statistics.behavior.average_complexity =
            total_complexity / collective_behaviors.len() as Float;

        let total_adaptation_rate: Float = collective_behaviors
            .values()
            .map(|b| b.behavior_parameters.adaptation_rate)
            .sum();
        self.statistics.behavior.average_adaptation_rate =
            total_adaptation_rate / collective_behaviors.len() as Float;

        let total_integration_level: Float = collective_behaviors
            .values()
            .map(|b| b.behavior_parameters.integration_level)
            .sum();
        self.statistics.behavior.average_integration_level =
            total_integration_level / collective_behaviors.len() as Float;
    }

    /// Record consciousness statistics
    fn record_consciousness_statistics(
        &mut self,
        group_consciousness: &HashMap<EntityId, GroupConsciousness>,
    ) {
        if group_consciousness.is_empty() {
            return;
        }

        // Average consciousness coherence
        let total_coherence: Float = group_consciousness
            .values()
            .map(|gc| gc.consciousness_coherence)
            .sum();
        self.statistics
            .consciousness
            .average_consciousness_coherence = total_coherence / group_consciousness.len() as Float;

        // Average information integration
        let total_integration: Float = group_consciousness
            .values()
            .map(|gc| gc.information_integration)
            .sum();
        self.statistics
            .consciousness
            .average_information_integration =
            total_integration / group_consciousness.len() as Float;

        // Shared experience types
        self.statistics
            .consciousness
            .shared_experience_types
            .clear();
        for gc in group_consciousness.values() {
            for exp in &gc.shared_experiences {
                let type_name = format!("{:?}", exp.experience_type);
                *self
                    .statistics
                    .consciousness
                    .shared_experience_types
                    .entry(type_name)
                    .or_insert(0) += 1;
            }
        }

        // Average shared experience intensity
        let all_intensities: Vec<Float> = group_consciousness
            .values()
            .flat_map(|gc| gc.shared_experiences.iter().map(|exp| exp.intensity))
            .collect();
        if !all_intensities.is_empty() {
            self.statistics
                .consciousness
                .average_shared_experience_intensity =
                all_intensities.iter().sum::<Float>() / all_intensities.len() as Float;
        }

        // Total collective memories
        self.statistics.consciousness.total_collective_memories = group_consciousness
            .values()
            .map(|gc| gc.collective_memory.len())
            .sum();

        // Average memory strength
        let all_memory_strengths: Vec<Float> = group_consciousness
            .values()
            .flat_map(|gc| gc.collective_memory.iter().map(|mem| mem.strength))
            .collect();
        if !all_memory_strengths.is_empty() {
            self.statistics.consciousness.average_memory_strength =
                all_memory_strengths.iter().sum::<Float>() / all_memory_strengths.len() as Float;
        }

        // Emergent properties (simplified - average across all collectives)
        self.statistics.consciousness.emergent_properties.clear();
        if let Some(first_gc) = group_consciousness.values().next() {
            for (key, value) in &first_gc.emergent_properties {
                self.statistics
                    .consciousness
                    .emergent_properties
                    .insert(key.clone(), *value);
            }
        }
    }

    /// Record evolution statistics
    fn record_evolution_statistics(
        &mut self,
        collective_evolution: &HashMap<EntityId, CollectiveEvolution>,
    ) {
        if collective_evolution.is_empty() {
            return;
        }

        // Density distribution
        self.statistics.evolution.density_distribution.clear();
        for evolution in collective_evolution.values() {
            let density_name = format!("{:?}", evolution.collective_density);
            *self
                .statistics
                .evolution
                .density_distribution
                .entry(density_name)
                .or_insert(0) += 1;
        }

        // Total density transitions
        self.statistics.evolution.total_density_transitions = collective_evolution
            .values()
            .map(|e| e.collective_transitions.len())
            .sum();

        // Average evolutionary progress
        let total_progress: Float = collective_evolution
            .values()
            .map(|e| e.evolutionary_progress)
            .sum();
        self.statistics.evolution.average_evolutionary_progress =
            total_progress / collective_evolution.len() as Float;

        // Average collective learning rate
        let total_learning_rate: Float = collective_evolution
            .values()
            .map(|e| e.collective_learning_rate)
            .sum();
        self.statistics.evolution.average_collective_learning_rate =
            total_learning_rate / collective_evolution.len() as Float;

        // Transitions by density
        self.statistics.evolution.transitions_by_density.clear();
        for evolution in collective_evolution.values() {
            for transition in &evolution.collective_transitions {
                let density_name = format!("{:?}", transition.from_density);
                *self
                    .statistics
                    .evolution
                    .transitions_by_density
                    .entry(density_name)
                    .or_insert(0) += 1;
            }
        }
    }

    /// Record free will statistics
    fn record_free_will_statistics(
        &mut self,
        collective_free_will: &HashMap<EntityId, CollectiveFreeWill>,
    ) {
        if collective_free_will.is_empty() {
            return;
        }

        // Total decisions
        self.statistics.free_will.total_decisions = collective_free_will
            .values()
            .map(|fw| fw.decision_history.len())
            .sum();

        // Consensus mechanism distribution
        self.statistics
            .free_will
            .consensus_mechanism_distribution
            .clear();
        for fw in collective_free_will.values() {
            let mechanism_name = format!("{:?}", fw.consensus_mechanism);
            *self
                .statistics
                .free_will
                .consensus_mechanism_distribution
                .entry(mechanism_name)
                .or_insert(0) += 1;
        }

        // Average voting power
        let all_voting_powers: Vec<Float> = collective_free_will
            .values()
            .flat_map(|fw| fw.voting_power.values().cloned())
            .collect();
        if !all_voting_powers.is_empty() {
            self.statistics.free_will.average_voting_power =
                all_voting_powers.iter().sum::<Float>() / all_voting_powers.len() as Float;
        }

        // Average decision confidence and alignment
        let all_confidences: Vec<Float> = collective_free_will
            .values()
            .flat_map(|fw| fw.decision_history.iter().map(|d| d.outcome.confidence))
            .collect();
        let all_alignments: Vec<Float> = collective_free_will
            .values()
            .flat_map(|fw| fw.decision_history.iter().map(|d| d.outcome.alignment))
            .collect();

        if !all_confidences.is_empty() {
            self.statistics.free_will.average_decision_confidence =
                all_confidences.iter().sum::<Float>() / all_confidences.len() as Float;
        }

        if !all_alignments.is_empty() {
            self.statistics.free_will.average_decision_alignment =
                all_alignments.iter().sum::<Float>() / all_alignments.len() as Float;
        }

        // Decision frequency
        self.statistics.free_will.decision_frequency = if self.simulation_time > 0 {
            self.statistics.free_will.total_decisions as Float / self.simulation_time as Float
        } else {
            0.0
        };
    }

    /// Record resonance statistics
    fn record_resonance_statistics(
        &mut self,
        collective_behaviors: &HashMap<EntityId, CollectiveBehavior>,
        holographic_field: &HolographicFieldManager,
    ) {
        if collective_behaviors.is_empty() {
            return;
        }

        // Average collective resonance
        let total_resonance: Float = collective_behaviors
            .values()
            .map(|b| b.collective_resonance)
            .sum();
        self.statistics.resonance.average_collective_resonance =
            total_resonance / collective_behaviors.len() as Float;

        // All collectives start at First density, so no "resonance by scale" needed
        // In the future, this could be based on current density instead

        // Resonance clusters
        self.statistics.resonance.resonance_clusters =
            holographic_field.resonance_tracker.resonance_clusters.len();

        // Average resonance strength (from holographic field)
        self.statistics.resonance.average_resonance_strength =
            holographic_field.statistics.average_connection_strength;

        // Resonance coherence
        self.statistics.resonance.resonance_coherence =
            holographic_field.statistics.global_phase_coherence;
    }

    /// Record historical data
    fn record_historical_data(&mut self) {
        let historical = HistoricalCollectiveData {
            timestamp: self.simulation_time,
            total_collectives: self.statistics.basic.total_collectives,
            average_collective_resonance: self.statistics.basic.average_collective_resonance,
            average_group_consciousness: self.statistics.basic.average_group_consciousness,
            total_density_transitions: self.statistics.evolution.total_density_transitions,
            total_collective_decisions: self.statistics.free_will.total_decisions,
        };

        self.historical_data.push(historical);

        // Limit historical data size
        if self.historical_data.len() > 100 {
            self.historical_data.remove(0);
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &DetailedCollectiveStatistics {
        &self.statistics
    }

    /// Get historical data
    pub fn get_historical_data(&self) -> &Vec<HistoricalCollectiveData> {
        &self.historical_data
    }
}

impl Default for CollectiveStatisticsTracker {
    fn default() -> Self {
        Self::new()
    }
}
