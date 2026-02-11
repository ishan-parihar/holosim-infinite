//! SDL2 GUI Module
//!
//! Provides SDL2-based window management, event handling, input processing, audio support, and performance profiling.
//! This module replaces Winit while preserving all WGPU rendering infrastructure.
//!
//! From SDL2_INTEGRATION_ROADMAP.md:
//! "SDL2 Native GUI Integration - Professional-grade SDL2 + WGPU visualization"
//!
//! Audio infrastructure (Day 14) provides future-proofing for emergence sounds and notifications.
//! Performance profiling (Day 15) enables optimization and benchmarking of SDL2 event loop and rendering.

#[cfg(feature = "mixer")]
pub mod audio;
pub mod event_loop;
pub mod input;
pub mod profiling;
pub mod window;

// Re-exports for convenience
#[cfg(feature = "mixer")]
pub use audio::{AudioConfig, AudioError, AudioFormat, AudioManager, GlobalAudioManager, SoundId};
pub use event_loop::{EventLoop, EventLoopBuilder, Modifiers, WindowEvent};
pub use input::{CameraInputController, InputMap, InputState};
pub use profiling::{AdaptiveVsync, EventPollingConfig, PerformanceReport, PerformanceTracker};
pub use window::{DisplayInfo, Sdl2Window, Sdl2WindowError, WindowMode, WindowState};

/// SDL2 GUI configuration
#[derive(Debug, Clone)]
pub struct Sdl2Config {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window title
    pub title: String,
    /// Enable vsync
    pub vsync: bool,
    /// Target FPS (0 = unlimited)
    pub target_fps: u32,
}

impl Default for Sdl2Config {
    fn default() -> Self {
        Sdl2Config {
            width: 1920,
            height: 1080,
            title: "Holonic Realms".to_string(),
            vsync: true,
            target_fps: 60,
        }
    }
}

impl Sdl2Config {
    /// Create new config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set window size
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set window title
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set vsync
    pub fn with_vsync(mut self, enabled: bool) -> Self {
        self.vsync = enabled;
        self
    }

    /// Set target FPS
    pub fn with_target_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }
}

/// Initialize SDL2 and create window with event loop
///
/// This is a convenience function for quick setup.
pub fn init_sdl2(config: &Sdl2Config) -> Result<(Sdl2Window, EventLoop), Sdl2WindowError> {
    let window = Sdl2Window::new(&config.title, config.width, config.height)?;

    let event_loop = EventLoop::new(window.sdl_context())
        .map_err(|e| Sdl2WindowError::InitializationFailed(e))?;

    Ok((window, event_loop))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdl2_config_default() {
        let config = Sdl2Config::default();
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.title, "Holonic Realms");
        assert!(config.vsync);
        assert_eq!(config.target_fps, 60);
    }

    #[test]
    fn test_sdl2_config_builder() {
        let config = Sdl2Config::new()
            .with_size(1280, 720)
            .with_title("Test Window")
            .with_vsync(false)
            .with_target_fps(30);

        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
        assert_eq!(config.title, "Test Window");
        assert!(!config.vsync);
        assert_eq!(config.target_fps, 30);
    }
}
