//! Multi-Scale Camera System
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 5:
//! "All scales (quantum to cosmic, 7 scales, 52 orders of magnitude) are playable"
//! "Each scale contains the whole (holographic principle)"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each entity contains within it all densities and sub-densities of the octave"
//! "Any portion contains the whole" - the Law of One principle
//!
//! This module implements:
//! 1. 7 scale levels (Quantum → Cosmic)
//! 2. Smooth scale transitions (holographic continuity)
//! 3. Logarithmic depth rendering
//! 4. Holographic continuity across scales

use crate::types::Float;
use std::time::{Duration, Instant};

/// Scale levels for multi-scale simulation
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5.1:
/// "7 scales (quantum to cosmic, 52 orders of magnitude)"
///
/// Each scale represents a range of physical scales that the player can experience.
/// All scales are holographically complete - each contains the whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleLevel {
    /// Quantum scale: 10^-35 to 10^-15 meters
    /// Play as: particle, quantum field
    /// Physics mode: Quantum (v ≈ 1)
    /// Examples: Planck scale, subatomic particles, atoms
    Quantum,

    /// Cellular scale: 10^-12 to 10^-6 meters
    /// Play as: cell, DNA molecule
    /// Physics mode: Quantum
    /// Examples: prokaryotes, eukaryotes, DNA, RNA, proteins
    Cellular,

    /// Biological scale: 10^-5 to 10^0 meters
    /// Play as: organism, being
    /// Physics mode: Space/Time (v = s/t)
    /// Examples: plants, animals, humans, organisms
    Biological,

    /// Planetary scale: 10^0 to 10^7 meters
    /// Play as: civilization, collective
    /// Physics mode: Space/Time (v = s/t)
    /// Examples: planets, cities, nations, Gaia consciousness
    Planetary,

    /// Stellar scale: 10^8 to 10^13 meters
    /// Play as: solar system, Logos
    /// Physics mode: Space/Time (v = s/t)
    /// Examples: stars, solar systems, solar Logos
    Stellar,

    /// Galactic scale: 10^14 to 10^21 meters
    /// Play as: galaxy, galactic Logos
    /// Physics mode: Space/Time (v = s/t)
    /// Examples: galaxies, galactic Logos, spiral arms
    Galactic,

    /// Cosmic scale: 10^22 to 10^26 meters
    /// Play as: universe, intelligent infinity
    /// Physics mode: Time/Space (v = t/s)
    /// Examples: universe, multiverse, intelligent infinity
    Cosmic,
}

impl ScaleLevel {
    /// Get the minimum size in meters for this scale
    pub fn min_size(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1e-35,
            ScaleLevel::Cellular => 1e-12,
            ScaleLevel::Biological => 1e-5,
            ScaleLevel::Planetary => 1e0,
            ScaleLevel::Stellar => 1e8,
            ScaleLevel::Galactic => 1e14,
            ScaleLevel::Cosmic => 1e22,
        }
    }

    /// Get the maximum size in meters for this scale
    pub fn max_size(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1e-15,
            ScaleLevel::Cellular => 1e-6,
            ScaleLevel::Biological => 1e0,
            ScaleLevel::Planetary => 1e7,
            ScaleLevel::Stellar => 1e13,
            ScaleLevel::Galactic => 1e21,
            ScaleLevel::Cosmic => 1e26,
        }
    }

    /// Get the logarithmic center of this scale (log10)
    pub fn log_center(&self) -> Float {
        (self.min_size().log10() + self.max_size().log10()) / 2.0
    }

    /// Get the display name for this scale
    pub fn display_name(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum",
            ScaleLevel::Cellular => "Cellular",
            ScaleLevel::Biological => "Biological",
            ScaleLevel::Planetary => "Planetary",
            ScaleLevel::Stellar => "Stellar",
            ScaleLevel::Galactic => "Galactic",
            ScaleLevel::Cosmic => "Cosmic",
        }
    }

    /// Get the physics mode for this scale
    pub fn physics_mode(&self) -> PhysicsMode {
        match self {
            ScaleLevel::Quantum => PhysicsMode::Quantum,
            ScaleLevel::Cellular => PhysicsMode::Quantum,
            ScaleLevel::Biological => PhysicsMode::SpaceTime,
            ScaleLevel::Planetary => PhysicsMode::SpaceTime,
            ScaleLevel::Stellar => PhysicsMode::SpaceTime,
            ScaleLevel::Galactic => PhysicsMode::SpaceTime,
            ScaleLevel::Cosmic => PhysicsMode::TimeSpace,
        }
    }

    /// Get the index of this scale (0 = Quantum, 6 = Cosmic)
    pub fn index(&self) -> usize {
        match self {
            ScaleLevel::Quantum => 0,
            ScaleLevel::Cellular => 1,
            ScaleLevel::Biological => 2,
            ScaleLevel::Planetary => 3,
            ScaleLevel::Stellar => 4,
            ScaleLevel::Galactic => 5,
            ScaleLevel::Cosmic => 6,
        }
    }

    /// Create a ScaleLevel from an index (0 = Quantum, 6 = Cosmic)
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(ScaleLevel::Quantum),
            1 => Some(ScaleLevel::Cellular),
            2 => Some(ScaleLevel::Biological),
            3 => Some(ScaleLevel::Planetary),
            4 => Some(ScaleLevel::Stellar),
            5 => Some(ScaleLevel::Galactic),
            6 => Some(ScaleLevel::Cosmic),
            _ => None,
        }
    }

    /// Get the next scale (or None if at Cosmic)
    pub fn next(&self) -> Option<ScaleLevel> {
        Self::from_index(self.index() + 1)
    }

    /// Get the previous scale (or None if at Quantum)
    pub fn prev(&self) -> Option<ScaleLevel> {
        if self.index() > 0 {
            Self::from_index(self.index() - 1)
        } else {
            None
        }
    }
}

/// Physics mode based on spectrum ratio
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "v = s/t > 1: Space/Time physics (classical)"
/// "v = t/s > 1: Time/Space physics (metaphysical)"
/// "v ≈ 1: Quantum physics (transition at Veil)"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicsMode {
    /// Space/Time physics (v = s/t > 1)
    /// Classical physics, separation consciousness, many-ness dominant
    SpaceTime,

    /// Time/Space physics (v = t/s > 1)
    /// Metaphysical physics, unity consciousness, oneness dominant
    TimeSpace,

    /// Quantum physics (v ≈ 1)
    /// Quantum mechanics, transition at Veil
    Quantum,
}

impl PhysicsMode {
    /// Get the display name for this physics mode
    pub fn display_name(&self) -> &'static str {
        match self {
            PhysicsMode::SpaceTime => "Space/Time (Classical)",
            PhysicsMode::TimeSpace => "Time/Space (Metaphysical)",
            PhysicsMode::Quantum => "Quantum (Veil Transition)",
        }
    }
}

/// Transition between scale levels
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5.2:
/// "Smooth transition between scales"
/// "Maintain holographic continuity"
#[derive(Debug, Clone)]
pub struct ScaleTransition {
    /// Source scale
    from_scale: ScaleLevel,

    /// Target scale
    to_scale: ScaleLevel,

    /// Transition progress (0.0 = at from_scale, 1.0 = at to_scale)
    transition_progress: Float,

    /// Transition duration in seconds
    duration: Duration,

    /// Transition start time
    start_time: Instant,

    /// Is the transition complete?
    complete: bool,

    /// Interpolation mode
    interpolation: InterpolationMode,
}

impl ScaleTransition {
    /// Create a new scale transition
    pub fn new(from_scale: ScaleLevel, to_scale: ScaleLevel, duration: Duration) -> Self {
        ScaleTransition {
            from_scale,
            to_scale,
            transition_progress: 0.0,
            duration,
            start_time: Instant::now(),
            complete: false,
            interpolation: InterpolationMode::Smooth,
        }
    }

    /// Create a new scale transition with custom interpolation
    pub fn new_with_interpolation(
        from_scale: ScaleLevel,
        to_scale: ScaleLevel,
        duration: Duration,
        interpolation: InterpolationMode,
    ) -> Self {
        ScaleTransition {
            from_scale,
            to_scale,
            transition_progress: 0.0,
            duration,
            start_time: Instant::now(),
            complete: false,
            interpolation,
        }
    }

    /// Update the transition
    pub fn update(&mut self, delta: Duration) {
        if self.complete {
            return;
        }

        let elapsed = self.start_time.elapsed();
        let progress = elapsed.as_secs_f64() / self.duration.as_secs_f64();

        self.transition_progress = progress.clamp(0.0, 1.0);

        if self.transition_progress >= 1.0 {
            self.complete = true;
        }
    }

    /// Check if the transition is complete
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get the current interpolated scale
    pub fn current_scale(&self) -> InterpolatedScale {
        InterpolatedScale::new(
            self.from_scale,
            self.to_scale,
            self.interpolation.apply(self.transition_progress),
        )
    }

    /// Get the transition progress
    pub fn progress(&self) -> Float {
        self.transition_progress
    }

    /// Get the source scale
    pub fn from_scale(&self) -> ScaleLevel {
        self.from_scale
    }

    /// Get the target scale
    pub fn to_scale(&self) -> ScaleLevel {
        self.to_scale
    }
}

/// Interpolated scale during transition
///
/// Represents the intermediate state between two scales during a smooth transition.
#[derive(Debug, Clone)]
pub struct InterpolatedScale {
    /// Source scale
    from_scale: ScaleLevel,

    /// Target scale
    to_scale: ScaleLevel,

    /// Interpolation factor (0.0 = from_scale, 1.0 = to_scale)
    factor: Float,
}

impl InterpolatedScale {
    /// Create a new interpolated scale
    pub fn new(from_scale: ScaleLevel, to_scale: ScaleLevel, factor: Float) -> Self {
        InterpolatedScale {
            from_scale,
            to_scale,
            factor: factor.clamp(0.0, 1.0),
        }
    }

    /// Get the interpolated log position (log10 of meters)
    pub fn log_position(&self) -> Float {
        let from_log = self.from_scale.log_center();
        let to_log = self.to_scale.log_center();
        from_log + (to_log - from_log) * self.factor
    }

    /// Get the interpolated size in meters
    pub fn size(&self) -> Float {
        10.0_f64.powf(self.log_position())
    }

    /// Get the interpolated physics mode
    pub fn physics_mode(&self) -> PhysicsMode {
        if self.factor < 0.5 {
            self.from_scale.physics_mode()
        } else {
            self.to_scale.physics_mode()
        }
    }

    /// Get the interpolation factor
    pub fn factor(&self) -> Float {
        self.factor
    }
}

/// Interpolation mode for smooth transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpolationMode {
    /// Linear interpolation
    Linear,

    /// Smooth interpolation (easing in and out)
    Smooth,

    /// Exponential interpolation (faster at start, slower at end)
    EaseOut,

    /// Logarithmic interpolation (slower at start, faster at end)
    EaseIn,
}

impl InterpolationMode {
    /// Apply the interpolation to a progress value
    pub fn apply(&self, progress: Float) -> Float {
        match self {
            InterpolationMode::Linear => progress,

            InterpolationMode::Smooth => {
                // Smoothstep: 3x^2 - 2x^3
                let t = progress.clamp(0.0, 1.0);
                t * t * (3.0 - 2.0 * t)
            }

            InterpolationMode::EaseOut => {
                // Exponential ease out: 1 - (1 - x)^3
                let t = progress.clamp(0.0, 1.0);
                1.0 - (1.0 - t).powi(3)
            }

            InterpolationMode::EaseIn => {
                // Cubic ease in: x^3
                let t = progress.clamp(0.0, 1.0);
                t.powi(3)
            }
        }
    }
}

/// Multi-Scale Camera
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
/// "All scales are simulated simultaneously, each contains the whole"
/// "Holographic continuity: each scale contains the whole"
///
/// The multi-scale camera provides smooth transitions between all 7 scale levels
/// while maintaining holographic continuity.
///
/// Key principles:
/// 1. All scales are simulated simultaneously
/// 2. Resolution changes, but substance is the same
/// 3. Transitions are smooth, not discrete jumps
/// 4. Each scale contains the whole (holographic principle)
#[derive(Debug, Clone)]
pub struct MultiScaleCamera {
    /// Current scale level
    current_scale: ScaleLevel,

    /// Target scale level (for transitions)
    target_scale: ScaleLevel,

    /// Current position in the world (log10 of meters)
    position_log: Float,

    /// Current zoom level (log10 multiplier)
    zoom_log: Float,

    /// Rotation in radians
    rotation: Float,

    /// Active transition (if any)
    transition: Option<ScaleTransition>,

    /// Minimum zoom level (log10)
    min_zoom_log: Float,

    /// Maximum zoom level (log10)
    max_zoom_log: Float,

    /// Aspect ratio
    aspect_ratio: Float,

    /// Screen size (width, height)
    screen_size: (u32, u32),

    /// Holographic field reference (for maintaining continuity)
    holographic_continuity: HolographicContinuity,
}

/// Holographic continuity reference
///
/// Ensures that transitions maintain the holographic principle:
/// "Each part contains the whole"
#[derive(Debug, Clone)]
pub struct HolographicContinuity {
    /// The holographic field is the same at all scales
    /// Resolution changes, but completeness is maintained
    field_resolution_scale: Float,

    /// Continuity strength (0.0 = no continuity, 1.0 = perfect continuity)
    continuity_strength: Float,
}

impl Default for HolographicContinuity {
    fn default() -> Self {
        HolographicContinuity {
            field_resolution_scale: 1.0,
            continuity_strength: 1.0,
        }
    }
}

impl MultiScaleCamera {
    /// Create a new multi-scale camera
    pub fn new(initial_scale: ScaleLevel, aspect_ratio: Float) -> Self {
        let initial_log = initial_scale.log_center();

        MultiScaleCamera {
            current_scale: initial_scale,
            target_scale: initial_scale,
            position_log: initial_log,
            zoom_log: 0.0,
            rotation: 0.0,
            transition: None,
            min_zoom_log: -3.0,
            max_zoom_log: 3.0,
            aspect_ratio,
            screen_size: (1280, 720),
            holographic_continuity: HolographicContinuity::default(),
        }
    }

    /// Update the camera (handle transitions)
    pub fn update(&mut self, delta: Duration) {
        if let Some(ref mut transition) = self.transition {
            transition.update(delta);

            if transition.is_complete() {
                self.current_scale = transition.to_scale();
                self.position_log = self.current_scale.log_center();
                self.transition = None;
            } else {
                let current = transition.current_scale();
                self.position_log = current.log_position();
            }
        }
    }

    /// Transition to a new scale level
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 5.2:
    /// "Smooth transition between scales"
    /// "Maintain holographic continuity"
    pub fn transition_to_scale(
        &mut self,
        target: ScaleLevel,
        duration: Duration,
        interpolation: InterpolationMode,
    ) {
        if self.current_scale == target {
            return;
        }

        self.target_scale = target;
        self.transition = Some(ScaleTransition::new_with_interpolation(
            self.current_scale,
            target,
            duration,
            interpolation,
        ));
    }

    /// Transition to the next scale (zoom out)
    pub fn zoom_out(&mut self, duration: Duration) {
        if let Some(next_scale) = self.current_scale.next() {
            self.transition_to_scale(next_scale, duration, InterpolationMode::Smooth);
        }
    }

    /// Transition to the previous scale (zoom in)
    pub fn zoom_in(&mut self, duration: Duration) {
        if let Some(prev_scale) = self.current_scale.prev() {
            self.transition_to_scale(prev_scale, duration, InterpolationMode::Smooth);
        }
    }

    /// Pan the camera (adjust position)
    pub fn pan(&mut self, delta_log: Float) {
        self.position_log += delta_log;
    }

    /// Rotate the camera
    pub fn rotate(&mut self, delta: Float) {
        self.rotation += delta;
    }

    /// Set the aspect ratio
    pub fn set_aspect_ratio(&mut self, aspect_ratio: Float) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Set the screen size
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.screen_size = (width, height);
        self.aspect_ratio = width as Float / height as Float;
    }

    /// Get the current scale level
    pub fn current_scale(&self) -> ScaleLevel {
        self.current_scale
    }

    /// Get the target scale level
    pub fn target_scale(&self) -> ScaleLevel {
        self.target_scale
    }

    /// Get the current position in meters
    pub fn position(&self) -> Float {
        10.0_f64.powf(self.position_log)
    }

    /// Get the current zoom level
    pub fn zoom(&self) -> Float {
        10.0_f64.powf(self.zoom_log)
    }

    /// Get the current rotation in radians
    pub fn rotation(&self) -> Float {
        self.rotation
    }

    /// Get the current physics mode
    pub fn physics_mode(&self) -> PhysicsMode {
        self.current_scale.physics_mode()
    }

    /// Check if a transition is in progress
    pub fn is_transitioning(&self) -> bool {
        self.transition.is_some()
    }

    /// Get the current transition (if any)
    pub fn transition(&self) -> Option<&ScaleTransition> {
        self.transition.as_ref()
    }

    /// Get transition progress (0.0 to 1.0)
    pub fn transition_progress(&self) -> Float {
        self.transition
            .as_ref()
            .map(|t| t.progress())
            .unwrap_or(0.0)
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_x: Float, screen_y: Float) -> (Float, Float) {
        let aspect = self.aspect_ratio;
        let zoom = self.zoom();

        let normalized_x = (screen_x / self.screen_size.0 as Float) * 2.0 - 1.0;
        let normalized_y = (screen_y / self.screen_size.1 as Float) * 2.0 - 1.0;

        let world_x = (normalized_x * aspect / zoom) + self.position();
        let world_y = (normalized_y / zoom) + self.position();

        (world_x, world_y)
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: Float, world_y: Float) -> (Float, Float) {
        let aspect = self.aspect_ratio;
        let zoom = self.zoom();
        let position = self.position();

        let normalized_x = (world_x - position) * zoom / aspect;
        let normalized_y = (world_y - position) * zoom;

        let screen_x = (normalized_x + 1.0) * 0.5 * self.screen_size.0 as Float;
        let screen_y = (normalized_y + 1.0) * 0.5 * self.screen_size.1 as Float;

        (screen_x, screen_y)
    }

    /// Get the view at the current scale
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
    /// "The same field, viewed at different resolutions"
    /// "Resolution changes, but completeness is maintained"
    pub fn get_view(&self) -> ScaleView {
        ScaleView {
            scale: self.current_scale,
            position: self.position(),
            zoom: self.zoom(),
            rotation: self.rotation,
            physics_mode: self.physics_mode(),
            holographic_continuity: self.holographic_continuity.clone(),
        }
    }

    /// Get the interpolated view during transition
    pub fn get_interpolated_view(&self) -> Option<InterpolatedScaleView> {
        if let Some(ref transition) = self.transition {
            let current = transition.current_scale();

            Some(InterpolatedScaleView {
                from_scale: transition.from_scale(),
                to_scale: transition.to_scale(),
                position: current.size(),
                zoom: self.zoom(),
                rotation: self.rotation,
                physics_mode: current.physics_mode(),
                interpolation_factor: current.factor(),
                holographic_continuity: self.holographic_continuity.clone(),
            })
        } else {
            None
        }
    }
}

/// View at a specific scale
///
/// Represents the holographic view at a specific scale level.
/// The same field, viewed at different resolutions.
#[derive(Debug, Clone)]
pub struct ScaleView {
    /// Scale level
    pub scale: ScaleLevel,

    /// Position in meters
    pub position: Float,

    /// Zoom level
    pub zoom: Float,

    /// Rotation in radians
    pub rotation: Float,

    /// Physics mode at this scale
    pub physics_mode: PhysicsMode,

    /// Holographic continuity reference
    pub holographic_continuity: HolographicContinuity,
}

/// Interpolated view during scale transition
///
/// Represents the holographic view during a smooth transition between scales.
#[derive(Debug, Clone)]
pub struct InterpolatedScaleView {
    /// Source scale
    pub from_scale: ScaleLevel,

    /// Target scale
    pub to_scale: ScaleLevel,

    /// Position in meters (interpolated)
    pub position: Float,

    /// Zoom level
    pub zoom: Float,

    /// Rotation in radians
    pub rotation: Float,

    /// Physics mode (interpolated)
    pub physics_mode: PhysicsMode,

    /// Interpolation factor (0.0 = from_scale, 1.0 = to_scale)
    pub interpolation_factor: Float,

    /// Holographic continuity reference
    pub holographic_continuity: HolographicContinuity,
}

impl Default for MultiScaleCamera {
    fn default() -> Self {
        MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_level_min_max_size() {
        assert_eq!(ScaleLevel::Quantum.min_size(), 1e-35);
        assert_eq!(ScaleLevel::Quantum.max_size(), 1e-15);
        assert_eq!(ScaleLevel::Cosmic.min_size(), 1e22);
        assert_eq!(ScaleLevel::Cosmic.max_size(), 1e26);
    }

    #[test]
    fn test_scale_level_log_center() {
        let quantum_log = ScaleLevel::Quantum.log_center();
        assert!(quantum_log < -15.0);
        assert!(quantum_log > -40.0);

        let cosmic_log = ScaleLevel::Cosmic.log_center();
        assert!(cosmic_log > 20.0);
        assert!(cosmic_log < 30.0);
    }

    #[test]
    fn test_scale_level_physics_mode() {
        assert_eq!(ScaleLevel::Quantum.physics_mode(), PhysicsMode::Quantum);
        assert_eq!(ScaleLevel::Cellular.physics_mode(), PhysicsMode::Quantum);
        assert_eq!(
            ScaleLevel::Biological.physics_mode(),
            PhysicsMode::SpaceTime
        );
        assert_eq!(ScaleLevel::Planetary.physics_mode(), PhysicsMode::SpaceTime);
        assert_eq!(ScaleLevel::Stellar.physics_mode(), PhysicsMode::SpaceTime);
        assert_eq!(ScaleLevel::Galactic.physics_mode(), PhysicsMode::SpaceTime);
        assert_eq!(ScaleLevel::Cosmic.physics_mode(), PhysicsMode::TimeSpace);
    }

    #[test]
    fn test_scale_level_navigation() {
        assert_eq!(ScaleLevel::Quantum.next(), Some(ScaleLevel::Cellular));
        assert_eq!(ScaleLevel::Quantum.prev(), None);
        assert_eq!(ScaleLevel::Cosmic.next(), None);
        assert_eq!(ScaleLevel::Cosmic.prev(), Some(ScaleLevel::Galactic));
    }

    #[test]
    fn test_scale_level_index() {
        assert_eq!(ScaleLevel::Quantum.index(), 0);
        assert_eq!(ScaleLevel::Cellular.index(), 1);
        assert_eq!(ScaleLevel::Biological.index(), 2);
        assert_eq!(ScaleLevel::Planetary.index(), 3);
        assert_eq!(ScaleLevel::Stellar.index(), 4);
        assert_eq!(ScaleLevel::Galactic.index(), 5);
        assert_eq!(ScaleLevel::Cosmic.index(), 6);
    }

    #[test]
    fn test_scale_level_from_index() {
        assert_eq!(ScaleLevel::from_index(0), Some(ScaleLevel::Quantum));
        assert_eq!(ScaleLevel::from_index(3), Some(ScaleLevel::Planetary));
        assert_eq!(ScaleLevel::from_index(6), Some(ScaleLevel::Cosmic));
        assert_eq!(ScaleLevel::from_index(7), None);
    }

    #[test]
    fn test_interpolation_mode() {
        assert_eq!(InterpolationMode::Linear.apply(0.0), 0.0);
        assert_eq!(InterpolationMode::Linear.apply(0.5), 0.5);
        assert_eq!(InterpolationMode::Linear.apply(1.0), 1.0);

        let smooth = InterpolationMode::Smooth.apply(0.5);
        assert!(smooth > 0.0);
        assert!(smooth < 1.0);
    }

    #[test]
    fn test_scale_transition_creation() {
        let transition = ScaleTransition::new(
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            Duration::from_millis(100),
        );

        assert_eq!(transition.from_scale(), ScaleLevel::Quantum);
        assert_eq!(transition.to_scale(), ScaleLevel::Cellular);
        assert_eq!(transition.progress(), 0.0);
        assert!(!transition.is_complete());
    }

    #[test]
    fn test_scale_transition_update() {
        let mut transition = ScaleTransition::new(
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            Duration::from_millis(10),
        );

        std::thread::sleep(Duration::from_millis(20));
        transition.update(Duration::from_millis(20));

        assert!(transition.progress() > 0.0);
    }

    #[test]
    fn test_interpolated_scale() {
        let interpolated = InterpolatedScale::new(ScaleLevel::Quantum, ScaleLevel::Cellular, 0.5);

        let from_log = ScaleLevel::Quantum.log_center();
        let to_log = ScaleLevel::Cellular.log_center();
        let expected_log = from_log + (to_log - from_log) * 0.5;

        assert!((interpolated.log_position() - expected_log).abs() < 1e-10);
        assert_eq!(interpolated.factor(), 0.5);
    }

    #[test]
    fn test_multiscale_camera_creation() {
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);

        assert_eq!(camera.current_scale(), ScaleLevel::Biological);
        assert_eq!(camera.target_scale(), ScaleLevel::Biological);
        assert!(!camera.is_transitioning());
        assert_eq!(camera.transition_progress(), 0.0);
    }

    #[test]
    fn test_multiscale_camera_default() {
        let camera = MultiScaleCamera::default();

        assert_eq!(camera.current_scale(), ScaleLevel::Biological);
    }

    #[test]
    fn test_multiscale_camera_transition() {
        let mut camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);

        camera.transition_to_scale(
            ScaleLevel::Planetary,
            Duration::from_millis(100),
            InterpolationMode::Smooth,
        );

        assert!(camera.is_transitioning());
        assert_eq!(camera.target_scale(), ScaleLevel::Planetary);
        assert!(camera.transition().is_some());
    }

    #[test]
    fn test_multiscale_camera_zoom_in_out() {
        let mut camera = MultiScaleCamera::new(ScaleLevel::Planetary, 16.0 / 9.0);

        camera.zoom_in(Duration::from_millis(100));
        assert!(camera.is_transitioning());
        assert_eq!(camera.target_scale(), ScaleLevel::Biological);

        camera.transition = None;
        camera.current_scale = ScaleLevel::Planetary;

        camera.zoom_out(Duration::from_millis(100));
        assert!(camera.is_transitioning());
        assert_eq!(camera.target_scale(), ScaleLevel::Stellar);
    }

    #[test]
    fn test_multiscale_camera_screen_world_conversion() {
        let mut camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        camera.set_screen_size(1280, 720);

        let (world_x, world_y) = camera.screen_to_world(640.0, 360.0);
        let (screen_x, screen_y) = camera.world_to_screen(world_x, world_y);

        assert!((screen_x - 640.0).abs() < 1.0);
        assert!((screen_y - 360.0).abs() < 1.0);
    }

    #[test]
    fn test_multiscale_camera_get_view() {
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let view = camera.get_view();

        assert_eq!(view.scale, ScaleLevel::Biological);
        assert_eq!(view.physics_mode, PhysicsMode::SpaceTime);
    }

    #[test]
    fn test_multiscale_camera_get_interpolated_view() {
        let mut camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);

        camera.transition_to_scale(
            ScaleLevel::Planetary,
            Duration::from_millis(100),
            InterpolationMode::Smooth,
        );

        let interpolated = camera.get_interpolated_view();
        assert!(interpolated.is_some());

        let view = interpolated.unwrap();
        assert_eq!(view.from_scale, ScaleLevel::Biological);
        assert_eq!(view.to_scale, ScaleLevel::Planetary);
    }
}
