//! Emergent Behavior System
//!
//! This module implements Phase 5 of the refactor plan: Emergent Behavior.
//!
//! Emergent behavior arises from complex interactions between entities, creating
//! system-level properties that are not present in individual components.
//!
//! ## Emergence Types
//!
//! 1. **System-Level Emergence**: Properties that emerge from the whole system
//!    - Global coherence
//!    - Collective consciousness fields
//!    - System-wide resonance patterns
//!    - Emergent intelligence
//!
//! 2. **Environmental Emergence**: How environment emerges from 1st Density materials
//!    - Environment creation from quantum particles, atoms, molecules
//!    - Planetary and galactic structures
//!    - Co-evolution of environment and entities
//!    - Environmental feedback loops
//!
//! ## Key Concepts
//!
//! - **Emergence**: The whole is greater than the sum of parts
//! - **Self-Organization**: System spontaneously organizes without central control
//! - **Criticality**: System operates near phase transitions
//! - **Coherence**: System-wide alignment and harmony
//! - **Resilience**: System's ability to maintain structure despite perturbations

use crate::entity_layer7::layer7::{EntityId, EntityType, SubSubLogos};
use std::collections::HashMap;

/// System-level emergent properties that arise from the collective
#[derive(Debug, Clone)]
pub struct SystemEmergence {
    /// Global coherence: how aligned the entire system is
    pub global_coherence: f64,

    /// Collective consciousness field strength
    pub collective_consciousness_strength: f64,

    /// System-wide resonance
    pub system_resonance: f64,

    /// Emergent intelligence level
    pub emergent_intelligence: f64,

    /// System complexity (information content)
    pub system_complexity: f64,

    /// Self-organization level
    pub self_organization: f64,

    /// System resilience to perturbations
    pub resilience: f64,

    /// Criticality: proximity to phase transitions
    pub criticality: f64,

    /// Timestamp of this emergence measurement
    pub timestamp: u64,
}

impl Default for SystemEmergence {
    fn default() -> Self {
        Self {
            global_coherence: 0.0,
            collective_consciousness_strength: 0.0,
            system_resonance: 0.0,
            emergent_intelligence: 0.0,
            system_complexity: 0.0,
            self_organization: 0.0,
            resilience: 0.5,
            criticality: 0.5,
            timestamp: 0,
        }
    }
}

/// Environmental emergence from 1st Density materials
#[derive(Debug, Clone)]
pub struct EnvironmentalEmergence {
    /// How much environment has emerged from 1st Density materials
    pub environment_emergence_level: f64,

    /// Planetary structure formation
    pub planetary_formation: f64,

    /// Galactic structure formation
    pub galactic_formation: f64,

    /// Environmental stability
    pub environmental_stability: f64,

    /// Environmental diversity (number of distinct environments)
    pub environmental_diversity: f64,

    /// Entity-environment integration
    pub entity_environment_integration: f64,

    /// Environmental influence on entity evolution
    pub environmental_influence: f64,

    /// Co-evolution strength between environment and entities
    pub co_evolution_strength: f64,

    /// Timestamp of this emergence measurement
    pub timestamp: u64,
}

impl Default for EnvironmentalEmergence {
    fn default() -> Self {
        Self {
            environment_emergence_level: 0.0,
            planetary_formation: 0.0,
            galactic_formation: 0.0,
            environmental_stability: 0.5,
            environmental_diversity: 0.0,
            entity_environment_integration: 0.0,
            environmental_influence: 0.0,
            co_evolution_strength: 0.0,
            timestamp: 0,
        }
    }
}

/// Emergent event: when significant emergence occurs
#[derive(Debug, Clone)]
pub struct EmergentEvent {
    /// Type of emergent event
    pub event_type: EmergentEventType,

    /// When the event occurred
    pub timestamp: u64,

    /// Description of the event
    pub description: String,

    /// Magnitude of the event
    pub magnitude: f64,

    /// Affected entities (IDs)
    pub affected_entities: Vec<EntityId>,

    /// System state before event
    pub system_state_before: SystemEmergence,

    /// System state after event
    pub system_state_after: SystemEmergence,

    /// Environmental state before event (if applicable)
    pub environmental_state_before: Option<EnvironmentalEmergence>,

    /// Environmental state after event (if applicable)
    pub environmental_state_after: Option<EnvironmentalEmergence>,
}

/// Types of emergent events
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmergentEventType {
    /// Global coherence breakthrough
    GlobalCoherenceBreakthrough,

    /// Collective consciousness emergence
    CollectiveConsciousnessEmergence,

    /// System phase transition
    SystemPhaseTransition,

    /// Environmental emergence milestone
    EnvironmentalEmergenceMilestone,

    /// Self-organization spike
    SelfOrganizationSpike,

    /// Criticality threshold crossing
    CriticalityThresholdCrossing,

    /// Co-evolution synchronization
    CoEvolutionSynchronization,

    /// Resilience test
    ResilienceTest,
}

/// Emergence history over time
#[derive(Debug, Clone)]
pub struct EmergenceHistory {
    /// Historical system emergence measurements
    pub system_history: Vec<SystemEmergence>,

    /// Historical environmental emergence measurements
    pub environmental_history: Vec<EnvironmentalEmergence>,

    /// Emergent events
    pub events: Vec<EmergentEvent>,

    /// Maximum history length
    pub max_history_length: usize,
}

impl Default for EmergenceHistory {
    fn default() -> Self {
        Self {
            system_history: Vec::new(),
            environmental_history: Vec::new(),
            events: Vec::new(),
            max_history_length: 1000,
        }
    }
}

impl EmergenceHistory {
    /// Add system emergence measurement
    pub fn add_system_measurement(&mut self, emergence: SystemEmergence) {
        self.system_history.push(emergence);
        if self.system_history.len() > self.max_history_length {
            self.system_history.remove(0);
        }
    }

    /// Add environmental emergence measurement
    pub fn add_environmental_measurement(&mut self, emergence: EnvironmentalEmergence) {
        self.environmental_history.push(emergence);
        if self.environmental_history.len() > self.max_history_length {
            self.environmental_history.remove(0);
        }
    }

    /// Add emergent event
    pub fn add_event(&mut self, event: EmergentEvent) {
        self.events.push(event);
        if self.events.len() > self.max_history_length {
            self.events.remove(0);
        }
    }

    /// Get latest system emergence
    pub fn latest_system_emergence(&self) -> Option<&SystemEmergence> {
        self.system_history.last()
    }

    /// Get latest environmental emergence
    pub fn latest_environmental_emergence(&self) -> Option<&EnvironmentalEmergence> {
        self.environmental_history.last()
    }

    /// Get events by type
    pub fn events_by_type(&self, event_type: EmergentEventType) -> Vec<&EmergentEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Get events in time range
    pub fn events_in_range(&self, start: u64, end: u64) -> Vec<&EmergentEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

/// Emergence statistics
#[derive(Debug, Clone, Default)]
pub struct EmergenceStatistics {
    /// Average global coherence over time
    pub avg_global_coherence: f64,

    /// Peak global coherence achieved
    pub peak_global_coherence: f64,

    /// Average collective consciousness strength
    pub avg_collective_consciousness: f64,

    /// System complexity growth rate
    pub complexity_growth_rate: f64,

    /// Number of emergent events
    pub total_events: usize,

    /// Events by type
    pub events_by_type: HashMap<EmergentEventType, usize>,

    /// Environmental emergence level
    pub final_environment_emergence: f64,

    /// Co-evolution strength
    pub final_co_evolution_strength: f64,
}

/// Emergence Manager
///
/// Tracks and calculates emergent properties of the system.
pub struct EmergenceManager {
    /// Current system emergence
    pub current_system_emergence: SystemEmergence,

    /// Current environmental emergence
    pub current_environmental_emergence: EnvironmentalEmergence,

    /// Emergence history
    pub history: EmergenceHistory,

    /// Current timestamp
    pub timestamp: u64,

    /// Configuration
    pub config: EmergenceConfig,
}

/// Emergence configuration
#[derive(Debug, Clone)]
pub struct EmergenceConfig {
    /// How often to calculate emergence (every N steps)
    pub calculation_interval: u64,

    /// Coherence threshold for breakthrough events
    pub coherence_breakthrough_threshold: f64,

    /// Criticality threshold for phase transition
    pub criticality_threshold: f64,

    /// Environmental emergence threshold
    pub environmental_emergence_threshold: f64,

    /// Co-evolution synchronization threshold
    pub co_evolution_sync_threshold: f64,

    /// Maximum history length
    pub max_history_length: usize,
}

impl Default for EmergenceConfig {
    fn default() -> Self {
        Self {
            calculation_interval: 10,
            coherence_breakthrough_threshold: 0.85,
            criticality_threshold: 0.9,
            environmental_emergence_threshold: 0.7,
            co_evolution_sync_threshold: 0.8,
            max_history_length: 1000,
        }
    }
}

impl Default for EmergenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EmergenceManager {
    /// Create a new emergence manager
    pub fn new() -> Self {
        let config = EmergenceConfig::default();
        Self {
            current_system_emergence: SystemEmergence::default(),
            current_environmental_emergence: EnvironmentalEmergence::default(),
            history: EmergenceHistory {
                max_history_length: config.max_history_length,
                ..Default::default()
            },
            timestamp: 0,
            config,
        }
    }

    /// Create with custom config
    pub fn with_config(config: EmergenceConfig) -> Self {
        Self {
            current_system_emergence: SystemEmergence::default(),
            current_environmental_emergence: EnvironmentalEmergence::default(),
            history: EmergenceHistory {
                max_history_length: config.max_history_length,
                ..Default::default()
            },
            timestamp: 0,
            config,
        }
    }

    /// Calculate system-level emergence from entities
    pub fn calculate_system_emergence(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> SystemEmergence {
        if entities.is_empty() {
            return SystemEmergence::default();
        }

        // Calculate global coherence
        let global_coherence = self.calculate_global_coherence(entities);

        // Calculate collective consciousness strength
        let collective_consciousness = self.calculate_collective_consciousness(entities);

        // Calculate system resonance
        let system_resonance = self.calculate_system_resonance(entities);

        // Calculate emergent intelligence
        let emergent_intelligence = self.calculate_emergent_intelligence(entities);

        // Calculate system complexity
        let system_complexity = self.calculate_system_complexity(entities);

        // Calculate self-organization
        let self_organization = self.calculate_self_organization(entities);

        // Calculate resilience
        let resilience = self.calculate_resilience(entities);

        // Calculate criticality
        let criticality = self.calculate_criticality(entities);

        let emergence = SystemEmergence {
            global_coherence,
            collective_consciousness_strength: collective_consciousness,
            system_resonance,
            emergent_intelligence,
            system_complexity,
            self_organization,
            resilience,
            criticality,
            timestamp: self.timestamp,
        };

        self.current_system_emergence = emergence.clone();
        self.history.add_system_measurement(emergence.clone());

        emergence
    }

    /// Calculate environmental emergence from 1st Density materials
    pub fn calculate_environmental_emergence(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> EnvironmentalEmergence {
        if entities.is_empty() {
            return EnvironmentalEmergence::default();
        }

        // Calculate environment emergence level
        let environment_emergence = self.calculate_environment_emergence_level(entities);

        // Calculate planetary formation
        let planetary_formation = self.calculate_planetary_formation(entities);

        // Calculate galactic formation
        let galactic_formation = self.calculate_galactic_formation(entities);

        // Calculate environmental stability
        let environmental_stability = self.calculate_environmental_stability(entities);

        // Calculate environmental diversity
        let environmental_diversity = self.calculate_environmental_diversity(entities);

        // Calculate entity-environment integration
        let entity_environment_integration =
            self.calculate_entity_environment_integration(entities);

        // Calculate environmental influence
        let environmental_influence = self.calculate_environmental_influence(entities);

        // Calculate co-evolution strength
        let co_evolution_strength = self.calculate_co_evolution_strength(entities);

        let emergence = EnvironmentalEmergence {
            environment_emergence_level: environment_emergence,
            planetary_formation,
            galactic_formation,
            environmental_stability,
            environmental_diversity,
            entity_environment_integration,
            environmental_influence,
            co_evolution_strength,
            timestamp: self.timestamp,
        };

        self.current_environmental_emergence = emergence.clone();
        self.history
            .add_environmental_measurement(emergence.clone());

        emergence
    }

    /// Calculate global coherence
    ///
    /// Global coherence measures how aligned the entire system is.
    /// High coherence means entities are working in harmony.
    fn calculate_global_coherence(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Calculate average spectrum access alignment
        let spectrum_alignments: Vec<f64> = entities
            .values()
            .map(|e| {
                // Get density from evolutionary parameters or spectrum configuration
                // For now, use spectrum access as a proxy for density level
                let density_factor = (e.spectrum_access.space_time_access * 3.0 + 0.5).min(1.5);

                // Use polarization alignment (STO entities contribute positively)
                let polarization_bias = e
                    .spectrum_configuration
                    .evolutionary_parameters
                    .polarity_bias;
                let polarization_factor = if polarization_bias >= 0.0 {
                    (polarization_bias + 1.0) / 2.0
                } else {
                    (1.0 - polarization_bias.abs()) / 2.0
                };

                density_factor * polarization_factor
            })
            .collect();

        let avg_alignment =
            spectrum_alignments.iter().sum::<f64>() / spectrum_alignments.len() as f64;

        // Coherence increases with number of entities (network effect)
        let network_effect = 1.0 - (1.0 / (entities.len() as f64).sqrt());

        avg_alignment * network_effect
    }

    /// Calculate collective consciousness strength
    ///
    /// Measures the strength of the collective consciousness field.
    fn calculate_collective_consciousness(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Count collective entities
        let collective_count = entities
            .values()
            .filter(|e| e.entity_type == EntityType::Collective)
            .count();

        if collective_count == 0 {
            return 0.0;
        }

        // Calculate average collective consciousness
        let collective_consciousness: Vec<f64> = entities
            .values()
            .filter(|e| e.entity_type == EntityType::Collective)
            .map(|e| {
                // Collective consciousness increases with spectrum access
                let density_factor = (e.spectrum_access.space_time_access * 3.0 + 0.5).min(4.0);
                // And with polarization bias (STO enhances collective consciousness)
                let polarization_bias = e
                    .spectrum_configuration
                    .evolutionary_parameters
                    .polarity_bias;
                let polarization_factor = if polarization_bias >= 0.0 {
                    (polarization_bias + 1.0) / 2.0
                } else {
                    (1.0 - polarization_bias.abs()) / 2.0
                };

                density_factor * polarization_factor
            })
            .collect();

        let avg =
            collective_consciousness.iter().sum::<f64>() / collective_consciousness.len() as f64;

        // Scale by proportion of collective entities
        let collective_ratio = collective_count as f64 / entities.len() as f64;

        avg * collective_ratio
    }

    /// Calculate system resonance
    ///
    /// Measures the harmonic resonance across the system.
    fn calculate_system_resonance(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Calculate resonance based on spectrum access harmony
        let spectrum_resonances: Vec<f64> = entities
            .values()
            .map(|e| {
                // Resonance increases with spectrum access
                let spectrum_factor =
                    e.spectrum_access.space_time_access + e.spectrum_access.time_space_access;

                // And with balanced spectrum ratio (closer to 1.0 is more balanced)
                let ratio_balance = 1.0 / (1.0 + (e.spectrum_access.ratio - 1.0).abs());

                spectrum_factor * ratio_balance
            })
            .collect();

        spectrum_resonances.iter().sum::<f64>() / spectrum_resonances.len() as f64
    }

    /// Calculate emergent intelligence
    ///
    /// Measures intelligence that emerges from collective interactions.
    fn calculate_emergent_intelligence(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Emergent intelligence comes from complex interactions
        // Higher spectrum access entities contribute more
        let intelligence_contributions: Vec<f64> = entities
            .values()
            .map(|e| {
                let density = e.spectrum_access.space_time_access * 3.0 + 0.5;

                // Intelligence increases exponentially with density
                let intelligence = density.powf(1.5);

                // Modulated by polarization
                let polarization_bias = e
                    .spectrum_configuration
                    .evolutionary_parameters
                    .polarity_bias;
                let polarization_factor = if polarization_bias >= 0.0 {
                    (polarization_bias + 1.0) / 2.0
                } else {
                    (1.0 - polarization_bias.abs()) / 2.0
                };

                intelligence * polarization_factor
            })
            .collect();

        let sum = intelligence_contributions.iter().sum::<f64>();

        // Normalize by number of entities
        sum / entities.len() as f64
    }

    /// Calculate system complexity
    ///
    /// Measures the information content of the system.
    fn calculate_system_complexity(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Complexity increases with diversity of entities
        // Use spectrum access as a proxy for density level
        let densities: Vec<u8> = entities
            .values()
            .map(|e| ((e.spectrum_access.space_time_access * 3.0).ceil() as u8).max(1))
            .collect();

        // Shannon entropy of density distribution
        let mut density_counts = HashMap::new();
        for density in densities {
            *density_counts.entry(density).or_insert(0) += 1;
        }

        let total = entities.len() as f64;
        let entropy: f64 = density_counts
            .values()
            .map(|&count| {
                let p = count as f64 / total;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum();

        entropy
    }

    /// Calculate self-organization
    ///
    /// Measures how much the system organizes itself spontaneously.
    fn calculate_self_organization(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Self-organization increases with collective entities
        let collective_count = entities
            .values()
            .filter(|e| e.entity_type == EntityType::Collective)
            .count();

        if collective_count == 0 {
            return 0.0;
        }

        // Calculate organization based on collective structure
        let organization_scores: Vec<f64> = entities
            .values()
            .filter(|e| e.entity_type == EntityType::Collective)
            .map(|e| {
                // Organization increases with number of children
                let children_count = e.children.len() as f64;

                // But follows diminishing returns
                children_count.sqrt() / 10.0
            })
            .collect();

        if organization_scores.is_empty() {
            return 0.0;
        }

        let avg = organization_scores.iter().sum::<f64>() / organization_scores.len() as f64;

        avg.min(1.0)
    }

    /// Calculate resilience
    ///
    /// Measures the system's ability to maintain structure despite perturbations.
    fn calculate_resilience(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.5;
        }

        // Resilience increases with:
        // 1. System diversity
        // 2. Redundancy (multiple entities at each scale)
        // 3. Collective support structures

        // Use spectrum access distribution as a proxy for diversity
        let spectrum_access_values: Vec<u8> = entities
            .values()
            .map(|e| ((e.spectrum_access.space_time_access * 3.0).ceil() as u8).max(1))
            .collect();
        let unique_densities = spectrum_access_values
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len() as f64;

        let diversity_factor = unique_densities / 8.0; // Divide by max densities

        // Collective entities provide structural support
        let collective_ratio = entities
            .values()
            .filter(|e| e.entity_type == EntityType::Collective)
            .count() as f64
            / entities.len() as f64;

        0.3 + 0.3 * diversity_factor + 0.4 * collective_ratio
    }

    /// Calculate criticality
    ///
    /// Measures proximity to phase transitions.
    fn calculate_criticality(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.5;
        }

        // Criticality increases with system complexity and coherence
        let complexity = self.calculate_system_complexity(entities);
        let coherence = self.calculate_global_coherence(entities);

        // Criticality peaks when complexity is high and coherence is moderate
        // (too much coherence = order, too little = chaos)
        let optimal_coherence = 0.7;
        let coherence_distance = (coherence - optimal_coherence).abs();

        let criticality = complexity * (1.0 - coherence_distance * 2.0);

        criticality.max(0.0).min(1.0)
    }

    /// Calculate environment emergence level
    ///
    /// Measures how much environment has emerged from 1st Density materials.
    fn calculate_environment_emergence_level(
        &self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Count 1st Density entities (low spectrum access) that create environment
        let first_density_count = entities
            .values()
            .filter(|e| e.spectrum_access.space_time_access <= 0.5)
            .count();

        if first_density_count == 0 {
            return 0.0;
        }

        // Environment emergence increases with number of 1st Density entities
        // But follows diminishing returns
        let emergence = (first_density_count as f64).ln() / 10.0;

        emergence.min(1.0)
    }

    /// Calculate planetary formation

    /// Calculate planetary formation
    ///
    /// Measures how much planetary structure has formed.
    fn calculate_planetary_formation(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Planetary formation requires entities with medium spectrum access
        let molecule_count = entities
            .values()
            .filter(|e| {
                e.spectrum_access.space_time_access > 0.3
                    && e.spectrum_access.space_time_access < 0.7
            })
            .count();

        if molecule_count == 0 {
            return 0.0;
        }

        // Formation increases with molecule count
        let formation = (molecule_count as f64).ln() / 8.0;

        formation.min(1.0)
    }

    /// Calculate galactic formation
    ///
    /// Measures how much galactic structure has formed.
    fn calculate_galactic_formation(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Galactic formation requires entities with low spectrum access
        let atom_count = entities
            .values()
            .filter(|e| {
                e.spectrum_access.space_time_access > 0.1
                    && e.spectrum_access.space_time_access < 0.4
            })
            .count();

        if atom_count == 0 {
            return 0.0;
        }

        // Formation increases with atom count
        let formation = (atom_count as f64).ln() / 10.0;

        formation.min(1.0)
    }

    /// Calculate environmental stability
    ///
    /// Measures how stable the environment is.
    fn calculate_environmental_stability(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.5;
        }

        // Stability increases with environmental diversity and coherence
        let diversity = self.calculate_environmental_diversity(entities);
        let coherence = self.calculate_global_coherence(entities);

        0.5 * diversity + 0.5 * coherence
    }

    /// Calculate environmental diversity
    ///
    /// Measures the number of distinct environments.
    fn calculate_environmental_diversity(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Diversity based on spectrum access distribution
        let spectrum_access_values: Vec<f64> = entities
            .values()
            .map(|e| e.spectrum_access.space_time_access)
            .collect();

        // Calculate standard deviation as a measure of diversity
        let mean = spectrum_access_values.iter().sum::<f64>() / spectrum_access_values.len() as f64;
        let variance = spectrum_access_values
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / spectrum_access_values.len() as f64;

        variance.sqrt().min(1.0)
    }

    /// Calculate entity-environment integration
    ///
    /// Measures how well entities are integrated into their environment.
    fn calculate_entity_environment_integration(
        &self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Integration increases with entities having environment relationships
        let entities_with_environment = entities
            .values()
            .filter(|e| e.environment_id.is_some())
            .count();

        if entities_with_environment == 0 {
            return 0.0;
        }

        entities_with_environment as f64 / entities.len() as f64
    }

    /// Calculate environmental influence
    ///
    /// Measures how much the environment influences entity evolution.
    fn calculate_environmental_influence(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Influence increases with environmental emergence and entity-environment integration
        let environment_emergence = self
            .current_environmental_emergence
            .environment_emergence_level;
        let integration = self.calculate_entity_environment_integration(entities);

        environment_emergence * integration
    }

    /// Calculate co-evolution strength
    ///
    /// Measures the co-evolution between environment and entities.
    fn calculate_co_evolution_strength(&self, entities: &HashMap<EntityId, SubSubLogos>) -> f64 {
        if entities.is_empty() {
            return 0.0;
        }

        // Co-evolution increases when both environment and entities are evolving
        let environmental_emergence = self
            .current_environmental_emergence
            .environment_emergence_level;
        let system_emergence = self.current_system_emergence.global_coherence;

        // Co-evolution is strongest when both are high
        environmental_emergence * system_emergence
    }

    /// Detect emergent events
    pub fn detect_emergent_events(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> Vec<EmergentEvent> {
        let mut events = Vec::new();

        // Check for global coherence breakthrough
        if self.current_system_emergence.global_coherence
            >= self.config.coherence_breakthrough_threshold
        {
            events.push(EmergentEvent {
                event_type: EmergentEventType::GlobalCoherenceBreakthrough,
                timestamp: self.timestamp,
                description: format!(
                    "Global coherence reached {:.2}",
                    self.current_system_emergence.global_coherence
                ),
                magnitude: self.current_system_emergence.global_coherence,
                affected_entities: entities.keys().cloned().collect(),
                system_state_before: self
                    .history
                    .system_history
                    .get(self.history.system_history.len().saturating_sub(2))
                    .cloned()
                    .unwrap_or_default(),
                system_state_after: self.current_system_emergence.clone(),
                environmental_state_before: None,
                environmental_state_after: None,
            });
        }

        // Check for criticality threshold crossing
        if self.current_system_emergence.criticality >= self.config.criticality_threshold {
            events.push(EmergentEvent {
                event_type: EmergentEventType::CriticalityThresholdCrossing,
                timestamp: self.timestamp,
                description: format!(
                    "System criticality reached {:.2}",
                    self.current_system_emergence.criticality
                ),
                magnitude: self.current_system_emergence.criticality,
                affected_entities: entities.keys().cloned().collect(),
                system_state_before: self
                    .history
                    .system_history
                    .get(self.history.system_history.len().saturating_sub(2))
                    .cloned()
                    .unwrap_or_default(),
                system_state_after: self.current_system_emergence.clone(),
                environmental_state_before: None,
                environmental_state_after: None,
            });
        }

        // Check for environmental emergence milestone
        if self
            .current_environmental_emergence
            .environment_emergence_level
            >= self.config.environmental_emergence_threshold
        {
            events.push(EmergentEvent {
                event_type: EmergentEventType::EnvironmentalEmergenceMilestone,
                timestamp: self.timestamp,
                description: format!(
                    "Environmental emergence reached {:.2}",
                    self.current_environmental_emergence
                        .environment_emergence_level
                ),
                magnitude: self
                    .current_environmental_emergence
                    .environment_emergence_level,
                affected_entities: entities.keys().cloned().collect(),
                system_state_before: self.current_system_emergence.clone(),
                system_state_after: self.current_system_emergence.clone(),
                environmental_state_before: self
                    .history
                    .environmental_history
                    .get(self.history.environmental_history.len().saturating_sub(2))
                    .cloned(),
                environmental_state_after: Some(self.current_environmental_emergence.clone()),
            });
        }

        // Check for co-evolution synchronization
        if self.current_environmental_emergence.co_evolution_strength
            >= self.config.co_evolution_sync_threshold
        {
            events.push(EmergentEvent {
                event_type: EmergentEventType::CoEvolutionSynchronization,
                timestamp: self.timestamp,
                description: format!(
                    "Co-evolution synchronization reached {:.2}",
                    self.current_environmental_emergence.co_evolution_strength
                ),
                magnitude: self.current_environmental_emergence.co_evolution_strength,
                affected_entities: entities.keys().cloned().collect(),
                system_state_before: self.current_system_emergence.clone(),
                system_state_after: self.current_system_emergence.clone(),
                environmental_state_before: Some(self.current_environmental_emergence.clone()),
                environmental_state_after: Some(self.current_environmental_emergence.clone()),
            });
        }

        // Add events to history
        for event in events.iter() {
            self.history.add_event(event.clone());
        }

        events
    }

    /// Calculate emergence statistics
    pub fn calculate_statistics(&self) -> EmergenceStatistics {
        let mut stats = EmergenceStatistics::default();

        if self.history.system_history.is_empty() {
            return stats;
        }

        // Calculate averages
        let avg_coherence: f64 = self
            .history
            .system_history
            .iter()
            .map(|s| s.global_coherence)
            .sum::<f64>()
            / self.history.system_history.len() as f64;

        let peak_coherence = self
            .history
            .system_history
            .iter()
            .map(|s| s.global_coherence)
            .fold(0.0_f64, f64::max);

        let avg_collective = self
            .history
            .system_history
            .iter()
            .map(|s| s.collective_consciousness_strength)
            .sum::<f64>()
            / self.history.system_history.len() as f64;

        stats.avg_global_coherence = avg_coherence;
        stats.peak_global_coherence = peak_coherence;
        stats.avg_collective_consciousness = avg_collective;

        // Calculate complexity growth rate
        if self.history.system_history.len() >= 2 {
            let first_complexity = self
                .history
                .system_history
                .first()
                .unwrap()
                .system_complexity;
            let last_complexity = self
                .history
                .system_history
                .last()
                .unwrap()
                .system_complexity;
            stats.complexity_growth_rate =
                (last_complexity - first_complexity) / self.history.system_history.len() as f64;
        }

        // Count events by type
        stats.total_events = self.history.events.len();
        for event in &self.history.events {
            *stats
                .events_by_type
                .entry(event.event_type.clone())
                .or_insert(0) += 1;
        }

        // Final environmental emergence
        if let Some(env) = self.history.environmental_history.last() {
            stats.final_environment_emergence = env.environment_emergence_level;
            stats.final_co_evolution_strength = env.co_evolution_strength;
        }

        stats
    }

    /// Increment timestamp
    pub fn increment_timestamp(&mut self) {
        self.timestamp += 1;
    }

    /// Get current timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Get history
    pub fn history(&self) -> &EmergenceHistory {
        &self.history
    }

    /// Get current system emergence
    pub fn current_system_emergence(&self) -> &SystemEmergence {
        &self.current_system_emergence
    }

    /// Get current environmental emergence
    pub fn current_environmental_emergence(&self) -> &EnvironmentalEmergence {
        &self.current_environmental_emergence
    }
}
