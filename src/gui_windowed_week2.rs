// Holonic Realms - Windowed GUI with Camera System & EGUI Integration
//
// Phase 1 Week 2 Implementation:
// - Multi-scale camera with pan/zoom controls
// - EGUI integration for UI panels
// - Entity inspector panel
// - Interactive entity selection

use holonic_realms::entity_layer7::layer7::EntityId;
use holonic_realms::gui::{
    camera::{Camera2D as Camera, CameraControls},
    ui::{EguiIntegration, EntityInspector},
    visualization_engine::WgpuRenderer,
};
use holonic_realms::integrated_system::IntegratedSystem;
use std::hash::Hasher;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Holonic Realms GUI (Phase 1 Week 2)...");
    println!("Features:");
    println!("  - Multi-scale camera with pan/zoom controls");
    println!("  - EGUI integration for UI panels");
    println!("  - Entity inspector panel");
    println!("  - Interactive entity selection");

    // Initialize simulation
    let mut simulation = IntegratedSystem::new();
    if let Err(e) = simulation.initialize() {
        eprintln!("Failed to initialize simulation: {:?}", e);
        return Err(format!("Simulation initialization failed: {:?}", e).into());
    }
    println!(
        "Simulation initialized with {} entities",
        simulation.state().entity_count
    );

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create window
    let window = WindowBuilder::new()
        .with_title("Holonic Realms - Cosmological Simulation (Phase 1 Week 2)")
        .with_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
        .build(&event_loop)?;
    println!("Window created successfully!");

    // Initialize WGPU renderer
    let mut renderer = WgpuRenderer::new();
    pollster::block_on(renderer.initialize(&window))?;
    println!("WGPU renderer initialized");

    // Initialize camera
    let mut camera =
        Camera::new(window.inner_size().width as f32 / window.inner_size().height as f32);
    println!("Camera initialized");

    // Initialize camera controls
    let mut camera_controls = CameraControls::new();
    println!("Camera controls initialized");

    // Initialize EGUI integration
    let context_ref = renderer
        .context
        .as_ref()
        .ok_or_else(|| "Context not initialized".to_string())?;
    let mut egui_integration = EguiIntegration::new(context_ref);
    println!("EGUI integration initialized");

    // Initialize entity inspector
    let mut entity_inspector = EntityInspector::new();
    println!("Entity inspector initialized");

    println!("\nControls:");
    println!("  Mouse:");
    println!("    - Left drag: Pan camera");
    println!("    - Scroll: Zoom in/out");
    println!("  Keyboard:");
    println!("    - Arrow keys: Pan camera");
    println!("    - Page Up/Down: Zoom in/out");
    println!("    - Home: Reset camera");
    println!("    - Q/E: Rotate camera");
    println!("    - +/-: Zoom in/out");
    println!("    - ESC: Quit");
    println!("\nStarting main loop...");

    // Simulation state
    let mut paused = false;
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut fps = 0.0;
    let mut sim_step = 0;

    // Main event loop
    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => {
                // Handle camera controls
                camera_controls.handle_event(&event, &mut camera);

                // Handle EGUI events
                egui_integration.handle_event(&window, &event);

                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window close requested");
                        window_target.exit();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            match event.logical_key {
                                Key::Named(NamedKey::Escape) => {
                                    println!("Quitting...");
                                    window_target.exit();
                                }
                                Key::Character(ref c) if c == "p" || c == "P" => {
                                    paused = !paused;
                                    println!("Simulation {}", if paused { "paused" } else { "resumed" });
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        println!("Window resized to: {}x{}", physical_size.width, physical_size.height);
                        renderer.resize(physical_size.width, physical_size.height);
                        camera.set_aspect_ratio(physical_size.width as f32 / physical_size.height as f32);
                        if let Some(ctx) = renderer.context.as_ref() {
                            egui_integration.resize(ctx, physical_size.width, physical_size.height);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        // Update simulation
                        if !paused
                            && simulation.run(1).is_ok() {
                                sim_step += 1;
                            }

                        // Calculate FPS
                        frame_count += 1;
                        let elapsed = last_fps_update.elapsed();
                        if elapsed >= std::time::Duration::from_secs(1) {
                            fps = frame_count as f64 / elapsed.as_secs_f64();
                            frame_count = 0;
                            last_fps_update = Instant::now();

                            // Update window title with status
                            let state = simulation.state();
                            let health = simulation.health_metrics();
                            let title = format!(
                                "Holonic Realms | FPS: {:.1} | Step: {} | Coherence: {:.3} | Health: {:.0}% | Entities: {} | Zoom: {:.2}x",
                                fps,
                                sim_step,
                                state.coherence,
                                health.overall_health * 100.0,
                                state.entity_count,
                                camera.zoom
                            );
                            window.set_title(&title);
                        }

                        // Render frame
                        if let Err(e) = render_frame(
                            &mut renderer,
                            &mut egui_integration,
                            &mut entity_inspector,
                            &camera,
                            &simulation,
                        ) {
                            eprintln!("Render error: {}", e);
                        }

                        // Request next frame
                        window.request_redraw();
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

fn render_frame(
    renderer: &mut WgpuRenderer,
    egui_integration: &mut EguiIntegration,
    entity_inspector: &mut EntityInspector,
    camera: &Camera,
    simulation: &IntegratedSystem,
) -> Result<(), String> {
    // Get surface
    let surface = renderer
        .context
        .as_ref()
        .ok_or_else(|| "Context not initialized".to_string())?
        .surface
        .as_ref()
        .ok_or_else(|| "Surface not initialized".to_string())?;

    // Get current texture
    let output = surface
        .get_current_texture()
        .map_err(|e| format!("Failed to get current texture: {}", e))?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    // Create command encoder
    let mut encoder = renderer
        .context
        .as_ref()
        .unwrap()
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    // Render entities (background)
    render_entities(renderer, &mut encoder, &view, camera, simulation)?;

    // Render EGUI UI (foreground)
    egui_integration.begin_frame();
    show_ui(
        egui_integration.context_mut(),
        entity_inspector,
        camera,
        simulation,
    );
    if let Some(ctx) = renderer.context.as_ref() {
        egui_integration.end_frame(ctx, &mut encoder, &view);
    }

    // Submit commands
    renderer
        .context
        .as_ref()
        .unwrap()
        .queue
        .submit(std::iter::once(encoder.finish()));

    // Present
    output.present();

    Ok(())
}

fn render_entities(
    renderer: &mut WgpuRenderer,
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    camera: &Camera,
    simulation: &IntegratedSystem,
) -> Result<(), String> {
    let context = renderer
        .context
        .as_ref()
        .ok_or_else(|| "Context not initialized".to_string())?;
    let buffer_manager = renderer
        .buffer_manager
        .as_ref()
        .ok_or_else(|| "Buffer manager not initialized".to_string())?;
    let render_pipeline = renderer
        .render_pipeline
        .as_ref()
        .ok_or_else(|| "Render pipeline not initialized".to_string())?;

    // Update uniforms with camera transform
    let width = context.surface_config.width;
    let height = context.surface_config.height;
    buffer_manager.update_uniforms(
        &context.queue,
        width,
        height,
        camera.zoom,
        camera.position.x,
        camera.position.y,
    );

    // Create bind group
    let bind_group = context
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &render_pipeline.pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer_manager.uniform_buffer().as_entire_binding(),
            }],
        });

    // Begin render pass
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&render_pipeline.pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, buffer_manager.vertex_buffer().slice(..));
        render_pass.set_index_buffer(
            buffer_manager.index_buffer().slice(..),
            wgpu::IndexFormat::Uint16,
        );

        // Draw entities
        let state = simulation.state();
        let entity_count = state.entity_count.min(100);
        for i in 0..entity_count {
            let entity_id =
                holonic_realms::entity_layer7::layer7::EntityId::new(format!("entity-{}", i));
            let position = calculate_entity_position(&entity_id);
            let density = get_entity_density(&entity_id, state);
            let color = get_density_color(density);

            buffer_manager.update_instance_data(
                &context.queue,
                0,
                position[0],
                position[1],
                20.0 / camera.zoom,
                color,
            );

            render_pass.draw_indexed(0..6, 0, 0..1);
        }
    }

    Ok(())
}

fn show_ui(
    ctx: &egui::Context,
    entity_inspector: &mut EntityInspector,
    camera: &Camera,
    simulation: &IntegratedSystem,
) {
    // Show entity inspector
    entity_inspector.show(ctx, None);

    // Show camera controls info
    egui::Window::new("Camera Controls")
        .collapsible(true)
        .resizable(true)
        .default_pos([10.0, 300.0])
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Camera Position");
            ui.label(format!("X: {:.2}", camera.position.x));
            ui.label(format!("Y: {:.2}", camera.position.y));
            ui.separator();
            ui.heading("Camera Zoom");
            ui.label(format!("Zoom: {:.2}x", camera.zoom));
            ui.label(format!("Rotation: {:.2}°", camera.rotation.to_degrees()));
            ui.separator();
            ui.heading("Controls");
            ui.label("Mouse drag: Pan");
            ui.label("Mouse scroll: Zoom");
            ui.label("Arrow keys: Pan");
            ui.label("Page Up/Down: Zoom");
            ui.label("Home: Reset");
            ui.label("Q/E: Rotate");
        });

    // Show simulation info
    egui::Window::new("Simulation Info")
        .collapsible(true)
        .resizable(true)
        .default_pos([10.0, 500.0])
        .default_width(250.0)
        .show(ctx, |ui| {
            let state = simulation.state();
            let health = simulation.health_metrics();

            ui.heading("Simulation");
            ui.label(format!("Step: {}", state.step));
            ui.label(format!("Entities: {}", state.entity_count));
            ui.separator();
            ui.heading("Health");
            ui.label(format!("Overall: {:.0}%", health.overall_health * 100.0));
            ui.label(format!("Coherence: {:.3}", state.coherence));
            ui.label(format!("Energy Balance: {:.3}", state.energy_balance));
            ui.separator();
            ui.heading("Emergence");
            ui.label(format!(
                "Biological: {} cells",
                state.emergence.biological.cell_count
            ));
            ui.label(format!(
                "Noospheric: {} complexes",
                state.emergence.noospheric.social_complexes_count
            ));
            ui.label(format!(
                "Gaia: {:.3}",
                state.emergence.gaia.consciousness_score
            ));
        });
}

fn calculate_entity_position(entity_id: &EntityId) -> [f32; 2] {
    let hash = {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&entity_id.uuid, &mut hasher);
        std::hash::Hash::hash(&entity_id.incarnation_number, &mut hasher);
        hasher.finish()
    };
    let x = ((hash >> 32) & 0xFFFFFFFFu64) as f32 / 0xFFFFFFFFu64 as f32 * 2000.0 - 1000.0;
    let y = (hash & 0xFFFFFFFFu64) as f32 / 0xFFFFFFFFu64 as f32 * 2000.0 - 1000.0;
    [x, y]
}

fn get_entity_density(
    entity_id: &EntityId,
    _state: &holonic_realms::integrated_system::SimulationState,
) -> u8 {
    let hash = {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&entity_id.uuid, &mut hasher);
        std::hash::Hash::hash(&entity_id.incarnation_number, &mut hasher);
        hasher.finish()
    };
    ((hash % 8) + 1) as u8
}

fn get_density_color(density: u8) -> [f32; 4] {
    match density {
        1 => [1.0, 0.27, 0.27, 1.0], // Red
        2 => [1.0, 0.53, 0.27, 1.0], // Orange
        3 => [1.0, 0.8, 0.27, 1.0],  // Yellow
        4 => [0.27, 1.0, 0.27, 1.0], // Green
        5 => [0.27, 1.0, 1.0, 1.0],  // Cyan
        6 => [0.27, 0.27, 1.0, 1.0], // Blue
        7 => [0.53, 0.27, 1.0, 1.0], // Violet
        8 => [1.0, 1.0, 1.0, 1.0],   // White
        _ => [0.5, 0.5, 0.5, 1.0],   // Gray
    }
}
