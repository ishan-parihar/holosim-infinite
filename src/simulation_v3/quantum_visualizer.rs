// Quantum Visualizer - Phase 7: Consciousness-First Cosmology
//
// This module provides visualization for quantum information structures,
// consciousness-to-matter transitions, and attractor fields.
//
// Visualizations:
// - Quantum pool distribution
// - Decoherence dynamics
// - Attractor field map
// - Consciousness-to-matter transition flow
// - Quantum information metrics

use crate::physical_manifestation::consciousness_to_matter::ConsciousnessToMatterManager;
use crate::simulation_v3::quantum_statistics::{
    AttractorFieldStatistics, ConsciousnessToMatterStatisticsTracker, DecoherenceStatistics,
    QuantumPoolStatistics, TransitionStatistics,
};
use crate::types::Float;

// ============================================================================
// QUANTUM VISUALIZER
// ============================================================================

/// Visualizer for quantum information structures and consciousness-to-matter transitions
pub struct QuantumVisualizer;

impl QuantumVisualizer {
    /// Visualize quantum pool statistics
    ///
    /// # Arguments
    /// * `stats` - Quantum pool statistics
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_quantum_pools(stats: &QuantumPoolStatistics) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║           QUANTUM ENERGY POOL STATISTICS                   ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        // Pool counts
        output.push_str("Pool Distribution:\n");
        output.push_str(&format!("  Total Pools:       {}\n", stats.total_pools));
        output.push_str(&format!(
            "  Coherent Pools:    {} ({:.1}%)\n",
            stats.coherent_pools,
            if stats.total_pools > 0 {
                stats.coherent_pools as Float / stats.total_pools as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push_str(&format!(
            "  Decohered Pools:   {} ({:.1}%)\n",
            stats.decohered_pools,
            if stats.total_pools > 0 {
                stats.decohered_pools as Float / stats.total_pools as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push('\n');

        // Information content
        output.push_str("Information Content:\n");
        output.push_str(&format!(
            "  Total:             {:.2} bits\n",
            stats.total_information_content
        ));
        output.push_str(&format!(
            "  Average:           {:.2} bits\n",
            stats.average_information_content
        ));
        output.push('\n');

        // Energy
        output.push_str("Energy Content:\n");
        output.push_str(&format!(
            "  Total:             {:.2e} J\n",
            stats.total_energy
        ));
        output.push('\n');

        // Coherence
        output.push_str("Coherence Levels:\n");
        output.push_str(&format!(
            "  Average:           {:.3}\n",
            stats.average_coherence
        ));
        output.push('\n');

        // Coherence distribution
        output.push_str("Coherence Distribution:\n");
        let mut coherence_entries: Vec<_> = stats.coherence_distribution.iter().collect();
        coherence_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (range, count) in coherence_entries {
            output.push_str(&format!("  {:<20}: {}\n", range, count));
        }
        output.push('\n');

        // Information distribution
        output.push_str("Information Distribution:\n");
        let mut info_entries: Vec<_> = stats.information_distribution.iter().collect();
        info_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (range, count) in info_entries {
            output.push_str(&format!("  {:<20}: {}\n", range, count));
        }
        output.push('\n');

        // Entanglement
        output.push_str("Entanglement:\n");
        output.push_str(&format!(
            "  Total Links:       {}\n",
            stats.total_entanglement_links
        ));
        output.push_str(&format!(
            "  Avg per Pool:      {:.2}\n",
            stats.average_entanglement_per_pool
        ));

        output
    }

    /// Visualize decoherence statistics
    ///
    /// # Arguments
    /// * `stats` - Decoherence statistics
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_decoherence(stats: &DecoherenceStatistics) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║               DECOHERENCE STATISTICS                       ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        // Collapse attempts
        output.push_str("Wave Function Collapse:\n");
        output.push_str(&format!(
            "  Total Attempts:    {}\n",
            stats.total_collapse_attempts
        ));
        output.push_str(&format!(
            "  Successful:        {} ({:.1}%)\n",
            stats.successful_collapses,
            stats.collapse_success_rate * 100.0
        ));
        output.push_str(&format!(
            "  Failed:            {} ({:.1}%)\n",
            stats.failed_collapses,
            (1.0 - stats.collapse_success_rate) * 100.0
        ));
        output.push('\n');

        // Collapse probability
        output.push_str("Collapse Probability:\n");
        output.push_str(&format!(
            "  Average:           {:.3}\n",
            stats.average_collapse_probability
        ));
        output.push('\n');

        // Decoherence time
        output.push_str("Decoherence Time:\n");
        output.push_str(&format!(
            "  Average:           {:.2} s\n",
            stats.average_decoherence_time
        ));
        output.push('\n');

        // Decoherence time distribution
        output.push_str("Decoherence Time Distribution:\n");
        let mut time_entries: Vec<_> = stats.decoherence_time_distribution.iter().collect();
        time_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (range, count) in time_entries {
            output.push_str(&format!("  {:<20}: {}\n", range, count));
        }
        output.push('\n');

        // Coherence loss rate
        output.push_str("Coherence Dynamics:\n");
        output.push_str(&format!(
            "  Loss Rate:         {:.3} /s\n",
            stats.coherence_loss_rate
        ));

        output
    }

    /// Visualize attractor field statistics
    ///
    /// # Arguments
    /// * `stats` - Attractor field statistics
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_attractor_fields(stats: &AttractorFieldStatistics) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              ATTRACTOR FIELD STATISTICS                    ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        // Attractor counts
        output.push_str("Attractor Field Distribution:\n");
        output.push_str(&format!(
            "  Total:             {}\n",
            stats.total_attractor_fields
        ));
        output.push_str(&format!(
            "  Strong:            {} ({:.1}%)\n",
            stats.strong_attractor_fields,
            if stats.total_attractor_fields > 0 {
                stats.strong_attractor_fields as Float / stats.total_attractor_fields as Float
                    * 100.0
            } else {
                0.0
            }
        ));
        output.push_str(&format!(
            "  Medium:            {} ({:.1}%)\n",
            stats.medium_attractor_fields,
            if stats.total_attractor_fields > 0 {
                stats.medium_attractor_fields as Float / stats.total_attractor_fields as Float
                    * 100.0
            } else {
                0.0
            }
        ));
        output.push_str(&format!(
            "  Weak:              {} ({:.1}%)\n",
            stats.weak_attractor_fields,
            if stats.total_attractor_fields > 0 {
                stats.weak_attractor_fields as Float / stats.total_attractor_fields as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push('\n');

        // Attractor strength
        output.push_str("Attractor Strength:\n");
        output.push_str(&format!(
            "  Average:           {:.3}\n",
            stats.average_attractor_strength
        ));
        output.push('\n');

        // Stability
        output.push_str("Stability:\n");
        output.push_str(&format!(
            "  Average:           {:.3}\n",
            stats.average_stability
        ));
        output.push('\n');

        // Strength distribution
        output.push_str("Strength Distribution:\n");
        let mut strength_entries: Vec<_> = stats.strength_distribution.iter().collect();
        strength_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (range, count) in strength_entries {
            output.push_str(&format!("  {:<20}: {}\n", range, count));
        }
        output.push('\n');

        // Element period distribution
        output.push_str("Element Period Distribution:\n");
        let mut period_entries: Vec<_> = stats.element_period_distribution.iter().collect();
        period_entries.sort_by(|a, b| a.0.cmp(b.0));
        for (period, count) in period_entries {
            output.push_str(&format!("  {:<20}: {}\n", period, count));
        }
        output.push('\n');

        // Energy well depth
        output.push_str("Energy Well Depth:\n");
        output.push_str(&format!(
            "  Total:             {:.2e} J\n",
            stats.total_energy_well_depth
        ));
        output.push_str(&format!(
            "  Average:           {:.2e} J\n",
            stats.average_energy_well_depth
        ));

        output
    }

    /// Visualize transition statistics
    ///
    /// # Arguments
    /// * `stats` - Transition statistics
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_transitions(stats: &TransitionStatistics) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║       CONSCIOUSNESS-TO-MATTER TRANSITION STATISTICS          ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        // Transition counts
        output.push_str("Transition Distribution:\n");
        output.push_str(&format!(
            "  Total:             {}\n",
            stats.total_transitions
        ));
        output.push_str(&format!(
            "  Active:            {} ({:.1}%)\n",
            stats.active_transitions,
            if stats.total_transitions > 0 {
                stats.active_transitions as Float / stats.total_transitions as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push_str(&format!(
            "  Completed:         {} ({:.1}%)\n",
            stats.completed_transitions,
            if stats.total_transitions > 0 {
                stats.completed_transitions as Float / stats.total_transitions as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push_str(&format!(
            "  Failed:            {} ({:.1}%)\n",
            stats.failed_transitions,
            if stats.total_transitions > 0 {
                stats.failed_transitions as Float / stats.total_transitions as Float * 100.0
            } else {
                0.0
            }
        ));
        output.push('\n');

        // Success rate
        output.push_str("Success Metrics:\n");
        output.push_str(&format!(
            "  Success Rate:      {:.1}%\n",
            stats.transition_success_rate * 100.0
        ));
        output.push_str(&format!(
            "  Attractor Match:   {:.1}%\n",
            stats.attractor_match_rate * 100.0
        ));
        output.push('\n');

        // Transition time
        output.push_str("Transition Time:\n");
        output.push_str(&format!(
            "  Average:           {:.2} s\n",
            stats.average_transition_time
        ));
        output.push('\n');

        // Transition time distribution
        output.push_str("Transition Time Distribution:\n");
        let mut time_entries: Vec<_> = stats.transition_time_distribution.iter().collect();
        time_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (range, count) in time_entries {
            output.push_str(&format!("  {:<20}: {}\n", range, count));
        }
        output.push('\n');

        // State distribution
        output.push_str("Active States:\n");
        let mut state_entries: Vec<_> = stats.state_distribution.iter().collect();
        state_entries.sort_by(|a, b| b.1.cmp(a.1));
        for (state, count) in state_entries {
            output.push_str(&format!("  {:<20}: {}\n", state, count));
        }
        output.push('\n');

        // Decoherence level
        output.push_str("Decoherence at Completion:\n");
        output.push_str(&format!(
            "  Average:           {:.3}\n",
            stats.average_decoherence_level
        ));

        output
    }

    /// Visualize complete consciousness-to-matter system
    ///
    /// # Arguments
    /// * `tracker` - Statistics tracker
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_consciousness_to_matter_system(
        tracker: &ConsciousnessToMatterStatisticsTracker,
    ) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║        CONSCIOUSNESS-TO-MATTER SYSTEM OVERVIEW              ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        // Summary
        output.push_str(&tracker.get_summary());
        output.push_str("\n\n");

        // Quantum pools
        output.push_str(&Self::visualize_quantum_pools(&tracker.quantum_pool_stats));
        output.push('\n');

        // Decoherence
        output.push_str(&Self::visualize_decoherence(&tracker.decoherence_stats));
        output.push('\n');

        // Attractor fields
        output.push_str(&Self::visualize_attractor_fields(
            &tracker.attractor_field_stats,
        ));
        output.push('\n');

        // Transitions
        output.push_str(&Self::visualize_transitions(&tracker.transition_stats));

        output
    }

    /// Visualize historical trends
    ///
    /// # Arguments
    /// * `tracker` - Statistics tracker
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_historical_trends(tracker: &ConsciousnessToMatterStatisticsTracker) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║             HISTORICAL QUANTUM TRENDS                      ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        if tracker.historical_data.is_empty() {
            output.push_str("No historical data available.\n");
            return output;
        }

        // Show first and last data points
        let first = tracker.historical_data.first().unwrap();
        let last = tracker.historical_data.last().unwrap();

        output.push_str("First Data Point:\n");
        output.push_str(&format!("  Timestamp:         {}\n", first.timestamp));
        output.push_str(&format!("  Coherent Pools:    {}\n", first.coherent_pools));
        output.push_str(&format!("  Decohered Pools:   {}\n", first.decohered_pools));
        output.push_str(&format!(
            "  Active Transitions:{}\n",
            first.active_transitions
        ));
        output.push_str(&format!(
            "  Completed Trans:   {}\n",
            first.completed_transitions
        ));
        output.push_str(&format!(
            "  Average Coherence: {:.3}\n",
            first.average_coherence
        ));
        output.push_str(&format!(
            "  Total Information: {:.2} bits\n",
            first.total_information_content
        ));
        output.push('\n');

        output.push_str("Last Data Point:\n");
        output.push_str(&format!("  Timestamp:         {}\n", last.timestamp));
        output.push_str(&format!("  Coherent Pools:    {}\n", last.coherent_pools));
        output.push_str(&format!("  Decohered Pools:   {}\n", last.decohered_pools));
        output.push_str(&format!(
            "  Active Transitions:{}\n",
            last.active_transitions
        ));
        output.push_str(&format!(
            "  Completed Trans:   {}\n",
            last.completed_transitions
        ));
        output.push_str(&format!(
            "  Average Coherence: {:.3}\n",
            last.average_coherence
        ));
        output.push_str(&format!(
            "  Total Information: {:.2} bits\n",
            last.total_information_content
        ));
        output.push('\n');

        // Calculate trends
        if tracker.historical_data.len() > 1 {
            let coherence_change = last.average_coherence - first.average_coherence;
            let info_change = last.total_information_content - first.total_information_content;

            output.push_str("Trends:\n");
            output.push_str(&format!("  Coherence Change:  {:+.3}\n", coherence_change));
            output.push_str(&format!("  Information Change:{:+.2} bits\n", info_change));
        }

        output.push_str(&format!(
            "\nTotal Data Points:  {}\n",
            tracker.historical_data.len()
        ));

        output
    }

    /// Visualize periodic table attractor fields
    ///
    /// # Arguments
    /// * `manager` - Consciousness-to-matter manager
    ///
    /// # Returns
    /// Formatted visualization string
    pub fn visualize_periodic_table_attractors(manager: &ConsciousnessToMatterManager) -> String {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║            PERIODIC TABLE ATTRACTOR FIELDS                 ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");

        let fields = manager.attractor_fields.all_fields();

        output.push_str(&format!("Total Attractor Fields: {}\n\n", fields.len()));

        // Group by period
        let mut period_groups: std::collections::HashMap<u32, Vec<String>> =
            std::collections::HashMap::new();

        for (field_id, attractor) in fields {
            let period = Self::get_period(attractor.element_type);
            period_groups
                .entry(period)
                .or_default()
                .push(field_id.clone());
        }

        // Display by period
        for period in 1..=7 {
            if let Some(field_ids) = period_groups.get(&period) {
                output.push_str(&format!(
                    "Period {} ({} elements):\n",
                    period,
                    field_ids.len()
                ));

                for field_id in field_ids {
                    if let Some(attractor) = fields.get(field_id.as_str()) {
                        output.push_str(&format!(
                            "  {:<3}: Z={}, Strength={:.2}, Stability={:.2}, Energy={:.2e} J\n",
                            Self::get_element_symbol(attractor.element_type),
                            attractor.element_type,
                            attractor.attractor_strength,
                            attractor.stability,
                            attractor.energy_well_depth
                        ));
                    }
                }
                output.push('\n');
            }
        }

        output
    }

    /// Get period from atomic number
    fn get_period(atomic_number: u32) -> u32 {
        match atomic_number {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 1,
        }
    }

    /// Get element symbol from atomic number
    fn get_element_symbol(atomic_number: u32) -> String {
        let symbols = [
            "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
            "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga",
            "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh",
            "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr",
            "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta",
            "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr",
            "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md",
            "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc",
            "Lv", "Ts", "Og",
        ];

        if atomic_number == 0 || atomic_number > symbols.len() as u32 {
            return "?".to_string();
        }

        symbols[(atomic_number - 1) as usize].to_string()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualize_quantum_pools() {
        let stats = QuantumPoolStatistics {
            total_pools: 10,
            coherent_pools: 7,
            decohered_pools: 3,
            total_information_content: 1000.0,
            average_information_content: 100.0,
            total_energy: 1.0e-18,
            average_coherence: 0.7,
            coherence_distribution: std::collections::HashMap::new(),
            information_distribution: std::collections::HashMap::new(),
            total_entanglement_links: 20,
            average_entanglement_per_pool: 2.0,
        };

        let output = QuantumVisualizer::visualize_quantum_pools(&stats);
        assert!(output.contains("QUANTUM ENERGY POOL STATISTICS"));
    }

    #[test]
    fn test_get_element_symbol() {
        assert_eq!(QuantumVisualizer::get_element_symbol(1), "H");
        assert_eq!(QuantumVisualizer::get_element_symbol(6), "C");
        assert_eq!(QuantumVisualizer::get_element_symbol(8), "O");
        assert_eq!(QuantumVisualizer::get_element_symbol(118), "Og");
    }

    #[test]
    fn test_get_period() {
        assert_eq!(QuantumVisualizer::get_period(1), 1);
        assert_eq!(QuantumVisualizer::get_period(6), 2);
        assert_eq!(QuantumVisualizer::get_period(19), 4);
        assert_eq!(QuantumVisualizer::get_period(92), 7);
    }
}
