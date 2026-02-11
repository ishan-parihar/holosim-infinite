//! Interaction System - Entity inspection and interaction
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 5:
//! "Interactive Exploration:
//! - Click-to-inspect entities
//! - Follow entity evolution
//! - View archetype activation
//! - Monitor catalyst processing
//! - Track biological emergence
//! - Observe noospheric development
//! - Monitor Gaia health"
//!
//! This module provides:
//! - Click-to-inspect functionality
//! - Entity following and tracking
//! - Archetype activation visualization
//! - Catalyst status monitoring
//! - Biological emergence tracking
//! - Noospheric development observation
//! - Gaia health monitoring

use crate::entity_layer7::layer7::EntityId;
use crate::gui::{Coordinate3D, ScreenPosition};

/// Click result from raycasting
#[derive(Debug, Clone, PartialEq)]
pub enum ClickResult {
    /// Clicked on an entity
    Entity(EntityId),

    /// Clicked on empty space
    EmptySpace(Coordinate3D),

    /// Clicked outside the visible area
    OutOfBounds,
}

/// Entity information for inspection
#[derive(Debug, Clone, PartialEq)]
pub struct EntityInfo {
    /// Entity ID
    pub entity_id: EntityId,

    /// Entity type
    pub entity_type: String,

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Scale in meters
    pub scale: f64,

    /// Density level (1-8)
    pub density: u8,

    /// Polarity (-1 to 1)
    pub polarity: i8,

    /// Consciousness level (0.0 to 1.0)
    pub consciousness: f64,

    /// Free will capacity (0.0 to 1.0)
    pub free_will_capacity: f64,

    /// Evolution step number
    pub evolution_step: u64,

    /// Health status
    pub health_status: String,

    /// Archetype activations
    pub archetype_activations: Vec<(usize, f64)>,

    /// Active catalysts
    pub active_catalysts: Vec<String>,

    /// Biological stage (if applicable)
    pub biological_stage: Option<String>,

    /// Noospheric connections (if applicable)
    pub noospheric_connections: Vec<EntityId>,

    /// Gaia connection strength (if applicable)
    pub gaia_connection: Option<f64>,
}

impl EntityInfo {
    /// Create a new entity info
    pub fn new(entity_id: EntityId) -> Self {
        EntityInfo {
            entity_id,
            entity_type: "Unknown".to_string(),
            position: Coordinate3D::origin(),
            scale: 1.0e-6,
            density: 1,
            polarity: 0,
            consciousness: 0.0,
            free_will_capacity: 0.0,
            evolution_step: 0,
            health_status: "Healthy".to_string(),
            archetype_activations: Vec::new(),
            active_catalysts: Vec::new(),
            biological_stage: None,
            noospheric_connections: Vec::new(),
            gaia_connection: None,
        }
    }

    /// Get a summary of the entity
    pub fn summary(&self) -> String {
        format!(
            "Entity {}: {} - Density {} - Polarity {} - Consciousness {:.2}",
            self.entity_id, self.entity_type, self.density, self.polarity, self.consciousness
        )
    }

    /// Get detailed information
    pub fn details(&self) -> String {
        let mut details = vec![
            format!("Entity ID: {}", self.entity_id),
            format!("Type: {}", self.entity_type),
            format!(
                "Position: ({:.2}, {:.2}, {:.2})",
                self.position.x, self.position.y, self.position.z
            ),
            format!("Scale: {:.2e} m", self.scale),
            format!("Density: {}", self.density),
            format!("Polarity: {}", self.polarity),
            format!("Consciousness: {:.2}", self.consciousness),
            format!("Free Will Capacity: {:.2}", self.free_will_capacity),
            format!("Evolution Step: {}", self.evolution_step),
            format!("Health Status: {}", self.health_status),
        ];

        if !self.archetype_activations.is_empty() {
            details.push("Archetype Activations:".to_string());
            for (archetype, activation) in &self.archetype_activations {
                details.push(format!("  Archetype {}: {:.2}", archetype, activation));
            }
        }

        if !self.active_catalysts.is_empty() {
            details.push("Active Catalysts:".to_string());
            for catalyst in &self.active_catalysts {
                details.push(format!("  {}", catalyst));
            }
        }

        if let Some(stage) = &self.biological_stage {
            details.push(format!("Biological Stage: {}", stage));
        }

        if !self.noospheric_connections.is_empty() {
            details.push(format!(
                "Noospheric Connections: {} entities",
                self.noospheric_connections.len()
            ));
        }

        if let Some(connection) = self.gaia_connection {
            details.push(format!("Gaia Connection: {:.2}", connection));
        }

        details.join("\n")
    }
}

/// Archetype activation information
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeActivation {
    /// Entity ID
    pub entity_id: EntityId,

    /// Archetype number (1-22)
    pub archetype_number: usize,

    /// Activation level (0.0 to 1.0)
    pub activation_level: f64,

    /// Lambda value (0.0 to 100.0)
    pub lambda_value: f64,

    /// Current rung (1-7)
    pub current_rung: usize,

    /// Pathological indicators
    pub pathological_indicators: Vec<String>,

    /// Evolution trend (increasing, decreasing, stable)
    pub evolution_trend: String,
}

impl ArchetypeActivation {
    /// Create a new archetype activation
    pub fn new(entity_id: EntityId, archetype_number: usize) -> Self {
        ArchetypeActivation {
            entity_id,
            archetype_number,
            activation_level: 0.0,
            lambda_value: 0.0,
            current_rung: 1,
            pathological_indicators: Vec::new(),
            evolution_trend: "Stable".to_string(),
        }
    }

    /// Get the archetype name
    pub fn archetype_name(&self) -> &'static str {
        match self.archetype_number {
            1 => "Matrix of the Mind",
            2 => "Potentiator of the Mind",
            3 => "Catalyst of the Mind",
            4 => "Experience of the Mind",
            5 => "Significator of the Mind",
            6 => "Matrix of the Body",
            7 => "Potentiator of the Body",
            8 => "Catalyst of the Body",
            9 => "Experience of the Body",
            10 => "Significator of the Body",
            11 => "Matrix of the Spirit",
            12 => "Potentiator of the Spirit",
            13 => "Catalyst of the Spirit",
            14 => "Experience of the Spirit",
            15 => "Significator of the Spirit",
            16 => "Choice",
            17 => "Matrix of the Mind/Body/Spirit",
            18 => "Potentiator of the Mind/Body/Spirit",
            19 => "Catalyst of the Mind/Body/Spirit",
            20 => "Experience of the Mind/Body/Spirit",
            21 => "Significator of the Mind/Body/Spirit",
            22 => "The Great Way",
            _ => "Unknown",
        }
    }

    /// Get a summary
    pub fn summary(&self) -> String {
        format!(
            "Archetype {}: {} - Activation {:.2}, Lambda {:.1}, Rung {}",
            self.archetype_number,
            self.archetype_name(),
            self.activation_level,
            self.lambda_value,
            self.current_rung
        )
    }
}

/// Catalyst status for monitoring
#[derive(Debug, Clone, PartialEq)]
pub struct CatalystStatus {
    /// Entity ID
    pub entity_id: EntityId,

    /// Catalyst type
    pub catalyst_type: String,

    /// Catalyst intensity (0.0 to 1.0)
    pub intensity: f64,

    /// Processing state
    pub processing_state: String,

    /// Progress (0.0 to 1.0)
    pub progress: f64,

    /// Choice made (if any)
    pub choice_made: Option<String>,

    /// Experience gained (0.0 to 1.0)
    pub experience_gained: f64,

    /// Time remaining (in simulation steps)
    pub time_remaining: Option<u64>,
}

impl CatalystStatus {
    /// Create a new catalyst status
    pub fn new(entity_id: EntityId, catalyst_type: String) -> Self {
        CatalystStatus {
            entity_id,
            catalyst_type,
            intensity: 0.0,
            processing_state: "Pending".to_string(),
            progress: 0.0,
            choice_made: None,
            experience_gained: 0.0,
            time_remaining: None,
        }
    }

    /// Get a summary
    pub fn summary(&self) -> String {
        format!(
            "Catalyst {}: {} - Intensity {:.2}, State {}, Progress {:.2}",
            self.entity_id,
            self.catalyst_type,
            self.intensity,
            self.processing_state,
            self.progress
        )
    }
}

/// Entity tracker for following entities over time
#[derive(Debug, Clone, PartialEq)]
pub struct EntityTracker {
    /// Currently tracked entity and step
    pub tracked_entity: Option<(EntityId, u64)>,

    /// Tracking history
    pub tracking_history: Vec<(EntityId, u64)>,

    /// Maximum history size
    pub max_history_size: usize,

    /// Auto-follow enabled
    pub auto_follow: bool,

    /// Follow speed (0.0 to 1.0)
    pub follow_speed: f64,
}

impl Default for EntityTracker {
    fn default() -> Self {
        EntityTracker {
            tracked_entity: None,
            tracking_history: Vec::new(),
            max_history_size: 100,
            auto_follow: false,
            follow_speed: 0.5,
        }
    }
}

impl EntityTracker {
    /// Create a new entity tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Track an entity
    pub fn track_entity(&mut self, entity_id: EntityId, current_step: u64) {
        // Save previous entity to history
        if let Some((ref prev_entity, prev_step)) = self.tracked_entity {
            self.tracking_history.push((prev_entity.clone(), prev_step));

            // Trim history if too large
            if self.tracking_history.len() > self.max_history_size {
                self.tracking_history.remove(0);
            }
        }

        self.tracked_entity = Some((entity_id, current_step));
    }

    /// Stop tracking
    pub fn stop_tracking(&mut self) {
        self.tracked_entity = None;
    }

    /// Get the currently tracked entity
    pub fn get_tracked_entity(&self) -> Option<EntityId> {
        self.tracked_entity
            .as_ref()
            .map(|(entity_id, _)| entity_id.clone())
    }

    /// Check if tracking
    pub fn is_tracking(&self) -> bool {
        self.tracked_entity.is_some()
    }

    /// Navigate back in tracking history
    pub fn navigate_back(&mut self) -> Option<(EntityId, u64)> {
        if let Some((entity_id, step)) = self.tracking_history.pop() {
            self.tracked_entity = Some((entity_id.clone(), step));
            Some((entity_id, step))
        } else {
            None
        }
    }

    /// Set auto-follow
    pub fn set_auto_follow(&mut self, enabled: bool) {
        self.auto_follow = enabled;
    }

    /// Set follow speed
    pub fn set_follow_speed(&mut self, speed: f64) {
        self.follow_speed = speed.max(0.0).min(1.0);
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.tracking_history.clear();
    }
}

/// Entity inspector for detailed entity information
#[derive(Debug, Clone, Default)]
pub struct EntityInspector {
    /// Inspected entity
    pub inspected_entity: Option<EntityInfo>,

    /// Archetype activations
    pub archetype_activations: Vec<ArchetypeActivation>,

    /// Catalyst statuses
    pub catalyst_statuses: Vec<CatalystStatus>,

    /// Inspection history
    pub inspection_history: Vec<EntityInfo>,

    /// Maximum history size
    pub max_history_size: usize,
}

impl EntityInspector {
    /// Create a new entity inspector
    pub fn new() -> Self {
        Self::default()
    }

    /// Inspect an entity
    pub fn inspect_entity(&mut self, entity_info: EntityInfo) {
        // Save previous entity to history
        if let Some(prev_entity) = &self.inspected_entity {
            self.inspection_history.push(prev_entity.clone());

            // Trim history if too large
            if self.inspection_history.len() > self.max_history_size {
                self.inspection_history.remove(0);
            }
        }

        self.inspected_entity = Some(entity_info);

        // Clear previous archetype and catalyst data
        self.archetype_activations.clear();
        self.catalyst_statuses.clear();
    }

    /// Add archetype activation
    pub fn add_archetype_activation(&mut self, activation: ArchetypeActivation) {
        self.archetype_activations.push(activation);
    }

    /// Add catalyst status
    pub fn add_catalyst_status(&mut self, status: CatalystStatus) {
        self.catalyst_statuses.push(status);
    }

    /// Get the inspected entity
    pub fn get_inspected_entity(&self) -> Option<&EntityInfo> {
        self.inspected_entity.as_ref()
    }

    /// Check if inspecting
    pub fn is_inspecting(&self) -> bool {
        self.inspected_entity.is_some()
    }

    /// Stop inspecting
    pub fn stop_inspecting(&mut self) {
        self.inspected_entity = None;
        self.archetype_activations.clear();
        self.catalyst_statuses.clear();
    }

    /// Navigate back in inspection history
    pub fn navigate_back(&mut self) -> Option<EntityInfo> {
        if let Some(prev_entity) = self.inspection_history.pop() {
            self.inspected_entity = Some(prev_entity.clone());
            Some(prev_entity)
        } else {
            None
        }
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.inspection_history.clear();
    }
}

/// Raycaster for detecting clicks on entities
#[derive(Debug, Clone)]
pub struct Raycaster {
    /// Screen width
    pub screen_width: u32,

    /// Screen height
    pub screen_height: u32,

    /// Field of view in degrees
    pub fov: f32,

    /// Camera position
    pub camera_position: Coordinate3D,

    /// Camera target
    pub camera_target: Coordinate3D,
}

impl Raycaster {
    /// Create a new raycaster
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Raycaster {
            screen_width,
            screen_height,
            fov: 60.0,
            camera_position: Coordinate3D::new(0.0, 0.0, 10.0),
            camera_target: Coordinate3D::origin(),
        }
    }

    /// Set camera position and target
    pub fn set_camera(&mut self, position: Coordinate3D, target: Coordinate3D) {
        self.camera_position = position;
        self.camera_target = target;
    }

    /// Set screen dimensions
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    /// Cast a ray from screen position and detect what it hits
    /// Returns the entity ID if an entity is clicked, or the 3D position of the click
    pub fn cast_ray(
        &self,
        screen_pos: ScreenPosition,
        entities: &[(EntityId, Coordinate3D, f64)],
    ) -> ClickResult {
        // Check if screen position is valid
        if screen_pos.x < 0.0
            || screen_pos.x >= self.screen_width as f32
            || screen_pos.y < 0.0
            || screen_pos.y >= self.screen_height as f32
        {
            return ClickResult::OutOfBounds;
        }

        // Simple raycast - check if click is near any entity
        // In a real implementation, this would use proper 3D raycasting
        let click_2d = (screen_pos.x, screen_pos.y);

        for (entity_id, position, scale) in entities {
            // Project 3D position to 2D screen space (simplified)
            let projected_x = self.screen_width as f32 / 2.0 + (position.x * 100.0) as f32;
            let projected_y = self.screen_height as f32 / 2.0 + (position.y * 100.0) as f32;

            // Calculate distance in screen space
            let dx = click_2d.0 - projected_x;
            let dy = click_2d.1 - projected_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if click is within entity's hit area (simplified)
            let hit_radius = (scale.log10().abs() * 20.0).max(10.0) as f32;

            if distance <= hit_radius {
                return ClickResult::Entity(entity_id.clone());
            }
        }

        // Clicked on empty space - calculate 3D position (simplified)
        let world_x = (click_2d.0 - self.screen_width as f32 / 2.0) / 100.0;
        let world_y = (click_2d.1 - self.screen_height as f32 / 2.0) / 100.0;
        let world_pos = Coordinate3D::new(world_x as f64, world_y as f64, 0.0);

        ClickResult::EmptySpace(world_pos)
    }
}

/// Interaction system - main entry point for user interactions
#[derive(Debug, Clone)]
pub struct InteractionSystem {
    /// Raycaster
    raycaster: Raycaster,

    /// Entity tracker
    tracker: EntityTracker,

    /// Entity inspector
    inspector: EntityInspector,

    /// Click history
    click_history: Vec<ClickResult>,

    /// Maximum click history size
    pub max_click_history: usize,
}

impl Default for InteractionSystem {
    fn default() -> Self {
        InteractionSystem {
            raycaster: Raycaster::new(1920, 1080),
            tracker: EntityTracker::new(),
            inspector: EntityInspector::new(),
            click_history: Vec::new(),
            max_click_history: 100,
        }
    }
}

impl InteractionSystem {
    /// Create a new interaction system
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a click event
    pub fn on_click(
        &mut self,
        screen_pos: ScreenPosition,
        entities: &[(EntityId, Coordinate3D, f64)],
    ) -> ClickResult {
        let result = self.raycaster.cast_ray(screen_pos, entities);

        // Save to history
        self.click_history.push(result.clone());
        if self.click_history.len() > self.max_click_history {
            self.click_history.remove(0);
        }

        result
    }

    /// Inspect an entity
    pub fn inspect_entity(&mut self, entity_info: EntityInfo) {
        self.inspector.inspect_entity(entity_info);
    }

    /// Track an entity
    pub fn track_entity(&mut self, entity_id: EntityId, current_step: u64) {
        self.tracker.track_entity(entity_id, current_step);
    }

    /// Stop tracking
    pub fn stop_tracking(&mut self) {
        self.tracker.stop_tracking();
    }

    /// Get the tracked entity
    pub fn get_tracked_entity(&self) -> Option<EntityId> {
        self.tracker.get_tracked_entity()
    }

    /// Get the inspected entity info
    pub fn get_inspected_entity(&self) -> Option<&EntityInfo> {
        self.inspector.get_inspected_entity()
    }

    /// Check if inspecting
    pub fn is_inspecting(&self) -> bool {
        self.inspector.is_inspecting()
    }

    /// View archetype activation
    pub fn view_archetype_activation(&mut self, entity_id: EntityId) -> Vec<ArchetypeActivation> {
        // In a real implementation, this would query the simulation for archetype data
        vec![ArchetypeActivation::new(entity_id, 1)]
    }

    /// Monitor catalyst processing
    pub fn monitor_catalyst_processing(&mut self, entity_id: EntityId) -> Vec<CatalystStatus> {
        // In a real implementation, this would query the simulation for catalyst data
        vec![CatalystStatus::new(
            entity_id,
            "Generic Catalyst".to_string(),
        )]
    }

    /// Set screen size
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.raycaster.set_screen_size(width, height);
    }

    /// Set camera position
    pub fn set_camera(&mut self, position: Coordinate3D, target: Coordinate3D) {
        self.raycaster.set_camera(position, target);
    }

    /// Navigate back in click history
    pub fn navigate_back_click(&mut self) -> Option<ClickResult> {
        if self.click_history.len() > 1 {
            self.click_history.pop();
            Some(self.click_history.last().cloned().unwrap())
        } else {
            None
        }
    }

    /// Clear click history
    pub fn clear_click_history(&mut self) {
        self.click_history.clear();
    }

    /// Get click history
    pub fn get_click_history(&self) -> &[ClickResult] {
        &self.click_history
    }
}

/// Builder for InteractionSystem
#[derive(Debug, Clone, Default)]
pub struct InteractionSystemBuilder {
    screen_width: u32,
    screen_height: u32,
    max_click_history: usize,
    max_inspection_history: usize,
    max_tracking_history: usize,
}

impl InteractionSystemBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set screen size
    pub fn with_screen_size(mut self, width: u32, height: u32) -> Self {
        self.screen_width = width;
        self.screen_height = height;
        self
    }

    /// Set max click history size
    pub fn with_max_click_history(mut self, size: usize) -> Self {
        self.max_click_history = size;
        self
    }

    /// Set max inspection history size
    pub fn with_max_inspection_history(mut self, size: usize) -> Self {
        self.max_inspection_history = size;
        self
    }

    /// Set max tracking history size
    pub fn with_max_tracking_history(mut self, size: usize) -> Self {
        self.max_tracking_history = size;
        self
    }

    /// Build the interaction system
    pub fn build(self) -> InteractionSystem {
        let mut system = InteractionSystem::new();
        system.raycaster = Raycaster::new(self.screen_width, self.screen_height);
        system.max_click_history = self.max_click_history;
        system.inspector.max_history_size = self.max_inspection_history;
        system.tracker.max_history_size = self.max_tracking_history;
        system
    }
}

impl InteractionSystem {
    /// Create an interaction system from configuration
    pub fn from_config(config: &crate::gui::GuiConfig) -> Self {
        InteractionSystemBuilder::new()
            .with_screen_size(config.window_width, config.window_height)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::GuiConfig;

    #[test]
    fn test_entity_info_new() {
        let info = EntityInfo::new(EntityId::new("1".to_string()));
        assert_eq!(info.entity_id, EntityId::new("1".to_string()));
        assert_eq!(info.entity_type, "Unknown");
        assert_eq!(info.density, 1);
        assert_eq!(info.polarity, 0);
    }

    #[test]
    fn test_entity_info_summary() {
        let mut info = EntityInfo::new(EntityId::new("1".to_string()));
        info.entity_type = "Human".to_string();
        info.density = 3;
        info.polarity = 1;
        info.consciousness = 0.5;

        let summary = info.summary();
        assert!(summary.contains("Entity"));
        assert!(summary.contains("Human"));
        assert!(summary.contains("Density 3"));
        assert!(summary.contains("Polarity 1"));
        assert!(summary.contains("Consciousness 0.50"));
    }

    #[test]
    fn test_entity_info_details() {
        let mut info = EntityInfo::new(EntityId::new("1".to_string()));
        info.entity_type = "Human".to_string();
        info.position = Coordinate3D::new(1.0, 2.0, 3.0);
        info.scale = 1.75;
        info.density = 3;
        info.polarity = 1;
        info.consciousness = 0.5;
        info.free_will_capacity = 0.7;
        info.evolution_step = 100;
        info.health_status = "Healthy".to_string();

        let details = info.details();
        assert!(details.contains("Entity ID: 1"));
        assert!(details.contains("Type: Human"));
        assert!(details.contains("Position: (1.00, 2.00, 3.00)"));
        assert!(details.contains("Scale:"));
        assert!(details.contains("Density: 3"));
        assert!(details.contains("Polarity: 1"));
        assert!(details.contains("Consciousness: 0.50"));
        assert!(details.contains("Free Will Capacity: 0.70"));
        assert!(details.contains("Evolution Step: 100"));
        assert!(details.contains("Health Status: Healthy"));
    }

    #[test]
    fn test_archetype_activation_new() {
        let activation = ArchetypeActivation::new(EntityId::new("1".to_string()), 1);
        assert_eq!(activation.entity_id, EntityId::new("1".to_string()));
        assert_eq!(activation.archetype_number, 1);
        assert_eq!(activation.current_rung, 1);
        assert_eq!(activation.evolution_trend, "Stable");
    }

    #[test]
    fn test_archetype_activation_names() {
        let activation1 = ArchetypeActivation::new(EntityId::new("1".to_string()), 1);
        assert_eq!(activation1.archetype_name(), "Matrix of the Mind");

        let activation16 = ArchetypeActivation::new(EntityId::new("1".to_string()), 16);
        assert_eq!(activation16.archetype_name(), "Choice");

        let activation22 = ArchetypeActivation::new(EntityId::new("1".to_string()), 22);
        assert_eq!(activation22.archetype_name(), "The Great Way");
    }

    #[test]
    fn test_archetype_activation_summary() {
        let mut activation = ArchetypeActivation::new(EntityId::new("1".to_string()), 1);
        activation.activation_level = 0.8;
        activation.lambda_value = 50.0;
        activation.current_rung = 3;

        let summary = activation.summary();
        assert!(summary.contains("Archetype 1"));
        assert!(summary.contains("Matrix of the Mind"));
        assert!(summary.contains("Activation 0.80"));
        assert!(summary.contains("Lambda 50.0"));
        assert!(summary.contains("Rung 3"));
    }

    #[test]
    fn test_catalyst_status_new() {
        let status =
            CatalystStatus::new(EntityId::new("1".to_string()), "Test Catalyst".to_string());
        assert_eq!(status.entity_id, EntityId::new("1".to_string()));
        assert_eq!(status.catalyst_type, "Test Catalyst");
        assert_eq!(status.processing_state, "Pending");
        assert_eq!(status.progress, 0.0);
    }

    #[test]
    fn test_catalyst_status_summary() {
        let mut status =
            CatalystStatus::new(EntityId::new("1".to_string()), "Test Catalyst".to_string());
        status.intensity = 0.7;
        status.processing_state = "Processing".to_string();
        status.progress = 0.5;

        let summary = status.summary();
        assert!(summary.contains("Catalyst 1"));
        assert!(summary.contains("Test Catalyst"));
        assert!(summary.contains("Intensity 0.70"));
        assert!(summary.contains("State Processing"));
        assert!(summary.contains("Progress 0.50"));
    }

    #[test]
    fn test_entity_tracker_default() {
        let tracker = EntityTracker::new();
        assert!(tracker.tracked_entity.is_none());
        assert!(tracker.tracking_history.is_empty());
        assert!(!tracker.auto_follow);
        assert_eq!(tracker.follow_speed, 0.5);
    }

    #[test]
    fn test_entity_tracker_track() {
        let mut tracker = EntityTracker::new();
        tracker.track_entity(EntityId::new("1".to_string()), 100);

        assert_eq!(
            tracker.get_tracked_entity(),
            Some(EntityId::new("1".to_string()))
        );
        assert!(tracker.is_tracking());
        assert_eq!(tracker.tracking_history.len(), 0); // No previous entity
    }

    #[test]
    fn test_entity_tracker_track_multiple() {
        let mut tracker = EntityTracker::new();
        tracker.track_entity(EntityId::new("1".to_string()), 100);
        tracker.track_entity(EntityId::new("2".to_string()), 200);

        assert_eq!(
            tracker.get_tracked_entity(),
            Some(EntityId::new("2".to_string()))
        );
        assert_eq!(tracker.tracking_history.len(), 1);
        assert_eq!(
            tracker.tracking_history[0],
            (EntityId::new("1".to_string()), 100)
        );
    }

    #[test]
    fn test_entity_tracker_navigate_back() {
        let mut tracker = EntityTracker::new();
        tracker.track_entity(EntityId::new("1".to_string()), 100);
        tracker.track_entity(EntityId::new("2".to_string()), 200);

        let prev = tracker.navigate_back();
        assert_eq!(prev, Some((EntityId::new("1".to_string()), 100)));
        assert_eq!(
            tracker.get_tracked_entity(),
            Some(EntityId::new("1".to_string()))
        );
    }

    #[test]
    fn test_entity_tracker_stop() {
        let mut tracker = EntityTracker::new();
        tracker.track_entity(EntityId::new("1".to_string()), 100);
        tracker.stop_tracking();

        assert!(!tracker.is_tracking());
        assert!(tracker.get_tracked_entity().is_none());
    }

    #[test]
    fn test_entity_inspector_default() {
        let inspector = EntityInspector::new();
        assert!(inspector.inspected_entity.is_none());
        assert!(inspector.archetype_activations.is_empty());
        assert!(inspector.catalyst_statuses.is_empty());
    }

    #[test]
    fn test_entity_inspector_inspect() {
        let mut inspector = EntityInspector::new();
        let info = EntityInfo::new(EntityId::new("1".to_string()));
        inspector.inspect_entity(info);

        assert!(inspector.is_inspecting());
        assert_eq!(
            inspector.get_inspected_entity().unwrap().entity_id,
            EntityId::new("1".to_string())
        );
    }

    #[test]
    fn test_entity_inspector_add_archetype() {
        let mut inspector = EntityInspector::new();
        let activation = ArchetypeActivation::new(EntityId::new("1".to_string()), 1);
        inspector.add_archetype_activation(activation);

        assert_eq!(inspector.archetype_activations.len(), 1);
    }

    #[test]
    fn test_entity_inspector_add_catalyst() {
        let mut inspector = EntityInspector::new();
        let status = CatalystStatus::new(EntityId::new("1".to_string()), "Test".to_string());
        inspector.add_catalyst_status(status);

        assert_eq!(inspector.catalyst_statuses.len(), 1);
    }

    #[test]
    fn test_raycaster_new() {
        let raycaster = Raycaster::new(1920, 1080);
        assert_eq!(raycaster.screen_width, 1920);
        assert_eq!(raycaster.screen_height, 1080);
        assert_eq!(raycaster.fov, 60.0);
    }

    #[test]
    fn test_raycaster_cast_ray_out_of_bounds() {
        let raycaster = Raycaster::new(1920, 1080);
        let entities = vec![];
        let result = raycaster.cast_ray(ScreenPosition::new(-1.0, 500.0), &entities);

        assert_eq!(result, ClickResult::OutOfBounds);
    }

    #[test]
    fn test_raycaster_cast_ray_empty_space() {
        let raycaster = Raycaster::new(1920, 1080);
        let entities = vec![];
        let result = raycaster.cast_ray(ScreenPosition::new(500.0, 500.0), &entities);

        match result {
            ClickResult::EmptySpace(_) => (),
            _ => panic!("Expected EmptySpace result"),
        }
    }

    #[test]
    fn test_raycaster_cast_ray_entity() {
        let raycaster = Raycaster::new(1920, 1080);
        let entities = vec![(EntityId::new("1".to_string()), Coordinate3D::origin(), 1.0)];
        let result = raycaster.cast_ray(ScreenPosition::new(960.0, 540.0), &entities);

        assert_eq!(result, ClickResult::Entity(EntityId::new("1".to_string())));
    }

    #[test]
    fn test_interaction_system_default() {
        let system = InteractionSystem::new();
        assert!(!system.inspector.is_inspecting());
        assert!(!system.tracker.is_tracking());
        assert!(system.click_history.is_empty());
    }

    #[test]
    fn test_interaction_system_on_click() {
        let mut system = InteractionSystem::new();
        let entities = vec![(EntityId::new("1".to_string()), Coordinate3D::origin(), 1.0)];
        let result = system.on_click(ScreenPosition::new(960.0, 540.0), &entities);

        assert_eq!(result, ClickResult::Entity(EntityId::new("1".to_string())));
        assert_eq!(system.click_history.len(), 1);
    }

    #[test]
    fn test_interaction_system_inspect() {
        let mut system = InteractionSystem::new();
        let info = EntityInfo::new(EntityId::new("1".to_string()));
        system.inspect_entity(info);

        assert!(system.is_inspecting());
    }

    #[test]
    fn test_interaction_system_track() {
        let mut system = InteractionSystem::new();
        system.track_entity(EntityId::new("1".to_string()), 100);

        assert_eq!(
            system.get_tracked_entity(),
            Some(EntityId::new("1".to_string()))
        );
    }

    #[test]
    fn test_interaction_system_view_archetype() {
        let mut system = InteractionSystem::new();
        let activations = system.view_archetype_activation(EntityId::new("1".to_string()));

        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].archetype_number, 1);
    }

    #[test]
    fn test_interaction_system_monitor_catalyst() {
        let mut system = InteractionSystem::new();
        let statuses = system.monitor_catalyst_processing(EntityId::new("1".to_string()));

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].catalyst_type, "Generic Catalyst");
    }

    #[test]
    fn test_interaction_system_navigate_back_click() {
        let mut system = InteractionSystem::new();
        let entities = vec![(EntityId::new("1".to_string()), Coordinate3D::origin(), 1.0)];

        system.on_click(ScreenPosition::new(960.0, 540.0), &entities);
        system.on_click(ScreenPosition::new(500.0, 500.0), &entities);

        assert_eq!(system.click_history.len(), 2);

        let prev = system.navigate_back_click();
        assert!(prev.is_some());
        assert_eq!(system.click_history.len(), 1);
    }

    #[test]
    fn test_interaction_system_builder() {
        let system = InteractionSystemBuilder::new()
            .with_screen_size(1280, 720)
            .with_max_click_history(50)
            .with_max_inspection_history(50)
            .with_max_tracking_history(50)
            .build();

        assert_eq!(system.raycaster.screen_width, 1280);
        assert_eq!(system.raycaster.screen_height, 720);
        assert_eq!(system.max_click_history, 50);
        assert_eq!(system.inspector.max_history_size, 50);
        assert_eq!(system.tracker.max_history_size, 50);
    }

    #[test]
    fn test_interaction_system_from_config() {
        let config = GuiConfig::new().with_window_size(1280, 720);
        let system = InteractionSystem::from_config(&config);

        assert_eq!(system.raycaster.screen_width, 1280);
        assert_eq!(system.raycaster.screen_height, 720);
    }
}
