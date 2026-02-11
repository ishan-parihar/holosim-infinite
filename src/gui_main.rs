// Holonic Realms GUI - Graphical Simulation Viewer
//
// This binary opens a windowed application to visualize the Holonic Realms
// simulation with real-time rendering, multi-scale viewing, and interactive controls.

use holonic_realms::gui::{
    GuiConfig, InteractionSystem, SpaceController, TimeController, VisualizationEngine,
};
use holonic_realms::integrated_system::IntegratedSystem;
use std::time::{Duration, Instant};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS GUI - SIMULATION VIEWER                      ║");
    println!("║   Real-time visualization of cosmological emergence                 ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize GUI configuration
    let gui_config = GuiConfig::new()
        .with_window_size(1920, 1080)
        .with_initial_zoom(1e-6) // Start at cellular scale
        .with_focus_dilation(true)
        .with_msaa(4)
        .with_vsync(true);

    println!("Initializing GUI system...");
    println!(
        "  Window size: {}x{}",
        gui_config.window_width, gui_config.window_height
    );
    println!("  Zoom level: {:.2e} m", gui_config.initial_zoom);
    println!(
        "  Focus dilation: {}",
        if gui_config.enable_focus_dilation {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!();

    // Initialize visualization engine
    println!("Creating visualization engine...");
    let mut viz_engine = VisualizationEngine::new(gui_config.clone());

    // Initialize time controller
    println!("Creating time controller...");
    let mut time_controller = TimeController::new();

    // Initialize space controller
    println!("Creating space controller...");
    let mut space_controller = SpaceController::new();

    // Initialize interaction system
    println!("Creating interaction system...");
    let _interaction_system = InteractionSystem::new();

    // Initialize integrated simulation system
    println!("Initializing integrated simulation system...");
    let mut simulation = IntegratedSystem::new();

    // Initialize the simulation system
    println!("Initializing simulation components...");
    if let Err(e) = simulation.initialize() {
        eprintln!("Failed to initialize simulation: {:?}", e);
        return;
    }
    println!("Simulation initialized successfully!");

    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("STARTING GUI");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    println!("Controls:");
    println!("  Mouse wheel: Zoom in/out");
    println!("  Click: Select entity");
    println!("  Drag: Pan camera");
    println!("  Keyboard: Time controls (space=pause, +/-=speed)");
    println!();
    println!("Scale Levels:");
    println!("  1: Quantum (10^-35 m) - Planck scale");
    println!("  2: Atomic (10^-10 m) - Subatomic particles");
    println!("  3: Molecular (10^-9 m) - Atoms and molecules");
    println!("  4: Cellular (10^-6 m) - Biological cells");
    println!("  5: Organism (10^-3 m) - Life forms");
    println!("  6: Planetary (10^6 m) - Planets and moons");
    println!("  7: Stellar (10^9 m) - Stars and solar systems");
    println!("  8: Galactic (10^21 m) - Galaxies");
    println!("  9: Universal (10^26 m) - Observable universe");
    println!();
    println!("Press Ctrl+C to exit");
    println!();

    // Main simulation loop
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut fps = 0.0;
    let mut sim_step = 0;

    loop {
        let frame_start = Instant::now();

        // Run simulation step
        if let Err(e) = simulation.run(1) {
            eprintln!("Simulation run error: {:?}", e);
        } else {
            sim_step += 1;
        }

        // Update visualization
        if let Err(e) = viz_engine.render_frame() {
            eprintln!("Render error: {:?}", e);
        }

        // Update time controller
        time_controller.update(1.0 / 60.0);

        // Update space controller
        space_controller.update(1.0 / 60.0);

        // Calculate FPS
        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_fps_update = Instant::now();

            // Print status every second
            let state = simulation.state();
            println!(
                "\rFPS: {:.1} | Step: {} | Coherence: {:.3} | Scale: {:.2e} m | Time: {:.2}x    ",
                fps,
                sim_step,
                state.coherence,
                space_controller.get_zoom(),
                time_controller.get_rate()
            );
        }

        // Cap frame rate to 60 FPS
        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
    }
}
