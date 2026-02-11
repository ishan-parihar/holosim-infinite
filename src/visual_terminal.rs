// Holonic Realms - Visual Terminal Simulation
//
// A beautiful terminal-based visualization

use holonic_realms::gui::ScaleLevel;
use holonic_realms::integrated_system::IntegratedSystem;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS - COSMOLOGICAL SIMULATION                   ║");
    println!("║   Watch the creation of reality emerge before you                  ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();

    println!("Initializing simulation...");
    let mut simulation = IntegratedSystem::new();
    simulation.initialize()?;

    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("SIMULATION STARTING");
    println!("════════════════════════════════════════════════════════════════════");
    println!();
    println!("The simulation will cycle through all 9 scale levels automatically.");
    println!("Watch as entities emerge and consciousness evolves!");
    println!();
    println!("Press Ctrl+C to stop.");
    println!();

    // Simulation state
    let mut sim_step = 0;
    let mut current_scale = ScaleLevel::Quantum;
    let mut last_scale_change = std::time::Instant::now();
    let mut frame_count = 0;
    let mut last_fps_update = std::time::Instant::now();
    let mut fps = 0.0;

    // Scale cycle
    let scales = vec![
        ScaleLevel::Quantum,
        ScaleLevel::Atomic,
        ScaleLevel::Molecular,
        ScaleLevel::Cellular,
        ScaleLevel::Organism,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
        ScaleLevel::Universal,
    ];
    let mut scale_index = 0;

    // Main loop
    loop {
        let frame_start = std::time::Instant::now();

        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        // Draw visualization
        draw_frame(&simulation, sim_step, fps, current_scale)?;

        io::stdout().flush()?;

        // Run simulation
        if let Ok(_) = simulation.run(1) {
            sim_step += 1;
        }

        // Change scale every 5 seconds
        if last_scale_change.elapsed() >= Duration::from_secs(5) {
            last_scale_change = std::time::Instant::now();
            scale_index = (scale_index + 1) % scales.len();
            current_scale = scales[scale_index].clone();
        }

        // Calculate FPS
        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_fps_update = std::time::Instant::now();
        }

        // Cap to ~10 FPS for readability
        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(100) {
            thread::sleep(Duration::from_millis(100) - frame_time);
        }
    }
}

fn draw_frame(
    sim: &IntegratedSystem,
    step: usize,
    fps: f64,
    scale: ScaleLevel,
) -> Result<(), io::Error> {
    let state = sim.state();
    let health = sim.health_metrics();
    let emergence = &state.emergence;

    // Title
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS - COSMOLOGICAL SIMULATION                   ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();

    // Status
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!(
        "│ Step: {:>8} │ FPS: {:>6.1} │ Coherence: {:>6.3f} │ Health: {:>6.1%} │",
        step, fps, state.coherence, health.overall_health
    );
    println!(
        "│ Entities: {:>6} │ Scale: {:>12} │ Time: {:>8.1}s │",
        state.entity_count,
        scale.name(),
        state.simulation_time
    );
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();

    // Coherence bar
    println!(
        "COHERENCE: [{:.0█<50}] {:.0}%",
        state.coherence * 50.0,
        state.coherence * 100.0
    );
    println!();

    // Spectrum
    println!("SPACE/TIME ────│──── TIME/SPACE");
    println!("           Veil");
    println!();

    // Scale info
    println!("CURRENT SCALE: {}", scale.name());
    println!("Description: {}", scale.description());
    println!("Size: {:.2e} meters", scale.scale_in_meters());
    println!();

    // Entities visualization
    println!("ENTITIES (showing first 250):");
    let display_count = std::cmp::min(state.entity_count, 250);
    let per_line = 50;
    let lines = (display_count + per_line - 1) / per_line;

    for line in 0..lines.min(5) {
        print!("  ");
        for col in 0..per_line {
            let idx = line * per_line + col;
            if idx < display_count {
                let symbol = ["●", "○", "◉", "◎", "◐", "◑", "◒"][idx % 7];
                print!("{}", symbol);
            }
        }
        println!();
    }

    if state.entity_count > 250 {
        println!("  ... and {} more entities", state.entity_count - 250);
    }
    println!();

    // Emergence
    println!("EMERGENCE METRICS:");
    println!("  Biological:");
    println!("    • Species: {}", emergence.biological.species_count);
    println!(
        "    • Genetic Diversity: {:.2}",
        emergence.biological.genetic_diversity
    );
    println!("  Noospheric:");
    println!(
        "    • Social Complexes: {}",
        emergence.noospheric.social_complexes_count
    );
    println!(
        "    • Collective Intelligence: {:.2}",
        emergence.noospheric.collective_intelligence
    );
    println!("  Gaia:");
    println!(
        "    • Consciousness: {:.2}",
        emergence.gaia.consciousness_score
    );
    println!(
        "    • Ecosystem Stability: {:.2}",
        emergence.gaia.ecosystem_stability
    );
    println!();

    // Health
    println!(
        "SYSTEM HEALTH: [{:.0█<40}] {:.0}%",
        health.overall_health * 40.0,
        health.overall_health * 100.0
    );
    println!();

    // Info
    println!("Scale changes every 5 seconds. Press Ctrl+C to stop.");

    Ok(())
}
