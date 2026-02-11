//! Interaction System Module
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 5 Week 10:
//! "Interaction System - Raycasting system, Input handling, Camera bookmarks, Time controls"
//!
//! This module provides:
//! - Raycasting for 3D entity selection
//! - Unified input handling
//! - Camera bookmark management
//! - Selection and hover state management
//! - Mouse interaction handling

pub mod bookmarks;
pub mod input_handler;
pub mod raycaster;

// Re-export main types
pub use bookmarks::{
    BookmarkManager, CameraBookmark, CameraTransition, EasingFunction, ScaleLevel,
};
pub use input_handler::{
    InputAction, InputEvent, InputHandler, KeyBinding, Modifiers, MouseInteractionHandler,
};
pub use raycaster::{Ray, RaycastResult, Raycaster3D, SelectionManager, SelectionState};

use crate::entity_layer7::layer7::EntityId;
use crate::gui::time_controller::TimeController;
use crate::gui::Camera2D as Camera3D;
use crate::gui::{Coordinate3D, ScreenPosition};
use std::time::Instant;
use winit::event::WindowEvent;

/// Unified interaction system integrating all interaction components
pub struct InteractionSystem {
    /// Raycaster for 3D picking
    raycaster: Raycaster3D,
    /// Selection manager
    selection_manager: SelectionManager,
    /// Input handler
    input_handler: InputHandler,
    /// Mouse interaction handler
    mouse_handler: MouseInteractionHandler,
    /// Bookmark manager
    bookmark_manager: BookmarkManager,
    /// Entity data for raycasting
    entities: Vec<(EntityId, Coordinate3D, f64)>,
    /// Last update time
    last_update: Instant,
    /// Current camera
    current_camera: Camera3D,
    /// Callbacks for selection events
    selection_callbacks: Vec<Box<dyn Fn(&EntityId) + Send + Sync>>,
    /// Callbacks for deselection events
    deselect_callbacks: Vec<Box<dyn Fn() + Send + Sync>>,
}

impl InteractionSystem {
    /// Create a new interaction system
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let mouse_handler = MouseInteractionHandler::new(screen_width, screen_height);

        InteractionSystem {
            raycaster: Raycaster3D::new(screen_width, screen_height),
            selection_manager: SelectionManager::new(),
            input_handler: InputHandler::new(),
            mouse_handler,
            bookmark_manager: BookmarkManager::new(),
            entities: Vec::new(),
            last_update: Instant::now(),
            current_camera: Camera3D::default(),
            selection_callbacks: Vec::new(),
            deselect_callbacks: Vec::new(),
        }
    }

    /// Update screen dimensions
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.raycaster.set_screen_size(width, height);
        self.mouse_handler.set_screen_size(width, height);
    }

    /// Update camera
    pub fn set_camera(&mut self, camera: &Camera3D) {
        self.current_camera = camera.clone();
        self.raycaster.set_camera(camera);
        self.mouse_handler.update_camera(camera);
    }

    /// Set entity data for raycasting
    pub fn set_entities(&mut self, entities: Vec<(EntityId, Coordinate3D, f64)>) {
        self.entities = entities.clone();
        self.mouse_handler.set_entities(entities);
    }

    /// Handle a window event
    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<InteractionEvent> {
        // Update mouse handler first
        let mouse_result = self.mouse_handler.handle_input(event);

        // Update our selection manager from mouse handler
        self.selection_manager = self.mouse_handler.selection_manager().clone();

        // Handle input events
        if let Some((action, _)) = self.input_handler.handle_event(event) {
            match action {
                InputAction::Deselect => {
                    self.selection_manager.deselect();
                    self.trigger_deselect_callbacks();
                    return Some(InteractionEvent::EntityDeselected);
                }
                InputAction::TrackEntity => {
                    let entity_id = self.selection_manager.get_selected().cloned();
                    if let Some(entity_id) = entity_id {
                        self.selection_manager.track(entity_id.clone());
                        return Some(InteractionEvent::EntityTracked(entity_id.clone()));
                    }
                }
                InputAction::StopTracking => {
                    self.selection_manager.stop_tracking();
                    return Some(InteractionEvent::TrackingStopped);
                }
                InputAction::BookmarkLoad1 => {
                    return Some(InteractionEvent::BookmarkLoadRequest("slot1".to_string()));
                }
                InputAction::BookmarkLoad2 => {
                    return Some(InteractionEvent::BookmarkLoadRequest("slot2".to_string()));
                }
                InputAction::BookmarkLoad3 => {
                    return Some(InteractionEvent::BookmarkLoadRequest("slot3".to_string()));
                }
                InputAction::BookmarkSave1 => {
                    return Some(InteractionEvent::BookmarkSaveRequest("slot1".to_string()));
                }
                InputAction::BookmarkSave2 => {
                    return Some(InteractionEvent::BookmarkSaveRequest("slot2".to_string()));
                }
                InputAction::BookmarkSave3 => {
                    return Some(InteractionEvent::BookmarkSaveRequest("slot3".to_string()));
                }
                _ => {}
            }
        }

        // Check for selection from mouse handler
        if let Some(raycast_result) = mouse_result {
            match &raycast_result {
                raycaster::RaycastResult::Entity { entity_id, .. } => {
                    self.selection_manager.select(entity_id.clone());
                    self.trigger_selection_callbacks(entity_id);
                    return Some(InteractionEvent::EntitySelected(entity_id.clone()));
                }
                raycaster::RaycastResult::EmptySpace { .. } => {
                    self.selection_manager.deselect();
                    self.trigger_deselect_callbacks();
                    return Some(InteractionEvent::EntityDeselected);
                }
                _ => {}
            }
        }

        None
    }

    /// Update the interaction system
    pub fn update(&mut self, delta_time: f32) {
        // Update bookmark manager transitions
        self.bookmark_manager
            .update_transition(delta_time, &mut self.current_camera);
    }

    /// Perform a raycast at a screen position
    pub fn raycast(&self, screen_pos: ScreenPosition) -> RaycastResult {
        self.raycaster.cast_ray(screen_pos, &self.entities)
    }

    /// Get the entity at a screen position
    pub fn get_entity_at(&self, screen_pos: ScreenPosition) -> Option<EntityId> {
        match self.raycaster.cast_ray(screen_pos, &self.entities) {
            RaycastResult::Entity { entity_id, .. } => Some(entity_id),
            _ => None,
        }
    }

    /// Select an entity by ID
    pub fn select_entity(&mut self, entity_id: EntityId) {
        self.selection_manager.select(entity_id.clone());
        self.trigger_selection_callbacks(&entity_id);
    }

    /// Deselect current entity
    pub fn deselect(&mut self) {
        self.selection_manager.deselect();
        self.trigger_deselect_callbacks();
    }

    /// Track an entity
    pub fn track_entity(&mut self, entity_id: EntityId) {
        self.selection_manager.track(entity_id);
    }

    /// Stop tracking
    pub fn stop_tracking(&mut self) {
        self.selection_manager.stop_tracking();
    }

    /// Get currently selected entity
    pub fn selected_entity(&self) -> Option<&EntityId> {
        self.selection_manager.get_selected()
    }

    /// Get currently hovered entity
    pub fn hovered_entity(&self) -> Option<&EntityId> {
        self.selection_manager.get_hovered()
    }

    /// Get currently tracked entity
    pub fn tracked_entity(&self) -> Option<&EntityId> {
        self.selection_manager.get_tracked()
    }

    /// Check if an entity is selected
    pub fn is_selected(&self, entity_id: &EntityId) -> bool {
        self.selection_manager.is_selected(entity_id)
    }

    /// Check if an entity is hovered
    pub fn is_hovered(&self, entity_id: &EntityId) -> bool {
        self.selection_manager.is_hovered(entity_id)
    }

    /// Check if an entity is tracked
    pub fn is_tracked(&self, entity_id: &EntityId) -> bool {
        self.selection_manager.is_tracked(entity_id)
    }

    /// Get selection state for an entity
    pub fn selection_state(&self, entity_id: &EntityId) -> SelectionState {
        self.selection_manager.get_state(entity_id)
    }

    /// Register a callback for entity selection
    pub fn on_select<F>(&mut self, callback: F)
    where
        F: Fn(&EntityId) + Send + Sync + 'static,
    {
        self.selection_callbacks.push(Box::new(callback));
    }

    /// Register a callback for deselection
    pub fn on_deselect<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.deselect_callbacks.push(Box::new(callback));
    }

    /// Trigger selection callbacks
    fn trigger_selection_callbacks(&self, entity_id: &EntityId) {
        for callback in &self.selection_callbacks {
            callback(entity_id);
        }
    }

    /// Trigger deselection callbacks
    fn trigger_deselect_callbacks(&self) {
        for callback in &self.deselect_callbacks {
            callback();
        }
    }

    /// Navigate to a bookmark
    pub fn navigate_to_bookmark(&mut self, bookmark_id: &str) -> bool {
        self.bookmark_manager
            .navigate_to(bookmark_id, &mut self.current_camera)
    }

    /// Save current view as bookmark
    pub fn save_bookmark(&mut self, id: impl Into<String>, name: impl Into<String>) {
        self.bookmark_manager
            .bookmark_current(id, name, &self.current_camera);
    }

    /// Get bookmark manager
    pub fn bookmark_manager(&self) -> &BookmarkManager {
        &self.bookmark_manager
    }

    /// Get mutable bookmark manager
    pub fn bookmark_manager_mut(&mut self) -> &mut BookmarkManager {
        &mut self.bookmark_manager
    }

    /// Get raycaster
    pub fn raycaster(&self) -> &Raycaster3D {
        &self.raycaster
    }

    /// Get input handler
    pub fn input_handler(&self) -> &InputHandler {
        &self.input_handler
    }

    /// Get mutable input handler
    pub fn input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }

    /// Get selection manager
    pub fn selection_manager(&self) -> &SelectionManager {
        &self.selection_manager
    }

    /// Get mouse position
    pub fn mouse_position(&self) -> ScreenPosition {
        self.input_handler.mouse_position()
    }

    /// Check if a key is pressed
    pub fn is_key_pressed(&self, key: &winit::keyboard::Key) -> bool {
        self.input_handler.is_key_pressed(key)
    }

    /// Check if a mouse button is pressed
    pub fn is_mouse_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.input_handler.is_mouse_pressed(button)
    }

    /// Project world position to screen
    pub fn world_to_screen(&self, world_pos: &Coordinate3D) -> Option<ScreenPosition> {
        self.raycaster.world_to_screen(world_pos)
    }

    /// Unproject screen position to world ray
    pub fn screen_to_ray(&self, screen_pos: ScreenPosition) -> Option<raycaster::Ray> {
        self.raycaster.get_ray(screen_pos)
    }

    /// Check if transitioning between bookmarks
    pub fn is_transitioning(&self) -> bool {
        self.bookmark_manager.is_transitioning()
    }

    /// Cancel bookmark transition
    pub fn cancel_transition(&mut self) {
        self.bookmark_manager.cancel_transition();
    }

    /// Get current camera (updated after transitions)
    pub fn current_camera(&self) -> &Camera3D {
        &self.current_camera
    }

    /// Bind an input action to a key
    pub fn bind_input(&mut self, action: InputAction, binding: KeyBinding) {
        self.input_handler.bind(action, binding);
    }

    /// Enable/disable hover detection
    pub fn set_hover_enabled(&mut self, enabled: bool) {
        self.mouse_handler.set_hover_enabled(enabled);
    }

    /// Set selection radius multiplier
    pub fn set_selection_radius_multiplier(&mut self, multiplier: f32) {
        self.raycaster.set_selection_radius_multiplier(multiplier);
    }

    /// Enable/disable raycaster debug mode
    pub fn set_raycaster_debug(&mut self, enabled: bool) {
        self.raycaster.set_debug_mode(enabled);
    }
}

impl Default for InteractionSystem {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Events emitted by the interaction system
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionEvent {
    /// An entity was selected
    EntitySelected(EntityId),
    /// The current entity was deselected
    EntityDeselected,
    /// An entity is now being tracked
    EntityTracked(EntityId),
    /// Tracking was stopped
    TrackingStopped,
    /// Request to load a bookmark
    BookmarkLoadRequest(String),
    /// Request to save a bookmark
    BookmarkSaveRequest(String),
    /// Entity was hovered
    EntityHovered(EntityId),
    /// Hover was cleared
    HoverCleared,
}

/// Builder for InteractionSystem
pub struct InteractionSystemBuilder {
    screen_width: u32,
    screen_height: u32,
    enable_hover: bool,
    selection_radius_multiplier: f32,
    custom_bindings: Vec<(InputAction, KeyBinding)>,
}

impl InteractionSystemBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        InteractionSystemBuilder {
            screen_width: 1920,
            screen_height: 1080,
            enable_hover: true,
            selection_radius_multiplier: 2.0,
            custom_bindings: Vec::new(),
        }
    }

    /// Set screen size
    pub fn with_screen_size(mut self, width: u32, height: u32) -> Self {
        self.screen_width = width;
        self.screen_height = height;
        self
    }

    /// Enable/disable hover detection
    pub fn with_hover(mut self, enabled: bool) -> Self {
        self.enable_hover = enabled;
        self
    }

    /// Set selection radius multiplier
    pub fn with_selection_radius(mut self, multiplier: f32) -> Self {
        self.selection_radius_multiplier = multiplier;
        self
    }

    /// Add a custom key binding
    pub fn with_binding(mut self, action: InputAction, binding: KeyBinding) -> Self {
        self.custom_bindings.push((action, binding));
        self
    }

    /// Build the interaction system
    pub fn build(self) -> InteractionSystem {
        let mut system = InteractionSystem::new(self.screen_width, self.screen_height);

        system.set_hover_enabled(self.enable_hover);
        system.set_selection_radius_multiplier(self.selection_radius_multiplier);

        for (action, binding) in self.custom_bindings {
            system.bind_input(action, binding);
        }

        system
    }
}

impl Default for InteractionSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::camera::Camera2D as Camera3D;

    #[test]
    fn test_interaction_system_new() {
        let system = InteractionSystem::new(1920, 1080);
        assert!(system.selected_entity().is_none());
        assert!(system.hovered_entity().is_none());
        assert!(!system.is_transitioning());
    }

    #[test]
    fn test_interaction_system_default() {
        let system = InteractionSystem::default();
        assert!(system.selected_entity().is_none());
    }

    #[test]
    fn test_interaction_system_select_entity() {
        let mut system = InteractionSystem::new(1920, 1080);
        let entity_id = EntityId::new("1".to_string());

        system.select_entity(entity_id.clone());
        assert_eq!(system.selected_entity(), Some(&entity_id));
        assert!(system.is_selected(&entity_id));
    }

    #[test]
    fn test_interaction_system_deselect() {
        let mut system = InteractionSystem::new(1920, 1080);
        let entity_id = EntityId::new("1".to_string());

        system.select_entity(entity_id.clone());
        assert!(system.selected_entity().is_some());

        system.deselect();
        assert!(system.selected_entity().is_none());
    }

    #[test]
    fn test_interaction_system_track_entity() {
        let mut system = InteractionSystem::new(1920, 1080);
        let entity_id = EntityId::new("1".to_string());

        system.track_entity(entity_id.clone());
        assert_eq!(system.tracked_entity(), Some(&entity_id));
        assert!(system.is_tracked(&entity_id));
    }

    #[test]
    fn test_interaction_system_stop_tracking() {
        let mut system = InteractionSystem::new(1920, 1080);
        let entity_id = EntityId::new("1".to_string());

        system.track_entity(entity_id.clone());
        system.stop_tracking();
        assert!(system.tracked_entity().is_none());
    }

    #[test]
    fn test_interaction_system_selection_state() {
        let mut system = InteractionSystem::new(1920, 1080);
        let entity_id = EntityId::new("1".to_string());

        assert_eq!(system.selection_state(&entity_id), SelectionState::None);

        system.select_entity(entity_id.clone());
        assert_eq!(system.selection_state(&entity_id), SelectionState::Selected);

        system.track_entity(entity_id.clone());
        assert_eq!(system.selection_state(&entity_id), SelectionState::Tracked);
    }

    #[test]
    fn test_interaction_system_set_screen_size() {
        let mut system = InteractionSystem::new(1920, 1080);
        system.set_screen_size(1280, 720);

        // Should not panic
    }

    #[test]
    fn test_interaction_system_set_camera() {
        let mut system = InteractionSystem::new(1920, 1080);
        let camera = Camera3D::default();

        system.set_camera(&camera);

        // Should not panic
    }

    #[test]
    fn test_interaction_system_bookmark_save_and_navigate() {
        let mut system = InteractionSystem::new(1920, 1080);
        let camera = Camera3D::default();

        system.set_camera(&camera);
        system.save_bookmark("test", "Test Bookmark");

        assert!(system.bookmark_manager().has_bookmark("test"));

        // Navigate (should start transition)
        let result = system.navigate_to_bookmark("test");
        assert!(result);
    }

    #[test]
    fn test_interaction_system_builder() {
        let system = InteractionSystemBuilder::new()
            .with_screen_size(1280, 720)
            .with_hover(true)
            .with_selection_radius(3.0)
            .build();

        // Should not panic
        assert!(system.selected_entity().is_none());
    }

    #[test]
    fn test_interaction_event_equality() {
        let entity1 = EntityId::new("1".to_string());
        let entity2 = EntityId::new("2".to_string());

        assert_eq!(
            InteractionEvent::EntitySelected(entity1.clone()),
            InteractionEvent::EntitySelected(entity1.clone())
        );

        assert_ne!(
            InteractionEvent::EntitySelected(entity1.clone()),
            InteractionEvent::EntitySelected(entity2.clone())
        );

        assert_eq!(
            InteractionEvent::BookmarkLoadRequest("test".to_string()),
            InteractionEvent::BookmarkLoadRequest("test".to_string())
        );
    }

    #[test]
    fn test_interaction_system_world_to_screen() {
        let system = InteractionSystem::new(1920, 1080);
        let world_pos = Coordinate3D::new(0.0, 0.0, 0.0);

        // Without camera set, world_to_screen uses identity matrices
        // Returns Some with transformed coordinates
        let result = system.world_to_screen(&world_pos);
        assert!(result.is_some());
    }

    #[test]
    fn test_interaction_system_callback_registration() {
        let mut system = InteractionSystem::new(1920, 1080);

        // Should not panic when registering callbacks
        system.on_select(|_entity_id| {
            // Selection callback
        });

        system.on_deselect(|| {
            // Deselection callback
        });
    }
}
