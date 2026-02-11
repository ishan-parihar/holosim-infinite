// Holonic Realms Simulation - Main Entry Point
// Runs the complete simulation and observes emergent behavior
//
// This is the main entry point for running the Holonic Realms Simulation.
// It initializes the simulation, runs it for a specified number of steps,
// and reports on the emergent behavior and characteristics.

use holonic_realms::complete_simulation::{CompleteSimulation, SimulationResult};
use holonic_realms::organic_reality_generator::{OrganicReality, OrganicRealityGenerator};
use std::io::{self, Write};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║              HOLONIC REALMS SIMULATION                              ║");
    println!("║       Law of One Cosmology with 8-Layer Fractal Architecture         ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!();

    // Get simulation parameters
    let num_steps = get_simulation_steps();
    let show_details = get_detail_level();

    println!("════════════════════════════════════════════════════════════════════");
    println!("INITIALIZING SIMULATION");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    // Create initial organic reality
    println!("Creating initial organic reality...");
    let reality = OrganicRealityGenerator::new().generate_reality();
    let characteristics = reality.get_characteristics();

    println!("Reality Characteristics:");
    println!("  - Speed of Light: {:.2}", characteristics.speed_of_light);
    println!(
        "  - Gravitational Constant: {:.6}",
        characteristics.gravitational_constant
    );
    println!(
        "  - Spatial Dimensions: {}",
        characteristics.spatial_dimensions
    );
    println!(
        "  - Temporal Dimensions: {}",
        characteristics.temporal_dimensions
    );
    println!(
        "  - Quantum Mechanics: {}",
        if characteristics.quantum_mechanics_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!("  - Veil Thickness: {:.2}", characteristics.veil_thickness);
    println!(
        "  - Free Will Capacity: {:.2}",
        characteristics.free_will_capacity
    );
    println!(
        "  - Holographic Coherence: {:.2}",
        characteristics.holographic_coherence
    );
    println!("  - Scale Count: {}", characteristics.scale_count);
    println!();

    // Initialize simulation
    println!("Initializing complete simulation...");
    let mut simulation = CompleteSimulation::from_organic_reality(reality);

    println!("Simulation State:");
    let state = simulation.get_state();
    println!("  - Space-Time Energy: {:.2}", state.space_time_energy);
    println!("  - Time-Space Energy: {:.2}", state.time_space_energy);
    println!("  - Balance: {:.2}", state.balance);
    println!(
        "  - Coherence: {:.2}",
        simulation.dual_dimensional_integration.coherence
    );
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("RUNNING SIMULATION");
    println!("════════════════════════════════════════════════════════════════════");
    println!("Steps: {}", num_steps);
    println!();

    // Run simulation
    let result = simulation.run_simulation(num_steps);

    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("SIMULATION COMPLETE");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    // Display results
    display_simulation_results(&result, show_details);

    // Display emergent behavior analysis
    analyze_emergent_behavior(&result);

    // Display final state
    display_final_state(&result);

    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("SIMULATION SUMMARY");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    display_summary(&result);
}

fn get_simulation_steps() -> usize {
    println!("Enter number of simulation steps (default: 100):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap_or(100)
}

fn get_detail_level() -> bool {
    println!("Show detailed output? (y/n, default: n):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}

fn display_simulation_results(result: &SimulationResult, show_details: bool) {
    println!("SIMULATION RESULTS");
    println!("─────────────────────────────────────────────────────────────────────");
    println!();

    // Statistics
    println!("Statistics:");
    println!("  Total Steps: {}", result.statistics.total_steps);
    println!("  Total Choices: {}", result.statistics.total_choices);
    println!(
        "  Energy Conserved: {}",
        if result.statistics.energy_conserved {
            "✓ Yes"
        } else {
            "✗ No"
        }
    );
    println!(
        "  Average Coherence: {:.4}",
        result.statistics.average_coherence
    );
    println!(
        "  Minimum Coherence: {:.4}",
        result.statistics.minimum_coherence
    );
    println!(
        "  Maximum Coherence: {:.4}",
        result.statistics.maximum_coherence
    );
    println!();

    // Polarity Distribution
    println!("Polarity Distribution:");
    println!(
        "  STO (Service-to-Others): {} ({:.1}%)",
        result.polarization_distribution.sto_count,
        result.polarization_distribution.sto_percentage * 100.0
    );
    println!(
        "  STS (Service-to-Self): {} ({:.1}%)",
        result.polarization_distribution.sts_count,
        result.polarization_distribution.sts_percentage * 100.0
    );
    println!(
        "  Neutral: {}",
        result.polarization_distribution.neutral_count
    );
    println!();

    // Entity Evolution
    if show_details {
        println!("Entity Evolution:");
        println!(
            "  Total Entities: {}",
            result.entity_evolution.total_entities
        );
        println!(
            "  Average Developmental Level: {:.2}",
            result.entity_evolution.average_developmental_level
        );
        println!(
            "  Evolution Paths Tracked: {}",
            result.entity_evolution.evolution_paths.len()
        );
        println!();

        // Show some evolution paths
        if !result.entity_evolution.evolution_paths.is_empty() {
            println!("Sample Evolution Paths (first 3):");
            let mut count = 0;
            for (entity_id, steps) in &result.entity_evolution.evolution_paths {
                if count >= 3 {
                    break;
                }
                println!("  Entity {}:", entity_id);
                for step in steps.iter().take(5) {
                    println!(
                        "    Step {}: {:?}, XP: {:.2}, Progress: {:.2}%",
                        step.step,
                        step.choice,
                        step.experience,
                        step.developmental_progress * 100.0
                    );
                }
                if steps.len() > 5 {
                    println!("    ... and {} more steps", steps.len() - 5);
                }
                println!();
                count += 1;
            }
        }
    }

    // Final State
    println!("Final Integration State:");
    println!(
        "  Space-Time Energy: {:.2}",
        result.final_state.space_time_energy
    );
    println!(
        "  Time-Space Energy: {:.2}",
        result.final_state.time_space_energy
    );
    println!("  Balance: {:.2}", result.final_state.balance);
    println!();

    // Universe Characteristics
    println!("Final Universe Characteristics:");
    println!(
        "  Speed of Light: {:.2}",
        result.universe_characteristics.speed_of_light
    );
    println!(
        "  Gravitational Constant: {:.6}",
        result.universe_characteristics.gravitational_constant
    );
    println!(
        "  Spatial Dimensions: {}",
        result.universe_characteristics.spatial_dimensions
    );
    println!(
        "  Temporal Dimensions: {}",
        result.universe_characteristics.temporal_dimensions
    );
    println!(
        "  Quantum Mechanics: {}",
        if result.universe_characteristics.quantum_mechanics_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!(
        "  Veil Thickness: {:.2}",
        result.universe_characteristics.veil_thickness
    );
    println!(
        "  Free Will Capacity: {:.2}",
        result.universe_characteristics.free_will_capacity
    );
    println!(
        "  Holographic Coherence: {:.2}",
        result.universe_characteristics.holographic_coherence
    );
    println!(
        "  Scale Count: {}",
        result.universe_characteristics.scale_count
    );
    println!();
}

fn analyze_emergent_behavior(result: &SimulationResult) {
    println!("════════════════════════════════════════════════════════════════════");
    println!("EMERGENT BEHAVIOR ANALYSIS");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    // Coherence Analysis
    println!("Coherence Evolution:");
    let coherence_range = result.statistics.maximum_coherence - result.statistics.minimum_coherence;
    if coherence_range < 0.1 {
        println!("  Status: STABLE");
        println!("  Coherence remained very stable throughout simulation");
    } else if coherence_range < 0.3 {
        println!("  Status: MODERATELY STABLE");
        println!("  Coherence fluctuated within normal range");
    } else {
        println!("  Status: UNSTABLE");
        println!("  Coherence experienced significant fluctuations");
    }
    println!(
        "  Range: {:.4} ({:.4} - {:.4})",
        coherence_range, result.statistics.minimum_coherence, result.statistics.maximum_coherence
    );
    println!();

    // Polarity Balance Analysis
    println!("Polarity Balance:");
    let polarity_balance = (result.polarization_distribution.sto_percentage
        - result.polarization_distribution.sts_percentage)
        .abs();
    if polarity_balance < 0.1 {
        println!("  Status: BALANCED");
        println!("  STO and STS polarities are well-balanced");
    } else if polarity_balance < 0.3 {
        println!("  Status: MODERATELY BALANCED");
        println!("  Slight imbalance between polarities");
    } else {
        println!("  Status: IMBALANCED");
        println!("  Significant imbalance between polarities");
    }
    println!(
        "  Balance: {:.2} (STO: {:.1}%, STS: {:.1}%)",
        polarity_balance,
        result.polarization_distribution.sto_percentage * 100.0,
        result.polarization_distribution.sts_percentage * 100.0
    );
    println!();

    // Energy Conservation Analysis
    println!("Energy Conservation:");
    if result.statistics.energy_conserved {
        println!("  Status: CONSERVED ✓");
        println!("  Energy was properly conserved throughout simulation");
    } else {
        println!("  Status: NOT CONSERVED ✗");
        println!("  Energy conservation violation detected");
    }
    println!();

    // Emergent Patterns
    println!("Emergent Patterns:");
    let patterns = identify_emergent_patterns(result);
    if patterns.is_empty() {
        println!("  No clear emergent patterns detected");
    } else {
        for pattern in patterns {
            println!("  • {}", pattern);
        }
    }
    println!();

    // System Health
    println!("System Health Assessment:");
    let health_score = calculate_health_score(result);
    println!("  Overall Health Score: {:.1}/100", health_score);

    if health_score >= 80.0 {
        println!("  Status: EXCELLENT - System functioning optimally");
    } else if health_score >= 60.0 {
        println!("  Status: GOOD - System functioning well with minor issues");
    } else if health_score >= 40.0 {
        println!("  Status: FAIR - System functional but showing stress");
    } else {
        println!("  Status: POOR - System experiencing significant issues");
    }
    println!();
}

fn identify_emergent_patterns(result: &SimulationResult) -> Vec<String> {
    let mut patterns = Vec::new();

    // Check for coherence stability
    if result.statistics.average_coherence > 0.8 {
        patterns.push("High coherence maintained - indicates stable system dynamics".to_string());
    }

    // Check for polarity balance
    let polarity_balance = (result.polarization_distribution.sto_percentage
        - result.polarization_distribution.sts_percentage)
        .abs();
    if polarity_balance < 0.15 {
        patterns.push(
            "Balanced polarity distribution - suggests natural choice distribution".to_string(),
        );
    } else if result.polarization_distribution.sto_percentage > 0.6 {
        patterns.push("STO dominance - entities favoring service to others".to_string());
    } else if result.polarization_distribution.sts_percentage > 0.6 {
        patterns.push("STS dominance - entities favoring service to self".to_string());
    }

    // Check for energy conservation
    if result.statistics.energy_conserved {
        patterns
            .push("Energy conservation maintained - dual-dimensional balance stable".to_string());
    }

    // Check for development
    if result.entity_evolution.average_developmental_level > 0.3 {
        patterns.push("Positive developmental progression - entities evolving".to_string());
    }

    patterns
}

fn calculate_health_score(result: &SimulationResult) -> f64 {
    let mut score = 0.0;

    // Coherence score (40 points)
    score += result.statistics.average_coherence * 40.0;

    // Energy conservation (30 points)
    if result.statistics.energy_conserved {
        score += 30.0;
    }

    // Polarity balance (20 points)
    let polarity_balance = (result.polarization_distribution.sto_percentage
        - result.polarization_distribution.sts_percentage)
        .abs();
    score += (1.0 - polarity_balance) * 20.0;

    // Holographic coherence (10 points)
    score += result.universe_characteristics.holographic_coherence * 10.0;

    score.min(100.0).max(0.0)
}

fn display_final_state(result: &SimulationResult) {
    println!("════════════════════════════════════════════════════════════════════");
    println!("FINAL STATE");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    println!("Dual-Dimensional Integration:");
    println!(
        "  Space-Time Energy: {:.4}",
        result.final_state.space_time_energy
    );
    println!(
        "  Time-Space Energy: {:.4}",
        result.final_state.time_space_energy
    );
    println!("  Net Balance: {:.4}", result.final_state.balance);
    println!();

    println!("Universe Characteristics:");
    println!(
        "  Speed of Light: {:.4}",
        result.universe_characteristics.speed_of_light
    );
    println!(
        "  Gravitational Constant: {:.6}",
        result.universe_characteristics.gravitational_constant
    );
    println!(
        "  Spatial Dimensions: {}",
        result.universe_characteristics.spatial_dimensions
    );
    println!(
        "  Temporal Dimensions: {}",
        result.universe_characteristics.temporal_dimensions
    );
    println!(
        "  Quantum Mechanics: {}",
        if result.universe_characteristics.quantum_mechanics_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!(
        "  Veil Thickness: {:.4}",
        result.universe_characteristics.veil_thickness
    );
    println!(
        "  Free Will Capacity: {:.4}",
        result.universe_characteristics.free_will_capacity
    );
    println!(
        "  Holographic Coherence: {:.4}",
        result.universe_characteristics.holographic_coherence
    );
    println!(
        "  Scale Count: {}",
        result.universe_characteristics.scale_count
    );
    println!();
}

fn display_summary(result: &SimulationResult) {
    println!("Summary:");
    println!("  • Total Steps: {}", result.statistics.total_steps);
    println!(
        "  • Energy Conserved: {}",
        if result.statistics.energy_conserved {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  • Average Coherence: {:.2}%",
        result.statistics.average_coherence * 100.0
    );
    println!(
        "  • STO Polarity: {:.1}%",
        result.polarization_distribution.sto_percentage * 100.0
    );
    println!(
        "  • STS Polarity: {:.1}%",
        result.polarization_distribution.sts_percentage * 100.0
    );
    println!(
        "  • Entities Tracked: {}",
        result.entity_evolution.total_entities
    );
    println!(
        "  • Health Score: {:.1}/100",
        calculate_health_score(result)
    );
    println!();

    println!("Key Insights:");
    let insights = generate_insights(result);
    for insight in insights {
        println!("  • {}", insight);
    }
    println!();

    println!("From COSMOLOGICAL-ARCHITECTURE.md:");
    println!("  \"All is One, and One is All\"");
    println!("  \"The Law of One states that all things are one, that there is no polarity,");
    println!("   no right or wrong, no disharmony, but only identity\"");
    println!("  \"All is, well and good\"");
    println!();
}

fn generate_insights(result: &SimulationResult) -> Vec<String> {
    let mut insights = Vec::new();

    // Coherence insight
    if result.statistics.average_coherence > 0.8 {
        insights.push(
            "System maintained high coherence - indicates strong holographic integration"
                .to_string(),
        );
    } else if result.statistics.average_coherence > 0.5 {
        insights.push("System coherence moderate - room for holographic improvement".to_string());
    } else {
        insights.push("Low coherence detected - holographic integrity concerns".to_string());
    }

    // Polarity insight
    let total =
        result.polarization_distribution.sto_count + result.polarization_distribution.sts_count;
    if total > 0 {
        let sto_ratio = result.polarization_distribution.sto_count as f64 / total as f64;
        if sto_ratio > 0.6 {
            insights.push(
                "STO polarity dominant - entities leaning toward service to others".to_string(),
            );
        } else if sto_ratio < 0.4 {
            insights.push(
                "STS polarity dominant - entities leaning toward service to self".to_string(),
            );
        } else {
            insights.push("Polarity balanced - natural distribution of choices".to_string());
        }
    }

    // Energy insight
    if result.statistics.energy_conserved {
        insights
            .push("Energy conservation maintained - dual-dimensional balance stable".to_string());
    } else {
        insights.push("Energy not conserved - dual-dimensional imbalance detected".to_string());
    }

    // Development insight
    if result.entity_evolution.average_developmental_level > 0.5 {
        insights.push("Strong developmental progression - entities evolving well".to_string());
    } else if result.entity_evolution.average_developmental_level > 0.3 {
        insights.push("Moderate developmental progression - entities evolving".to_string());
    } else {
        insights.push("Limited developmental progression - early stage evolution".to_string());
    }

    insights
}
