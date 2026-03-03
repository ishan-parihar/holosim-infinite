//! Social Memory via Resonance (Phase 5)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "The sixth phase implements collective consciousness formation through resonance rather
//! than proximity. This transforms the current proximity-based collective formation into a
//! more sophisticated system where entities can form mental or spiritual connections across
//! any distance if their field configurations align sufficiently."
//!
//! This module implements:
//! - Resonance computation between entities (phase alignment)
//! - Collective formation via resonance threshold
//! - Collective consciousness with emergent properties
//! - Resonance-based social memory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Collectives form by resonance, not proximity"

use super::field_state::{Complex, FieldNodeData, Float, HolographicFieldState, OctreeNode};
use std::collections::{HashMap, HashSet};

/// Entity phase information for resonance calculation
#[derive(Debug, Clone)]
pub struct EntityPhase {
    /// Entity position
    pub position: [Float; 3],

    /// Phase vector (8 components for 8 densities)
    pub phase_vector: [Float; 8],

    /// Consciousness level
    pub consciousness: Float,

    /// Entity ID
    pub entity_id: usize,
}

impl EntityPhase {
    pub fn new(entity_id: usize, position: [Float; 3]) -> Self {
        EntityPhase {
            position,
            phase_vector: [0.0; 8],
            consciousness: 0.5,
            entity_id,
        }
    }

    /// Calculate phase from field at position
    pub fn calculate_from_field(&mut self, field: &HolographicFieldState) {
        // Get field data at position
        // Simplified - in full implementation would query octree
        let center = field.root.bounds.center();

        // Use position as seed for phase
        let seed = (center[0] * 10.0 + center[1] * 100.0 + center[2] * 1000.0) as usize;

        for i in 0..8 {
            self.phase_vector[i] =
                ((seed * (i + 1) * 7) % 360) as Float / 180.0 * std::f64::consts::PI;
        }

        // Consciousness from field
        self.consciousness = field.root.field_data.coherence;
    }
}

/// Resonance between two entities
#[derive(Debug, Clone)]
pub struct Resonance {
    /// First entity ID
    pub entity_a: usize,

    /// Second entity ID
    pub entity_b: usize,

    /// Resonance strength (0.0 - 1.0)
    pub strength: Float,

    /// Whether entities are in a collective
    pub is_collective: bool,
}

impl Resonance {
    pub fn new(entity_a: usize, entity_b: usize, strength: Float) -> Self {
        Resonance {
            entity_a,
            entity_b,
            strength: strength.clamp(0.0, 1.0),
            is_collective: false,
        }
    }
}

/// Configuration for social memory
#[derive(Debug, Clone)]
pub struct SocialMemoryConfig {
    /// Resonance threshold for collective formation (0.0 - 1.0)
    pub collective_threshold: Float,

    /// Maximum collective size
    pub max_collective_size: usize,

    /// Resonance decay rate
    pub resonance_decay: Float,

    /// Whether to enable cosmic-scale collectives
    pub enable_cosmic_collectives: bool,

    /// Minimum consciousness for collective membership
    pub min_consciousness: Float,
}

impl Default for SocialMemoryConfig {
    fn default() -> Self {
        SocialMemoryConfig {
            collective_threshold: 0.7,
            max_collective_size: 1000,
            resonance_decay: 0.99,
            enable_cosmic_collectives: true,
            min_consciousness: 0.3,
        }
    }
}

/// A collective of entities
#[derive(Debug, Clone)]
pub struct Collective {
    /// Collective ID
    pub id: usize,

    /// Member entity IDs
    pub members: HashSet<usize>,

    /// Collective consciousness level
    pub collective_consciousness: Float,

    /// Collective phase (merged from members)
    pub collective_phase: [Float; 8],

    /// Shared experiences/memories
    pub shared_experiences: Vec<SharedExperience>,

    /// Creation time
    pub created_at: Float,
}

/// Shared experience in a collective
#[derive(Debug, Clone)]
pub struct SharedExperience {
    /// Experience ID
    pub id: usize,

    /// Experience data
    pub data: Float,

    /// When shared
    pub timestamp: Float,

    /// Importance
    pub importance: Float,
}

impl Collective {
    pub fn new(id: usize) -> Self {
        Collective {
            id,
            members: HashSet::new(),
            collective_consciousness: 0.0,
            collective_phase: [0.0; 8],
            shared_experiences: Vec::new(),
            created_at: 0.0,
        }
    }

    /// Add a member to the collective
    pub fn add_member(
        &mut self,
        entity_id: usize,
        phase: &[Float; 8],
        consciousness: Float,
    ) -> bool {
        if self.members.len() < 1000 {
            self.members.insert(entity_id);

            // Update collective phase (average of members)
            for i in 0..8 {
                self.collective_phase[i] =
                    (self.collective_phase[i] * (self.members.len() - 1) as Float + phase[i])
                        / self.members.len() as Float;
            }

            // Update collective consciousness (can exceed individual)
            self.collective_consciousness =
                (self.collective_consciousness + consciousness).min(1.0);

            true
        } else {
            false
        }
    }

    /// Remove a member
    pub fn remove_member(&mut self, entity_id: usize) -> bool {
        if self.members.remove(&entity_id) {
            // Recalculate collective phase (simplified)
            // In full implementation would track member phases
            self.collective_consciousness *= 0.99;
            true
        } else {
            false
        }
    }

    /// Add shared experience
    pub fn add_experience(&mut self, data: Float, importance: Float) {
        let id = self.shared_experiences.len();
        self.shared_experiences.push(SharedExperience {
            id,
            data,
            timestamp: 0.0,
            importance,
        });

        // Keep bounded
        if self.shared_experiences.len() > 1000 {
            self.shared_experiences.remove(0);
        }
    }

    /// Get collective coherence (how unified is the collective)
    pub fn get_coherence(&self) -> Float {
        if self.members.is_empty() {
            return 0.0;
        }

        // Simplified coherence calculation
        // In full implementation would calculate variance of member phases
        self.collective_consciousness
    }
}

/// Resonance-based social memory system
pub struct SocialMemory {
    /// Entity phases
    entity_phases: HashMap<usize, EntityPhase>,

    /// Computed resonances
    resonances: HashMap<(usize, usize), Resonance>,

    /// Active collectives
    collectives: Vec<Collective>,

    /// Entity to collective mapping
    entity_collective: HashMap<usize, usize>,

    /// Configuration
    config: SocialMemoryConfig,

    /// Statistics
    pub statistics: SocialMemoryStatistics,
}

/// Statistics for social memory
#[derive(Debug, Clone, Default)]
pub struct SocialMemoryStatistics {
    /// Total entities tracked
    pub entity_count: usize,

    /// Total resonances computed
    pub resonance_count: usize,

    /// Active collective count
    pub collective_count: usize,

    /// Average collective size
    pub average_collective_size: Float,

    /// Entities in collectives
    pub entities_in_collectives: usize,

    /// Cosmic-scale collectives
    pub cosmic_collectives: usize,
}

impl SocialMemory {
    pub fn new(config: SocialMemoryConfig) -> Self {
        SocialMemory {
            entity_phases: HashMap::new(),
            resonances: HashMap::new(),
            collectives: Vec::new(),
            entity_collective: HashMap::new(),
            config,
            statistics: SocialMemoryStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(SocialMemoryConfig::default())
    }

    /// Register an entity
    pub fn register_entity(&mut self, entity_id: usize, position: [Float; 3]) {
        let phase = EntityPhase::new(entity_id, position);
        self.entity_phases.insert(entity_id, phase);
        self.statistics.entity_count = self.entity_phases.len();
    }

    /// Update entity phase from field
    pub fn update_entity_phase(&mut self, entity_id: usize, field: &HolographicFieldState) {
        if let Some(phase) = self.entity_phases.get_mut(&entity_id) {
            phase.calculate_from_field(field);
        }
    }

    /// Compute resonance between two entities
    pub fn compute_resonance(&self, entity_a: usize, entity_b: usize) -> Float {
        let phase_a = match self.entity_phases.get(&entity_a) {
            Some(p) => p,
            None => return 0.0,
        };

        let phase_b = match self.entity_phases.get(&entity_b) {
            Some(p) => p,
            None => return 0.0,
        };

        // Resonance = dot product of normalized phase vectors
        // This measures how aligned the entities' field configurations are
        let mut dot_product = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..8 {
            dot_product += phase_a.phase_vector[i] * phase_b.phase_vector[i];
            norm_a += phase_a.phase_vector[i] * phase_a.phase_vector[i];
            norm_b += phase_b.phase_vector[i] * phase_b.phase_vector[i];
        }

        let norm = (norm_a * norm_b).sqrt();
        if norm > 0.001 {
            // Convert to 0-1 range
            (dot_product / norm + 1.0) / 2.0
        } else {
            0.0
        }
    }

    /// Compute all resonances (expensive - use sparingly)
    pub fn compute_all_resonances(&mut self) {
        let entities: Vec<_> = self.entity_phases.keys().cloned().collect();
        let entity_count = entities.len();

        self.resonances.clear();

        // Compute pairwise resonances
        for i in 0..entity_count {
            for j in (i + 1)..entity_count {
                let a = entities[i];
                let b = entities[j];
                let strength = self.compute_resonance(a, b);

                if strength > 0.0 {
                    let resonance = Resonance::new(a, b, strength);
                    self.resonances.insert((a, b), resonance);
                }
            }
        }

        self.statistics.resonance_count = self.resonances.len();
    }

    /// Form collectives based on resonance
    pub fn form_collectives(&mut self) {
        // Start new collective formation cycle
        self.collectives.retain(|c| !c.members.is_empty());

        // Group entities by resonance
        let mut potential_groups: Vec<HashSet<usize>> = Vec::new();
        let entities: Vec<_> = self.entity_phases.keys().cloned().collect();

        for &entity_id in &entities {
            // Skip if already in collective
            if self.entity_collective.contains_key(&entity_id) {
                continue;
            }

            // Find all entities with high resonance
            let mut group = HashSet::new();
            group.insert(entity_id);

            for (&(a, b), resonance) in &self.resonances {
                if resonance.strength >= self.config.collective_threshold {
                    if a == entity_id {
                        group.insert(b);
                    } else if b == entity_id {
                        group.insert(a);
                    }
                }
            }

            // Add group if large enough
            if group.len() > 1 {
                potential_groups.push(group);
            }
        }

        // Form collectives from groups
        for group in potential_groups {
            // Check if any group members are already in collectives
            let mut new_group = true;
            for &member in &group {
                if let Some(&collective_id) = self.entity_collective.get(&member) {
                    // Add to existing collective
                    if collective_id < self.collectives.len() {
                        let phase = self
                            .entity_phases
                            .get(&member)
                            .map(|p| p.phase_vector.clone())
                            .unwrap_or([0.0; 8]);
                        let consciousness = self
                            .entity_phases
                            .get(&member)
                            .map(|p| p.consciousness)
                            .unwrap_or(0.5);

                        for &m in &group {
                            if !self.entity_collective.contains_key(&m) {
                                let _ = self.collectives[collective_id].add_member(
                                    m,
                                    &phase,
                                    consciousness,
                                );
                                self.entity_collective.insert(m, collective_id);
                            }
                        }
                        new_group = false;
                    }
                }
            }

            // Create new collective
            if new_group {
                let id = self.collectives.len();
                let mut collective = Collective::new(id);

                for &member in &group {
                    if let Some(phase) = self.entity_phases.get(&member) {
                        let _ =
                            collective.add_member(member, &phase.phase_vector, phase.consciousness);
                        self.entity_collective.insert(member, id);
                    }
                }

                self.collectives.push(collective);
            }
        }

        // Update statistics
        self.statistics.collective_count = self.collectives.len();
        self.statistics.entities_in_collectives = self.entity_collective.len();

        // Calculate average collective size
        if !self.collectives.is_empty() {
            let total: usize = self.collectives.iter().map(|c| c.members.len()).sum();
            self.statistics.average_collective_size =
                total as Float / self.collectives.len() as Float;
        }

        // Count cosmic collectives
        self.statistics.cosmic_collectives = self
            .collectives
            .iter()
            .filter(|c| c.members.len() > 100)
            .count();
    }

    /// Get entity's collective
    pub fn get_entity_collective(&self, entity_id: usize) -> Option<&Collective> {
        let collective_id = self.entity_collective.get(&entity_id)?;
        self.collectives.get(*collective_id)
    }

    /// Get resonance between two entities
    pub fn get_resonance(&self, entity_a: usize, entity_b: usize) -> Float {
        let key = if entity_a < entity_b {
            (entity_a, entity_b)
        } else {
            (entity_b, entity_a)
        };
        self.resonances.get(&key).map(|r| r.strength).unwrap_or(0.0)
    }

    /// Get all collectives
    pub fn get_collectives(&self) -> &[Collective] {
        &self.collectives
    }

    /// Apply collective experiences to field
    pub fn apply_collective_to_field(&self, field: &mut HolographicFieldState) {
        for collective in &self.collectives {
            if collective.members.is_empty() {
                continue;
            }

            // Get collective center (average of member positions)
            let mut center = [0.0, 0.0, 0.0];
            let mut count = 0;

            for &member_id in &collective.members {
                if let Some(phase) = self.entity_phases.get(&member_id) {
                    center[0] += phase.position[0];
                    center[1] += phase.position[1];
                    center[2] += phase.position[2];
                    count += 1;
                }
            }

            if count > 0 {
                center[0] /= count as Float;
                center[1] /= count as Float;
                center[2] /= count as Float;

                // Apply collective influence to field
                // The collective creates a stronger field signature
                let collective_strength = collective.get_coherence();

                // Add energy at collective center
                field.add_energy_at(center, 3, collective_strength * 2.0);
            }
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &SocialMemoryStatistics {
        &self.statistics
    }
}

/// Apply social memory influence to a field node
pub fn apply_social_memory_to_node(
    node: &mut OctreeNode,
    collective_coherence: Float,
    resonance_strength: Float,
) {
    // Collective consciousness boosts local field
    let boost = collective_coherence * resonance_strength * 0.1;

    // Increase coherence
    node.field_data.coherence = (node.field_data.coherence + boost).min(1.0);

    // Add energy
    node.field_data.energy += boost * 0.5;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_phase() {
        let phase = EntityPhase::new(0, [0.0, 0.0, 0.0]);
        assert_eq!(phase.entity_id, 0);
    }

    #[test]
    fn test_collective_creation() {
        let mut collective = Collective::new(0);
        let phase = [0.0; 8];
        let result = collective.add_member(1, &phase, 0.5);
        assert!(result);
        assert_eq!(collective.members.len(), 1);
    }

    #[test]
    fn test_social_memory() {
        let mut memory = SocialMemory::with_defaults();
        memory.register_entity(0, [0.0, 0.0, 0.0]);
        memory.register_entity(1, [1.0, 1.0, 1.0]);

        assert_eq!(memory.statistics.entity_count, 2);
    }

    #[test]
    fn test_resonance_computation() {
        let mut memory = SocialMemory::with_defaults();
        memory.register_entity(0, [0.0, 0.0, 0.0]);
        memory.register_entity(1, [1.0, 1.0, 1.0]);

        let resonance = memory.compute_resonance(0, 1);
        assert!(resonance >= 0.0 && resonance <= 1.0);
    }
}
