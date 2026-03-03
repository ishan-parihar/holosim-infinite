//! Source Merger Mechanics - 8th Density
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "8th Density - Return:
//!  - Merger with Intelligent Infinity field
//!  - Unique pattern preserved in unified field
//!  - Preparation for next octave seeding"
//!
//! # Key Insight
//!
//! At 8th Density, the entity MERGES with Intelligent Infinity while
//! PRESERVING its unique pattern. This is not annihilation but completion -
//! the pattern becomes part of the infinite field while maintaining its signature.

use crate::types::Float;
use std::collections::HashMap;

/// Source Merger State - stages of source merger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceMergerState {
    Approaching,
    Aligning,
    Merging,
    Unified,
    Preserving,
    Complete,
}

impl SourceMergerState {
    pub fn merger_progress(&self) -> Float {
        match self {
            SourceMergerState::Approaching => 0.0,
            SourceMergerState::Aligning => 0.2,
            SourceMergerState::Merging => 0.4,
            SourceMergerState::Unified => 0.6,
            SourceMergerState::Preserving => 0.8,
            SourceMergerState::Complete => 1.0,
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            SourceMergerState::Approaching => Some(SourceMergerState::Aligning),
            SourceMergerState::Aligning => Some(SourceMergerState::Merging),
            SourceMergerState::Merging => Some(SourceMergerState::Unified),
            SourceMergerState::Unified => Some(SourceMergerState::Preserving),
            SourceMergerState::Preserving => Some(SourceMergerState::Complete),
            SourceMergerState::Complete => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SourceMergerState::Approaching => "Approaching the Infinite Source",
            SourceMergerState::Aligning => "Aligning field with Source frequency",
            SourceMergerState::Merging => "Merging with unified field",
            SourceMergerState::Unified => "Unified with Intelligent Infinity",
            SourceMergerState::Preserving => "Preserving unique pattern",
            SourceMergerState::Complete => "Complete - pattern preserved in infinity",
        }
    }
}

/// Source Connection - the connection to the Infinite Source
#[derive(Debug, Clone)]
pub struct SourceConnection {
    pub entity_id: u64,
    pub connection_strength: Float,
    pub resonance_with_source: Float,
    pub frequency_alignment: Float,
    pub phase_synchronization: Float,
    pub bandwidth: Float,
    pub last_transmission: Float,
}

impl SourceConnection {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            connection_strength: 0.0,
            resonance_with_source: 0.0,
            frequency_alignment: 0.0,
            phase_synchronization: 0.0,
            bandwidth: 0.0,
            last_transmission: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        let strength_drift = 0.001 * dt * (1.0 - self.connection_strength);
        self.connection_strength = (self.connection_strength + strength_drift).min(1.0);

        self.resonance_with_source = self.connection_strength * 0.95;
        self.frequency_alignment = self.connection_strength * 0.9;
        self.phase_synchronization = self.connection_strength * 0.85;
        self.bandwidth = self.connection_strength * self.frequency_alignment;
    }

    pub fn strengthen(&mut self, amount: Float) {
        self.connection_strength = (self.connection_strength + amount).min(1.0);
    }

    pub fn can_transmit(&self) -> bool {
        self.connection_strength >= 0.5 && self.bandwidth > 0.1
    }

    pub fn transmit(&mut self, data_amount: Float, current_time: Float) -> Float {
        if !self.can_transmit() {
            return 0.0;
        }

        let transmitted = data_amount.min(self.bandwidth);
        self.last_transmission = current_time;
        transmitted
    }

    pub fn receive(&self) -> Float {
        self.bandwidth * self.resonance_with_source
    }
}

/// Pattern Preservation - preserving the unique pattern during merger
#[derive(Debug, Clone)]
pub struct PatternPreservation {
    pub entity_id: u64,
    pub pattern_signature: Vec<Float>,
    pub preservation_strength: Float,
    pub crystallization_level: Float,
    pub experiences_encoded: HashMap<String, Float>,
    pub wisdom_compressed: Float,
    pub love_compressed: Float,
    pub light_compressed: Float,
}

impl PatternPreservation {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            pattern_signature: vec![0.0; 22],
            preservation_strength: 0.0,
            crystallization_level: 0.0,
            experiences_encoded: HashMap::new(),
            wisdom_compressed: 0.0,
            love_compressed: 0.0,
            light_compressed: 0.0,
        }
    }

    pub fn with_signature(mut self, signature: Vec<Float>) -> Self {
        self.pattern_signature = signature;
        self
    }

    pub fn encode_experience(&mut self, key: &str, value: Float) {
        let compressed = value * 0.1;
        self.experiences_encoded.insert(key.to_string(), compressed);

        self.wisdom_compressed = (self.wisdom_compressed + compressed * 0.4).min(1.0);
        self.love_compressed = (self.love_compressed + compressed * 0.35).min(1.0);
        self.light_compressed = (self.light_compressed + compressed * 0.25).min(1.0);
    }

    pub fn crystallize(&mut self, dt: Float) {
        let experience_count = self.experiences_encoded.len() as Float;
        let compression_factor =
            (self.wisdom_compressed + self.love_compressed + self.light_compressed) / 3.0;

        self.crystallization_level =
            (self.crystallization_level + 0.001 * dt * compression_factor).min(1.0);

        self.preservation_strength = self.crystallization_level
            * (1.0 + 0.01 * experience_count).min(2.0)
            * compression_factor;

        for sig in &mut self.pattern_signature {
            *sig = (*sig + 0.001 * dt * self.preservation_strength).min(1.0);
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.crystallize(dt);
    }

    pub fn is_preserved(&self) -> bool {
        self.preservation_strength >= 0.9
            && self.crystallization_level >= 0.9
            && self.pattern_signature.iter().all(|&s| s > 0.1)
    }

    pub fn signature_coherence(&self) -> Float {
        if self.pattern_signature.is_empty() {
            return 0.0;
        }

        let mean: Float =
            self.pattern_signature.iter().sum::<Float>() / self.pattern_signature.len() as Float;
        let variance: Float = self
            .pattern_signature
            .iter()
            .map(|s| (s - mean).powi(2))
            .sum::<Float>()
            / self.pattern_signature.len() as Float;

        1.0 - variance.sqrt().min(1.0)
    }
}

/// Merger Progress - tracking the merger process
#[derive(Debug, Clone)]
pub struct MergerProgress {
    pub entity_id: u64,
    pub state: SourceMergerState,
    pub total_time_in_merger: Float,
    pub field_coherence_at_entry: Float,
    pub current_field_coherence: Float,
    pub unity_achieved: Float,
    pub infinity_contact_count: u32,
    pub pattern_preserved: bool,
}

impl MergerProgress {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            state: SourceMergerState::Approaching,
            total_time_in_merger: 0.0,
            field_coherence_at_entry: 0.0,
            current_field_coherence: 0.5,
            unity_achieved: 0.0,
            infinity_contact_count: 0,
            pattern_preserved: false,
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.total_time_in_merger += dt;

        let coherence_drift = 0.001 * dt * (0.99 - self.current_field_coherence);
        self.current_field_coherence = (self.current_field_coherence + coherence_drift).min(1.0);

        self.unity_achieved = self.state.merger_progress() * self.current_field_coherence;

        let threshold = match self.state {
            SourceMergerState::Approaching => 0.15,
            SourceMergerState::Aligning => 0.35,
            SourceMergerState::Merging => 0.55,
            SourceMergerState::Unified => 0.75,
            SourceMergerState::Preserving => 0.9,
            SourceMergerState::Complete => 1.0,
        };

        if self.unity_achieved >= threshold {
            if let Some(next) = self.state.next() {
                self.state = next;
                self.infinity_contact_count += 1;
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.state == SourceMergerState::Complete && self.pattern_preserved
    }

    pub fn progress_percentage(&self) -> Float {
        self.state.merger_progress()
    }
}

/// Intelligent Infinity Merger - the full 8th density merger system
#[derive(Debug, Clone)]
pub struct IntelligentInfinityMerger {
    pub entity_id: u64,
    pub connection: SourceConnection,
    pub preservation: PatternPreservation,
    pub progress: MergerProgress,
    pub source_frequency: Float,
    pub entity_frequency: Float,
    pub harmonics_locked: bool,
}

impl IntelligentInfinityMerger {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            connection: SourceConnection::new(entity_id),
            preservation: PatternPreservation::new(entity_id),
            progress: MergerProgress::new(entity_id),
            source_frequency: 1.0,
            entity_frequency: 0.1,
            harmonics_locked: false,
        }
    }

    pub fn with_signature(mut self, signature: Vec<Float>) -> Self {
        self.preservation = self.preservation.with_signature(signature);
        self
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        self.connection.update(dt);
        self.preservation.update(dt);
        self.progress.update(dt);

        let alignment_speed = 0.001 * dt * self.connection.connection_strength;
        self.entity_frequency =
            (self.entity_frequency + alignment_speed).min(self.source_frequency);

        if (self.entity_frequency - self.source_frequency).abs() < 0.01 {
            self.harmonics_locked = true;
        }

        if self.progress.state == SourceMergerState::Preserving && !self.progress.pattern_preserved
        {
            if self.preservation.is_preserved() {
                self.progress.pattern_preserved = true;
            }
        }
    }

    pub fn approach_source(&mut self) -> Float {
        let approach_rate = self.connection.bandwidth * 0.1;
        self.connection.strengthen(approach_rate);
        self.progress.infinity_contact_count += 1;
        approach_rate
    }

    pub fn encode_final_experience(&mut self, key: &str, value: Float) {
        self.preservation.encode_experience(key, value);
    }

    pub fn is_harmonically_locked(&self) -> bool {
        self.harmonics_locked
    }

    pub fn merger_complete(&self) -> bool {
        self.progress.is_complete()
    }

    pub fn get_preserved_pattern(&self) -> Option<Vec<Float>> {
        if self.progress.pattern_preserved {
            Some(self.preservation.pattern_signature.clone())
        } else {
            None
        }
    }

    pub fn total_evolution(&self) -> Float {
        self.progress.unity_achieved
            * self.connection.connection_strength
            * self.preservation.preservation_strength
    }

    pub fn next_octave_potential(&self) -> Float {
        if !self.merger_complete() {
            return 0.0;
        }

        self.preservation.wisdom_compressed * 0.4
            + self.preservation.love_compressed * 0.35
            + self.preservation.light_compressed * 0.25
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_merger_state_progress() {
        assert_eq!(SourceMergerState::Approaching.merger_progress(), 0.0);
        assert_eq!(SourceMergerState::Complete.merger_progress(), 1.0);
    }

    #[test]
    fn test_source_merger_state_next() {
        assert_eq!(
            SourceMergerState::Approaching.next(),
            Some(SourceMergerState::Aligning)
        );
        assert_eq!(SourceMergerState::Complete.next(), None);
    }

    #[test]
    fn test_source_connection_creation() {
        let conn = SourceConnection::new(1);
        assert_eq!(conn.entity_id, 1);
        assert_eq!(conn.connection_strength, 0.0);
    }

    #[test]
    fn test_source_connection_update() {
        let mut conn = SourceConnection::new(1);
        conn.update(1.0);
        assert!(conn.connection_strength > 0.0);
    }

    #[test]
    fn test_source_connection_strengthen() {
        let mut conn = SourceConnection::new(1);
        conn.strengthen(0.5);
        assert!((conn.connection_strength - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_source_connection_can_transmit() {
        let mut conn = SourceConnection::new(1);
        conn.connection_strength = 0.6;
        conn.frequency_alignment = 0.5;
        conn.bandwidth = 0.3;
        assert!(conn.can_transmit());
    }

    #[test]
    fn test_pattern_preservation_creation() {
        let pres = PatternPreservation::new(1);
        assert_eq!(pres.entity_id, 1);
        assert_eq!(pres.pattern_signature.len(), 22);
    }

    #[test]
    fn test_pattern_preservation_encode() {
        let mut pres = PatternPreservation::new(1);
        pres.encode_experience("test", 1.0);
        assert!(pres.experiences_encoded.contains_key("test"));
        assert!(pres.wisdom_compressed > 0.0);
    }

    #[test]
    fn test_pattern_preservation_crystallize() {
        let mut pres = PatternPreservation::new(1);
        pres.encode_experience("test", 1.0);
        pres.crystallize(1.0);
        assert!(pres.crystallization_level > 0.0);
    }

    #[test]
    fn test_pattern_preservation_is_preserved() {
        let mut pres = PatternPreservation::new(1);
        pres.preservation_strength = 0.95;
        pres.crystallization_level = 0.95;
        for sig in &mut pres.pattern_signature {
            *sig = 0.5;
        }
        assert!(pres.is_preserved());
    }

    #[test]
    fn test_merger_progress_creation() {
        let progress = MergerProgress::new(1);
        assert_eq!(progress.entity_id, 1);
        assert_eq!(progress.state, SourceMergerState::Approaching);
    }

    #[test]
    fn test_merger_progress_update() {
        let mut progress = MergerProgress::new(1);
        progress.update(1.0);
        assert!(progress.total_time_in_merger > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_merger_creation() {
        let merger = IntelligentInfinityMerger::new(1);
        assert_eq!(merger.entity_id, 1);
    }

    #[test]
    fn test_intelligent_infinity_merger_update() {
        let mut merger = IntelligentInfinityMerger::new(1);
        merger.update(1.0, 1.0);
        assert!(merger.connection.connection_strength > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_merger_approach() {
        let mut merger = IntelligentInfinityMerger::new(1);
        merger.connection.connection_strength = 0.5;
        merger.connection.bandwidth = 0.5;
        let approach = merger.approach_source();
        assert!(approach > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_merger_encode() {
        let mut merger = IntelligentInfinityMerger::new(1);
        merger.encode_final_experience("wisdom", 1.0);
        assert!(merger.preservation.wisdom_compressed > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_merger_evolution() {
        let mut merger = IntelligentInfinityMerger::new(1);
        merger.progress.unity_achieved = 0.5;
        merger.connection.connection_strength = 0.5;
        merger.preservation.preservation_strength = 0.5;
        let evolution = merger.total_evolution();
        assert!(evolution > 0.0);
    }
}
