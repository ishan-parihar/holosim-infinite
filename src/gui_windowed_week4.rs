// Week 4: Scale Transitions & Scale-Specific Viz (Simplified)
// Demonstrates smooth scale transitions and per-scale entity visualizations

use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use holonic_realms::{
    entity_layer7::layer7::{EntityId, EntityType},
    gui::{
        camera::{Camera2D, CameraControls, ScaleTransition},
        scene::entity_visualizer::EntityVisualizer,
    },
    types::Density,
};

// Constants
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

struct Week4Application {
    camera: Camera2D,
    camera_controls: CameraControls,
    scale_transition: ScaleTransition,
    entity_visualizer: EntityVisualizer,
    entity_ids: Vec<EntityId>,
    entity_types: Vec<EntityType>,
    entity_densities: Vec<Density>,
    fps: f32,
    last_frame_time: Instant,
    frame_count: u32,
    fps_update_time: Instant,
}

impl Week4Application {
    fn new() -> Self {
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

        let camera = Camera2D::new(aspect_ratio);
        let camera_controls = CameraControls::new();
        let scale_transition = ScaleTransition::default_duration();
        let entity_visualizer = EntityVisualizer::new();

        let mut app = Self {
            camera,
            camera_controls,
            scale_transition,
            entity_visualizer,
            entity_ids: Vec::new(),
            entity_types: Vec::new(),
            entity_densities: Vec::new(),
            fps: 0.0,
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_update_time: Instant::now(),
        };

        // Initialize entities
        app.initialize_entities();

        app
    }

    fn initialize_entities(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Create entities at different scales
        for i in 0..100 {
            let entity_id = EntityId::new(format!("entity_{}", i));

            // Distribute entities across scale levels
            let scale_factor = rng.gen_range(0.0..1.0);
            let (entity_type, density) = match scale_factor {
                s if s < 0.1 => (EntityType::Individual, Density::First),
                s if s < 0.2 => (EntityType::Individual, Density::Second),
                s if s < 0.3 => (EntityType::Individual, Density::Third),
                s if s < 0.4 => (EntityType::Individual, Density::Fourth),
                s if s < 0.5 => (EntityType::Individual, Density::Fifth),
                s if s < 0.6 => (EntityType::SolarLogos, Density::Sixth),
                s if s < 0.8 => (EntityType::SolarLogos, Density::Seventh),
                _ => (EntityType::GalacticLogos, Density::Eighth),
            };

            self.entity_ids.push(entity_id);
            self.entity_types.push(entity_type);
            self.entity_densities.push(density);
        }
    }

    fn handle_window_event(&mut self, event: &WindowEvent) {
        self.camera_controls.handle_event(event, &mut self.camera);

        // Handle keyboard shortcuts for scale transitions
        if let WindowEvent::KeyboardInput { event, .. } = event {
            if event.state == winit::event::ElementState::Pressed {
                if let winit::keyboard::Key::Character(ch) = &event.logical_key {
                    match ch.as_str() {
                        "1" => {
                            self.scale_transition
                                .start_zoom_transition(1e-30, &self.camera);
                        }
                        "2" => {
                            self.scale_transition
                                .start_zoom_transition(1e-15, &self.camera);
                        }
                        "3" => {
                            self.scale_transition
                                .start_zoom_transition(1e-3, &self.camera);
                        }
                        "4" => {
                            self.scale_transition
                                .start_zoom_transition(1.0, &self.camera);
                        }
                        "5" => {
                            self.scale_transition
                                .start_zoom_transition(1e10, &self.camera);
                        }
                        "6" => {
                            self.scale_transition
                                .start_zoom_transition(1e20, &self.camera);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn update(&mut self, _dt: Duration) {
        self.scale_transition.update(&mut self.camera);
        self.entity_visualizer.update_scale(&self.camera);

        self.frame_count += 1;
        if self.fps_update_time.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32;
            self.frame_count = 0;
            self.fps_update_time = Instant::now();
        }
    }

    fn print_status(&self) {
        println!("=== Week 4 Status ===");
        println!("FPS: {:.1}", self.fps);
        println!(
            "Current Scale: {}",
            self.entity_visualizer.current_scale().name()
        );
        println!("Camera Zoom: {:.2e}", self.camera.zoom);
        println!(
            "Camera Position: ({:.2}, {:.2}, {:.2})",
            self.camera.position.x, self.camera.position.y, self.camera.position.z
        );
        println!("Transition Active: {}", self.scale_transition.is_active());
        if self.scale_transition.is_active() {
            println!(
                "Transition Progress: {:.0}%",
                self.scale_transition.get_progress() * 100.0
            );
        }
        println!("Entity Count: {}", self.entity_ids.len());
        println!();

        let mut scale_counts = [0usize; 8];
        for density in &self.entity_densities {
            let index = *density as usize - 1;
            if index < 8 {
                scale_counts[index] += 1;
            }
        }

        let scale_names = [
            "Quantum",
            "Atomic",
            "Molecular",
            "Cellular",
            "Organism",
            "Planetary",
            "Stellar",
            "Galactic",
        ];
        for (i, count) in scale_counts.iter().enumerate() {
            println!("  {}: {} entities", scale_names[i], count);
        }
        println!();

        println!(
            "Visualization Styles at {} Scale:",
            self.entity_visualizer.current_scale().name()
        );
        for entity_type in [
            EntityType::Individual,
            EntityType::SolarLogos,
            EntityType::GalacticLogos,
        ] {
            let style = self
                .entity_visualizer
                .get_visualization_style(entity_type, self.entity_visualizer.current_scale());
            println!(
                "  {:?}: {}",
                entity_type,
                EntityVisualizer::style_name(style)
            );
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "=== Holonic Realms GUI - Week 4: Scale Transitions & Scale-Specific Viz (Simplified) ==="
    );
    println!("Features:");
    println!("  - Smooth scale transitions (0.5-1.0s)");
    println!("  - Per-scale entity visualizations");
    println!("  - Quantum: Probability clouds");
    println!("  - Atomic: Glowing particles");
    println!("  - Molecular: Bond networks");
    println!("  - Cellular: Organic shapes");
    println!("  - Organism: Detailed entities");
    println!("  - Planetary: Spheres with atmosphere");
    println!("  - Stellar: Glowing spheres with corona");
    println!("  - Galactic: Spiral galaxies");
    println!();
    println!("Controls:");
    println!("  - Mouse drag: Pan camera");
    println!("  - Mouse scroll: Zoom");
    println!("  - Keys 1-6: Jump to scale levels");
    println!("  - Q/E: Rotate camera");
    println!("  - Arrow keys: Pan camera");
    println!("  - Page Up/Down: Zoom");
    println!();
    println!("Initializing...");

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Holonic Realms - Week 4: Scale Transitions & Scale-Specific Viz")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)?;

    let mut app = Week4Application::new();

    println!("Ready!");
    app.print_status();

    let mut status_update_time = Instant::now();

    event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => {
                let _dt = app.last_frame_time.elapsed();
                app.last_frame_time = Instant::now();
                
                app.update(_dt);
                
                let transition_progress = app.scale_transition.get_progress() * 100.0;
                let transition_status = if app.scale_transition.is_active() {
                    format!("Transitioning: {:.0}%", transition_progress)
                } else {
                    "Idle".to_string()
                };
                
                window.set_title(&format!(
                    "Holonic Realms - Week 4 | FPS: {:.1} | Entities: {} | Scale: {} | Zoom: {:.2e} | {}",
                    app.fps,
                    app.entity_ids.len(),
                    app.entity_visualizer.current_scale().name(),
                    app.camera.zoom,
                    transition_status
                ));
                
                if status_update_time.elapsed() >= Duration::from_secs(5) {
                    app.print_status();
                    status_update_time = Instant::now();
                }
            }
            Event::WindowEvent { event, .. } => {
                app.handle_window_event(&event);
                
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        app.camera.aspect_ratio = physical_size.width as f32 / physical_size.height as f32;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
