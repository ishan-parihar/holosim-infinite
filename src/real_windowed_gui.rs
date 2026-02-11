// Holonic Realms - Actual Windowed GUI with WGPU Rendering
//
// This creates a real graphical window showing the simulation visualization.

use holonic_realms::gui::{GuiConfig, ScaleLevel, VisualizationEngine};
use holonic_realms::integrated_system::IntegratedSystem;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Holonic Realms GUI...");

    // Initialize simulation
    let mut simulation = IntegratedSystem::new();
    simulation.initialize()?;

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create window
    let window = WindowBuilder::new()
        .with_title("Holonic Realms - Cosmological Simulation")
        .with_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
        .build(&event_loop)?;

    println!("Window created successfully!");
    println!("Controls:");
    println!("  - Press 'Q' to quit");
    println!("  - Press '1-9' to change scale level");
    println!("  - Press 'P' to pause/resume");
    println!("  - Press '+'/'-' to change time speed");

    // Initialize GUI config
    let gui_config = GuiConfig::new()
        .with_window_size(1920, 1080)
        .with_initial_zoom(1e-6)
        .with_focus_dilation(true);

    // Initialize visualization engine
    let mut viz_engine = VisualizationEngine::new(gui_config);

    // Simulation state
    let mut paused = false;
    let mut time_speed = 1.0f32;
    let mut current_scale = ScaleLevel::Cellular;
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut fps = 0.0;
    let mut sim_step = 0;

    // Main event loop
    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Window close requested");
                    window_target.exit();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.logical_key {
                            Key::Named(NamedKey::Escape) | Key::Character('q') | Key::Character('Q') => {
                                println!("Quitting...");
                                window_target.exit();
                            }
                            Key::Character('p') | Key::Character('P') => {
                                paused = !paused;
                                println!("Simulation {}", if paused { "paused" } else { "resumed" });
                            }
                            Key::Character('1') => {
                                current_scale = ScaleLevel::Quantum;
                                println!("Scale: Quantum (10^-35 m)");
                            }
                            Key::Character('2') => {
                                current_scale = ScaleLevel::Atomic;
                                println!("Scale: Atomic (10^-10 m)");
                            }
                            Key::Character('3') => {
                                current_scale = ScaleLevel::Molecular;
                                println!("Scale: Molecular (10^-9 m)");
                            }
                            Key::Character('4') => {
                                current_scale = ScaleLevel::Cellular;
                                println!("Scale: Cellular (10^-6 m)");
                            }
                            Key::Character('5') => {
                                current_scale = ScaleLevel::Organism;
                                println!("Scale: Organism (10^-3 m)");
                            }
                            Key::Character('6') => {
                                current_scale = ScaleLevel::Planetary;
                                println!("Scale: Planetary (10^6 m)");
                            }
                            Key::Character('7') => {
                                current_scale = ScaleLevel::Stellar;
                                println!("Scale: Stellar (10^9 m)");
                            }
                            Key::Character('8') => {
                                current_scale = ScaleLevel::Galactic;
                                println!("Scale: Galactic (10^21 m)");
                            }
                            Key::Character('9') => {
                                current_scale = ScaleLevel::Universal;
                                println!("Scale: Universal (10^26 m)");
                            }
                            Key::Named(NamedKey::Plus) | Key::Character('=') => {
                                time_speed = (time_speed * 1.5).min(1000.0);
                                println!("Time speed: {:.1}x", time_speed);
                            }
                            Key::Named(NamedKey::Minus) => {
                                time_speed = (time_speed / 1.5).max(0.1);
                                println!("Time speed: {:.1}x", time_speed);
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Update simulation
                    if !paused {
                        let steps_to_run = if time_speed >= 1.0 {
                            time_speed as usize
                        } else {
                            if frame_count as f32 % (1.0 / time_speed) as i32 == 0 {
                                1
                            } else {
                                0
                            }
                        };
                        
                        for _ in 0..steps_to_run {
                            if let Ok(_) = simulation.run(1) {
                                sim_step += 1;
                            }
                        }
                    }
                    
                    // Update visualization
                    let _ = viz_engine.render_frame();
                    
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
                            "Holonic Realms | FPS: {:.1} | Step: {} | Coherence: {:.3} | Health: {:.0}% | Scale: {} | Speed: {:.1}x | Entities: {}",
                            fps,
                            sim_step,
                            state.coherence,
                            health.overall_health * 100.0,
                            current_scale.name(),
                            time_speed,
                            state.entity_count
                        );
                        window.set_title(&title);
                    }
                    
                    // Request next frame
                    window.request_redraw();
                }
                WindowEvent::Resized(physical_size) => {
                    println!("Window resized to: {}x{}", physical_size.width, physical_size.height);
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
