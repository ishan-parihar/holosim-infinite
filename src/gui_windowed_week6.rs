//! Week 6 Binary: Emergence Visualization
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 3 Week 6:
//! "Emergence metrics visualization: Biological level (genetic diversity, species count),
//! Noospheric level (social complexes, collective intelligence), Gaia level (consciousness, ecosystem stability)"
//!
//! This binary demonstrates:
//! - Emergence visualization with 3 levels (biological, noospheric, Gaia)
//! - Emergence dashboard with real-time graphs
//! - Particle systems for emergence
//! - Emergence event visualization

use holonic_realms::gui::ui::panels::emergence_dashboard::EmergenceDashboard;
use holonic_realms::gui::visualization::emergence_viz::{
    EmergenceEventType, EmergenceLevel, EmergenceVisualizer,
};
use nalgebra_glm::Vec3;
use std::time::{Duration, Instant};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     Holonic Realms - Week 6: Emergence Visualization         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Create emergence visualizer
    let mut visualizer = EmergenceVisualizer::new();

    // Create emergence dashboard
    let mut dashboard = EmergenceDashboard::new();

    println!("Emergence Visualization System Initialized");
    println!();

    // Simulation parameters
    let simulation_duration = Duration::from_secs(10);
    let update_interval = Duration::from_millis(100);
    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let mut simulation_time = 0.0;

    println!("Running simulation for {:?}...", simulation_duration);
    println!("Press Ctrl+C to exit early");
    println!();

    // Main simulation loop
    while start_time.elapsed() < simulation_duration {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Update simulation time
        simulation_time += delta_time as f64;

        // Update emergence metrics
        visualizer.update_metrics(simulation_time);

        // Update particles
        visualizer.update_particles(delta_time);

        // Randomly spawn emergence events
        if rand::random::<f32>() < 0.05 {
            let event_type = match rand::random::<u8>() % 3 {
                0 => EmergenceEventType::SpeciesEmergence,
                1 => EmergenceEventType::SocialComplexFormation,
                _ => EmergenceEventType::PlanetaryConsciousnessAwakening,
            };

            let position = Vec3::new(
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
            );

            let magnitude = rand::random::<f64>();
            visualizer.record_event(event_type, position, magnitude, vec![]);
        }

        // Print status every second
        if (simulation_time as i32) % 1 == 0 && (simulation_time * 10.0) as i32 % 10 == 0 {
            print_status(simulation_time, &visualizer, &dashboard);
        }

        // Sleep for update interval
        std::thread::sleep(update_interval);
    }

    println!();
    println!("Simulation complete!");
    println!();

    // Print final statistics
    print_final_statistics(&visualizer, &dashboard);
}

fn print_status(time: f64, visualizer: &EmergenceVisualizer, dashboard: &EmergenceDashboard) {
    let bio_metrics = visualizer.get_metrics(EmergenceLevel::Biological);
    let noo_metrics = visualizer.get_metrics(EmergenceLevel::Noospheric);
    let gaia_metrics = visualizer.get_metrics(EmergenceLevel::Gaia);

    println!("═══════════════════════════════════════════════════════════════");
    println!("Time: {:.2}s", time);
    println!();

    // Biological level
    println!("🌱 Biological Emergence:");
    match &bio_metrics {
        holonic_realms::gui::visualization::emergence_viz::EmergenceMetricsData::Biological(m) => {
            println!("  Genetic Diversity: {:.2}", m.genetic_diversity);
            println!("  Species Count: {}", m.species_count);
            println!("  Emergence Events: {}", m.emergence_events);
            println!("  Average Complexity: {:.2}", m.average_complexity);
            println!("  Evolution Rate: {:.2}", m.evolution_rate);
        }
        _ => {}
    }

    // Noospheric level
    println!();
    println!("💭 Noospheric Emergence:");
    match &noo_metrics {
        holonic_realms::gui::visualization::emergence_viz::EmergenceMetricsData::Noospheric(m) => {
            println!("  Social Complexes: {}", m.social_complexes);
            println!(
                "  Collective Intelligence: {:.2}",
                m.collective_intelligence
            );
            println!("  Thought Bubbles: {}", m.thought_bubbles);
            println!("  Average Complexity: {:.2}", m.average_complexity);
            println!("  Emergence Rate: {:.2}", m.emergence_rate);
        }
        _ => {}
    }

    // Gaia level
    println!();
    println!("🌍 Gaia Emergence:");
    match &gaia_metrics {
        holonic_realms::gui::visualization::emergence_viz::EmergenceMetricsData::Gaia(m) => {
            println!("  Consciousness: {:.2}", m.consciousness);
            println!("  Ecosystem Stability: {:.2}", m.ecosystem_stability);
            println!("  Glow Intensity: {:.2}", m.glow_intensity);
            println!("  Biodiversity: {:.2}", m.biodiversity);
            println!("  Emergence Events: {}", m.emergence_events);
        }
        _ => {}
    }

    // Particle counts
    println!();
    println!("✨ Particles:");
    println!(
        "  Biological: {}",
        visualizer.get_particle_count(EmergenceLevel::Biological)
    );
    println!(
        "  Noospheric: {}",
        visualizer.get_particle_count(EmergenceLevel::Noospheric)
    );
    println!(
        "  Gaia: {}",
        visualizer.get_particle_count(EmergenceLevel::Gaia)
    );
    println!("  Total: {}", visualizer.particles.len());

    // Event statistics
    println!();
    println!("📊 Events: {} total", visualizer.events.len());
}

fn print_final_statistics(visualizer: &EmergenceVisualizer, dashboard: &EmergenceDashboard) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                   Final Statistics                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let stats = dashboard.get_event_statistics(visualizer);

    println!("Total Events: {}", stats.total_events);
    println!("  Biological: {}", stats.biological_events);
    println!("  Noospheric: {}", stats.noospheric_events);
    println!("  Gaia: {}", stats.gaia_events);
    println!();

    println!("Event Types:");
    println!("  Species Emergence: {}", stats.species_emergence);
    println!("  Genetic Mutations: {}", stats.genetic_mutations);
    println!(
        "  Social Complex Formations: {}",
        stats.social_complex_formations
    );
    println!("  Collective Shifts: {}", stats.collective_shifts);
    println!("  Thought Bubbles: {}", stats.thought_bubbles);
    println!("  Planetary Awakenings: {}", stats.planetary_awakenings);
    println!(
        "  Ecosystem Reorganizations: {}",
        stats.ecosystem_reorganizations
    );
    println!();

    println!("Magnitude Statistics:");
    println!("  Total Magnitude: {:.2}", stats.total_magnitude);
    println!("  Average Magnitude: {:.2}", stats.average_magnitude);
    println!();

    // Recent events
    println!("Recent Events (last 5):");
    let recent_events = visualizer.events.iter().rev().take(5).collect::<Vec<_>>();
    for event in recent_events {
        println!(
            "  [{}] {} - Magnitude: {:.2}",
            event.level.name(),
            event.event_type.name(),
            event.magnitude
        );
    }
    println!();

    // Final metrics
    println!("Final Emergence Metrics:");
    let bio_metrics = visualizer.get_metrics(EmergenceLevel::Biological);
    let noo_metrics = visualizer.get_metrics(EmergenceLevel::Noospheric);
    let gaia_metrics = visualizer.get_metrics(EmergenceLevel::Gaia);

    println!(
        "  Biological Primary Metric: {:.2}",
        bio_metrics.primary_metric()
    );
    println!(
        "  Noospheric Primary Metric: {:.2}",
        noo_metrics.primary_metric()
    );
    println!(
        "  Gaia Primary Metric: {:.2}",
        gaia_metrics.primary_metric()
    );
    println!();

    println!("✓ Week 6 Emergence Visualization Demo Complete");
}
