//! UI Animations and Transitions
//!
//! Provides smooth animations for UI elements including panel transitions,
//! fade effects, and motion animations.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Smooth animations"

/// Easing function types for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum EasingFunction {
    /// Linear interpolation (no easing)
    Linear,
    /// Ease in (accelerate from zero)
    EaseIn,
    /// Ease out (decelerate to zero)
    EaseOut,
    /// Ease in-out (accelerate then decelerate)
    EaseInOut,
    /// Quadratic ease in
    QuadIn,
    /// Quadratic ease out
    QuadOut,
    /// Quadratic ease in-out
    QuadInOut,
    /// Cubic ease in
    CubicIn,
    /// Cubic ease out
    #[default]
    CubicOut,
    /// Cubic ease in-out
    CubicInOut,
    /// Elastic bounce effect
    Elastic,
    /// Bounce effect
    Bounce,
}


impl EasingFunction {
    /// Apply easing to a value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            EasingFunction::Linear => t,

            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }

            EasingFunction::QuadIn => t * t,
            EasingFunction::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::QuadInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }

            EasingFunction::CubicIn => t * t * t,
            EasingFunction::CubicOut => 1.0 - (1.0 - t).powi(3),
            EasingFunction::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }

            EasingFunction::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                    -(2.0_f32.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }

            EasingFunction::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;

                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

/// Animation state for a single animated value
#[derive(Debug, Clone)]
pub struct Animation {
    /// Current value
    pub current: f32,
    /// Target value
    pub target: f32,
    /// Start value
    pub start: f32,
    /// Duration in seconds
    pub duration: f32,
    /// Elapsed time
    pub elapsed: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Whether animation is complete
    pub complete: bool,
    /// Whether animation is paused
    pub paused: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            current: 0.0,
            target: 0.0,
            start: 0.0,
            duration: 0.3,
            elapsed: 0.0,
            easing: EasingFunction::default(),
            complete: true,
            paused: false,
        }
    }
}

impl Animation {
    /// Create a new animation
    pub fn new(target: f32, duration: f32) -> Self {
        Self {
            target,
            duration,
            ..Default::default()
        }
    }

    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Set initial value
    pub fn from(mut self, value: f32) -> Self {
        self.start = value;
        self.current = value;
        self.complete = false;
        self
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        if self.complete || self.paused {
            return;
        }

        self.elapsed += delta_time;

        if self.elapsed >= self.duration {
            self.current = self.target;
            self.complete = true;
        } else {
            let t = self.elapsed / self.duration;
            let eased_t = self.easing.apply(t);
            self.current = self.start + (self.target - self.start) * eased_t;
        }
    }

    /// Set new target
    pub fn set_target(&mut self, target: f32) {
        if self.target != target {
            self.start = self.current;
            self.target = target;
            self.elapsed = 0.0;
            self.complete = false;
        }
    }

    /// Skip to end
    pub fn complete(&mut self) {
        self.current = self.target;
        self.complete = true;
    }

    /// Reset animation
    pub fn reset(&mut self) {
        self.current = self.start;
        self.elapsed = 0.0;
        self.complete = false;
    }

    /// Pause animation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume animation
    pub fn resume(&mut self) {
        self.paused = false;
    }
}

/// Animation manager for coordinating multiple animations
#[derive(Debug, Default)]
pub struct AnimationManager {
    animations: std::collections::HashMap<String, Animation>,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an animation
    pub fn add(&mut self, id: impl Into<String>, animation: Animation) {
        self.animations.insert(id.into(), animation);
    }

    /// Get animation
    pub fn get(&self, id: &str) -> Option<&Animation> {
        self.animations.get(id)
    }

    /// Get mutable animation
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Animation> {
        self.animations.get_mut(id)
    }

    /// Update all animations
    pub fn update(&mut self, delta_time: f32) {
        for anim in self.animations.values_mut() {
            anim.update(delta_time);
        }
    }

    /// Set target for an animation
    pub fn set_target(&mut self, id: &str, target: f32) -> Result<(), String> {
        if let Some(anim) = self.animations.get_mut(id) {
            anim.set_target(target);
            Ok(())
        } else {
            Err(format!("Animation '{}' not found", id))
        }
    }

    /// Check if all animations are complete
    pub fn all_complete(&self) -> bool {
        self.animations.values().all(|a| a.complete)
    }

    /// Remove completed animations
    pub fn cleanup(&mut self) {
        self.animations.retain(|_, a| !a.complete);
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        self.animations.clear();
    }
}

/// Panel animation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelAnimation {
    /// Fade in/out
    Fade,
    /// Slide from left
    SlideFromLeft,
    /// Slide from right
    SlideFromRight,
    /// Slide from top
    SlideFromTop,
    /// Slide from bottom
    SlideFromBottom,
    /// Scale up/down
    Scale,
    /// Combined fade and slide
    FadeSlide,
}

impl PanelAnimation {
    /// Get default duration for this animation type
    pub fn default_duration(&self) -> f32 {
        match self {
            PanelAnimation::Fade => 0.2,
            PanelAnimation::SlideFromLeft => 0.3,
            PanelAnimation::SlideFromRight => 0.3,
            PanelAnimation::SlideFromTop => 0.3,
            PanelAnimation::SlideFromBottom => 0.3,
            PanelAnimation::Scale => 0.25,
            PanelAnimation::FadeSlide => 0.35,
        }
    }
}

/// Animation configuration for UI elements
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// Whether animations are enabled
    pub enabled: bool,
    /// Global animation speed multiplier
    pub speed_multiplier: f32,
    /// Default easing function
    pub default_easing: EasingFunction,
    /// Panel open animation
    pub panel_open: PanelAnimation,
    /// Panel close animation
    pub panel_close: PanelAnimation,
    /// Tooltip animation duration
    pub tooltip_duration: f32,
    /// Button hover duration
    pub button_hover_duration: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            speed_multiplier: 1.0,
            default_easing: EasingFunction::CubicOut,
            panel_open: PanelAnimation::FadeSlide,
            panel_close: PanelAnimation::Fade,
            tooltip_duration: 0.15,
            button_hover_duration: 0.1,
        }
    }
}

impl AnimationConfig {
    /// Create new animation config
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable all animations
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Enable animations
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Set speed multiplier
    pub fn set_speed(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier.clamp(0.1, 3.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_quad() {
        let easing = EasingFunction::QuadIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.25);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_cubic() {
        let easing = EasingFunction::CubicIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!((easing.apply(0.5) - 0.125).abs() < 0.001);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_bounds() {
        // All easings should return 0 at t=0 and 1 at t=1
        let easings = vec![
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
            EasingFunction::QuadIn,
            EasingFunction::QuadOut,
            EasingFunction::CubicIn,
            EasingFunction::CubicOut,
        ];

        for easing in easings {
            assert!(
                (easing.apply(0.0) - 0.0).abs() < 0.001,
                "{:?} at 0.0",
                easing
            );
            assert!(
                (easing.apply(1.0) - 1.0).abs() < 0.001,
                "{:?} at 1.0",
                easing
            );
        }
    }

    #[test]
    fn test_animation_lifecycle() {
        let mut anim = Animation::new(100.0, 1.0).from(0.0);

        assert!(!anim.complete);
        assert_eq!(anim.current, 0.0);

        // Update halfway
        anim.update(0.5);
        assert!(!anim.complete);
        assert!(anim.current > 0.0);
        assert!(anim.current < 100.0);

        // Complete
        anim.update(0.5);
        assert!(anim.complete);
        assert_eq!(anim.current, 100.0);
    }

    #[test]
    fn test_animation_target_change() {
        let mut anim = Animation::new(100.0, 1.0).from(0.0);
        anim.update(0.5); // Get to 50%

        let mid_value = anim.current;

        // Change target
        anim.set_target(200.0);
        assert!(!anim.complete);
        assert_eq!(anim.start, mid_value);
        assert_eq!(anim.target, 200.0);
    }

    #[test]
    fn test_animation_pause_resume() {
        let mut anim = Animation::new(100.0, 1.0).from(0.0);

        anim.update(0.3);
        let value = anim.current;

        anim.pause();
        anim.update(0.3); // Should not change
        assert_eq!(anim.current, value);

        anim.resume();
        anim.update(0.3); // Should change
        assert!(anim.current > value);
    }

    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new();

        manager.add("fade", Animation::new(1.0, 0.5).from(0.0));
        manager.add("slide", Animation::new(100.0, 0.5).from(0.0));

        assert!(manager.get("fade").is_some());
        assert!(manager.get("slide").is_some());

        // Update
        manager.update(0.3);

        // Check progress
        let fade = manager.get("fade").unwrap();
        assert!(!fade.complete);
        assert!(fade.current > 0.0);

        // Complete all
        manager.update(0.3);
        assert!(manager.all_complete());
    }

    #[test]
    fn test_animation_manager_cleanup() {
        let mut manager = AnimationManager::new();

        manager.add("anim1", Animation::new(1.0, 0.1).from(0.0));
        manager.add("anim2", Animation::new(1.0, 0.5).from(0.0));

        // Complete first animation
        manager.update(0.15);

        // Cleanup should remove completed
        manager.cleanup();

        assert!(manager.get("anim1").is_none());
        assert!(manager.get("anim2").is_some());
    }

    #[test]
    fn test_panel_animation_durations() {
        assert_eq!(PanelAnimation::Fade.default_duration(), 0.2);
        assert_eq!(PanelAnimation::SlideFromLeft.default_duration(), 0.3);
        assert_eq!(PanelAnimation::Scale.default_duration(), 0.25);
    }

    #[test]
    fn test_animation_config() {
        let mut config = AnimationConfig::new();
        assert!(config.enabled);

        config.disable();
        assert!(!config.enabled);

        config.set_speed(2.0);
        assert_eq!(config.speed_multiplier, 2.0);
    }
}
