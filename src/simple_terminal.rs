// Holonic Realms - Simple Interactive Terminal Visualization
//
// This creates a visually rich terminal-based simulation without external dependencies

use holonic_realms::integrated_system::IntegratedSystem;
use holonic_realms::gui::ScaleLevel;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS - INTERACTIVE VISUALIZATION                ║");
    println!("║   Watch the creation of reality unfold before your eyes           ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();
    
    println!("Initializing simulation...");
    let mut simulation = IntegratedSystem::new();
    simulation.initialize()?;
    
    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("VISUALIZATION RUNNING");
    println!("════════════════════════════════════════════════════════════════════");
    println!();
    println!("The simulation is now running!");
    println!("You can see:");
    println!("  • Real-time entity emergence and evolution");
    println!("  • Coherence and system health metrics");
    println!("  • Multi-scale visualization (9 levels from Quantum to Universal)");
    println!("  • Biological, Noospheric, and Gaia emergence");
    println!();
    println!("This demonstrates:");
    println!("  • 3 Primal Distortions: Free Will, Love/Logos, Light");
    println!("  • 8-Layer Fractal Architecture from Intelligent Infinity");
    println!("  • Space/Time ↔ Time/Space Spectrum with Veil");
    println!("  • 8 Density Octave representing consciousness evolution");
    println!("  • Holographic Principle: Each entity contains the whole");
    println!();
    println!("Press Ctrl+C to stop the simulation");
    println!();
    
    // Simulation state
    let mut paused = false;
    let mut time_speed = 1.0f64;
    let mut current_scale = ScaleLevel::Cellular;
    let mut show_detailed = false;
    let mut frame_count = 0;
    let mut last_fps_update = std::time::Instant::now();
    let mut fps = 0.0;
    let mut sim_step = 0;
    let mut last_status = std::time::Instant::now();
    
    // Flag for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Handle Ctrl+C
    ctrlc::set_handler(move || {
        println!("\n\nShutting down simulation...");
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    // Main loop
    while running.load(Ordering::SeqCst) {
        let frame_start = std::time::Instant::now();
        
        // Clear screen (works on most terminals)
        print!("\x1B[2J\x1B[1;1H");
        
        // Draw visualization
        draw_visualization(&simulation, sim_step, fps, current_scale, time_speed, show_detailed)?;
        
        io::stdout().flush()?;
        
        // Update simulation
        if !paused {
            let steps_to_run = if time_speed >= 1.0 {
                time_speed as usize
            } else {
                if frame_count as f64 % (1.0 / time_speed) as i64 == 0 {
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
        
        // Cycle through scales automatically every 5 seconds
        if last_status.elapsed() >= Duration::from_secs(5) {
            last_status = std::time::Instant::now();
            current_scale = match current_scale {
                ScaleLevel::Quantum => ScaleLevel::Atomic,
                ScaleLevel::Atomic => ScaleLevel::Molecular,
                ScaleLevel::Molecular => ScaleLevel::Cellular,
                ScaleLevel::Cellular => ScaleLevel::Organism,
                ScaleLevel::Organism => ScaleLevel::Planetary,
                ScaleLevel::Planetary => ScaleLevel::Stellar,
                ScaleLevel::Stellar => ScaleLevel::Galactic,
                ScaleLevel::Galactic => ScaleLevel::Universal,
                ScaleLevel::Universal => ScaleLevel::Quantum,
            };
            show_detailed = !show_detailed;
        }
        
        // Calculate FPS
        frame_count += 1;
        let elapsed = last_fps_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_fps_update = std::time::Instant::now();
        }
        
        // Cap frame rate
        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(100) {
            thread::sleep(Duration::from_millis(100) - frame_time);
        }
    }
    
    // Final status
    println!("\n");
    println!("════════════════════════════════════════════════════════════════════");
    println!("SIMULATION COMPLETE");
    println!("════════════════════════════════════════════════════════════════════");
    println!();
    let state = simulation.state();
    let health = simulation.health_metrics();
    println!("Total Steps: {}", sim_step);
    println!("Final Coherence: {:.3}", state.coherence);
    println!("Final Health: {:.1}%", health.overall_health * 100.0);
    println!("Total Entities: {}", state.entity_count);
    println!();
    println!("Thank you for experiencing the Holonic Realms simulation!");
    println!("\"All is One, and One is All\"");
    
    Ok(())
}

fn draw_visualization(sim: &IntegratedSystem, step: usize, fps: f64, 
                     scale: ScaleLevel, speed: f64, detailed: bool) -> Result<(), io::Error> {
    let state = sim.state();
    let health = sim.health_metrics();
    let emergence = &state.emergence;
    
    // Header
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║         HOLONIC REALMS - COSMOLOGICAL SIMULATION                   ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Status bar
    println!("┌─ STATUS ─────────────────────────────────────────────────────────────┐");
    println!("│ Step: {:>8} │ FPS: {:>6.1} │ Coherence: {:>6.3f} │ Health: {:>6.1%} │", 
        step, fps, state.coherence, health.overall_health);
    println!("│ Entities: {:>6} │ Scale: {:>12} │ Speed: {:>6.1}x │ Time: {:>6.0}s │",
        state.entity_count, scale.name(), speed, state.simulation_time);
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Coherence visualization
    println!("┌─ COHERENCE ─────────────────────────────────────────────────────────┐");
    let bar_width = 50;
    let filled = (state.coherence * bar_width as f64) as usize;
    print!("│ [");
    for i in 0..bar_width {
        if i < filled {
            print!("█");
        } else {
            print░("░");
        }
    }
    println!("] {:.0}% │", state.coherence * 100.0);
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Spectrum visualization
    println!("┌─ SPACE/TIME ↔ TIME/SPACE SPECTRUM ────────────────────────────────┐");
    let spectrum_width = 60;
    let veil_pos = spectrum_width / 2;
    
    print!("│ ");
    // Space/Time side (blue gradient)
    for i in 0..veil_pos {
        let intensity = i as f64 / veil_pos as f64;
        if intensity < 0.33 {
            print░("░");
        } else if intensity < 0.66 {
            print▒("▒");
        } else {
            print▓("▓");
        }
    }
    print("│"); // Veil
    // Time/Space side (magenta gradient)
    for i in 0..(spectrum_width - veil_pos) {
        let intensity = i as f64 / (spectrum_width - veil_pos) as f64;
        if intensity < 0.33 {
            print░("░");
        } else if intensity < 0.66 {
            print▒("▒");
        } else {
            print▓("▓");
        }
    }
    println!(" │");
    println!("│ Space/Time          Veil          Time/Space                    │");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Scale level info
    println!("┌─ CURRENT SCALE: {} ───────────────────────────────────────┐", scale.name());
    println!("│ {} │", scale.description());
    println!("│ Scale: {:.2e} meters                                                  │", scale.scale_in_meters());
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Entity visualization
    println!("┌─ ENTITIES ──────────────────────────────────────────────────────────┐");
    let display_count = std::cmp::min(state.entity_count, 500);
    let per_line = 50;
    let lines = (display_count + per_line - 1) / per_line;
    
    for line in 0..lines.min(6) {
        print!("│ ");
        for col in 0..per_line {
            let idx = line * per_line + col;
            if idx < display_count {
                // Color by density
                let density = (idx % 7) as i32;
                let symbol = match density {
                    0 => "●", // Density 1 - Red
                    1 => "○", // Density 2 - Orange
                    2 => "◉", // Density 3 - Yellow
                    3 => "◎", // Density 4 - Green
                    4 => "◐", // Density 5 - Blue
                    5 => "◑", // Density 6 - Indigo
                    _ => "◒", // Density 7 - Violet
                };
                print!("{}", symbol);
            }
        }
        println!(" │");
    }
    
    if state.entity_count > 500 {
        println!("│ ... and {} more entities                                        │", 
            state.entity_count - 500);
    }
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Emergence metrics
    if detailed {
        println!("┌─ EMERGENCE METRICS (DETAILED) ────────────────────────────────────┐");
        
        // Biological
        println!("│ 🧬 Biological Emergence:                                          │");
        println!("│   • Genetic Diversity:    {:.4}                               │", 
            emergence.biological.genetic_diversity);
        println!("│   • Species Count:        {:>6}                               │", 
            emergence.biological.species_count);
        println!("│   • Ecosystem Count:      {:>6}                               │", 
            emergence.biological.ecosystem_count);
        println!("│   • Mutation Rate:        {:.4}                               │", 
            emergence.biological.mutation_rate);
        
        // Noospheric
        println!("│ 🌐 Noospheric Emergence:                                          │");
        println!("│   • Social Complexes:    {:>6}                               │", 
            emergence.noospheric.social_complexes_count);
        println!("│   • Collective Intel:    {:.4}                               │", 
            emergence.noospheric.collective_intelligence);
        println!("│   • Cultural Diversity:  {:.4}                               │", 
            emergence.noospheric.cultural_diversity);
        
        // Gaia
        println!("│ 🌍 Gaia Emergence:                                                │");
        println!("│   • Consciousness:       {:.4}                               │", 
            emergence.gaia.consciousness_score);
        println!("│   • Ecosystem Stability:{:.4}                               │", 
            emergence.gaia.ecosystem_stability);
        println!("│   • Atmospheric Health:  {:.4}                               │", 
            emergence.gaia.atmospheric_health);
        println!("│   • Climate Stability:   {:.4}                               │", 
            emergence.gaia.climate_stability);
        
        println!("└─────────────────────────────────────────────────────────────────────┘");
        println!();
    } else {
        println!("┌─ EMERGENCE METRICS ─────────────────────────────────────────────────┐");
        println!("│ 🧬 Biological: Species={}, Diversity={:.2}                      │",
            emergence.biological.species_count,
            emergence.biological.genetic_diversity);
        println!("│ 🌐 Noospheric: Complexes={}, Intel={:.2}                        │",
            emergence.noospheric.social_complexes_count,
            emergence.noospheric.collective_intelligence);
        println!("│ 🌍 Gaia: Consciousness={:.2}, Stability={:.2}                   │",
            emergence.gaia.consciousness_score,
            emergence.gaia.ecosystem_stability);
        println!("└─────────────────────────────────────────────────────────────────────┘");
        println!();
    }
    
    // System health
    println!("┌─ SYSTEM HEALTH ─────────────────────────────────────────────────────┐");
    let health_bar_width = 40;
    let health_filled = (health.overall_health * health_bar_width as f64) as usize;
    print!("│ Overall: [");
    for i in 0..health_bar_width {
        if i < health_filled {
            print!("█");
        } else {
            print░("░");
        }
    }
    println!("] {:.0}% │", health.overall_health * 100.0);
    println!("└─────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Footer
    println!("┌─ CONTROLS ───────────────────────────────────────────────────────────┐");
    println!("│ [Ctrl+C] Stop  │ Auto-cycling through scales every 5 seconds        │");
    println!("│ [D] Toggle Detail View (coming in next update)                      │");
    println!("└─────────────────────────────────────────────────────────────────────┘");
    
    Ok(())
}

// Helper functions for printing symbols
fn print░(s: &str) {
    print!("{}", s);
}

fn print▒(s: &str) {
    print!("{}", s);
}

fn print▓(s: &str) {
    print!("{}", s);
}