// Scale transition system for smooth multi-scale navigation
// From GUI_IMPLEMENTATION_ROADMAP.md: "Scale transitions take 0.5-1.0 seconds"

use crate::gui::camera::Camera2D;
use nalgebra_glm::Vec3;
use std::time::{Duration, Instant};

/// Transition state for camera movements
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionState {
    Idle,
    InProgress {
        start_time: Instant,
        duration: Duration,
        start_zoom: f32,
        target_zoom: f32,
        start_position: Vec3,
        target_position: Vec3,
    },
}

/// Scale transition manager with logarithmic interpolation
pub struct ScaleTransition {
    state: TransitionState,
    transition_duration: Duration,
    active: bool,
}

impl ScaleTransition {
    /// Create a new scale transition manager
    ///
    /// # Arguments
    /// * `transition_duration` - Duration for scale transitions (default: 750ms)
    pub fn new(transition_duration: Duration) -> Self {
        Self {
            state: TransitionState::Idle,
            transition_duration,
            active: false,
        }
    }

    /// Create with default 750ms transition duration
    pub fn default_duration() -> Self {
        Self::new(Duration::from_millis(750))
    }

    /// Check if a transition is currently active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Start a transition to a target zoom level
    ///
    /// # Arguments
    /// * `target_zoom` - Target zoom level in logarithmic space
    /// * `current_camera` - Current camera state
    pub fn start_zoom_transition(&mut self, target_zoom: f32, current_camera: &Camera2D) {
        self.state = TransitionState::InProgress {
            start_time: Instant::now(),
            duration: self.transition_duration,
            start_zoom: current_camera.zoom,
            target_zoom,
            start_position: current_camera.position,
            target_position: current_camera.position, // Keep position same for zoom-only transition
        };
        self.active = true;
    }

    /// Start a transition to a target position
    ///
    /// # Arguments
    /// * `target_position` - Target position in logarithmic space
    /// * `current_camera` - Current camera state
    pub fn start_position_transition(&mut self, target_position: Vec3, current_camera: &Camera2D) {
        self.state = TransitionState::InProgress {
            start_time: Instant::now(),
            duration: self.transition_duration,
            start_zoom: current_camera.zoom,
            target_zoom: current_camera.zoom, // Keep zoom same for position-only transition
            start_position: current_camera.position,
            target_position,
        };
        self.active = true;
    }

    /// Start a full transition to target zoom and position
    ///
    /// # Arguments
    /// * `target_zoom` - Target zoom level in logarithmic space
    /// * `target_position` - Target position in logarithmic space
    /// * `current_camera` - Current camera state
    pub fn start_full_transition(
        &mut self,
        target_zoom: f32,
        target_position: Vec3,
        current_camera: &Camera2D,
    ) {
        self.state = TransitionState::InProgress {
            start_time: Instant::now(),
            duration: self.transition_duration,
            start_zoom: current_camera.zoom,
            target_zoom,
            start_position: current_camera.position,
            target_position,
        };
        self.active = true;
    }

    /// Update transition and apply to camera
    ///
    /// # Arguments
    /// * `camera` - Camera to update
    ///
    /// # Returns
    /// * `true` if transition is still in progress
    /// * `false` if transition is complete
    pub fn update(&mut self, camera: &mut Camera2D) -> bool {
        if !self.active {
            return false;
        }

        match &self.state {
            TransitionState::Idle => {
                self.active = false;
                false
            }
            TransitionState::InProgress {
                start_time,
                duration,
                start_zoom,
                target_zoom,
                start_position,
                target_position,
            } => {
                let elapsed = start_time.elapsed();
                let progress = (elapsed.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0);

                if progress >= 1.0 {
                    // Transition complete
                    camera.zoom = *target_zoom;
                    camera.position = *target_position;
                    self.state = TransitionState::Idle;
                    self.active = false;
                    false
                } else {
                    // Smooth easing function (ease-in-out cubic)
                    let eased_progress = self.ease_in_out_cubic(progress);

                    // Interpolate zoom in logarithmic space
                    camera.zoom = self.lerp_logarithmic(*start_zoom, *target_zoom, eased_progress);

                    // Interpolate position in logarithmic space
                    camera.position =
                        Vec3::lerp(start_position, target_position, eased_progress as f32);

                    true
                }
            }
        }
    }

    /// Cancel any ongoing transition
    pub fn cancel(&mut self) {
        self.state = TransitionState::Idle;
        self.active = false;
    }

    /// Ease-in-out cubic easing function for smooth transitions
    ///
    /// # Arguments
    /// * `t` - Normalized progress [0.0, 1.0]
    ///
    /// # Returns
    /// * Eased progress value [0.0, 1.0]
    fn ease_in_out_cubic(&self, t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    /// Interpolate between two zoom levels in logarithmic space
    ///
    /// # Arguments
    /// * `start` - Starting zoom level
    /// * `target` - Target zoom level
    /// * `t` - Normalized progress [0.0, 1.0]
    ///
    /// # Returns
    /// * Interpolated zoom level
    fn lerp_logarithmic(&self, start: f32, target: f32, t: f64) -> f32 {
        // Linear interpolation in logarithmic space
        let log_start = (start.abs()).ln_1p();
        let log_target = (target.abs()).ln_1p();
        let log_current = log_start + (log_target - log_start) * t as f32;

        // Convert back from logarithmic space
        log_current.exp_m1() * target.signum()
    }

    /// Get current transition progress [0.0, 1.0]
    ///
    /// # Returns
    /// * Progress value, or 0.0 if no transition is active
    pub fn get_progress(&self) -> f64 {
        match &self.state {
            TransitionState::Idle => 0.0,
            TransitionState::InProgress {
                start_time,
                duration,
                ..
            } => {
                let elapsed = start_time.elapsed();
                (elapsed.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0)
            }
        }
    }

    /// Set transition duration
    ///
    /// # Arguments
    /// * `duration` - New transition duration
    pub fn set_transition_duration(&mut self, duration: Duration) {
        self.transition_duration = duration;
    }

    /// Get transition duration
    pub fn get_transition_duration(&self) -> Duration {
        self.transition_duration
    }
}

impl Default for ScaleTransition {
    fn default() -> Self {
        Self::default_duration()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_creation() {
        let transition = ScaleTransition::default_duration();
        assert!(!transition.is_active());
        assert_eq!(transition.get_progress(), 0.0);
    }

    #[test]
    fn test_zoom_transition_start() {
        let mut transition = ScaleTransition::default_duration();
        let camera = Camera2D::new(1.0);

        transition.start_zoom_transition(2.0, &camera);
        assert!(transition.is_active());
    }

    #[test]
    fn test_transition_cancel() {
        let mut transition = ScaleTransition::default_duration();
        let camera = Camera2D::new(1.0);

        transition.start_zoom_transition(2.0, &camera);
        assert!(transition.is_active());

        transition.cancel();
        assert!(!transition.is_active());
    }

    #[test]
    fn test_ease_in_out_cubic() {
        let transition = ScaleTransition::default_duration();

        assert_eq!(transition.ease_in_out_cubic(0.0), 0.0);
        assert_eq!(transition.ease_in_out_cubic(1.0), 1.0);
        assert_eq!(transition.ease_in_out_cubic(0.5), 0.5);

        // Check monotonicity
        let p1 = transition.ease_in_out_cubic(0.25);
        let p2 = transition.ease_in_out_cubic(0.75);
        assert!(p1 < p2);
    }

    #[test]
    fn test_logarithmic_interpolation() {
        let transition = ScaleTransition::default_duration();

        // Test interpolation
        let result = transition.lerp_logarithmic(1.0, 10.0, 0.5);
        assert!(result > 1.0 && result < 10.0);

        // Test endpoints (using approximate comparison for floating point)
        let result0 = transition.lerp_logarithmic(1.0, 10.0, 0.0);
        let result1 = transition.lerp_logarithmic(1.0, 10.0, 1.0);
        assert!((result0 - 1.0).abs() < 1e-6);
        assert!((result1 - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_custom_duration() {
        let duration = Duration::from_millis(500);
        let transition = ScaleTransition::new(duration);

        assert_eq!(transition.get_transition_duration(), duration);
    }
}
