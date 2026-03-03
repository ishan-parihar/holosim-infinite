//! Social Memory + Resonance - Collective formation through resonance
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 5:
//! "Implement collective formation through resonance, not proximity.
//!  Entities can connect across any distance if their field configurations align."
//!
//! KEY PRINCIPLE: "Collectives form by resonance, not proximity"
//! - Phase alignment determines connection strength
//! - High resonance = strong connection potential
//! - Distance is irrelevant for resonance-based connections

mod collective_dynamics;
mod collective_formation;
mod resonance_calculation;
mod social_memory;

pub use collective_dynamics::{
    compute_emergent_properties, CollectiveDecision, CollectiveDynamics, CollectivePolarity,
    DecisionOutcome, EmergenceSource, EmergentProperty, EmergentPropertyType,
};
pub use collective_formation::{
    Collective, CollectiveConfig, CollectiveFormation, CollectiveState, FormationEvent,
    FormationEventType,
};
pub use resonance_calculation::{PhaseAlignment, ResonanceCalculation, ResonanceResult};
pub use social_memory::{
    ExperienceEncoding, ExperienceType, SharedExperience, SocialMemory, SocialMemoryEntry,
};

use crate::types::Float;

pub const MIN_RESONANCE_THRESHOLD: Float = 0.3;
pub const STABLE_COLLECTIVE_THRESHOLD: Float = 0.5;
pub const DISSOLUTION_THRESHOLD: Float = 0.2;
pub const MAX_COLLECTIVE_SIZE: usize = 1000;
pub const MEMORY_CAPACITY: usize = 10000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionType {
    Resonance,
    Polarity,
    Density,
    Archetype,
    Catalyst,
}

impl ConnectionType {
    pub fn weight(&self) -> Float {
        match self {
            ConnectionType::Resonance => 1.0,
            ConnectionType::Polarity => 0.8,
            ConnectionType::Density => 0.6,
            ConnectionType::Archetype => 0.7,
            ConnectionType::Catalyst => 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResonanceConnection {
    pub entity_a: u64,
    pub entity_b: u64,
    pub connection_type: ConnectionType,
    pub strength: Float,
    pub distance: Float,
    pub established_time: Float,
}

impl ResonanceConnection {
    pub fn new(
        entity_a: u64,
        entity_b: u64,
        connection_type: ConnectionType,
        strength: Float,
        distance: Float,
        time: Float,
    ) -> Self {
        Self {
            entity_a,
            entity_b,
            connection_type,
            strength: strength.clamp(0.0, 1.0),
            distance,
            established_time: time,
        }
    }

    pub fn is_strong(&self) -> bool {
        self.strength >= STABLE_COLLECTIVE_THRESHOLD
    }

    pub fn effective_strength(&self) -> Float {
        self.strength * self.connection_type.weight()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_type_weights() {
        assert!(ConnectionType::Resonance.weight() >= ConnectionType::Catalyst.weight());
    }

    #[test]
    fn test_resonance_connection_creation() {
        let conn = ResonanceConnection::new(1, 2, ConnectionType::Resonance, 0.7, 100.0, 0.0);
        assert!(conn.is_strong());
    }

    #[test]
    fn test_effective_strength() {
        let conn = ResonanceConnection::new(1, 2, ConnectionType::Resonance, 0.5, 0.0, 0.0);
        assert_eq!(conn.effective_strength(), 0.5);
    }
}
