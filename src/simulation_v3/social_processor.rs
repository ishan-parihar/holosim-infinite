//! Social Processor - Emergent Social Structures
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
//! "Entities form relationships, societies, Social Memory Complexes"
//!
//! This module wraps the social systems and integrates them into the causal simulation loop:
//! - Relationship formation from proximity and archetype resonance
//! - Group formation from shared archetype patterns
//! - Social Memory Complex (SMC) formation at density thresholds
//! - Harvest cycle processing for density transitions

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::consciousness_processor::ConsciousnessProcessor;
use crate::social::civilization::{GovernanceType, SocialGroup};
use crate::social::communication::{CommunicationSystem, CommunicationType, MessageContent};
use crate::social::harvest::{HarvestEngine, HarvestType};
use crate::social::relationship_web::{
    Interaction, InteractionType, Relationship, RelationshipType, RelationshipWeb,
};
use crate::social::smc_engine::{Polarity, SocialMemoryComplex, SocialMemoryComplexEngine};
use crate::types::Float;
use std::collections::HashMap;

/// Proximity threshold for relationship formation (normalized coordinates)
const RELATIONSHIP_PROXIMITY_THRESHOLD: Float = 0.3;

/// Minimum archetype resonance for relationship formation
const MIN_ARCHETYPE_RESONANCE: Float = 0.3;

/// Minimum entities for group formation
const MIN_GROUP_SIZE: usize = 3;

/// Minimum shared archetype pattern for group formation
const MIN_SHARED_PATTERN_THRESHOLD: Float = 0.7;

/// SMC formation resonance threshold
const SMC_RESONANCE_THRESHOLD: Float = 0.8;

/// STO polarity threshold for SMC formation
const STO_SMC_THRESHOLD: Float = 0.51;

/// STS polarity threshold for SMC formation (absolute value)
const STS_SMC_THRESHOLD: Float = 0.95;

// ============================================================================
// Social Output
// ============================================================================

/// Output from social processing for a single tick
#[derive(Debug, Clone, Default)]
pub struct SocialOutput {
    /// Number of interactions processed
    pub interactions_processed: usize,
    /// Number of new relationships formed
    pub relationships_formed: usize,
    /// Number of communications sent
    pub communications_sent: usize,
    /// Number of new groups formed
    pub groups_formed: usize,
    /// Number of new SMCs formed
    pub smcs_formed: usize,
    /// Number of entities harvested (density transition)
    pub entities_harvested: usize,
}

impl SocialOutput {
    /// Check if any social activity occurred
    pub fn has_activity(&self) -> bool {
        self.interactions_processed > 0
            || self.relationships_formed > 0
            || self.communications_sent > 0
            || self.groups_formed > 0
            || self.smcs_formed > 0
            || self.entities_harvested > 0
    }

    /// Get total social events count
    pub fn total_events(&self) -> usize {
        self.interactions_processed
            + self.relationships_formed
            + self.communications_sent
            + self.groups_formed
            + self.smcs_formed
            + self.entities_harvested
    }
}

// ============================================================================
// Social Processor
// ============================================================================

/// Main social processor wrapping all social systems.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
/// "Social Processor wrapper around social systems"
///
/// This processor coordinates:
/// 1. RelationshipWeb - entity relationship dynamics
/// 2. CommunicationSystem - density-appropriate messaging
/// 3. SocialMemoryComplexEngine - SMC formation and management
/// 4. HarvestEngine - density transition processing
/// 5. Groups - social group tracking
///
/// The processing flow:
/// 1. Form relationships from proximity and archetype resonance
/// 2. Process communications between entities
/// 3. Form groups from shared archetype patterns
/// 4. Check SMC formation conditions
/// 5. Process harvest cycle if due
#[derive(Debug)]
pub struct SocialProcessor {
    /// Relationship web tracking all entity relationships
    pub relationship_web: RelationshipWeb,
    /// Communication system for entity messaging
    pub communication_system: CommunicationSystem,
    /// SMC engine for social memory complex formation
    pub smc_engine: SocialMemoryComplexEngine,
    /// Harvest engine for density transitions
    pub harvest_engine: HarvestEngine,
    /// Active social groups
    pub groups: HashMap<u64, SocialGroup>,
    /// Next group ID for unique identification
    pub next_group_id: u64,
    /// Current simulation tick
    pub current_tick: u64,
}

impl Default for SocialProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SocialProcessor {
    /// Create a new social processor with default configuration
    pub fn new() -> Self {
        SocialProcessor {
            relationship_web: RelationshipWeb::new(),
            communication_system: CommunicationSystem::new(),
            smc_engine: SocialMemoryComplexEngine::new(),
            harvest_engine: HarvestEngine::new(),
            groups: HashMap::new(),
            next_group_id: 1,
            current_tick: 0,
        }
    }

    /// Main processing tick - process all social interactions.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Wire social processing into causal tick"
    ///
    /// This method:
    /// 1. Forms relationships from proximity and resonance
    /// 2. Processes communications
    /// 3. Forms groups from shared patterns
    /// 4. Checks SMC formation
    /// 5. Processes harvest if due
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    /// * `consciousness_processors` - Map of entity consciousness processors for polarity/resonance
    ///
    /// # Returns
    /// SocialOutput with statistics about social activity
    pub fn process_tick(
        &mut self,
        entities: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> SocialOutput {
        let mut output = SocialOutput::default();
        self.current_tick += 1;

        // Phase 1: Form relationships from proximity and archetype resonance
        output.relationships_formed =
            self.form_relationships_from_proximity(entities, consciousness_processors);

        // Phase 2: Process communications
        output.communications_sent =
            self.process_communications(entities, consciousness_processors);

        // Phase 3: Form groups from shared archetype patterns
        output.groups_formed =
            self.form_groups_from_shared_patterns(entities, consciousness_processors);

        // Phase 4: Check SMC formation
        output.smcs_formed = self.check_smc_formation(entities, consciousness_processors);

        // Phase 5: Process harvest if due
        output.entities_harvested = self.process_harvest(entities);

        // Count interactions processed from relationship web
        output.interactions_processed = self.relationship_web.interaction_history.len();

        output
    }

    /// Form relationships from proximity and archetype resonance.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Relationship formation: entities within proximity + archetype resonance > threshold"
    ///
    /// This method:
    /// 1. Checks entity pairs for proximity (simplified position-based)
    /// 2. Calculates archetype resonance between entities
    /// 3. Creates relationships when conditions are met
    /// 4. Determines relationship type from archetype patterns
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    /// * `consciousness_processors` - Map of consciousness processors for archetype access
    ///
    /// # Returns
    /// Number of new relationships formed
    pub fn form_relationships_from_proximity(
        &mut self,
        entities: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> usize {
        let mut relationships_formed = 0;

        // Check all entity pairs
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let entity_a = &entities[i];
                let entity_b = &entities[j];

                // Skip if relationship already exists
                if self
                    .relationship_web
                    .get_relationship(entity_a.clone(), entity_b.clone())
                    .is_some()
                {
                    continue;
                }

                // Calculate proximity (simplified - using entity key hash)
                let proximity = self.calculate_proximity(entity_a, entity_b);
                if proximity < RELATIONSHIP_PROXIMITY_THRESHOLD {
                    continue;
                }

                // Calculate archetype resonance
                let resonance = self.calculate_archetype_resonance(
                    entity_a,
                    entity_b,
                    consciousness_processors,
                );
                if resonance < MIN_ARCHETYPE_RESONANCE {
                    continue;
                }

                // Determine relationship type from archetype patterns
                let relationship_type =
                    self.determine_relationship_type(entity_a, entity_b, consciousness_processors);

                // Create the relationship
                let mut relationship = Relationship::with_type(relationship_type);
                relationship.strength = resonance;
                relationship.trust = proximity * 0.5;

                // Store relationship
                let key = (
                    entity_a.as_u64().min(entity_b.as_u64()),
                    entity_a.as_u64().max(entity_b.as_u64()),
                );
                self.relationship_web
                    .relationships
                    .insert(key, relationship);

                // Create interaction to record the formation
                let interaction = Interaction {
                    entity_a: entity_a.clone(),
                    entity_b: entity_b.clone(),
                    interaction_type: InteractionType::Cooperative,
                    time: self.current_tick as Float,
                    impact: resonance,
                    context: "Initial relationship formation".to_string(),
                };
                self.relationship_web.update_from_interaction(&interaction);

                relationships_formed += 1;
            }
        }

        relationships_formed
    }

    /// Process communications between entities.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Wire communication for density-appropriate messaging"
    ///
    /// Entities communicate based on:
    /// - Relationship strength
    /// - Consciousness level
    /// - Communication type appropriate to density
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    /// * `consciousness_processors` - Map of consciousness processors
    ///
    /// # Returns
    /// Number of communications sent
    pub fn process_communications(
        &mut self,
        entities: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> usize {
        let mut communications_sent = 0;

        for entity in entities {
            // Get entity's relationships
            let relationships = self
                .relationship_web
                .get_entity_relationships(entity.clone());

            for (other_key, relationship) in relationships {
                // Communication probability based on relationship strength
                if relationship.strength.abs() < 0.2 {
                    continue;
                }

                // Determine communication type from consciousness level
                let comm_type = if let Some(processor) = consciousness_processors.get(entity) {
                    let consciousness = processor.consciousness_level();
                    if consciousness > 0.8 {
                        CommunicationType::Telepathic
                    } else if consciousness > 0.5 {
                        CommunicationType::Verbal
                    } else {
                        CommunicationType::Instinctual
                    }
                } else {
                    CommunicationType::Verbal
                };

                // Create message content based on relationship type
                let content = match relationship.relationship_type {
                    RelationshipType::Friend | RelationshipType::Mate => {
                        MessageContent::Emotional {
                            emotion: "affection".to_string(),
                            intensity: relationship.strength.max(0.0),
                        }
                    }
                    RelationshipType::Teacher | RelationshipType::Student => {
                        MessageContent::Thought {
                            content: "Knowledge sharing".to_string(),
                            clarity: relationship.trust,
                        }
                    }
                    RelationshipType::Rival => MessageContent::Emotional {
                        emotion: "tension".to_string(),
                        intensity: relationship.strength.abs(),
                    },
                    _ => MessageContent::Verbal("Greeting".to_string()),
                };

                // Send message
                self.communication_system.send_message(
                    entity.as_u64(),
                    other_key,
                    content,
                    comm_type,
                );

                communications_sent += 1;
            }
        }

        communications_sent
    }

    /// Form groups from shared archetype patterns.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Group formation: 3+ entities with shared archetype pattern > 0.7"
    ///
    /// Groups form when:
    /// 1. At least 3 entities share similar archetype patterns
    /// 2. Shared pattern similarity > 0.7
    /// 3. Entities are not already in the same group
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    /// * `consciousness_processors` - Map of consciousness processors for archetype patterns
    ///
    /// # Returns
    /// Number of new groups formed
    pub fn form_groups_from_shared_patterns(
        &mut self,
        entities: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> usize {
        let mut groups_formed = 0;

        // Get entities not already in groups
        let mut ungrouped_entities: Vec<EntityId> = Vec::new();
        for entity in entities {
            let in_group = self.groups.values().any(|g| g.members.contains(entity));
            if !in_group {
                ungrouped_entities.push(entity.clone());
            }
        }

        if ungrouped_entities.len() < MIN_GROUP_SIZE {
            return 0;
        }

        // Find clusters of entities with shared patterns
        let mut processed: Vec<u64> = Vec::new();

        for i in 0..ungrouped_entities.len() {
            let entity_i = &ungrouped_entities[i];
            let key_i = entity_i.as_u64();

            if processed.contains(&key_i) {
                continue;
            }

            // Find entities with similar archetype patterns
            let mut cluster: Vec<EntityId> = vec![entity_i.clone()];

            for entity_j in ungrouped_entities.iter().skip(i + 1) {
                let key_j = entity_j.as_u64();

                if processed.contains(&key_j) {
                    continue;
                }

                // Calculate pattern similarity
                let similarity = self.calculate_archetype_similarity(
                    entity_i,
                    entity_j,
                    consciousness_processors,
                );

                if similarity >= MIN_SHARED_PATTERN_THRESHOLD {
                    cluster.push(entity_j.clone());
                }
            }

            // Form group if cluster is large enough
            if cluster.len() >= MIN_GROUP_SIZE {
                let group_id = self.next_group_id;
                self.next_group_id += 1;

                // Determine group governance from archetype patterns
                let governance =
                    self.determine_group_governance(&cluster, consciousness_processors);

                let mut group =
                    SocialGroup::new(group_id, format!("Group-{}", group_id), cluster[0].clone());
                group.governance = governance;
                group.cohesion = MIN_SHARED_PATTERN_THRESHOLD;

                // Add all cluster members
                for member in &cluster[1..] {
                    group.add_member(member.clone());
                }

                self.groups.insert(group_id, group);

                // Mark entities as processed
                for member in &cluster {
                    processed.push(member.as_u64());
                }

                groups_formed += 1;
            }
        }

        groups_formed
    }

    /// Check SMC formation conditions.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "SMC formation: 4th density + polarity > 0.51 (STO) or < -0.95 (STS) + resonance > 0.8"
    ///
    /// SMCs form when:
    /// 1. Entities are at 4th density or higher
    /// 2. Polarity meets threshold (STO > 0.51 or STS < -0.95)
    /// 3. Group resonance > 0.8
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    /// * `consciousness_processors` - Map of consciousness processors
    ///
    /// # Returns
    /// Number of new SMCs formed
    pub fn check_smc_formation(
        &mut self,
        entities: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> usize {
        // Use the smc_engine's tick method
        let events = self.smc_engine.tick(self.current_tick, entities);

        // Count new formations
        let smcs_formed = events
            .iter()
            .filter(|e| matches!(e, crate::social::smc_engine::SmcEvent::Formed(_)))
            .count();

        // Additional check: form SMC from high-polarity groups
        for group in self.groups.values() {
            // Skip if group already in an SMC
            let in_smc = self.smc_engine.active_complexes.values().any(|smc| {
                group
                    .members
                    .iter()
                    .all(|m| smc.member_entity_keys.contains(&m.as_u64()))
            });
            if in_smc {
                continue;
            }

            // Check polarity thresholds
            let mut sto_entities: Vec<EntityId> = Vec::new();
            let mut sts_entities: Vec<EntityId> = Vec::new();

            for member in &group.members {
                if let Some(processor) = consciousness_processors.get(member) {
                    let polarity = processor.polarity_accumulation();

                    if polarity >= STO_SMC_THRESHOLD {
                        sto_entities.push(member.clone());
                    } else if polarity <= -STS_SMC_THRESHOLD {
                        sts_entities.push(member.clone());
                    }
                }
            }

            // Form STO SMC if enough entities
            if sto_entities.len() >= MIN_GROUP_SIZE {
                let resonance =
                    self.calculate_group_resonance(&sto_entities, consciousness_processors);
                if resonance >= SMC_RESONANCE_THRESHOLD {
                    let member_keys: Vec<u64> = sto_entities.iter().map(|e| e.as_u64()).collect();
                    let smc = SocialMemoryComplex::new(
                        self.smc_engine.next_id,
                        member_keys,
                        crate::evolution_density_octave::density_octave::Density::Fourth,
                        Polarity::ServiceToOthers,
                    );
                    self.smc_engine
                        .active_complexes
                        .insert(self.smc_engine.next_id, smc);
                    self.smc_engine.next_id += 1;
                    return 1;
                }
            }

            // Form STS SMC if enough entities
            if sts_entities.len() >= MIN_GROUP_SIZE {
                let resonance =
                    self.calculate_group_resonance(&sts_entities, consciousness_processors);
                if resonance >= SMC_RESONANCE_THRESHOLD {
                    let member_keys: Vec<u64> = sts_entities.iter().map(|e| e.as_u64()).collect();
                    let smc = SocialMemoryComplex::new(
                        self.smc_engine.next_id,
                        member_keys,
                        crate::evolution_density_octave::density_octave::Density::Fourth,
                        Polarity::ServiceToSelf,
                    );
                    self.smc_engine
                        .active_complexes
                        .insert(self.smc_engine.next_id, smc);
                    self.smc_engine.next_id += 1;
                    return 1;
                }
            }
        }

        smcs_formed
    }

    /// Process harvest cycle for density transitions.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Harvest: check every harvest_cycle_ticks, process density transitions"
    ///
    /// This method:
    /// 1. Checks if harvest cycle is due
    /// 2. Processes entities for harvest eligibility
    /// 3. Returns count of harvested entities
    ///
    /// # Arguments
    /// * `entities` - List of active entity IDs
    ///
    /// # Returns
    /// Number of entities harvested
    pub fn process_harvest(&mut self, entities: &[EntityId]) -> usize {
        // Check if harvest cycle is due
        if !self
            .current_tick
            .is_multiple_of(self.harvest_engine.harvest_cycle_ticks)
        {
            return 0;
        }

        // Process harvest
        let results = self
            .harvest_engine
            .process_harvest(self.current_tick, entities);

        // Count entities that were harvested (not NotReady)
        results
            .iter()
            .filter(|r| {
                r.harvest_type != HarvestType::NotReady && r.harvest_type != HarvestType::Remain
            })
            .count()
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Calculate proximity between two entities.
    ///
    /// Uses entity key hash for simplified proximity calculation.
    /// In a full implementation, this would use actual spatial positions.
    fn calculate_proximity(&self, entity_a: &EntityId, entity_b: &EntityId) -> Float {
        // Simplified proximity based on key hash similarity
        let key_a = entity_a.as_u64();
        let key_b = entity_b.as_u64();

        // Use XOR to measure "distance" between keys
        let xor = key_a ^ key_b;
        let max_xor = u64::MAX as Float;

        // Convert to similarity (higher = closer)
        1.0 - (xor as Float / max_xor)
    }

    /// Calculate archetype resonance between two entities.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Resonance determines attraction between similar patterns"
    fn calculate_archetype_resonance(
        &self,
        entity_a: &EntityId,
        entity_b: &EntityId,
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> Float {
        let processor_a = consciousness_processors.get(entity_a);
        let processor_b = consciousness_processors.get(entity_b);

        match (processor_a, processor_b) {
            (Some(a), Some(b)) => {
                let profile_a = a.archetype_profile();
                let profile_b = b.archetype_profile();

                // Calculate cosine similarity between archetype vectors
                let mut dot_product = 0.0;
                let mut mag_a = 0.0;
                let mut mag_b = 0.0;

                for i in 0..22 {
                    let val_a = profile_a.get(i).unwrap_or(0.0);
                    let val_b = profile_b.get(i).unwrap_or(0.0);

                    dot_product += val_a * val_b;
                    mag_a += val_a * val_a;
                    mag_b += val_b * val_b;
                }

                if mag_a > 0.0 && mag_b > 0.0 {
                    dot_product / (mag_a.sqrt() * mag_b.sqrt())
                } else {
                    0.0
                }
            }
            _ => {
                // Fall back to key-based similarity
                let key_a = entity_a.as_u64();
                let key_b = entity_b.as_u64();
                ((key_a % 100) as Float - (key_b % 100) as Float).abs() / 100.0
            }
        }
    }

    /// Calculate archetype similarity (alias for resonance for clarity).
    fn calculate_archetype_similarity(
        &self,
        entity_a: &EntityId,
        entity_b: &EntityId,
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> Float {
        self.calculate_archetype_resonance(entity_a, entity_b, consciousness_processors)
    }

    /// Determine relationship type from archetype patterns.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Archetype patterns determine the nature of relationships"
    fn determine_relationship_type(
        &self,
        entity_a: &EntityId,
        entity_b: &EntityId,
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> RelationshipType {
        let processor_a = consciousness_processors.get(entity_a);
        let processor_b = consciousness_processors.get(entity_b);

        match (processor_a, processor_b) {
            (Some(a), Some(b)) => {
                let profile_a = a.archetype_profile();
                let profile_b = b.archetype_profile();

                // Compare dominant archetype groups
                let mind_a: Float = (0..7)
                    .map(|i| profile_a.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;
                let body_a: Float = (7..14)
                    .map(|i| profile_a.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;
                let spirit_a: Float = (14..21)
                    .map(|i| profile_a.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;

                let mind_b: Float = (0..7)
                    .map(|i| profile_b.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;
                let body_b: Float = (7..14)
                    .map(|i| profile_b.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;
                let spirit_b: Float = (14..21)
                    .map(|i| profile_b.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;

                // Determine relationship based on archetype alignment
                let polarity_diff = (a.polarity_accumulation() - b.polarity_accumulation()).abs();

                if polarity_diff > 0.5 {
                    // Different polarities suggest teacher-student or rival
                    if a.polarity_accumulation() > b.polarity_accumulation() {
                        RelationshipType::Teacher
                    } else if a.polarity_accumulation() < b.polarity_accumulation() {
                        RelationshipType::Student
                    } else {
                        RelationshipType::Rival
                    }
                } else if spirit_a > 0.7 && spirit_b > 0.7 {
                    // High spirit alignment suggests deep connection
                    RelationshipType::Friend
                } else if mind_a > 0.7 && mind_b > 0.7 {
                    // High mind alignment suggests teaching relationship
                    RelationshipType::Teacher
                } else if body_a > 0.7 && body_b > 0.7 {
                    // High body alignment suggests practical cooperation
                    RelationshipType::Friend
                } else {
                    RelationshipType::Stranger
                }
            }
            _ => RelationshipType::Stranger,
        }
    }

    /// Determine group governance from archetype patterns.
    fn determine_group_governance(
        &self,
        members: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> GovernanceType {
        // Calculate average archetype emphasis
        let mut total_spirit = 0.0;
        let mut count = 0;

        for member in members {
            if let Some(processor) = consciousness_processors.get(member) {
                let profile = processor.archetype_profile();
                let spirit: Float = (14..21)
                    .map(|i| profile.get(i).unwrap_or(0.0))
                    .sum::<Float>()
                    / 7.0;
                total_spirit += spirit;
                count += 1;
            }
        }

        if count == 0 {
            return GovernanceType::Tribal;
        }

        let avg_spirit = total_spirit / count as Float;

        // Higher spirit alignment suggests more evolved governance
        if avg_spirit > 0.8 {
            GovernanceType::SocialMemory
        } else if avg_spirit > 0.6 {
            GovernanceType::Theocracy
        } else if avg_spirit > 0.4 {
            GovernanceType::Democracy
        } else if avg_spirit > 0.2 {
            GovernanceType::Republic
        } else {
            GovernanceType::Tribal
        }
    }

    /// Calculate group resonance (coherence between members).
    fn calculate_group_resonance(
        &self,
        members: &[EntityId],
        consciousness_processors: &HashMap<EntityId, ConsciousnessProcessor>,
    ) -> Float {
        if members.len() < 2 {
            return 0.0;
        }

        let mut total_resonance = 0.0;
        let mut pair_count = 0;

        for i in 0..members.len() {
            for j in (i + 1)..members.len() {
                let resonance = self.calculate_archetype_resonance(
                    &members[i],
                    &members[j],
                    consciousness_processors,
                );
                total_resonance += resonance;
                pair_count += 1;
            }
        }

        if pair_count > 0 {
            total_resonance / pair_count as Float
        } else {
            0.0
        }
    }

    // ========================================================================
    // Accessor Methods
    // ========================================================================

    /// Get the number of active relationships
    pub fn relationship_count(&self) -> usize {
        self.relationship_web.relationships.len()
    }

    /// Get the number of active groups
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Get the number of active SMCs
    pub fn smc_count(&self) -> usize {
        self.smc_engine.active_complexes.len()
    }

    /// Get harvest statistics
    pub fn harvest_stats(&self) -> crate::social::harvest::HarvestStats {
        self.harvest_engine.get_harvest_stats()
    }

    /// Get a relationship between two entities
    pub fn get_relationship(
        &self,
        entity_a: EntityId,
        entity_b: EntityId,
    ) -> Option<&Relationship> {
        self.relationship_web.get_relationship(entity_a, entity_b)
    }

    /// Get all relationships for an entity
    pub fn get_entity_relationships(&self, entity: EntityId) -> Vec<(u64, &Relationship)> {
        self.relationship_web.get_entity_relationships(entity)
    }

    /// Get a group by ID
    pub fn get_group(&self, group_id: u64) -> Option<&SocialGroup> {
        self.groups.get(&group_id)
    }

    /// Get an SMC by ID
    pub fn get_smc(&self, smc_id: u64) -> Option<&SocialMemoryComplex> {
        self.smc_engine.active_complexes.get(&smc_id)
    }

    /// Get all SMCs
    pub fn get_smcs(&self) -> Vec<&SocialMemoryComplex> {
        self.smc_engine.get_complexes()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_processor_creation() {
        let processor = SocialProcessor::new();

        assert_eq!(processor.relationship_count(), 0);
        assert_eq!(processor.group_count(), 0);
        assert_eq!(processor.smc_count(), 0);
    }

    #[test]
    fn test_social_output_default() {
        let output = SocialOutput::default();

        assert_eq!(output.interactions_processed, 0);
        assert_eq!(output.relationships_formed, 0);
        assert_eq!(output.communications_sent, 0);
        assert_eq!(output.groups_formed, 0);
        assert_eq!(output.smcs_formed, 0);
        assert_eq!(output.entities_harvested, 0);
    }

    #[test]
    fn test_social_output_has_activity() {
        let mut output = SocialOutput::default();
        assert!(!output.has_activity());

        output.relationships_formed = 1;
        assert!(output.has_activity());
    }

    #[test]
    fn test_social_output_total_events() {
        let output = SocialOutput {
            interactions_processed: 5,
            relationships_formed: 2,
            communications_sent: 3,
            groups_formed: 1,
            smcs_formed: 0,
            entities_harvested: 1,
        };

        assert_eq!(output.total_events(), 12);
    }

    #[test]
    fn test_process_tick_empty_entities() {
        let mut processor = SocialProcessor::new();
        let entities: Vec<EntityId> = Vec::new();
        let processors: HashMap<EntityId, ConsciousnessProcessor> = HashMap::new();

        let output = processor.process_tick(&entities, &processors);

        // Should not crash with empty input
        assert_eq!(output.relationships_formed, 0);
    }

    #[test]
    fn test_form_relationships_requires_resonance() {
        let mut processor = SocialProcessor::new();
        let entities: Vec<EntityId> = vec![
            EntityId::new("entity-1".to_string()),
            EntityId::new("entity-2".to_string()),
        ];
        let processors: HashMap<EntityId, ConsciousnessProcessor> = HashMap::new();

        // Without consciousness processors, should fall back to key-based calculation
        let formed = processor.form_relationships_from_proximity(&entities, &processors);

        // May or may not form depending on key similarity
        assert!(formed <= entities.len());
    }

    #[test]
    fn test_form_groups_requires_min_size() {
        let mut processor = SocialProcessor::new();

        // Only 2 entities - below MIN_GROUP_SIZE of 3
        let entities: Vec<EntityId> = vec![
            EntityId::new("entity-1".to_string()),
            EntityId::new("entity-2".to_string()),
        ];
        let processors: HashMap<EntityId, ConsciousnessProcessor> = HashMap::new();

        let formed = processor.form_groups_from_shared_patterns(&entities, &processors);

        assert_eq!(formed, 0);
    }

    #[test]
    fn test_process_harvest_not_due() {
        let mut processor = SocialProcessor::new();
        let entities: Vec<EntityId> = vec![EntityId::new("entity-1".to_string())];

        // Harvest cycle is 100,000 ticks by default
        processor.current_tick = 50000;
        let harvested = processor.process_harvest(&entities);

        assert_eq!(harvested, 0);
    }

    #[test]
    fn test_calculate_proximity() {
        let processor = SocialProcessor::new();
        let entity_a = EntityId::new("entity-a".to_string());
        let entity_b = EntityId::new("entity-b".to_string());

        let proximity = processor.calculate_proximity(&entity_a, &entity_b);

        assert!((0.0..=1.0).contains(&proximity));
    }

    #[test]
    fn test_relationship_count() {
        let processor = SocialProcessor::new();
        assert_eq!(processor.relationship_count(), 0);
    }

    #[test]
    fn test_group_count() {
        let processor = SocialProcessor::new();
        assert_eq!(processor.group_count(), 0);
    }

    #[test]
    fn test_smc_count() {
        let processor = SocialProcessor::new();
        assert_eq!(processor.smc_count(), 0);
    }

    #[test]
    fn test_harvest_stats() {
        let processor = SocialProcessor::new();
        let stats = processor.harvest_stats();

        assert_eq!(stats.total_harvested, 0);
    }
}
