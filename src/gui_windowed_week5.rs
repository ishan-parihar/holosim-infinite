// Week 5 Binary: Entity & Spectrum Visualization
// From GUI_IMPLEMENTATION_ROADMAP.md Phase 3 Week 5:
// "Advanced entity visualization: 3D rendering, density-based colors, polarity overlay, archetype activation glow"
// "Spectrum visualization overlay: Space/time vs time/space continuum, entity positions on spectrum, Veil indicator at v=1"

use holonic_realms::entity_layer7::layer7::{EntityId, EntityType};
use holonic_realms::gui::scene::entity_visualizer::ScaleLevel;
use holonic_realms::gui::ui::panels::{SpectrumDashboard, SpectrumEntityData};
use holonic_realms::gui::visualization::{
    entity_viz::{EntityVisualizationData, EntityVisualizer, GeometryType, VisualizationStyle},
    spectrum_viz::{SpectrumOverlay, SpectrumPosition, SpectrumRegion, SpectrumVisualizer},
};
use holonic_realms::types::{Density, Polarity};
use nalgebra_glm::Vec3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Phase 3 Week 5: Entity & Spectrum Visualization ===");
    println!("Demonstrating:");
    println!("- Advanced entity visualization with density, polarity, archetype coding");
    println!("- Spectrum visualization overlay");
    println!("- Real-time spectrum graph");
    println!("- Interactive spectrum controls");
    println!();

    // Initialize entity visualizer
    let mut entity_visualizer = EntityVisualizer::new();
    println!("✓ Entity visualizer created");
    println!("  - Current scale: {:?}", entity_visualizer.current_scale());
    println!(
        "  - Style mappings: {} entity types",
        entity_visualizer.style_mappings.len()
    );
    println!(
        "  - Geometry mappings: {} entity types",
        entity_visualizer.geometry_mappings.len()
    );
    println!();

    // Initialize spectrum visualizer
    let mut spectrum_visualizer = SpectrumVisualizer::new();
    println!("✓ Spectrum visualizer created");
    println!(
        "  - Veil transparency: {:.2}",
        spectrum_visualizer.veil_transparency()
    );
    println!(
        "  - Filter range: {:.2} - {:.2}",
        spectrum_visualizer.filter_range().0,
        spectrum_visualizer.filter_range().1
    );
    println!(
        "  - Overlay visible: {}",
        spectrum_visualizer.is_overlay_visible()
    );
    println!();

    // Initialize spectrum dashboard
    let mut spectrum_dashboard = SpectrumDashboard::new();
    println!("✓ Spectrum dashboard created");
    println!("  - Show graph: {}", spectrum_dashboard.show_graph);
    println!("  - Show histogram: {}", spectrum_dashboard.show_histogram);
    println!(
        "  - Show veil indicator: {}",
        spectrum_dashboard.show_veil_indicator
    );
    println!();

    // Create test entities
    println!("Creating test entities...");
    let test_entities = create_test_entities();
    println!("✓ Created {} test entities", test_entities.len());
    println!();

    // Demonstrate entity visualization
    println!("=== Entity Visualization Demo ===");
    for (i, entity) in test_entities.iter().take(5).enumerate() {
        let viz_data = entity_visualizer.entity_to_visualization_data(
            &entity.id,
            entity.entity_type,
            entity.density,
            entity.polarity,
            entity.consciousness,
            entity.archetype_activations,
            entity.position,
            entity.scale,
            entity.focused,
        );

        println!("Entity {}: {}", i + 1, entity.id.uuid);
        println!("  Type: {:?}", entity.entity_type);
        println!("  Density: {:?}", entity.density);
        println!("  Polarity: {:?}", entity.polarity);
        println!("  Consciousness: {:.2}", entity.consciousness);
        println!(
            "  Position: ({:.2}, {:.2}, {:.2})",
            entity.position.x, entity.position.y, entity.position.z
        );
        println!("  Scale: {:.2}", entity.scale);
        println!(
            "  Visualization style: {:?}",
            entity_visualizer
                .get_visualization_style(entity.entity_type, entity_visualizer.current_scale())
        );
        println!(
            "  Geometry type: {:?}",
            entity_visualizer.get_geometry_type(entity.entity_type)
        );
        println!(
            "  Color: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            viz_data.color[0], viz_data.color[1], viz_data.color[2], viz_data.color[3]
        );
        println!(
            "  Polarity color: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            viz_data.polarity_color[0],
            viz_data.polarity_color[1],
            viz_data.polarity_color[2],
            viz_data.polarity_color[3]
        );
        println!(
            "  Archetype glow: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            viz_data.archetype_glow[0],
            viz_data.archetype_glow[1],
            viz_data.archetype_glow[2],
            viz_data.archetype_glow[3]
        );
        println!();
    }

    // Demonstrate spectrum visualization
    println!("=== Spectrum Visualization Demo ===");
    for (i, entity) in test_entities.iter().take(5).enumerate() {
        let spectrum_pos = SpectrumPosition::from_ratio(entity.spectrum_ratio);
        let normalized_pos = spectrum_pos.normalized_position();
        let region = SpectrumRegion::from_position(normalized_pos);

        println!("Entity {}: {}", i + 1, entity.id.uuid);
        println!("  Spectrum ratio: {:.4}", entity.spectrum_ratio);
        println!("  Normalized position: {:.4}", normalized_pos);
        println!("  Region: {}", region.name());
        println!("  Space/Time: {:.4}", spectrum_pos.space_time);
        println!("  Time/Space: {:.4}", spectrum_pos.time_space);
        println!("  Is space/time: {}", spectrum_pos.is_space_time);
        println!(
            "  Distance from Veil: {:.4}",
            spectrum_pos.distance_from_veil()
        );
        println!("  Is at Veil: {}", spectrum_pos.is_at_veil());
        println!();
    }

    // Demonstrate spectrum dashboard
    println!("=== Spectrum Dashboard Demo ===");
    let spectrum_entities: Vec<SpectrumEntityData> = test_entities
        .iter()
        .map(|e| SpectrumEntityData {
            entity_id: e.id.uuid.parse().unwrap_or(0),
            spectrum_position: SpectrumPosition::from_ratio(e.spectrum_ratio).normalized_position()
                as f32,
            density: e.density,
            polarity: match e.polarity {
                Polarity::STO | Polarity::ServiceToOthers => 1.0,
                Polarity::STS | Polarity::ServiceToSelf => -1.0,
                Polarity::Neutral => 0.0,
                Polarity::SinkholeOfIndifference => 0.0,
            },
            consciousness: e.consciousness,
        })
        .collect();

    spectrum_dashboard.update_entities(spectrum_entities);
    println!(
        "✓ Updated spectrum dashboard with {} entities",
        spectrum_dashboard.total_entity_count()
    );
    println!(
        "  Filtered entities: {}",
        spectrum_dashboard.filtered_entity_count()
    );
    println!();

    println!("Entity distribution by spectrum region:");
    for region in &[
        SpectrumRegion::DeepTimeSpace,
        SpectrumRegion::MidTimeSpace,
        SpectrumRegion::NearVeilTimeSpace,
        SpectrumRegion::TheVeil,
        SpectrumRegion::NearVeilSpaceTime,
        SpectrumRegion::MidSpaceTime,
        SpectrumRegion::DeepSpaceTime,
    ] {
        let count = *spectrum_dashboard.distribution.get(region).unwrap_or(&0);
        println!("  {}: {} entities", region.name(), count);
    }
    println!();

    // Demonstrate spectrum filtering
    println!("=== Spectrum Filtering Demo ===");
    println!("Applying filter to Mid Space/Time region...");
    spectrum_dashboard
        .visualizer_mut()
        .select_region(SpectrumRegion::MidSpaceTime);
    println!(
        "  Filter range: {:.2} - {:.2}",
        spectrum_dashboard.visualizer().filter_range().0,
        spectrum_dashboard.visualizer().filter_range().1
    );
    println!(
        "  Filtered entities: {}",
        spectrum_dashboard.filtered_entity_count()
    );
    println!();

    println!("Clearing filter...");
    spectrum_dashboard.visualizer_mut().clear_filter();
    println!(
        "  Filter range: {:.2} - {:.2}",
        spectrum_dashboard.visualizer().filter_range().0,
        spectrum_dashboard.visualizer().filter_range().1
    );
    println!(
        "  Filtered entities: {}",
        spectrum_dashboard.filtered_entity_count()
    );
    println!();

    // Demonstrate visualization styles per scale
    println!("=== Visualization Styles Per Scale ===");
    use holonic_realms::gui::scene::entity_visualizer::ScaleLevel;
    for scale in &[
        ScaleLevel::Quantum,
        ScaleLevel::Atomic,
        ScaleLevel::Molecular,
        ScaleLevel::Cellular,
        ScaleLevel::Organism,
        ScaleLevel::Planetary,
        ScaleLevel::Stellar,
        ScaleLevel::Galactic,
    ] {
        let style = entity_visualizer.get_visualization_style(EntityType::Individual, *scale);
        println!("  {:?}: {}", scale, style.name());
    }
    println!();

    // Demonstrate density colors
    println!("=== Density Colors ===");
    for density in &[
        Density::First,
        Density::Second,
        Density::Third,
        Density::Fourth,
        Density::Fifth,
        Density::Sixth,
        Density::Seventh,
        Density::Eighth,
    ] {
        let color = EntityVisualizer::get_density_color(*density);
        println!(
            "  {:?}: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            density, color[0], color[1], color[2], color[3]
        );
    }
    println!();

    // Demonstrate polarity colors
    println!("=== Polarity Colors ===");
    for polarity in &[
        Polarity::STO,
        Polarity::STS,
        Polarity::Neutral,
        Polarity::SinkholeOfIndifference,
    ] {
        let color = EntityVisualizer::get_polarity_color(*polarity);
        println!(
            "  {:?}: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            polarity, color[0], color[1], color[2], color[3]
        );
    }
    println!();

    // Demonstrate archetype glow colors
    println!("=== Archetype Glow Colors (Sample) ===");
    for archetype_num in [1, 3, 7, 22] {
        let color = EntityVisualizer::get_archetype_glow_color(archetype_num);
        println!(
            "  Archetype {}: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            archetype_num, color[0], color[1], color[2], color[3]
        );
    }
    println!();

    // Demonstrate spectrum gradient colors
    println!("=== Spectrum Gradient Colors ===");
    let gradient_colors = spectrum_visualizer.get_gradient_colors();
    for (i, color) in gradient_colors.iter().enumerate() {
        println!(
            "  Region {}: RGBA({:.2}, {:.2}, {:.2}, {:.2})",
            i, color[0], color[1], color[2], color[3]
        );
    }
    println!();

    // Demonstrate spectrum overlay
    println!("=== Spectrum Overlay Demo ===");
    let spectrum_overlay = SpectrumOverlay::new(100.0, 500.0, 800.0, 50.0);
    println!(
        "  Position: ({:.2}, {:.2})",
        spectrum_overlay.position.0, spectrum_overlay.position.1
    );
    println!(
        "  Size: ({:.2}, {:.2})",
        spectrum_overlay.size.0, spectrum_overlay.size.1
    );
    println!("  Orientation: {:?}", spectrum_overlay.orientation);
    println!();

    // Test spectrum position conversion
    println!(
        "  Spectrum position 0.5 (Veil) -> Screen: {:?}",
        spectrum_overlay.spectrum_to_screen(0.5)
    );
    println!(
        "  Screen position (500.0, 525.0) -> Spectrum: {:?}",
        spectrum_overlay.screen_to_spectrum(500.0, 525.0)
    );
    println!();

    println!("=== Week 5 Demo Complete ===");
    println!("All Phase 3 Week 5 features demonstrated successfully!");
    println!();

    Ok(())
}

/// Test entity data
#[derive(Debug, Clone)]
struct TestEntity {
    id: EntityId,
    entity_type: EntityType,
    density: Density,
    polarity: Polarity,
    consciousness: f32,
    archetype_activations: [f32; 22],
    position: Vec3,
    scale: f32,
    spectrum_ratio: f64,
    focused: bool,
}

/// Create test entities for demonstration
fn create_test_entities() -> Vec<TestEntity> {
    vec![
        TestEntity {
            id: EntityId::new("entity-001".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Fourth,
            polarity: Polarity::STO,
            consciousness: 0.7,
            archetype_activations: create_archetype_activations(3, 0.8),
            position: Vec3::new(1.0, 0.0, 0.0),
            scale: 1.0e-3,
            spectrum_ratio: 1.5,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-002".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Sixth,
            polarity: Polarity::STS,
            consciousness: 0.9,
            archetype_activations: create_archetype_activations(22, 0.9),
            position: Vec3::new(2.0, 1.0, 0.0),
            scale: 1.0e-3,
            spectrum_ratio: 0.5,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-003".to_string()),
            entity_type: EntityType::SolarLogos,
            density: Density::Eighth,
            polarity: Polarity::STO,
            consciousness: 1.0,
            archetype_activations: create_archetype_activations(1, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0e9,
            spectrum_ratio: 10.0,
            focused: true,
        },
        TestEntity {
            id: EntityId::new("entity-004".to_string()),
            entity_type: EntityType::GalacticLogos,
            density: Density::Eighth,
            polarity: Polarity::STO,
            consciousness: 1.0,
            archetype_activations: create_archetype_activations(1, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0e21,
            spectrum_ratio: 100.0,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-005".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Third,
            polarity: Polarity::Neutral,
            consciousness: 0.4,
            archetype_activations: create_archetype_activations(5, 0.5),
            position: Vec3::new(3.0, 2.0, 1.0),
            scale: 1.0e-6,
            spectrum_ratio: 0.1,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-006".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Fifth,
            polarity: Polarity::STO,
            consciousness: 0.8,
            archetype_activations: create_archetype_activations(7, 0.7),
            position: Vec3::new(4.0, 3.0, 2.0),
            scale: 1.0e-3,
            spectrum_ratio: 1.0,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-007".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Seventh,
            polarity: Polarity::STS,
            consciousness: 0.95,
            archetype_activations: create_archetype_activations(12, 0.85),
            position: Vec3::new(5.0, 4.0, 3.0),
            scale: 1.0e-3,
            spectrum_ratio: 0.8,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-008".to_string()),
            entity_type: EntityType::Environmental,
            density: Density::First,
            polarity: Polarity::Neutral,
            consciousness: 0.1,
            archetype_activations: create_archetype_activations(14, 0.2),
            position: Vec3::new(1.0e6, 0.0, 0.0),
            scale: 1.0e6,
            spectrum_ratio: 20.0,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-009".to_string()),
            entity_type: EntityType::Collective,
            density: Density::Second,
            polarity: Polarity::STO,
            consciousness: 0.3,
            archetype_activations: create_archetype_activations(6, 0.4),
            position: Vec3::new(1.0e-6, 0.0, 0.0),
            scale: 1.0e-6,
            spectrum_ratio: 0.05,
            focused: false,
        },
        TestEntity {
            id: EntityId::new("entity-010".to_string()),
            entity_type: EntityType::Individual,
            density: Density::Eighth,
            polarity: Polarity::STO,
            consciousness: 1.0,
            archetype_activations: create_archetype_activations(22, 1.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0e-3,
            spectrum_ratio: 1.0,
            focused: true,
        },
    ]
}

/// Create archetype activations with one dominant archetype
fn create_archetype_activations(dominant: usize, activation: f32) -> [f32; 22] {
    let mut activations = [0.0_f32; 22];
    if dominant < 22 {
        activations[dominant] = activation;
    }
    activations
}
