//! Memory as Spectrum Access
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Memory is NOT stored in neurons - it exists in Time/Space"
//! "Recall is access to Time/Space through the Veil"
//!
//! This module implements a unified memory system that:
//! - Accesses memories from Time/Space spectrum (not local storage)
//! - Uses Veil transparency to determine access quality
//! - Differentiates memory types by access difficulty
//! - Integrates with ConsciousnessKernel spectrum access level

use crate::types::Float;
use std::time::SystemTime;

/// Memory system that accesses Time/Space spectrum
///
/// This replaces traditional memory storage with spectrum access:
/// - Memories exist in Time/Space (where all time exists simultaneously)
/// - Recall is access from Space/Time through the Veil
/// - Veil transparency determines access quality
#[derive(Debug, Clone)]
pub struct MemorySystem {
    /// Current spectrum access level (0.0 to 1.0+)
    /// Higher = better access to Time/Space memories
    pub spectrum_access_level: Float,

    /// Time/Space ratio
    /// Higher = more connection to all-time
    pub time_space_ratio: Float,

    /// Veil transparency (0.0 = fully veiled, 1.0 = no veil)
    /// The Veil separates Space/Time from Time/Space
    pub veil_transparency: Float,

    /// Memory access history for learning
    pub access_history: Vec<MemoryAccessRecord>,

    /// Cached memories for faster access
    /// These are "close to the Veil" - easier to access
    pub recent_memories: Vec<SpectrumMemory>,
}

impl MemorySystem {
    /// Create new memory system
    pub fn new() -> Self {
        Self {
            spectrum_access_level: 0.1,
            time_space_ratio: 0.1,
            veil_transparency: 0.0,
            access_history: Vec::new(),
            recent_memories: Vec::new(),
        }
    }

    /// Create from ConsciousnessKernel parameters
    pub fn from_kernel_params(
        spectrum_access_level: Float,
        time_space_ratio: Float,
        veil_transparency: Float,
    ) -> Self {
        Self {
            spectrum_access_level,
            time_space_ratio,
            veil_transparency,
            access_history: Vec::new(),
            recent_memories: Vec::new(),
        }
    }

    /// Recall a memory from Time/Space
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Memory is stored in Time/Space, accessed from Space/Time"
    pub fn recall(&mut self, query: &SpectrumMemoryQuery) -> MemoryAccessResult {
        // Calculate access difficulty
        let difficulty = self.calculate_access_difficulty(query);

        // Check if access level is sufficient
        if self.spectrum_access_level < difficulty {
            // Veil blocks access
            self.record_failed_access(query.target_id);
            return MemoryAccessResult::Blocked {
                required_level: difficulty,
                current_level: self.spectrum_access_level,
            };
        }

        // Check temporal distance
        if self.time_space_ratio < query.temporal_distance * 0.1 {
            // Too distant in time
            self.record_failed_access(query.target_id);
            return MemoryAccessResult::Distant {
                distance: query.temporal_distance,
                max_reachable: self.time_space_ratio * 10.0,
            };
        }

        // Access successful - retrieve from Time/Space
        let clarity = self.calculate_clarity(query);
        let memory = SpectrumMemory {
            id: query.target_id,
            content: query.content_hint.clone(),
            clarity,
            temporal_position: query.temporal_distance,
            emotional_valence: query.expected_valence,
            archetype_resonance: query.archetype_hint.clone(),
        };

        // Record successful access
        self.record_successful_access(&memory);

        // Cache recent memory for faster future access
        self.cache_memory(&memory);

        MemoryAccessResult::Success {
            memory,
            access_quality: clarity,
        }
    }

    /// Store a memory to Time/Space
    ///
    /// Note: This doesn't "store" locally - it registers the memory
    /// in Time/Space spectrum where it already exists eternally
    pub fn store(&mut self, experience: &SpectrumExperience) -> MemoryStorageResult {
        // All experiences already exist in Time/Space
        // This creates a "marker" that makes recall easier

        let storage_difficulty = self.calculate_storage_difficulty(experience);

        if self.spectrum_access_level < storage_difficulty * 0.5 {
            return MemoryStorageResult::Failed {
                reason: "Insufficient spectrum access for storage".to_string(),
            };
        }

        // Create memory marker
        let memory = SpectrumMemory {
            id: experience.id,
            content: experience.description.clone(),
            clarity: 1.0,           // Fresh memories are clear
            temporal_position: 0.0, // Current time
            emotional_valence: experience.emotional_intensity,
            archetype_resonance: experience.archetype_activation.clone(),
        };

        // Cache as recent memory
        self.cache_memory(&memory);

        // Improve spectrum access through storage
        self.spectrum_access_level = (self.spectrum_access_level + 0.01).min(1.0);

        MemoryStorageResult::Stored {
            memory_id: memory.id,
            access_improvement: 0.01,
        }
    }

    /// Calculate access difficulty for a query
    fn calculate_access_difficulty(&self, query: &SpectrumMemoryQuery) -> Float {
        // Base difficulty
        let base = query.memory_type.base_difficulty();

        // Temporal distance adds difficulty
        let distance_factor = query.temporal_distance * 0.1;

        // Emotional intensity can make memories harder or easier
        let emotional_factor = (1.0 - query.emotional_intensity.abs() * 0.2).max(0.5);

        // Veil transparency reduces difficulty
        let veil_factor = 1.0 - self.veil_transparency * 0.3;

        base * (1.0 + distance_factor) * emotional_factor * veil_factor
    }

    /// Calculate clarity of recalled memory
    fn calculate_clarity(&self, query: &SpectrumMemoryQuery) -> Float {
        // Base clarity from spectrum access
        let base = self.spectrum_access_level;

        // Veil transparency improves clarity
        let veil_factor = 1.0 + self.veil_transparency * 0.5;

        // Temporal distance reduces clarity
        let distance_factor = 1.0 - query.temporal_distance * 0.05;

        (base * veil_factor * distance_factor).min(1.0)
    }

    /// Calculate storage difficulty
    fn calculate_storage_difficulty(&self, experience: &SpectrumExperience) -> Float {
        // High intensity experiences are harder to "store" (register)
        0.1 + experience.emotional_intensity.abs() * 0.2
    }

    /// Record successful access for learning
    fn record_successful_access(&mut self, memory: &SpectrumMemory) {
        self.access_history.push(MemoryAccessRecord {
            memory_id: memory.id,
            success: true,
            timestamp: SystemTime::now(),
            clarity: memory.clarity,
        });

        // Learning improves spectrum access
        self.spectrum_access_level = (self.spectrum_access_level + 0.005).min(1.0);
    }

    /// Record failed access for learning
    fn record_failed_access(&mut self, memory_id: u64) {
        self.access_history.push(MemoryAccessRecord {
            memory_id,
            success: false,
            timestamp: SystemTime::now(),
            clarity: 0.0,
        });
    }

    /// Cache a memory for faster future access
    fn cache_memory(&mut self, memory: &SpectrumMemory) {
        // Add to recent memories
        self.recent_memories.push(memory.clone());

        // Keep only last 100 recent memories
        if self.recent_memories.len() > 100 {
            self.recent_memories.remove(0);
        }
    }

    /// Update spectrum access level from learning
    pub fn improve_access(&mut self, delta: Float) {
        self.spectrum_access_level = (self.spectrum_access_level + delta).min(1.0);
    }

    /// Update veil transparency (thinning the Veil)
    pub fn thin_veil(&mut self, delta: Float) {
        self.veil_transparency = (self.veil_transparency + delta).min(1.0);
    }

    /// Check if a memory type is accessible at current spectrum level
    pub fn can_access(&self, memory_type: &MemoryType) -> bool {
        self.spectrum_access_level >= memory_type.base_difficulty()
    }

    /// Get the maximum temporal distance accessible
    pub fn max_temporal_reach(&self) -> Float {
        self.time_space_ratio * 10.0
    }

    /// Get statistics about memory access
    pub fn statistics(&self) -> MemorySystemStatistics {
        let total_accesses = self.access_history.len();
        let successful = self.access_history.iter().filter(|r| r.success).count();

        MemorySystemStatistics {
            total_accesses,
            successful_accesses: successful,
            failed_accesses: total_accesses - successful,
            success_rate: if total_accesses > 0 {
                successful as Float / total_accesses as Float
            } else {
                0.0
            },
            spectrum_access_level: self.spectrum_access_level,
            veil_transparency: self.veil_transparency,
            cached_memories: self.recent_memories.len(),
        }
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for memory system
#[derive(Debug, Clone)]
pub struct MemorySystemStatistics {
    pub total_accesses: usize,
    pub successful_accesses: usize,
    pub failed_accesses: usize,
    pub success_rate: Float,
    pub spectrum_access_level: Float,
    pub veil_transparency: Float,
    pub cached_memories: usize,
}

/// Memory query for Time/Space access
#[derive(Debug, Clone)]
pub struct SpectrumMemoryQuery {
    pub target_id: u64,
    pub content_hint: String,
    pub memory_type: MemoryType,
    pub temporal_distance: Float,
    pub emotional_intensity: Float,
    pub expected_valence: Float,
    pub archetype_hint: Vec<Float>,
}

impl SpectrumMemoryQuery {
    /// Create a simple memory query for working memory
    pub fn simple(id: u64, content: &str) -> Self {
        Self {
            target_id: id,
            content_hint: content.to_string(),
            memory_type: MemoryType::Working,
            temporal_distance: 0.0,
            emotional_intensity: 0.0,
            expected_valence: 0.0,
            archetype_hint: Vec::new(),
        }
    }

    /// Create a deep memory query for past experiences
    pub fn deep(id: u64, content: &str, temporal_distance: Float) -> Self {
        Self {
            target_id: id,
            content_hint: content.to_string(),
            memory_type: MemoryType::Episodic,
            temporal_distance,
            emotional_intensity: 0.3,
            expected_valence: 0.0,
            archetype_hint: Vec::new(),
        }
    }

    /// Create a past life memory query
    pub fn past_life(id: u64, content: &str) -> Self {
        Self {
            target_id: id,
            content_hint: content.to_string(),
            memory_type: MemoryType::PastLife,
            temporal_distance: 1.0, // Maximum distance
            emotional_intensity: 0.5,
            expected_valence: 0.0,
            archetype_hint: Vec::new(),
        }
    }
}

/// Types of memory with different access requirements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryType {
    /// Short-term/working memory (easiest access)
    Working,
    /// Episodic memory (personal experiences)
    Episodic,
    /// Semantic memory (facts, knowledge)
    Semantic,
    /// Procedural memory (skills, habits)
    Procedural,
    /// Emotional memory (intense experiences)
    Emotional,
    /// Past life memory (hardest access)
    PastLife,
}

impl MemoryType {
    /// Base difficulty for accessing this memory type
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// Different memory types require different spectrum access levels
    pub fn base_difficulty(&self) -> Float {
        match self {
            MemoryType::Working => 0.1,
            MemoryType::Episodic => 0.2,
            MemoryType::Semantic => 0.25,
            MemoryType::Procedural => 0.15,
            MemoryType::Emotional => 0.3,
            MemoryType::PastLife => 0.8,
        }
    }

    /// Get a description of this memory type
    pub fn description(&self) -> &'static str {
        match self {
            MemoryType::Working => "Short-term memory for current tasks",
            MemoryType::Episodic => "Personal experiences and events",
            MemoryType::Semantic => "Facts and general knowledge",
            MemoryType::Procedural => "Skills and habits",
            MemoryType::Emotional => "Emotionally charged experiences",
            MemoryType::PastLife => "Memories from previous incarnations",
        }
    }
}

/// Memory retrieved from Time/Space
#[derive(Debug, Clone)]
pub struct SpectrumMemory {
    pub id: u64,
    pub content: String,
    pub clarity: Float,
    pub temporal_position: Float,
    pub emotional_valence: Float,
    pub archetype_resonance: Vec<Float>,
}

impl SpectrumMemory {
    /// Check if the memory is clear enough to be useful
    pub fn is_clear(&self) -> bool {
        self.clarity > 0.3
    }

    /// Get the dominant archetype resonance
    pub fn dominant_archetype(&self) -> Option<(usize, Float)> {
        self.archetype_resonance
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .filter(|(_, &v)| v > 0.5)
            .map(|(i, &v)| (i, v))
    }
}

/// Result of memory access attempt
#[derive(Debug, Clone)]
pub enum MemoryAccessResult {
    Success {
        memory: SpectrumMemory,
        access_quality: Float,
    },
    Blocked {
        required_level: Float,
        current_level: Float,
    },
    Distant {
        distance: Float,
        max_reachable: Float,
    },
}

impl MemoryAccessResult {
    /// Check if access was successful
    pub fn is_success(&self) -> bool {
        matches!(self, MemoryAccessResult::Success { .. })
    }

    /// Get the access quality (0.0 if not successful)
    pub fn quality(&self) -> Float {
        match self {
            MemoryAccessResult::Success { access_quality, .. } => *access_quality,
            _ => 0.0,
        }
    }
}

/// Result of memory storage
#[derive(Debug, Clone)]
pub enum MemoryStorageResult {
    Stored {
        memory_id: u64,
        access_improvement: Float,
    },
    Failed {
        reason: String,
    },
}

impl MemoryStorageResult {
    /// Check if storage was successful
    pub fn is_stored(&self) -> bool {
        matches!(self, MemoryStorageResult::Stored { .. })
    }
}

/// Experience to store in Time/Space
#[derive(Debug, Clone)]
pub struct SpectrumExperience {
    pub id: u64,
    pub description: String,
    pub emotional_intensity: Float,
    pub archetype_activation: Vec<Float>,
    pub polarity_effect: Float,
    pub catalyst_source: String,
}

impl SpectrumExperience {
    /// Create a new experience
    pub fn new(
        id: u64,
        description: &str,
        emotional_intensity: Float,
        archetype_activation: Vec<Float>,
    ) -> Self {
        Self {
            id,
            description: description.to_string(),
            emotional_intensity,
            archetype_activation,
            polarity_effect: 0.0,
            catalyst_source: String::new(),
        }
    }

    /// Create a simple experience
    pub fn simple(id: u64, description: &str) -> Self {
        Self {
            id,
            description: description.to_string(),
            emotional_intensity: 0.5,
            archetype_activation: vec![0.5; 22],
            polarity_effect: 0.0,
            catalyst_source: String::new(),
        }
    }
}

/// Record of memory access attempt
#[derive(Debug, Clone)]
pub struct MemoryAccessRecord {
    pub memory_id: u64,
    pub success: bool,
    pub timestamp: SystemTime,
    pub clarity: Float,
}

impl MemoryAccessRecord {
    /// Check if this record is recent (within last hour)
    pub fn is_recent(&self) -> bool {
        self.timestamp
            .elapsed()
            .map(|d| d.as_secs() < 3600)
            .unwrap_or(false)
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_system_creation() {
        let system = MemorySystem::new();
        assert!(system.spectrum_access_level > 0.0);
        assert!(system.access_history.is_empty());
        assert!(system.recent_memories.is_empty());
    }

    #[test]
    fn test_memory_access_blocked() {
        let mut system = MemorySystem::new();
        // Default spectrum_access_level is 0.1
        // PastLife has base_difficulty of 0.8
        let query = SpectrumMemoryQuery::past_life(1, "test");

        // Low access level should block
        let result = system.recall(&query);
        assert!(matches!(result, MemoryAccessResult::Blocked { .. }));
    }

    #[test]
    fn test_memory_access_success() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5; // Sufficient

        let query = SpectrumMemoryQuery::simple(1, "test");

        let result = system.recall(&query);
        assert!(result.is_success());
    }

    #[test]
    fn test_memory_access_distant() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;
        system.time_space_ratio = 0.05; // Low time/space ratio

        // Temporal distance of 0.8 requires time_space_ratio >= 0.08
        let query = SpectrumMemoryQuery::deep(1, "test", 0.8);

        let result = system.recall(&query);
        assert!(matches!(result, MemoryAccessResult::Distant { .. }));
    }

    #[test]
    fn test_learning_improves_access() {
        let mut system = MemorySystem::new();
        let initial = system.spectrum_access_level;

        system.improve_access(0.1);

        assert!(system.spectrum_access_level > initial);
    }

    #[test]
    fn test_veil_thinning() {
        let mut system = MemorySystem::new();
        let initial = system.veil_transparency;

        system.thin_veil(0.1);

        assert!(system.veil_transparency > initial);
    }

    #[test]
    fn test_memory_storage() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;

        let experience = SpectrumExperience::simple(1, "test experience");

        let result = system.store(&experience);
        assert!(result.is_stored());

        // Check that memory was cached
        assert_eq!(system.recent_memories.len(), 1);
    }

    #[test]
    fn test_memory_storage_improves_access() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;
        let initial = system.spectrum_access_level;

        let experience = SpectrumExperience::simple(1, "test experience");
        system.store(&experience);

        assert!(system.spectrum_access_level > initial);
    }

    #[test]
    fn test_memory_storage_blocked() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.05; // Too low

        // High emotional intensity makes storage harder
        let experience = SpectrumExperience::new(
            1,
            "intense experience",
            0.9, // High intensity
            vec![0.5; 22],
        );

        let result = system.store(&experience);
        assert!(matches!(result, MemoryStorageResult::Failed { .. }));
    }

    #[test]
    fn test_memory_type_difficulties() {
        // Verify memory type difficulties are ordered correctly
        assert!(MemoryType::Working.base_difficulty() < MemoryType::Episodic.base_difficulty());
        assert!(MemoryType::Episodic.base_difficulty() < MemoryType::Emotional.base_difficulty());
        assert!(MemoryType::Emotional.base_difficulty() < MemoryType::PastLife.base_difficulty());
    }

    #[test]
    fn test_can_access_memory_type() {
        let mut system = MemorySystem::new();

        // Default spectrum_access_level is 0.1
        assert!(system.can_access(&MemoryType::Working)); // 0.1
        assert!(!system.can_access(&MemoryType::PastLife)); // 0.8

        // After improving access
        system.spectrum_access_level = 0.9;
        assert!(system.can_access(&MemoryType::PastLife));
    }

    #[test]
    fn test_max_temporal_reach() {
        let system = MemorySystem::new();
        assert_eq!(system.max_temporal_reach(), 1.0); // 0.1 * 10

        let mut high_ratio = MemorySystem::new();
        high_ratio.time_space_ratio = 0.5;
        assert_eq!(high_ratio.max_temporal_reach(), 5.0); // 0.5 * 10
    }

    #[test]
    fn test_memory_statistics() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;

        // Make some successful accesses
        for i in 0..5 {
            let query = SpectrumMemoryQuery::simple(i, "test");
            system.recall(&query);
        }

        // Make some failed accesses
        system.spectrum_access_level = 0.1;
        for i in 5..8 {
            let query = SpectrumMemoryQuery::past_life(i, "test");
            system.recall(&query);
        }

        let stats = system.statistics();
        assert_eq!(stats.total_accesses, 8);
        assert_eq!(stats.successful_accesses, 5);
        assert_eq!(stats.failed_accesses, 3);
        assert!((stats.success_rate - 0.625).abs() < 0.01);
    }

    #[test]
    fn test_memory_clarity() {
        let memory = SpectrumMemory {
            id: 1,
            content: "test".to_string(),
            clarity: 0.5,
            temporal_position: 0.0,
            emotional_valence: 0.0,
            archetype_resonance: vec![],
        };

        assert!(memory.is_clear());

        let unclear = SpectrumMemory {
            id: 1,
            content: "test".to_string(),
            clarity: 0.2,
            temporal_position: 0.0,
            emotional_valence: 0.0,
            archetype_resonance: vec![],
        };

        assert!(!unclear.is_clear());
    }

    #[test]
    fn test_dominant_archetype() {
        let memory = SpectrumMemory {
            id: 1,
            content: "test".to_string(),
            clarity: 0.5,
            temporal_position: 0.0,
            emotional_valence: 0.0,
            archetype_resonance: vec![0.3, 0.7, 0.4, 0.2],
        };

        let dominant = memory.dominant_archetype();
        assert!(dominant.is_some());
        let (index, value) = dominant.unwrap();
        assert_eq!(index, 1);
        assert!((value - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_access_history_learning() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;

        // Successful access should improve spectrum access
        let initial = system.spectrum_access_level;
        let query = SpectrumMemoryQuery::simple(1, "test");
        system.recall(&query);

        // Access should improve from learning
        assert!(system.spectrum_access_level >= initial);
    }

    #[test]
    fn test_cache_memory_limit() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;

        // Store more than 100 memories
        for i in 0..150 {
            let experience = SpectrumExperience::simple(i, "test");
            system.store(&experience);
        }

        // Should be limited to 100
        assert_eq!(system.recent_memories.len(), 100);
    }

    #[test]
    fn test_from_kernel_params() {
        let system = MemorySystem::from_kernel_params(0.7, 0.4, 0.2);

        assert!((system.spectrum_access_level - 0.7).abs() < 0.01);
        assert!((system.time_space_ratio - 0.4).abs() < 0.01);
        assert!((system.veil_transparency - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_veil_transparency_reduces_difficulty() {
        let mut system = MemorySystem::new();
        system.spectrum_access_level = 0.5;

        // Without veil transparency
        system.veil_transparency = 0.0;
        let query_no_veil = SpectrumMemoryQuery::simple(1, "test");
        let result_no_veil = system.recall(&query_no_veil);

        // With veil transparency
        system.veil_transparency = 0.5;
        let query_with_veil = SpectrumMemoryQuery::simple(2, "test");
        let result_with_veil = system.recall(&query_with_veil);

        // Both should succeed, but transparency should give better clarity
        assert!(result_no_veil.is_success());
        assert!(result_with_veil.is_success());

        if let (
            MemoryAccessResult::Success {
                access_quality: q1, ..
            },
            MemoryAccessResult::Success {
                access_quality: q2, ..
            },
        ) = (result_no_veil, result_with_veil)
        {
            assert!(q2 >= q1);
        }
    }
}
