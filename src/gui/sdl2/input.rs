//! SDL2 Input Module
//!
//! Provides input state tracking and mapping for keyboard, mouse, and gamepad.
//! Replaces Winit input handling while providing enhanced gamepad support.
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 2:
//! "Implement comprehensive input handling with gamepad support"

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::event_loop::{GamepadAxis, GamepadButton, Modifiers, WindowEvent};

/// Normalized Device Coordinates (NDC)
///
/// Coordinates in range [-1, 1] where:
/// - x: -1 (left) to 1 (right)
/// - y: -1 (bottom) to 1 (top)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NdcPosition {
    pub x: f32,
    pub y: f32,
}

impl NdcPosition {
    pub fn new(x: f32, y: f32) -> Self {
        NdcPosition { x, y }
    }

    pub fn from_screen(screen_x: f32, screen_y: f32, width: u32, height: u32) -> Self {
        let ndc_x = (2.0 * screen_x / width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / height as f32); // Flip Y for SDL2
        NdcPosition { x: ndc_x, y: ndc_y }
    }

    pub fn to_screen(&self, width: u32, height: u32) -> (f32, f32) {
        let screen_x = ((self.x + 1.0) / 2.0) * width as f32;
        let screen_y = ((1.0 - self.y) / 2.0) * height as f32;
        (screen_x, screen_y)
    }
}

/// World position in 3D space
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        WorldPosition { x, y, z }
    }

    pub fn origin() -> Self {
        WorldPosition {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Mouse drag state
#[derive(Debug, Clone, PartialEq)]
pub enum DragState {
    /// Not dragging
    Idle,
    /// Dragging with start position
    Dragging {
        button: MouseButton,
        start_x: i32,
        start_y: i32,
        current_x: i32,
        current_y: i32,
    },
}

impl Default for DragState {
    fn default() -> Self {
        DragState::Idle
    }
}

impl DragState {
    pub fn is_dragging(&self) -> bool {
        matches!(self, DragState::Dragging { .. })
    }

    pub fn get_button(&self) -> Option<MouseButton> {
        match self {
            DragState::Dragging { button, .. } => Some(*button),
            _ => None,
        }
    }

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            DragState::Dragging {
                start_x,
                start_y,
                current_x,
                current_y,
                button: _,
            } => (*current_x - *start_x, *current_y - *start_y),
            _ => (0, 0),
        }
    }

    pub fn reset(&mut self) {
        *self = DragState::Idle;
    }
}

/// Click detection state
#[derive(Debug, Clone)]
pub struct ClickDetector {
    /// Click start time
    click_start_time: Option<Instant>,
    /// Click start position
    click_start_pos: Option<(i32, i32)>,
    /// Maximum time for a click (ms)
    max_click_duration: u64,
    /// Maximum movement for a click (pixels)
    max_click_movement: i32,
    /// Double click detection
    last_click_time: Option<Instant>,
    last_click_pos: Option<(i32, i32)>,
    /// Maximum time between double clicks (ms)
    max_double_click_duration: u64,
    /// Maximum movement between double clicks (pixels)
    max_double_click_movement: i32,
}

impl Default for ClickDetector {
    fn default() -> Self {
        ClickDetector {
            click_start_time: None,
            click_start_pos: None,
            max_click_duration: 300,
            max_click_movement: 5,
            last_click_time: None,
            last_click_pos: None,
            max_double_click_duration: 500,
            max_double_click_movement: 10,
        }
    }
}

impl ClickDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_click_duration(mut self, duration_ms: u64) -> Self {
        self.max_click_duration = duration_ms;
        self
    }

    pub fn with_max_click_movement(mut self, movement_pixels: i32) -> Self {
        self.max_click_movement = movement_pixels;
        self
    }

    pub fn with_max_double_click_duration(mut self, duration_ms: u64) -> Self {
        self.max_double_click_duration = duration_ms;
        self
    }

    pub fn with_max_double_click_movement(mut self, movement_pixels: i32) -> Self {
        self.max_double_click_movement = movement_pixels;
        self
    }

    pub fn on_mouse_down(&mut self, x: i32, y: i32) {
        self.click_start_time = Some(Instant::now());
        self.click_start_pos = Some((x, y));
    }

    pub fn on_mouse_up(&mut self, x: i32, y: i32) -> ClickResult {
        let result = if let (Some(start_time), Some(start_pos)) =
            (self.click_start_time, self.click_start_pos)
        {
            let duration = start_time.elapsed();
            let dx = (x - start_pos.0).abs();
            let dy = (y - start_pos.1).abs();
            let movement = dx.max(dy);

            if duration.as_millis() as u64 <= self.max_click_duration
                && movement <= self.max_click_movement
            {
                // Valid click - check for double click
                let is_double_click = if let (Some(last_time), Some(last_pos)) =
                    (self.last_click_time, self.last_click_pos)
                {
                    let time_since = last_time.elapsed();
                    let dx = (x - last_pos.0).abs();
                    let dy = (y - last_pos.1).abs();
                    let movement = dx.max(dy);

                    time_since.as_millis() as u64 <= self.max_double_click_duration
                        && movement <= self.max_double_click_movement
                } else {
                    false
                };

                self.last_click_time = Some(Instant::now());
                self.last_click_pos = Some((x, y));

                ClickResult::Click {
                    x,
                    y,
                    is_double: is_double_click,
                }
            } else {
                ClickResult::None
            }
        } else {
            ClickResult::None
        };

        self.click_start_time = None;
        self.click_start_pos = None;

        result
    }

    pub fn reset(&mut self) {
        self.click_start_time = None;
        self.click_start_pos = None;
        self.last_click_time = None;
        self.last_click_pos = None;
    }
}

/// Result of a click detection
#[derive(Debug, Clone, PartialEq)]
pub enum ClickResult {
    /// No click (drag or too long)
    None,
    /// Single or double click
    Click { x: i32, y: i32, is_double: bool },
}

/// Gamepad state
#[derive(Debug, Clone)]
pub struct GamepadState {
    /// Gamepad instance ID (from SDL2)
    pub which: u32,
    /// Currently pressed buttons
    pub pressed_buttons: HashSet<GamepadButton>,
    /// Axis values (normalized to [-1.0, 1.0])
    pub axis_values: HashMap<GamepadAxis, f32>,
    /// Connection state
    pub connected: bool,
}

impl GamepadState {
    pub fn new(which: u32) -> Self {
        GamepadState {
            which,
            pressed_buttons: HashSet::new(),
            axis_values: HashMap::new(),
            connected: true,
        }
    }

    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        *self.axis_values.get(&axis).unwrap_or(&0.0)
    }

    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        self.pressed_buttons.contains(&button)
    }
}

/// Input state tracker
///
/// Maintains current state of all input devices and provides
/// query methods for checking input state.
#[derive(Debug, Clone)]
pub struct InputState {
    /// Currently pressed keys
    pressed_keys: HashSet<Keycode>,

    /// Currently pressed mouse buttons
    pressed_mouse_buttons: HashSet<MouseButton>,

    /// Current mouse position (screen coordinates)
    mouse_position: (i32, i32),

    /// Mouse delta from last frame
    mouse_delta: (i32, i32),

    /// Current modifier keys
    modifiers: Modifiers,

    /// Mouse wheel scroll delta
    scroll_delta: (i32, i32),

    /// Key repeat tracking: key -> repeat count
    key_repeat_counts: HashMap<Keycode, u32>,

    /// Text input buffer (for typing)
    text_input: String,

    /// Enable/disable key repeat
    key_repeat_enabled: bool,

    /// Screen dimensions
    screen_size: (u32, u32),

    /// Normalized Device Coordinates
    ndc_position: NdcPosition,

    /// Mouse drag state
    drag_state: DragState,

    /// Click detector
    click_detector: ClickDetector,

    /// World position (requires camera to be set)
    world_position: WorldPosition,

    /// Mouse wheel accumulated scroll (for smooth scrolling)
    accumulated_scroll: (f32, f32),

    /// Connected gamepads: instance ID -> gamepad state
    gamepads: HashMap<u32, GamepadState>,

    /// Currently active gamepad (for single-player scenarios)
    active_gamepad: Option<u32>,

    /// Window has input focus
    window_focused: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            mouse_position: (0, 0),
            mouse_delta: (0, 0),
            modifiers: Modifiers::default(),
            scroll_delta: (0, 0),
            key_repeat_counts: HashMap::new(),
            text_input: String::new(),
            key_repeat_enabled: true,
            screen_size: (1920, 1080),
            ndc_position: NdcPosition::default(),
            drag_state: DragState::default(),
            click_detector: ClickDetector::new(),
            world_position: WorldPosition::origin(),
            accumulated_scroll: (0.0, 0.0),
            gamepads: HashMap::new(),
            active_gamepad: None,
            window_focused: true,
        }
    }
}

impl InputState {
    /// Create new empty input state
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a window event and update input state
    pub fn process_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::FocusGained => {
                self.window_focused = true;
            }
            WindowEvent::FocusLost => {
                self.window_focused = false;

                // Clear button states to simulate key/button release
                self.pressed_keys.clear();
                self.pressed_mouse_buttons.clear();
            }
            WindowEvent::KeyPressed { key, mods, repeat } => {
                self.pressed_keys.insert(*key);
                self.modifiers = *mods;

                // Handle key repeat
                if self.key_repeat_enabled {
                    if *repeat {
                        *self.key_repeat_counts.entry(*key).or_insert(0) += 1;
                    } else {
                        self.key_repeat_counts.insert(*key, 0);
                    }
                }
            }
            WindowEvent::KeyReleased { key, mods } => {
                self.pressed_keys.remove(key);
                self.modifiers = *mods;
                self.key_repeat_counts.remove(key);
            }
            WindowEvent::TextInput { text } => {
                self.text_input.push_str(text);
            }
            WindowEvent::MouseMoved { x, y, dx, dy } => {
                self.mouse_position = (*x, *y);
                self.mouse_delta = (*dx, *dy);

                // Update NDC position
                self.ndc_position = NdcPosition::from_screen(
                    *x as f32,
                    *y as f32,
                    self.screen_size.0,
                    self.screen_size.1,
                );

                // Update drag state if dragging
                if let DragState::Dragging {
                    button,
                    start_x,
                    start_y,
                    ..
                } = self.drag_state
                {
                    self.drag_state = DragState::Dragging {
                        button,
                        start_x,
                        start_y,
                        current_x: *x,
                        current_y: *y,
                    };
                }
            }
            WindowEvent::MousePressed { button, x, y } => {
                self.pressed_mouse_buttons.insert(*button);

                // Start click detection
                self.click_detector.on_mouse_down(*x, *y);

                // Start drag state
                self.drag_state = DragState::Dragging {
                    button: *button,
                    start_x: *x,
                    start_y: *y,
                    current_x: *x,
                    current_y: *y,
                };
            }
            WindowEvent::MouseReleased { button, x, y } => {
                self.pressed_mouse_buttons.remove(button);

                // Check for click
                let _click_result = self.click_detector.on_mouse_up(*x, *y);

                // Reset drag state if this was the dragging button
                if self.drag_state.get_button() == Some(*button) {
                    self.drag_state.reset();
                }
            }
            WindowEvent::MouseScrolled { dx, dy } => {
                self.scroll_delta = (*dx, *dy);
                self.accumulated_scroll.0 += *dx as f32;
                self.accumulated_scroll.1 += *dy as f32;
            }
            WindowEvent::GamepadConnected { which } => {
                self.gamepads.insert(*which, GamepadState::new(*which));

                // Set as active if no active gamepad
                if self.active_gamepad.is_none() {
                    self.active_gamepad = Some(*which);
                }
            }
            WindowEvent::GamepadDisconnected { which } => {
                self.gamepads.remove(which);

                // Remove from active if it was the active gamepad
                if self.active_gamepad == Some(*which) {
                    self.active_gamepad = self.gamepads.keys().next().copied();
                }
            }
            WindowEvent::GamepadPressed { which, button } => {
                if let Some(gamepad) = self.gamepads.get_mut(which) {
                    gamepad.pressed_buttons.insert(*button);
                }
            }
            WindowEvent::GamepadReleased { which, button } => {
                if let Some(gamepad) = self.gamepads.get_mut(which) {
                    gamepad.pressed_buttons.remove(button);
                }
            }
            WindowEvent::GamepadAxisMoved { which, axis, value } => {
                // Normalize from i16 to f32 [-1.0, 1.0]
                let normalized = *value as f32 / 32767.0;

                if let Some(gamepad) = self.gamepads.get_mut(which) {
                    gamepad.axis_values.insert(*axis, normalized);
                }
            }
            _ => {}
        }
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Check if any of the given keys are pressed
    pub fn is_any_key_pressed(&self, keys: &[Keycode]) -> bool {
        keys.iter().any(|k| self.pressed_keys.contains(k))
    }

    /// Check if all of the given keys are pressed
    pub fn are_all_keys_pressed(&self, keys: &[Keycode]) -> bool {
        keys.iter().all(|k| self.pressed_keys.contains(k))
    }

    /// Check if a mouse button is pressed
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    /// Get current mouse position
    pub fn mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    /// Get mouse X coordinate
    pub fn mouse_x(&self) -> i32 {
        self.mouse_position.0
    }

    /// Get mouse Y coordinate
    pub fn mouse_y(&self) -> i32 {
        self.mouse_position.1
    }

    /// Get mouse delta from last frame
    pub fn mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    /// Get scroll wheel delta
    pub fn scroll_delta(&self) -> (i32, i32) {
        self.scroll_delta
    }

    /// Get vertical scroll amount
    pub fn scroll_y(&self) -> i32 {
        self.scroll_delta.1
    }

    /// Check if shift is pressed
    pub fn shift_pressed(&self) -> bool {
        self.modifiers.shift
    }

    /// Check if ctrl is pressed
    pub fn ctrl_pressed(&self) -> bool {
        self.modifiers.ctrl
    }

    /// Check if alt is pressed
    pub fn alt_pressed(&self) -> bool {
        self.modifiers.alt
    }

    /// Check if logo/super key is pressed
    pub fn logo_pressed(&self) -> bool {
        self.modifiers.logo
    }

    /// Reset per-frame deltas (call at end of frame)
    pub fn end_frame(&mut self) {
        self.mouse_delta = (0, 0);
        self.scroll_delta = (0, 0);
        self.text_input.clear();
    }

    /// Clear all input state
    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.pressed_mouse_buttons.clear();
        self.mouse_delta = (0, 0);
        self.scroll_delta = (0, 0);
        self.modifiers = Modifiers::default();
        self.key_repeat_counts.clear();
        self.text_input.clear();
    }

    /// Get repeat count for a key (0 = initial press, >0 = repeats)
    pub fn key_repeat_count(&self, key: Keycode) -> u32 {
        *self.key_repeat_counts.get(&key).unwrap_or(&0)
    }

    /// Check if this is a repeated key press
    pub fn is_key_repeat(&self, key: Keycode) -> bool {
        self.key_repeat_count(key) > 0
    }

    /// Get pending text input
    pub fn take_text_input(&mut self) -> String {
        std::mem::take(&mut self.text_input)
    }

    /// Enable or disable key repeat
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        self.key_repeat_enabled = enabled;
    }

    /// Check if key repeat is enabled
    pub fn is_key_repeat_enabled(&self) -> bool {
        self.key_repeat_enabled
    }

    /// Set screen dimensions for NDC conversion
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.screen_size = (width, height);
        // Update NDC position
        self.ndc_position = NdcPosition::from_screen(
            self.mouse_position.0 as f32,
            self.mouse_position.1 as f32,
            width,
            height,
        );
    }

    /// Get screen dimensions
    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    /// Get normalized device coordinates
    pub fn ndc_position(&self) -> NdcPosition {
        self.ndc_position
    }

    /// Get world position (requires camera to update this)
    pub fn world_position(&self) -> WorldPosition {
        self.world_position
    }

    /// Set world position (typically called by camera system)
    pub fn set_world_position(&mut self, pos: WorldPosition) {
        self.world_position = pos;
    }

    /// Get drag state
    pub fn drag_state(&self) -> &DragState {
        &self.drag_state
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.drag_state.is_dragging()
    }

    /// Check if dragging with specific button
    pub fn is_dragging_with(&self, button: MouseButton) -> bool {
        self.drag_state.get_button() == Some(button)
    }

    /// Get drag delta (from start to current)
    pub fn drag_delta(&self) -> (i32, i32) {
        self.drag_state.get_delta()
    }

    /// Get click detector
    pub fn click_detector(&self) -> &ClickDetector {
        &self.click_detector
    }

    /// Get click detector (mutable)
    pub fn click_detector_mut(&mut self) -> &mut ClickDetector {
        &mut self.click_detector
    }

    /// Get accumulated scroll (for smooth scrolling)
    pub fn accumulated_scroll(&self) -> (f32, f32) {
        self.accumulated_scroll
    }

    /// Take and reset accumulated scroll
    pub fn take_accumulated_scroll(&mut self) -> (f32, f32) {
        let scroll = self.accumulated_scroll;
        self.accumulated_scroll = (0.0, 0.0);
        scroll
    }

    /// Convert screen coordinates to world coordinates
    ///
    /// This is a placeholder - actual implementation depends on camera
    pub fn screen_to_world(&self, x: i32, y: i32) -> WorldPosition {
        // Simple orthographic projection without camera
        let aspect_ratio = self.screen_size.0 as f32 / self.screen_size.1 as f32;
        let ndc_x = (2.0 * x as f32 / self.screen_size.0 as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * y as f32 / self.screen_size.1 as f32);

        WorldPosition {
            x: ndc_x * aspect_ratio,
            y: ndc_y,
            z: 0.0,
        }
    }

    /// Convert world coordinates to screen coordinates
    ///
    /// This is a placeholder - actual implementation depends on camera
    pub fn world_to_screen(&self, world: &WorldPosition) -> (i32, i32) {
        let aspect_ratio = self.screen_size.0 as f32 / self.screen_size.1 as f32;
        let ndc_x = world.x / aspect_ratio;
        let ndc_y = world.y;

        let screen_x = ((ndc_x + 1.0) / 2.0) * self.screen_size.0 as f32;
        let screen_y = ((1.0 - ndc_y) / 2.0) * self.screen_size.1 as f32;

        (screen_x as i32, screen_y as i32)
    }

    /// Update NDC position based on current mouse position and screen size
    pub fn update_ndc_position(&mut self) {
        self.ndc_position = NdcPosition::from_screen(
            self.mouse_position.0 as f32,
            self.mouse_position.1 as f32,
            self.screen_size.0,
            self.screen_size.1,
        );
    }

    /// Get all connected gamepads
    pub fn gamepads(&self) -> &HashMap<u32, GamepadState> {
        &self.gamepads
    }

    /// Get gamepad by instance ID
    pub fn gamepad(&self, which: u32) -> Option<&GamepadState> {
        self.gamepads.get(&which)
    }

    /// Get active gamepad (for single-player)
    pub fn active_gamepad(&self) -> Option<&GamepadState> {
        self.active_gamepad.and_then(|id| self.gamepads.get(&id))
    }

    /// Set active gamepad
    pub fn set_active_gamepad(&mut self, which: u32) {
        if self.gamepads.contains_key(&which) {
            self.active_gamepad = Some(which);
        }
    }

    /// Get active gamepad ID
    pub fn active_gamepad_id(&self) -> Option<u32> {
        self.active_gamepad
    }

    /// Check if any gamepad is connected
    pub fn has_gamepad(&self) -> bool {
        !self.gamepads.is_empty()
    }

    /// Get number of connected gamepads
    pub fn gamepad_count(&self) -> usize {
        self.gamepads.len()
    }

    /// Check if window has input focus
    pub fn has_window_focus(&self) -> bool {
        self.window_focused
    }

    /// Set window focus state (for manual control)
    pub fn set_window_focus(&mut self, focused: bool) {
        self.window_focused = focused;

        // Clear input states when losing focus
        if !focused {
            self.pressed_keys.clear();
            self.pressed_mouse_buttons.clear();
        }
    }
}

/// Virtual key codes for simulation actions
///
/// Provides a platform-independent abstraction over SDL2 keycodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualKey {
    /// Escape key
    Escape,
    /// Space bar
    Space,
    /// Enter/Return key
    Enter,
    /// Tab key
    Tab,
    /// Backspace
    Backspace,
    /// Delete key
    Delete,

    /// Arrow keys
    Up,
    Down,
    Left,
    Right,

    /// WASD keys
    W,
    A,
    S,
    D,

    /// Modifier keys
    Shift,
    Ctrl,
    Alt,
    Logo,

    /// Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    /// Zoom controls
    Plus,
    Minus,
    Equals,

    /// Page navigation
    PageUp,
    PageDown,
    Home,
    End,

    /// Additional camera controls
    Q,
    E,
    R,
    F,

    /// UI controls
    P,
    B,
    M,
    H,
    I,

    /// Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    /// Unknown key
    Unknown,
}

/// Keycode mapper
///
/// Converts SDL2 keycodes to virtual keys for platform-independent input handling
pub struct KeycodeMapper;

impl KeycodeMapper {
    /// Map SDL2 keycode to virtual key
    pub fn map_keycode(keycode: Keycode) -> VirtualKey {
        match keycode {
            Keycode::Escape => VirtualKey::Escape,
            Keycode::Space => VirtualKey::Space,
            Keycode::Return => VirtualKey::Enter,
            Keycode::Return2 => VirtualKey::Enter,
            Keycode::KpEnter => VirtualKey::Enter,
            Keycode::Tab => VirtualKey::Tab,
            Keycode::Backspace => VirtualKey::Backspace,
            Keycode::Delete => VirtualKey::Delete,

            // Arrow keys
            Keycode::Up => VirtualKey::Up,
            Keycode::Down => VirtualKey::Down,
            Keycode::Left => VirtualKey::Left,
            Keycode::Right => VirtualKey::Right,

            // WASD
            Keycode::W => VirtualKey::W,
            Keycode::A => VirtualKey::A,
            Keycode::S => VirtualKey::S,
            Keycode::D => VirtualKey::D,

            // Modifiers
            Keycode::LShift | Keycode::RShift => VirtualKey::Shift,
            Keycode::LCtrl | Keycode::RCtrl => VirtualKey::Ctrl,
            Keycode::LAlt | Keycode::RAlt => VirtualKey::Alt,
            Keycode::LGui | Keycode::RGui => VirtualKey::Logo,

            // Function keys
            Keycode::F1 => VirtualKey::F1,
            Keycode::F2 => VirtualKey::F2,
            Keycode::F3 => VirtualKey::F3,
            Keycode::F4 => VirtualKey::F4,
            Keycode::F5 => VirtualKey::F5,
            Keycode::F6 => VirtualKey::F6,
            Keycode::F7 => VirtualKey::F7,
            Keycode::F8 => VirtualKey::F8,
            Keycode::F9 => VirtualKey::F9,
            Keycode::F10 => VirtualKey::F10,
            Keycode::F11 => VirtualKey::F11,
            Keycode::F12 => VirtualKey::F12,

            // Zoom
            Keycode::Plus => VirtualKey::Plus,
            Keycode::Minus => VirtualKey::Minus,
            Keycode::Equals => VirtualKey::Equals,
            Keycode::KpPlus => VirtualKey::Plus,
            Keycode::KpMinus => VirtualKey::Minus,

            // Page navigation
            Keycode::PageUp => VirtualKey::PageUp,
            Keycode::PageDown => VirtualKey::PageDown,
            Keycode::Home => VirtualKey::Home,
            Keycode::End => VirtualKey::End,

            // Camera controls
            Keycode::Q => VirtualKey::Q,
            Keycode::E => VirtualKey::E,
            Keycode::R => VirtualKey::R,
            Keycode::F => VirtualKey::F,

            // UI controls
            Keycode::P => VirtualKey::P,
            Keycode::B => VirtualKey::B,
            Keycode::M => VirtualKey::M,
            Keycode::H => VirtualKey::H,
            Keycode::I => VirtualKey::I,

            // Numbers
            Keycode::Num0 | Keycode::Kp0 => VirtualKey::Num0,
            Keycode::Num1 | Keycode::Kp1 => VirtualKey::Num1,
            Keycode::Num2 | Keycode::Kp2 => VirtualKey::Num2,
            Keycode::Num3 | Keycode::Kp3 => VirtualKey::Num3,
            Keycode::Num4 | Keycode::Kp4 => VirtualKey::Num4,
            Keycode::Num5 | Keycode::Kp5 => VirtualKey::Num5,
            Keycode::Num6 | Keycode::Kp6 => VirtualKey::Num6,
            Keycode::Num7 | Keycode::Kp7 => VirtualKey::Num7,
            Keycode::Num8 | Keycode::Kp8 => VirtualKey::Num8,
            Keycode::Num9 | Keycode::Kp9 => VirtualKey::Num9,

            _ => VirtualKey::Unknown,
        }
    }

    /// Check if keycode is a movement key
    pub fn is_movement_key(keycode: Keycode) -> bool {
        matches!(
            keycode,
            Keycode::W
                | Keycode::A
                | Keycode::S
                | Keycode::D
                | Keycode::Up
                | Keycode::Down
                | Keycode::Left
                | Keycode::Right
                | Keycode::Q
                | Keycode::E
                | Keycode::PageUp
                | Keycode::PageDown
        )
    }

    /// Check if keycode is a zoom key
    pub fn is_zoom_key(keycode: Keycode) -> bool {
        matches!(
            keycode,
            Keycode::Plus | Keycode::Minus | Keycode::Equals | Keycode::KpPlus | Keycode::KpMinus
        )
    }

    /// Check if keycode is a UI toggle key
    pub fn is_ui_key(keycode: Keycode) -> bool {
        matches!(
            keycode,
            Keycode::Tab
                | Keycode::F1
                | Keycode::F2
                | Keycode::F3
                | Keycode::F4
                | Keycode::F5
                | Keycode::F6
                | Keycode::F7
                | Keycode::F8
                | Keycode::F9
                | Keycode::F10
                | Keycode::F11
                | Keycode::F12
                | Keycode::H
                | Keycode::I
                | Keycode::B
                | Keycode::M
                | Keycode::P
        )
    }
}

/// Input action mapping
///
/// Maps input combinations to named actions for the simulation
#[derive(Debug, Clone)]
pub struct InputMap {
    /// Key bindings: action name -> (primary key, optional modifiers)
    key_bindings: std::collections::HashMap<String, (Keycode, Option<Modifiers>)>,
}

impl Default for InputMap {
    fn default() -> Self {
        let mut map = InputMap {
            key_bindings: std::collections::HashMap::new(),
        };

        // Default camera controls
        map.bind_key("camera_forward", Keycode::W, None);
        map.bind_key("camera_backward", Keycode::S, None);
        map.bind_key("camera_left", Keycode::A, None);
        map.bind_key("camera_right", Keycode::D, None);
        map.bind_key("camera_up", Keycode::Q, None);
        map.bind_key("camera_down", Keycode::E, None);

        // Time controls
        map.bind_key("pause", Keycode::Space, None);
        map.bind_key("speed_up", Keycode::Up, None);
        map.bind_key("slow_down", Keycode::Down, None);

        // Zoom controls
        map.bind_key("zoom_in", Keycode::Equals, None); // + key
        map.bind_key("zoom_out", Keycode::Minus, None);

        // UI toggles
        map.bind_key("toggle_ui", Keycode::Tab, None);
        map.bind_key("fullscreen", Keycode::F11, None);

        map
    }
}

impl InputMap {
    /// Create new empty input map
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind a key to an action
    pub fn bind_key(&mut self, action: &str, key: Keycode, mods: Option<Modifiers>) {
        self.key_bindings.insert(action.to_string(), (key, mods));
    }

    /// Check if an action is triggered
    pub fn is_action_triggered(&self, action: &str, input: &InputState) -> bool {
        if let Some((key, mods)) = self.key_bindings.get(action) {
            let key_pressed = input.is_key_pressed(*key);

            if let Some(required_mods) = mods {
                let mods_match = input.shift_pressed() == required_mods.shift
                    && input.ctrl_pressed() == required_mods.ctrl
                    && input.alt_pressed() == required_mods.alt
                    && input.logo_pressed() == required_mods.logo;

                key_pressed && mods_match
            } else {
                key_pressed
            }
        } else {
            false
        }
    }

    /// Get the key bound to an action
    pub fn get_action_key(&self, action: &str) -> Option<&(Keycode, Option<Modifiers>)> {
        self.key_bindings.get(action)
    }

    /// Remove a binding
    pub fn unbind_action(&mut self, action: &str) {
        self.key_bindings.remove(action);
    }
}

/// Camera input controller
///
/// Processes input and outputs camera movement commands
#[derive(Debug, Clone)]
pub struct CameraInputController {
    /// Movement speed multiplier
    move_speed: f32,

    /// Rotation speed multiplier
    rotate_speed: f32,

    /// Zoom speed multiplier
    zoom_speed: f32,
}

impl Default for CameraInputController {
    fn default() -> Self {
        CameraInputController {
            move_speed: 1.0,
            rotate_speed: 0.005,
            zoom_speed: 0.1,
        }
    }
}

impl CameraInputController {
    /// Create new camera controller with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Update movement speed
    pub fn with_move_speed(mut self, speed: f32) -> Self {
        self.move_speed = speed;
        self
    }

    /// Update rotation speed
    pub fn with_rotate_speed(mut self, speed: f32) -> Self {
        self.rotate_speed = speed;
        self
    }

    /// Update zoom speed
    pub fn with_zoom_speed(mut self, speed: f32) -> Self {
        self.zoom_speed = speed;
        self
    }

    /// Check if quit was requested (ESC key)
    pub fn should_quit(&self, input: &InputState) -> bool {
        input.is_key_pressed(Keycode::Escape)
    }

    /// Check if pause was requested (Space key)
    pub fn should_pause(&self, input: &InputState) -> bool {
        input.is_key_pressed(Keycode::Space) && !input.is_key_repeat(Keycode::Space)
    }

    /// Check if fullscreen toggle was requested (F11 key)
    pub fn should_toggle_fullscreen(&self, input: &InputState) -> bool {
        input.is_key_pressed(Keycode::F11) && !input.is_key_repeat(Keycode::F11)
    }

    /// Check if UI toggle was requested (Tab key)
    pub fn should_toggle_ui(&self, input: &InputState) -> bool {
        input.is_key_pressed(Keycode::Tab) && !input.is_key_repeat(Keycode::Tab)
    }

    /// Check if bookmark should be added (B key)
    pub fn should_add_bookmark(&self, input: &InputState) -> bool {
        input.is_key_pressed(Keycode::B) && !input.is_key_repeat(Keycode::B)
    }

    /// Process input and return camera movement
    ///
    /// Returns: (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    pub fn process(&self, input: &InputState) -> (f32, f32, f32, f32, f32, f32) {
        let mut move_x = 0.0f32;
        let mut move_y = 0.0f32;
        let mut move_z = 0.0f32;
        let mut rotate_x = 0.0f32;
        let mut rotate_y = 0.0f32;
        let mut zoom = 0.0f32;

        // Keyboard movement (check for key repeat to allow smooth movement)
        let is_w_pressed = input.is_key_pressed(Keycode::W);
        let is_s_pressed = input.is_key_pressed(Keycode::S);
        let is_a_pressed = input.is_key_pressed(Keycode::A);
        let is_d_pressed = input.is_key_pressed(Keycode::D);
        let is_q_pressed = input.is_key_pressed(Keycode::Q);
        let is_e_pressed = input.is_key_pressed(Keycode::E);

        // Arrow key movement
        let is_up_pressed = input.is_key_pressed(Keycode::Up);
        let is_down_pressed = input.is_key_pressed(Keycode::Down);
        let is_left_pressed = input.is_key_pressed(Keycode::Left);
        let is_right_pressed = input.is_key_pressed(Keycode::Right);

        // Page up/down for vertical movement
        let is_page_up_pressed = input.is_key_pressed(Keycode::PageUp);
        let is_page_down_pressed = input.is_key_pressed(Keycode::PageDown);

        // Forward/backward
        if is_w_pressed || is_up_pressed {
            move_z -= self.move_speed;
        }
        if is_s_pressed || is_down_pressed {
            move_z += self.move_speed;
        }

        // Strafe left/right
        if is_a_pressed || is_left_pressed {
            move_x -= self.move_speed;
        }
        if is_d_pressed || is_right_pressed {
            move_x += self.move_speed;
        }

        // Vertical movement
        if is_q_pressed || is_page_up_pressed {
            move_y += self.move_speed;
        }
        if is_e_pressed || is_page_down_pressed {
            move_y -= self.move_speed;
        }

        // Mouse drag rotation (when right button held)
        if input.is_mouse_button_pressed(MouseButton::Right) {
            let (dx, dy) = input.mouse_delta();
            rotate_x = dx as f32 * self.rotate_speed;
            rotate_y = dy as f32 * self.rotate_speed;
        }

        // Scroll zoom
        zoom = input.scroll_y() as f32 * self.zoom_speed;

        // Zoom keys (with repeat for smooth zooming)
        let is_plus_pressed =
            input.is_key_pressed(Keycode::Equals) || input.is_key_pressed(Keycode::Plus);
        let is_minus_pressed = input.is_key_pressed(Keycode::Minus);

        if is_plus_pressed {
            zoom += self.zoom_speed;
        }
        if is_minus_pressed {
            zoom -= self.zoom_speed;
        }

        (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    }

    /// Get movement vector (only x, y, z)
    pub fn get_movement(&self, input: &InputState) -> (f32, f32, f32) {
        let (_, my, mz, rx, ry, zoom) = self.process(input);
        (0.0, my, mz)
    }

    /// Get rotation vector (only rotate_x, rotate_y)
    pub fn get_rotation(&self, input: &InputState) -> (f32, f32) {
        let (_, _, _, rx, ry, _) = self.process(input);
        (rx, ry)
    }

    /// Get zoom amount
    pub fn get_zoom(&self, input: &InputState) -> f32 {
        let (_, _, _, _, _, zoom) = self.process(input);
        zoom
    }

    /// Check if any movement input is active
    pub fn is_movement_active(&self, input: &InputState) -> bool {
        let (mx, my, mz, _, _, _) = self.process(input);
        mx != 0.0 || my != 0.0 || mz != 0.0
    }

    /// Check if any rotation input is active
    pub fn is_rotation_active(&self, input: &InputState) -> bool {
        let (_, _, _, rx, ry, _) = self.process(input);
        rx != 0.0 || ry != 0.0
    }

    /// Check if any zoom input is active
    pub fn is_zoom_active(&self, input: &InputState) -> bool {
        self.get_zoom(input) != 0.0
    }

    /// Check if left mouse button is pressed (for panning)
    pub fn is_left_mouse_pressed(&self, input: &InputState) -> bool {
        input.is_mouse_button_pressed(MouseButton::Left)
    }

    /// Check if right mouse button is pressed (for rotation)
    pub fn is_right_mouse_pressed(&self, input: &InputState) -> bool {
        input.is_mouse_button_pressed(MouseButton::Right)
    }

    /// Check if middle mouse button is pressed (for alternative pan)
    pub fn is_middle_mouse_pressed(&self, input: &InputState) -> bool {
        input.is_mouse_button_pressed(MouseButton::Middle)
    }

    /// Check if currently dragging with left mouse button
    pub fn is_left_dragging(&self, input: &InputState) -> bool {
        input.is_dragging_with(MouseButton::Left)
    }

    /// Check if currently dragging with right mouse button
    pub fn is_right_dragging(&self, input: &InputState) -> bool {
        input.is_dragging_with(MouseButton::Right)
    }

    /// Get drag delta for left mouse button
    pub fn left_drag_delta(&self, input: &InputState) -> (i32, i32) {
        if input.is_dragging_with(MouseButton::Left) {
            input.drag_delta()
        } else {
            (0, 0)
        }
    }

    /// Get drag delta for right mouse button
    pub fn right_drag_delta(&self, input: &InputState) -> (i32, i32) {
        if input.is_dragging_with(MouseButton::Right) {
            input.drag_delta()
        } else {
            (0, 0)
        }
    }

    /// Check if mouse click occurred (single click)
    ///
    /// Returns click position if a single click was detected, None otherwise
    pub fn get_click_position(&self, _input: &InputState) -> Option<(i32, i32)> {
        // This would need to be checked in the event handler
        // For now, return None as placeholder
        None
    }

    /// Get smooth scroll amount (accumulated)
    pub fn get_smooth_scroll(&self, input: &InputState) -> f32 {
        input.accumulated_scroll().1
    }

    /// Take smooth scroll amount and reset accumulator
    pub fn take_smooth_scroll(&self, input: &mut InputState) -> f32 {
        input.take_accumulated_scroll().1
    }

    /// Get gamepad left stick X axis (for camera pan)
    pub fn get_gamepad_left_stick_x(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::LeftX))
            .unwrap_or(0.0)
    }

    /// Get gamepad left stick Y axis (for camera pan)
    pub fn get_gamepad_left_stick_y(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::LeftY))
            .unwrap_or(0.0)
    }

    /// Get gamepad right stick X axis (for camera rotation)
    pub fn get_gamepad_right_stick_x(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::RightX))
            .unwrap_or(0.0)
    }

    /// Get gamepad right stick Y axis (for camera rotation)
    pub fn get_gamepad_right_stick_y(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::RightY))
            .unwrap_or(0.0)
    }

    /// Get gamepad left trigger (for zoom out)
    pub fn get_gamepad_left_trigger(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::TriggerLeft))
            .unwrap_or(0.0)
    }

    /// Get gamepad right trigger (for zoom in)
    pub fn get_gamepad_right_trigger(&self, input: &InputState) -> f32 {
        input
            .active_gamepad()
            .map(|gp| gp.get_axis(GamepadAxis::TriggerRight))
            .unwrap_or(0.0)
    }

    /// Check if gamepad button is pressed
    pub fn is_gamepad_button_pressed(&self, input: &InputState, button: GamepadButton) -> bool {
        input
            .active_gamepad()
            .map(|gp| gp.is_button_pressed(button))
            .unwrap_or(false)
    }

    /// Check if gamepad Start button is pressed (pause)
    pub fn is_gamepad_start_pressed(&self, input: &InputState) -> bool {
        self.is_gamepad_button_pressed(input, GamepadButton::Start)
    }

    /// Check if gamepad A button is pressed (confirm/action)
    pub fn is_gamepad_a_pressed(&self, input: &InputState) -> bool {
        self.is_gamepad_button_pressed(input, GamepadButton::A)
    }

    /// Check if gamepad B button is pressed (cancel/back)
    pub fn is_gamepad_b_pressed(&self, input: &InputState) -> bool {
        self.is_gamepad_button_pressed(input, GamepadButton::B)
    }

    /// Check if gamepad X button is pressed (bookmark)
    pub fn is_gamepad_x_pressed(&self, input: &InputState) -> bool {
        self.is_gamepad_button_pressed(input, GamepadButton::X)
    }

    /// Check if gamepad Y button is pressed (toggle UI)
    pub fn is_gamepad_y_pressed(&self, input: &InputState) -> bool {
        self.is_gamepad_button_pressed(input, GamepadButton::Y)
    }

    /// Process gamepad input and return camera movement
    ///
    /// Returns: (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    pub fn process_gamepad(&self, input: &InputState) -> (f32, f32, f32, f32, f32, f32) {
        let mut move_x = 0.0f32;
        let mut move_y = 0.0f32;
        let mut move_z = 0.0f32;
        let mut rotate_x = 0.0f32;
        let mut rotate_y = 0.0f32;
        let mut zoom = 0.0f32;

        if let Some(gamepad) = input.active_gamepad() {
            // Left stick: camera pan (move_x, move_z)
            let left_x = gamepad.get_axis(GamepadAxis::LeftX);
            let left_y = gamepad.get_axis(GamepadAxis::LeftY);

            // Apply deadzone
            let deadzone = 0.15f32;
            let left_x = if left_x.abs() < deadzone {
                0.0
            } else {
                left_x * self.move_speed
            };
            let left_y = if left_y.abs() < deadzone {
                0.0
            } else {
                left_y * self.move_speed
            };

            move_x = left_x;
            move_z = left_y;

            // Right stick: camera rotation
            let right_x = gamepad.get_axis(GamepadAxis::RightX);
            let right_y = gamepad.get_axis(GamepadAxis::RightY);

            let right_x = if right_x.abs() < deadzone {
                0.0
            } else {
                right_x * self.rotate_speed * 100.0
            };
            let right_y = if right_y.abs() < deadzone {
                0.0
            } else {
                right_y * self.rotate_speed * 100.0
            };

            rotate_x = right_x;
            rotate_y = right_y;

            // Triggers: zoom (right trigger = zoom in, left trigger = zoom out)
            let left_trigger = gamepad.get_axis(GamepadAxis::TriggerLeft);
            let right_trigger = gamepad.get_axis(GamepadAxis::TriggerRight);

            // Normalize triggers from [0, 1] to [-1, 1]
            zoom = (right_trigger - left_trigger) * self.zoom_speed * 2.0;

            // D-pad: additional movement
            if gamepad.is_button_pressed(GamepadButton::DPadUp) {
                move_z -= self.move_speed;
            }
            if gamepad.is_button_pressed(GamepadButton::DPadDown) {
                move_z += self.move_speed;
            }
            if gamepad.is_button_pressed(GamepadButton::DPadLeft) {
                move_x -= self.move_speed;
            }
            if gamepad.is_button_pressed(GamepadButton::DPadRight) {
                move_x += self.move_speed;
            }
        }

        (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    }

    /// Process both keyboard and gamepad input
    ///
    /// Returns: (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    pub fn process_all(&self, input: &InputState) -> (f32, f32, f32, f32, f32, f32) {
        let (kbd_mx, kbd_my, kbd_mz, kbd_rx, kbd_ry, kbd_zoom) = self.process(input);
        let (gp_mx, gp_my, gp_mz, gp_rx, gp_ry, gp_zoom) = self.process_gamepad(input);

        // Combine inputs (keyboard takes priority if both active)
        let move_x = if kbd_mx != 0.0 || kbd_my != 0.0 || kbd_mz != 0.0 {
            kbd_mx
        } else {
            gp_mx
        };
        let move_y = if kbd_mx != 0.0 || kbd_my != 0.0 || kbd_mz != 0.0 {
            kbd_my
        } else {
            gp_my
        };
        let move_z = if kbd_mx != 0.0 || kbd_my != 0.0 || kbd_mz != 0.0 {
            kbd_mz
        } else {
            gp_mz
        };

        let rotate_x = if kbd_rx != 0.0 || kbd_ry != 0.0 {
            kbd_rx
        } else {
            gp_rx
        };
        let rotate_y = if kbd_rx != 0.0 || kbd_ry != 0.0 {
            kbd_ry
        } else {
            gp_ry
        };

        let zoom = if kbd_zoom != 0.0 { kbd_zoom } else { gp_zoom };

        (move_x, move_y, move_z, rotate_x, rotate_y, zoom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_default() {
        let state = InputState::new();
        assert!(!state.is_key_pressed(Keycode::A));
        assert_eq!(state.mouse_position(), (0, 0));
        assert_eq!(state.screen_size(), (1920, 1080));
        assert!(!state.is_dragging());
    }

    #[test]
    fn test_input_state_key_events() {
        let mut state = InputState::new();

        // Simulate key press (initial, not repeat)
        state.process_event(&WindowEvent::KeyPressed {
            key: Keycode::A,
            mods: Modifiers::default(),
            repeat: false,
        });
        assert!(state.is_key_pressed(Keycode::A));
        assert_eq!(state.key_repeat_count(Keycode::A), 0);
        assert!(!state.is_key_repeat(Keycode::A));

        // Simulate key repeat
        state.process_event(&WindowEvent::KeyPressed {
            key: Keycode::A,
            mods: Modifiers::default(),
            repeat: true,
        });
        assert!(state.is_key_pressed(Keycode::A));
        assert_eq!(state.key_repeat_count(Keycode::A), 1);
        assert!(state.is_key_repeat(Keycode::A));

        // Simulate key release
        state.process_event(&WindowEvent::KeyReleased {
            key: Keycode::A,
            mods: Modifiers::default(),
        });
        assert!(!state.is_key_pressed(Keycode::A));
        assert_eq!(state.key_repeat_count(Keycode::A), 0);
    }

    #[test]
    fn test_input_map_default_bindings() {
        let map = InputMap::new();

        assert!(map.get_action_key("camera_forward").is_some());
        assert!(map.get_action_key("pause").is_some());
    }

    #[test]
    fn test_camera_controller() {
        let controller = CameraInputController::new();
        let input = InputState::new();

        let (mx, my, mz, rx, ry, zoom) = controller.process(&input);
        assert_eq!(mx, 0.0);
        assert_eq!(my, 0.0);
        assert_eq!(mz, 0.0);
    }

    #[test]
    fn test_camera_controller_movement() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Press W key
        input.process_event(&WindowEvent::KeyPressed {
            key: Keycode::W,
            mods: Modifiers::default(),
            repeat: false,
        });

        let (mx, my, mz, _, _, _) = controller.process(&input);
        assert_eq!(mx, 0.0);
        assert_eq!(my, 0.0);
        assert!(mz < 0.0); // Should move backward (negative Z)
    }

    #[test]
    fn test_camera_controller_zoom() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Press plus key
        input.process_event(&WindowEvent::KeyPressed {
            key: Keycode::Plus,
            mods: Modifiers::default(),
            repeat: false,
        });

        let (_, _, _, _, _, zoom) = controller.process(&input);
        assert!(zoom > 0.0);
    }

    #[test]
    fn test_keycode_mapper() {
        assert_eq!(
            KeycodeMapper::map_keycode(Keycode::Escape),
            VirtualKey::Escape
        );
        assert_eq!(KeycodeMapper::map_keycode(Keycode::W), VirtualKey::W);
        assert_eq!(KeycodeMapper::map_keycode(Keycode::Up), VirtualKey::Up);
        assert_eq!(KeycodeMapper::map_keycode(Keycode::F11), VirtualKey::F11);
    }

    #[test]
    fn test_keycode_mapper_movement_keys() {
        assert!(KeycodeMapper::is_movement_key(Keycode::W));
        assert!(KeycodeMapper::is_movement_key(Keycode::A));
        assert!(KeycodeMapper::is_movement_key(Keycode::S));
        assert!(KeycodeMapper::is_movement_key(Keycode::D));
        assert!(KeycodeMapper::is_movement_key(Keycode::Up));
        assert!(KeycodeMapper::is_movement_key(Keycode::Down));
        assert!(KeycodeMapper::is_movement_key(Keycode::Left));
        assert!(KeycodeMapper::is_movement_key(Keycode::Right));
        assert!(!KeycodeMapper::is_movement_key(Keycode::Escape));
    }

    #[test]
    fn test_keycode_mapper_zoom_keys() {
        assert!(KeycodeMapper::is_zoom_key(Keycode::Plus));
        assert!(KeycodeMapper::is_zoom_key(Keycode::Minus));
        assert!(KeycodeMapper::is_zoom_key(Keycode::Equals));
        assert!(!KeycodeMapper::is_zoom_key(Keycode::W));
    }

    #[test]
    fn test_keycode_mapper_ui_keys() {
        assert!(KeycodeMapper::is_ui_key(Keycode::Tab));
        assert!(KeycodeMapper::is_ui_key(Keycode::F1));
        assert!(KeycodeMapper::is_ui_key(Keycode::H));
        assert!(!KeycodeMapper::is_ui_key(Keycode::W));
    }

    #[test]
    fn test_text_input() {
        let mut state = InputState::new();

        // Add text input
        state.process_event(&WindowEvent::TextInput {
            text: "Hello".to_string(),
        });
        state.process_event(&WindowEvent::TextInput {
            text: " World".to_string(),
        });

        let text = state.take_text_input();
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_key_repeat_toggle() {
        let mut state = InputState::new();

        assert!(state.is_key_repeat_enabled());

        state.set_key_repeat_enabled(false);
        assert!(!state.is_key_repeat_enabled());

        state.set_key_repeat_enabled(true);
        assert!(state.is_key_repeat_enabled());
    }

    #[test]
    fn test_camera_controller_actions() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Should not quit initially
        assert!(!controller.should_quit(&input));

        // Press ESC
        input.process_event(&WindowEvent::KeyPressed {
            key: Keycode::Escape,
            mods: Modifiers::default(),
            repeat: false,
        });
        assert!(controller.should_quit(&input));
    }

    #[test]
    fn test_ndc_position_from_screen() {
        let ndc = NdcPosition::from_screen(0.0, 0.0, 1000, 800);
        assert_eq!(ndc.x, -1.0);
        assert_eq!(ndc.y, 1.0);

        let ndc = NdcPosition::from_screen(500.0, 400.0, 1000, 800);
        assert_eq!(ndc.x, 0.0);
        assert_eq!(ndc.y, 0.0);

        let ndc = NdcPosition::from_screen(1000.0, 800.0, 1000, 800);
        assert_eq!(ndc.x, 1.0);
        assert_eq!(ndc.y, -1.0);
    }

    #[test]
    fn test_ndc_position_to_screen() {
        let ndc = NdcPosition::new(-1.0, 1.0);
        let (x, y) = ndc.to_screen(1000, 800);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);

        let ndc = NdcPosition::new(0.0, 0.0);
        let (x, y) = ndc.to_screen(1000, 800);
        assert_eq!(x, 500.0);
        assert_eq!(y, 400.0);

        let ndc = NdcPosition::new(1.0, -1.0);
        let (x, y) = ndc.to_screen(1000, 800);
        assert_eq!(x, 1000.0);
        assert_eq!(y, 800.0);
    }

    #[test]
    fn test_drag_state() {
        let mut drag = DragState::default();
        assert!(!drag.is_dragging());
        assert_eq!(drag.get_button(), None);
        assert_eq!(drag.get_delta(), (0, 0));

        // Start dragging
        drag = DragState::Dragging {
            button: MouseButton::Left,
            start_x: 100,
            start_y: 100,
            current_x: 150,
            current_y: 150,
        };

        assert!(drag.is_dragging());
        assert_eq!(drag.get_button(), Some(MouseButton::Left));
        assert_eq!(drag.get_delta(), (50, 50));

        drag.reset();
        assert!(!drag.is_dragging());
    }

    #[test]
    fn test_click_detector() {
        let mut detector = ClickDetector::new();

        // Simulate quick click
        detector.on_mouse_down(100, 100);
        let result = detector.on_mouse_up(102, 102);
        assert!(matches!(result, ClickResult::Click { .. }));

        // Simulate drag (too much movement)
        detector.on_mouse_down(100, 100);
        let result = detector.on_mouse_up(200, 200);
        assert_eq!(result, ClickResult::None);
    }

    #[test]
    fn test_input_state_mouse_tracking() {
        let mut state = InputState::new();
        state.set_screen_size(800, 600);

        // Mouse move
        state.process_event(&WindowEvent::MouseMoved {
            x: 400,
            y: 300,
            dx: 10,
            dy: 5,
        });

        assert_eq!(state.mouse_position(), (400, 300));
        assert_eq!(state.mouse_delta(), (10, 5));

        // Check NDC position
        let ndc = state.ndc_position();
        assert_eq!(ndc.x, 0.0);
        assert_eq!(ndc.y, 0.0);
    }

    #[test]
    fn test_input_state_mouse_buttons() {
        let mut state = InputState::new();

        assert!(!state.is_mouse_button_pressed(MouseButton::Left));

        state.process_event(&WindowEvent::MousePressed {
            button: MouseButton::Left,
            x: 100,
            y: 100,
        });

        assert!(state.is_mouse_button_pressed(MouseButton::Left));

        state.process_event(&WindowEvent::MouseReleased {
            button: MouseButton::Left,
            x: 100,
            y: 100,
        });

        assert!(!state.is_mouse_button_pressed(MouseButton::Left));
    }

    #[test]
    fn test_input_state_dragging() {
        let mut state = InputState::new();

        assert!(!state.is_dragging());

        // Start drag
        state.process_event(&WindowEvent::MousePressed {
            button: MouseButton::Right,
            x: 100,
            y: 100,
        });

        assert!(state.is_dragging());
        assert!(state.is_dragging_with(MouseButton::Right));
        assert_eq!(state.drag_delta(), (0, 0));

        // Move while dragging
        state.process_event(&WindowEvent::MouseMoved {
            x: 150,
            y: 150,
            dx: 50,
            dy: 50,
        });

        assert_eq!(state.drag_delta(), (50, 50));

        // End drag
        state.process_event(&WindowEvent::MouseReleased {
            button: MouseButton::Right,
            x: 150,
            y: 150,
        });

        assert!(!state.is_dragging());
    }

    #[test]
    fn test_input_state_scroll() {
        let mut state = InputState::new();

        state.process_event(&WindowEvent::MouseScrolled { dx: 0, dy: 1 });
        assert_eq!(state.scroll_delta(), (0, 1));
        assert_eq!(state.accumulated_scroll(), (0.0, 1.0));

        state.process_event(&WindowEvent::MouseScrolled { dx: 0, dy: 2 });
        assert_eq!(state.scroll_delta(), (0, 2));
        assert_eq!(state.accumulated_scroll(), (0.0, 3.0));

        let scroll = state.take_accumulated_scroll();
        assert_eq!(scroll, (0.0, 3.0));
        assert_eq!(state.accumulated_scroll(), (0.0, 0.0));
    }

    #[test]
    fn test_camera_controller_mouse() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        assert!(!controller.is_left_mouse_pressed(&input));
        assert!(!controller.is_right_mouse_pressed(&input));
        assert!(!controller.is_left_dragging(&input));
        assert!(!controller.is_right_dragging(&input));

        // Press left mouse
        input.process_event(&WindowEvent::MousePressed {
            button: MouseButton::Left,
            x: 100,
            y: 100,
        });

        assert!(controller.is_left_mouse_pressed(&input));
        assert!(controller.is_left_dragging(&input));
        assert_eq!(controller.left_drag_delta(&input), (0, 0));

        // Move while dragging
        input.process_event(&WindowEvent::MouseMoved {
            x: 120,
            y: 110,
            dx: 20,
            dy: 10,
        });

        assert_eq!(controller.left_drag_delta(&input), (20, 10));
    }

    #[test]
    fn test_screen_to_world_simple() {
        let mut state = InputState::new();
        state.set_screen_size(1000, 800);

        let world = state.screen_to_world(500, 400);
        assert_eq!(world.x, 0.0);
        assert_eq!(world.y, 0.0);

        let world = state.screen_to_world(0, 0);
        assert_eq!(world.x, -1.25); // -1.0 * (1000/800)
        assert_eq!(world.y, 1.0);
    }

    #[test]
    fn test_world_to_screen_simple() {
        let mut state = InputState::new();
        state.set_screen_size(1000, 800);

        let world = WorldPosition::new(0.0, 0.0, 0.0);
        let screen = state.world_to_screen(&world);
        assert_eq!(screen, (500, 400));

        let world = WorldPosition::new(-1.25, 1.0, 0.0);
        let screen = state.world_to_screen(&world);
        assert_eq!(screen, (0, 0));
    }

    #[test]
    fn test_gamepad_state() {
        let gamepad = GamepadState::new(0);
        assert_eq!(gamepad.which, 0);
        assert!(gamepad.connected);
        assert!(!gamepad.is_button_pressed(GamepadButton::A));
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftX), 0.0);
    }

    #[test]
    fn test_gamepad_button_state() {
        let mut gamepad = GamepadState::new(0);

        gamepad.pressed_buttons.insert(GamepadButton::A);
        assert!(gamepad.is_button_pressed(GamepadButton::A));
        assert!(!gamepad.is_button_pressed(GamepadButton::B));

        gamepad.pressed_buttons.insert(GamepadButton::B);
        assert!(gamepad.is_button_pressed(GamepadButton::A));
        assert!(gamepad.is_button_pressed(GamepadButton::B));

        gamepad.pressed_buttons.remove(&GamepadButton::A);
        assert!(!gamepad.is_button_pressed(GamepadButton::A));
        assert!(gamepad.is_button_pressed(GamepadButton::B));
    }

    #[test]
    fn test_gamepad_axis_state() {
        let mut gamepad = GamepadState::new(0);

        gamepad.axis_values.insert(GamepadAxis::LeftX, 0.5);
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftX), 0.5);
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftY), 0.0);

        gamepad.axis_values.insert(GamepadAxis::LeftY, -0.3);
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftX), 0.5);
        assert_eq!(gamepad.get_axis(GamepadAxis::LeftY), -0.3);
    }

    #[test]
    fn test_input_state_gamepad_connection() {
        let mut state = InputState::new();

        assert!(!state.has_gamepad());
        assert_eq!(state.gamepad_count(), 0);

        // Connect gamepad
        state.process_event(&WindowEvent::GamepadConnected { which: 0 });
        assert!(state.has_gamepad());
        assert_eq!(state.gamepad_count(), 1);
        assert_eq!(state.active_gamepad_id(), Some(0));

        // Connect second gamepad
        state.process_event(&WindowEvent::GamepadConnected { which: 1 });
        assert_eq!(state.gamepad_count(), 2);
        assert_eq!(state.active_gamepad_id(), Some(0)); // First one still active

        // Disconnect active gamepad
        state.process_event(&WindowEvent::GamepadDisconnected { which: 0 });
        assert_eq!(state.gamepad_count(), 1);
        assert_eq!(state.active_gamepad_id(), Some(1)); // Second one becomes active

        // Disconnect all gamepads
        state.process_event(&WindowEvent::GamepadDisconnected { which: 1 });
        assert!(!state.has_gamepad());
        assert_eq!(state.active_gamepad_id(), None);
    }

    #[test]
    fn test_input_state_gamepad_buttons() {
        let mut state = InputState::new();

        // Connect gamepad
        state.process_event(&WindowEvent::GamepadConnected { which: 0 });

        // Press button
        state.process_event(&WindowEvent::GamepadPressed {
            which: 0,
            button: GamepadButton::A,
        });

        let gamepad = state.gamepad(0).unwrap();
        assert!(gamepad.is_button_pressed(GamepadButton::A));
        assert!(!gamepad.is_button_pressed(GamepadButton::B));

        // Release button
        state.process_event(&WindowEvent::GamepadReleased {
            which: 0,
            button: GamepadButton::A,
        });

        let gamepad = state.gamepad(0).unwrap();
        assert!(!gamepad.is_button_pressed(GamepadButton::A));
    }

    #[test]
    fn test_input_state_gamepad_axis() {
        let mut state = InputState::new();

        // Connect gamepad
        state.process_event(&WindowEvent::GamepadConnected { which: 0 });

        // Move axis (normalized from i16)
        state.process_event(&WindowEvent::GamepadAxisMoved {
            which: 0,
            axis: GamepadAxis::LeftX,
            value: 16383, // 0.5 when normalized
        });

        let gamepad = state.gamepad(0).unwrap();
        assert!((gamepad.get_axis(GamepadAxis::LeftX) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_camera_controller_gamepad() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Connect gamepad
        input.process_event(&WindowEvent::GamepadConnected { which: 0 });

        // Test initial state
        assert_eq!(controller.get_gamepad_left_stick_x(&input), 0.0);
        assert_eq!(controller.get_gamepad_left_stick_y(&input), 0.0);
        assert_eq!(controller.get_gamepad_right_stick_x(&input), 0.0);
        assert_eq!(controller.get_gamepad_right_stick_y(&input), 0.0);

        // Move left stick
        input.process_event(&WindowEvent::GamepadAxisMoved {
            which: 0,
            axis: GamepadAxis::LeftX,
            value: 16383,
        });

        assert!((controller.get_gamepad_left_stick_x(&input) - 0.5).abs() < 0.01);

        // Test button states
        assert!(!controller.is_gamepad_start_pressed(&input));
        assert!(!controller.is_gamepad_a_pressed(&input));

        input.process_event(&WindowEvent::GamepadPressed {
            which: 0,
            button: GamepadButton::Start,
        });

        assert!(controller.is_gamepad_start_pressed(&input));
    }

    #[test]
    fn test_camera_controller_process_gamepad() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Connect gamepad
        input.process_event(&WindowEvent::GamepadConnected { which: 0 });

        // Move left stick forward (negative Y)
        input.process_event(&WindowEvent::GamepadAxisMoved {
            which: 0,
            axis: GamepadAxis::LeftY,
            value: -16383,
        });

        let (_, _, mz, _, _, _) = controller.process_gamepad(&input);
        assert!(mz < 0.0); // Moving forward
    }

    #[test]
    fn test_camera_controller_process_all() {
        let controller = CameraInputController::new();
        let mut input = InputState::new();

        // Connect gamepad
        input.process_event(&WindowEvent::GamepadConnected { which: 0 });

        // Press keyboard W
        input.process_event(&WindowEvent::KeyPressed {
            key: Keycode::W,
            mods: Modifiers::default(),
            repeat: false,
        });

        let (_, _, mz, _, _, _) = controller.process_all(&input);
        assert!(mz < 0.0); // Keyboard input takes priority
    }

    #[test]
    fn test_gamepad_button_from_sdl2() {
        assert_eq!(GamepadButton::from_sdl2(Button::A), GamepadButton::A);
        assert_eq!(GamepadButton::from_sdl2(Button::B), GamepadButton::B);
        assert_eq!(GamepadButton::from_sdl2(Button::X), GamepadButton::X);
        assert_eq!(GamepadButton::from_sdl2(Button::Y), GamepadButton::Y);
        assert_eq!(
            GamepadButton::from_sdl2(Button::Start),
            GamepadButton::Start
        );
    }

    #[test]
    fn test_gamepad_axis_from_sdl2() {
        assert_eq!(GamepadAxis::from_sdl2(Axis::LeftX), GamepadAxis::LeftX);
        assert_eq!(GamepadAxis::from_sdl2(Axis::LeftY), GamepadAxis::LeftY);
        assert_eq!(GamepadAxis::from_sdl2(Axis::RightX), GamepadAxis::RightX);
        assert_eq!(GamepadAxis::from_sdl2(Axis::RightY), GamepadAxis::RightY);
        assert_eq!(
            GamepadAxis::from_sdl2(Axis::TriggerLeft),
            GamepadAxis::TriggerLeft
        );
        assert_eq!(
            GamepadAxis::from_sdl2(Axis::TriggerRight),
            GamepadAxis::TriggerRight
        );
    }

    #[test]
    fn test_active_gamepad_selection() {
        let mut state = InputState::new();

        // Connect multiple gamepads
        state.process_event(&WindowEvent::GamepadConnected { which: 0 });
        state.process_event(&WindowEvent::GamepadConnected { which: 1 });
        state.process_event(&WindowEvent::GamepadConnected { which: 2 });

        assert_eq!(state.active_gamepad_id(), Some(0));

        // Set active gamepad
        state.set_active_gamepad(1);
        assert_eq!(state.active_gamepad_id(), Some(1));

        // Disconnect active gamepad
        state.process_event(&WindowEvent::GamepadDisconnected { which: 1 });
        // Should auto-select another available gamepad
        assert_ne!(state.active_gamepad_id(), None);
    }
}
