// Spectrum Access Mechanism
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Evolution is not about moving from Space/Time to Time/Space—it's about
// accessing more of the spectrum"
//
// This module implements:
// 1. Spectrum access mechanism (evolution as accessing more of the spectrum)
// 2. The Veil as access limitation mechanism
// 3. Spectrum access levels by density
// 4. Evolution as spectrum access, not movement between realms

use crate::entity_layer7::layer7::{EntitySpectrumAccess, EntityState, SpectrumAccessLevel};
use crate::evolution_density_octave::density_octave::Density1SubLevel;

/// Spectrum Access Mechanism
///
/// Evolution is about accessing more of the spectrum, not moving between realms.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The holographic principle ensures that each entity contains the ENTIRE spectrum—
/// the Veil is a limitation of ACCESS, not of BEING"
#[derive(Debug, Clone)]
pub struct SpectrumAccessMechanism {
    /// Current spectrum access level
    pub access_level: SpectrumAccessLevel,

    /// Veil state
    pub veil_state: SpectrumAccessVeilState,

    /// Spectrum access capabilities
    pub access_capabilities: AccessCapabilities,

    /// Evolutionary trajectory through spectrum access
    pub evolutionary_trajectory: AccessTrajectory,
}

impl SpectrumAccessMechanism {
    /// Create a new spectrum access mechanism starting at 3rd density
    pub fn new() -> Self {
        SpectrumAccessMechanism {
            access_level: SpectrumAccessLevel::ThirdDensity,
            veil_state: SpectrumAccessVeilState::FullyActive,
            access_capabilities: AccessCapabilities::third_density(),
            evolutionary_trajectory: AccessTrajectory::new(),
        }
    }

    /// Calculate spectrum access based on entity state
    pub fn calculate_access(&self, entity_state: &EntityState) -> EntitySpectrumAccess {
        // Calculate access level based on entity's consciousness level (not shared state)
        let access_level = entity_state.calculate_access_level();

        let oneness_access = self.calculate_oneness_access(entity_state, access_level);
        let veil_transparency = self.calculate_veil_transparency(entity_state, access_level);

        // Phase 7: Calculate space/time and time/space ratios
        let space_time_ratio = 1.0 - oneness_access;
        let time_space_ratio = oneness_access;

        // Phase 7: Calculate spectrum position on continuum
        let spectrum_position = space_time_ratio;

        EntitySpectrumAccess {
            space_time_access: space_time_ratio,
            oneness_access,
            veil_transparency,
            evolutionary_level: access_level,
            space_time_ratio,
            time_space_ratio,
            spectrum_position,
        }
    }

    /// Calculate oneness access based on evolutionary state
    fn calculate_oneness_access(
        &self,
        entity_state: &EntityState,
        access_level: SpectrumAccessLevel,
    ) -> f64 {
        match access_level {
            SpectrumAccessLevel::ThirdDensity => {
                // Veil fully active - limited oneness access
                (entity_state.consciousness_level * 0.3).min(0.1)
            }
            SpectrumAccessLevel::FourthDensity => {
                // Veil begins to thin - increased oneness access
                (entity_state.consciousness_level * 0.7).min(0.4)
            }
            SpectrumAccessLevel::FifthDensity => {
                // Veil mostly dissolved - significant oneness access
                (entity_state.consciousness_level * 0.9).min(0.7)
            }
            SpectrumAccessLevel::SixthDensity => {
                // Veil completely dissolved - full oneness access
                entity_state.consciousness_level.min(1.0)
            }
            SpectrumAccessLevel::SeventhDensity => {
                // Transcends spectrum - complete oneness access
                1.0
            }
        }
    }

    /// Calculate veil transparency based on evolutionary state
    fn calculate_veil_transparency(
        &self,
        entity_state: &EntityState,
        access_level: SpectrumAccessLevel,
    ) -> f64 {
        match access_level {
            SpectrumAccessLevel::ThirdDensity => {
                // Veil fully active
                0.1
            }
            SpectrumAccessLevel::FourthDensity => {
                // Veil begins to thin
                (entity_state.consciousness_level * 0.5).min(0.4)
            }
            SpectrumAccessLevel::FifthDensity => {
                // Veil mostly dissolved
                (entity_state.consciousness_level * 0.8).min(0.7)
            }
            SpectrumAccessLevel::SixthDensity => {
                // Veil completely dissolved
                1.0
            }
            SpectrumAccessLevel::SeventhDensity => {
                // Transcends veil
                1.0
            }
        }
    }

    /// Attempt to access a specific part of the spectrum
    pub fn attempt_access(&self, spectrum_position: SpectrumPosition) -> AccessResult {
        let access_level = self.get_access_level_for_position(spectrum_position);

        if access_level <= self.access_level as u8 {
            AccessResult::Success {
                spectrum_position,
                access_quality: self.calculate_access_quality(spectrum_position),
            }
        } else {
            AccessResult::Limited {
                spectrum_position,
                limitation_reason: self.get_limitation_reason(spectrum_position),
                current_access: self.access_level as u8,
                required_access: access_level,
            }
        }
    }

    /// Get access level required for a specific spectrum position
    fn get_access_level_for_position(&self, position: SpectrumPosition) -> u8 {
        match position {
            SpectrumPosition::ExtremeSpaceDominance => 0,
            SpectrumPosition::SpaceDominance => 1,
            SpectrumPosition::VeilTransition => 2,
            SpectrumPosition::TimeDominance => 3,
            SpectrumPosition::ExtremeTimeDominance => 4,
        }
    }

    /// Calculate access quality for a specific spectrum position
    fn calculate_access_quality(&self, position: SpectrumPosition) -> AccessQuality {
        let base_quality = match self.access_level {
            SpectrumAccessLevel::ThirdDensity => 0.3,
            SpectrumAccessLevel::FourthDensity => 0.5,
            SpectrumAccessLevel::FifthDensity => 0.7,
            SpectrumAccessLevel::SixthDensity => 0.9,
            SpectrumAccessLevel::SeventhDensity => 1.0,
        };

        let position_modifier = match position {
            SpectrumPosition::ExtremeSpaceDominance => 1.0,
            SpectrumPosition::SpaceDominance => 0.9,
            SpectrumPosition::VeilTransition => 0.5,
            SpectrumPosition::TimeDominance => 0.7,
            SpectrumPosition::ExtremeTimeDominance => 0.3,
        };

        AccessQuality {
            clarity: base_quality * position_modifier,
            stability: base_quality * 0.9,
            coherence: base_quality * 0.8,
        }
    }

    /// Get limitation reason for a specific spectrum position
    fn get_limitation_reason(&self, position: SpectrumPosition) -> LimitationReason {
        match position {
            SpectrumPosition::TimeDominance | SpectrumPosition::ExtremeTimeDominance => {
                LimitationReason::VeilActive
            }
            SpectrumPosition::VeilTransition => LimitationReason::InsufficientEvolutionaryProgress,
            _ => LimitationReason::InsufficientConsciousnessLevel,
        }
    }

    /// Evolve spectrum access based on entity state
    pub fn evolve_access(&mut self, entity_state: &EntityState) -> AccessEvolutionResult {
        let previous_level = self.access_level;
        let new_level = self.calculate_next_access_level(entity_state);

        if new_level != previous_level {
            self.access_level = new_level;
            self.update_veil_state(new_level);
            self.update_access_capabilities(new_level);

            AccessEvolutionResult::Advanced {
                previous_level,
                new_level,
                access_improvement: self.calculate_access_improvement(previous_level, new_level),
            }
        } else {
            AccessEvolutionResult::Unchanged {
                current_level: new_level,
                progress_toward_next: self.calculate_progress_toward_next(entity_state),
            }
        }
    }

    /// Calculate next access level based on entity state
    fn calculate_next_access_level(&self, entity_state: &EntityState) -> SpectrumAccessLevel {
        let consciousness = entity_state.consciousness_level;

        if consciousness < 0.3 {
            SpectrumAccessLevel::ThirdDensity
        } else if consciousness < 0.6 {
            SpectrumAccessLevel::FourthDensity
        } else if consciousness < 0.8 {
            SpectrumAccessLevel::FifthDensity
        } else if consciousness < 0.95 {
            SpectrumAccessLevel::SixthDensity
        } else {
            SpectrumAccessLevel::SeventhDensity
        }
    }

    /// Update veil state based on access level
    fn update_veil_state(&mut self, level: SpectrumAccessLevel) {
        self.veil_state = match level {
            SpectrumAccessLevel::ThirdDensity => SpectrumAccessVeilState::FullyActive,
            SpectrumAccessLevel::FourthDensity => SpectrumAccessVeilState::Thinning,
            SpectrumAccessLevel::FifthDensity => SpectrumAccessVeilState::MostlyDissolved,
            SpectrumAccessLevel::SixthDensity => SpectrumAccessVeilState::CompletelyDissolved,
            SpectrumAccessLevel::SeventhDensity => SpectrumAccessVeilState::Transcended,
        };
    }

    /// Update access capabilities based on access level
    fn update_access_capabilities(&mut self, level: SpectrumAccessLevel) {
        self.access_capabilities = match level {
            SpectrumAccessLevel::ThirdDensity => AccessCapabilities::third_density(),
            SpectrumAccessLevel::FourthDensity => AccessCapabilities::fourth_density(),
            SpectrumAccessLevel::FifthDensity => AccessCapabilities::fifth_density(),
            SpectrumAccessLevel::SixthDensity => AccessCapabilities::sixth_density(),
            SpectrumAccessLevel::SeventhDensity => AccessCapabilities::seventh_density(),
        };
    }

    /// Calculate access improvement between levels
    fn calculate_access_improvement(
        &self,
        previous: SpectrumAccessLevel,
        new: SpectrumAccessLevel,
    ) -> AccessImprovement {
        let improvement = (new as u8 - previous as u8) as f64 / 4.0;

        AccessImprovement {
            oneness_access_increase: improvement * 0.5,
            veil_transparency_increase: improvement * 0.5,
            spectrum_access_increase: improvement * 0.6,
        }
    }

    /// Calculate progress toward next access level
    fn calculate_progress_toward_next(&self, entity_state: &EntityState) -> f64 {
        match self.access_level {
            SpectrumAccessLevel::ThirdDensity => entity_state.consciousness_level / 0.3,
            SpectrumAccessLevel::FourthDensity => (entity_state.consciousness_level - 0.3) / 0.3,
            SpectrumAccessLevel::FifthDensity => (entity_state.consciousness_level - 0.6) / 0.2,
            SpectrumAccessLevel::SixthDensity => (entity_state.consciousness_level - 0.8) / 0.15,
            SpectrumAccessLevel::SeventhDensity => (entity_state.consciousness_level - 0.95) / 0.05,
        }
    }

    /// Get access statistics
    pub fn get_access_statistics(&self, entity_state: &EntityState) -> AccessStatistics {
        let spectrum_access = self.calculate_access(entity_state);

        AccessStatistics {
            current_level: self.access_level,
            veil_state: self.veil_state,
            space_time_access: spectrum_access.space_time_access,
            oneness_access: spectrum_access.oneness_access,
            veil_transparency: spectrum_access.veil_transparency,
            access_capabilities: self.access_capabilities.clone(),
        }
    }
}

/// Veil state (Phase 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumAccessVeilState {
    FullyActive,         // 3rd Density - Veil fully active
    Thinning,            // 4th Density - Veil begins to thin
    MostlyDissolved,     // 5th Density - Veil mostly dissolved
    CompletelyDissolved, // 6th Density - Veil completely dissolved
    Transcended,         // 7th Density - Veil transcended
}

/// Access capabilities
#[derive(Debug, Clone)]
pub struct AccessCapabilities {
    /// Can access physical realm (Space/Time)
    pub can_access_physical: bool,

    /// Can access metaphysical realm (Time/Space)
    pub can_access_metaphysical: bool,

    /// Can access both realms simultaneously
    pub can_access_both_realms: bool,

    /// Can transcend the spectrum entirely
    pub can_transcend_spectrum: bool,

    /// Level of conscious awareness
    pub conscious_awareness: f64,

    /// Level of unity consciousness
    pub unity_consciousness: f64,
}

impl AccessCapabilities {
    fn third_density() -> Self {
        AccessCapabilities {
            can_access_physical: true,
            can_access_metaphysical: false,
            can_access_both_realms: false,
            can_transcend_spectrum: false,
            conscious_awareness: 0.3,
            unity_consciousness: 0.05,
        }
    }

    fn fourth_density() -> Self {
        AccessCapabilities {
            can_access_physical: true,
            can_access_metaphysical: true,
            can_access_both_realms: false,
            can_transcend_spectrum: false,
            conscious_awareness: 0.6,
            unity_consciousness: 0.3,
        }
    }

    fn fifth_density() -> Self {
        AccessCapabilities {
            can_access_physical: true,
            can_access_metaphysical: true,
            can_access_both_realms: true,
            can_transcend_spectrum: false,
            conscious_awareness: 0.8,
            unity_consciousness: 0.6,
        }
    }

    fn sixth_density() -> Self {
        AccessCapabilities {
            can_access_physical: true,
            can_access_metaphysical: true,
            can_access_both_realms: true,
            can_transcend_spectrum: false,
            conscious_awareness: 0.95,
            unity_consciousness: 0.9,
        }
    }

    fn seventh_density() -> Self {
        AccessCapabilities {
            can_access_physical: false,
            can_access_metaphysical: false,
            can_access_both_realms: false,
            can_transcend_spectrum: true,
            conscious_awareness: 1.0,
            unity_consciousness: 1.0,
        }
    }
}

/// Access trajectory
#[derive(Debug, Clone)]
pub struct AccessTrajectory {
    /// Historical access levels
    pub access_history: Vec<SpectrumAccessLevel>,

    /// Progression through spectrum access
    pub progression_progress: f64,
}

impl AccessTrajectory {
    fn new() -> Self {
        AccessTrajectory {
            access_history: vec![SpectrumAccessLevel::ThirdDensity],
            progression_progress: 0.0,
        }
    }
}

/// Spectrum position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumPosition {
    ExtremeSpaceDominance, // v = s/t → ∞ (Many-ness dominant)
    SpaceDominance,        // v = s/t > 1 (Many-ness dominant)
    VeilTransition,        // v = 1 (Transition point)
    TimeDominance,         // v = t/s > 1 (Oneness dominant)
    ExtremeTimeDominance,  // v = t/s → ∞ (Oneness dominant)
}

/// Access result
#[derive(Debug, Clone, PartialEq)]
pub enum AccessResult {
    Success {
        spectrum_position: SpectrumPosition,
        access_quality: AccessQuality,
    },
    Limited {
        spectrum_position: SpectrumPosition,
        limitation_reason: LimitationReason,
        current_access: u8,
        required_access: u8,
    },
}

/// Access quality
#[derive(Debug, Clone, PartialEq)]
pub struct AccessQuality {
    pub clarity: f64,
    pub stability: f64,
    pub coherence: f64,
}

/// Limitation reason
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitationReason {
    VeilActive,
    InsufficientEvolutionaryProgress,
    InsufficientConsciousnessLevel,
}

/// Access evolution result
#[derive(Debug, Clone, PartialEq)]
pub enum AccessEvolutionResult {
    Advanced {
        previous_level: SpectrumAccessLevel,
        new_level: SpectrumAccessLevel,
        access_improvement: AccessImprovement,
    },
    Unchanged {
        current_level: SpectrumAccessLevel,
        progress_toward_next: f64,
    },
}

/// Access improvement
#[derive(Debug, Clone, PartialEq)]
pub struct AccessImprovement {
    pub oneness_access_increase: f64,
    pub veil_transparency_increase: f64,
    pub spectrum_access_increase: f64,
}

/// Access statistics
#[derive(Debug, Clone)]
pub struct AccessStatistics {
    pub current_level: SpectrumAccessLevel,
    pub veil_state: SpectrumAccessVeilState,
    pub space_time_access: f64,
    pub oneness_access: f64,
    pub veil_transparency: f64,
    pub access_capabilities: AccessCapabilities,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::{EntityState, PolarityState, VibrationalState};

    fn create_test_entity_state(consciousness_level: f64) -> EntityState {
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: crate::evolution_density_octave::density_octave::Density::First(
                    Density1SubLevel::Quantum,
                ),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level,
            experience_accumulation: 10.0,
            learning_progress: 5.0,
        }
    }

    #[test]
    fn test_spectrum_access_mechanism_creation() {
        let mechanism = SpectrumAccessMechanism::new();

        assert_eq!(mechanism.access_level, SpectrumAccessLevel::ThirdDensity);
        assert_eq!(mechanism.veil_state, SpectrumAccessVeilState::FullyActive);
    }

    #[test]
    fn test_calculate_access_third_density() {
        let mechanism = SpectrumAccessMechanism::new();
        let entity_state = create_test_entity_state(0.2);

        let spectrum_access = mechanism.calculate_access(&entity_state);

        assert_eq!(
            spectrum_access.evolutionary_level,
            SpectrumAccessLevel::ThirdDensity
        );
        assert!(spectrum_access.oneness_access <= 0.1);
        assert_eq!(spectrum_access.veil_transparency, 0.1);
        assert!(spectrum_access.space_time_access > 0.9);
    }

    #[test]
    fn test_calculate_access_fourth_density() {
        let mut mechanism = SpectrumAccessMechanism::new();
        mechanism.access_level = SpectrumAccessLevel::FourthDensity;
        mechanism.veil_state = SpectrumAccessVeilState::Thinning;

        let entity_state = create_test_entity_state(0.5);

        let spectrum_access = mechanism.calculate_access(&entity_state);

        assert_eq!(
            spectrum_access.evolutionary_level,
            SpectrumAccessLevel::FourthDensity
        );
        assert!(spectrum_access.oneness_access >= 0.3);
        assert!(spectrum_access.oneness_access <= 0.4);
        assert!(spectrum_access.veil_transparency >= 0.2);
        assert!(spectrum_access.space_time_access < 0.7);
    }

    #[test]
    fn test_calculate_access_sixth_density() {
        let mut mechanism = SpectrumAccessMechanism::new();
        mechanism.access_level = SpectrumAccessLevel::SixthDensity;
        mechanism.veil_state = SpectrumAccessVeilState::CompletelyDissolved;

        let entity_state = create_test_entity_state(0.95);

        let spectrum_access = mechanism.calculate_access(&entity_state);

        assert_eq!(
            spectrum_access.evolutionary_level,
            SpectrumAccessLevel::SixthDensity
        );
        assert_eq!(spectrum_access.oneness_access, 0.95);
        assert_eq!(spectrum_access.veil_transparency, 1.0);
        assert_eq!(spectrum_access.space_time_access, 0.05);
    }

    #[test]
    fn test_attempt_access_success() {
        let mechanism = SpectrumAccessMechanism::new();

        let result = mechanism.attempt_access(SpectrumPosition::SpaceDominance);

        assert!(matches!(result, AccessResult::Success { .. }));
    }

    #[test]
    fn test_attempt_access_limited() {
        let mechanism = SpectrumAccessMechanism::new();

        let result = mechanism.attempt_access(SpectrumPosition::TimeDominance);

        assert!(matches!(result, AccessResult::Limited { .. }));
        if let AccessResult::Limited {
            limitation_reason, ..
        } = result
        {
            assert_eq!(limitation_reason, LimitationReason::VeilActive);
        }
    }

    #[test]
    fn test_evolve_access_advancement() {
        let mut mechanism = SpectrumAccessMechanism::new();
        let entity_state = create_test_entity_state(0.4);

        let result = mechanism.evolve_access(&entity_state);

        assert!(matches!(result, AccessEvolutionResult::Advanced { .. }));
        if let AccessEvolutionResult::Advanced { new_level, .. } = result {
            assert_eq!(new_level, SpectrumAccessLevel::FourthDensity);
        }

        assert_eq!(mechanism.access_level, SpectrumAccessLevel::FourthDensity);
        assert_eq!(mechanism.veil_state, SpectrumAccessVeilState::Thinning);
    }

    #[test]
    fn test_evolve_access_unchanged() {
        let mut mechanism = SpectrumAccessMechanism::new();
        let entity_state = create_test_entity_state(0.2);

        let result = mechanism.evolve_access(&entity_state);

        assert!(matches!(result, AccessEvolutionResult::Unchanged { .. }));
        if let AccessEvolutionResult::Unchanged { current_level, .. } = result {
            assert_eq!(current_level, SpectrumAccessLevel::ThirdDensity);
        }

        assert_eq!(mechanism.access_level, SpectrumAccessLevel::ThirdDensity);
    }

    #[test]
    fn test_access_capabilities() {
        let third = AccessCapabilities::third_density();
        assert!(third.can_access_physical);
        assert!(!third.can_access_metaphysical);
        assert!(!third.can_access_both_realms);
        assert!(!third.can_transcend_spectrum);

        let fourth = AccessCapabilities::fourth_density();
        assert!(fourth.can_access_physical);
        assert!(fourth.can_access_metaphysical);
        assert!(!fourth.can_access_both_realms);
        assert!(!fourth.can_transcend_spectrum);

        let sixth = AccessCapabilities::sixth_density();
        assert!(sixth.can_access_physical);
        assert!(sixth.can_access_metaphysical);
        assert!(sixth.can_access_both_realms);
        assert!(!sixth.can_transcend_spectrum);

        let seventh = AccessCapabilities::seventh_density();
        assert!(!seventh.can_access_physical);
        assert!(!seventh.can_access_metaphysical);
        assert!(!seventh.can_access_both_realms);
        assert!(seventh.can_transcend_spectrum);
    }

    #[test]
    fn test_access_statistics() {
        let mechanism = SpectrumAccessMechanism::new();
        let entity_state = create_test_entity_state(0.2);

        let stats = mechanism.get_access_statistics(&entity_state);

        assert_eq!(stats.current_level, SpectrumAccessLevel::ThirdDensity);
        assert_eq!(stats.veil_state, SpectrumAccessVeilState::FullyActive);
        assert!(stats.space_time_access > 0.9);
        assert!(stats.oneness_access <= 0.1);
    }

    #[test]
    fn test_spectrum_position_access_levels() {
        let mechanism = SpectrumAccessMechanism::new();

        // These should be accessible at 3rd density
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::ExtremeSpaceDominance),
            AccessResult::Success { .. }
        ));
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::SpaceDominance),
            AccessResult::Success { .. }
        ));

        // These should be limited at 3rd density
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::TimeDominance),
            AccessResult::Limited { .. }
        ));
        assert!(matches!(
            mechanism.attempt_access(SpectrumPosition::ExtremeTimeDominance),
            AccessResult::Limited { .. }
        ));
    }
}
