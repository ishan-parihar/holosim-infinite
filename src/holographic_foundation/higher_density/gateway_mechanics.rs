//! Gateway Mechanics - 6th-7th Density Gateways
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Gateway access to Intelligent Infinity"
//! "Gateway mechanics based on resonance threshold"
//!
//! # Key Insight
//!
//! Gateways are NOT physical portals but resonance thresholds where
//! the entity's field configuration allows connection to Intelligent Infinity.
//! The gateway opens when resonance exceeds 95%, partially opens at 80-95%,
//! and remains closed below 50%.

use crate::types::Float;
use std::collections::HashMap;

/// Gateway State - the state of a gateway connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GatewayState {
    Closed,
    Partial,
    Open,
    Active,
    Transcendent,
}

impl GatewayState {
    pub fn from_resonance(resonance: Float) -> Self {
        if resonance >= 0.95 {
            GatewayState::Transcendent
        } else if resonance >= 0.90 {
            GatewayState::Active
        } else if resonance >= 0.80 {
            GatewayState::Open
        } else if resonance >= 0.50 {
            GatewayState::Partial
        } else {
            GatewayState::Closed
        }
    }

    pub fn connection_strength(&self) -> Float {
        match self {
            GatewayState::Closed => 0.0,
            GatewayState::Partial => 0.3,
            GatewayState::Open => 0.6,
            GatewayState::Active => 0.85,
            GatewayState::Transcendent => 1.0,
        }
    }

    pub fn can_receive_intelligence(&self) -> bool {
        matches!(
            self,
            GatewayState::Open | GatewayState::Active | GatewayState::Transcendent
        )
    }

    pub fn can_send_intelligence(&self) -> bool {
        matches!(self, GatewayState::Active | GatewayState::Transcendent)
    }
}

/// Gateway Resonance - the resonance calculation for gateway access
#[derive(Debug, Clone)]
pub struct GatewayResonance {
    pub entity_id: u64,
    pub field_coherence: Float,
    pub unity_factor: Float,
    pub polarity_balance: Float,
    pub wisdom_accumulated: Float,
    pub service_rendered: Float,
    pub catalyst_integrated: Float,
    pub veil_transparency: Float,
}

impl GatewayResonance {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            field_coherence: 0.5,
            unity_factor: 0.0,
            polarity_balance: 0.5,
            wisdom_accumulated: 0.0,
            service_rendered: 0.0,
            catalyst_integrated: 0.0,
            veil_transparency: 0.0,
        }
    }

    pub fn calculate_resonance(&self) -> Float {
        let coherence_component = self.field_coherence * 0.20;
        let unity_component = self.unity_factor * 0.25;
        let polarity_component = self.polarity_balance * 0.15;
        let wisdom_component = (self.wisdom_accumulated / 100.0).min(1.0) * 0.15;
        let service_component = (self.service_rendered / 100.0).min(1.0) * 0.10;
        let catalyst_component = self.catalyst_integrated * 0.10;
        let veil_component = self.veil_transparency * 0.05;

        (coherence_component
            + unity_component
            + polarity_component
            + wisdom_component
            + service_component
            + catalyst_component
            + veil_component)
            .min(1.0)
    }

    pub fn update(&mut self, dt: Float) {
        let coherence_drift = 0.001 * dt * (0.7 - self.field_coherence);
        self.field_coherence = (self.field_coherence + coherence_drift).clamp(0.1, 1.0);

        self.wisdom_accumulated += 0.01 * dt * self.field_coherence;
    }

    pub fn add_wisdom(&mut self, amount: Float) {
        self.wisdom_accumulated += amount;
    }

    pub fn add_service(&mut self, amount: Float) {
        self.service_rendered += amount;
    }

    pub fn integrate_catalyst(&mut self, amount: Float) {
        self.catalyst_integrated = (self.catalyst_integrated + amount).min(1.0);
    }

    pub fn thin_veil(&mut self, amount: Float) {
        self.veil_transparency = (self.veil_transparency + amount).min(1.0);
    }
}

/// Gateway Access - the mechanism for accessing a gateway
#[derive(Debug, Clone)]
pub struct GatewayAccess {
    pub entity_id: u64,
    pub resonance: GatewayResonance,
    pub state: GatewayState,
    pub connection_attempts: u32,
    pub successful_connections: u32,
    pub intelligence_received: Float,
    pub intelligence_transmitted: Float,
    pub last_connection_time: Float,
}

impl GatewayAccess {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            resonance: GatewayResonance::new(entity_id),
            state: GatewayState::Closed,
            connection_attempts: 0,
            successful_connections: 0,
            intelligence_received: 0.0,
            intelligence_transmitted: 0.0,
            last_connection_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        self.resonance.update(dt);

        let resonance_value = self.resonance.calculate_resonance();
        self.state = GatewayState::from_resonance(resonance_value);

        if self.state.can_receive_intelligence() {
            let received = self.receive_intelligence(dt);
            if received > 0.0 {
                self.successful_connections += 1;
                self.last_connection_time = current_time;
            }
        }
    }

    pub fn attempt_connection(&mut self) -> bool {
        self.connection_attempts += 1;

        let resonance = self.resonance.calculate_resonance();
        if resonance >= 0.8 {
            self.state = GatewayState::from_resonance(resonance);
            true
        } else {
            false
        }
    }

    fn receive_intelligence(&mut self, dt: Float) -> Float {
        let strength = self.state.connection_strength();
        let received = strength * 0.1 * dt * self.resonance.field_coherence;
        self.intelligence_received += received;
        received
    }

    pub fn transmit_intelligence(&mut self, amount: Float) -> Float {
        if !self.state.can_send_intelligence() {
            return 0.0;
        }

        let strength = self.state.connection_strength();
        let transmitted = amount * strength * self.resonance.unity_factor;
        self.intelligence_transmitted += transmitted;
        transmitted
    }

    pub fn request_pattern(&self, pattern_type: &str) -> Option<IntelligentInfinityPattern> {
        if !self.state.can_receive_intelligence() {
            return None;
        }

        Some(IntelligentInfinityPattern::new(
            pattern_type,
            self.state.connection_strength(),
        ))
    }

    pub fn connection_success_rate(&self) -> Float {
        if self.connection_attempts == 0 {
            return 0.0;
        }
        self.successful_connections as Float / self.connection_attempts as Float
    }
}

/// Intelligent Infinity Pattern - patterns received from II
#[derive(Debug, Clone)]
pub struct IntelligentInfinityPattern {
    pub pattern_type: String,
    pub strength: Float,
    pub clarity: Float,
    pub content: HashMap<String, Float>,
}

impl IntelligentInfinityPattern {
    pub fn new(pattern_type: &str, strength: Float) -> Self {
        Self {
            pattern_type: pattern_type.to_string(),
            strength,
            clarity: strength * 0.8,
            content: HashMap::new(),
        }
    }

    pub fn with_content(mut self, key: &str, value: Float) -> Self {
        self.content.insert(key.to_string(), value);
        self
    }

    pub fn get_content(&self, key: &str) -> Float {
        self.content.get(key).copied().unwrap_or(0.0)
    }

    pub fn is_valid(&self) -> bool {
        self.strength > 0.5 && self.clarity > 0.3
    }
}

/// Intelligent Infinity Access - the full II connection system
#[derive(Debug, Clone)]
pub struct IntelligentInfinityAccess {
    pub entity_id: u64,
    pub gateway: GatewayAccess,
    pub pattern_library: HashMap<String, IntelligentInfinityPattern>,
    pub teleological_pull: Float,
    pub active_feedback_loop: bool,
    pub resonance_with_source: Float,
}

impl IntelligentInfinityAccess {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            gateway: GatewayAccess::new(entity_id),
            pattern_library: HashMap::new(),
            teleological_pull: 0.0,
            active_feedback_loop: false,
            resonance_with_source: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        self.gateway.update(dt, current_time);

        let connection_strength = self.gateway.state.connection_strength();
        self.teleological_pull = connection_strength * 0.1 * dt;
        self.resonance_with_source = connection_strength * self.gateway.resonance.unity_factor;

        if self.gateway.state.can_send_intelligence()
            && self.gateway.state.can_receive_intelligence()
        {
            self.active_feedback_loop = true;
        }
    }

    pub fn request_pattern(&mut self, pattern_type: &str) -> Option<IntelligentInfinityPattern> {
        let pattern = self.gateway.request_pattern(pattern_type)?;

        if pattern.is_valid() {
            self.pattern_library
                .insert(pattern_type.to_string(), pattern.clone());
            Some(pattern)
        } else {
            None
        }
    }

    pub fn apply_teleological_pull(&mut self) -> Float {
        let pull = self.teleological_pull;
        self.teleological_pull *= 0.9;
        pull
    }

    pub fn get_pattern(&self, pattern_type: &str) -> Option<&IntelligentInfinityPattern> {
        self.pattern_library.get(pattern_type)
    }

    pub fn send_feedback(&mut self, experience_quality: Float) -> Float {
        if !self.active_feedback_loop {
            return 0.0;
        }

        self.gateway.transmit_intelligence(experience_quality * 0.1)
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.gateway.state,
            GatewayState::Active | GatewayState::Transcendent
        )
    }

    pub fn connection_level(&self) -> Float {
        self.resonance_with_source
    }
}

/// Gateway Mechanics - the full gateway system
#[derive(Debug, Clone)]
pub struct GatewayMechanics {
    pub entity_id: u64,
    pub ii_access: IntelligentInfinityAccess,
    pub gateway_history: Vec<(Float, GatewayState)>,
    pub total_time_open: Float,
    pub peak_resonance: Float,
}

impl GatewayMechanics {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            ii_access: IntelligentInfinityAccess::new(entity_id),
            gateway_history: Vec::new(),
            total_time_open: 0.0,
            peak_resonance: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float, current_time: Float) {
        let previous_state = self.ii_access.gateway.state;
        self.ii_access.update(dt, current_time);
        let current_state = self.ii_access.gateway.state;

        if current_state != previous_state {
            self.gateway_history.push((current_time, current_state));
        }

        if current_state != GatewayState::Closed {
            self.total_time_open += dt;
        }

        let resonance = self.ii_access.gateway.resonance.calculate_resonance();
        if resonance > self.peak_resonance {
            self.peak_resonance = resonance;
        }
    }

    pub fn open_gateway(&mut self) -> bool {
        self.ii_access.gateway.attempt_connection()
    }

    pub fn request_wisdom_pattern(&mut self) -> Option<IntelligentInfinityPattern> {
        self.ii_access.request_pattern("wisdom")
    }

    pub fn request_healing_pattern(&mut self) -> Option<IntelligentInfinityPattern> {
        self.ii_access.request_pattern("healing")
    }

    pub fn request_emergence_pattern(&mut self) -> Option<IntelligentInfinityPattern> {
        self.ii_access.request_pattern("emergence")
    }

    pub fn record_experience(&mut self, quality: Float) -> Float {
        self.ii_access.send_feedback(quality)
    }

    pub fn is_gateway_open(&self) -> bool {
        self.ii_access.gateway.state != GatewayState::Closed
    }

    pub fn gateway_uptime(&self, total_simulation_time: Float) -> Float {
        if total_simulation_time <= 0.0 {
            return 0.0;
        }
        self.total_time_open / total_simulation_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_state_from_resonance() {
        assert_eq!(GatewayState::from_resonance(0.3), GatewayState::Closed);
        assert_eq!(GatewayState::from_resonance(0.6), GatewayState::Partial);
        assert_eq!(GatewayState::from_resonance(0.85), GatewayState::Open);
        assert_eq!(GatewayState::from_resonance(0.92), GatewayState::Active);
        assert_eq!(
            GatewayState::from_resonance(0.98),
            GatewayState::Transcendent
        );
    }

    #[test]
    fn test_gateway_state_connection_strength() {
        assert_eq!(GatewayState::Closed.connection_strength(), 0.0);
        assert!((GatewayState::Open.connection_strength() - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_gateway_state_can_receive() {
        assert!(!GatewayState::Closed.can_receive_intelligence());
        assert!(GatewayState::Open.can_receive_intelligence());
    }

    #[test]
    fn test_gateway_state_can_send() {
        assert!(!GatewayState::Open.can_send_intelligence());
        assert!(GatewayState::Active.can_send_intelligence());
    }

    #[test]
    fn test_gateway_resonance_creation() {
        let resonance = GatewayResonance::new(1);
        assert_eq!(resonance.entity_id, 1);
    }

    #[test]
    fn test_gateway_resonance_calculate() {
        let mut resonance = GatewayResonance::new(1);
        resonance.field_coherence = 0.9;
        resonance.unity_factor = 0.9;
        resonance.polarity_balance = 0.9;
        let calculated = resonance.calculate_resonance();
        assert!(calculated > 0.0 && calculated <= 1.0);
    }

    #[test]
    fn test_gateway_resonance_update() {
        let mut resonance = GatewayResonance::new(1);
        resonance.update(1.0);
        assert!(resonance.wisdom_accumulated > 0.0);
    }

    #[test]
    fn test_gateway_access_creation() {
        let access = GatewayAccess::new(1);
        assert_eq!(access.entity_id, 1);
        assert_eq!(access.state, GatewayState::Closed);
    }

    #[test]
    fn test_gateway_access_attempt() {
        let mut access = GatewayAccess::new(1);
        access.resonance.field_coherence = 1.0;
        access.resonance.unity_factor = 1.0;
        access.resonance.polarity_balance = 1.0;
        access.resonance.catalyst_integrated = 1.0;
        access.resonance.veil_transparency = 1.0;
        access.resonance.wisdom_accumulated = 100.0;
        access.resonance.service_rendered = 100.0;
        let result = access.attempt_connection();
        assert!(result);
    }

    #[test]
    fn test_intelligent_infinity_pattern_creation() {
        let pattern = IntelligentInfinityPattern::new("wisdom", 0.8);
        assert_eq!(pattern.pattern_type, "wisdom");
        assert!(pattern.is_valid());
    }

    #[test]
    fn test_intelligent_infinity_access_creation() {
        let access = IntelligentInfinityAccess::new(1);
        assert_eq!(access.entity_id, 1);
    }

    #[test]
    fn test_intelligent_infinity_access_update() {
        let mut access = IntelligentInfinityAccess::new(1);
        access.update(1.0, 0.0);
        assert!(access.teleological_pull >= 0.0);
    }

    #[test]
    fn test_intelligent_infinity_access_connected() {
        let mut access = IntelligentInfinityAccess::new(1);
        access.gateway.state = GatewayState::Active;
        assert!(access.is_connected());
    }

    #[test]
    fn test_gateway_mechanics_creation() {
        let mechanics = GatewayMechanics::new(1);
        assert_eq!(mechanics.entity_id, 1);
    }

    #[test]
    fn test_gateway_mechanics_update() {
        let mut mechanics = GatewayMechanics::new(1);
        mechanics.update(1.0, 1.0);
        assert!(mechanics.peak_resonance >= 0.0);
    }

    #[test]
    fn test_gateway_mechanics_open() {
        let mut mechanics = GatewayMechanics::new(1);
        mechanics.ii_access.gateway.resonance.field_coherence = 1.0;
        mechanics.ii_access.gateway.resonance.unity_factor = 1.0;
        mechanics.ii_access.gateway.resonance.polarity_balance = 1.0;
        mechanics.ii_access.gateway.resonance.catalyst_integrated = 1.0;
        mechanics.ii_access.gateway.resonance.veil_transparency = 1.0;
        mechanics.ii_access.gateway.resonance.wisdom_accumulated = 100.0;
        mechanics.ii_access.gateway.resonance.service_rendered = 100.0;
        let result = mechanics.open_gateway();
        assert!(result);
    }
}
