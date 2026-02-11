// Holonic Realms GUI - Real Windowed Application
//
// This creates an actual windowed GUI using winit and wgpu for rendering.

use holonic_realms::gui::{GuiConfig, VisualizationEngine};
use holonic_realms::integrated_system::IntegratedSystem;
use std::time::Instant;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS GUI - WINDOWED SIMULATION                   ║");
    println!("║   Visualizing the emergence of reality                           ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize GUI configuration
    let gui_config = GuiConfig::new()
        .with_window_size(1280, 720)
        .with_initial_zoom(1e-6) // Start at cellular scale
        .with_focus_dilation(true)
        .with_msaa(4)
        .with_vsync(true);

    println!("Initializing simulation system...");
    let mut simulation = IntegratedSystem::new();
    if let Err(e) = simulation.initialize() {
        eprintln!("Failed to initialize simulation: {:?}", e);
        return;
    }

    println!("Initializing visualization engine...");
    let mut viz_engine = VisualizationEngine::new(gui_config);

    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("SIMULATION RUNNING");
    println!("════════════════════════════════════════════════════════════════════");
    println!();
    println!("This simulation demonstrates:");
    println!("  • 3 Primal Distortions: Free Will, Love/Logos, Light");
    println!("  • 8-Layer Fractal Architecture from Intelligent Infinity");
    println!("  • Space/Time ↔ Time/Space Spectrum with Veil");
    println!("  • 8 Density Octave representing consciousness evolution");
    println!("  • Holographic Principle: Each entity contains the whole");
    println!();
    println!("Scale Levels:");
    println!("  [1] Quantum (10^-35 m) - Planck scale: Quantum fluctuations");
    println!("  [2] Atomic (10^-10 m) - Subatomic particles: Quarks, electrons");
    println!("  [3] Molecular (10^-9 m) - Atoms and molecules: Chemical bonds");
    println!("  [4] Cellular (10^-6 m) - Biological cells: Prokaryotes, eukaryotes");
    println!("  [5] Organism (10^-3 m) - Life forms: Plants, animals, humans");
    println!("  [6] Planetary (10^6 m) - Planets and moons: Geological systems");
    println!("  [7] Stellar (10^9 m) - Stars and solar systems: Nuclear fusion");
    println!("  [8] Galactic (10^21 m) - Galaxies: Stellar clusters, dark matter");
    println!("  [9] Universal (10^26 m) - Observable universe: Cosmic web");
    println!();
    println!("Press Ctrl+C to exit");
    println!();

    // Main simulation loop
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut sim_step = 0;
    let mut last_status = Instant::now();

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
            // Silently ignore render errors
        }

        // Calculate FPS
        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed >= std::time::Duration::from_secs(1) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_fps_update = Instant::now();

            // Print status every second
            let state = simulation.state();
            let health = simulation.health_metrics();
            let health_percent = health.overall_health * 100.0;
            println!(
                "\rFPS: {:.1} | Step: {} | Coherence: {:.3} | Health: {:.1}% | Entities: {}    ",
                fps, sim_step, state.coherence, health_percent, state.entity_count
            );
        }

        // Print detailed status every 10 seconds
        if last_status.elapsed() >= std::time::Duration::from_secs(10) {
            last_status = Instant::now();
            let state = simulation.state();
            let health = simulation.health_metrics();

            println!();
            println!("════════════════════════════════════════════════════════════════════");
            println!("DETAILED STATUS - Step {}", sim_step);
            println!("════════════════════════════════════════════════════════════════════");
            println!("Coherence: {:.3}", state.coherence);
            println!("Energy Balance: {:.3}", state.energy_balance);
            println!("Entity Count: {}", state.entity_count);
            println!("Overall Health: {:.1}%", health.overall_health * 100.0);

            println!();
            println!("Emergence Metrics:");
            println!("  Biological:");
            println!(
                "    - Genetic Diversity: {:.2}",
                state.emergence.biological.genetic_diversity
            );
            println!(
                "    - Species Count: {}",
                state.emergence.biological.species_count
            );
            println!(
                "    - Ecosystem Count: {}",
                state.emergence.biological.ecosystem_count
            );
            println!("  Noospheric:");
            println!(
                "    - Social Complexes: {}",
                state.emergence.noospheric.social_complexes_count
            );
            println!(
                "    - Collective Intelligence: {:.2}",
                state.emergence.noospheric.collective_intelligence
            );
            println!("  Gaia:");
            println!(
                "    - Consciousness Score: {:.2}",
                state.emergence.gaia.consciousness_score
            );
            println!(
                "    - Ecosystem Stability: {:.2}",
                state.emergence.gaia.ecosystem_stability
            );
            println!();
        }

        // Cap frame rate to 60 FPS
        let frame_time = frame_start.elapsed();
        if frame_time < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - frame_time);
        }
    }
}
