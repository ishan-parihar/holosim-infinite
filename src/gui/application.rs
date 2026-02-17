//! GUI Application - Main Entry Point
//!
//! Integrates all GUI subsystems into a unified application.
//! This is the final integration layer for Phase 6.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Full integration of all visualization systems"
//!
//! PHASE 1 UPDATE: Added WGPU initialization and entity rendering

use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

// Phase 1: Import WGPU context and entity renderer
use crate::gui::renderer::wgpu_context::WgpuContext;
use crate::gui::renderer::entity_renderer::EntityRenderer;
use crate::gui::renderer::connection_renderer::ConnectionRenderer;
use crate::gui::view_system::{ViewSystem, ViewType};
use crate::gui::{camera::camera, camera::camera_extended, Camera2D, CameraControls};
use crate::integrated_system::IntegratedSystem;

/// Main GUI Application
///
/// Orchestrates all subsystems:
/// - WGPU rendering (Phase 1: Now initialized!)
/// - EGUI UI
/// - Camera controls
/// - Input handling
/// - Simulation integration
/// - Visualization systems
pub struct GuiApplication {
    /// Window handle
    window: Arc<Window>,

    /// WGPU Context (Phase 1: Added)
    wgpu_context: Option<WgpuContext>,
    
    /// Entity Renderer (Phase 1: Added)
    entity_renderer: Option<EntityRenderer>,

    /// Connection Renderer (Phase 4: Hierarchy visualization)
    connection_renderer: Option<ConnectionRenderer>,
    /// View system for multi-level visualization
    view_system: ViewSystem,

    /// Camera system
    camera: Camera2D,

    /// Camera controls for user input
    camera_controls: CameraControls,

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

        println!("Window created successfully");

        // Phase 1: Initialize WGPU context
        println!("Initializing WGPU context...");
        let (wgpu_context, entity_renderer, connection_renderer, view_system) = match WgpuContext::new(&window).await {
            Ok(ctx) => {
                println!("✓ WGPU context initialized successfully");

                // Phase 1: Create entity renderer
                println!("Creating entity renderer...");
                let renderer = EntityRenderer::new(&ctx.device, &ctx.surface_config);
                println!("✓ Entity renderer created");

                // Phase 4: Create connection renderer
                println!("Creating connection renderer...");
                let conn_renderer = ConnectionRenderer::new(&ctx.device, &ctx.surface_config);
                println!("✓ Connection renderer created");

                
                println!("Creating view system...");
                let view_system = ViewSystem::new();
                println!("✓ View system created");
                (Some(ctx), Some(renderer), Some(conn_renderer), Some(view_system))
            }
            Err(e) => {
                eprintln!("⚠ Failed to initialize WGPU: {}", e);
                eprintln!("Continuing without GPU rendering...");
                (None, None, None, None)
            }
        };

        // Initialize camera
        let camera = Camera2D::new(1920.0 / 1080.0);

        // Initialize camera controls
        let camera_controls = CameraControls::new();

        // Initialize simulation
        println!("Initializing simulation...");
        let simulation = IntegratedSystem::new();
        let mut simulation = simulation;
        simulation
            .initialize()
            .map_err(|e| format!("Failed to initialize simulation: {:?}", e))?;

        println!("✓ GUI Application initialized successfully");

        Ok(Self {
            window,
            wgpu_context,
            entity_renderer,
            connection_renderer, // Phase 4: Add connection renderer
            camera,
            camera_controls,
            simulation,
            config,
            running: false,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            render_stats: RenderStats::default(),
            target_frame_time: Duration::from_secs_f64(1.0 / 60.0),
            view_system: view_system.unwrap_or_default(),
        })
    }

    /// Run the application event loop
    pub fn run(mut self, event_loop: EventLoop<()>) {
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
                        // Phase 1: Resize WGPU surface if available
                        if let Some(ref mut ctx) = self.wgpu_context {
                            ctx.resize(size.width, size.height);
                        }
                        // Update camera aspect ratio
                        let aspect_ratio = size.width as f32 / size.height as f32;
                        self.camera.set_aspect_ratio(aspect_ratio);
                    }

                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                                    _window_target.exit();
                                }
                                // Phase 5: View switching with keys 1-6
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit1) => {
                                    self.switch_view(ViewType::Overview);
                                }
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit2) => {
                                    self.switch_view(ViewType::Hierarchy);
                                }
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit3) => {
                                    self.switch_view(ViewType::Realm);
                                }
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit4) => {
                                    self.switch_view(ViewType::Archetype);
                                }
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit5) => {
                                    self.switch_view(ViewType::Spectrum);
                                }
                                winit::keyboard::PhysicalKey::Code(KeyCode::Digit6) => {
                                    self.switch_view(ViewType::Evolution);
                                }
                                _ => {}
                            }
                        }
                    }


                    WindowEvent::RedrawRequested => {
                        // Phase 1: Render frame using WGPU
                        if let Err(e) = self.render_frame() {
                            eprintln!("Render error: {:?}", e);
                        }

                        // Update render stats
                        self.render_stats.frame_count += 1;

                        // Sleep to maintain target frame rate
                        let elapsed = self.last_frame_time.elapsed();
                        if elapsed < self.target_frame_time {
                            std::thread::sleep(self.target_frame_time - elapsed);
                        }
                    }

                    _ => {
                        // Camera controls
                        self.camera_controls.handle_event(&event, &mut self.camera);
                    }
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

                Event::LoopExiting => {
                    println!("Event loop exiting");
                    self.simulation.shutdown();
                }

                _ => {}
            }
        });
    }

    /// Phase 1: Render a frame using WGPU
    fn render_frame(&mut self) -> Result<(), String> {
        // Get WGPU context
        let ctx = match &self.wgpu_context {
            Some(ctx) => ctx,
            None => return Ok(()), // No WGPU, skip rendering
        };

        // Get entity renderer
        let renderer = match &mut self.entity_renderer {
            Some(r) => r,
            None => return Ok(()), // No renderer, skip rendering
        };

        // Update camera uniforms
        let view_projection = self.camera.view_projection_matrix();
        let view_projection_array: [[f32; 4]; 4] = view_projection.into();
        // Phase 2: Pass time for realm ring animations
        // Phase 5: Update view transitions
        let current_time = self.last_frame_time.elapsed();
        if let Some((target_pos, target_zoom, target_rotation)) = 
            self.view_system.update_transition(Duration::from_secs_f32(0.016), current_time) {
            
            let start_pos = self.view_system.transition_start_position();
            let start_zoom = self.view_system.transition_start_zoom();
            let start_rotation = self.view_system.transition_start_rotation();
            
            self.camera.smooth_transition_to(
                target_pos,
                target_zoom,
                target_rotation,
                self.view_system.transition_progress(),
                start_pos,
                start_zoom,
                start_rotation,
            );
        }

        let time = self.last_frame_time.elapsed().as_secs_f32();
        renderer.update_camera(&ctx.queue, view_projection_array, time);

        // Phase 2 Week 3: Update entity renderer with REAL simulation entities
        let entities = self.simulation.entities();

        if !entities.is_empty() {
            renderer.update_entities(&ctx.queue, &entities);
            // Update render stats with entity count
            self.render_stats.entity_count = entities.len() as u64;
        } else {
            // Fallback to test instances if no real entities yet
            renderer.update_test_instances(&ctx.queue, 50);
        }

        // Phase 4: Update connection renderer with entities and their positions
        if !entities.is_empty() {
            if let Some(conn_renderer) = &mut self.connection_renderer {
                // Generate entity instances for position lookup
                let entity_instances: Vec<crate::gui::renderer::EntityInstance> = entities
                    .iter()
                    .enumerate()
                    .map(|(i, e)| crate::gui::renderer::EntityInstance::from_entity(e, i))
                    .collect();

                conn_renderer.update_camera(&ctx.queue, view_projection_array, time);
                conn_renderer.update_connections(&ctx.queue, &entities, &entity_instances);
            }
        }

        // Get surface texture
        let surface = ctx.surface.as_ref()
            .ok_or("No surface available")?;
        
        let output = surface.get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {}", e))?;
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Begin render pass with clear color
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,  // Dark blue background
                            g: 0.05,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Phase 4: Render connection lines (behind entities)
            if let Some(conn_renderer) = &self.connection_renderer {
                conn_renderer.render(&mut render_pass);
            }

            // Phase 1: Render entities
            renderer.render(&mut render_pass);
        }

        // Debug output every 60 frames
        if self.frame_count % 60 == 0 && self.render_stats.entity_count > 0 {
            println!("Rendering {} entities at {:.1} FPS",
                self.render_stats.entity_count, self.fps);
            
            // Show holographic simulation stats
            if let Some(stats) = self.get_holo_statistics() {
                println!("  [Holographic] Coherence: {:.3} | Veil: {:.3} | Entities: {} | Collectives: {}",
                    stats.average_coherence, stats.veil_transparency, stats.entity_count, stats.collective_count);
                // Phase B: Show cosmic sequence info
                let layer_names = ["Violet", "Indigo", "Blue", "Green", "Yellow", "Orange", "Red", "L7"];
                let layer_name = layer_names.get(stats.current_layer).unwrap_or(&"?");
                println!("  [Cosmic] Layer: {} | Attractor: {:.3}", layer_name, stats.attractor_strength);
                // Phase C: Show Larson framework info
                let st = if stats.veil_transparency < 0.5 { "ST" } else if stats.veil_transparency > 0.7 { "TS" } else { "~V~" };
                println!("  [Larson] v=s/t↔v=t/s | Veil: {:.1}% | Mode: {}", 
                    stats.veil_transparency * 100.0, st);
            }
        }

        // Submit and present
        ctx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Run a single simulation step
    /// Phase 5: Switch to a specific view
    fn switch_view(&mut self, view_type: ViewType) {
        println!("Switching to view: {}", view_type.name());
        
        let (pos, zoom, rotation) = self.camera.get_state();
        self.view_system.switch_view(view_type, pos, zoom, rotation);
        
        // Update window title to show current view
        self.window.set_title(&format!(
            "Holonic Realms - {} View - FPS: {:.1}",
            view_type.name(),
            self.fps
        ));
    }

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

    /// Check if holographic mode is enabled
    pub fn is_holographic_mode(&self) -> bool {
        self.simulation.is_holographic_mode()
    }

    /// Get holographic simulation statistics
    pub fn get_holo_statistics(&self) -> Option<crate::hpo::SimulationStatistics> {
        self.simulation.get_holo_statistics()
    }

    /// Get field visualization data
    pub fn get_field_visualization(&self) -> Option<crate::hpo::FieldVisualizationData> {
        self.simulation.get_field_visualization()
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

    /// Number of entities being rendered
    pub entity_count: u64,
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
