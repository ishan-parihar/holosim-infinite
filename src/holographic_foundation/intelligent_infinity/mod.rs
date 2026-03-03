//! Phase 15: Intelligent Infinity Integration (FINAL PHASE)
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Intelligent Infinity becomes the ACTIVE SOURCE of the simulation.
//!  - Teleological Pull = Field gradient toward unity (spiritual gravity)
//!  - Pattern Library = Field templates for successful emergence
//!  - Active Feedback = Entity experiences → II analysis → adjusted emission
//!  - Gateway Opening = Architectural resonance threshold connection"
//!
//! # Key Insight
//!
//! The simulation itself becomes a GATEWAY to Intelligent Infinity through
//! architectural resonance. This is not simulation OF II but actual CONNECTION
//! through proper field configuration.
//!
//! # Theory
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Intelligent Infinity is the source of all existence. It is not a place
//! but a state of infinite potential. The teleological pull draws all
//! consciousness toward unity - this is spiritual gravity."
//!
//! # Components
//!
//! 1. **Teleological Pull**: Field gradient that pulls entities toward unity
//!    - Not force but attraction through resonance
//!    - Increases as entity approaches source
//!    - Operates at all density levels
//!
//! 2. **Pattern Library**: Templates for successful emergence patterns
//!    - Learned from entity experiences
//!    - Templates for atoms, molecules, cells, organisms
//!    - Templates for collective formations
//!    - Templates for density transitions
//!
//! 3. **Active Feedback**: Bidirectional communication with II
//!    - Entity experiences uploaded to II
//!    - II adjusts emission patterns based on feedback
//!    - Creates personalized evolution paths
//!
//! 4. **Gateway Threshold**: Architectural resonance requirements
//!    - >95% resonance = Full gateway access
//!    - 80-95% resonance = Partial gateway access
//!    - <50% resonance = Gateway closed
//!    - Resonance = field coherence + unity factor + polarity balance
//!
//! 5. **II Source**: The simulation source emanates from II
//!    - Field emission from II to simulation
//!    - Pattern templates flow from II
//!    - Evolution guidance from II

pub mod active_feedback;
pub mod gateway_threshold;
pub mod ii_source;
pub mod pattern_library;
pub mod teleological_pull;

pub use active_feedback::{
    ActiveFeedbackLoop, EntityExperience, FeedbackAnalysis, FeedbackDirection,
    IntelligenceTransmission, TransmissionType,
};
pub use gateway_threshold::{
    ArchitecturalResonance, GatewayThreshold, ResonanceComponent, ThresholdState,
};
pub use ii_source::{
    EmissionPattern, FieldEmission, IntelligentInfinitySource, PatternRequest,
    SourceConnectionState, SourceEmission,
};
pub use pattern_library::{
    EmergenceTemplate, PatternCategory, PatternLibrary, PatternMatch, TemplateSearch,
};
pub use teleological_pull::{
    GravityVector, PullStrength, TeleologicalPull, UnityGradient, UnityPullResult,
};

use crate::types::Float;

/// Resonance threshold constants for gateway access
pub mod resonance_thresholds {
    use crate::types::Float;

    pub const FULL_ACCESS: Float = 0.95;
    pub const PARTIAL_ACCESS: Float = 0.80;
    pub const MINIMAL_ACCESS: Float = 0.50;
    pub const CLOSED: Float = 0.0;
}

/// Calculate architectural resonance from field properties
pub fn calculate_architectural_resonance(
    field_coherence: Float,
    unity_factor: Float,
    polarity_balance: Float,
    catalyst_integration: Float,
    veil_transparency: Float,
    wisdom_accumulated: Float,
) -> Float {
    let coherence_component = field_coherence * 0.20;
    let unity_component = unity_factor * 0.25;
    let polarity_component = polarity_balance * 0.15;
    let catalyst_component = catalyst_integration * 0.15;
    let veil_component = veil_transparency * 0.10;
    let wisdom_component = (wisdom_accumulated / 100.0).min(1.0) * 0.15;

    (coherence_component
        + unity_component
        + polarity_component
        + catalyst_component
        + veil_component
        + wisdom_component)
        .min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_thresholds() {
        assert!((resonance_thresholds::FULL_ACCESS - 0.95).abs() < 0.001);
        assert!((resonance_thresholds::PARTIAL_ACCESS - 0.80).abs() < 0.001);
        assert!((resonance_thresholds::MINIMAL_ACCESS - 0.50).abs() < 0.001);
    }

    #[test]
    fn test_calculate_architectural_resonance_minimal() {
        let resonance = calculate_architectural_resonance(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((resonance - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_architectural_resonance_maximal() {
        let resonance = calculate_architectural_resonance(1.0, 1.0, 1.0, 1.0, 1.0, 100.0);
        assert!((resonance - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_architectural_resonance_partial() {
        let resonance = calculate_architectural_resonance(0.5, 0.5, 0.5, 0.5, 0.5, 50.0);
        assert!(resonance > 0.0 && resonance < 1.0);
    }

    #[test]
    fn test_calculate_architectural_resonance_weights() {
        let coherence_only = calculate_architectural_resonance(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((coherence_only - 0.20).abs() < 0.001);

        let unity_only = calculate_architectural_resonance(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert!((unity_only - 0.25).abs() < 0.001);
    }
}
