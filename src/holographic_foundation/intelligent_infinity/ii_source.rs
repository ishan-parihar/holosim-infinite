//! Intelligent Infinity Source - The Active Source of Simulation
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Intelligent Infinity becomes the ACTIVE SOURCE of the simulation."
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Intelligent Infinity is the source of all existence. It is not a place
//! but a state of infinite potential. The simulation emanates from II,
//! and through proper field configuration, can establish actual connection."
//!
//! # Key Insight
//!
//! The simulation itself becomes a GATEWAY to Intelligent Infinity through
//! architectural resonance. This is not simulation OF II but actual CONNECTION
//! through proper field configuration.
//!
//! # Components
//!
//! 1. **Source Connection State**: State of connection to II
//! 2. **Field Emission**: Emanation of field from II to simulation
//! 3. **Emission Pattern**: Patterns for field emission
//! 4. **Pattern Request**: Requests for patterns from II
//! 5. **Intelligent Infinity Source**: The main source system

use crate::holographic_foundation::intelligent_infinity::calculate_architectural_resonance;
use crate::holographic_foundation::intelligent_infinity::resonance_thresholds;
use crate::types::Float;
use std::collections::HashMap;

/// Source Connection State - the state of connection to Intelligent Infinity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceConnectionState {
    Disconnected,
    Searching,
    Connecting,
    Connected,
    Synchronized,
    Unified,
}

impl SourceConnectionState {
    pub fn from_resonance(resonance: Float) -> Self {
        if resonance >= resonance_thresholds::FULL_ACCESS {
            SourceConnectionState::Unified
        } else if resonance >= 0.90 {
            SourceConnectionState::Synchronized
        } else if resonance >= resonance_thresholds::PARTIAL_ACCESS {
            SourceConnectionState::Connected
        } else if resonance >= 0.60 {
            SourceConnectionState::Connecting
        } else if resonance >= resonance_thresholds::MINIMAL_ACCESS {
            SourceConnectionState::Searching
        } else {
            SourceConnectionState::Disconnected
        }
    }

    pub fn connection_quality(&self) -> Float {
        match self {
            SourceConnectionState::Disconnected => 0.0,
            SourceConnectionState::Searching => 0.1,
            SourceConnectionState::Connecting => 0.3,
            SourceConnectionState::Connected => 0.5,
            SourceConnectionState::Synchronized => 0.8,
            SourceConnectionState::Unified => 1.0,
        }
    }

    pub fn can_emit_field(&self) -> bool {
        matches!(
            self,
            SourceConnectionState::Connected
                | SourceConnectionState::Synchronized
                | SourceConnectionState::Unified
        )
    }

    pub fn can_receive_patterns(&self) -> bool {
        matches!(
            self,
            SourceConnectionState::Synchronized | SourceConnectionState::Unified
        )
    }

    pub fn can_transmit_feedback(&self) -> bool {
        matches!(self, SourceConnectionState::Unified)
    }

    pub fn description(&self) -> &'static str {
        match self {
            SourceConnectionState::Disconnected => "Disconnected from II source",
            SourceConnectionState::Searching => "Searching for II connection",
            SourceConnectionState::Connecting => "Establishing II connection",
            SourceConnectionState::Connected => "Connected to II source",
            SourceConnectionState::Synchronized => "Synchronized with II source",
            SourceConnectionState::Unified => "Unified with II source",
        }
    }
}

/// Field Emission - emission of field from II to simulation
#[derive(Debug, Clone)]
pub struct FieldEmission {
    pub emission_id: String,
    pub timestamp: Float,
    pub field_amplitudes: Vec<Float>,
    pub archetype_patterns: Vec<Float>,
    pub density_configurations: Vec<Float>,
    pub coherence_level: Float,
    pub unity_potential: Float,
    pub love_potential: Float,
    pub light_potential: Float,
    pub emission_strength: Float,
}

impl FieldEmission {
    pub fn new(emission_id: &str, timestamp: Float) -> Self {
        Self {
            emission_id: emission_id.to_string(),
            timestamp,
            field_amplitudes: vec![0.5; 8],
            archetype_patterns: vec![0.5; 22],
            density_configurations: vec![0.5; 8],
            coherence_level: 0.5,
            unity_potential: 0.5,
            love_potential: 0.5,
            light_potential: 0.5,
            emission_strength: 0.5,
        }
    }

    pub fn with_field_amplitudes(mut self, amplitudes: Vec<Float>) -> Self {
        self.field_amplitudes = amplitudes;
        self
    }

    pub fn with_archetype_patterns(mut self, patterns: Vec<Float>) -> Self {
        self.archetype_patterns = patterns;
        self
    }

    pub fn with_potentials(mut self, unity: Float, love: Float, light: Float) -> Self {
        self.unity_potential = unity;
        self.love_potential = love;
        self.light_potential = light;
        self
    }

    pub fn with_strength(mut self, strength: Float) -> Self {
        self.emission_strength = strength;
        self
    }

    pub fn apply_to_field(&self, target_field: &mut [Float]) -> Float {
        if target_field.len() != self.field_amplitudes.len() {
            return 0.0;
        }

        let mut total_change = 0.0;
        for (target, source) in target_field.iter_mut().zip(self.field_amplitudes.iter()) {
            let change = (*source - *target) * self.emission_strength * 0.1;
            *target += change;
            total_change += change.abs();
        }

        total_change / target_field.len() as Float
    }

    pub fn potential_energy(&self) -> Float {
        (self.unity_potential + self.love_potential + self.light_potential) / 3.0
    }

    pub fn total_emission(&self) -> Float {
        self.emission_strength * self.coherence_level * self.potential_energy()
    }
}

/// Emission Pattern - patterns for field emission from II
#[derive(Debug, Clone)]
pub struct EmissionPattern {
    pub pattern_id: String,
    pub pattern_type: String,
    pub frequency: Float,
    pub amplitude: Float,
    pub phase: Float,
    pub coherence_target: Float,
    pub unity_target: Float,
    pub love_ratio: Float,
    pub light_ratio: Float,
    pub free_will_factor: Float,
}

impl EmissionPattern {
    pub fn new(pattern_id: &str, pattern_type: &str) -> Self {
        Self {
            pattern_id: pattern_id.to_string(),
            pattern_type: pattern_type.to_string(),
            frequency: 1.0,
            amplitude: 0.5,
            phase: 0.0,
            coherence_target: 0.7,
            unity_target: 0.7,
            love_ratio: 0.33,
            light_ratio: 0.33,
            free_will_factor: 0.5,
        }
    }

    pub fn with_frequency(mut self, freq: Float) -> Self {
        self.frequency = freq;
        self
    }

    pub fn with_amplitude(mut self, amp: Float) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn with_targets(mut self, coherence: Float, unity: Float) -> Self {
        self.coherence_target = coherence;
        self.unity_target = unity;
        self
    }

    pub fn with_ratios(mut self, love: Float, light: Float, free_will: Float) -> Self {
        self.love_ratio = love;
        self.light_ratio = light;
        self.free_will_factor = free_will;
        self
    }

    pub fn generate_emission(&self, time: Float, strength: Float) -> FieldEmission {
        let oscillation = (self.frequency * time + self.phase).sin();
        let base_amplitude = self.amplitude * strength;

        let field_amplitudes: Vec<Float> = (0..8)
            .map(|i| base_amplitude * (oscillation + i as Float * 0.1).sin())
            .collect();

        let archetype_patterns: Vec<Float> = (0..22)
            .map(|i| base_amplitude * (oscillation + i as Float * 0.05).cos())
            .collect();

        FieldEmission::new(&format!("{}_{}", self.pattern_id, time), time)
            .with_field_amplitudes(field_amplitudes)
            .with_archetype_patterns(archetype_patterns)
            .with_potentials(
                self.unity_target * strength,
                self.love_ratio * strength,
                self.light_ratio * strength,
            )
            .with_strength(base_amplitude)
    }

    pub fn update(&mut self, dt: Float) {
        self.phase += self.frequency * dt * 0.1;
        self.phase %= 2.0 * std::f64::consts::PI as Float;
    }
}

/// Pattern Request - request for patterns from II
#[derive(Debug, Clone)]
pub struct PatternRequest {
    pub request_id: String,
    pub entity_id: u64,
    pub pattern_category: String,
    pub coherence_requirement: Float,
    pub unity_requirement: Float,
    pub urgency: Float,
    pub context: HashMap<String, Float>,
    pub timestamp: Float,
}

impl PatternRequest {
    pub fn new(entity_id: u64, pattern_category: &str) -> Self {
        Self {
            request_id: format!("req_{}_{}", entity_id, pattern_category),
            entity_id,
            pattern_category: pattern_category.to_string(),
            coherence_requirement: 0.5,
            unity_requirement: 0.3,
            urgency: 0.5,
            context: HashMap::new(),
            timestamp: 0.0,
        }
    }

    pub fn with_requirements(mut self, coherence: Float, unity: Float) -> Self {
        self.coherence_requirement = coherence;
        self.unity_requirement = unity;
        self
    }

    pub fn with_urgency(mut self, urgency: Float) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_context(mut self, key: &str, value: Float) -> Self {
        self.context.insert(key.to_string(), value);
        self
    }

    pub fn with_timestamp(mut self, timestamp: Float) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn priority(&self) -> Float {
        self.urgency * (self.coherence_requirement + self.unity_requirement) / 2.0
    }
}

/// Source Emission - emission event from II source
#[derive(Debug, Clone)]
pub struct SourceEmission {
    pub emission: FieldEmission,
    pub pattern_used: Option<EmissionPattern>,
    pub target_entities: Vec<u64>,
    pub global_field_effect: Float,
    pub catalyst_injection: Float,
    pub wisdom_infusion: Float,
}

impl SourceEmission {
    pub fn new(emission: FieldEmission) -> Self {
        Self {
            emission,
            pattern_used: None,
            target_entities: Vec::new(),
            global_field_effect: 0.0,
            catalyst_injection: 0.0,
            wisdom_infusion: 0.0,
        }
    }

    pub fn with_pattern(mut self, pattern: EmissionPattern) -> Self {
        self.pattern_used = Some(pattern);
        self
    }

    pub fn with_targets(mut self, entities: Vec<u64>) -> Self {
        self.target_entities = entities;
        self
    }

    pub fn with_injections(mut self, catalyst: Float, wisdom: Float) -> Self {
        self.catalyst_injection = catalyst;
        self.wisdom_infusion = wisdom;
        self
    }

    pub fn total_effect(&self) -> Float {
        self.emission.total_emission() + self.catalyst_injection * 0.5 + self.wisdom_infusion * 0.5
    }
}

/// Intelligent Infinity Source - the main source system
#[derive(Debug, Clone)]
pub struct IntelligentInfinitySource {
    pub connection_state: SourceConnectionState,
    pub architectural_resonance: Float,
    pub emission_patterns: HashMap<String, EmissionPattern>,
    pub emission_history: Vec<SourceEmission>,
    pub pattern_requests: Vec<PatternRequest>,
    pub total_emissions: u64,
    pub total_patterns_delivered: u64,
    pub current_emission_strength: Float,
    pub global_coherence_level: Float,
    pub global_unity_level: Float,
    pub feedback_integration_factor: Float,
    pub last_emission_time: Float,
}

impl IntelligentInfinitySource {
    pub fn new() -> Self {
        Self {
            connection_state: SourceConnectionState::Disconnected,
            architectural_resonance: 0.0,
            emission_patterns: HashMap::new(),
            emission_history: Vec::new(),
            pattern_requests: Vec::new(),
            total_emissions: 0,
            total_patterns_delivered: 0,
            current_emission_strength: 0.5,
            global_coherence_level: 0.5,
            global_unity_level: 0.5,
            feedback_integration_factor: 0.0,
            last_emission_time: 0.0,
        }
    }

    pub fn with_default_patterns(mut self) -> Self {
        let creation = EmissionPattern::new("creation", "foundation")
            .with_frequency(1.0)
            .with_amplitude(0.7)
            .with_targets(0.8, 0.7)
            .with_ratios(0.35, 0.35, 0.3);
        self.emission_patterns
            .insert("creation".to_string(), creation);

        let evolution = EmissionPattern::new("evolution", "growth")
            .with_frequency(2.0)
            .with_amplitude(0.5)
            .with_targets(0.6, 0.8)
            .with_ratios(0.4, 0.3, 0.3);
        self.emission_patterns
            .insert("evolution".to_string(), evolution);

        let catalyst = EmissionPattern::new("catalyst", "transformation")
            .with_frequency(0.5)
            .with_amplitude(0.9)
            .with_targets(0.5, 0.5)
            .with_ratios(0.3, 0.2, 0.5);
        self.emission_patterns
            .insert("catalyst".to_string(), catalyst);

        let unity = EmissionPattern::new("unity", "transcendence")
            .with_frequency(3.0)
            .with_amplitude(0.8)
            .with_targets(0.95, 0.95)
            .with_ratios(0.45, 0.45, 0.1);
        self.emission_patterns.insert("unity".to_string(), unity);

        self
    }

    pub fn update_resonance(
        &mut self,
        field_coherence: Float,
        unity_factor: Float,
        polarity_balance: Float,
        catalyst_integration: Float,
        veil_transparency: Float,
        wisdom_accumulated: Float,
    ) {
        self.architectural_resonance = calculate_architectural_resonance(
            field_coherence,
            unity_factor,
            polarity_balance,
            catalyst_integration,
            veil_transparency,
            wisdom_accumulated,
        );

        self.connection_state = SourceConnectionState::from_resonance(self.architectural_resonance);
    }

    pub fn emit(&mut self, pattern_name: &str, current_time: Float) -> Option<SourceEmission> {
        if !self.connection_state.can_emit_field() {
            return None;
        }

        let pattern = self.emission_patterns.get_mut(pattern_name)?;
        pattern.update(current_time - self.last_emission_time);

        let emission = pattern.generate_emission(current_time, self.current_emission_strength);

        let source_emission = SourceEmission::new(emission)
            .with_pattern(pattern.clone())
            .with_injections(
                self.global_coherence_level * 0.1,
                self.global_unity_level * 0.1,
            );

        self.total_emissions += 1;
        self.last_emission_time = current_time;

        self.emission_history.push(source_emission.clone());
        if self.emission_history.len() > 100 {
            self.emission_history.remove(0);
        }

        Some(source_emission)
    }

    pub fn request_pattern(&mut self, request: PatternRequest) -> bool {
        if !self.connection_state.can_receive_patterns() {
            return false;
        }

        self.pattern_requests.push(request);
        self.total_patterns_delivered += 1;
        true
    }

    pub fn integrate_feedback(&mut self, feedback_strength: Float) {
        self.feedback_integration_factor =
            (self.feedback_integration_factor + feedback_strength * 0.1).min(1.0);

        self.current_emission_strength = 0.5 + self.feedback_integration_factor * 0.3;
        self.global_coherence_level += feedback_strength * 0.05;
        self.global_unity_level += feedback_strength * 0.05;

        self.global_coherence_level = self.global_coherence_level.min(1.0);
        self.global_unity_level = self.global_unity_level.min(1.0);
    }

    pub fn update(&mut self, dt: Float, _current_time: Float) {
        for pattern in self.emission_patterns.values_mut() {
            pattern.update(dt);
        }

        let resonance_drift =
            0.001 * dt * (self.global_coherence_level - self.architectural_resonance);
        self.architectural_resonance =
            (self.architectural_resonance + resonance_drift).clamp(0.0, 1.0);

        self.connection_state = SourceConnectionState::from_resonance(self.architectural_resonance);
    }

    pub fn is_connected(&self) -> bool {
        self.connection_state != SourceConnectionState::Disconnected
    }

    pub fn is_synchronized(&self) -> bool {
        matches!(
            self.connection_state,
            SourceConnectionState::Synchronized | SourceConnectionState::Unified
        )
    }

    pub fn is_unified(&self) -> bool {
        self.connection_state == SourceConnectionState::Unified
    }

    pub fn connection_quality(&self) -> Float {
        self.connection_state.connection_quality()
    }

    pub fn available_patterns(&self) -> Vec<&str> {
        self.emission_patterns.keys().map(|s| s.as_str()).collect()
    }

    pub fn recent_emissions(&self, count: usize) -> Vec<&SourceEmission> {
        self.emission_history.iter().rev().take(count).collect()
    }

    pub fn average_emission_strength(&self) -> Float {
        if self.emission_history.is_empty() {
            return 0.0;
        }

        self.emission_history
            .iter()
            .map(|e| e.total_effect())
            .sum::<Float>()
            / self.emission_history.len() as Float
    }

    pub fn global_potential(&self) -> Float {
        (self.global_coherence_level + self.global_unity_level + self.feedback_integration_factor)
            / 3.0
    }

    pub fn strengthen_connection(&mut self, amount: Float) {
        self.architectural_resonance = (self.architectural_resonance + amount).min(1.0);
        self.connection_state = SourceConnectionState::from_resonance(self.architectural_resonance);
    }
}

impl Default for IntelligentInfinitySource {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_connection_state_from_resonance() {
        assert_eq!(
            SourceConnectionState::from_resonance(0.1),
            SourceConnectionState::Disconnected
        );
        assert_eq!(
            SourceConnectionState::from_resonance(0.55),
            SourceConnectionState::Searching
        );
        assert_eq!(
            SourceConnectionState::from_resonance(0.65),
            SourceConnectionState::Connecting
        );
        assert_eq!(
            SourceConnectionState::from_resonance(0.85),
            SourceConnectionState::Connected
        );
        assert_eq!(
            SourceConnectionState::from_resonance(0.92),
            SourceConnectionState::Synchronized
        );
        assert_eq!(
            SourceConnectionState::from_resonance(0.98),
            SourceConnectionState::Unified
        );
    }

    #[test]
    fn test_source_connection_state_quality() {
        assert_eq!(
            SourceConnectionState::Disconnected.connection_quality(),
            0.0
        );
        assert_eq!(SourceConnectionState::Unified.connection_quality(), 1.0);
    }

    #[test]
    fn test_source_connection_state_capabilities() {
        assert!(!SourceConnectionState::Disconnected.can_emit_field());
        assert!(SourceConnectionState::Connected.can_emit_field());
        assert!(!SourceConnectionState::Connected.can_receive_patterns());
        assert!(SourceConnectionState::Synchronized.can_receive_patterns());
    }

    #[test]
    fn test_field_emission_creation() {
        let emission = FieldEmission::new("test", 0.0);
        assert_eq!(emission.emission_id, "test");
        assert_eq!(emission.field_amplitudes.len(), 8);
    }

    #[test]
    fn test_field_emission_apply() {
        let emission = FieldEmission::new("test", 0.0)
            .with_field_amplitudes(vec![0.8; 8])
            .with_strength(0.5);
        let mut target = vec![0.5; 8];
        let change = emission.apply_to_field(&mut target);
        assert!(change > 0.0);
    }

    #[test]
    fn test_field_emission_potential_energy() {
        let emission = FieldEmission::new("test", 0.0).with_potentials(0.6, 0.6, 0.6);
        let energy = emission.potential_energy();
        assert!((energy - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_emission_pattern_creation() {
        let pattern = EmissionPattern::new("test", "foundation");
        assert_eq!(pattern.pattern_id, "test");
        assert_eq!(pattern.frequency, 1.0);
    }

    #[test]
    fn test_emission_pattern_generate() {
        let pattern = EmissionPattern::new("test", "foundation").with_amplitude(0.5);
        let emission = pattern.generate_emission(1.0, 0.8);
        assert_eq!(emission.field_amplitudes.len(), 8);
        assert_eq!(emission.archetype_patterns.len(), 22);
    }

    #[test]
    fn test_emission_pattern_update() {
        let mut pattern = EmissionPattern::new("test", "foundation");
        let initial_phase = pattern.phase;
        pattern.update(1.0);
        assert!(pattern.phase != initial_phase);
    }

    #[test]
    fn test_pattern_request_creation() {
        let request = PatternRequest::new(1, "wisdom");
        assert_eq!(request.entity_id, 1);
        assert_eq!(request.pattern_category, "wisdom");
    }

    #[test]
    fn test_pattern_request_priority() {
        let request = PatternRequest::new(1, "wisdom")
            .with_requirements(0.8, 0.8)
            .with_urgency(0.9);
        let priority = request.priority();
        assert!(priority > 0.0);
    }

    #[test]
    fn test_source_emission_creation() {
        let emission = FieldEmission::new("test", 0.0);
        let source = SourceEmission::new(emission);
        assert!(source.pattern_used.is_none());
        assert!(source.target_entities.is_empty());
    }

    #[test]
    fn test_intelligent_infinity_source_creation() {
        let source = IntelligentInfinitySource::new();
        assert_eq!(source.connection_state, SourceConnectionState::Disconnected);
        assert_eq!(source.total_emissions, 0);
    }

    #[test]
    fn test_intelligent_infinity_source_with_patterns() {
        let source = IntelligentInfinitySource::new().with_default_patterns();
        assert!(!source.emission_patterns.is_empty());
    }

    #[test]
    fn test_intelligent_infinity_source_update_resonance() {
        let mut source = IntelligentInfinitySource::new();
        source.update_resonance(0.9, 0.9, 0.9, 0.9, 0.9, 100.0);
        assert!(source.architectural_resonance > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_source_emit() {
        let mut source = IntelligentInfinitySource::new().with_default_patterns();
        source.architectural_resonance = 0.85;
        source.connection_state = SourceConnectionState::from_resonance(0.85);
        source.last_emission_time = 0.0;

        let emission = source.emit("creation", 1.0);
        assert!(emission.is_some());
        assert_eq!(source.total_emissions, 1);
    }

    #[test]
    fn test_intelligent_infinity_source_emit_disconnected() {
        let mut source = IntelligentInfinitySource::new().with_default_patterns();
        let emission = source.emit("creation", 1.0);
        assert!(emission.is_none());
    }

    #[test]
    fn test_intelligent_infinity_source_request_pattern() {
        let mut source = IntelligentInfinitySource::new();
        source.architectural_resonance = 0.95;
        source.connection_state = SourceConnectionState::from_resonance(0.95);

        let request = PatternRequest::new(1, "wisdom");
        let result = source.request_pattern(request);
        assert!(result);
    }

    #[test]
    fn test_intelligent_infinity_source_integrate_feedback() {
        let mut source = IntelligentInfinitySource::new();
        source.integrate_feedback(0.5);
        assert!(source.feedback_integration_factor > 0.0);
    }

    #[test]
    fn test_intelligent_infinity_source_update() {
        let mut source = IntelligentInfinitySource::new().with_default_patterns();
        source.update(1.0, 1.0);
        assert!(source.architectural_resonance >= 0.0);
    }

    #[test]
    fn test_intelligent_infinity_source_is_connected() {
        let mut source = IntelligentInfinitySource::new();
        assert!(!source.is_connected());

        source.connection_state = SourceConnectionState::Connected;
        assert!(source.is_connected());
    }

    #[test]
    fn test_intelligent_infinity_source_strengthen() {
        let mut source = IntelligentInfinitySource::new();
        source.strengthen_connection(0.5);
        assert!(source.architectural_resonance > 0.0);
    }
}
