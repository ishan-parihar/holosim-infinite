//! GUI Module - Multi-scale visualization with time/space control
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 5:
//! "Build multi-scale visualization with time/space control."
//!
//! This module provides:
//! - Multi-scale rendering engine (10^-35 to 10^26 m)
//! - Time rate control (0.1x to 1000x)
//! - Focus-based dilation (where you look, time flows differently)
//! - Interactive entity inspection
//! - Real-time emergence visualization
//!
//! Technology Stack:
//! - Rendering: WGPU (WebGPU) for high-performance GPU rendering
//! - UI Framework: EGUI for Rust native UI
//! - Windowing: Winit for cross-platform window management
//! - Math: GLM for 3D transformations
//! - Async: Tokio for async simulation updates

pub mod camera;
pub mod interaction;
pub mod interaction_system;
pub mod renderer;
pub mod scene;
pub mod space_controller;
pub mod time_controller;
pub mod view_system;
pub mod ui;
pub mod visualization;
pub mod visualization_engine;

// Phase 6: Integration & Testing
pub mod application;
pub mod demo_scenarios;
pub mod profiler;
pub mod simulation_integration;

// SDL2 Windowing System (replaces Winit)
// Temporarily disabled due to WGPU Send+Sync requirements
#[cfg(feature = "sdl2")]
pub mod sdl2;

// Re-exports for convenience
pub use camera::camera::Camera as Camera2D;
pub use camera::controls::CameraControls;
pub use interaction::BookmarkManager as CameraBookmarks;
pub use interaction_system::{
    ArchetypeActivation, CatalystStatus, ClickResult, EntityInfo, EntityInspector, EntityTracker,
    InteractionSystem, InteractionSystemBuilder, Raycaster,
};
pub use space_controller::{
    NavigationState, ScaleTransition, SpaceController, SpaceControllerBuilder, SpatialFocus,
};
pub use time_controller::{
    FocusDilation, SimulationTime, TimeController, TimeControllerBuilder, TimelineState,
};
pub use visualization_engine::{
    Camera, EntityRenderData, InvolutionState, MultiScaleCamera, Renderable, RenderableEntity,
    RenderablePlanet, RenderableStar, RenderableSystem, SceneGraph, VisualizationEngine,
    VisualizationEngineBuilder, WgpuRenderer,
};

// Phase 6: Integration & Testing exports
pub use application::{GuiApplication, GuiApplicationBuilder, RenderStats};
pub use demo_scenarios::{
    DemoScenario, DemoScenarioManager, DifficultyLevel, ScenarioBookmark, ScenarioCategory,
    ScenarioConfig,
};
pub use profiler::{
    Bottleneck, FramePercentiles, GpuProfiler, MemoryProfiler, PerformanceProfiler,
    PerformanceSnapshot, ProfileCategory,
};
pub use simulation_integration::{
    CollectiveRenderData, EmergenceMetrics as SimEmergenceMetrics,
    EntityRenderData as SimEntityRenderData, EventBridge, SimulationEvent,
    SimulationGuiIntegration, SpectrumData as SimSpectrumData,
};

/// Configuration for the GUI system
#[derive(Debug, Clone)]
pub struct GuiConfig {
    /// Window width in pixels
    pub window_width: u32,

    /// Window height in pixels
    pub window_height: u32,

    /// Initial time rate (1.0 = normal speed)
    pub initial_time_rate: f64,

    /// Minimum time rate
    pub min_time_rate: f64,

    /// Maximum time rate
    pub max_time_rate: f64,

    /// Initial zoom level (meters)
    pub initial_zoom: f64,

    /// Minimum zoom level (10^-35 m - Planck length)
    pub min_zoom: f64,

    /// Maximum zoom level (10^26 m - observable universe)
    pub max_zoom: f64,

    /// Enable focus-based time dilation
    pub enable_focus_dilation: bool,

    /// Multi-sampling anti-aliasing samples
    pub msaa_samples: u32,

    /// Enable vsync
    pub enable_vsync: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        GuiConfig {
            window_width: 1920,
            window_height: 1080,
            initial_time_rate: 1.0,
            min_time_rate: 0.1,
            max_time_rate: 1000.0,
            initial_zoom: 1e-6,     // Start at cellular scale
            min_zoom: 1.616255e-35, // Planck length in meters
            max_zoom: 8.8e26,       // Observable universe radius in meters
            enable_focus_dilation: true,
            msaa_samples: 4,
            enable_vsync: true,
        }
    }
}

impl GuiConfig {
    /// Create a new GUI config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to set window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    /// Builder method to set time rate range
    pub fn with_time_rate_range(mut self, min: f64, max: f64) -> Self {
        self.min_time_rate = min;
        self.max_time_rate = max;
        self
    }

    /// Builder method to set initial time rate
    pub fn with_initial_rate(mut self, rate: f64) -> Self {
        self.initial_time_rate = rate;
        self
    }

    /// Builder method to set zoom range
    pub fn with_zoom_range(mut self, min: f64, max: f64) -> Self {
        self.min_zoom = min;
        self.max_zoom = max;
        self
    }

    /// Builder method to set initial zoom
    pub fn with_initial_zoom(mut self, zoom: f64) -> Self {
        self.initial_zoom = zoom;
        self
    }

    /// Enable or disable focus-based time dilation
    pub fn with_focus_dilation(mut self, enable: bool) -> Self {
        self.enable_focus_dilation = enable;
        self
    }

    /// Set MSAA samples
    pub fn with_msaa(mut self, samples: u32) -> Self {
        self.msaa_samples = samples;
        self
    }

    /// Enable or disable vsync
    pub fn with_vsync(mut self, enable: bool) -> Self {
        self.enable_vsync = enable;
        self
    }
}

/// Scale levels for multi-scale visualization
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Multi-Scale Visualization Engine:
/// - Quantum scale (10^-35 m)
/// - Atomic scale (10^-10 m)
/// - Molecular scale (10^-9 m)
/// - Cellular scale (10^-6 m)
/// - Organism scale (10^-3 m)
/// - Planetary scale (10^6 m)
/// - Stellar scale (10^9 m)
/// - Galactic scale (10^21 m)
/// - Universal scale (10^26 m)"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScaleLevel {
    /// Quantum scale (10^-35 m) - Planck scale
    Quantum,

    /// Atomic scale (10^-10 m) - Subatomic particles
    Atomic,

    /// Molecular scale (10^-9 m) - Atoms and molecules
    Molecular,

    /// Cellular scale (10^-6 m) - Biological cells
    Cellular,

    /// Organism scale (10^-3 m) - Life forms
    Organism,

    /// Planetary scale (10^6 m) - Planets and moons
    Planetary,

    /// Stellar scale (10^9 m) - Stars and solar systems
    Stellar,

    /// Galactic scale (10^21 m) - Galaxies
    Galactic,

    /// Universal scale (10^26 m) - Observable universe
    Universal,
}

impl ScaleLevel {
    /// Get the approximate scale in meters for this level
    pub fn scale_in_meters(&self) -> f64 {
        match self {
            ScaleLevel::Quantum => 1.0e-35,
            ScaleLevel::Atomic => 1.0e-10,
            ScaleLevel::Molecular => 1.0e-9,
            ScaleLevel::Cellular => 1.0e-6,
            ScaleLevel::Organism => 1.0e-3,
            ScaleLevel::Planetary => 1.0e6,
            ScaleLevel::Stellar => 1.0e9,
            ScaleLevel::Galactic => 1.0e21,
            ScaleLevel::Universal => 1.0e26,
        }
    }

    /// Get the scale level from a size in meters
    pub fn from_meters(size: f64) -> ScaleLevel {
        const QUANTUM_THRESHOLD: f64 = 1.0e-15;
        const ATOMIC_THRESHOLD: f64 = 1.0e-8;
        const MOLECULAR_THRESHOLD: f64 = 1.0e-6;
        const CELLULAR_THRESHOLD: f64 = 1.0e-4;
        const ORGANISM_THRESHOLD: f64 = 1.0e2;
        const PLANETARY_THRESHOLD: f64 = 1.0e8;
        const STELLAR_THRESHOLD: f64 = 1.0e15;
        const GALACTIC_THRESHOLD: f64 = 1.0e23;

        if size < QUANTUM_THRESHOLD {
            ScaleLevel::Quantum
        } else if size < ATOMIC_THRESHOLD {
            ScaleLevel::Atomic
        } else if size < MOLECULAR_THRESHOLD {
            ScaleLevel::Molecular
        } else if size < CELLULAR_THRESHOLD {
            ScaleLevel::Cellular
        } else if size < ORGANISM_THRESHOLD {
            ScaleLevel::Organism
        } else if size < PLANETARY_THRESHOLD {
            ScaleLevel::Planetary
        } else if size < STELLAR_THRESHOLD {
            ScaleLevel::Stellar
        } else if size < GALACTIC_THRESHOLD {
            ScaleLevel::Galactic
        } else {
            ScaleLevel::Universal
        }
    }

    /// Get the name of this scale level
    pub fn name(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum",
            ScaleLevel::Atomic => "Atomic",
            ScaleLevel::Molecular => "Molecular",
            ScaleLevel::Cellular => "Cellular",
            ScaleLevel::Organism => "Organism",
            ScaleLevel::Planetary => "Planetary",
            ScaleLevel::Stellar => "Stellar",
            ScaleLevel::Galactic => "Galactic",
            ScaleLevel::Universal => "Universal",
        }
    }

    /// Get the description of this scale level
    pub fn description(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Planck scale - quantum fluctuations and spacetime foam",
            ScaleLevel::Atomic => "Subatomic particles - quarks, electrons, photons",
            ScaleLevel::Molecular => "Atoms and molecules - chemical bonding",
            ScaleLevel::Cellular => "Biological cells - prokaryotes and eukaryotes",
            ScaleLevel::Organism => "Life forms - plants, animals, humans",
            ScaleLevel::Planetary => "Planets and moons - geological systems",
            ScaleLevel::Stellar => "Stars and solar systems - nuclear fusion",
            ScaleLevel::Galactic => "Galaxies - stellar clusters and dark matter",
            ScaleLevel::Universal => "Observable universe - cosmic web structure",
        }
    }

    /// Get the next higher scale level
    pub fn next(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => Some(ScaleLevel::Atomic),
            ScaleLevel::Atomic => Some(ScaleLevel::Molecular),
            ScaleLevel::Molecular => Some(ScaleLevel::Cellular),
            ScaleLevel::Cellular => Some(ScaleLevel::Organism),
            ScaleLevel::Organism => Some(ScaleLevel::Planetary),
            ScaleLevel::Planetary => Some(ScaleLevel::Stellar),
            ScaleLevel::Stellar => Some(ScaleLevel::Galactic),
            ScaleLevel::Galactic => Some(ScaleLevel::Universal),
            ScaleLevel::Universal => None,
        }
    }

    /// Get the next lower scale level
    pub fn previous(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => None,
            ScaleLevel::Atomic => Some(ScaleLevel::Quantum),
            ScaleLevel::Molecular => Some(ScaleLevel::Atomic),
            ScaleLevel::Cellular => Some(ScaleLevel::Molecular),
            ScaleLevel::Organism => Some(ScaleLevel::Cellular),
            ScaleLevel::Planetary => Some(ScaleLevel::Organism),
            ScaleLevel::Stellar => Some(ScaleLevel::Planetary),
            ScaleLevel::Galactic => Some(ScaleLevel::Stellar),
            ScaleLevel::Universal => Some(ScaleLevel::Galactic),
        }
    }
}

/// 3D coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate3D {
    /// Create a new 3D coordinate
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Coordinate3D { x, y, z }
    }

    /// Get the origin (0, 0, 0)
    pub fn origin() -> Self {
        Coordinate3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculate distance to another coordinate
    pub fn distance_to(&self, other: &Coordinate3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Normalize this coordinate to unit length
    pub fn normalize(&self) -> Coordinate3D {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length > 0.0 {
            Coordinate3D {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        } else {
            *self
        }
    }

    /// Scale this coordinate by a factor
    pub fn scale(&self, factor: f64) -> Coordinate3D {
        Coordinate3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    /// Add another coordinate to this one
    pub fn add(&self, other: &Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtract another coordinate from this one
    pub fn subtract(&self, other: &Coordinate3D) -> Coordinate3D {
        Coordinate3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Default for Coordinate3D {
    fn default() -> Self {
        Self::origin()
    }
}

/// Screen position (2D)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    /// Create a new screen position
    pub fn new(x: f32, y: f32) -> Self {
        ScreenPosition { x, y }
    }

    /// Get the center of the screen
    pub fn center(width: u32, height: u32) -> Self {
        ScreenPosition {
            x: width as f32 / 2.0,
            y: height as f32 / 2.0,
        }
    }
}

impl Default for ScreenPosition {
    fn default() -> Self {
        ScreenPosition { x: 0.0, y: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_config_default() {
        let config = GuiConfig::default();
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert_eq!(config.initial_time_rate, 1.0);
        assert_eq!(config.min_time_rate, 0.1);
        assert_eq!(config.max_time_rate, 1000.0);
    }

    #[test]
    fn test_gui_config_builder() {
        let config = GuiConfig::new()
            .with_window_size(1280, 720)
            .with_time_rate_range(0.5, 500.0)
            .with_zoom_range(1.0e-10, 1.0e20)
            .with_focus_dilation(false)
            .with_msaa(8)
            .with_vsync(false);

        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
        assert_eq!(config.min_time_rate, 0.5);
        assert_eq!(config.max_time_rate, 500.0);
        assert_eq!(config.min_zoom, 1.0e-10);
        assert_eq!(config.max_zoom, 1.0e20);
        assert_eq!(config.enable_focus_dilation, false);
        assert_eq!(config.msaa_samples, 8);
        assert_eq!(config.enable_vsync, false);
    }
}
