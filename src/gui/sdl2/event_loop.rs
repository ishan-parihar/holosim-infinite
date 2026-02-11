//! SDL2 Event Loop Module
//!
//! Provides SDL2-based event polling and dispatching.
//! Replaces Winit event loop while maintaining similar event structure.
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 1:
//! "Implement SDL2 event polling loop with quit and resize handling"

use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

/// Gamepad button mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad,
    Unknown(u8),
}

impl GamepadButton {
    pub fn from_sdl2(button: Button) -> Self {
        match button {
            Button::A => GamepadButton::A,
            Button::B => GamepadButton::B,
            Button::X => GamepadButton::X,
            Button::Y => GamepadButton::Y,
            Button::Back => GamepadButton::Back,
            Button::Guide => GamepadButton::Guide,
            Button::Start => GamepadButton::Start,
            Button::LeftStick => GamepadButton::LeftStick,
            Button::RightStick => GamepadButton::RightStick,
            Button::LeftShoulder => GamepadButton::LeftShoulder,
            Button::RightShoulder => GamepadButton::RightShoulder,
            Button::DPadUp => GamepadButton::DPadUp,
            Button::DPadDown => GamepadButton::DPadDown,
            Button::DPadLeft => GamepadButton::DPadLeft,
            Button::DPadRight => GamepadButton::DPadRight,
            Button::Misc1 => GamepadButton::Misc1,
            Button::Paddle1 => GamepadButton::Paddle1,
            Button::Paddle2 => GamepadButton::Paddle2,
            Button::Paddle3 => GamepadButton::Paddle3,
            Button::Paddle4 => GamepadButton::Paddle4,
            Button::Touchpad => GamepadButton::Touchpad,
            _ => GamepadButton::Unknown(0),
        }
    }
}

/// Gamepad axis mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
    Unknown(u8),
}

impl GamepadAxis {
    pub fn from_sdl2(axis: Axis) -> Self {
        match axis {
            Axis::LeftX => GamepadAxis::LeftX,
            Axis::LeftY => GamepadAxis::LeftY,
            Axis::RightX => GamepadAxis::RightX,
            Axis::RightY => GamepadAxis::RightY,
            Axis::TriggerLeft => GamepadAxis::TriggerLeft,
            Axis::TriggerRight => GamepadAxis::TriggerRight,
            _ => GamepadAxis::Unknown(0),
        }
    }
}

/// Internal event types for the simulation
#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    /// Window should close
    CloseRequested,

    /// Window resized
    Resized { width: u32, height: u32 },

    /// Window gained focus
    FocusGained,

    /// Window lost focus
    FocusLost,

    /// Window minimized
    Minimized,

    /// Window maximized
    Maximized,

    /// Window restored from minimized or maximized
    Restored,

    /// Window moved
    Moved { x: i32, y: i32 },

    /// Keyboard key pressed
    KeyPressed {
        key: Keycode,
        mods: Modifiers,
        repeat: bool,
    },

    /// Keyboard key released
    KeyReleased { key: Keycode, mods: Modifiers },

    /// Text input (for typing)
    TextInput { text: String },

    /// Mouse moved
    MouseMoved { x: i32, y: i32, dx: i32, dy: i32 },

    /// Mouse button pressed
    MousePressed { button: MouseButton, x: i32, y: i32 },

    /// Mouse button released
    MouseReleased { button: MouseButton, x: i32, y: i32 },

    /// Mouse wheel scrolled
    MouseScrolled { dx: i32, dy: i32 },

    /// Gamepad connected
    GamepadConnected { which: u32 },

    /// Gamepad disconnected
    GamepadDisconnected { which: u32 },

    /// Gamepad button pressed
    GamepadPressed { which: u32, button: GamepadButton },

    /// Gamepad button released
    GamepadReleased { which: u32, button: GamepadButton },

    /// Gamepad axis moved (analog stick or trigger)
    GamepadAxisMoved {
        which: u32,
        axis: GamepadAxis,
        value: i16,
    },

    /// No event (event pump empty)
    None,
}

/// Modifier keys state
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl Modifiers {
    /// Create modifiers from SDL2 keymod
    pub fn from_sdl2(keymod: sdl2::keyboard::Mod) -> Self {
        Modifiers {
            shift: keymod
                .intersects(sdl2::keyboard::Mod::LSHIFTMOD | sdl2::keyboard::Mod::RSHIFTMOD),
            ctrl: keymod.intersects(sdl2::keyboard::Mod::LCTRLMOD | sdl2::keyboard::Mod::RCTRLMOD),
            alt: keymod.intersects(sdl2::keyboard::Mod::LALTMOD | sdl2::keyboard::Mod::RALTMOD),
            logo: keymod.intersects(sdl2::keyboard::Mod::LGUIMOD | sdl2::keyboard::Mod::RGUIMOD),
        }
    }
}

/// SDL2 Event Loop
///
/// Polls SDL2 events and converts them to internal WindowEvent types.
/// This provides a clean abstraction over SDL2's event system.
pub struct EventLoop {
    event_pump: sdl2::EventPump,
    should_quit: bool,
}

impl EventLoop {
    /// Create a new event loop from SDL2 context
    ///
    /// # Arguments
    /// * `sdl_context` - The SDL2 context
    ///
    /// # Returns
    /// Result containing the event loop or an error
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;

        Ok(EventLoop {
            event_pump,
            should_quit: false,
        })
    }

    /// Poll for the next event (non-blocking)
    ///
    /// Returns immediately with the next event or WindowEvent::None
    pub fn poll_event(&mut self) -> WindowEvent {
        match self.event_pump.poll_event() {
            Some(event) => self.convert_event(event),
            None => WindowEvent::None,
        }
    }

    /// Wait for the next event (blocking)
    ///
    /// Blocks until an event is available
    pub fn wait_event(&mut self) -> WindowEvent {
        let event = self.event_pump.wait_event();
        self.convert_event(event)
    }

    /// Check if quit was requested
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Set quit flag manually
    pub fn request_quit(&mut self) {
        self.should_quit = true;
    }

    /// Convert SDL2 event to internal WindowEvent
    fn convert_event(&mut self, event: Event) -> WindowEvent {
        match event {
            // Window events
            Event::Quit { .. } => {
                self.should_quit = true;
                WindowEvent::CloseRequested
            }

            Event::Window { win_event, .. } => match win_event {
                sdl2::event::WindowEvent::Close => {
                    self.should_quit = true;
                    WindowEvent::CloseRequested
                }
                sdl2::event::WindowEvent::Resized(width, height) => WindowEvent::Resized {
                    width: width as u32,
                    height: height as u32,
                },
                sdl2::event::WindowEvent::FocusGained => WindowEvent::FocusGained,
                sdl2::event::WindowEvent::FocusLost => WindowEvent::FocusLost,
                sdl2::event::WindowEvent::Minimized => WindowEvent::Minimized,
                sdl2::event::WindowEvent::Maximized => WindowEvent::Maximized,
                sdl2::event::WindowEvent::Restored => WindowEvent::Restored,
                sdl2::event::WindowEvent::Moved(x, y) => WindowEvent::Moved { x, y },
                _ => WindowEvent::None,
            },

            // Keyboard events
            Event::KeyDown {
                keycode: Some(key),
                keymod,
                repeat,
                ..
            } => {
                // Check for quit shortcut (ESC key)
                if key == Keycode::Escape {
                    self.should_quit = true;
                }

                WindowEvent::KeyPressed {
                    key,
                    mods: Modifiers::from_sdl2(keymod),
                    repeat,
                }
            }

            Event::KeyUp {
                keycode: Some(key),
                keymod,
                ..
            } => WindowEvent::KeyReleased {
                key,
                mods: Modifiers::from_sdl2(keymod),
            },

            // Mouse events
            Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => WindowEvent::MouseMoved {
                x,
                y,
                dx: xrel,
                dy: yrel,
            },

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => WindowEvent::MousePressed {
                button: mouse_btn,
                x,
                y,
            },

            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => WindowEvent::MouseReleased {
                button: mouse_btn,
                x,
                y,
            },

            Event::MouseWheel { x, y, .. } => WindowEvent::MouseScrolled { dx: x, dy: y },

            // Text input events
            Event::TextInput { text, .. } => WindowEvent::TextInput {
                text: text.to_string(),
            },

            // Gamepad events
            Event::ControllerDeviceAdded { which, .. } => WindowEvent::GamepadConnected { which },
            Event::ControllerDeviceRemoved { which, .. } => {
                WindowEvent::GamepadDisconnected { which }
            }
            Event::ControllerButtonDown { which, button, .. } => WindowEvent::GamepadPressed {
                which,
                button: GamepadButton::from_sdl2(button),
            },
            Event::ControllerButtonUp { which, button, .. } => WindowEvent::GamepadReleased {
                which,
                button: GamepadButton::from_sdl2(button),
            },
            Event::ControllerAxisMotion {
                which, axis, value, ..
            } => WindowEvent::GamepadAxisMoved {
                which,
                axis: GamepadAxis::from_sdl2(axis),
                value,
            },

            _ => WindowEvent::None,
        }
    }

    /// Pump all pending events and return them as a vector
    pub fn pump_events(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();

        while let Some(event) = self.event_pump.poll_event() {
            let window_event = self.convert_event(event);
            if window_event != WindowEvent::None {
                events.push(window_event);
            }
        }

        events
    }

    /// Get mutable reference to event pump (for advanced usage)
    pub fn event_pump(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }
}

/// Event loop builder for configuration
pub struct EventLoopBuilder;

impl EventLoopBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        EventLoopBuilder
    }

    /// Build event loop from SDL2 context
    pub fn build(self, sdl_context: &sdl2::Sdl) -> Result<EventLoop, String> {
        EventLoop::new(sdl_context)
    }
}

impl Default for EventLoopBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifiers_default() {
        let mods = Modifiers::default();
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.logo);
    }

    #[test]
    fn test_modifiers_from_sdl2() {
        let mods = Modifiers::from_sdl2(sdl2::keyboard::Mod::LSHIFTMOD);
        assert!(mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.logo);

        let mods =
            Modifiers::from_sdl2(sdl2::keyboard::Mod::LSHIFTMOD | sdl2::keyboard::Mod::LCTRLMOD);
        assert!(mods.shift);
        assert!(mods.ctrl);
    }

    #[test]
    fn test_window_event_equality() {
        let e1 = WindowEvent::Resized {
            width: 1920,
            height: 1080,
        };
        let e2 = WindowEvent::Resized {
            width: 1920,
            height: 1080,
        };
        let e3 = WindowEvent::Resized {
            width: 1280,
            height: 720,
        };

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }

    #[test]
    fn test_event_loop_builder() {
        // Note: Can't actually test without SDL2 context
        // This tests the API is correct
        let _builder = EventLoopBuilder::new();
    }
}
