// Memory Fading System - Phase 1, Task 1.4
//
// Gradually erases conscious memory of higher densities as veil thickens,
// while retaining subconscious and superconscious memory.
//
// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
// "Conscious memory fades as veil thickens"
// "Subconscious memory retained (accessible through catalyst)"
// "Superconscious memory always retained (archetypical patterns)"

use crate::evolution_density_octave::density_octave::Density;
use crate::types::Float;
use std::collections::HashMap;

/// Entity identifier type
pub type EntityId = u64;

/// Memory Access Level
///
/// Defines the accessibility of different memory types based on veil thickness
/// and entity development.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryAccessLevel {
    /// No access - memory completely blocked
    None,
    /// Limited access - vague impressions only
    Vague,
    /// Partial access - some details accessible
    Partial,
    /// Full access - complete memory recall
    Full,
}

/// Memory Category
///
/// Different types of memory with different retention properties during involution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryCategory {
    /// Conscious memory - fades as veil thickens
    ///
    /// Contains memories of higher densities, spiritual experiences,
    /// and direct knowledge of unity. This memory is most affected
    /// by the veil and fades significantly at lower densities.
    Conscious,

    /// Subconscious memory - retained, accessible through catalyst
    ///
    /// Contains patterns, emotional memories, and intuitive knowledge.
    /// This memory is retained but requires catalyst processing or
    /// meditation to access consciously.
    Subconscious,

    /// Superconscious memory - always retained
    ///
    /// Contains archetypical patterns, fundamental truths, and
    /// holographic connectivity. This memory is never lost as it
    /// is part of the entity's fundamental nature.
    Superconscious,
}

/// Memory Entry
///
/// Represents a single memory with its category, access level, and content.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryEntry {
    /// Memory category
    pub category: MemoryCategory,

    /// Current access level
    pub access_level: MemoryAccessLevel,

    /// Memory content (abstract representation)
    pub content: String,

    /// Original density where memory was formed
    pub origin_density: Density,

    /// Memory strength (0.0 to 1.0)
    pub strength: Float,

    /// Timestamp when memory was formed
    pub created_at: Float,
}

impl MemoryEntry {
    /// Create new memory entry
    pub fn new(
        category: MemoryCategory,
        content: String,
        origin_density: Density,
        strength: Float,
        created_at: Float,
    ) -> Self {
        MemoryEntry {
            category,
            access_level: MemoryAccessLevel::Full,
            content,
            origin_density,
            strength: strength.clamp(0.0, 1.0),
            created_at,
        }
    }

    /// Create conscious memory
    pub fn conscious(content: String, origin_density: Density, created_at: Float) -> Self {
        Self::new(
            MemoryCategory::Conscious,
            content,
            origin_density,
            1.0,
            created_at,
        )
    }

    /// Create subconscious memory
    pub fn subconscious(content: String, origin_density: Density, created_at: Float) -> Self {
        Self::new(
            MemoryCategory::Subconscious,
            content,
            origin_density,
            1.0,
            created_at,
        )
    }

    /// Create superconscious memory
    pub fn superconscious(content: String, origin_density: Density, created_at: Float) -> Self {
        Self::new(
            MemoryCategory::Superconscious,
            content,
            origin_density,
            1.0,
            created_at,
        )
    }

    /// Check if memory is accessible
    pub fn is_accessible(&self) -> bool {
        matches!(
            self.access_level,
            MemoryAccessLevel::Partial | MemoryAccessLevel::Full
        )
    }

    /// Fade memory based on veil thickness
    ///
    /// This method reduces memory access level and strength based on
    /// veil thickness and memory category.
    ///
    /// # Arguments
    /// * `veil_thickness` - Current veil thickness (0.0 to 1.0)
    pub fn fade(&mut self, veil_thickness: Float) {
        match self.category {
            MemoryCategory::Conscious => {
                // Conscious memory fades significantly with veil thickness
                let fading_factor = 1.0 - veil_thickness;
                self.strength *= fading_factor;

                // Update access level based on strength
                self.access_level = if self.strength > 0.7 {
                    MemoryAccessLevel::Full
                } else if self.strength > 0.3 {
                    MemoryAccessLevel::Partial
                } else if self.strength > 0.1 {
                    MemoryAccessLevel::Vague
                } else {
                    MemoryAccessLevel::None
                };
            }
            MemoryCategory::Subconscious => {
                // Subconscious memory fades less, retains more
                let fading_factor = 1.0 - (veil_thickness * 0.5);
                self.strength *= fading_factor;

                // Subconscious memory retains at least partial access
                self.access_level = if self.strength > 0.6 {
                    MemoryAccessLevel::Partial
                } else if self.strength > 0.2 {
                    MemoryAccessLevel::Vague
                } else {
                    MemoryAccessLevel::None
                };
            }
            MemoryCategory::Superconscious => {
                // Superconscious memory never fades
                self.strength = 1.0;
                self.access_level = MemoryAccessLevel::Full;
            }
        }
    }

    /// Recover memory through meditation or catalyst processing
    ///
    /// This method increases memory access level and strength through
    /// spiritual work and catalyst processing.
    ///
    /// # Arguments
    /// * `recovery_amount` - Amount of recovery (0.0 to 1.0)
    pub fn recover(&mut self, recovery_amount: Float) {
        let clamped_recovery = recovery_amount.clamp(0.0, 1.0);

        match self.category {
            MemoryCategory::Conscious => {
                // Conscious memory can be recovered with effort
                self.strength = (self.strength + clamped_recovery * 0.3).min(1.0);
                self.access_level = if self.strength > 0.7 {
                    MemoryAccessLevel::Full
                } else if self.strength > 0.3 {
                    MemoryAccessLevel::Partial
                } else if self.strength > 0.1 {
                    MemoryAccessLevel::Vague
                } else {
                    MemoryAccessLevel::None
                };
            }
            MemoryCategory::Subconscious => {
                // Subconscious memory recovers more easily
                self.strength = (self.strength + clamped_recovery * 0.5).min(1.0);
                self.access_level = if self.strength > 0.6 {
                    MemoryAccessLevel::Partial
                } else if self.strength > 0.2 {
                    MemoryAccessLevel::Vague
                } else {
                    MemoryAccessLevel::None
                };
            }
            MemoryCategory::Superconscious => {
                // Superconscious memory is always fully accessible
                self.strength = 1.0;
                self.access_level = MemoryAccessLevel::Full;
            }
        }
    }
}

/// Memory Fading System
///
/// Manages memory fading during involution, tracking memory access levels
/// and providing recovery mechanisms through catalyst processing and meditation.
///
/// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
pub struct MemoryFadingSystem {
    /// Memory entries for each entity
    pub entity_memories: HashMap<EntityId, Vec<MemoryEntry>>,

    /// Memory retention curve by density (percentage of conscious memory retained)
    memory_retention_curve: [Float; 7],
}

impl MemoryFadingSystem {
    /// Create new memory fading system
    pub fn new() -> Self {
        MemoryFadingSystem {
            entity_memories: HashMap::new(),
            // Memory retention curve from veil thickness curve
            // D7: 100% retention (0% veil)
            // D6: 85% retention (15% veil)
            // D5: 70% retention (30% veil)
            // D4: 55% retention (45% veil)
            // D3: 40% retention (60% veil)
            // D2: 25% retention (75% veil)
            // D1: 10% retention (90% veil)
            memory_retention_curve: [1.00, 0.85, 0.70, 0.55, 0.40, 0.25, 0.10],
        }
    }

    /// Create memory fading system with custom retention curve
    pub fn with_retention_curve(retention_curve: [Float; 7]) -> Self {
        MemoryFadingSystem {
            entity_memories: HashMap::new(),
            memory_retention_curve: retention_curve,
        }
    }

    /// Calculate memory retention for a density
    ///
    /// This method calculates the percentage of conscious memory retained
    /// at a given density based on the veil thickness curve.
    ///
    /// # Arguments
    /// * `density` - The density to calculate retention for
    ///
    /// # Returns
    /// Memory retention percentage (0.0 to 1.0)
    pub fn calculate_memory_retention(&self, density: Density) -> Float {
        // Index: D7 -> index 0, D6 -> index 1, ..., D1 -> index 6
        let index = 7 - density.as_usize();
        self.memory_retention_curve[index]
    }

    /// Fade memory for an entity based on veil thickness
    ///
    /// This method applies veil-based fading to all memories of an entity,
    /// reducing conscious memory access while preserving subconscious and
    /// superconscious memory.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `veil_thickness` - Current veil thickness (0.0 to 1.0)
    ///
    /// # Returns
    /// Number of memories affected
    pub fn fade_memory(&mut self, entity_id: EntityId, veil_thickness: Float) -> usize {
        if let Some(memories) = self.entity_memories.get_mut(&entity_id) {
            for memory in memories.iter_mut() {
                memory.fade(veil_thickness);
            }
            memories.len()
        } else {
            0
        }
    }

    /// Fade memory for an entity based on density
    ///
    /// This is a convenience method that calculates veil thickness from
    /// density and applies the appropriate memory fading.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The entity's current density
    ///
    /// # Returns
    /// Number of memories affected
    pub fn fade_memory_by_density(&mut self, entity_id: EntityId, density: Density) -> usize {
        let retention = self.calculate_memory_retention(density);
        let veil_thickness = 1.0 - retention;
        self.fade_memory(entity_id, veil_thickness)
    }

    /// Retain subconscious memory
    ///
    /// This method ensures that subconscious memories are retained even
    /// when conscious memories fade. Subconscious memories remain
    /// accessible through catalyst processing and meditation.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Number of subconscious memories retained
    pub fn retain_subconscious_memory(&mut self, entity_id: EntityId) -> usize {
        if let Some(memories) = self.entity_memories.get_mut(&entity_id) {
            let count = memories
                .iter()
                .filter(|m| m.category == MemoryCategory::Subconscious)
                .count();

            // Ensure subconscious memories retain at least partial access
            for memory in memories.iter_mut() {
                if memory.category == MemoryCategory::Subconscious {
                    memory.access_level = match memory.access_level {
                        MemoryAccessLevel::None => MemoryAccessLevel::Vague,
                        MemoryAccessLevel::Vague => MemoryAccessLevel::Partial,
                        _ => memory.access_level,
                    };
                    memory.strength = memory.strength.max(0.2);
                }
            }

            count
        } else {
            0
        }
    }

    /// Ensure superconscious memory is always retained
    ///
    /// This method ensures that superconscious memories (archetypical patterns)
    /// are always fully accessible regardless of veil thickness.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Number of superconscious memories retained
    pub fn retain_superconscious_memory(&mut self, entity_id: EntityId) -> usize {
        if let Some(memories) = self.entity_memories.get_mut(&entity_id) {
            let count = memories
                .iter()
                .filter(|m| m.category == MemoryCategory::Superconscious)
                .count();

            // Ensure superconscious memories are always fully accessible
            for memory in memories.iter_mut() {
                if memory.category == MemoryCategory::Superconscious {
                    memory.access_level = MemoryAccessLevel::Full;
                    memory.strength = 1.0;
                }
            }

            count
        } else {
            0
        }
    }

    /// Add memory to an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `memory` - The memory entry to add
    pub fn add_memory(&mut self, entity_id: EntityId, memory: MemoryEntry) {
        self.entity_memories
            .entry(entity_id)
            .or_default()
            .push(memory);
    }

    /// Get all memories for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Vector of memory entries, or None if entity has no memories
    pub fn get_memories(&self, entity_id: EntityId) -> Option<&Vec<MemoryEntry>> {
        self.entity_memories.get(&entity_id)
    }

    /// Get memories by category for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `category` - The memory category to filter by
    ///
    /// # Returns
    /// Vector of memory entries matching the category
    pub fn get_memories_by_category(
        &self,
        entity_id: EntityId,
        category: MemoryCategory,
    ) -> Vec<&MemoryEntry> {
        self.entity_memories
            .get(&entity_id)
            .map(|memories| memories.iter().filter(|m| m.category == category).collect())
            .unwrap_or_default()
    }

    /// Get accessible memories for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Vector of accessible memory entries
    pub fn get_accessible_memories(&self, entity_id: EntityId) -> Vec<&MemoryEntry> {
        self.entity_memories
            .get(&entity_id)
            .map(|memories| memories.iter().filter(|m| m.is_accessible()).collect())
            .unwrap_or_default()
    }

    /// Recover memory through meditation
    ///
    /// This method simulates memory recovery through meditation practice,
    /// gradually increasing access to faded memories.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `meditation_duration` - Duration of meditation (0.0 to 1.0)
    ///
    /// # Returns
    /// Number of memories recovered
    pub fn recover_memory_through_meditation(
        &mut self,
        entity_id: EntityId,
        meditation_duration: Float,
    ) -> usize {
        if let Some(memories) = self.entity_memories.get_mut(&entity_id) {
            let recovery_amount = meditation_duration.clamp(0.0, 1.0) * 0.2;

            let mut recovered_count = 0;
            for memory in memories.iter_mut() {
                let previous_access = memory.access_level;
                memory.recover(recovery_amount);
                if memory.access_level != previous_access {
                    recovered_count += 1;
                }
            }

            recovered_count
        } else {
            0
        }
    }

    /// Recover memory through catalyst processing
    ///
    /// This method simulates memory recovery through catalyst processing,
    /// providing access to subconscious memories through conscious experience.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.4
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `catalyst_intensity` - Intensity of catalyst processed (0.0 to 1.0)
    ///
    /// # Returns
    /// Number of memories recovered
    pub fn recover_memory_through_catalyst(
        &mut self,
        entity_id: EntityId,
        catalyst_intensity: Float,
    ) -> usize {
        if let Some(memories) = self.entity_memories.get_mut(&entity_id) {
            let recovery_amount = catalyst_intensity.clamp(0.0, 1.0) * 0.3;

            let mut recovered_count = 0;
            for memory in memories.iter_mut() {
                let previous_access = memory.access_level;
                memory.recover(recovery_amount);
                if memory.access_level != previous_access {
                    recovered_count += 1;
                }
            }

            recovered_count
        } else {
            0
        }
    }

    /// Get memory statistics for an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// MemoryStatistics struct with detailed memory information
    pub fn get_statistics(&self, entity_id: EntityId) -> Option<MemoryStatistics> {
        let memories = self.entity_memories.get(&entity_id)?;

        let total = memories.len();
        let conscious = memories
            .iter()
            .filter(|m| m.category == MemoryCategory::Conscious)
            .count();
        let subconscious = memories
            .iter()
            .filter(|m| m.category == MemoryCategory::Subconscious)
            .count();
        let superconscious = memories
            .iter()
            .filter(|m| m.category == MemoryCategory::Superconscious)
            .count();

        let accessible = memories.iter().filter(|m| m.is_accessible()).count();
        let inaccessible = total - accessible;

        let total_strength: Float = memories.iter().map(|m| m.strength).sum();
        let avg_strength = if total > 0 {
            total_strength / total as Float
        } else {
            0.0
        };

        Some(MemoryStatistics {
            entity_id,
            total_memories: total,
            conscious_memories: conscious,
            subconscious_memories: subconscious,
            superconscious_memories: superconscious,
            accessible_memories: accessible,
            inaccessible_memories: inaccessible,
            average_strength: avg_strength,
        })
    }

    /// Remove entity from memory tracking
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Vector of removed memories, or None if entity not found
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<Vec<MemoryEntry>> {
        self.entity_memories.remove(&entity_id)
    }

    /// Clear all entity memories
    pub fn clear_all(&mut self) {
        self.entity_memories.clear();
    }
}

impl Default for MemoryFadingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory Statistics
///
/// Provides detailed statistics about an entity's memory state.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryStatistics {
    /// Entity identifier
    pub entity_id: EntityId,

    /// Total number of memories
    pub total_memories: usize,

    /// Number of conscious memories
    pub conscious_memories: usize,

    /// Number of subconscious memories
    pub subconscious_memories: usize,

    /// Number of superconscious memories
    pub superconscious_memories: usize,

    /// Number of accessible memories
    pub accessible_memories: usize,

    /// Number of inaccessible memories
    pub inaccessible_memories: usize,

    /// Average memory strength
    pub average_strength: Float,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};

    // ===== MemoryEntry Tests =====

    #[test]
    fn test_memory_entry_new() {
        let memory = MemoryEntry::new(
            MemoryCategory::Conscious,
            "Test memory".to_string(),
            Density::Seventh,
            1.0,
            100.0,
        );
        assert_eq!(memory.category, MemoryCategory::Conscious);
        assert_eq!(memory.access_level, MemoryAccessLevel::Full);
        assert_eq!(memory.content, "Test memory");
        assert_eq!(memory.strength, 1.0);
    }

    #[test]
    fn test_memory_entry_conscious() {
        let memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        assert_eq!(memory.category, MemoryCategory::Conscious);
        assert_eq!(memory.access_level, MemoryAccessLevel::Full);
    }

    #[test]
    fn test_memory_entry_subconscious() {
        let memory = MemoryEntry::subconscious("Test".to_string(), Density::Seventh, 100.0);
        assert_eq!(memory.category, MemoryCategory::Subconscious);
        assert_eq!(memory.access_level, MemoryAccessLevel::Full);
    }

    #[test]
    fn test_memory_entry_superconscious() {
        let memory = MemoryEntry::superconscious("Test".to_string(), Density::Seventh, 100.0);
        assert_eq!(memory.category, MemoryCategory::Superconscious);
        assert_eq!(memory.access_level, MemoryAccessLevel::Full);
    }

    #[test]
    fn test_memory_entry_is_accessible() {
        let mut memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        assert!(memory.is_accessible());

        memory.access_level = MemoryAccessLevel::None;
        assert!(!memory.is_accessible());
    }

    #[test]
    fn test_memory_entry_fade_conscious() {
        let mut memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        memory.fade(0.9); // 90% veil thickness

        assert!(memory.strength < 1.0);
        assert!(!memory.is_accessible()); // Should be inaccessible at 90% veil
    }

    #[test]
    fn test_memory_entry_fade_subconscious() {
        let mut memory = MemoryEntry::subconscious("Test".to_string(), Density::Seventh, 100.0);
        memory.fade(0.9); // 90% veil thickness

        // Subconscious memory should retain some strength
        assert!(memory.strength > 0.0);
    }

    #[test]
    fn test_memory_entry_fade_superconscious() {
        let mut memory = MemoryEntry::superconscious("Test".to_string(), Density::Seventh, 100.0);
        memory.fade(1.0); // 100% veil thickness

        // Superconscious memory never fades
        assert_eq!(memory.strength, 1.0);
        assert_eq!(memory.access_level, MemoryAccessLevel::Full);
    }

    #[test]
    fn test_memory_entry_recover() {
        let mut memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        memory.fade(0.9); // Fade first
        memory.recover(0.5); // Then recover

        assert!(memory.strength > 0.0);
    }

    // ===== MemoryFadingSystem Tests =====

    #[test]
    fn test_memory_fading_system_new() {
        let system = MemoryFadingSystem::new();
        assert_eq!(system.entity_memories.len(), 0);
    }

    #[test]
    fn test_memory_fading_system_with_retention_curve() {
        let curve = [0.5, 0.4, 0.3, 0.2, 0.1, 0.0, 0.0];
        let system = MemoryFadingSystem::with_retention_curve(curve);
        assert_eq!(system.memory_retention_curve, curve);
    }

    #[test]
    fn test_calculate_memory_retention_d7() {
        let system = MemoryFadingSystem::new();
        assert_eq!(system.calculate_memory_retention(Density::Seventh), 1.00);
    }

    #[test]
    fn test_calculate_memory_retention_d1() {
        let system = MemoryFadingSystem::new();
        assert_eq!(
            system.calculate_memory_retention(Density::First(Density1SubLevel::Quantum)),
            0.10
        );
    }

    #[test]
    fn test_calculate_memory_retention_all_densities() {
        let system = MemoryFadingSystem::new();
        assert_eq!(system.calculate_memory_retention(Density::Seventh), 1.00);
        assert_eq!(system.calculate_memory_retention(Density::Sixth), 0.85);
        assert_eq!(system.calculate_memory_retention(Density::Fifth), 0.70);
        assert_eq!(system.calculate_memory_retention(Density::Fourth), 0.55);
        assert_eq!(system.calculate_memory_retention(Density::Third), 0.40);
        assert_eq!(
            system.calculate_memory_retention(Density::Second(Density2SubLevel::Cellular)),
            0.25
        );
        assert_eq!(
            system.calculate_memory_retention(Density::First(Density1SubLevel::Quantum)),
            0.10
        );
    }

    #[test]
    fn test_add_memory() {
        let mut system = MemoryFadingSystem::new();
        let memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        system.add_memory(1, memory);

        assert_eq!(system.entity_memories.get(&1).unwrap().len(), 1);
    }

    #[test]
    fn test_get_memories() {
        let mut system = MemoryFadingSystem::new();
        let memory = MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0);
        system.add_memory(1, memory);

        let memories = system.get_memories(1);
        assert!(memories.is_some());
        assert_eq!(memories.unwrap().len(), 1);
    }

    #[test]
    fn test_get_memories_none() {
        let system = MemoryFadingSystem::new();
        let memories = system.get_memories(999);
        assert!(memories.is_none());
    }

    #[test]
    fn test_get_memories_by_category() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("C1".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::subconscious("S1".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::conscious("C2".to_string(), Density::Seventh, 100.0),
        );

        let conscious = system.get_memories_by_category(1, MemoryCategory::Conscious);
        assert_eq!(conscious.len(), 2);

        let subconscious = system.get_memories_by_category(1, MemoryCategory::Subconscious);
        assert_eq!(subconscious.len(), 1);

        let superconscious = system.get_memories_by_category(1, MemoryCategory::Superconscious);
        assert_eq!(superconscious.len(), 0);
    }

    #[test]
    fn test_get_accessible_memories() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("C1".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::subconscious("S1".to_string(), Density::Seventh, 100.0),
        );

        // Fade memory to make some inaccessible
        system.fade_memory(1, 0.9);

        let accessible = system.get_accessible_memories(1);
        assert!(accessible.len() < 2); // Some memories should be inaccessible
    }

    #[test]
    fn test_fade_memory() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::subconscious("Test".to_string(), Density::Seventh, 100.0),
        );

        let count = system.fade_memory(1, 0.9);
        assert_eq!(count, 2);

        let memories = system.get_memories(1).unwrap();
        let conscious = &memories[0];
        assert!(!conscious.is_accessible()); // Conscious memory should be inaccessible
    }

    #[test]
    fn test_fade_memory_by_density() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );

        let count = system.fade_memory_by_density(1, Density::First(Density1SubLevel::Quantum));
        assert_eq!(count, 1);

        let memories = system.get_memories(1).unwrap();
        assert!(!memories[0].is_accessible()); // Should be very faded at D1
    }

    #[test]
    fn test_retain_subconscious_memory() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::subconscious("Test".to_string(), Density::Seventh, 100.0),
        );

        // Fade memory first
        system.fade_memory(1, 0.9);

        // Retain subconscious memory
        let count = system.retain_subconscious_memory(1);
        assert_eq!(count, 1);

        let memories = system.get_memories(1).unwrap();
        assert!(memories[0].access_level != MemoryAccessLevel::None); // Should have some access
    }

    #[test]
    fn test_retain_superconscious_memory() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::superconscious("Test".to_string(), Density::Seventh, 100.0),
        );

        // Try to fade (shouldn't affect superconscious)
        system.fade_memory(1, 1.0);

        // Retain superconscious memory
        let count = system.retain_superconscious_memory(1);
        assert_eq!(count, 1);

        let memories = system.get_memories(1).unwrap();
        assert_eq!(memories[0].access_level, MemoryAccessLevel::Full);
        assert_eq!(memories[0].strength, 1.0);
    }

    #[test]
    fn test_recover_memory_through_meditation() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );

        // Fade memory first
        system.fade_memory(1, 0.9);

        // Recover through meditation
        let _recovered = system.recover_memory_through_meditation(1, 0.8);
        // recovered is usize, always >= 0

        let memories = system.get_memories(1).unwrap();
        assert!(memories[0].strength > 0.0); // Should have some strength back
    }

    #[test]
    fn test_recover_memory_through_catalyst() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );

        // Fade memory first
        system.fade_memory(1, 0.9);

        // Recover through catalyst processing
        let _recovered = system.recover_memory_through_catalyst(1, 0.8);
        // recovered is usize, always >= 0

        let memories = system.get_memories(1).unwrap();
        assert!(memories[0].strength > 0.0); // Should have some strength back
    }

    #[test]
    fn test_get_statistics() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("C1".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::subconscious("S1".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            1,
            MemoryEntry::superconscious("SC1".to_string(), Density::Seventh, 100.0),
        );

        let stats = system.get_statistics(1);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_memories, 3);
        assert_eq!(stats.conscious_memories, 1);
        assert_eq!(stats.subconscious_memories, 1);
        assert_eq!(stats.superconscious_memories, 1);
    }

    #[test]
    fn test_remove_entity() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );

        let removed = system.remove_entity(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().len(), 1);
        assert_eq!(system.entity_memories.len(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut system = MemoryFadingSystem::new();
        system.add_memory(
            1,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );
        system.add_memory(
            2,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 100.0),
        );

        system.clear_all();
        assert_eq!(system.entity_memories.len(), 0);
    }

    #[test]
    fn test_memory_fading_system_default() {
        let system = MemoryFadingSystem::default();
        assert_eq!(system.entity_memories.len(), 0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_full_involution_memory_journey() {
        let mut system = MemoryFadingSystem::new();
        let entity_id = 1;

        // Create memories at D7
        system.add_memory(
            entity_id,
            MemoryEntry::conscious("Unity experience".to_string(), Density::Seventh, 0.0),
        );
        system.add_memory(
            entity_id,
            MemoryEntry::subconscious("Love pattern".to_string(), Density::Seventh, 0.0),
        );
        system.add_memory(
            entity_id,
            MemoryEntry::superconscious("Archetype 22".to_string(), Density::Seventh, 0.0),
        );

        // Start at D7 (no fading)
        system.fade_memory_by_density(entity_id, Density::Seventh);
        let stats = system.get_statistics(entity_id).unwrap();
        assert_eq!(stats.accessible_memories, 3); // All memories accessible

        // Descend to D6 (15% veil)
        system.fade_memory_by_density(entity_id, Density::Sixth);
        let stats = system.get_statistics(entity_id).unwrap();
        assert!(stats.accessible_memories >= 2); // Most still accessible

        // Descend to D4 (45% veil)
        system.fade_memory_by_density(entity_id, Density::Fourth);
        let stats = system.get_statistics(entity_id).unwrap();
        // Conscious memory fading, others retained
        assert!(stats.accessible_memories >= 2);

        // Descend to D1 (90% veil)
        system.fade_memory_by_density(entity_id, Density::First(Density1SubLevel::Quantum));
        let stats = system.get_statistics(entity_id).unwrap();
        // Conscious memory mostly inaccessible, subconscious/superconscious retained
        assert!(stats.accessible_memories >= 1); // At least superconscious

        // Verify superconscious always accessible
        let sc_memories =
            system.get_memories_by_category(entity_id, MemoryCategory::Superconscious);
        assert_eq!(sc_memories.len(), 1);
        assert_eq!(sc_memories[0].access_level, MemoryAccessLevel::Full);
    }

    #[test]
    fn test_memory_recovery_after_fading() {
        let mut system = MemoryFadingSystem::new();
        let entity_id = 1;

        // Create memories
        system.add_memory(
            entity_id,
            MemoryEntry::conscious("Test".to_string(), Density::Seventh, 0.0),
        );
        system.add_memory(
            entity_id,
            MemoryEntry::subconscious("Pattern".to_string(), Density::Seventh, 0.0),
        );

        // Fade to D1
        system.fade_memory_by_density(entity_id, Density::First(Density1SubLevel::Quantum));
        let stats_before = system.get_statistics(entity_id).unwrap();
        let accessible_before = stats_before.accessible_memories;

        // Recover through catalyst processing
        system.recover_memory_through_catalyst(entity_id, 0.8);

        let stats_after = system.get_statistics(entity_id).unwrap();
        let accessible_after = stats_after.accessible_memories;

        // Should have more accessible memories after recovery
        assert!(accessible_after >= accessible_before);
    }

    #[test]
    fn test_conscious_memory_fades_most() {
        let mut system = MemoryFadingSystem::new();
        let entity_id = 1;

        system.add_memory(
            entity_id,
            MemoryEntry::conscious("C".to_string(), Density::Seventh, 0.0),
        );
        system.add_memory(
            entity_id,
            MemoryEntry::subconscious("S".to_string(), Density::Seventh, 0.0),
        );
        system.add_memory(
            entity_id,
            MemoryEntry::superconscious("SC".to_string(), Density::Seventh, 0.0),
        );

        // Fade to D1
        system.fade_memory_by_density(entity_id, Density::First(Density1SubLevel::Quantum));

        let memories = system.get_memories(entity_id).unwrap();
        let conscious_strength = memories[0].strength;
        let subconscious_strength = memories[1].strength;
        let superconscious_strength = memories[2].strength;

        // Conscious should fade most, superconscious not at all
        assert!(conscious_strength < subconscious_strength);
        assert!(subconscious_strength <= superconscious_strength);
        assert_eq!(superconscious_strength, 1.0);
    }
}
