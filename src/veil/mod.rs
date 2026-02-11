// Veil System - Phase 3: Green-Ray Realm
// Implements the separation mechanism that enables evolution
//
// The Veil creates the illusion of separation between entities and unity,
// obscuring higher truths while allowing genuine free will choice.
//
// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 3.2
// "Veil is 'playground' not barrier"

pub mod density_variation;
pub mod mechanism;
pub mod piercing;
pub mod playground;

pub use density_variation::{DensityTransparency, PolarizationAccess, PolarizationState};
pub use mechanism::{ThinSpot, VeilEffects, VeilMechanism, VeilState, VeilStatistics};
pub use piercing::{PiercingEvent, PiercingEventType, PiercingLocation, PiercingResult};
pub use playground::{
    PlaygroundAction, PlaygroundResult, PlaygroundStatistics, VeilChallenge, VeilChallengeType,
    VeilOpportunity, VeilOpportunityType, VeilPlayground,
};

// Constants
pub const ACCESS_THRESHOLD: f64 = 0.3;
pub const THIN_SPOT_THRESHOLD: f64 = 0.05;
pub const TRANSLATION_THRESHOLD: f64 = 0.4;
