// Simulation Reporter Module (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md:
// "Implement reporting system for NEW simulation"
//
// This module implements:
// 1. SimulationReporter - main reporter for generating reports
// 2. Report types - summary, detailed, JSON, CSV
// 3. Visualization helpers - ASCII art tables, progress bars

use crate::simulation_v3::statistics::SimulationStatistics;
use crate::types::Float;
use std::io::Write;

// ============================================================================
// REPORTER
// ============================================================================

/// Simulation Reporter
///
/// Generates various types of reports from simulation statistics.
pub struct SimulationReporter {
    /// Statistics to report on
    statistics: SimulationStatistics,
}

impl SimulationReporter {
    /// Create a new reporter with given statistics
    pub fn new(statistics: SimulationStatistics) -> Self {
        SimulationReporter { statistics }
    }

    /// Generate a summary report
    pub fn generate_summary_report(&self) -> String {
        let mut report = String::new();

        report.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║          HOLOGRAPHIC ARCHITECTURE SIMULATION REPORT           ║\n");
        report.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        // Overview
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("OVERVIEW\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Total Execution Time: {:?}\n",
            self.statistics.performance.total_time
        ));
        report.push_str(&format!(
            "Total Simulation Steps: {}\n",
            self.statistics.evolution.total_steps
        ));
        report.push_str(&format!(
            "Total Entities: {}\n",
            self.statistics.evolution.num_entities
        ));
        report.push_str(&format!(
            "Architecture Alignment: {:.2}%\n",
            self.statistics.architecture.alignment_score * 100.0
        ));
        report.push('\n');

        // Involution Phase
        report.push_str(&self.generate_involution_report());

        // Evolution Phase
        report.push_str(&self.generate_evolution_report());

        // Holographic Field
        report.push_str(&self.generate_holographic_report());

        // Physical Manifestation
        report.push_str(&self.generate_physical_report());

        // Performance
        report.push_str(&self.generate_performance_report());

        // Architecture Validation
        report.push_str(&self.generate_architecture_report());

        // Phase 2: Individual Variation
        report.push_str(&self.generate_individual_variation_report());

        // Phase 7: Consciousness-to-Matter
        report.push_str(&self.generate_consciousness_to_matter_report());

        report
    }

    /// Generate involution phase report
    fn generate_involution_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("INVOLUTION PHASE (Creation Sequence)\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Execution Time: {:?}\n",
            self.statistics.involution.execution_time
        ));
        report.push_str(&format!(
            "Entities Created: {}\n",
            self.statistics.involution.entities_created
        ));
        report.push_str(&format!(
            "Attractor Fields Created: {}\n",
            self.statistics.involution.attractor_fields_created
        ));
        report.push_str(&format!(
            "Stage Transitions: {}\n",
            self.statistics.involution.stage_transitions
        ));
        report.push('\n');

        // Stage details table
        report.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        report.push_str("│ Stage Details                                                  │\n");
        report.push_str("├──────┬──────────────┬────────┬──────────┬─────────┬─────────────┤\n");
        report.push_str("│ #    │ Name         │ Success │ Time(ms) │ Entities │ Attractor   │\n");
        report.push_str("├──────┼──────────────┼────────┼──────────┼─────────┼─────────────┤\n");

        for stage in &self.statistics.involution.stage_details {
            report.push_str(&format!(
                "│ {:>4} │ {:<12} │ {:>6} │ {:>8} │ {:>7} │ {:>11} │\n",
                stage.stage_number,
                stage.stage_name,
                if stage.success { "✓" } else { "✗" },
                stage.duration.as_millis(),
                stage.entities_created,
                stage.attractor_fields_created
            ));
        }

        report.push_str("└──────┴──────────────┴────────┴──────────┴─────────┴─────────────┘\n\n");

        report
    }

    /// Generate evolution phase report
    fn generate_evolution_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("EVOLUTION PHASE (Density Octave)\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Execution Time: {:?}\n",
            self.statistics.evolution.execution_time
        ));
        report.push_str(&format!(
            "Current Step: {} / {}\n",
            self.statistics.evolution.current_step, self.statistics.evolution.total_steps
        ));
        report.push_str(&format!(
            "Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));
        report.push('\n');

        // Density distribution
        report.push_str("Density Distribution:\n");
        for (density, count) in &self.statistics.evolution.density_distribution {
            let percentage = if self.statistics.evolution.num_entities > 0 {
                (*count as Float / self.statistics.evolution.num_entities as Float) * 100.0
            } else {
                0.0
            };
            report.push_str(&format!(
                "  {:<15}: {:>6} ({:>5.1}%)\n",
                density, count, percentage
            ));
        }
        report.push('\n');

        // Polarity distribution
        report.push_str(&self.generate_polarization_report());
        report.push('\n');

        report
    }

    /// Generate polarization distribution report
    fn generate_polarization_report(&self) -> String {
        let mut report = String::new();

        report.push_str("Polarity Distribution:\n");

        let pol = &self.statistics.evolution.polarization_distribution;
        let total = pol.sto + pol.sts + pol.unpolarized;

        let sto_pct = if total > 0 {
            (pol.sto as Float / total as Float) * 100.0
        } else {
            0.0
        };
        let sts_pct = if total > 0 {
            (pol.sts as Float / total as Float) * 100.0
        } else {
            0.0
        };
        let unpolarized_pct = if total > 0 {
            (pol.unpolarized as Float / total as Float) * 100.0
        } else {
            0.0
        };

        report.push_str(&format!(
            "  {:<15}: {:>6} ({:>5.1}%) {}\n",
            "STO (Positive)",
            pol.sto,
            sto_pct,
            Self::progress_bar(sto_pct)
        ));
        report.push_str(&format!(
            "  {:<15}: {:>6} ({:>5.1}%) {}\n",
            "STS (Negative)",
            pol.sts,
            sts_pct,
            Self::progress_bar(sts_pct)
        ));
        report.push_str(&format!(
            "  {:<15}: {:>6} ({:>5.1}%) {}\n",
            "Unpolarized",
            pol.unpolarized,
            unpolarized_pct,
            Self::progress_bar(unpolarized_pct)
        ));
        report.push_str(&format!(
            "  {:<15}: {:>6.4}\n",
            "Average Bias", pol.average_bias
        ));

        report
    }

    /// Generate holographic field report
    fn generate_holographic_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("HOLOGRAPHIC FIELD\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Total Connections: {}\n",
            self.statistics.holographic.connection_count
        ));
        report.push_str(&format!(
            "Global Phase Coherence: {:.4}\n",
            self.statistics.holographic.global_phase_coherence
        ));
        report.push_str(&format!(
            "Average Connection Strength: {:.4}\n",
            self.statistics.holographic.average_connection_strength
        ));
        report.push_str(&format!(
            "Resonance Clusters: {}\n",
            self.statistics.holographic.resonance_cluster_count
        ));
        report.push_str(&format!(
            "Resonant Connections: {}\n",
            self.statistics.holographic.resonant_connections
        ));
        report.push_str(&format!(
            "Entangled Connections: {}\n",
            self.statistics.holographic.entangled_connections
        ));
        report.push('\n');

        // Note: Connection type distribution is not available in HolographicFieldStatistics
        report.push_str("(Connection type distribution requires additional tracking)\n");
        report.push('\n');

        report
    }

    /// Generate physical manifestation report
    fn generate_physical_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("PHYSICAL MANIFESTATION\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Physical Entities: {}\n",
            self.statistics.physical.num_physical_entities
        ));
        report.push_str(&format!(
            "Total Energy: {:.2e} J\n",
            self.statistics.physical.total_energy
        ));
        report.push_str(&format!(
            "Average Kinetic Energy: {:.2e} J\n",
            self.statistics.physical.average_kinetic_energy
        ));
        report.push('\n');

        // Scale distribution
        report.push_str("Physical Scale Distribution:\n");
        for (scale, count) in &self.statistics.physical.scale_distribution {
            report.push_str(&format!("  {:<15}: {}\n", scale, count));
        }
        report.push('\n');

        // Energy level distribution
        report.push_str("Energy Level Distribution:\n");
        for (energy_level, count) in &self.statistics.physical.energy_level_distribution {
            report.push_str(&format!("  {:<15}: {}\n", energy_level, count));
        }
        report.push('\n');

        report
    }

    /// Generate performance report
    fn generate_performance_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("PERFORMANCE METRICS\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Total Time: {:?}\n",
            self.statistics.performance.total_time
        ));
        report.push_str(&format!(
            "Time per Step: {:?}\n",
            self.statistics.performance.time_per_step
        ));
        report.push_str(&format!(
            "Steps per Second: {:.2}\n",
            self.statistics.performance.steps_per_second
        ));
        report.push_str(&format!(
            "Entities per Second: {:.2}\n",
            self.statistics.performance.entities_per_second
        ));
        report.push_str(&format!(
            "Peak Memory Usage: {} MB\n",
            self.statistics.performance.peak_memory_mb
        ));
        report.push('\n');

        report
    }

    /// Generate architecture validation report
    fn generate_architecture_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("ARCHITECTURE VALIDATION\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Overall Alignment Score: {:.2}%\n",
            self.statistics.architecture.alignment_score * 100.0
        ));
        report.push_str(&format!(
            "Transcend and Include Stages: {} / {}\n",
            self.statistics.architecture.transcend_include_stages,
            self.statistics.architecture.transcend_include_stages_total
        ));
        report.push('\n');

        report.push_str("Architecture Features:\n");
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Three Primal Distortions",
            Self::checkmark(self.statistics.architecture.three_primal_distortions)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Space/Time Spectrum",
            Self::checkmark(self.statistics.architecture.space_time_spectrum)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Veil Position (v=1)",
            Self::checkmark(self.statistics.architecture.veil_position)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Logos Hierarchy",
            Self::checkmark(self.statistics.architecture.logos_hierarchy)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Density Octave",
            Self::checkmark(self.statistics.architecture.density_octave)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Holographic Principle",
            Self::checkmark(self.statistics.architecture.holographic_principle)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Environmental Interaction (Phase 4)",
            Self::checkmark(self.statistics.architecture.environmental_interaction)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Collective Influence (Phase 4)",
            Self::checkmark(self.statistics.architecture.collective_influence)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Emergent Properties (Phase 4)",
            Self::checkmark(self.statistics.architecture.emergent_properties)
        ));
        report.push_str(&format!(
            "  {:<40} {}\n",
            "Free Will / Polarity Diversity (Phase 3)",
            Self::checkmark(self.statistics.architecture.polarization_diversity)
        ));
        report.push('\n');

        // Phase 4: Add emergent properties display
        if self.statistics.emergent_properties.emergence_score > 0.0 {
            report.push_str("Emergent Properties (Phase 4):\n");
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Global Coherence", self.statistics.emergent_properties.global_coherence
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Collective Consciousness Level",
                self.statistics
                    .emergent_properties
                    .collective_consciousness_level
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "System Entropy", self.statistics.emergent_properties.system_entropy
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Emergence Score", self.statistics.emergent_properties.emergence_score
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Organic Emergence Score",
                self.statistics.emergent_properties.organic_emergence_score
            ));
            report.push_str(&format!(
                "  {:<40} {}\n",
                "Unique Evolutionary Trajectories",
                self.statistics
                    .emergent_properties
                    .unique_evolutionary_trajectories
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Holographic Coherence", self.statistics.emergent_properties.holographic_coherence
            ));
            report.push_str(&format!(
                "  {:<40} {:.4}\n",
                "Environmental Integration",
                self.statistics
                    .emergent_properties
                    .environmental_integration
            ));
            report.push('\n');
        }

        report
    }

    /// Generate individual variation report (Phase 2)
    fn generate_individual_variation_report(&self) -> String {
        use crate::simulation_v3::individual_variation_visualizer::IndividualVariationVisualizer;
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("INDIVIDUAL VARIATION (Phase 2)\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");

        // Use the IndividualVariationVisualizer to generate the report
        let visualizer =
            IndividualVariationVisualizer::new(self.statistics.individual_variation.clone());
        report.push_str(&visualizer.generate_variation_visualization());

        report
    }

    /// Generate a detailed report with more information
    pub fn generate_detailed_report(&self) -> String {
        let mut report = self.generate_summary_report();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("DETAILED ANALYSIS\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("This section contains detailed analysis of the simulation results.\n");
        report.push_str("(To be implemented with more specific metrics)\n\n");

        report
    }

    /// Generate JSON report (requires serde_json dependency)
    pub fn generate_json_report(&self) -> Result<String, String> {
        // Note: This requires serde_json dependency
        // For now, return a placeholder
        Err("JSON report requires serde_json dependency".to_string())
    }

    /// Generate CSV report (polarization distribution)
    pub fn generate_csv_report(&self) -> String {
        let mut csv = String::new();

        csv.push_str("metric,value\n");
        csv.push_str(&format!(
            "total_entities,{}\n",
            self.statistics.evolution.num_entities
        ));
        csv.push_str(&format!(
            "sto_entities,{}\n",
            self.statistics.evolution.polarization_distribution.sto
        ));
        csv.push_str(&format!(
            "sts_entities,{}\n",
            self.statistics.evolution.polarization_distribution.sts
        ));
        csv.push_str(&format!(
            "unpolarized_entities,{}\n",
            self.statistics
                .evolution
                .polarization_distribution
                .unpolarized
        ));
        csv.push_str(&format!(
            "density_transitions,{}\n",
            self.statistics.evolution.density_transitions
        ));
        csv.push_str(&format!(
            "total_connections,{}\n",
            self.statistics.holographic.connection_count
        ));
        csv.push_str(&format!(
            "global_phase_coherence,{:.4}\n",
            self.statistics.holographic.global_phase_coherence
        ));
        csv.push_str(&format!(
            "architecture_alignment,{:.4}\n",
            self.statistics.architecture.alignment_score
        ));

        csv
    }

    /// Helper: generate a progress bar
    fn progress_bar(percentage: Float) -> String {
        let width = 20;
        let filled = (percentage / 100.0 * width as Float).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push('[');
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in filled..width {
            bar.push('░');
        }
        bar.push(']');

        bar
    }

    /// Helper: checkmark or cross
    fn checkmark(value: bool) -> &'static str {
        if value {
            "✓"
        } else {
            "✗"
        }
    }

    /// Save report to file
    pub fn save_report(&self, filename: &str) -> std::io::Result<()> {
        let report = self.generate_summary_report();
        let mut file = std::fs::File::create(filename)?;
        file.write_all(report.as_bytes())?;
        Ok(())
    }

    /// Generate consciousness-to-matter report (Phase 7)
    fn generate_consciousness_to_matter_report(&self) -> String {
        let mut report = String::new();

        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str("PHASE 7: CONSCIOUSNESS-TO-MATTER TRANSITIONS\n");
        report.push_str("══════════════════════════════════════════════════════════════════\n");
        report.push_str(&format!(
            "Total Quantum Pools: {}\n",
            self.statistics.consciousness_to_matter.total_pools
        ));
        report.push_str(&format!(
            "Coherent Pools: {}\n",
            self.statistics.consciousness_to_matter.coherent_pools
        ));
        report.push_str(&format!(
            "Decohered Pools: {}\n",
            self.statistics.consciousness_to_matter.decohered_pools
        ));
        report.push_str(&format!(
            "Active Transitions: {}\n",
            self.statistics.consciousness_to_matter.active_transitions
        ));
        report.push_str(&format!(
            "Completed Transitions: {}\n",
            self.statistics
                .consciousness_to_matter
                .completed_transitions
        ));
        report.push_str(&format!(
            "Failed Transitions: {}\n",
            self.statistics.consciousness_to_matter.failed_transitions
        ));
        report.push_str(&format!(
            "Total Information Content: {:.2} bits\n",
            self.statistics
                .consciousness_to_matter
                .total_information_content
        ));
        report.push_str(&format!(
            "Average Coherence: {:.4}\n",
            self.statistics.consciousness_to_matter.average_coherence
        ));

        // Calculate success rate
        let success_rate = if self.statistics.consciousness_to_matter.total_pools > 0 {
            (self
                .statistics
                .consciousness_to_matter
                .completed_transitions as Float
                / self.statistics.consciousness_to_matter.total_pools as Float)
                * 100.0
        } else {
            0.0
        };
        report.push_str(&format!("Success Rate: {:.2}%\n", success_rate));

        report.push('\n');

        report
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::statistics::SimulationStatistics;

    #[test]
    fn test_reporter_creation() {
        let stats = SimulationStatistics::default();
        let reporter = SimulationReporter::new(stats);
        // Just check it doesn't panic
        assert_eq!(reporter.statistics.involution.entities_created, 0);
    }

    #[test]
    fn test_generate_summary_report() {
        let mut stats = SimulationStatistics::default();
        stats.evolution.num_entities = 100;
        stats.evolution.total_steps = 200;
        stats.evolution.current_step = 100;

        let reporter = SimulationReporter::new(stats);
        let report = reporter.generate_summary_report();

        assert!(report.contains("HOLOGRAPHIC ARCHITECTURE SIMULATION REPORT"));
        assert!(report.contains("Total Entities: 100"));
        assert!(report.contains("Current Step: 100"));
    }

    #[test]
    fn test_progress_bar() {
        let bar = SimulationReporter::progress_bar(50.0);
        assert!(bar.contains('['));
        assert!(bar.contains(']'));
        assert!(bar.contains('█') || bar.contains('░'));
    }

    #[test]
    fn test_checkmark() {
        assert_eq!(SimulationReporter::checkmark(true), "✓");
        assert_eq!(SimulationReporter::checkmark(false), "✗");
    }

    #[test]
    fn test_generate_csv_report() {
        let stats = SimulationStatistics::default();
        let reporter = SimulationReporter::new(stats);
        let csv = reporter.generate_csv_report();

        assert!(csv.contains("metric,value"));
        assert!(csv.contains("total_entities"));
    }

    #[test]
    fn test_generate_polarization_report() {
        let mut stats = SimulationStatistics::default();
        stats.evolution.polarization_distribution.sto = 50;
        stats.evolution.polarization_distribution.sts = 30;
        stats.evolution.polarization_distribution.unpolarized = 20;

        let reporter = SimulationReporter::new(stats);
        let report = reporter.generate_polarization_report();

        assert!(report.contains("Polarity Distribution"));
        assert!(report.contains("STO (Positive)"));
        assert!(report.contains("STS (Negative)"));
    }
}
