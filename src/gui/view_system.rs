//! View System - Multi-Level View System for Holographic Architecture
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md: "Multi-level view system
//! allows users to drill down into specific aspects of the cosmological
//! architecture with different views for different purposes."
//!
//! This module provides 6 distinct views with smooth camera transitions:
//! 1. Overview - Bird's eye view of all entities
//! 2. Hierarchy - Visualizes composition hierarchy and relationships
//! 3. Realm - 7-layer involution architecture visualization
//! 4. Archetype - 22 archetypical mind visualization
//! 5. Spectrum - Space/Time vs Time/Space spectrum positioning
//! 6. Evolution - Evolution clock and karmic patterns

use nalgebra_glm::Vec3;
use std::collections::HashMap;
use std::time::Duration;

/// View types available in the holographic architecture visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewType {
    /// Level 1: Overview - Bird's eye view of all entities
    Overview,
    /// Level 2: Hierarchy - Visualizes composition hierarchy and relationships
    Hierarchy,
    /// Level 3: Realm - 7-layer involution architecture visualization
    Realm,
    /// Level 4: Archetype - 22 archetypical mind visualization
    Archetype,
    /// Level 5: Spectrum - Space/Time vs Time/Space spectrum positioning
    Spectrum,
    /// Level 6: Evolution - Evolution clock and karmic patterns
    Evolution,
}

impl ViewType {
    /// Get display name for the view
    pub fn name(&self) -> &'static str {
        match self {
            ViewType::Overview => "Overview",
            ViewType::Hierarchy => "Hierarchy",
            ViewType::Realm => "Realm",
            ViewType::Archetype => "Archetype",
            ViewType::Spectrum => "Spectrum",
            ViewType::Evolution => "Evolution",
        }
    }

    /// Get keyboard shortcut for this view
    pub fn shortcut(&self) -> u8 {
        match self {
            ViewType::Overview => 1,
            ViewType::Hierarchy => 2,
            ViewType::Realm => 3,
            ViewType::Archetype => 4,
            ViewType::Spectrum => 5,
            ViewType::Evolution => 6,
        }
    }

    /// Get description of this view
    pub fn description(&self) -> &'static str {
        match self {
            ViewType::Overview => "Bird's eye view of all entities",
            ViewType::Hierarchy => "Visualizes composition hierarchy and relationships",
            ViewType::Realm => "7-layer involution architecture visualization",
            ViewType::Archetype => "22 archetypical mind visualization",
            ViewType::Spectrum => "Space/Time vs Time/Space spectrum positioning",
            ViewType::Evolution => "Evolution clock and karmic patterns",
        }
    }
}

/// Camera target configuration for a specific view
#[derive(Debug, Clone)]
pub struct ViewConfig {
    /// Target camera position
    pub camera_target: Vec3,
    /// Target zoom level
    pub zoom_target: f32,
    /// Target rotation angle (radians)
    pub rotation_target: f32,
    /// Duration of transition to this view (seconds)
    pub transition_duration: f32,
}

impl ViewConfig {
    /// Create a new view configuration
    pub fn new(
        camera_target: Vec3,
        zoom_target: f32,
        rotation_target: f32,
        transition_duration: f32,
    ) -> Self {
        ViewConfig {
            camera_target,
            zoom_target,
            rotation_target,
            transition_duration,
        }
    }
}

/// View state for smooth transitions
#[derive(Debug, Clone)]
pub struct ViewState {
    /// Current active view
    pub current_view: ViewType,
    /// Target view (if transitioning)
    pub target_view: Option<ViewType>,
    /// Transition progress (0.0 to 1.0)
    pub transition_progress: f32,
    /// Transition start time
    pub transition_start: Option<Duration>,
    /// Camera state at transition start
    pub start_position: Vec3,
    pub start_zoom: f32,
    pub start_rotation: f32,
}

impl Default for ViewState {
    fn default() -> Self {
        ViewState {
            current_view: ViewType::Overview,
            target_view: None,
            transition_progress: 0.0,
            transition_start: None,
            start_position: Vec3::new(0.0, 0.0, 0.0),
            start_zoom: 1.0,
            start_rotation: 0.0,
        }
    }
}

/// Multi-Level View System
///
/// Provides smooth camera transitions between different visualization
/// perspectives of the holographic architecture.
pub struct ViewSystem {
    /// Current view state
    state: ViewState,
    /// Configuration for each view type
    view_configs: HashMap<ViewType, ViewConfig>,
}

impl ViewSystem {
    /// Create a new view system with default configurations
    pub fn new() -> Self {
        let mut view_configs = HashMap::new();

        // View 1: Overview - Bird's eye view
        view_configs.insert(
            ViewType::Overview,
            ViewConfig::new(
                Vec3::new(0.0, 0.0, 0.0), // Center position
                1.0,                      // Default zoom
                0.0,                      // No rotation
                1.0,                      // 1 second transition
            ),
        );

        // View 2: Hierarchy - Isometric angle for relationships
        view_configs.insert(
            ViewType::Hierarchy,
            ViewConfig::new(
                Vec3::new(10.0, 10.0, 0.0),  // Offset position
                1.5,                         // Medium zoom
                std::f32::consts::FRAC_PI_4, // 45° rotation
                1.5,                         // 1.5 second transition
            ),
        );

        // View 3: Realm - Top-down, close zoom to see realm rings
        view_configs.insert(
            ViewType::Realm,
            ViewConfig::new(
                Vec3::new(0.0, 0.0, 0.0), // Center
                2.5,                      // Close zoom
                0.0,                      // Top-down (no rotation)
                1.2,                      // 1.2 second transition
            ),
        );

        // View 4: Archetype - Top-down, very close zoom for archetype wheel
        view_configs.insert(
            ViewType::Archetype,
            ViewConfig::new(
                Vec3::new(0.0, 0.0, 0.0), // Center
                3.5,                      // Very close zoom
                0.0,                      // Top-down
                1.2,                      // 1.2 second transition
            ),
        );

        // View 5: Spectrum - Side view for spectrum visualization
        view_configs.insert(
            ViewType::Spectrum,
            ViewConfig::new(
                Vec3::new(0.0, -20.0, 0.0),  // Offset to show spectrum axis
                1.2,                         // Medium-close zoom
                std::f32::consts::FRAC_PI_2, // 90° rotation for side view
                1.5,                         // 1.5 second transition
            ),
        );

        // View 6: Evolution - Timeline perspective
        view_configs.insert(
            ViewType::Evolution,
            ViewConfig::new(
                Vec3::new(-30.0, 0.0, 0.0),  // Offset to show timeline
                1.3,                         // Medium zoom
                std::f32::consts::FRAC_PI_6, // 30° rotation
                1.5,                         // 1.5 second transition
            ),
        );

        ViewSystem {
            state: ViewState::default(),
            view_configs,
        }
    }

    /// Get the current view type
    pub fn current_view(&self) -> ViewType {
        self.state.current_view
    }

    /// Check if currently transitioning between views
    pub fn is_transitioning(&self) -> bool {
        self.state.target_view.is_some()
    }

    /// Get transition progress (0.0 to 1.0)
    pub fn transition_progress(&self) -> f32 {
        self.state.transition_progress
    }

    /// Switch to a specific view
    pub fn switch_view(
        &mut self,
        view_type: ViewType,
        current_position: Vec3,
        current_zoom: f32,
        current_rotation: f32,
    ) {
        if view_type == self.state.current_view && !self.is_transitioning() {
            return; // Already at this view
        }

        // Store current camera state as transition start
        self.state.start_position = current_position;
        self.state.start_zoom = current_zoom;
        self.state.start_rotation = current_rotation;

        // Set target view
        self.state.target_view = Some(view_type);
        self.state.transition_progress = 0.0;
        self.state.transition_start = None; // Will be set on first update
    }

    /// Update transition state, returns (interpolated_position, interpolated_zoom, interpolated_rotation)
    pub fn update_transition(
        &mut self,
        delta_time: Duration,
        current_time: Duration,
    ) -> Option<(Vec3, f32, f32)> {
        if let Some(target_view) = self.state.target_view {
            // Initialize transition start time on first update
            if self.state.transition_start.is_none() {
                self.state.transition_start = Some(current_time);
            }

            let start_time = self.state.transition_start.unwrap();
            let config = self.view_configs.get(&target_view)?;
            let elapsed = current_time.saturating_sub(start_time);
            let duration = Duration::from_secs_f32(config.transition_duration);

            // Calculate progress with ease-in-out smoothing
            let raw_progress = (elapsed.as_secs_f32() / config.transition_duration).min(1.0);
            self.state.transition_progress = ease_in_out(raw_progress);

            // Interpolate camera state
            let position = lerp_vec3(
                &self.state.start_position,
                &config.camera_target,
                self.state.transition_progress,
            );
            let zoom = lerp(
                self.state.start_zoom,
                config.zoom_target,
                self.state.transition_progress,
            );
            let rotation = lerp_angle(
                self.state.start_rotation,
                config.rotation_target,
                self.state.transition_progress,
            );

            // Check if transition is complete
            if self.state.transition_progress >= 1.0 {
                self.state.current_view = target_view;
                self.state.target_view = None;
                self.state.transition_progress = 0.0;
                self.state.transition_start = None;
            }

            Some((position, zoom, rotation))
        } else {
            None
        }
    }

    /// Get configuration for a specific view
    pub fn get_config(&self, view_type: ViewType) -> Option<&ViewConfig> {
        self.view_configs.get(&view_type)
    }

    /// Get the start position for current transition
    pub fn transition_start_position(&self) -> Vec3 {
        self.state.start_position
    }

    /// Get the start zoom for current transition
    pub fn transition_start_zoom(&self) -> f32 {
        self.state.start_zoom
    }

    /// Get the start rotation for current transition
    pub fn transition_start_rotation(&self) -> f32 {
        self.state.start_rotation
    }
}

impl Default for ViewSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear interpolation between two values
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linear interpolation between two Vec3 values
fn lerp_vec3(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    Vec3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t))
}

/// Linear interpolation between two angles (handles wrapping)
fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff = b - a;
    let normalized_diff =
        ((diff + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)) - std::f32::consts::PI;
    a + normalized_diff * t
}

/// Ease-in-out easing function for smooth transitions
fn ease_in_out<T: Into<f32>>(t: T) -> f32 {
    let t_val = t.into();
    if t_val < 0.5 {
        2.0 * t_val * t_val
    } else {
        -1.0 + (4.0 - 2.0 * t_val) * t_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_type_names() {
        assert_eq!(ViewType::Overview.name(), "Overview");
        assert_eq!(ViewType::Hierarchy.name(), "Hierarchy");
        assert_eq!(ViewType::Realm.name(), "Realm");
        assert_eq!(ViewType::Archetype.name(), "Archetype");
        assert_eq!(ViewType::Spectrum.name(), "Spectrum");
        assert_eq!(ViewType::Evolution.name(), "Evolution");
    }

    #[test]
    fn test_view_type_shortcuts() {
        assert_eq!(ViewType::Overview.shortcut(), 1);
        assert_eq!(ViewType::Hierarchy.shortcut(), 2);
        assert_eq!(ViewType::Realm.shortcut(), 3);
        assert_eq!(ViewType::Archetype.shortcut(), 4);
        assert_eq!(ViewType::Spectrum.shortcut(), 5);
        assert_eq!(ViewType::Evolution.shortcut(), 6);
    }

    #[test]
    fn test_view_system_creation() {
        let view_system = ViewSystem::new();
        assert_eq!(view_system.current_view(), ViewType::Overview);
        assert!(!view_system.is_transitioning());
    }

    #[test]
    fn test_view_switch() {
        let mut view_system = ViewSystem::new();
        view_system.switch_view(ViewType::Hierarchy, Vec3::new(0.0, 0.0, 0.0), 1.0, 0.0);
        assert!(view_system.is_transitioning());
        assert_eq!(view_system.transition_progress(), 0.0);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_lerp_vec3() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);
        let result = lerp_vec3(&a, &b, 0.5);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, 15.0);
    }

    #[test]
    fn test_ease_in_out() {
        assert_eq!(ease_in_out(0.0), 0.0);
        assert_eq!(ease_in_out(0.5), 0.5);
        assert_eq!(ease_in_out(1.0), 1.0);
    }
}
