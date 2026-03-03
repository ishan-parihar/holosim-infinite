//! Density Transition Implementation
//!
//! From ROADMAP: "Implement density transitions (2nd→3rd, 3rd→4th, etc.)"
//! From COSMOLOGICAL-ARCHITECTURE: "Each density transition requires specific catalysts"
//!
//! ## Overview
//!
//! Density transitions are the major evolutionary milestones in consciousness
//! development. Each transition requires specific conditions and catalysts.
//!
//! ## Key Transitions
//!
//! - **2nd→3rd**: Consciousness awakening ("I AM" moment) - handled by ConsciousnessAwakening
//! - **3rd→4th**: Polarization harvest (51% STO or 95% STS)
//! - **4th→5th**: Wisdom integration
//! - **5th→6th**: Unity consciousness
//! - **6th→7th**: Completion
//! - **7th→8th**: Return to Intelligent Infinity
//!
//! ## Transcend and Include
//!
//! From COSMOLOGICAL-ARCHITECTURE.md section 2.6:
//! "Each stage of involution/evolution operates by a single universal constant:
//!  the Transcend and Include operation"
//!
//! At each density transition:
//! 1. INCLUDE: Retain all previous development
//! 2. TRANSCEND: Add new capabilities/awareness
//! 3. EVOLVE INTO: Create attractor for next stage

use crate::entity_layer7::layer7::EntityId;
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::polarization::PolarityDirection;
use crate::template::transcend_include::{
    AttractorField, Feature, Orientation, TargetDensity, TranscendInclude,
};
use crate::types::{Float, Polarity};

/// STO harvest threshold - minimum polarization for harvest
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Service-to-others polarization of 51% or greater is harvestable"
pub const STO_HARVEST_THRESHOLD: Float = 0.51;

/// STS harvest threshold - minimum polarization for harvest
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Service-to-self polarization of 95% or greater is harvestable"
pub const STS_HARVEST_THRESHOLD: Float = 0.95;

/// Result of a density transition attempt
#[derive(Debug, Clone)]
pub struct DensityTransitionResult {
    /// Entity that attempted transition
    pub entity_id: EntityId,

    /// Whether transition succeeded
    pub success: bool,

    /// Previous density
    pub from_density: Density,

    /// New density (if transition succeeded)
    pub to_density: Option<Density>,

    /// Reason for success or failure
    pub reason: String,

    /// Catalyst that triggered the transition
    pub catalyst: TransitionCatalyst,

    /// Transcend and Include operation (if transition occurred)
    pub transcend_include: Option<TranscendData>,
}

/// Data about the Transcend and Include operation at transition
#[derive(Debug, Clone)]
pub struct TranscendData {
    /// What was included (previous development retained)
    pub included: Vec<String>,

    /// What was transcended (new capabilities added)
    pub transcended: Vec<String>,

    /// Attractor field for next stage
    pub next_attractor: String,
}

/// Catalyst that triggers a density transition
#[derive(Debug, Clone)]
pub enum TransitionCatalyst {
    /// Awakening moment (2nd→3rd)
    AwakeningMoment,

    /// Polarization harvest (3rd→4th)
    PolarizationHarvest {
        polarity: Polarity,
        intensity: Float,
    },

    /// Wisdom integration (4th→5th)
    WisdomIntegration { wisdom_level: Float },

    /// Unity consciousness (5th→6th)
    UnityConsciousness,

    /// Completion (6th→7th)
    Completion,

    /// Return to source (7th→8th)
    ReturnToSource,

    /// Not applicable
    NotApplicable,
}

/// Requirements for a density transition
#[derive(Debug, Clone)]
pub struct TransitionRequirements {
    /// Minimum consciousness level required
    pub min_consciousness: Float,

    /// Minimum polarization required (if applicable)
    pub min_polarization: Option<Float>,

    /// Minimum spectrum access required
    pub min_spectrum_access: Float,

    /// Specific catalyst required
    pub required_catalyst: Option<TransitionCatalyst>,

    /// Description of requirements
    pub description: String,
}

impl TransitionRequirements {
    /// Get requirements for 2nd→3rd density transition
    pub fn second_to_third() -> Self {
        Self {
            min_consciousness: 0.5,
            min_polarization: None,
            min_spectrum_access: 0.3,
            required_catalyst: Some(TransitionCatalyst::AwakeningMoment),
            description: "Consciousness awakening ('I AM' moment) required".to_string(),
        }
    }

    /// Get requirements for 3rd→4th density transition
    pub fn third_to_fourth() -> Self {
        Self {
            min_consciousness: 0.6,
            min_polarization: Some(STO_HARVEST_THRESHOLD),
            min_spectrum_access: 0.5,
            required_catalyst: Some(TransitionCatalyst::PolarizationHarvest {
                polarity: Polarity::Neutral,
                intensity: STO_HARVEST_THRESHOLD,
            }),
            description: format!("51% STO or 95% STS polarization required for harvest"),
        }
    }

    /// Get requirements for 4th→5th density transition
    pub fn fourth_to_fifth() -> Self {
        Self {
            min_consciousness: 0.8,
            min_polarization: None, // Wisdom, not polarization
            min_spectrum_access: 0.7,
            required_catalyst: Some(TransitionCatalyst::WisdomIntegration { wisdom_level: 0.7 }),
            description: "Wisdom integration required".to_string(),
        }
    }

    /// Get requirements for 5th→6th density transition
    pub fn fifth_to_sixth() -> Self {
        Self {
            min_consciousness: 0.9,
            min_polarization: None,
            min_spectrum_access: 0.85,
            required_catalyst: Some(TransitionCatalyst::UnityConsciousness),
            description: "Unity consciousness required".to_string(),
        }
    }

    /// Get requirements for 6th→7th density transition
    pub fn sixth_to_seventh() -> Self {
        Self {
            min_consciousness: 0.95,
            min_polarization: None,
            min_spectrum_access: 0.95,
            required_catalyst: Some(TransitionCatalyst::Completion),
            description: "Completion of lessons required".to_string(),
        }
    }

    /// Get requirements for 7th→8th density transition
    pub fn seventh_to_eighth() -> Self {
        Self {
            min_consciousness: 1.0,
            min_polarization: None,
            min_spectrum_access: 1.0,
            required_catalyst: Some(TransitionCatalyst::ReturnToSource),
            description: "Return to Intelligent Infinity".to_string(),
        }
    }

    /// Get requirements for a specific transition
    pub fn for_transition(from: &Density, to: &Density) -> Option<Self> {
        match (from, to) {
            (Density::Second(_), Density::Third) => Some(Self::second_to_third()),
            (Density::Third, Density::Fourth) => Some(Self::third_to_fourth()),
            (Density::Fourth, Density::Fifth) => Some(Self::fourth_to_fifth()),
            (Density::Fifth, Density::Sixth) => Some(Self::fifth_to_sixth()),
            (Density::Sixth, Density::Seventh) => Some(Self::sixth_to_seventh()),
            (Density::Seventh, Density::Eighth) => Some(Self::seventh_to_eighth()),
            _ => None,
        }
    }
}

/// Harvest eligibility status
#[derive(Debug, Clone)]
pub struct HarvestEligibility {
    /// Whether entity is eligible for harvest
    pub eligible: bool,

    /// Current polarity
    pub polarity: Option<Polarity>,

    /// Current polarization intensity
    pub polarization_intensity: Float,

    /// Target density if eligible
    pub target_density: Option<Density>,

    /// Reason for ineligibility (if not eligible)
    pub reason: Option<String>,

    /// Progress toward harvest threshold (0.0 to 1.0)
    pub progress: Float,
}

impl HarvestEligibility {
    /// Create an eligible result
    pub fn eligible(polarity: Polarity, intensity: Float, target_density: Density) -> Self {
        Self {
            eligible: true,
            polarity: Some(polarity),
            polarization_intensity: intensity,
            target_density: Some(target_density),
            reason: None,
            progress: 1.0,
        }
    }

    /// Create an ineligible result
    pub fn ineligible(
        polarity: Option<Polarity>,
        intensity: Float,
        target_density: Density,
        reason: &str,
    ) -> Self {
        // Calculate progress toward threshold
        let progress = match polarity {
            Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                (intensity / STO_HARVEST_THRESHOLD).min(1.0)
            }
            Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => {
                (intensity / STS_HARVEST_THRESHOLD).min(1.0)
            }
            _ => intensity.min(1.0), // Neutral or unknown
        };

        Self {
            eligible: false,
            polarity,
            polarization_intensity: intensity,
            target_density: Some(target_density),
            reason: Some(reason.to_string()),
            progress,
        }
    }
}

/// Entity state for transition checking
///
/// This is a lightweight representation of entity state needed for
/// transition evaluation, avoiding heavy dependencies on full Entity type.
#[derive(Debug, Clone)]
pub struct EntityTransitionState {
    /// Entity ID
    pub entity_id: EntityId,

    /// Current density
    pub current_density: Density,

    /// Consciousness level (0.0 to 1.0+)
    pub consciousness_level: Float,

    /// Spectrum access level (0.0 to 1.0)
    pub spectrum_access: Float,

    /// Current polarity (if chosen)
    pub polarity: Option<Polarity>,

    /// Polarization intensity (0.0 to 1.0)
    pub polarization_intensity: Float,

    /// Wisdom level (for 4th→5th transition)
    pub wisdom_level: Float,

    /// Unity consciousness level (for 5th→6th transition)
    pub unity_level: Float,

    /// Completion level (for 6th→7th transition)
    pub completion_level: Float,

    /// Whether the entity has awakened (2nd→3rd transition)
    pub awakened: bool,

    /// Time in current density (simulation ticks)
    pub time_in_density: u64,
}

impl EntityTransitionState {
    /// Create a new transition state for an entity
    pub fn new(entity_id: EntityId, current_density: Density) -> Self {
        Self {
            entity_id,
            current_density,
            consciousness_level: 0.1,
            spectrum_access: 0.1,
            polarity: None,
            polarization_intensity: 0.0,
            wisdom_level: 0.0,
            unity_level: 0.0,
            completion_level: 0.0,
            awakened: false,
            time_in_density: 0,
        }
    }

    /// Create with explicit values
    pub fn with_values(
        entity_id: EntityId,
        current_density: Density,
        consciousness_level: Float,
        spectrum_access: Float,
        polarity: Option<Polarity>,
        polarization_intensity: Float,
    ) -> Self {
        Self {
            entity_id,
            current_density,
            consciousness_level,
            spectrum_access,
            polarity,
            polarization_intensity,
            wisdom_level: 0.0,
            unity_level: 0.0,
            completion_level: 0.0,
            awakened: false,
            time_in_density: 0,
        }
    }
}

/// Density Transition System
///
/// Manages density transitions for entities, including:
/// - Harvest eligibility checking
/// - Transcend and Include mechanics
/// - Transition execution
///
/// From ROADMAP:
/// ```text
/// DensityTransitionSystem {
///     entity: Entity,
///     current_density: Density,
/// }
///
/// check_harvest_eligibility() -> HarvestEligibility
/// ```
#[derive(Debug, Clone)]
pub struct DensityTransitionSystem {
    /// Entity being tracked
    pub entity_state: EntityTransitionState,

    /// History of transition attempts
    pub transition_history: Vec<DensityTransitionResult>,

    /// Whether currently transitioning
    pub is_transitioning: bool,

    /// Current transition progress (0.0 to 1.0)
    pub transition_progress: Float,
}

impl DensityTransitionSystem {
    /// Create a new density transition system for an entity
    pub fn new(entity_state: EntityTransitionState) -> Self {
        Self {
            entity_state,
            transition_history: Vec::new(),
            is_transitioning: false,
            transition_progress: 0.0,
        }
    }

    /// Create a new system with minimal entity info
    pub fn for_entity(entity_id: EntityId, current_density: Density) -> Self {
        Self::new(EntityTransitionState::new(entity_id, current_density))
    }

    /// Check harvest eligibility for 3rd→4th density transition
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// - STO: 51%+ polarization required
    /// - STS: 95%+ polarization required
    ///
    /// # Returns
    ///
    /// HarvestEligibility indicating whether the entity can transition to 4th density
    pub fn check_harvest_eligibility(&self) -> HarvestEligibility {
        // Only applicable for 3rd density entities
        if self.entity_state.current_density != Density::Third {
            return HarvestEligibility::ineligible(
                self.entity_state.polarity,
                self.entity_state.polarization_intensity,
                Density::Fourth,
                "Not in 3rd density - harvest not applicable",
            );
        }

        match self.entity_state.polarity {
            Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                if self.entity_state.polarization_intensity >= STO_HARVEST_THRESHOLD {
                    HarvestEligibility::eligible(
                        Polarity::STO,
                        self.entity_state.polarization_intensity,
                        Density::Fourth,
                    )
                } else {
                    HarvestEligibility::ineligible(
                        self.entity_state.polarity,
                        self.entity_state.polarization_intensity,
                        Density::Fourth,
                        &format!(
                            "STO polarization {:.0}% below 51% threshold",
                            self.entity_state.polarization_intensity * 100.0
                        ),
                    )
                }
            }
            Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => {
                if self.entity_state.polarization_intensity >= STS_HARVEST_THRESHOLD {
                    HarvestEligibility::eligible(
                        Polarity::STS,
                        self.entity_state.polarization_intensity,
                        Density::Fourth,
                    )
                } else {
                    HarvestEligibility::ineligible(
                        self.entity_state.polarity,
                        self.entity_state.polarization_intensity,
                        Density::Fourth,
                        &format!(
                            "STS polarization {:.0}% below 95% threshold",
                            self.entity_state.polarization_intensity * 100.0
                        ),
                    )
                }
            }
            Some(Polarity::Neutral) | None => HarvestEligibility::ineligible(
                self.entity_state.polarity,
                self.entity_state.polarization_intensity,
                Density::Fourth,
                "No polarity chosen - must polarize before harvest",
            ),
            Some(Polarity::SinkholeOfIndifference) => HarvestEligibility::ineligible(
                self.entity_state.polarity,
                self.entity_state.polarization_intensity,
                Density::Fourth,
                "Sinkhole of Indifference - not harvestable",
            ),
        }
    }

    /// Check readiness for any density transition
    ///
    /// # Returns
    ///
    /// (is_ready, requirements) tuple
    pub fn check_transition_readiness(&self) -> (bool, Option<TransitionRequirements>) {
        let next_density = self.get_next_density();

        if let Some(next) = next_density {
            let requirements =
                TransitionRequirements::for_transition(&self.entity_state.current_density, &next);

            if let Some(reqs) = requirements {
                let is_ready = self.meets_requirements(&reqs);
                return (is_ready, Some(reqs));
            }
        }

        (false, None)
    }

    /// Get the next density in the sequence
    pub fn get_next_density(&self) -> Option<Density> {
        match &self.entity_state.current_density {
            Density::First(sub) => Some(match sub {
                Density1SubLevel::Quantum => Density::First(Density1SubLevel::Atomic),
                Density1SubLevel::Atomic => Density::First(Density1SubLevel::Molecular),
                Density1SubLevel::Molecular => Density::First(Density1SubLevel::Planetary),
                Density1SubLevel::Planetary => Density::Second(Density2SubLevel::Cellular),
            }),
            Density::Second(sub) => Some(match sub {
                Density2SubLevel::Cellular => Density::Second(Density2SubLevel::SimpleLife),
                Density2SubLevel::SimpleLife => Density::Second(Density2SubLevel::ComplexLife),
                Density2SubLevel::ComplexLife => Density::Third,
            }),
            Density::Third => Some(Density::Fourth),
            Density::Fourth => Some(Density::Fifth),
            Density::Fifth => Some(Density::Sixth),
            Density::Sixth => Some(Density::Seventh),
            Density::Seventh => Some(Density::Eighth),
            Density::Eighth => None,
        }
    }

    /// Check if entity meets transition requirements
    fn meets_requirements(&self, requirements: &TransitionRequirements) -> bool {
        // Check consciousness level
        if self.entity_state.consciousness_level < requirements.min_consciousness {
            return false;
        }

        // Check spectrum access
        if self.entity_state.spectrum_access < requirements.min_spectrum_access {
            return false;
        }

        // Check polarization if required
        if let Some(min_pol) = requirements.min_polarization {
            match self.entity_state.polarity {
                Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                    if self.entity_state.polarization_intensity < STO_HARVEST_THRESHOLD {
                        return false;
                    }
                }
                Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => {
                    if self.entity_state.polarization_intensity < STS_HARVEST_THRESHOLD {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        // Check for awakening (2nd→3rd)
        if matches!(
            requirements.required_catalyst,
            Some(TransitionCatalyst::AwakeningMoment)
        ) {
            if !self.entity_state.awakened {
                return false;
            }
        }

        true
    }

    /// Execute a density transition
    ///
    /// This applies the Transcend and Include operation:
    /// 1. INCLUDE: Retain all previous development
    /// 2. TRANSCEND: Add new capabilities
    /// 3. EVOLVE INTO: Create attractor for next stage
    ///
    /// # Returns
    ///
    /// DensityTransitionResult indicating success or failure
    pub fn execute_transition(&mut self) -> DensityTransitionResult {
        // Check if already transitioning
        if self.is_transitioning {
            return DensityTransitionResult {
                entity_id: self.entity_state.entity_id.clone(),
                success: false,
                from_density: self.entity_state.current_density.clone(),
                to_density: None,
                reason: "Already transitioning".to_string(),
                catalyst: TransitionCatalyst::NotApplicable,
                transcend_include: None,
            };
        }

        // Check for maximum density FIRST
        let next_density = match self.get_next_density() {
            Some(d) => d,
            None => {
                return DensityTransitionResult {
                    entity_id: self.entity_state.entity_id.clone(),
                    success: false,
                    from_density: self.entity_state.current_density.clone(),
                    to_density: None,
                    reason: "Already at maximum density".to_string(),
                    catalyst: TransitionCatalyst::NotApplicable,
                    transcend_include: None,
                }
            }
        };

        // Check readiness
        let (is_ready, requirements) = self.check_transition_readiness();

        if !is_ready {
            let reqs_desc = requirements
                .map(|r| r.description)
                .unwrap_or_else(|| "Unknown requirements".to_string());

            return DensityTransitionResult {
                entity_id: self.entity_state.entity_id.clone(),
                success: false,
                from_density: self.entity_state.current_density.clone(),
                to_density: None,
                reason: format!("Not ready: {}", reqs_desc),
                catalyst: TransitionCatalyst::NotApplicable,
                transcend_include: None,
            };
        }

        // Determine catalyst
        let catalyst = self.determine_catalyst(&next_density);

        // Apply Transcend and Include
        let transcend_data = self.apply_transcend_include(&next_density);

        // Execute the transition
        let old_density = self.entity_state.current_density.clone();
        self.entity_state.current_density = next_density.clone();
        self.entity_state.time_in_density = 0;

        // Record the result
        let result = DensityTransitionResult {
            entity_id: self.entity_state.entity_id.clone(),
            success: true,
            from_density: old_density,
            to_density: Some(next_density),
            reason: "Transition successful".to_string(),
            catalyst,
            transcend_include: Some(transcend_data),
        };

        self.transition_history.push(result.clone());

        result
    }

    /// Determine the catalyst for a transition
    fn determine_catalyst(&self, next_density: &Density) -> TransitionCatalyst {
        match next_density {
            Density::Third => TransitionCatalyst::AwakeningMoment,
            Density::Fourth => TransitionCatalyst::PolarizationHarvest {
                polarity: self.entity_state.polarity.unwrap_or(Polarity::Neutral),
                intensity: self.entity_state.polarization_intensity,
            },
            Density::Fifth => TransitionCatalyst::WisdomIntegration {
                wisdom_level: self.entity_state.wisdom_level,
            },
            Density::Sixth => TransitionCatalyst::UnityConsciousness,
            Density::Seventh => TransitionCatalyst::Completion,
            Density::Eighth => TransitionCatalyst::ReturnToSource,
            _ => TransitionCatalyst::NotApplicable,
        }
    }

    /// Apply Transcend and Include operation
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md section 2.6:
    /// "Each stage of involution/evolution operates by a single universal constant"
    fn apply_transcend_include(&self, next_density: &Density) -> TranscendData {
        let (included, transcended, next_attractor) = match next_density {
            Density::Third => {
                // 2nd→3rd: Awaken self-awareness
                (
                    vec![
                        "instinctual awareness".to_string(),
                        "biological consciousness".to_string(),
                        "growth and survival drives".to_string(),
                    ],
                    vec![
                        "self-awareness".to_string(),
                        "choice capacity".to_string(),
                        "polarity potential".to_string(),
                    ],
                    "4th Density Attractor (Love/Understanding)".to_string(),
                )
            }
            Density::Fourth => {
                // 3rd→4th: Polarization harvest
                (
                    vec![
                        "self-awareness".to_string(),
                        "choice capacity".to_string(),
                        "polarity choice".to_string(),
                        "catalyst processing".to_string(),
                    ],
                    vec![
                        "social memory complex potential".to_string(),
                        "enhanced telepathy".to_string(),
                        "thinning veil".to_string(),
                    ],
                    "5th Density Attractor (Light/Wisdom)".to_string(),
                )
            }
            Density::Fifth => {
                // 4th→5th: Wisdom integration
                (
                    vec![
                        "social memory complex".to_string(),
                        "telepathy".to_string(),
                        "love/understanding".to_string(),
                    ],
                    vec![
                        "wisdom integration".to_string(),
                        "light body potential".to_string(),
                        "teaching/learning focus".to_string(),
                    ],
                    "6th Density Attractor (Unity)".to_string(),
                )
            }
            Density::Sixth => {
                // 5th→6th: Unity consciousness
                (
                    vec![
                        "wisdom".to_string(),
                        "light body".to_string(),
                        "teaching capacity".to_string(),
                    ],
                    vec![
                        "unity consciousness".to_string(),
                        "balanced polarity".to_string(),
                        "equality of all".to_string(),
                    ],
                    "7th Density Attractor (Gateway)".to_string(),
                )
            }
            Density::Seventh => {
                // 6th→7th: Completion
                (
                    vec![
                        "unity consciousness".to_string(),
                        "balanced polarity".to_string(),
                    ],
                    vec![
                        "completion of lessons".to_string(),
                        "gateway readiness".to_string(),
                    ],
                    "8th Density Attractor (Return to Source)".to_string(),
                )
            }
            Density::Eighth => {
                // 7th→8th: Return to source
                (
                    vec![
                        "all accumulated experience".to_string(),
                        "all wisdom".to_string(),
                    ],
                    vec![
                        "merging with Intelligent Infinity".to_string(),
                        "next octave potential".to_string(),
                    ],
                    "Next Octave".to_string(),
                )
            }
            _ => (
                vec!["previous development".to_string()],
                vec!["new capabilities".to_string()],
                "Next Stage".to_string(),
            ),
        };

        TranscendData {
            included,
            transcended,
            next_attractor,
        }
    }

    /// Update entity state
    pub fn update_state(&mut self, state: EntityTransitionState) {
        self.entity_state = state;
    }

    /// Update specific values in entity state
    pub fn update_values(
        &mut self,
        consciousness_level: Float,
        spectrum_access: Float,
        polarization_intensity: Float,
    ) {
        self.entity_state.consciousness_level = consciousness_level;
        self.entity_state.spectrum_access = spectrum_access;
        self.entity_state.polarization_intensity = polarization_intensity;
    }

    /// Set the entity's polarity
    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.entity_state.polarity = Some(polarity);
    }

    /// Set the awakened state
    pub fn set_awakened(&mut self, awakened: bool) {
        self.entity_state.awakened = awakened;
    }

    /// Increment time in current density
    pub fn increment_time(&mut self) {
        self.entity_state.time_in_density += 1;
    }

    /// Get transition history
    pub fn get_history(&self) -> &[DensityTransitionResult] {
        &self.transition_history
    }

    /// Check if entity is at maximum density
    pub fn is_complete(&self) -> bool {
        matches!(self.entity_state.current_density, Density::Eighth)
    }
}

/// Check if an entity is ready for density transition
///
/// This is a convenience function for quick transition checks.
pub fn check_transition_readiness(
    entity_id: &EntityId,
    current_density: &Density,
    consciousness_level: Float,
    polarization: Option<(Polarity, Float)>,
    spectrum_access: Float,
) -> TransitionRequirements {
    let (polarity, intensity) = polarization.unwrap_or((Polarity::Neutral, 0.0));

    match current_density {
        Density::Second(_) => {
            let mut reqs = TransitionRequirements::second_to_third();
            if consciousness_level >= reqs.min_consciousness
                && spectrum_access >= reqs.min_spectrum_access
            {
                reqs.description = "Ready for awakening".to_string();
            }
            reqs
        }
        Density::Third => {
            let mut reqs = TransitionRequirements::third_to_fourth();
            match polarity {
                Polarity::STO | Polarity::ServiceToOthers => {
                    if intensity >= STO_HARVEST_THRESHOLD {
                        reqs.description = "Ready for harvest (STO)".to_string();
                    }
                }
                Polarity::STS | Polarity::ServiceToSelf => {
                    if intensity >= STS_HARVEST_THRESHOLD {
                        reqs.description = "Ready for harvest (STS)".to_string();
                    }
                }
                _ => {}
            }
            reqs
        }
        _ => TransitionRequirements::second_to_third(), // Default
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_to_third_requirements() {
        let reqs = TransitionRequirements::second_to_third();

        assert!((reqs.min_consciousness - 0.5).abs() < 0.001);
        assert!(reqs.min_polarization.is_none());
        assert!((reqs.min_spectrum_access - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_third_to_fourth_requirements() {
        let reqs = TransitionRequirements::third_to_fourth();

        assert!((reqs.min_consciousness - 0.6).abs() < 0.001);
        assert!(reqs.min_polarization.is_some());
    }

    #[test]
    fn test_harvest_threshold_constants() {
        assert!((STO_HARVEST_THRESHOLD - 0.51).abs() < 0.001);
        assert!((STS_HARVEST_THRESHOLD - 0.95).abs() < 0.001);
    }

    #[test]
    fn test_density_transition_system_creation() {
        let entity_id = EntityId::new("test".to_string());
        let system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);

        assert_eq!(system.entity_state.entity_id, entity_id);
        assert_eq!(system.entity_state.current_density, Density::Third);
        assert!(system.transition_history.is_empty());
        assert!(!system.is_transitioning);
    }

    #[test]
    fn test_check_harvest_eligibility_sto_eligible() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.55); // 55% STO polarization

        let eligibility = system.check_harvest_eligibility();

        assert!(eligibility.eligible);
        assert_eq!(eligibility.polarity, Some(Polarity::STO));
        assert_eq!(eligibility.target_density, Some(Density::Fourth));
    }

    #[test]
    fn test_check_harvest_eligibility_sto_insufficient() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.40); // 40% STO polarization (below 51%)

        let eligibility = system.check_harvest_eligibility();

        assert!(!eligibility.eligible);
        assert!(eligibility.reason.is_some());
        assert!((eligibility.progress - (0.40 / 0.51)).abs() < 0.01);
    }

    #[test]
    fn test_check_harvest_eligibility_sts_eligible() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        system.set_polarity(Polarity::STS);
        system.update_values(0.7, 0.6, 0.96); // 96% STS polarization

        let eligibility = system.check_harvest_eligibility();

        assert!(eligibility.eligible);
        assert_eq!(eligibility.polarity, Some(Polarity::STS));
        assert_eq!(eligibility.target_density, Some(Density::Fourth));
    }

    #[test]
    fn test_check_harvest_eligibility_sts_insufficient() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        system.set_polarity(Polarity::STS);
        system.update_values(0.7, 0.6, 0.80); // 80% STS polarization (below 95%)

        let eligibility = system.check_harvest_eligibility();

        assert!(!eligibility.eligible);
        assert!(eligibility.reason.is_some());
        assert!((eligibility.progress - (0.80 / 0.95)).abs() < 0.01);
    }

    #[test]
    fn test_check_harvest_eligibility_no_polarity() {
        let entity_id = EntityId::new("test".to_string());
        let system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        let eligibility = system.check_harvest_eligibility();

        assert!(!eligibility.eligible);
        assert!(eligibility.reason.unwrap().contains("No polarity chosen"));
    }

    #[test]
    fn test_check_harvest_eligibility_wrong_density() {
        let entity_id = EntityId::new("test".to_string());
        let system = DensityTransitionSystem::for_entity(
            entity_id,
            Density::Second(Density2SubLevel::ComplexLife),
        );

        let eligibility = system.check_harvest_eligibility();

        assert!(!eligibility.eligible);
        assert!(eligibility.reason.unwrap().contains("Not in 3rd density"));
    }

    #[test]
    fn test_get_next_density() {
        let entity_id = EntityId::new("test".to_string());

        let system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);
        assert_eq!(system.get_next_density(), Some(Density::Fourth));

        let system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Fourth);
        assert_eq!(system.get_next_density(), Some(Density::Fifth));

        let system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Eighth);
        assert_eq!(system.get_next_density(), None);
    }

    #[test]
    fn test_execute_transition_success() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);

        // Set up for successful transition
        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.55);

        let result = system.execute_transition();

        assert!(result.success);
        assert_eq!(result.from_density, Density::Third);
        assert_eq!(result.to_density, Some(Density::Fourth));
        assert!(result.transcend_include.is_some());
        assert_eq!(system.entity_state.current_density, Density::Fourth);
    }

    #[test]
    fn test_execute_transition_not_ready() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);

        // Not polarized enough
        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.30);

        let result = system.execute_transition();

        assert!(!result.success);
        assert!(result.reason.contains("Not ready"));
    }

    #[test]
    fn test_execute_transition_already_complete() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Eighth);

        let result = system.execute_transition();

        assert!(!result.success);
        assert!(result.reason.contains("maximum density"));
    }

    #[test]
    fn test_transcend_include_data() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);

        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.55);

        let result = system.execute_transition();
        let ti = result.transcend_include.unwrap();

        assert!(!ti.included.is_empty());
        assert!(!ti.transcended.is_empty());
        assert!(ti.next_attractor.contains("5th"));
    }

    #[test]
    fn test_transition_history() {
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);

        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.55);

        let _ = system.execute_transition();

        assert_eq!(system.transition_history.len(), 1);
        assert!(system.transition_history[0].success);
    }

    #[test]
    fn test_entity_transition_state() {
        let entity_id = EntityId::new("test".to_string());
        let state = EntityTransitionState::with_values(
            entity_id.clone(),
            Density::Third,
            0.7,
            0.6,
            Some(Polarity::STO),
            0.55,
        );

        assert_eq!(state.entity_id, entity_id);
        assert_eq!(state.current_density, Density::Third);
        assert!((state.consciousness_level - 0.7).abs() < 0.001);
        assert_eq!(state.polarity, Some(Polarity::STO));
    }

    #[test]
    fn test_is_complete() {
        let entity_id = EntityId::new("test".to_string());

        let system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);
        assert!(!system.is_complete());

        let system = DensityTransitionSystem::for_entity(entity_id, Density::Eighth);
        assert!(system.is_complete());
    }

    #[test]
    fn test_transition_requirements_for_transition() {
        let reqs = TransitionRequirements::for_transition(&Density::Third, &Density::Fourth);
        assert!(reqs.is_some());
        assert!(reqs.unwrap().description.contains("51%"));

        let reqs = TransitionRequirements::for_transition(
            &Density::Eighth,
            &Density::First(Density1SubLevel::Quantum),
        );
        assert!(reqs.is_none());
    }

    #[test]
    fn test_harvest_eligibility_progress_calculation() {
        // STO at 25.5% should be 50% progress toward 51%
        let entity_id = EntityId::new("test".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);
        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.255);

        let eligibility = system.check_harvest_eligibility();
        assert!((eligibility.progress - 0.5).abs() < 0.01);

        // STS at 47.5% should be 50% progress toward 95%
        let entity_id = EntityId::new("test2".to_string());
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Third);
        system.set_polarity(Polarity::STS);
        system.update_values(0.7, 0.6, 0.475);

        let eligibility = system.check_harvest_eligibility();
        assert!((eligibility.progress - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_catalyst_determination() {
        let entity_id = EntityId::new("test".to_string());

        // 3rd→4th should have polarization harvest catalyst
        let mut system = DensityTransitionSystem::for_entity(entity_id.clone(), Density::Third);
        system.set_polarity(Polarity::STO);
        system.update_values(0.7, 0.6, 0.55);
        let result = system.execute_transition();
        assert!(matches!(
            result.catalyst,
            TransitionCatalyst::PolarizationHarvest { .. }
        ));

        // 4th→5th should have wisdom integration catalyst
        let mut system = DensityTransitionSystem::for_entity(entity_id, Density::Fourth);
        system.entity_state.wisdom_level = 0.8;
        system.update_values(0.85, 0.75, 0.0);
        let result = system.execute_transition();
        assert!(matches!(
            result.catalyst,
            TransitionCatalyst::WisdomIntegration { .. }
        ));
    }
}
