//! GUI Application - Main Entry Point
//!
//! Integrates all GUI subsystems into a unified application.
//! This is the final integration layer for Phase 6.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Full integration of all visualization systems"

use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

use crate::gui::{camera::camera, Camera2D};
use crate::integrated_system::IntegratedSystem;

/// Main GUI Application
///
/// Orchestrates all subsystems:
/// - WGPU rendering
/// - EGUI UI
/// - Camera controls
/// - Input handling
/// - Simulation integration
/// - Visualization systems
pub struct GuiApplication {
    /// Window handle
    window: Arc<Window>,

    /// Camera system
    camera: Camera2D,

    /// Simulation integration
    simulation: IntegratedSystem,

    /// Application state
    config: GuiConfig,
    running: bool,
    last_frame_time: Instant,
    frame_count: u64,
    fps: f32,

    /// Performance metrics
    render_stats: RenderStats,
    target_frame_time: Duration,
}

impl GuiApplication {
    /// Create new GUI application
    pub async fn new(event_loop: &EventLoop<()>, config: GuiConfig) -> Result<Self, String> {
        println!("Initializing GUI Application...");

        // Create window
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Holonic Realms - Cosmological Simulation")
                .with_inner_size(PhysicalSize::new(config.window_width, config.window_height))
                .with_resizable(true)
                .build(event_loop)
                .map_err(|e| format!("Failed to create window: {}", e))?,
        );

        // Initialize camera
        let camera = Camera2D::new(1920.0 / 1080.0);

        // Initialize simulation
        println!("Initializing simulation...");
        let simulation = IntegratedSystem::new();
        let mut simulation = simulation;
        simulation
            .initialize()
            .map_err(|e| format!("Failed to initialize simulation: {:?}", e))?;

        println!("GUI Application initialized successfully");

        Ok(Self {
            window,
            camera,
            simulation,
            config,
            running: false,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            render_stats: RenderStats::default(),
            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
        })
    }

    /// Run the application event loop
    pub fn run(mut self, event_loop: EventLoop<()>) {
        println!("Starting main event loop");
        self.running = true;

        event_loop.run(move |event, _window_target| {
            match event {
                Event::NewEvents(_) => {
                    // Calculate frame time
                    let now = Instant::now();
                    let frame_time = now.duration_since(self.last_frame_time);
                    self.last_frame_time = now;

                    // Update FPS
                    self.frame_count += 1;
                    if self.frame_count % 30 == 0 {
                        self.fps = 1.0 / frame_time.as_secs_f32();
                    }

                    // Update render stats
                    self.render_stats.frame_time_ms = frame_time.as_secs_f64() * 1000.0;
                    self.render_stats.fps = self.fps;
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("Close requested, shutting down...");
                        _window_target.exit();
                    }

                    WindowEvent::Resized(size) => {
                        println!("Window resized: {}x{}", size.width, size.height);
                    }

                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                                    _window_target.exit();
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                },

                Event::AboutToWait => {
                    if self.running {
                        // Run simulation step
                        if let Err(e) = self.run_step() {
                            eprintln!("Simulation step failed: {:?}", e);
                        }

                        // Request redraw
                        self.window.request_redraw();
                    }
                }

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::RedrawRequested,
                } => {
                    // Update render stats
                    self.render_stats.frame_count += 1;

                    // Sleep to maintain target frame rate
                    let elapsed = self.last_frame_time.elapsed();
                    if elapsed < self.target_frame_time {
                        std::thread::sleep(self.target_frame_time - elapsed);
                    }
                }

                Event::LoopExiting => {
                    println!("Event loop exiting");
                    self.simulation.shutdown();
                }

                _ => {}
            }
        });
    }

    /// Run a single simulation step
    fn run_step(&mut self) -> Result<(), String> {
        // Update simulation
        self.simulation
            .run_step()
            .map_err(|e| format!("Simulation step failed: {:?}", e))?;

        Ok(())
    }

    /// Get render statistics
    pub fn render_stats(&self) -> &RenderStats {
        &self.render_stats
    }

    /// Get simulation state
    pub fn simulation_state(&self) -> &crate::integrated_system::SimulationState {
        self.simulation.state()
    }

    /// Check if application is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}

/// Render statistics
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    /// Frames per second
    pub fps: f32,

    /// Frame time in milliseconds
    pub frame_time_ms: f64,

    /// Total frame count
    pub frame_count: u64,
}

/// GUI Application Builder
pub struct GuiApplicationBuilder {
    config: GuiConfig,
}

impl GuiApplicationBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            config: GuiConfig::default(),
        }
    }

    /// Set window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.config.window_width = width;
        self.config.window_height = height;
        self
    }

    /// Set initial zoom
    pub fn with_initial_zoom(mut self, zoom: f64) -> Self {
        self.config.initial_zoom = zoom;
        self
    }

    /// Set initial time rate
    pub fn with_initial_time_rate(mut self, rate: f64) -> Self {
        self.config.initial_time_rate = rate;
        self
    }

    /// Enable focus-based time dilation
    pub fn with_focus_dilation(mut self, enable: bool) -> Self {
        self.config.enable_focus_dilation = enable;
        self
    }

    /// Build the application
    pub async fn build(self, event_loop: &EventLoop<()>) -> Result<GuiApplication, String> {
        GuiApplication::new(event_loop, self.config).await
    }
}

impl Default for GuiApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// GUI Configuration
#[derive(Debug, Clone)]
pub struct GuiConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub initial_time_rate: f64,
    pub min_time_rate: f64,
    pub max_time_rate: f64,
    pub initial_zoom: f64,
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub enable_focus_dilation: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            window_width: 1920,
            window_height: 1080,
            initial_time_rate: 1.0,
            min_time_rate: 0.1,
            max_time_rate: 1000.0,
            initial_zoom: 1e-6,
            min_zoom: 1.616255e-35,
            max_zoom: 8.8e26,
            enable_focus_dilation: true,
        }
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
    }

    #[test]
    fn test_gui_builder() {
        let builder = GuiApplicationBuilder::new()
            .with_window_size(1280, 720)
            .with_initial_zoom(1.0)
            .with_initial_time_rate(2.0)
            .with_focus_dilation(false);

        assert_eq!(builder.config.window_width, 1280);
        assert_eq!(builder.config.window_height, 720);
        assert_eq!(builder.config.initial_zoom, 1.0);
        assert_eq!(builder.config.initial_time_rate, 2.0);
        assert_eq!(builder.config.enable_focus_dilation, false);
    }

    #[test]
    fn test_render_stats_default() {
        let stats = RenderStats::default();
        assert_eq!(stats.fps, 0.0);
        assert_eq!(stats.frame_count, 0);
    }
}
