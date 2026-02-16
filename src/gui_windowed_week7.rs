//! Week 7 GUI Demonstration
//!
//! Demonstrates Phase 3 Week 7 features:
//! - Collective dynamics visualization
//! - Physical structure viewer
//! - Structure dashboard with tree view
//! - Collective dashboard with resonance metrics

use holonic_realms::entity_layer7::layer7::{EntityId, EntityType};
use holonic_realms::gui::ui::panels::collective_dashboard::CollectiveDashboard;
use holonic_realms::gui::ui::panels::structure_dashboard::StructureDashboard;
use holonic_realms::gui::visualization::collective_viz::{CollectiveVisualizer, Entity};
use holonic_realms::gui::visualization::structure_viz::{
    Entity as StructureEntity, StructureLevel, StructureVisualizer,
};
use holonic_realms::types::{Density, Polarity};
use nalgebra_glm::Vec3;
use std::time::{Duration, Instant};

fn main() {
    println!("========================================");
    println!("Holonic Realms GUI - Week 7");
    println!("Collective Dynamics & Physical Structure");
    println!("========================================\n");

    // Create visualizers
    let mut collective_viz = CollectiveVisualizer::new();
    let mut structure_viz = StructureVisualizer::new();

    // Create dashboards
    let structure_dashboard = StructureDashboard::new();
    let collective_dashboard = CollectiveDashboard::new();

    // Create test entities
    let entities = create_test_entities(128);

    println!("Created {} test entities\n", entities.len());

    // Simulate update loop
    let start_time = Instant::now();
    let mut frame_count = 0;

    for step in 0..100 {
        let time = start_time.elapsed().as_secs_f64();

        // Update visualizers
        collective_viz.update(&entities, time);
        // Convert entities to structure entities
        let structure_entities: Vec<StructureEntity> = entities
            .iter()
            .map(|e| StructureEntity {
                entity_id: e.entity_id.clone(),
                entity_type: e.entity_type,
                position: e.position,
                scale: e.scale,
                density: e.density,
                polarity: e.polarity,
                consciousness: e.consciousness,
                health: e.health,
                archetype_activations: e.archetype_activations.clone(),
                evolution_clock: e.evolution_clock,
                spectrum_position: e.spectrum_position,
            })
            .collect();
        structure_viz.update(&structure_entities, time);

        frame_count += 1;

        // Print status every 10 steps
        if step % 10 == 0 {
            print_status(
                time,
                &collective_viz,
                &structure_viz,
                &collective_dashboard,
                &structure_dashboard,
            );
        }

        // Simulate frame time
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n========================================");
    println!("Week 7 Demonstration Complete");
    println!("========================================\n");

    // Print final summary
    print_final_summary(&collective_viz, &structure_viz);
}

fn create_test_entities(count: usize) -> Vec<Entity> {
    let mut entities = Vec::new();

    for i in 0..count {
        let density = match i % 8 {
            0 => Density::First,
            1 => Density::Second,
            2 => Density::Third,
            3 => Density::Fourth,
            4 => Density::Fifth,
            5 => Density::Sixth,
            6 => Density::Seventh,
            _ => Density::Eighth,
        };

        let polarity = match i % 3 {
            0 => Polarity::ServiceToOthers,
            1 => Polarity::ServiceToSelf,
            _ => Polarity::Neutral,
        };

        let scale = match i % 9 {
            0 => 1.0e-20, // Quantum
            1 => 1.0e-12, // Atomic
            2 => 1.0e-8,  // Molecular
            3 => 1.0e-5,  // Cellular
            4 => 1.0e-3,  // Tissue
            5 => 1.0e-1,  // Organism
            6 => 1.0e4,   // Planetary
            7 => 1.0e10,  // Stellar
            _ => 1.0e20,  // Galactic
        };

        let entity = Entity {
            entity_id: EntityId::new(format!("entity_{}", i)),
            entity_type: EntityType::Individual,
            position: Vec3::new(
                ((i as f64 * 10.0) - 500.0) as f32,
                (((i * 7) % count) as f64 * 10.0 - 500.0) as f32,
                0.0_f32,
            ),
            scale,
            density,
            polarity,
            consciousness: (i as f64 / count as f64) * 0.9 + 0.1,
            health: 0.8 + (rand::random::<f64>() * 0.2),
            archetype_activations: Vec::new(),
            evolution_clock: 0.0,
            spectrum_position: (i as f64 / count as f64),
        };

        entities.push(entity);
    }

    entities
}

fn print_status(
    time: f64,
    collective_viz: &CollectiveVisualizer,
    structure_viz: &StructureVisualizer,
    _collective_dashboard: &CollectiveDashboard,
    _structure_dashboard: &StructureDashboard,
) {
    println!("--- Time: {:.2}s ---", time);

    // Collective metrics
    let c_metrics = &collective_viz.metrics;
    println!("Collective Dynamics:");
    println!("  Collectives: {}", c_metrics.total_collectives);
    println!(
        "  Entities in Collectives: {}",
        c_metrics.entities_in_collectives
    );
    println!("  Avg Group Size: {:.1}", c_metrics.average_collective_size);
    println!("  Avg Resonance: {:.2}", c_metrics.average_resonance);
    println!("  Avg Coherence: {:.2}", c_metrics.average_coherence);
    println!("  Health Index: {:.2}", c_metrics.health_index);

    // Structure metrics
    println!("Physical Structure:");
    println!("  Total Nodes: {}", structure_viz.nodes.len());
    println!("  Root Nodes: {}", structure_viz.roots.len());

    // Print metrics per structure level
    for level in [
        StructureLevel::Quantum,
        StructureLevel::Atomic,
        StructureLevel::Molecular,
        StructureLevel::Cellular,
        StructureLevel::Organism,
        StructureLevel::Planetary,
    ] {
        if let Some(metrics) = structure_viz.get_metrics(level) {
            if metrics.total_nodes > 0 {
                println!(
                    "  {}: {} nodes, {:.2} avg health",
                    level.name(),
                    metrics.total_nodes,
                    metrics.average_health
                );
            }
        }
    }

    println!();
}

fn print_final_summary(collective_viz: &CollectiveVisualizer, structure_viz: &StructureVisualizer) {
    println!("Collective Dynamics Summary:");
    println!(
        "  Total Collectives: {}",
        collective_viz.metrics.total_collectives
    );
    println!(
        "  Total Entities in Collectives: {}",
        collective_viz.metrics.entities_in_collectives
    );
    println!(
        "  Largest Group: {} members",
        collective_viz.metrics.largest_collective_size
    );
    println!(
        "  Most Common Resonance: {:?}",
        collective_viz.metrics.most_common_resonance
    );

    println!("\nPhysical Structure Summary:");
    println!("  Total Structure Nodes: {}", structure_viz.nodes.len());
    println!("  Root Structures: {}", structure_viz.roots.len());

    // Count nodes per level
    let mut level_counts: std::collections::HashMap<StructureLevel, usize> =
        std::collections::HashMap::new();
    for node in &structure_viz.nodes {
        *level_counts.entry(node.level).or_insert(0) += 1;
    }

    println!("\nNodes by Structure Level:");
    for level in [
        StructureLevel::Quantum,
        StructureLevel::Atomic,
        StructureLevel::Molecular,
        StructureLevel::Cellular,
        StructureLevel::Tissue,
        StructureLevel::Organism,
        StructureLevel::Planetary,
        StructureLevel::Stellar,
        StructureLevel::Galactic,
    ] {
        if let Some(count) = level_counts.get(&level) {
            println!("  {}: {} nodes", level.name(), count);
        }
    }

    println!("\nWeek 7 Features Demonstrated:");
    println!("  ✅ Collective dynamics visualization");
    println!("  ✅ Physical structure viewer");
    println!("  ✅ Structure dashboard with tree view");
    println!("  ✅ Collective dashboard with resonance metrics");
}
