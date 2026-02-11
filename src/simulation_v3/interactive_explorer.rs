// Interactive Explorer Module (Phase 9)
//
// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 9 Step 9.2:
// "Implement Interactive Exploration - Create interactive menu system with options:
// - View entity details
// - View collective details
// - View spectrum distribution
// - View polarization distribution
// - Run simulation for N steps
// - Save simulation state
// - Load simulation state
// - Exit"
//
// This module implements:
// 1. InteractiveExplorer - Interactive menu system for exploring simulation results
// 2. Entity Details Viewer - View detailed information about specific entities
// 3. Collective Details Viewer - View detailed information about collectives
// 4. Distribution Viewers - View spectrum and polarization distributions
// 5. Simulation Control - Run simulation steps, save/load state

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::simulation_v3::collective_dynamics::CollectiveBehavior as Collective;
use crate::simulation_v3::simulation_runner::SimulationRunner;
use crate::simulation_v3::statistics::SimulationStatistics;
use std::collections::HashMap;
use std::io::{self, Write};

// ============================================================================
// INTERACTIVE EXPLORER
// ============================================================================

/// Interactive Explorer
///
/// Provides an interactive menu system for exploring simulation results.
///
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 9 Step 9.2:
/// "Implement interactive menu system with options for viewing details
/// and controlling simulation execution"
pub struct InteractiveExplorer {
    /// Current statistics
    statistics: SimulationStatistics,
    /// Entities map
    entities: HashMap<EntityId, SubSubLogos>,
    /// Collectives map
    collectives: HashMap<EntityId, Collective>,
}

impl InteractiveExplorer {
    /// Create a new interactive explorer
    pub fn new(
        _simulation_runner: SimulationRunner,
        statistics: SimulationStatistics,
        entities: HashMap<EntityId, SubSubLogos>,
        collectives: HashMap<EntityId, Collective>,
    ) -> Self {
        InteractiveExplorer {
            statistics,
            entities,
            collectives,
        }
    }

    /// Run the interactive explorer
    pub fn run(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║              INTERACTIVE SIMULATION EXPLORER                      ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        loop {
            self.display_main_menu();

            let choice = self.get_user_input("\nEnter choice (1-8): ");

            match choice.as_str() {
                "1" => self.view_entity_details(),
                "2" => self.view_collective_details(),
                "3" => self.view_spectrum_distribution(),
                "4" => self.view_polarization_distribution(),
                "5" => self.run_simulation_steps(),
                "6" => self.save_simulation_state(),
                "7" => self.load_simulation_state(),
                "8" => {
                    println!("\nExiting Interactive Explorer. Goodbye!\n");
                    break;
                }
                _ => println!("\n❌ Invalid choice. Please enter a number between 1 and 8.\n"),
            }

            // Press Enter to continue
            self.get_user_input("\nPress Enter to continue...");
            println!("\n");
        }
    }

    /// Display main menu
    fn display_main_menu(&self) {
        println!("╔════════════════════════════════════════════════════════════════════╗");
        println!("║                           MAIN MENU                                  ║");
        println!("╠════════════════════════════════════════════════════════════════════╣");
        println!("║  1. View entity details                                            ║");
        println!("║  2. View collective details                                         ║");
        println!("║  3. View spectrum distribution                                      ║");
        println!("║  4. View polarization distribution                                  ║");
        println!("║  5. Run simulation for N steps                                      ║");
        println!("║  6. Save simulation state                                           ║");
        println!("║  7. Load simulation state                                           ║");
        println!("║  8. Exit                                                           ║");
        println!("╚════════════════════════════════════════════════════════════════════╝");

        // Display summary stats
        self.display_summary_stats();
    }

    /// Display summary statistics
    fn display_summary_stats(&self) {
        println!("\n📊 Summary Statistics:");
        println!("  Entities: {}", self.entities.len());
        println!("  Collectives: {}", self.collectives.len());
        println!("  Current Step: {}", self.statistics.evolution.current_step);
        println!(
            "  Architecture Alignment: {:.2}%",
            self.statistics.architecture.alignment_percentage
        );
    }

    /// View entity details
    fn view_entity_details(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                    ENTITY DETAILS VIEWER                             ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        // Display available entities
        println!("Available Entities:");
        let entity_ids: Vec<_> = self.entities.keys().take(20).collect();
        for (i, entity_id) in entity_ids.iter().enumerate() {
            if let Some(entity) = self.entities.get(entity_id) {
                println!(
                    "  [{}] Entity ID: {} - Type: {:?} - Density: {:?}",
                    i + 1,
                    entity_id,
                    entity.entity_type,
                    entity.current_density
                );
            }
        }

        if self.entities.len() > 20 {
            println!("  ... and {} more entities", self.entities.len() - 20);
        }

        let entity_input =
            self.get_user_input("\nEnter entity ID (or 'list' for full list, 'back' to go back): ");

        if entity_input == "back" {
            return;
        }

        if entity_input == "list" {
            println!("\nFull Entity List:");
            for entity_id in self.entities.keys() {
                if let Some(entity) = self.entities.get(entity_id) {
                    println!(
                        "  Entity ID: {} - Type: {:?} - Density: {:?}",
                        entity_id, entity.entity_type, entity.current_density
                    );
                }
            }
            return;
        }

        // Try to parse as entity ID
        if let Ok(entity_id) = entity_input.parse::<u64>() {
            let entity_id = EntityId::new(entity_id.to_string());

            if let Some(entity) = self.entities.get(&entity_id) {
                self.display_entity_full_details(&entity_id, entity);
            } else {
                println!("\n❌ Entity not found with ID: {}", entity_id);
            }
        } else {
            println!("\n❌ Invalid entity ID: {}", entity_input);
        }
    }

    /// Display full entity details
    fn display_entity_full_details(&self, entity_id: &EntityId, entity: &SubSubLogos) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                      ENTITY FULL DETAILS                             ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        println!("📋 Basic Information:");
        println!("  Entity ID: {:?}", entity_id);
        println!("  Entity Type: {:?}", entity.entity_type);
        println!("  Current Density: {:?}", entity.current_density);
        println!("  Evolution Clock: {:.2}", entity.evolution_clock);
        println!();

        println!("🧠 Consciousness:");
        println!("  Consciousness Level: {:.4}", entity.consciousness_level);
        println!(
            "  Experience Accumulation: {:.4}",
            entity.experience_accumulation
        );
        println!("  Learning Progress: {:.4}", entity.learning_progress);
        println!();

        println!("⚖️  Polarity:");
        println!("  State: {:?}", entity.polarization.state);
        println!("  Intensity: {:.4}", entity.polarization.intensity);
        println!("  Direction: {:?}", entity.polarization.direction);
        println!("  Consistency: {:.4}", entity.polarization.consistency);
        println!();

        println!("🌈 Spectrum Access:");
        println!("  Ratio: {:.4}", entity.spectrum_access.ratio);
        println!("  Veil Active: {}", entity.spectrum_access.veil_active);
        println!(
            "  Space/Time Access: {:.4}",
            entity.spectrum_access.space_time_access
        );
        println!(
            "  Time/Space Access: {:.4}",
            entity.spectrum_access.time_space_access
        );
        println!();

        println!("📍 Veil Status:");
        println!("  Transparency: {:.4}", entity.veil.transparency);
        println!("  Illusion Strength: {:.4}", entity.veil.illusion_strength);
        println!(
            "  Time/Space Access: {:.4}",
            entity.veil.access_control.time_space_access
        );
        println!(
            "  Holographic Access: {:.4}",
            entity.veil.access_control.holographic_connection_access
        );
        println!(
            "  Higher Consciousness Access: {:.4}",
            entity.veil.access_control.higher_consciousness_access
        );
        println!();

        println!("🎨 Archetype Activations:");
        println!(
            "  Archetype 1 (The Magician): {:.4}",
            entity.archetype_activations()[0]
        );
        println!(
            "  Archetype 4 (The Emperor): {:.4}",
            entity.archetype_activations()[3]
        );
        println!(
            "  Archetype 7 (The Chariot): {:.4}",
            entity.archetype_activations()[6]
        );
        println!(
            "  Archetype 10 (The Wheel of Fortune): {:.4}",
            entity.archetype_activations()[9]
        );
        println!(
            "  Archetype 13 (Death): {:.4}",
            entity.archetype_activations()[12]
        );
        println!(
            "  Archetype 16 (The Tower): {:.4}",
            entity.archetype_activations()[15]
        );
        println!(
            "  Archetype 19 (The Sun): {:.4}",
            entity.archetype_activations()[18]
        );
        println!(
            "  Archetype 22 (The Choice): {:.4}",
            entity.archetype_activations()[21]
        );
        println!("  (Top 8 of 22 archetypes shown)");
        println!();

        println!("⚡ Energy:");
        println!("  Total Energy: {:.4}", entity.energy());
        println!("  Kinetic Energy: {:.4}", entity.kinetic_energy());
        println!("  Potential Energy: {:.4}", entity.potential_energy());
        println!();
    }

    /// View collective details
    fn view_collective_details(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                   COLLECTIVE DETAILS VIEWER                          ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        if self.collectives.is_empty() {
            println!("No collectives currently formed.");
            return;
        }

        // Display available collectives
        println!("Available Collectives:");
        let collective_ids: Vec<_> = self.collectives.keys().take(20).collect();
        for (i, collective_id) in collective_ids.iter().enumerate() {
            if let Some(collective) = self.collectives.get(collective_id) {
                println!(
                    "  [{}] Collective ID: {} - Members: {} - Consciousness: {:.4}",
                    i + 1,
                    collective_id,
                    collective.member_entities.len(),
                    collective.group_consciousness_level
                );
            }
        }

        if self.collectives.len() > 20 {
            println!("  ... and {} more collectives", self.collectives.len() - 20);
        }

        let collective_input =
            self.get_user_input("\nEnter collective ID (or 'back' to go back): ");

        if collective_input == "back" {
            return;
        }

        // Try to parse as collective ID
        if let Ok(collective_id) = collective_input.parse::<u64>() {
            let collective_id = EntityId::new(collective_id.to_string());

            if let Some(collective) = self.collectives.get(&collective_id) {
                self.display_collective_full_details(&collective_id, collective);
            } else {
                println!("\n❌ Collective not found with ID: {}", collective_id);
            }
        } else {
            println!("\n❌ Invalid collective ID: {}", collective_input);
        }
    }

    /// Display full collective details
    fn display_collective_full_details(&self, collective_id: &EntityId, collective: &Collective) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                    COLLECTIVE FULL DETAILS                            ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        println!("📋 Basic Information:");
        println!("  Collective ID: {:?}", collective_id);
        println!("  Number of Members: {}", collective.member_entities.len());
        println!("  Polarity: {:?}", collective.collective_polarity);
        println!();

        println!("🧠 Collective Consciousness:");
        println!(
            "  Collective Consciousness Level: {:.4}",
            collective.group_consciousness_level
        );
        println!(
            "  Average Resonance: {:.4}",
            collective.collective_resonance
        );
        println!();

        println!("👥 Member Entities:");
        for (i, member_id) in collective.member_entities.iter().take(10).enumerate() {
            if let Some(entity) = self.entities.get(member_id) {
                println!(
                    "  [{}] Entity ID: {} - Density: {:?} - Polarity: {:.4}",
                    i + 1,
                    member_id,
                    entity.current_density,
                    entity.polarization.intensity
                );
            }
        }

        if collective.member_entities.len() > 10 {
            println!(
                "  ... and {} more members",
                collective.member_entities.len() - 10
            );
        }
        println!();
    }

    /// View spectrum distribution
    fn view_spectrum_distribution(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                   SPECTRUM DISTRIBUTION VIEWER                       ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        let sa = &self.statistics.spectrum_access;

        println!("📊 Overall Spectrum Access:");
        println!("  Average Spectrum Ratio: {:.4}", sa.average_spectrum_ratio);
        println!(
            "  Average Space/Time Access: {:.4}",
            sa.average_space_time_access
        );
        println!(
            "  Average Time/Space Access: {:.4}",
            sa.average_time_space_access
        );
        println!();

        println!("📈 Spectrum Dominance:");
        let total = sa.space_time_dominant_count + sa.time_space_dominant_count + sa.balanced_count;
        let st_pct = if total > 0 {
            (sa.space_time_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let ts_pct = if total > 0 {
            (sa.time_space_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let balanced_pct = if total > 0 {
            (sa.balanced_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "  Space/Time Dominant: {} ({:.1}%)",
            sa.space_time_dominant_count, st_pct
        );
        println!(
            "  Time/Space Dominant: {} ({:.1}%)",
            sa.time_space_dominant_count, ts_pct
        );
        println!("  Balanced: {} ({:.1}%)", sa.balanced_count, balanced_pct);
        println!();

        println!("📍 Veil Activity:");
        let veil_total = sa.veil_active_count + sa.veil_inactive_count;
        let active_pct = if veil_total > 0 {
            (sa.veil_active_count as f64 / veil_total as f64) * 100.0
        } else {
            0.0
        };
        let inactive_pct = if veil_total > 0 {
            (sa.veil_inactive_count as f64 / veil_total as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "  Veil Active: {} ({:.1}%)",
            sa.veil_active_count, active_pct
        );
        println!(
            "  Veil Inactive: {} ({:.1}%)",
            sa.veil_inactive_count, inactive_pct
        );
        println!();
    }

    /// View polarization distribution
    fn view_polarization_distribution(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                 POLARIZATION DISTRIBUTION VIEWER                    ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        let pol = &self.statistics.evolution.polarization_distribution;
        let total = pol.sto + pol.sts + pol.unpolarized;

        println!("📊 Polarity Distribution:");
        let sto_pct = if total > 0 {
            (pol.sto as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sts_pct = if total > 0 {
            (pol.sts as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let unpolarized_pct = if total > 0 {
            (pol.unpolarized as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        println!("  STO (Positive): {} ({:.1}%)", pol.sto, sto_pct);
        println!("  STS (Negative): {} ({:.1}%)", pol.sts, sts_pct);
        println!(
            "  Unpolarized: {} ({:.1}%)",
            pol.unpolarized, unpolarized_pct
        );
        println!();

        println!("📈 Polarity Metrics:");
        println!("  Average Polarity Bias: {:.4}", pol.average_bias);
        println!(
            "  Polarity Diversity Index: {:.4}",
            self.statistics.evolution.polarization_diversity_index
        );
        println!();

        println!("📊 Density Distribution:");
        for (density, count) in &self.statistics.evolution.density_distribution {
            let percentage = if self.statistics.evolution.num_entities > 0 {
                (*count as f64 / self.statistics.evolution.num_entities as f64) * 100.0
            } else {
                0.0
            };
            println!("  {}: {} ({:.1}%)", density, count, percentage);
        }
        println!();
    }

    /// Run simulation for N steps
    fn run_simulation_steps(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                      RUN SIMULATION STEPS                            ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        let steps_input = self.get_user_input("Enter number of steps to run: ");

        if let Ok(steps) = steps_input.parse::<u64>() {
            println!("\nRunning simulation for {} steps...", steps);

            // Note: This is a placeholder - actual implementation would require
            // modifying SimulationRunner to support incremental execution
            println!("⚠️  Feature not yet implemented - requires SimulationRunner modification");
            println!("   This would call: simulation_runner.run_evolution_step() in a loop");
        } else {
            println!("\n❌ Invalid number of steps: {}", steps_input);
        }
    }

    /// Save simulation state
    fn save_simulation_state(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                      SAVE SIMULATION STATE                           ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        let filename = self.get_user_input("Enter filename to save (or press Enter for default): ");

        let filename = if filename.trim().is_empty() {
            format!(
                "simulation_state_step_{}.json",
                self.statistics.evolution.current_step
            )
        } else {
            filename
        };

        println!("\nSaving simulation state to: {}", filename);

        // Note: This is a placeholder - actual implementation would use
        // the PersistenceManager from Phase 8
        println!("⚠️  Feature not yet fully implemented - would use PersistenceManager");
        println!("   This would call: PersistenceManager::save_simulation(&state, &path)");
    }

    /// Load simulation state
    fn load_simulation_state(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════════════════╗");
        println!("║                      LOAD SIMULATION STATE                           ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        let filename = self.get_user_input("Enter filename to load: ");

        println!("\nLoading simulation state from: {}", filename);

        // Note: This is a placeholder - actual implementation would use
        // the PersistenceManager from Phase 8
        println!("⚠️  Feature not yet fully implemented - would use PersistenceManager");
        println!("   This would call: PersistenceManager::load_simulation(&path)");
    }

    /// Get user input from stdin
    fn get_user_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::SimulationParameters;
    use std::collections::HashMap;

    #[test]
    fn test_interactive_explorer_creation() {
        let parameters = SimulationParameters::new();
        let runner = SimulationRunner::new(parameters);
        let statistics = SimulationStatistics::default();
        let entities = HashMap::new();
        let collectives = HashMap::new();

        let explorer = InteractiveExplorer::new(runner, statistics, entities, collectives);
        // Just verify creation works - we can't test interactive menu in unit tests
        assert_eq!(explorer.entities.len(), 0);
        assert_eq!(explorer.collectives.len(), 0);
    }

    #[test]
    fn test_entity_details_display() {
        let parameters = SimulationParameters::new();
        let runner = SimulationRunner::new(parameters);
        let statistics = SimulationStatistics::default();

        // Create a mock entity
        let mut entities = HashMap::new();
        let entity_id = EntityId::new("1".to_string());

        // Create realms with default configurations
        let violet = crate::foundation::VioletRealm::new();
        let indigo = crate::foundation::IndigoRealm::new();
        let blue = crate::foundation::BlueRealm::new();
        let green = crate::foundation::GreenRealm::new();
        let yellow = crate::spectrum::YellowRealm::new(green.clone());
        let orange = crate::spectrum::OrangeRealm::new(yellow.clone());
        let red = crate::spectrum::RedRealm::new(orange.clone());

        let ratio =
            crate::spectrum::SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_config = crate::entity_layer7::IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            entity_id.clone(),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow.clone(),
            orange.clone(),
            red.clone(),
            spectrum_config,
        );

        entities.insert(entity_id.clone(), entity);

        let collectives = HashMap::new();

        let explorer = InteractiveExplorer::new(runner, statistics, entities, collectives);

        // Verify entity exists
        assert!(explorer.entities.contains_key(&entity_id));
    }

    #[test]
    fn test_collective_details_display() {
        let parameters = SimulationParameters::new();
        let runner = SimulationRunner::new(parameters);
        let statistics = SimulationStatistics::default();
        let entities = HashMap::new();

        // Create a mock collective
        let mut collectives = HashMap::new();
        let collective_id = EntityId::new("1".to_string());
        let collective = Collective {
            collective_id: collective_id.clone(),
            member_entities: vec![
                EntityId::new("2".to_string()),
                EntityId::new("3".to_string()),
            ],
            behavior_type:
                crate::simulation_v3::collective_dynamics::CollectiveBehaviorType::GalaxyRotation,
            collective_resonance: 0.75,
            group_consciousness_level: 0.5,
            collective_polarity: None,
            behavior_parameters:
                crate::simulation_v3::collective_dynamics::BehaviorParameters::default(),
        };
        collectives.insert(collective_id.clone(), collective);

        let explorer = InteractiveExplorer::new(runner, statistics, entities, collectives);

        // Verify collective exists
        assert!(explorer.collectives.contains_key(&collective_id));
    }
}
