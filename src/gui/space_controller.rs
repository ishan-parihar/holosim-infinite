//! Space Controller - Spatial navigation and zoom controls
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 5:
//! "Space Controller:
//! - Spatial expansion rate control
//! - Zoom controls (10^-35 to 10^26 m)
//! - Focus-based spatial manipulation
//! - Multi-scale navigation"
//!
//! This module provides:
//! - Zoom level control (10^-35 to 10^26 m)
//! - Spatial expansion rate control
//! - Focus-based spatial manipulation
//! - Multi-scale navigation

use crate::gui::{Coordinate3D, GuiConfig, ScaleLevel};

/// Spatial focus - determines where the user is looking
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialFocus {
    /// Focus point in 3D space
    pub point: Coordinate3D,

    /// Focus radius in meters
    pub radius: f64,

    /// Enable focus-based manipulation
    pub enabled: bool,
}

impl Default for SpatialFocus {
    fn default() -> Self {
        SpatialFocus {
            point: Coordinate3D::origin(),
            radius: 1.0e6, // 1000 km default
            enabled: false,
        }
    }
}

impl SpatialFocus {
    /// Create a new spatial focus
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the focus point
    pub fn set_point(&mut self, point: Coordinate3D) {
        self.point = point;
    }

    /// Set the focus radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Enable or disable focus-based manipulation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if a point is within the focus radius
    pub fn contains_point(&self, point: &Coordinate3D) -> bool {
        self.point.distance_to(point) <= self.radius
    }

    /// Calculate the spatial dilation factor for a point
    pub fn calculate_dilation(&self, point: &Coordinate3D) -> f64 {
        if !self.enabled {
            return 1.0;
        }

        let distance = self.point.distance_to(point);
        if distance <= self.radius {
            // Closer to focus = higher dilation
            1.0 + (1.0 - distance / self.radius)
        } else {
            1.0
        }
    }
}

/// Navigation state
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationState {
    /// Current position
    pub position: Coordinate3D,

    /// Target position (for smooth transitions)
    pub target_position: Coordinate3D,

    /// Current velocity
    pub velocity: Coordinate3D,

    /// Navigation in progress
    pub is_navigating: bool,

    /// Navigation progress (0.0 to 1.0)
    pub navigation_progress: f64,
}

impl Default for NavigationState {
    fn default() -> Self {
        NavigationState {
            position: Coordinate3D::origin(),
            target_position: Coordinate3D::origin(),
            velocity: Coordinate3D::origin(),
            is_navigating: false,
            navigation_progress: 0.0,
        }
    }
}

impl NavigationState {
    /// Create a new navigation state
    pub fn new() -> Self {
        Self::default()
    }

    /// Navigate to a position
    pub fn navigate_to(&mut self, target: Coordinate3D) {
        self.target_position = target;
        self.is_navigating = true;
        self.navigation_progress = 0.0;
    }

    /// Update navigation
    pub fn update(&mut self, delta_time: f64) {
        if self.is_navigating {
            self.navigation_progress += delta_time * 2.0; // 0.5 second transition
            if self.navigation_progress >= 1.0 {
                self.navigation_progress = 1.0;
                self.is_navigating = false;
                self.position = self.target_position;
                self.velocity = Coordinate3D::origin();
            } else {
                // Smooth interpolation
                let t = self.navigation_progress;
                let smooth_t = t * t * (3.0 - 2.0 * t);
                let new_pos = Coordinate3D::new(
                    self.position.x + (self.target_position.x - self.position.x) * smooth_t,
                    self.position.y + (self.target_position.y - self.position.y) * smooth_t,
                    self.position.z + (self.target_position.z - self.position.z) * smooth_t,
                );

                // Calculate velocity
                self.velocity = Coordinate3D::new(
                    new_pos.x - self.position.x,
                    new_pos.y - self.position.y,
                    new_pos.z - self.position.z,
                );

                self.position = new_pos;
            }
        }
    }

    /// Get the current position
    pub fn current_position(&self) -> Coordinate3D {
        self.position
    }

    /// Check if navigating
    pub fn is_navigating(&self) -> bool {
        self.is_navigating
    }
}

/// Scale transition state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleTransition {
    /// From scale level
    pub from: ScaleLevel,

    /// To scale level
    pub to: ScaleLevel,

    /// Transition progress (0.0 to 1.0)
    pub progress: f64,

    /// Transition in progress
    pub is_transitioning: bool,
}

impl Default for ScaleTransition {
    fn default() -> Self {
        ScaleTransition {
            from: ScaleLevel::Cellular,
            to: ScaleLevel::Cellular,
            progress: 0.0,
            is_transitioning: false,
        }
    }
}

impl ScaleTransition {
    /// Create a new scale transition
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a transition
    pub fn start(&mut self, from: ScaleLevel, to: ScaleLevel) {
        self.from = from;
        self.to = to;
        self.progress = 0.0;
        self.is_transitioning = true;
    }

    /// Update the transition
    pub fn update(&mut self, delta_time: f64) {
        if self.is_transitioning {
            self.progress += delta_time * 2.0; // 0.5 second transition
            if self.progress >= 1.0 {
                self.progress = 1.0;
                self.is_transitioning = false;
            }
        }
    }

    /// Get the current scale level (interpolated)
    pub fn current_scale(&self) -> ScaleLevel {
        if !self.is_transitioning {
            return self.to;
        }

        // Interpolate based on progress
        if self.progress < 0.5 {
            self.from
        } else {
            self.to
        }
    }

    /// Get the current zoom level (interpolated in meters)
    pub fn current_zoom(&self) -> f64 {
        let from_zoom = self.from.scale_in_meters();
        let to_zoom = self.to.scale_in_meters();

        if !self.is_transitioning {
            return to_zoom;
        }

        // Smooth interpolation
        let t = self.progress;
        let smooth_t = t * t * (3.0 - 2.0 * t);
        from_zoom + (to_zoom - from_zoom) * smooth_t
    }

    /// Check if transitioning
    pub fn is_transitioning(&self) -> bool {
        self.is_transitioning
    }
}

/// Hierarchy navigation state for nested entity exploration
///
/// This enables drilling down into entities to explore their internal structure:
/// - Double-click an entity to "enter" it and see its children/composition
/// - Press Escape or Back to go up one level
/// - Maintains a navigation stack for full history
#[derive(Debug, Clone)]
pub struct HierarchyNavigationState {
    /// Navigation stack - each entry is an entity ID we've entered
    /// The last entry is the current focus entity
    pub navigation_stack: Vec<String>,

    /// Current focus entity ID (None = root/universe level)
    pub focus_entity_id: Option<String>,

    /// Current focus entity name for display
    pub focus_entity_name: String,

    /// Current hierarchy level depth (0 = universe root)
    pub depth: usize,

    /// Entity types at each level for UI display
    pub level_types: Vec<String>,

    /// Transition animation progress (0.0 to 1.0)
    pub transition_progress: f64,

    /// Is currently transitioning between levels
    pub is_transitioning: bool,
}

impl Default for HierarchyNavigationState {
    fn default() -> Self {
        HierarchyNavigationState {
            navigation_stack: Vec::new(),
            focus_entity_id: None,
            focus_entity_name: "Universe".to_string(),
            depth: 0,
            level_types: vec!["Universe".to_string()],
            transition_progress: 1.0,
            is_transitioning: false,
        }
    }
}

impl HierarchyNavigationState {
    /// Create a new hierarchy navigation state
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter an entity (drill down into its children/composition)
    pub fn enter_entity(&mut self, entity_id: String, entity_name: String, entity_type: String) {
        self.navigation_stack.push(entity_id.clone());
        self.focus_entity_id = Some(entity_id);
        self.focus_entity_name = entity_name;
        self.depth += 1;
        self.level_types.push(entity_type);
        self.transition_progress = 0.0;
        self.is_transitioning = true;
    }

    /// Go back up one level
    pub fn go_back(&mut self) -> bool {
        if self.navigation_stack.is_empty() {
            return false;
        }

        self.navigation_stack.pop();
        self.focus_entity_id = self.navigation_stack.last().cloned();
        self.depth = self.depth.saturating_sub(1);

        if self.level_types.len() > 1 {
            self.level_types.pop();
        }

        self.focus_entity_name = if let Some(id) = &self.focus_entity_id {
            id.clone()
        } else {
            "Universe".to_string()
        };

        self.transition_progress = 0.0;
        self.is_transitioning = true;
        true
    }

    /// Go back to the root (universe level)
    pub fn go_to_root(&mut self) {
        self.navigation_stack.clear();
        self.focus_entity_id = None;
        self.focus_entity_name = "Universe".to_string();
        self.depth = 0;
        self.level_types = vec!["Universe".to_string()];
        self.transition_progress = 0.0;
        self.is_transitioning = true;
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        !self.navigation_stack.is_empty()
    }

    /// Get the current focus path as a string (for UI display)
    pub fn get_path_string(&self) -> String {
        if self.navigation_stack.is_empty() {
            "Universe".to_string()
        } else {
            format!("Universe → {}", self.navigation_stack.join(" → "))
        }
    }

    /// Update transition animation
    pub fn update_transition(&mut self, delta_time: f64) {
        if self.is_transitioning {
            self.transition_progress += delta_time * 3.0; // ~333ms transition
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.is_transitioning = false;
            }
        }
    }

    /// Check if a specific entity is an ancestor of the current focus
    pub fn is_ancestor(&self, entity_id: &str) -> bool {
        self.navigation_stack.iter().any(|id| id == entity_id)
    }

    /// Check if we're currently inside a specific entity
    pub fn is_inside(&self, entity_id: &str) -> bool {
        self.focus_entity_id.as_deref() == Some(entity_id)
    }
}

/// Space controller - manages spatial navigation and zoom
#[derive(Debug, Clone)]
pub struct SpaceController {
    /// Current zoom level in meters
    pub zoom_level: f64,

    /// Minimum zoom level
    pub min_zoom: f64,

    /// Maximum zoom level
    pub max_zoom: f64,

    /// Focus point
    pub focus_point: Coordinate3D,

    /// Spatial expansion rate
    pub expansion_rate: f64,

    /// Spatial focus
    pub spatial_focus: SpatialFocus,

    /// Navigation state
    pub navigation: NavigationState,

    /// Scale transition
    pub scale_transition: ScaleTransition,

    /// Current scale level
    pub current_scale: ScaleLevel,

    /// Target scale level
    pub target_scale: ScaleLevel,

    /// Hierarchy navigation for nested entity exploration
    pub hierarchy: HierarchyNavigationState,
}

impl Default for SpaceController {
    fn default() -> Self {
        SpaceController {
            zoom_level: 1.0e-6,
            min_zoom: 1.616255e-35, // Planck length
            max_zoom: 8.8e26,       // Observable universe
            focus_point: Coordinate3D::origin(),
            expansion_rate: 0.0,
            spatial_focus: SpatialFocus::default(),
            navigation: NavigationState::default(),
            scale_transition: ScaleTransition::default(),
            current_scale: ScaleLevel::Cellular,
            target_scale: ScaleLevel::Cellular,
            hierarchy: HierarchyNavigationState::default(),
        }
    }
}

impl SpaceController {
    /// Create a new space controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the zoom level
    pub fn set_zoom(&mut self, level: f64) {
        self.zoom_level = level.max(self.min_zoom).min(self.max_zoom);
        self.current_scale = ScaleLevel::from_meters(self.zoom_level);
    }

    /// Get the current zoom level
    pub fn get_zoom(&self) -> f64 {
        self.zoom_level
    }

    /// Zoom in by a factor
    pub fn zoom_in(&mut self, factor: f64) {
        let new_zoom = self.zoom_level / factor;
        self.set_zoom(new_zoom);
    }

    /// Zoom out by a factor
    pub fn zoom_out(&mut self, factor: f64) {
        let new_zoom = self.zoom_level * factor;
        self.set_zoom(new_zoom);
    }

    /// Set the focus point
    pub fn set_focus_point(&mut self, point: Coordinate3D) {
        self.focus_point = point;
        self.navigation.navigate_to(point);
    }

    /// Get the focus point
    pub fn get_focus_point(&self) -> Coordinate3D {
        self.focus_point
    }

    /// Set the spatial expansion rate
    pub fn set_expansion_rate(&mut self, rate: f64) {
        self.expansion_rate = rate;
    }

    /// Get the spatial expansion rate
    pub fn get_expansion_rate(&self) -> f64 {
        self.expansion_rate
    }

    /// Navigate to a scale level
    pub fn navigate_scale(&mut self, scale: ScaleLevel) {
        if self.current_scale != scale {
            self.target_scale = scale;
            self.scale_transition.start(self.current_scale, scale);
        }
    }

    /// Navigate up one scale level
    pub fn navigate_up(&mut self) -> bool {
        if let Some(next_scale) = self.current_scale.next() {
            self.navigate_scale(next_scale);
            true
        } else {
            false
        }
    }

    /// Navigate down one scale level
    pub fn navigate_down(&mut self) -> bool {
        if let Some(prev_scale) = self.current_scale.previous() {
            self.navigate_scale(prev_scale);
            true
        } else {
            false
        }
    }

    /// Set spatial focus
    pub fn set_spatial_focus(&mut self, point: Coordinate3D, radius: f64) {
        self.spatial_focus.set_point(point);
        self.spatial_focus.set_radius(radius);
        self.spatial_focus.set_enabled(true);
    }

    /// Clear spatial focus
    pub fn clear_spatial_focus(&mut self) {
        self.spatial_focus.set_enabled(false);
    }

    /// Calculate the effective zoom for a point
    pub fn calculate_effective_zoom(&self, point: &Coordinate3D) -> f64 {
        let dilation = self.spatial_focus.calculate_dilation(point);
        self.zoom_level * dilation
    }

    /// Update the controller
    pub fn update(&mut self, delta_time: f64) {
        // Update navigation
        self.navigation.update(delta_time);

        // Update scale transition
        self.scale_transition.update(delta_time);

        // Update hierarchy transition
        self.hierarchy.update_transition(delta_time);

        // Update current scale if transition complete
        if !self.scale_transition.is_transitioning() {
            self.current_scale = self.target_scale;
            self.zoom_level = self.scale_transition.current_zoom();
        } else {
            self.zoom_level = self.scale_transition.current_zoom();
        }

        // Apply spatial expansion
        if self.expansion_rate != 0.0 {
            self.zoom_level *= (1.0 + self.expansion_rate * delta_time).max(0.0);
            self.set_zoom(self.zoom_level);
        }
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.zoom_level = 1.0e-6;
        self.focus_point = Coordinate3D::origin();
        self.expansion_rate = 0.0;
        self.spatial_focus = SpatialFocus::default();
        self.navigation = NavigationState::default();
        self.scale_transition = ScaleTransition::default();
        self.current_scale = ScaleLevel::Cellular;
        self.target_scale = ScaleLevel::Cellular;
        self.hierarchy = HierarchyNavigationState::default();
    }

    /// Get the current scale level
    pub fn get_current_scale(&self) -> ScaleLevel {
        if self.scale_transition.is_transitioning() {
            self.scale_transition.current_scale()
        } else {
            self.current_scale
        }
    }

    /// Check if navigating
    pub fn is_navigating(&self) -> bool {
        self.navigation.is_navigating() || self.scale_transition.is_transitioning() || self.hierarchy.is_transitioning
    }

    /// Get the navigation progress (0.0 to 1.0)
    pub fn get_navigation_progress(&self) -> f64 {
        if self.navigation.is_navigating() {
            self.navigation.navigation_progress
        } else if self.scale_transition.is_transitioning() {
            self.scale_transition.progress
        } else if self.hierarchy.is_transitioning {
            self.hierarchy.transition_progress
        } else {
            1.0
        }
    }

    // ========================================================================
    // Hierarchy Navigation Methods
    // ========================================================================

    /// Enter an entity (drill down into its internal structure)
    ///
    /// This "zooms into" an entity to show its children and composition.
    /// The scale level is automatically adjusted based on entity type.
    pub fn enter_entity(&mut self, entity_id: String, entity_name: String, entity_type: String, target_scale: ScaleLevel) {
        self.hierarchy.enter_entity(entity_id, entity_name, entity_type);
        self.navigate_scale(target_scale);
    }

    /// Go back up one level in the hierarchy
    ///
    /// Returns true if successful, false if already at root level.
    pub fn hierarchy_go_back(&mut self) -> bool {
        if self.hierarchy.go_back() {
            // Adjust scale based on new depth
            let target_scale = self.scale_for_depth(self.hierarchy.depth);
            self.navigate_scale(target_scale);
            true
        } else {
            false
        }
    }

    /// Go to the root (universe level)
    pub fn hierarchy_go_to_root(&mut self) {
        self.hierarchy.go_to_root();
        self.navigate_scale(ScaleLevel::Universal);
    }

    /// Check if we can go back in hierarchy
    pub fn can_go_back_hierarchy(&self) -> bool {
        self.hierarchy.can_go_back()
    }

    /// Get current hierarchy depth
    pub fn hierarchy_depth(&self) -> usize {
        self.hierarchy.depth
    }

    /// Get the current hierarchy path for display
    pub fn hierarchy_path(&self) -> String {
        self.hierarchy.get_path_string()
    }

    /// Get the current focus entity ID
    pub fn focus_entity_id(&self) -> Option<&String> {
        self.hierarchy.focus_entity_id.as_ref()
    }

    /// Determine appropriate scale level for hierarchy depth
    fn scale_for_depth(&self, depth: usize) -> ScaleLevel {
        match depth {
            0 => ScaleLevel::Universal,    // Universe root
            1 => ScaleLevel::Galactic,     // Galactic Logos / Galaxy
            2 => ScaleLevel::Stellar,      // Solar Logos / Star
            3 => ScaleLevel::Planetary,    // Planet
            4 => ScaleLevel::Organism,     // Lifeform / Being
            5 => ScaleLevel::Cellular,     // Cell
            6 => ScaleLevel::Molecular,    // Molecule
            7 => ScaleLevel::Atomic,       // Atom
            _ => ScaleLevel::Quantum,      // Quantum particles
        }
    }
}

/// Builder for SpaceController
#[derive(Debug, Clone, Default)]
pub struct SpaceControllerBuilder {
    min_zoom: f64,
    max_zoom: f64,
    initial_zoom: f64,
    initial_focus: Coordinate3D,
}

impl SpaceControllerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the zoom range
    pub fn with_zoom_range(mut self, min: f64, max: f64) -> Self {
        self.min_zoom = min;
        self.max_zoom = max;
        self
    }

    /// Set the initial zoom level
    pub fn with_initial_zoom(mut self, zoom: f64) -> Self {
        self.initial_zoom = zoom;
        self
    }

    /// Set the initial focus point
    pub fn with_initial_focus(mut self, point: Coordinate3D) -> Self {
        self.initial_focus = point;
        self
    }

    /// Build the space controller
    pub fn build(self) -> SpaceController {
        let mut controller = SpaceController::new();
        controller.min_zoom = self.min_zoom;
        controller.max_zoom = self.max_zoom;
        controller.zoom_level = self.initial_zoom;
        controller.focus_point = self.initial_focus;
        controller.current_scale = ScaleLevel::from_meters(self.initial_zoom);
        controller.target_scale = controller.current_scale;
        controller
    }
}

impl SpaceController {
    /// Create a space controller from configuration
    pub fn from_config(config: &GuiConfig) -> Self {
        SpaceControllerBuilder::new()
            .with_zoom_range(config.min_zoom, config.max_zoom)
            .with_initial_zoom(config.initial_zoom)
            .with_initial_focus(Coordinate3D::origin())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_focus_default() {
        let focus = SpatialFocus::new();
        assert_eq!(focus.point, Coordinate3D::origin());
        assert_eq!(focus.radius, 1.0e6);
        assert!(!focus.enabled);
    }

    #[test]
    fn test_spatial_focus_set_point() {
        let mut focus = SpatialFocus::new();
        let point = Coordinate3D::new(1.0, 2.0, 3.0);
        focus.set_point(point);

        assert_eq!(focus.point, point);
    }

    #[test]
    fn test_spatial_focus_contains_point() {
        let mut focus = SpatialFocus::new();
        focus.set_radius(10.0);

        assert!(focus.contains_point(&Coordinate3D::origin()));
        assert!(focus.contains_point(&Coordinate3D::new(5.0, 5.0, 5.0)));
        assert!(!focus.contains_point(&Coordinate3D::new(11.0, 0.0, 0.0)));
    }

    #[test]
    fn test_spatial_focus_calculate_dilation() {
        let mut focus = SpatialFocus::new();
        focus.set_enabled(true);
        focus.set_radius(10.0);

        // At focus point, dilation is 2.0
        assert_eq!(focus.calculate_dilation(&Coordinate3D::origin()), 2.0);

        // At half radius, dilation is 1.5
        assert_eq!(
            focus.calculate_dilation(&Coordinate3D::new(5.0, 0.0, 0.0)),
            1.5
        );

        // At radius, dilation is 1.0
        assert_eq!(
            focus.calculate_dilation(&Coordinate3D::new(10.0, 0.0, 0.0)),
            1.0
        );

        // Outside radius, dilation is 1.0
        assert_eq!(
            focus.calculate_dilation(&Coordinate3D::new(15.0, 0.0, 0.0)),
            1.0
        );

        // Disabled, dilation is always 1.0
        focus.set_enabled(false);
        assert_eq!(focus.calculate_dilation(&Coordinate3D::origin()), 1.0);
    }

    #[test]
    fn test_navigation_state_default() {
        let nav = NavigationState::new();
        assert_eq!(nav.position, Coordinate3D::origin());
        assert!(!nav.is_navigating);
    }

    #[test]
    fn test_navigation_state_navigate_to() {
        let mut nav = NavigationState::new();
        let target = Coordinate3D::new(10.0, 20.0, 30.0);
        nav.navigate_to(target);

        assert_eq!(nav.target_position, target);
        assert!(nav.is_navigating);
        assert_eq!(nav.navigation_progress, 0.0);
    }

    #[test]
    fn test_navigation_state_update() {
        let mut nav = NavigationState::new();
        nav.navigate_to(Coordinate3D::new(10.0, 20.0, 30.0));

        // Update to completion
        nav.update(1.0);

        assert!(!nav.is_navigating);
        assert_eq!(nav.position, Coordinate3D::new(10.0, 20.0, 30.0));
        assert_eq!(nav.navigation_progress, 1.0);
    }

    #[test]
    fn test_scale_transition_default() {
        let transition = ScaleTransition::new();
        assert_eq!(transition.from, ScaleLevel::Cellular);
        assert_eq!(transition.to, ScaleLevel::Cellular);
        assert!(!transition.is_transitioning);
    }

    #[test]
    fn test_scale_transition_start() {
        let mut transition = ScaleTransition::new();
        transition.start(ScaleLevel::Cellular, ScaleLevel::Planetary);

        assert_eq!(transition.from, ScaleLevel::Cellular);
        assert_eq!(transition.to, ScaleLevel::Planetary);
        assert!(transition.is_transitioning);
        assert_eq!(transition.progress, 0.0);
    }

    #[test]
    fn test_scale_transition_update() {
        let mut transition = ScaleTransition::new();
        transition.start(ScaleLevel::Cellular, ScaleLevel::Planetary);

        // Update to completion
        transition.update(1.0);

        assert!(!transition.is_transitioning);
        assert_eq!(transition.progress, 1.0);
        assert_eq!(transition.current_scale(), ScaleLevel::Planetary);
    }

    #[test]
    fn test_scale_transition_current_zoom() {
        let mut transition = ScaleTransition::new();
        transition.start(ScaleLevel::Cellular, ScaleLevel::Planetary);

        let from_zoom = ScaleLevel::Cellular.scale_in_meters();
        let to_zoom = ScaleLevel::Planetary.scale_in_meters();

        // At start
        assert_eq!(transition.current_zoom(), from_zoom);

        // At 50% progress
        transition.progress = 0.5;
        let mid_zoom = transition.current_zoom();
        assert!(mid_zoom > from_zoom && mid_zoom < to_zoom);

        // At completion
        transition.update(1.0);
        assert_eq!(transition.current_zoom(), to_zoom);
    }

    #[test]
    fn test_space_controller_default() {
        let controller = SpaceController::new();
        assert_eq!(controller.zoom_level, 1.0e-6);
        assert_eq!(controller.min_zoom, 1.616255e-35);
        assert_eq!(controller.max_zoom, 8.8e26);
        assert_eq!(controller.current_scale, ScaleLevel::Cellular);
    }

    #[test]
    fn test_space_controller_set_zoom() {
        let mut controller = SpaceController::new();
        controller.set_zoom(1.0e-3);

        assert_eq!(controller.zoom_level, 1.0e-3);
        assert_eq!(controller.current_scale, ScaleLevel::Organism);

        // Test bounds
        controller.set_zoom(1.0e-40);
        assert_eq!(controller.zoom_level, 1.616255e-35);

        controller.set_zoom(1.0e30);
        assert_eq!(controller.zoom_level, 8.8e26);
    }

    #[test]
    fn test_space_controller_zoom_in_out() {
        let mut controller = SpaceController::new();
        let initial_zoom = controller.zoom_level;

        controller.zoom_in(10.0);
        assert!(controller.zoom_level < initial_zoom);

        controller.zoom_out(10.0);
        assert!((controller.zoom_level - initial_zoom).abs() < 0.001);
    }

    #[test]
    fn test_space_controller_set_focus_point() {
        let mut controller = SpaceController::new();
        let point = Coordinate3D::new(5.0, 10.0, 15.0);
        controller.set_focus_point(point);

        assert_eq!(controller.focus_point, point);
        assert!(controller.navigation.is_navigating);
    }

    #[test]
    fn test_space_controller_navigate_scale() {
        let mut controller = SpaceController::new();
        controller.navigate_scale(ScaleLevel::Planetary);

        assert_eq!(controller.target_scale, ScaleLevel::Planetary);
        assert!(controller.scale_transition.is_transitioning);
    }

    #[test]
    fn test_space_controller_navigate_up_down() {
        let mut controller = SpaceController::new();

        // Navigate up
        assert!(controller.navigate_up());
        assert_eq!(controller.target_scale, ScaleLevel::Organism);
        // Update to complete transition
        controller.update(1.0);
        assert_eq!(controller.current_scale, ScaleLevel::Organism);

        // Navigate down
        assert!(controller.navigate_down());
        assert_eq!(controller.target_scale, ScaleLevel::Cellular);

        // Navigate down from lowest scale
        controller.current_scale = ScaleLevel::Quantum;
        assert!(!controller.navigate_down());

        // Navigate up from highest scale
        controller.current_scale = ScaleLevel::Universal;
        assert!(!controller.navigate_up());
    }

    #[test]
    fn test_space_controller_spatial_focus() {
        let mut controller = SpaceController::new();
        let point = Coordinate3D::new(5.0, 10.0, 15.0);
        controller.set_spatial_focus(point, 100.0);

        assert_eq!(controller.spatial_focus.point, point);
        assert_eq!(controller.spatial_focus.radius, 100.0);
        assert!(controller.spatial_focus.enabled);

        controller.clear_spatial_focus();
        assert!(!controller.spatial_focus.enabled);
    }

    #[test]
    fn test_space_controller_calculate_effective_zoom() {
        let mut controller = SpaceController::new();
        controller.set_spatial_focus(Coordinate3D::origin(), 10.0);

        // At focus point, zoom is doubled
        let effective_zoom = controller.calculate_effective_zoom(&Coordinate3D::origin());
        assert_eq!(effective_zoom, controller.zoom_level * 2.0);

        // Outside focus, zoom is normal
        let effective_zoom =
            controller.calculate_effective_zoom(&Coordinate3D::new(20.0, 0.0, 0.0));
        assert_eq!(effective_zoom, controller.zoom_level);
    }

    #[test]
    fn test_space_controller_update() {
        let mut controller = SpaceController::new();
        controller.set_focus_point(Coordinate3D::new(10.0, 20.0, 30.0));

        // Update to complete navigation
        controller.update(1.0);

        assert!(!controller.navigation.is_navigating());
        assert_eq!(controller.focus_point, Coordinate3D::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn test_space_controller_expansion_rate() {
        let mut controller = SpaceController::new();
        let initial_zoom = controller.zoom_level;

        controller.set_expansion_rate(0.1); // 10% expansion per second
        controller.update(1.0);

        assert!(controller.zoom_level > initial_zoom);
    }

    #[test]
    fn test_space_controller_reset() {
        let mut controller = SpaceController::new();
        controller.set_zoom(1.0e-3);
        controller.set_focus_point(Coordinate3D::new(10.0, 20.0, 30.0));
        controller.set_expansion_rate(0.1);
        controller.set_spatial_focus(Coordinate3D::origin(), 100.0);

        controller.reset();

        assert_eq!(controller.zoom_level, 1.0e-6);
        assert_eq!(controller.focus_point, Coordinate3D::origin());
        assert_eq!(controller.expansion_rate, 0.0);
        assert!(!controller.spatial_focus.enabled);
    }

    #[test]
    fn test_space_controller_from_config() {
        let config = GuiConfig::new()
            .with_zoom_range(1.0e-10, 1.0e20)
            .with_initial_zoom(1.0e-3);

        let controller = SpaceController::from_config(&config);

        assert_eq!(controller.min_zoom, 1.0e-10);
        assert_eq!(controller.max_zoom, 1.0e20);
        assert_eq!(controller.zoom_level, 1.0e-3);
    }

    #[test]
    fn test_space_controller_builder() {
        let controller = SpaceControllerBuilder::new()
            .with_zoom_range(1.0e-10, 1.0e20)
            .with_initial_zoom(1.0e-3)
            .with_initial_focus(Coordinate3D::new(5.0, 10.0, 15.0))
            .build();

        assert_eq!(controller.min_zoom, 1.0e-10);
        assert_eq!(controller.max_zoom, 1.0e20);
        assert_eq!(controller.zoom_level, 1.0e-3);
        assert_eq!(controller.focus_point, Coordinate3D::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_space_controller_is_navigating() {
        let mut controller = SpaceController::new();
        assert!(!controller.is_navigating());

        controller.set_focus_point(Coordinate3D::new(10.0, 20.0, 30.0));
        assert!(controller.is_navigating());

        controller.update(1.0);
        assert!(!controller.is_navigating());

        controller.navigate_scale(ScaleLevel::Planetary);
        assert!(controller.is_navigating());
    }

    #[test]
    fn test_space_controller_get_navigation_progress() {
        let mut controller = SpaceController::new();
        assert_eq!(controller.get_navigation_progress(), 1.0);

        controller.set_focus_point(Coordinate3D::new(10.0, 20.0, 30.0));
        assert_eq!(controller.get_navigation_progress(), 0.0);

        controller.update(0.1);
        assert!(controller.get_navigation_progress() > 0.0);
        assert!(controller.get_navigation_progress() < 1.0);
    }
}
