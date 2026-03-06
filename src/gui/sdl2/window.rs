//! SDL2 Window Module
//!
//! Provides SDL2-based window management with WGPU surface integration.
//! Replaces Winit window handling while preserving all WGPU rendering.
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 1:
//! "Implement SDL2 window creation with WGPU surface bridge"
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 3 Day 11:
//! "Implement fullscreen toggle, window state persistence, minimize/maximize handling, and focus tracking"
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 3 Day 12:
//! "Detect available displays, implement monitor enumeration, add window position across monitors, handle DPI/scaling changes"
//!
//! From SDL2_INTEGRATION_ROADMAP.md Phase 3 Day 13:
//! "Implement clipboard integration - Copy: Entity info, screenshots; Paste: Configuration data"

use serde::{Deserialize, Serialize};

/// Display information for multi-monitor support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    /// Display index (0-based)
    pub index: i32,
    /// Display name (e.g., "HDMI-0", "DP-1")
    pub name: Option<String>,
    /// Display bounds (x, y, width, height)
    pub bounds: (i32, i32, i32, i32),
    /// DPI scaling factor (1.0 = 96 DPI)
    pub dpi_scale: f32,
    /// Whether this is the primary display
    pub is_primary: bool,
    /// Display mode (width, height, refresh rate)
    pub display_mode: (i32, i32, i32),
}

impl DisplayInfo {
    /// Get display width
    pub fn width(&self) -> i32 {
        self.bounds.2
    }

    /// Get display height
    pub fn height(&self) -> i32 {
        self.bounds.3
    }

    /// Get display refresh rate
    pub fn refresh_rate(&self) -> i32 {
        self.display_mode.2
    }
}

/// Window display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowMode {
    /// Normal windowed mode
    Windowed,
    /// Fullscreen mode (exclusive)
    Fullscreen,
    /// Borderless fullscreen (windowed fullscreen)
    Borderless,
}

/// Window state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    /// Window position (x, y)
    pub position: (i32, i32),
    /// Window size (width, height)
    pub size: (u32, u32),
    /// Window display mode
    pub mode: WindowMode,
    /// Window is maximized
    pub maximized: bool,
    /// Window is minimized
    pub minimized: bool,
    /// Window has focus
    pub focused: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            position: (0, 0),
            size: (1920, 1080),
            mode: WindowMode::Windowed,
            maximized: false,
            minimized: false,
            focused: true,
        }
    }
}

/// SDL2 Window wrapper with WGPU surface integration
///
/// This struct bridges SDL2 window management with WGPU rendering.
/// It handles window creation, surface management, resize events,
/// fullscreen toggles, state persistence, and focus tracking.
pub struct Sdl2Window {
    /// SDL2 context (keeps SDL2 initialized)
    #[allow(dead_code)]
    sdl_context: sdl2::Sdl,

    /// Video subsystem for window operations
    #[allow(dead_code)]
    video_subsystem: sdl2::VideoSubsystem,

    /// The SDL2 window handle
    window: sdl2::video::Window,

    /// Current window dimensions
    width: u32,
    height: u32,

    /// Window title
    title: String,

    /// Current window mode
    mode: WindowMode,

    /// Window is maximized
    maximized: bool,

    /// Window is minimized
    minimized: bool,

    /// Window has input focus
    focused: bool,
}

impl Sdl2Window {
    /// Create a WGPU surface from this window
    ///
    /// This creates a new surface each time it's called. The caller is responsible
    /// for managing the surface lifecycle.
    ///
    /// # Safety
    ///
    /// The returned surface is only valid as long as this window exists.
    /// Note: Due to SDL2's internal Rc usage, this must be called from the main thread
    /// and the surface cannot be sent to other threads.
    pub unsafe fn create_surface(
        &self,
        instance: &wgpu::Instance,
    ) -> Result<wgpu::Surface<'_>, Sdl2WindowError> {
        // Use raw-window-handle to get window/display handles
        use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

        // SDL2 implements these traits when raw-window-handle feature is enabled
        let display_handle = self
            .window
            .display_handle()
            .map_err(|e| Sdl2WindowError::SurfaceCreationFailed(e.to_string()))?;

        let window_handle = self
            .window
            .window_handle()
            .map_err(|e| Sdl2WindowError::SurfaceCreationFailed(e.to_string()))?;

        // Use unsafe surface creation to bypass Send+Sync requirement
        instance
            .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: display_handle.as_raw(),
                raw_window_handle: window_handle.as_raw(),
            })
            .map_err(|e| Sdl2WindowError::SurfaceCreationFailed(e.to_string()))
    }

    /// Get the SDL2 window reference
    /// Create a new SDL2 window
    ///
    /// # Arguments
    /// * `title` - Window title
    /// * `width` - Initial window width in pixels
    /// * `height` - Initial window height in pixels
    ///
    /// # Returns
    /// Result containing the window or an error
    ///
    /// # Example
    /// ```rust,ignore
    /// let window = Sdl2Window::new("Test", 1920, 1080)?;
    /// ```
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Sdl2WindowError> {
        // Initialize SDL2
        let sdl_context = sdl2::init().map_err(Sdl2WindowError::InitializationFailed)?;

        // Get video subsystem
        let video_subsystem = sdl_context
            .video()
            .map_err(Sdl2WindowError::VideoSubsystemFailed)?;

        // Configure window
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| Sdl2WindowError::WindowCreationFailed(e.to_string()))?;

        Ok(Sdl2Window {
            sdl_context,
            video_subsystem,
            window,
            width,
            height,
            title: title.to_string(),
            mode: WindowMode::Windowed,
            maximized: false,
            minimized: false,
            focused: true,
        })
    }

    /// Get the SDL2 window reference
    pub fn window(&self) -> &sdl2::video::Window {
        &self.window
    }

    /// Get mutable reference to SDL2 window
    pub fn window_mut(&mut self) -> &mut sdl2::video::Window {
        &mut self.window
    }

    /// Get current window size
    pub fn size(&self) -> (u32, u32) {
        self.window.size()
    }

    /// Get current window width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get current window height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Update stored dimensions (call after resize)
    pub fn update_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Get window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set window title
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title).ok();
        self.title = title.to_string();
    }

    /// Check if window is visible
    pub fn is_visible(&self) -> bool {
        // SDL2 doesn't have is_shown(), but we can check window flags
        let flags = self.window.window_flags();
        (flags & sdl2::sys::SDL_WindowFlags::SDL_WINDOW_SHOWN as u32) != 0
    }

    /// Show the window
    pub fn show(&mut self) {
        self.window.show();
    }

    /// Hide the window
    pub fn hide(&mut self) {
        self.window.hide();
    }

    /// Raise window to foreground
    pub fn raise(&mut self) {
        self.window.raise();
    }

    /// Get the window ID for event matching
    pub fn id(&self) -> u32 {
        self.window.id()
    }

    /// Get the drawable size (may differ from window size on high-DPI)
    pub fn drawable_size(&self) -> (u32, u32) {
        self.window.drawable_size()
    }

    /// Get the display index this window is on
    pub fn display_index(&self) -> Result<i32, Sdl2WindowError> {
        self.window
            .display_index()
            .map_err(Sdl2WindowError::DisplayIndexFailed)
    }

    /// Get the total number of available displays
    ///
    /// # Returns
    /// The number of connected displays
    pub fn display_count(&self) -> Result<i32, Sdl2WindowError> {
        self.video_subsystem
            .num_video_displays()
            .map_err(|e| Sdl2WindowError::DisplayInfoFailed(e.to_string()))
    }

    /// Get information about a specific display
    ///
    /// # Arguments
    /// * `display_index` - The index of the display (0 to display_count-1)
    ///
    /// # Returns
    /// Display information or an error
    pub fn display_info(&self, display_index: i32) -> Result<DisplayInfo, Sdl2WindowError> {
        let display_name = self.video_subsystem.display_name(display_index).ok();

        let bounds = self
            .video_subsystem
            .display_bounds(display_index)
            .map_err(|e| Sdl2WindowError::DisplayInfoFailed(e.to_string()))?;

        let (ddpi, _hdpi, _vdpi) = self
            .video_subsystem
            .display_dpi(display_index)
            .unwrap_or((96.0, 96.0, 96.0));
        let dpi_scale = ddpi / 96.0;

        let is_primary = display_index == 0;

        let display_mode = self
            .video_subsystem
            .current_display_mode(display_index)
            .map(|mode| (mode.w, mode.h, mode.refresh_rate))
            .unwrap_or((bounds.width() as i32, bounds.height() as i32, 60));

        Ok(DisplayInfo {
            index: display_index,
            name: display_name,
            bounds: (
                bounds.x(),
                bounds.y(),
                bounds.width() as i32,
                bounds.height() as i32,
            ),
            dpi_scale,
            is_primary,
            display_mode,
        })
    }

    /// Get information about all available displays
    ///
    /// # Returns
    /// Vector of display information for all connected displays
    pub fn all_displays(&self) -> Result<Vec<DisplayInfo>, Sdl2WindowError> {
        let count = self.display_count()?;
        let mut displays = Vec::new();

        for i in 0..count {
            displays.push(self.display_info(i)?);
        }

        Ok(displays)
    }

    /// Get the primary display information
    ///
    /// # Returns
    /// Primary display information or an error
    pub fn primary_display(&self) -> Result<DisplayInfo, Sdl2WindowError> {
        self.display_info(0)
    }

    /// Move window to a specific display
    ///
    /// # Arguments
    /// * `display_index` - The index of the target display
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn set_display(&mut self, display_index: i32) -> Result<(), Sdl2WindowError> {
        let display_info = self.display_info(display_index)?;

        let bounds = display_info.bounds;
        let center_x = bounds.0 + bounds.2 / 2;
        let center_y = bounds.1 + bounds.3 / 2;

        self.window.set_position(
            sdl2::video::WindowPos::Positioned(center_x),
            sdl2::video::WindowPos::Positioned(center_y),
        );
        Ok(())
    }

    /// Center window on a specific display
    ///
    /// # Arguments
    /// * `display_index` - The index of the display to center on
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn center_on_display(&mut self, display_index: i32) -> Result<(), Sdl2WindowError> {
        let display_info = self.display_info(display_index)?;

        let bounds = display_info.bounds;
        let window_size = self.size();

        let center_x = bounds.0 + (bounds.2 - window_size.0 as i32) / 2;
        let center_y = bounds.1 + (bounds.3 - window_size.1 as i32) / 2;

        self.window.set_position(
            sdl2::video::WindowPos::Positioned(center_x),
            sdl2::video::WindowPos::Positioned(center_y),
        );
        Ok(())
    }

    /// Get the DPI scale for the current display
    ///
    /// # Returns
    /// DPI scaling factor (1.0 = 96 DPI)
    pub fn dpi_scale(&self) -> Result<f32, Sdl2WindowError> {
        let display_index = self.display_index()?;
        let (ddpi, _, _) = self
            .video_subsystem
            .display_dpi(display_index)
            .unwrap_or((96.0, 96.0, 96.0));
        Ok(ddpi / 96.0)
    }

    /// Get reference to SDL context (for creating event loop)
    pub fn sdl_context(&self) -> &sdl2::Sdl {
        &self.sdl_context
    }

    /// Get current window mode
    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    /// Set window mode (fullscreen/windowed/borderless)
    ///
    /// # Arguments
    /// * `mode` - The desired window mode
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn set_mode(&mut self, mode: WindowMode) -> Result<(), Sdl2WindowError> {
        match mode {
            WindowMode::Windowed => {
                self.window
                    .set_fullscreen(sdl2::video::FullscreenType::Off)
                    .map_err(|e| Sdl2WindowError::FullscreenFailed(e.to_string()))?;
            }
            WindowMode::Fullscreen => {
                self.window
                    .set_fullscreen(sdl2::video::FullscreenType::True)
                    .map_err(|e| Sdl2WindowError::FullscreenFailed(e.to_string()))?;
            }
            WindowMode::Borderless => {
                self.window
                    .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                    .map_err(|e| Sdl2WindowError::FullscreenFailed(e.to_string()))?;
            }
        }
        self.mode = mode;

        if mode == WindowMode::Windowed {
            self.maximized = false;
        }

        Ok(())
    }

    /// Toggle between windowed and fullscreen modes
    ///
    /// Uses exclusive fullscreen when toggling from windowed
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn toggle_fullscreen(&mut self) -> Result<(), Sdl2WindowError> {
        let new_mode = match self.mode {
            WindowMode::Windowed => WindowMode::Fullscreen,
            WindowMode::Fullscreen | WindowMode::Borderless => WindowMode::Windowed,
        };
        self.set_mode(new_mode)
    }

    /// Toggle between windowed and borderless fullscreen
    ///
    /// Uses borderless fullscreen when toggling from windowed
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn toggle_borderless_fullscreen(&mut self) -> Result<(), Sdl2WindowError> {
        let new_mode = match self.mode {
            WindowMode::Windowed => WindowMode::Borderless,
            WindowMode::Fullscreen | WindowMode::Borderless => WindowMode::Windowed,
        };
        self.set_mode(new_mode)
    }

    /// Check if window is maximized
    pub fn is_maximized(&self) -> bool {
        self.maximized
    }

    /// Maximize the window
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn maximize(&mut self) -> Result<(), Sdl2WindowError> {
        self.window.maximize();
        self.maximized = true;
        Ok(())
    }

    /// Restore the window from maximized/minimized state
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn restore(&mut self) -> Result<(), Sdl2WindowError> {
        self.window.restore();
        self.maximized = false;
        self.minimized = false;
        Ok(())
    }

    /// Check if window is minimized
    pub fn is_minimized(&self) -> bool {
        self.minimized
    }

    /// Minimize the window
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn minimize(&mut self) -> Result<(), Sdl2WindowError> {
        self.window.minimize();
        self.minimized = true;
        Ok(())
    }

    /// Check if window has input focus
    pub fn has_focus(&self) -> bool {
        self.focused
    }

    /// Update focus state (should be called from event loop)
    pub fn update_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Get current window state for persistence
    pub fn get_state(&self) -> WindowState {
        WindowState {
            position: self.window.position(),
            size: self.size(),
            mode: self.mode,
            maximized: self.maximized,
            minimized: self.minimized,
            focused: self.focused,
        }
    }

    /// Restore window from saved state
    ///
    /// # Arguments
    /// * `state` - The window state to restore
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn restore_state(&mut self, state: &WindowState) -> Result<(), Sdl2WindowError> {
        self.set_mode(state.mode)?;

        if state.mode == WindowMode::Windowed {
            self.window.set_position(
                sdl2::video::WindowPos::Positioned(state.position.0),
                sdl2::video::WindowPos::Positioned(state.position.1),
            );
            let _ = self.window.set_size(state.size.0, state.size.1);
        }

        if state.maximized && !self.maximized {
            self.maximize()?;
        } else if !state.maximized && self.maximized {
            self.restore()?;
        }

        if state.minimized && !self.minimized {
            self.minimize()?;
        } else if !state.minimized && self.minimized {
            self.restore()?;
        }

        Ok(())
    }

    /// Save window state to JSON file
    ///
    /// # Arguments
    /// * `path` - Path to save the window state
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn save_state_to_file(&self, path: &std::path::Path) -> Result<(), Sdl2WindowError> {
        let state = self.get_state();
        let json = serde_json::to_string_pretty(&state)
            .map_err(|e| Sdl2WindowError::StateSaveFailed(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| Sdl2WindowError::StateSaveFailed(e.to_string()))?;
        Ok(())
    }

    /// Load window state from JSON file
    ///
    /// # Arguments
    /// * `path` - Path to load the window state from
    ///
    /// # Returns
    /// Result containing the window state or an error
    pub fn load_state_from_file(path: &std::path::Path) -> Result<WindowState, Sdl2WindowError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| Sdl2WindowError::StateLoadFailed(e.to_string()))?;
        let state: WindowState = serde_json::from_str(&json)
            .map_err(|e| Sdl2WindowError::StateLoadFailed(e.to_string()))?;
        Ok(state)
    }

    /// Set clipboard text
    ///
    /// Copies the specified text to the system clipboard.
    ///
    /// # Arguments
    /// * `text` - The text to copy to the clipboard
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// # Example
    /// ```rust,ignore
    /// window.set_clipboard_text("Hello, World!").unwrap();
    /// ```
    ///
    /// From SDL2_INTEGRATION_ROADMAP.md Day 13:
    /// "Implement clipboard integration - Copy: Entity info, screenshots"
    pub fn set_clipboard_text(&mut self, text: &str) -> Result<(), Sdl2WindowError> {
        self.video_subsystem
            .clipboard()
            .set_clipboard_text(text)
            .map_err(|e| Sdl2WindowError::ClipboardFailed(e.to_string()))
    }

    /// Get clipboard text
    ///
    /// Retrieves the current text from the system clipboard.
    ///
    /// # Returns
    /// Result containing the clipboard text or an error
    ///
    /// # Example
    /// ```rust,ignore
    /// let text = window.get_clipboard_text().unwrap();
    /// println!("Clipboard: {}", text);
    /// ```
    ///
    /// From SDL2_INTEGRATION_ROADMAP.md Day 13:
    /// "Implement clipboard integration - Paste: Configuration data"
    pub fn get_clipboard_text(&self) -> Result<String, Sdl2WindowError> {
        self.video_subsystem
            .clipboard()
            .clipboard_text()
            .map_err(|e| Sdl2WindowError::ClipboardFailed(e.to_string()))
    }

    /// Check if clipboard has text content
    ///
    /// # Returns
    /// Result indicating whether clipboard has text or an error
    pub fn has_clipboard_text(&self) -> Result<bool, Sdl2WindowError> {
        let text = self.get_clipboard_text()?;
        Ok(!text.is_empty())
    }

    /// Copy entity information to clipboard
    ///
    /// Formats entity data as JSON and copies to clipboard.
    ///
    /// # Arguments
    /// * `entity_id` - The entity ID
    /// * `entity_data` - The entity data as a serializable value
    ///
    /// # Returns
    /// Result indicating success or failure
    ///
    /// From SDL2_INTEGRATION_ROADMAP.md Day 13:
    /// "Copy: Entity info, screenshots"
    pub fn copy_entity_info<T: serde::Serialize>(
        &mut self,
        entity_id: u64,
        entity_data: &T,
    ) -> Result<(), Sdl2WindowError> {
        let info = serde_json::json!({
            "entity_id": entity_id,
            "data": entity_data,
        });

        let text = serde_json::to_string_pretty(&info)
            .map_err(|e| Sdl2WindowError::ClipboardFailed(e.to_string()))?;

        self.set_clipboard_text(&text)
    }

    /// Paste configuration data from clipboard
    ///
    /// Attempts to parse clipboard content as JSON configuration.
    ///
    /// # Returns
    /// Result containing the parsed configuration or an error
    ///
    /// From SDL2_INTEGRATION_ROADMAP.md Day 13:
    /// "Paste: Configuration data"
    pub fn paste_configuration(&self) -> Result<serde_json::Value, Sdl2WindowError> {
        let text = self.get_clipboard_text()?;
        serde_json::from_str(&text)
            .map_err(|e| Sdl2WindowError::ClipboardFailed(format!("Invalid JSON: {}", e)))
    }
}

/// Errors that can occur during SDL2 window operations
#[derive(Debug, Clone, PartialEq)]
pub enum Sdl2WindowError {
    /// SDL2 initialization failed
    InitializationFailed(String),

    /// Video subsystem unavailable
    VideoSubsystemFailed(String),

    /// Window creation failed
    WindowCreationFailed(String),

    /// WGPU surface creation failed
    SurfaceCreationFailed(String),

    /// Display index query failed
    DisplayIndexFailed(String),

    /// Display information query failed
    DisplayInfoFailed(String),

    /// Fullscreen mode change failed
    FullscreenFailed(String),

    /// State save failed
    StateSaveFailed(String),

    /// State load failed
    StateLoadFailed(String),

    /// Clipboard operation failed
    ClipboardFailed(String),
}

impl std::fmt::Display for Sdl2WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sdl2WindowError::InitializationFailed(msg) => {
                write!(f, "SDL2 initialization failed: {}", msg)
            }
            Sdl2WindowError::VideoSubsystemFailed(msg) => {
                write!(f, "SDL2 video subsystem failed: {}", msg)
            }
            Sdl2WindowError::WindowCreationFailed(msg) => {
                write!(f, "Window creation failed: {}", msg)
            }
            Sdl2WindowError::SurfaceCreationFailed(msg) => {
                write!(f, "WGPU surface creation failed: {}", msg)
            }
            Sdl2WindowError::DisplayIndexFailed(msg) => {
                write!(f, "Display index query failed: {}", msg)
            }
            Sdl2WindowError::DisplayInfoFailed(msg) => {
                write!(f, "Display information query failed: {}", msg)
            }
            Sdl2WindowError::FullscreenFailed(msg) => {
                write!(f, "Fullscreen mode change failed: {}", msg)
            }
            Sdl2WindowError::StateSaveFailed(msg) => {
                write!(f, "Window state save failed: {}", msg)
            }
            Sdl2WindowError::StateLoadFailed(msg) => {
                write!(f, "Window state load failed: {}", msg)
            }
            Sdl2WindowError::ClipboardFailed(msg) => {
                write!(f, "Clipboard operation failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for Sdl2WindowError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdl2_window_error_display() {
        let err = Sdl2WindowError::InitializationFailed("test".to_string());
        assert!(err.to_string().contains("SDL2 initialization failed"));
    }

    #[test]
    fn test_window_mode_equality() {
        assert_eq!(WindowMode::Windowed, WindowMode::Windowed);
        assert_eq!(WindowMode::Fullscreen, WindowMode::Fullscreen);
        assert_eq!(WindowMode::Borderless, WindowMode::Borderless);
        assert_ne!(WindowMode::Windowed, WindowMode::Fullscreen);
        assert_ne!(WindowMode::Fullscreen, WindowMode::Borderless);
    }

    #[test]
    fn test_window_state_default() {
        let state = WindowState::default();
        assert_eq!(state.position, (0, 0));
        assert_eq!(state.size, (1920, 1080));
        assert_eq!(state.mode, WindowMode::Windowed);
        assert!(!state.maximized);
        assert!(!state.minimized);
        assert!(state.focused);
    }

    #[test]
    fn test_window_state_serialization() {
        let state = WindowState {
            position: (100, 200),
            size: (1280, 720),
            mode: WindowMode::Borderless,
            maximized: true,
            minimized: false,
            focused: false,
        };

        let json = serde_json::to_string(&state).unwrap();
        let restored: WindowState = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.position, state.position);
        assert_eq!(restored.size, state.size);
        assert_eq!(restored.mode, state.mode);
        assert_eq!(restored.maximized, state.maximized);
        assert_eq!(restored.minimized, state.minimized);
        assert_eq!(restored.focused, state.focused);
    }

    #[test]
    fn test_sdl2_window_creation_with_state() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert_eq!(window.mode(), WindowMode::Windowed);
        assert!(!window.is_maximized());
        assert!(!window.is_minimized());
        assert!(window.has_focus());
    }

    #[test]
    fn test_sdl2_window_get_state() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();
        let state = window.get_state();

        assert_eq!(state.size, (800, 600));
        assert_eq!(state.mode, WindowMode::Windowed);
        assert!(!state.maximized);
        assert!(!state.minimized);
    }

    #[test]
    fn test_sdl2_window_fullscreen_toggle() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert_eq!(window.mode(), WindowMode::Windowed);

        window.toggle_fullscreen().unwrap();
        assert_eq!(window.mode(), WindowMode::Fullscreen);

        window.toggle_fullscreen().unwrap();
        assert_eq!(window.mode(), WindowMode::Windowed);
    }

    #[test]
    fn test_sdl2_window_borderless_fullscreen_toggle() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert_eq!(window.mode(), WindowMode::Windowed);

        window.toggle_borderless_fullscreen().unwrap();
        assert_eq!(window.mode(), WindowMode::Borderless);

        window.toggle_borderless_fullscreen().unwrap();
        assert_eq!(window.mode(), WindowMode::Windowed);
    }

    #[test]
    fn test_sdl2_window_set_mode() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.set_mode(WindowMode::Fullscreen).unwrap();
        assert_eq!(window.mode(), WindowMode::Fullscreen);

        window.set_mode(WindowMode::Borderless).unwrap();
        assert_eq!(window.mode(), WindowMode::Borderless);

        window.set_mode(WindowMode::Windowed).unwrap();
        assert_eq!(window.mode(), WindowMode::Windowed);
    }

    #[test]
    fn test_sdl2_window_maximize() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert!(!window.is_maximized());

        window.maximize().unwrap();
        assert!(window.is_maximized());

        window.restore().unwrap();
        assert!(!window.is_maximized());
    }

    #[test]
    fn test_sdl2_window_minimize() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert!(!window.is_minimized());

        window.minimize().unwrap();
        assert!(window.is_minimized());

        window.restore().unwrap();
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_sdl2_window_focus_tracking() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert!(window.has_focus());

        window.update_focus(false);
        assert!(!window.has_focus());

        window.update_focus(true);
        assert!(window.has_focus());
    }

    #[test]
    fn test_window_state_file_io() {
        let state = WindowState {
            position: (150, 250),
            size: (1024, 768),
            mode: WindowMode::Fullscreen,
            maximized: false,
            minimized: true,
            focused: true,
        };

        let temp_path = std::path::Path::new("/tmp/test_window_state.json");

        // Save state directly as JSON (WindowState is serializable)
        let json = serde_json::to_string_pretty(&state).unwrap();
        std::fs::write(temp_path, json).unwrap();

        let loaded_state = Sdl2Window::load_state_from_file(temp_path).unwrap();

        assert_eq!(loaded_state.position, state.position);
        assert_eq!(loaded_state.size, state.size);
        assert_eq!(loaded_state.mode, state.mode);
        assert_eq!(loaded_state.maximized, state.maximized);
        assert_eq!(loaded_state.minimized, state.minimized);
        assert_eq!(loaded_state.focused, state.focused);

        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn test_display_info_structure() {
        let display_info = DisplayInfo {
            index: 0,
            name: Some("HDMI-0".to_string()),
            bounds: (0, 0, 1920, 1080),
            dpi_scale: 1.0,
            is_primary: true,
            display_mode: (1920, 1080, 60),
        };

        assert_eq!(display_info.index, 0);
        assert_eq!(display_info.name, Some("HDMI-0".to_string()));
        assert_eq!(display_info.width(), 1920);
        assert_eq!(display_info.height(), 1080);
        assert_eq!(display_info.refresh_rate(), 60);
        assert_eq!(display_info.dpi_scale, 1.0);
        assert!(display_info.is_primary);
    }

    #[test]
    fn test_display_info_serialization() {
        let display_info = DisplayInfo {
            index: 1,
            name: Some("DP-1".to_string()),
            bounds: (1920, 0, 2560, 1440),
            dpi_scale: 1.5,
            is_primary: false,
            display_mode: (2560, 1440, 144),
        };

        let json = serde_json::to_string(&display_info).unwrap();
        let restored: DisplayInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.index, display_info.index);
        assert_eq!(restored.name, display_info.name);
        assert_eq!(restored.bounds, display_info.bounds);
        assert_eq!(restored.dpi_scale, display_info.dpi_scale);
        assert_eq!(restored.is_primary, display_info.is_primary);
        assert_eq!(restored.display_mode, display_info.display_mode);
    }

    #[test]
    fn test_sdl2_window_display_count() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let count = window.display_count().unwrap();
        assert!(count >= 1);
    }

    #[test]
    fn test_sdl2_window_display_info() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let display_info = window.display_info(0).unwrap();
        assert_eq!(display_info.index, 0);
        assert!(display_info.width() > 0);
        assert!(display_info.height() > 0);
        assert!(display_info.dpi_scale > 0.0);
        assert!(display_info.refresh_rate() > 0);
    }

    #[test]
    fn test_sdl2_window_all_displays() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let displays = window.all_displays().unwrap();
        assert!(!displays.is_empty());
        assert_eq!(displays[0].index, 0);
        assert!(displays[0].is_primary);
    }

    #[test]
    fn test_sdl2_window_primary_display() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let primary = window.primary_display().unwrap();
        assert_eq!(primary.index, 0);
        assert!(primary.is_primary);
    }

    #[test]
    fn test_sdl2_window_display_index() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let display_index = window.display_index().unwrap();
        assert!(display_index >= 0);
    }

    #[test]
    fn test_sdl2_window_dpi_scale() {
        let window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let dpi_scale = window.dpi_scale().unwrap();
        assert!(dpi_scale > 0.0);
    }

    #[test]
    fn test_sdl2_window_set_display() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let count = window.display_count().unwrap();
        if count > 0 {
            window.set_display(0).unwrap();
        }
    }

    #[test]
    fn test_sdl2_window_center_on_display() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.center_on_display(0).unwrap();

        let position = window.window().position();
        assert!(position.0 >= 0);
        assert!(position.1 >= 0);
    }

    #[test]
    fn test_clipboard_set_text() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.set_clipboard_text("Hello, World!").unwrap();
    }

    #[test]
    fn test_clipboard_get_text() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.set_clipboard_text("Test clipboard content").unwrap();
        let text = window.get_clipboard_text().unwrap();

        assert_eq!(text, "Test clipboard content");
    }

    #[test]
    fn test_clipboard_has_text() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        assert!(!window.has_clipboard_text().unwrap_or(true));

        window.set_clipboard_text("Some text").unwrap();
        assert!(window.has_clipboard_text().unwrap());
    }

    #[test]
    fn test_clipboard_unicode() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let unicode_text = "Hello 世界 🌍";
        window.set_clipboard_text(unicode_text).unwrap();

        let retrieved = window.get_clipboard_text().unwrap();
        assert_eq!(retrieved, unicode_text);
    }

    #[test]
    fn test_clipboard_empty_string() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.set_clipboard_text("").unwrap();
        let text = window.get_clipboard_text().unwrap();

        assert_eq!(text, "");
    }

    #[test]
    fn test_clipboard_long_text() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let long_text = "A".repeat(10000);
        window.set_clipboard_text(&long_text).unwrap();

        let retrieved = window.get_clipboard_text().unwrap();
        assert_eq!(retrieved.len(), 10000);
    }

    #[test]
    fn test_copy_entity_info() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let entity_data = serde_json::json!({
            "name": "Test Entity",
            "position": [1.0, 2.0, 3.0],
            "velocity": [0.1, 0.2, 0.3],
        });

        window.copy_entity_info(12345, &entity_data).unwrap();

        let clipboard_text = window.get_clipboard_text().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&clipboard_text).unwrap();

        assert_eq!(parsed["entity_id"], 12345);
        assert_eq!(parsed["data"]["name"], "Test Entity");
    }

    #[test]
    fn test_paste_configuration() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        let config = serde_json::json!({
            "window_size": [1920, 1080],
            "fullscreen": false,
            "theme": "dark",
        });

        let config_text = serde_json::to_string(&config).unwrap();
        window.set_clipboard_text(&config_text).unwrap();

        let pasted = window.paste_configuration().unwrap();
        assert_eq!(pasted["window_size"], config["window_size"]);
        assert_eq!(pasted["fullscreen"], config["fullscreen"]);
        assert_eq!(pasted["theme"], config["theme"]);
    }

    #[test]
    fn test_paste_configuration_invalid_json() {
        let mut window = Sdl2Window::new("Test Window", 800, 600).unwrap();

        window.set_clipboard_text("not valid json").unwrap();

        let result = window.paste_configuration();
        assert!(result.is_err());
    }
}
