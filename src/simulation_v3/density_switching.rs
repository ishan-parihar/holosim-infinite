//! Density Switching System
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 6:
//! "All densities (1st → 8th) have unique mechanics"
//! "Each density has completely different gameplay mechanics while maintaining holographic continuity"
//!
//! From MASTER_R&D_ROADMAP.md Phase 0, Week 7-8:
//! "Implement density state machine (1st → 8th)"
//! "Implement density transition mechanics (reincarnation)"
//! "Implement veil thickness variations by density"
//! "Implement spectrum access by density"
//!
//! This module implements:
//! 1. Density state machine for gameplay
//! 2. Smooth density transitions (reincarnation)
//! 3. Veil state management by density
//! 4. Spectrum access by density
//! 5. Holographic continuity across densities

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::evolution_density_octave::spectrum_access::SpectrumPosition;
use crate::types::Float;
use std::time::{Duration, Instant};

/// Density state for gameplay
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6:
/// "Each density has unique mechanics (not just visual differences)"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameplayDensity {
    /// 1st Density: Matter awareness
    /// No veil
    /// Core mechanics: Particle physics, atomic bonding
    /// Player goal: Stabilize matter, create elements
    First(Density1SubLevel),

    /// 2nd Density: Instinctual growth
    /// No veil
    /// Core mechanics: Biological evolution, food chains
    /// Player goal: Survive, grow, reproduce
    Second(Density2SubLevel),

    /// 3rd Density: Self-aware with veil
    /// Veil fully active
    /// Core mechanics: Polarity choice (STO/STS), catalyst
    /// Player goal: Polarize (choose path)
    Third,

    /// 4th Density: Love/understanding
    /// Veil thinning
    /// Core mechanics: Collective consciousness, telepathy
    /// Player goal: Integrate polarities
    Fourth,

    /// 5th Density: Wisdom/light
    /// Veil mostly dissolved
    /// Core mechanics: Teaching/learning, light bodies
    /// Player goal: Master wisdom
    Fifth,

    /// 6th Density: Unity/balance
    /// Veil completely dissolved
    /// Core mechanics: Social memory complexes
    /// Player goal: Balance all polarities
    Sixth,

    /// 7th Density: Gateway
    /// Transcends veil
    /// Core mechanics: Violet-ray activation
    /// Player goal: Prepare for next octave
    Seventh,

    /// 8th Density: Intelligent Infinity
    /// No veil
    /// Core mechanics: Return to source
    /// Player goal: Merge with infinity
    Eighth,
}

impl GameplayDensity {
    /// Get the density level (1-8)
    pub fn level(&self) -> u8 {
        match self {
            GameplayDensity::First(_) => 1,
            GameplayDensity::Second(_) => 2,
            GameplayDensity::Third => 3,
            GameplayDensity::Fourth => 4,
            GameplayDensity::Fifth => 5,
            GameplayDensity::Sixth => 6,
            GameplayDensity::Seventh => 7,
            GameplayDensity::Eighth => 8,
        }
    }

    /// Get the display name for this density
    pub fn display_name(&self) -> &'static str {
        match self {
            GameplayDensity::First(sub) => match sub {
                Density1SubLevel::Quantum => "1st Density - Quantum Realm",
                Density1SubLevel::Atomic => "1st Density - Atomic Realm",
                Density1SubLevel::Molecular => "1st Density - Molecular Realm",
                Density1SubLevel::Planetary => "1st Density - Planetary Realm",
            },
            GameplayDensity::Second(sub) => match sub {
                Density2SubLevel::Cellular => "2nd Density - Cellular Realm",
                Density2SubLevel::SimpleLife => "2nd Density - Simple Life Realm",
                Density2SubLevel::ComplexLife => "2nd Density - Complex Life Realm",
            },
            GameplayDensity::Third => "3rd Density - Self-Awareness",
            GameplayDensity::Fourth => "4th Density - Love/Understanding",
            GameplayDensity::Fifth => "5th Density - Wisdom/Light",
            GameplayDensity::Sixth => "6th Density - Unity/Balance",
            GameplayDensity::Seventh => "7th Density - Gateway",
            GameplayDensity::Eighth => "8th Density - Intelligent Infinity",
        }
    }

    /// Get the ray color for this density
    pub fn ray_color(&self) -> &'static str {
        match self {
            GameplayDensity::First(_) => "Red Ray",
            GameplayDensity::Second(_) => "Orange Ray",
            GameplayDensity::Third => "Yellow Ray",
            GameplayDensity::Fourth => "Green Ray",
            GameplayDensity::Fifth => "Blue Ray",
            GameplayDensity::Sixth => "Indigo Ray",
            GameplayDensity::Seventh => "Violet Ray",
            GameplayDensity::Eighth => "White Light",
        }
    }

    /// Get the primary focus for this density
    pub fn primary_focus(&self) -> &'static str {
        match self {
            GameplayDensity::First(_) => "Physical manifestation",
            GameplayDensity::Second(_) => "Biological growth",
            GameplayDensity::Third => "Self-awareness and choice",
            GameplayDensity::Fourth => "Understanding, love, compassion",
            GameplayDensity::Fifth => "Wisdom, light, teaching/learning",
            GameplayDensity::Sixth => "Unity, balance, harmony",
            GameplayDensity::Seventh => "Completion, gateway to next octave",
            GameplayDensity::Eighth => "Return to IntelligentInfinity",
        }
    }

    /// Get the consciousness level for this density
    pub fn consciousness_level(&self) -> Float {
        match self {
            GameplayDensity::First(_) => 0.05,
            GameplayDensity::Second(_) => 0.15,
            GameplayDensity::Third => 0.3,
            GameplayDensity::Fourth => 0.6,
            GameplayDensity::Fifth => 0.8,
            GameplayDensity::Sixth => 0.95,
            GameplayDensity::Seventh => 0.99,
            GameplayDensity::Eighth => 1.0,
        }
    }

    /// Convert to Density enum
    pub fn to_density(&self) -> Density {
        match self {
            GameplayDensity::First(sub) => Density::First(*sub),
            GameplayDensity::Second(sub) => Density::Second(*sub),
            GameplayDensity::Third => Density::Third,
            GameplayDensity::Fourth => Density::Fourth,
            GameplayDensity::Fifth => Density::Fifth,
            GameplayDensity::Sixth => Density::Sixth,
            GameplayDensity::Seventh => Density::Seventh,
            GameplayDensity::Eighth => Density::Eighth,
        }
    }

    /// Create from Density enum
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First(sub) => GameplayDensity::First(sub),
            Density::Second(sub) => GameplayDensity::Second(sub),
            Density::Third => GameplayDensity::Third,
            Density::Fourth => GameplayDensity::Fourth,
            Density::Fifth => GameplayDensity::Fifth,
            Density::Sixth => GameplayDensity::Sixth,
            Density::Seventh => GameplayDensity::Seventh,
            Density::Eighth => GameplayDensity::Eighth,
        }
    }

    /// Get the next density (for progression)
    pub fn next(&self) -> Option<GameplayDensity> {
        match self {
            GameplayDensity::First(_) => Some(GameplayDensity::Second(Density2SubLevel::Cellular)),
            GameplayDensity::Second(_) => Some(GameplayDensity::Third),
            GameplayDensity::Third => Some(GameplayDensity::Fourth),
            GameplayDensity::Fourth => Some(GameplayDensity::Fifth),
            GameplayDensity::Fifth => Some(GameplayDensity::Sixth),
            GameplayDensity::Sixth => Some(GameplayDensity::Seventh),
            GameplayDensity::Seventh => Some(GameplayDensity::Eighth),
            GameplayDensity::Eighth => None, // Already at highest density
        }
    }

    /// Get the previous density (for regression)
    pub fn previous(&self) -> Option<GameplayDensity> {
        match self {
            GameplayDensity::First(_) => None, // Already at lowest density
            GameplayDensity::Second(_) => Some(GameplayDensity::First(Density1SubLevel::Planetary)),
            GameplayDensity::Third => Some(GameplayDensity::Second(Density2SubLevel::ComplexLife)),
            GameplayDensity::Fourth => Some(GameplayDensity::Third),
            GameplayDensity::Fifth => Some(GameplayDensity::Fourth),
            GameplayDensity::Sixth => Some(GameplayDensity::Fifth),
            GameplayDensity::Seventh => Some(GameplayDensity::Sixth),
            GameplayDensity::Eighth => Some(GameplayDensity::Seventh),
        }
    }
}

/// Veil state for gameplay
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6:
/// "Veil thickness variations by density"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayVeilState {
    /// No veil - complete access to the spectrum
    NoVeil,

    /// Veil fully active - very limited spectrum access
    FullyActive,

    /// Veil thinning - partial spectrum access
    Thinning,

    /// Veil mostly dissolved - full spectrum access
    MostlyDissolved,

    /// Veil completely dissolved - complete spectrum access
    CompletelyDissolved,

    /// Transcends veil - beyond the spectrum
    Transcended,
}

impl GameplayVeilState {
    /// Get veil transparency (0.0 = opaque, 1.0 = fully transparent)
    pub fn transparency(&self) -> Float {
        match self {
            GameplayVeilState::NoVeil => 1.0,
            GameplayVeilState::FullyActive => 0.1,
            GameplayVeilState::Thinning => 0.4,
            GameplayVeilState::MostlyDissolved => 0.7,
            GameplayVeilState::CompletelyDissolved => 1.0,
            GameplayVeilState::Transcended => 1.0,
        }
    }

    /// Get veil thickness (0.0 = no veil, 1.0 = full veil)
    pub fn thickness(&self) -> Float {
        1.0 - self.transparency()
    }

    /// Check if veil is active
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            GameplayVeilState::FullyActive
                | GameplayVeilState::Thinning
                | GameplayVeilState::MostlyDissolved
        )
    }
}

/// Spectrum access for gameplay
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6:
/// "Spectrum access by density"
#[derive(Debug, Clone, PartialEq)]
pub struct GameplaySpectrumAccess {
    /// Space/time access (0.0 to 1.0)
    pub space_time_access: Float,

    /// Time/space access (0.0 to 1.0)
    pub time_space_access: Float,

    /// Oneness access (0.0 to 1.0)
    pub oneness_access: Float,

    /// Veil transparency (0.0 to 1.0)
    pub veil_transparency: Float,

    /// Can access physical realm
    pub can_access_physical: bool,

    /// Can access metaphysical realm
    pub can_access_metaphysical: bool,

    /// Can access both realms simultaneously
    pub can_access_both_realms: bool,

    /// Can transcend the spectrum entirely
    pub can_transcend_spectrum: bool,
}

impl GameplaySpectrumAccess {
    /// Create spectrum access for a given density
    pub fn from_density(density: GameplayDensity) -> Self {
        match density {
            GameplayDensity::First(_) => GameplaySpectrumAccess {
                space_time_access: 0.95,
                time_space_access: 0.05,
                oneness_access: 0.05,
                veil_transparency: 1.0, // No veil in 1st density
                can_access_physical: true,
                can_access_metaphysical: false,
                can_access_both_realms: false,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Second(_) => GameplaySpectrumAccess {
                space_time_access: 0.9,
                time_space_access: 0.1,
                oneness_access: 0.1,
                veil_transparency: 1.0, // No veil in 2nd density
                can_access_physical: true,
                can_access_metaphysical: false,
                can_access_both_realms: false,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Third => GameplaySpectrumAccess {
                space_time_access: 0.9,
                time_space_access: 0.1,
                oneness_access: 0.1,
                veil_transparency: 0.1, // Veil fully active
                can_access_physical: true,
                can_access_metaphysical: false,
                can_access_both_realms: false,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Fourth => GameplaySpectrumAccess {
                space_time_access: 0.6,
                time_space_access: 0.4,
                oneness_access: 0.4,
                veil_transparency: 0.4, // Veil thinning
                can_access_physical: true,
                can_access_metaphysical: true,
                can_access_both_realms: false,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Fifth => GameplaySpectrumAccess {
                space_time_access: 0.3,
                time_space_access: 0.7,
                oneness_access: 0.7,
                veil_transparency: 0.7, // Veil mostly dissolved
                can_access_physical: true,
                can_access_metaphysical: true,
                can_access_both_realms: true,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Sixth => GameplaySpectrumAccess {
                space_time_access: 0.05,
                time_space_access: 0.95,
                oneness_access: 0.95,
                veil_transparency: 1.0, // Veil completely dissolved
                can_access_physical: true,
                can_access_metaphysical: true,
                can_access_both_realms: true,
                can_transcend_spectrum: false,
            },
            GameplayDensity::Seventh => GameplaySpectrumAccess {
                space_time_access: 0.0,
                time_space_access: 1.0,
                oneness_access: 1.0,
                veil_transparency: 1.0, // Transcends veil
                can_access_physical: false,
                can_access_metaphysical: false,
                can_access_both_realms: false,
                can_transcend_spectrum: true,
            },
            GameplayDensity::Eighth => GameplaySpectrumAccess {
                space_time_access: 0.0,
                time_space_access: 1.0,
                oneness_access: 1.0,
                veil_transparency: 1.0, // No veil in 8th density
                can_access_physical: false,
                can_access_metaphysical: false,
                can_access_both_realms: false,
                can_transcend_spectrum: true,
            },
        }
    }

    /// Check if a spectrum position is accessible
    pub fn can_access_position(&self, position: SpectrumPosition) -> bool {
        match position {
            SpectrumPosition::ExtremeSpaceDominance => self.space_time_access >= 0.8,
            SpectrumPosition::SpaceDominance => self.space_time_access >= 0.5,
            SpectrumPosition::VeilTransition => {
                self.space_time_access >= 0.3 || self.time_space_access >= 0.3
            }
            SpectrumPosition::TimeDominance => self.time_space_access >= 0.5,
            SpectrumPosition::ExtremeTimeDominance => self.time_space_access >= 0.8,
        }
    }
}

/// Density transition for gameplay
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6:
/// "Density transition mechanics (reincarnation)"
#[derive(Debug, Clone)]
pub struct DensityTransition {
    /// From which density
    pub from_density: GameplayDensity,

    /// To which density
    pub to_density: GameplayDensity,

    /// Transition progress (0.0 to 1.0)
    pub progress: Float,

    /// Transition start time
    pub start_time: Instant,

    /// Transition duration
    pub duration: Duration,

    /// Holographic continuity preserved
    pub holographic_continuity: HolographicContinuity,

    /// Interpolation mode
    pub interpolation: TransitionInterpolationMode,
}

impl DensityTransition {
    /// Create a new density transition
    pub fn new(
        from_density: GameplayDensity,
        to_density: GameplayDensity,
        duration: Duration,
        interpolation: TransitionInterpolationMode,
    ) -> Self {
        DensityTransition {
            from_density,
            to_density,
            progress: 0.0,
            start_time: Instant::now(),
            duration,
            holographic_continuity: HolographicContinuity::new(),
            interpolation,
        }
    }

    /// Update the transition
    pub fn update(&mut self) -> TransitionUpdateResult {
        let elapsed = self.start_time.elapsed();
        let progress_fraction = (elapsed.as_secs_f64() / self.duration.as_secs_f64()).min(1.0);

        self.progress = match self.interpolation {
            TransitionInterpolationMode::Linear => progress_fraction,
            TransitionInterpolationMode::Smooth => Self::smooth_step(progress_fraction),
            TransitionInterpolationMode::EaseOut => Self::ease_out(progress_fraction),
            TransitionInterpolationMode::EaseIn => Self::ease_in(progress_fraction),
        };

        if self.progress >= 1.0 {
            TransitionUpdateResult::Complete
        } else {
            TransitionUpdateResult::InProgress(self.progress)
        }
    }

    /// Get the interpolated density state
    pub fn get_interpolated_state(&self) -> InterpolatedDensityState {
        InterpolatedDensityState {
            density: if self.progress >= 0.5 {
                self.to_density
            } else {
                self.from_density
            },
            veil_state: self.interpolate_veil_state(),
            spectrum_access: self.interpolate_spectrum_access(),
            transition_progress: self.progress,
        }
    }

    /// Interpolate veil state
    fn interpolate_veil_state(&self) -> GameplayVeilState {
        let from_veil = GameplayVeilState::from_density(self.from_density);
        let to_veil = GameplayVeilState::from_density(self.to_density);

        if self.progress < 0.5 {
            from_veil
        } else {
            to_veil
        }
    }

    /// Interpolate spectrum access
    fn interpolate_spectrum_access(&self) -> GameplaySpectrumAccess {
        let from_access = GameplaySpectrumAccess::from_density(self.from_density);
        let to_access = GameplaySpectrumAccess::from_density(self.to_density);

        GameplaySpectrumAccess {
            space_time_access: Self::lerp(
                from_access.space_time_access,
                to_access.space_time_access,
                self.progress,
            ),
            time_space_access: Self::lerp(
                from_access.time_space_access,
                to_access.time_space_access,
                self.progress,
            ),
            oneness_access: Self::lerp(
                from_access.oneness_access,
                to_access.oneness_access,
                self.progress,
            ),
            veil_transparency: Self::lerp(
                from_access.veil_transparency,
                to_access.veil_transparency,
                self.progress,
            ),
            can_access_physical: self.progress < 0.5 || to_access.can_access_physical,
            can_access_metaphysical: self.progress >= 0.5 && to_access.can_access_metaphysical,
            can_access_both_realms: self.progress >= 0.5 && to_access.can_access_both_realms,
            can_transcend_spectrum: self.progress >= 0.5 && to_access.can_transcend_spectrum,
        }
    }

    /// Linear interpolation
    fn lerp(a: Float, b: Float, t: Float) -> Float {
        a + (b - a) * t
    }

    /// Smooth step interpolation
    fn smooth_step(t: Float) -> Float {
        t * t * (3.0 - 2.0 * t)
    }

    /// Ease out interpolation
    fn ease_out(t: Float) -> Float {
        1.0 - (1.0 - t).powi(3)
    }

    /// Ease in interpolation
    fn ease_in(t: Float) -> Float {
        t.powi(3)
    }
}

/// Transition interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionInterpolationMode {
    /// Linear interpolation
    Linear,

    /// Smooth step interpolation
    Smooth,

    /// Ease out interpolation
    EaseOut,

    /// Ease in interpolation
    EaseIn,
}

/// Transition update result
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionUpdateResult {
    /// Transition is in progress
    InProgress(Float),

    /// Transition is complete
    Complete,
}

/// Holographic continuity
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6:
/// "Reincarnation maintains holographic continuity"
#[derive(Debug, Clone)]
pub struct HolographicContinuity {
    /// Memory preserved across densities
    pub memory_preserved: bool,

    /// Experience preserved across densities
    pub experience_preserved: bool,

    /// Holographic blueprint preserved
    pub blueprint_preserved: bool,

    /// Archetypical profile preserved
    pub archetype_preserved: bool,
}

impl Default for HolographicContinuity {
    fn default() -> Self {
        Self::new()
    }
}

impl HolographicContinuity {
    /// Create new holographic continuity
    pub fn new() -> Self {
        HolographicContinuity {
            memory_preserved: true,
            experience_preserved: true,
            blueprint_preserved: true,
            archetype_preserved: true,
        }
    }
}

/// Interpolated density state
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedDensityState {
    /// Current density (based on transition progress)
    pub density: GameplayDensity,

    /// Current veil state
    pub veil_state: GameplayVeilState,

    /// Current spectrum access
    pub spectrum_access: GameplaySpectrumAccess,

    /// Transition progress
    pub transition_progress: Float,
}

/// Density switching state machine
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 6.1:
/// "Density State Machine"
#[derive(Debug, Clone)]
pub struct DensityStateMachine {
    /// Current density
    pub current_density: GameplayDensity,

    /// Target density (for transition)
    pub target_density: Option<GameplayDensity>,

    /// Current transition
    pub transition: Option<DensityTransition>,

    /// Holographic continuity
    pub holographic_continuity: HolographicContinuity,

    /// Transition history
    pub transition_history: Vec<DensityTransition>,
}

impl DensityStateMachine {
    /// Create a new density state machine
    pub fn new(initial_density: GameplayDensity) -> Self {
        DensityStateMachine {
            current_density: initial_density,
            target_density: None,
            transition: None,
            holographic_continuity: HolographicContinuity::new(),
            transition_history: Vec::new(),
        }
    }

    /// Create a new density state machine starting at 1st density
    pub fn first_density() -> Self {
        DensityStateMachine::new(GameplayDensity::First(Density1SubLevel::Quantum))
    }

    /// Check if a transition is possible
    pub fn can_transition(&self, target: GameplayDensity) -> bool {
        // Cannot transition to current density
        if self.current_density == target {
            return false;
        }

        // Cannot transition during another transition
        if self.transition.is_some() {
            return false;
        }

        // Check if target density is valid
        // All transitions are valid in holographic architecture
        true
    }

    /// Start a density transition
    pub fn start_transition(
        &mut self,
        target: GameplayDensity,
        duration: Duration,
        interpolation: TransitionInterpolationMode,
    ) -> Result<(), DensityTransitionError> {
        if !self.can_transition(target) {
            return Err(DensityTransitionError::TransitionNotAllowed);
        }

        let transition =
            DensityTransition::new(self.current_density, target, duration, interpolation);
        self.target_density = Some(target);
        self.transition = Some(transition);

        Ok(())
    }

    /// Update the density state machine
    pub fn update(&mut self) -> DensityStateUpdateResult {
        if let Some(ref mut transition) = self.transition {
            match transition.update() {
                TransitionUpdateResult::InProgress(progress) => {
                    DensityStateUpdateResult::Transitioning {
                        from: self.current_density,
                        to: self.target_density.unwrap(),
                        progress,
                        interpolated_state: transition.get_interpolated_state(),
                    }
                }
                TransitionUpdateResult::Complete => {
                    // Transition complete
                    let completed = self.transition.take().unwrap();
                    self.current_density = self.target_density.unwrap();
                    self.target_density = None;
                    self.transition_history.push(completed);

                    DensityStateUpdateResult::TransitionComplete(self.current_density)
                }
            }
        } else {
            DensityStateUpdateResult::Stable(self.current_density)
        }
    }

    /// Get the current veil state
    pub fn get_veil_state(&self) -> GameplayVeilState {
        if let Some(ref transition) = self.transition {
            transition.interpolate_veil_state()
        } else {
            GameplayVeilState::from_density(self.current_density)
        }
    }

    /// Get the current spectrum access
    pub fn get_spectrum_access(&self) -> GameplaySpectrumAccess {
        if let Some(ref transition) = self.transition {
            transition.interpolate_spectrum_access()
        } else {
            GameplaySpectrumAccess::from_density(self.current_density)
        }
    }

    /// Get the interpolated density state
    pub fn get_interpolated_state(&self) -> InterpolatedDensityState {
        if let Some(ref transition) = self.transition {
            transition.get_interpolated_state()
        } else {
            InterpolatedDensityState {
                density: self.current_density,
                veil_state: GameplayVeilState::from_density(self.current_density),
                spectrum_access: GameplaySpectrumAccess::from_density(self.current_density),
                transition_progress: 0.0,
            }
        }
    }

    /// Transition to next density
    pub fn transition_to_next(&mut self, duration: Duration) -> Result<(), DensityTransitionError> {
        if let Some(next) = self.current_density.next() {
            self.start_transition(next, duration, TransitionInterpolationMode::Smooth)
        } else {
            Err(DensityTransitionError::AlreadyAtHighestDensity)
        }
    }

    /// Transition to previous density
    pub fn transition_to_previous(
        &mut self,
        duration: Duration,
    ) -> Result<(), DensityTransitionError> {
        if let Some(prev) = self.current_density.previous() {
            self.start_transition(prev, duration, TransitionInterpolationMode::Smooth)
        } else {
            Err(DensityTransitionError::AlreadyAtLowestDensity)
        }
    }
}

impl GameplayVeilState {
    /// Create veil state from density
    pub fn from_density(density: GameplayDensity) -> Self {
        match density {
            GameplayDensity::First(_) => GameplayVeilState::NoVeil,
            GameplayDensity::Second(_) => GameplayVeilState::NoVeil,
            GameplayDensity::Third => GameplayVeilState::FullyActive,
            GameplayDensity::Fourth => GameplayVeilState::Thinning,
            GameplayDensity::Fifth => GameplayVeilState::MostlyDissolved,
            GameplayDensity::Sixth => GameplayVeilState::CompletelyDissolved,
            GameplayDensity::Seventh => GameplayVeilState::Transcended,
            GameplayDensity::Eighth => GameplayVeilState::NoVeil,
        }
    }
}

/// Density state update result
#[derive(Debug, Clone, PartialEq)]
pub enum DensityStateUpdateResult {
    /// Stable state (no transition)
    Stable(GameplayDensity),

    /// Transitioning
    Transitioning {
        from: GameplayDensity,
        to: GameplayDensity,
        progress: Float,
        interpolated_state: InterpolatedDensityState,
    },

    /// Transition complete
    TransitionComplete(GameplayDensity),
}

/// Density transition error
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionError {
    /// Transition not allowed
    TransitionNotAllowed,

    /// Already at highest density
    AlreadyAtHighestDensity,

    /// Already at lowest density
    AlreadyAtLowestDensity,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_gameplay_density_level() {
        assert_eq!(GameplayDensity::First(Density1SubLevel::Quantum).level(), 1);
        assert_eq!(
            GameplayDensity::Second(Density2SubLevel::Cellular).level(),
            2
        );
        assert_eq!(GameplayDensity::Third.level(), 3);
        assert_eq!(GameplayDensity::Fourth.level(), 4);
        assert_eq!(GameplayDensity::Fifth.level(), 5);
        assert_eq!(GameplayDensity::Sixth.level(), 6);
        assert_eq!(GameplayDensity::Seventh.level(), 7);
        assert_eq!(GameplayDensity::Eighth.level(), 8);
    }

    #[test]
    fn test_gameplay_density_display_name() {
        assert_eq!(
            GameplayDensity::First(Density1SubLevel::Quantum).display_name(),
            "1st Density - Quantum Realm"
        );
        assert_eq!(
            GameplayDensity::Third.display_name(),
            "3rd Density - Self-Awareness"
        );
        assert_eq!(
            GameplayDensity::Eighth.display_name(),
            "8th Density - Intelligent Infinity"
        );
    }

    #[test]
    fn test_gameplay_density_ray_color() {
        assert_eq!(
            GameplayDensity::First(Density1SubLevel::Quantum).ray_color(),
            "Red Ray"
        );
        assert_eq!(GameplayDensity::Third.ray_color(), "Yellow Ray");
        assert_eq!(GameplayDensity::Sixth.ray_color(), "Indigo Ray");
        assert_eq!(GameplayDensity::Eighth.ray_color(), "White Light");
    }

    #[test]
    fn test_gameplay_density_consciousness_level() {
        assert_eq!(
            GameplayDensity::First(Density1SubLevel::Quantum).consciousness_level(),
            0.05
        );
        assert_eq!(
            GameplayDensity::Second(Density2SubLevel::Cellular).consciousness_level(),
            0.15
        );
        assert_eq!(GameplayDensity::Third.consciousness_level(), 0.3);
        assert_eq!(GameplayDensity::Fourth.consciousness_level(), 0.6);
        assert_eq!(GameplayDensity::Fifth.consciousness_level(), 0.8);
        assert_eq!(GameplayDensity::Sixth.consciousness_level(), 0.95);
        assert_eq!(GameplayDensity::Seventh.consciousness_level(), 0.99);
        assert_eq!(GameplayDensity::Eighth.consciousness_level(), 1.0);
    }

    #[test]
    fn test_gameplay_density_next() {
        assert_eq!(
            GameplayDensity::First(Density1SubLevel::Quantum).next(),
            Some(GameplayDensity::Second(Density2SubLevel::Cellular))
        );
        assert_eq!(GameplayDensity::Third.next(), Some(GameplayDensity::Fourth));
        assert_eq!(
            GameplayDensity::Seventh.next(),
            Some(GameplayDensity::Eighth)
        );
        assert_eq!(GameplayDensity::Eighth.next(), None);
    }

    #[test]
    fn test_gameplay_density_previous() {
        assert_eq!(
            GameplayDensity::First(Density1SubLevel::Quantum).previous(),
            None
        );
        assert_eq!(
            GameplayDensity::Third.previous(),
            Some(GameplayDensity::Second(Density2SubLevel::ComplexLife))
        );
        assert_eq!(
            GameplayDensity::Eighth.previous(),
            Some(GameplayDensity::Seventh)
        );
    }

    #[test]
    fn test_gameplay_density_to_from_density() {
        let density = Density::Third;
        let gameplay = GameplayDensity::from_density(density);
        assert_eq!(gameplay.to_density(), density);
    }

    #[test]
    fn test_gameplay_veil_state_transparency() {
        assert_eq!(GameplayVeilState::NoVeil.transparency(), 1.0);
        assert_eq!(GameplayVeilState::FullyActive.transparency(), 0.1);
        assert_eq!(GameplayVeilState::Thinning.transparency(), 0.4);
        assert_eq!(GameplayVeilState::MostlyDissolved.transparency(), 0.7);
        assert_eq!(GameplayVeilState::CompletelyDissolved.transparency(), 1.0);
        assert_eq!(GameplayVeilState::Transcended.transparency(), 1.0);
    }

    #[test]
    fn test_gameplay_veil_state_thickness() {
        assert_eq!(GameplayVeilState::NoVeil.thickness(), 0.0);
        assert_eq!(GameplayVeilState::FullyActive.thickness(), 0.9);
        assert_eq!(GameplayVeilState::CompletelyDissolved.thickness(), 0.0);
    }

    #[test]
    fn test_gameplay_veil_state_is_active() {
        assert!(!GameplayVeilState::NoVeil.is_active());
        assert!(GameplayVeilState::FullyActive.is_active());
        assert!(GameplayVeilState::Thinning.is_active());
        assert!(GameplayVeilState::MostlyDissolved.is_active());
        assert!(!GameplayVeilState::CompletelyDissolved.is_active());
        assert!(!GameplayVeilState::Transcended.is_active());
    }

    #[test]
    fn test_gameplay_spectrum_access_from_density() {
        let third_access = GameplaySpectrumAccess::from_density(GameplayDensity::Third);
        assert!(third_access.space_time_access > 0.8);
        assert!(third_access.oneness_access <= 0.1);
        assert_eq!(third_access.veil_transparency, 0.1);
        assert!(third_access.can_access_physical);
        assert!(!third_access.can_access_metaphysical);

        let fourth_access = GameplaySpectrumAccess::from_density(GameplayDensity::Fourth);
        assert!(fourth_access.space_time_access >= 0.5);
        assert!(fourth_access.oneness_access >= 0.3);
        assert_eq!(fourth_access.veil_transparency, 0.4);
        assert!(fourth_access.can_access_physical);
        assert!(fourth_access.can_access_metaphysical);

        let sixth_access = GameplaySpectrumAccess::from_density(GameplayDensity::Sixth);
        assert!(sixth_access.space_time_access <= 0.1);
        assert!(sixth_access.oneness_access >= 0.9);
        assert_eq!(sixth_access.veil_transparency, 1.0);
        assert!(sixth_access.can_access_physical);
        assert!(sixth_access.can_access_metaphysical);
        assert!(sixth_access.can_access_both_realms);

        let eighth_access = GameplaySpectrumAccess::from_density(GameplayDensity::Eighth);
        assert!(!eighth_access.can_access_physical);
        assert!(!eighth_access.can_access_metaphysical);
        assert!(eighth_access.can_transcend_spectrum);
    }

    #[test]
    fn test_gameplay_spectrum_access_can_access_position() {
        let third_access = GameplaySpectrumAccess::from_density(GameplayDensity::Third);
        assert!(third_access.can_access_position(SpectrumPosition::SpaceDominance));
        assert!(!third_access.can_access_position(SpectrumPosition::TimeDominance));

        let sixth_access = GameplaySpectrumAccess::from_density(GameplayDensity::Sixth);
        // 6th density has very low space_time_access (0.05), so cannot access SpaceDominance
        assert!(!sixth_access.can_access_position(SpectrumPosition::SpaceDominance));
        assert!(sixth_access.can_access_position(SpectrumPosition::TimeDominance));
        assert!(sixth_access.can_access_position(SpectrumPosition::ExtremeTimeDominance));
    }

    #[test]
    fn test_density_state_machine_creation() {
        let sm = DensityStateMachine::first_density();
        assert_eq!(
            sm.current_density,
            GameplayDensity::First(Density1SubLevel::Quantum)
        );
        assert!(sm.transition.is_none());
        assert_eq!(sm.transition_history.len(), 0);
    }

    #[test]
    fn test_density_state_machine_can_transition() {
        let sm = DensityStateMachine::first_density();

        assert!(!sm.can_transition(GameplayDensity::First(Density1SubLevel::Quantum)));
        assert!(sm.can_transition(GameplayDensity::Second(Density2SubLevel::Cellular)));
        assert!(sm.can_transition(GameplayDensity::Third));
    }

    #[test]
    fn test_density_state_machine_start_transition() {
        let mut sm = DensityStateMachine::first_density();
        let result = sm.start_transition(
            GameplayDensity::Second(Density2SubLevel::Cellular),
            Duration::from_millis(100),
            TransitionInterpolationMode::Smooth,
        );

        assert!(result.is_ok());
        assert!(sm.target_density.is_some());
        assert!(sm.transition.is_some());
    }

    #[test]
    fn test_density_state_machine_update_stable() {
        let mut sm = DensityStateMachine::first_density();
        match sm.update() {
            DensityStateUpdateResult::Stable(density) => {
                assert_eq!(density, GameplayDensity::First(Density1SubLevel::Quantum));
            }
            _ => panic!("Expected Stable state"),
        }
    }

    #[test]
    fn test_density_state_machine_update_transitioning() {
        let mut sm = DensityStateMachine::first_density();
        sm.start_transition(
            GameplayDensity::Second(Density2SubLevel::Cellular),
            Duration::from_millis(100),
            TransitionInterpolationMode::Smooth,
        )
        .unwrap();

        match sm.update() {
            DensityStateUpdateResult::Transitioning {
                from, to, progress, ..
            } => {
                assert_eq!(from, GameplayDensity::First(Density1SubLevel::Quantum));
                assert_eq!(to, GameplayDensity::Second(Density2SubLevel::Cellular));
                assert!(progress >= 0.0);
                assert!(progress < 1.0);
            }
            _ => panic!("Expected Transitioning state"),
        }
    }

    #[test]
    fn test_density_state_machine_transition_to_next() {
        let mut sm = DensityStateMachine::first_density();
        let result = sm.transition_to_next(Duration::from_millis(100));

        assert!(result.is_ok());
        assert!(sm.transition.is_some());
    }

    #[test]
    fn test_density_state_machine_transition_to_next_error() {
        let mut sm = DensityStateMachine::new(GameplayDensity::Eighth);
        let result = sm.transition_to_next(Duration::from_millis(100));

        assert_eq!(result, Err(DensityTransitionError::AlreadyAtHighestDensity));
    }

    #[test]
    fn test_density_state_machine_transition_to_previous() {
        let mut sm = DensityStateMachine::new(GameplayDensity::Third);
        let result = sm.transition_to_previous(Duration::from_millis(100));

        assert!(result.is_ok());
        assert!(sm.transition.is_some());
    }

    #[test]
    fn test_density_state_machine_transition_to_previous_error() {
        let mut sm = DensityStateMachine::first_density();
        let result = sm.transition_to_previous(Duration::from_millis(100));

        assert_eq!(result, Err(DensityTransitionError::AlreadyAtLowestDensity));
    }

    #[test]
    fn test_density_state_machine_get_veil_state() {
        let sm = DensityStateMachine::new(GameplayDensity::Third);
        let veil_state = sm.get_veil_state();

        assert_eq!(veil_state, GameplayVeilState::FullyActive);
    }

    #[test]
    fn test_density_state_machine_get_spectrum_access() {
        let sm = DensityStateMachine::new(GameplayDensity::Third);
        let spectrum_access = sm.get_spectrum_access();

        assert!(spectrum_access.can_access_physical);
        assert!(!spectrum_access.can_access_metaphysical);
    }

    #[test]
    fn test_density_state_machine_get_interpolated_state() {
        let sm = DensityStateMachine::new(GameplayDensity::Third);
        let state = sm.get_interpolated_state();

        assert_eq!(state.density, GameplayDensity::Third);
        assert_eq!(state.transition_progress, 0.0);
    }

    #[test]
    fn test_holographic_continuity_creation() {
        let continuity = HolographicContinuity::new();

        assert!(continuity.memory_preserved);
        assert!(continuity.experience_preserved);
        assert!(continuity.blueprint_preserved);
        assert!(continuity.archetype_preserved);
    }

    #[test]
    fn test_density_transition_creation() {
        let transition = DensityTransition::new(
            GameplayDensity::Third,
            GameplayDensity::Fourth,
            Duration::from_millis(100),
            TransitionInterpolationMode::Smooth,
        );

        assert_eq!(transition.from_density, GameplayDensity::Third);
        assert_eq!(transition.to_density, GameplayDensity::Fourth);
        assert_eq!(transition.progress, 0.0);
    }

    #[test]
    fn test_density_transition_update() {
        let mut transition = DensityTransition::new(
            GameplayDensity::Third,
            GameplayDensity::Fourth,
            Duration::from_millis(10),
            TransitionInterpolationMode::Linear,
        );

        match transition.update() {
            TransitionUpdateResult::InProgress(progress) => {
                assert!(progress >= 0.0);
                assert!(progress < 1.0);
            }
            _ => panic!("Expected InProgress"),
        }
    }

    #[test]
    fn test_density_transition_complete() {
        let mut transition = DensityTransition::new(
            GameplayDensity::Third,
            GameplayDensity::Fourth,
            Duration::from_millis(1),
            TransitionInterpolationMode::Linear,
        );

        // Force progress to 1.0
        transition.progress = 1.0;
        // Also set start_time to make elapsed > duration
        transition.start_time = Instant::now() - Duration::from_millis(2);

        match transition.update() {
            TransitionUpdateResult::Complete => {
                // Success
            }
            _ => panic!("Expected Complete"),
        }
    }

    #[test]
    fn test_density_transition_interpolated_state() {
        let transition = DensityTransition::new(
            GameplayDensity::Third,
            GameplayDensity::Fourth,
            Duration::from_millis(100),
            TransitionInterpolationMode::Smooth,
        );

        let state = transition.get_interpolated_state();

        assert_eq!(state.density, GameplayDensity::Third);
        assert_eq!(state.transition_progress, 0.0);
    }

    #[test]
    fn test_density_transition_interpolation_modes() {
        let test_t = 0.5;

        let linear = DensityTransition::lerp(0.0, 1.0, test_t);
        assert_eq!(linear, 0.5);

        let smooth = DensityTransition::smooth_step(test_t);
        assert!((smooth - 0.5).abs() < 0.01);

        let ease_out = DensityTransition::ease_out(test_t);
        assert!((ease_out - 0.875).abs() < 0.01);

        let ease_in = DensityTransition::ease_in(test_t);
        assert!((ease_in - 0.125).abs() < 0.01);
    }
}
