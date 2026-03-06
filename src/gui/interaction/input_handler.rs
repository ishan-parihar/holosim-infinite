//! Input Handler - Unified Input System for Mouse, Keyboard, and Gamepad
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 5 Week 10:
//! "Input handling - Unified input system, Mouse, keyboard, gamepad support, Input remapping"
//!
//! This module provides:
//! - Unified input event handling
//! - Keyboard shortcuts and hotkeys
//! - Mouse interaction (click, drag, scroll)
//! - Gamepad support (optional)
//! - Input remapping and configuration

use crate::entity_layer7::layer7::EntityId;
use crate::gui::interaction::raycaster::{RaycastResult, Raycaster3D, SelectionManager};
use crate::gui::{Coordinate3D, ScreenPosition};
use std::collections::HashMap;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, NamedKey},
};

/// Type of input action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    // Camera controls
    CameraPanLeft,
    CameraPanRight,
    CameraPanUp,
    CameraPanDown,
    CameraZoomIn,
    CameraZoomOut,
    CameraRotateLeft,
    CameraRotateRight,
    CameraReset,

    // Selection
    SelectEntity,
    Deselect,
    SelectPrevious,
    TrackEntity,
    StopTracking,

    // Simulation
    PauseResume,
    StepForward,
    StepBackward,
    SpeedUp,
    SpeedDown,

    // UI
    ToggleInspector,
    ToggleSpectrum,
    ToggleEmergence,
    ToggleTimeControls,
    ToggleFullscreen,
    ToggleDebug,

    // Bookmarks
    BookmarkSave1,
    BookmarkSave2,
    BookmarkSave3,
    BookmarkLoad1,
    BookmarkLoad2,
    BookmarkLoad3,

    // General
    Escape,
    Help,
}

impl InputAction {
    /// Get the default key binding
    pub fn default_key(&self) -> Option<KeyBinding> {
        match self {
            // Camera
            InputAction::CameraPanLeft => Some(KeyBinding::Key(Key::Named(NamedKey::ArrowLeft))),
            InputAction::CameraPanRight => Some(KeyBinding::Key(Key::Named(NamedKey::ArrowRight))),
            InputAction::CameraPanUp => Some(KeyBinding::Key(Key::Named(NamedKey::ArrowUp))),
            InputAction::CameraPanDown => Some(KeyBinding::Key(Key::Named(NamedKey::ArrowDown))),
            InputAction::CameraZoomIn => Some(KeyBinding::Key(Key::Character("+".into()))),
            InputAction::CameraZoomOut => Some(KeyBinding::Key(Key::Character("-".into()))),
            InputAction::CameraRotateLeft => Some(KeyBinding::Key(Key::Character("q".into()))),
            InputAction::CameraRotateRight => Some(KeyBinding::Key(Key::Character("e".into()))),
            InputAction::CameraReset => Some(KeyBinding::Key(Key::Named(NamedKey::Home))),

            // Selection
            InputAction::SelectEntity => Some(KeyBinding::Mouse(MouseButton::Left)),
            InputAction::Deselect => Some(KeyBinding::Key(Key::Named(NamedKey::Escape))),
            InputAction::SelectPrevious => Some(KeyBinding::KeyWithModifiers(
                Key::Named(NamedKey::Tab),
                Modifiers::SHIFT,
            )),
            InputAction::TrackEntity => Some(KeyBinding::Key(Key::Character("t".into()))),
            InputAction::StopTracking => Some(KeyBinding::KeyWithModifiers(
                Key::Character("t".into()),
                Modifiers::SHIFT,
            )),

            // Simulation
            InputAction::PauseResume => Some(KeyBinding::Key(Key::Named(NamedKey::Space))),
            InputAction::StepForward => Some(KeyBinding::Key(Key::Character(".".into()))),
            InputAction::StepBackward => Some(KeyBinding::Key(Key::Character(",".into()))),
            InputAction::SpeedUp => Some(KeyBinding::Key(Key::Character(">".into()))),
            InputAction::SpeedDown => Some(KeyBinding::Key(Key::Character("<".into()))),

            // UI
            InputAction::ToggleInspector => Some(KeyBinding::Key(Key::Character("i".into()))),
            InputAction::ToggleSpectrum => Some(KeyBinding::Key(Key::Character("s".into()))),
            InputAction::ToggleEmergence => Some(KeyBinding::Key(Key::Character("g".into()))),
            InputAction::ToggleTimeControls => Some(KeyBinding::Key(Key::Character("c".into()))),
            InputAction::ToggleFullscreen => Some(KeyBinding::Key(Key::Named(NamedKey::F11))),
            InputAction::ToggleDebug => Some(KeyBinding::Key(Key::Character("d".into()))),

            // Bookmarks
            InputAction::BookmarkSave1 => Some(KeyBinding::KeyWithModifiers(
                Key::Character("1".into()),
                Modifiers::CTRL,
            )),
            InputAction::BookmarkSave2 => Some(KeyBinding::KeyWithModifiers(
                Key::Character("2".into()),
                Modifiers::CTRL,
            )),
            InputAction::BookmarkSave3 => Some(KeyBinding::KeyWithModifiers(
                Key::Character("3".into()),
                Modifiers::CTRL,
            )),
            InputAction::BookmarkLoad1 => Some(KeyBinding::Key(Key::Character("1".into()))),
            InputAction::BookmarkLoad2 => Some(KeyBinding::Key(Key::Character("2".into()))),
            InputAction::BookmarkLoad3 => Some(KeyBinding::Key(Key::Character("3".into()))),

            // General
            InputAction::Escape => Some(KeyBinding::Key(Key::Named(NamedKey::Escape))),
            InputAction::Help => Some(KeyBinding::Key(Key::Character("?".into()))),
        }
    }

    /// Get a description of the action
    pub fn description(&self) -> &'static str {
        match self {
            InputAction::CameraPanLeft => "Pan camera left",
            InputAction::CameraPanRight => "Pan camera right",
            InputAction::CameraPanUp => "Pan camera up",
            InputAction::CameraPanDown => "Pan camera down",
            InputAction::CameraZoomIn => "Zoom in",
            InputAction::CameraZoomOut => "Zoom out",
            InputAction::CameraRotateLeft => "Rotate camera left",
            InputAction::CameraRotateRight => "Rotate camera right",
            InputAction::CameraReset => "Reset camera to origin",
            InputAction::SelectEntity => "Select entity under cursor",
            InputAction::Deselect => "Deselect current entity",
            InputAction::SelectPrevious => "Select previous entity",
            InputAction::TrackEntity => "Track selected entity",
            InputAction::StopTracking => "Stop tracking entity",
            InputAction::PauseResume => "Pause/Resume simulation",
            InputAction::StepForward => "Step simulation forward",
            InputAction::StepBackward => "Step simulation backward",
            InputAction::SpeedUp => "Increase simulation speed",
            InputAction::SpeedDown => "Decrease simulation speed",
            InputAction::ToggleInspector => "Toggle entity inspector",
            InputAction::ToggleSpectrum => "Toggle spectrum panel",
            InputAction::ToggleEmergence => "Toggle emergence panel",
            InputAction::ToggleTimeControls => "Toggle time controls",
            InputAction::ToggleFullscreen => "Toggle fullscreen",
            InputAction::ToggleDebug => "Toggle debug overlay",
            InputAction::BookmarkSave1 => "Save bookmark 1",
            InputAction::BookmarkSave2 => "Save bookmark 2",
            InputAction::BookmarkSave3 => "Save bookmark 3",
            InputAction::BookmarkLoad1 => "Load bookmark 1",
            InputAction::BookmarkLoad2 => "Load bookmark 2",
            InputAction::BookmarkLoad3 => "Load bookmark 3",
            InputAction::Escape => "Exit/Back",
            InputAction::Help => "Show help",
        }
    }
}

/// Modifier keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

impl Modifiers {
    pub const NONE: Modifiers = Modifiers {
        shift: false,
        ctrl: false,
        alt: false,
        super_key: false,
    };

    pub const SHIFT: Modifiers = Modifiers {
        shift: true,
        ctrl: false,
        alt: false,
        super_key: false,
    };

    pub const CTRL: Modifiers = Modifiers {
        shift: false,
        ctrl: true,
        alt: false,
        super_key: false,
    };

    pub const ALT: Modifiers = Modifiers {
        shift: false,
        ctrl: false,
        alt: true,
        super_key: false,
    };

    pub fn new(shift: bool, ctrl: bool, alt: bool, super_key: bool) -> Self {
        Modifiers {
            shift,
            ctrl,
            alt,
            super_key,
        }
    }

    pub fn from_winit(state: &winit::keyboard::ModifiersState) -> Self {
        Modifiers {
            shift: state.shift_key(),
            ctrl: state.control_key(),
            alt: state.alt_key(),
            super_key: state.super_key(),
        }
    }
}

impl Default for Modifiers {
    fn default() -> Self {
        Modifiers::NONE
    }
}

/// Key binding definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyBinding {
    /// Simple key press
    Key(Key),
    /// Key with modifiers
    KeyWithModifiers(Key, Modifiers),
    /// Mouse button
    Mouse(MouseButton),
    /// Mouse button with modifiers
    MouseWithModifiers(MouseButton, Modifiers),
}

/// Input event type
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Key pressed
    KeyPressed { key: Key, modifiers: Modifiers },
    /// Key released
    KeyReleased { key: Key, modifiers: Modifiers },
    /// Mouse button pressed
    MousePressed {
        button: MouseButton,
        position: ScreenPosition,
        modifiers: Modifiers,
    },
    /// Mouse button released
    MouseReleased {
        button: MouseButton,
        position: ScreenPosition,
        modifiers: Modifiers,
    },
    /// Mouse moved
    MouseMoved { position: ScreenPosition },
    /// Mouse wheel scrolled
    MouseScrolled {
        delta: MouseScrollDelta,
        modifiers: Modifiers,
    },
}

/// Type alias for input event callback
type InputCallback = Box<dyn Fn(&InputEvent) + Send + Sync>;

/// Input handler for unified input management
pub struct InputHandler {
    /// Current key bindings
    bindings: HashMap<KeyBinding, InputAction>,
    /// Reverse lookup for action to binding
    action_bindings: HashMap<InputAction, KeyBinding>,
    /// Currently pressed keys
    pressed_keys: HashMap<Key, bool>,
    /// Currently pressed mouse buttons
    pressed_buttons: HashMap<MouseButton, bool>,
    /// Current mouse position
    mouse_position: ScreenPosition,
    /// Previous mouse position
    last_mouse_position: ScreenPosition,
    /// Is dragging
    is_dragging: bool,
    /// Drag start position
    drag_start: Option<ScreenPosition>,
    /// Current modifiers
    modifiers: Modifiers,
    /// Input callbacks
    callbacks: HashMap<InputAction, Vec<InputCallback>>,
    /// Enable debug output
    debug_mode: bool,
}

impl InputHandler {
    /// Create a new input handler with default bindings
    pub fn new() -> Self {
        let mut handler = InputHandler {
            bindings: HashMap::new(),
            action_bindings: HashMap::new(),
            pressed_keys: HashMap::new(),
            pressed_buttons: HashMap::new(),
            mouse_position: ScreenPosition::new(0.0, 0.0),
            last_mouse_position: ScreenPosition::new(0.0, 0.0),
            is_dragging: false,
            drag_start: None,
            modifiers: Modifiers::NONE,
            callbacks: HashMap::new(),
            debug_mode: false,
        };

        // Load default bindings
        handler.load_default_bindings();
        handler
    }

    /// Load default key bindings
    pub fn load_default_bindings(&mut self) {
        self.bindings.clear();
        self.action_bindings.clear();

        for action in [
            InputAction::CameraPanLeft,
            InputAction::CameraPanRight,
            InputAction::CameraPanUp,
            InputAction::CameraPanDown,
            InputAction::CameraZoomIn,
            InputAction::CameraZoomOut,
            InputAction::CameraRotateLeft,
            InputAction::CameraRotateRight,
            InputAction::CameraReset,
            InputAction::SelectEntity,
            InputAction::Deselect,
            InputAction::SelectPrevious,
            InputAction::TrackEntity,
            InputAction::StopTracking,
            InputAction::PauseResume,
            InputAction::StepForward,
            InputAction::StepBackward,
            InputAction::SpeedUp,
            InputAction::SpeedDown,
            InputAction::ToggleInspector,
            InputAction::ToggleSpectrum,
            InputAction::ToggleEmergence,
            InputAction::ToggleTimeControls,
            InputAction::ToggleFullscreen,
            InputAction::ToggleDebug,
            InputAction::BookmarkSave1,
            InputAction::BookmarkSave2,
            InputAction::BookmarkSave3,
            InputAction::BookmarkLoad1,
            InputAction::BookmarkLoad2,
            InputAction::BookmarkLoad3,
            InputAction::Escape,
            InputAction::Help,
        ] {
            if let Some(binding) = action.default_key() {
                self.bindings.insert(binding.clone(), action);
                self.action_bindings.insert(action, binding);
            }
        }
    }

    /// Bind an action to a key
    pub fn bind(&mut self, action: InputAction, binding: KeyBinding) {
        // Remove existing binding for this action
        if let Some(old_binding) = self.action_bindings.remove(&action) {
            self.bindings.remove(&old_binding);
        }

        // Remove existing action for this binding
        if let Some(old_action) = self.bindings.remove(&binding) {
            self.action_bindings.remove(&old_action);
        }

        // Insert new binding
        self.bindings.insert(binding.clone(), action);
        self.action_bindings.insert(action, binding);
    }

    /// Unbind an action
    pub fn unbind(&mut self, action: InputAction) {
        if let Some(binding) = self.action_bindings.remove(&action) {
            self.bindings.remove(&binding);
        }
    }

    /// Get binding for an action
    pub fn get_binding(&self, action: InputAction) -> Option<&KeyBinding> {
        self.action_bindings.get(&action)
    }

    /// Get action for a binding
    pub fn get_action(&self, binding: &KeyBinding) -> Option<InputAction> {
        self.bindings.get(binding).copied()
    }

    /// Register a callback for an action
    pub fn register_callback<F>(&mut self, action: InputAction, callback: F)
    where
        F: Fn(&InputEvent) + Send + Sync + 'static,
    {
        self.callbacks
            .entry(action)
            .or_default()
            .push(Box::new(callback));
    }

    /// Handle a winit window event
    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<(InputAction, InputEvent)> {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state, logical_key, ..
                },
                ..
            } => {
                let event = if *state == ElementState::Pressed {
                    InputEvent::KeyPressed {
                        key: logical_key.clone(),
                        modifiers: self.modifiers,
                    }
                } else {
                    InputEvent::KeyReleased {
                        key: logical_key.clone(),
                        modifiers: self.modifiers,
                    }
                };

                // Update pressed state
                if *state == ElementState::Pressed {
                    self.pressed_keys.insert(logical_key.clone(), true);
                } else {
                    self.pressed_keys.remove(logical_key);
                }

                // Look up action
                let binding = if self.modifiers == Modifiers::NONE {
                    KeyBinding::Key(logical_key.clone())
                } else {
                    KeyBinding::KeyWithModifiers(logical_key.clone(), self.modifiers)
                };

                let action = self.bindings.get(&binding).copied();

                if self.debug_mode {
                    if let Some(action) = action {
                        println!("Input: {:?} -> {:?}", binding, action);
                    }
                }

                action.map(|a| {
                    self.trigger_callbacks(a, &event);
                    (a, event)
                })
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let event = if *state == ElementState::Pressed {
                    InputEvent::MousePressed {
                        button: *button,
                        position: self.mouse_position,
                        modifiers: self.modifiers,
                    }
                } else {
                    InputEvent::MouseReleased {
                        button: *button,
                        position: self.mouse_position,
                        modifiers: self.modifiers,
                    }
                };

                // Update pressed state
                if *state == ElementState::Pressed {
                    self.pressed_buttons.insert(*button, true);

                    // Start drag if left button
                    if *button == MouseButton::Left {
                        self.is_dragging = true;
                        self.drag_start = Some(self.mouse_position);
                    }
                } else {
                    self.pressed_buttons.remove(button);

                    // End drag if left button
                    if *button == MouseButton::Left {
                        self.is_dragging = false;
                        self.drag_start = None;
                    }
                }

                // Look up action
                let binding = if self.modifiers == Modifiers::NONE {
                    KeyBinding::Mouse(*button)
                } else {
                    KeyBinding::MouseWithModifiers(*button, self.modifiers)
                };

                let action = self.bindings.get(&binding).copied();

                if self.debug_mode {
                    if let Some(action) = action {
                        println!("Input: {:?} -> {:?}", binding, action);
                    }
                }

                action.map(|a| {
                    self.trigger_callbacks(a, &event);
                    (a, event)
                })
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_position = self.mouse_position;
                self.mouse_position = ScreenPosition::new(position.x as f32, position.y as f32);

                let _event = InputEvent::MouseMoved {
                    position: self.mouse_position,
                };

                None // No action for mouse movement
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let _event = InputEvent::MouseScrolled {
                    delta: *delta,
                    modifiers: self.modifiers,
                };

                None // No action for scroll (handled directly)
            }

            _ => None,
        }
    }

    /// Trigger callbacks for an action
    fn trigger_callbacks(&self, action: InputAction, event: &InputEvent) {
        if let Some(callbacks) = self.callbacks.get(&action) {
            for callback in callbacks {
                callback(event);
            }
        }
    }

    /// Check if a key is pressed
    pub fn is_key_pressed(&self, key: &Key) -> bool {
        self.pressed_keys.get(key).copied().unwrap_or(false)
    }

    /// Check if a mouse button is pressed
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.get(&button).copied().unwrap_or(false)
    }

    /// Get current mouse position
    pub fn mouse_position(&self) -> ScreenPosition {
        self.mouse_position
    }

    /// Get last mouse position
    pub fn last_mouse_position(&self) -> ScreenPosition {
        self.last_mouse_position
    }

    /// Check if dragging
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Get drag delta
    pub fn drag_delta(&self) -> Option<(f32, f32)> {
        self.drag_start.map(|start| {
            (
                self.mouse_position.x - start.x,
                self.mouse_position.y - start.y,
            )
        })
    }

    /// Get current modifiers
    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    /// Enable/disable debug mode
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    /// Get all bindings as a list
    pub fn get_all_bindings(&self) -> Vec<(InputAction, KeyBinding)> {
        self.action_bindings
            .iter()
            .map(|(action, binding)| (*action, binding.clone()))
            .collect()
    }

    /// Clear all inputs
    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.pressed_buttons.clear();
        self.is_dragging = false;
        self.drag_start = None;
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Mouse interaction handler for entity selection
pub struct MouseInteractionHandler {
    /// Input handler reference
    input_handler: InputHandler,
    /// Raycaster for entity picking
    raycaster: Raycaster3D,
    /// Selection manager
    selection_manager: SelectionManager,
    /// Entity data for raycasting
    entities: Vec<(EntityId, Coordinate3D, f64)>,
    /// Hover enabled
    hover_enabled: bool,
    /// Click debounce timer
    last_click_time: std::time::Instant,
    /// Minimum time between clicks
    click_debounce_ms: u64,
}

impl MouseInteractionHandler {
    /// Create a new mouse interaction handler
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        MouseInteractionHandler {
            input_handler: InputHandler::new(),
            raycaster: Raycaster3D::new(screen_width, screen_height),
            selection_manager: SelectionManager::new(),
            entities: Vec::new(),
            hover_enabled: true,
            last_click_time: std::time::Instant::now(),
            click_debounce_ms: 100,
        }
    }

    /// Set entity data for raycasting
    pub fn set_entities(&mut self, entities: Vec<(EntityId, Coordinate3D, f64)>) {
        self.entities = entities;
    }

    /// Update screen size
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.raycaster.set_screen_size(width, height);
    }

    /// Update camera
    pub fn update_camera(&mut self, camera: &crate::gui::camera::camera::Camera) {
        self.raycaster.set_camera(camera);
    }

    /// Handle input and return any selection changes
    pub fn handle_input(&mut self, event: &WindowEvent) -> Option<RaycastResult> {
        // Handle the event
        if let Some((action, input_event)) = self.input_handler.handle_event(event) {
            match action {
                InputAction::SelectEntity => {
                    if let InputEvent::MousePressed { position, .. } = input_event {
                        // Debounce clicks
                        let now = std::time::Instant::now();
                        if now.duration_since(self.last_click_time).as_millis()
                            < self.click_debounce_ms as u128
                        {
                            return None;
                        }
                        self.last_click_time = now;

                        // Perform raycast
                        let result = self.raycaster.cast_ray(position, &self.entities);

                        // Update selection
                        match &result {
                            RaycastResult::Entity { entity_id, .. } => {
                                self.selection_manager.select(entity_id.clone());
                            }
                            RaycastResult::EmptySpace { .. } => {
                                self.selection_manager.deselect();
                            }
                            _ => {}
                        }

                        return Some(result);
                    }
                }

                InputAction::Deselect => {
                    self.selection_manager.deselect();
                    return Some(RaycastResult::OutOfBounds);
                }

                InputAction::TrackEntity => {
                    if let Some(entity_id) = self.selection_manager.get_selected() {
                        self.selection_manager.track(entity_id.clone());
                    }
                }

                InputAction::StopTracking => {
                    self.selection_manager.stop_tracking();
                }

                InputAction::SelectPrevious => {
                    if let Some(entity_id) = self.selection_manager.select_previous() {
                        return Some(RaycastResult::Entity {
                            entity_id,
                            hit_distance: 0.0,
                            world_position: Coordinate3D::origin(),
                        });
                    }
                }

                _ => {}
            }
        }

        // Handle hover
        if self.hover_enabled {
            if let WindowEvent::CursorMoved { position, .. } = event {
                let screen_pos = ScreenPosition::new(position.x as f32, position.y as f32);
                let result = self.raycaster.cast_ray(screen_pos, &self.entities);

                match &result {
                    RaycastResult::Entity { entity_id, .. } => {
                        self.selection_manager.set_hovered(Some(entity_id.clone()));
                    }
                    _ => {
                        self.selection_manager.set_hovered(None);
                    }
                }
            }
        }

        None
    }

    /// Get selection manager
    pub fn selection_manager(&self) -> &SelectionManager {
        &self.selection_manager
    }

    /// Get mutable selection manager
    pub fn selection_manager_mut(&mut self) -> &mut SelectionManager {
        &mut self.selection_manager
    }

    /// Get input handler
    pub fn input_handler(&self) -> &InputHandler {
        &self.input_handler
    }

    /// Get mutable input handler
    pub fn input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }

    /// Get raycaster
    pub fn raycaster(&self) -> &Raycaster3D {
        &self.raycaster
    }

    /// Get current mouse position
    pub fn mouse_position(&self) -> ScreenPosition {
        self.input_handler.mouse_position()
    }

    /// Enable/disable hover
    pub fn set_hover_enabled(&mut self, enabled: bool) {
        self.hover_enabled = enabled;
    }

    /// Check if hover is enabled
    pub fn hover_enabled(&self) -> bool {
        self.hover_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_action_default_key() {
        assert!(InputAction::CameraPanLeft.default_key().is_some());
        assert!(InputAction::SelectEntity.default_key().is_some());
        assert!(InputAction::PauseResume.default_key().is_some());
    }

    #[test]
    fn test_modifiers() {
        assert_eq!(Modifiers::NONE, Modifiers::new(false, false, false, false));
        assert_eq!(Modifiers::SHIFT, Modifiers::new(true, false, false, false));
        assert_eq!(Modifiers::CTRL, Modifiers::new(false, true, false, false));
    }

    #[test]
    fn test_input_handler_new() {
        let handler = InputHandler::new();
        assert!(handler.pressed_keys.is_empty());
        assert!(handler.pressed_buttons.is_empty());
        assert!(!handler.is_dragging);
    }

    #[test]
    fn test_input_handler_bind() {
        let mut handler = InputHandler::new();
        let action = InputAction::CameraReset;
        let binding = KeyBinding::Key(Key::Named(NamedKey::Home));

        handler.bind(action, binding.clone());
        assert_eq!(handler.get_action(&binding), Some(action));
        assert_eq!(handler.get_binding(action), Some(&binding));
    }

    #[test]
    fn test_input_handler_unbind() {
        let mut handler = InputHandler::new();
        let action = InputAction::CameraReset;
        let binding = KeyBinding::Key(Key::Named(NamedKey::Home));

        handler.bind(action, binding.clone());
        handler.unbind(action);

        assert!(handler.get_binding(action).is_none());
        assert!(handler.get_action(&binding).is_none());
    }

    #[test]
    fn test_selection_manager() {
        let mut manager = SelectionManager::new();
        let entity_id = EntityId::new("1".to_string());

        manager.select(entity_id.clone());
        assert_eq!(manager.get_selected(), Some(&entity_id));
        assert!(manager.is_selected(&entity_id));

        manager.deselect();
        assert!(manager.get_selected().is_none());
        assert!(!manager.is_selected(&entity_id));
    }

    #[test]
    fn test_mouse_interaction_handler_new() {
        let handler = MouseInteractionHandler::new(1920, 1080);
        assert!(handler.hover_enabled());
        assert!(handler.selection_manager().get_selected().is_none());
    }
}
