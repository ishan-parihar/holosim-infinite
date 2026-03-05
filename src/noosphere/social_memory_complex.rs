//! Social Memory Complex - Resonance-Based Collective Consciousness
//!
//! This module implements social memory complexes, which form when entities
//! achieve sufficient resonance to share collective memory and consciousness.
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
//! "Social Memory Complex: Entity resonance calculation, collective memory
//! formation, telepathic communication, group consciousness"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "A social memory complex is a collection of entities who have achieved
//! sufficient harmony to share a collective mind and memory."

use crate::noosphere::{EntityId, Float, NoosphereConfig};
use std::collections::{HashMap, HashSet};

/// Resonance calculator for entity relationships
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Resonance between entities is based
/// on polarity alignment, consciousness level similarity, and shared
/// experiences."
#[derive(Debug, Clone, PartialEq)]
pub struct ResonanceCalculator {
    /// Polarity alignment weight (0.0-1.0)
    polarity_weight: Float,

    /// Consciousness similarity weight (0.0-1.0)
    consciousness_weight: Float,

    /// Shared experience weight (0.0-1.0)
    experience_weight: Float,

    /// Archetype alignment weight (0.0-1.0)
    archetype_weight: Float,
}

impl Default for ResonanceCalculator {
    fn default() -> Self {
        ResonanceCalculator {
            polarity_weight: 0.3,
            consciousness_weight: 0.3,
            experience_weight: 0.2,
            archetype_weight: 0.2,
        }
    }
}

impl ResonanceCalculator {
    /// Create a new resonance calculator
    pub fn new(
        polarity_weight: Float,
        consciousness_weight: Float,
        experience_weight: Float,
        archetype_weight: Float,
    ) -> Self {
        ResonanceCalculator {
            polarity_weight,
            consciousness_weight,
            experience_weight,
            archetype_weight,
        }
    }

    /// Calculate resonance between two entities
    ///
    /// Resonance is a measure of harmonic alignment between entities,
    /// based on polarity, consciousness, shared experiences, and archetype alignment.
    ///
    /// Returns resonance value in range [0.0, 1.0] where:
    /// - 0.0 = No resonance (antagonistic)
    /// - 0.5 = Neutral resonance
    /// - 1.0 = Perfect resonance (identical)
    pub fn calculate_resonance(
        &self,
        entity_a_polarity: Float,
        entity_b_polarity: Float,
        entity_a_consciousness: Float,
        entity_b_consciousness: Float,
        shared_experiences: Float,
        archetype_alignment: Float,
    ) -> Float {
        // Calculate polarity resonance (1.0 = same polarity)
        let polarity_resonance = 1.0 - (entity_a_polarity - entity_b_polarity).abs();

        // Calculate consciousness resonance (1.0 = same level)
        let consciousness_resonance = 1.0 - (entity_a_consciousness - entity_b_consciousness).abs();

        // Normalize shared experiences (0.0-1.0)
        let experience_resonance = shared_experiences.min(1.0);

        // Normalize archetype alignment (0.0-1.0)
        let archetype_resonance = archetype_alignment.min(1.0);

        // Weighted combination
        let total_weight = self.polarity_weight
            + self.consciousness_weight
            + self.experience_weight
            + self.archetype_weight;

        let resonance = (self.polarity_weight * polarity_resonance
            + self.consciousness_weight * consciousness_resonance
            + self.experience_weight * experience_resonance
            + self.archetype_weight * archetype_resonance)
            / total_weight;

        resonance.clamp(0.0, 1.0)
    }

    /// Calculate group resonance for multiple entities
    ///
    /// Returns the average pairwise resonance among all entities in the group.
    pub fn calculate_group_resonance(
        &self,
        entities: &[(EntityId, Float, Float)], // (id, polarity, consciousness)
        shared_experiences: &HashMap<(EntityId, EntityId), Float>,
        archetype_alignments: &HashMap<(EntityId, EntityId), Float>,
    ) -> Float {
        if entities.len() < 2 {
            return 0.0;
        }

        let mut total_resonance = 0.0;
        let mut pair_count = 0_usize;

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let (id_a, polarity_a, consciousness_a) = entities[i];
                let (id_b, polarity_b, consciousness_b) = entities[j];

                let shared_exp = shared_experiences
                    .get(&(id_a, id_b))
                    .copied()
                    .unwrap_or(0.0);
                let archetype_align = archetype_alignments
                    .get(&(id_a, id_b))
                    .copied()
                    .unwrap_or(0.0);

                let resonance = self.calculate_resonance(
                    polarity_a,
                    polarity_b,
                    consciousness_a,
                    consciousness_b,
                    shared_exp,
                    archetype_align,
                );

                total_resonance += resonance;
                pair_count += 1;
            }
        }

        if pair_count == 0 {
            return 0.0;
        }

        total_resonance / pair_count as Float
    }
}

/// Memory former for collective memory creation
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Collective memory emerges when entities
/// share experiences and synchronize their understanding through resonance."
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryFormer {
    /// Memory retention rate (0.0-1.0)
    retention_rate: Float,

    /// Memory integration rate (0.0-1.0)
    integration_rate: Float,

    /// Memory decay rate (0.0-1.0)
    decay_rate: Float,
}

impl Default for MemoryFormer {
    fn default() -> Self {
        MemoryFormer {
            retention_rate: 0.8,
            integration_rate: 0.6,
            decay_rate: 0.01,
        }
    }
}

impl MemoryFormer {
    /// Create a new memory former
    pub fn new(retention_rate: Float, integration_rate: Float, decay_rate: Float) -> Self {
        MemoryFormer {
            retention_rate,
            integration_rate,
            decay_rate,
        }
    }

    /// Form collective memory from entity experiences
    ///
    /// Collective memory integrates individual experiences into a shared
    /// understanding that can be accessed by all members of the complex.
    pub fn form_collective_memory(
        &self,
        entity_experiences: &HashMap<EntityId, Vec<Float>>,
        resonance: Float,
    ) -> CollectiveMemory {
        if entity_experiences.is_empty() {
            return CollectiveMemory::default();
        }

        // Calculate average experience across all entities
        let mut integrated_memory: Vec<Float> = Vec::new();
        let max_length = entity_experiences
            .values()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);

        for i in 0..max_length {
            let mut sum = 0.0;
            let mut count = 0_usize;

            for experiences in entity_experiences.values() {
                if i < experiences.len() {
                    sum += experiences[i];
                    count += 1;
                }
            }

            if count > 0 {
                integrated_memory.push(sum / count as Float);
            }
        }

        // Apply resonance to memory strength
        let memory_strength = resonance * self.retention_rate;

        CollectiveMemory {
            memory: integrated_memory,
            strength: memory_strength,
            coherence: resonance,
            entity_count: entity_experiences.len(),
        }
    }

    /// Update collective memory with new experiences
    pub fn update_memory(
        &self,
        memory: &mut CollectiveMemory,
        new_experiences: &HashMap<EntityId, Vec<Float>>,
    ) {
        if new_experiences.is_empty() {
            // Apply decay
            memory.strength *= 1.0 - self.decay_rate;
            return;
        }

        // Integrate new experiences
        let mut integrated_new: Vec<Float> = Vec::new();
        let max_length = new_experiences.values().map(|v| v.len()).max().unwrap_or(0);

        for i in 0..max_length {
            let mut sum = 0.0;
            let mut count = 0_usize;

            for experiences in new_experiences.values() {
                if i < experiences.len() {
                    sum += experiences[i];
                    count += 1;
                }
            }

            if count > 0 {
                integrated_new.push(sum / count as Float);
            }
        }

        // Merge with existing memory
        if memory.memory.len() < integrated_new.len() {
            memory.memory.resize(integrated_new.len(), 0.0);
        }

        for i in 0..memory.memory.len() {
            if i < integrated_new.len() {
                memory.memory[i] = memory.memory[i] * (1.0 - self.integration_rate)
                    + integrated_new[i] * self.integration_rate;
            }
        }

        // Update strength
        memory.strength = (memory.strength + self.integration_rate).min(1.0);
        memory.entity_count = new_experiences.len();
    }
}

/// Collective memory shared by a social memory complex
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Collective memory is the shared
/// understanding that emerges when entities achieve sufficient resonance."
#[derive(Debug, Clone, PartialEq)]
pub struct CollectiveMemory {
    /// Integrated memory content
    pub memory: Vec<Float>,

    /// Memory strength (0.0-1.0)
    pub strength: Float,

    /// Memory coherence (0.0-1.0)
    pub coherence: Float,

    /// Number of entities contributing to memory
    pub entity_count: usize,
}

impl Default for CollectiveMemory {
    fn default() -> Self {
        CollectiveMemory {
            memory: Vec::new(),
            strength: 0.0,
            coherence: 0.0,
            entity_count: 0,
        }
    }
}

/// Social memory complex
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "A social memory complex is a collection
/// of entities who have achieved sufficient harmony to share a collective
/// mind and memory."
#[derive(Debug, Clone, PartialEq)]
pub struct SocialMemoryComplex {
    /// Unique identifier for this complex
    pub complex_id: EntityId,

    /// Entities in this complex
    pub entities: HashSet<EntityId>,

    /// Collective memory
    pub collective_memory: CollectiveMemory,

    /// Group consciousness level
    pub group_consciousness: Float,

    /// Telepathic links between entities
    pub telepathic_links: HashMap<(EntityId, EntityId), TelepathicLink>,

    /// Formation timestamp
    pub formation_time: Float,
}

impl Default for SocialMemoryComplex {
    fn default() -> Self {
        SocialMemoryComplex {
            complex_id: 0,
            entities: HashSet::new(),
            collective_memory: CollectiveMemory::default(),
            group_consciousness: 0.0,
            telepathic_links: HashMap::new(),
            formation_time: 0.0,
        }
    }
}

impl SocialMemoryComplex {
    /// Create a new social memory complex
    pub fn new(complex_id: EntityId, formation_time: Float) -> Self {
        SocialMemoryComplex {
            complex_id,
            formation_time,
            ..Default::default()
        }
    }

    /// Add an entity to the complex
    pub fn add_entity(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id);
    }

    /// Remove an entity from the complex
    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.entities.remove(&entity_id);

        // Remove telepathic links involving this entity
        self.telepathic_links
            .retain(|(a, b), _| *a != entity_id && *b != entity_id);
    }

    /// Check if complex is stable (has minimum entities)
    pub fn is_stable(&self, min_entities: usize) -> bool {
        self.entities.len() >= min_entities
    }
}

/// Telepathic link between entities
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Telepathy emerges when entities achieve
/// sufficient resonance to communicate directly without physical means."
#[derive(Debug, Clone, PartialEq)]
pub struct TelepathicLink {
    /// Source entity
    pub source: EntityId,

    /// Target entity
    pub target: EntityId,

    /// Link strength (0.0-1.0)
    pub strength: Float,

    /// Bandwidth (0.0-1.0)
    pub bandwidth: Float,

    /// Latency (0.0-1.0, lower is better)
    pub latency: Float,
}

impl Default for TelepathicLink {
    fn default() -> Self {
        TelepathicLink {
            source: 0,
            target: 0,
            strength: 0.0,
            bandwidth: 0.0,
            latency: 1.0,
        }
    }
}

impl TelepathicLink {
    /// Create a new telepathic link
    pub fn new(source: EntityId, target: EntityId, strength: Float) -> Self {
        TelepathicLink {
            source,
            target,
            strength: strength.clamp(0.0, 1.0),
            bandwidth: strength * 0.8,
            latency: 1.0 - strength * 0.5,
        }
    }

    /// Check if link is functional
    pub fn is_functional(&self) -> bool {
        self.strength > 0.3 && self.bandwidth > 0.1
    }
}

/// Group consciousness
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Group consciousness emerges when
/// entities in a social memory complex synchronize their awareness."
#[derive(Debug, Clone, PartialEq)]
pub struct GroupConsciousness {
    /// Collective awareness level (0.0-1.0)
    pub awareness: Float,

    /// Collective intentionality (0.0-1.0)
    pub intentionality: Float,

    /// Collective wisdom (0.0-1.0)
    pub wisdom: Float,

    /// Synchronization level (0.0-1.0)
    pub synchronization: Float,
}

impl Default for GroupConsciousness {
    fn default() -> Self {
        GroupConsciousness {
            awareness: 0.0,
            intentionality: 0.0,
            wisdom: 0.0,
            synchronization: 0.0,
        }
    }
}

impl GroupConsciousness {
    /// Create a new group consciousness
    pub fn new(
        awareness: Float,
        intentionality: Float,
        wisdom: Float,
        synchronization: Float,
    ) -> Self {
        GroupConsciousness {
            awareness: awareness.clamp(0.0, 1.0),
            intentionality: intentionality.clamp(0.0, 1.0),
            wisdom: wisdom.clamp(0.0, 1.0),
            synchronization: synchronization.clamp(0.0, 1.0),
        }
    }

    /// Calculate overall group consciousness
    pub fn overall_consciousness(&self) -> Float {
        (self.awareness + self.intentionality + self.wisdom + self.synchronization) / 4.0
    }
}

/// Main social memory complex orchestrator
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
/// "SocialMemoryComplex: Entity resonance calculation, collective memory
/// formation, telepathic communication, group consciousness"
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SocialMemoryComplexSystem {
    /// Resonance calculator
    resonance_calculator: ResonanceCalculator,

    /// Memory former
    memory_former: MemoryFormer,

    /// Configuration
    config: NoosphereConfig,
}

impl SocialMemoryComplexSystem {
    /// Create a new social memory complex system
    pub fn new(
        resonance_calculator: ResonanceCalculator,
        memory_former: MemoryFormer,
        config: NoosphereConfig,
    ) -> Self {
        SocialMemoryComplexSystem {
            resonance_calculator,
            memory_former,
            config,
        }
    }

    /// Calculate resonance between entities
    ///
    /// Returns resonance value in range [0.0, 1.0].
    pub fn calculate_resonance(
        &self,
        entity_a_polarity: Float,
        entity_b_polarity: Float,
        entity_a_consciousness: Float,
        entity_b_consciousness: Float,
        shared_experiences: Float,
        archetype_alignment: Float,
    ) -> ResonanceResult {
        let resonance = self.resonance_calculator.calculate_resonance(
            entity_a_polarity,
            entity_b_polarity,
            entity_a_consciousness,
            entity_b_consciousness,
            shared_experiences,
            archetype_alignment,
        );

        ResonanceResult {
            resonance,
            can_form_complex: resonance >= self.config.resonance_threshold,
        }
    }

    /// Form a social memory complex from entities
    ///
    /// Entities must have sufficient resonance to form a complex.
    pub fn form_collective_memory(
        &self,
        entity_experiences: &HashMap<EntityId, Vec<Float>>,
        resonance: Float,
    ) -> MemoryFormationResult {
        if !resonance.is_finite() || !(0.0..=1.0).contains(&resonance) {
            return MemoryFormationResult {
                collective_memory: CollectiveMemory::default(),
                success: false,
                reason: Some("Invalid resonance value".to_string()),
            };
        }

        if entity_experiences.is_empty() {
            return MemoryFormationResult {
                collective_memory: CollectiveMemory::default(),
                success: false,
                reason: Some("No entities provided".to_string()),
            };
        }

        let collective_memory = self
            .memory_former
            .form_collective_memory(entity_experiences, resonance);

        let success = collective_memory.strength > 0.3;

        MemoryFormationResult {
            collective_memory,
            success,
            reason: if success {
                None
            } else {
                Some("Insufficient memory strength".to_string())
            },
        }
    }

    /// Enable telepathy between entities
    ///
    /// Creates telepathic links based on resonance strength.
    pub fn enable_telepathy(
        &self,
        complex: &SocialMemoryComplex,
        entity_polarities: &HashMap<EntityId, Float>,
        entity_consciousness: &HashMap<EntityId, Float>,
        shared_experiences: &HashMap<(EntityId, EntityId), Float>,
        archetype_alignments: &HashMap<(EntityId, EntityId), Float>,
    ) -> TelepathyResult {
        let mut telepathic_links = HashMap::new();

        for entity_a in complex.entities.iter() {
            for entity_b in complex.entities.iter() {
                if entity_a == entity_b {
                    continue;
                }

                let polarity_a = entity_polarities.get(entity_a).copied().unwrap_or(0.5);
                let polarity_b = entity_polarities.get(entity_b).copied().unwrap_or(0.5);
                let consciousness_a = entity_consciousness.get(entity_a).copied().unwrap_or(0.5);
                let consciousness_b = entity_consciousness.get(entity_b).copied().unwrap_or(0.5);
                let shared_exp = shared_experiences
                    .get(&(*entity_a, *entity_b))
                    .copied()
                    .unwrap_or(0.0);
                let archetype_align = archetype_alignments
                    .get(&(*entity_a, *entity_b))
                    .copied()
                    .unwrap_or(0.0);

                let resonance = self.resonance_calculator.calculate_resonance(
                    polarity_a,
                    polarity_b,
                    consciousness_a,
                    consciousness_b,
                    shared_exp,
                    archetype_align,
                );

                // Create telepathic link if resonance is sufficient
                if resonance >= self.config.resonance_threshold * 0.8 {
                    let link = TelepathicLink::new(*entity_a, *entity_b, resonance);
                    telepathic_links.insert((*entity_a, *entity_b), link);
                }
            }
        }

        let success = !telepathic_links.is_empty();
        let link_count = telepathic_links.len();

        TelepathyResult {
            telepathic_links,
            success,
            link_count,
        }
    }

    /// Calculate group consciousness
    ///
    /// Group consciousness emerges from synchronized entity awareness.
    pub fn calculate_group_consciousness(
        &self,
        complex: &SocialMemoryComplex,
        entity_awareness: &HashMap<EntityId, Float>,
        entity_intentionality: &HashMap<EntityId, Float>,
        entity_wisdom: &HashMap<EntityId, Float>,
    ) -> GroupConsciousnessResult {
        if complex.entities.is_empty() {
            return GroupConsciousnessResult {
                group_consciousness: GroupConsciousness::default(),
                success: false,
                reason: Some("No entities in complex".to_string()),
            };
        }

        // Calculate average awareness, intentionality, wisdom
        let mut total_awareness = 0.0;
        let mut total_intentionality = 0.0;
        let mut total_wisdom = 0.0;
        let mut count = 0_usize;

        for entity_id in complex.entities.iter() {
            if let Some(&awareness) = entity_awareness.get(entity_id) {
                total_awareness += awareness;
            }
            if let Some(&intentionality) = entity_intentionality.get(entity_id) {
                total_intentionality += intentionality;
            }
            if let Some(&wisdom) = entity_wisdom.get(entity_id) {
                total_wisdom += wisdom;
            }
            count += 1;
        }

        if count == 0 {
            return GroupConsciousnessResult {
                group_consciousness: GroupConsciousness::default(),
                success: false,
                reason: Some("No entity data available".to_string()),
            };
        }

        let avg_awareness = total_awareness / count as Float;
        let avg_intentionality = total_intentionality / count as Float;
        let avg_wisdom = total_wisdom / count as Float;

        // Synchronization based on collective memory coherence
        let synchronization = complex.collective_memory.coherence;

        let group_consciousness = GroupConsciousness::new(
            avg_awareness,
            avg_intentionality,
            avg_wisdom,
            synchronization,
        );

        let success = group_consciousness.overall_consciousness() > 0.3;

        GroupConsciousnessResult {
            group_consciousness,
            success,
            reason: if success {
                None
            } else {
                Some("Insufficient group consciousness".to_string())
            },
        }
    }
}

/// Result of resonance calculation
#[derive(Debug, Clone, PartialEq)]
pub struct ResonanceResult {
    /// Resonance value (0.0-1.0)
    pub resonance: Float,

    /// Whether entities can form a social memory complex
    pub can_form_complex: bool,
}

/// Result of memory formation
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryFormationResult {
    /// Collective memory formed
    pub collective_memory: CollectiveMemory,

    /// Whether memory formation was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of telepathy enablement
#[derive(Debug, Clone, PartialEq)]
pub struct TelepathyResult {
    /// Telepathic links created
    pub telepathic_links: HashMap<(EntityId, EntityId), TelepathicLink>,

    /// Whether telepathy was successfully enabled
    pub success: bool,

    /// Number of links created
    pub link_count: usize,
}

/// Result of group consciousness calculation
#[derive(Debug, Clone, PartialEq)]
pub struct GroupConsciousnessResult {
    /// Group consciousness calculated
    pub group_consciousness: GroupConsciousness,

    /// Whether group consciousness is sufficient
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_calculator_default() {
        let calculator = ResonanceCalculator::default();
        assert_eq!(calculator.polarity_weight, 0.3);
        assert_eq!(calculator.consciousness_weight, 0.3);
        assert_eq!(calculator.experience_weight, 0.2);
        assert_eq!(calculator.archetype_weight, 0.2);
    }

    #[test]
    fn test_calculate_resonance_identical() {
        let calculator = ResonanceCalculator::default();
        let resonance = calculator.calculate_resonance(
            0.8, // polarity
            0.8, // polarity
            0.7, // consciousness
            0.7, // consciousness
            1.0, // shared experiences
            1.0, // archetype alignment
        );
        assert_eq!(resonance, 1.0);
    }

    #[test]
    fn test_calculate_resonance_different() {
        let calculator = ResonanceCalculator::default();
        let resonance = calculator.calculate_resonance(
            0.2, // polarity
            0.8, // polarity
            0.3, // consciousness
            0.7, // consciousness
            0.0, // shared experiences
            0.0, // archetype alignment
        );
        assert!(resonance < 0.5);
    }

    #[test]
    fn test_calculate_group_resonance() {
        let calculator = ResonanceCalculator::default();
        let entities = vec![
            (1_u64, 0.8_f64, 0.7_f64),
            (2_u64, 0.8_f64, 0.7_f64),
            (3_u64, 0.8_f64, 0.7_f64),
        ];
        let mut shared_experiences = HashMap::new();
        shared_experiences.insert((1, 2), 1.0);
        shared_experiences.insert((1, 3), 1.0);
        shared_experiences.insert((2, 3), 1.0);
        let mut archetype_alignments = HashMap::new();
        archetype_alignments.insert((1, 2), 1.0);
        archetype_alignments.insert((1, 3), 1.0);
        archetype_alignments.insert((2, 3), 1.0);

        let resonance = calculator.calculate_group_resonance(
            &entities,
            &shared_experiences,
            &archetype_alignments,
        );
        assert_eq!(resonance, 1.0);
    }

    #[test]
    fn test_memory_former_default() {
        let former = MemoryFormer::default();
        assert_eq!(former.retention_rate, 0.8);
        assert_eq!(former.integration_rate, 0.6);
        assert_eq!(former.decay_rate, 0.01);
    }

    #[test]
    fn test_form_collective_memory() {
        let former = MemoryFormer::default();
        let mut entity_experiences = HashMap::new();
        entity_experiences.insert(1_u64, vec![0.5, 0.6, 0.7]);
        entity_experiences.insert(2_u64, vec![0.5, 0.6, 0.7]);

        let memory = former.form_collective_memory(&entity_experiences, 0.8);
        assert_eq!(memory.memory.len(), 3);
        assert_eq!(memory.memory[0], 0.5);
        assert!((memory.strength - 0.64).abs() < 0.001); // 0.8 * 0.8
        assert_eq!(memory.coherence, 0.8);
        assert_eq!(memory.entity_count, 2);
    }

    #[test]
    fn test_update_memory() {
        let former = MemoryFormer::default();
        let mut memory = CollectiveMemory {
            memory: vec![0.5, 0.6, 0.7],
            strength: 0.64,
            coherence: 0.8,
            entity_count: 2,
        };

        let mut new_experiences = HashMap::new();
        new_experiences.insert(1_u64, vec![0.8, 0.9, 1.0]);
        new_experiences.insert(2_u64, vec![0.8, 0.9, 1.0]);

        former.update_memory(&mut memory, &new_experiences);
        assert!(memory.memory[0] > 0.5); // Should have integrated new experiences
        assert!(memory.strength > 0.64); // Should have increased
    }

    #[test]
    fn test_update_memory_decay() {
        let former = MemoryFormer::default();
        let mut memory = CollectiveMemory {
            memory: vec![0.5, 0.6, 0.7],
            strength: 0.64,
            coherence: 0.8,
            entity_count: 2,
        };

        former.update_memory(&mut memory, &HashMap::new());
        assert!(memory.strength < 0.64); // Should have decayed
    }

    #[test]
    fn test_social_memory_complex_default() {
        let complex = SocialMemoryComplex::default();
        assert_eq!(complex.complex_id, 0);
        assert!(complex.entities.is_empty());
        assert_eq!(complex.group_consciousness, 0.0);
    }

    #[test]
    fn test_social_memory_complex_new() {
        let complex = SocialMemoryComplex::new(123, 42.0);
        assert_eq!(complex.complex_id, 123);
        assert_eq!(complex.formation_time, 42.0);
    }

    #[test]
    fn test_add_remove_entity() {
        let mut complex = SocialMemoryComplex::new(1, 0.0);
        complex.add_entity(10);
        assert!(complex.entities.contains(&10));
        complex.remove_entity(10);
        assert!(!complex.entities.contains(&10));
    }

    #[test]
    fn test_is_stable() {
        let mut complex = SocialMemoryComplex::new(1, 0.0);
        assert!(!complex.is_stable(3));
        complex.add_entity(1);
        complex.add_entity(2);
        complex.add_entity(3);
        assert!(complex.is_stable(3));
    }

    #[test]
    fn test_telepathic_link_new() {
        let link = TelepathicLink::new(1, 2, 0.8);
        assert_eq!(link.source, 1);
        assert_eq!(link.target, 2);
        assert_eq!(link.strength, 0.8);
        assert!((link.bandwidth - 0.64).abs() < 0.001); // 0.8 * 0.8
        assert_eq!(link.latency, 0.6); // 1.0 - 0.8 * 0.5
    }

    #[test]
    fn test_telepathic_link_functional() {
        let functional_link = TelepathicLink::new(1, 2, 0.8);
        assert!(functional_link.is_functional());

        let weak_link = TelepathicLink::new(1, 2, 0.2);
        assert!(!weak_link.is_functional());
    }

    #[test]
    fn test_group_consciousness_new() {
        let gc = GroupConsciousness::new(0.8, 0.7, 0.6, 0.9);
        assert_eq!(gc.awareness, 0.8);
        assert_eq!(gc.intentionality, 0.7);
        assert_eq!(gc.wisdom, 0.6);
        assert_eq!(gc.synchronization, 0.9);
    }

    #[test]
    fn test_group_consciousness_overall() {
        let gc = GroupConsciousness::new(0.8, 0.7, 0.6, 0.9);
        let overall = gc.overall_consciousness();
        assert_eq!(overall, (0.8 + 0.7 + 0.6 + 0.9) / 4.0);
    }

    #[test]
    fn test_social_memory_complex_system_default() {
        let system = SocialMemoryComplexSystem::default();
        assert_eq!(system.config.resonance_threshold, 0.7);
    }

    #[test]
    fn test_calculate_resonance_can_form() {
        let system = SocialMemoryComplexSystem::default();
        let result = system.calculate_resonance(0.8, 0.8, 0.7, 0.7, 1.0, 1.0);
        assert_eq!(result.resonance, 1.0);
        assert!(result.can_form_complex);
    }

    #[test]
    fn test_calculate_resonance_cannot_form() {
        let system = SocialMemoryComplexSystem::default();
        let result = system.calculate_resonance(0.2, 0.8, 0.3, 0.7, 0.0, 0.0);
        assert!(!result.can_form_complex);
    }

    #[test]
    fn test_form_collective_memory_success() {
        let system = SocialMemoryComplexSystem::default();
        let mut entity_experiences = HashMap::new();
        entity_experiences.insert(1_u64, vec![0.5, 0.6, 0.7]);
        entity_experiences.insert(2_u64, vec![0.5, 0.6, 0.7]);

        let result = system.form_collective_memory(&entity_experiences, 0.8);
        assert!(result.success);
        assert_eq!(result.collective_memory.entity_count, 2);
    }

    #[test]
    fn test_form_collective_memory_failure() {
        let system = SocialMemoryComplexSystem::default();
        let result = system.form_collective_memory(&HashMap::new(), 0.8);
        assert!(!result.success);
        assert!(result.reason.is_some());
    }

    #[test]
    fn test_enable_telepathy() {
        let system = SocialMemoryComplexSystem::default();
        let mut complex = SocialMemoryComplex::new(1, 0.0);
        complex.add_entity(1);
        complex.add_entity(2);

        let mut entity_polarities = HashMap::new();
        entity_polarities.insert(1, 0.8);
        entity_polarities.insert(2, 0.8);
        let mut entity_consciousness = HashMap::new();
        entity_consciousness.insert(1, 0.7);
        entity_consciousness.insert(2, 0.7);
        let mut shared_experiences = HashMap::new();
        shared_experiences.insert((1, 2), 1.0);
        let mut archetype_alignments = HashMap::new();
        archetype_alignments.insert((1, 2), 1.0);

        let result = system.enable_telepathy(
            &complex,
            &entity_polarities,
            &entity_consciousness,
            &shared_experiences,
            &archetype_alignments,
        );
        assert!(result.success);
        assert_eq!(result.link_count, 2); // 1->2 and 2->1
    }

    #[test]
    fn test_calculate_group_consciousness() {
        let system = SocialMemoryComplexSystem::default();
        let mut complex = SocialMemoryComplex::new(1, 0.0);
        complex.add_entity(1);
        complex.add_entity(2);
        complex.collective_memory = CollectiveMemory {
            memory: vec![0.5],
            strength: 0.8,
            coherence: 0.9,
            entity_count: 2,
        };

        let mut entity_awareness = HashMap::new();
        entity_awareness.insert(1, 0.8);
        entity_awareness.insert(2, 0.8);
        let mut entity_intentionality = HashMap::new();
        entity_intentionality.insert(1, 0.7);
        entity_intentionality.insert(2, 0.7);
        let mut entity_wisdom = HashMap::new();
        entity_wisdom.insert(1, 0.6);
        entity_wisdom.insert(2, 0.6);

        let result = system.calculate_group_consciousness(
            &complex,
            &entity_awareness,
            &entity_intentionality,
            &entity_wisdom,
        );
        assert!(result.success);
        assert_eq!(result.group_consciousness.awareness, 0.8);
        assert_eq!(result.group_consciousness.synchronization, 0.9);
    }

    #[test]
    fn test_noosphere_config_default() {
        let config = NoosphereConfig::default();
        assert_eq!(config.resonance_threshold, 0.7);
    }

    #[test]
    fn test_noosphere_config_validate() {
        let config = NoosphereConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = NoosphereConfig {
            resonance_threshold: 1.5,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
