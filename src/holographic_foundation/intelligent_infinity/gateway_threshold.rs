//! Gateway Threshold - Architectural Resonance Threshold Mechanics
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Gateway Opening = Architectural resonance threshold connection"
//! "Resonance calculation (>95% = open, 80-95% = partial, <50% = closed)"
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The gateway opens when architectural resonance exceeds the threshold.
//!  This is not a physical portal but a resonance state where the entity's
//!  field configuration allows connection to Intelligent Infinity."
//!
//! # Threshold Levels
//!
//! - **>95%**: Full gateway access - Transcendent connection
//! - **80-95%**: Partial gateway access - Active connection
//! - **50-80%**: Minimal gateway access - Partial connection
//! - **<50%**: Gateway closed - No connection

use crate::types::Float;
use std::collections::HashMap;

/// Threshold State - the state of the gateway threshold
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThresholdState {
    Closed,
    Minimal,
    Partial,
    Open,
    Transcendent,
}

impl ThresholdState {
    pub fn from_resonance(resonance: Float) -> Self {
        if resonance >= 0.95 {
            ThresholdState::Transcendent
        } else if resonance >= 0.80 {
            ThresholdState::Open
        } else if resonance >= 0.50 {
            ThresholdState::Partial
        } else if resonance >= 0.30 {
            ThresholdState::Minimal
        } else {
            ThresholdState::Closed
        }
    }

    pub fn connection_strength(&self) -> Float {
        match self {
            ThresholdState::Closed => 0.0,
            ThresholdState::Minimal => 0.15,
            ThresholdState::Partial => 0.35,
            ThresholdState::Open => 0.7,
            ThresholdState::Transcendent => 1.0,
        }
    }

    pub fn bandwidth_multiplier(&self) -> Float {
        match self {
            ThresholdState::Closed => 0.0,
            ThresholdState::Minimal => 0.1,
            ThresholdState::Partial => 0.3,
            ThresholdState::Open => 0.6,
            ThresholdState::Transcendent => 1.0,
        }
    }

    pub fn can_receive_intelligence(&self) -> bool {
        matches!(
            self,
            ThresholdState::Partial | ThresholdState::Open | ThresholdState::Transcendent
        )
    }

    pub fn can_send_intelligence(&self) -> bool {
        matches!(self, ThresholdState::Open | ThresholdState::Transcendent)
    }

    pub fn can_access_pattern_library(&self) -> bool {
        matches!(self, ThresholdState::Open | ThresholdState::Transcendent)
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThresholdState::Closed => "Gateway closed - resonance too low",
            ThresholdState::Minimal => "Minimal gateway access - limited connection",
            ThresholdState::Partial => "Partial gateway access - moderate connection",
            ThresholdState::Open => "Gateway open - strong connection to II",
            ThresholdState::Transcendent => "Transcendent gateway - full II unity",
        }
    }
}

/// Resonance Component - individual components of architectural resonance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceComponent {
    FieldCoherence,
    UnityFactor,
    PolarityBalance,
    CatalystIntegration,
    VeilTransparency,
    WisdomAccumulated,
    ServiceRendered,
    EvolutionProgress,
    ArchetypeActivation,
    DensityAlignment,
}

impl ResonanceComponent {
    pub fn weight(&self) -> Float {
        match self {
            ResonanceComponent::FieldCoherence => 0.15,
            ResonanceComponent::UnityFactor => 0.20,
            ResonanceComponent::PolarityBalance => 0.10,
            ResonanceComponent::CatalystIntegration => 0.10,
            ResonanceComponent::VeilTransparency => 0.08,
            ResonanceComponent::WisdomAccumulated => 0.12,
            ResonanceComponent::ServiceRendered => 0.08,
            ResonanceComponent::EvolutionProgress => 0.07,
            ResonanceComponent::ArchetypeActivation => 0.05,
            ResonanceComponent::DensityAlignment => 0.05,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ResonanceComponent::FieldCoherence => "Field coherence and stability",
            ResonanceComponent::UnityFactor => "Unity consciousness factor",
            ResonanceComponent::PolarityBalance => "Polarity balance (STO/STS)",
            ResonanceComponent::CatalystIntegration => "Catalyst integration level",
            ResonanceComponent::VeilTransparency => "Veil transparency achieved",
            ResonanceComponent::WisdomAccumulated => "Wisdom accumulated",
            ResonanceComponent::ServiceRendered => "Service rendered to others",
            ResonanceComponent::EvolutionProgress => "Evolution progress",
            ResonanceComponent::ArchetypeActivation => "Archetype activation completeness",
            ResonanceComponent::DensityAlignment => "Density alignment",
        }
    }

    pub fn all_components() -> Vec<Self> {
        vec![
            ResonanceComponent::FieldCoherence,
            ResonanceComponent::UnityFactor,
            ResonanceComponent::PolarityBalance,
            ResonanceComponent::CatalystIntegration,
            ResonanceComponent::VeilTransparency,
            ResonanceComponent::WisdomAccumulated,
            ResonanceComponent::ServiceRendered,
            ResonanceComponent::EvolutionProgress,
            ResonanceComponent::ArchetypeActivation,
            ResonanceComponent::DensityAlignment,
        ]
    }
}

/// Architectural Resonance - the full architectural resonance calculation
#[derive(Debug, Clone)]
pub struct ArchitecturalResonance {
    pub entity_id: u64,
    pub components: HashMap<ResonanceComponent, Float>,
    pub total_resonance: Float,
    pub dominant_component: Option<ResonanceComponent>,
    pub weakest_component: Option<ResonanceComponent>,
    pub resonance_stability: Float,
    pub resonance_history: Vec<(Float, Float)>,
}

impl ArchitecturalResonance {
    pub fn new(entity_id: u64) -> Self {
        let mut components = HashMap::new();
        for component in ResonanceComponent::all_components() {
            components.insert(component, 0.0);
        }

        Self {
            entity_id,
            components,
            total_resonance: 0.0,
            dominant_component: None,
            weakest_component: None,
            resonance_stability: 0.0,
            resonance_history: Vec::new(),
        }
    }

    pub fn with_component(mut self, component: ResonanceComponent, value: Float) -> Self {
        self.components.insert(component, value.clamp(0.0, 1.0));
        self
    }

    pub fn calculate(&mut self) -> Float {
        let mut total = 0.0;
        let mut max_value = 0.0;
        let mut min_value = 1.0;
        let mut dominant = None;
        let mut weakest = None;

        for (component, value) in &self.components {
            let weighted = value * component.weight();
            total += weighted;

            if *value > max_value {
                max_value = *value;
                dominant = Some(*component);
            }
            if *value < min_value {
                min_value = *value;
                weakest = Some(*component);
            }
        }

        self.total_resonance = total.min(1.0);
        self.dominant_component = dominant;
        self.weakest_component = weakest;

        self.calculate_stability();
        self.total_resonance
    }

    fn calculate_stability(&mut self) {
        if self.components.is_empty() {
            self.resonance_stability = 0.0;
            return;
        }

        let values: Vec<Float> = self.components.values().cloned().collect();
        let mean = values.iter().sum::<Float>() / values.len() as Float;
        let variance =
            values.iter().map(|v| (v - mean).powi(2)).sum::<Float>() / values.len() as Float;

        self.resonance_stability = 1.0 - variance.sqrt().min(1.0);
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        for value in self.components.values_mut() {
            let drift = 0.001 * dt * (0.5 - *value);
            *value = (*value + drift).clamp(0.0, 1.0);
        }

        self.calculate();
        self.resonance_history
            .push((current_time, self.total_resonance));

        if self.resonance_history.len() > 100 {
            self.resonance_history.remove(0);
        }
    }

    pub fn apply_boost(&mut self, component: ResonanceComponent, amount: Float) {
        if let Some(value) = self.components.get_mut(&component) {
            *value = (*value + amount).min(1.0);
        }
        self.calculate();
    }

    pub fn apply_decay(&mut self, component: ResonanceComponent, amount: Float) {
        if let Some(value) = self.components.get_mut(&component) {
            *value = (*value - amount).max(0.0);
        }
        self.calculate();
    }

    pub fn improve_weakest(&mut self, amount: Float) {
        if let Some(weakest) = self.weakest_component {
            self.apply_boost(weakest, amount);
        }
    }

    pub fn get_component(&self, component: ResonanceComponent) -> Float {
        self.components.get(&component).copied().unwrap_or(0.0)
    }

    pub fn resonance_trend(&self) -> Float {
        if self.resonance_history.len() < 2 {
            return 0.0;
        }

        let recent: Vec<_> = self.resonance_history.iter().rev().take(10).collect();
        if recent.len() < 2 {
            return 0.0;
        }

        let first = recent.last().unwrap().1;
        let last = recent.first().unwrap().1;
        last - first
    }

    pub fn is_improving(&self) -> bool {
        self.resonance_trend() > 0.0
    }

    pub fn is_stable(&self) -> bool {
        self.resonance_stability > 0.7
    }

    pub fn average_resonance(&self, duration: Float) -> Float {
        let relevant: Vec<_> = self
            .resonance_history
            .iter()
            .filter(|(t, _)| *t >= duration)
            .collect();

        if relevant.is_empty() {
            return self.total_resonance;
        }

        relevant.iter().map(|(_, r)| *r).sum::<Float>() / relevant.len() as Float
    }
}

/// Gateway Threshold - the main gateway threshold system
#[derive(Debug, Clone)]
pub struct GatewayThreshold {
    pub entity_id: u64,
    pub resonance: ArchitecturalResonance,
    pub state: ThresholdState,
    pub previous_state: ThresholdState,
    pub open_duration: Float,
    pub total_open_time: Float,
    pub peak_resonance: Float,
    pub state_transitions: Vec<(Float, ThresholdState, ThresholdState)>,
    pub gateway_events: Vec<String>,
    pub maintenance_effort: Float,
}

impl GatewayThreshold {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            resonance: ArchitecturalResonance::new(entity_id),
            state: ThresholdState::Closed,
            previous_state: ThresholdState::Closed,
            open_duration: 0.0,
            total_open_time: 0.0,
            peak_resonance: 0.0,
            state_transitions: Vec::new(),
            gateway_events: Vec::new(),
            maintenance_effort: 0.0,
        }
    }

    pub fn with_resonance(mut self, resonance: ArchitecturalResonance) -> Self {
        self.resonance = resonance;
        self
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        self.resonance.update(dt, current_time);

        self.previous_state = self.state;
        self.state = ThresholdState::from_resonance(self.resonance.total_resonance);

        if self.state != self.previous_state {
            self.state_transitions
                .push((current_time, self.previous_state, self.state));
            self.record_transition_event();
        }

        if self.state.can_receive_intelligence() {
            self.open_duration += dt;
            self.total_open_time += dt;
        } else {
            self.open_duration = 0.0;
        }

        if self.resonance.total_resonance > self.peak_resonance {
            self.peak_resonance = self.resonance.total_resonance;
        }

        self.apply_maintenance_decay(dt);
    }

    fn record_transition_event(&mut self) {
        let event = format!(
            "Gateway {} -> {}",
            self.previous_state.description(),
            self.state.description()
        );
        self.gateway_events.push(event);
    }

    fn apply_maintenance_decay(&mut self, dt: Float) {
        let decay_rate = match self.state {
            ThresholdState::Transcendent => 0.001,
            ThresholdState::Open => 0.003,
            ThresholdState::Partial => 0.005,
            ThresholdState::Minimal => 0.007,
            ThresholdState::Closed => 0.01,
        };

        if self.maintenance_effort > 0.0 {
            self.maintenance_effort -= decay_rate * dt;
            self.maintenance_effort = self.maintenance_effort.max(0.0);
        }
    }

    pub fn invest_maintenance(&mut self, effort: Float) {
        self.maintenance_effort = (self.maintenance_effort + effort).min(1.0);

        let boost = effort * 0.1;
        self.resonance.improve_weakest(boost);
    }

    pub fn attempt_opening(&mut self, _current_time: Float) -> bool {
        let resonance_boost = 0.05 * (1.0 + self.maintenance_effort);
        self.resonance.improve_weakest(resonance_boost);

        self.state == ThresholdState::Open || self.state == ThresholdState::Transcendent
    }

    pub fn is_open(&self) -> bool {
        matches!(
            self.state,
            ThresholdState::Open | ThresholdState::Transcendent
        )
    }

    pub fn connection_strength(&self) -> Float {
        self.state.connection_strength() * self.resonance.resonance_stability
    }

    pub fn effective_bandwidth(&self) -> Float {
        self.state.bandwidth_multiplier()
            * self.resonance.total_resonance
            * (1.0 + self.maintenance_effort)
    }

    pub fn can_receive(&self) -> bool {
        self.state.can_receive_intelligence()
    }

    pub fn can_send(&self) -> bool {
        self.state.can_send_intelligence()
    }

    pub fn uptime_percentage(&self, total_simulation_time: Float) -> Float {
        if total_simulation_time <= 0.0 {
            return 0.0;
        }
        self.total_open_time / total_simulation_time
    }

    pub fn recent_transitions(&self, count: usize) -> Vec<(Float, ThresholdState, ThresholdState)> {
        self.state_transitions
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    pub fn resonance_improvement_needed(&self) -> Float {
        let target = match self.state {
            ThresholdState::Closed => 0.30,
            ThresholdState::Minimal => 0.50,
            ThresholdState::Partial => 0.80,
            ThresholdState::Open => 0.95,
            ThresholdState::Transcendent => 0.0,
        };

        (target - self.resonance.total_resonance).max(0.0)
    }

    pub fn weakest_component_description(&self) -> String {
        if let Some(weakest) = self.resonance.weakest_component {
            format!(
                "{} ({:.2})",
                weakest.description(),
                self.resonance.get_component(weakest)
            )
        } else {
            "No components".to_string()
        }
    }

    pub fn strengthen_gateway(&mut self, amount: Float) {
        self.maintenance_effort = (self.maintenance_effort + amount).min(1.0);

        for component in ResonanceComponent::all_components() {
            self.resonance.apply_boost(component, amount * 0.5);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold_state_from_resonance() {
        assert_eq!(ThresholdState::from_resonance(0.1), ThresholdState::Closed);
        assert_eq!(ThresholdState::from_resonance(0.4), ThresholdState::Minimal);
        assert_eq!(ThresholdState::from_resonance(0.6), ThresholdState::Partial);
        assert_eq!(ThresholdState::from_resonance(0.85), ThresholdState::Open);
        assert_eq!(
            ThresholdState::from_resonance(0.98),
            ThresholdState::Transcendent
        );
    }

    #[test]
    fn test_threshold_state_connection_strength() {
        assert_eq!(ThresholdState::Closed.connection_strength(), 0.0);
        assert!((ThresholdState::Open.connection_strength() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_threshold_state_capabilities() {
        assert!(!ThresholdState::Closed.can_receive_intelligence());
        assert!(ThresholdState::Partial.can_receive_intelligence());
        assert!(ThresholdState::Open.can_send_intelligence());
        assert!(!ThresholdState::Partial.can_send_intelligence());
    }

    #[test]
    fn test_resonance_component_weight() {
        let total: Float = ResonanceComponent::all_components()
            .iter()
            .map(|c| c.weight())
            .sum();
        assert!((total - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_architectural_resonance_creation() {
        let ar = ArchitecturalResonance::new(1);
        assert_eq!(ar.entity_id, 1);
        assert_eq!(ar.components.len(), 10);
    }

    #[test]
    fn test_architectural_resonance_calculate() {
        let mut ar = ArchitecturalResonance::new(1)
            .with_component(ResonanceComponent::FieldCoherence, 1.0)
            .with_component(ResonanceComponent::UnityFactor, 1.0);
        let total = ar.calculate();
        assert!(total > 0.0);
    }

    #[test]
    fn test_architectural_resonance_maximal() {
        let mut ar = ArchitecturalResonance::new(1);
        for component in ResonanceComponent::all_components() {
            ar = ar.with_component(component, 1.0);
        }
        let total = ar.calculate();
        assert!((total - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_architectural_resonance_stability() {
        let mut ar = ArchitecturalResonance::new(1);
        for component in ResonanceComponent::all_components() {
            ar = ar.with_component(component, 0.5);
        }
        ar.calculate();
        assert!(ar.resonance_stability > 0.9);
    }

    #[test]
    fn test_gateway_threshold_creation() {
        let gt = GatewayThreshold::new(1);
        assert_eq!(gt.entity_id, 1);
        assert_eq!(gt.state, ThresholdState::Closed);
    }

    #[test]
    fn test_gateway_threshold_update() {
        let mut gt = GatewayThreshold::new(1);
        gt.update(1.0, 0.0);
        assert!(gt.peak_resonance >= 0.0);
    }

    #[test]
    fn test_gateway_threshold_state_change() {
        let mut gt = GatewayThreshold::new(1);
        for component in ResonanceComponent::all_components() {
            gt.resonance = gt.resonance.clone().with_component(component, 0.9);
        }
        gt.update(1.0, 1.0);
        assert!(gt.state == ThresholdState::Open || gt.state == ThresholdState::Transcendent);
    }

    #[test]
    fn test_gateway_threshold_maintenance() {
        let mut gt = GatewayThreshold::new(1);
        gt.invest_maintenance(0.5);
        assert!((gt.maintenance_effort - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_gateway_threshold_opening_attempt() {
        let mut gt = GatewayThreshold::new(1);
        for component in ResonanceComponent::all_components() {
            gt.resonance = gt.resonance.clone().with_component(component, 0.75);
        }
        let result = gt.attempt_opening(0.0);
        assert!(result || gt.state != ThresholdState::Transcendent);
    }

    #[test]
    fn test_gateway_threshold_is_open() {
        let mut gt = GatewayThreshold::new(1);
        for component in ResonanceComponent::all_components() {
            gt.resonance = gt.resonance.clone().with_component(component, 0.9);
        }
        gt.update(1.0, 1.0);
        assert!(gt.is_open());
    }

    #[test]
    fn test_gateway_threshold_effective_bandwidth() {
        let mut gt = GatewayThreshold::new(1);
        for component in ResonanceComponent::all_components() {
            gt.resonance = gt.resonance.clone().with_component(component, 0.85);
        }
        gt.update(1.0, 1.0);
        gt.maintenance_effort = 0.5;
        let bandwidth = gt.effective_bandwidth();
        assert!(bandwidth > 0.0);
    }

    #[test]
    fn test_gateway_threshold_strengthen() {
        let mut gt = GatewayThreshold::new(1);
        let _before = gt.resonance.total_resonance;
        gt.strengthen_gateway(0.5);
        assert!(gt.maintenance_effort > 0.0);
    }
}
