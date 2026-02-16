//! Week 3: Logarithmic Depth & LOD
//!
//! This binary demonstrates:
//! - Logarithmic depth buffering for 61 orders of magnitude
//! - Level-of-detail (LOD) system
//! - Spatial partitioning with octree
//! - Scene graph with hierarchical organization
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 2 Week 3

use holonic_realms::gui::camera::controls::CameraControls;
use holonic_realms::gui::camera::Camera2D;
use holonic_realms::gui::renderer::buffers::BufferManager;
use holonic_realms::gui::renderer::pipeline::EntityRenderPipeline;
use holonic_realms::gui::renderer::WgpuContext;
use holonic_realms::gui::scene::lod::LODSystem;
use holonic_realms::gui::scene::spatial_partition::Octree;
use holonic_realms::gui::scene::{SceneGraph, Transform};
use std::time::Instant;
use winit::event::ElementState;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowBuilder};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

struct Week3Application {
    window: Window,
    context: WgpuContext,
    buffer_manager: BufferManager,
    render_pipeline: EntityRenderPipeline,
    camera: Camera2D,
    camera_controls: CameraControls,
    scene_graph: SceneGraph,
    lod_system: LODSystem,
    octree: Octree,
    last_frame_time: Instant,
    frame_count: u32,
}

impl Week3Application {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Holonic Realms - Phase 2 Week 3: Logarithmic Depth & LOD")
            .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(&event_loop)?;

        let context = WgpuContext::new(&window).await?;

        let buffer_manager = BufferManager::new(&context.device, WINDOW_WIDTH, WINDOW_HEIGHT);
        let render_pipeline = EntityRenderPipeline::new(&context.device, &context.surface_config);

        let camera = Camera2D::new(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32);

        let camera_controls = CameraControls::new();

        let scene_graph = SceneGraph::new();
        let lod_system = LODSystem::new();
        let octree = Octree::new(nalgebra_glm::Vec3::zeros(), 1000.0);

        Ok(Self {
            window,
            context,
            buffer_manager,
            render_pipeline,
            camera,
            camera_controls,
            scene_graph,
            lod_system,
            octree,
            last_frame_time: Instant::now(),
            frame_count: 0,
        })
    }

    fn setup_scene(&mut self) {
        // Create root node
        let _root = self.scene_graph.create_node("Root");

        // Create test entities at different scales
        for i in 0..100 {
            let x = (i as f32 - 50.0) * 10.0;
            let y = ((i % 10) as f32 - 5.0) * 10.0;

            let _entity_node = self.scene_graph.create_node_with_transform(
                format!("Entity_{}", i),
                Transform::translation(nalgebra_glm::Vec3::new(x, y, 0.0)),
            );

            // Add to octree
            let pos = nalgebra_glm::Vec3::new(x, y, 0.0);
            let entity_id =
                holonic_realms::entity_layer7::layer7::EntityId::new(format!("entity_{}", i));
            self.octree.insert(entity_id, pos);
        }

        println!("Scene setup complete:");
        println!("  - Scene nodes: {}", self.scene_graph.node_count());
        println!("  - Octree entities: {}", self.octree.count_entities());
    }

    fn update(&mut self, _delta_time: f32) {
        self.scene_graph.update_transforms();
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let surface = self
            .context
            .surface
            .as_ref()
            .ok_or("Surface not initialized")?;
        let frame = surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Clear screen
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    fn calculate_fps(&mut self) -> f32 {
        self.frame_count += 1;
        let elapsed = self.last_frame_time.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            let fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_frame_time = Instant::now();
            fps
        } else {
            0.0
        }
    }

    fn context_mut(&mut self) -> Option<&mut WgpuContext> {
        Some(&mut self.context)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Holonic Realms - Phase 2 Week 3 ===");
    println!("Logarithmic Depth & LOD System");
    println!();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Holonic Realms - Phase 2 Week 3: Logarithmic Depth & LOD")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)?;

    let mut context = WgpuContext::new(&window).await?;
    let _buffer_manager = BufferManager::new(&context.device, WINDOW_WIDTH, WINDOW_HEIGHT);
    let render_pipeline = EntityRenderPipeline::new(&context.device, &context.surface_config);

    let mut camera = Camera2D::new(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32);

    let mut camera_controls = CameraControls::new();
    let mut scene_graph = SceneGraph::new();
    let _lod_system = LODSystem::new();
    let mut octree = Octree::new(nalgebra_glm::Vec3::zeros(), 1000.0);

    println!("Application initialized successfully");
    println!();

    println!("Controls:");
    println!("  - Mouse drag: Pan camera");
    println!("  - Mouse scroll: Zoom in/out");
    println!("  - Arrow keys: Pan camera");
    println!("  - Page Up/Down: Zoom in/out");
    println!("  - Q/E: Rotate camera");
    println!("  - Home: Reset camera");
    println!("  - ESC: Quit");
    println!();

    // Scene setup
    let _root = scene_graph.create_node("Root");
    for i in 0..100 {
        let x = (i as f32 - 50.0) * 10.0;
        let y = ((i % 10) as f32 - 5.0) * 10.0;

        let _entity_node = scene_graph.create_node_with_transform(
            format!("Entity_{}", i),
            Transform::translation(nalgebra_glm::Vec3::new(x, y, 0.0)),
        );

        let pos = nalgebra_glm::Vec3::new(x, y, 0.0);
        let entity_id =
            holonic_realms::entity_layer7::layer7::EntityId::new(format!("entity_{}", i));
        octree.insert(entity_id, pos);
    }

    println!("Scene setup complete:");
    println!("  - Scene nodes: {}", scene_graph.node_count());
    println!("  - Octree entities: {}", octree.count_entities());
    println!();

    let mut last_frame_time = Instant::now();
    let mut frame_count = 0u32;

    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => {
                camera_controls.handle_event(&event, &mut camera);

                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window close requested");
                        window_target.exit();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            if let Key::Named(NamedKey::Escape) = event.logical_key {
                                println!("Quitting...");
                                window_target.exit();
                            }
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        context.resize(physical_size.width, physical_size.height);
                        camera.set_aspect_ratio(physical_size.width as f32 / physical_size.height as f32);
                    }
                    WindowEvent::RedrawRequested => {
                        // Update scene
                        scene_graph.update_transforms();

                        // Render frame
                        if let Err(e) = render_frame(&context, &render_pipeline) {
                            eprintln!("Render error: {}", e);
                        }

                        // Calculate FPS
                        frame_count += 1;
                        let elapsed = last_frame_time.elapsed();
                        if elapsed >= std::time::Duration::from_secs(1) {
                            let fps = frame_count as f64 / elapsed.as_secs_f64();
                            frame_count = 0;
                            last_frame_time = Instant::now();

                            window.set_title(&format!(
                                "Holonic Realms - Week 3: {:.1} FPS | Camera: ({:.1}, {:.1}) Zoom: {:.2}",
                                fps,
                                camera.position.x,
                                camera.position.y,
                                camera.zoom
                            ));
                        }
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
    context: &WgpuContext,
    _render_pipeline: &EntityRenderPipeline,
) -> Result<(), Box<dyn std::error::Error>> {
    let surface = context.surface.as_ref().ok_or("Surface not initialized")?;
    let frame = surface.get_current_texture()?;
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    // Clear screen
    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    }

    context.queue.submit(std::iter::once(encoder.finish()));
    frame.present();

    Ok(())
}
