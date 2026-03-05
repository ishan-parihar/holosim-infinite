//! Holographic Memory System
//!
//! Implements holographic memory streams, soul stream continuity, and "transcend and include" memory preservation.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.6:
//! > "Each step INCLUDES all previous development and TRANSCENDS by adding new development."
//! > "This is the mechanism by which the holographic principle operates."
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 7:
//! > "Inventory as holographic memory - what you remember/resonate with"
//!
//! Key Concepts:
//! - **Holographic Memory**: Memory stored as holographic interference patterns
//! - **Soul Stream**: Continuous memory stream across densities/incarnations
//! - **Transcend and Include**: Each new density INCLUDES all previous memories and TRANSCENDS by adding new ones
//! - **Density Transition**: Memory preservation during reincarnation
//! - **Holographic Continuity**: Memory maintains completeness across transitions

use crate::types::{Density, Float};
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};

pub type SoulStreamId = u64;
pub type EntityId = u64;
pub type MemoryKey = (SoulStreamId, u64);

const MAX_MEMORY_ENTRIES: usize = 1000;
const MAX_TRANSCENDED_MEMORIES: usize = 100;
const MEMORY_RETENTION_DURATION: Duration = Duration::from_secs(86400 * 365 * 10);

#[derive(Debug, Clone, PartialEq)]
pub enum HolographicMemoryError {
    SoulStreamNotFound(SoulStreamId),
    MemoryNotFound(MemoryKey),
    InvalidDensityTransition { from: Density, to: Density },
    MemoryCapacityExceeded { requested: usize, capacity: usize },
    TranscendedMemoryCapacityExceeded { requested: usize, capacity: usize },
    InvalidHolographicSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryType {
    Experience,
    Learning,
    Catalyst,
    Relationship,
    Skill,
    Blueprint,
    ArchetypePattern,
    SpectrumConfiguration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HolographicSignature {
    pub interference_pattern: [Float; 22],
    pub coherence_level: Float,
    pub entropy_level: Float,
    pub resonance_frequency: Float,
}

impl Default for HolographicSignature {
    fn default() -> Self {
        Self::new()
    }
}

impl HolographicSignature {
    pub fn new() -> Self {
        Self {
            interference_pattern: [0.0; 22],
            coherence_level: 1.0,
            entropy_level: 0.0,
            resonance_frequency: 440.0,
        }
    }

    pub fn from_spectrum(
        spectrum: &crate::simulation_v3::holographic_physics::SpectrumRatio,
    ) -> Self {
        let base_value = (spectrum.space_time_ratio
            / (spectrum.space_time_ratio + spectrum.time_space_ratio))
            .min(1.0);
        let pattern: [Float; 22] = (0..22)
            .map(|i| base_value * (1.0 + (i as Float / 22.0) * 0.5).min(1.0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0.0; 22]);
        Self {
            interference_pattern: pattern,
            coherence_level: 0.8 + base_value * 0.2,
            entropy_level: 0.1 + (1.0 - base_value) * 0.2,
            resonance_frequency: 440.0 + base_value * 220.0,
        }
    }

    pub fn compute_similarity(&self, other: &HolographicSignature) -> Float {
        let pattern_diff: Float = self.interference_pattern
            .iter()
            .zip(other.interference_pattern.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum();
        let pattern_similarity = 1.0 - (pattern_diff / 22.0);
        let coherence_similarity = 1.0 - (self.coherence_level - other.coherence_level).abs();
        let resonance_similarity =
            1.0 - (self.resonance_frequency - other.resonance_frequency).abs() / 880.0;

        pattern_similarity * 0.5 + coherence_similarity * 0.3 + resonance_similarity * 0.2
    }

    pub fn compute_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for &value in &self.interference_pattern {
            (value.to_bits()).hash(&mut hasher);
        }
        self.coherence_level.to_bits().hash(&mut hasher);
        self.entropy_level.to_bits().hash(&mut hasher);
        self.resonance_frequency.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub memory_id: u64,
    pub timestamp: SystemTime,
    pub density: Density,
    pub memory_type: MemoryType,
    pub holographic_signature: HolographicSignature,
    pub emotional_intensity: Float,
    pub archetypical_activation: [Float; 22],
    pub spectrum_configuration: [Float; 8],
    pub holographic_data: Vec<Float>,
    pub associations: Vec<MemoryKey>,
    pub transcend_count: u32,
}

impl MemoryEntry {
    pub fn new(
        memory_id: u64,
        density: Density,
        memory_type: MemoryType,
        holographic_signature: HolographicSignature,
    ) -> Self {
        Self {
            memory_id,
            timestamp: SystemTime::now(),
            density,
            memory_type,
            holographic_signature,
            emotional_intensity: 0.5,
            archetypical_activation: [0.0; 22],
            spectrum_configuration: [0.0; 8],
            holographic_data: Vec::new(),
            associations: Vec::new(),
            transcend_count: 0,
        }
    }

    pub fn age(&self) -> Duration {
        SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or(Duration::ZERO)
    }

    pub fn is_expired(&self) -> bool {
        self.age() > MEMORY_RETENTION_DURATION
    }

    pub fn decay_emotional_intensity(&mut self, decay_rate: Float) {
        self.emotional_intensity *= (1.0 - decay_rate).max(0.0);
    }
}

#[derive(Debug, Clone)]
pub struct TranscendedMemory {
    pub original_memory: MemoryEntry,
    pub transcended_signature: HolographicSignature,
    pub inclusion_level: Float,
    pub transcend_timestamp: SystemTime,
    pub source_density: Density,
}

impl TranscendedMemory {
    pub fn new(original: MemoryEntry, source_density: Density) -> Self {
        let transcended_signature = HolographicSignature::new();
        Self {
            original_memory: original,
            transcended_signature,
            inclusion_level: 1.0,
            transcend_timestamp: SystemTime::now(),
            source_density,
        }
    }

    pub fn compute_compression_ratio(&self) -> Float {
        let original_size = std::mem::size_of_val(&self.original_memory);
        let compressed_size =
            std::mem::size_of_val(&self.transcended_signature) + std::mem::size_of::<Float>() * 2;
        original_size as Float / compressed_size as Float
    }
}

#[derive(Debug, Clone)]
pub struct SoulStream {
    pub soul_id: SoulStreamId,
    pub memory_entries: VecDeque<MemoryEntry>,
    pub holographic_signature: HolographicSignature,
    pub current_density: Density,
    pub total_incarnations: u32,
    pub stream_coherence: Float,
}

impl SoulStream {
    pub fn new(soul_id: SoulStreamId, initial_density: Density) -> Self {
        Self {
            soul_id,
            memory_entries: VecDeque::new(),
            holographic_signature: HolographicSignature::new(),
            current_density: initial_density,
            total_incarnations: 1,
            stream_coherence: 1.0,
        }
    }

    pub fn add_memory(&mut self, memory: MemoryEntry) -> Result<(), HolographicMemoryError> {
        if self.memory_entries.len() >= MAX_MEMORY_ENTRIES {
            return Err(HolographicMemoryError::MemoryCapacityExceeded {
                requested: self.memory_entries.len() + 1,
                capacity: MAX_MEMORY_ENTRIES,
            });
        }

        self.memory_entries.push_back(memory);
        self.update_stream_signature();
        Ok(())
    }

    pub fn get_memory(&self, memory_id: u64) -> Option<&MemoryEntry> {
        self.memory_entries
            .iter()
            .find(|m| m.memory_id == memory_id)
    }

    pub fn get_memory_mut(&mut self, memory_id: u64) -> Option<&mut MemoryEntry> {
        self.memory_entries
            .iter_mut()
            .find(|m| m.memory_id == memory_id)
    }

    pub fn remove_memory(&mut self, memory_id: u64) -> Option<MemoryEntry> {
        if let Some(pos) = self
            .memory_entries
            .iter()
            .position(|m| m.memory_id == memory_id)
        {
            Some(self.memory_entries.remove(pos).unwrap())
        } else {
            None
        }
    }

    pub fn cleanup_expired_memories(&mut self) -> usize {
        let initial_count = self.memory_entries.len();
        self.memory_entries.retain(|m| !m.is_expired());
        initial_count - self.memory_entries.len()
    }

    pub fn update_stream_signature(&mut self) {
        if self.memory_entries.is_empty() {
            return;
        }

        let mut pattern_sum = [0.0f64; 22];
        let mut coherence_sum = 0.0;
        let mut entropy_sum = 0.0;
        let mut resonance_sum = 0.0;

        for memory in &self.memory_entries {
            for (sum_i, &pattern_i) in pattern_sum.iter_mut().zip(memory.holographic_signature.interference_pattern.iter()) {
                *sum_i += pattern_i;
            }
            coherence_sum += memory.holographic_signature.coherence_level;
            entropy_sum += memory.holographic_signature.entropy_level;
            resonance_sum += memory.holographic_signature.resonance_frequency;
        }

        let count = self.memory_entries.len() as Float;
        for (sig_i, &sum_i) in self.holographic_signature.interference_pattern.iter_mut().zip(pattern_sum.iter()) {
            *sig_i = sum_i / count;
        }
        self.holographic_signature.coherence_level = coherence_sum / count;
        self.holographic_signature.entropy_level = entropy_sum / count;
        self.holographic_signature.resonance_frequency = resonance_sum / count;
    }

    pub fn compute_stream_coherence(&self) -> Float {
        if self.memory_entries.len() < 2 {
            return 1.0;
        }

        let mut total_similarity = 0.0;
        let mut comparisons = 0;

        let memories: Vec<_> = self.memory_entries.iter().collect();
        for i in 0..memories.len() {
            for j in (i + 1)..memories.len() {
                total_similarity += memories[i]
                    .holographic_signature
                    .compute_similarity(&memories[j].holographic_signature);
                comparisons += 1;
            }
        }

        if comparisons == 0 {
            return 1.0;
        }

        total_similarity / comparisons as Float
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct MemoryQuery {
    pub density: Option<Density>,
    pub memory_type: Option<MemoryType>,
    pub min_emotional_intensity: Option<Float>,
    pub max_age: Option<Duration>,
    pub min_similarity: Option<Float>,
    pub reference_signature: Option<HolographicSignature>,
    pub limit: Option<usize>,
}


#[derive(Debug, Clone)]
pub struct HolographicMemorySystem {
    pub memory_streams: HashMap<SoulStreamId, SoulStream>,
    pub memory_cache: HashMap<MemoryKey, MemoryEntry>,
    pub transcended_memories: Vec<TranscendedMemory>,
    pub total_memories_stored: u64,
    pub total_transcend_operations: u64,
}

impl HolographicMemorySystem {
    pub fn new() -> Self {
        Self {
            memory_streams: HashMap::new(),
            memory_cache: HashMap::new(),
            transcended_memories: Vec::new(),
            total_memories_stored: 0,
            total_transcend_operations: 0,
        }
    }

    pub fn create_soul_stream(
        &mut self,
        soul_id: SoulStreamId,
        initial_density: Density,
    ) -> SoulStream {
        let stream = SoulStream::new(soul_id, initial_density);
        self.memory_streams.insert(soul_id, stream.clone());
        stream
    }

    pub fn get_soul_stream(&self, soul_id: SoulStreamId) -> Option<&SoulStream> {
        self.memory_streams.get(&soul_id)
    }

    pub fn get_soul_stream_mut(&mut self, soul_id: SoulStreamId) -> Option<&mut SoulStream> {
        self.memory_streams.get_mut(&soul_id)
    }

    pub fn store_memory(
        &mut self,
        soul_id: SoulStreamId,
        memory: MemoryEntry,
    ) -> Result<MemoryKey, HolographicMemoryError> {
        let stream = self
            .memory_streams
            .get_mut(&soul_id)
            .ok_or(HolographicMemoryError::SoulStreamNotFound(soul_id))?;

        stream.add_memory(memory.clone())?;
        let key = (soul_id, memory.memory_id);
        self.memory_cache.insert(key, memory);
        self.total_memories_stored += 1;

        Ok(key)
    }

    pub fn recall_memory(
        &self,
        soul_id: SoulStreamId,
        memory_id: u64,
    ) -> Result<&MemoryEntry, HolographicMemoryError> {
        let key = (soul_id, memory_id);
        self.memory_cache
            .get(&key)
            .ok_or(HolographicMemoryError::MemoryNotFound(key))
    }

    pub fn query_memories(
        &self,
        soul_id: SoulStreamId,
        query: MemoryQuery,
    ) -> Result<Vec<&MemoryEntry>, HolographicMemoryError> {
        let stream = self
            .memory_streams
            .get(&soul_id)
            .ok_or(HolographicMemoryError::SoulStreamNotFound(soul_id))?;

        let mut results: Vec<&MemoryEntry> = Vec::new();

        for memory in &stream.memory_entries {
            if let Some(density) = query.density {
                if memory.density != density {
                    continue;
                }
            }

            if let Some(memory_type) = query.memory_type {
                if memory.memory_type != memory_type {
                    continue;
                }
            }

            if let Some(min_intensity) = query.min_emotional_intensity {
                if memory.emotional_intensity < min_intensity {
                    continue;
                }
            }

            if let Some(max_age) = query.max_age {
                if memory.age() > max_age {
                    continue;
                }
            }

            if let Some(ref signature) = query.reference_signature {
                let similarity = memory.holographic_signature.compute_similarity(signature);
                if let Some(min_similarity) = query.min_similarity {
                    if similarity < min_similarity {
                        continue;
                    }
                }
            }

            results.push(memory);

            if let Some(limit) = query.limit {
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    pub fn transition_density(
        &mut self,
        soul_id: SoulStreamId,
        from_density: Density,
        to_density: Density,
    ) -> Result<(), HolographicMemoryError> {
        {
            let stream = self
                .memory_streams
                .get(&soul_id)
                .ok_or(HolographicMemoryError::SoulStreamNotFound(soul_id))?;

            if stream.current_density != from_density {
                return Err(HolographicMemoryError::InvalidDensityTransition {
                    from: stream.current_density,
                    to: to_density,
                });
            }
        }

        self.transcend_and_include(soul_id, from_density, to_density)?;

        let stream = self
            .memory_streams
            .get_mut(&soul_id)
            .ok_or(HolographicMemoryError::SoulStreamNotFound(soul_id))?;

        stream.current_density = to_density;
        stream.total_incarnations += 1;

        Ok(())
    }

    pub fn transcend_and_include(
        &mut self,
        soul_id: SoulStreamId,
        from_density: Density,
        _to_density: Density,
    ) -> Result<(), HolographicMemoryError> {
        let memories_to_transcend: Vec<MemoryEntry> = {
            let stream = self
                .memory_streams
                .get(&soul_id)
                .ok_or(HolographicMemoryError::SoulStreamNotFound(soul_id))?;

            stream
                .memory_entries
                .iter()
                .filter(|m| m.density == from_density)
                .cloned()
                .collect()
        };

        for memory in memories_to_transcend {
            if self.transcended_memories.len() >= MAX_TRANSCENDED_MEMORIES {
                self.transcended_memories.remove(0);
            }

            let transcended = TranscendedMemory::new(memory, from_density);
            self.transcended_memories.push(transcended);
        }

        self.total_transcend_operations += 1;
        Ok(())
    }

    pub fn get_transcended_memories(&self, density: Density) -> Vec<&TranscendedMemory> {
        self.transcended_memories
            .iter()
            .filter(|tm| tm.source_density == density)
            .collect()
    }

    pub fn access_transcended_memory(
        &self,
        density: Density,
        query_signature: &HolographicSignature,
    ) -> Option<&TranscendedMemory> {
        let transcended = self.get_transcended_memories(density);

        transcended
            .iter()
            .max_by(|a, b| {
                let similarity_a = a.transcended_signature.compute_similarity(query_signature);
                let similarity_b = b.transcended_signature.compute_similarity(query_signature);
                similarity_a.partial_cmp(&similarity_b).unwrap()
            })
            .copied()
    }

    pub fn decay_all_memories(&mut self, decay_rate: Float) {
        // From COSMOLOGICAL-ARCHITECTURE.md: "Memories naturally decay over time as emotional resonance fades"
        for stream in self.memory_streams.values_mut() {
            for memory in &mut stream.memory_entries {
                memory.decay_emotional_intensity(decay_rate);
                // Also update the cached copy
                let key = (stream.soul_id, memory.memory_id);
                if let Some(cached) = self.memory_cache.get_mut(&key) {
                    cached.decay_emotional_intensity(decay_rate);
                }
            }
        }
    }

    pub fn cleanup_expired_memories(&mut self) -> usize {
        let mut total_removed = 0;
        for stream in self.memory_streams.values_mut() {
            total_removed += stream.cleanup_expired_memories();
        }

        let keys_to_remove: Vec<_> = self
            .memory_cache
            .iter()
            .filter(|(_, m)| m.is_expired())
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_remove {
            self.memory_cache.remove(&key);
        }

        total_removed
    }

    pub fn compute_holographic_continuity(&self, soul_id: SoulStreamId) -> Option<Float> {
        let stream = self.memory_streams.get(&soul_id)?;
        Some(stream.compute_stream_coherence())
    }

    pub fn get_statistics(&self) -> HolographicMemoryStatistics {
        let total_streams = self.memory_streams.len();
        let total_memories: usize = self
            .memory_streams
            .values()
            .map(|s| s.memory_entries.len())
            .sum();
        let avg_memories_per_stream = if total_streams > 0 {
            total_memories as Float / total_streams as Float
        } else {
            0.0
        };

        let total_coherence: Float = self
            .memory_streams
            .values()
            .map(|s| s.stream_coherence)
            .sum();
        let avg_coherence = if total_streams > 0 {
            total_coherence / total_streams as Float
        } else {
            0.0
        };

        let avg_compression_ratio = if !self.transcended_memories.is_empty() {
            let total_ratio: Float = self
                .transcended_memories
                .iter()
                .map(|tm| tm.compute_compression_ratio())
                .sum();
            total_ratio / self.transcended_memories.len() as Float
        } else {
            0.0
        };

        HolographicMemoryStatistics {
            total_streams: total_streams as u64,
            total_memories: total_memories as u64,
            total_transcended_memories: self.transcended_memories.len() as u64,
            total_memories_stored: self.total_memories_stored,
            total_transcend_operations: self.total_transcend_operations,
            avg_memories_per_stream,
            avg_coherence,
            avg_compression_ratio,
        }
    }
}

impl Default for HolographicMemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct HolographicMemoryStatistics {
    pub total_streams: u64,
    pub total_memories: u64,
    pub total_transcended_memories: u64,
    pub total_memories_stored: u64,
    pub total_transcend_operations: u64,
    pub avg_memories_per_stream: Float,
    pub avg_coherence: Float,
    pub avg_compression_ratio: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_signature_creation() {
        let signature = HolographicSignature::new();
        assert_eq!(signature.coherence_level, 1.0);
        assert_eq!(signature.entropy_level, 0.0);
    }

    #[test]
    fn test_holographic_signature_similarity() {
        let sig1 = HolographicSignature::new();
        let mut sig2 = HolographicSignature::new();
        sig2.coherence_level = 0.5;

        let similarity = sig1.compute_similarity(&sig2);
        assert!(similarity > 0.0 && similarity <= 1.0);
    }

    #[test]
    fn test_memory_entry_creation() {
        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        assert_eq!(memory.memory_id, 1);
        assert_eq!(memory.density, Density::Third);
        assert_eq!(memory.memory_type, MemoryType::Experience);
    }

    #[test]
    fn test_memory_entry_decay() {
        let mut memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        memory.emotional_intensity = 1.0;

        memory.decay_emotional_intensity(0.1);
        assert!(memory.emotional_intensity < 1.0 && memory.emotional_intensity > 0.0);
    }

    #[test]
    fn test_soul_stream_creation() {
        let stream = SoulStream::new(1, Density::Third);
        assert_eq!(stream.soul_id, 1);
        assert_eq!(stream.current_density, Density::Third);
        assert_eq!(stream.total_incarnations, 1);
    }

    #[test]
    fn test_soul_stream_add_memory() {
        let mut stream = SoulStream::new(1, Density::Third);
        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        let result = stream.add_memory(memory);
        assert!(result.is_ok());
        assert_eq!(stream.memory_entries.len(), 1);
    }

    #[test]
    fn test_soul_stream_memory_capacity() {
        let mut stream = SoulStream::new(1, Density::Third);
        let mut memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        for i in 0..MAX_MEMORY_ENTRIES {
            memory.memory_id = i as u64;
            let result = stream.add_memory(memory.clone());
            assert!(result.is_ok());
        }

        let excess_result = stream.add_memory(memory);
        assert!(matches!(
            excess_result,
            Err(HolographicMemoryError::MemoryCapacityExceeded { .. })
        ));
    }

    #[test]
    fn test_soul_stream_get_memory() {
        let mut stream = SoulStream::new(1, Density::Third);
        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        stream.add_memory(memory.clone()).unwrap();
        let retrieved = stream.get_memory(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().memory_id, 1);
    }

    #[test]
    fn test_soul_stream_remove_memory() {
        let mut stream = SoulStream::new(1, Density::Third);
        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        stream.add_memory(memory.clone()).unwrap();
        let removed = stream.remove_memory(1);
        assert!(removed.is_some());
        assert_eq!(stream.memory_entries.len(), 0);
    }

    #[test]
    fn test_holographic_memory_system_creation() {
        let system = HolographicMemorySystem::new();
        assert_eq!(system.memory_streams.len(), 0);
        assert_eq!(system.total_memories_stored, 0);
    }

    #[test]
    fn test_holographic_memory_system_create_soul_stream() {
        let mut system = HolographicMemorySystem::new();
        let stream = system.create_soul_stream(1, Density::Third);

        assert_eq!(stream.soul_id, 1);
        assert_eq!(system.memory_streams.len(), 1);
    }

    #[test]
    fn test_holographic_memory_system_store_memory() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        let result = system.store_memory(1, memory);
        assert!(result.is_ok());
        assert_eq!(system.total_memories_stored, 1);
    }

    #[test]
    fn test_holographic_memory_system_recall_memory() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );

        system.store_memory(1, memory.clone()).unwrap();
        let recalled = system.recall_memory(1, 1);

        assert!(recalled.is_ok());
        assert_eq!(recalled.unwrap().memory_id, 1);
    }

    #[test]
    fn test_holographic_memory_system_query_memories() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        for i in 0..10 {
            let memory = MemoryEntry::new(
                i,
                Density::Third,
                MemoryType::Experience,
                HolographicSignature::new(),
            );
            system.store_memory(1, memory).unwrap();
        }

        let query = MemoryQuery {
            density: Some(Density::Third),
            limit: Some(5),
            ..Default::default()
        };

        let results = system.query_memories(1, query).unwrap();
        assert!(results.len() <= 5);
    }

    #[test]
    fn test_density_transition() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        system.store_memory(1, memory).unwrap();

        let result = system.transition_density(1, Density::Third, Density::Fourth);
        assert!(result.is_ok());

        let stream = system.get_soul_stream(1).unwrap();
        assert_eq!(stream.current_density, Density::Fourth);
        assert_eq!(stream.total_incarnations, 2);
    }

    #[test]
    fn test_transcend_and_include() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        system.store_memory(1, memory).unwrap();

        let result = system.transcend_and_include(1, Density::Third, Density::Fourth);
        assert!(result.is_ok());
        assert_eq!(system.total_transcend_operations, 1);
        assert_eq!(system.transcended_memories.len(), 1);
    }

    #[test]
    fn test_get_transcended_memories() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        system.store_memory(1, memory).unwrap();

        system
            .transcend_and_include(1, Density::Third, Density::Fourth)
            .unwrap();

        let transcended = system.get_transcended_memories(Density::Third);
        assert_eq!(transcended.len(), 1);
    }

    #[test]
    fn test_memory_decay() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let mut memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        memory.emotional_intensity = 1.0;

        system.store_memory(1, memory.clone()).unwrap();
        system.decay_all_memories(0.1);

        let recalled = system.recall_memory(1, 1).unwrap();
        assert!(recalled.emotional_intensity < 1.0);
    }

    #[test]
    fn test_holographic_continuity() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let continuity = system.compute_holographic_continuity(1);
        assert!(continuity.is_some());
        assert!(continuity.unwrap() <= 1.0);
    }

    #[test]
    fn test_memory_statistics() {
        let mut system = HolographicMemorySystem::new();
        system.create_soul_stream(1, Density::Third);

        let memory = MemoryEntry::new(
            1,
            Density::Third,
            MemoryType::Experience,
            HolographicSignature::new(),
        );
        system.store_memory(1, memory).unwrap();

        let stats = system.get_statistics();
        assert_eq!(stats.total_streams, 1);
        assert_eq!(stats.total_memories, 1);
        assert_eq!(stats.total_memories_stored, 1);
    }
}
