//! Noosphere Module - Collective Consciousness and Societal Emergence
//!
//! This module implements the noospheric systems for collective consciousness,
//! social memory complexes, and societal emergence patterns.
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
//! "Noospheric Systems: Social memory complexes, collective consciousness,
//! cultural emergence, ideological formation, societal evolution"

pub mod noosphere_emergence;
pub mod social_memory_complex;

pub use social_memory_complex::{
    CollectiveMemory, GroupConsciousness, GroupConsciousnessResult, MemoryFormationResult,
    MemoryFormer, ResonanceCalculator, ResonanceResult, SocialMemoryComplex, TelepathicLink,
    TelepathyResult,
};

pub use noosphere_emergence::{
    CollectiveIntelligence, CollectiveIntelligenceResult, CulturalEmerger, Culture,
    CultureEmergenceResult, IdeologicalFormer, Ideology, IdeologyFormationResult, Noosphere,
    NoosphereEmergence, SocietalEvolution, SocietalEvolutionResult, SocietalTracker, Society,
};

/// Configuration for noosphere systems
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The noosphere represents the sphere of
/// human thought and collective consciousness, emerging from biological systems
/// as social entities develop shared memory and communication."
#[derive(Debug, Clone, PartialEq)]
pub struct NoosphereConfig {
    /// Resonance threshold for social memory complex formation (0.0-1.0)
    pub resonance_threshold: Float,

    /// Telepathy range for collective communication (0.0-1.0)
    pub telepathy_range: Float,

    /// Cultural emergence rate (0.0-1.0)
    pub cultural_emergence_rate: Float,

    /// Ideological formation rate (0.0-1.0)
    pub ideological_formation_rate: Float,

    /// Societal evolution rate (0.0-1.0)
    pub societal_evolution_rate: Float,

    /// Collective intelligence weight (0.0-1.0)
    pub collective_intelligence_weight: Float,
}

impl Default for NoosphereConfig {
    fn default() -> Self {
        NoosphereConfig {
            resonance_threshold: 0.7,
            telepathy_range: 0.5,
            cultural_emergence_rate: 0.3,
            ideological_formation_rate: 0.2,
            societal_evolution_rate: 0.1,
            collective_intelligence_weight: 0.5,
        }
    }
}

impl NoosphereConfig {
    /// Create a new noosphere configuration
    pub fn new(
        resonance_threshold: Float,
        telepathy_range: Float,
        cultural_emergence_rate: Float,
        ideological_formation_rate: Float,
        societal_evolution_rate: Float,
        collective_intelligence_weight: Float,
    ) -> Self {
        NoosphereConfig {
            resonance_threshold,
            telepathy_range,
            cultural_emergence_rate,
            ideological_formation_rate,
            societal_evolution_rate,
            collective_intelligence_weight,
        }
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.resonance_threshold) {
            return Err(format!(
                "Resonance threshold must be between 0.0 and 1.0, got {}",
                self.resonance_threshold
            ));
        }
        if !(0.0..=1.0).contains(&self.telepathy_range) {
            return Err(format!(
                "Telepathy range must be between 0.0 and 1.0, got {}",
                self.telepathy_range
            ));
        }
        if !(0.0..=1.0).contains(&self.cultural_emergence_rate) {
            return Err(format!(
                "Cultural emergence rate must be between 0.0 and 1.0, got {}",
                self.cultural_emergence_rate
            ));
        }
        if !(0.0..=1.0).contains(&self.ideological_formation_rate) {
            return Err(format!(
                "Ideological formation rate must be between 0.0 and 1.0, got {}",
                self.ideological_formation_rate
            ));
        }
        if !(0.0..=1.0).contains(&self.societal_evolution_rate) {
            return Err(format!(
                "Societal evolution rate must be between 0.0 and 1.0, got {}",
                self.societal_evolution_rate
            ));
        }
        if !(0.0..=1.0).contains(&self.collective_intelligence_weight) {
            return Err(format!(
                "Collective intelligence weight must be between 0.0 and 1.0, got {}",
                self.collective_intelligence_weight
            ));
        }
        Ok(())
    }
}

// Re-export common types
pub type Float = f64;
pub type EntityId = u64;
pub type SocietyId = u64;
pub type CultureId = u64;
pub type IdeologyId = u64;
