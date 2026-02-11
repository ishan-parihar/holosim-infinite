//! Wayland-Fixed GUI - Proper rendering for Wayland visibility
//!
//! Key fix: On Wayland, windows don't appear until you render/present to them

use std::sync::Arc;
use std::time::Instant;
use wgpu::{
    Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration, SurfaceError,
    TextureViewDescriptor,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

struct WgpuState {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   Holonic Realms - Wayland-Working GUI                      ║");
    println!("║   Key: Window appears AFTER first render/present             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create window
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Holonic Realms - Wayland GUI")
            .with_inner_size(PhysicalSize::new(1920, 1080))
            .with_resizable(true)
            .with_visible(true)
            .build(&event_loop)?,
    );

    println!("✓ Window created");
    println!(
        "  Size: {}x{}",
        window.inner_size().width,
        window.inner_size().height
    );
    println!();

    // Initialize WGPU
    let wgpu_state = pollster::block_on(initialize_wgpu(&window))?;
    println!("✓ WGPU initialized");
    println!();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    Controls                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("  ESC        - Exit application");
    println!();

    println!("Starting simulation window...");
    println!("The window will appear after the first render frame!");
    println!();

    // Run event loop with rendering
    run_event_loop(window, wgpu_state, event_loop)?;

    Ok(())
}

async fn initialize_wgpu(window: &Window) -> Result<WgpuState, String> {
    println!("Initializing WGPU...");

    // Use all backends (let WGPU choose)
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = unsafe { instance.create_surface(window) }
        .map_err(|e| format!("Failed to create surface: {}", e))?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or("Failed to find GPU adapter")?;

    println!("  GPU: {}", adapter.get_info().name);
    println!("  Backend: {:?}", adapter.get_info().backend);

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("WGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .map_err(|e| format!("Failed to create device: {}", e))?;

    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities
        .formats
        .first()
        .ok_or("No surface format available")?;

    let surface_config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![*format],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    // Create a simple render pipeline
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Simple Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("gui/renderer/shader.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    println!("  Surface Format: {:?}", format);
    println!("  Pipeline created");

    Ok(WgpuState {
        device,
        queue,
        surface: unsafe { std::mem::transmute(surface) },
        config: surface_config,
        pipeline,
    })
}

fn run_event_loop(
    window: Arc<Window>,
    mut wgpu_state: WgpuState,
    event_loop: EventLoop<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut running = true;
    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;
    let mut first_render_done = false;

    event_loop.run(move |event, window_target| {
        // Force continuous rendering for Wayland
        window_target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                let elapsed = now.duration_since(last_frame).as_secs_f32();
                if elapsed >= 1.0 {
                    fps = frame_count as f32 / elapsed;
                    frame_count = 0;
                    last_frame = now;
                    window.set_title(&format!("Holonic Realms - FPS: {:.1} - Wayland", fps));
                }
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested");
                    running = false;
                    window_target.exit();
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        if let winit::keyboard::PhysicalKey::Code(KeyCode::Escape) =
                            event.physical_key
                        {
                            println!("ESC pressed, exiting");
                            running = false;
                            window_target.exit();
                        }
                    }
                }

                WindowEvent::Resized(size) => {
                    println!("Window resized: {}x{}", size.width, size.height);
                    wgpu_state.config.width = size.width;
                    wgpu_state.config.height = size.height;
                    wgpu_state
                        .surface
                        .configure(&wgpu_state.device, &wgpu_state.config);
                }

                WindowEvent::Focused(focused) => {
                    if focused && !first_render_done {
                        println!("✓ Window focused!");
                    }
                }

                _ => {}
            },

            Event::AboutToWait => {
                if running {
                    frame_count += 1;
                    window.request_redraw();
                }
            }

            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::RedrawRequested,
            } => {
                // Get the current surface texture
                let output = match wgpu_state.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(SurfaceError::Outdated) => {
                        wgpu_state
                            .surface
                            .configure(&wgpu_state.device, &wgpu_state.config);
                        wgpu_state.surface.get_current_texture().unwrap()
                    }
                    Err(e) => {
                        eprintln!("Failed to get surface texture: {:?}", e);
                        return;
                    }
                };

                let view = output
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                // Create command encoder
                let mut encoder =
                    wgpu_state
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
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
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    // Draw a simple triangle
                    render_pass.set_pipeline(&wgpu_state.pipeline);
                    render_pass.draw(0..3, 0..1);
                }

                // Submit commands
                wgpu_state.queue.submit(Some(encoder.finish()));

                // PRESENT THE FRAME - THIS MAKES THE WINDOW VISIBLE ON WAYLAND!
                output.present();

                // First render complete - window should now be visible
                if !first_render_done {
                    first_render_done = true;
                    println!("═══════════════════════════════════════════════════════════════");
                    println!("✓✓✓ FIRST RENDER COMPLETE - WINDOW SHOULD NOW BE VISIBLE! ✓✓✓");
                    println!("═══════════════════════════════════════════════════════════════");
                    println!();
                    println!("If you still don't see the window:");
                    println!("  1. Press Alt+Tab to cycle through windows");
                    println!("  2. Check your taskbar/dock");
                    println!("  3. Check other virtual desktops");
                    println!();
                }
            }

            _ => {}
        }
    })?;

    Ok(())
}
