// Holonic Realms - Interactive Terminal Visualization
//
// This creates a visually rich terminal-based simulation with:
// - Color-coded entities by density
// - Real-time progress bars
// - Visual representation of the spectrum
// - Interactive controls

use holonic_realms::gui::ScaleLevel;
use holonic_realms::integrated_system::IntegratedSystem;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor, style};

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
    println!("VISUALIZATION CONTROLS");
    println!("════════════════════════════════════════════════════════════════════");
    println!("  [SPACE] Pause/Resume");
    println!("  [1-9]   Change scale level");
    println!("  [+/-]   Change simulation speed");
    println!("  [Q]     Quit");
    println!("  [D]     Toggle detailed view");
    println!();
    println!("Press any key to begin...");
    println!();

    // Wait for key press
    let stdin = io::stdin();
    for _ in stdin.keys() {
        break;
    }

    // Switch to raw mode for interactive input
    let mut stdout = io::stdout().into_raw_mode()?;

    // Simulation state
    let mut paused = false;
    let mut time_speed = 1.0f64;
    let mut current_scale = ScaleLevel::Cellular;
    let mut detailed_view = false;
    let mut frame_count = 0;
    let mut last_fps_update = std::time::Instant::now();
    let mut fps = 0.0;
    let mut sim_step = 0;

    // Main loop
    loop {
        let frame_start = std::time::Instant::now();

        // Clear screen and draw header
        write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))?;

        // Draw header
        draw_header(
            &stdout,
            &simulation,
            sim_step,
            fps,
            current_scale,
            time_speed,
        )?;

        // Draw spectrum visualization
        draw_spectrum(&stdout, &simulation)?;

        // Draw entities visualization
        draw_entities(&stdout, &simulation, current_scale, detailed_view)?;

        // Draw emergence metrics
        draw_emergence(&stdout, &simulation)?;

        // Draw controls
        draw_controls(&stdout, paused, time_speed)?;

        stdout.flush()?;

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

        // Handle input (non-blocking)
        let stdin = io::stdin();
        let mut keys = stdin.keys();

        // Try to read a key without blocking
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char('q') | Key::Char('Q') | Key::Esc => {
                    write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))?;
                    println!("Simulation ended. Total steps: {}", sim_step);
                    break;
                }
                Key::Char(' ') => {
                    paused = !paused;
                }
                Key::Char('d') | Key::Char('D') => {
                    detailed_view = !detailed_view;
                }
                Key::Char('1') => current_scale = ScaleLevel::Quantum,
                Key::Char('2') => current_scale = ScaleLevel::Atomic,
                Key::Char('3') => current_scale = ScaleLevel::Molecular,
                Key::Char('4') => current_scale = ScaleLevel::Cellular,
                Key::Char('5') => current_scale = ScaleLevel::Organism,
                Key::Char('6') => current_scale = ScaleLevel::Planetary,
                Key::Char('7') => current_scale = ScaleLevel::Stellar,
                Key::Char('8') => current_scale = ScaleLevel::Galactic,
                Key::Char('9') => current_scale = ScaleLevel::Universal,
                Key::Char('+') | Key::Char('=') => {
                    time_speed = (time_speed * 1.5).min(1000.0);
                }
                Key::Char('-') => {
                    time_speed = (time_speed / 1.5).max(0.1);
                }
                _ => {}
            }
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
        if frame_time < Duration::from_millis(50) {
            thread::sleep(Duration::from_millis(50) - frame_time);
        }
    }

    Ok(())
}

fn draw_header<W: Write>(
    stdout: &mut W,
    sim: &IntegratedSystem,
    step: usize,
    fps: f64,
    scale: ScaleLevel,
    speed: f64,
) -> Result<(), io::Error> {
    let state = sim.state();
    let health = sim.health_metrics();

    write!(
        stdout,
        "{}{}{}HOLONIC REALMS - COSMOLOGICAL SIMULATION{}\r\n",
        color::Fg(color::Cyan),
        style::Bold,
        style::Reset,
        style::Reset
    )?;

    write!(stdout, "{}Step: {} {}FPS: {:.1} {}Coherence: {:.3} {}Health: {:.0}% {}Scale: {} {}Speed: {:.1}x {}Entities: {}\r\n",
        color::Fg(color::Yellow), step,
        color::Fg(color::Green), fps,
        color::Fg(color::Blue), state.coherence,
        color::Fg(color::Magenta), health.overall_health * 100.0,
        color::Fg(color::White), scale.name(),
        color::Fg(color::Cyan), speed,
        color::Fg(color::Red), state.entity_count
    )?;

    // Coherence bar
    write!(stdout, "{}Coherence: ", color::Fg(color::White))?;
    let bar_width = 40;
    let filled = (state.coherence * bar_width as f64) as usize;
    write!(stdout, "[")?;
    for i in 0..bar_width {
        if i < filled {
            write!(stdout, "{}█", color::Fg(color::Green))?;
        } else {
            write!(stdout, "{}░", color::Fg(color::DarkGrey))?;
        }
    }
    write!(
        stdout,
        "{}] {:.0}%\r\n\r\n",
        color::Fg(color::White),
        state.coherence * 100.0
    )?;

    Ok(())
}

fn draw_spectrum<W: Write>(stdout: &mut W, sim: &IntegratedSystem) -> Result<(), io::Error> {
    let state = sim.state();

    write!(
        stdout,
        "{}SPACE/TIME ↔ TIME/SPACE SPECTRUM\r\n",
        color::Fg(color::Cyan)
    )?;

    // Draw spectrum bar
    let bar_width = 60;
    let veil_position = bar_width / 2;

    // Space/Time side
    for i in 0..veil_position {
        let intensity = (i as f64 / veil_position as f64) * 255.0;
        if intensity < 85.0 {
            write!(stdout, "{}░", color::Fg(color::DarkBlue))?;
        } else if intensity < 170.0 {
            write!(stdout, "{}▓", color::Fg(color::Blue))?;
        } else {
            write!(stdout, "{}▒", color::Fg(color::LightBlue))?;
        }
    }

    // Veil
    write!(stdout, "{}│", color::Fg(color::White))?;

    // Time/Space side
    for i in 0..(bar_width - veil_position) {
        let intensity = (i as f64 / (bar_width - veil_position) as f64) * 255.0;
        if intensity < 85.0 {
            write!(stdout, "{}░", color::Fg(color::DarkMagenta))?;
        } else if intensity < 170.0 {
            write!(stdout, "{}▓", color::Fg(color::Magenta))?;
        } else {
            write!(stdout, "{}▒", color::Fg(color::LightMagenta))?;
        }
    }

    write!(stdout, "{}\r\n", color::Fg(color::White))?;
    write!(
        stdout,
        "Space/Time          Veil          Time/Space\r\n\r\n",
    )?;

    Ok(())
}

fn draw_entities<W: Write>(
    stdout: &mut W,
    sim: &IntegratedSystem,
    scale: ScaleLevel,
    detailed: bool,
) -> Result<(), io::Error> {
    let state = sim.state();
    let emergence = &state.emergence;

    write!(
        stdout,
        "{}ENTITIES AT {} SCALE ({})\r\n",
        color::Fg(color::Cyan),
        scale.name(),
        scale.description()
    )?;

    // Draw entity representation
    let display_count = std::cmp::min(state.entity_count, 1000);
    let per_row = 50;
    let rows = (display_count + per_row - 1) / per_row;

    for row in 0..rows.min(5) {
        for col in 0..per_row {
            let entity_index = row * per_row + col;
            if entity_index < display_count {
                // Color based on density (cycling through colors)
                let density = (entity_index % 8) + 1;
                let entity_color = match density {
                    1 => color::Red,
                    2 => color::Yellow,
                    3 => color::Green,
                    4 => color::Blue,
                    5 => color::Magenta,
                    6 => color::Cyan,
                    7 => color::White,
                    _ => color::LightRed,
                };
                write!(stdout, "{}●", color::Fg(entity_color))?;
            }
        }
        write!(stdout, "\r\n")?;
    }

    if state.entity_count > 5000 {
        write!(
            stdout,
            "{}... and {} more entities\r\n",
            color::Fg(color::DarkGrey),
            state.entity_count - 5000
        )?;
    }

    write!(stdout, "\r\n")?;

    // Detailed view
    if detailed {
        write!(stdout, "{}DETAILED EMERGENCE:\r\n", color::Fg(color::Cyan))?;
        write!(stdout, "{}  Biological:\r\n", color::Fg(color::Green))?;
        write!(
            stdout,
            "{}    - Genetic Diversity: {:.2}\r\n",
            color::Fg(color::White),
            emergence.biological.genetic_diversity
        )?;
        write!(
            stdout,
            "{}    - Species: {}\r\n",
            color::Fg(color::White),
            emergence.biological.species_count
        )?;
        write!(
            stdout,
            "{}    - Ecosystems: {}\r\n",
            color::Fg(color::White),
            emergence.biological.ecosystem_count
        )?;
        write!(stdout, "{}  Noospheric:\r\n", color::Fg(color::Blue))?;
        write!(
            stdout,
            "{}    - Social Complexes: {}\r\n",
            color::Fg(color::White),
            emergence.noospheric.social_complexes_count
        )?;
        write!(
            stdout,
            "{}    - Collective Intelligence: {:.2}\r\n",
            color::Fg(color::White),
            emergence.noospheric.collective_intelligence
        )?;
        write!(stdout, "{}  Gaia:\r\n", color::Fg(color::Magenta))?;
        write!(
            stdout,
            "{}    - Consciousness: {:.2}\r\n",
            color::Fg(color::White),
            emergence.gaia.consciousness_score
        )?;
        write!(
            stdout,
            "{}    - Ecosystem Stability: {:.2}\r\n",
            color::Fg(color::White),
            emergence.gaia.ecosystem_stability
        )?;
        write!(stdout, "\r\n")?;
    }

    Ok(())
}

fn draw_emergence<W: Write>(stdout: &mut W, sim: &IntegratedSystem) -> Result<(), io::Error> {
    let health = sim.health_metrics();

    write!(
        stdout,
        "{}SYSTEM EMERGENCE METRICS\r\n",
        color::Fg(color::Cyan)
    )?;

    // Overall health bar
    write!(stdout, "{}Overall Health: ", color::Fg(color::White))?;
    let bar_width = 30;
    let filled = (health.overall_health * bar_width as f64) as usize;
    write!(stdout, "[")?;
    for i in 0..bar_width {
        if i < filled {
            write!(stdout, "{}█", color::Fg(color::Green))?;
        } else {
            write!(stdout, "{}░", color::Fg(color::DarkGrey))?;
        }
    }
    write!(
        stdout,
        "{}] {:.0}%\r\n\r\n",
        color::Fg(color::White),
        health.overall_health * 100.0
    )?;

    Ok(())
}

fn draw_controls<W: Write>(stdout: &mut W, paused: bool, speed: f64) -> Result<(), io::Error> {
    write!(stdout, "{}CONTROLS: [SPACE] {}Pause/Resume │ [+/-] Speed: {:.1}x │ [1-9] Scale │ [D] Detail │ [Q] Quit{}\r\n",
        color::Fg(color::Cyan),
        if paused { color::Fg(color::Red) } else { color::Fg(color::Green) },
        speed,
        color::Fg(color::Reset)
    )?;

    Ok(())
}
