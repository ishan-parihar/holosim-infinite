//! Physiology Engine - Organ Communication via Field Wave Propagation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Organ communication as field wave propagation - organs communicate
//!  through field waves, not just chemical signals."
//!
//! # Key Insight
//!
//! Organ communication is field-based:
//! - Hormones and nerves are ONE channel, not the only channel
//! - Field waves propagate between resonant organs
//! - Communication efficiency depends on archetype resonance

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::Position3D;
use crate::holographic_foundation::organism_physiology::organ_field::{OrganFieldNode, OrganId};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType {
    Neural,
    Hormonal,
    Electrical,
    FieldWave,
    Mechanical,
}

impl SignalType {
    pub fn propagation_speed(&self) -> Float {
        match self {
            SignalType::Neural => 100.0,
            SignalType::Hormonal => 0.1,
            SignalType::Electrical => 0.5,
            SignalType::FieldWave => 1.0,
            SignalType::Mechanical => 0.05,
        }
    }

    pub fn attenuation_rate(&self) -> Float {
        match self {
            SignalType::Neural => 0.01,
            SignalType::Hormonal => 0.1,
            SignalType::Electrical => 0.05,
            SignalType::FieldWave => 0.001,
            SignalType::Mechanical => 0.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysiologySignal {
    pub source_organ: OrganId,
    pub target_organ: OrganId,
    pub signal_type: SignalType,
    pub strength: Float,
    pub frequency: Float,
    pub archetype_modulation: [Float; NUM_ARCHETYPES],
    pub timestamp: Float,
}

impl PhysiologySignal {
    pub fn new(source: OrganId, target: OrganId, signal_type: SignalType, strength: Float) -> Self {
        Self {
            source_organ: source,
            target_organ: target,
            signal_type,
            strength,
            frequency: 1.0,
            archetype_modulation: [0.5; NUM_ARCHETYPES],
            timestamp: 0.0,
        }
    }

    pub fn with_frequency(mut self, freq: Float) -> Self {
        self.frequency = freq;
        self
    }

    pub fn with_archetype_modulation(mut self, modulation: [Float; NUM_ARCHETYPES]) -> Self {
        self.archetype_modulation = modulation;
        self
    }

    pub fn propagate(&self, distance: Float) -> Float {
        let attenuation = self.signal_type.attenuation_rate() * distance;
        (self.strength - attenuation).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct FieldWave {
    pub origin: Position3D,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub amplitude: Float,
    pub wavelength: Float,
    pub phase: Float,
    pub velocity: Float,
}

impl FieldWave {
    pub fn new(
        origin: Position3D,
        archetype_pattern: [Float; NUM_ARCHETYPES],
        amplitude: Float,
    ) -> Self {
        Self {
            origin,
            archetype_pattern,
            amplitude,
            wavelength: 1.0,
            phase: 0.0,
            velocity: 1.0,
        }
    }

    pub fn amplitude_at(&self, position: &Position3D, time: Float) -> Float {
        let distance = self.origin.distance(position);
        let phase_at_point = self.phase
            + 2.0 * std::f64::consts::PI * (distance / self.wavelength - time * self.velocity);
        self.amplitude * phase_at_point.cos() * (-distance * 0.1).exp()
    }

    pub fn archetype_resonance_at(
        &self,
        position: &Position3D,
        time: Float,
        organ_pattern: &[Float; NUM_ARCHETYPES],
    ) -> Float {
        let wave_amplitude = self.amplitude_at(position, time);
        let mut resonance = 0.0;

        for (&self_i, &organ_i) in self.archetype_pattern.iter().zip(organ_pattern.iter()) {
            resonance += self_i * organ_i;
        }

        resonance / NUM_ARCHETYPES as Float * wave_amplitude
    }

    pub fn update(&mut self, dt: Float) {
        self.phase += dt * self.velocity * 2.0 * std::f64::consts::PI;
        self.amplitude *= 0.999;
    }
}

#[derive(Debug, Clone)]
pub struct OrganCommunication {
    organs: HashMap<OrganId, OrganFieldNode>,
    signals: Vec<PhysiologySignal>,
    field_waves: Vec<FieldWave>,
    communication_matrix: HashMap<(OrganId, OrganId), Float>,
}

impl OrganCommunication {
    pub fn new() -> Self {
        Self {
            organs: HashMap::new(),
            signals: Vec::new(),
            field_waves: Vec::new(),
            communication_matrix: HashMap::new(),
        }
    }

    pub fn add_organ(&mut self, organ: OrganFieldNode) {
        self.organs.insert(organ.id, organ);
        self.rebuild_communication_matrix();
    }

    fn rebuild_communication_matrix(&mut self) {
        self.communication_matrix.clear();

        let organ_ids: Vec<OrganId> = self.organs.keys().cloned().collect();

        for id1 in &organ_ids {
            for id2 in &organ_ids {
                if id1 != id2 {
                    if let (Some(o1), Some(o2)) = (self.organs.get(id1), self.organs.get(id2)) {
                        let resonance = o1.resonance_with(o2);
                        let distance = o1.position.distance(&o2.position);
                        let efficiency = resonance / (1.0 + distance * 0.01);
                        self.communication_matrix.insert((*id1, *id2), efficiency);
                    }
                }
            }
        }
    }

    pub fn send_signal(&mut self, signal: PhysiologySignal) {
        self.signals.push(signal);
    }

    pub fn emit_field_wave(&mut self, organ_id: OrganId) {
        if let Some(organ) = self.organs.get(&organ_id) {
            let wave = FieldWave::new(
                organ.position,
                organ.archetype_pattern,
                organ.field_output(),
            );
            self.field_waves.push(wave);
        }
    }

    pub fn communication_efficiency(&self, from: &OrganId, to: &OrganId) -> Float {
        self.communication_matrix
            .get(&(*from, *to))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn update(&mut self, dt: Float) {
        for wave in &mut self.field_waves {
            wave.update(dt);
        }

        self.field_waves.retain(|w| w.amplitude > 0.01);

        let mut new_signals = Vec::new();
        for signal in &self.signals {
            if let (Some(source), Some(target)) = (
                self.organs.get(&signal.source_organ),
                self.organs.get(&signal.target_organ),
            ) {
                let distance = source.position.distance(&target.position);
                let efficiency =
                    self.communication_efficiency(&signal.source_organ, &signal.target_organ);
                let propagated_strength = signal.propagate(distance) * efficiency;

                if propagated_strength > 0.01 {
                    let mut new_signal = signal.clone();
                    new_signal.strength = propagated_strength;
                    new_signal.timestamp += dt;
                    new_signals.push(new_signal);
                }
            }
        }

        self.signals = new_signals;
    }

    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    pub fn wave_count(&self) -> usize {
        self.field_waves.len()
    }

    pub fn organ_count(&self) -> usize {
        self.organs.len()
    }
}

impl Default for OrganCommunication {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PhysiologyEngine {
    communication: OrganCommunication,
    global_coherence: Float,
    signal_history: Vec<(Float, PhysiologySignal)>,
    time: Float,
}

impl PhysiologyEngine {
    pub fn new() -> Self {
        Self {
            communication: OrganCommunication::new(),
            global_coherence: 0.8,
            signal_history: Vec::new(),
            time: 0.0,
        }
    }

    pub fn add_organ(&mut self, organ: OrganFieldNode) {
        self.communication.add_organ(organ);
        self.calculate_global_coherence();
    }

    fn calculate_global_coherence(&mut self) {
        if self.communication.organ_count() == 0 {
            self.global_coherence = 0.8;
            return;
        }

        let organs: Vec<&OrganFieldNode> = self.communication.organs.values().collect();
        let mut total_resonance = 0.0;
        let mut count = 0;

        for i in 0..organs.len() {
            for j in (i + 1)..organs.len() {
                total_resonance += organs[i].resonance_with(organs[j]);
                count += 1;
            }
        }

        self.global_coherence = if count > 0 {
            total_resonance / count as Float
        } else {
            0.8
        };
    }

    pub fn send_signal(&mut self, signal: PhysiologySignal) {
        self.signal_history.push((self.time, signal.clone()));
        self.communication.send_signal(signal);
    }

    pub fn emit_wave(&mut self, organ_id: OrganId) {
        self.communication.emit_field_wave(organ_id);
    }

    pub fn update(&mut self, dt: Float) {
        self.time += dt;
        self.communication.update(dt);
        self.calculate_global_coherence();

        if self.signal_history.len() > 1000 {
            self.signal_history.drain(0..100);
        }
    }

    pub fn coherence(&self) -> Float {
        self.global_coherence
    }

    pub fn time(&self) -> Float {
        self.time
    }

    pub fn signal_count(&self) -> usize {
        self.communication.signal_count()
    }

    pub fn wave_count(&self) -> usize {
        self.communication.wave_count()
    }

    pub fn communication_efficiency(&self, from: &OrganId, to: &OrganId) -> Float {
        self.communication.communication_efficiency(from, to)
    }
}

impl Default for PhysiologyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic_foundation::organism_physiology::organ_field::OrganType;

    fn create_test_organ(id: u64, organ_type: OrganType, pos: Position3D) -> OrganFieldNode {
        let mut node = OrganFieldNode::new(organ_type, pos);
        node.id = OrganId::new(id);
        node
    }

    #[test]
    fn test_signal_type_propagation_speed() {
        assert!(SignalType::Neural.propagation_speed() > SignalType::Hormonal.propagation_speed());
    }

    #[test]
    fn test_physiology_signal_creation() {
        let signal =
            PhysiologySignal::new(OrganId::new(1), OrganId::new(2), SignalType::Neural, 0.8);
        assert_eq!(signal.strength, 0.8);
    }

    #[test]
    fn test_physiology_signal_propagate() {
        let signal =
            PhysiologySignal::new(OrganId::new(1), OrganId::new(2), SignalType::Neural, 1.0);
        let propagated = signal.propagate(10.0);
        assert!(propagated < 1.0);
    }

    #[test]
    fn test_field_wave_creation() {
        let wave = FieldWave::new(Position3D::new(0.0, 0.0, 0.0), [0.5; NUM_ARCHETYPES], 1.0);
        assert_eq!(wave.amplitude, 1.0);
    }

    #[test]
    fn test_field_wave_amplitude_at() {
        let wave = FieldWave::new(Position3D::new(0.0, 0.0, 0.0), [0.5; NUM_ARCHETYPES], 1.0);
        let amp = wave.amplitude_at(&Position3D::new(1.0, 0.0, 0.0), 0.0);
        assert!(amp < 1.0);
    }

    #[test]
    fn test_organ_communication_creation() {
        let comm = OrganCommunication::new();
        assert_eq!(comm.organ_count(), 0);
    }

    #[test]
    fn test_organ_communication_add_organ() {
        let mut comm = OrganCommunication::new();
        let organ = create_test_organ(1, OrganType::Heart, Position3D::new(0.0, 0.0, 0.0));
        comm.add_organ(organ);
        assert_eq!(comm.organ_count(), 1);
    }

    #[test]
    fn test_organ_communication_send_signal() {
        let mut comm = OrganCommunication::new();
        let organ1 = create_test_organ(1, OrganType::Heart, Position3D::new(0.0, 0.0, 0.0));
        let organ2 = create_test_organ(2, OrganType::Brain, Position3D::new(1.0, 0.0, 0.0));
        comm.add_organ(organ1);
        comm.add_organ(organ2);

        let signal =
            PhysiologySignal::new(OrganId::new(1), OrganId::new(2), SignalType::Neural, 0.8);
        comm.send_signal(signal);
        assert_eq!(comm.signal_count(), 1);
    }

    #[test]
    fn test_physiology_engine_creation() {
        let engine = PhysiologyEngine::new();
        assert!(engine.coherence() > 0.5);
    }

    #[test]
    fn test_physiology_engine_update() {
        let mut engine = PhysiologyEngine::new();
        engine.update(1.0);
        assert!(engine.time() > 0.0);
    }

    #[test]
    fn test_field_wave_update() {
        let mut wave = FieldWave::new(Position3D::new(0.0, 0.0, 0.0), [0.5; NUM_ARCHETYPES], 1.0);
        wave.update(1.0);
        assert!(wave.amplitude < 1.0);
    }
}
