// Inter-Scale Interactions Manager
//
// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 4:
// "Implement interactions between entities at different scales"
//
// This module implements:
// 1. Composition interactions - how entities at different scales interact through composition
// 2. Holographic interactions - archetype similarity, resonant/entangled/harmonic connections
// 3. Collective influence - collective entities influence members, bidirectional influence, co-evolution
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "The holographic principle ensures that each entity contains the ENTIRE spectrum"
// "Each scale contains the pattern of the whole"
// "Universal, galactic, and individual scales are holographically related"

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Interaction type between entities at different scales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    /// Composition interaction (entity is composed of another entity)
    Composition,
    /// Resonant interaction (archetype similarity creates resonance)
    Resonant,
    /// Entangled interaction (quantum entanglement across scales)
    Entangled,
    /// Harmonic interaction (harmonic relationship between scales)
    Harmonic,
    /// Environmental influence (environment affects entity)
    Environmental,
    /// Collective influence (collective affects member)
    Collective,
    /// Member influence (member affects collective)
    Member,
}

/// Interaction strength (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct InteractionStrength(pub f64);

impl InteractionStrength {
    pub fn zero() -> Self {
        InteractionStrength(0.0)
    }

    pub fn one() -> Self {
        InteractionStrength(1.0)
    }

    pub fn is_significant(&self) -> bool {
        self.0 > 0.1
    }
}

/// Interaction between two entities
#[derive(Debug, Clone)]
pub struct EntityInteraction {
    /// Source entity ID
    pub source_id: EntityId,
    /// Target entity ID
    pub target_id: EntityId,
    /// Interaction type
    pub interaction_type: InteractionType,
    /// Interaction strength
    pub strength: InteractionStrength,
    /// Timestamp of interaction
    pub timestamp: u64,
    /// Direction (true = source affects target, false = bidirectional)
    pub direction: bool,
}

/// Composition interaction effect
#[derive(Debug, Clone)]
pub struct CompositionEffect {
    /// Entity ID of the composite entity
    pub composite_id: EntityId,
    /// Entity ID of the component entity
    pub component_id: EntityId,
    /// Influence of component on composite (0.0 to 1.0)
    pub influence: f64,
    /// Stability contribution from this component (0.0 to 1.0)
    pub stability_contribution: f64,
    /// Energy contribution from this component (0.0 to 1.0)
    pub energy_contribution: f64,
}

/// Holographic similarity between entities
#[derive(Debug, Clone)]
pub struct HolographicSimilarity {
    /// First entity ID
    pub entity_id_1: EntityId,
    /// Second entity ID
    pub entity_id_2: EntityId,
    /// Archetype similarity (0.0 to 1.0)
    pub archetype_similarity: f64,
    /// Spectrum similarity (0.0 to 1.0)
    pub spectrum_similarity: f64,
    /// Holographic encoding similarity (0.0 to 1.0)
    pub holographic_similarity: f64,
    /// Overall similarity (weighted average)
    pub overall_similarity: f64,
}

/// Holographic connection types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HolographicConnectionType {
    /// Resonant connection (archetype-based resonance)
    Resonant,
    /// Entangled connection (quantum entanglement)
    Entangled,
    /// Harmonic connection (harmonic relationship)
    Harmonic,
}

/// Holographic connection between entities
#[derive(Debug, Clone)]
pub struct HolographicConnection {
    /// First entity ID
    pub entity_id_1: EntityId,
    /// Second entity ID
    pub entity_id_2: EntityId,
    /// Connection type
    pub connection_type: HolographicConnectionType,
    /// Connection strength (0.0 to 1.0)
    pub strength: f64,
    /// Archetype alignment (0.0 to 1.0)
    pub archetype_alignment: f64,
    /// Frequency alignment (0.0 to 1.0)
    pub frequency_alignment: f64,
}

/// Influence from collective to member or vice versa
#[derive(Debug, Clone)]
pub struct CollectiveInfluence {
    /// Collective entity ID
    pub collective_id: EntityId,
    /// Member entity ID
    pub member_id: EntityId,
    /// Influence direction (true = collective to member, false = member to collective)
    pub collective_to_member: bool,
    /// Influence strength (0.0 to 1.0)
    pub strength: f64,
    /// Type of influence
    pub influence_type: InterScaleInfluenceType,
}

/// Type of collective influence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterScaleInfluenceType {
    /// Consciousness influence
    Consciousness,
    /// Energy influence
    Energy,
    /// Coherence influence
    Coherence,
    /// Evolution influence
    Evolution,
}

/// Co-evolution tracking
#[derive(Debug, Clone)]
pub struct CoEvolutionTracking {
    /// Entity ID
    pub entity_id: EntityId,
    /// Related entities (co-evolving with)
    pub related_entities: Vec<EntityId>,
    /// Co-evolution rate (how fast they co-evolve)
    pub co_evolution_rate: f64,
    /// Alignment score (how aligned their evolution is)
    pub alignment_score: f64,
}

/// Inter-Scale Interaction Manager
///
/// Manages interactions between entities at different scales.
///
/// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
/// "Implement interactions between entities at different scales"
/// "How do quantum particles affect atoms?"
/// "How do atoms affect molecules?"
/// "How do molecules affect cells?"
/// "How do cells affect organisms?"
/// "How do organisms affect beings?"
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The holographic principle ensures that each entity contains the ENTIRE spectrum"
/// "Each scale contains the pattern of the whole"
pub struct InterScaleInteractionManager {
    /// All interactions between entities
    pub interactions: HashMap<(EntityId, EntityId), EntityInteraction>,

    /// Composition effects (component → composite)
    pub composition_effects: HashMap<(EntityId, EntityId), CompositionEffect>,

    /// Holographic similarities between entities
    pub holographic_similarities: HashMap<(EntityId, EntityId), HolographicSimilarity>,

    /// Holographic connections between entities
    pub holographic_connections: Vec<HolographicConnection>,

    /// Collective influences
    pub collective_influences: Vec<CollectiveInfluence>,

    /// Co-evolution tracking
    pub co_evolution_tracking: Vec<CoEvolutionTracking>,

    /// Current timestamp
    pub current_timestamp: u64,
}

impl Default for InterScaleInteractionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InterScaleInteractionManager {
    /// Create a new inter-scale interaction manager
    pub fn new() -> Self {
        InterScaleInteractionManager {
            interactions: HashMap::new(),
            composition_effects: HashMap::new(),
            holographic_similarities: HashMap::new(),
            holographic_connections: Vec::new(),
            collective_influences: Vec::new(),
            co_evolution_tracking: Vec::new(),
            current_timestamp: 0,
        }
    }

    /// Update timestamp
    pub fn increment_timestamp(&mut self) {
        self.current_timestamp += 1;
    }

    /// Step 4.1: Process composition interactions
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "How do quantum particles affect atoms?"
    /// "How do atoms affect molecules?"
    /// "How do molecules affect cells?"
    /// "How do cells affect organisms?"
    /// "How do organisms affect beings?"
    ///
    /// This method calculates how component entities influence their composite entities.
    pub fn process_composition_interactions(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> Vec<CompositionEffect> {
        let mut effects = Vec::new();

        for (entity_id, entity) in entities.iter() {
            // Get all components of this entity
            let components = entity.composition();

            for component_id in components.iter() {
                if let Some(component_entity) = entities.get(component_id) {
                    // Calculate composition effect
                    let effect = self.calculate_composition_effect(
                        entity_id,
                        component_id,
                        entity,
                        component_entity,
                    );

                    // Store the effect
                    self.composition_effects
                        .insert((component_id.clone(), entity_id.clone()), effect.clone());

                    // Create interaction record
                    self.record_interaction(
                        component_id,
                        entity_id,
                        InteractionType::Composition,
                        InteractionStrength(effect.influence),
                        false, // bidirectional
                    );

                    effects.push(effect);
                }
            }
        }

        effects
    }

    /// Calculate composition effect from component to composite
    fn calculate_composition_effect(
        &self,
        composite_id: &EntityId,
        component_id: &EntityId,
        composite: &SubSubLogos,
        component: &SubSubLogos,
    ) -> CompositionEffect {
        // Calculate influence based on component vibrational energy and holographic alignment
        let component_energy = component.current_state.vibrational_state.amplitude;
        let composite_capacity = composite.current_state.vibrational_state.amplitude;

        // Influence is proportional to component energy relative to composite capacity
        let influence = if composite_capacity > 0.0 {
            (component_energy / composite_capacity).min(1.0)
        } else {
            0.0
        };

        // Stability contribution based on component coherence
        let stability_contribution = component.current_state.vibrational_state.coherence;

        // Energy contribution based on component vibrational amplitude
        let energy_contribution = component_energy.min(1.0);

        CompositionEffect {
            composite_id: composite_id.clone(),
            component_id: component_id.clone(),
            influence,
            stability_contribution,
            energy_contribution,
        }
    }

    /// Step 4.2: Calculate holographic similarity between two entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each scale contains the pattern of the whole"
    /// "Universal, galactic, and individual scales are holographically related"
    ///
    /// This calculates how holographically similar two entities are based on:
    /// - Archetype similarity (shared archetypical patterns)
    /// - Spectrum similarity (spectrum ratio alignment)
    /// - Holographic encoding similarity (encoding pattern similarity)
    pub fn calculate_holographic_similarity(
        &mut self,
        entity_id_1: &EntityId,
        entity_id_2: &EntityId,
        entity_1: &SubSubLogos,
        entity_2: &SubSubLogos,
    ) -> HolographicSimilarity {
        // Calculate archetype similarity
        let archetype_similarity = self.calculate_archetype_similarity(entity_1, entity_2);

        // Calculate spectrum similarity
        let spectrum_similarity = self.calculate_spectrum_similarity(entity_1, entity_2);

        // Calculate holographic encoding similarity
        let holographic_similarity =
            self.calculate_holographic_encoding_similarity(entity_1, entity_2);

        // Overall similarity is weighted average
        let overall_similarity =
            (archetype_similarity * 0.4 + spectrum_similarity * 0.3 + holographic_similarity * 0.3)
                .min(1.0);

        let similarity = HolographicSimilarity {
            entity_id_1: entity_id_1.clone(),
            entity_id_2: entity_id_2.clone(),
            archetype_similarity,
            spectrum_similarity,
            holographic_similarity,
            overall_similarity,
        };

        // Store the similarity
        self.holographic_similarities.insert(
            (entity_id_1.clone(), entity_id_2.clone()),
            similarity.clone(),
        );
        self.holographic_similarities.insert(
            (entity_id_2.clone(), entity_id_1.clone()),
            similarity.clone(),
        );

        similarity
    }

    /// Calculate archetype similarity between two entities
    fn calculate_archetype_similarity(
        &self,
        entity_1: &SubSubLogos,
        entity_2: &SubSubLogos,
    ) -> f64 {
        // Compare archetypical mind patterns
        let archetypes_1 = &entity_1.archetypical_mind.archetypes;
        let archetypes_2 = &entity_2.archetypical_mind.archetypes;

        if archetypes_1.is_empty() || archetypes_2.is_empty() {
            return 0.0;
        }

        // Calculate similarity based on shared archetype values
        let mut similarity = 0.0;
        let count = archetypes_1.len().min(archetypes_2.len());

        for _i in 0..count {
            // For simplicity, use the archetype count as a similarity metric
            // In a more sophisticated implementation, we would compare specific archetype values
            similarity += 0.5; // Base similarity
        }

        similarity / count as f64
    }

    /// Calculate spectrum similarity between two entities
    fn calculate_spectrum_similarity(&self, entity_1: &SubSubLogos, entity_2: &SubSubLogos) -> f64 {
        // Compare spectrum ratios
        let ratio_1 = entity_1.spectrum_access.ratio;
        let ratio_2 = entity_2.spectrum_access.ratio;

        // Similarity is inverse of ratio difference
        let ratio_diff = (ratio_1 - ratio_2).abs();
        1.0 - ratio_diff.min(1.0)
    }

    /// Calculate holographic encoding similarity between two entities
    fn calculate_holographic_encoding_similarity(
        &self,
        entity_1: &SubSubLogos,
        entity_2: &SubSubLogos,
    ) -> f64 {
        // Compare vibrational frequencies as a proxy for holographic encoding similarity
        let freq_1 = entity_1.current_state.vibrational_state.frequency;
        let freq_2 = entity_2.current_state.vibrational_state.frequency;

        let freq_diff = (freq_1 - freq_2).abs();
        let freq_similarity = 1.0 - freq_diff.min(1.0);

        // Compare coherence as a proxy for encoding quality
        let coherence_1 = entity_1.current_state.vibrational_state.coherence;
        let coherence_2 = entity_2.current_state.vibrational_state.coherence;
        let coherence_diff = (coherence_1 - coherence_2).abs();
        let coherence_similarity = 1.0 - coherence_diff.min(1.0);

        (freq_similarity + coherence_similarity) / 2.0
    }

    /// Step 4.2: Create holographic connections between entities
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Create resonant connections"
    /// "Create entangled connections"
    /// "Create harmonic connections"
    ///
    /// This creates holographic connections based on archetype similarity.
    pub fn create_holographic_connections(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
        similarity_threshold: f64,
    ) -> Vec<HolographicConnection> {
        let mut connections = Vec::new();
        let entity_ids: Vec<&EntityId> = entities.keys().collect();

        // Compare all pairs of entities
        for i in 0..entity_ids.len() {
            for j in (i + 1)..entity_ids.len() {
                if let (Some(entity_1), Some(entity_2)) =
                    (entities.get(entity_ids[i]), entities.get(entity_ids[j]))
                {
                    // Calculate holographic similarity
                    let similarity = self.calculate_holographic_similarity(
                        entity_ids[i],
                        entity_ids[j],
                        entity_1,
                        entity_2,
                    );

                    // Only create connections if similarity exceeds threshold
                    if similarity.overall_similarity >= similarity_threshold {
                        // Determine connection type based on similarity level
                        let connection_type = if similarity.overall_similarity >= 0.9 {
                            HolographicConnectionType::Entangled
                        } else if similarity.overall_similarity >= 0.7 {
                            HolographicConnectionType::Resonant
                        } else {
                            HolographicConnectionType::Harmonic
                        };

                        // Calculate connection strength
                        let strength = similarity.overall_similarity;

                        // Calculate archetype alignment
                        let archetype_alignment = similarity.archetype_similarity;

                        // Calculate frequency alignment
                        let frequency_alignment = similarity.spectrum_similarity;

                        let connection = HolographicConnection {
                            entity_id_1: entity_ids[i].clone(),
                            entity_id_2: entity_ids[j].clone(),
                            connection_type,
                            strength,
                            archetype_alignment,
                            frequency_alignment,
                        };

                        // Store the connection
                        self.holographic_connections.push(connection.clone());

                        // Record interaction
                        let interaction_type = match connection_type {
                            HolographicConnectionType::Resonant => InteractionType::Resonant,
                            HolographicConnectionType::Entangled => InteractionType::Entangled,
                            HolographicConnectionType::Harmonic => InteractionType::Harmonic,
                        };

                        self.record_interaction(
                            entity_ids[i],
                            entity_ids[j],
                            interaction_type,
                            InteractionStrength(strength),
                            true, // bidirectional
                        );

                        connections.push(connection);
                    }
                }
            }
        }

        connections
    }

    /// Step 4.3: Process collective influence
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Collective entities influence their members"
    /// "Individual entities influence their collectives"
    /// "Co-evolution of different scales"
    ///
    /// This calculates how collective entities influence their members and vice versa.
    pub fn process_collective_influence(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> Vec<CollectiveInfluence> {
        let mut influences = Vec::new();

        for (entity_id, entity) in entities.iter() {
            // Process collective → member influence
            if entity.entity_type.is_collective() {
                let children = entity.children();

                for child_id in children.iter() {
                    if let Some(child_entity) = entities.get(child_id) {
                        // Calculate collective → member influence
                        let influence = self.calculate_collective_to_member_influence(
                            entity_id,
                            child_id,
                            entity,
                            child_entity,
                        );

                        self.collective_influences.push(influence.clone());

                        // Record interaction
                        self.record_interaction(
                            entity_id,
                            child_id,
                            InteractionType::Collective,
                            InteractionStrength(influence.strength),
                            false, // directional
                        );

                        influences.push(influence);
                    }
                }
            }

            // Process member → collective influence
            if let Some(parent_id) = entity.parent() {
                if let Some(parent_entity) = entities.get(parent_id) {
                    if parent_entity.entity_type.is_collective() {
                        // Calculate member → collective influence
                        let influence = self.calculate_member_to_collective_influence(
                            entity_id,
                            parent_id,
                            entity,
                            parent_entity,
                        );

                        self.collective_influences.push(influence.clone());

                        // Record interaction
                        self.record_interaction(
                            entity_id,
                            parent_id,
                            InteractionType::Member,
                            InteractionStrength(influence.strength),
                            false, // directional
                        );

                        influences.push(influence);
                    }
                }
            }
        }

        influences
    }

    /// Calculate collective → member influence
    fn calculate_collective_to_member_influence(
        &self,
        collective_id: &EntityId,
        member_id: &EntityId,
        collective: &SubSubLogos,
        member: &SubSubLogos,
    ) -> CollectiveInfluence {
        // Influence based on collective consciousness strength
        // Use a simplified calculation based on coherence and amplitude
        let collective_strength = collective.current_state.vibrational_state.coherence
            * collective.current_state.vibrational_state.amplitude;
        let member_receptivity = member.current_state.vibrational_state.coherence;

        let strength = collective_strength * member_receptivity;

        CollectiveInfluence {
            collective_id: collective_id.clone(),
            member_id: member_id.clone(),
            collective_to_member: true,
            strength,
            influence_type: InterScaleInfluenceType::Consciousness,
        }
    }

    /// Calculate member → collective influence
    fn calculate_member_to_collective_influence(
        &self,
        member_id: &EntityId,
        collective_id: &EntityId,
        member: &SubSubLogos,
        collective: &SubSubLogos,
    ) -> CollectiveInfluence {
        // Influence based on member's contribution to collective
        let member_contribution = member.current_state.vibrational_state.amplitude;
        let collective_capacity = collective.current_state.vibrational_state.amplitude;

        let strength = if collective_capacity > 0.0 {
            (member_contribution / collective_capacity).min(1.0)
        } else {
            0.0
        };

        CollectiveInfluence {
            collective_id: collective_id.clone(),
            member_id: member_id.clone(),
            collective_to_member: false,
            strength,
            influence_type: InterScaleInfluenceType::Energy,
        }
    }

    /// Step 4.3: Track co-evolution between entities
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Co-evolution of different scales"
    ///
    /// This tracks which entities are co-evolving and at what rate.
    pub fn track_co_evolution(
        &mut self,
        entities: &HashMap<EntityId, SubSubLogos>,
    ) -> Vec<CoEvolutionTracking> {
        let mut co_evolution = Vec::new();

        for (entity_id, entity) in entities.iter() {
            // Find related entities (in composition hierarchy or holographic connections)
            let mut related_entities = Vec::new();

            // Add components and composites
            for component_id in entity.composition().iter() {
                related_entities.push(component_id.clone());
            }

            if let Some(parent_id) = entity.parent() {
                related_entities.push(parent_id.clone());
            }

            for child_id in entity.children().iter() {
                related_entities.push(child_id.clone());
            }

            // Add holographically connected entities
            for connection in &self.holographic_connections {
                if connection.entity_id_1 == *entity_id {
                    related_entities.push(connection.entity_id_2.clone());
                } else if connection.entity_id_2 == *entity_id {
                    related_entities.push(connection.entity_id_1.clone());
                }
            }

            // Calculate co-evolution rate based on related entities
            let co_evolution_rate = if related_entities.is_empty() {
                0.0
            } else {
                // Use entity's evolutionary rate as base
                entity.evolutionary_rate * related_entities.len() as f64 * 0.1
            };

            // Calculate alignment score based on holographic similarities
            let mut alignment_sum = 0.0;
            let mut alignment_count = 0;

            for related_id in related_entities.iter() {
                if let Some(similarity) = self
                    .holographic_similarities
                    .get(&(entity_id.clone(), related_id.clone()))
                {
                    alignment_sum += similarity.overall_similarity;
                    alignment_count += 1;
                }
            }

            let alignment_score = if alignment_count > 0 {
                alignment_sum / alignment_count as f64
            } else {
                0.0
            };

            let tracking = CoEvolutionTracking {
                entity_id: entity_id.clone(),
                related_entities,
                co_evolution_rate,
                alignment_score,
            };

            co_evolution.push(tracking);
        }

        self.co_evolution_tracking = co_evolution.clone();
        co_evolution
    }

    /// Record an interaction between two entities
    fn record_interaction(
        &mut self,
        source_id: &EntityId,
        target_id: &EntityId,
        interaction_type: InteractionType,
        strength: InteractionStrength,
        direction: bool,
    ) {
        let interaction = EntityInteraction {
            source_id: source_id.clone(),
            target_id: target_id.clone(),
            interaction_type,
            strength,
            timestamp: self.current_timestamp,
            direction,
        };

        self.interactions
            .insert((source_id.clone(), target_id.clone()), interaction);
    }

    /// Get all interactions for a specific entity
    pub fn get_entity_interactions(&self, entity_id: &EntityId) -> Vec<&EntityInteraction> {
        self.interactions
            .iter()
            .filter(|((source, target), _)| source == entity_id || target == entity_id)
            .map(|(_, interaction)| interaction)
            .collect()
    }

    /// Get composition effects for a specific entity
    pub fn get_composition_effects(&self, entity_id: &EntityId) -> Vec<&CompositionEffect> {
        self.composition_effects
            .iter()
            .filter(|((component, composite), _)| component == entity_id || composite == entity_id)
            .map(|(_, effect)| effect)
            .collect()
    }

    /// Get holographic connections for a specific entity
    pub fn get_holographic_connections(&self, entity_id: &EntityId) -> Vec<&HolographicConnection> {
        self.holographic_connections
            .iter()
            .filter(|conn| &conn.entity_id_1 == entity_id || &conn.entity_id_2 == entity_id)
            .collect()
    }

    /// Get collective influences for a specific entity
    pub fn get_collective_influences(&self, entity_id: &EntityId) -> Vec<&CollectiveInfluence> {
        self.collective_influences
            .iter()
            .filter(|inf| &inf.collective_id == entity_id || &inf.member_id == entity_id)
            .collect()
    }

    /// Get interaction statistics
    pub fn get_interaction_statistics(&self) -> InteractionStatistics {
        let total_interactions = self.interactions.len();
        let total_composition_effects = self.composition_effects.len();
        let total_holographic_connections = self.holographic_connections.len();
        let total_collective_influences = self.collective_influences.len();
        let total_co_evolution_tracking = self.co_evolution_tracking.len();

        // Count interactions by type
        let mut interaction_counts: HashMap<InteractionType, usize> = HashMap::new();
        for interaction in self.interactions.values() {
            *interaction_counts
                .entry(interaction.interaction_type)
                .or_insert(0) += 1;
        }

        // Count holographic connections by type
        let mut connection_counts: HashMap<HolographicConnectionType, usize> = HashMap::new();
        for connection in &self.holographic_connections {
            *connection_counts
                .entry(connection.connection_type)
                .or_insert(0) += 1;
        }

        InteractionStatistics {
            total_interactions,
            total_composition_effects,
            total_holographic_connections,
            total_collective_influences,
            total_co_evolution_tracking,
            interaction_counts,
            connection_counts,
        }
    }
}

/// Interaction statistics summary
#[derive(Debug, Clone)]
pub struct InteractionStatistics {
    pub total_interactions: usize,
    pub total_composition_effects: usize,
    pub total_holographic_connections: usize,
    pub total_collective_influences: usize,
    pub total_co_evolution_tracking: usize,
    pub interaction_counts: HashMap<InteractionType, usize>,
    pub connection_counts: HashMap<HolographicConnectionType, usize>,
}
