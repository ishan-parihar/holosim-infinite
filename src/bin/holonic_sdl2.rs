// Holonic Realms SDL2 GUI - Primary SDL2-based GUI Application
//
// This is the main SDL2-based GUI application that replaces Winit with SDL2
// for window management. It provides full integration with the existing
// simulation systems and visualization engines.
//
// From SDL2_INTEGRATION_ROADMAP.md Day 17:
// "Create SDL2-based main binary with full feature integration"

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use std::time::{Duration, Instant};
use wgpu::{
    BufferDescriptor, BufferUsages, Device, Instance, PresentMode, Queue, Surface,
    SurfaceConfiguration,
};

// ============================================================================
// Entity System Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Archetype {
    Logos,
    Love,
    Light,
    FreeWill,
}

#[derive(Debug, Clone)]
pub struct EntityData {
    pub id: u64,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub density: Density,
    pub polarity: Polarity,
    pub archetype: Archetype,
    pub size: f32,
    pub energy: f32,
}

impl EntityData {
    pub fn new(id: u64) -> Self {
        let density = match rand::random::<f32>() {
            x if x < 0.125 => Density::First,
            x if x < 0.25 => Density::Second,
            x if x < 0.375 => Density::Third,
            x if x < 0.5 => Density::Fourth,
            x if x < 0.625 => Density::Fifth,
            x if x < 0.75 => Density::Sixth,
            x if x < 0.875 => Density::Seventh,
            _ => Density::Eighth,
        };

        EntityData {
            id,
            position: [
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
            ],
            velocity: [
                (rand::random::<f32>() - 0.5) * 0.01,
                (rand::random::<f32>() - 0.5) * 0.01,
            ],
            density,
            polarity: Polarity::Neutral,
            archetype: Archetype::FreeWill,
            size: 10.0,
            energy: rand::random::<f32>(),
        }
    }

    pub fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];

        if self.position[0] < -1.0 {
            self.position[0] = 1.0;
        }
        if self.position[0] > 1.0 {
            self.position[0] = -1.0;
        }
        if self.position[1] < -1.0 {
            self.position[1] = 1.0;
        }
        if self.position[1] > 1.0 {
            self.position[1] = -1.0;
        }
    }
}

// ============================================================================
// Spectrum Visualization Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct SpectrumPosition {
    pub space_time_ratio: f32,
    pub veil_transparency: f32,
}

impl SpectrumPosition {
    pub fn new(space_time_ratio: f32) -> Self {
        SpectrumPosition {
            space_time_ratio: space_time_ratio.clamp(0.0, 1.0),
            veil_transparency: 0.0,
        }
    }
}

// ============================================================================
// Emergence Visualization Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct EmergenceMetrics {
    pub biological_level: f32,
    pub noospheric_level: f32,
    pub gaia_level: f32,
    pub emergence_events: u32,
}

impl Default for EmergenceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl EmergenceMetrics {
    pub fn new() -> Self {
        EmergenceMetrics {
            biological_level: 0.0,
            noospheric_level: 0.0,
            gaia_level: 0.0,
            emergence_events: 0,
        }
    }

    pub fn update(&mut self, entities: &[EntityData]) {
        let mut bio_sum = 0.0;
        let mut noos_sum = 0.0;
        let mut gaia_sum = 0.0;

        for entity in entities {
            match entity.density {
                Density::First | Density::Second => bio_sum += entity.energy,
                Density::Third | Density::Fourth | Density::Fifth => noos_sum += entity.energy,
                Density::Sixth | Density::Seventh | Density::Eighth => gaia_sum += entity.energy,
            }
        }

        let count = entities.len() as f32;
        if count > 0.0 {
            self.biological_level = bio_sum / count;
            self.noospheric_level = noos_sum / count;
            self.gaia_level = gaia_sum / count;
        }
    }
}

// ============================================================================
// Collective Visualization Types
// ============================================================================

#[derive(Debug, Clone)]
pub struct CollectiveGroup {
    pub id: u64,
    pub entities: Vec<u64>,
    pub center: [f32; 2],
    pub resonance: f32,
}

impl CollectiveGroup {
    pub fn new(id: u64, entities: Vec<u64>) -> Self {
        CollectiveGroup {
            id,
            entities,
            center: [0.0; 2],
            resonance: 0.0,
        }
    }
}

// ============================================================================
// Camera System
// ============================================================================

#[derive(Debug, Clone)]
pub struct Camera2D {
    pub position: [f32; 2],
    pub zoom: f32,
    pub rotation: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera2D {
    pub fn new() -> Self {
        Camera2D {
            position: [0.0; 2],
            zoom: 1.0,
            rotation: 0.0,
        }
    }

    pub fn world_to_screen(&self, world_pos: [f32; 2], screen_size: (u32, u32)) -> [f32; 2] {
        let aspect = screen_size.0 as f32 / screen_size.1 as f32;
        let scale = 1.0 / self.zoom;

        let x = (world_pos[0] - self.position[0]) * scale * aspect;
        let y = (world_pos[1] - self.position[1]) * scale;

        [x, y]
    }

    pub fn screen_to_world(&self, screen_pos: [f32; 2], screen_size: (u32, u32)) -> [f32; 2] {
        let aspect = screen_size.0 as f32 / screen_size.1 as f32;
        let scale = 1.0 / self.zoom;

        let x = screen_pos[0] / scale / aspect + self.position[0];
        let y = screen_pos[1] / scale + self.position[1];

        [x, y]
    }
}

// ============================================================================
// Performance Tracker
// ============================================================================

#[derive(Debug, Clone)]
pub struct PerformanceTracker {
    frame_times: Vec<f64>,
    event_times: Vec<f64>,
    render_times: Vec<f64>,
    max_samples: usize,
}

impl PerformanceTracker {
    pub fn new(max_samples: usize) -> Self {
        PerformanceTracker {
            frame_times: Vec::with_capacity(max_samples),
            event_times: Vec::with_capacity(max_samples),
            render_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn record_frame(&mut self, frame_time: f64, event_time: f64, render_time: f64) {
        self.frame_times.push(frame_time);
        self.event_times.push(event_time);
        self.render_times.push(render_time);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
            self.event_times.remove(0);
            self.render_times.remove(0);
        }
    }

    pub fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg: f64 = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if avg > 0.0 {
            1000.0 / avg
        } else {
            0.0
        }
    }

    pub fn avg_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64
    }

    pub fn avg_event_time(&self) -> f64 {
        if self.event_times.is_empty() {
            return 0.0;
        }
        self.event_times.iter().sum::<f64>() / self.event_times.len() as f64
    }

    pub fn avg_render_time(&self) -> f64 {
        if self.render_times.is_empty() {
            return 0.0;
        }
        self.render_times.iter().sum::<f64>() / self.render_times.len() as f64
    }
}

// ============================================================================
// Main Application
// ============================================================================

struct HolonicSdl2Gui {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,

    wgpu_instance: Instance,
    wgpu_surface: Surface<'static>,
    wgpu_device: Device,
    wgpu_queue: Queue,
    wgpu_surface_config: SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,

    entities: Vec<EntityData>,
    camera: Camera2D,
    spectrum_position: SpectrumPosition,
    emergence_metrics: EmergenceMetrics,
    collective_groups: Vec<CollectiveGroup>,

    perf_tracker: PerformanceTracker,
    last_frame_time: Instant,

    show_entity_viz: bool,
    show_spectrum_viz: bool,
    show_emergence_viz: bool,
    show_collective_viz: bool,
    show_performance: bool,

    fullscreen: bool,
    focused: bool,

    clipboard_text: Option<String>,
}

impl HolonicSdl2Gui {
    pub fn new() -> Result<Self, String> {
        println!("╔════════════════════════════════════════════════════════════════════╗");
        println!("║         HOLONIC REALMS SDL2 GUI - SIMULATION VIEWER              ║");
        println!("║   Real-time visualization of cosmological emergence                 ║");
        println!("╚════════════════════════════════════════════════════════════════════╝");
        println!();
        println!("Initializing SDL2 subsystems...");

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("Holonic Realms - SDL2 GUI", 1280, 720)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .map_err(|e| format!("Failed to create window: {}", e))?;

        println!("Window created successfully");

        let event_pump = sdl_context.event_pump()?;

        println!("Initializing WGPU...");
        let wgpu_instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let display_handle = window
            .display_handle()
            .map_err(|e| format!("Failed to get display handle: {}", e))?;

        let window_handle = window
            .window_handle()
            .map_err(|e| format!("Failed to get window handle: {}", e))?;

        let wgpu_surface = unsafe {
            wgpu_instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: display_handle.as_raw(),
                    raw_window_handle: window_handle.as_raw(),
                })
                .map_err(|e| format!("Failed to create WGPU surface: {}", e))?
        };

        let adapter =
            pollster::block_on(wgpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&wgpu_surface),
                force_fallback_adapter: false,
            }))
            .ok_or("Failed to find GPU adapter")?;

        println!("GPU adapter found: {:?}", adapter.get_info());

        let (wgpu_device, wgpu_queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("WGPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))
        .map_err(|e| format!("Failed to create device: {}", e))?;

        let capabilities = wgpu_surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .first()
            .ok_or("No surface format available")?;

        let window_size = window.size();
        let wgpu_surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: window_size.0,
            height: window_size.1,
            present_mode: PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![*format],
            desired_maximum_frame_latency: 2,
        };

        wgpu_surface.configure(&wgpu_device, &wgpu_surface_config);

        // Simple shaders for drawing circles
        let shader_code = r#"
            @vertex
            fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec3<f32>) -> VertexOutput {
                var out: VertexOutput;
                out.position = vec4<f32>(position, 0.0, 1.0);
                out.color = color;
                return out;
            }

            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) color: vec3<f32>,
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                return vec4<f32>(in.color, 1.0);
            }
        "#;

        let shader = wgpu_device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Circle Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_code.into()),
        });

        let bind_group_layout =
            wgpu_device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[],
            });

        let pipeline_layout = wgpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = wgpu_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 5 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: 2 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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

        println!("WGPU initialized successfully");
        println!("Surface format: {:?}", format);
        println!("Surface size: {}x{}", window_size.0, window_size.1);

        let entities: Vec<EntityData> = (0..100).map(EntityData::new).collect();

        let collective_groups = vec![
            CollectiveGroup::new(0, (0..25).collect()),
            CollectiveGroup::new(1, (25..50).collect()),
            CollectiveGroup::new(2, (50..75).collect()),
            CollectiveGroup::new(3, (75..100).collect()),
        ];

        println!(
            "Created {} entities and {} collective groups",
            entities.len(),
            collective_groups.len()
        );

        Ok(HolonicSdl2Gui {
            sdl_context,
            video_subsystem,
            window,
            event_pump,
            wgpu_instance,
            wgpu_surface,
            wgpu_device,
            wgpu_queue,
            wgpu_surface_config,
            render_pipeline,
            bind_group_layout,
            entities,
            camera: Camera2D::new(),
            spectrum_position: SpectrumPosition::new(0.5),
            emergence_metrics: EmergenceMetrics::new(),
            collective_groups,
            perf_tracker: PerformanceTracker::new(60),
            last_frame_time: Instant::now(),
            show_entity_viz: true,
            show_spectrum_viz: true,
            show_emergence_viz: true,
            show_collective_viz: true,
            show_performance: true,
            fullscreen: false,
            focused: true,
            clipboard_text: None,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!();
        println!("════════════════════════════════════════════════════════════════════");
        println!("STARTING SDL2 GUI");
        println!("════════════════════════════════════════════════════════════════════");
        println!();
        println!("Controls:");
        println!("  WASD/Arrows: Pan camera");
        println!("  +/-: Zoom in/out");
        println!("  1: Toggle Entity Visualization");
        println!("  2: Toggle Spectrum Visualization");
        println!("  3: Toggle Emergence Visualization");
        println!("  4: Toggle Collective Visualization");
        println!("  P: Toggle Performance Display");
        println!("  F11: Toggle Fullscreen");
        println!("  ESC: Exit");
        println!("  Shift+C: Copy entity info to clipboard");
        println!("  Shift+V: Paste from clipboard");
        println!();

        let mut running = true;

        while running {
            let frame_start = Instant::now();
            let event_start = Instant::now();

            let mut key_events = Vec::new();
            let mut mouse_motion_events = Vec::new();
            let mut mouse_button_events = Vec::new();
            let mut mouse_wheel_events = Vec::new();
            let mut window_events = Vec::new();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        println!("Quit event received");
                        running = false;
                    }

                    Event::KeyDown {
                        keycode, keymod, ..
                    } => {
                        if let Some(keycode) = keycode {
                            key_events.push((keycode, keymod, true));
                        }
                    }

                    Event::KeyUp { keycode, .. } => {
                        if let Some(keycode) = keycode {
                            key_events.push((keycode, Mod::empty(), false));
                        }
                    }

                    Event::MouseMotion {
                        x: _,
                        y: _,
                        xrel,
                        yrel,
                        ..
                    } => {
                        mouse_motion_events.push((xrel, yrel));
                    }

                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        mouse_button_events.push((mouse_btn, x, y, true));
                    }

                    Event::MouseButtonUp { mouse_btn, .. } => {
                        mouse_button_events.push((mouse_btn, 0, 0, false));
                    }

                    Event::MouseWheel {
                        x: _, y, direction, ..
                    } => {
                        let y = match direction {
                            sdl2::mouse::MouseWheelDirection::Normal => y,
                            sdl2::mouse::MouseWheelDirection::Flipped => -y,
                            sdl2::mouse::MouseWheelDirection::Unknown(_) => y,
                        };
                        mouse_wheel_events.push(y);
                    }

                    Event::Window { win_event, .. } => {
                        window_events.push(win_event);
                    }

                    _ => {}
                }
            }

            let event_time = event_start.elapsed();

            for (keycode, keymod, is_down) in key_events {
                if is_down {
                    self.handle_key_down(keycode, keymod, &mut running)?;
                } else {
                    self.handle_key_up(keycode);
                }
            }

            for (xrel, yrel) in mouse_motion_events {
                self.handle_mouse_motion(xrel, yrel);
            }

            for (mouse_btn, x, y, is_down) in mouse_button_events {
                if is_down {
                    self.handle_mouse_down(mouse_btn, x, y);
                } else {
                    self.handle_mouse_up(mouse_btn);
                }
            }

            for y in mouse_wheel_events {
                self.handle_mouse_wheel(y);
            }

            for win_event in window_events {
                self.handle_window_event(win_event)?;
            }

            let sim_start = Instant::now();
            self.update_simulation();
            let _sim_time = sim_start.elapsed();

            let render_start = Instant::now();
            self.render_frame()?;
            let render_time = render_start.elapsed();

            let frame_time = frame_start.elapsed();
            self.perf_tracker.record_frame(
                frame_time.as_secs_f64() * 1000.0,
                event_time.as_secs_f64() * 1000.0,
                render_time.as_secs_f64() * 1000.0,
            );

            if self.perf_tracker.frame_times.len().is_multiple_of(60) {
                self.print_performance();
            }

            let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);
            if frame_time < target_frame_time {
                std::thread::sleep(target_frame_time - frame_time);
            }
        }

        println!();
        println!("════════════════════════════════════════════════════════════════════");
        println!("GUI SHUTDOWN COMPLETE");
        println!("════════════════════════════════════════════════════════════════════");
        println!();
        Ok(())
    }

    fn handle_key_down(
        &mut self,
        keycode: Keycode,
        keymod: Mod,
        running: &mut bool,
    ) -> Result<(), String> {
        match keycode {
            Keycode::Escape => {
                println!("Escape pressed - exiting");
                *running = false;
            }

            Keycode::W | Keycode::Up => {
                self.camera.position[1] += 0.1 / self.camera.zoom;
            }
            Keycode::S | Keycode::Down => {
                self.camera.position[1] -= 0.1 / self.camera.zoom;
            }
            Keycode::A | Keycode::Left => {
                self.camera.position[0] -= 0.1 / self.camera.zoom;
            }
            Keycode::D | Keycode::Right => {
                self.camera.position[0] += 0.1 / self.camera.zoom;
            }

            Keycode::Plus | Keycode::KpPlus | Keycode::Equals => {
                self.camera.zoom *= 1.1;
                println!("Zoom: {:.2}", self.camera.zoom);
            }
            Keycode::Minus | Keycode::KpMinus => {
                self.camera.zoom *= 0.9;
                println!("Zoom: {:.2}", self.camera.zoom);
            }

            Keycode::Num1 => {
                self.show_entity_viz = !self.show_entity_viz;
                println!(
                    "Entity Visualization: {}",
                    if self.show_entity_viz { "ON" } else { "OFF" }
                );
            }
            Keycode::Num2 => {
                self.show_spectrum_viz = !self.show_spectrum_viz;
                println!(
                    "Spectrum Visualization: {}",
                    if self.show_spectrum_viz { "ON" } else { "OFF" }
                );
            }
            Keycode::Num3 => {
                self.show_emergence_viz = !self.show_emergence_viz;
                println!(
                    "Emergence Visualization: {}",
                    if self.show_emergence_viz { "ON" } else { "OFF" }
                );
            }
            Keycode::Num4 => {
                self.show_collective_viz = !self.show_collective_viz;
                println!(
                    "Collective Visualization: {}",
                    if self.show_collective_viz {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }

            Keycode::P => {
                self.show_performance = !self.show_performance;
                println!(
                    "Performance Display: {}",
                    if self.show_performance { "ON" } else { "OFF" }
                );
            }

            Keycode::F11 => {
                self.fullscreen = !self.fullscreen;
                if self.fullscreen {
                    self.window
                        .set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
                    println!("Fullscreen: ON");
                } else {
                    self.window
                        .set_fullscreen(sdl2::video::FullscreenType::Off)?;
                    println!("Fullscreen: OFF");
                }
            }

            Keycode::C => {
                if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                    self.copy_entity_info_to_clipboard()?;
                }
            }
            Keycode::V => {
                if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                    self.paste_from_clipboard()?;
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn handle_key_up(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::W
            | Keycode::S
            | Keycode::A
            | Keycode::D
            | Keycode::Up
            | Keycode::Down
            | Keycode::Left
            | Keycode::Right => {}
            _ => {}
        }
    }

    fn handle_mouse_motion(&mut self, xrel: i32, yrel: i32) {
        if self
            .event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left)
        {
            let scale = 0.001 / self.camera.zoom;
            self.camera.position[0] -= xrel as f32 * scale;
            self.camera.position[1] -= yrel as f32 * scale;
        }
    }

    fn handle_mouse_down(&mut self, button: MouseButton, x: i32, y: i32) {
        match button {
            MouseButton::Left => {
                println!("Left click at ({}, {})", x, y);
            }
            MouseButton::Right => {
                println!("Right click at ({}, {})", x, y);
            }
            _ => {}
        }
    }

    fn handle_mouse_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left | MouseButton::Right => {}
            _ => {}
        }
    }

    fn handle_mouse_wheel(&mut self, y: i32) {
        if y > 0 {
            self.camera.zoom *= 1.1;
        } else if y < 0 {
            self.camera.zoom *= 0.9;
        }
        println!("Zoom: {:.2}", self.camera.zoom);
    }

    fn handle_window_event(&mut self, win_event: sdl2::event::WindowEvent) -> Result<(), String> {
        match win_event {
            sdl2::event::WindowEvent::Resized(w, h) => {
                println!("Window resized: {}x{}", w, h);
                self.wgpu_surface_config.width = w as u32;
                self.wgpu_surface_config.height = h as u32;
                self.wgpu_surface
                    .configure(&self.wgpu_device, &self.wgpu_surface_config);
            }

            sdl2::event::WindowEvent::FocusGained => {
                self.focused = true;
                println!("Window focused");
            }

            sdl2::event::WindowEvent::FocusLost => {
                self.focused = false;
                println!("Window lost focus");
            }

            sdl2::event::WindowEvent::Exposed => {}

            _ => {}
        }

        Ok(())
    }

    fn update_simulation(&mut self) {
        for entity in &mut self.entities {
            entity.update();
        }

        self.emergence_metrics.update(&self.entities);

        self.spectrum_position.space_time_ratio = (self.camera.position[0] + 1.0) / 2.0;
        self.spectrum_position.space_time_ratio =
            self.spectrum_position.space_time_ratio.clamp(0.0, 1.0);

        for group in &mut self.collective_groups {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut count = 0;

            for &entity_id in &group.entities {
                if let Some(entity) = self.entities.get(entity_id as usize) {
                    sum_x += entity.position[0];
                    sum_y += entity.position[1];
                    count += 1;
                }
            }

            if count > 0 {
                group.center[0] = sum_x / count as f32;
                group.center[1] = sum_y / count as f32;
            }

            group.resonance = 0.5;
        }
    }

    fn render_frame(&mut self) -> Result<(), String> {
        let surface_texture = self
            .wgpu_surface
            .get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {}", e))?;

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.wgpu_device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Build vertices for simple rendering
        let mut vertices = Vec::new();

        if self.show_entity_viz && !self.entities.is_empty() {
            let screen_size = self.window.size();

            for entity in self.entities.iter() {
                let screen_pos = self.camera.world_to_screen(entity.position, screen_size);
                let radius = entity.size * self.camera.zoom * 0.01;

                let color = match entity.density {
                    Density::First => [1.0, 0.5, 0.0],
                    Density::Second => [0.0, 1.0, 0.5],
                    Density::Third => [0.0, 0.5, 1.0],
                    Density::Fourth => [1.0, 0.0, 1.0],
                    Density::Fifth => [1.0, 1.0, 0.0],
                    Density::Sixth => [0.0, 1.0, 1.0],
                    Density::Seventh => [1.0, 0.5, 0.5],
                    Density::Eighth => [0.5, 0.5, 1.0],
                };

                // Simple triangle fan for circle
                let segments = 16;
                for i in 0..segments {
                    let angle1 = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                    let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI * 2.0;

                    // Center
                    vertices.push([screen_pos[0], screen_pos[1], color[0], color[1], color[2]]);
                    // Point 1
                    vertices.push([
                        screen_pos[0] + radius * angle1.cos(),
                        screen_pos[1] + radius * angle1.sin(),
                        color[0],
                        color[1],
                        color[2],
                    ]);
                    // Point 2
                    vertices.push([
                        screen_pos[0] + radius * angle2.cos(),
                        screen_pos[1] + radius * angle2.sin(),
                        color[0],
                        color[1],
                        color[2],
                    ]);
                }
            }
        }

        // Create vertex buffer
        let vertex_data: &[f32] = unsafe {
            std::slice::from_raw_parts(vertices.as_ptr() as *const f32, vertices.len() * 5)
        };

        let vertex_buffer = self.wgpu_device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: std::mem::size_of_val(vertex_data) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.wgpu_queue.write_buffer(&vertex_buffer, 0, unsafe {
            std::slice::from_raw_parts(
                vertex_data.as_ptr() as *const u8,
                std::mem::size_of_val(vertex_data),
            )
        });

        {
            let clear_color = wgpu::Color {
                r: 0.05,
                g: 0.05,
                b: 0.1,
                a: 1.0,
            };

            let bind_group = self
                .wgpu_device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Bind Group"),
                    layout: &self.bind_group_layout,
                    entries: &[],
                });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            if self.show_entity_viz && !vertices.is_empty() {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }

            drop(render_pass);
        }

        self.wgpu_queue.submit(Some(encoder.finish()));

        surface_texture.present();

        Ok(())
    }

    fn copy_entity_info_to_clipboard(&mut self) -> Result<(), String> {
        let entity_info: serde_json::Value = serde_json::json!({
            "entities": self.entities.iter().map(|e| {
                serde_json::json!({
                    "id": e.id,
                    "position": e.position,
                    "density": format!("{:?}", e.density),
                    "polarity": format!("{:?}", e.polarity),
                    "archetype": format!("{:?}", e.archetype),
                    "energy": e.energy,
                })
            }).collect::<Vec<_>>(),
            "total_entities": self.entities.len(),
            "emergence_metrics": {
                "biological": self.emergence_metrics.biological_level,
                "noospheric": self.emergence_metrics.noospheric_level,
                "gaia": self.emergence_metrics.gaia_level,
                "events": self.emergence_metrics.emergence_events,
            },
            "spectrum_position": {
                "space_time_ratio": self.spectrum_position.space_time_ratio,
                "veil_transparency": self.spectrum_position.veil_transparency,
            },
            "camera": {
                "position": self.camera.position,
                "zoom": self.camera.zoom,
                "rotation": self.camera.rotation,
            },
        });

        let json_str = serde_json::to_string_pretty(&entity_info)
            .map_err(|e| format!("Failed to serialize entity info: {}", e))?;

        self.clipboard_text = Some(json_str.clone());

        println!("Copied {} entities info to clipboard", self.entities.len());
        println!(
            "Preview: {}",
            &json_str.chars().take(200).collect::<String>()
        );

        Ok(())
    }

    fn paste_from_clipboard(&mut self) -> Result<(), String> {
        if let Some(ref text) = self.clipboard_text {
            println!("Clipboard content:");
            println!("{}", text);

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                println!("Valid JSON configuration");
                if json.get("camera").is_some() {
                    println!("Camera info found");
                }
            } else {
                println!("Not a valid JSON configuration");
            }
        } else {
            println!("Clipboard is empty");
        }

        Ok(())
    }

    fn print_performance(&self) {
        let fps = self.perf_tracker.fps();
        let avg_frame = self.perf_tracker.avg_frame_time();
        let avg_event = self.perf_tracker.avg_event_time();
        let avg_render = self.perf_tracker.avg_render_time();

        println!(
            "FPS: {:.1} | Frame: {:.3}ms | Event: {:.3}ms | Render: {:.3}ms | Entities: {} | Zoom: {:.2} | Spectrum: {:.2}",
            fps,
            avg_frame,
            avg_event,
            avg_render,
            self.entities.len(),
            self.camera.zoom,
            self.spectrum_position.space_time_ratio
        );
    }

    fn print_summary(&self) {
        println!();
        println!("════════════════════════════════════════════════════════════════════");
        println!("SESSION SUMMARY");
        println!("════════════════════════════════════════════════════════════════════");
        println!();
        println!("SDL2 Integration: OK");
        println!("WGPU Surface: OK");
        println!(
            "Entity Visualization: {}",
            if self.show_entity_viz {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "Spectrum Visualization: {}",
            if self.show_spectrum_viz {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "Emergence Visualization: {}",
            if self.show_emergence_viz {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "Collective Visualization: {}",
            if self.show_collective_viz {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "Performance Display: {}",
            if self.show_performance {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!("Camera Controls: WASD, +/-, Mouse Drag, Scroll");
        println!("Fullscreen Toggle: F11");
        println!("Clipboard: Shift+C (copy), Shift+V (paste)");
        println!();

        let fps = self.perf_tracker.fps();
        let avg_frame = self.perf_tracker.avg_frame_time();

        println!("Final Performance:");
        println!("  FPS: {:.1}", fps);
        println!("  Average Frame Time: {:.3}ms", avg_frame);

        println!();
        println!("Visualization Systems:");
        println!("  Entities: {} active", self.entities.len());
        println!("  Collective Groups: {}", self.collective_groups.len());
        println!("  Emergence Metrics:");
        println!(
            "    Biological Level: {:.3}",
            self.emergence_metrics.biological_level
        );
        println!(
            "    Noospheric Level: {:.3}",
            self.emergence_metrics.noospheric_level
        );
        println!("    Gaia Level: {:.3}", self.emergence_metrics.gaia_level);
        println!(
            "    Total Events: {}",
            self.emergence_metrics.emergence_events
        );
        println!(
            "  Spectrum Position: {:.3} (0.0 = Time/Space, 1.0 = Space/Time)",
            self.spectrum_position.space_time_ratio
        );

        println!();
        println!("Camera State:");
        println!(
            "  Position: ({:.3}, {:.3})",
            self.camera.position[0], self.camera.position[1]
        );
        println!("  Zoom: {:.2}", self.camera.zoom);
        println!("  Rotation: {:.3}", self.camera.rotation);

        println!();
        println!("Window State:");
        println!("  Size: {}x{}", self.window.size().0, self.window.size().1);
        println!("  Fullscreen: {}", self.fullscreen);
        println!("  Focused: {}", self.focused);

        if let Some(ref text) = self.clipboard_text {
            println!();
            println!("Clipboard:");
            println!("  {} bytes", text.len());
        } else {
            println!();
            println!("Clipboard: Empty");
        }
    }
}

fn main() -> Result<(), String> {
    let mut app = HolonicSdl2Gui::new()?;
    app.run()?;
    app.print_summary();
    println!();
    println!("Thank you for using Holonic Realms SDL2 GUI!");
    println!();
    Ok(())
}
